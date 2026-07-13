//! Tests for the M05-1170 platform-fit accessibility parity capstone: the honest auto-narrowing logic, the
//! per-family parity contract, the weakened-never-trusted guarantee, no-loss shortcut / path / appearance /
//! credential-wording / input-method truth integrity, and the checked-in support export / CSV / report.

use super::*;

fn row(id: &str) -> PlatformFitAccessibilityRow {
    seeded_m5_platform_fit_a11y_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_platform_fit_a11y_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified() {
    let packet = seeded_m5_platform_fit_a11y_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5PlatformFitFamily::ALL.len()
    );
    // Six rows cover the six families one-to-one (one certified fully green — the platform-convention family
    // whose window / menu / chrome stays host-correct — the other five narrowed-yellow: four auto-narrowed
    // claims plus the reviewable shortcut-notation surface whose high-zoom traversal narrows to a disclosed
    // reflow walk).
    assert_eq!(packet.rows.len(), 6);
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_platform_fit_a11y_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5PlatformFitClaimDimension::ALL.len()
    );
}

#[test]
fn every_condition_state_is_exercised() {
    let packet = seeded_m5_platform_fit_a11y_packet();
    let states = packet.exercised_condition_states();
    for state in M5PlatformFitConditionState::ALL {
        assert!(
            states.contains(&state),
            "condition state {} is not exercised",
            state.as_str()
        );
    }
}

#[test]
fn every_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_platform_fit_a11y_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5PlatformFitA11yClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_platform_fit_a11y_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5PlatformFitConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_one_green_five_yellow_zero_red() {
    let packet = seeded_m5_platform_fit_a11y_packet();
    assert_eq!(packet.summary.green_count, 1);
    assert_eq!(packet.summary.yellow_count, 5);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary.row_count, 6);
    assert_eq!(packet.summary.family_count, M5PlatformFitFamily::ALL.len());
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn summary_reports_all_truth_and_trusted_honesty() {
    let packet = seeded_m5_platform_fit_a11y_packet();
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
fn platform_convention_is_trusted_and_green() {
    let conv = row("a11y:platform-convention-window-menu-chrome-host-correct");
    assert_eq!(
        conv.full_ready_claim,
        M5PlatformFitA11yClaim::TrustedPlatformFitSurface
    );
    assert_eq!(
        conv.effective_claim(),
        M5PlatformFitA11yClaim::TrustedPlatformFitSurface
    );
    assert!(conv.claim_narrow.is_none());
    assert_eq!(conv.status(), PlatformFitAccessibilityStatus::Parity);
    assert!(conv.effective_claim().asserts_trusted_surface());
}

#[test]
fn unconfirmed_credential_wording_narrows_and_is_never_trusted() {
    let cred = row("a11y:credential-store-wording-truthful-non-leaky-unconfirmed");
    assert_eq!(
        cred.effective_claim(),
        M5PlatformFitA11yClaim::CredentialWordingUnverifiedProjection
    );
    assert!(!cred.effective_claim().asserts_trusted_surface());
    assert!(cred.trusted_honesty_holds());
    assert_eq!(
        cred.status(),
        PlatformFitAccessibilityStatus::NarrowedDisclosed
    );
    let narrow = cred.claim_narrow.as_ref().unwrap();
    assert_eq!(
        narrow.trigger,
        M5PlatformFitDowngradeTrigger::SecretStorageFellBackToPlaintextSilently
    );
    assert_eq!(
        narrow.binding_dimension,
        M5PlatformFitClaimDimension::CredentialWordingClarity
    );
}

#[test]
fn each_weak_family_narrows_to_its_permitted_ceiling() {
    for (id, expected) in [
        (
            "a11y:file-path-reveal-localization-disclosed-partial",
            M5PlatformFitA11yClaim::PathTerminologyDisclosedProjection,
        ),
        (
            "a11y:theme-contrast-live-apply-unconfirmed",
            M5PlatformFitA11yClaim::AppearanceResponseUnverifiedProjection,
        ),
        (
            "a11y:credential-store-wording-truthful-non-leaky-unconfirmed",
            M5PlatformFitA11yClaim::CredentialWordingUnverifiedProjection,
        ),
        (
            "a11y:input-method-text-trust-fidelity-unconfirmed",
            M5PlatformFitA11yClaim::InputFidelityUnverifiedProjection,
        ),
    ] {
        let r = row(id);
        assert_eq!(r.effective_claim(), expected, "row {id}");
        assert_eq!(r.permitted_claim(), expected, "row {id} permitted");
        assert!(r.claim_is_honest(), "row {id} honest");
        assert_eq!(
            r.status(),
            PlatformFitAccessibilityStatus::NarrowedDisclosed,
            "row {id} yellow"
        );
    }
}

#[test]
fn cannot_be_shown_trusted_flags_exactly_the_overclaim_states() {
    use M5PlatformFitConditionState as C;
    assert!(C::AppearanceResponseUnconfirmed.cannot_be_shown_trusted());
    assert!(C::CredentialWordingUnconfirmed.cannot_be_shown_trusted());
    assert!(C::InputFidelityUnconfirmed.cannot_be_shown_trusted());
    // An honest disclosed-absence operation (a partial path localization) is not a truth overstatement.
    assert!(!C::PathTerminologyDisclosedPartial.cannot_be_shown_trusted());
    assert!(!C::FullyQualified.cannot_be_shown_trusted());
}

#[test]
fn shortcut_notation_is_reviewable_and_yellow() {
    let shortcut = row("a11y:shortcut-notation-adapts-from-one-registry");
    assert!(shortcut.has_non_visual_fallback());
    assert_eq!(
        shortcut.effective_claim(),
        M5PlatformFitA11yClaim::ReviewablePlatformFitSurface
    );
    assert!(shortcut.claim_narrow.is_none());
    assert_eq!(
        shortcut.status(),
        PlatformFitAccessibilityStatus::NarrowedDisclosed
    );
}

#[test]
fn structure_heavy_input_method_binds_a_structured_and_non_visual_path() {
    let input = row("a11y:input-method-text-trust-fidelity-unconfirmed");
    assert!(input.is_structure_heavy());
    assert!(input.has_non_visual_fallback());
    assert!(input
        .fallback_modalities
        .contains(&M5PlatformFitFallbackModality::Structured));
}

#[test]
fn unconfirmed_appearance_is_never_shown_as_trusted() {
    let theme = row("a11y:theme-contrast-live-apply-unconfirmed");
    assert!(!theme.effective_claim().asserts_trusted_surface());
    assert!(theme.trusted_honesty_holds());
    let narrow = theme.claim_narrow.as_ref().unwrap();
    assert_eq!(
        narrow.trigger,
        M5PlatformFitDowngradeTrigger::ThemeOrContrastChangeDidNotApplyLiveOrExplainFallback
    );
}

#[test]
fn high_contrast_reach_is_exercised_as_a_disclosed_reduction() {
    let theme = row("a11y:theme-contrast-live-apply-unconfirmed");
    assert!(theme.high_contrast_reach.is_disclosed_reduction());
    assert!(theme.high_contrast_reach.never_traps());
}

#[test]
fn localization_reach_is_exercised_as_a_disclosed_reduction() {
    let path = row("a11y:file-path-reveal-localization-disclosed-partial");
    assert!(path.localization_reach.is_disclosed_reduction());
    assert!(path.localization_reach.never_traps());
}

// --- negative / red-detection cases ---

#[test]
fn over_asserting_a_weak_state_is_stranded() {
    let mut cred = row("a11y:credential-store-wording-truthful-non-leaky-unconfirmed");
    // Drop the narrow so the unconfirmed-credential state keeps a trusted claim.
    cred.claim_narrow = None;
    assert!(!cred.claim_is_honest());
    assert!(!cred.trusted_honesty_holds());
    assert_eq!(cred.status(), PlatformFitAccessibilityStatus::Stranded);
}

#[test]
fn an_os_chrome_only_view_only_trap_is_stranded() {
    let mut shortcut = row("a11y:shortcut-notation-adapts-from-one-registry");
    shortcut.keyboard_reach = PlatformFitNonVisualReachState::ViewOnlyTrap;
    assert!(!shortcut.reaches_canonical_truth_via_at());
    assert_eq!(shortcut.status(), PlatformFitAccessibilityStatus::Stranded);
}

#[test]
fn a_localization_trap_is_stranded() {
    let mut conv = row("a11y:platform-convention-window-menu-chrome-host-correct");
    conv.localization_reach = PlatformFitNonVisualReachState::ViewOnlyTrap;
    assert!(!conv.reaches_canonical_truth_via_at());
    assert_eq!(conv.status(), PlatformFitAccessibilityStatus::Stranded);
}

#[test]
fn a_raw_payload_only_export_is_stranded() {
    let mut conv = row("a11y:platform-convention-window-menu-chrome-host-correct");
    conv.export_summary = PlatformFitExportSummaryState::RequiresRawPayload;
    assert!(!conv.export_preserves_meaning());
    assert_eq!(conv.status(), PlatformFitAccessibilityStatus::Stranded);
}

#[test]
fn a_generic_narrow_label_is_dishonest() {
    let mut cred = row("a11y:credential-store-wording-truthful-non-leaky-unconfirmed");
    cred.claim_narrow.as_mut().unwrap().narrowed_label = "stale".to_owned();
    assert!(!cred.claim_is_honest());
}

#[test]
fn dropping_a_family_reports_missing_coverage() {
    let mut packet = seeded_m5_platform_fit_a11y_packet();
    packet.rows.pop();
    let mut packet = PlatformFitAccessibilityPacket::new(PlatformFitAccessibilityPacketInput {
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
    let mut packet = seeded_m5_platform_fit_a11y_packet();
    packet.rows[0]
        .copy_export
        .export_fields
        .push("bearer abc".to_owned());
    let tokens: Vec<&str> = packet.validate().iter().map(|v| v.as_str()).collect();
    assert!(tokens.contains(&"raw_platform_fit_material_in_export"));
}

// --- rendering ---

#[test]
fn csv_has_a_row_per_certified_family() {
    let packet = seeded_m5_platform_fit_a11y_packet();
    let csv = packet.render_matrix_csv();
    // header + 6 rows + trailing newline
    assert_eq!(csv.lines().count(), 7);
    for row in &packet.rows {
        assert!(csv.contains(&row.row_id));
    }
}

#[test]
fn markdown_summary_lists_every_row_and_narrowing() {
    let packet = seeded_m5_platform_fit_a11y_packet();
    let md = packet.render_markdown_summary();
    for row in &packet.rows {
        assert!(md.contains(&row.row_id));
    }
    assert!(md.contains("Auto-narrow"));
}

// --- byte-lock against the checked-in artifacts ---

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_m5_platform_fit_a11y_export()
        .expect("checked M5 platform-fit a11y export validates");
    assert_eq!(from_disk.packet_id, PLATFORM_FIT_A11Y_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_platform_fit_a11y_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn checked_matrix_csv_matches_builder() {
    let expected = seeded_m5_platform_fit_a11y_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-platform-fit-accessibility-parity/matrix.csv"
    ));
    assert_eq!(
        on_disk, expected,
        "checked matrix CSV drifted from the builder"
    );
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_platform_fit_a11y_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-platform-fit-accessibility-parity.md"
    ));
    assert_eq!(on_disk, expected, "checked report drifted from the builder");
}

#[test]
fn checked_fixtures_mirror_the_release_artifacts() {
    let fixture_export = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/platform/m5-platform-fit-accessibility-parity/support_export.json"
    ));
    let packet: PlatformFitAccessibilityPacket =
        serde_json::from_str(fixture_export).expect("fixture export parses");
    assert!(packet.validate().is_empty());
    assert_eq!(packet, seeded_m5_platform_fit_a11y_packet());

    let fixture_csv = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/platform/m5-platform-fit-accessibility-parity/matrix.csv"
    ));
    assert_eq!(
        fixture_csv,
        seeded_m5_platform_fit_a11y_packet().render_matrix_csv()
    );
}

// --- gated artifact regeneration ---
//
// Set `GEN_PLATFORM_FIT_A11Y_ARTIFACTS=1` to (re)write the checked-in release artifacts and fixtures from the
// seed builder. Off by default so a normal `cargo test` never mutates the tree.
#[test]
fn regenerate_checked_artifacts_when_requested() {
    if std::env::var("GEN_PLATFORM_FIT_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_platform_fit_a11y_packet();
    assert!(
        packet.validate().is_empty(),
        "seed must validate before write"
    );

    let manifest = env!("CARGO_MANIFEST_DIR");
    let repo = Path::new(manifest).join("../..");

    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let md = packet.render_markdown_summary();

    let release_dir = repo.join("artifacts/release/m5-platform-fit-accessibility-parity");
    fs::create_dir_all(&release_dir).expect("create release dir");
    fs::write(release_dir.join("support_export.json"), &json).expect("write release export");
    fs::write(release_dir.join("matrix.csv"), &csv).expect("write release csv");
    fs::write(
        repo.join("artifacts/release/m5-platform-fit-accessibility-parity.md"),
        &md,
    )
    .expect("write release report");

    let fixture_dir = repo.join("fixtures/platform/m5-platform-fit-accessibility-parity");
    fs::create_dir_all(&fixture_dir).expect("create fixture dir");
    fs::write(fixture_dir.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixture_dir.join("matrix.csv"), &csv).expect("write fixture csv");
}
