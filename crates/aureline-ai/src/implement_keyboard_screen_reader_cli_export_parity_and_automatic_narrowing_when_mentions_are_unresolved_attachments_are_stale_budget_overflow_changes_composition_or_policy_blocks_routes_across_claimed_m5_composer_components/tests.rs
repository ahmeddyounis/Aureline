//! Tests for the M05-890 prompt-composer-component accessibility fallback capstone: the
//! honest auto-narrowing logic, the per-family parity contract, draft integrity, and the
//! checked-in support export / CSV / report.

use super::*;

fn row(id: &str) -> ComposerComponentAccessibilityRow {
    seeded_m5_composer_component_a11y_fallback_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_composer_component_a11y_fallback_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified_once() {
    let packet = seeded_m5_composer_component_a11y_fallback_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5PromptComposerComponentFamily::ALL.len()
    );
    assert_eq!(
        packet.rows.len(),
        M5PromptComposerComponentFamily::ALL.len()
    );
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_composer_component_a11y_fallback_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5ComposerClaimDimension::ALL.len()
    );
}

#[test]
fn every_support_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_composer_component_a11y_fallback_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5ComposerSupportClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_composer_component_a11y_fallback_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5ComposerConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_two_green_seven_yellow_zero_red() {
    let packet = seeded_m5_composer_component_a11y_fallback_packet();
    assert_eq!(packet.summary.green_count, 2);
    assert_eq!(packet.summary.yellow_count, 7);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn summary_reports_all_drafts_preserved() {
    let packet = seeded_m5_composer_component_a11y_fallback_packet();
    assert!(packet.summary.all_drafts_preserved);
}

// --- AC1: unresolved / stale / over-budget / offline / blocked can no longer keep a send-ready label ---

#[test]
fn clear_route_header_is_send_ready_and_green() {
    let header = row("a11y:prompt-composer-header");
    assert_eq!(
        header.full_support_claim,
        M5ComposerSupportClaim::ReadyToSend
    );
    assert_eq!(
        header.effective_claim(),
        M5ComposerSupportClaim::ReadyToSend
    );
    assert!(header.claim_narrow.is_none());
    assert_eq!(
        header.status(),
        ComposerComponentAccessibilityStatus::Parity
    );
    assert!(header.effective_claim().asserts_ready_to_send());
}

#[test]
fn available_slash_command_is_reviewable_and_green() {
    let slash = row("a11y:slash-command-row");
    assert_eq!(
        slash.effective_claim(),
        M5ComposerSupportClaim::ReviewableComposition
    );
    assert!(slash.claim_narrow.is_none());
    assert_eq!(slash.status(), ComposerComponentAccessibilityStatus::Parity);
    assert!(slash.effective_claim().asserts_full_composition());
    assert!(!slash.effective_claim().asserts_ready_to_send());
}

#[test]
fn narrowed_attachment_narrows_to_narrowed_composition() {
    let pill = row("a11y:context-attachment-pill");
    assert_eq!(
        pill.effective_claim(),
        M5ComposerSupportClaim::NarrowedComposition
    );
    assert!(!pill.effective_claim().asserts_ready_to_send());
    let narrow = pill.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5ComposerDowngradeTrigger::AttachmentIdentityUnstated
    );
    assert_eq!(
        narrow.binding_dimension,
        M5ComposerClaimDimension::AttachmentTrust
    );
    assert!(pill.claim_is_honest());
}

#[test]
fn unresolved_mention_narrows_to_unresolved_composition() {
    let mention = row("a11y:mention-resolver");
    assert_eq!(
        mention.effective_claim(),
        M5ComposerSupportClaim::UnresolvedComposition
    );
    assert!(!mention.effective_claim().asserts_full_composition());
    let narrow = mention.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5ComposerDowngradeTrigger::MentionLeftUnresolved
    );
    assert!(mention.claim_is_honest());
}

#[test]
fn over_budget_strip_narrows_to_policy_blocked() {
    let budget = row("a11y:budget-size-strip");
    assert_eq!(
        budget.effective_claim(),
        M5ComposerSupportClaim::PolicyBlockedComposition
    );
    assert!(!budget.effective_claim().asserts_full_composition());
    let narrow = budget.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5ComposerDowngradeTrigger::BudgetOverrunHidden
    );
    assert!(budget.claim_is_honest());
}

#[test]
fn tainted_context_narrows_to_unresolved_composition() {
    let taint = row("a11y:tainted-context-warning");
    assert_eq!(
        taint.effective_claim(),
        M5ComposerSupportClaim::UnresolvedComposition
    );
    assert!(taint.claim_is_honest());
}

#[test]
fn offline_draft_narrows_to_local_only_composition() {
    let draft = row("a11y:draft-state-row");
    assert_eq!(
        draft.effective_claim(),
        M5ComposerSupportClaim::LocalOnlyComposition
    );
    assert!(draft.claim_is_honest());
}

#[test]
fn stale_attachment_narrows_to_local_only_composition() {
    let stale = row("a11y:attachment-stale-banner");
    assert_eq!(
        stale.effective_claim(),
        M5ComposerSupportClaim::LocalOnlyComposition
    );
    assert!(stale.claim_is_honest());
}

#[test]
fn blocked_route_send_control_narrows_to_policy_blocked() {
    let send = row("a11y:send-review-control");
    assert_eq!(
        send.effective_claim(),
        M5ComposerSupportClaim::PolicyBlockedComposition
    );
    assert!(send.claim_is_honest());
}

#[test]
fn over_asserting_control_is_rejected() {
    // Force the effective claim above the permitted ceiling: an unresolved mention resolver
    // claiming ReadyToSend.
    let mut mention = row("a11y:mention-resolver");
    mention.claim_narrow = None;
    assert!(!mention.claim_is_honest());
    assert_eq!(
        mention.status(),
        ComposerComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn spurious_narrow_on_send_ready_row_is_rejected() {
    let mut header = row("a11y:prompt-composer-header");
    header.claim_narrow = Some(ComposerClaimAutoNarrow {
        narrowed_to: M5ComposerSupportClaim::LocalOnlyComposition,
        binding_dimension: M5ComposerClaimDimension::RouteReadiness,
        trigger: M5ComposerDowngradeTrigger::RouteOrProviderMasked,
        narrowed_label: "spurious".to_owned(),
        preserves_canonical_identity: true,
        preserves_draft_integrity: true,
    });
    assert!(!header.claim_is_honest());
}

#[test]
fn narrow_must_bind_the_ceiling_imposing_dimension() {
    let mut mention = row("a11y:mention-resolver");
    if let Some(narrow) = mention.claim_narrow.as_mut() {
        narrow.binding_dimension = M5ComposerClaimDimension::RouteReadiness;
    }
    assert!(!mention.claim_is_honest());
}

#[test]
fn narrow_trigger_must_match_binding_dimension() {
    let mut mention = row("a11y:mention-resolver");
    if let Some(narrow) = mention.claim_narrow.as_mut() {
        narrow.trigger = M5ComposerDowngradeTrigger::RouteOrProviderMasked;
    }
    assert!(!mention.claim_is_honest());
}

#[test]
fn narrowed_label_must_not_be_generic() {
    let mut mention = row("a11y:mention-resolver");
    if let Some(narrow) = mention.claim_narrow.as_mut() {
        narrow.narrowed_label = "unresolved".to_owned();
    }
    assert!(!mention.claim_is_honest());
}

#[test]
fn permitted_ceilings_map_condition_states_one_to_one() {
    use M5ComposerConditionState as C;
    use M5ComposerSupportClaim as S;
    assert_eq!(C::Composed.permitted_ceiling(), S::ReadyToSend);
    assert_eq!(
        C::NarrowedInScope.permitted_ceiling(),
        S::NarrowedComposition
    );
    assert_eq!(C::LocalOnly.permitted_ceiling(), S::LocalOnlyComposition);
    assert_eq!(C::Unresolved.permitted_ceiling(), S::UnresolvedComposition);
    assert_eq!(C::Blocked.permitted_ceiling(), S::PolicyBlockedComposition);
}

#[test]
fn dimension_triggers_map_to_frozen_matrix_vocabulary() {
    use M5ComposerClaimDimension as D;
    use M5ComposerDowngradeTrigger as T;
    assert_eq!(
        D::RouteReadiness.default_trigger(),
        T::RouteOrProviderMasked
    );
    assert_eq!(
        D::AttachmentTrust.default_trigger(),
        T::AttachmentIdentityUnstated
    );
    assert_eq!(
        D::MentionResolution.default_trigger(),
        T::MentionLeftUnresolved
    );
    assert_eq!(
        D::CommandAvailability.default_trigger(),
        T::ComposerModeUnstated
    );
    assert_eq!(D::BudgetHeadroom.default_trigger(), T::BudgetOverrunHidden);
    assert_eq!(D::ContextTaint.default_trigger(), T::TaintStateHidden);
    assert_eq!(D::DraftLocality.default_trigger(), T::DraftLocalityMasked);
    assert_eq!(
        D::AttachmentFreshness.default_trigger(),
        T::AttachmentStalenessUndisclosed
    );
    assert_eq!(D::SendGate.default_trigger(), T::SendReviewGateBypassed);
}

// --- AC2: accessibility / CLI / export reach the same canonical truth + draft integrity ---

#[test]
fn every_row_reaches_canonical_truth_via_at() {
    for r in seeded_m5_composer_component_a11y_fallback_packet().rows {
        assert!(
            r.reaches_canonical_truth_via_at(),
            "row {} strands assistive tech",
            r.row_id
        );
    }
}

#[test]
fn hierarchy_heavy_budget_strip_binds_a_non_visual_fallback() {
    let budget = row("a11y:budget-size-strip");
    assert!(budget.is_hierarchy_heavy());
    assert!(budget.has_non_visual_fallback());
    assert!(budget
        .fallback_modalities
        .contains(&M5ComposerFallbackModality::Structured));
}

#[test]
fn view_only_trap_strands_and_reds_a_row() {
    let mut header = row("a11y:prompt-composer-header");
    header.keyboard_reach = ComposerNonVisualReachState::ViewOnlyTrap;
    assert!(!header.reaches_canonical_truth_via_at());
    assert_eq!(
        header.status(),
        ComposerComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn empty_composer_context_ref_strands_a_row() {
    let mut header = row("a11y:prompt-composer-header");
    header.composer_context_ref = "  ".to_owned();
    assert!(!header.reaches_canonical_truth_via_at());
}

#[test]
fn export_needing_screenshot_is_rejected() {
    let mut header = row("a11y:prompt-composer-header");
    header.export_summary = ComposerExportSummaryState::AbsentNeedsScreenshot;
    assert!(!header.export_preserves_meaning());
    assert_eq!(
        header.status(),
        ComposerComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn copy_export_requires_text_json_markdown() {
    let mut header = row("a11y:prompt-composer-header");
    header.copy_export.formats.retain(|f| f != "markdown");
    assert!(!header.export_preserves_meaning());
}

#[test]
fn dropped_draft_strands_a_row() {
    let mut draft = row("a11y:draft-state-row");
    draft.draft_preserved = false;
    assert!(!draft.preserves_draft_integrity());
    assert_eq!(
        draft.status(),
        ComposerComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn narrow_dropping_draft_integrity_strands_a_row() {
    let mut draft = row("a11y:draft-state-row");
    if let Some(narrow) = draft.claim_narrow.as_mut() {
        narrow.preserves_draft_integrity = false;
    }
    assert!(!draft.preserves_draft_integrity());
    assert!(!draft.claim_is_honest());
}

// --- AC3: narrowing disclosed across every narrower surface ---

#[test]
fn narrowed_surface_without_disclosure_is_rejected() {
    let mut mention = row("a11y:mention-resolver");
    mention.narrowing_disclosures.clear();
    assert!(!mention.narrowing_disclosed());
}

#[test]
fn silently_dropped_surface_is_rejected() {
    let mut mention = row("a11y:mention-resolver");
    mention.narrowing_disclosures[0].state = ComposerNarrowingDisclosureState::SilentlyDropped;
    assert!(!mention.narrowing_disclosed());
}

#[test]
fn narrowed_surface_must_preserve_labels() {
    let mut mention = row("a11y:mention-resolver");
    mention.narrowing_disclosures[0].preserved_labels.clear();
    assert!(!mention.narrowing_disclosed());
}

#[test]
fn dropping_a_mandatory_label_strands_a_row() {
    let mut header = row("a11y:prompt-composer-header");
    header
        .required_labels
        .retain(|l| *l != M5ComposerRequiredLabel::Identity);
    assert!(!header.preserves_mandatory_labels());
    assert_eq!(
        header.status(),
        ComposerComponentAccessibilityStatus::Stranded
    );
}

// --- packet-level validation ---

#[test]
fn missing_family_coverage_is_flagged() {
    let mut packet = seeded_m5_composer_component_a11y_fallback_packet();
    packet.rows.retain(|r| r.row_id != "a11y:slash-command-row");
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ComposerComponentAccessibilityViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn single_consumer_surface_fails_parity() {
    let mut packet = seeded_m5_composer_component_a11y_fallback_packet();
    packet.rows[0].consumer_surfaces = vec![M5ComposerConsumerSurface::SupportExport];
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ComposerComponentAccessibilityViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut packet = seeded_m5_composer_component_a11y_fallback_packet();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ComposerComponentAccessibilityViolation::DuplicateId { .. }
    )));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut packet = seeded_m5_composer_component_a11y_fallback_packet();
    packet.summary.green_count += 1;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, ComposerComponentAccessibilityViolation::SummaryMismatch)));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut packet = seeded_m5_composer_component_a11y_fallback_packet();
    packet.rows[0]
        .source_refs
        .push("api_key=hunter2".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ComposerComponentAccessibilityViolation::RawComposerMaterialInExport
    )));
}

// --- rendering ---

#[test]
fn chip_tokens_render_stable_fields() {
    let chip = row("a11y:mention-resolver").chip_tokens();
    assert!(chip.contains("family=mention_resolver"));
    assert!(chip.contains("effective_claim=unresolved_composition"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_composer_component_a11y_fallback_packet();
    assert_eq!(
        packet.record_kind,
        COMPOSER_COMPONENT_A11Y_FALLBACK_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        COMPOSER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION
    );
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_composer_component_a11y_fallback_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn csv_has_one_header_and_one_line_per_row() {
    let packet = seeded_m5_composer_component_a11y_fallback_packet();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), packet.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,component_family"));
}

// --- checked-in artifacts ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_m5_composer_component_a11y_fallback_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_composer_component_a11y_fallback_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_composer_component_a11y_fallback_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/m5-prompt-composer-component-accessibility-fallback/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_composer_component_a11y_fallback_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/m5-prompt-composer-component-accessibility-fallback.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var so it
/// never runs in the normal suite. Run with
/// `GEN_COMPOSER_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-ai generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_COMPOSER_COMPONENT_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_composer_component_a11y_fallback_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest)
        .join("../../artifacts/ai/m5/m5-prompt-composer-component-accessibility-fallback");
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(
        Path::new(manifest)
            .join("../../artifacts/ai/m5/m5-prompt-composer-component-accessibility-fallback.md"),
        &report,
    )
    .expect("write report");

    let fixtures = Path::new(manifest)
        .join("../../fixtures/ai/m5/m5-prompt-composer-component-accessibility-fallback");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
    fs::write(
        fixtures.join("README.md"),
        "# M5 prompt-composer-component accessibility fallback fixtures\n\n\
         Mirror of `artifacts/ai/m5/m5-prompt-composer-component-accessibility-fallback/`.\n\
         Regenerate with `GEN_COMPOSER_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-ai generate_artifacts`.\n",
    )
    .expect("write fixture readme");
}
