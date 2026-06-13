use super::*;

fn packet() -> M5RuntimeAuthorityMatrixPacket {
    frozen_stable_m5_runtime_authority_matrix_packet()
}

#[test]
fn frozen_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn frozen_packet_names_every_surface() {
    let present: std::collections::BTreeSet<_> = packet()
        .surface_rows
        .iter()
        .map(|row| row.surface)
        .collect();
    for surface in M5ExecutingSurface::ALL {
        assert!(
            present.contains(&surface),
            "matrix missing surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn missing_surface_fails_validation() {
    let mut packet = packet();
    packet
        .surface_rows
        .retain(|row| row.surface != M5ExecutingSurface::NotebookKernel);
    assert!(packet
        .validate()
        .contains(&M5RuntimeAuthorityMatrixViolation::RequiredSurfaceMissing));
}

#[test]
fn stable_surface_missing_evidence_fails() {
    let mut packet = packet();
    let row = packet
        .surface_rows
        .iter_mut()
        .find(|row| row.qualification.is_stable())
        .expect("a stable surface row exists");
    row.required_evidence_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5RuntimeAuthorityMatrixViolation::StableSurfaceMissingEvidence));
}

#[test]
fn empty_capability_envelope_fails() {
    let mut packet = packet();
    packet.surface_rows[0].allowed_capability_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5RuntimeAuthorityMatrixViolation::CapabilityEnvelopeEmpty));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.surface_rows[1].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5RuntimeAuthorityMatrixViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = packet();
    packet.surface_rows[2].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5RuntimeAuthorityMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn untrusted_helper_self_issuing_authority_fails() {
    let mut packet = packet();
    let row = packet
        .surface_rows
        .iter_mut()
        .find(|row| row.surface == M5ExecutingSurface::AiTool)
        .expect("ai tool surface row exists");
    row.approval_ticket_posture = M5ApprovalTicketPosture::NoTicketRequiredReadOnly;
    assert!(packet
        .validate()
        .contains(&M5RuntimeAuthorityMatrixViolation::SelfIssuedAuthorityForbidden));
}

#[test]
fn elevated_capability_without_ticket_fails() {
    let mut packet = packet();
    let row = packet
        .surface_rows
        .iter_mut()
        .find(|row| {
            row.allowed_capability_classes
                .iter()
                .any(|cap| cap.is_elevated())
        })
        .expect("an elevated surface row exists");
    row.approval_ticket_posture = M5ApprovalTicketPosture::NoTicketRequiredReadOnly;
    assert!(packet
        .validate()
        .contains(&M5RuntimeAuthorityMatrixViolation::ElevatedCapabilityWithoutTicket));
}

#[test]
fn time_bounded_ticket_without_expiry_fails() {
    let mut packet = packet();
    let row = packet
        .surface_rows
        .iter_mut()
        .find(|row| row.approval_ticket_posture.is_time_bounded_ticket())
        .expect("a time-bounded ticket row exists");
    row.ticket_expiry_seconds = 0;
    assert!(packet
        .validate()
        .contains(&M5RuntimeAuthorityMatrixViolation::TicketExpiryMissing));
}

#[test]
fn secret_projection_without_scope_fails() {
    let mut packet = packet();
    let row = packet
        .surface_rows
        .iter_mut()
        .find(|row| {
            row.allowed_capability_classes
                .contains(&M5CapabilityClass::SecretHandleProjection)
        })
        .expect("a secret-projecting surface row exists");
    row.secret_scope = M5SecretScope::NoSecretAccess;
    assert!(packet
        .validate()
        .contains(&M5RuntimeAuthorityMatrixViolation::SecretScopeInconsistent));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5RuntimeAuthorityMatrixViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.no_ambient_machine_privilege = false;
    assert!(packet
        .validate()
        .contains(&M5RuntimeAuthorityMatrixViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_projection.desktop_shows_authority_envelope = false;
    assert!(packet
        .validate()
        .contains(&M5RuntimeAuthorityMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5RuntimeAuthorityMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn untrusted_helpers_require_external_authority() {
    for row in packet().surface_rows {
        if row.surface.is_untrusted_helper() {
            assert!(
                row.approval_ticket_posture.is_externally_issued(),
                "untrusted helper {} must not self-issue authority",
                row.surface.as_str()
            );
        }
    }
}

#[test]
fn markdown_summary_lists_every_surface() {
    let summary = packet().render_markdown_summary();
    for surface in M5ExecutingSurface::ALL {
        assert!(
            summary.contains(surface.as_str()),
            "summary missing surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_matches_frozen_packet() {
    let checked = current_stable_m5_runtime_authority_matrix_export()
        .expect("checked M5 runtime-authority matrix export validates");
    assert_eq!(checked.packet_id, M5_RUNTIME_AUTHORITY_MATRIX_PACKET_ID);
    assert_eq!(
        checked,
        frozen_stable_m5_runtime_authority_matrix_packet(),
        "checked-in support export drifted from the frozen in-code packet; regenerate with the matrix dumper"
    );
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/execution-auth/m5/freeze-the-m5-runtime-authority-approval-ticket-sandbox-profile-and-capability-envelope-matrix/remote_mutation_unsupported_held.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/execution-auth/m5/freeze-the-m5-runtime-authority-approval-ticket-sandbox-profile-and-capability-envelope-matrix/ai_tool_enforcement_backend_missing_narrowed.json"
        )),
    ] {
        let packet: M5RuntimeAuthorityMatrixPacket =
            serde_json::from_str(raw).expect("fixture parses as matrix packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
