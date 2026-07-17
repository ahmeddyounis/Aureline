//! Tests for the M05-1282 review-pack accessibility parity capstone: the honest auto-narrowing logic,
//! the per-object parity contract, the weakened-never-trusted guarantee, no-loss pack / ownership / evidence
//! / parity / AI-policy / template truth integrity, and the checked-in support export / CSV / report.

use super::*;

fn row(id: &str) -> ReviewPackAccessibilityRow {
    seeded_m5_review_pack_accessibility_parity_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_review_pack_accessibility_parity_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_object_is_certified() {
    let packet = seeded_m5_review_pack_accessibility_parity_packet();
    assert_eq!(
        packet.represented_objects().len(),
        M5ReviewPackObject::ALL.len()
    );
    // Eight rows cover the six objects (two certified fully green — the fresh review-pack record and the
    // attribution-bound review-template packet — the other six auto-narrowed yellow across the six spec
    // narrowing axes: pack version / digest, owner provenance, evidence-check state, local-versus-provider
    // parity, AI pack binding, and template attribution).
    assert_eq!(packet.rows.len(), 8);
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_review_pack_accessibility_parity_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5ReviewPackClaimDimension::ALL.len()
    );
}

#[test]
fn every_condition_state_is_exercised() {
    let packet = seeded_m5_review_pack_accessibility_parity_packet();
    let states = packet.exercised_condition_states();
    for state in M5ReviewPackConditionState::ALL {
        assert!(
            states.contains(&state),
            "condition state {} is not exercised",
            state.as_str()
        );
    }
}

#[test]
fn every_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_review_pack_accessibility_parity_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5ReviewPackA11yClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_review_pack_accessibility_parity_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5ReviewPackConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_two_green_six_yellow_zero_red() {
    let packet = seeded_m5_review_pack_accessibility_parity_packet();
    assert_eq!(packet.summary.green_count, 2);
    assert_eq!(packet.summary.yellow_count, 6);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary.row_count, 8);
    assert_eq!(packet.summary.object_count, M5ReviewPackObject::ALL.len());
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn summary_reports_all_truth_and_trusted_honesty() {
    let packet = seeded_m5_review_pack_accessibility_parity_packet();
    assert!(packet.summary.all_truth_preserved);
    assert!(packet.summary.all_trusted_honesty_holds);
    assert!(packet.summary.all_claims_honest);
    assert!(packet.summary.all_export_summaries_preserve_meaning);
    assert!(packet.summary.all_reach_canonical_truth_via_at);
    assert!(packet.summary.all_narrowing_disclosed);
    assert!(packet.summary.all_structure_heavy_have_non_visual_fallback);
    // Rows 2, 5, and 8 key on structure-heavy objects (review-template packet ×2, required-evidence-check
    // row ×1).
    assert_eq!(packet.summary.structure_heavy_object_count, 3);
}

// --- auto-narrowing per object ---

#[test]
fn fresh_record_is_trusted_and_green() {
    let record = row("a11y:review-pack-record-fresh-pack-version");
    assert_eq!(
        record.full_ready_claim,
        M5ReviewPackA11yClaim::TrustedReviewSurface
    );
    assert_eq!(
        record.effective_claim(),
        M5ReviewPackA11yClaim::TrustedReviewSurface
    );
    assert!(record.claim_narrow.is_none());
    assert_eq!(record.status(), ReviewPackAccessibilityStatus::Parity);
    assert!(record.effective_claim().asserts_trusted_surface());
}

#[test]
fn stale_pack_record_narrows_and_is_never_trusted() {
    let record = row("a11y:review-pack-record-pack-version-stale");
    assert_eq!(
        record.effective_claim(),
        M5ReviewPackA11yClaim::PackVersionUnverifiedProjection
    );
    assert!(!record.effective_claim().asserts_trusted_surface());
    assert!(record.trusted_honesty_holds());
    assert_eq!(
        record.status(),
        ReviewPackAccessibilityStatus::NarrowedDisclosed
    );
    let narrow = record.claim_narrow.as_ref().unwrap();
    assert_eq!(
        narrow.trigger,
        M5ReviewPackDowngradeTrigger::PackVersionOrDigestDropped
    );
    assert_eq!(
        narrow.binding_dimension,
        M5ReviewPackClaimDimension::PackVersionDigestClarity
    );
}

#[test]
fn each_weak_object_narrows_to_its_permitted_ceiling() {
    for (id, expected) in [
        (
            "a11y:review-pack-record-pack-version-stale",
            M5ReviewPackA11yClaim::PackVersionUnverifiedProjection,
        ),
        (
            "a11y:ownership-signal-owner-provenance-missing",
            M5ReviewPackA11yClaim::OwnerProvenanceUnverifiedProjection,
        ),
        (
            "a11y:required-evidence-check-row-check-unevaluated",
            M5ReviewPackA11yClaim::EvidenceCheckUnverifiedProjection,
        ),
        (
            "a11y:local-ci-parity-strip-capability-difference",
            M5ReviewPackA11yClaim::LocalParityUnverifiedProjection,
        ),
        (
            "a11y:ai-policy-hook-undisclosed-pack-version",
            M5ReviewPackA11yClaim::AiPackVersionUnverifiedProjection,
        ),
        (
            "a11y:review-template-packet-attribution-stale",
            M5ReviewPackA11yClaim::TemplateAttributionUnverifiedProjection,
        ),
    ] {
        let r = row(id);
        assert_eq!(r.effective_claim(), expected, "row {id}");
        assert_eq!(r.permitted_claim(), expected, "row {id} permitted");
        assert!(r.claim_is_honest(), "row {id} honest");
        assert_eq!(
            r.status(),
            ReviewPackAccessibilityStatus::NarrowedDisclosed,
            "row {id} yellow"
        );
    }
}

#[test]
fn cannot_be_shown_trusted_flags_every_weak_state() {
    use M5ReviewPackConditionState as C;
    assert!(C::PackVersionDigestStale.cannot_be_shown_trusted());
    assert!(C::OwnerProvenanceMissing.cannot_be_shown_trusted());
    assert!(C::EvidenceCheckUnevaluated.cannot_be_shown_trusted());
    assert!(C::LocalParityCapabilityDifference.cannot_be_shown_trusted());
    assert!(C::AiPackVersionUndisclosed.cannot_be_shown_trusted());
    assert!(C::TemplateAttributionStale.cannot_be_shown_trusted());
    assert!(!C::FullyQualified.cannot_be_shown_trusted());
}

#[test]
fn bound_template_packet_is_reviewable_and_green() {
    let template = row("a11y:review-template-packet-attribution-bound");
    assert!(template.is_structure_heavy());
    assert!(template.has_non_visual_fallback());
    assert_eq!(
        template.effective_claim(),
        M5ReviewPackA11yClaim::ReviewableReviewSurface
    );
    assert!(template.claim_narrow.is_none());
    assert_eq!(template.status(), ReviewPackAccessibilityStatus::Parity);
}

#[test]
fn structure_heavy_evidence_row_binds_a_structured_and_non_visual_path() {
    let evidence = row("a11y:required-evidence-check-row-check-unevaluated");
    assert!(evidence.is_structure_heavy());
    assert!(evidence.has_non_visual_fallback());
    assert!(evidence
        .fallback_modalities
        .contains(&M5ReviewPackFallbackModality::Structured));
}

#[test]
fn local_parity_estimate_never_shown_as_provider_authoritative() {
    let parity = row("a11y:local-ci-parity-strip-capability-difference");
    assert!(!parity.effective_claim().asserts_trusted_surface());
    assert!(parity.trusted_honesty_holds());
    let narrow = parity.claim_narrow.as_ref().unwrap();
    assert_eq!(
        narrow.trigger,
        M5ReviewPackDowngradeTrigger::LocalEstimateShownAsProviderAuthoritative
    );
}

// --- negative / red-detection cases ---

#[test]
fn over_asserting_a_weak_state_is_stranded() {
    let mut record = row("a11y:review-pack-record-pack-version-stale");
    // Drop the narrow so the stale-pack state keeps a trusted claim.
    record.claim_narrow = None;
    assert!(!record.claim_is_honest());
    assert!(!record.trusted_honesty_holds());
    assert_eq!(record.status(), ReviewPackAccessibilityStatus::Stranded);
}

#[test]
fn a_view_only_hover_trap_is_stranded() {
    let mut template = row("a11y:review-template-packet-attribution-bound");
    template.keyboard_reach = ReviewPackNonVisualReachState::ViewOnlyTrap;
    assert!(!template.reaches_canonical_truth_via_at());
    assert_eq!(template.status(), ReviewPackAccessibilityStatus::Stranded);
}

#[test]
fn a_raw_payload_only_export_is_stranded() {
    let mut record = row("a11y:review-pack-record-fresh-pack-version");
    record.export_summary = ReviewPackExportSummaryState::RequiresRawPayload;
    assert!(!record.export_preserves_meaning());
    assert_eq!(record.status(), ReviewPackAccessibilityStatus::Stranded);
}

#[test]
fn a_generic_narrow_label_is_dishonest() {
    let mut record = row("a11y:review-pack-record-pack-version-stale");
    record.claim_narrow.as_mut().unwrap().narrowed_label = "stale".to_owned();
    assert!(!record.claim_is_honest());
}

#[test]
fn dropping_an_object_reports_missing_coverage() {
    let packet = seeded_m5_review_pack_accessibility_parity_packet();
    // Drop the only row that keys on the ownership signal to remove that object entirely.
    let rows: Vec<ReviewPackAccessibilityRow> = packet
        .rows
        .into_iter()
        .filter(|r| r.object != M5ReviewPackObject::OwnershipSignal)
        .collect();
    let packet = ReviewPackAccessibilityPacket::new(ReviewPackAccessibilityPacketInput {
        packet_id: packet.packet_id,
        as_of: packet.as_of,
        matrix_ref: packet.matrix_ref,
        rows,
    });
    let tokens: Vec<&str> = packet.validate().iter().map(|v| v.as_str()).collect();
    assert!(tokens.contains(&"missing_object_coverage"));
}

// --- forbidden material ---

#[test]
fn forbidden_material_in_export_is_rejected() {
    let mut packet = seeded_m5_review_pack_accessibility_parity_packet();
    packet.rows[0]
        .copy_export
        .export_fields
        .push("bearer abc".to_owned());
    let tokens: Vec<&str> = packet.validate().iter().map(|v| v.as_str()).collect();
    assert!(tokens.contains(&"raw_object_material_in_export"));
}

// --- rendering ---

#[test]
fn csv_has_a_row_per_certified_object() {
    let packet = seeded_m5_review_pack_accessibility_parity_packet();
    let csv = packet.render_matrix_csv();
    // header + 8 rows
    assert_eq!(csv.lines().count(), 9);
    for row in &packet.rows {
        assert!(csv.contains(&row.row_id));
    }
}

#[test]
fn markdown_summary_lists_every_row_and_narrowing() {
    let packet = seeded_m5_review_pack_accessibility_parity_packet();
    let md = packet.render_markdown_summary();
    for row in &packet.rows {
        assert!(md.contains(&row.row_id));
    }
    assert!(md.contains("Auto-narrow"));
}

// --- byte-lock against the checked-in artifacts ---

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_m5_review_pack_accessibility_parity_export()
        .expect("checked M5 review-pack a11y export validates");
    assert_eq!(from_disk.packet_id, REVIEW_PACK_A11Y_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_review_pack_accessibility_parity_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn checked_matrix_csv_matches_builder() {
    let expected = seeded_m5_review_pack_accessibility_parity_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5-review-pack-accessibility-parity/matrix.csv"
    ));
    assert_eq!(
        on_disk, expected,
        "checked matrix CSV drifted from the builder"
    );
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_review_pack_accessibility_parity_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5-review-pack-accessibility-parity.md"
    ));
    assert_eq!(on_disk, expected, "checked report drifted from the builder");
}

#[test]
fn checked_fixtures_mirror_the_release_artifacts() {
    let fixture_export = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/review/m5-review-pack-accessibility-parity/support_export.json"
    ));
    let packet: ReviewPackAccessibilityPacket =
        serde_json::from_str(fixture_export).expect("fixture export parses");
    assert!(packet.validate().is_empty());
    assert_eq!(packet, seeded_m5_review_pack_accessibility_parity_packet());

    let fixture_csv = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/review/m5-review-pack-accessibility-parity/matrix.csv"
    ));
    assert_eq!(
        fixture_csv,
        seeded_m5_review_pack_accessibility_parity_packet().render_matrix_csv()
    );
}

// --- gated artifact regeneration ---
//
// Set `GEN_REVIEW_PACK_A11Y_ARTIFACTS=1` to (re)write the checked-in release artifacts and fixtures from the
// seed builder. Off by default so a normal `cargo test` never mutates the tree.
#[test]
fn regenerate_checked_artifacts_when_requested() {
    if std::env::var("GEN_REVIEW_PACK_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_review_pack_accessibility_parity_packet();
    assert!(
        packet.validate().is_empty(),
        "seed must validate before write"
    );

    let manifest = env!("CARGO_MANIFEST_DIR");
    let repo = Path::new(manifest).join("../..");

    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let md = packet.render_markdown_summary();

    let release_dir = repo.join("artifacts/review/m5-review-pack-accessibility-parity");
    fs::create_dir_all(&release_dir).expect("create release dir");
    fs::write(release_dir.join("support_export.json"), &json).expect("write release export");
    fs::write(release_dir.join("matrix.csv"), &csv).expect("write release csv");
    fs::write(
        repo.join("artifacts/review/m5-review-pack-accessibility-parity.md"),
        &md,
    )
    .expect("write release report");

    let fixture_dir = repo.join("fixtures/review/m5-review-pack-accessibility-parity");
    fs::create_dir_all(&fixture_dir).expect("create fixture dir");
    fs::write(fixture_dir.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixture_dir.join("matrix.csv"), &csv).expect("write fixture csv");
}
