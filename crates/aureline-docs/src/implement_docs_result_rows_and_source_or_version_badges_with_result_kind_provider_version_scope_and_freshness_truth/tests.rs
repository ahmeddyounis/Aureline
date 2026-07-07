use super::*;

fn live_ready_input(title: &str) -> M5DocsResultRowResolutionInput {
    M5DocsResultRowResolutionInput {
        title_repr: title.to_owned(),
        result_kind: M5DocsResultKind::DocPage,
        corpus_class: M5DocsCorpusClass::FirstPartyDocs,
        source_provider: M5DocsSourceProvider::FirstPartyHosted,
        match_state: M5DocsMatchState::ExactMatch,
        override_reason: M5DocsOverrideReason::NoOverride,
        symbol_match_confidence: M5DocsSymbolMatchConfidence::NotSymbolScoped,
        version_scope: M5DocsVersionScope::ExactVersionMatch,
        freshness_state: M5DocsFreshnessState::LiveCurrent,
        open_action_target_repr: "open:docs/getting-started".to_owned(),
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_live_first_party_reads_as_first_party_reference() {
    let resolved = resolve_docs_result_row(&live_ready_input("Getting started")).expect("resolves");
    assert_eq!(
        resolved.source_badge_class,
        M5DocsSourceBadgeClass::FirstPartyReference
    );
    assert!(!resolved.is_local_or_project);
    assert_eq!(
        resolved.freshness_posture,
        M5DocsResultFreshnessPosture::CurrentLive
    );
    assert!(resolved.shows_as_live);
    assert!(resolved.rank_reason_disclosure.is_none());
}

#[test]
fn resolver_project_specific_scope_reads_as_local_project() {
    let resolved = resolve_docs_result_row(&M5DocsResultRowResolutionInput {
        corpus_class: M5DocsCorpusClass::CodebaseSymbol,
        version_scope: M5DocsVersionScope::ProjectSpecific,
        override_reason: M5DocsOverrideReason::ProjectPinnedOverride,
        ..live_ready_input("resolve_run_context")
    })
    .expect("resolves");
    // Project-specific scope wins ahead of the codebase-symbol check.
    assert_eq!(
        resolved.source_badge_class,
        M5DocsSourceBadgeClass::LocalProjectDocs
    );
    assert!(resolved.is_local_or_project);
    let disclosure = resolved.rank_reason_disclosure.expect("disclosure present");
    assert_eq!(
        disclosure.rank_factor,
        M5DocsRankFactor::ProjectDocPrecedence
    );
    assert!(!disclosure.headline.trim().is_empty());
}

#[test]
fn resolver_codebase_symbol_reads_as_workspace_spec() {
    let resolved = resolve_docs_result_row(&M5DocsResultRowResolutionInput {
        corpus_class: M5DocsCorpusClass::CodebaseSymbol,
        source_provider: M5DocsSourceProvider::OfflineImport,
        version_scope: M5DocsVersionScope::PinnedRange,
        ..live_ready_input("internal::cache_key")
    })
    .expect("resolves");
    assert_eq!(
        resolved.source_badge_class,
        M5DocsSourceBadgeClass::WorkspaceSpec
    );
    assert!(resolved.is_local_or_project);
}

#[test]
fn resolver_vendor_and_extension_and_ai_badges() {
    let vendor = resolve_docs_result_row(&M5DocsResultRowResolutionInput {
        corpus_class: M5DocsCorpusClass::VendorDependency,
        source_provider: M5DocsSourceProvider::ThirdPartyHosted,
        ..live_ready_input("Vendor SDK guide")
    })
    .expect("resolves");
    assert_eq!(
        vendor.source_badge_class,
        M5DocsSourceBadgeClass::LiveVendorUpstream
    );
    assert!(!vendor.is_local_or_project);

    let extension = resolve_docs_result_row(&M5DocsResultRowResolutionInput {
        corpus_class: M5DocsCorpusClass::CommunityContributed,
        source_provider: M5DocsSourceProvider::OfflineImport,
        ..live_ready_input("Community setup guide")
    })
    .expect("resolves");
    assert_eq!(
        extension.source_badge_class,
        M5DocsSourceBadgeClass::ExtensionContributed
    );

    let ai = resolve_docs_result_row(&M5DocsResultRowResolutionInput {
        source_provider: M5DocsSourceProvider::AiDerived,
        ..live_ready_input("How retries work")
    })
    .expect("resolves");
    assert_eq!(
        ai.source_badge_class,
        M5DocsSourceBadgeClass::AiDerivedExplanation
    );
}

#[test]
fn resolver_never_shows_cached_or_mirrored_or_stale_match_as_live() {
    // Declared freshness says live, but a cached match must not read as live.
    let cached = resolve_docs_result_row(&M5DocsResultRowResolutionInput {
        match_state: M5DocsMatchState::CachedMatch,
        ..live_ready_input("Client::new")
    })
    .expect("resolves");
    assert_eq!(
        cached.freshness_posture,
        M5DocsResultFreshnessPosture::CachedExplicitNotLive
    );
    assert!(!cached.shows_as_live);

    let mirrored = resolve_docs_result_row(&M5DocsResultRowResolutionInput {
        match_state: M5DocsMatchState::MirroredMatch,
        ..live_ready_input("Rate limits")
    })
    .expect("resolves");
    assert_eq!(
        mirrored.freshness_posture,
        M5DocsResultFreshnessPosture::MirroredExplicitNotLive
    );
    assert!(!mirrored.shows_as_live);

    let stale = resolve_docs_result_row(&M5DocsResultRowResolutionInput {
        match_state: M5DocsMatchState::StaleMatch,
        ..live_ready_input("Deprecated flag")
    })
    .expect("resolves");
    assert_eq!(
        stale.freshness_posture,
        M5DocsResultFreshnessPosture::StaleFlagged
    );
    assert!(!stale.shows_as_live);
}

#[test]
fn resolver_nearby_version_with_no_override_reads_as_version_adjacency() {
    let resolved = resolve_docs_result_row(&M5DocsResultRowResolutionInput {
        version_scope: M5DocsVersionScope::NearbyVersion,
        override_reason: M5DocsOverrideReason::NoOverride,
        ..live_ready_input("How retries work")
    })
    .expect("resolves");
    let disclosure = resolved.rank_reason_disclosure.expect("disclosure present");
    assert_eq!(disclosure.rank_factor, M5DocsRankFactor::VersionAdjacency);
}

#[test]
fn resolver_default_ranking_has_no_disclosure() {
    let resolved = resolve_docs_result_row(&live_ready_input("Getting started")).expect("resolves");
    assert!(resolved.rank_reason_disclosure.is_none());
}

#[test]
fn resolver_maps_every_override_reason_to_a_factor() {
    use M5DocsOverrideReason as Override;
    let expectations = [
        (
            Override::ProjectPinnedOverride,
            M5DocsRankFactor::ProjectDocPrecedence,
        ),
        (
            Override::LocalFreshnessOverride,
            M5DocsRankFactor::MirrorFreshness,
        ),
        (
            Override::ExplicitUserPreference,
            M5DocsRankFactor::ExplicitPreference,
        ),
        (
            Override::VendorSourceUnavailable,
            M5DocsRankFactor::VendorFallback,
        ),
        (
            Override::PolicyScopedOverride,
            M5DocsRankFactor::PolicyScopedRanking,
        ),
    ];
    for (reason, factor) in expectations {
        let resolved = resolve_docs_result_row(&M5DocsResultRowResolutionInput {
            override_reason: reason,
            ..live_ready_input("Ranked result")
        })
        .expect("resolves");
        assert_eq!(
            resolved
                .rank_reason_disclosure
                .expect("disclosure")
                .rank_factor,
            factor,
            "override {} mapped to wrong factor",
            reason.as_str()
        );
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty_title = M5DocsResultRowResolutionInput {
        title_repr: "  ".to_owned(),
        ..live_ready_input("Getting started")
    };
    assert_eq!(
        resolve_docs_result_row(&empty_title),
        Err(M5DocsResultRowResolutionError::EmptyTitle)
    );

    let empty_open = M5DocsResultRowResolutionInput {
        open_action_target_repr: "".to_owned(),
        ..live_ready_input("Getting started")
    };
    assert_eq!(
        resolve_docs_result_row(&empty_open),
        Err(M5DocsResultRowResolutionError::EmptyOpenActionTarget)
    );

    let forbidden = M5DocsResultRowResolutionInput {
        open_action_target_repr: "https://example.test/docs".to_owned(),
        ..live_ready_input("Getting started")
    };
    assert_eq!(
        resolve_docs_result_row(&forbidden),
        Err(M5DocsResultRowResolutionError::ForbiddenResultMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_docs_result_row_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_DOCS_RESULT_ROW_PRIMITIVE_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_docs_result_row_primitive_packet();
    let present: std::collections::BTreeSet<_> = packet
        .result_rows
        .iter()
        .map(|r| r.consumer_surface)
        .collect();
    for surface in M5DocsResultConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.result_rows.len(),
        M5DocsResultConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_docs_result_row_primitive_packet();
    for row in &packet.result_rows {
        for part in M5DocsResultRowAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5DocsResultRowExportField::MANDATORY {
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
    let packet = seeded_m5_docs_result_row_primitive_packet();
    let cases: Vec<&M5DocsResultRowResolutionCase> = packet
        .result_rows
        .iter()
        .flat_map(|row| row.example_resolutions.iter())
        .collect();

    for kind in M5DocsResultKind::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.result_kind == kind),
            "no worked resolution exercises result kind {}",
            kind.as_str()
        );
    }
    for badge in M5DocsSourceBadgeClass::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.source_badge_class == badge),
            "no worked resolution exercises source-badge class {}",
            badge.as_str()
        );
    }
    for posture in M5DocsResultFreshnessPosture::ALL {
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
    for confidence in M5DocsSymbolMatchConfidence::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.symbol_match_confidence == confidence),
            "no worked resolution exercises symbol-match confidence {}",
            confidence.as_str()
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
    for factor in M5DocsRankFactor::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .rank_reason_disclosure
                .as_ref()
                .is_some_and(|d| d.rank_factor == factor)),
            "no worked resolution exercises rank factor {}",
            factor.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_docs_result_row_primitive_packet();
    for row in &packet.result_rows {
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
    let mut packet = seeded_m5_docs_result_row_primitive_packet();
    packet
        .result_rows
        .retain(|row| row.consumer_surface != M5DocsResultConsumerSurface::CliResultList);
    assert!(packet
        .validate()
        .contains(&M5DocsResultRowPrimitiveViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_docs_result_row_primitive_packet();
    packet.vocabulary_set.source_badge_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5DocsResultRowPrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_docs_result_row_primitive_packet();
    packet.result_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5DocsResultRowAnatomyPart::SourceProviderBadge);
    assert!(packet
        .validate()
        .contains(&M5DocsResultRowPrimitiveViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_docs_result_row_primitive_packet();
    packet.result_rows[0]
        .export_fields
        .retain(|f| *f != M5DocsResultRowExportField::SourceBadgeClass);
    assert!(packet
        .validate()
        .contains(&M5DocsResultRowPrimitiveViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_docs_result_row_primitive_packet();
    packet.result_rows[0].example_resolutions[0]
        .resolved
        .shows_as_live = false;
    assert!(packet
        .validate()
        .contains(&M5DocsResultRowPrimitiveViolation::ExampleResolutionDrift));
}

#[test]
fn example_resolution_missing_fails() {
    let mut packet = seeded_m5_docs_result_row_primitive_packet();
    packet.result_rows[2].example_resolutions.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsResultRowPrimitiveViolation::ExampleResolutionMissing));
}

#[test]
fn local_vs_upstream_coverage_unproven_fails_when_no_local_example_present() {
    let mut packet = seeded_m5_docs_result_row_primitive_packet();
    // Replace every example with an upstream first-party reference so the coverage
    // lint fires (no local/project result remains).
    for row in &mut packet.result_rows {
        row.example_resolutions = vec![M5DocsResultRowResolutionCase::resolved(live_ready_input(
            "Getting started",
        ))];
    }
    assert!(packet
        .validate()
        .contains(&M5DocsResultRowPrimitiveViolation::LocalVsUpstreamCoverageUnproven));
}

#[test]
fn freshness_visibility_unproven_fails_when_only_live_examples_present() {
    let mut packet = seeded_m5_docs_result_row_primitive_packet();
    for row in &mut packet.result_rows {
        row.example_resolutions = vec![M5DocsResultRowResolutionCase::resolved(live_ready_input(
            "Getting started",
        ))];
    }
    assert!(packet
        .validate()
        .contains(&M5DocsResultRowPrimitiveViolation::FreshnessVisibilityUnproven));
}

#[test]
fn rank_reason_inspectable_unproven_fails_when_no_disclosure_present() {
    let mut packet = seeded_m5_docs_result_row_primitive_packet();
    // Every example is a default-ranking live result with no disclosure.
    for row in &mut packet.result_rows {
        row.example_resolutions = vec![M5DocsResultRowResolutionCase::resolved(live_ready_input(
            "Getting started",
        ))];
    }
    assert!(packet
        .validate()
        .contains(&M5DocsResultRowPrimitiveViolation::RankReasonInspectableUnproven));
}

#[test]
fn result_invariant_violation_fails() {
    let mut packet = seeded_m5_docs_result_row_primitive_packet();
    packet.result_rows[0].shows_cached_or_stale_as_live = true;
    assert!(packet
        .validate()
        .contains(&M5DocsResultRowPrimitiveViolation::ResultInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_docs_result_row_primitive_packet();
    packet.result_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsResultRowPrimitiveViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_docs_result_row_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsResultRowPrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_docs_result_row_primitive_packet();
    packet.governance_review.cached_or_stale_never_shown_as_live = false;
    assert!(packet
        .validate()
        .contains(&M5DocsResultRowPrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_docs_result_row_primitive_packet();
    packet.consumer_projection.rank_reason_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5DocsResultRowPrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_docs_result_row_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5DocsResultRowPrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_docs_result_row_primitive_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5DocsResultRowPrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_docs_result_row_primitive_packet().render_markdown_summary();
    for surface in M5DocsResultConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_docs_result_row_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5DocsResultConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5DocsResultConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_docs_result_row_primitive_export()
        .expect("checked M5 docs-result-row primitive export validates");
    assert_eq!(from_disk.packet_id, M5_DOCS_RESULT_ROW_PRIMITIVE_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_docs_result_row_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_docs_result_row_primitive_onboarding_reference_beta_narrowed(),
        seeded_m5_docs_result_row_primitive_ai_citation_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.result_rows.len(),
            M5DocsResultConsumerSurface::ALL.len()
        );
    }

    let onboarding = seeded_m5_docs_result_row_primitive_onboarding_reference_beta_narrowed();
    let row = onboarding
        .result_rows
        .iter()
        .find(|r| r.consumer_surface == M5DocsResultConsumerSurface::OnboardingStepReference)
        .expect("onboarding row present");
    assert_eq!(row.qualification, M5DocsQualificationClass::Beta);

    let ai = seeded_m5_docs_result_row_primitive_ai_citation_preview_narrowed();
    let row = ai
        .result_rows
        .iter()
        .find(|r| r.consumer_surface == M5DocsResultConsumerSurface::AiAnswerCitation)
        .expect("ai row present");
    assert_eq!(row.qualification, M5DocsQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let onboarding: M5DocsResultRowPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/docs/m5/m5-docs-result-row-and-source-version-badge-primitive/onboarding_reference_beta_narrowed.json"
    )))
    .expect("onboarding fixture parses");
    assert!(onboarding.validate().is_empty());
    assert_eq!(
        onboarding,
        seeded_m5_docs_result_row_primitive_onboarding_reference_beta_narrowed()
    );

    let ai: M5DocsResultRowPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/docs/m5/m5-docs-result-row-and-source-version-badge-primitive/ai_citation_preview_narrowed.json"
    )))
    .expect("ai fixture parses");
    assert!(ai.validate().is_empty());
    assert_eq!(
        ai,
        seeded_m5_docs_result_row_primitive_ai_citation_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_docs_result_row_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
