use super::*;

#[test]
fn seeded_catalog_validates() {
    let catalog = seeded_error_recovery_copy_catalog();
    assert!(catalog.validate().is_empty(), "{:?}", catalog.validate());
    assert_eq!(catalog.catalog_id, ERROR_RECOVERY_COPY_CATALOG_ID);
}

#[test]
fn inventories_match_canonical() {
    let catalog = seeded_error_recovery_copy_catalog();
    assert_eq!(catalog.domain_inventory.len(), FailureDomain::ALL.len());
    assert_eq!(
        catalog.severity_inventory.len(),
        RecoverySeverity::ALL.len()
    );
    assert_eq!(
        catalog.degraded_state_inventory.len(),
        DegradedState::ALL.len()
    );
    assert_eq!(
        catalog.surface_inventory.len(),
        RecoveryConsumerSurface::ALL.len()
    );
}

#[test]
fn inventory_drift_fails() {
    let mut catalog = seeded_error_recovery_copy_catalog();
    catalog.severity_inventory.pop();
    assert!(catalog
        .validate()
        .contains(&ErrorRecoveryCopyViolation::InventoryDrift));
}

#[test]
fn every_degraded_state_has_a_chip() {
    let catalog = seeded_error_recovery_copy_catalog();
    let states: std::collections::BTreeSet<_> = catalog.chips.iter().map(|c| c.state).collect();
    assert_eq!(states.len(), DegradedState::ALL.len());
    for state in DegradedState::ALL {
        assert!(states.contains(&state), "missing chip for {state:?}");
    }
}

#[test]
fn degraded_state_not_covered_fails() {
    let mut catalog = seeded_error_recovery_copy_catalog();
    catalog
        .chips
        .retain(|c| c.state != DegradedState::RollbackAvailable);
    assert!(catalog
        .validate()
        .contains(&ErrorRecoveryCopyViolation::DegradedStateNotCovered));
}

#[test]
fn chip_ids_and_tokens_are_locale_neutral() {
    let catalog = seeded_error_recovery_copy_catalog();
    let ok = |t: &str| {
        !t.is_empty()
            && t.chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '.')
    };
    for chip in &catalog.chips {
        assert!(
            ok(&chip.chip_id),
            "chip id not locale-neutral: {}",
            chip.chip_id
        );
        assert!(
            ok(&chip.machine_token),
            "machine token not locale-neutral: {}",
            chip.machine_token
        );
    }
    for block in &catalog.blocks {
        assert!(
            ok(&block.block_id),
            "block id not locale-neutral: {}",
            block.block_id
        );
        assert!(ok(&block.next_action.action_id));
        assert!(ok(&block.next_action.recovery_link.link_id));
        assert!(ok(&block.next_action.recovery_link.target_ref));
    }
}

#[test]
fn non_locale_neutral_chip_id_fails() {
    let mut catalog = seeded_error_recovery_copy_catalog();
    catalog.chips[0].chip_id = "Chip.Restricted".to_owned();
    assert!(catalog
        .validate()
        .contains(&ErrorRecoveryCopyViolation::ChipTokenNotLocaleNeutral));
}

#[test]
fn duplicate_chip_fails() {
    let mut catalog = seeded_error_recovery_copy_catalog();
    let clone = catalog.chips[0].clone();
    catalog.chips.push(clone);
    assert!(catalog
        .validate()
        .contains(&ErrorRecoveryCopyViolation::DuplicateChip));
}

#[test]
fn ungrounded_chip_fails() {
    let mut catalog = seeded_error_recovery_copy_catalog();
    catalog.chips[0].grounded = false;
    assert!(catalog
        .validate()
        .contains(&ErrorRecoveryCopyViolation::ChipNotGrounded));
}

#[test]
fn euphemistic_chip_label_fails() {
    let mut catalog = seeded_error_recovery_copy_catalog();
    catalog.chips[0].canonical_label = "Oops, restricted".to_owned();
    assert!(catalog
        .validate()
        .contains(&ErrorRecoveryCopyViolation::ChipNotGrounded));
}

#[test]
fn non_locale_neutral_block_id_fails() {
    let mut catalog = seeded_error_recovery_copy_catalog();
    catalog.blocks[0].block_id = "Recovery.Network".to_owned();
    assert!(catalog
        .validate()
        .contains(&ErrorRecoveryCopyViolation::BlockIdNotLocaleNeutral));
}

#[test]
fn recovery_block_requires_all_four_parts() {
    let mut catalog = seeded_error_recovery_copy_catalog();
    catalog.blocks[0].why_likely.reference_template = "   ".to_owned();
    assert!(catalog
        .validate()
        .contains(&ErrorRecoveryCopyViolation::RecoveryBlockMissingPart));
}

#[test]
fn copy_line_role_mismatch_fails() {
    let mut catalog = seeded_error_recovery_copy_catalog();
    catalog.blocks[0].what_failed.role = CopyRole::WhyLikely;
    assert!(catalog
        .validate()
        .contains(&ErrorRecoveryCopyViolation::CopyLineRoleMismatch));
}

#[test]
fn what_still_works_must_say_something_remains() {
    let mut catalog = seeded_error_recovery_copy_catalog();
    catalog.blocks[0].what_still_works.reference_template = "Nothing remains available.".to_owned();
    catalog.blocks[0].what_still_works.chip_refs.clear();
    assert!(catalog
        .validate()
        .contains(&ErrorRecoveryCopyViolation::WhatStillWorksMissing));
}

#[test]
fn next_action_must_be_verb_first() {
    let mut catalog = seeded_error_recovery_copy_catalog();
    catalog.blocks[0].next_action.label = "Continue now".to_owned();
    catalog.blocks[0].next_action.variables.clear();
    assert!(catalog
        .validate()
        .contains(&ErrorRecoveryCopyViolation::NextActionNotVerbFirst));
}

#[test]
fn next_action_missing_recovery_link_fails() {
    let mut catalog = seeded_error_recovery_copy_catalog();
    catalog.blocks[0].next_action.recovery_link.target_ref = String::new();
    assert!(catalog
        .validate()
        .contains(&ErrorRecoveryCopyViolation::NextActionMissingRecoveryLink));
}

#[test]
fn recovery_link_must_resolve_offline() {
    let mut catalog = seeded_error_recovery_copy_catalog();
    catalog.blocks[0]
        .next_action
        .recovery_link
        .offline_available = false;
    assert!(catalog
        .validate()
        .contains(&ErrorRecoveryCopyViolation::RecoveryLinkNotOfflineResolvable));
}

#[test]
fn unresolved_chip_ref_fails() {
    let mut catalog = seeded_error_recovery_copy_catalog();
    catalog.blocks[0]
        .what_failed
        .chip_refs
        .push("chip.does_not_exist".to_owned());
    assert!(catalog
        .validate()
        .contains(&ErrorRecoveryCopyViolation::ChipRefUnresolved));
}

#[test]
fn chip_on_disallowed_surface_fails() {
    let mut catalog = seeded_error_recovery_copy_catalog();
    // The stale chip is embedded by the network block on several surfaces; restrict
    // it to one surface and the block's reuse becomes illegal.
    let chip = catalog
        .chips
        .iter_mut()
        .find(|c| c.chip_id == "chip.stale")
        .expect("chip present");
    chip.allowed_surfaces = vec![RecoveryConsumerSurface::DynamicBanner];
    assert!(catalog
        .validate()
        .contains(&ErrorRecoveryCopyViolation::ChipUsedOnDisallowedSurface));
}

#[test]
fn template_placeholder_must_resolve() {
    let mut catalog = seeded_error_recovery_copy_catalog();
    catalog.blocks[0].what_failed.reference_template =
        "{chip:chip.remote_host} {var:undeclared_var}".to_owned();
    assert!(catalog
        .validate()
        .contains(&ErrorRecoveryCopyViolation::TemplatePlaceholderUnresolved));
}

#[test]
fn declared_but_unused_chip_fails() {
    let mut catalog = seeded_error_recovery_copy_catalog();
    // A real chip ref the template never renders.
    catalog.blocks[0]
        .what_failed
        .chip_refs
        .push("chip.cached".to_owned());
    assert!(catalog
        .validate()
        .contains(&ErrorRecoveryCopyViolation::DeclaredTokenUnused));
}

#[test]
fn playful_or_generic_tone_fails() {
    let mut catalog = seeded_error_recovery_copy_catalog();
    catalog.blocks[0].what_failed.reference_template =
        "Oops, the {chip:chip.remote_host} dropped.".to_owned();
    catalog.blocks[0].what_failed.variables.clear();
    assert!(catalog
        .validate()
        .contains(&ErrorRecoveryCopyViolation::BlockUsesGenericOrPlayfulTone));
}

#[test]
fn variable_name_must_be_locale_neutral() {
    let mut catalog = seeded_error_recovery_copy_catalog();
    catalog.blocks[0].what_failed.variables[0].name = "Host Name".to_owned();
    assert!(catalog
        .validate()
        .contains(&ErrorRecoveryCopyViolation::VariableNameNotLocaleNeutral));
}

#[test]
fn coverage_spans_every_domain_severity_and_surface() {
    let catalog = seeded_error_recovery_copy_catalog();
    let domains: std::collections::BTreeSet<_> =
        catalog.blocks.iter().map(|b| b.failure_domain).collect();
    let severities: std::collections::BTreeSet<_> =
        catalog.blocks.iter().map(|b| b.severity).collect();
    let surfaces: std::collections::BTreeSet<_> = catalog
        .blocks
        .iter()
        .flat_map(|b| b.consumer_surfaces.iter().copied())
        .collect();
    assert_eq!(domains.len(), FailureDomain::ALL.len());
    assert_eq!(severities.len(), RecoverySeverity::ALL.len());
    assert_eq!(surfaces.len(), RecoveryConsumerSurface::ALL.len());
}

#[test]
fn coverage_gap_fails() {
    let mut catalog = seeded_error_recovery_copy_catalog();
    // Drop the only docs/help block, leaving that domain uncovered.
    catalog
        .blocks
        .retain(|b| b.failure_domain != FailureDomain::DocsHelp);
    assert!(catalog
        .validate()
        .contains(&ErrorRecoveryCopyViolation::CoverageGap));
}

#[test]
fn shared_chips_reuse_across_surfaces() {
    let catalog = seeded_error_recovery_copy_catalog();
    let reuse = catalog.cross_surface_reuse();
    for chip_id in &catalog.shared_reuse_chip_ids {
        let spans = reuse.get(chip_id).map(|s| s.len()).unwrap_or(0);
        assert!(
            spans >= SHARED_CHIP_MIN_REUSE_SURFACES,
            "shared chip {chip_id} only spans {spans} surfaces"
        );
    }
}

#[test]
fn empty_shared_reuse_fails() {
    let mut catalog = seeded_error_recovery_copy_catalog();
    catalog.shared_reuse_chip_ids.clear();
    assert!(catalog
        .validate()
        .contains(&ErrorRecoveryCopyViolation::SharedChipReuseInsufficient));
}

#[test]
fn render_block_reference_resolves_chips() {
    let catalog = seeded_error_recovery_copy_catalog();
    let rendered = catalog
        .render_block_reference("recovery.network.remote_host_unreachable")
        .expect("block resolves");
    // The chip is resolved to its canonical label, not inlined.
    assert!(rendered.contains("Remote host"));
    assert!(rendered.contains("Stale"));
    assert!(rendered.contains("Reconnecting"));
    // Variables stay named slots; chip placeholders never leak.
    assert!(rendered.contains("{host_name}"));
    assert!(!rendered.contains("{chip:"));
    // The structure forces a remaining-capability and a next action.
    assert!(rendered.contains("Still works:"));
    assert!(rendered.contains("Next: Reconnect"));
    assert!(rendered.contains("Open reconnect status"));
}

#[test]
fn missing_source_contracts_fails() {
    let mut catalog = seeded_error_recovery_copy_catalog();
    catalog.source_contract_refs.clear();
    assert!(catalog
        .validate()
        .contains(&ErrorRecoveryCopyViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut catalog = seeded_error_recovery_copy_catalog();
    catalog
        .trust_review
        .recovery_messaging_states_what_still_works_and_how_to_proceed = false;
    assert!(catalog
        .validate()
        .contains(&ErrorRecoveryCopyViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut catalog = seeded_error_recovery_copy_catalog();
    catalog
        .consumer_projection
        .support_export_uses_catalog_blocks = false;
    assert!(catalog
        .validate()
        .contains(&ErrorRecoveryCopyViolation::ConsumerProjectionIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut catalog = seeded_error_recovery_copy_catalog();
    catalog.release_posture.mirror_offline_parity_required = false;
    assert!(catalog
        .validate()
        .contains(&ErrorRecoveryCopyViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_chips_and_blocks() {
    let catalog = seeded_error_recovery_copy_catalog();
    let summary = catalog.render_markdown_summary();
    for chip in &catalog.chips {
        assert!(
            summary.contains(&chip.chip_id),
            "summary missing {}",
            chip.chip_id
        );
    }
    for block in &catalog.blocks {
        assert!(
            summary.contains(&block.block_id),
            "summary missing {}",
            block.block_id
        );
    }
}

#[test]
fn localized_overlay_preserves_machine_identity() {
    let canonical = seeded_error_recovery_copy_catalog();
    let localized = seeded_error_recovery_copy_catalog_localized();
    assert!(
        localized.validate().is_empty(),
        "localized overlay failed validation: {:?}",
        localized.validate()
    );
    assert_ne!(canonical.reference_locale, localized.reference_locale);

    let canon_block_ids: Vec<&str> = canonical
        .blocks
        .iter()
        .map(|b| b.block_id.as_str())
        .collect();
    let loc_block_ids: Vec<&str> = localized
        .blocks
        .iter()
        .map(|b| b.block_id.as_str())
        .collect();
    assert_eq!(canon_block_ids, loc_block_ids);

    let canon_chip_ids: Vec<&str> = canonical.chips.iter().map(|c| c.chip_id.as_str()).collect();
    let loc_chip_ids: Vec<&str> = localized.chips.iter().map(|c| c.chip_id.as_str()).collect();
    assert_eq!(canon_chip_ids, loc_chip_ids);

    // Prose differs but placeholders (machine tokens) and chip refs are identical.
    let mut any_prose_changed = false;
    for (canon, loc) in canonical.blocks.iter().zip(localized.blocks.iter()) {
        for (cl, ll) in canon.lines().iter().zip(loc.lines().iter()) {
            assert_eq!(cl.chip_refs, ll.chip_refs);
            assert_eq!(
                placeholders(&cl.reference_template),
                placeholders(&ll.reference_template),
                "placeholders drifted for {}",
                canon.block_id
            );
            if cl.reference_template != ll.reference_template {
                any_prose_changed = true;
            }
        }
        // The verb-first label and its variables never move.
        assert_eq!(canon.next_action.label, loc.next_action.label);
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
    let canonical = seeded_error_recovery_copy_catalog();
    let mirror = seeded_error_recovery_copy_catalog_offline_mirror();
    assert!(mirror.validate().is_empty(), "{:?}", mirror.validate());
    assert_eq!(mirror.blocks, canonical.blocks);
    assert_eq!(mirror.chips, canonical.chips);
    assert_ne!(mirror.catalog_id, canonical.catalog_id);
}

#[test]
fn checked_support_export_validates() {
    let catalog = current_error_recovery_copy_catalog_export()
        .expect("checked error/recovery copy catalog export validates");
    assert_eq!(catalog.catalog_id, ERROR_RECOVERY_COPY_CATALOG_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_error_recovery_copy_catalog_export()
        .expect("checked error/recovery copy catalog export validates");
    assert_eq!(
        from_disk,
        seeded_error_recovery_copy_catalog(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn checked_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/content/m5-error-recovery-copy/localized_overlay.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/content/m5-error-recovery-copy/offline_mirror.json"
        )),
    ] {
        let catalog: ErrorRecoveryCopyCatalog =
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
    let json = seeded_error_recovery_copy_catalog().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
}
