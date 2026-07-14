//! Tests for the M05-1194 repository-bootstrap accessibility parity capstone: the honest auto-narrowing logic,
//! the per-family parity contract, the weakened-never-trusted guarantee, no-loss source-locator / checkout-plan
//! / credential-posture / staged-trust / bootstrap-evidence truth integrity, and the checked-in support export
//! / CSV / report.

use super::*;

fn row(id: &str) -> RepositoryBootstrapAccessibilityRow {
    seeded_m5_repository_bootstrap_a11y_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_repository_bootstrap_a11y_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified() {
    let packet = seeded_m5_repository_bootstrap_a11y_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5RepositoryBootstrapFamily::ALL.len()
    );
    // Five rows cover the five families one-to-one (one certified fully green — the open-local family whose
    // existing checkout is located rather than recloned — the other four narrowed-yellow: three auto-narrowed
    // claims plus the reviewable clone-remote surface whose high-zoom traversal narrows to a disclosed reflow
    // walk).
    assert_eq!(packet.rows.len(), 5);
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_repository_bootstrap_a11y_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5RepositoryBootstrapClaimDimension::ALL.len()
    );
}

#[test]
fn every_condition_state_is_exercised() {
    let packet = seeded_m5_repository_bootstrap_a11y_packet();
    let states = packet.exercised_condition_states();
    for state in M5RepositoryBootstrapConditionState::ALL {
        assert!(
            states.contains(&state),
            "condition state {} is not exercised",
            state.as_str()
        );
    }
}

#[test]
fn every_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_repository_bootstrap_a11y_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5RepositoryBootstrapA11yClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_repository_bootstrap_a11y_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5RepositoryBootstrapConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_one_green_four_yellow_zero_red() {
    let packet = seeded_m5_repository_bootstrap_a11y_packet();
    assert_eq!(packet.summary.green_count, 1);
    assert_eq!(packet.summary.yellow_count, 4);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary.row_count, 5);
    assert_eq!(
        packet.summary.family_count,
        M5RepositoryBootstrapFamily::ALL.len()
    );
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn summary_reports_all_truth_and_trusted_honesty() {
    let packet = seeded_m5_repository_bootstrap_a11y_packet();
    assert!(packet.summary.all_truth_preserved);
    assert!(packet.summary.all_trusted_honesty_holds);
    assert!(packet.summary.all_claims_honest);
    assert!(packet.summary.all_export_summaries_preserve_meaning);
    assert!(packet.summary.all_reach_canonical_truth_via_at);
    assert!(packet.summary.all_narrowing_disclosed);
    assert!(packet.summary.all_structure_heavy_have_non_visual_fallback);
    assert_eq!(packet.summary.structure_heavy_family_count, 3);
}

// --- auto-narrowing per family ---

#[test]
fn open_local_is_trusted_and_green() {
    let conv = row("a11y:open-local-source-locator-resolved");
    assert_eq!(
        conv.full_ready_claim,
        M5RepositoryBootstrapA11yClaim::TrustedAcquisitionSurface
    );
    assert_eq!(
        conv.effective_claim(),
        M5RepositoryBootstrapA11yClaim::TrustedAcquisitionSurface
    );
    assert!(conv.claim_narrow.is_none());
    assert_eq!(
        conv.status(),
        RepositoryBootstrapAccessibilityStatus::Parity
    );
    assert!(conv.effective_claim().asserts_trusted_surface());
}

#[test]
fn unconfirmed_trust_stage_narrows_and_is_never_trusted() {
    let bundle = row("a11y:import-bundle-trust-stage-unconfirmed");
    assert_eq!(
        bundle.effective_claim(),
        M5RepositoryBootstrapA11yClaim::TrustStageUnverifiedProjection
    );
    assert!(!bundle.effective_claim().asserts_trusted_surface());
    assert!(bundle.trusted_honesty_holds());
    assert_eq!(
        bundle.status(),
        RepositoryBootstrapAccessibilityStatus::NarrowedDisclosed
    );
    let narrow = bundle.claim_narrow.as_ref().unwrap();
    assert_eq!(
        narrow.trigger,
        M5RepositoryBootstrapDowngradeTrigger::StagedTrustRuleUnstated
    );
    assert_eq!(
        narrow.binding_dimension,
        M5RepositoryBootstrapClaimDimension::TrustStageFenceClarity
    );
}

#[test]
fn each_weak_family_narrows_to_its_permitted_ceiling() {
    for (id, expected) in [
        (
            "a11y:open-archive-checkout-plan-disclosed-partial",
            M5RepositoryBootstrapA11yClaim::CheckoutPlanDisclosedProjection,
        ),
        (
            "a11y:import-bundle-trust-stage-unconfirmed",
            M5RepositoryBootstrapA11yClaim::TrustStageUnverifiedProjection,
        ),
        (
            "a11y:resume-snapshot-bootstrap-evidence-unconfirmed",
            M5RepositoryBootstrapA11yClaim::BootstrapEvidenceUnverifiedProjection,
        ),
    ] {
        let r = row(id);
        assert_eq!(r.effective_claim(), expected, "row {id}");
        assert_eq!(r.permitted_claim(), expected, "row {id} permitted");
        assert!(r.claim_is_honest(), "row {id} honest");
        assert_eq!(
            r.status(),
            RepositoryBootstrapAccessibilityStatus::NarrowedDisclosed,
            "row {id} yellow"
        );
    }
}

#[test]
fn cannot_be_shown_trusted_flags_exactly_the_overclaim_states() {
    use M5RepositoryBootstrapConditionState as C;
    assert!(C::TrustStageUnconfirmed.cannot_be_shown_trusted());
    assert!(C::BootstrapEvidenceUnconfirmed.cannot_be_shown_trusted());
    // An honest disclosed-absence operation (a partial checkout-plan proof) is not a truth overstatement.
    assert!(!C::CheckoutPlanDisclosedPartial.cannot_be_shown_trusted());
    assert!(!C::FullyQualified.cannot_be_shown_trusted());
}

#[test]
fn clone_remote_is_reviewable_and_yellow() {
    let topo = row("a11y:clone-remote-credential-posture-disclosed");
    assert!(topo.has_non_visual_fallback());
    assert_eq!(
        topo.effective_claim(),
        M5RepositoryBootstrapA11yClaim::ReviewableAcquisitionSurface
    );
    assert!(topo.claim_narrow.is_none());
    assert_eq!(
        topo.status(),
        RepositoryBootstrapAccessibilityStatus::NarrowedDisclosed
    );
}

#[test]
fn structure_heavy_resume_binds_a_structured_and_non_visual_path() {
    let resume = row("a11y:resume-snapshot-bootstrap-evidence-unconfirmed");
    assert!(resume.is_structure_heavy());
    assert!(resume.has_non_visual_fallback());
    assert!(resume
        .fallback_modalities
        .contains(&M5RepositoryBootstrapFallbackModality::Structured));
}

#[test]
fn aged_out_bootstrap_evidence_is_never_shown_as_trusted() {
    let resume = row("a11y:resume-snapshot-bootstrap-evidence-unconfirmed");
    assert!(!resume.effective_claim().asserts_trusted_surface());
    assert!(resume.trusted_honesty_holds());
    let narrow = resume.claim_narrow.as_ref().unwrap();
    assert_eq!(
        narrow.trigger,
        M5RepositoryBootstrapDowngradeTrigger::LostSignerOrMirrorProvenanceAcrossOfflineOrMirroredFetches
    );
}

#[test]
fn high_contrast_reach_is_exercised_as_a_disclosed_reduction() {
    let bundle = row("a11y:import-bundle-trust-stage-unconfirmed");
    assert!(bundle.high_contrast_reach.is_disclosed_reduction());
    assert!(bundle.high_contrast_reach.never_traps());
}

#[test]
fn localization_reach_is_exercised_as_a_disclosed_reduction() {
    let archive = row("a11y:open-archive-checkout-plan-disclosed-partial");
    assert!(archive.localization_reach.is_disclosed_reduction());
    assert!(archive.localization_reach.never_traps());
}

// --- negative / red-detection cases ---

#[test]
fn over_asserting_a_weak_state_is_stranded() {
    let mut bundle = row("a11y:import-bundle-trust-stage-unconfirmed");
    // Drop the narrow so the unconfirmed-staged-trust state keeps a trusted claim.
    bundle.claim_narrow = None;
    assert!(!bundle.claim_is_honest());
    assert!(!bundle.trusted_honesty_holds());
    assert_eq!(
        bundle.status(),
        RepositoryBootstrapAccessibilityStatus::Stranded
    );
}

#[test]
fn an_entry_chrome_only_view_only_trap_is_stranded() {
    let mut topo = row("a11y:clone-remote-credential-posture-disclosed");
    topo.keyboard_reach = RepositoryBootstrapNonVisualReachState::ViewOnlyTrap;
    assert!(!topo.reaches_canonical_truth_via_at());
    assert_eq!(
        topo.status(),
        RepositoryBootstrapAccessibilityStatus::Stranded
    );
}

#[test]
fn a_localization_trap_is_stranded() {
    let mut conv = row("a11y:open-local-source-locator-resolved");
    conv.localization_reach = RepositoryBootstrapNonVisualReachState::ViewOnlyTrap;
    assert!(!conv.reaches_canonical_truth_via_at());
    assert_eq!(
        conv.status(),
        RepositoryBootstrapAccessibilityStatus::Stranded
    );
}

#[test]
fn a_raw_payload_only_export_is_stranded() {
    let mut conv = row("a11y:open-local-source-locator-resolved");
    conv.export_summary = RepositoryBootstrapExportSummaryState::RequiresRawPayload;
    assert!(!conv.export_preserves_meaning());
    assert_eq!(
        conv.status(),
        RepositoryBootstrapAccessibilityStatus::Stranded
    );
}

#[test]
fn a_generic_narrow_label_is_dishonest() {
    let mut bundle = row("a11y:import-bundle-trust-stage-unconfirmed");
    bundle.claim_narrow.as_mut().unwrap().narrowed_label = "stale".to_owned();
    assert!(!bundle.claim_is_honest());
}

#[test]
fn dropping_a_family_reports_missing_coverage() {
    let mut packet = seeded_m5_repository_bootstrap_a11y_packet();
    packet.rows.pop();
    let mut packet =
        RepositoryBootstrapAccessibilityPacket::new(RepositoryBootstrapAccessibilityPacketInput {
            packet_id: packet.packet_id,
            as_of: packet.as_of,
            matrix_ref: packet.matrix_ref,
            rows: packet.rows,
        });
    packet.summary = packet.computed_summary();
    let tokens: Vec<&str> = packet.validate().iter().map(|v| v.as_str()).collect();
    assert!(tokens.contains(&"missing_family_coverage"));
}

// --- forbidden material ---

#[test]
fn forbidden_material_in_export_is_rejected() {
    let mut packet = seeded_m5_repository_bootstrap_a11y_packet();
    packet.rows[0]
        .copy_export
        .export_fields
        .push("bearer abc".to_owned());
    let tokens: Vec<&str> = packet.validate().iter().map(|v| v.as_str()).collect();
    assert!(tokens.contains(&"raw_repository_bootstrap_material_in_export"));
}

// --- rendering ---

#[test]
fn csv_has_a_row_per_certified_family() {
    let packet = seeded_m5_repository_bootstrap_a11y_packet();
    let csv = packet.render_matrix_csv();
    // header + 5 rows + trailing newline
    assert_eq!(csv.lines().count(), 6);
    for row in &packet.rows {
        assert!(csv.contains(&row.row_id));
    }
}

#[test]
fn markdown_summary_lists_every_row_and_narrowing() {
    let packet = seeded_m5_repository_bootstrap_a11y_packet();
    let md = packet.render_markdown_summary();
    for row in &packet.rows {
        assert!(md.contains(&row.row_id));
    }
    assert!(md.contains("Auto-narrow"));
}

// --- byte-lock against the checked-in artifacts ---

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_m5_repository_bootstrap_a11y_export()
        .expect("checked M5 repository-bootstrap a11y export validates");
    assert_eq!(from_disk.packet_id, REPOSITORY_BOOTSTRAP_A11Y_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_repository_bootstrap_a11y_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn checked_matrix_csv_matches_builder() {
    let expected = seeded_m5_repository_bootstrap_a11y_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-repository-bootstrap-accessibility-parity/matrix.csv"
    ));
    assert_eq!(
        on_disk, expected,
        "checked matrix CSV drifted from the builder"
    );
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_repository_bootstrap_a11y_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-repository-bootstrap-accessibility-parity.md"
    ));
    assert_eq!(on_disk, expected, "checked report drifted from the builder");
}

#[test]
fn checked_fixtures_mirror_the_release_artifacts() {
    let fixture_export = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/workspaces/m5-repository-bootstrap-accessibility-parity/support_export.json"
    ));
    let packet: RepositoryBootstrapAccessibilityPacket =
        serde_json::from_str(fixture_export).expect("fixture export parses");
    assert!(packet.validate().is_empty());
    assert_eq!(packet, seeded_m5_repository_bootstrap_a11y_packet());

    let fixture_csv = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/workspaces/m5-repository-bootstrap-accessibility-parity/matrix.csv"
    ));
    assert_eq!(
        fixture_csv,
        seeded_m5_repository_bootstrap_a11y_packet().render_matrix_csv()
    );
}

// --- gated artifact regeneration ---
//
// Set `GEN_REPOSITORY_BOOTSTRAP_A11Y_ARTIFACTS=1` to (re)write the checked-in release artifacts and fixtures
// from the seed builder. Off by default so a normal `cargo test` never mutates the tree.
#[test]
fn regenerate_checked_artifacts_when_requested() {
    if std::env::var("GEN_REPOSITORY_BOOTSTRAP_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_repository_bootstrap_a11y_packet();
    assert!(
        packet.validate().is_empty(),
        "seed must validate before write"
    );

    let manifest = env!("CARGO_MANIFEST_DIR");
    let repo = Path::new(manifest).join("../..");

    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let md = packet.render_markdown_summary();

    let release_dir = repo.join("artifacts/release/m5-repository-bootstrap-accessibility-parity");
    fs::create_dir_all(&release_dir).expect("create release dir");
    fs::write(release_dir.join("support_export.json"), &json).expect("write release export");
    fs::write(release_dir.join("matrix.csv"), &csv).expect("write release csv");
    fs::write(
        repo.join("artifacts/release/m5-repository-bootstrap-accessibility-parity.md"),
        &md,
    )
    .expect("write release report");

    let fixture_dir = repo.join("fixtures/workspaces/m5-repository-bootstrap-accessibility-parity");
    fs::create_dir_all(&fixture_dir).expect("create fixture dir");
    fs::write(fixture_dir.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixture_dir.join("matrix.csv"), &csv).expect("write fixture csv");
}
