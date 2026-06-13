use super::*;

fn packet() -> M5CapabilityEnvelopePacket {
    frozen_stable_m5_capability_envelope_packet()
}

#[test]
fn frozen_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn frozen_packet_names_every_surface() {
    let present: std::collections::BTreeSet<_> =
        packet().envelopes.iter().map(|env| env.surface).collect();
    for surface in M5ExecutingSurface::ALL {
        assert!(
            present.contains(&surface),
            "packet missing envelope for surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn missing_surface_fails_validation() {
    let mut packet = packet();
    packet
        .envelopes
        .retain(|env| env.surface != M5ExecutingSurface::NotebookKernel);
    assert!(packet
        .validate()
        .contains(&M5CapabilityEnvelopeViolation::RequiredSurfaceMissing));
}

#[test]
fn empty_capability_envelope_fails() {
    let mut packet = packet();
    packet.envelopes[0].granted_capability_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5CapabilityEnvelopeViolation::CapabilityEnvelopeEmpty));
}

#[test]
fn missing_allowed_scope_fails() {
    let mut packet = packet();
    packet.envelopes[0].allowed_scope.clear();
    assert!(packet
        .validate()
        .contains(&M5CapabilityEnvelopeViolation::AllowedScopeMissing));
}

#[test]
fn capability_widening_beyond_matrix_fails() {
    let mut packet = packet();
    let envelope = packet
        .envelopes
        .iter_mut()
        .find(|env| env.surface == M5ExecutingSurface::NotebookKernel)
        .expect("notebook envelope exists");
    // The notebook matrix row never grants remote mutation.
    envelope
        .granted_capability_classes
        .push(M5CapabilityClass::RemoteMutation);
    assert!(packet
        .validate()
        .contains(&M5CapabilityEnvelopeViolation::CapabilityWidensBeyondMatrix));
}

#[test]
fn sandbox_profile_widening_fails() {
    let mut packet = packet();
    let envelope = packet
        .envelopes
        .iter_mut()
        .find(|env| env.surface == M5ExecutingSurface::RemoteMutation)
        .expect("remote mutation envelope exists");
    // Widening an isolated-remote envelope to in-process trusted-local is forbidden.
    envelope.sandbox_profile = M5SandboxProfile::InProcessTrustedLocal;
    assert!(packet
        .validate()
        .contains(&M5CapabilityEnvelopeViolation::SandboxProfileWidens));
}

#[test]
fn inert_fail_closed_profile_is_allowed() {
    let mut packet = packet();
    let envelope = packet
        .envelopes
        .iter_mut()
        .find(|env| env.surface == M5ExecutingSurface::RemoteMutation)
        .expect("remote mutation envelope exists");
    envelope.sandbox_profile = M5SandboxProfile::InertNoExecution;
    assert!(!packet
        .validate()
        .contains(&M5CapabilityEnvelopeViolation::SandboxProfileWidens));
}

#[test]
fn untrusted_helper_self_issuing_authority_fails() {
    let mut packet = packet();
    let envelope = packet
        .envelopes
        .iter_mut()
        .find(|env| env.surface == M5ExecutingSurface::AiTool)
        .expect("ai tool envelope exists");
    envelope.audit_lineage.self_issued_by_executor = true;
    assert!(packet
        .validate()
        .contains(&M5CapabilityEnvelopeViolation::SelfIssuedAuthorityForbidden));
}

#[test]
fn elevated_capability_without_ticket_fails() {
    let mut packet = packet();
    let envelope = packet
        .envelopes
        .iter_mut()
        .find(|env| {
            env.granted_capability_classes
                .iter()
                .any(|cap| cap.is_elevated())
        })
        .expect("an elevated envelope exists");
    envelope.audit_lineage.approval_ticket_ref.clear();
    assert!(packet
        .validate()
        .contains(&M5CapabilityEnvelopeViolation::ElevatedCapabilityWithoutTicket));
}

#[test]
fn missing_expiry_fails() {
    let mut packet = packet();
    packet.envelopes[0].expiry.ttl_seconds = 0;
    assert!(packet
        .validate()
        .contains(&M5CapabilityEnvelopeViolation::ExpiryMissing));
}

#[test]
fn missing_policy_epoch_fails() {
    let mut packet = packet();
    packet.envelopes[0].policy_epoch.epoch_id.clear();
    assert!(packet
        .validate()
        .contains(&M5CapabilityEnvelopeViolation::PolicyEpochMissing));
}

#[test]
fn incomplete_audit_lineage_fails() {
    let mut packet = packet();
    packet.envelopes[0].audit_lineage.decision_chain.clear();
    assert!(packet
        .validate()
        .contains(&M5CapabilityEnvelopeViolation::AuditLineageIncomplete));
}

#[test]
fn secret_projection_without_handle_ref_fails() {
    let mut packet = packet();
    let envelope = packet
        .envelopes
        .iter_mut()
        .find(|env| {
            env.granted_capability_classes
                .contains(&M5CapabilityClass::SecretHandleProjection)
        })
        .expect("a secret-projecting envelope exists");
    envelope.secret_handle_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CapabilityEnvelopeViolation::SecretScopeInconsistent));
}

#[test]
fn secret_refs_without_projection_fails() {
    let mut packet = packet();
    let envelope = packet
        .envelopes
        .iter_mut()
        .find(|env| env.surface == M5ExecutingSurface::NotebookKernel)
        .expect("notebook envelope exists");
    envelope.secret_handle_refs.push(M5SecretHandleRef {
        handle_ref: "broker-handle:stray:0001".to_owned(),
        scope: M5SecretScope::NoSecretAccess,
        broker_contract_ref: secret_handle_contract(),
    });
    assert!(packet
        .validate()
        .contains(&M5CapabilityEnvelopeViolation::SecretScopeInconsistent));
}

#[test]
fn off_device_unverified_target_fails() {
    let mut packet = packet();
    let envelope = packet
        .envelopes
        .iter_mut()
        .find(|env| env.target.off_device)
        .expect("an off-device envelope exists");
    envelope.target.identity_verified = false;
    assert!(packet
        .validate()
        .contains(&M5CapabilityEnvelopeViolation::OffDeviceTargetUnverified));
}

#[test]
fn narrowing_inconsistent_fails() {
    let mut packet = packet();
    packet.envelopes[0].narrowed_from_default = true;
    assert!(packet
        .validate()
        .contains(&M5CapabilityEnvelopeViolation::NarrowingInconsistent));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CapabilityEnvelopeViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.no_ambient_machine_privilege = false;
    assert!(packet
        .validate()
        .contains(&M5CapabilityEnvelopeViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_projection.desktop_shows_envelope = false;
    assert!(packet
        .validate()
        .contains(&M5CapabilityEnvelopeViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5CapabilityEnvelopeViolation::ProofFreshnessIncomplete));
}

#[test]
fn untrusted_helpers_never_self_issue() {
    for envelope in packet().envelopes {
        if envelope.actor.actor_class.is_untrusted_helper() {
            assert!(
                !envelope.audit_lineage.self_issued_by_executor,
                "untrusted helper {} must not self-issue authority",
                envelope.surface.as_str()
            );
        }
    }
}

#[test]
fn granted_capabilities_never_widen_the_matrix() {
    let matrix = frozen_stable_m5_runtime_authority_matrix_packet();
    for envelope in packet().envelopes {
        let row = matrix
            .surface_rows
            .iter()
            .find(|row| row.surface == envelope.surface)
            .expect("matrix has a row for every issued surface");
        for cap in &envelope.granted_capability_classes {
            assert!(
                row.allowed_capability_classes.contains(cap),
                "envelope for {} granted {} outside the matrix row",
                envelope.surface.as_str(),
                cap.as_str()
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
    let checked = current_stable_m5_capability_envelope_export()
        .expect("checked M5 capability-envelope export validates");
    assert_eq!(checked.packet_id, M5_CAPABILITY_ENVELOPE_PACKET_ID);
    assert_eq!(
        checked,
        frozen_stable_m5_capability_envelope_packet(),
        "checked-in support export drifted from the frozen in-code packet; regenerate with the envelope dumper"
    );
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/execution-auth/m5/ship-capability-envelope-packets-with-actor-target-allowed-roots-or-sinks-or-endpoints-secret-handle-refs-policy-epoch-e/remote_mutation_off_device_brokered.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/execution-auth/m5/ship-capability-envelope-packets-with-actor-target-allowed-roots-or-sinks-or-endpoints-secret-handle-refs-policy-epoch-e/ai_tool_ticket_expired_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/execution-auth/m5/ship-capability-envelope-packets-with-actor-target-allowed-roots-or-sinks-or-endpoints-secret-handle-refs-policy-epoch-e/database_action_write_ticket_unavailable_read_only.json"
        )),
    ] {
        let packet: M5CapabilityEnvelopePacket =
            serde_json::from_str(raw).expect("fixture parses as capability-envelope packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
