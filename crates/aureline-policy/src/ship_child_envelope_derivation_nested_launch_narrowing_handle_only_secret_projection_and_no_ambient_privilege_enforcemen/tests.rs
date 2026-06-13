use super::*;

fn packet() -> M5ChildEnvelopeDerivationPacket {
    frozen_stable_m5_child_envelope_derivation_packet()
}

fn nominal_index(packet: &M5ChildEnvelopeDerivationPacket, lane: M5NestedLaunchLane) -> usize {
    packet
        .derivations
        .iter()
        .position(|d| d.lane == lane && !d.narrowed_below_baseline)
        .expect("a nominal derivation exists for the lane")
}

#[test]
fn frozen_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn frozen_packet_covers_every_lane() {
    let present: std::collections::BTreeSet<_> =
        packet().derivations.iter().map(|d| d.lane).collect();
    for lane in M5NestedLaunchLane::ALL {
        assert!(
            present.contains(&lane),
            "packet missing a derivation for lane {}",
            lane.as_str()
        );
    }
}

#[test]
fn every_child_only_narrows_parent_capabilities() {
    for derivation in packet().derivations {
        for cap in &derivation.child.granted_capability_classes {
            assert!(
                derivation.parent.granted_capability_classes.contains(cap),
                "derivation {} child holds {} outside the parent",
                derivation.derivation_id,
                cap.as_str()
            );
        }
    }
}

#[test]
fn every_child_runs_equal_or_stricter_sandbox() {
    for derivation in packet().derivations {
        assert!(
            sandbox_strictness_rank(derivation.child.sandbox_profile)
                >= sandbox_strictness_rank(derivation.parent.sandbox_profile),
            "derivation {} child sandbox widens the parent",
            derivation.derivation_id
        );
    }
}

#[test]
fn no_child_inherits_raw_os_environment_or_full_authority() {
    for derivation in packet().derivations {
        assert!(!derivation
            .ambient_environment_posture
            .is_ambient_privilege_leak());
        assert!(!derivation.inherits_full_parent_authority);
    }
}

#[test]
fn helpers_never_self_issue_in_frozen_packet() {
    for derivation in packet().derivations {
        if derivation.actor.actor_class.is_untrusted_helper() {
            assert!(
                !derivation.audit_lineage.self_issued_by_executor,
                "helper derivation {} self-issued authority",
                derivation.derivation_id
            );
        }
    }
}

#[test]
fn child_widening_parent_capability_fails() {
    let mut packet = packet();
    let idx = nominal_index(&packet, M5NestedLaunchLane::Request);
    // The request parent never holds workspace write authority.
    packet.derivations[idx]
        .child
        .granted_capability_classes
        .push(M5CapabilityClass::WriteWorkspace);
    assert!(packet
        .validate()
        .contains(&M5ChildEnvelopeDerivationViolation::ChildCapabilityWidensParent));
}

#[test]
fn child_widening_parent_scope_fails() {
    let mut packet = packet();
    let idx = nominal_index(&packet, M5NestedLaunchLane::Notebook);
    packet.derivations[idx].child.allowed_scope = vec![M5AllowedScopeEntry {
        kind: M5AllowedScopeKind::FilesystemRoot,
        label: "workspace://other-project".to_owned(),
        access: M5ScopeAccessMode::ReadWrite,
    }];
    assert!(packet
        .validate()
        .contains(&M5ChildEnvelopeDerivationViolation::ChildScopeWidensParent));
}

#[test]
fn child_widening_scope_access_mode_fails() {
    let mut packet = packet();
    let idx = nominal_index(&packet, M5NestedLaunchLane::Request);
    // The request parent grants send-only egress; escalating a contained endpoint
    // to read-write widens the access mode beyond the parent.
    packet.derivations[idx].child.allowed_scope = vec![M5AllowedScopeEntry {
        kind: M5AllowedScopeKind::NetworkEndpoint,
        label: "https://api.example.test/v1/orders".to_owned(),
        access: M5ScopeAccessMode::ReadWrite,
    }];
    assert!(packet
        .validate()
        .contains(&M5ChildEnvelopeDerivationViolation::ChildScopeWidensParent));
}

#[test]
fn child_widening_sandbox_fails() {
    let mut packet = packet();
    let idx = nominal_index(&packet, M5NestedLaunchLane::Notebook);
    packet.derivations[idx].child.sandbox_profile = M5SandboxProfile::InProcessTrustedLocal;
    assert!(packet
        .validate()
        .contains(&M5ChildEnvelopeDerivationViolation::ChildSandboxWidensParent));
}

#[test]
fn child_inert_sandbox_is_allowed() {
    let mut packet = packet();
    let idx = nominal_index(&packet, M5NestedLaunchLane::Notebook);
    packet.derivations[idx].child.sandbox_profile = M5SandboxProfile::InertNoExecution;
    assert!(!packet
        .validate()
        .contains(&M5ChildEnvelopeDerivationViolation::ChildSandboxWidensParent));
}

#[test]
fn child_widening_secret_scope_fails() {
    let mut packet = packet();
    let idx = nominal_index(&packet, M5NestedLaunchLane::Notebook);
    // The notebook parent has no secret access; the child cannot acquire one.
    packet.derivations[idx].child.secret_scope = M5SecretScope::HandleOnlyDelegated;
    assert!(packet
        .validate()
        .contains(&M5ChildEnvelopeDerivationViolation::ChildSecretScopeWidensParent));
}

#[test]
fn child_expiry_after_parent_fails() {
    let mut packet = packet();
    let idx = nominal_index(&packet, M5NestedLaunchLane::Notebook);
    packet.derivations[idx].child.expires_at = "2026-06-10T05:00:00Z".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ChildEnvelopeDerivationViolation::ChildExpiryExceedsParent));
}

#[test]
fn child_policy_epoch_mismatch_fails() {
    let mut packet = packet();
    let idx = nominal_index(&packet, M5NestedLaunchLane::Notebook);
    packet.derivations[idx].child.policy_epoch.epoch_sequence = 99;
    assert!(packet
        .validate()
        .contains(&M5ChildEnvelopeDerivationViolation::PolicyEpochMismatch));
}

#[test]
fn raw_os_environment_inheritance_fails() {
    let mut packet = packet();
    packet.derivations[0].ambient_environment_posture =
        M5AmbientEnvironmentPosture::RawOsEnvironmentInherited;
    assert!(packet
        .validate()
        .contains(&M5ChildEnvelopeDerivationViolation::RawOsEnvironmentInherited));
}

#[test]
fn full_parent_authority_fan_out_fails() {
    let mut packet = packet();
    packet.derivations[0].inherits_full_parent_authority = true;
    assert!(packet
        .validate()
        .contains(&M5ChildEnvelopeDerivationViolation::FullParentAuthorityFannedOut));
}

#[test]
fn silently_permissive_enforcement_fails() {
    let mut packet = packet();
    packet.derivations[0].enforcement_status =
        M5EnforcementBackendStatus::SilentlyPermissiveUnsupported;
    assert!(packet
        .validate()
        .contains(&M5ChildEnvelopeDerivationViolation::EnforcementSilentlyPermissive));
}

#[test]
fn missing_enforcement_backend_fails() {
    let mut packet = packet();
    packet.derivations[0].enforcement_backend.clear();
    assert!(packet
        .validate()
        .contains(&M5ChildEnvelopeDerivationViolation::EnforcementBackendMissing));
}

#[test]
fn helper_self_issuing_authority_fails() {
    let mut packet = packet();
    let idx = nominal_index(&packet, M5NestedLaunchLane::Ai);
    assert!(packet.derivations[idx]
        .actor
        .actor_class
        .is_untrusted_helper());
    packet.derivations[idx]
        .audit_lineage
        .self_issued_by_executor = true;
    assert!(packet
        .validate()
        .contains(&M5ChildEnvelopeDerivationViolation::SelfIssuedAuthorityForbidden));
}

#[test]
fn secret_projection_without_scope_fails() {
    let mut packet = packet();
    let idx = nominal_index(&packet, M5NestedLaunchLane::Database);
    // The database child projects a secret; clearing the refs is inconsistent.
    packet.derivations[idx].child.secret_handle_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ChildEnvelopeDerivationViolation::SecretScopeInconsistent));
}

#[test]
fn empty_secret_handle_ref_fails() {
    let mut packet = packet();
    let idx = nominal_index(&packet, M5NestedLaunchLane::Database);
    packet.derivations[idx].child.secret_handle_refs[0]
        .handle_ref
        .clear();
    assert!(packet
        .validate()
        .contains(&M5ChildEnvelopeDerivationViolation::SecretProjectionNotHandleOnly));
}

#[test]
fn off_device_unverified_target_fails() {
    let mut packet = packet();
    packet.derivations[0].child.off_device = true;
    packet.derivations[0].child.identity_verified = false;
    assert!(packet
        .validate()
        .contains(&M5ChildEnvelopeDerivationViolation::OffDeviceTargetUnverified));
}

#[test]
fn narrowing_flag_inconsistency_fails() {
    let mut packet = packet();
    // A nominal derivation with the narrowed flag set but no triggers is inconsistent.
    let idx = nominal_index(&packet, M5NestedLaunchLane::Notebook);
    packet.derivations[idx].narrowed_below_baseline = true;
    assert!(packet
        .validate()
        .contains(&M5ChildEnvelopeDerivationViolation::NarrowingInconsistent));
}

#[test]
fn empty_decision_chain_fails() {
    let mut packet = packet();
    packet.derivations[0].audit_lineage.decision_chain.clear();
    assert!(packet
        .validate()
        .contains(&M5ChildEnvelopeDerivationViolation::AuditLineageIncomplete));
}

#[test]
fn missing_required_lane_fails() {
    let mut packet = packet();
    packet
        .derivations
        .retain(|d| d.lane != M5NestedLaunchLane::Debug);
    assert!(packet
        .validate()
        .contains(&M5ChildEnvelopeDerivationViolation::RequiredLaneMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ChildEnvelopeDerivationViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.no_raw_os_environment_inheritance = false;
    assert!(packet
        .validate()
        .contains(&M5ChildEnvelopeDerivationViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .remote_and_browser_preserve_derivation_semantics = false;
    assert!(packet
        .validate()
        .contains(&M5ChildEnvelopeDerivationViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ChildEnvelopeDerivationViolation::ProofFreshnessIncomplete));
}

#[test]
fn narrowed_derivations_validate_and_fail_closed() {
    let packet = build_derivation_packet(
        "m5-child-envelope-derivation:fixture:all-narrowed",
        "M5 Child-Envelope Derivations — all narrowed",
        narrowed_derivations(),
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    for derivation in packet.derivations {
        assert!(derivation.narrowed_below_baseline);
        assert_eq!(
            derivation.child.sandbox_profile,
            M5SandboxProfile::InertNoExecution
        );
        assert_eq!(derivation.child.secret_scope, M5SecretScope::NoSecretAccess);
    }
}

#[test]
fn markdown_summary_lists_every_lane() {
    let summary = packet().render_markdown_summary();
    for lane in M5NestedLaunchLane::ALL {
        assert!(
            summary.contains(lane.as_str()),
            "summary missing lane {}",
            lane.as_str()
        );
    }
}

#[test]
fn checked_support_export_matches_frozen_packet() {
    let checked = current_stable_m5_child_envelope_derivation_export()
        .expect("checked M5 child-envelope derivation export validates");
    assert_eq!(checked.packet_id, M5_CHILD_ENVELOPE_DERIVATION_PACKET_ID);
    assert_eq!(
        checked,
        frozen_stable_m5_child_envelope_derivation_packet(),
        "checked-in support export drifted from the frozen in-code packet; regenerate with the derivation dumper"
    );
}

#[test]
fn checked_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/execution-auth/m5/ship-child-envelope-derivation-nested-launch-narrowing-handle-only-secret-projection-and-no-ambient-privilege-enforcemen/all_nominal_derivations.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/execution-auth/m5/ship-child-envelope-derivation-nested-launch-narrowing-handle-only-secret-projection-and-no-ambient-privilege-enforcemen/all_narrowed_derivations.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/execution-auth/m5/ship-child-envelope-derivation-nested-launch-narrowing-handle-only-secret-projection-and-no-ambient-privilege-enforcemen/mixed_derivations.json"
        )),
    ] {
        let packet: M5ChildEnvelopeDerivationPacket =
            serde_json::from_str(raw).expect("fixture parses as child-envelope derivation packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
