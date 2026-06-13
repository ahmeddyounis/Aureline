use super::*;

fn packet() -> M5ApprovalTicketLedgerPacket {
    frozen_stable_m5_approval_ticket_ledger_packet()
}

fn ticket_index(packet: &M5ApprovalTicketLedgerPacket, action: M5TicketActionClass) -> usize {
    packet
        .tickets
        .iter()
        .position(|ticket| ticket.action_class == action && ticket.verification_state.is_valid())
        .expect("a valid ticket exists for the action class")
}

#[test]
fn frozen_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn frozen_packet_issues_every_action_class() {
    let present: std::collections::BTreeSet<_> =
        packet().tickets.iter().map(|t| t.action_class).collect();
    for action in M5TicketActionClass::ALL {
        assert!(
            present.contains(&action),
            "ledger missing a ticket for action class {}",
            action.as_str()
        );
    }
}

#[test]
fn every_ticket_binds_required_capability() {
    for ticket in packet().tickets {
        assert!(
            ticket
                .binding
                .bound_capability_classes
                .contains(&ticket.action_class.required_capability()),
            "ticket {} does not bind the capability its action requires",
            ticket.ticket_id
        );
    }
}

#[test]
fn every_ticket_carries_replay_protection_and_expiry() {
    for ticket in packet().tickets {
        assert!(!ticket.replay_protection.nonce.trim().is_empty());
        assert_ne!(ticket.replay_protection.replay_window_seconds, 0);
        assert_ne!(ticket.validity.ttl_seconds, 0);
    }
}

#[test]
fn missing_action_class_fails() {
    let mut packet = packet();
    packet
        .tickets
        .retain(|ticket| ticket.action_class != M5TicketActionClass::BrowserRoutedAction);
    assert!(packet
        .validate()
        .contains(&M5ApprovalTicketLedgerViolation::RequiredActionClassMissing));
}

#[test]
fn action_class_capability_unbound_fails() {
    let mut packet = packet();
    let idx = ticket_index(&packet, M5TicketActionClass::WorkspaceMutation);
    packet.tickets[idx]
        .binding
        .bound_capability_classes
        .retain(|cap| *cap != M5CapabilityClass::WriteWorkspace);
    assert!(packet
        .validate()
        .contains(&M5ApprovalTicketLedgerViolation::ActionClassCapabilityUnbound));
}

#[test]
fn capability_widening_beyond_matrix_fails() {
    let mut packet = packet();
    let idx = ticket_index(&packet, M5TicketActionClass::ProcessExecution);
    // The notebook matrix row never grants remote mutation.
    packet.tickets[idx]
        .binding
        .bound_capability_classes
        .push(M5CapabilityClass::RemoteMutation);
    assert!(packet
        .validate()
        .contains(&M5ApprovalTicketLedgerViolation::CapabilityWidensBeyondMatrix));
}

#[test]
fn sandbox_profile_widening_fails() {
    let mut packet = packet();
    let idx = ticket_index(&packet, M5TicketActionClass::RemoteMutation);
    packet.tickets[idx].binding.sandbox_profile = M5SandboxProfile::InProcessTrustedLocal;
    assert!(packet
        .validate()
        .contains(&M5ApprovalTicketLedgerViolation::SandboxProfileWidens));
}

#[test]
fn inert_fail_closed_profile_is_allowed() {
    let mut packet = packet();
    let idx = ticket_index(&packet, M5TicketActionClass::RemoteMutation);
    packet.tickets[idx].binding.sandbox_profile = M5SandboxProfile::InertNoExecution;
    assert!(!packet
        .validate()
        .contains(&M5ApprovalTicketLedgerViolation::SandboxProfileWidens));
}

#[test]
fn missing_binding_hash_fails() {
    let mut packet = packet();
    packet.tickets[0].binding.capability_envelope_hash.clear();
    assert!(packet
        .validate()
        .contains(&M5ApprovalTicketLedgerViolation::BindingHashMissing));
}

#[test]
fn zero_ttl_fails() {
    let mut packet = packet();
    packet.tickets[0].validity.ttl_seconds = 0;
    assert!(packet
        .validate()
        .contains(&M5ApprovalTicketLedgerViolation::ValidityIncomplete));
}

#[test]
fn empty_nonce_fails() {
    let mut packet = packet();
    packet.tickets[0].replay_protection.nonce.clear();
    assert!(packet
        .validate()
        .contains(&M5ApprovalTicketLedgerViolation::ReplayProtectionMissing));
}

#[test]
fn helper_self_issuing_authority_fails() {
    let mut packet = packet();
    let idx = ticket_index(&packet, M5TicketActionClass::SecretProjection);
    assert!(packet.tickets[idx].actor.actor_class.is_untrusted_helper());
    packet.tickets[idx].issuance_lineage.self_issued_by_executor = true;
    assert!(packet
        .validate()
        .contains(&M5ApprovalTicketLedgerViolation::SelfIssuedAuthorityForbidden));
}

#[test]
fn read_only_posture_is_not_externally_issued_check() {
    let mut packet = packet();
    packet.tickets[0].issuance_lineage.approval_posture =
        M5ApprovalTicketPosture::NoTicketRequiredReadOnly;
    assert!(packet
        .validate()
        .contains(&M5ApprovalTicketLedgerViolation::TicketPostureNotExternallyIssued));
}

#[test]
fn empty_decision_chain_fails() {
    let mut packet = packet();
    packet.tickets[0].issuance_lineage.decision_chain.clear();
    assert!(packet
        .validate()
        .contains(&M5ApprovalTicketLedgerViolation::PrivilegedActionWithoutLineage));
}

#[test]
fn secret_scope_without_projection_fails() {
    let mut packet = packet();
    let idx = ticket_index(&packet, M5TicketActionClass::ProcessExecution);
    // The notebook ticket binds no secret-projecting capability, so a secret
    // scope is inconsistent.
    packet.tickets[idx].binding.secret_scope = M5SecretScope::HandleOnlyDelegated;
    assert!(packet
        .validate()
        .contains(&M5ApprovalTicketLedgerViolation::SecretScopeInconsistent));
}

#[test]
fn valid_off_device_unverified_target_fails() {
    let mut packet = packet();
    let idx = ticket_index(&packet, M5TicketActionClass::RemoteMutation);
    packet.tickets[idx].target.identity_verified = false;
    assert!(packet
        .validate()
        .contains(&M5ApprovalTicketLedgerViolation::OffDeviceTargetUnverified));
}

#[test]
fn denied_ticket_without_reason_fails() {
    let mut packet = packet();
    let idx = packet
        .tickets
        .iter()
        .position(|ticket| ticket.verification_state.is_denied())
        .expect("a denied ticket exists");
    packet.tickets[idx].deny_reason = None;
    assert!(packet
        .validate()
        .contains(&M5ApprovalTicketLedgerViolation::DenyReasonMissing));
}

#[test]
fn valid_ticket_with_deny_reason_fails() {
    let mut packet = packet();
    packet.tickets[0].deny_reason = Some(M5TicketDenyReason {
        dimension: M5TicketDenyDimension::ExpiryElapsed,
        narrowed_to: M5DegradedFallback::RequireFreshTicket,
        explanation: "x".to_owned(),
        recovery_action: "y".to_owned(),
    });
    assert!(packet
        .validate()
        .contains(&M5ApprovalTicketLedgerViolation::DenyReasonOnValidTicket));
}

#[test]
fn deny_dimension_state_mismatch_fails() {
    let mut packet = packet();
    let idx = packet
        .tickets
        .iter()
        .position(|ticket| ticket.verification_state == M5TicketVerificationState::DeniedExpired)
        .expect("an expired ticket exists");
    // Replace the dimension with one that maps to a different state.
    packet.tickets[idx].deny_reason.as_mut().unwrap().dimension =
        M5TicketDenyDimension::PolicyEpochSuperseded;
    assert!(packet
        .validate()
        .contains(&M5ApprovalTicketLedgerViolation::DenyDimensionStateMismatch));
}

#[test]
fn deny_reason_without_recovery_fails() {
    let mut packet = packet();
    let idx = packet
        .tickets
        .iter()
        .position(|ticket| ticket.verification_state.is_denied())
        .expect("a denied ticket exists");
    packet.tickets[idx]
        .deny_reason
        .as_mut()
        .unwrap()
        .recovery_action = "   ".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ApprovalTicketLedgerViolation::DenyRecoveryMissing));
}

#[test]
fn local_action_requiring_control_plane_fails() {
    let mut packet = packet();
    let idx = ticket_index(&packet, M5TicketActionClass::WorkspaceMutation);
    assert!(!packet.tickets[idx].target.off_device);
    packet.tickets[idx]
        .local_first_verification
        .requires_live_control_plane = true;
    assert!(packet
        .validate()
        .contains(&M5ApprovalTicketLedgerViolation::LocalFirstRequiresControlPlane));
}

#[test]
fn offline_widening_authority_fails() {
    let mut packet = packet();
    packet.tickets[0]
        .local_first_verification
        .authority_widened_offline = true;
    assert!(packet
        .validate()
        .contains(&M5ApprovalTicketLedgerViolation::LocalFirstWidensAuthority));
}

#[test]
fn dropping_audit_lineage_fails() {
    let mut packet = packet();
    packet.tickets[0]
        .local_first_verification
        .audit_lineage_preserved = false;
    assert!(packet
        .validate()
        .contains(&M5ApprovalTicketLedgerViolation::AuditLineageDropped));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ApprovalTicketLedgerViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.no_ambient_machine_privilege = false;
    assert!(packet
        .validate()
        .contains(&M5ApprovalTicketLedgerViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .remote_and_browser_preserve_ticket_semantics = false;
    assert!(packet
        .validate()
        .contains(&M5ApprovalTicketLedgerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ApprovalTicketLedgerViolation::ProofFreshnessIncomplete));
}

#[test]
fn helpers_never_self_issue_in_frozen_packet() {
    for ticket in packet().tickets {
        if ticket.actor.actor_class.is_untrusted_helper() {
            assert!(
                !ticket.issuance_lineage.self_issued_by_executor,
                "helper ticket {} self-issued authority",
                ticket.ticket_id
            );
        }
    }
}

#[test]
fn bound_capabilities_never_widen_the_matrix() {
    let matrix = frozen_stable_m5_runtime_authority_matrix_packet();
    for ticket in packet().tickets {
        let row = matrix
            .surface_rows
            .iter()
            .find(|row| row.surface == ticket.surface)
            .expect("matrix has a row for every ticket surface");
        for cap in &ticket.binding.bound_capability_classes {
            assert!(
                row.allowed_capability_classes.contains(cap),
                "ticket {} bound {} outside the matrix row",
                ticket.ticket_id,
                cap.as_str()
            );
        }
    }
}

#[test]
fn deny_reasons_name_dimension_and_recovery() {
    for ticket in packet().tickets {
        if ticket.verification_state.is_denied() {
            let deny = ticket
                .deny_reason
                .as_ref()
                .expect("denied ticket carries a deny reason");
            assert_eq!(deny.dimension.denies_as(), ticket.verification_state);
            assert!(!deny.explanation.trim().is_empty());
            assert!(!deny.recovery_action.trim().is_empty());
        }
    }
}

#[test]
fn markdown_summary_lists_every_action_class() {
    let summary = packet().render_markdown_summary();
    for action in M5TicketActionClass::ALL {
        assert!(
            summary.contains(action.as_str()),
            "summary missing action class {}",
            action.as_str()
        );
    }
}

#[test]
fn checked_support_export_matches_frozen_packet() {
    let checked = current_stable_m5_approval_ticket_ledger_export()
        .expect("checked M5 approval-ticket ledger export validates");
    assert_eq!(checked.packet_id, M5_APPROVAL_TICKET_LEDGER_PACKET_ID);
    assert_eq!(
        checked,
        frozen_stable_m5_approval_ticket_ledger_packet(),
        "checked-in support export drifted from the frozen in-code packet; regenerate with the approval-ticket dumper"
    );
}

#[test]
fn checked_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/execution-auth/m5/implement-approval-ticket-issuance-deny-reason-packets-replay-nonce-or-expiry-enforcement-and-local-first-verification-f/all_local_first_valid_ledger.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/execution-auth/m5/implement-approval-ticket-issuance-deny-reason-packets-replay-nonce-or-expiry-enforcement-and-local-first-verification-f/expiry_and_replay_denied_ledger.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/execution-auth/m5/implement-approval-ticket-issuance-deny-reason-packets-replay-nonce-or-expiry-enforcement-and-local-first-verification-f/epoch_and_binding_denied_ledger.json"
        )),
    ] {
        let packet: M5ApprovalTicketLedgerPacket =
            serde_json::from_str(raw).expect("fixture parses as approval-ticket ledger packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
