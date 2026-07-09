//! Tests for the M05-986 work-item component accessibility parity capstone: the honest
//! auto-narrowing logic, the per-family parity contract, the cached-or-offline
//! never-committed guarantee, no-loss work-item-lineage integrity, and the checked-in support
//! export / CSV / report.

use super::*;

fn row(id: &str) -> WorkItemComponentAccessibilityRow {
    seeded_m5_work_item_component_a11y_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_work_item_component_a11y_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified() {
    let packet = seeded_m5_work_item_component_a11y_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5WorkItemComponentFamily::ALL.len()
    );
    // Eight rows: one per frozen family.
    assert_eq!(packet.rows.len(), 8);
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_work_item_component_a11y_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5WorkItemComponentClaimDimension::ALL.len()
    );
}

#[test]
fn every_condition_state_is_exercised() {
    let packet = seeded_m5_work_item_component_a11y_packet();
    let states = packet.exercised_condition_states();
    for state in M5WorkItemComponentConditionState::ALL {
        assert!(
            states.contains(&state),
            "condition state {} is not exercised",
            state.as_str()
        );
    }
}

#[test]
fn every_provider_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_work_item_component_a11y_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5WorkItemComponentClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_work_item_component_a11y_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5WorkItemConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_four_green_four_yellow_zero_red() {
    let packet = seeded_m5_work_item_component_a11y_packet();
    assert_eq!(packet.summary.green_count, 4);
    assert_eq!(packet.summary.yellow_count, 4);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary.row_count, 8);
    assert_eq!(
        packet.summary.family_count,
        M5WorkItemComponentFamily::ALL.len()
    );
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn summary_reports_all_lineage_and_commit_honesty() {
    let packet = seeded_m5_work_item_component_a11y_packet();
    assert!(packet.summary.all_lineage_preserved);
    assert!(packet.summary.all_commit_honesty_holds);
}

// --- AC1: stale-freshness / read-only / local-only / unpublishable-packet can no longer keep a committed label ---

#[test]
fn relation_strip_is_reviewable_and_green() {
    let relation = row("a11y:relation-strip");
    assert_eq!(
        relation.effective_claim(),
        M5WorkItemComponentClaim::ReviewableProjection
    );
    assert!(relation.claim_narrow.is_none());
    assert_eq!(
        relation.status(),
        WorkItemComponentAccessibilityStatus::Parity
    );
    assert!(relation.effective_claim().asserts_full_projection());
    assert!(!relation.effective_claim().asserts_provider_committed());
}

#[test]
fn detail_header_is_provider_committed_and_green() {
    let header = row("a11y:work-item-detail-header");
    assert_eq!(
        header.full_provider_claim,
        M5WorkItemComponentClaim::ProviderCommitted
    );
    assert_eq!(
        header.effective_claim(),
        M5WorkItemComponentClaim::ProviderCommitted
    );
    assert!(header.claim_narrow.is_none());
    assert_eq!(
        header.status(),
        WorkItemComponentAccessibilityStatus::Parity
    );
    assert!(header.effective_claim().asserts_provider_committed());
}

#[test]
fn stale_freshness_narrows_to_stale_freshness_projection() {
    let item = row("a11y:work-item-row-freshness-stale");
    assert_eq!(
        item.effective_claim(),
        M5WorkItemComponentClaim::StaleFreshnessProjection
    );
    assert!(!item.effective_claim().asserts_provider_committed());
    let narrow = item.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5WorkItemDowngradeTrigger::LocalVersusProviderStateHidden
    );
    assert_eq!(
        narrow.binding_dimension,
        M5WorkItemComponentClaimDimension::ProviderFreshness
    );
    assert!(item.claim_is_honest());
    // A stale (cached) state must never be shown as committed.
    assert!(item.commit_honesty_holds());
}

#[test]
fn write_scope_blocked_narrows_to_read_only_projection() {
    let chip = row("a11y:provider-chip-group-write-scope-blocked");
    assert_eq!(
        chip.effective_claim(),
        M5WorkItemComponentClaim::ReadOnlyProjection
    );
    let narrow = chip.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5WorkItemDowngradeTrigger::ProviderAuthorityUnstated
    );
    assert!(chip.claim_is_honest());
}

#[test]
fn sync_local_only_narrows_to_local_only_projection() {
    let pill = row("a11y:sync-pending-pill-local-only");
    assert_eq!(
        pill.effective_claim(),
        M5WorkItemComponentClaim::LocalOnlyProjection
    );
    assert!(!pill.effective_claim().asserts_provider_committed());
    let narrow = pill.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5WorkItemDowngradeTrigger::SyncPendingStateHidden
    );
    assert!(pill.claim_is_honest());
    assert!(pill.commit_honesty_holds());
}

#[test]
fn unpublishable_packet_narrows_to_unpublishable_packet_projection() {
    let packet_card = row("a11y:offline-handoff-packet-card-unpublishable");
    assert_eq!(
        packet_card.effective_claim(),
        M5WorkItemComponentClaim::UnpublishablePacketProjection
    );
    assert!(!packet_card.effective_claim().asserts_full_projection());
    let narrow = packet_card.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5WorkItemDowngradeTrigger::PublishLaterContinuityHidden
    );
    assert!(packet_card.claim_is_honest());
    assert!(packet_card.commit_honesty_holds());
}

#[test]
fn over_asserting_control_is_rejected() {
    // Force the effective claim above the permitted ceiling: a stale-freshness row claiming
    // ProviderCommitted.
    let mut item = row("a11y:work-item-row-freshness-stale");
    item.claim_narrow = None;
    assert!(!item.claim_is_honest());
    assert_eq!(
        item.status(),
        WorkItemComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn cached_or_offline_shown_as_committed_is_rejected() {
    // A stale-freshness row whose narrow claims ProviderCommitted violates commit honesty.
    let mut item = row("a11y:work-item-row-freshness-stale");
    if let Some(narrow) = item.claim_narrow.as_mut() {
        narrow.narrowed_to = M5WorkItemComponentClaim::ProviderCommitted;
    }
    assert!(!item.commit_honesty_holds());
    let violations = {
        let mut packet = seeded_m5_work_item_component_a11y_packet();
        packet.rows[0] = item;
        packet.validate()
    };
    assert!(violations.iter().any(|v| matches!(
        v,
        WorkItemComponentAccessibilityViolation::CachedOrOfflineShownAsCommitted { .. }
    )));
}

#[test]
fn commit_honesty_unproven_when_no_cached_or_offline_row() {
    let mut packet = seeded_m5_work_item_component_a11y_packet();
    // Drop the three cached/offline rows (stale freshness + local-only sync + unpublishable packet).
    packet.rows.retain(|r| {
        r.row_id != "a11y:work-item-row-freshness-stale"
            && r.row_id != "a11y:sync-pending-pill-local-only"
            && r.row_id != "a11y:offline-handoff-packet-card-unpublishable"
    });
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        WorkItemComponentAccessibilityViolation::CommitHonestyUnproven
    )));
}

#[test]
fn spurious_narrow_on_committed_row_is_rejected() {
    let mut header = row("a11y:work-item-detail-header");
    header.claim_narrow = Some(WorkItemComponentClaimAutoNarrow {
        narrowed_to: M5WorkItemComponentClaim::LocalOnlyProjection,
        binding_dimension: M5WorkItemComponentClaimDimension::SyncState,
        trigger: M5WorkItemDowngradeTrigger::SyncPendingStateHidden,
        narrowed_label: "spurious".to_owned(),
        preserves_canonical_identity: true,
        preserves_lineage_continuity: true,
    });
    assert!(!header.claim_is_honest());
}

#[test]
fn narrow_must_bind_the_ceiling_imposing_dimension() {
    let mut item = row("a11y:work-item-row-freshness-stale");
    if let Some(narrow) = item.claim_narrow.as_mut() {
        narrow.binding_dimension = M5WorkItemComponentClaimDimension::PacketPublishability;
    }
    assert!(!item.claim_is_honest());
}

#[test]
fn narrow_trigger_must_match_binding_condition() {
    let mut item = row("a11y:work-item-row-freshness-stale");
    if let Some(narrow) = item.claim_narrow.as_mut() {
        narrow.trigger = M5WorkItemDowngradeTrigger::ExportBoundaryHidden;
    }
    assert!(!item.claim_is_honest());
}

#[test]
fn narrowed_label_must_not_be_generic() {
    let mut item = row("a11y:work-item-row-freshness-stale");
    if let Some(narrow) = item.claim_narrow.as_mut() {
        narrow.narrowed_label = "stale".to_owned();
    }
    assert!(!item.claim_is_honest());
}

#[test]
fn permitted_ceilings_map_condition_states_one_to_one() {
    use M5WorkItemComponentClaim as S;
    use M5WorkItemComponentConditionState as C;
    assert_eq!(C::FreshCommitted.permitted_ceiling(), S::ProviderCommitted);
    assert_eq!(
        C::FreshnessStale.permitted_ceiling(),
        S::StaleFreshnessProjection
    );
    assert_eq!(
        C::WriteScopeBlocked.permitted_ceiling(),
        S::ReadOnlyProjection
    );
    assert_eq!(C::SyncLocalOnly.permitted_ceiling(), S::LocalOnlyProjection);
    assert_eq!(
        C::PacketUnpublishable.permitted_ceiling(),
        S::UnpublishablePacketProjection
    );
}

#[test]
fn condition_triggers_map_to_frozen_matrix_vocabulary() {
    use M5WorkItemComponentConditionState as C;
    use M5WorkItemDowngradeTrigger as T;
    assert_eq!(
        C::FreshnessStale.default_trigger(),
        T::LocalVersusProviderStateHidden
    );
    assert_eq!(
        C::WriteScopeBlocked.default_trigger(),
        T::ProviderAuthorityUnstated
    );
    assert_eq!(
        C::SyncLocalOnly.default_trigger(),
        T::SyncPendingStateHidden
    );
    assert_eq!(
        C::PacketUnpublishable.default_trigger(),
        T::PublishLaterContinuityHidden
    );
}

#[test]
fn cached_or_offline_states_are_flagged() {
    use M5WorkItemComponentConditionState as C;
    assert!(C::FreshnessStale.is_cached_or_offline());
    assert!(C::SyncLocalOnly.is_cached_or_offline());
    assert!(C::PacketUnpublishable.is_cached_or_offline());
    assert!(!C::WriteScopeBlocked.is_cached_or_offline());
    assert!(!C::FreshCommitted.is_cached_or_offline());
}

// --- AC2: accessibility / CLI / export reach the same canonical truth + no-loss ---

#[test]
fn every_row_reaches_canonical_truth_via_at() {
    for r in seeded_m5_work_item_component_a11y_packet().rows {
        assert!(
            r.reaches_canonical_truth_via_at(),
            "row {} strands assistive tech",
            r.row_id
        );
    }
}

#[test]
fn hierarchy_heavy_offline_packet_binds_a_non_visual_fallback() {
    let packet_card = row("a11y:offline-handoff-packet-card-unpublishable");
    assert!(packet_card.is_hierarchy_heavy());
    assert!(packet_card.has_non_visual_fallback());
    assert!(packet_card
        .fallback_modalities
        .contains(&M5WorkItemComponentFallbackModality::Structured));
}

#[test]
fn view_only_trap_strands_and_reds_a_row() {
    let mut header = row("a11y:work-item-detail-header");
    header.keyboard_reach = WorkItemComponentNonVisualReachState::ViewOnlyTrap;
    assert!(!header.reaches_canonical_truth_via_at());
    assert_eq!(
        header.status(),
        WorkItemComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn empty_work_item_context_ref_strands_a_row() {
    let mut header = row("a11y:work-item-detail-header");
    header.work_item_context_ref = "  ".to_owned();
    assert!(!header.reaches_canonical_truth_via_at());
}

#[test]
fn export_needing_screenshot_is_rejected() {
    let mut header = row("a11y:work-item-detail-header");
    header.export_summary = WorkItemComponentExportSummaryState::AbsentNeedsScreenshot;
    assert!(!header.export_preserves_meaning());
    assert_eq!(
        header.status(),
        WorkItemComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn copy_export_requires_text_json_markdown() {
    let mut header = row("a11y:work-item-detail-header");
    header.copy_export.formats.retain(|f| f != "markdown");
    assert!(!header.export_preserves_meaning());
}

#[test]
fn dropped_lineage_strands_a_row() {
    let mut item = row("a11y:work-item-row-freshness-stale");
    item.lineage_preserved = false;
    assert!(!item.preserves_lineage_continuity());
    assert_eq!(
        item.status(),
        WorkItemComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn narrow_dropping_lineage_continuity_strands_a_row() {
    let mut item = row("a11y:work-item-row-freshness-stale");
    if let Some(narrow) = item.claim_narrow.as_mut() {
        narrow.preserves_lineage_continuity = false;
    }
    assert!(!item.preserves_lineage_continuity());
    assert!(!item.claim_is_honest());
}

// --- AC3: narrowing disclosed across every narrower surface ---

#[test]
fn narrowed_surface_without_disclosure_is_rejected() {
    let mut item = row("a11y:work-item-row-freshness-stale");
    item.narrowing_disclosures.clear();
    assert!(!item.narrowing_disclosed());
}

#[test]
fn silently_dropped_surface_is_rejected() {
    let mut item = row("a11y:work-item-row-freshness-stale");
    item.narrowing_disclosures[0].state =
        WorkItemComponentNarrowingDisclosureState::SilentlyDropped;
    assert!(!item.narrowing_disclosed());
}

#[test]
fn narrowed_surface_must_preserve_labels() {
    let mut item = row("a11y:work-item-row-freshness-stale");
    item.narrowing_disclosures[0].preserved_labels.clear();
    assert!(!item.narrowing_disclosed());
}

#[test]
fn dropping_a_mandatory_label_strands_a_row() {
    let mut header = row("a11y:work-item-detail-header");
    header
        .required_labels
        .retain(|l| *l != M5WorkItemRequiredLabel::Identity);
    assert!(!header.preserves_mandatory_labels());
    assert_eq!(
        header.status(),
        WorkItemComponentAccessibilityStatus::Stranded
    );
}

// --- packet-level validation ---

#[test]
fn missing_family_coverage_is_flagged() {
    let mut packet = seeded_m5_work_item_component_a11y_packet();
    packet.rows.retain(|r| r.row_id != "a11y:relation-strip");
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        WorkItemComponentAccessibilityViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn single_consumer_surface_fails_parity() {
    let mut packet = seeded_m5_work_item_component_a11y_packet();
    packet.rows[0].consumer_surfaces = vec![M5WorkItemConsumerSurface::SupportExport];
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        WorkItemComponentAccessibilityViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut packet = seeded_m5_work_item_component_a11y_packet();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        WorkItemComponentAccessibilityViolation::DuplicateId { .. }
    )));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut packet = seeded_m5_work_item_component_a11y_packet();
    packet.summary.green_count += 1;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, WorkItemComponentAccessibilityViolation::SummaryMismatch)));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut packet = seeded_m5_work_item_component_a11y_packet();
    packet.rows[0]
        .source_refs
        .push("api_key=hunter2".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        WorkItemComponentAccessibilityViolation::RawProviderMaterialInExport
    )));
}

// --- rendering ---

#[test]
fn chip_tokens_render_stable_fields() {
    let chip = row("a11y:work-item-row-freshness-stale").chip_tokens();
    assert!(chip.contains("family=work_item_row"));
    assert!(chip.contains("effective_claim=stale_freshness_projection"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_work_item_component_a11y_packet();
    assert_eq!(packet.record_kind, WORK_ITEM_COMPONENT_A11Y_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        WORK_ITEM_COMPONENT_A11Y_SCHEMA_VERSION
    );
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_work_item_component_a11y_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn csv_has_one_header_and_one_line_per_row() {
    let packet = seeded_m5_work_item_component_a11y_packet();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), packet.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,component_family"));
}

// --- checked-in artifacts ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_m5_work_item_component_a11y_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_work_item_component_a11y_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_work_item_component_a11y_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-work-item-component-accessibility-proof/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_work_item_component_a11y_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-work-item-component-accessibility-proof.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var so it never
/// runs in the normal suite. Run with
/// `GEN_WORK_ITEM_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-provider generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_WORK_ITEM_COMPONENT_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_work_item_component_a11y_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest)
        .join("../../artifacts/release/m5-work-item-component-accessibility-proof");
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(
        Path::new(manifest)
            .join("../../artifacts/release/m5-work-item-component-accessibility-proof.md"),
        &report,
    )
    .expect("write report");

    let fixtures =
        Path::new(manifest).join("../../fixtures/ui/m5-work-item-component-accessibility-parity");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
}
