use super::*;

fn live_ready_input(scope: &str) -> M5DocsSearchResolutionInput {
    M5DocsSearchResolutionInput {
        search_bar_label: "Docs search".to_owned(),
        scope_target_repr: scope.to_owned(),
        corpus_classes: vec![M5DocsCorpusClass::FirstPartyDocs],
        source_provider: M5DocsSourceProvider::FirstPartyHosted,
        provider_availability: M5DocsProviderAvailability::ProviderAvailable,
        retrieval_mode: M5DocsRetrievalMode::LiveRetrieval,
        version_scope: M5DocsVersionScope::ExactVersionMatch,
        keyboard_hint_repr: "cmd+k".to_owned(),
        freshness_state: M5DocsFreshnessState::LiveCurrent,
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_live_ready_has_no_banner() {
    let resolved = resolve_docs_search(&live_ready_input("aureline-docs@1.4.0")).expect("resolves");
    assert_eq!(
        resolved.search_availability,
        M5DocsSearchAvailability::SearchLiveReady
    );
    assert!(resolved.is_ready);
    assert!(!resolved.is_narrowed);
    assert!(!resolved.is_degraded);
    assert!(!resolved.is_blocked);
    assert!(resolved.degraded_banner.is_none());
    assert_eq!(resolved.corpus_count, 1);
}

#[test]
fn resolver_cached_and_mirrored_are_ready_but_not_shown_as_live() {
    let cached = resolve_docs_search(&M5DocsSearchResolutionInput {
        retrieval_mode: M5DocsRetrievalMode::CachedRetrieval,
        freshness_state: M5DocsFreshnessState::CachedOffline,
        ..live_ready_input("guides@latest-stable")
    })
    .expect("resolves");
    assert_eq!(
        cached.search_availability,
        M5DocsSearchAvailability::SearchCachedReady
    );
    assert!(cached.is_ready);
    // Cached retrieval is carried explicitly, never as live.
    assert_eq!(cached.retrieval_mode, M5DocsRetrievalMode::CachedRetrieval);
    assert!(cached.degraded_banner.is_none());

    let mirrored = resolve_docs_search(&M5DocsSearchResolutionInput {
        provider_availability: M5DocsProviderAvailability::ProviderMirrorOnly,
        retrieval_mode: M5DocsRetrievalMode::MirroredRetrieval,
        freshness_state: M5DocsFreshnessState::RecentlySynced,
        ..live_ready_input("onboarding-pack@pinned-2.1")
    })
    .expect("resolves");
    assert_eq!(
        mirrored.search_availability,
        M5DocsSearchAvailability::SearchMirroredReady
    );
    assert!(mirrored.is_ready);
}

#[test]
fn resolver_policy_and_degraded_narrow_with_self_contained_banner() {
    let policy = resolve_docs_search(&M5DocsSearchResolutionInput {
        provider_availability: M5DocsProviderAvailability::ProviderPolicyLimited,
        ..live_ready_input("vendor-docs@unversioned")
    })
    .expect("resolves");
    assert_eq!(
        policy.search_availability,
        M5DocsSearchAvailability::NarrowedPolicyLimited
    );
    assert!(policy.is_narrowed);
    let banner = policy.degraded_banner.expect("banner present");
    assert_eq!(banner.reason, M5DocsSearchLimitReason::PolicyLimitedScope);
    assert_eq!(
        banner.next_action,
        M5DocsSearchNextAction::RequestPolicyAccess
    );
    assert!(!banner.limited_corpus_classes.is_empty());
    assert!(!banner.headline.trim().is_empty());
    // The banner explains rather than presenting empty results.
    assert!(banner.headline.to_lowercase().contains("policy"));

    let degraded = resolve_docs_search(&M5DocsSearchResolutionInput {
        provider_availability: M5DocsProviderAvailability::ProviderDegraded,
        retrieval_mode: M5DocsRetrievalMode::OfflineBundledRetrieval,
        ..live_ready_input("project-guides@this-project")
    })
    .expect("resolves");
    assert_eq!(
        degraded.search_availability,
        M5DocsSearchAvailability::NarrowedProviderDegraded
    );
    assert_eq!(
        degraded.degraded_banner.unwrap().next_action,
        M5DocsSearchNextAction::UseCachedCorpus
    );
}

#[test]
fn resolver_offline_states_degrade_with_distinct_reasons() {
    let unavailable = resolve_docs_search(&M5DocsSearchResolutionInput {
        provider_availability: M5DocsProviderAvailability::ProviderUnavailable,
        retrieval_mode: M5DocsRetrievalMode::CachedRetrieval,
        freshness_state: M5DocsFreshnessState::StaleExpired,
        ..live_ready_input("community-docs@nearby-3.0")
    })
    .expect("resolves");
    assert_eq!(
        unavailable.search_availability,
        M5DocsSearchAvailability::DegradedProviderUnavailable
    );
    assert!(unavailable.is_degraded);
    assert_eq!(
        unavailable.degraded_banner.unwrap().reason,
        M5DocsSearchLimitReason::ProviderUnavailableOffline
    );

    let no_corpus = resolve_docs_search(&M5DocsSearchResolutionInput {
        retrieval_mode: M5DocsRetrievalMode::NoCorpusAvailable,
        freshness_state: M5DocsFreshnessState::UnknownFreshness,
        ..live_ready_input("release-notes@unversioned")
    })
    .expect("resolves");
    assert_eq!(
        no_corpus.search_availability,
        M5DocsSearchAvailability::DegradedOfflineNoCorpus
    );
    assert_eq!(
        no_corpus.degraded_banner.unwrap().next_action,
        M5DocsSearchNextAction::ImportOrHandOffToBrowser
    );
}

#[test]
fn resolver_unknown_state_blocks_first() {
    let unknown_provider = resolve_docs_search(&M5DocsSearchResolutionInput {
        provider_availability: M5DocsProviderAvailability::ProviderAvailabilityUnknown,
        freshness_state: M5DocsFreshnessState::UnknownFreshness,
        ..live_ready_input("help-center@unversioned")
    })
    .expect("resolves");
    assert_eq!(
        unknown_provider.search_availability,
        M5DocsSearchAvailability::BlockedUnknownState
    );
    assert!(unknown_provider.is_blocked);
    assert_eq!(
        unknown_provider.degraded_banner.unwrap().reason,
        M5DocsSearchLimitReason::SearchStateUnknown
    );

    let unknown_retrieval = resolve_docs_search(&M5DocsSearchResolutionInput {
        retrieval_mode: M5DocsRetrievalMode::RetrievalModeUnknown,
        freshness_state: M5DocsFreshnessState::UnknownFreshness,
        ..live_ready_input("codebase-symbols@this-project")
    })
    .expect("resolves");
    assert_eq!(
        unknown_retrieval.search_availability,
        M5DocsSearchAvailability::BlockedUnknownState
    );
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty_label = M5DocsSearchResolutionInput {
        search_bar_label: "  ".to_owned(),
        ..live_ready_input("aureline-docs@1.4.0")
    };
    assert_eq!(
        resolve_docs_search(&empty_label),
        Err(M5DocsSearchResolutionError::EmptySearchBarLabel)
    );

    let empty_scope = M5DocsSearchResolutionInput {
        scope_target_repr: "".to_owned(),
        ..live_ready_input("aureline-docs@1.4.0")
    };
    assert_eq!(
        resolve_docs_search(&empty_scope),
        Err(M5DocsSearchResolutionError::EmptyScopeTarget)
    );

    let empty_corpus = M5DocsSearchResolutionInput {
        corpus_classes: vec![],
        ..live_ready_input("aureline-docs@1.4.0")
    };
    assert_eq!(
        resolve_docs_search(&empty_corpus),
        Err(M5DocsSearchResolutionError::EmptyCorpusSet)
    );

    let empty_keyboard = M5DocsSearchResolutionInput {
        keyboard_hint_repr: "   ".to_owned(),
        ..live_ready_input("aureline-docs@1.4.0")
    };
    assert_eq!(
        resolve_docs_search(&empty_keyboard),
        Err(M5DocsSearchResolutionError::EmptyKeyboardHint)
    );

    let forbidden = M5DocsSearchResolutionInput {
        scope_target_repr: "https://example.test/docs".to_owned(),
        ..live_ready_input("aureline-docs@1.4.0")
    };
    assert_eq!(
        resolve_docs_search(&forbidden),
        Err(M5DocsSearchResolutionError::ForbiddenSearchMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_docs_search_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_DOCS_SEARCH_PRIMITIVE_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_docs_search_primitive_packet();
    let present: std::collections::BTreeSet<_> = packet
        .search_rows
        .iter()
        .map(|r| r.consumer_surface)
        .collect();
    for surface in M5DocsSearchConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.search_rows.len(),
        M5DocsSearchConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_docs_search_primitive_packet();
    for row in &packet.search_rows {
        for part in M5DocsSearchBarAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5DocsSearchBarExportField::MANDATORY {
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
    let packet = seeded_m5_docs_search_primitive_packet();
    let cases: Vec<&M5DocsSearchResolutionCase> = packet
        .search_rows
        .iter()
        .flat_map(|row| row.example_resolutions.iter())
        .collect();

    for posture in M5DocsSearchAvailability::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.search_availability == posture),
            "no worked resolution exercises availability {}",
            posture.as_str()
        );
    }
    for provider in M5DocsProviderAvailability::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.provider_availability == provider),
            "no worked resolution exercises provider availability {}",
            provider.as_str()
        );
    }
    for retrieval in M5DocsRetrievalMode::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.retrieval_mode == retrieval),
            "no worked resolution exercises retrieval mode {}",
            retrieval.as_str()
        );
    }
    for freshness in M5DocsFreshnessState::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.freshness_state == freshness),
            "no worked resolution exercises freshness {}",
            freshness.as_str()
        );
    }
    for scope in M5DocsVersionScope::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.version_scope == scope),
            "no worked resolution exercises version scope {}",
            scope.as_str()
        );
    }
    for reason in M5DocsSearchLimitReason::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .degraded_banner
                .as_ref()
                .is_some_and(|b| b.reason == reason)),
            "no worked resolution exercises limit reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_docs_search_primitive_packet();
    for row in &packet.search_rows {
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
    let mut packet = seeded_m5_docs_search_primitive_packet();
    packet
        .search_rows
        .retain(|row| row.consumer_surface != M5DocsSearchConsumerSurface::CliDocsSearch);
    assert!(packet
        .validate()
        .contains(&M5DocsSearchPrimitiveViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_docs_search_primitive_packet();
    packet.vocabulary_set.search_availabilities.pop();
    assert!(packet
        .validate()
        .contains(&M5DocsSearchPrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_docs_search_primitive_packet();
    packet.search_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5DocsSearchBarAnatomyPart::ScopeTargetSwitcher);
    assert!(packet
        .validate()
        .contains(&M5DocsSearchPrimitiveViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_docs_search_primitive_packet();
    packet.search_rows[0]
        .export_fields
        .retain(|f| *f != M5DocsSearchBarExportField::ProviderAvailability);
    assert!(packet
        .validate()
        .contains(&M5DocsSearchPrimitiveViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_docs_search_primitive_packet();
    packet.search_rows[0].example_resolutions[0]
        .resolved
        .is_ready = false;
    assert!(packet
        .validate()
        .contains(&M5DocsSearchPrimitiveViolation::ExampleResolutionDrift));
}

#[test]
fn example_resolution_missing_fails() {
    let mut packet = seeded_m5_docs_search_primitive_packet();
    packet.search_rows[2].example_resolutions.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsSearchPrimitiveViolation::ExampleResolutionMissing));
}

#[test]
fn availability_coverage_unproven_fails_when_no_not_ready_example_present() {
    let mut packet = seeded_m5_docs_search_primitive_packet();
    // Replace every example with a clean live-ready one so the coverage lint fires.
    for row in &mut packet.search_rows {
        row.example_resolutions = vec![M5DocsSearchResolutionCase::resolved(live_ready_input(
            "aureline-docs@9.9.9",
        ))];
    }
    assert!(packet
        .validate()
        .contains(&M5DocsSearchPrimitiveViolation::AvailabilityCoverageUnproven));
}

#[test]
fn search_invariant_violation_fails() {
    let mut packet = seeded_m5_docs_search_primitive_packet();
    packet.search_rows[0].shows_cached_or_mirrored_as_live = true;
    assert!(packet
        .validate()
        .contains(&M5DocsSearchPrimitiveViolation::SearchInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_docs_search_primitive_packet();
    packet.search_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsSearchPrimitiveViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_docs_search_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsSearchPrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_docs_search_primitive_packet();
    packet
        .governance_review
        .cached_or_mirrored_never_shown_as_live = false;
    assert!(packet
        .validate()
        .contains(&M5DocsSearchPrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_docs_search_primitive_packet();
    packet
        .consumer_projection
        .retrieval_mode_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5DocsSearchPrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_docs_search_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5DocsSearchPrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_docs_search_primitive_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5DocsSearchPrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_docs_search_primitive_packet().render_markdown_summary();
    for surface in M5DocsSearchConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_docs_search_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5DocsSearchConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5DocsSearchConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_docs_search_primitive_export()
        .expect("checked M5 docs-search primitive export validates");
    assert_eq!(from_disk.packet_id, M5_DOCS_SEARCH_PRIMITIVE_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_docs_search_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_docs_search_primitive_onboarding_lookup_beta_narrowed(),
        seeded_m5_docs_search_primitive_ai_citation_follow_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.search_rows.len(),
            M5DocsSearchConsumerSurface::ALL.len()
        );
    }

    let onboarding = seeded_m5_docs_search_primitive_onboarding_lookup_beta_narrowed();
    let row = onboarding
        .search_rows
        .iter()
        .find(|r| r.consumer_surface == M5DocsSearchConsumerSurface::OnboardingTutorialLookup)
        .expect("onboarding row present");
    assert_eq!(row.qualification, M5DocsQualificationClass::Beta);

    let ai = seeded_m5_docs_search_primitive_ai_citation_follow_preview_narrowed();
    let row = ai
        .search_rows
        .iter()
        .find(|r| r.consumer_surface == M5DocsSearchConsumerSurface::AiCitationFollow)
        .expect("ai row present");
    assert_eq!(row.qualification, M5DocsQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let onboarding: M5DocsSearchPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/docs/m5/m5-docs-search-bar-and-scope-switcher-primitive/onboarding_lookup_beta_narrowed.json"
    )))
    .expect("onboarding fixture parses");
    assert!(onboarding.validate().is_empty());
    assert_eq!(
        onboarding,
        seeded_m5_docs_search_primitive_onboarding_lookup_beta_narrowed()
    );

    let ai: M5DocsSearchPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/docs/m5/m5-docs-search-bar-and-scope-switcher-primitive/ai_citation_follow_preview_narrowed.json"
    )))
    .expect("ai fixture parses");
    assert!(ai.validate().is_empty());
    assert_eq!(
        ai,
        seeded_m5_docs_search_primitive_ai_citation_follow_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_docs_search_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
