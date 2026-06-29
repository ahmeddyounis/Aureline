//! Inline tests for the update-center summary objects lane.

use super::*;

fn packet() -> M5UpdateCenterSummary {
    seeded_m5_update_center_summary()
}

#[test]
fn canonical_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_UPDATE_CENTER_SUMMARY_PACKET_ID);
    assert_eq!(packet.record_kind, M5_UPDATE_CENTER_SUMMARY_RECORD_KIND);
    assert_eq!(packet.entries.len(), ArtifactFamily::ALL.len());
    assert_eq!(packet.consumers.len(), SummaryConsumer::ALL.len());
    assert!(packet.conformance.all_hold());
    assert!(packet.vocabulary.matches_canonical());
}

#[test]
fn every_family_is_summarized_exactly_once() {
    let packet = packet();
    for family in ArtifactFamily::ALL {
        let matches: Vec<&UpdateSummaryEntry> = packet
            .entries
            .iter()
            .filter(|e| e.family == family)
            .collect();
        assert_eq!(
            matches.len(),
            1,
            "family `{}` not summarized once",
            family.as_str()
        );
        let entry = matches[0];
        assert_eq!(
            entry.primary_artifact_class,
            family.primary_artifact_class()
        );
        assert_eq!(entry.owner_role, family.owner_role());
        assert!(!entry.delta_rows.is_empty());
    }
}

#[test]
fn canonical_certifies_every_consumer() {
    // Acceptance criterion: every claimed consumer reads governed entries and certifies.
    let packet = packet();
    for c in &packet.consumers {
        assert!(
            c.is_certified(),
            "consumer `{}` not certified",
            c.consumer.as_str()
        );
        assert_eq!(c.effective_qualification, c.claimed_qualification);
        assert!(c.gaps.is_empty());
    }
    assert_eq!(
        packet.summary.certified_consumer_count,
        SummaryConsumer::ALL.len() as u32
    );
    assert_eq!(packet.summary.narrowed_consumer_count, 0);
    assert_eq!(packet.summary.blocked_consumer_count, 0);
    assert!(!packet.blocks_stable_promotion());
}

#[test]
fn entry_discloses_every_artifact_class_its_delta_rows_change() {
    // Guardrail: an update cannot hide a changed artifact class behind a generic desktop-app update.
    let packet = packet();
    for entry in &packet.entries {
        // The primary class is always disclosed.
        assert!(entry
            .affected_artifact_classes
            .contains(&entry.primary_artifact_class));
        // Every delta row's class appears in the disclosed set.
        for row in &entry.delta_rows {
            assert!(
                entry
                    .affected_artifact_classes
                    .contains(&row.artifact_class),
                "entry `{}` hides changed class `{}`",
                entry.family.as_str(),
                row.artifact_class.as_str()
            );
        }
    }
    // The framework pack updates three distinct artifact classes, none flattened away.
    let framework = packet.entry(ArtifactFamily::FrameworkPack).unwrap();
    assert!(framework.affected_artifact_classes.len() >= 3);
}

#[test]
fn delta_rows_carry_per_class_verification_and_restart_truth() {
    // Acceptance criterion: users can inspect verification/restart truth per artifact class.
    let packet = packet();
    let mut saw_added = false;
    for entry in &packet.entries {
        for row in &entry.delta_rows {
            assert!(row
                .detail_message_id
                .starts_with(M5_UPDATE_CENTER_SUMMARY_MESSAGE_ID_PREFIX));
            // Verification + data state determine the row gate.
            let expected = if matches!(row.verification_state.gate(), DescriptorGate::Blocked)
                || matches!(row.release_data_state.gate(), DescriptorGate::Blocked)
            {
                DescriptorGate::Blocked
            } else if matches!(row.verification_state.gate(), DescriptorGate::Narrowed)
                || matches!(row.release_data_state.gate(), DescriptorGate::Narrowed)
            {
                DescriptorGate::Narrowed
            } else {
                DescriptorGate::Governed
            };
            assert_eq!(row.gate, expected);
            if row.change_kind == DeltaChangeKind::Added {
                saw_added = true;
                assert!(row.from_version.is_none());
                assert!(row.to_version.is_some());
            }
        }
    }
    assert!(saw_added, "at least one delta row is an addition");
}

#[test]
fn rollback_is_never_overclaimed() {
    // Guardrail: rollback is only disclosed when a true version rollback exists.
    let packet = packet();
    for entry in &packet.entries {
        assert_eq!(entry.rollback_disclosed, entry.rollback.is_true_rollback());
        if matches!(
            entry.rollback,
            RollbackAvailability::SideBySideFallback | RollbackAvailability::ReinstallOnly
        ) {
            assert!(
                !entry.rollback_disclosed,
                "entry `{}` overclaims rollback for `{}`",
                entry.family.as_str(),
                entry.rollback.as_str()
            );
        }
    }
    // The extension offers a side-by-side fallback, not a true rollback.
    let ext = packet.entry(ArtifactFamily::Extension).unwrap();
    assert_eq!(ext.rollback, RollbackAvailability::SideBySideFallback);
    assert!(!ext.rollback_disclosed);
}

#[test]
fn mirrored_and_offline_data_is_labelled_not_masqueraded() {
    // Acceptance criterion: mirrored/offline data stays labeled instead of posing as live.
    let packet = packet();
    let ext = packet.entry(ArtifactFamily::Extension).unwrap();
    assert_eq!(ext.release_data_state, ReleaseDataState::Mirrored);
    assert!(!ext.release_data_state.is_live());
    assert_eq!(ext.gate, DescriptorGate::Governed); // still usable, just labeled
    let docs = packet.entry(ArtifactFamily::DocsPack).unwrap();
    assert_eq!(docs.release_data_state, ReleaseDataState::Offline);
    assert!(!docs.release_data_state.is_live());
}

#[test]
fn stale_data_narrows_exactly_the_consumers_that_read_it() {
    // Acceptance criterion: stale release data narrows claims deterministically.
    let packet = seeded_m5_update_center_summary_stale_data_narrowed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let stale = ArtifactFamily::DocsPack;
    assert_eq!(
        packet.entry(stale).unwrap().release_data_state,
        ReleaseDataState::Stale
    );
    assert_eq!(packet.entry(stale).unwrap().gate, DescriptorGate::Narrowed);
    for c in &packet.consumers {
        if c.read_families.contains(&stale) {
            assert!(
                c.is_narrowed(),
                "consumer `{}` reads stale family but did not narrow",
                c.consumer.as_str()
            );
            assert_eq!(c.effective_qualification, QualificationClass::Beta);
            assert!(c
                .gaps
                .iter()
                .any(|g| g.family == stale && g.gap_kind == SummaryGapKind::DataStale));
        } else {
            assert!(
                c.is_certified(),
                "consumer `{}` does not read stale family but narrowed",
                c.consumer.as_str()
            );
        }
    }
    assert!(!packet.blocks_stable_promotion());
    assert_eq!(packet.summary.narrowed_consumer_count, 2);
    assert_eq!(packet.summary.certified_consumer_count, 1);
    assert!(packet
        .release_gate
        .affected_families
        .contains(&stale.as_str().to_owned()));
}

#[test]
fn not_provided_data_blocks_exactly_the_consumers_that_read_it() {
    // Acceptance criterion: no live data blocks Stable promotion deterministically.
    let packet = seeded_m5_update_center_summary_not_provided_blocked();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let blocked = ArtifactFamily::FrameworkPack;
    assert_eq!(
        packet.entry(blocked).unwrap().release_data_state,
        ReleaseDataState::NotProvided
    );
    assert_eq!(packet.entry(blocked).unwrap().gate, DescriptorGate::Blocked);
    for c in &packet.consumers {
        if c.read_families.contains(&blocked) {
            assert!(
                c.is_blocked(),
                "consumer `{}` reads not-provided family but was not blocked",
                c.consumer.as_str()
            );
            assert_eq!(c.effective_qualification, QualificationClass::Unavailable);
            assert!(c
                .gaps
                .iter()
                .any(|g| g.family == blocked && g.gap_kind == SummaryGapKind::DataNotProvided));
        } else {
            assert!(
                c.is_certified(),
                "consumer `{}` does not read not-provided family but was blocked",
                c.consumer.as_str()
            );
        }
    }
    assert!(packet.blocks_stable_promotion());
    assert_eq!(packet.summary.blocked_consumer_count, 2);
    assert_eq!(packet.summary.certified_consumer_count, 1);
    assert_eq!(packet.release_gate.blocked_consumers.len(), 2);
}

#[test]
fn apply_ready_only_when_update_available_and_verified() {
    let packet = packet();
    let desktop = packet.entry(ArtifactFamily::DesktopApp).unwrap();
    assert!(!desktop.update_available);
    assert!(!desktop.apply_ready, "up-to-date entry is not apply-ready");
    let ext = packet.entry(ArtifactFamily::Extension).unwrap();
    assert!(ext.update_available);
    assert!(ext.apply_ready);
    // A blocked entry is never apply-ready.
    let blocked = seeded_m5_update_center_summary_not_provided_blocked();
    let fw = blocked.entry(ArtifactFamily::FrameworkPack).unwrap();
    assert!(!fw.apply_ready);
}

#[test]
fn consumers_read_one_summary() {
    // Acceptance criterion: release center, update center, and Help/About read one summary object.
    let packet = packet();
    assert_eq!(
        packet.consumer_tokens,
        tokens(&SummaryConsumer::ALL, |c| c.as_str())
    );
    assert!(packet.disclosure.all_consume());
    assert!(packet.conformance.consumers_read_one_summary);
    // Each consumer's disclosed classes are exactly the union of the families it reads.
    for c in &packet.consumers {
        let mut expected: Vec<ArtifactClass> = Vec::new();
        for &family in &c.read_families {
            expected.extend(
                packet
                    .entry(family)
                    .unwrap()
                    .affected_artifact_classes
                    .iter()
                    .copied(),
            );
        }
        expected.sort_by_key(|x| artifact_rank(*x));
        expected.dedup();
        assert_eq!(c.disclosed_artifact_classes, expected);
    }
}

#[test]
fn channels_produce_identical_output() {
    let packet = packet();
    let desktop = packet.render_for_channel(SummaryChannel::DesktopUi);
    let cli = packet.render_for_channel(SummaryChannel::CliHeadless);
    let offline = packet.render_for_channel(SummaryChannel::OfflineMirror);
    assert_eq!(desktop, cli);
    assert_eq!(cli, offline);
    assert_eq!(desktop, packet.export_safe_json());
}

#[test]
fn controlled_vocabulary_is_frozen() {
    let vocab = SummaryVocabulary::canonical();
    assert_eq!(vocab.families.len(), ArtifactFamily::ALL.len());
    assert_eq!(vocab.consumers.len(), SummaryConsumer::ALL.len());
    for needle in [
        "desktop_app",
        "extension",
        "docs_pack",
        "policy_bundle",
        "framework_pack",
        "runtime_toolchain",
    ] {
        assert!(vocab.families.contains(&needle.to_owned()));
    }
    for needle in ["live", "mirrored", "offline", "stale", "not_provided"] {
        assert!(vocab.release_data_states.contains(&needle.to_owned()));
    }
    for needle in [
        "rollback_supported",
        "side_by_side_fallback",
        "reinstall_only",
        "no_rollback",
    ] {
        assert!(vocab.rollback_availabilities.contains(&needle.to_owned()));
    }
}

#[test]
fn packet_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: M5UpdateCenterSummary = serde_json::from_str(&json).expect("packet deserializes");
    assert_eq!(parsed, packet);
    assert!(parsed.validate().is_empty());
}

#[test]
fn delta_csv_enumerates_every_row() {
    let csv = packet().render_delta_csv();
    let header = csv.lines().next().unwrap();
    assert!(header.starts_with("family,artifact_class,change_kind,"));
    assert!(header.contains("verification_state"));
    assert!(header.contains("release_data_state"));
    let rows = csv.lines().count() - 1;
    let expected: usize = packet().entries.iter().map(|e| e.delta_rows.len()).sum();
    assert_eq!(rows, expected);
}

#[test]
fn markdown_summary_names_families_and_consumers() {
    let md = seeded_m5_update_center_summary_stale_data_narrowed().render_markdown_summary();
    assert!(md.contains("update-center summary"));
    assert!(md.contains("Update summary entries"));
    assert!(md.contains("desktop_app"));
    assert!(md.contains("gap:"));
}

#[test]
fn tampered_consumer_verdict_is_rejected() {
    let mut packet = seeded_m5_update_center_summary_stale_data_narrowed();
    let idx = packet
        .consumers
        .iter()
        .position(|c| c.is_narrowed())
        .expect("a narrowed consumer exists");
    packet.consumers[idx].gate_decision = DescriptorGate::Governed;
    packet.consumers[idx].effective_qualification = QualificationClass::Stable;
    assert!(packet
        .validate()
        .contains(&M5UpdateCenterSummaryViolation::ConsumerVerdictDrift));
}

#[test]
fn tampered_entry_rollup_is_rejected() {
    let mut packet = packet();
    packet.entries[0].verification_state = VerificationState::Failed;
    let violations = packet.validate();
    assert!(
        violations.contains(&M5UpdateCenterSummaryViolation::EntryRollupDrift)
            || violations.contains(&M5UpdateCenterSummaryViolation::SummaryDrift),
        "{violations:?}"
    );
}

#[test]
fn tampered_rollback_disclosure_is_rejected() {
    let mut packet = packet();
    let idx = packet
        .entries
        .iter()
        .position(|e| !e.rollback.is_true_rollback())
        .expect("a non-rollback entry exists");
    packet.entries[idx].rollback_disclosed = true;
    assert!(packet
        .validate()
        .contains(&M5UpdateCenterSummaryViolation::RollbackOverclaim));
}

#[test]
fn dropping_a_family_is_rejected() {
    let mut packet = packet();
    packet
        .entries
        .retain(|e| e.family != ArtifactFamily::PolicyBundle);
    assert!(packet
        .validate()
        .contains(&M5UpdateCenterSummaryViolation::FamilyCoverageDrift));
}

#[test]
fn export_carries_no_raw_material() {
    for packet in [
        seeded_m5_update_center_summary(),
        seeded_m5_update_center_summary_stale_data_narrowed(),
        seeded_m5_update_center_summary_not_provided_blocked(),
    ] {
        assert!(packet.conformance.export_carries_no_raw_material);
        assert!(!packet
            .export_safe_json()
            .to_ascii_lowercase()
            .contains("bearer_token"));
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
