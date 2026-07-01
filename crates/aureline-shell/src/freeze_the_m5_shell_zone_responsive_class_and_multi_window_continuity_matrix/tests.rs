use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_shell_zone_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_SHELL_ZONE_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_family() {
    let packet = seeded_m5_shell_zone_matrix();
    let present: std::collections::BTreeSet<_> =
        packet.surface_rows.iter().map(|r| r.family).collect();
    for family in M5ShellSurfaceFamily::ALL {
        assert!(
            present.contains(&family),
            "missing family {}",
            family.as_str()
        );
    }
    assert_eq!(packet.surface_rows.len(), M5ShellSurfaceFamily::ALL.len());
}

#[test]
fn every_row_names_canonical_and_fallback_slot() {
    let packet = seeded_m5_shell_zone_matrix();
    for row in &packet.surface_rows {
        // Canonical and fallback slots are typed enums, always present; assert
        // the collapse ladder terminates in a placeholder and the family lives
        // in the primary workspace window.
        assert_eq!(
            row.fallback_placements.last(),
            Some(&M5FallbackPlacement::Placeholder),
            "family {} ladder must end in placeholder",
            row.family.as_str()
        );
        assert!(
            row.window_classes
                .contains(&M5WindowClass::PrimaryWorkspaceWindow),
            "family {} must admit the primary workspace window",
            row.family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_is_carried_by_some_row() {
    let packet = seeded_m5_shell_zone_matrix();
    for vocab in M5ShellStateVocabulary::ALL {
        assert!(
            packet
                .surface_rows
                .iter()
                .any(|row| row.state_vocabularies.contains(&vocab)),
            "no row carries vocabulary {}",
            vocab.as_str()
        );
    }
}

#[test]
fn missing_family_fails_validation() {
    let mut packet = seeded_m5_shell_zone_matrix();
    packet
        .surface_rows
        .retain(|row| row.family != M5ShellSurfaceFamily::Companion);
    assert!(packet
        .validate()
        .contains(&M5ShellZoneMatrixViolation::RequiredFamilyMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_shell_zone_matrix();
    packet.vocabulary_set.window_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5ShellZoneMatrixViolation::VocabularySetDrift));
}

#[test]
fn declared_vocabulary_without_tokens_fails() {
    let mut packet = seeded_m5_shell_zone_matrix();
    packet.surface_rows[0].window_classes.clear();
    let violations = packet.validate();
    assert!(violations.contains(&M5ShellZoneMatrixViolation::DeclaredVocabularyHasNoTokens));
}

#[test]
fn undeclared_vocabulary_with_tokens_fails() {
    let mut packet = seeded_m5_shell_zone_matrix();
    let row = &mut packet.surface_rows[0];
    row.state_vocabularies
        .retain(|v| *v != M5ShellStateVocabulary::WindowClass);
    // Tokens remain populated for the now-undeclared vocabulary.
    let violations = packet.validate();
    assert!(violations.contains(&M5ShellZoneMatrixViolation::UndeclaredVocabularyHasTokens));
}

#[test]
fn required_vocabulary_missing_fails() {
    let mut packet = seeded_m5_shell_zone_matrix();
    let row = &mut packet.surface_rows[0];
    row.state_vocabularies
        .retain(|v| *v != M5ShellStateVocabulary::ContinuityTruth);
    row.continuity_truths.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellZoneMatrixViolation::RequiredVocabularyMissing));
}

#[test]
fn fallback_ladder_without_placeholder_fails() {
    let mut packet = seeded_m5_shell_zone_matrix();
    packet.surface_rows[0]
        .fallback_placements
        .retain(|p| *p != M5FallbackPlacement::Placeholder);
    assert!(packet
        .validate()
        .contains(&M5ShellZoneMatrixViolation::FallbackLadderNotTerminatedByPlaceholder));
}

#[test]
fn missing_primary_window_fails() {
    let mut packet = seeded_m5_shell_zone_matrix();
    packet.surface_rows[0]
        .window_classes
        .retain(|w| *w != M5WindowClass::PrimaryWorkspaceWindow);
    assert!(packet
        .validate()
        .contains(&M5ShellZoneMatrixViolation::PrimaryWindowMissing));
}

#[test]
fn incomplete_responsive_coverage_fails() {
    let mut packet = seeded_m5_shell_zone_matrix();
    packet.surface_rows[0]
        .responsive_classes
        .retain(|c| *c != M5ResponsiveClass::CompactDesktop);
    assert!(packet
        .validate()
        .contains(&M5ShellZoneMatrixViolation::ResponsiveClassCoverageIncomplete));
}

#[test]
fn incomplete_owning_window_routing_fails() {
    let mut packet = seeded_m5_shell_zone_matrix();
    packet.surface_rows[0]
        .owning_window_routing
        .retain(|r| *r != M5OwningWindowRouting::NoFocusTheft);
    assert!(packet
        .validate()
        .contains(&M5ShellZoneMatrixViolation::OwningWindowRoutingIncomplete));
}

#[test]
fn incomplete_continuity_truth_fails() {
    let mut packet = seeded_m5_shell_zone_matrix();
    packet.surface_rows[0]
        .continuity_truths
        .retain(|t| *t != M5ContinuityTruth::RecoveryState);
    assert!(packet
        .validate()
        .contains(&M5ShellZoneMatrixViolation::ContinuityTruthIncomplete));
}

#[test]
fn stable_family_missing_proof_fails() {
    let mut packet = seeded_m5_shell_zone_matrix();
    let row = packet
        .surface_rows
        .iter_mut()
        .find(|r| r.family == M5ShellSurfaceFamily::Notebook)
        .expect("notebook row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellZoneMatrixViolation::StableFamilyMissingProof));
}

#[test]
fn missing_owner_role_fails() {
    let mut packet = seeded_m5_shell_zone_matrix();
    packet.surface_rows[0].owner_role.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellZoneMatrixViolation::SurfaceRowIncomplete));
}

#[test]
fn every_row_names_an_owner() {
    let packet = seeded_m5_shell_zone_matrix();
    for row in &packet.surface_rows {
        assert!(
            !row.owner_role.trim().is_empty(),
            "family {} has no owner",
            row.family.as_str()
        );
    }
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = seeded_m5_shell_zone_matrix();
    packet.surface_rows[0].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellZoneMatrixViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_shell_zone_matrix();
    packet.surface_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellZoneMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_shell_zone_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellZoneMatrixViolation::MissingSourceContracts));
}

#[test]
fn continuity_review_incomplete_fails() {
    let mut packet = seeded_m5_shell_zone_matrix();
    packet
        .continuity_review
        .dialogs_notifications_approvals_route_to_owning_window_object = false;
    assert!(packet
        .validate()
        .contains(&M5ShellZoneMatrixViolation::ContinuityReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_shell_zone_matrix();
    packet.consumer_projection.windowing_consumes_window_classes = false;
    assert!(packet
        .validate()
        .contains(&M5ShellZoneMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_shell_zone_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ShellZoneMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_shell_zone_matrix();
    packet.release_posture.multi_window_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ShellZoneMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_family() {
    let summary = seeded_m5_shell_zone_matrix().render_markdown_summary();
    for family in M5ShellSurfaceFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing family {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_family() {
    let csv = seeded_m5_shell_zone_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    // header + one row per family
    assert_eq!(lines.len(), 1 + M5ShellSurfaceFamily::ALL.len());
    assert!(lines[0].starts_with("family,qualification,owner,canonical_slot,"));
    for family in M5ShellSurfaceFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing family {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_shell_zone_matrix_export()
        .expect("checked M5 shell-zone matrix export validates");
    assert_eq!(packet.packet_id, M5_SHELL_ZONE_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_shell_zone_matrix_export()
        .expect("checked M5 shell-zone matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_shell_zone_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_families_mapped() {
    for packet in [
        seeded_m5_shell_zone_matrix_profiler_remote_held(),
        seeded_m5_shell_zone_matrix_companion_overlay_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        // Downgrade narrows the claim without removing the surface family.
        assert_eq!(packet.surface_rows.len(), M5ShellSurfaceFamily::ALL.len());
    }

    let held = seeded_m5_shell_zone_matrix_profiler_remote_held();
    let row = held
        .surface_rows
        .iter()
        .find(|r| r.family == M5ShellSurfaceFamily::Profiler)
        .expect("profiler row present");
    assert_eq!(row.qualification, M5ShellQualificationClass::Held);

    let narrowed = seeded_m5_shell_zone_matrix_companion_overlay_narrowed();
    let row = narrowed
        .surface_rows
        .iter()
        .find(|r| r.family == M5ShellSurfaceFamily::Companion)
        .expect("companion row present");
    assert_eq!(row.qualification, M5ShellQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-shell-layouts/profiler_remote_held.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-shell-layouts/companion_overlay_narrowed.json"
        )),
    ] {
        let packet: M5ShellZoneMatrixPacket =
            serde_json::from_str(raw).expect("fixture parses as matrix packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn checked_narrowed_fixtures_match_seed_builders() {
    let held: M5ShellZoneMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-shell-layouts/profiler_remote_held.json"
    )))
    .expect("held fixture parses");
    assert_eq!(held, seeded_m5_shell_zone_matrix_profiler_remote_held());

    let narrowed: M5ShellZoneMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-shell-layouts/companion_overlay_narrowed.json"
    )))
    .expect("narrowed fixture parses");
    assert_eq!(
        narrowed,
        seeded_m5_shell_zone_matrix_companion_overlay_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_shell_zone_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
}
