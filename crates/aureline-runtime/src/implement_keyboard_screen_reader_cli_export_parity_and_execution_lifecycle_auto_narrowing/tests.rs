//! Tests for the M05-826 execution-lifecycle component accessibility fallback and
//! auto-narrowing packet.

use super::*;

fn packet() -> ExecutionAccessibilityPacket {
    seeded_m5_execution_a11y_fallback_packet()
}

fn row(id: &str) -> ExecutionAccessibilityRow {
    packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

#[test]
fn seeded_packet_validates_clean() {
    let violations = packet().validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn packet_identity_is_stamped() {
    let p = packet();
    assert_eq!(p.record_kind, EXECUTION_A11Y_FALLBACK_RECORD_KIND);
    assert_eq!(p.schema_version, EXECUTION_A11Y_FALLBACK_SCHEMA_VERSION);
    assert_eq!(p.matrix_ref, EXECUTION_A11Y_FALLBACK_COMPONENT_MATRIX_REF);
}

#[test]
fn every_frozen_family_is_certified() {
    let families = packet().represented_families();
    for family in M5ExecutionComponentFamily::ALL {
        assert!(
            families.contains(&family),
            "family {family:?} is not certified"
        );
    }
    assert_eq!(families.len(), M5ExecutionComponentFamily::ALL.len());
}

#[test]
fn every_claim_dimension_is_exercised() {
    let exercised = packet().exercised_dimensions();
    for dimension in M5ExecutionClaimDimension::ALL {
        assert!(
            exercised.contains(&dimension),
            "dimension {dimension:?} is not exercised"
        );
    }
}

#[test]
fn every_claim_tier_appears_as_effective_claim() {
    let effective = packet().represented_effective_claims();
    for claim in M5ExecutionInteractiveClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {claim:?} never appears as an effective claim"
        );
    }
}

#[test]
fn summary_matches_computed() {
    let p = packet();
    assert_eq!(p.summary, p.computed_summary());
}

#[test]
fn seeded_status_split_is_two_green_five_yellow_zero_red() {
    let p = packet();
    assert_eq!(p.summary.green_count, 2, "green");
    assert_eq!(p.summary.yellow_count, 5, "yellow");
    assert_eq!(p.summary.red_count, 0, "red");
}

#[test]
fn hierarchy_heavy_families_offer_a_non_visual_fallback() {
    for row in &packet().rows {
        if row.is_hierarchy_heavy() {
            assert!(
                row.fallback_modalities
                    .contains(&M5FallbackModality::Structured),
                "{} must render a structured modality",
                row.row_id
            );
            assert!(
                row.has_non_visual_fallback(),
                "{} must offer a non-visual fallback",
                row.row_id
            );
        }
    }
}

#[test]
fn ac1_intact_lane_asserts_full_claim_without_narrowing() {
    let r = row("a11y:run-attempt-header");
    assert!(!r.has_weak_dimension());
    assert!(r.claim_narrow.is_none());
    assert_eq!(
        r.permitted_claim(),
        M5ExecutionInteractiveClaim::FullInteractive
    );
    assert_eq!(
        r.effective_claim(),
        M5ExecutionInteractiveClaim::FullInteractive
    );
    assert!(r.claim_is_honest());
    assert_eq!(r.status(), ExecutionAccessibilityStatus::Parity);
}

#[test]
fn ac1_policy_blocked_input_narrows_to_inspect_only() {
    let r = row("a11y:input-request-prompt");
    assert_eq!(
        r.condition_for(M5ExecutionClaimDimension::InputState),
        M5ClaimConditionState::PolicyBlocked
    );
    assert_eq!(
        r.permitted_claim(),
        M5ExecutionInteractiveClaim::InspectOnly
    );
    assert_eq!(
        r.effective_claim(),
        M5ExecutionInteractiveClaim::InspectOnly
    );
    assert!(!r.effective_claim().asserts_control());
    assert!(r.claim_is_honest());
}

#[test]
fn ac1_stale_artifact_narrows_to_read_only() {
    let r = row("a11y:artifact-publish-row");
    assert_eq!(r.effective_claim(), M5ExecutionInteractiveClaim::ReadOnly);
    let narrow = r.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.binding_dimension,
        M5ExecutionClaimDimension::ArtifactFreshness
    );
    assert_eq!(
        narrow.trigger,
        M5ExecutionDowngradeTrigger::ArtifactRetentionExpired
    );
    assert!(narrow.preserves_canonical_identity);
}

#[test]
fn ac1_over_asserting_control_is_stranded() {
    let mut r = row("a11y:artifact-publish-row");
    // Drop the narrow while keeping the stale dimension: the row now over-claims.
    r.claim_narrow = None;
    assert!(!r.claim_is_honest());
    assert_eq!(r.status(), ExecutionAccessibilityStatus::Stranded);
}

#[test]
fn ac1_spurious_narrow_on_intact_lane_is_stranded() {
    let mut r = row("a11y:run-attempt-header");
    r.claim_narrow = Some(ClaimAutoNarrow {
        narrowed_to: M5ExecutionInteractiveClaim::ReadOnly,
        binding_dimension: M5ExecutionClaimDimension::AttemptLineage,
        trigger: M5ExecutionClaimDimension::AttemptLineage.default_trigger(),
        narrowed_label: "spurious".to_owned(),
        preserves_canonical_identity: true,
    });
    assert!(!r.claim_is_honest());
}

#[test]
fn ac1_narrow_must_match_permitted_ceiling() {
    let mut r = row("a11y:input-request-prompt");
    // Policy-blocked permits only inspect-only; claiming read-only over-asserts.
    if let Some(narrow) = r.claim_narrow.as_mut() {
        narrow.narrowed_to = M5ExecutionInteractiveClaim::ReadOnly;
    }
    assert!(!r.claim_is_honest());
}

#[test]
fn ac1_narrow_with_generic_label_is_dishonest() {
    let mut r = row("a11y:thread-process-tree");
    if let Some(narrow) = r.claim_narrow.as_mut() {
        narrow.narrowed_label = "unavailable".to_owned();
    }
    assert!(!r.claim_is_honest());
}

#[test]
fn ac1_narrow_dropping_identity_is_dishonest() {
    let mut r = row("a11y:thread-process-tree");
    if let Some(narrow) = r.claim_narrow.as_mut() {
        narrow.preserves_canonical_identity = false;
    }
    assert!(!r.claim_is_honest());
}

#[test]
fn ac2_view_only_trap_is_stranded() {
    let mut r = row("a11y:run-attempt-header");
    r.screen_reader_reach = NonVisualReachState::ViewOnlyTrap;
    assert!(!r.reaches_canonical_truth_via_at());
    assert_eq!(r.status(), ExecutionAccessibilityStatus::Stranded);
}

#[test]
fn ac2_cli_trap_is_stranded() {
    let mut r = row("a11y:run-attempt-header");
    r.cli_reach = NonVisualReachState::ViewOnlyTrap;
    assert!(!r.reaches_canonical_truth_via_at());
}

#[test]
fn ac2_empty_execution_context_is_stranded() {
    let mut r = row("a11y:run-attempt-header");
    r.execution_context_ref = "  ".to_owned();
    assert!(!r.reaches_canonical_truth_via_at());
}

#[test]
fn ac2_export_without_screenshot_parity_is_stranded() {
    let mut r = row("a11y:run-attempt-header");
    r.copy_export.formats = vec!["text".to_owned()];
    assert!(!r.export_preserves_meaning());
}

#[test]
fn ac2_screenshot_only_export_is_stranded() {
    let mut r = row("a11y:run-attempt-header");
    r.export_summary = ExportSummaryState::AbsentNeedsScreenshot;
    assert!(!r.export_preserves_meaning());
}

#[test]
fn ac3_narrowed_surface_without_disclosure_is_flagged() {
    let mut r = row("a11y:run-attempt-header");
    r.narrowing_disclosures.clear();
    assert!(!r.narrowing_disclosed());
}

#[test]
fn ac3_silently_dropped_surface_is_flagged() {
    let mut r = row("a11y:run-attempt-header");
    if let Some(d) = r.narrowing_disclosures.first_mut() {
        d.state = NarrowingDisclosureState::SilentlyDropped;
    }
    assert!(!r.narrowing_disclosed());
}

#[test]
fn ac3_narrowed_surface_must_preserve_labels() {
    let mut r = row("a11y:run-attempt-header");
    if let Some(d) = r.narrowing_disclosures.first_mut() {
        d.preserved_labels.clear();
    }
    assert!(!r.narrowing_disclosed());
}

#[test]
fn primary_dimension_must_be_modeled() {
    let mut r = row("a11y:input-request-prompt");
    r.claim_conditions.clear();
    r.claim_conditions.push(condition(
        M5ExecutionClaimDimension::TargetIdentity,
        M5ClaimConditionState::Intact,
    ));
    r.claim_narrow = None;
    assert!(!r.models_primary_dimension());
    assert_eq!(r.status(), ExecutionAccessibilityStatus::Stranded);
}

#[test]
fn missing_family_coverage_is_flagged() {
    let mut p = packet();
    p.rows
        .retain(|r| r.component_family != M5ExecutionComponentFamily::DumpCrashArtifactCard);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ExecutionAccessibilityViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut p = packet();
    let dup = p.rows[0].clone();
    p.rows.push(dup);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, ExecutionAccessibilityViolation::DuplicateId { .. })));
}

#[test]
fn missing_consumer_parity_is_flagged() {
    let mut p = packet();
    p.rows[0].consumer_surfaces = vec![M5RunAttemptSurfaceFamily::TaskRunPane];
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ExecutionAccessibilityViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut p = packet();
    p.rows[0].source_refs.push("bearer abc123".to_owned());
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ExecutionAccessibilityViolation::RawBoundaryMaterialInExport
    )));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut p = packet();
    p.summary.green_count += 1;
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, ExecutionAccessibilityViolation::SummaryMismatch)));
}

#[test]
fn permitted_claim_takes_the_weakest_ceiling() {
    let mut r = row("a11y:run-attempt-header");
    r.claim_conditions = vec![
        condition(
            M5ExecutionClaimDimension::AttemptLineage,
            M5ClaimConditionState::Partial,
        ),
        condition(
            M5ExecutionClaimDimension::TargetIdentity,
            M5ClaimConditionState::Unavailable,
        ),
    ];
    // Partial → review-required, Unavailable → inspect-only; the weakest wins.
    assert_eq!(
        r.permitted_claim(),
        M5ExecutionInteractiveClaim::InspectOnly
    );
    assert_eq!(
        r.binding_dimension(),
        Some(M5ExecutionClaimDimension::TargetIdentity)
    );
}

#[test]
fn claim_capability_ranks_are_ordered() {
    assert!(
        M5ExecutionInteractiveClaim::FullInteractive.capability_rank()
            > M5ExecutionInteractiveClaim::ReviewRequired.capability_rank()
    );
    assert!(
        M5ExecutionInteractiveClaim::ReviewRequired.capability_rank()
            > M5ExecutionInteractiveClaim::ReadOnly.capability_rank()
    );
    assert!(
        M5ExecutionInteractiveClaim::ReadOnly.capability_rank()
            > M5ExecutionInteractiveClaim::InspectOnly.capability_rank()
    );
}

#[test]
fn csv_has_a_header_and_one_row_each() {
    let csv = packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet().rows.len());
    assert!(lines[0].starts_with("row_id,component_family,"));
}

#[test]
fn markdown_summary_lists_every_row() {
    let md = packet().render_markdown_summary();
    for row in &packet().rows {
        assert!(md.contains(&row.row_id), "summary missing {}", row.row_id);
    }
}

#[test]
fn export_json_roundtrips() {
    let p = packet();
    let json = p.export_safe_json();
    let back: ExecutionAccessibilityPacket = serde_json::from_str(&json).expect("roundtrips");
    assert_eq!(p, back);
}

#[test]
fn chip_tokens_are_deterministic_and_named() {
    let r = row("a11y:input-request-prompt");
    let chip = r.chip_tokens();
    assert!(chip.contains("family=input_request_prompt"));
    assert!(chip.contains("effective_claim=inspect_only"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn on_disk_export_matches_builder() {
    let disk = current_m5_execution_a11y_fallback_export().expect("checked-in export validates");
    assert_eq!(
        disk,
        seeded_m5_execution_a11y_fallback_packet(),
        "checked-in support export drifted from the seeded builder; regenerate the artifact"
    );
}
