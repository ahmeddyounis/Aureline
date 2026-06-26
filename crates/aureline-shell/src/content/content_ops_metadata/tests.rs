use super::*;

#[test]
fn seeded_catalog_validates() {
    let catalog = seeded_content_ops_metadata_catalog();
    assert!(catalog.validate().is_empty(), "{:?}", catalog.validate());
    assert_eq!(catalog.catalog_id, CONTENT_OPS_METADATA_CATALOG_ID);
}

#[test]
fn inventories_match_canonical() {
    let catalog = seeded_content_ops_metadata_catalog();
    assert_eq!(catalog.kind_inventory.len(), ContentArtifactKind::ALL.len());
    assert_eq!(
        catalog.consumer_inventory.len(),
        ContentOpsConsumer::ALL.len()
    );
    assert_eq!(
        catalog.capture_posture_inventory.len(),
        CapturePosture::ALL.len()
    );
    assert_eq!(
        catalog.caption_sync_state_inventory.len(),
        CaptionSyncState::ALL.len()
    );
    assert_eq!(
        catalog.fallback_strategy_inventory.len(),
        LocaleFallbackStrategy::ALL.len()
    );
    assert_eq!(
        catalog.placeholder_kind_inventory.len(),
        PlaceholderKind::ALL.len()
    );
    assert_eq!(
        catalog.token_fidelity_class_inventory.len(),
        TokenFidelityClass::ALL.len()
    );
    assert_eq!(
        catalog.translator_note_class_inventory.len(),
        TranslatorNoteClass::ALL.len()
    );
}

#[test]
fn inventory_drift_fails() {
    let mut catalog = seeded_content_ops_metadata_catalog();
    catalog.placeholder_kind_inventory.pop();
    assert!(catalog
        .validate()
        .contains(&ContentOpsViolation::InventoryDrift));
}

#[test]
fn every_artifact_kind_has_an_entry() {
    let catalog = seeded_content_ops_metadata_catalog();
    let kinds: std::collections::BTreeSet<_> = catalog.entries.iter().map(|e| e.kind).collect();
    for kind in ContentArtifactKind::ALL {
        assert!(kinds.contains(&kind), "missing entry for {kind:?}");
    }
}

#[test]
fn coverage_spans_kinds_consumers_postures_and_strategies() {
    let catalog = seeded_content_ops_metadata_catalog();
    let kinds: std::collections::BTreeSet<_> = catalog.entries.iter().map(|e| e.kind).collect();
    let consumers: std::collections::BTreeSet<_> = catalog
        .entries
        .iter()
        .flat_map(|e| e.consumers.iter().copied())
        .collect();
    let postures: std::collections::BTreeSet<_> = catalog
        .entries
        .iter()
        .map(|e| e.version_context.capture_posture)
        .collect();
    let strategies: std::collections::BTreeSet<_> = catalog
        .entries
        .iter()
        .map(|e| e.locale_fallback.strategy)
        .collect();
    assert_eq!(kinds.len(), ContentArtifactKind::ALL.len());
    assert_eq!(consumers.len(), ContentOpsConsumer::ALL.len());
    assert_eq!(postures.len(), CapturePosture::ALL.len());
    assert_eq!(strategies.len(), LocaleFallbackStrategy::ALL.len());
}

#[test]
fn coverage_gap_fails_when_a_consumer_is_missing() {
    let mut catalog = seeded_content_ops_metadata_catalog();
    for entry in &mut catalog.entries {
        entry
            .consumers
            .retain(|c| *c != ContentOpsConsumer::ScreenshotDemoPipeline);
    }
    assert!(catalog
        .validate()
        .contains(&ContentOpsViolation::CoverageGap));
}

#[test]
fn entry_ids_machine_fields_and_tokens_are_locale_neutral() {
    let catalog = seeded_content_ops_metadata_catalog();
    let ok = |t: &str| {
        !t.is_empty()
            && t.chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '.')
    };
    for entry in &catalog.entries {
        assert!(
            ok(&entry.entry_id),
            "entry id not locale-neutral: {}",
            entry.entry_id
        );
        if let Some(field) = &entry.machine_field_name {
            assert!(ok(field), "machine field not locale-neutral: {field}");
        }
        if let Some(command) = &entry.command_ref {
            assert!(ok(command), "command ref not locale-neutral: {command}");
        }
        for note in &entry.placeholder_notes {
            assert!(
                ok(&note.token_id),
                "token id not locale-neutral: {}",
                note.token_id
            );
        }
    }
}

#[test]
fn non_locale_neutral_entry_id_fails() {
    let mut catalog = seeded_content_ops_metadata_catalog();
    catalog.entries[0].entry_id = "Entry.Docs.Findings".to_owned();
    assert!(catalog
        .validate()
        .contains(&ContentOpsViolation::EntryTokenNotLocaleNeutral));
}

#[test]
fn duplicate_entry_fails() {
    let mut catalog = seeded_content_ops_metadata_catalog();
    let clone = catalog.entries[0].clone();
    catalog.entries.push(clone);
    assert!(catalog
        .validate()
        .contains(&ContentOpsViolation::DuplicateEntry));
}

#[test]
fn entry_incomplete_fails() {
    let mut catalog = seeded_content_ops_metadata_catalog();
    catalog.entries[0].source_ref = "  ".to_owned();
    assert!(catalog
        .validate()
        .contains(&ContentOpsViolation::EntryIncomplete));
}

#[test]
fn heading_without_machine_field_fails() {
    let mut catalog = seeded_content_ops_metadata_catalog();
    let heading = catalog
        .entries
        .iter_mut()
        .find(|e| e.kind == ContentArtifactKind::ExportReportHeading)
        .expect("heading present");
    heading.machine_field_name = None;
    assert!(catalog
        .validate()
        .contains(&ContentOpsViolation::HeadingMissingMachineField));
}

#[test]
fn every_heading_pairs_human_label_with_machine_code() {
    let catalog = seeded_content_ops_metadata_catalog();
    for heading in catalog.entries_for_kind(ContentArtifactKind::ExportReportHeading) {
        assert!(!heading.canonical_text.trim().is_empty());
        let field = heading
            .machine_field_name
            .as_deref()
            .expect("heading carries a machine field");
        assert!(!field.trim().is_empty());
    }
}

#[test]
fn translator_note_without_class_or_target_fails() {
    let mut catalog = seeded_content_ops_metadata_catalog();
    let note = catalog
        .entries
        .iter_mut()
        .find(|e| e.kind == ContentArtifactKind::TranslatorNote)
        .expect("translator note present");
    note.translator_note_class = None;
    assert!(catalog
        .validate()
        .contains(&ContentOpsViolation::TranslatorNoteIncomplete));
}

#[test]
fn rendered_artifact_without_version_fails() {
    let mut catalog = seeded_content_ops_metadata_catalog();
    let docs = catalog
        .entries
        .iter_mut()
        .find(|e| e.kind == ContentArtifactKind::DocsHelpSnippet)
        .expect("docs snippet present");
    docs.version_context.build_ref = String::new();
    assert!(catalog
        .validate()
        .contains(&ContentOpsViolation::MissingVersionContext));
}

#[test]
fn release_support_path_without_version_fails() {
    let mut catalog = seeded_content_ops_metadata_catalog();
    // A translator note is exempt from version context — until it is marked as a
    // release/support-path artifact, at which point versionless is denied.
    let note = catalog
        .entries
        .iter_mut()
        .find(|e| e.kind == ContentArtifactKind::TranslatorNote)
        .expect("translator note present");
    note.release_support_path = true;
    note.version_context.product_version_ref = String::new();
    note.version_context.build_ref = String::new();
    assert!(catalog
        .validate()
        .contains(&ContentOpsViolation::MissingVersionContext));
}

#[test]
fn screenshot_caption_without_posture_fails() {
    let mut catalog = seeded_content_ops_metadata_catalog();
    let caption = catalog
        .entries
        .iter_mut()
        .find(|e| e.kind == ContentArtifactKind::ScreenshotDemoCaption)
        .expect("caption present");
    caption.version_context.capture_posture = CapturePosture::NotApplicable;
    assert!(catalog
        .validate()
        .contains(&ContentOpsViolation::CaptionPostureUndeclared));
}

#[test]
fn screenshot_caption_without_disclosure_fails() {
    let mut catalog = seeded_content_ops_metadata_catalog();
    let caption = catalog
        .entries
        .iter_mut()
        .find(|e| e.kind == ContentArtifactKind::ScreenshotDemoCaption)
        .expect("caption present");
    caption.version_context.mocked_versus_live_disclosed = false;
    assert!(catalog
        .validate()
        .contains(&ContentOpsViolation::CaptionPostureUndeclared));
}

#[test]
fn every_screenshot_caption_declares_posture_and_sync() {
    let catalog = seeded_content_ops_metadata_catalog();
    for caption in catalog.entries_for_kind(ContentArtifactKind::ScreenshotDemoCaption) {
        let vc = &caption.version_context;
        assert!(
            vc.capture_posture.is_media(),
            "{} not media",
            caption.entry_id
        );
        assert_ne!(vc.caption_sync_state, CaptionSyncState::NotApplicable);
        assert!(
            vc.mocked_versus_live_disclosed,
            "{} undisclosed",
            caption.entry_id
        );
        assert!(!vc.product_version_ref.trim().is_empty());
        assert!(!vc.build_ref.trim().is_empty());
    }
}

#[test]
fn variable_rich_string_without_placeholder_note_fails() {
    let mut catalog = seeded_content_ops_metadata_catalog();
    let entry = catalog
        .entries
        .iter_mut()
        .find(|e| e.entry_id == "entry.docs.project_doctor_findings")
        .expect("docs snippet present");
    entry.placeholder_notes.clear();
    assert!(catalog
        .validate()
        .contains(&ContentOpsViolation::PlaceholderNoteMissing));
}

#[test]
fn every_rendered_placeholder_has_a_note() {
    let catalog = seeded_content_ops_metadata_catalog();
    for entry in &catalog.entries {
        if !entry.kind.canonical_text_is_rendered() {
            continue;
        }
        let declared: std::collections::BTreeSet<&str> = entry
            .placeholder_notes
            .iter()
            .map(|n| n.placeholder.as_str())
            .collect();
        for placeholder in extract_placeholders(&entry.canonical_text) {
            assert!(
                declared.contains(placeholder.as_str()),
                "entry {} placeholder {placeholder} has no note",
                entry.entry_id
            );
        }
    }
}

#[test]
fn count_placeholder_without_plural_rule_fails() {
    let mut catalog = seeded_content_ops_metadata_catalog();
    let entry = catalog
        .entries
        .iter_mut()
        .find(|e| e.entry_id == "entry.heading.findings_exported_count")
        .expect("count heading present");
    entry.placeholder_notes[0].plural_rule_ref = None;
    assert!(catalog
        .validate()
        .contains(&ContentOpsViolation::PluralRuleMissing));
}

#[test]
fn glossary_token_without_ref_fails() {
    let mut catalog = seeded_content_ops_metadata_catalog();
    let entry = catalog
        .entries
        .iter_mut()
        .find(|e| e.entry_id == "entry.docs.project_doctor_findings")
        .expect("docs snippet present");
    let scope = entry
        .placeholder_notes
        .iter_mut()
        .find(|n| n.kind == PlaceholderKind::GlossaryTermToken)
        .expect("scope token present");
    scope.glossary_term_ref = None;
    assert!(catalog
        .validate()
        .contains(&ContentOpsViolation::GlossaryRefMissing));
}

#[test]
fn locale_fallback_chain_must_terminate_at_source_language() {
    let mut catalog = seeded_content_ops_metadata_catalog();
    let entry = catalog
        .entries
        .iter_mut()
        .find(|e| e.locale_fallback.strategy == LocaleFallbackStrategy::SourceLanguageRoute)
        .expect("source-language entry present");
    entry.locale_fallback.fallback_chain = vec!["fr".to_owned()];
    assert!(catalog
        .validate()
        .contains(&ContentOpsViolation::LocaleFallbackIncomplete));
}

#[test]
fn policy_blocked_fallback_without_ref_fails() {
    let mut catalog = seeded_content_ops_metadata_catalog();
    let entry = catalog
        .entries
        .iter_mut()
        .find(|e| e.locale_fallback.strategy == LocaleFallbackStrategy::PolicyBlocked)
        .expect("policy-blocked entry present");
    entry.locale_fallback.policy_block_ref = None;
    assert!(catalog
        .validate()
        .contains(&ContentOpsViolation::LocaleFallbackIncomplete));
}

#[test]
fn extract_placeholders_is_ordered_and_deduped() {
    let found = extract_placeholders("a {count} and {scope} and {count} again");
    assert_eq!(found, vec!["{count}".to_owned(), "{scope}".to_owned()]);
    assert!(extract_placeholders("no tokens here").is_empty());
    assert!(extract_placeholders("empty {} ignored").is_empty());
}

#[test]
fn render_provenance_explains_source_command_and_version() {
    let catalog = seeded_content_ops_metadata_catalog();
    let rendered = catalog
        .render_provenance("entry.caption.activity_center_live")
        .expect("entry resolves");
    assert!(rendered.contains("source: string.shell.activity_center_title"));
    assert!(rendered.contains("command: command.window.activity_center"));
    assert!(rendered.contains("version: version.channel.stable.2026.06"));
    assert!(rendered.contains("build: build.m5.content_ops.0001"));
    // A captured caption discloses its posture and sync state in the provenance line.
    assert!(rendered.contains("posture: live"));
    assert!(rendered.contains("sync: in_sync"));
}

#[test]
fn render_provenance_includes_machine_field_for_headings() {
    let catalog = seeded_content_ops_metadata_catalog();
    let rendered = catalog
        .render_provenance("entry.heading.findings_by_severity")
        .expect("entry resolves");
    assert!(rendered.contains("field: report.column.findings_by_severity"));
}

#[test]
fn shared_entries_reuse_across_consumers() {
    let catalog = seeded_content_ops_metadata_catalog();
    let reuse = catalog.cross_consumer_reuse();
    for entry_id in &catalog.shared_reuse_entry_ids {
        let spans = reuse.get(entry_id).map(|s| s.len()).unwrap_or(0);
        assert!(
            spans >= SHARED_ENTRY_MIN_REUSE_CONSUMERS,
            "shared entry {entry_id} only spans {spans} consumers"
        );
    }
}

#[test]
fn empty_shared_reuse_fails() {
    let mut catalog = seeded_content_ops_metadata_catalog();
    catalog.shared_reuse_entry_ids.clear();
    assert!(catalog
        .validate()
        .contains(&ContentOpsViolation::SharedEntryReuseInsufficient));
}

#[test]
fn missing_source_contracts_fails() {
    let mut catalog = seeded_content_ops_metadata_catalog();
    catalog.source_contract_refs.clear();
    assert!(catalog
        .validate()
        .contains(&ContentOpsViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut catalog = seeded_content_ops_metadata_catalog();
    catalog
        .trust_review
        .captions_never_imply_live_without_metadata = false;
    assert!(catalog
        .validate()
        .contains(&ContentOpsViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut catalog = seeded_content_ops_metadata_catalog();
    catalog
        .consumer_projection
        .report_headings_pair_human_and_machine_codes = false;
    assert!(catalog
        .validate()
        .contains(&ContentOpsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut catalog = seeded_content_ops_metadata_catalog();
    catalog.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(catalog
        .validate()
        .contains(&ContentOpsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut catalog = seeded_content_ops_metadata_catalog();
    catalog.release_posture.mirror_offline_parity_required = false;
    assert!(catalog
        .validate()
        .contains(&ContentOpsViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_entries_and_placeholders() {
    let catalog = seeded_content_ops_metadata_catalog();
    let summary = catalog.render_markdown_summary();
    for entry in &catalog.entries {
        assert!(
            summary.contains(&entry.entry_id),
            "summary missing {}",
            entry.entry_id
        );
    }
    // The variable-rich docs snippet surfaces its placeholders in the summary.
    assert!(summary.contains("{count}"));
    assert!(summary.contains("{scope}"));
}

#[test]
fn localized_overlay_preserves_machine_identity() {
    let canonical = seeded_content_ops_metadata_catalog();
    let localized = seeded_content_ops_metadata_catalog_localized();
    assert!(
        localized.validate().is_empty(),
        "localized overlay failed validation: {:?}",
        localized.validate()
    );
    assert_ne!(canonical.reference_locale, localized.reference_locale);

    let canon_ids: Vec<&str> = canonical
        .entries
        .iter()
        .map(|e| e.entry_id.as_str())
        .collect();
    let loc_ids: Vec<&str> = localized
        .entries
        .iter()
        .map(|e| e.entry_id.as_str())
        .collect();
    assert_eq!(canon_ids, loc_ids);

    let mut any_prose_changed = false;
    for (canon, loc) in canonical.entries.iter().zip(localized.entries.iter()) {
        assert_eq!(canon.machine_field_name, loc.machine_field_name);
        assert_eq!(canon.command_ref, loc.command_ref);
        assert_eq!(canon.source_ref, loc.source_ref);
        assert_eq!(canon.kind, loc.kind);
        assert_eq!(canon.consumers, loc.consumers);
        assert_eq!(canon.locale_fallback, loc.locale_fallback);
        // Placeholder literals and token ids are machine identity and never localize.
        assert_eq!(
            canon
                .placeholder_notes
                .iter()
                .map(|n| (n.placeholder.as_str(), n.token_id.as_str()))
                .collect::<Vec<_>>(),
            loc.placeholder_notes
                .iter()
                .map(|n| (n.placeholder.as_str(), n.token_id.as_str()))
                .collect::<Vec<_>>()
        );
        if canon.canonical_text != loc.canonical_text {
            any_prose_changed = true;
        }
    }
    assert!(any_prose_changed, "localized overlay changed no prose");
}

#[test]
fn offline_mirror_variant_validates_and_keeps_identity() {
    let canonical = seeded_content_ops_metadata_catalog();
    let mirror = seeded_content_ops_metadata_catalog_offline_mirror();
    assert!(mirror.validate().is_empty(), "{:?}", mirror.validate());
    assert_eq!(mirror.entries, canonical.entries);
    assert_ne!(mirror.catalog_id, canonical.catalog_id);
}

#[test]
fn checked_support_export_validates() {
    let catalog = current_content_ops_metadata_catalog_export()
        .expect("checked content-ops metadata catalog export validates");
    assert_eq!(catalog.catalog_id, CONTENT_OPS_METADATA_CATALOG_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_content_ops_metadata_catalog_export()
        .expect("checked content-ops metadata catalog export validates");
    assert_eq!(
        from_disk,
        seeded_content_ops_metadata_catalog(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn checked_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/content/m5-content-ops-metadata/localized_overlay.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/content/m5-content-ops-metadata/offline_mirror.json"
        )),
    ] {
        let catalog: ContentOpsMetadataCatalog =
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
    let json = seeded_content_ops_metadata_catalog().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
}
