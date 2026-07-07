use super::*;

fn exact_input(title: &str) -> M5DocsReferenceCardResolutionInput {
    M5DocsReferenceCardResolutionInput {
        card_title_repr: title.to_owned(),
        initiating_file_repr: "src/client.rs".to_owned(),
        initiating_symbol_repr: "Client::send".to_owned(),
        symbol_anchor: M5DocsSymbolAnchor::FunctionSymbol,
        corpus_class: M5DocsCorpusClass::ApiReference,
        source_provider: M5DocsSourceProvider::FirstPartyHosted,
        match_state: M5DocsMatchState::ExactMatch,
        override_reason: M5DocsOverrideReason::NoOverride,
        version_scope: M5DocsVersionScope::ExactVersionMatch,
        freshness_state: M5DocsFreshnessState::LiveCurrent,
        cited_source_revision_repr: "rev:api-1.4.0".to_owned(),
        open_action_target_repr: "open:doc/api/client-send".to_owned(),
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_exact_match_reads_as_exact_symbol_linkage() {
    let resolved = resolve_reference_card(&exact_input("Client::send")).expect("resolves");
    assert_eq!(
        resolved.linkage_strength,
        M5DocsSymbolLinkageStrength::ExactSymbolLinkage
    );
    assert!(resolved.is_exact_symbol_linkage);
    assert!(resolved.is_symbol_resolved);
    assert_eq!(
        resolved.freshness_posture,
        M5DocsCardFreshnessPosture::CurrentLive
    );
    assert!(resolved.shows_as_live);
    // The linkage disclosure is always present.
    assert!(!resolved.linkage_disclosure.headline.trim().is_empty());
    assert_eq!(
        resolved.linkage_disclosure.linkage_strength,
        M5DocsSymbolLinkageStrength::ExactSymbolLinkage
    );
}

#[test]
fn resolver_unresolved_anchor_never_reads_as_exact_symbol() {
    // Declared exact match, but an unresolved anchor must fall back to keyword.
    let keyword = resolve_reference_card(&M5DocsReferenceCardResolutionInput {
        symbol_anchor: M5DocsSymbolAnchor::UnresolvedAnchor,
        ..exact_input("retry helpers")
    })
    .expect("resolves");
    assert_eq!(
        keyword.linkage_strength,
        M5DocsSymbolLinkageStrength::KeywordFallbackLinkage
    );
    assert!(!keyword.is_exact_symbol_linkage);
    assert!(!keyword.is_symbol_resolved);
    assert!(keyword.linkage_strength.is_keyword_or_unresolved());

    // Unresolved AND stale becomes an explicit no-linkage stub.
    let stub = resolve_reference_card(&M5DocsReferenceCardResolutionInput {
        symbol_anchor: M5DocsSymbolAnchor::UnresolvedAnchor,
        match_state: M5DocsMatchState::StaleMatch,
        ..exact_input("legacy keys")
    })
    .expect("resolves");
    assert_eq!(
        stub.linkage_strength,
        M5DocsSymbolLinkageStrength::UnresolvedNoLinkage
    );
    assert!(!stub.is_symbol_resolved);
}

#[test]
fn resolver_project_precedence_reads_as_project_specific_linkage() {
    let resolved = resolve_reference_card(&M5DocsReferenceCardResolutionInput {
        symbol_anchor: M5DocsSymbolAnchor::FunctionSymbol,
        corpus_class: M5DocsCorpusClass::CodebaseSymbol,
        match_state: M5DocsMatchState::ProjectSpecificMatch,
        override_reason: M5DocsOverrideReason::ProjectPinnedOverride,
        version_scope: M5DocsVersionScope::ProjectSpecific,
        ..exact_input("resolve_run_context")
    })
    .expect("resolves");
    assert_eq!(
        resolved.linkage_strength,
        M5DocsSymbolLinkageStrength::ProjectSpecificLinkage
    );
    assert!(resolved.linkage_strength.is_project_specific());
}

#[test]
fn resolver_nearby_version_reads_as_nearby_version_linkage() {
    let resolved = resolve_reference_card(&M5DocsReferenceCardResolutionInput {
        match_state: M5DocsMatchState::NearbyMatch,
        version_scope: M5DocsVersionScope::NearbyVersion,
        ..exact_input("Widget")
    })
    .expect("resolves");
    assert_eq!(
        resolved.linkage_strength,
        M5DocsSymbolLinkageStrength::NearbyVersionLinkage
    );

    // Even an exact match at a nearby version reads as nearby, not exact.
    let exact_but_nearby = resolve_reference_card(&M5DocsReferenceCardResolutionInput {
        match_state: M5DocsMatchState::ExactMatch,
        version_scope: M5DocsVersionScope::NearbyVersion,
        ..exact_input("Widget")
    })
    .expect("resolves");
    assert_eq!(
        exact_but_nearby.linkage_strength,
        M5DocsSymbolLinkageStrength::NearbyVersionLinkage
    );
}

#[test]
fn resolver_mirror_or_cache_served_reads_as_heuristic_linkage() {
    for match_state in [
        M5DocsMatchState::MirroredMatch,
        M5DocsMatchState::CachedMatch,
        M5DocsMatchState::StaleMatch,
    ] {
        let resolved = resolve_reference_card(&M5DocsReferenceCardResolutionInput {
            match_state,
            version_scope: M5DocsVersionScope::PinnedRange,
            ..exact_input("Config")
        })
        .expect("resolves");
        assert_eq!(
            resolved.linkage_strength,
            M5DocsSymbolLinkageStrength::HeuristicLinkage,
            "match {} should read as heuristic linkage",
            match_state.as_str()
        );
    }
}

#[test]
fn resolver_never_shows_cached_or_mirrored_or_stale_match_as_live() {
    let cached = resolve_reference_card(&M5DocsReferenceCardResolutionInput {
        match_state: M5DocsMatchState::CachedMatch,
        ..exact_input("Config")
    })
    .expect("resolves");
    assert_eq!(
        cached.freshness_posture,
        M5DocsCardFreshnessPosture::CachedExplicitNotLive
    );
    assert!(!cached.shows_as_live);

    let mirrored = resolve_reference_card(&M5DocsReferenceCardResolutionInput {
        match_state: M5DocsMatchState::MirroredMatch,
        ..exact_input("Config")
    })
    .expect("resolves");
    assert_eq!(
        mirrored.freshness_posture,
        M5DocsCardFreshnessPosture::MirroredExplicitNotLive
    );
    assert!(!mirrored.shows_as_live);

    let stale = resolve_reference_card(&M5DocsReferenceCardResolutionInput {
        match_state: M5DocsMatchState::StaleMatch,
        ..exact_input("Config")
    })
    .expect("resolves");
    assert_eq!(
        stale.freshness_posture,
        M5DocsCardFreshnessPosture::StaleFlagged
    );
    assert!(!stale.shows_as_live);
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty_title = M5DocsReferenceCardResolutionInput {
        card_title_repr: "  ".to_owned(),
        ..exact_input("Client::send")
    };
    assert_eq!(
        resolve_reference_card(&empty_title),
        Err(M5DocsReferenceCardResolutionError::EmptyCardTitle)
    );

    let empty_file = M5DocsReferenceCardResolutionInput {
        initiating_file_repr: "".to_owned(),
        ..exact_input("Client::send")
    };
    assert_eq!(
        resolve_reference_card(&empty_file),
        Err(M5DocsReferenceCardResolutionError::EmptyInitiatingAnchor)
    );

    let empty_symbol = M5DocsReferenceCardResolutionInput {
        initiating_symbol_repr: "   ".to_owned(),
        ..exact_input("Client::send")
    };
    assert_eq!(
        resolve_reference_card(&empty_symbol),
        Err(M5DocsReferenceCardResolutionError::EmptyInitiatingAnchor)
    );

    let empty_open = M5DocsReferenceCardResolutionInput {
        open_action_target_repr: "".to_owned(),
        ..exact_input("Client::send")
    };
    assert_eq!(
        resolve_reference_card(&empty_open),
        Err(M5DocsReferenceCardResolutionError::EmptyOpenActionTarget)
    );

    let forbidden = M5DocsReferenceCardResolutionInput {
        open_action_target_repr: "https://example.test/docs".to_owned(),
        ..exact_input("Client::send")
    };
    assert_eq!(
        resolve_reference_card(&forbidden),
        Err(M5DocsReferenceCardResolutionError::ForbiddenCardMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_reference_card_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_DOCS_REFERENCE_CARD_PRIMITIVE_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_reference_card_primitive_packet();
    let present: std::collections::BTreeSet<_> = packet
        .reference_card_rows
        .iter()
        .map(|r| r.consumer_surface)
        .collect();
    for surface in M5DocsReferenceCardConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.reference_card_rows.len(),
        M5DocsReferenceCardConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_reference_card_primitive_packet();
    for row in &packet.reference_card_rows {
        for part in M5DocsReferenceCardAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5DocsReferenceCardExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5DocsAccessibilityRoute::KeyboardFocusable));
        assert!(!row.example_resolutions.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_reference_card_primitive_packet();
    let cases: Vec<&M5DocsReferenceCardResolutionCase> = packet
        .reference_card_rows
        .iter()
        .flat_map(|row| row.example_resolutions.iter())
        .collect();

    for anchor in M5DocsSymbolAnchor::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.symbol_anchor == anchor),
            "no worked resolution exercises symbol anchor {}",
            anchor.as_str()
        );
    }
    for linkage in M5DocsSymbolLinkageStrength::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.linkage_strength == linkage),
            "no worked resolution exercises linkage strength {}",
            linkage.as_str()
        );
    }
    for posture in M5DocsCardFreshnessPosture::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.freshness_posture == posture),
            "no worked resolution exercises freshness posture {}",
            posture.as_str()
        );
    }
    for match_state in M5DocsMatchState::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.match_state == match_state),
            "no worked resolution exercises match state {}",
            match_state.as_str()
        );
    }
    for reason in M5DocsOverrideReason::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.override_reason == reason),
            "no worked resolution exercises override reason {}",
            reason.as_str()
        );
    }
    for scope in M5DocsVersionScope::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.version_scope == scope),
            "no worked resolution exercises version scope {}",
            scope.as_str()
        );
    }
    for provider in M5DocsSourceProvider::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.source_provider == provider),
            "no worked resolution exercises source provider {}",
            provider.as_str()
        );
    }
    for freshness in M5DocsFreshnessState::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.freshness_state == freshness),
            "no worked resolution exercises freshness state {}",
            freshness.as_str()
        );
    }
    for corpus in M5DocsCorpusClass::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.corpus_class == corpus),
            "no worked resolution exercises corpus class {}",
            corpus.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_anchor() {
    let packet = seeded_m5_reference_card_primitive_packet();
    for row in &packet.reference_card_rows {
        for case in &row.example_resolutions {
            assert!(
                case.is_self_consistent(),
                "worked case for {} drifted from resolver output",
                row.consumer_surface.as_str()
            );
            assert!(
                !case.resolved.initiating_file_repr.trim().is_empty()
                    && !case.resolved.initiating_symbol_repr.trim().is_empty(),
                "worked case for {} dropped its initiating anchor",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_reference_card_primitive_packet();
    packet.reference_card_rows.retain(|row| {
        row.consumer_surface != M5DocsReferenceCardConsumerSurface::SupportEvidenceCard
    });
    assert!(packet
        .validate()
        .contains(&M5DocsReferenceCardPrimitiveViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_reference_card_primitive_packet();
    packet.vocabulary_set.linkage_strengths.pop();
    assert!(packet
        .validate()
        .contains(&M5DocsReferenceCardPrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_reference_card_primitive_packet();
    packet.reference_card_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5DocsReferenceCardAnatomyPart::InitiatingCodeAnchor);
    assert!(packet
        .validate()
        .contains(&M5DocsReferenceCardPrimitiveViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_reference_card_primitive_packet();
    packet.reference_card_rows[0]
        .export_fields
        .retain(|f| *f != M5DocsReferenceCardExportField::InitiatingAnchor);
    assert!(packet
        .validate()
        .contains(&M5DocsReferenceCardPrimitiveViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_reference_card_primitive_packet();
    packet.reference_card_rows[0].example_resolutions[0]
        .resolved
        .is_exact_symbol_linkage = false;
    assert!(packet
        .validate()
        .contains(&M5DocsReferenceCardPrimitiveViolation::ExampleResolutionDrift));
}

#[test]
fn example_resolution_missing_fails() {
    let mut packet = seeded_m5_reference_card_primitive_packet();
    packet.reference_card_rows[2].example_resolutions.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsReferenceCardPrimitiveViolation::ExampleResolutionMissing));
}

#[test]
fn linkage_state_coverage_unproven_fails_when_a_named_state_is_missing() {
    let mut packet = seeded_m5_reference_card_primitive_packet();
    // Replace every example with an exact-symbol match so nearby/project/keyword states
    // are no longer proven.
    for row in &mut packet.reference_card_rows {
        row.example_resolutions = vec![M5DocsReferenceCardResolutionCase::resolved(exact_input(
            "Client::send",
        ))];
    }
    assert!(packet
        .validate()
        .contains(&M5DocsReferenceCardPrimitiveViolation::LinkageStateCoverageUnproven));
}

#[test]
fn anchor_identity_unproven_fails_when_no_unresolved_example_present() {
    let mut packet = seeded_m5_reference_card_primitive_packet();
    // Every example resolves to a symbol, so the unresolved-anchor side is unproven.
    for row in &mut packet.reference_card_rows {
        row.example_resolutions = vec![M5DocsReferenceCardResolutionCase::resolved(exact_input(
            "Client::send",
        ))];
    }
    assert!(packet
        .validate()
        .contains(&M5DocsReferenceCardPrimitiveViolation::AnchorIdentityUnproven));
}

#[test]
fn freshness_visibility_unproven_fails_when_only_live_examples_present() {
    let mut packet = seeded_m5_reference_card_primitive_packet();
    for row in &mut packet.reference_card_rows {
        row.example_resolutions = vec![M5DocsReferenceCardResolutionCase::resolved(exact_input(
            "Client::send",
        ))];
    }
    assert!(packet
        .validate()
        .contains(&M5DocsReferenceCardPrimitiveViolation::FreshnessVisibilityUnproven));
}

#[test]
fn card_invariant_violation_fails() {
    let mut packet = seeded_m5_reference_card_primitive_packet();
    packet.reference_card_rows[0].hides_symbol_linkage = true;
    assert!(packet
        .validate()
        .contains(&M5DocsReferenceCardPrimitiveViolation::CardInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_reference_card_primitive_packet();
    packet.reference_card_rows[0]
        .required_proof_packet_refs
        .clear();
    assert!(packet
        .validate()
        .contains(&M5DocsReferenceCardPrimitiveViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_reference_card_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsReferenceCardPrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_reference_card_primitive_packet();
    packet.governance_review.cached_or_stale_never_shown_as_live = false;
    assert!(packet
        .validate()
        .contains(&M5DocsReferenceCardPrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_reference_card_primitive_packet();
    packet.consumer_projection.anchor_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5DocsReferenceCardPrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_reference_card_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5DocsReferenceCardPrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_reference_card_primitive_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5DocsReferenceCardPrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_reference_card_primitive_packet().render_markdown_summary();
    for surface in M5DocsReferenceCardConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_reference_card_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5DocsReferenceCardConsumerSurface::ALL.len()
    );
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5DocsReferenceCardConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_reference_card_primitive_export()
        .expect("checked M5 reference-card primitive export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_DOCS_REFERENCE_CARD_PRIMITIVE_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_reference_card_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_reference_card_primitive_onboarding_reference_beta_narrowed(),
        seeded_m5_reference_card_primitive_ai_explanation_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.reference_card_rows.len(),
            M5DocsReferenceCardConsumerSurface::ALL.len()
        );
    }

    let onboarding = seeded_m5_reference_card_primitive_onboarding_reference_beta_narrowed();
    let row = onboarding
        .reference_card_rows
        .iter()
        .find(|r| r.consumer_surface == M5DocsReferenceCardConsumerSurface::OnboardingReferenceCard)
        .expect("onboarding row present");
    assert_eq!(row.qualification, M5DocsQualificationClass::Beta);

    let ai = seeded_m5_reference_card_primitive_ai_explanation_preview_narrowed();
    let row = ai
        .reference_card_rows
        .iter()
        .find(|r| r.consumer_surface == M5DocsReferenceCardConsumerSurface::AiExplanationCard)
        .expect("ai row present");
    assert_eq!(row.qualification, M5DocsQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let onboarding: M5DocsReferenceCardPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/docs/m5/m5-symbol-linked-reference-card-primitive/onboarding_reference_beta_narrowed.json"
    )))
    .expect("onboarding fixture parses");
    assert!(onboarding.validate().is_empty());
    assert_eq!(
        onboarding,
        seeded_m5_reference_card_primitive_onboarding_reference_beta_narrowed()
    );

    let ai: M5DocsReferenceCardPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/docs/m5/m5-symbol-linked-reference-card-primitive/ai_explanation_preview_narrowed.json"
    )))
    .expect("ai fixture parses");
    assert!(ai.validate().is_empty());
    assert_eq!(
        ai,
        seeded_m5_reference_card_primitive_ai_explanation_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_reference_card_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
