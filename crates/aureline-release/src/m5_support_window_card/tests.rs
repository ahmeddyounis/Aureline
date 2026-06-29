//! Inline tests for the support-window card-set lane.

use super::*;

fn packet() -> SupportWindowCardSet {
    seeded_m5_support_window_card_set()
}

#[test]
fn canonical_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_SUPPORT_WINDOW_CARD_SET_PACKET_ID);
    assert_eq!(packet.record_kind, M5_SUPPORT_WINDOW_CARD_SET_RECORD_KIND);
    assert_eq!(packet.channels.len(), ChannelScope::ALL.len());
    assert_eq!(packet.subjects.len(), CompatibilitySubject::ALL.len());
    assert_eq!(packet.consumers.len(), SupportConsumer::ALL.len());
    assert!(packet.conformance.all_hold());
    assert!(packet.vocabulary.matches_canonical());
}

#[test]
fn every_channel_and_subject_carded_exactly_once() {
    let packet = packet();
    for channel in ChannelScope::ALL {
        let n = packet
            .channels
            .iter()
            .filter(|c| c.channel == channel)
            .count();
        assert_eq!(n, 1, "channel `{}` not carded once", channel.as_str());
    }
    for subject in CompatibilitySubject::ALL {
        let n = packet
            .subjects
            .iter()
            .filter(|c| c.subject == subject)
            .count();
        assert_eq!(n, 1, "subject `{}` not carded once", subject.as_str());
        let card = packet.subject(subject).unwrap();
        assert_eq!(
            card.primary_artifact_class,
            subject.primary_artifact_class()
        );
        assert_eq!(card.owner_role, subject.owner_role());
    }
}

#[test]
fn canonical_keeps_every_consumer_supported() {
    // Acceptance: a healthy lifecycle leaves every consumer supported, with no migration action.
    let packet = packet();
    for c in &packet.consumers {
        assert!(
            c.is_supported(),
            "consumer `{}` not supported",
            c.consumer.as_str()
        );
        assert_eq!(c.readiness, SupportReadiness::Supported);
        assert!(c.gaps.is_empty());
        assert!(!c.requires_migration_action);
    }
    assert_eq!(
        packet.summary.supported_consumers,
        SupportConsumer::ALL.len() as u32
    );
    assert!(!packet.requires_migration_action());
    assert!(!packet.coverage.has_lifecycle_pressure);
}

#[test]
fn channel_card_discloses_identity_window_overlap_deprecation_pin() {
    // Acceptance: channel/support-lifecycle truth is inspectable directly — identity, support window,
    // overlap window, deprecation horizon, removal target, and pin/postpone path.
    let packet = seeded_m5_support_window_card_set_deprecation();
    let card = packet.channel(ChannelScope::Preview).unwrap();
    assert_eq!(card.channel_label, "Preview");
    assert!(!card.channel_description.is_empty());
    assert_eq!(card.support_window_state, SupportWindowState::GraceWindow);
    assert!(card.support_window.end_of_support_on.is_some());
    assert!(card.overlap_window.has_overlap);
    assert!(card.overlap_window.predecessor_version.is_some());
    assert_eq!(
        card.deprecation_horizon.successor_channel,
        Some(ChannelScope::Stable)
    );
    assert!(card.deprecation_horizon.removal_target_version.is_some());
    assert_eq!(
        card.pin_postpone.choice,
        PinPostponeChoice::MoveToSuccessorChannel
    );
    assert_eq!(card.readiness, SupportReadiness::PlanMigration);
}

#[test]
fn deprecated_states_carry_replacement_overlap_and_recovery() {
    // Acceptance: deprecated / end-of-support states carry replacement, overlap, and recovery guidance
    // instead of a bare warning.
    let dep = seeded_m5_support_window_card_set_deprecation();
    let dep_card = dep.channel(ChannelScope::Preview).unwrap();
    assert!(dep_card.needs_recovery_guidance());
    assert!(dep_card.carries_recovery_guidance);
    assert!(dep_card.deprecation_horizon.names_replacement());
    assert!(dep_card.overlap_window.is_disclosed());
    assert!(dep_card.pin_postpone.is_active());

    let eos = seeded_m5_support_window_card_set_end_of_support();
    let eos_card = eos.channel(ChannelScope::Preview).unwrap();
    assert_eq!(eos_card.gate, DescriptorGate::Blocked);
    assert!(eos_card.carries_recovery_guidance);
    assert!(eos_card.requires_migration_action);
    assert_eq!(
        eos_card.pin_postpone.choice,
        PinPostponeChoice::UpgradeRequired
    );
}

#[test]
fn deprecation_narrows_consumers_without_forcing_action() {
    let packet = seeded_m5_support_window_card_set_deprecation();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let card = packet.channel(ChannelScope::Preview).unwrap();
    assert_eq!(card.gate, DescriptorGate::Narrowed);
    // Every consumer reads every channel, so all plan a migration; none is forced to act.
    for c in &packet.consumers {
        assert!(c.is_plan_migration(), "consumer `{}`", c.consumer.as_str());
        assert!(c
            .gaps
            .iter()
            .any(|g| g.target_token == "preview"
                && g.gap_kind == SupportGapKind::MigrationRecommended));
        assert!(!c.requires_migration_action);
    }
    assert!(!packet.requires_migration_action());
    assert!(packet
        .release_gate
        .affected_channels
        .contains(&"preview".to_owned()));
}

#[test]
fn end_of_support_forces_action_on_consumers() {
    let packet = seeded_m5_support_window_card_set_end_of_support();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let card = packet.channel(ChannelScope::Preview).unwrap();
    assert_eq!(card.gate, DescriptorGate::Blocked);
    for c in &packet.consumers {
        assert!(c.is_action_required(), "consumer `{}`", c.consumer.as_str());
        assert!(c.requires_migration_action);
        assert!(c.gaps.iter().any(|g| g.target_token == "preview"
            && g.gap_kind == SupportGapKind::ActionRequiredBeforeUpgrade));
    }
    assert!(packet.requires_migration_action());
    assert!(packet.summary.action_required_channels >= 1);
}

#[test]
fn subject_compatibility_posture_is_shown_and_narrows() {
    // Acceptance: end-of-support and compatibility-window posture is shown for claimed subjects.
    let packet = seeded_m5_support_window_card_set_subject_compat();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let card = packet
        .subject(CompatibilitySubject::ExtensionManifest)
        .unwrap();
    assert_eq!(card.end_of_support_state, EndOfSupportState::Deprecated);
    assert_eq!(
        card.compatibility_window.posture,
        CompatibilityWindowPosture::NearingCeiling
    );
    assert!(card.compatibility_window.ceiling_version.is_some());
    assert_eq!(card.gate, DescriptorGate::Narrowed);
    assert!(card.carries_recovery_guidance);
    for c in &packet.consumers {
        assert!(c.is_plan_migration());
        assert!(c
            .gaps
            .iter()
            .any(|g| g.target_kind == SupportTargetKind::Subject
                && g.target_token == "extension_manifest"));
    }
}

#[test]
fn help_update_and_compatibility_report_share_one_packet() {
    // Acceptance: Help, update, and compatibility-report surfaces present the same support-window data.
    let packet = seeded_m5_support_window_card_set_deprecation();
    assert!(packet.disclosure.all_consume());
    assert!(
        packet
            .conformance
            .help_update_compatibility_share_one_packet
    );
    let help = packet.consumer(SupportConsumer::HelpAbout).unwrap();
    let update = packet.consumer(SupportConsumer::UpdateCenter).unwrap();
    let compat = packet
        .consumer(SupportConsumer::CompatibilityReport)
        .unwrap();
    assert_eq!(help.readiness, update.readiness);
    assert_eq!(update.readiness, compat.readiness);
    // The same cards drive the same support-window data on each surface; the gaps differ only by the
    // owning consumer token and its routing id, never by which target / cause is surfaced.
    let surfaced = |row: &SupportConsumerRow| -> Vec<(SupportTargetKind, String, SupportGapKind)> {
        row.gaps
            .iter()
            .map(|g| (g.target_kind, g.target_token.clone(), g.gap_kind))
            .collect()
    };
    assert_eq!(surfaced(help), surfaced(update));
    assert_eq!(surfaced(update), surfaced(compat));
}

#[test]
fn gate_is_the_worse_of_support_window_and_end_of_support() {
    // Guardrail: a card never advertises a wider commitment than its weakest promise.
    let card = ChannelSupportCard::new(ChannelSupportCardInput {
        channel: ChannelScope::Stable,
        support_window_state: SupportWindowState::FullSupport, // governed
        end_of_support_state: EndOfSupportState::Deprecated,   // narrowed
        support_window: SupportWindowDates::committed("2027-01-01", "2027-07-01"),
        overlap_window: OverlapWindow::overlapping("stable", "1.7.0", "2026-10-01"),
        deprecation_horizon: DeprecationHorizon {
            successor_channel: Some(ChannelScope::Lts),
            deprecation_on: Some("2026-06-01".to_owned()),
            removal_target_version: Some("2.0.0".to_owned()),
            removal_on: Some("2026-12-01".to_owned()),
            replacement_message_id: Some(
                "release_support_window.channel.stable.replacement".to_owned(),
            ),
        },
        pin_postpone: PinPostponeGuidance::new(
            "stable",
            PinPostponeChoice::MoveToSuccessorChannel,
            &["ref"],
        ),
        compatibility_caveats: Vec::new(),
        profiles: vec![DeploymentProfile::Managed],
        evidence_refs: vec!["ref".to_owned()],
    });
    // The weaker (deprecated → narrowed) promise wins, not the full-support one.
    assert_eq!(card.gate, DescriptorGate::Narrowed);
    assert_eq!(card.readiness, SupportReadiness::PlanMigration);
}

#[test]
fn tampering_a_card_to_broaden_commitment_is_rejected() {
    // Guardrail enforced in validation: a gate less severe than warranted fails.
    let mut packet = seeded_m5_support_window_card_set_deprecation();
    let idx = packet
        .channels
        .iter()
        .position(|c| c.channel == ChannelScope::Preview)
        .unwrap();
    packet.channels[idx].gate = DescriptorGate::Governed;
    packet.channels[idx].readiness = SupportReadiness::Supported;
    let violations = packet.validate();
    assert!(
        violations.contains(&SupportWindowViolation::OverBroadenedCommitment)
            || violations.contains(&SupportWindowViolation::ChannelDerivationDrift),
        "{violations:?}"
    );
}

#[test]
fn stripping_recovery_guidance_from_a_deprecated_card_is_rejected() {
    let mut packet = seeded_m5_support_window_card_set_deprecation();
    let idx = packet
        .channels
        .iter()
        .position(|c| c.channel == ChannelScope::Preview)
        .unwrap();
    // Remove the replacement path entirely — a bare warning — but keep the derivation consistent.
    packet.channels[idx].deprecation_horizon = DeprecationHorizon::none();
    packet.channels[idx].pin_postpone = PinPostponeGuidance::stay("preview");
    packet.channels[idx].recompute();
    assert!(packet
        .validate()
        .contains(&SupportWindowViolation::MissingRecoveryGuidance));
}

#[test]
fn data_state_is_labelled_local_safe() {
    // Track invariant: stale / mirrored / no-live-data is labelled honestly.
    let packet = packet();
    assert_eq!(packet.data_state, StaleDataBehavior::LiveVerified);
    assert!(packet.coverage.live_data);
    assert!(packet.conformance.data_state_labelled_local_safe);
    // The vocabulary carries the full stale-data behavior set so a mirrored / offline state is sayable.
    assert!(packet
        .vocabulary
        .stale_data_behaviors
        .contains(&"local_only_no_live_data".to_owned()));
}

#[test]
fn consumers_derive_verdict_from_cards() {
    let packet = packet();
    assert!(packet.conformance.consumer_verdict_derived_from_cards);
    for c in &packet.consumers {
        let mut expected: Vec<DeploymentProfile> = Vec::new();
        for &channel in &c.read_channels {
            expected.extend(packet.channel(channel).unwrap().profiles.iter().copied());
        }
        for &subject in &c.read_subjects {
            expected.extend(packet.subject(subject).unwrap().profiles.iter().copied());
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
    let vocab = SupportVocabulary::canonical();
    assert_eq!(vocab.channels.len(), ChannelScope::ALL.len());
    assert_eq!(
        vocab.compatibility_subjects.len(),
        CompatibilitySubject::ALL.len()
    );
    assert_eq!(vocab.consumers.len(), SupportConsumer::ALL.len());
    for needle in ["stable", "beta", "preview", "nightly", "lts"] {
        assert!(
            vocab.channels.contains(&needle.to_owned()),
            "missing {needle}"
        );
    }
    for needle in [
        "workspace_profile_files",
        "extension_sdk",
        "extension_manifest",
        "remote_helper",
        "public_schema",
    ] {
        assert!(
            vocab.compatibility_subjects.contains(&needle.to_owned()),
            "missing {needle}"
        );
    }
    for needle in [
        "stay_on_channel",
        "pin_current_version",
        "postpone_upgrade",
        "move_to_successor_channel",
        "side_by_side_during_overlap",
        "upgrade_required",
    ] {
        assert!(
            vocab.pin_postpone_choices.contains(&needle.to_owned()),
            "missing {needle}"
        );
    }
}

#[test]
fn packet_round_trips() {
    for packet in [
        seeded_m5_support_window_card_set(),
        seeded_m5_support_window_card_set_deprecation(),
        seeded_m5_support_window_card_set_end_of_support(),
        seeded_m5_support_window_card_set_subject_compat(),
    ] {
        let json = packet.export_safe_json();
        let parsed: SupportWindowCardSet =
            serde_json::from_str(&json).expect("packet deserializes");
        assert_eq!(parsed, packet);
        assert!(parsed.validate().is_empty(), "{:?}", parsed.validate());
    }
}

#[test]
fn card_csv_enumerates_every_card() {
    let csv = packet().render_card_csv();
    let header = csv.lines().next().unwrap();
    assert!(header.starts_with("card_kind,target,support_window_state,"));
    assert!(header.contains("pin_postpone"));
    let rows = csv.lines().count() - 1;
    assert_eq!(rows, packet().channels.len() + packet().subjects.len());
}

#[test]
fn markdown_summary_names_channels_subjects_and_consumers() {
    let md = seeded_m5_support_window_card_set_deprecation().render_markdown_summary();
    assert!(md.contains("Channel support lifecycle"));
    assert!(md.contains("Compatibility-window subjects"));
    assert!(md.contains("preview"));
    assert!(md.contains("gap:"));
}

#[test]
fn tampered_consumer_verdict_is_rejected() {
    let mut packet = seeded_m5_support_window_card_set_deprecation();
    let idx = packet
        .consumers
        .iter()
        .position(|c| c.is_plan_migration())
        .expect("a plan-migration consumer exists");
    packet.consumers[idx].gate_decision = DescriptorGate::Governed;
    packet.consumers[idx].readiness = SupportReadiness::Supported;
    assert!(packet
        .validate()
        .contains(&SupportWindowViolation::ConsumerVerdictDrift));
}

#[test]
fn dropping_a_channel_is_rejected() {
    let mut packet = packet();
    packet
        .channels
        .retain(|c| c.channel != ChannelScope::Nightly);
    assert!(packet
        .validate()
        .contains(&SupportWindowViolation::ChannelCoverageDrift));
}

#[test]
fn dropping_a_subject_is_rejected() {
    let mut packet = packet();
    packet
        .subjects
        .retain(|c| c.subject != CompatibilitySubject::PublicSchema);
    assert!(packet
        .validate()
        .contains(&SupportWindowViolation::SubjectCoverageDrift));
}

#[test]
fn export_carries_no_raw_material() {
    for packet in [
        seeded_m5_support_window_card_set(),
        seeded_m5_support_window_card_set_deprecation(),
        seeded_m5_support_window_card_set_end_of_support(),
        seeded_m5_support_window_card_set_subject_compat(),
    ] {
        assert!(packet.conformance.export_carries_no_raw_material);
        assert!(!packet
            .export_safe_json()
            .to_ascii_lowercase()
            .contains("bearer_token"));
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
