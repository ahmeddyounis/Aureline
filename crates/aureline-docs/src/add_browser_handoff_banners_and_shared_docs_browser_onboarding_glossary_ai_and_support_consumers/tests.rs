use super::*;

fn context_preserved_input(dest: &str) -> M5DocsHandoffBannerResolutionInput {
    M5DocsHandoffBannerResolutionInput {
        banner_title_repr: "Open upstream docs".to_owned(),
        handoff_reason: M5DocsHandoffReason::NoLocalCorpus,
        destination_repr: dest.to_owned(),
        corpus_class: M5DocsCorpusClass::ApiReference,
        source_provider: M5DocsSourceProvider::ThirdPartyHosted,
        version_scope: M5DocsVersionScope::ExactVersionMatch,
        freshness_state: M5DocsFreshnessState::RecentlySynced,
        pack_state: M5DocsPackState::UnpinnedTracking,
        privacy_exposure: M5DocsHandoffPrivacyExposure::DocumentContextLeaves,
        return_anchor_repr: "return:docs-browser/std".to_owned(),
        return_context_source_repr: "ctx-src:rust-std".to_owned(),
        return_context_version_repr: "ctx-ver:1.75".to_owned(),
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_no_local_corpus_reads_as_cannot_serve_in_product() {
    let resolved =
        resolve_docs_handoff_banner(&context_preserved_input("external:std")).expect("resolves");
    assert_eq!(
        resolved.necessity,
        M5DocsHandoffNecessity::CannotServeInProduct
    );
    // No in-product alternative → no stay-in-product action.
    assert!(!resolved
        .available_actions
        .contains(&M5DocsHandoffAction::StayInProduct));
    assert!(resolved
        .available_actions
        .contains(&M5DocsHandoffAction::OpenInBrowser));
    assert!(resolved
        .available_actions
        .contains(&M5DocsHandoffAction::CopyReturnAnchor));
    assert!(resolved
        .available_actions
        .contains(&M5DocsHandoffAction::ExportHandoffPacket));
    assert!(!resolved.disclosure_headline.trim().is_empty());
}

#[test]
fn resolver_preserves_source_version_context_on_return() {
    let resolved =
        resolve_docs_handoff_banner(&context_preserved_input("external:std")).expect("resolves");
    assert!(resolved.preserves_return_context);
    assert_eq!(
        resolved.return_path_posture,
        M5DocsHandoffReturnPathPosture::ContextPreservedReturn
    );
    assert!(resolved.return_path_posture.preserves_context());
}

#[test]
fn resolver_without_return_context_is_anchored_not_context_preserved() {
    let mut input = context_preserved_input("external:std");
    input.return_context_source_repr = String::new();
    input.return_context_version_repr = String::new();
    let resolved = resolve_docs_handoff_banner(&input).expect("resolves");
    assert!(!resolved.preserves_return_context);
    assert_eq!(
        resolved.return_path_posture,
        M5DocsHandoffReturnPathPosture::AnchoredReturn
    );
}

#[test]
fn resolver_user_requested_no_data_stays_fully_in_product() {
    let mut input = context_preserved_input("external:bundled-mirror");
    input.handoff_reason = M5DocsHandoffReason::UserRequestedBrowser;
    input.privacy_exposure = M5DocsHandoffPrivacyExposure::NoDataLeaves;
    let resolved = resolve_docs_handoff_banner(&input).expect("resolves");
    assert_eq!(
        resolved.necessity,
        M5DocsHandoffNecessity::UserRequestedExternal
    );
    assert_eq!(
        resolved.privacy_consequence,
        M5DocsHandoffPrivacyConsequence::StaysFullyInProduct
    );
    assert!(!resolved.privacy_leaves_boundary);
    // A user-requested handoff keeps a stay-in-product option.
    assert!(resolved
        .available_actions
        .contains(&M5DocsHandoffAction::StayInProduct));
}

#[test]
fn resolver_auth_gated_escalates_privacy_even_when_no_data_declared() {
    let mut input = context_preserved_input("external:vendor-portal");
    input.handoff_reason = M5DocsHandoffReason::AuthGatedSource;
    input.privacy_exposure = M5DocsHandoffPrivacyExposure::NoDataLeaves;
    let resolved = resolve_docs_handoff_banner(&input).expect("resolves");
    // Auth-gated is never understated as no-data-leaves.
    assert_eq!(
        resolved.privacy_consequence,
        M5DocsHandoffPrivacyConsequence::IdentifiedRequestShared
    );
    assert!(resolved.privacy_leaves_boundary);
    assert_eq!(
        resolved.necessity,
        M5DocsHandoffNecessity::ShouldDeferToCanonical
    );
}

#[test]
fn resolver_external_account_reads_as_account_and_identity_shared() {
    let mut input = context_preserved_input("external:api-console");
    input.handoff_reason = M5DocsHandoffReason::ExternalCanonicalSource;
    input.privacy_exposure = M5DocsHandoffPrivacyExposure::ExternalAccountRequired;
    let resolved = resolve_docs_handoff_banner(&input).expect("resolves");
    assert_eq!(
        resolved.privacy_consequence,
        M5DocsHandoffPrivacyConsequence::ExternalAccountAndIdentityShared
    );
    assert!(resolved.privacy_leaves_boundary);
}

#[test]
fn resolver_interactive_and_dynamic_reasons_cannot_serve_in_product() {
    for reason in [
        M5DocsHandoffReason::InteractiveContentRequired,
        M5DocsHandoffReason::DynamicRenderingRequired,
    ] {
        let mut input = context_preserved_input("external:playground");
        input.handoff_reason = reason;
        let resolved = resolve_docs_handoff_banner(&input).expect("resolves");
        assert_eq!(
            resolved.necessity,
            M5DocsHandoffNecessity::CannotServeInProduct
        );
    }
}

#[test]
fn resolver_rejects_empty_and_forbidden_material() {
    let mut empty_title = context_preserved_input("external:std");
    empty_title.banner_title_repr = "   ".to_owned();
    assert_eq!(
        resolve_docs_handoff_banner(&empty_title),
        Err(M5DocsHandoffResolutionError::EmptyBannerTitle)
    );

    let mut empty_dest = context_preserved_input("");
    empty_dest.destination_repr = String::new();
    assert_eq!(
        resolve_docs_handoff_banner(&empty_dest),
        Err(M5DocsHandoffResolutionError::EmptyDestination)
    );

    let mut empty_return = context_preserved_input("external:std");
    empty_return.return_anchor_repr = String::new();
    assert_eq!(
        resolve_docs_handoff_banner(&empty_return),
        Err(M5DocsHandoffResolutionError::MissingReturnPath)
    );

    let mut raw_url = context_preserved_input("https://example.com/docs");
    raw_url.destination_repr = "https://example.com/docs".to_owned();
    assert_eq!(
        resolve_docs_handoff_banner(&raw_url),
        Err(M5DocsHandoffResolutionError::ForbiddenHandoffMaterial)
    );
}

#[test]
fn resolver_is_deterministic() {
    let input = context_preserved_input("external:std");
    let a = resolve_docs_handoff_banner(&input).expect("resolves");
    let b = resolve_docs_handoff_banner(&input).expect("resolves");
    assert_eq!(a, b);
}

// ---- vocabulary ---------------------------------------------------------

#[test]
fn shared_component_canonical_refs_are_non_empty() {
    for component in M5DocsSharedComponent::ALL {
        assert!(!component.canonical_schema_ref().trim().is_empty());
        assert!(!component.canonical_doc_ref().trim().is_empty());
    }
}

#[test]
fn vocabulary_set_matches_canonical() {
    assert!(M5DocsHandoffVocabularySet::canonical().matches_canonical());
}

#[test]
fn mandatory_anatomy_and_export_are_subsets_of_all() {
    for part in M5DocsHandoffBannerAnatomyPart::MANDATORY {
        assert!(M5DocsHandoffBannerAnatomyPart::ALL.contains(&part));
    }
    for field in M5DocsHandoffExportField::MANDATORY {
        assert!(M5DocsHandoffExportField::ALL.contains(&field));
    }
}

// ---- packet -------------------------------------------------------------

#[test]
fn seed_packet_validates_clean() {
    let packet = seeded_m5_docs_handoff_consumer_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn narrowed_variants_validate_clean() {
    for packet in [
        seeded_m5_docs_handoff_consumer_onboarding_tour_beta_narrowed(),
        seeded_m5_docs_handoff_consumer_ai_evidence_preview_narrowed(),
    ] {
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}

#[test]
fn seed_packet_covers_every_consumer() {
    let packet = seeded_m5_docs_handoff_consumer_packet();
    let present: BTreeSet<M5DocsHandoffConsumerSurface> = packet
        .consumer_rows
        .iter()
        .map(|row| row.consumer_surface)
        .collect();
    for consumer in M5DocsHandoffConsumerSurface::ALL {
        assert!(present.contains(&consumer), "missing {consumer:?}");
    }
}

#[test]
fn every_shared_component_reused_by_at_least_two_consumers() {
    let packet = seeded_m5_docs_handoff_consumer_packet();
    for component in M5DocsSharedComponent::ALL {
        let count = packet
            .consumer_rows
            .iter()
            .filter(|row| row.reused_components.contains(&component))
            .count();
        assert!(count >= 2, "{component:?} reused by only {count} consumers");
    }
}

#[test]
fn seed_worked_cases_are_self_consistent() {
    let packet = seeded_m5_docs_handoff_consumer_packet();
    for row in &packet.consumer_rows {
        for case in &row.handoff_examples {
            assert!(case.is_self_consistent(), "{:?}", case.input);
        }
    }
}

#[test]
fn seed_proves_stays_in_product_and_leaves_boundary_contrast() {
    let packet = seeded_m5_docs_handoff_consumer_packet();
    let cases: Vec<_> = packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.handoff_examples.iter())
        .collect();
    assert!(cases.iter().any(|c| !c.resolved.privacy_leaves_boundary));
    assert!(cases.iter().any(|c| c.resolved.privacy_leaves_boundary));
}

#[test]
fn seed_proves_context_preserved_return() {
    let packet = seeded_m5_docs_handoff_consumer_packet();
    assert!(packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.handoff_examples.iter())
        .any(|c| c.resolved.return_path_posture.preserves_context()));
}

#[test]
fn detects_missing_source_contracts() {
    let mut packet = seeded_m5_docs_handoff_consumer_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsHandoffConsumerViolation::MissingSourceContracts));
}

#[test]
fn detects_vocabulary_drift() {
    let mut packet = seeded_m5_docs_handoff_consumer_packet();
    packet.vocabulary_set.necessities.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsHandoffConsumerViolation::VocabularySetDrift));
}

#[test]
fn detects_missing_consumer() {
    let mut packet = seeded_m5_docs_handoff_consumer_packet();
    packet
        .consumer_rows
        .retain(|row| row.consumer_surface != M5DocsHandoffConsumerSurface::SupportHelp);
    assert!(packet
        .validate()
        .contains(&M5DocsHandoffConsumerViolation::RequiredConsumerMissing));
}

#[test]
fn detects_component_reuse_gap() {
    let mut packet = seeded_m5_docs_handoff_consumer_packet();
    // Strip the reference card from all but one consumer.
    let mut seen = false;
    for row in &mut packet.consumer_rows {
        if row
            .reused_components
            .contains(&M5DocsSharedComponent::ReferenceCard)
        {
            if seen {
                row.reused_components
                    .retain(|c| *c != M5DocsSharedComponent::ReferenceCard);
            } else {
                seen = true;
            }
        }
    }
    assert!(packet
        .validate()
        .contains(&M5DocsHandoffConsumerViolation::ComponentReuseUnproven));
}

#[test]
fn detects_mandatory_anatomy_missing() {
    let mut packet = seeded_m5_docs_handoff_consumer_packet();
    packet.consumer_rows[0].banner_anatomy_parts =
        vec![M5DocsHandoffBannerAnatomyPart::BannerTitleLabel];
    assert!(packet
        .validate()
        .contains(&M5DocsHandoffConsumerViolation::MandatoryAnatomyMissing));
}

#[test]
fn detects_mandatory_export_field_missing() {
    let mut packet = seeded_m5_docs_handoff_consumer_packet();
    packet.consumer_rows[0].export_fields = vec![M5DocsHandoffExportField::Destination];
    assert!(packet
        .validate()
        .contains(&M5DocsHandoffConsumerViolation::MandatoryExportFieldMissing));
}

#[test]
fn detects_missing_keyboard_route() {
    let mut packet = seeded_m5_docs_handoff_consumer_packet();
    packet.consumer_rows[0]
        .accessibility_routes
        .retain(|route| *route != M5DocsAccessibilityRoute::KeyboardFocusable);
    assert!(packet
        .validate()
        .contains(&M5DocsHandoffConsumerViolation::AccessibilityRouteMissing));
}

#[test]
fn detects_resolution_drift() {
    let mut packet = seeded_m5_docs_handoff_consumer_packet();
    packet.consumer_rows[0].handoff_examples[0]
        .resolved
        .privacy_consequence = M5DocsHandoffPrivacyConsequence::StaysFullyInProduct;
    assert!(packet
        .validate()
        .contains(&M5DocsHandoffConsumerViolation::HandoffResolutionDrift));
}

#[test]
fn detects_row_invariant_violation() {
    let mut packet = seeded_m5_docs_handoff_consumer_packet();
    packet.consumer_rows[0].strips_source_version_context = true;
    assert!(packet
        .validate()
        .contains(&M5DocsHandoffConsumerViolation::RowInvariantViolated));
}

#[test]
fn detects_stable_consumer_missing_proof() {
    let mut packet = seeded_m5_docs_handoff_consumer_packet();
    packet.consumer_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsHandoffConsumerViolation::StableConsumerMissingProof));
}

#[test]
fn detects_governance_review_incomplete() {
    let mut packet = seeded_m5_docs_handoff_consumer_packet();
    packet
        .governance_review
        .privacy_consequence_never_understated = false;
    assert!(packet
        .validate()
        .contains(&M5DocsHandoffConsumerViolation::GovernanceReviewIncomplete));
}

#[test]
fn detects_consumer_projection_incomplete() {
    let mut packet = seeded_m5_docs_handoff_consumer_packet();
    packet.consumer_projection.return_path_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5DocsHandoffConsumerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn detects_release_posture_incomplete() {
    let mut packet = seeded_m5_docs_handoff_consumer_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5DocsHandoffConsumerViolation::ReleasePostureIncomplete));
}

#[test]
fn export_json_and_csv_and_report_are_non_empty() {
    let packet = seeded_m5_docs_handoff_consumer_packet();
    assert!(packet.export_safe_json().contains("record_kind"));
    assert!(packet.render_matrix_csv().contains("consumer_surface"));
    assert!(packet.render_markdown_summary().contains("Handoff Banner"));
}

#[test]
fn checked_in_export_matches_seed() {
    let current = current_stable_m5_docs_handoff_consumer_export()
        .expect("checked-in export parses and validates");
    assert_eq!(current, seeded_m5_docs_handoff_consumer_packet());
}
