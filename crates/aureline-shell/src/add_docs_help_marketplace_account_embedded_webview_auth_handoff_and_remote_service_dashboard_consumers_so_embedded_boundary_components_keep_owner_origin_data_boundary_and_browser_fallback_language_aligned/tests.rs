//! Tests for the M05-1074 embedded-boundary component consumer adoption lane.

use super::*;

fn packet() -> EmbeddedConsumerPacket {
    seeded_m5_embedded_boundary_component_consumers_packet()
}

#[test]
fn seeded_packet_validates() {
    let violations = packet().validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn packet_record_kind_and_schema_version_are_stamped() {
    let p = packet();
    assert_eq!(p.record_kind, EMBEDDED_CONSUMER_RECORD_KIND);
    assert_eq!(p.schema_version, EMBEDDED_CONSUMER_SCHEMA_VERSION);
}

#[test]
fn all_six_consumer_classes_present() {
    let p = packet();
    assert!(p.summary.docs_help_pane_consumer_present);
    assert!(p.summary.marketplace_account_consumer_present);
    assert!(p.summary.embedded_webview_consumer_present);
    assert!(p.summary.auth_handoff_consumer_present);
    assert!(p.summary.remote_service_dashboard_consumer_present);
    assert!(p.summary.support_export_help_consumer_present);
    assert_eq!(p.summary.consumer_class_count, ConsumerClass::ALL.len());
}

#[test]
fn every_frozen_family_is_adopted() {
    let p = packet();
    let families = p.represented_families();
    for family in M5EmbeddedBoundaryComponentFamily::ALL {
        assert!(families.contains(&family), "missing family {family:?}");
    }
    assert_eq!(
        p.summary.component_family_count,
        M5EmbeddedBoundaryComponentFamily::ALL.len()
    );
}

#[test]
fn every_controls_lane_is_exercised() {
    let p = packet();
    let lanes: BTreeSet<M5EmbeddedControlsLane> = p.rows.iter().map(|r| r.controls_lane).collect();
    for lane in M5EmbeddedControlsLane::ALL {
        assert!(
            lanes.contains(&lane),
            "controls lane {lane:?} is not exercised"
        );
    }
    assert_eq!(
        p.summary.controls_lane_count,
        M5EmbeddedControlsLane::ALL.len()
    );
}

#[test]
fn controls_lanes_are_canonical_and_stable_across_surfaces() {
    let p = packet();
    for row in &p.rows {
        assert!(
            row.controls_lane_is_canonical(),
            "row {} forks the controls lane",
            row.row_id
        );
        assert_eq!(row.controls_lane, controls_lane_for(row.component_family));
    }
    assert!(p.controls_lanes_stable_across_surfaces());
    assert!(p.summary.controls_lanes_stable_across_surfaces);
    assert!(p.summary.all_rows_use_canonical_controls_lane);
}

#[test]
fn same_family_shares_one_controls_lane_across_classes() {
    // The auth-handoff card appears on the auth-handoff surface and the support
    // export; every one must resolve to the same controls lane.
    let p = packet();
    let lanes: BTreeSet<M5EmbeddedControlsLane> = p
        .rows
        .iter()
        .filter(|r| r.component_family == M5EmbeddedBoundaryComponentFamily::AuthHandoffCard)
        .map(|r| r.controls_lane)
        .collect();
    assert_eq!(lanes.len(), 1, "auth-handoff-card lane forked: {lanes:?}");
}

#[test]
fn at_least_one_family_reused_across_classes() {
    let p = packet();
    assert!(
        p.families_reused_across_classes() >= 1,
        "expected a family adopted by two or more consumer classes"
    );
    assert_eq!(
        p.summary.families_reused_across_classes,
        p.families_reused_across_classes()
    );
}

#[test]
fn auth_handoff_card_is_reused_across_multiple_classes() {
    let p = packet();
    let classes: BTreeSet<ConsumerClass> = p
        .rows
        .iter()
        .filter(|r| r.component_family == M5EmbeddedBoundaryComponentFamily::AuthHandoffCard)
        .map(|r| r.consumer_class)
        .collect();
    assert!(
        classes.len() >= 2,
        "auth-handoff card should be adopted by >= 2 classes, saw {classes:?}"
    );
}

#[test]
fn all_rows_point_to_canonical_family() {
    let p = packet();
    for row in &p.rows {
        assert!(
            row.points_to_canonical_family(),
            "row {} does not point to canonical family",
            row.row_id
        );
    }
    assert!(p.summary.all_rows_point_to_canonical_family);
}

#[test]
fn canonical_family_schema_refs_are_per_family_and_distinct() {
    let refs: BTreeSet<&str> = M5EmbeddedBoundaryComponentFamily::ALL
        .iter()
        .map(|f| canonical_family_schema_ref_for(*f))
        .collect();
    assert_eq!(refs.len(), M5EmbeddedBoundaryComponentFamily::ALL.len());
}

#[test]
fn controls_lane_refs_are_per_lane_and_distinct() {
    let schema_refs: BTreeSet<&str> = M5EmbeddedControlsLane::ALL
        .iter()
        .map(|l| l.canonical_schema_ref())
        .collect();
    let artifact_refs: BTreeSet<&str> = M5EmbeddedControlsLane::ALL
        .iter()
        .map(|l| l.canonical_artifact_ref())
        .collect();
    assert_eq!(schema_refs.len(), M5EmbeddedControlsLane::ALL.len());
    assert_eq!(artifact_refs.len(), M5EmbeddedControlsLane::ALL.len());
}

#[test]
fn every_family_maps_to_a_single_stable_lane() {
    for family in M5EmbeddedBoundaryComponentFamily::ALL {
        let lane = controls_lane_for(family);
        // The lane's canonical refs are non-empty and resolve deterministically.
        assert!(!lane.canonical_schema_ref().is_empty());
        assert!(!lane.canonical_doc_ref().is_empty());
        assert!(!lane.canonical_artifact_ref().is_empty());
    }
}

#[test]
fn all_rows_preserve_labels() {
    let p = packet();
    for row in &p.rows {
        assert!(
            row.preserves_labels(),
            "row {} does not preserve labels",
            row.row_id
        );
    }
    assert!(p.summary.all_rows_preserve_labels);
}

#[test]
fn label_family_coverage_is_complete() {
    let p = packet();
    let covered = p.covered_label_families();
    for family in REQUIRED_LABEL_FAMILIES {
        assert!(
            covered.contains(family),
            "label family {family} not covered"
        );
    }
    assert!(p.summary.label_family_coverage_complete);
}

#[test]
fn boundary_disposition_coverage_is_complete() {
    let p = packet();
    let covered = p.covered_boundary_dispositions();
    for disposition in M5EmbeddedBoundaryDisposition::ALL {
        assert!(
            covered.contains(disposition.as_str()),
            "boundary disposition {} not covered",
            disposition.as_str()
        );
    }
    assert!(p.summary.boundary_disposition_coverage_complete);
    assert_eq!(
        p.summary.boundary_disposition_count,
        M5EmbeddedBoundaryDisposition::ALL.len()
    );
}

#[test]
fn every_row_preserves_its_family_primary_boundary_label() {
    let p = packet();
    for row in &p.rows {
        assert!(
            row.preserves_primary_boundary_truth(),
            "row {} drops its family's primary boundary label {}",
            row.row_id,
            family_primary_label(row.component_family)
        );
    }
    assert!(p.summary.all_boundary_rows_preserve_primary_truth);
}

#[test]
fn boundary_crossing_rows_keep_owner_origin_and_data_boundary_where_declared() {
    // Boundary-crossing classes must never render an anonymous pane: their rows
    // preserve the family primary label and at least one of owner_origin or
    // data_boundary so the user can tell whose content they see.
    let p = packet();
    let crossing: Vec<&EmbeddedConsumerRow> = p
        .rows
        .iter()
        .filter(|r| r.consumer_class.is_boundary_crossing())
        .collect();
    assert!(!crossing.is_empty());
    for row in crossing {
        assert!(row.preserves_primary_boundary_truth());
        let has = |f: &str| row.preserved_label_families.iter().any(|v| v == f);
        assert!(
            has("owner_origin") || has("data_boundary") || has("browser_fallback"),
            "boundary-crossing row {} names no owner/origin, data boundary, or browser fallback",
            row.row_id
        );
    }
}

#[test]
fn every_row_supports_state_reconstruction() {
    let p = packet();
    for row in &p.rows {
        assert!(
            row.supports_state_reconstruction(),
            "row {} cannot be reconstructed",
            row.row_id
        );
        assert!(row.boundary_state_ref.starts_with("boundary-state:"));
    }
    assert!(p.summary.all_rows_reconstructable);
}

#[test]
fn narrowed_rows_disclose_with_banner_and_note() {
    let p = packet();
    for row in &p.rows {
        assert!(
            row.discloses_narrowing(),
            "row {} does not disclose narrowing",
            row.row_id
        );
        if row.is_narrowed() {
            assert!(
                row.reduced_capability_banner.is_some(),
                "narrowed row {} lacks a banner",
                row.row_id
            );
            assert_eq!(row.label_parity, LabelParityState::DisclosedNarrowed);
        }
        if row.handoff_target.requires_note() {
            assert!(
                !row.handoff_note_ref.trim().is_empty(),
                "handoff row {} lacks a note",
                row.row_id
            );
        }
    }
    assert!(p.summary.all_narrowed_rows_disclose);
}

#[test]
fn full_interactive_rows_carry_no_banner() {
    for row in &packet().rows {
        if !row.is_narrowed() {
            assert!(
                row.reduced_capability_banner.is_none(),
                "full-interactive row {} carries a spurious banner",
                row.row_id
            );
            assert_eq!(row.label_parity, LabelParityState::Preserved);
        }
    }
}

#[test]
fn docs_help_and_support_export_references_present() {
    let p = packet();
    assert!(p.has_docs_help_reference());
    assert!(p.has_support_export_reference());
    assert!(p.summary.docs_help_reference_present);
    assert!(p.summary.support_export_reference_present);
}

#[test]
fn all_rows_have_copy_export_parity() {
    let p = packet();
    for row in &p.rows {
        assert!(
            row.copy_export.is_complete(),
            "row {} lacks copy/export parity",
            row.row_id
        );
    }
    assert!(p.summary.all_rows_have_copy_export);
}

#[test]
fn all_rows_guardrails_are_clear() {
    let p = packet();
    for row in &p.rows {
        assert!(
            row.guardrails_clear(),
            "row {} has a set guardrail: {:?}",
            row.row_id,
            row.first_failed_guardrail()
        );
    }
    assert!(p.summary.all_rows_guardrails_clear);
}

#[test]
fn surface_class_is_consistent_for_every_row() {
    for row in &packet().rows {
        assert!(
            row.surface_class_consistent(),
            "row {} surface/class mismatch",
            row.row_id
        );
    }
}

#[test]
fn row_ids_are_unique() {
    let p = packet();
    let unique: BTreeSet<&str> = p.rows.iter().map(|r| r.row_id.as_str()).collect();
    assert_eq!(unique.len(), p.rows.len());
}

#[test]
fn boundary_disposition_vocab_is_canonical_on_every_row() {
    for row in &packet().rows {
        for token in &row.boundary_disposition_vocab {
            assert!(
                is_canonical_boundary_disposition(token),
                "row {} carries non-canonical boundary-disposition token {token}",
                row.row_id
            );
        }
    }
}

#[test]
fn banner_capability_state_matches_authority() {
    for row in &packet().rows {
        if let Some(banner) = &row.reduced_capability_banner {
            assert_eq!(
                banner.capability_state,
                row.authority_mode.capability_state(),
                "row {} banner state mismatch",
                row.row_id
            );
        }
    }
}

#[test]
fn computed_summary_matches_stored_summary() {
    let p = packet();
    assert_eq!(p.summary, p.computed_summary());
}

// --- negative cases -------------------------------------------------------

#[test]
fn missing_consumer_class_is_rejected() {
    let mut p = packet();
    p.rows
        .retain(|r| r.consumer_class != ConsumerClass::SupportExportHelp);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EmbeddedConsumerViolation::MissingConsumerClass { .. })));
}

#[test]
fn dropped_family_is_rejected() {
    let mut p = packet();
    p.rows
        .retain(|r| r.component_family != M5EmbeddedBoundaryComponentFamily::BoundaryFactGrid);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EmbeddedConsumerViolation::MissingFamilyCoverage { .. })));
}

#[test]
fn forked_controls_lane_is_rejected() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.component_family == M5EmbeddedBoundaryComponentFamily::AuthHandoffCard)
        .expect("an auth-handoff-card row exists");
    p.rows[idx].controls_lane = M5EmbeddedControlsLane::DocsBoundaryFacts;
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        EmbeddedConsumerViolation::NonCanonicalControlsLane { .. }
    )));
    assert!(violations.iter().any(|v| matches!(
        v,
        EmbeddedConsumerViolation::ControlsLaneForkedAcrossSurfaces
    )));
}

#[test]
fn dropping_primary_boundary_label_is_rejected() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.component_family == M5EmbeddedBoundaryComponentFamily::EmbeddedOriginBar)
        .expect("an embedded-origin-bar row exists");
    // owner_origin is the origin bar's primary boundary label.
    p.rows[idx]
        .preserved_label_families
        .retain(|f| f != "owner_origin");
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        EmbeddedConsumerViolation::PrimaryBoundaryTruthDropped { .. }
    )));
}

#[test]
fn missing_boundary_state_ref_breaks_reconstruction() {
    let mut p = packet();
    p.rows[0].boundary_state_ref = String::new();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EmbeddedConsumerViolation::StateNotReconstructable { .. })));
}

#[test]
fn renamed_label_parity_is_rejected() {
    let mut p = packet();
    p.rows[0].label_parity = LabelParityState::RenamedOrDropped;
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EmbeddedConsumerViolation::LabelParityBroken { .. })));
}

#[test]
fn non_canonical_family_schema_ref_is_rejected() {
    let mut p = packet();
    p.rows[0].canonical_family_schema_ref = "schemas/ui/made-up.schema.json".to_owned();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EmbeddedConsumerViolation::NotCanonicalFamily { .. })));
}

#[test]
fn narrowed_without_banner_is_rejected() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("a narrowed row exists");
    p.rows[idx].reduced_capability_banner = None;
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        EmbeddedConsumerViolation::NarrowedWithoutDisclosure { .. }
    )));
}

#[test]
fn spurious_banner_on_full_row_is_rejected() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| !r.is_narrowed())
        .expect("a full-interactive row exists");
    p.rows[idx].reduced_capability_banner = Some(ReducedCapabilityBanner {
        banner_id: "banner:spurious".to_owned(),
        visible_label: "This should not be here".to_owned(),
        capability_state: "read_only".to_owned(),
        missing_capabilities: vec!["x".to_owned()],
    });
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        EmbeddedConsumerViolation::NarrowedWithoutDisclosure { .. }
    )));
}

#[test]
fn generic_banner_label_is_rejected() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("a narrowed row exists");
    if let Some(banner) = p.rows[idx].reduced_capability_banner.as_mut() {
        banner.visible_label = "Offline".to_owned();
    }
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        EmbeddedConsumerViolation::NarrowedWithoutDisclosure { .. }
    )));
}

#[test]
fn missing_handoff_note_is_rejected() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.handoff_target.requires_note())
        .expect("a handoff row exists");
    p.rows[idx].handoff_note_ref = String::new();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        EmbeddedConsumerViolation::NarrowedWithoutDisclosure { .. }
    )));
}

#[test]
fn set_guardrail_is_rejected() {
    let mut p = packet();
    p.rows[0].embeds_high_risk_approval_without_native_step_up = true;
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EmbeddedConsumerViolation::GuardrailViolated { .. })));
}

#[test]
fn forbidden_material_in_export_is_rejected() {
    let mut p = packet();
    p.rows[0]
        .evidence_refs
        .push("bearer abc123def456".to_owned());
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        EmbeddedConsumerViolation::RawBoundaryMaterialInExport { .. }
    )));
}

#[test]
fn summary_mismatch_is_rejected() {
    let mut p = packet();
    p.summary.row_count += 1;
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EmbeddedConsumerViolation::SummaryMismatch)));
}

#[test]
fn duplicate_row_id_is_rejected() {
    let mut p = packet();
    let dup = p.rows[0].clone();
    p.rows.push(dup);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EmbeddedConsumerViolation::DuplicateId { .. })));
}

#[test]
fn export_json_is_deterministic() {
    let a = packet().export_safe_json();
    let b = packet().export_safe_json();
    assert_eq!(a, b);
}

#[test]
fn export_json_round_trips() {
    let p = packet();
    let json = p.export_safe_json();
    let back: EmbeddedConsumerPacket = serde_json::from_str(&json).expect("round trips");
    assert_eq!(p, back);
    assert!(back.validate().is_empty());
}

#[test]
fn csv_has_header_and_one_line_per_row() {
    let p = packet();
    let csv = p.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), p.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,consumer_class,consumer_surface"));
    assert!(lines[0].contains("controls_lane"));
}

#[test]
fn markdown_summary_lists_every_row() {
    let p = packet();
    let md = p.render_markdown_summary();
    for row in &p.rows {
        assert!(
            md.contains(&row.row_id),
            "missing {} in markdown",
            row.row_id
        );
    }
}

#[test]
fn surface_tokens_are_unique() {
    let tokens: BTreeSet<&str> = M5EmbeddedConsumerSurface::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(tokens.len(), M5EmbeddedConsumerSurface::ALL.len());
}

#[test]
fn every_authority_mode_maps_to_a_distinct_capability_state() {
    let states: BTreeSet<&str> = AuthorityMode::ALL
        .iter()
        .map(|a| a.capability_state())
        .collect();
    assert_eq!(states.len(), AuthorityMode::ALL.len());
}

// --- checked-in artifacts -------------------------------------------------

#[test]
fn checked_in_export_matches_seeded_builder() {
    let on_disk = current_m5_embedded_boundary_component_consumers_export()
        .expect("checked-in export is valid");
    assert_eq!(
        on_disk.export_safe_json(),
        packet().export_safe_json(),
        "checked-in support export drifted from the seeded builder; regenerate the artifact"
    );
}

#[test]
fn checked_csv_matches_builder() {
    let expected = packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-embedded-boundary-component-consumer-proof/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected = packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-embedded-boundary-component-consumer-proof/report.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env
/// var so it never runs in the normal suite. Run with
/// `GEN_EMBEDDED_BOUNDARY_CONSUMER_ARTIFACTS=1 cargo test -p aureline-shell generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_EMBEDDED_BOUNDARY_CONSUMER_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_embedded_boundary_component_consumers_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest)
        .join("../../artifacts/release/m5-embedded-boundary-component-consumer-proof");
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(art.join("report.md"), &report).expect("write report");

    let fixtures =
        Path::new(manifest).join("../../fixtures/ui/m5-embedded-boundary-component-consumers");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
    fs::write(
        fixtures.join("README.md"),
        "# M5 embedded-boundary component consumer fixtures\n\n\
         Mirror of `artifacts/release/m5-embedded-boundary-component-consumer-proof/`. Proves the\n\
         docs/help pane, marketplace/account content, extension-owned embedded webview, browser /\n\
         device-code auth handoff, remote/service dashboard, and support/export + release-packet\n\
         consumers all reuse the same frozen embedded-boundary component families and vocabulary.\n\
         Regenerate with `GEN_EMBEDDED_BOUNDARY_CONSUMER_ARTIFACTS=1 cargo test -p aureline-shell generate_artifacts`.\n",
    )
    .expect("write fixture readme");
}
