use super::*;

#[test]
fn seeded_catalog_validates() {
    let catalog = seeded_ai_copy_guardrail_catalog();
    assert!(catalog.validate().is_empty(), "{:?}", catalog.validate());
    assert_eq!(catalog.catalog_id, AI_COPY_GUARDRAIL_CATALOG_ID);
}

#[test]
fn inventories_match_canonical() {
    let catalog = seeded_ai_copy_guardrail_catalog();
    assert_eq!(catalog.domain_inventory.len(), AiCopyDomain::ALL.len());
    assert_eq!(
        catalog.concept_inventory.len(),
        AiTaxonomyConcept::ALL.len()
    );
    assert_eq!(catalog.surface_inventory.len(), AiCopySurface::ALL.len());
    assert_eq!(catalog.consumer_inventory.len(), AiCopyConsumer::ALL.len());
    assert_eq!(
        catalog.forbidden_class_inventory.len(),
        ForbiddenPhraseClass::ALL.len()
    );
}

#[test]
fn inventory_drift_fails() {
    let mut catalog = seeded_ai_copy_guardrail_catalog();
    catalog.concept_inventory.pop();
    assert!(catalog
        .validate()
        .contains(&AiCopyGuardrailViolation::InventoryDrift));
}

#[test]
fn every_taxonomy_concept_has_a_term() {
    let catalog = seeded_ai_copy_guardrail_catalog();
    let concepts: std::collections::BTreeSet<_> = catalog.terms.iter().map(|t| t.concept).collect();
    for concept in AiTaxonomyConcept::ALL {
        assert!(concepts.contains(&concept), "missing term for {concept:?}");
    }
}

#[test]
fn taxonomy_concept_not_covered_fails() {
    let mut catalog = seeded_ai_copy_guardrail_catalog();
    catalog
        .terms
        .retain(|t| t.concept != AiTaxonomyConcept::RevertUndoAvailable);
    assert!(catalog
        .validate()
        .contains(&AiCopyGuardrailViolation::TaxonomyConceptNotCovered));
}

#[test]
fn term_ids_and_tokens_are_locale_neutral() {
    let catalog = seeded_ai_copy_guardrail_catalog();
    let ok = |t: &str| {
        !t.is_empty()
            && t.chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '.')
    };
    for term in &catalog.terms {
        assert!(
            ok(&term.term_id),
            "term id not locale-neutral: {}",
            term.term_id
        );
        assert!(
            ok(&term.machine_token),
            "machine token not locale-neutral: {}",
            term.machine_token
        );
    }
    for phrase in &catalog.forbidden_phrases {
        assert!(
            ok(&phrase.phrase_id),
            "phrase id not locale-neutral: {}",
            phrase.phrase_id
        );
    }
}

#[test]
fn non_locale_neutral_term_id_fails() {
    let mut catalog = seeded_ai_copy_guardrail_catalog();
    catalog.terms[0].term_id = "Term.Proposal.Suggested".to_owned();
    assert!(catalog
        .validate()
        .contains(&AiCopyGuardrailViolation::TermTokenNotLocaleNeutral));
}

#[test]
fn duplicate_term_fails() {
    let mut catalog = seeded_ai_copy_guardrail_catalog();
    let clone = catalog.terms[0].clone();
    catalog.terms.push(clone);
    assert!(catalog
        .validate()
        .contains(&AiCopyGuardrailViolation::DuplicateTerm));
}

#[test]
fn term_must_be_provisional() {
    let mut catalog = seeded_ai_copy_guardrail_catalog();
    catalog.terms[0].ai_provisional = false;
    assert!(catalog
        .validate()
        .contains(&AiCopyGuardrailViolation::TermNotProvisional));
}

#[test]
fn low_confidence_must_suppress_direct_mutation() {
    let mut catalog = seeded_ai_copy_guardrail_catalog();
    let term = catalog
        .terms
        .iter_mut()
        .find(|t| t.concept == AiTaxonomyConcept::LowConfidence)
        .expect("low confidence term present");
    term.suppresses_direct_mutation = false;
    assert!(catalog
        .validate()
        .contains(&AiCopyGuardrailViolation::MutationSuppressionMissing));
}

#[test]
fn term_domain_must_match_concept() {
    let mut catalog = seeded_ai_copy_guardrail_catalog();
    catalog.terms[0].domain = AiCopyDomain::Confidence;
    assert!(catalog
        .validate()
        .contains(&AiCopyGuardrailViolation::TermDomainMismatch));
}

#[test]
fn term_incomplete_fails() {
    let mut catalog = seeded_ai_copy_guardrail_catalog();
    catalog.terms[0].required_context.clear();
    assert!(catalog
        .validate()
        .contains(&AiCopyGuardrailViolation::TermIncomplete));
}

#[test]
fn term_copy_that_overclaims_fails() {
    let mut catalog = seeded_ai_copy_guardrail_catalog();
    // Inject a forbidden high-trust phrase into an approved term's own copy.
    catalog.terms[0].reserved_meaning =
        "This suggestion is guaranteed to work with no review needed.".to_owned();
    assert!(catalog
        .validate()
        .contains(&AiCopyGuardrailViolation::TermCopyOverclaims));
}

#[test]
fn forbidden_phrase_incomplete_fails() {
    let mut catalog = seeded_ai_copy_guardrail_catalog();
    catalog.forbidden_phrases[0]
        .approved_replacement_term_ids
        .clear();
    assert!(catalog
        .validate()
        .contains(&AiCopyGuardrailViolation::ForbiddenPhraseIncomplete));
}

#[test]
fn forbidden_pattern_must_be_lowercase() {
    let mut catalog = seeded_ai_copy_guardrail_catalog();
    catalog.forbidden_phrases[0].pattern = "Guaranteed".to_owned();
    assert!(catalog
        .validate()
        .contains(&AiCopyGuardrailViolation::ForbiddenPatternNotLowercase));
}

#[test]
fn duplicate_forbidden_phrase_fails() {
    let mut catalog = seeded_ai_copy_guardrail_catalog();
    let clone = catalog.forbidden_phrases[0].clone();
    catalog.forbidden_phrases.push(clone);
    assert!(catalog
        .validate()
        .contains(&AiCopyGuardrailViolation::DuplicateForbiddenPhrase));
}

#[test]
fn forbidden_replacement_must_resolve() {
    let mut catalog = seeded_ai_copy_guardrail_catalog();
    catalog.forbidden_phrases[0].approved_replacement_term_ids =
        vec!["term.does.not.exist".to_owned()];
    assert!(catalog
        .validate()
        .contains(&AiCopyGuardrailViolation::ForbiddenReplacementUnresolved));
}

#[test]
fn required_high_trust_phrases_are_present() {
    let catalog = seeded_ai_copy_guardrail_catalog();
    let patterns: std::collections::BTreeSet<&str> = catalog
        .forbidden_phrases
        .iter()
        .map(|p| p.pattern.as_str())
        .collect();
    for required in REQUIRED_FORBIDDEN_PATTERNS {
        assert!(
            patterns.contains(required),
            "missing required phrase: {required}"
        );
    }
}

#[test]
fn required_high_trust_phrase_missing_fails() {
    let mut catalog = seeded_ai_copy_guardrail_catalog();
    catalog
        .forbidden_phrases
        .retain(|p| p.pattern != "guaranteed");
    assert!(catalog
        .validate()
        .contains(&AiCopyGuardrailViolation::RequiredHighTrustPhraseMissing));
}

#[test]
fn lint_rejects_forbidden_high_trust_phrases() {
    let catalog = seeded_ai_copy_guardrail_catalog();
    for (candidate, expected) in [
        (
            "This fix is guaranteed to work.",
            "forbidden.perfection.guaranteed",
        ),
        (
            "A perfect, complete answer.",
            "forbidden.perfection.perfect",
        ),
        (
            "Apply it, no review needed.",
            "forbidden.review_free.no_review_needed",
        ),
        (
            "We did it, all done for you.",
            "forbidden.autonomy.done_for_you",
        ),
    ] {
        let findings = catalog.lint(candidate, AiCopySurface::PatchReview);
        assert!(
            findings.iter().any(|f| f.phrase_id == expected),
            "expected {expected} for {candidate:?}, got {findings:?}"
        );
        // Every finding offers an approved replacement.
        for finding in &findings {
            assert!(!finding.approved_replacement_term_ids.is_empty());
            for term_id in &finding.approved_replacement_term_ids {
                assert!(
                    catalog.term(term_id).is_some(),
                    "unknown replacement {term_id}"
                );
            }
        }
    }
}

#[test]
fn lint_respects_forbidden_on_surface() {
    let mut catalog = seeded_ai_copy_guardrail_catalog();
    // Narrow one phrase to a single surface, then lint a different surface.
    let phrase = catalog
        .forbidden_phrases
        .iter_mut()
        .find(|p| p.pattern == "guaranteed")
        .expect("guaranteed phrase present");
    phrase.forbidden_on = vec![AiCopySurface::PromptComposer];
    let findings = catalog.lint("guaranteed result", AiCopySurface::DocsHelp);
    assert!(
        !findings.iter().any(|f| f.matched_pattern == "guaranteed"),
        "phrase leaked onto a surface it was not forbidden on: {findings:?}"
    );
    // It still matches on the surface it is forbidden on.
    let on_surface = catalog.lint("guaranteed result", AiCopySurface::PromptComposer);
    assert!(on_surface.iter().any(|f| f.matched_pattern == "guaranteed"));
}

#[test]
fn approved_copy_is_clean_on_its_surfaces() {
    let catalog = seeded_ai_copy_guardrail_catalog();
    for term in &catalog.terms {
        let copy = format!(
            "{} {} {}",
            term.canonical_label,
            term.reserved_meaning,
            term.required_context.join(" ")
        );
        for surface in &term.surfaces {
            assert!(
                catalog.is_clean(&copy, *surface),
                "approved term {} overclaims on {:?}: {:?}",
                term.term_id,
                surface,
                catalog.lint(&copy, *surface)
            );
        }
    }
}

#[test]
fn render_term_reference_reconstructs_in_product_wording() {
    let catalog = seeded_ai_copy_guardrail_catalog();
    let rendered = catalog
        .render_term_reference("term.confidence.low_confidence")
        .expect("term resolves");
    assert!(rendered.starts_with("Low confidence"));
    assert!(rendered.contains("confidence floor"));
    assert!(rendered.contains("Requires:"));
    // Low confidence reconstructs the suppression rule too.
    assert!(rendered.contains("Direct mutation controls are suppressed."));
}

#[test]
fn coverage_gap_fails_when_a_forbidden_class_is_missing() {
    let mut catalog = seeded_ai_copy_guardrail_catalog();
    catalog
        .forbidden_phrases
        .retain(|p| p.class != ForbiddenPhraseClass::FalseFreshness);
    assert!(catalog
        .validate()
        .contains(&AiCopyGuardrailViolation::CoverageGap));
}

#[test]
fn coverage_spans_every_domain_surface_consumer_and_class() {
    let catalog = seeded_ai_copy_guardrail_catalog();
    let domains: std::collections::BTreeSet<_> = catalog.terms.iter().map(|t| t.domain).collect();
    let surfaces: std::collections::BTreeSet<_> = catalog
        .terms
        .iter()
        .flat_map(|t| t.surfaces.iter().copied())
        .collect();
    let consumers: std::collections::BTreeSet<_> = catalog
        .terms
        .iter()
        .flat_map(|t| t.consumers.iter().copied())
        .collect();
    let classes: std::collections::BTreeSet<_> =
        catalog.forbidden_phrases.iter().map(|p| p.class).collect();
    assert_eq!(domains.len(), AiCopyDomain::ALL.len());
    assert_eq!(surfaces.len(), AiCopySurface::ALL.len());
    assert_eq!(consumers.len(), AiCopyConsumer::ALL.len());
    assert_eq!(classes.len(), ForbiddenPhraseClass::ALL.len());
}

#[test]
fn shared_terms_reuse_across_consumers() {
    let catalog = seeded_ai_copy_guardrail_catalog();
    let reuse = catalog.cross_consumer_reuse();
    for term_id in &catalog.shared_reuse_term_ids {
        let spans = reuse.get(term_id).map(|s| s.len()).unwrap_or(0);
        assert!(
            spans >= SHARED_TERM_MIN_REUSE_CONSUMERS,
            "shared term {term_id} only spans {spans} consumers"
        );
    }
}

#[test]
fn empty_shared_reuse_fails() {
    let mut catalog = seeded_ai_copy_guardrail_catalog();
    catalog.shared_reuse_term_ids.clear();
    assert!(catalog
        .validate()
        .contains(&AiCopyGuardrailViolation::SharedTermReuseInsufficient));
}

#[test]
fn missing_source_contracts_fails() {
    let mut catalog = seeded_ai_copy_guardrail_catalog();
    catalog.source_contract_refs.clear();
    assert!(catalog
        .validate()
        .contains(&AiCopyGuardrailViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut catalog = seeded_ai_copy_guardrail_catalog();
    catalog
        .trust_review
        .ai_wording_never_implies_review_free_completion = false;
    assert!(catalog
        .validate()
        .contains(&AiCopyGuardrailViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut catalog = seeded_ai_copy_guardrail_catalog();
    catalog.consumer_projection.patch_review_honors_guardrails = false;
    assert!(catalog
        .validate()
        .contains(&AiCopyGuardrailViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut catalog = seeded_ai_copy_guardrail_catalog();
    catalog.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(catalog
        .validate()
        .contains(&AiCopyGuardrailViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut catalog = seeded_ai_copy_guardrail_catalog();
    catalog.release_posture.mirror_offline_parity_required = false;
    assert!(catalog
        .validate()
        .contains(&AiCopyGuardrailViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_terms_and_phrases() {
    let catalog = seeded_ai_copy_guardrail_catalog();
    let summary = catalog.render_markdown_summary();
    for term in &catalog.terms {
        assert!(
            summary.contains(&term.term_id),
            "summary missing {}",
            term.term_id
        );
    }
    for phrase in &catalog.forbidden_phrases {
        assert!(
            summary.contains(&phrase.phrase_id),
            "summary missing {}",
            phrase.phrase_id
        );
    }
}

#[test]
fn localized_overlay_preserves_machine_identity() {
    let canonical = seeded_ai_copy_guardrail_catalog();
    let localized = seeded_ai_copy_guardrail_catalog_localized();
    assert!(
        localized.validate().is_empty(),
        "localized overlay failed validation: {:?}",
        localized.validate()
    );
    assert_ne!(canonical.reference_locale, localized.reference_locale);

    // Term ids, tokens, surfaces, and consumers are byte-for-byte identical.
    let canon_term_ids: Vec<&str> = canonical.terms.iter().map(|t| t.term_id.as_str()).collect();
    let loc_term_ids: Vec<&str> = localized.terms.iter().map(|t| t.term_id.as_str()).collect();
    assert_eq!(canon_term_ids, loc_term_ids);

    let mut any_prose_changed = false;
    for (canon, loc) in canonical.terms.iter().zip(localized.terms.iter()) {
        assert_eq!(canon.machine_token, loc.machine_token);
        assert_eq!(canon.concept, loc.concept);
        assert_eq!(canon.surfaces, loc.surfaces);
        assert_eq!(canon.consumers, loc.consumers);
        if canon.reserved_meaning != loc.reserved_meaning
            || canon.canonical_label != loc.canonical_label
        {
            any_prose_changed = true;
        }
    }
    assert!(any_prose_changed, "localized overlay changed no prose");

    // Forbidden patterns never localize: they are machine identity.
    for (canon, loc) in canonical
        .forbidden_phrases
        .iter()
        .zip(localized.forbidden_phrases.iter())
    {
        assert_eq!(canon.pattern, loc.pattern);
        assert_eq!(canon.phrase_id, loc.phrase_id);
        assert_eq!(canon.class, loc.class);
        assert_ne!(canon.rejection_reason, loc.rejection_reason);
    }
}

#[test]
fn offline_mirror_variant_validates_and_keeps_identity() {
    let canonical = seeded_ai_copy_guardrail_catalog();
    let mirror = seeded_ai_copy_guardrail_catalog_offline_mirror();
    assert!(mirror.validate().is_empty(), "{:?}", mirror.validate());
    assert_eq!(mirror.terms, canonical.terms);
    assert_eq!(mirror.forbidden_phrases, canonical.forbidden_phrases);
    assert_ne!(mirror.catalog_id, canonical.catalog_id);
}

#[test]
fn checked_support_export_validates() {
    let catalog = current_ai_copy_guardrail_catalog_export()
        .expect("checked ai copy guardrail catalog export validates");
    assert_eq!(catalog.catalog_id, AI_COPY_GUARDRAIL_CATALOG_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_ai_copy_guardrail_catalog_export()
        .expect("checked ai copy guardrail catalog export validates");
    assert_eq!(
        from_disk,
        seeded_ai_copy_guardrail_catalog(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn checked_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/content/m5-ai-copy-guardrails/localized_overlay.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/content/m5-ai-copy-guardrails/offline_mirror.json"
        )),
    ] {
        let catalog: AiCopyGuardrailCatalog =
            serde_json::from_str(raw).expect("fixture parses as catalog");
        assert!(
            catalog.validate().is_empty(),
            "fixture failed validation: {:?}",
            catalog.validate()
        );
    }
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_ai_copy_guardrail_catalog().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
}
