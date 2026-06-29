//! Inline tests for the service-health communication lane.

use super::*;

fn packet() -> ServiceHealthCommunication {
    seeded_m5_service_health_communication()
}

#[test]
fn canonical_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_SERVICE_HEALTH_COMMUNICATION_PACKET_ID);
    assert_eq!(
        packet.record_kind,
        M5_SERVICE_HEALTH_COMMUNICATION_RECORD_KIND
    );
    assert_eq!(packet.tiers.len(), ServiceTier::ALL.len());
    assert_eq!(packet.notes.len(), AdminNoteKind::ALL.len());
    assert_eq!(packet.consumers.len(), HealthConsumer::ALL.len());
    assert_eq!(packet.boundaries.len(), ServiceTier::ALL.len());
    assert!(packet.conformance.all_hold());
    assert!(packet.vocabulary.matches_canonical());
}

#[test]
fn every_tier_and_note_carded_exactly_once() {
    let packet = packet();
    for tier in ServiceTier::ALL {
        let n = packet.tiers.iter().filter(|c| c.tier == tier).count();
        assert_eq!(n, 1, "tier `{}` not carded once", tier.as_str());
    }
    for kind in AdminNoteKind::ALL {
        let n = packet.notes.iter().filter(|c| c.kind == kind).count();
        assert_eq!(n, 1, "note `{}` not carded once", kind.as_str());
    }
}

#[test]
fn canonical_keeps_every_consumer_live_trusted() {
    // Acceptance: an all-healthy posture leaves every consumer live-trusted and local editing safe.
    let packet = packet();
    for c in &packet.consumers {
        assert!(
            c.is_live_trusted(),
            "consumer `{}` not live",
            c.consumer.as_str()
        );
        assert_eq!(c.readiness, HealthReadiness::LiveTrusted);
        assert!(c.gaps.is_empty());
        assert!(c.local_continuation_safe);
    }
    assert!(packet.local_editing_safe());
    assert!(!packet.coverage.has_data_downgrade);
    assert!(packet.coverage.live_data);
}

#[test]
fn four_boundaries_are_distinguishable() {
    // Acceptance: users can distinguish local machine, remote target, control-plane, and vendor issues.
    let packet = seeded_m5_service_health_communication_local_only();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let local = packet.tier(ServiceTier::LocalMachine).unwrap();
    let remote = packet.tier(ServiceTier::RemoteTarget).unwrap();
    let control = packet.tier(ServiceTier::EnterpriseControlPlane).unwrap();
    let vendor = packet.tier(ServiceTier::VendorHostedService).unwrap();
    // Each boundary carries its own health independently.
    assert_eq!(local.health_state, HealthState::Operational);
    assert_eq!(remote.health_state, HealthState::Outage);
    assert_eq!(control.health_state, HealthState::Outage);
    assert_eq!(vendor.health_state, HealthState::Outage);
    // Only the vendor boundary is optional; only the local machine affects local editing.
    assert!(vendor.is_optional);
    assert!(!remote.is_optional);
    assert!(local.affects_local_editing);
    assert!(!vendor.affects_local_editing);
    // The boundary summary names all four distinctly.
    assert_eq!(packet.boundaries.len(), 4);
    for b in &packet.boundaries {
        assert!(!b.status_message_id.is_empty());
    }
}

#[test]
fn vendor_outage_never_makes_local_editing_unsafe() {
    // Track invariant: a managed / vendor outage must not imply local editing or recovery is unsafe.
    let packet = seeded_m5_service_health_communication_vendor_outage();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let vendor = packet.tier(ServiceTier::VendorHostedService).unwrap();
    assert_eq!(vendor.gate, DescriptorGate::Blocked);
    assert!(vendor.no_live_data);
    assert!(
        vendor.local_editing_safe,
        "vendor outage must stay local-safe"
    );
    assert!(vendor.carries_recovery_guidance);
    // The packet-level continuity keeps local editing safe and lists the vendor as the affected one.
    assert!(packet.local_editing_safe());
    assert!(packet
        .continuity
        .outage_tiers
        .contains(&"vendor_hosted_service".to_owned()));
    // Every consumer still carries the local-continuation truth even when it reads the outage.
    for c in &packet.consumers {
        assert!(c.local_continuation_safe);
        assert!(c.is_no_live_data());
    }
}

#[test]
fn local_only_proves_local_work_continues() {
    // Acceptance: every remote boundary out, local machine live; local editing stays safe.
    let packet = seeded_m5_service_health_communication_local_only();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.local_editing_safe());
    assert_eq!(packet.data_state, StaleDataBehavior::LocalOnlyNoLiveData);
    assert!(!packet.coverage.live_data);
    assert_eq!(packet.continuity.outage_tiers.len(), 3);
    assert!(packet
        .continuity
        .live_tiers
        .contains(&"local_machine".to_owned()));
    let local = packet.tier(ServiceTier::LocalMachine).unwrap();
    assert!(local.local_editing_safe);
    assert_eq!(local.readiness, HealthReadiness::LiveTrusted);
}

#[test]
fn stale_and_mirrored_data_is_downgraded_with_source_age() {
    // Acceptance: stale or mirrored data is visibly downgraded and exportable with source-age truth.
    let packet = seeded_m5_service_health_communication_mirror_note();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let vendor = packet.tier(ServiceTier::VendorHostedService).unwrap();
    assert_eq!(vendor.release_data_state, ReleaseDataState::Mirrored);
    assert_eq!(vendor.gate, DescriptorGate::Narrowed);
    assert!(vendor.release_data_state.is_downgraded());
    // Source-age truth is present and labelled.
    assert!(vendor.source_age.observed_at.is_some());
    assert!(vendor.source_age.age_label.contains("mirror"));
    // The mirror-change note carries the same vocabulary and is unacknowledged.
    let note = packet.note(AdminNoteKind::MirrorChange).unwrap();
    assert_eq!(note.release_data_state, ReleaseDataState::Mirrored);
    assert!(!note.acknowledged);
    assert!(packet
        .continuity
        .unacknowledged_notes
        .contains(&"mirror_change".to_owned()));
    for c in &packet.consumers {
        assert!(c.is_showing_downgraded());
        assert!(c.local_continuation_safe);
    }
}

#[test]
fn admin_notes_consistent_across_surfaces() {
    // Acceptance: admin notes and service-health messages stay consistent across UI, docs, support.
    let packet = seeded_m5_service_health_communication_mirror_note();
    assert!(packet.disclosure.all_consume());
    assert!(packet.conformance.messages_consistent_across_surfaces);
    let panel = packet.consumer(HealthConsumer::ServiceHealthPanel).unwrap();
    let docs = packet.consumer(HealthConsumer::DocsHelp).unwrap();
    let support = packet.consumer(HealthConsumer::SupportExport).unwrap();
    assert_eq!(panel.readiness, docs.readiness);
    assert_eq!(docs.readiness, support.readiness);
    // The same cards drive the same gaps on each surface; they differ only by the owning consumer.
    let surfaced = |row: &HealthConsumerRow| -> Vec<(HealthTargetKind, String, HealthGapKind)> {
        row.gaps
            .iter()
            .map(|g| (g.target_kind, g.target_token.clone(), g.gap_kind))
            .collect()
    };
    assert_eq!(surfaced(panel), surfaced(docs));
    assert_eq!(surfaced(docs), surfaced(support));
}

#[test]
fn gate_is_the_worse_of_health_and_data() {
    // Guardrail: a card never makes downgraded data look live.
    let card = tier_with(
        ServiceTier::RemoteTarget,
        HealthState::Operational, // governed
        ReleaseDataState::Stale,  // narrowed
    );
    assert_eq!(card.gate, DescriptorGate::Narrowed);
    assert_eq!(card.readiness, HealthReadiness::ShowingDowngraded);
}

fn tier_with(
    tier: ServiceTier,
    health: HealthState,
    data: ReleaseDataState,
) -> ServiceTierHealthCard {
    let token = tier.as_str();
    ServiceTierHealthCard::new(ServiceTierHealthCardInput {
        tier,
        health_state: health,
        release_data_state: data,
        source_age: SourceAge::aged(Some("2026-07-06T00:00:00Z"), None, "4h old"),
        continuation: ContinuationStatement::new(
            token,
            true,
            RecoveryGuidance::new(token, RecoveryAction::ContinueLocally, &["ref"]),
        ),
        profiles: vec![DeploymentProfile::Managed],
        evidence_refs: vec!["ref".to_owned()],
    })
}

#[test]
fn tampering_a_card_to_overstate_freshness_is_rejected() {
    let mut packet = seeded_m5_service_health_communication_mirror_note();
    let idx = packet
        .tiers
        .iter()
        .position(|c| c.tier == ServiceTier::VendorHostedService)
        .unwrap();
    packet.tiers[idx].gate = DescriptorGate::Governed;
    packet.tiers[idx].readiness = HealthReadiness::LiveTrusted;
    let violations = packet.validate();
    assert!(
        violations.contains(&HealthViolation::OverstatedDataFreshness)
            || violations.contains(&HealthViolation::TierDerivationDrift),
        "{violations:?}"
    );
}

#[test]
fn stripping_continuation_guidance_from_a_troubled_card_is_rejected() {
    let mut packet = seeded_m5_service_health_communication_vendor_outage();
    let idx = packet
        .tiers
        .iter()
        .position(|c| c.tier == ServiceTier::VendorHostedService)
        .unwrap();
    // Remove the recovery path entirely — a bare red banner — but keep the derivation consistent.
    packet.tiers[idx].continuation = ContinuationStatement::new(
        "vendor_hosted_service",
        true,
        RecoveryGuidance::none("vendor_hosted_service"),
    );
    packet.tiers[idx].recompute();
    assert!(packet
        .validate()
        .contains(&HealthViolation::MissingContinuationGuidance));
}

#[test]
fn marking_a_vendor_outage_local_unsafe_is_rejected() {
    // Guardrail: a non-local boundary can never mark local editing unsafe.
    let mut packet = seeded_m5_service_health_communication_vendor_outage();
    let idx = packet
        .tiers
        .iter()
        .position(|c| c.tier == ServiceTier::VendorHostedService)
        .unwrap();
    packet.tiers[idx].local_editing_safe = false;
    packet.tiers[idx].continuation.local_safe = false;
    assert!(packet
        .validate()
        .contains(&HealthViolation::MisreportedLocalContinuity));
}

#[test]
fn data_state_is_labelled_local_safe() {
    let packet = packet();
    assert_eq!(packet.data_state, StaleDataBehavior::LiveVerified);
    assert!(packet.coverage.live_data);
    assert!(packet.conformance.stale_data_downgraded_with_source_age);
    // The vocabulary carries every release-data state so a mirrored / cached / policy-limited /
    // local-only / unavailable state is sayable.
    for needle in [
        "live_verified",
        "mirrored",
        "offline_cached",
        "stale",
        "policy_limited",
        "local_only_safe",
        "unavailable",
    ] {
        assert!(
            packet
                .vocabulary
                .release_data_states
                .contains(&needle.to_owned()),
            "missing {needle}"
        );
    }
}

#[test]
fn consumers_derive_verdict_from_cards() {
    let packet = packet();
    assert!(packet.conformance.consumer_verdict_derived_from_cards);
    for c in &packet.consumers {
        let mut expected: Vec<DeploymentProfile> = Vec::new();
        for &tier in &c.read_tiers {
            expected.extend(packet.tier(tier).unwrap().profiles.iter().copied());
        }
        expected.sort_by_key(|p| profile_rank(*p));
        expected.dedup();
        assert_eq!(c.profiles, expected);
    }
}

#[test]
fn channels_produce_identical_output() {
    let packet = packet();
    let desktop = packet.render_for_channel(RenderChannel::DesktopUi);
    let cli = packet.render_for_channel(RenderChannel::CliHeadless);
    let export = packet.render_for_channel(RenderChannel::OfflineExport);
    assert_eq!(desktop, cli);
    assert_eq!(cli, export);
    assert_eq!(desktop, packet.export_safe_json());
}

#[test]
fn controlled_vocabulary_is_frozen() {
    let vocab = HealthVocabulary::canonical();
    assert_eq!(vocab.tiers.len(), ServiceTier::ALL.len());
    assert_eq!(vocab.admin_note_kinds.len(), AdminNoteKind::ALL.len());
    assert_eq!(vocab.consumers.len(), HealthConsumer::ALL.len());
    for needle in [
        "local_machine",
        "remote_target",
        "enterprise_control_plane",
        "vendor_hosted_service",
    ] {
        assert!(vocab.tiers.contains(&needle.to_owned()), "missing {needle}");
    }
    for needle in ["channel_change", "mirror_change", "deployment_change"] {
        assert!(
            vocab.admin_note_kinds.contains(&needle.to_owned()),
            "missing {needle}"
        );
    }
}

#[test]
fn packet_round_trips() {
    for packet in [
        seeded_m5_service_health_communication(),
        seeded_m5_service_health_communication_vendor_outage(),
        seeded_m5_service_health_communication_mirror_note(),
        seeded_m5_service_health_communication_local_only(),
    ] {
        let json = packet.export_safe_json();
        let parsed: ServiceHealthCommunication =
            serde_json::from_str(&json).expect("packet deserializes");
        assert_eq!(parsed, packet);
        assert!(parsed.validate().is_empty(), "{:?}", parsed.validate());
    }
}

#[test]
fn card_csv_enumerates_every_card() {
    let csv = seeded_m5_service_health_communication_mirror_note().render_card_csv();
    let header = csv.lines().next().unwrap();
    assert!(header.starts_with("card_kind,target,health_state,release_data_state,source_age,"));
    assert!(header.contains("recovery"));
    let packet = seeded_m5_service_health_communication_mirror_note();
    let rows = csv.lines().count() - 1;
    assert_eq!(rows, packet.tiers.len() + packet.notes.len());
}

#[test]
fn markdown_summary_names_boundaries_notes_and_consumers() {
    let md = seeded_m5_service_health_communication_vendor_outage().render_markdown_summary();
    assert!(md.contains("Service boundaries"));
    assert!(md.contains("Admin notes"));
    assert!(md.contains("vendor_hosted_service"));
    assert!(md.contains("local editing"));
}

#[test]
fn tampered_consumer_verdict_is_rejected() {
    let mut packet = seeded_m5_service_health_communication_vendor_outage();
    let idx = packet
        .consumers
        .iter()
        .position(|c| c.is_no_live_data())
        .expect("a no-live-data consumer exists");
    packet.consumers[idx].gate_decision = DescriptorGate::Governed;
    packet.consumers[idx].readiness = HealthReadiness::LiveTrusted;
    assert!(packet
        .validate()
        .contains(&HealthViolation::ConsumerVerdictDrift));
}

#[test]
fn dropping_a_tier_is_rejected() {
    let mut packet = packet();
    packet.tiers.retain(|c| c.tier != ServiceTier::RemoteTarget);
    assert!(packet
        .validate()
        .contains(&HealthViolation::TierCoverageDrift));
}

#[test]
fn dropping_an_admin_note_is_rejected() {
    let mut packet = packet();
    packet
        .notes
        .retain(|c| c.kind != AdminNoteKind::MirrorChange);
    assert!(packet
        .validate()
        .contains(&HealthViolation::NoteCoverageDrift));
}

#[test]
fn export_carries_no_raw_material() {
    for packet in [
        seeded_m5_service_health_communication(),
        seeded_m5_service_health_communication_vendor_outage(),
        seeded_m5_service_health_communication_mirror_note(),
        seeded_m5_service_health_communication_local_only(),
    ] {
        assert!(packet.conformance.export_carries_no_raw_material);
        assert!(!packet
            .export_safe_json()
            .to_ascii_lowercase()
            .contains("bearer_token"));
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
