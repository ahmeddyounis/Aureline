use super::*;

fn packet() -> M5RuntimeAuthorityCertificationPacket {
    frozen_stable_m5_runtime_authority_certification_packet()
}

fn entry_index(
    packet: &M5RuntimeAuthorityCertificationPacket,
    surface: M5ExecutingSurface,
) -> usize {
    packet
        .entries
        .iter()
        .position(|entry| entry.surface == surface)
        .unwrap_or_else(|| panic!("surface {} certified", surface.as_str()))
}

#[test]
fn frozen_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn frozen_packet_certifies_every_claimed_surface() {
    let present: std::collections::BTreeSet<_> =
        packet().entries.iter().map(|entry| entry.surface).collect();
    for surface in M5ExecutingSurface::ALL {
        assert!(
            present.contains(&surface),
            "certification missing surface {}",
            surface.as_str()
        );
    }
    assert_eq!(packet().certified_count(), M5ExecutingSurface::ALL.len());
    assert_eq!(packet().narrowed_count(), 0);
}

#[test]
fn frozen_packet_covers_every_dimension_per_entry() {
    for entry in packet().entries {
        let present: std::collections::BTreeSet<_> =
            entry.proofs.iter().map(|proof| proof.dimension).collect();
        for dimension in M5CertifiedAuthorityDimension::ALL {
            assert!(
                present.contains(&dimension),
                "surface {} missing proof dimension {}",
                entry.surface.as_str(),
                dimension.as_str()
            );
        }
    }
}

#[test]
fn certified_entries_mirror_matrix_claims() {
    let matrix = frozen_stable_m5_runtime_authority_matrix_packet();
    for entry in packet().entries {
        let row = matrix
            .surface_rows
            .iter()
            .find(|row| row.surface == entry.surface)
            .expect("matrix row exists for surface");
        assert_eq!(entry.claimed_qualification, row.qualification);
        assert_eq!(entry.effective_qualification, row.qualification);
        assert_eq!(entry.default_sandbox_profile, row.default_sandbox_profile);
        assert_eq!(entry.approval_ticket_posture, row.approval_ticket_posture);
        assert_eq!(entry.secret_scope, row.secret_scope);
    }
}

#[test]
fn verdict_gate_folds_worst_status() {
    use M5CertificationProofStatus::*;
    use M5CertificationVerdict::*;
    assert_eq!(certification_verdict_for([Current; 4]), Certified);
    assert_eq!(
        certification_verdict_for([Current, Stale, Current, Current]),
        NarrowedStaleProof
    );
    assert_eq!(
        certification_verdict_for([Current, Stale, Missing, Current]),
        NarrowedMissingProof
    );
    assert_eq!(
        certification_verdict_for([UnsupportedBackend, Missing, Stale, Current]),
        FailedClosedUnsupportedBackend
    );
}

#[test]
fn missing_proof_auto_narrows_surface() {
    let packet =
        build_certification_packet("fixture", "fixture", entries_with_missing_proof_surface());
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let idx = entry_index(&packet, M5ExecutingSurface::RemoteMutation);
    let entry = &packet.entries[idx];
    assert_eq!(entry.verdict, M5CertificationVerdict::NarrowedMissingProof);
    assert_eq!(
        entry.effective_qualification,
        M5RuntimeAuthorityQualificationClass::Held
    );
    assert_eq!(
        entry.narrowed_to,
        Some(M5DegradedFallback::RequireFreshTicket)
    );
    assert!(entry.downgrade_trigger.is_some());
}

#[test]
fn stale_proof_auto_narrows_surface() {
    let packet =
        build_certification_packet("fixture", "fixture", entries_with_stale_proof_surface());
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let idx = entry_index(&packet, M5ExecutingSurface::AiTool);
    let entry = &packet.entries[idx];
    assert_eq!(entry.verdict, M5CertificationVerdict::NarrowedStaleProof);
    assert_eq!(
        entry.effective_qualification,
        M5RuntimeAuthorityQualificationClass::Preview
    );
}

#[test]
fn unsupported_backend_fails_closed() {
    let packet = build_certification_packet(
        "fixture",
        "fixture",
        entries_with_unsupported_backend_surface(),
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let idx = entry_index(&packet, M5ExecutingSurface::NotebookKernel);
    let entry = &packet.entries[idx];
    assert_eq!(
        entry.verdict,
        M5CertificationVerdict::FailedClosedUnsupportedBackend
    );
    assert_eq!(
        entry.effective_qualification,
        M5RuntimeAuthorityQualificationClass::Unavailable
    );
    assert_eq!(entry.narrowed_to, Some(M5DegradedFallback::FailClosedBlock));
}

#[test]
fn silent_widening_verdict_fails() {
    // A surface whose proofs are not all current but is still marked Certified is
    // a silent-widening regression and must be rejected.
    let mut packet = packet();
    let idx = entry_index(&packet, M5ExecutingSurface::RemoteMutation);
    packet.entries[idx].proofs[2].status = M5CertificationProofStatus::Missing;
    let violations = packet.validate();
    assert!(violations
        .contains(&M5RuntimeAuthorityCertificationViolation::VerdictInconsistentWithProofs));
    assert!(violations
        .contains(&M5RuntimeAuthorityCertificationViolation::CertifiedSurfaceCarriesUnprovenProof));
}

#[test]
fn narrowed_entry_widening_qualification_fails() {
    let mut packet =
        build_certification_packet("fixture", "fixture", entries_with_missing_proof_surface());
    let idx = entry_index(&packet, M5ExecutingSurface::RemoteMutation);
    // Force the effective qualification wider than the claim while leaving the
    // narrowed verdict in place.
    packet.entries[idx].effective_qualification = M5RuntimeAuthorityQualificationClass::Stable;
    assert!(packet
        .validate()
        .contains(&M5RuntimeAuthorityCertificationViolation::NarrowedQualificationWidened));
}

#[test]
fn certified_entry_carrying_narrowing_fails() {
    let mut packet = packet();
    packet.entries[0].narrowed_to = Some(M5DegradedFallback::FailClosedBlock);
    assert!(packet
        .validate()
        .contains(&M5RuntimeAuthorityCertificationViolation::CertifiedEntryCarriesNarrowing));
}

#[test]
fn missing_surface_coverage_fails() {
    let mut packet = packet();
    packet
        .entries
        .retain(|entry| entry.surface != M5ExecutingSurface::Recipe);
    assert!(packet
        .validate()
        .contains(&M5RuntimeAuthorityCertificationViolation::SurfaceCoverageIncomplete));
}

#[test]
fn duplicate_surface_fails() {
    let mut packet = packet();
    let clone = packet.entries[0].clone();
    packet.entries.push(clone);
    assert!(packet
        .validate()
        .contains(&M5RuntimeAuthorityCertificationViolation::DuplicateSurface));
}

#[test]
fn missing_dimension_fails() {
    let mut packet = packet();
    packet.entries[0].proofs.pop();
    assert!(packet
        .validate()
        .contains(&M5RuntimeAuthorityCertificationViolation::DimensionCoverageIncomplete));
}

#[test]
fn proof_source_mismatch_fails() {
    let mut packet = packet();
    packet.entries[0].proofs[0].proof_packet_id = "wrong-packet".to_owned();
    assert!(packet
        .validate()
        .contains(&M5RuntimeAuthorityCertificationViolation::ProofSourceMismatch));
}

#[test]
fn empty_proof_note_fails() {
    let mut packet = packet();
    packet.entries[0].proofs[0].note = "   ".to_owned();
    assert!(packet
        .validate()
        .contains(&M5RuntimeAuthorityCertificationViolation::ProofIncomplete));
}

#[test]
fn helper_surface_flag_mismatch_fails() {
    let mut packet = packet();
    let idx = entry_index(&packet, M5ExecutingSurface::AiTool);
    assert!(packet.entries[idx].is_untrusted_helper);
    packet.entries[idx].is_untrusted_helper = false;
    assert!(packet
        .validate()
        .contains(&M5RuntimeAuthorityCertificationViolation::HelperSurfaceFlagMismatch));
}

#[test]
fn empty_recovery_action_fails() {
    let mut packet = packet();
    packet.entries[0].recovery_action = "   ".to_owned();
    assert!(packet
        .validate()
        .contains(&M5RuntimeAuthorityCertificationViolation::RecoveryActionMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5RuntimeAuthorityCertificationViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.never_silently_widens_a_claim = false;
    assert!(packet
        .validate()
        .contains(&M5RuntimeAuthorityCertificationViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .release_evidence_gates_on_certification = false;
    assert!(packet
        .validate()
        .contains(&M5RuntimeAuthorityCertificationViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5RuntimeAuthorityCertificationViolation::ProofFreshnessIncomplete));
}

#[test]
fn no_certified_helper_self_issues() {
    for entry in packet().entries {
        if entry.is_untrusted_helper {
            // Helper surfaces are still certified, but their approval-ticket
            // posture must never let them self-issue.
            assert_ne!(
                entry.approval_ticket_posture,
                M5ApprovalTicketPosture::StandingPolicyTicket,
                "helper surface {} carries a self-issuable standing ticket",
                entry.surface.as_str()
            );
        }
    }
}

#[test]
fn markdown_summary_lists_every_surface() {
    let summary = packet().render_markdown_summary();
    for entry in packet().entries {
        assert!(
            summary.contains(entry.surface.as_str()),
            "summary missing surface {}",
            entry.surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_matches_frozen_packet() {
    let checked = current_stable_m5_runtime_authority_certification_export()
        .expect("checked M5 runtime-authority certification export validates");
    assert_eq!(
        checked.packet_id,
        M5_RUNTIME_AUTHORITY_CERTIFICATION_PACKET_ID
    );
    assert_eq!(
        checked,
        frozen_stable_m5_runtime_authority_certification_packet(),
        "checked-in support export drifted from the frozen in-code packet; regenerate with the certification dumper"
    );
}

#[test]
fn checked_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/execution-auth/m5/certify-runtime-authority-approval-ticket-integrity-execution-isolation-and-capability-envelope-parity-on-all-claimed-m5/all_surfaces_certified.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/execution-auth/m5/certify-runtime-authority-approval-ticket-integrity-execution-isolation-and-capability-envelope-parity-on-all-claimed-m5/with_missing_proof_surface.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/execution-auth/m5/certify-runtime-authority-approval-ticket-integrity-execution-isolation-and-capability-envelope-parity-on-all-claimed-m5/with_stale_proof_surface.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/execution-auth/m5/certify-runtime-authority-approval-ticket-integrity-execution-isolation-and-capability-envelope-parity-on-all-claimed-m5/with_unsupported_backend_surface.json"
        )),
    ] {
        let packet: M5RuntimeAuthorityCertificationPacket =
            serde_json::from_str(raw).expect("fixture parses as certification packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
