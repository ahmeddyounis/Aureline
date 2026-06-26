use super::*;

#[test]
fn seeded_catalog_validates() {
    let catalog = seeded_action_label_scope_catalog();
    assert!(catalog.validate().is_empty(), "{:?}", catalog.validate());
    assert_eq!(catalog.catalog_id, ACTION_LABEL_SCOPE_CATALOG_ID);
}

#[test]
fn inventories_match_canonical() {
    let catalog = seeded_action_label_scope_catalog();
    assert_eq!(catalog.scope_inventory.len(), ScopeClass::ALL.len());
    assert_eq!(catalog.surface_inventory.len(), ActionSurface::ALL.len());
    assert_eq!(
        catalog.mutation_class_inventory.len(),
        MutationClass::ALL.len()
    );
    assert_eq!(catalog.review_state_inventory.len(), ReviewState::ALL.len());
}

#[test]
fn inventory_drift_fails() {
    let mut catalog = seeded_action_label_scope_catalog();
    catalog.surface_inventory.pop();
    assert!(catalog
        .validate()
        .contains(&ActionLabelScopeCatalogViolation::InventoryDrift));
}

#[test]
fn banned_token_set_must_be_complete() {
    let mut catalog = seeded_action_label_scope_catalog();
    catalog.banned_ambiguous_tokens.retain(|t| t != "continue");
    assert!(catalog
        .validate()
        .contains(&ActionLabelScopeCatalogViolation::BannedTokenSetMissing));
}

#[test]
fn ambiguous_continue_verb_is_rejected() {
    let mut catalog = seeded_action_label_scope_catalog();
    // A vague "Continue" verb on the first batch label trips the ambiguity gate.
    catalog.verbs.push(verb_for_test("continue", "Continue"));
    let label = catalog
        .labels
        .iter_mut()
        .find(|l| l.label_id == "action.batch.approve_all_matching_changes")
        .expect("label present");
    label.verb_ref = "continue".to_owned();
    let violations = catalog.validate();
    assert!(violations.contains(&ActionLabelScopeCatalogViolation::AmbiguousPrimaryLabel));
    assert!(violations.contains(&ActionLabelScopeCatalogViolation::VerbLabelAmbiguous));
}

/// Builds a bare verb for negative tests.
fn verb_for_test(verb_id: &str, label: &str) -> ActionVerb {
    ActionVerb {
        verb_id: verb_id.to_owned(),
        canonical_label: label.to_owned(),
        reversibility: ReversibilityClass::Reversible,
        default_mutation_class: MutationClass::BatchMutation,
    }
}

#[test]
fn label_must_be_verb_first() {
    let mut catalog = seeded_action_label_scope_catalog();
    let label = &mut catalog.labels[0];
    label.reference_label = "{count:count} {scope:selected} {verb} {object_many}".to_owned();
    assert!(catalog
        .validate()
        .contains(&ActionLabelScopeCatalogViolation::LabelNotVerbFirst));
}

#[test]
fn batch_label_must_declare_scope_unless_sheet_disambiguates() {
    let mut catalog = seeded_action_label_scope_catalog();
    // Drop the scope word from a selected-scope label that does not rely on a sheet.
    let label = catalog
        .labels
        .iter_mut()
        .find(|l| l.label_id == "action.review.approve_selected_changes")
        .expect("label present");
    label.reference_label = "{verb} {count:count} {object_many}".to_owned();
    label.screen_reader_label = "{verb} {count:count} {object_many}".to_owned();
    assert!(catalog
        .validate()
        .contains(&ActionLabelScopeCatalogViolation::ScopeNotDeclared));
}

#[test]
fn sheet_unambiguous_label_is_allowed_to_omit_scope_word() {
    let catalog = seeded_action_label_scope_catalog();
    let label = catalog
        .label("action.review.approve_changes_in_sheet")
        .expect("label present");
    // Visible button omits the scope word, narrated label keeps it, and the catalog
    // still validates clean.
    assert!(!label.reference_label.contains("{scope:"));
    assert!(label.screen_reader_label.contains("{scope:selected}"));
    assert!(catalog.validate().is_empty(), "{:?}", catalog.validate());
}

#[test]
fn label_must_narrow_object_class() {
    let mut catalog = seeded_action_label_scope_catalog();
    let label = &mut catalog.labels[0];
    label.reference_label = "{verb} {count:count} {scope:selected}".to_owned();
    assert!(catalog
        .validate()
        .contains(&ActionLabelScopeCatalogViolation::ObjectClassNotNarrowed));
}

#[test]
fn counted_batch_label_must_declare_count() {
    let mut catalog = seeded_action_label_scope_catalog();
    let label = catalog
        .labels
        .iter_mut()
        .find(|l| l.label_id == "action.batch.rerun_visible_tasks")
        .expect("label present");
    label.count_var = None;
    label.reference_label = "{verb} {scope:visible} {object_many}".to_owned();
    label.screen_reader_label = "{verb} {scope:visible} {object_many}".to_owned();
    assert!(catalog
        .validate()
        .contains(&ActionLabelScopeCatalogViolation::BatchCountMissing));
}

#[test]
fn count_var_and_template_slot_must_agree() {
    let mut catalog = seeded_action_label_scope_catalog();
    let label = &mut catalog.labels[0];
    label.count_var = Some("other".to_owned());
    assert!(catalog
        .validate()
        .contains(&ActionLabelScopeCatalogViolation::CountVarMismatch));
}

#[test]
fn action_cannot_target_an_exclusion_scope() {
    let mut catalog = seeded_action_label_scope_catalog();
    let label = &mut catalog.labels[0];
    label.scope_ref = "hidden_by_policy".to_owned();
    label.reference_label =
        "{verb} {count:count} {scope:hidden_by_policy} {object_many}".to_owned();
    label.screen_reader_label =
        "{verb} {count:count} {scope:hidden_by_policy} {object_many}".to_owned();
    assert!(catalog
        .validate()
        .contains(&ActionLabelScopeCatalogViolation::ActionScopeNotActionable));
}

#[test]
fn approval_label_must_declare_review_state() {
    let mut catalog = seeded_action_label_scope_catalog();
    let label = catalog
        .labels
        .iter_mut()
        .find(|l| l.label_id == "action.batch.approve_all_matching_changes")
        .expect("label present");
    label.review_state = ReviewState::NoReviewNeeded;
    assert!(catalog
        .validate()
        .contains(&ActionLabelScopeCatalogViolation::ReviewStateNotDeclared));
}

#[test]
fn destructive_label_must_disclose_side_effect() {
    let mut catalog = seeded_action_label_scope_catalog();
    let label = catalog
        .labels
        .iter_mut()
        .find(|l| l.label_id == "action.batch.delete_selected_files")
        .expect("label present");
    label.discloses_side_effect = false;
    assert!(catalog
        .validate()
        .contains(&ActionLabelScopeCatalogViolation::SideEffectNotDisclosed));
}

#[test]
fn screen_reader_label_must_carry_scope() {
    let mut catalog = seeded_action_label_scope_catalog();
    // The sheet-unambiguous label may drop the scope word visibly but its narrated
    // label must still name the scope; remove it and validation flags the gap.
    let label = catalog
        .labels
        .iter_mut()
        .find(|l| l.label_id == "action.review.approve_changes_in_sheet")
        .expect("label present");
    label.screen_reader_label = "{verb} {count:count} {object_many}".to_owned();
    assert!(catalog
        .validate()
        .contains(&ActionLabelScopeCatalogViolation::ScreenReaderLabelIncomplete));
}

#[test]
fn template_placeholder_must_resolve() {
    let mut catalog = seeded_action_label_scope_catalog();
    catalog.labels[0].reference_label =
        "{verb} {count:count} {scope:made_up} {object_many}".to_owned();
    assert!(catalog
        .validate()
        .contains(&ActionLabelScopeCatalogViolation::TemplatePlaceholderUnresolved));
}

#[test]
fn label_ids_and_refs_are_locale_neutral() {
    let catalog = seeded_action_label_scope_catalog();
    let ok = |t: &str| {
        !t.is_empty()
            && t.chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '.')
    };
    for verb in &catalog.verbs {
        assert!(ok(&verb.verb_id), "verb id not neutral: {}", verb.verb_id);
    }
    for scope in &catalog.scopes {
        assert!(
            ok(&scope.scope_id),
            "scope id not neutral: {}",
            scope.scope_id
        );
    }
    for object in &catalog.objects {
        assert!(
            ok(&object.object_id),
            "object id not neutral: {}",
            object.object_id
        );
    }
    for label in &catalog.labels {
        assert!(
            ok(&label.label_id),
            "label id not neutral: {}",
            label.label_id
        );
    }
}

#[test]
fn render_label_resolves_verb_scope_and_object() {
    let catalog = seeded_action_label_scope_catalog();
    let rendered = catalog
        .render_label("action.review.approve_selected_changes")
        .expect("label resolves");
    assert_eq!(rendered, "Approve {count} selected changes");
}

#[test]
fn render_disclosure_resolves_counts_scopes_and_status() {
    let catalog = seeded_action_label_scope_catalog();
    let rendered = catalog
        .render_disclosure("disclosure.batch_bar.selected_with_policy_excluded")
        .expect("disclosure resolves");
    assert_eq!(
        rendered,
        "{acted_count} selected changes (exact); {hidden_count} hidden by policy, {outside_count} outside current workset not included."
    );
}

#[test]
fn shared_scope_phrases_reuse_across_surfaces() {
    let catalog = seeded_action_label_scope_catalog();
    let reuse = catalog.cross_surface_reuse();
    for scope_id in &catalog.shared_scope_phrase_ids {
        let spans = reuse.get(scope_id).map(|s| s.len()).unwrap_or(0);
        assert!(
            spans >= SHARED_SCOPE_MIN_REUSE_SURFACES,
            "shared scope {scope_id} only spans {spans} surfaces"
        );
    }
}

#[test]
fn insufficient_shared_reuse_fails() {
    let mut catalog = seeded_action_label_scope_catalog();
    catalog.shared_scope_phrase_ids = vec!["single_object".to_owned()];
    assert!(catalog
        .validate()
        .contains(&ActionLabelScopeCatalogViolation::SharedScopeReuseInsufficient));
}

#[test]
fn coverage_spans_every_scope_surface_mutation_and_review() {
    let catalog = seeded_action_label_scope_catalog();
    assert!(catalog.validate().is_empty());

    let mutations: std::collections::BTreeSet<_> =
        catalog.labels.iter().map(|l| l.mutation_class).collect();
    let reviews: std::collections::BTreeSet<_> =
        catalog.labels.iter().map(|l| l.review_state).collect();
    assert_eq!(mutations.len(), MutationClass::ALL.len());
    assert_eq!(reviews.len(), ReviewState::ALL.len());
}

#[test]
fn coverage_gap_fails() {
    let mut catalog = seeded_action_label_scope_catalog();
    // Drop the only install label, leaving that mutation class uncovered.
    catalog
        .labels
        .retain(|l| l.mutation_class != MutationClass::Install);
    assert!(catalog
        .validate()
        .contains(&ActionLabelScopeCatalogViolation::CoverageGap));
}

#[test]
fn docs_export_parity_required() {
    let mut catalog = seeded_action_label_scope_catalog();
    for label in &mut catalog.labels {
        label
            .consumer_surfaces
            .retain(|c| *c != ConsumerSurface::Docs);
    }
    assert!(catalog
        .validate()
        .contains(&ActionLabelScopeCatalogViolation::DocsExportParityMissing));
}

#[test]
fn disclosure_declared_scope_must_be_used() {
    let mut catalog = seeded_action_label_scope_catalog();
    let disclosure = catalog
        .disclosures
        .iter_mut()
        .find(|d| d.disclosure_id == "disclosure.activity_row.reran_loaded_tasks")
        .expect("disclosure present");
    disclosure
        .disclosed_scope_refs
        .push("hidden_by_policy".to_owned());
    assert!(catalog
        .validate()
        .contains(&ActionLabelScopeCatalogViolation::DeclaredTokenUnused));
}

#[test]
fn missing_source_contracts_fails() {
    let mut catalog = seeded_action_label_scope_catalog();
    catalog.source_contract_refs.clear();
    assert!(catalog
        .validate()
        .contains(&ActionLabelScopeCatalogViolation::MissingSourceContracts));
}

#[test]
fn parity_review_incomplete_fails() {
    let mut catalog = seeded_action_label_scope_catalog();
    catalog.parity_review.no_ambiguous_primary_labels = false;
    assert!(catalog
        .validate()
        .contains(&ActionLabelScopeCatalogViolation::ParityReviewIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut catalog = seeded_action_label_scope_catalog();
    catalog.release_posture.mirror_offline_parity_required = false;
    assert!(catalog
        .validate()
        .contains(&ActionLabelScopeCatalogViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_labels_and_disclosures() {
    let catalog = seeded_action_label_scope_catalog();
    let summary = catalog.render_markdown_summary();
    for label in &catalog.labels {
        assert!(
            summary.contains(&label.label_id),
            "summary missing {}",
            label.label_id
        );
    }
    for disclosure in &catalog.disclosures {
        assert!(
            summary.contains(&disclosure.disclosure_id),
            "summary missing {}",
            disclosure.disclosure_id
        );
    }
}

#[test]
fn localized_overlay_preserves_machine_identity() {
    let canonical = seeded_action_label_scope_catalog();
    let localized = seeded_action_label_scope_catalog_localized();
    assert!(
        localized.validate().is_empty(),
        "localized overlay failed validation: {:?}",
        localized.validate()
    );
    assert_ne!(canonical.reference_locale, localized.reference_locale);

    let canon_label_ids: Vec<&str> = canonical
        .labels
        .iter()
        .map(|l| l.label_id.as_str())
        .collect();
    let loc_label_ids: Vec<&str> = localized
        .labels
        .iter()
        .map(|l| l.label_id.as_str())
        .collect();
    assert_eq!(canon_label_ids, loc_label_ids);

    // Machine identity (ids, refs, and the ordered placeholders) stays byte-for-byte
    // identical; only the resolved human prose — verb labels, scope phrases, object
    // nouns — localizes, so the rendered label differs while the template does not.
    let mut any_prose_changed = false;
    for (canon, loc) in canonical.labels.iter().zip(localized.labels.iter()) {
        assert_eq!(canon.scope_ref, loc.scope_ref);
        assert_eq!(canon.verb_ref, loc.verb_ref);
        assert_eq!(canon.object_ref, loc.object_ref);
        assert_eq!(
            placeholders(&canon.reference_label),
            placeholders(&loc.reference_label),
            "placeholders drifted for {}",
            canon.label_id
        );
        let canon_render = canonical.render_label(&canon.label_id);
        let loc_render = localized.render_label(&loc.label_id);
        assert_ne!(
            canon_render, loc_render,
            "rendered prose did not localize for {}",
            canon.label_id
        );
        if canon_render != loc_render {
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
    let canonical = seeded_action_label_scope_catalog();
    let mirror = seeded_action_label_scope_catalog_offline_mirror();
    assert!(mirror.validate().is_empty(), "{:?}", mirror.validate());
    assert_eq!(mirror.labels, canonical.labels);
    assert_eq!(mirror.scopes, canonical.scopes);
    assert_ne!(mirror.catalog_id, canonical.catalog_id);
}

#[test]
fn checked_support_export_validates() {
    let catalog = current_action_label_scope_catalog_export()
        .expect("checked action-label/scope catalog export validates");
    assert_eq!(catalog.catalog_id, ACTION_LABEL_SCOPE_CATALOG_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_action_label_scope_catalog_export()
        .expect("checked action-label/scope catalog export validates");
    assert_eq!(
        from_disk,
        seeded_action_label_scope_catalog(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn checked_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/content/m5-action-label-scope/localized_overlay.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/content/m5-action-label-scope/offline_mirror.json"
        )),
    ] {
        let catalog: ActionLabelScopeCatalog =
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
    let json = seeded_action_label_scope_catalog().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
}
