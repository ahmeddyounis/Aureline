//! Tests for the M05-994 credential component accessibility parity capstone: the honest
//! auto-narrowing logic, the per-family parity contract, the unverified/expired/reveal-blocked
//! never-brokered guarantee, no-loss credential-lineage integrity, and the checked-in support
//! export / CSV / report.

use super::*;

fn row(id: &str) -> CredentialComponentAccessibilityRow {
    seeded_m5_credential_component_a11y_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_credential_component_a11y_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified() {
    let packet = seeded_m5_credential_component_a11y_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5CredentialComponentFamily::ALL.len()
    );
    // Eight rows: one per frozen family.
    assert_eq!(packet.rows.len(), 8);
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_credential_component_a11y_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5CredentialComponentClaimDimension::ALL.len()
    );
}

#[test]
fn every_condition_state_is_exercised() {
    let packet = seeded_m5_credential_component_a11y_packet();
    let states = packet.exercised_condition_states();
    for state in M5CredentialComponentConditionState::ALL {
        assert!(
            states.contains(&state),
            "condition state {} is not exercised",
            state.as_str()
        );
    }
}

#[test]
fn every_credential_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_credential_component_a11y_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5CredentialComponentClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_credential_component_a11y_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5CredentialConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_four_green_four_yellow_zero_red() {
    let packet = seeded_m5_credential_component_a11y_packet();
    assert_eq!(packet.summary.green_count, 4);
    assert_eq!(packet.summary.yellow_count, 4);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary.row_count, 8);
    assert_eq!(
        packet.summary.family_count,
        M5CredentialComponentFamily::ALL.len()
    );
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn summary_reports_all_lineage_and_broker_honesty() {
    let packet = seeded_m5_credential_component_a11y_packet();
    assert!(packet.summary.all_lineage_preserved);
    assert!(packet.summary.all_broker_honesty_holds);
}

// --- AC1: unverified-store / expired-auth / drifted-delegation / reveal-blocked can no longer keep a brokered label ---

#[test]
fn store_capability_row_is_handle_ready_and_green() {
    let store = row("a11y:credential-store-capability-row");
    assert_eq!(
        store.effective_claim(),
        M5CredentialComponentClaim::HandleReadyProjection
    );
    assert!(store.claim_narrow.is_none());
    assert_eq!(
        store.status(),
        CredentialComponentAccessibilityStatus::Parity
    );
    assert!(store.effective_claim().asserts_full_projection());
    assert!(!store.effective_claim().asserts_verified_brokered());
}

#[test]
fn handoff_card_is_verified_brokered_and_green() {
    let card = row("a11y:browser-device-code-handoff-card");
    assert_eq!(
        card.full_credential_claim,
        M5CredentialComponentClaim::VerifiedBrokered
    );
    assert_eq!(
        card.effective_claim(),
        M5CredentialComponentClaim::VerifiedBrokered
    );
    assert!(card.claim_narrow.is_none());
    assert_eq!(
        card.status(),
        CredentialComponentAccessibilityStatus::Parity
    );
    assert!(card.effective_claim().asserts_verified_brokered());
}

#[test]
fn auth_expired_narrows_to_expired_auth_projection() {
    let item = row("a11y:credential-state-row-auth-expired");
    assert_eq!(
        item.effective_claim(),
        M5CredentialComponentClaim::ExpiredAuthProjection
    );
    assert!(!item.effective_claim().asserts_verified_brokered());
    let narrow = item.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5CredentialDowngradeTrigger::LifecycleStateHidden
    );
    assert_eq!(
        narrow.binding_dimension,
        M5CredentialComponentClaimDimension::AuthPosture
    );
    assert!(item.claim_is_honest());
    // An expired state must never be shown as verified-brokered.
    assert!(item.broker_honesty_holds());
}

#[test]
fn reveal_policy_blocked_narrows_to_reveal_blocked_projection() {
    let sheet = row("a11y:secret-access-prompt-sheet-reveal-blocked");
    assert_eq!(
        sheet.effective_claim(),
        M5CredentialComponentClaim::RevealBlockedProjection
    );
    let narrow = sheet.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5CredentialDowngradeTrigger::RevealPostureUnstated
    );
    assert!(sheet.claim_is_honest());
    assert!(sheet.broker_honesty_holds());
}

#[test]
fn store_unverified_narrows_to_unverified_store_projection() {
    let picker = row("a11y:vault-or-keychain-picker-store-unverified");
    assert_eq!(
        picker.effective_claim(),
        M5CredentialComponentClaim::UnverifiedStoreProjection
    );
    assert!(!picker.effective_claim().asserts_verified_brokered());
    let narrow = picker.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5CredentialDowngradeTrigger::StoreCapabilityUnstated
    );
    assert!(picker.claim_is_honest());
    assert!(picker.broker_honesty_holds());
}

#[test]
fn delegated_scope_drifted_narrows_to_drifted_delegation_projection() {
    let del = row("a11y:delegated-credential-row-scope-drifted");
    assert_eq!(
        del.effective_claim(),
        M5CredentialComponentClaim::DriftedDelegationProjection
    );
    assert!(!del.effective_claim().asserts_full_projection());
    let narrow = del.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5CredentialDowngradeTrigger::DelegatedIdentityUnstated
    );
    assert!(del.claim_is_honest());
    assert!(del.broker_honesty_holds());
}

#[test]
fn over_asserting_control_is_rejected() {
    // Force the effective claim above the permitted ceiling: an expired-auth row claiming
    // VerifiedBrokered.
    let mut item = row("a11y:credential-state-row-auth-expired");
    item.claim_narrow = None;
    assert!(!item.claim_is_honest());
    assert_eq!(
        item.status(),
        CredentialComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn unverified_shown_as_brokered_is_rejected() {
    // An expired-auth row whose narrow claims VerifiedBrokered violates broker honesty.
    let mut item = row("a11y:credential-state-row-auth-expired");
    if let Some(narrow) = item.claim_narrow.as_mut() {
        narrow.narrowed_to = M5CredentialComponentClaim::VerifiedBrokered;
    }
    assert!(!item.broker_honesty_holds());
    let violations = {
        let mut packet = seeded_m5_credential_component_a11y_packet();
        packet.rows[0] = item;
        packet.validate()
    };
    assert!(violations.iter().any(|v| matches!(
        v,
        CredentialComponentAccessibilityViolation::UnverifiedShownAsBrokered { .. }
    )));
}

#[test]
fn broker_honesty_unproven_when_no_unverified_expired_or_blocked_row() {
    let mut packet = seeded_m5_credential_component_a11y_packet();
    // Drop the three unverified/expired/blocked rows (expired auth + reveal blocked + store unverified).
    packet.rows.retain(|r| {
        r.row_id != "a11y:credential-state-row-auth-expired"
            && r.row_id != "a11y:secret-access-prompt-sheet-reveal-blocked"
            && r.row_id != "a11y:vault-or-keychain-picker-store-unverified"
    });
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        CredentialComponentAccessibilityViolation::BrokerHonestyUnproven
    )));
}

#[test]
fn spurious_narrow_on_brokered_row_is_rejected() {
    let mut card = row("a11y:browser-device-code-handoff-card");
    card.claim_narrow = Some(CredentialComponentClaimAutoNarrow {
        narrowed_to: M5CredentialComponentClaim::RevealBlockedProjection,
        binding_dimension: M5CredentialComponentClaimDimension::RevealPolicy,
        trigger: M5CredentialDowngradeTrigger::RevealPostureUnstated,
        narrowed_label: "spurious".to_owned(),
        preserves_canonical_identity: true,
        preserves_lineage_continuity: true,
    });
    assert!(!card.claim_is_honest());
}

#[test]
fn narrow_must_bind_the_ceiling_imposing_dimension() {
    let mut item = row("a11y:credential-state-row-auth-expired");
    if let Some(narrow) = item.claim_narrow.as_mut() {
        narrow.binding_dimension = M5CredentialComponentClaimDimension::RevealPolicy;
    }
    assert!(!item.claim_is_honest());
}

#[test]
fn narrow_trigger_must_match_binding_condition() {
    let mut item = row("a11y:credential-state-row-auth-expired");
    if let Some(narrow) = item.claim_narrow.as_mut() {
        narrow.trigger = M5CredentialDowngradeTrigger::ExportSafetyBoundaryHidden;
    }
    assert!(!item.claim_is_honest());
}

#[test]
fn narrowed_label_must_not_be_generic() {
    let mut item = row("a11y:credential-state-row-auth-expired");
    if let Some(narrow) = item.claim_narrow.as_mut() {
        narrow.narrowed_label = "expired".to_owned();
    }
    assert!(!item.claim_is_honest());
}

#[test]
fn permitted_ceilings_map_condition_states_one_to_one() {
    use M5CredentialComponentClaim as S;
    use M5CredentialComponentConditionState as C;
    assert_eq!(C::VerifiedCurrent.permitted_ceiling(), S::VerifiedBrokered);
    assert_eq!(
        C::StoreUnverified.permitted_ceiling(),
        S::UnverifiedStoreProjection
    );
    assert_eq!(C::AuthExpired.permitted_ceiling(), S::ExpiredAuthProjection);
    assert_eq!(
        C::DelegatedScopeDrifted.permitted_ceiling(),
        S::DriftedDelegationProjection
    );
    assert_eq!(
        C::RevealPolicyBlocked.permitted_ceiling(),
        S::RevealBlockedProjection
    );
}

#[test]
fn condition_triggers_map_to_frozen_matrix_vocabulary() {
    use M5CredentialComponentConditionState as C;
    use M5CredentialDowngradeTrigger as T;
    assert_eq!(
        C::StoreUnverified.default_trigger(),
        T::StoreCapabilityUnstated
    );
    assert_eq!(C::AuthExpired.default_trigger(), T::LifecycleStateHidden);
    assert_eq!(
        C::DelegatedScopeDrifted.default_trigger(),
        T::DelegatedIdentityUnstated
    );
    assert_eq!(
        C::RevealPolicyBlocked.default_trigger(),
        T::RevealPostureUnstated
    );
}

#[test]
fn unverified_expired_or_blocked_states_are_flagged() {
    use M5CredentialComponentConditionState as C;
    assert!(C::StoreUnverified.is_unverified_expired_or_blocked());
    assert!(C::AuthExpired.is_unverified_expired_or_blocked());
    assert!(C::RevealPolicyBlocked.is_unverified_expired_or_blocked());
    assert!(!C::DelegatedScopeDrifted.is_unverified_expired_or_blocked());
    assert!(!C::VerifiedCurrent.is_unverified_expired_or_blocked());
}

// --- AC2: accessibility / CLI / export reach the same canonical truth + no-loss ---

#[test]
fn every_row_reaches_canonical_truth_via_at() {
    for r in seeded_m5_credential_component_a11y_packet().rows {
        assert!(
            r.reaches_canonical_truth_via_at(),
            "row {} strands assistive tech",
            r.row_id
        );
    }
}

#[test]
fn hierarchy_heavy_export_banner_binds_a_non_visual_fallback() {
    let banner = row("a11y:export-safety-banner");
    assert!(banner.is_hierarchy_heavy());
    assert!(banner.has_non_visual_fallback());
    assert!(banner
        .fallback_modalities
        .contains(&M5CredentialComponentFallbackModality::Structured));
}

#[test]
fn view_only_trap_strands_and_reds_a_row() {
    let mut card = row("a11y:browser-device-code-handoff-card");
    card.keyboard_reach = CredentialComponentNonVisualReachState::ViewOnlyTrap;
    assert!(!card.reaches_canonical_truth_via_at());
    assert_eq!(
        card.status(),
        CredentialComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn empty_credential_context_ref_strands_a_row() {
    let mut card = row("a11y:browser-device-code-handoff-card");
    card.credential_context_ref = "  ".to_owned();
    assert!(!card.reaches_canonical_truth_via_at());
}

#[test]
fn export_needing_screenshot_is_rejected() {
    let mut card = row("a11y:browser-device-code-handoff-card");
    card.export_summary = CredentialComponentExportSummaryState::AbsentNeedsScreenshot;
    assert!(!card.export_preserves_meaning());
    assert_eq!(
        card.status(),
        CredentialComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn copy_export_requires_text_json_markdown() {
    let mut card = row("a11y:browser-device-code-handoff-card");
    card.copy_export.formats.retain(|f| f != "markdown");
    assert!(!card.export_preserves_meaning());
}

#[test]
fn dropped_lineage_strands_a_row() {
    let mut item = row("a11y:credential-state-row-auth-expired");
    item.lineage_preserved = false;
    assert!(!item.preserves_lineage_continuity());
    assert_eq!(
        item.status(),
        CredentialComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn narrow_dropping_lineage_continuity_strands_a_row() {
    let mut item = row("a11y:credential-state-row-auth-expired");
    if let Some(narrow) = item.claim_narrow.as_mut() {
        narrow.preserves_lineage_continuity = false;
    }
    assert!(!item.preserves_lineage_continuity());
    assert!(!item.claim_is_honest());
}

// --- AC3: narrowing disclosed across every narrower surface ---

#[test]
fn narrowed_surface_without_disclosure_is_rejected() {
    let mut item = row("a11y:credential-state-row-auth-expired");
    item.narrowing_disclosures.clear();
    assert!(!item.narrowing_disclosed());
}

#[test]
fn silently_dropped_surface_is_rejected() {
    let mut item = row("a11y:credential-state-row-auth-expired");
    item.narrowing_disclosures[0].state =
        CredentialComponentNarrowingDisclosureState::SilentlyDropped;
    assert!(!item.narrowing_disclosed());
}

#[test]
fn narrowed_surface_must_preserve_labels() {
    let mut item = row("a11y:credential-state-row-auth-expired");
    item.narrowing_disclosures[0].preserved_labels.clear();
    assert!(!item.narrowing_disclosed());
}

#[test]
fn dropping_a_mandatory_label_strands_a_row() {
    let mut card = row("a11y:browser-device-code-handoff-card");
    card.required_labels
        .retain(|l| *l != M5CredentialRequiredLabel::Identity);
    assert!(!card.preserves_mandatory_labels());
    assert_eq!(
        card.status(),
        CredentialComponentAccessibilityStatus::Stranded
    );
}

// --- packet-level validation ---

#[test]
fn missing_family_coverage_is_flagged() {
    let mut packet = seeded_m5_credential_component_a11y_packet();
    packet
        .rows
        .retain(|r| r.row_id != "a11y:credential-store-capability-row");
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        CredentialComponentAccessibilityViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn single_consumer_surface_fails_parity() {
    let mut packet = seeded_m5_credential_component_a11y_packet();
    packet.rows[0].consumer_surfaces = vec![M5CredentialConsumerSurface::SupportExport];
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        CredentialComponentAccessibilityViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut packet = seeded_m5_credential_component_a11y_packet();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        CredentialComponentAccessibilityViolation::DuplicateId { .. }
    )));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut packet = seeded_m5_credential_component_a11y_packet();
    packet.summary.green_count += 1;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        CredentialComponentAccessibilityViolation::SummaryMismatch
    )));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut packet = seeded_m5_credential_component_a11y_packet();
    packet.rows[0]
        .source_refs
        .push("password=hunter2".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        CredentialComponentAccessibilityViolation::RawCredentialMaterialInExport
    )));
}

// --- rendering ---

#[test]
fn chip_tokens_render_stable_fields() {
    let chip = row("a11y:credential-state-row-auth-expired").chip_tokens();
    assert!(chip.contains("family=credential_state_row"));
    assert!(chip.contains("effective_claim=expired_auth_projection"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_credential_component_a11y_packet();
    assert_eq!(packet.record_kind, CREDENTIAL_COMPONENT_A11Y_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        CREDENTIAL_COMPONENT_A11Y_SCHEMA_VERSION
    );
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_credential_component_a11y_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn csv_has_one_header_and_one_line_per_row() {
    let packet = seeded_m5_credential_component_a11y_packet();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), packet.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,component_family"));
}

// --- checked-in artifacts ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_m5_credential_component_a11y_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_credential_component_a11y_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_credential_component_a11y_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-credential-component-accessibility-proof/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_credential_component_a11y_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-credential-component-accessibility-proof.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var so it never
/// runs in the normal suite. Run with
/// `GEN_CREDENTIAL_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-provider generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_CREDENTIAL_COMPONENT_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_credential_component_a11y_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest)
        .join("../../artifacts/release/m5-credential-component-accessibility-proof");
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(
        Path::new(manifest)
            .join("../../artifacts/release/m5-credential-component-accessibility-proof.md"),
        &report,
    )
    .expect("write report");

    let fixtures =
        Path::new(manifest).join("../../fixtures/ui/m5-credential-component-accessibility-parity");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
}
