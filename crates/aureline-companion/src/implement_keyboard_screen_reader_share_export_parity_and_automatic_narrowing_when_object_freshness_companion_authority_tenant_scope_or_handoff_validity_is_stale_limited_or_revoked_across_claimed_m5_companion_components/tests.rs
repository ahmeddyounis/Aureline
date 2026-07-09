//! Tests for the M05-1002 companion component accessibility parity capstone: the honest
//! auto-narrowing logic, the per-family parity contract, the stale/limited/revoked never-live
//! guarantee, no-loss object-lineage integrity, and the checked-in support export / CSV / report.

use super::*;

fn row(id: &str) -> CompanionComponentAccessibilityRow {
    seeded_m5_companion_component_a11y_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_companion_component_a11y_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified() {
    let packet = seeded_m5_companion_component_a11y_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5CompanionComponentFamily::ALL.len()
    );
    // Six rows: one per frozen family.
    assert_eq!(packet.rows.len(), 6);
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_companion_component_a11y_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5CompanionComponentClaimDimension::ALL.len()
    );
}

#[test]
fn every_condition_state_is_exercised() {
    let packet = seeded_m5_companion_component_a11y_packet();
    let states = packet.exercised_condition_states();
    for state in M5CompanionComponentConditionState::ALL {
        assert!(
            states.contains(&state),
            "condition state {} is not exercised",
            state.as_str()
        );
    }
}

#[test]
fn every_companion_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_companion_component_a11y_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5CompanionComponentClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_companion_component_a11y_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5CompanionConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_two_green_four_yellow_zero_red() {
    let packet = seeded_m5_companion_component_a11y_packet();
    assert_eq!(packet.summary.green_count, 2);
    assert_eq!(packet.summary.yellow_count, 4);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary.row_count, 6);
    assert_eq!(
        packet.summary.family_count,
        M5CompanionComponentFamily::ALL.len()
    );
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn summary_reports_all_lineage_and_live_safety_honesty() {
    let packet = seeded_m5_companion_component_a11y_packet();
    assert!(packet.summary.all_lineage_preserved);
    assert!(packet.summary.all_live_safety_holds);
}

// --- AC1: stale-freshness / limited-authority / narrowed-tenant / revoked-handoff can no longer keep a live label ---

#[test]
fn incident_snapshot_card_is_cached_continuity_safe_and_green() {
    let incident = row("a11y:incident-snapshot-card");
    assert_eq!(
        incident.effective_claim(),
        M5CompanionComponentClaim::CachedContinuitySafe
    );
    assert!(incident.claim_narrow.is_none());
    assert_eq!(
        incident.status(),
        CompanionComponentAccessibilityStatus::Parity
    );
    assert!(incident.effective_claim().asserts_full_projection());
    assert!(!incident.effective_claim().asserts_live_companion_safe());
}

#[test]
fn ci_status_card_is_live_companion_safe_and_green() {
    let card = row("a11y:ci-status-card");
    assert_eq!(
        card.full_companion_claim,
        M5CompanionComponentClaim::LiveCompanionSafe
    );
    assert_eq!(
        card.effective_claim(),
        M5CompanionComponentClaim::LiveCompanionSafe
    );
    assert!(card.claim_narrow.is_none());
    assert_eq!(card.status(), CompanionComponentAccessibilityStatus::Parity);
    assert!(card.effective_claim().asserts_live_companion_safe());
}

#[test]
fn freshness_stale_narrows_to_stale_freshness_projection() {
    let item = row("a11y:notification-row-freshness-stale");
    assert_eq!(
        item.effective_claim(),
        M5CompanionComponentClaim::StaleFreshnessProjection
    );
    assert!(!item.effective_claim().asserts_live_companion_safe());
    let narrow = item.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(narrow.trigger, M5CompanionDowngradeTrigger::FreshnessHidden);
    assert_eq!(
        narrow.binding_dimension,
        M5CompanionComponentClaimDimension::ObjectFreshness
    );
    assert!(item.claim_is_honest());
    // A stale state must never be shown as live-companion-safe.
    assert!(item.live_safety_holds());
}

#[test]
fn authority_limited_narrows_to_limited_authority_projection() {
    let card = row("a11y:mobile-review-card-authority-limited");
    assert_eq!(
        card.effective_claim(),
        M5CompanionComponentClaim::LimitedAuthorityProjection
    );
    let narrow = card.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5CompanionDowngradeTrigger::CapabilityBoundaryUnstated
    );
    assert!(card.claim_is_honest());
    assert!(card.live_safety_holds());
}

#[test]
fn tenant_scope_narrowed_narrows_to_narrowed_tenant_projection() {
    let tile = row("a11y:session-follow-tile-tenant-narrowed");
    assert_eq!(
        tile.effective_claim(),
        M5CompanionComponentClaim::NarrowedTenantProjection
    );
    assert!(!tile.effective_claim().asserts_full_projection());
    let narrow = tile.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5CompanionDowngradeTrigger::ClientScopeUnstated
    );
    assert!(tile.claim_is_honest());
    assert!(tile.live_safety_holds());
}

#[test]
fn handoff_revoked_narrows_to_revoked_handoff_projection() {
    let sheet = row("a11y:desktop-handoff-sheet-handoff-revoked");
    assert_eq!(
        sheet.effective_claim(),
        M5CompanionComponentClaim::RevokedHandoffProjection
    );
    assert!(!sheet.effective_claim().asserts_live_companion_safe());
    let narrow = sheet.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5CompanionDowngradeTrigger::HandoffTargetUnresolved
    );
    assert!(sheet.claim_is_honest());
    assert!(sheet.live_safety_holds());
}

#[test]
fn over_asserting_control_is_rejected() {
    // Force the effective claim above the permitted ceiling: a stale-freshness row claiming
    // LiveCompanionSafe.
    let mut item = row("a11y:notification-row-freshness-stale");
    item.claim_narrow = None;
    assert!(!item.claim_is_honest());
    assert_eq!(
        item.status(),
        CompanionComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn stale_shown_as_live_and_safe_is_rejected() {
    // A stale-freshness row whose narrow claims LiveCompanionSafe violates live-safety honesty.
    let mut item = row("a11y:notification-row-freshness-stale");
    if let Some(narrow) = item.claim_narrow.as_mut() {
        narrow.narrowed_to = M5CompanionComponentClaim::LiveCompanionSafe;
    }
    assert!(!item.live_safety_holds());
    let violations = {
        let mut packet = seeded_m5_companion_component_a11y_packet();
        packet.rows[0] = item;
        packet.validate()
    };
    assert!(violations.iter().any(|v| matches!(
        v,
        CompanionComponentAccessibilityViolation::StaleShownAsLiveAndSafe { .. }
    )));
}

#[test]
fn live_safety_unproven_when_no_stale_limited_or_revoked_row() {
    let mut packet = seeded_m5_companion_component_a11y_packet();
    // Drop the three stale/limited/revoked rows (freshness stale + authority limited + handoff
    // revoked). The narrowed-tenant row is intentionally not in that set.
    packet.rows.retain(|r| {
        r.row_id != "a11y:notification-row-freshness-stale"
            && r.row_id != "a11y:mobile-review-card-authority-limited"
            && r.row_id != "a11y:desktop-handoff-sheet-handoff-revoked"
    });
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        CompanionComponentAccessibilityViolation::LiveSafetyHonestyUnproven
    )));
}

#[test]
fn spurious_narrow_on_live_row_is_rejected() {
    let mut card = row("a11y:ci-status-card");
    card.claim_narrow = Some(CompanionComponentClaimAutoNarrow {
        narrowed_to: M5CompanionComponentClaim::RevokedHandoffProjection,
        binding_dimension: M5CompanionComponentClaimDimension::HandoffValidity,
        trigger: M5CompanionDowngradeTrigger::HandoffTargetUnresolved,
        narrowed_label: "spurious".to_owned(),
        preserves_canonical_identity: true,
        preserves_lineage_continuity: true,
    });
    assert!(!card.claim_is_honest());
}

#[test]
fn narrow_must_bind_the_ceiling_imposing_dimension() {
    let mut item = row("a11y:notification-row-freshness-stale");
    if let Some(narrow) = item.claim_narrow.as_mut() {
        narrow.binding_dimension = M5CompanionComponentClaimDimension::HandoffValidity;
    }
    assert!(!item.claim_is_honest());
}

#[test]
fn narrow_trigger_must_match_binding_condition() {
    let mut item = row("a11y:notification-row-freshness-stale");
    if let Some(narrow) = item.claim_narrow.as_mut() {
        narrow.trigger = M5CompanionDowngradeTrigger::SeverityUnstated;
    }
    assert!(!item.claim_is_honest());
}

#[test]
fn narrowed_label_must_not_be_generic() {
    let mut item = row("a11y:notification-row-freshness-stale");
    if let Some(narrow) = item.claim_narrow.as_mut() {
        narrow.narrowed_label = "stale".to_owned();
    }
    assert!(!item.claim_is_honest());
}

#[test]
fn permitted_ceilings_map_condition_states_one_to_one() {
    use M5CompanionComponentClaim as S;
    use M5CompanionComponentConditionState as C;
    assert_eq!(C::LiveInScope.permitted_ceiling(), S::LiveCompanionSafe);
    assert_eq!(
        C::FreshnessStale.permitted_ceiling(),
        S::StaleFreshnessProjection
    );
    assert_eq!(
        C::AuthorityLimited.permitted_ceiling(),
        S::LimitedAuthorityProjection
    );
    assert_eq!(
        C::TenantScopeNarrowed.permitted_ceiling(),
        S::NarrowedTenantProjection
    );
    assert_eq!(
        C::HandoffRevoked.permitted_ceiling(),
        S::RevokedHandoffProjection
    );
}

#[test]
fn condition_triggers_map_to_frozen_matrix_vocabulary() {
    use M5CompanionComponentConditionState as C;
    use M5CompanionDowngradeTrigger as T;
    assert_eq!(C::FreshnessStale.default_trigger(), T::FreshnessHidden);
    assert_eq!(
        C::AuthorityLimited.default_trigger(),
        T::CapabilityBoundaryUnstated
    );
    assert_eq!(
        C::TenantScopeNarrowed.default_trigger(),
        T::ClientScopeUnstated
    );
    assert_eq!(
        C::HandoffRevoked.default_trigger(),
        T::HandoffTargetUnresolved
    );
}

#[test]
fn stale_limited_or_revoked_states_are_flagged() {
    use M5CompanionComponentConditionState as C;
    assert!(C::FreshnessStale.is_stale_limited_or_revoked());
    assert!(C::AuthorityLimited.is_stale_limited_or_revoked());
    assert!(C::HandoffRevoked.is_stale_limited_or_revoked());
    assert!(!C::TenantScopeNarrowed.is_stale_limited_or_revoked());
    assert!(!C::LiveInScope.is_stale_limited_or_revoked());
}

// --- AC2: accessibility / CLI / export reach the same canonical truth + no-loss ---

#[test]
fn every_row_reaches_canonical_truth_via_at() {
    for r in seeded_m5_companion_component_a11y_packet().rows {
        assert!(
            r.reaches_canonical_truth_via_at(),
            "row {} strands assistive tech",
            r.row_id
        );
    }
}

#[test]
fn hierarchy_heavy_incident_card_binds_a_non_visual_fallback() {
    let incident = row("a11y:incident-snapshot-card");
    assert!(incident.is_hierarchy_heavy());
    assert!(incident.has_non_visual_fallback());
    assert!(incident
        .fallback_modalities
        .contains(&M5CompanionComponentFallbackModality::Structured));
}

#[test]
fn view_only_trap_strands_and_reds_a_row() {
    let mut card = row("a11y:ci-status-card");
    card.keyboard_reach = CompanionComponentNonVisualReachState::ViewOnlyTrap;
    assert!(!card.reaches_canonical_truth_via_at());
    assert_eq!(
        card.status(),
        CompanionComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn empty_object_context_ref_strands_a_row() {
    let mut card = row("a11y:ci-status-card");
    card.object_context_ref = "  ".to_owned();
    assert!(!card.reaches_canonical_truth_via_at());
}

#[test]
fn export_needing_screenshot_is_rejected() {
    let mut card = row("a11y:ci-status-card");
    card.export_summary = CompanionComponentExportSummaryState::AbsentNeedsScreenshot;
    assert!(!card.export_preserves_meaning());
    assert_eq!(
        card.status(),
        CompanionComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn copy_export_requires_text_json_markdown() {
    let mut card = row("a11y:ci-status-card");
    card.copy_export.formats.retain(|f| f != "markdown");
    assert!(!card.export_preserves_meaning());
}

#[test]
fn dropped_lineage_strands_a_row() {
    let mut item = row("a11y:notification-row-freshness-stale");
    item.lineage_preserved = false;
    assert!(!item.preserves_lineage_continuity());
    assert_eq!(
        item.status(),
        CompanionComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn narrow_dropping_lineage_continuity_strands_a_row() {
    let mut item = row("a11y:notification-row-freshness-stale");
    if let Some(narrow) = item.claim_narrow.as_mut() {
        narrow.preserves_lineage_continuity = false;
    }
    assert!(!item.preserves_lineage_continuity());
    assert!(!item.claim_is_honest());
}

// --- AC3: narrowing disclosed across every narrower surface ---

#[test]
fn narrowed_surface_without_disclosure_is_rejected() {
    let mut item = row("a11y:notification-row-freshness-stale");
    item.narrowing_disclosures.clear();
    assert!(!item.narrowing_disclosed());
}

#[test]
fn silently_dropped_surface_is_rejected() {
    let mut item = row("a11y:notification-row-freshness-stale");
    item.narrowing_disclosures[0].state =
        CompanionComponentNarrowingDisclosureState::SilentlyDropped;
    assert!(!item.narrowing_disclosed());
}

#[test]
fn narrowed_surface_must_preserve_labels() {
    let mut item = row("a11y:notification-row-freshness-stale");
    item.narrowing_disclosures[0].preserved_labels.clear();
    assert!(!item.narrowing_disclosed());
}

#[test]
fn dropping_a_mandatory_label_strands_a_row() {
    let mut card = row("a11y:ci-status-card");
    card.required_labels
        .retain(|l| *l != M5CompanionRequiredLabel::Identity);
    assert!(!card.preserves_mandatory_labels());
    assert_eq!(
        card.status(),
        CompanionComponentAccessibilityStatus::Stranded
    );
}

// --- packet-level validation ---

#[test]
fn missing_family_coverage_is_flagged() {
    let mut packet = seeded_m5_companion_component_a11y_packet();
    packet.rows.retain(|r| r.row_id != "a11y:ci-status-card");
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        CompanionComponentAccessibilityViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn single_consumer_surface_fails_parity() {
    let mut packet = seeded_m5_companion_component_a11y_packet();
    packet.rows[0].consumer_surfaces = vec![M5CompanionConsumerSurface::SupportExport];
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        CompanionComponentAccessibilityViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut packet = seeded_m5_companion_component_a11y_packet();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        CompanionComponentAccessibilityViolation::DuplicateId { .. }
    )));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut packet = seeded_m5_companion_component_a11y_packet();
    packet.summary.green_count += 1;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, CompanionComponentAccessibilityViolation::SummaryMismatch)));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut packet = seeded_m5_companion_component_a11y_packet();
    packet.rows[0]
        .source_refs
        .push("password=hunter2".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        CompanionComponentAccessibilityViolation::RawCompanionMaterialInExport
    )));
}

// --- rendering ---

#[test]
fn chip_tokens_render_stable_fields() {
    let chip = row("a11y:notification-row-freshness-stale").chip_tokens();
    assert!(chip.contains("family=notification_row"));
    assert!(chip.contains("effective_claim=stale_freshness_projection"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_companion_component_a11y_packet();
    assert_eq!(packet.record_kind, COMPANION_COMPONENT_A11Y_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        COMPANION_COMPONENT_A11Y_SCHEMA_VERSION
    );
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_companion_component_a11y_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn csv_has_one_header_and_one_line_per_row() {
    let packet = seeded_m5_companion_component_a11y_packet();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), packet.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,component_family"));
}

// --- checked-in artifacts ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_m5_companion_component_a11y_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_companion_component_a11y_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_companion_component_a11y_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-companion-component-accessibility-proof/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_companion_component_a11y_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-companion-component-accessibility-proof.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var so it never
/// runs in the normal suite. Run with
/// `GEN_COMPANION_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-companion generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_COMPANION_COMPONENT_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_companion_component_a11y_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest)
        .join("../../artifacts/release/m5-companion-component-accessibility-proof");
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(
        Path::new(manifest)
            .join("../../artifacts/release/m5-companion-component-accessibility-proof.md"),
        &report,
    )
    .expect("write report");

    let fixtures =
        Path::new(manifest).join("../../fixtures/ui/m5-companion-component-accessibility-parity");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
}
