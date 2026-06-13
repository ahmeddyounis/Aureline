use super::*;

fn packet() -> M5AuthorityLifecycleLedgerPacket {
    frozen_stable_m5_authority_lifecycle_ledger_packet()
}

fn entry_index(packet: &M5AuthorityLifecycleLedgerPacket, entry_id: &str) -> usize {
    packet
        .entries
        .iter()
        .position(|entry| entry.entry_id == entry_id)
        .unwrap_or_else(|| panic!("entry {entry_id} exists"))
}

fn invalidated_index(packet: &M5AuthorityLifecycleLedgerPacket) -> usize {
    packet
        .entries
        .iter()
        .position(|entry| entry.lifecycle_state == M5AuthorityLifecycleState::Invalidated)
        .expect("an invalidated entry exists")
}

#[test]
fn frozen_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn frozen_packet_covers_every_origin_flow() {
    let present: std::collections::BTreeSet<_> = packet()
        .entries
        .iter()
        .map(|entry| entry.linkage.origin_flow)
        .collect();
    for flow in M5OriginFlowClass::ALL {
        assert!(
            present.contains(&flow),
            "ledger missing an entry for origin flow {}",
            flow.as_str()
        );
    }
}

#[test]
fn frozen_packet_demonstrates_every_drift_dimension() {
    let present: std::collections::BTreeSet<_> = packet()
        .entries
        .iter()
        .filter_map(|entry| entry.invalidation.as_ref().map(|inv| inv.drift_dimension))
        .collect();
    for dimension in M5DriftDimension::ALL {
        assert!(
            present.contains(&dimension),
            "ledger never invalidates on drift dimension {}",
            dimension.as_str()
        );
    }
}

#[test]
fn every_entry_joins_full_lineage() {
    for entry in packet().entries {
        assert!(!entry.linkage.session_ref.trim().is_empty());
        assert!(!entry.linkage.approval_ticket_ref.trim().is_empty());
        assert!(!entry.linkage.capability_envelope_ref.trim().is_empty());
        assert!(!entry.linkage.capability_envelope_hash.trim().is_empty());
    }
}

#[test]
fn missing_origin_flow_fails() {
    let mut packet = packet();
    packet
        .entries
        .retain(|entry| entry.linkage.origin_flow != M5OriginFlowClass::Repair);
    assert!(packet
        .validate()
        .contains(&M5AuthorityLifecycleLedgerViolation::RequiredOriginFlowMissing));
}

#[test]
fn missing_drift_dimension_fails() {
    let mut packet = packet();
    // Drop the only sandbox-profile-drift invalidation.
    packet.entries.retain(|entry| {
        entry
            .invalidation
            .as_ref()
            .map(|inv| inv.drift_dimension != M5DriftDimension::SandboxProfileDrift)
            .unwrap_or(true)
    });
    assert!(packet
        .validate()
        .contains(&M5AuthorityLifecycleLedgerViolation::RequiredDriftDimensionMissing));
}

#[test]
fn missing_linkage_fails() {
    let mut packet = packet();
    packet.entries[0].linkage.session_ref.clear();
    assert!(packet
        .validate()
        .contains(&M5AuthorityLifecycleLedgerViolation::LinkageIncomplete));
}

#[test]
fn sandbox_profile_widening_fails() {
    let mut packet = packet();
    let idx = entry_index(&packet, "ledger:notebook-kernel:0001");
    packet.entries[idx].issue.sandbox_profile = M5SandboxProfile::InProcessTrustedLocal;
    assert!(packet
        .validate()
        .contains(&M5AuthorityLifecycleLedgerViolation::SandboxProfileWidens));
}

#[test]
fn inert_fail_closed_profile_is_allowed() {
    let mut packet = packet();
    let idx = entry_index(&packet, "ledger:notebook-kernel:0001");
    packet.entries[idx].issue.sandbox_profile = M5SandboxProfile::InertNoExecution;
    assert!(!packet
        .validate()
        .contains(&M5AuthorityLifecycleLedgerViolation::SandboxProfileWidens));
}

#[test]
fn zero_ttl_fails() {
    let mut packet = packet();
    packet.entries[0].issue.ttl_seconds = 0;
    assert!(packet
        .validate()
        .contains(&M5AuthorityLifecycleLedgerViolation::IssueExpiryIncomplete));
}

#[test]
fn helper_self_issuing_authority_fails() {
    let mut packet = packet();
    let idx = entry_index(&packet, "ledger:ai-tool:0001");
    assert!(packet.entries[idx].actor.actor_class.is_untrusted_helper());
    packet.entries[idx].issue.self_issued_by_executor = true;
    assert!(packet
        .validate()
        .contains(&M5AuthorityLifecycleLedgerViolation::SelfIssuedAuthorityForbidden));
}

#[test]
fn empty_decision_chain_fails() {
    let mut packet = packet();
    packet.entries[0].issue.decision_chain.clear();
    assert!(packet
        .validate()
        .contains(&M5AuthorityLifecycleLedgerViolation::IssuanceLineageMissing));
}

#[test]
fn non_monotonic_use_sequence_fails() {
    let mut packet = packet();
    let idx = entry_index(&packet, "ledger:notebook-kernel:0001");
    packet.entries[idx].uses[1].sequence = 5;
    assert!(packet
        .validate()
        .contains(&M5AuthorityLifecycleLedgerViolation::UseSequenceNotMonotonic));
}

#[test]
fn empty_use_note_fails() {
    let mut packet = packet();
    packet.entries[0].uses[0].note = "   ".to_owned();
    assert!(packet
        .validate()
        .contains(&M5AuthorityLifecycleLedgerViolation::UseNoteMissing));
}

#[test]
fn denied_invalidated_use_without_invalidation_fails() {
    let mut packet = packet();
    let idx = entry_index(&packet, "ledger:scaffold-hook:0001");
    // The scaffold entry is Active with no invalidation; a denied-invalidated
    // use is therefore inconsistent.
    packet.entries[idx].uses[0].outcome = M5UseOutcome::DeniedInvalidated;
    assert!(packet
        .validate()
        .contains(&M5AuthorityLifecycleLedgerViolation::UseOutcomeInconsistent));
}

#[test]
fn invalidated_state_without_invalidation_fails() {
    let mut packet = packet();
    let idx = invalidated_index(&packet);
    packet.entries[idx].invalidation = None;
    assert!(packet
        .validate()
        .contains(&M5AuthorityLifecycleLedgerViolation::InvalidationMissing));
}

#[test]
fn invalidation_trigger_mismatch_fails() {
    let mut packet = packet();
    let idx = invalidated_index(&packet);
    packet.entries[idx].invalidation.as_mut().unwrap().trigger =
        M5RuntimeAuthorityDowngradeTrigger::SecretBrokerUnavailable;
    assert!(packet
        .validate()
        .contains(&M5AuthorityLifecycleLedgerViolation::InvalidationTriggerMismatch));
}

#[test]
fn invalidation_without_recovery_fails() {
    let mut packet = packet();
    let idx = invalidated_index(&packet);
    packet.entries[idx]
        .invalidation
        .as_mut()
        .unwrap()
        .recovery_action = "   ".to_owned();
    assert!(packet
        .validate()
        .contains(&M5AuthorityLifecycleLedgerViolation::InvalidationRecoveryMissing));
}

#[test]
fn revoked_state_without_revocation_fails() {
    let mut packet = packet();
    let idx = entry_index(&packet, "ledger:ai-tool:0001");
    packet.entries[idx].revocation = None;
    assert!(packet
        .validate()
        .contains(&M5AuthorityLifecycleLedgerViolation::RevocationMissing));
}

#[test]
fn revocation_without_reason_fails() {
    let mut packet = packet();
    let idx = entry_index(&packet, "ledger:ai-tool:0001");
    packet.entries[idx].revocation.as_mut().unwrap().reason = "   ".to_owned();
    assert!(packet
        .validate()
        .contains(&M5AuthorityLifecycleLedgerViolation::RevocationReasonMissing));
}

#[test]
fn spendable_entry_with_termination_evidence_fails() {
    let mut packet = packet();
    let idx = entry_index(&packet, "ledger:scaffold-hook:0001");
    assert!(packet.entries[idx].lifecycle_state.is_spendable());
    packet.entries[idx].invalidation = Some(M5Invalidation {
        detected_at: "2026-06-10T00:00:00Z".to_owned(),
        drift_dimension: M5DriftDimension::TargetIdentityDrift,
        trigger: M5DriftDimension::TargetIdentityDrift.trigger(),
        narrowed_to: M5DriftDimension::TargetIdentityDrift.default_fallback(),
        explanation: "x".to_owned(),
        recovery_action: "y".to_owned(),
    });
    assert!(packet
        .validate()
        .contains(&M5AuthorityLifecycleLedgerViolation::SpendableEntryCarriesTermination));
}

#[test]
fn terminated_entry_without_trigger_fails() {
    let mut packet = packet();
    let idx = invalidated_index(&packet);
    packet.entries[idx].applied_downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5AuthorityLifecycleLedgerViolation::TerminationTriggerMissing));
}

#[test]
fn active_entry_without_uses_fails() {
    let mut packet = packet();
    let idx = entry_index(&packet, "ledger:scaffold-hook:0001");
    packet.entries[idx].uses.clear();
    assert!(packet
        .validate()
        .contains(&M5AuthorityLifecycleLedgerViolation::LifecycleStateIncoherent));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AuthorityLifecycleLedgerViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.no_ambient_machine_privilege = false;
    assert!(packet
        .validate()
        .contains(&M5AuthorityLifecycleLedgerViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .remote_and_browser_preserve_ledger_semantics = false;
    assert!(packet
        .validate()
        .contains(&M5AuthorityLifecycleLedgerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5AuthorityLifecycleLedgerViolation::ProofFreshnessIncomplete));
}

#[test]
fn helpers_never_self_issue_in_frozen_packet() {
    for entry in packet().entries {
        if entry.actor.actor_class.is_untrusted_helper() {
            assert!(
                !entry.issue.self_issued_by_executor,
                "helper entry {} self-issued authority",
                entry.entry_id
            );
        }
    }
}

#[test]
fn invalidations_name_dimension_trigger_and_recovery() {
    for entry in packet().entries {
        if let Some(invalidation) = &entry.invalidation {
            assert_eq!(invalidation.trigger, invalidation.drift_dimension.trigger());
            assert!(!invalidation.explanation.trim().is_empty());
            assert!(!invalidation.recovery_action.trim().is_empty());
            assert!(entry
                .applied_downgrade_triggers
                .contains(&invalidation.trigger));
        }
    }
}

#[test]
fn markdown_summary_lists_every_entry() {
    let summary = packet().render_markdown_summary();
    for entry in packet().entries {
        assert!(
            summary.contains(&entry.entry_id),
            "summary missing entry {}",
            entry.entry_id
        );
    }
}

#[test]
fn checked_support_export_matches_frozen_packet() {
    let checked = current_stable_m5_authority_lifecycle_ledger_export()
        .expect("checked M5 authority-lifecycle ledger export validates");
    assert_eq!(checked.packet_id, M5_AUTHORITY_LIFECYCLE_LEDGER_PACKET_ID);
    assert_eq!(
        checked,
        frozen_stable_m5_authority_lifecycle_ledger_packet(),
        "checked-in support export drifted from the frozen in-code packet; regenerate with the authority-lifecycle dumper"
    );
}

#[test]
fn checked_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/execution-auth/m5/add-issue-use-revoke-audit-ledgers-invalidation-on-target-or-trust-or-policy-or-sandbox-drift-and-support-export-safe-au/full_issue_use_revoke_ledger.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/execution-auth/m5/add-issue-use-revoke-audit-ledgers-invalidation-on-target-or-trust-or-policy-or-sandbox-drift-and-support-export-safe-au/with_issued_grant_ledger.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/execution-auth/m5/add-issue-use-revoke-audit-ledgers-invalidation-on-target-or-trust-or-policy-or-sandbox-drift-and-support-export-safe-au/with_expired_grant_ledger.json"
        )),
    ] {
        let packet: M5AuthorityLifecycleLedgerPacket =
            serde_json::from_str(raw).expect("fixture parses as authority-lifecycle ledger packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
