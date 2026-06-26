use super::*;

#[test]
fn seeded_catalog_validates() {
    let catalog = seeded_safety_critical_string_catalog();
    assert!(catalog.validate().is_empty(), "{:?}", catalog.validate());
    assert_eq!(catalog.catalog_id, SAFETY_CRITICAL_STRING_CATALOG_ID);
}

#[test]
fn inventories_match_canonical() {
    let catalog = seeded_safety_critical_string_catalog();
    assert_eq!(catalog.audience_inventory.len(), MessageAudience::ALL.len());
    assert_eq!(catalog.severity_inventory.len(), MessageSeverity::ALL.len());
    assert_eq!(
        catalog.surface_inventory.len(),
        MessageSurfaceFamily::ALL.len()
    );
    assert_eq!(
        catalog.term_class_inventory.len(),
        ControlledTermClass::ALL.len()
    );
}

#[test]
fn inventory_drift_fails() {
    let mut catalog = seeded_safety_critical_string_catalog();
    catalog.severity_inventory.pop();
    assert!(catalog
        .validate()
        .contains(&SafetyCriticalStringCatalogViolation::InventoryDrift));
}

#[test]
fn every_message_glossary_ref_resolves() {
    let catalog = seeded_safety_critical_string_catalog();
    for message in &catalog.messages {
        for term_id in &message.glossary_term_refs {
            assert!(
                catalog.term(term_id).is_some(),
                "message {} references unknown term {term_id}",
                message.message_id
            );
        }
    }
}

#[test]
fn dangling_glossary_ref_fails() {
    let mut catalog = seeded_safety_critical_string_catalog();
    catalog.messages[0]
        .glossary_term_refs
        .push("term.does_not_exist".to_owned());
    let violations = catalog.validate();
    assert!(violations.contains(&SafetyCriticalStringCatalogViolation::GlossaryTermRefUnresolved));
}

#[test]
fn message_ids_and_term_ids_are_locale_neutral() {
    let catalog = seeded_safety_critical_string_catalog();
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
    for message in &catalog.messages {
        assert!(
            ok(&message.message_id),
            "message id not locale-neutral: {}",
            message.message_id
        );
        for variable in &message.variables {
            assert!(
                ok(&variable.name),
                "var not locale-neutral: {}",
                variable.name
            );
        }
    }
}

#[test]
fn non_locale_neutral_message_id_fails() {
    let mut catalog = seeded_safety_critical_string_catalog();
    catalog.messages[0].message_id = "Msg.Trust.Prompt".to_owned();
    assert!(catalog
        .validate()
        .contains(&SafetyCriticalStringCatalogViolation::MessageIdNotLocaleNeutral));
}

#[test]
fn template_placeholder_must_resolve() {
    let mut catalog = seeded_safety_critical_string_catalog();
    catalog.messages[0].reference_template =
        "{term:term.unverified_source} {var:undeclared_var}".to_owned();
    let violations = catalog.validate();
    assert!(
        violations.contains(&SafetyCriticalStringCatalogViolation::TemplatePlaceholderUnresolved)
    );
}

#[test]
fn declared_but_unused_term_fails() {
    let mut catalog = seeded_safety_critical_string_catalog();
    // Add a real term ref to a message whose template does not use it.
    catalog.messages[1]
        .glossary_term_refs
        .push("term.stale".to_owned());
    assert!(catalog
        .validate()
        .contains(&SafetyCriticalStringCatalogViolation::DeclaredTokenUnused));
}

#[test]
fn term_on_disallowed_surface_fails() {
    let mut catalog = seeded_safety_critical_string_catalog();
    // Restrict the unverified-source term to the trust prompt only, then it is
    // illegal on the support-export heading that also references it.
    let term = catalog
        .terms
        .iter_mut()
        .find(|t| t.term_id == "term.unverified_source")
        .expect("term present");
    term.allowed_surfaces = vec![MessageSurfaceFamily::TrustPrompt];
    assert!(catalog
        .validate()
        .contains(&SafetyCriticalStringCatalogViolation::TermUsedOnDisallowedSurface));
}

#[test]
fn recovery_block_requires_all_four_parts() {
    let mut catalog = seeded_safety_critical_string_catalog();
    let message = catalog
        .messages
        .iter_mut()
        .find(|m| m.message_class == MessageClass::ErrorRecoveryBlock)
        .expect("recovery block present");
    message.variables.retain(|v| v.name != "what_still_works");
    let violations = catalog.validate();
    assert!(violations.contains(&SafetyCriticalStringCatalogViolation::RecoveryBlockMissingPart));
}

#[test]
fn ai_copy_overclaim_fails() {
    let mut catalog = seeded_safety_critical_string_catalog();
    let message = catalog
        .messages
        .iter_mut()
        .find(|m| m.message_class == MessageClass::AiCopyLine)
        .expect("ai copy present");
    // Keep the declared placeholders so only the overclaim rule trips.
    message.reference_template =
        "Based on {var:source_count} sources; this answer is guaranteed 100% correct. {term:term.cached} {term:term.proven_current}".to_owned();
    assert!(catalog
        .validate()
        .contains(&SafetyCriticalStringCatalogViolation::AiCopyOverclaim));
}

#[test]
fn count_scope_must_disclose_freshness() {
    let mut catalog = seeded_safety_critical_string_catalog();
    let message = catalog
        .messages
        .iter_mut()
        .find(|m| m.message_id == "msg.count.search_stale_phrase")
        .expect("count phrase present");
    // Drop the freshness term and its placeholder.
    message.glossary_term_refs.clear();
    message.reference_template = "{var:match_count} matches.".to_owned();
    assert!(catalog
        .validate()
        .contains(&SafetyCriticalStringCatalogViolation::CountScopeNotFreshnessHonest));
}

#[test]
fn shared_terms_reuse_across_surfaces() {
    let catalog = seeded_safety_critical_string_catalog();
    let reuse = catalog.cross_surface_reuse();
    for term_id in &catalog.shared_reuse_term_ids {
        let spans = reuse.get(term_id).map(|s| s.len()).unwrap_or(0);
        assert!(
            spans >= SHARED_TERM_MIN_REUSE_SURFACES,
            "shared term {term_id} only spans {spans} surfaces"
        );
    }
}

#[test]
fn insufficient_shared_reuse_fails() {
    let mut catalog = seeded_safety_critical_string_catalog();
    // Strip every reference to a shared term except one surface.
    for message in &mut catalog.messages {
        if message.message_id != "msg.support.trust_state_heading" {
            message
                .glossary_term_refs
                .retain(|t| t != "term.trust_required");
        }
    }
    // The template of any message that lost the term still references it, which
    // would also flag DeclaredTokenUnused / placeholder errors; the reuse failure
    // is what we assert.
    assert!(catalog
        .validate()
        .contains(&SafetyCriticalStringCatalogViolation::SharedTermReuseInsufficient));
}

#[test]
fn coverage_spans_every_audience_severity_and_surface() {
    let catalog = seeded_safety_critical_string_catalog();
    let audiences: std::collections::BTreeSet<_> =
        catalog.messages.iter().map(|m| m.audience).collect();
    let severities: std::collections::BTreeSet<_> =
        catalog.messages.iter().map(|m| m.severity).collect();
    let surfaces: std::collections::BTreeSet<_> =
        catalog.messages.iter().map(|m| m.surface_family).collect();
    assert_eq!(audiences.len(), MessageAudience::ALL.len());
    assert_eq!(severities.len(), MessageSeverity::ALL.len());
    assert_eq!(surfaces.len(), MessageSurfaceFamily::ALL.len());
}

#[test]
fn coverage_gap_fails() {
    let mut catalog = seeded_safety_critical_string_catalog();
    // Drop the only screen-reader message, leaving that audience uncovered.
    catalog
        .messages
        .retain(|m| m.audience != MessageAudience::ScreenReader);
    assert!(catalog
        .validate()
        .contains(&SafetyCriticalStringCatalogViolation::CoverageGap));
}

#[test]
fn next_action_ref_must_resolve_to_action_label() {
    let mut catalog = seeded_safety_critical_string_catalog();
    catalog.messages[0].next_action_label_ref = Some("msg.not.an.action".to_owned());
    assert!(catalog
        .validate()
        .contains(&SafetyCriticalStringCatalogViolation::NextActionRefUnresolved));
}

#[test]
fn render_reference_resolves_controlled_terms() {
    let catalog = seeded_safety_critical_string_catalog();
    let rendered = catalog
        .render_reference("msg.trust.unverified_source_prompt")
        .expect("message resolves");
    assert!(rendered.contains("Unverified source"));
    assert!(rendered.contains("Trust required"));
    // The variable stays a named slot; the protected term is resolved, not inlined.
    assert!(rendered.contains("{source_name}"));
    assert!(!rendered.contains("{term:"));
}

#[test]
fn missing_source_contracts_fails() {
    let mut catalog = seeded_safety_critical_string_catalog();
    catalog.source_contract_refs.clear();
    assert!(catalog
        .validate()
        .contains(&SafetyCriticalStringCatalogViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut catalog = seeded_safety_critical_string_catalog();
    catalog.trust_review.controlled_terms_resolved_not_inlined = false;
    assert!(catalog
        .validate()
        .contains(&SafetyCriticalStringCatalogViolation::TrustReviewIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut catalog = seeded_safety_critical_string_catalog();
    catalog.release_posture.mirror_offline_parity_required = false;
    assert!(catalog
        .validate()
        .contains(&SafetyCriticalStringCatalogViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_terms_and_messages() {
    let catalog = seeded_safety_critical_string_catalog();
    let summary = catalog.render_markdown_summary();
    for term in &catalog.terms {
        assert!(
            summary.contains(&term.term_id),
            "summary missing {}",
            term.term_id
        );
    }
    for message in &catalog.messages {
        assert!(
            summary.contains(&message.message_id),
            "summary missing {}",
            message.message_id
        );
    }
}

#[test]
fn localized_overlay_preserves_machine_identity() {
    let canonical = seeded_safety_critical_string_catalog();
    let localized = seeded_safety_critical_string_catalog_localized();
    assert!(
        localized.validate().is_empty(),
        "localized overlay failed validation: {:?}",
        localized.validate()
    );
    assert_ne!(canonical.reference_locale, localized.reference_locale);

    // Same set of message ids and term ids, in the same order.
    let canon_msg_ids: Vec<&str> = canonical
        .messages
        .iter()
        .map(|m| m.message_id.as_str())
        .collect();
    let loc_msg_ids: Vec<&str> = localized
        .messages
        .iter()
        .map(|m| m.message_id.as_str())
        .collect();
    assert_eq!(canon_msg_ids, loc_msg_ids);

    let canon_term_ids: Vec<&str> = canonical.terms.iter().map(|t| t.term_id.as_str()).collect();
    let loc_term_ids: Vec<&str> = localized.terms.iter().map(|t| t.term_id.as_str()).collect();
    assert_eq!(canon_term_ids, loc_term_ids);

    // Prose differs but placeholders (machine tokens) are identical per message.
    let mut any_prose_changed = false;
    for (canon, loc) in canonical.messages.iter().zip(localized.messages.iter()) {
        assert_eq!(canon.glossary_term_refs, loc.glossary_term_refs);
        assert_eq!(
            placeholders(&canon.reference_template),
            placeholders(&loc.reference_template),
            "placeholders drifted for {}",
            canon.message_id
        );
        if canon.reference_template != loc.reference_template {
            any_prose_changed = true;
        }
    }
    assert!(any_prose_changed, "localized overlay changed no prose");
}

/// Extracts the ordered `{...}` placeholders from a template for comparison.
fn placeholders(template: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = template;
    while let Some(start) = rest.find('{') {
        if let Some(end) = rest[start..].find('}') {
            out.push(rest[start..start + end + 1].to_owned());
            rest = &rest[start + end + 1..];
        } else {
            break;
        }
    }
    out
}

#[test]
fn offline_mirror_variant_validates_and_keeps_identity() {
    let canonical = seeded_safety_critical_string_catalog();
    let mirror = seeded_safety_critical_string_catalog_offline_mirror();
    assert!(mirror.validate().is_empty(), "{:?}", mirror.validate());
    assert_eq!(mirror.messages, canonical.messages);
    assert_eq!(mirror.terms, canonical.terms);
    assert_ne!(mirror.catalog_id, canonical.catalog_id);
}

#[test]
fn checked_support_export_validates() {
    let catalog = current_stable_safety_critical_string_catalog_export()
        .expect("checked safety-critical string catalog export validates");
    assert_eq!(catalog.catalog_id, SAFETY_CRITICAL_STRING_CATALOG_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_safety_critical_string_catalog_export()
        .expect("checked safety-critical string catalog export validates");
    assert_eq!(
        from_disk,
        seeded_safety_critical_string_catalog(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn checked_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/content/m5-safety-critical-strings/localized_overlay.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/content/m5-safety-critical-strings/offline_mirror.json"
        )),
    ] {
        let catalog: SafetyCriticalStringCatalog =
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
    let json = seeded_safety_critical_string_catalog().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
}
