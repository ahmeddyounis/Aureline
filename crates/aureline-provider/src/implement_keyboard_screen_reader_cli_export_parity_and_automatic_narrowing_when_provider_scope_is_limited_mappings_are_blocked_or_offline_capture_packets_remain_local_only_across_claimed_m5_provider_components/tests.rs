//! Tests for the M05-922 provider-account / offline-capture component accessibility fallback
//! capstone: the honest auto-narrowing logic, the per-family parity contract, the
//! cached-or-offline never-committed guarantee, no-loss provider-lineage integrity, and the
//! checked-in support export / CSV / report.

use super::*;

fn row(id: &str) -> ProviderComponentAccessibilityRow {
    seeded_m5_provider_component_a11y_fallback_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_provider_component_a11y_fallback_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified() {
    let packet = seeded_m5_provider_component_a11y_fallback_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5ProviderAccountOfflineComponentFamily::ALL.len()
    );
    // Six rows cover the five families (the provider-account row is certified both
    // scope-limited-yellow and session-stale-yellow).
    assert_eq!(packet.rows.len(), 6);
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_provider_component_a11y_fallback_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5ProviderComponentClaimDimension::ALL.len()
    );
}

#[test]
fn every_condition_state_is_exercised() {
    let packet = seeded_m5_provider_component_a11y_fallback_packet();
    let states = packet.exercised_condition_states();
    for state in M5ProviderComponentConditionState::ALL {
        assert!(
            states.contains(&state),
            "condition state {} is not exercised",
            state.as_str()
        );
    }
}

#[test]
fn every_provider_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_provider_component_a11y_fallback_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5ProviderComponentClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_provider_component_a11y_fallback_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5ProviderConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_two_green_four_yellow_zero_red() {
    let packet = seeded_m5_provider_component_a11y_fallback_packet();
    assert_eq!(packet.summary.green_count, 2);
    assert_eq!(packet.summary.yellow_count, 4);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary.row_count, 6);
    assert_eq!(
        packet.summary.family_count,
        M5ProviderAccountOfflineComponentFamily::ALL.len()
    );
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn summary_reports_all_lineage_and_commit_honesty() {
    let packet = seeded_m5_provider_component_a11y_fallback_packet();
    assert!(packet.summary.all_lineage_preserved);
    assert!(packet.summary.all_commit_honesty_holds);
}

// --- AC1: limited-scope / stale-session / policy-blocked / local-only can no longer keep a committed label ---

#[test]
fn stated_redaction_row_is_reviewable_and_green() {
    let redaction = row("a11y:privacy-redaction-row");
    assert_eq!(
        redaction.effective_claim(),
        M5ProviderComponentClaim::ReviewableProjection
    );
    assert!(redaction.claim_narrow.is_none());
    assert_eq!(
        redaction.status(),
        ProviderComponentAccessibilityStatus::Parity
    );
    assert!(redaction.effective_claim().asserts_full_projection());
    assert!(!redaction.effective_claim().asserts_provider_committed());
}

#[test]
fn stated_sync_row_is_provider_committed_and_green() {
    let sync = row("a11y:sync-behavior-row");
    assert_eq!(
        sync.full_provider_claim,
        M5ProviderComponentClaim::ProviderCommitted
    );
    assert_eq!(
        sync.effective_claim(),
        M5ProviderComponentClaim::ProviderCommitted
    );
    assert!(sync.claim_narrow.is_none());
    assert_eq!(sync.status(), ProviderComponentAccessibilityStatus::Parity);
    assert!(sync.effective_claim().asserts_provider_committed());
}

#[test]
fn limited_scope_narrows_to_limited_scope_projection() {
    let account = row("a11y:provider-account-row-scope-limited");
    assert_eq!(
        account.effective_claim(),
        M5ProviderComponentClaim::LimitedScopeProjection
    );
    assert!(!account.effective_claim().asserts_provider_committed());
    let narrow = account.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5ProviderDowngradeTrigger::WriteScopeUnstated
    );
    assert_eq!(
        narrow.binding_dimension,
        M5ProviderComponentClaimDimension::ConnectionAndScope
    );
    assert!(account.claim_is_honest());
}

#[test]
fn stale_session_narrows_to_stale_session_projection() {
    let account = row("a11y:provider-account-row-session-stale");
    assert_eq!(
        account.effective_claim(),
        M5ProviderComponentClaim::StaleSessionProjection
    );
    let narrow = account.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5ProviderDowngradeTrigger::ConnectionStateUnstated
    );
    assert!(account.claim_is_honest());
    // Cached / offline state must never be shown as committed.
    assert!(account.commit_honesty_holds());
}

#[test]
fn policy_blocked_mapping_narrows_to_policy_blocked_mapping() {
    let mapping = row("a11y:project-or-board-mapping-row-policy-blocked");
    assert_eq!(
        mapping.effective_claim(),
        M5ProviderComponentClaim::PolicyBlockedMapping
    );
    let narrow = mapping.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5ProviderDowngradeTrigger::MappingOriginUnstated
    );
    assert!(mapping.claim_is_honest());
}

#[test]
fn local_only_packet_narrows_to_local_only_packet() {
    let offline = row("a11y:offline-capture-row");
    assert_eq!(
        offline.effective_claim(),
        M5ProviderComponentClaim::LocalOnlyPacket
    );
    assert!(!offline.effective_claim().asserts_full_projection());
    let narrow = offline.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5ProviderDowngradeTrigger::OfflineCaptureStateUnstated
    );
    assert!(offline.claim_is_honest());
    assert!(offline.commit_honesty_holds());
}

#[test]
fn over_asserting_control_is_rejected() {
    // Force the effective claim above the permitted ceiling: a scope-limited account claiming
    // ProviderCommitted.
    let mut account = row("a11y:provider-account-row-scope-limited");
    account.claim_narrow = None;
    assert!(!account.claim_is_honest());
    assert_eq!(
        account.status(),
        ProviderComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn cached_or_offline_shown_as_committed_is_rejected() {
    // A stale-session account whose narrow claims ProviderCommitted violates commit honesty.
    let mut account = row("a11y:provider-account-row-session-stale");
    if let Some(narrow) = account.claim_narrow.as_mut() {
        narrow.narrowed_to = M5ProviderComponentClaim::ProviderCommitted;
    }
    assert!(!account.commit_honesty_holds());
    let violations = {
        let mut packet = seeded_m5_provider_component_a11y_fallback_packet();
        packet.rows[1] = account;
        packet.validate()
    };
    assert!(violations.iter().any(|v| matches!(
        v,
        ProviderComponentAccessibilityViolation::CachedOrOfflineShownAsCommitted { .. }
    )));
}

#[test]
fn commit_honesty_unproven_when_no_cached_or_offline_row() {
    let mut packet = seeded_m5_provider_component_a11y_fallback_packet();
    // Drop the only two cached/offline rows (stale session + local-only packet).
    packet.rows.retain(|r| {
        r.row_id != "a11y:provider-account-row-session-stale"
            && r.row_id != "a11y:offline-capture-row"
    });
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ProviderComponentAccessibilityViolation::CommitHonestyUnproven
    )));
}

#[test]
fn spurious_narrow_on_committed_row_is_rejected() {
    let mut sync = row("a11y:sync-behavior-row");
    sync.claim_narrow = Some(ProviderComponentClaimAutoNarrow {
        narrowed_to: M5ProviderComponentClaim::LocalOnlyPacket,
        binding_dimension: M5ProviderComponentClaimDimension::OfflineCapture,
        trigger: M5ProviderDowngradeTrigger::OfflineCaptureStateUnstated,
        narrowed_label: "spurious".to_owned(),
        preserves_canonical_identity: true,
        preserves_lineage_continuity: true,
    });
    assert!(!sync.claim_is_honest());
}

#[test]
fn narrow_must_bind_the_ceiling_imposing_dimension() {
    let mut account = row("a11y:provider-account-row-scope-limited");
    if let Some(narrow) = account.claim_narrow.as_mut() {
        narrow.binding_dimension = M5ProviderComponentClaimDimension::RedactionBoundary;
    }
    assert!(!account.claim_is_honest());
}

#[test]
fn narrow_trigger_must_match_binding_condition() {
    let mut account = row("a11y:provider-account-row-scope-limited");
    if let Some(narrow) = account.claim_narrow.as_mut() {
        narrow.trigger = M5ProviderDowngradeTrigger::ExportBoundaryHidden;
    }
    assert!(!account.claim_is_honest());
}

#[test]
fn narrowed_label_must_not_be_generic() {
    let mut account = row("a11y:provider-account-row-scope-limited");
    if let Some(narrow) = account.claim_narrow.as_mut() {
        narrow.narrowed_label = "limited scope".to_owned();
    }
    assert!(!account.claim_is_honest());
}

#[test]
fn permitted_ceilings_map_condition_states_one_to_one() {
    use M5ProviderComponentClaim as S;
    use M5ProviderComponentConditionState as C;
    assert_eq!(
        C::InScopeCommitted.permitted_ceiling(),
        S::ProviderCommitted
    );
    assert_eq!(
        C::ScopeLimited.permitted_ceiling(),
        S::LimitedScopeProjection
    );
    assert_eq!(
        C::SessionStale.permitted_ceiling(),
        S::StaleSessionProjection
    );
    assert_eq!(
        C::MappingPolicyBlocked.permitted_ceiling(),
        S::PolicyBlockedMapping
    );
    assert_eq!(C::PacketLocalOnly.permitted_ceiling(), S::LocalOnlyPacket);
}

#[test]
fn condition_triggers_map_to_frozen_matrix_vocabulary() {
    use M5ProviderComponentConditionState as C;
    use M5ProviderDowngradeTrigger as T;
    assert_eq!(C::ScopeLimited.default_trigger(), T::WriteScopeUnstated);
    assert_eq!(
        C::SessionStale.default_trigger(),
        T::ConnectionStateUnstated
    );
    assert_eq!(
        C::MappingPolicyBlocked.default_trigger(),
        T::MappingOriginUnstated
    );
    assert_eq!(
        C::PacketLocalOnly.default_trigger(),
        T::OfflineCaptureStateUnstated
    );
}

#[test]
fn cached_or_offline_states_are_flagged() {
    use M5ProviderComponentConditionState as C;
    assert!(C::SessionStale.is_cached_or_offline());
    assert!(C::PacketLocalOnly.is_cached_or_offline());
    assert!(!C::ScopeLimited.is_cached_or_offline());
    assert!(!C::MappingPolicyBlocked.is_cached_or_offline());
    assert!(!C::InScopeCommitted.is_cached_or_offline());
}

// --- AC2: accessibility / CLI / export reach the same canonical truth + no-loss ---

#[test]
fn every_row_reaches_canonical_truth_via_at() {
    for r in seeded_m5_provider_component_a11y_fallback_packet().rows {
        assert!(
            r.reaches_canonical_truth_via_at(),
            "row {} strands assistive tech",
            r.row_id
        );
    }
}

#[test]
fn hierarchy_heavy_offline_capture_binds_a_non_visual_fallback() {
    let offline = row("a11y:offline-capture-row");
    assert!(offline.is_hierarchy_heavy());
    assert!(offline.has_non_visual_fallback());
    assert!(offline
        .fallback_modalities
        .contains(&M5ProviderComponentFallbackModality::Structured));
}

#[test]
fn view_only_trap_strands_and_reds_a_row() {
    let mut sync = row("a11y:sync-behavior-row");
    sync.keyboard_reach = ProviderComponentNonVisualReachState::ViewOnlyTrap;
    assert!(!sync.reaches_canonical_truth_via_at());
    assert_eq!(
        sync.status(),
        ProviderComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn empty_provider_context_ref_strands_a_row() {
    let mut sync = row("a11y:sync-behavior-row");
    sync.provider_context_ref = "  ".to_owned();
    assert!(!sync.reaches_canonical_truth_via_at());
}

#[test]
fn export_needing_screenshot_is_rejected() {
    let mut sync = row("a11y:sync-behavior-row");
    sync.export_summary = ProviderComponentExportSummaryState::AbsentNeedsScreenshot;
    assert!(!sync.export_preserves_meaning());
    assert_eq!(
        sync.status(),
        ProviderComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn copy_export_requires_text_json_markdown() {
    let mut sync = row("a11y:sync-behavior-row");
    sync.copy_export.formats.retain(|f| f != "markdown");
    assert!(!sync.export_preserves_meaning());
}

#[test]
fn dropped_lineage_strands_a_row() {
    let mut account = row("a11y:provider-account-row-scope-limited");
    account.lineage_preserved = false;
    assert!(!account.preserves_lineage_continuity());
    assert_eq!(
        account.status(),
        ProviderComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn narrow_dropping_lineage_continuity_strands_a_row() {
    let mut account = row("a11y:provider-account-row-scope-limited");
    if let Some(narrow) = account.claim_narrow.as_mut() {
        narrow.preserves_lineage_continuity = false;
    }
    assert!(!account.preserves_lineage_continuity());
    assert!(!account.claim_is_honest());
}

// --- AC3: narrowing disclosed across every narrower surface ---

#[test]
fn narrowed_surface_without_disclosure_is_rejected() {
    let mut account = row("a11y:provider-account-row-scope-limited");
    account.narrowing_disclosures.clear();
    assert!(!account.narrowing_disclosed());
}

#[test]
fn silently_dropped_surface_is_rejected() {
    let mut account = row("a11y:provider-account-row-scope-limited");
    account.narrowing_disclosures[0].state =
        ProviderComponentNarrowingDisclosureState::SilentlyDropped;
    assert!(!account.narrowing_disclosed());
}

#[test]
fn narrowed_surface_must_preserve_labels() {
    let mut account = row("a11y:provider-account-row-scope-limited");
    account.narrowing_disclosures[0].preserved_labels.clear();
    assert!(!account.narrowing_disclosed());
}

#[test]
fn dropping_a_mandatory_label_strands_a_row() {
    let mut sync = row("a11y:sync-behavior-row");
    sync.required_labels
        .retain(|l| *l != M5ProviderRequiredLabel::Identity);
    assert!(!sync.preserves_mandatory_labels());
    assert_eq!(
        sync.status(),
        ProviderComponentAccessibilityStatus::Stranded
    );
}

// --- packet-level validation ---

#[test]
fn missing_family_coverage_is_flagged() {
    let mut packet = seeded_m5_provider_component_a11y_fallback_packet();
    packet
        .rows
        .retain(|r| r.row_id != "a11y:privacy-redaction-row");
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ProviderComponentAccessibilityViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn single_consumer_surface_fails_parity() {
    let mut packet = seeded_m5_provider_component_a11y_fallback_packet();
    packet.rows[0].consumer_surfaces = vec![M5ProviderConsumerSurface::SupportExport];
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ProviderComponentAccessibilityViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut packet = seeded_m5_provider_component_a11y_fallback_packet();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ProviderComponentAccessibilityViolation::DuplicateId { .. }
    )));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut packet = seeded_m5_provider_component_a11y_fallback_packet();
    packet.summary.green_count += 1;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, ProviderComponentAccessibilityViolation::SummaryMismatch)));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut packet = seeded_m5_provider_component_a11y_fallback_packet();
    packet.rows[0]
        .source_refs
        .push("api_key=hunter2".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ProviderComponentAccessibilityViolation::RawProviderMaterialInExport
    )));
}

// --- rendering ---

#[test]
fn chip_tokens_render_stable_fields() {
    let chip = row("a11y:provider-account-row-scope-limited").chip_tokens();
    assert!(chip.contains("family=provider_account_row"));
    assert!(chip.contains("effective_claim=limited_scope_projection"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_provider_component_a11y_fallback_packet();
    assert_eq!(
        packet.record_kind,
        PROVIDER_COMPONENT_A11Y_FALLBACK_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        PROVIDER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION
    );
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_provider_component_a11y_fallback_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn csv_has_one_header_and_one_line_per_row() {
    let packet = seeded_m5_provider_component_a11y_fallback_packet();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), packet.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,component_family"));
}

// --- checked-in artifacts ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_m5_provider_component_a11y_fallback_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_provider_component_a11y_fallback_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_provider_component_a11y_fallback_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-provider-account-offline-capture-component-accessibility-fallback/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_provider_component_a11y_fallback_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-provider-account-offline-capture-component-accessibility-fallback.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var so it never
/// runs in the normal suite. Run with
/// `GEN_PROVIDER_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-provider generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_PROVIDER_COMPONENT_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_provider_component_a11y_fallback_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest).join(
        "../../artifacts/release/m5-provider-account-offline-capture-component-accessibility-fallback",
    );
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(
        Path::new(manifest).join(
            "../../artifacts/release/m5-provider-account-offline-capture-component-accessibility-fallback.md",
        ),
        &report,
    )
    .expect("write report");

    let fixtures = Path::new(manifest).join(
        "../../fixtures/ui/m5-provider-account-offline-capture-component-accessibility-fallback",
    );
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
}
