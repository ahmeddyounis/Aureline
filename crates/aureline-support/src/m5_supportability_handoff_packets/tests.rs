use super::*;

fn packet_family() -> M5SupportabilityHandoffPackets {
    current_m5_supportability_handoff_packets().expect("packet family parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let family = packet_family();
    assert_eq!(
        family.schema_version,
        M5_SUPPORTABILITY_HANDOFF_SCHEMA_VERSION
    );
    assert_eq!(family.record_kind, M5_SUPPORTABILITY_HANDOFF_RECORD_KIND);
    assert_eq!(family.validate(), Vec::new());
}

#[test]
fn summary_counts_match_packets() {
    let family = packet_family();
    assert_eq!(family.summary, family.computed_summary());
}

#[test]
fn every_packet_joins_at_least_one_source_object() {
    let family = packet_family();
    for packet in &family.packets {
        assert!(
            !packet.components.is_empty(),
            "{} joins no source objects",
            packet.packet_id
        );
    }
}

#[test]
fn every_component_kind_is_exercised() {
    // The packet family can include finding codes, repair ids, crash artifacts, install / advisory state,
    // credential-state descriptors, environment / precedence summaries, and restore-provenance records.
    let family = packet_family();
    let mut kinds: BTreeSet<HandoffComponentKind> = BTreeSet::new();
    for packet in &family.packets {
        for component in &packet.components {
            kinds.insert(component.component_kind);
        }
    }
    for kind in HandoffComponentKind::ALL {
        assert!(
            kinds.contains(&kind),
            "no packet exercises component kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn every_data_class_is_exercised() {
    let family = packet_family();
    let mut classes: BTreeSet<HandoffDataClass> = BTreeSet::new();
    for packet in &family.packets {
        for component in &packet.components {
            classes.insert(component.data_class);
        }
    }
    for class in HandoffDataClass::ALL {
        assert!(
            classes.contains(&class),
            "no packet exercises data class {}",
            class.as_str()
        );
    }
}

#[test]
fn all_three_modes_are_present() {
    // Local self-diagnosis, team share, and formal support handoff are all represented.
    let family = packet_family();
    let modes: BTreeSet<HandoffMode> = family.packets.iter().map(|p| p.mode).collect();
    for mode in HandoffMode::ALL {
        assert!(
            modes.contains(&mode),
            "no packet exercises mode {}",
            mode.as_str()
        );
    }
}

#[test]
fn every_packet_shows_a_visible_copyable_exact_build_id() {
    let family = packet_family();
    for packet in &family.packets {
        assert!(
            !packet.exact_build_id.trim().is_empty(),
            "{}",
            packet.packet_id
        );
        assert!(packet.build_id_copyable, "{}", packet.packet_id);
        assert!(
            !packet.incident_ref.trim().is_empty(),
            "{}",
            packet.packet_id
        );
    }
}

#[test]
fn every_packet_carries_one_step_explainability() {
    let family = packet_family();
    for packet in &family.packets {
        assert!(
            packet.has_one_step_explainability(),
            "{} lacks one-step explainability",
            packet.packet_id
        );
    }
}

#[test]
fn every_packet_keeps_data_classes_visible_and_excludes_raw_material() {
    // The guardrail: never a monolithic export that hides data-class differences or redaction posture.
    let family = packet_family();
    assert!(family.all_data_classes_visible());
    for packet in &family.packets {
        assert!(packet.data_classes_visible, "{}", packet.packet_id);
        assert!(packet.raw_material_excluded, "{}", packet.packet_id);
    }
}

#[test]
fn lineage_is_preserved_on_every_component() {
    // The track invariant: preserve exact-build, finding-code, and repair-id lineage.
    let family = packet_family();
    for packet in &family.packets {
        assert!(packet.lineage_is_complete(), "{}", packet.packet_id);
        assert!(packet.lineage_complete, "{}", packet.packet_id);
        for component in &packet.components {
            assert!(
                !component.lineage_ref.trim().is_empty(),
                "{} {}",
                packet.packet_id,
                component.component_id
            );
            assert!(
                !component.source_ref.trim().is_empty(),
                "{} {}",
                packet.packet_id,
                component.component_id
            );
        }
    }
}

#[test]
fn mode_policies_match_their_modes() {
    let family = packet_family();
    for mode in HandoffMode::ALL {
        let policy = family.mode_policy(mode).expect("mode policy present");
        assert!(policy.is_mode_consistent(), "{}", mode.as_str());
        assert_eq!(policy.allowed_data_classes, mode.allowed_data_classes());
        assert_eq!(
            policy.default_redaction_posture,
            mode.default_redaction_posture()
        );
    }
}

#[test]
fn local_mode_allows_every_data_class_and_never_leaves_machine() {
    assert!(!HandoffMode::LocalSelfDiagnosis.leaves_machine());
    assert_eq!(
        HandoffMode::LocalSelfDiagnosis.allowed_data_classes(),
        HandoffDataClass::ALL.to_vec()
    );
    // Credential state may reach a team but never a formal vendor; user content never leaves the machine.
    assert!(HandoffDataClass::CredentialState.may_reach(HandoffMode::TeamShare));
    assert!(!HandoffDataClass::CredentialState.may_reach(HandoffMode::FormalSupportHandoff));
    assert!(!HandoffDataClass::UserContentExcerpt.may_reach(HandoffMode::TeamShare));
    assert!(HandoffDataClass::UserContentExcerpt.may_reach(HandoffMode::LocalSelfDiagnosis));
}

#[test]
fn every_packet_is_gate_consistent() {
    let family = packet_family();
    assert!(family.all_packets_gate_consistent());
    for packet in &family.packets {
        assert_eq!(
            packet.status,
            packet.computed_status(),
            "{}",
            packet.packet_id
        );
        assert_eq!(
            packet.presentation,
            packet.effective_presentation(),
            "{}",
            packet.packet_id
        );
        assert_eq!(
            packet.downgrade_reasons,
            packet.computed_downgrade_reasons(),
            "{}",
            packet.packet_id
        );
        assert_eq!(
            packet.blocked_before_send,
            packet.effective_presentation().warns_before_send(),
            "{}",
            packet.packet_id
        );
    }
}

#[test]
fn ready_packet_carries_everything_in_full() {
    let family = packet_family();
    let ready = family.ready_to_share_packets().count();
    assert!(
        ready >= 1,
        "fixture needs at least one ready-to-share packet to prove the gate is not a blanket flag"
    );
    for packet in family.ready_to_share_packets() {
        assert!(packet.downgrade_reasons.is_empty());
        assert!(packet.caveats.is_empty());
        assert!(!packet.blocked_before_send);
        for component in &packet.components {
            assert_eq!(
                component.disposition(packet.mode),
                ComponentDisposition::Carried,
                "{} {}",
                packet.packet_id,
                component.component_id
            );
        }
    }
}

#[test]
fn no_upload_local_handoff_keeps_everything_on_machine() {
    let family = packet_family();
    let packet = family
        .packet("local-self-diagnosis-no-upload")
        .expect("local packet");
    assert_eq!(packet.mode, HandoffMode::LocalSelfDiagnosis);
    assert!(!packet.mode.leaves_machine());
    assert_eq!(packet.status, HandoffStatus::ReadyToShare);
    assert_eq!(packet.presentation, HandoffPresentation::ReadyToShare);
}

#[test]
fn team_and_formal_deltas_differ_on_credential_state() {
    // The team-vs-formal delta: team share may carry the credential-state descriptor (redacted), but
    // formal support withholds it.
    let family = packet_family();
    let team = family.packet("team-share-redacted").expect("team packet");
    let formal = family
        .packet("formal-support-handoff")
        .expect("formal packet");

    let team_credential = team.component("team-credential").expect("team credential");
    assert!(team_credential.included);
    assert_eq!(
        team_credential.disposition(team.mode),
        ComponentDisposition::Redacted
    );

    let formal_credential = formal
        .component("formal-credential")
        .expect("formal credential");
    assert!(!formal_credential.included);
    assert_eq!(
        formal_credential.disposition(formal.mode),
        ComponentDisposition::Withheld
    );
    assert!(formal
        .downgrade_reasons
        .contains(&HandoffDowngradeReason::ComponentExcludedForMode));
    assert!(formal
        .downgrade_reasons
        .contains(&HandoffDowngradeReason::LineageDowngraded));
}

#[test]
fn policy_locked_export_withholds_and_names_the_locked_class() {
    let family = packet_family();
    let packet = family
        .packet("policy-locked-export")
        .expect("policy packet");
    assert_eq!(packet.status, HandoffStatus::PolicyLocked);
    assert_eq!(packet.presentation, HandoffPresentation::Narrowed);
    assert!(packet
        .downgrade_reasons
        .contains(&HandoffDowngradeReason::PolicyLockedDataClass));
    let locked = packet
        .component("policy-credential")
        .expect("locked component");
    assert!(locked.policy_locked);
    assert!(!locked.included);
    assert_eq!(
        locked.disposition(packet.mode),
        ComponentDisposition::Withheld
    );
    assert!(!packet.caveats.is_empty());
}

#[test]
fn blocked_user_escalation_blocks_unsafe_send() {
    let family = packet_family();
    let packet = family
        .packet("blocked-user-escalation")
        .expect("blocked packet");
    assert_eq!(packet.status, HandoffStatus::SendBlocked);
    assert_eq!(packet.presentation, HandoffPresentation::SendBlocked);
    assert!(packet.blocked_before_send);
    assert!(packet.has_blocking_component());
    assert!(!packet.blockers.is_empty());
    assert!(packet
        .downgrade_reasons
        .contains(&HandoffDowngradeReason::SendBlockedUnsafeContent));
}

#[test]
fn narrowed_and_blocked_packets_carry_caveats_and_blockers() {
    let family = packet_family();
    for packet in &family.packets {
        if packet.effective_presentation().requires_attention() {
            assert!(!packet.caveats.is_empty(), "{}", packet.packet_id);
        }
        if packet.computed_status().requires_blockers() {
            assert!(!packet.blockers.is_empty(), "{}", packet.packet_id);
        }
    }
}

#[test]
fn presentations_are_exhaustive() {
    let family = packet_family();
    let present: BTreeSet<HandoffPresentation> =
        family.packets.iter().map(|p| p.presentation).collect();
    for decision in HandoffPresentation::ALL {
        assert!(
            present.contains(&decision),
            "no packet exercises {}",
            decision.as_str()
        );
    }
}

#[test]
fn statuses_are_exhaustive() {
    let family = packet_family();
    let present: BTreeSet<HandoffStatus> = family.packets.iter().map(|p| p.status).collect();
    for status in HandoffStatus::ALL {
        assert!(
            present.contains(&status),
            "no packet exercises status {}",
            status.as_str()
        );
    }
}

#[test]
fn downgrade_reasons_are_exhaustive() {
    let family = packet_family();
    let present: BTreeSet<HandoffDowngradeReason> = family
        .packets
        .iter()
        .flat_map(|p| p.downgrade_reasons.iter().copied())
        .collect();
    for reason in HandoffDowngradeReason::ALL {
        assert!(
            present.contains(&reason),
            "no packet exercises {}",
            reason.as_str()
        );
    }
}

#[test]
fn dispositions_are_exhaustive() {
    let family = packet_family();
    let mut present: BTreeSet<ComponentDisposition> = BTreeSet::new();
    for packet in &family.packets {
        for component in &packet.components {
            present.insert(component.disposition(packet.mode));
        }
    }
    for disposition in ComponentDisposition::ALL {
        assert!(
            present.contains(&disposition),
            "no component exercises {}",
            disposition.as_str()
        );
    }
}

#[test]
fn export_projection_reflects_packets_and_gate() {
    let family = packet_family();
    let projection = family.export_projection();
    assert_eq!(projection.rows.len(), family.packets.len());
    assert_eq!(projection.packet_id, family.packet_id);
    assert!(projection.all_packets_gate_consistent);
    assert!(projection.all_data_classes_visible);
    assert_eq!(
        projection.ready_to_share_count,
        family.ready_to_share_packets().count()
    );
    assert_eq!(projection.narrowed_count, family.narrowed_packets().count());
    assert_eq!(
        projection.send_blocked_count,
        family.send_blocked_packets().count()
    );
    for (packet, row) in family.packets.iter().zip(projection.rows.iter()) {
        assert_eq!(row.presentation, packet.presentation.as_str());
        assert_eq!(row.ready_to_share, packet.is_ready_to_share());
        assert_eq!(row.exact_build_id, packet.exact_build_id);
        assert_eq!(row.components.len(), packet.components.len());
    }
}

#[test]
fn support_export_is_export_safe() {
    let family = packet_family();
    let export = family.support_export("support:m5:supportability-handoff", "2026-06-17T12:00:00Z");
    assert!(export.is_export_safe());
    assert_eq!(export.packet_id_ref, family.packet_id);
    assert!(export.raw_material_excluded);
}

#[test]
fn every_required_consumer_surface_binds() {
    let family = packet_family();
    for surface in HandoffConsumerSurface::REQUIRED {
        assert!(
            family.has_binding_for(surface),
            "missing binding for {}",
            surface.as_str()
        );
    }
}

#[test]
fn validate_flags_overstated_presentation() {
    let mut family = packet_family();
    if let Some(packet) = family
        .packets
        .iter_mut()
        .find(|p| p.effective_presentation() != HandoffPresentation::ReadyToShare)
    {
        packet.presentation = HandoffPresentation::ReadyToShare;
        assert!(family.validate().iter().any(|v| matches!(
            v,
            M5SupportabilityHandoffViolation::OverstatedPresentation { .. }
        )));
    }
}

#[test]
fn validate_flags_unjustified_withheld_component() {
    // Withholding a component whose data class CAN reach the mode and is not policy-locked is a silent
    // drop the gate forbids.
    let mut family = packet_family();
    let packet = family
        .packet_mut_for_test("team-share-redacted")
        .expect("team packet");
    let component = packet
        .components
        .iter_mut()
        .find(|c| c.component_id == "team-repair")
        .expect("repair component");
    component.included = false;
    assert!(family.validate().iter().any(|v| matches!(
        v,
        M5SupportabilityHandoffViolation::UnjustifiedWithheldComponent { .. }
    )));
}

#[test]
fn validate_flags_send_block_when_unsafe_content_is_included() {
    // Including a user-content excerpt for a send mode must block the send.
    let mut family = packet_family();
    let packet = family
        .packet_mut_for_test("team-share-redacted")
        .expect("team packet");
    let component = packet
        .components
        .iter_mut()
        .find(|c| c.component_id == "team-user-content")
        .expect("user-content component");
    component.included = true;
    component.redaction_posture = RedactionPosture::BlockedUnsafeContent;
    assert!(family.validate().iter().any(|v| matches!(
        v,
        M5SupportabilityHandoffViolation::StatusMismatch { .. }
            | M5SupportabilityHandoffViolation::OverstatedPresentation { .. }
            | M5SupportabilityHandoffViolation::DowngradeReasonsMismatch { .. }
            | M5SupportabilityHandoffViolation::BlockedBeforeSendMismatch { .. }
    )));
}

#[test]
fn validate_flags_data_classes_not_visible() {
    let mut family = packet_family();
    if let Some(packet) = family.packets.first_mut() {
        packet.data_classes_visible = false;
        assert!(family.validate().iter().any(|v| matches!(
            v,
            M5SupportabilityHandoffViolation::DataClassesNotVisible { .. }
        )));
    }
}

#[test]
fn validate_flags_build_id_not_copyable() {
    let mut family = packet_family();
    if let Some(packet) = family.packets.first_mut() {
        packet.build_id_copyable = false;
        assert!(family.validate().iter().any(|v| matches!(
            v,
            M5SupportabilityHandoffViolation::BuildIdNotCopyable { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_consumer_binding() {
    let mut family = packet_family();
    family
        .consumer_bindings
        .retain(|b| b.consumer_surface != HandoffConsumerSurface::IssueReportFlow);
    assert!(family.validate().iter().any(|v| matches!(
        v,
        M5SupportabilityHandoffViolation::MissingConsumerBinding { .. }
    )));
}

#[test]
fn validate_flags_binding_that_drops_data_class_visibility() {
    let mut family = packet_family();
    if let Some(binding) = family.consumer_bindings.first_mut() {
        binding.keeps_data_classes_visible = false;
        assert!(family.validate().iter().any(|v| matches!(
            v,
            M5SupportabilityHandoffViolation::ConsumerBindingDrift { .. }
        )));
    }
}

#[test]
fn validate_flags_mode_policy_drift() {
    let mut family = packet_family();
    if let Some(policy) = family
        .mode_policies
        .iter_mut()
        .find(|p| p.mode == HandoffMode::FormalSupportHandoff)
    {
        policy
            .allowed_data_classes
            .push(HandoffDataClass::CredentialState);
        assert!(family
            .validate()
            .iter()
            .any(|v| matches!(v, M5SupportabilityHandoffViolation::ModePolicyDrift { .. })));
    }
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut family = packet_family();
    family.summary.total_packets = family.summary.total_packets.wrapping_add(1);
    assert!(family
        .validate()
        .contains(&M5SupportabilityHandoffViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(
        HandoffMode::LocalSelfDiagnosis.as_str(),
        "local_self_diagnosis"
    );
    assert_eq!(
        HandoffMode::FormalSupportHandoff.as_str(),
        "formal_support_handoff"
    );
    assert_eq!(
        HandoffComponentKind::RestoreProvenanceRecord.as_str(),
        "restore_provenance_record"
    );
    assert_eq!(
        HandoffDataClass::UserContentExcerpt.as_str(),
        "user_content_excerpt"
    );
    assert_eq!(
        RedactionPosture::BlockedUnsafeContent.as_str(),
        "blocked_unsafe_content"
    );
    assert_eq!(HandoffStatus::PolicyLocked.as_str(), "policy_locked");
    assert_eq!(HandoffPresentation::SendBlocked.as_str(), "send_blocked");
    assert_eq!(
        HandoffDowngradeReason::SendBlockedUnsafeContent.as_str(),
        "send_blocked_unsafe_content"
    );
    assert_eq!(
        HandoffConsumerSurface::SupportDrillPacket.as_str(),
        "support_drill_packet"
    );
    assert_eq!(ComponentDisposition::Blocking.as_str(), "blocking");
}

impl M5SupportabilityHandoffPackets {
    /// Test-only mutable accessor for a packet by id.
    fn packet_mut_for_test(&mut self, packet_id: &str) -> Option<&mut HandoffPacket> {
        self.packets.iter_mut().find(|p| p.packet_id == packet_id)
    }
}
