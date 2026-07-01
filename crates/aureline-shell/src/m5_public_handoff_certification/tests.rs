//! Inline unit tests for the M5 public-handoff certification.

use super::*;

#[test]
fn seeded_packet_covers_every_governed_object_and_is_clean() {
    let packet = seeded_public_handoff_certification_packet();
    validate_public_handoff_certification_packet(&packet)
        .expect("seeded packet must validate clean");

    assert_eq!(packet.rows.len(), REQUIRED_OBJECT_KINDS.len());
    for kind in REQUIRED_OBJECT_KINDS {
        assert!(
            packet.row(kind).is_some(),
            "missing row for {}",
            kind.as_str()
        );
    }
    assert!(packet.report_clean);
    assert!(packet.blocking_findings.is_empty());
    assert_eq!(packet.red_row_count, 0);
    assert!(packet.all_rows_publishable);
}

#[test]
fn seeded_packet_has_six_green_and_two_yellow_rows() {
    let packet = seeded_public_handoff_certification_packet();
    assert_eq!(packet.green_row_count, 6);
    assert_eq!(packet.yellow_row_count, 2);
    assert_eq!(packet.red_row_count, 0);

    // Both beta-qualified boundary objects auto-narrow to yellow.
    assert_eq!(
        packet
            .row(M5HandoffObjectKind::DevicePermissionBoundary)
            .unwrap()
            .derived_status,
        HandoffCertStatus::Yellow
    );
    assert_eq!(
        packet
            .row(M5HandoffObjectKind::EmbeddedAuthBoundary)
            .unwrap()
            .derived_status,
        HandoffCertStatus::Yellow
    );
}

#[test]
fn every_row_status_is_the_derived_value() {
    let packet = seeded_public_handoff_certification_packet();
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.object_kind.as_str()
        );
        assert_eq!(row.stale_proof_causes, row.recompute_causes());
    }
}

#[test]
fn narrowed_rows_disclose_a_reason() {
    let packet = seeded_public_handoff_certification_packet();
    for row in &packet.rows {
        if !matches!(row.derived_status, HandoffCertStatus::Green) {
            assert!(
                row.narrowing_reason
                    .as_deref()
                    .map(|reason| !reason.trim().is_empty())
                    .unwrap_or(false),
                "narrowed row {} hides its reason",
                row.object_kind.as_str()
            );
        }
    }
}

#[test]
fn disclosed_boundary_gap_carries_an_active_waiver() {
    let packet = seeded_public_handoff_certification_packet();
    let embedded = packet
        .row(M5HandoffObjectKind::EmbeddedAuthBoundary)
        .unwrap();
    assert!(matches!(
        embedded.boundary_honesty,
        BoundaryHonestyState::DisclosedGap
    ));
    assert!(embedded.requires_waiver());
    assert!(embedded.has_active_waiver());
    assert_eq!(packet.active_waivers.len(), 1);
    assert!(packet.active_waivers[0].is_active_at(&packet.generated_at));
}

#[test]
fn stable_rows_are_backed_by_proof_packets() {
    let packet = seeded_public_handoff_certification_packet();
    for row in &packet.rows {
        if row.is_stable_qualified() {
            assert!(
                !row.proof_packet_refs.is_empty(),
                "stable row {} cites no proof",
                row.object_kind.as_str()
            );
        }
    }
}

#[test]
fn hidden_impersonation_blocks_promotion() {
    let packet = seeded_public_handoff_certification_packet_embedded_impersonation_blocked();
    let row = packet.row(M5HandoffObjectKind::EmbeddedAuthBoundary).unwrap();
    assert_eq!(row.derived_status, HandoffCertStatus::Red);
    assert!(!packet.report_clean);
    assert!(!packet.all_rows_publishable);
    assert!(packet.red_row_count >= 1);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        HandoffCertificationFinding::UndisclosedImpersonation { .. }
    )));
    // The exact impersonation cause is recorded against the native-chrome trigger.
    assert!(row.stale_proof_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5HandoffDowngradeTrigger::NativeChromeImpersonation
    )));
    // The validator must reject it.
    assert!(validate_public_handoff_certification_packet(&packet).is_err());
}

#[test]
fn unsafe_redaction_blocks_the_reproduction_packet() {
    let packet = seeded_public_handoff_certification_packet_repro_redaction_unsafe();
    let row = packet.row(M5HandoffObjectKind::ReproductionPacket).unwrap();
    assert_eq!(row.derived_status, HandoffCertStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        HandoffCertificationFinding::UnsafeRedaction { .. }
    )));
    assert!(row.stale_proof_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5HandoffDowngradeTrigger::RedactionPreviewMissing
    )));
    assert!(validate_public_handoff_certification_packet(&packet).is_err());
}

#[test]
fn stale_proof_without_waiver_blocks_a_stable_row() {
    let packet = seeded_public_handoff_certification_packet_service_health_stale();
    let row = packet.row(M5HandoffObjectKind::ServiceHealthNotice).unwrap();
    assert_eq!(row.derived_status, HandoffCertStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        HandoffCertificationFinding::StaleProofWithoutWaiver { .. }
    )));
    // The exact stale-proof cause is recorded.
    assert!(row
        .stale_proof_causes
        .iter()
        .any(|cause| matches!(cause.trigger, M5HandoffDowngradeTrigger::ProofStale)));
}

#[test]
fn dashboard_projects_every_row_and_counts() {
    let packet = seeded_public_handoff_certification_packet();
    let dashboard = packet.dashboard();
    assert_eq!(dashboard.rows.len(), packet.rows.len());
    assert_eq!(dashboard.green_row_count, packet.green_row_count);
    assert_eq!(dashboard.yellow_row_count, packet.yellow_row_count);
    assert_eq!(dashboard.red_row_count, packet.red_row_count);
    assert_eq!(dashboard.all_rows_publishable, packet.all_rows_publishable);
    assert_eq!(dashboard.source_packet_ref, packet.packet_id);
    assert!(!dashboard.public_truth_refs.is_empty());

    // The yellow embedded-boundary row exposes its waiver and cause in the dashboard.
    let embedded = dashboard
        .rows
        .iter()
        .find(|row| row.object_kind == M5HandoffObjectKind::EmbeddedAuthBoundary)
        .unwrap();
    assert_eq!(embedded.status, HandoffCertStatus::Yellow);
    assert!(embedded.has_active_waiver);
    assert!(embedded.cause_tokens.contains(
        &M5HandoffDowngradeTrigger::NativeChromeImpersonation
            .as_str()
            .to_owned()
    ));
}

#[test]
fn support_export_quotes_packet_matrix_and_waiver_refs() {
    let packet = seeded_public_handoff_certification_packet();
    let export = PublicHandoffCertificationSupportExport::from_packet(
        M5_HANDOFF_CERT_SUPPORT_EXPORT_ID,
        packet.clone(),
    );
    assert!(export.case_ids.contains(&packet.packet_id));
    assert!(export.case_ids.contains(&packet.matrix_packet_ref));
    assert!(export.case_ids.contains(&packet.build_identity_ref));
    for row in &packet.rows {
        assert!(export
            .case_ids
            .contains(&row.object_kind.as_str().to_owned()));
        for proof_ref in &row.proof_packet_refs {
            assert!(export.case_ids.contains(proof_ref));
        }
    }
    for waiver in &packet.active_waivers {
        assert!(export.case_ids.contains(&waiver.waiver_id));
    }
    assert_eq!(export.dashboard, packet.dashboard());
}

#[test]
fn markdown_names_every_certified_surface_and_verification_gate() {
    let packet = seeded_public_handoff_certification_packet();
    let markdown = packet.render_markdown();
    for kind in REQUIRED_OBJECT_KINDS {
        assert!(
            markdown.contains(certified_surface_label(kind)),
            "markdown omits {}",
            kind.as_str()
        );
    }
    assert!(markdown.contains("m5_public_handoff_certification_fixtures"));
    assert!(markdown.contains("waiver:embedded-boundary-label-sync:0001"));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded_public_handoff_certification_packet();
    let json = packet.export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("bearer "));
    assert!(!json.contains("://"));
}

#[test]
fn expired_waiver_is_flagged() {
    let waiver = HandoffCertificationWaiver {
        waiver_id: "waiver:expired:0001".to_owned(),
        object_kind: M5HandoffObjectKind::EmbeddedAuthBoundary,
        reason: "test".to_owned(),
        owner_role: "owner".to_owned(),
        expires_at: "2026-01-01T00:00:00Z".to_owned(),
    };
    assert!(!waiver.is_active_at("2026-06-30T00:00:00Z"));
    assert!(waiver.is_active_at("2025-01-01T00:00:00Z"));
}
