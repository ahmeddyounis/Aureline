//! Inline tests for the release-note evidence-set lane.

use super::*;

fn packet() -> ReleaseNoteEvidenceSet {
    seeded_m5_release_note_evidence_set()
}

#[test]
fn canonical_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_RELEASE_NOTE_EVIDENCE_SET_PACKET_ID);
    assert_eq!(packet.record_kind, M5_RELEASE_NOTE_EVIDENCE_SET_RECORD_KIND);
    assert_eq!(packet.notes.len(), ChangeClass::ALL.len());
    assert_eq!(packet.consumers.len(), ReleaseNoteConsumer::ALL.len());
    assert!(packet.conformance.all_hold());
    assert!(packet.vocabulary.matches_canonical());
}

#[test]
fn every_change_class_appears_once_and_carries_its_class() {
    // Acceptance / vocabulary: the representative release covers every controlled change class.
    let packet = packet();
    for class in ChangeClass::ALL {
        let matches: Vec<&ReleaseNoteEvidenceRow> = packet
            .notes
            .iter()
            .filter(|n| n.change_class == class)
            .collect();
        assert_eq!(
            matches.len(),
            1,
            "class `{}` not present once",
            class.as_str()
        );
        let note = matches[0];
        assert_eq!(note.change_class_label, class.label());
        assert_eq!(note.owner_role, class.owner_role());
    }
}

#[test]
fn behavior_changing_and_security_notes_are_evidence_backed() {
    // Acceptance: link each behavior-changing or security-sensitive note to substantive evidence
    // rather than prose alone.
    let packet = packet();
    for note in &packet.notes {
        if note
            .change_class
            .is_behavior_changing_or_security_sensitive()
        {
            assert!(
                note.has_substantive_evidence,
                "note `{}` ({}) is not evidence-backed",
                note.note_id,
                note.change_class.as_str()
            );
        }
    }
    assert!(packet.coverage.all_required_links_present);
}

#[test]
fn breaking_and_migration_notes_link_directly() {
    // Acceptance: breaking or migration-relevant notes link directly to the setting / import / rollback
    // surface.
    let packet = packet();
    for note in &packet.notes {
        if note.change_class.requires_direct_action_link() {
            assert!(
                note.has_direct_action_link,
                "note `{}` ({}) has no direct-action link",
                note.note_id,
                note.change_class.as_str()
            );
            assert!(note
                .evidence_links
                .iter()
                .any(|l| l.direct_action && l.kind.is_direct_action()));
        }
    }
}

#[test]
fn security_note_links_to_advisory() {
    let packet = packet();
    let sec = packet.note("security_dependency_advisory").unwrap();
    assert_eq!(sec.change_class, ChangeClass::Security);
    assert!(sec
        .evidence_links
        .iter()
        .any(|l| l.kind == EvidenceLinkKind::SecurityAdvisory));
    assert!(sec.has_substantive_evidence);
    assert_eq!(sec.note_readiness, NoteReadiness::ActionRequired);
}

#[test]
fn docs_only_note_is_informational_and_needs_no_substantive_evidence() {
    // Guardrail: a routine docs-only note must not read like a behavior change, and is allowed to carry
    // only a docs link.
    let packet = packet();
    let docs = packet.note("docs_only_quickstart").unwrap();
    assert_eq!(docs.change_class, ChangeClass::DocsOnly);
    assert!(!docs
        .change_class
        .is_behavior_changing_or_security_sensitive());
    assert_eq!(docs.gate, DescriptorGate::Governed);
    assert_eq!(docs.note_readiness, NoteReadiness::Informational);
    assert!(!docs.requires_user_action);
}

#[test]
fn whats_new_cards_are_dismissible_reopenable_and_non_blocking() {
    // Acceptance + guardrail: what's-new cards are dismissible / reopenable and never block workflows.
    for packet in [
        seeded_m5_release_note_evidence_set(),
        seeded_m5_release_note_evidence_set_dismissed(),
        seeded_m5_release_note_evidence_set_security_and_migration(),
    ] {
        for note in &packet.notes {
            let card = &note.whats_new_card;
            assert!(card.dismissible && card.reopenable);
            assert!(card.is_non_blocking());
            assert!(
                !card.blocks_typing
                    && !card.blocks_save
                    && !card.blocks_restore
                    && !card.blocks_recovery
            );
            assert!(card.is_reopenable_everywhere());
            assert!(card.reopen_surfaces.contains(&ReopenSurface::UpdateCenter));
            assert!(card.reopen_surfaces.contains(&ReopenSurface::HelpCenter));
        }
        assert!(packet.coverage.all_cards_reopenable);
        assert!(packet.coverage.all_cards_non_blocking);
    }
}

#[test]
fn dismissed_cards_remain_reopenable() {
    // Acceptance: users can reopen dismissed release communication later from the update center or Help.
    let packet = seeded_m5_release_note_evidence_set_dismissed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.summary.dismissed_notes, packet.notes.len() as u32);
    assert_eq!(packet.summary.reopenable_notes, packet.notes.len() as u32);
    for note in &packet.notes {
        assert_eq!(
            note.whats_new_card.dismiss_state,
            WhatsNewDismissState::Dismissed
        );
        assert!(note.whats_new_card.is_reopenable_everywhere());
    }
}

#[test]
fn docs_only_release_keeps_every_consumer_informational() {
    let packet = seeded_m5_release_note_evidence_set_docs_only();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    for c in &packet.consumers {
        assert!(
            c.is_informational(),
            "consumer `{}` not informational",
            c.consumer.as_str()
        );
        assert!(c.gaps.is_empty());
        assert!(!c.requires_user_action);
    }
    assert!(!packet.requires_user_action());
    assert_eq!(
        packet.summary.informational_consumers,
        ReleaseNoteConsumer::ALL.len() as u32
    );
}

#[test]
fn action_required_note_drives_every_consumer_that_reads_it() {
    // A breaking / migration / security note surfaces as action-required for every consumer reading it.
    let packet = seeded_m5_release_note_evidence_set_security_and_migration();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.requires_user_action());
    for c in &packet.consumers {
        assert!(
            c.is_action_required(),
            "consumer `{}` not action-required",
            c.consumer.as_str()
        );
        assert!(c.requires_user_action);
        assert!(c
            .gaps
            .iter()
            .any(|g| g.gap_kind == NoteGapKind::ActionRequired));
    }
    assert!(!packet.action_gate.action_required_notes.is_empty());
}

#[test]
fn consumers_read_one_note_set_and_derive_their_scope() {
    let packet = packet();
    assert_eq!(
        packet.consumer_tokens,
        tokens(&ReleaseNoteConsumer::ALL, |c| c.as_str())
    );
    assert!(packet.disclosure.all_consume());
    assert!(packet.conformance.consumers_read_one_note_set);
    for c in &packet.consumers {
        let mut expected: Vec<ArtifactClass> = Vec::new();
        for id in &c.read_note_ids {
            expected.extend(
                packet
                    .note(id)
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
    let desktop = packet.render_for_channel(ReleaseNoteChannel::DesktopUi);
    let cli = packet.render_for_channel(ReleaseNoteChannel::CliHeadless);
    let docs = packet.render_for_channel(ReleaseNoteChannel::DocsHelp);
    let export = packet.render_for_channel(ReleaseNoteChannel::OfflineExport);
    assert_eq!(desktop, cli);
    assert_eq!(cli, docs);
    assert_eq!(docs, export);
    assert_eq!(desktop, packet.export_safe_json());
}

#[test]
fn controlled_vocabulary_is_frozen() {
    let vocab = ReleaseNoteVocabulary::canonical();
    assert_eq!(vocab.change_classes.len(), ChangeClass::ALL.len());
    assert_eq!(vocab.consumers.len(), ReleaseNoteConsumer::ALL.len());
    for needle in [
        "breaking",
        "behavioral",
        "security",
        "policy",
        "compatibility",
        "docs_only",
        "admin_action_required",
        "deprecated",
        "migration_required",
    ] {
        assert!(
            vocab.change_classes.contains(&needle.to_owned()),
            "missing {needle}"
        );
    }
    for needle in [
        "evidence_packet",
        "security_advisory",
        "migration_doc",
        "certification_delta",
        "rollback_control",
        "setting_surface",
        "import_surface",
    ] {
        assert!(
            vocab.evidence_link_kinds.contains(&needle.to_owned()),
            "missing {needle}"
        );
    }
}

#[test]
fn packet_round_trips() {
    for packet in [
        seeded_m5_release_note_evidence_set(),
        seeded_m5_release_note_evidence_set_dismissed(),
        seeded_m5_release_note_evidence_set_docs_only(),
        seeded_m5_release_note_evidence_set_security_and_migration(),
    ] {
        let json = packet.export_safe_json();
        let parsed: ReleaseNoteEvidenceSet =
            serde_json::from_str(&json).expect("packet deserializes");
        assert_eq!(parsed, packet);
        assert!(parsed.validate().is_empty(), "{:?}", parsed.validate());
    }
}

#[test]
fn note_csv_enumerates_every_note() {
    let csv = packet().render_note_csv();
    let header = csv.lines().next().unwrap();
    assert!(header.starts_with("note_id,change_class,note_readiness,"));
    assert!(header.contains("has_direct_action_link"));
    assert!(header.contains("reopenable"));
    let rows = csv.lines().count() - 1;
    assert_eq!(rows, packet().notes.len());
}

#[test]
fn markdown_summary_names_notes_and_consumers() {
    let md = seeded_m5_release_note_evidence_set_security_and_migration().render_markdown_summary();
    assert!(md.contains("release-note evidence"));
    assert!(md.contains("Release notes"));
    assert!(md.contains("breaking_extension_api"));
    assert!(md.contains("gap:"));
}

#[test]
fn tampering_evidence_link_off_a_breaking_note_is_rejected() {
    // Guardrail enforced in validation, not just the builder.
    let mut packet = packet();
    let idx = packet
        .notes
        .iter()
        .position(|n| n.change_class == ChangeClass::Breaking)
        .unwrap();
    packet.notes[idx].evidence_links.clear();
    packet.notes[idx].recompute();
    let violations = packet.validate();
    assert!(
        violations.contains(&ReleaseNoteViolation::MissingEvidenceLink)
            || violations.contains(&ReleaseNoteViolation::MissingDirectActionLink),
        "{violations:?}"
    );
}

#[test]
fn tampering_a_card_to_block_a_workflow_is_rejected() {
    let mut packet = packet();
    packet.notes[0].whats_new_card.blocks_recovery = true;
    assert!(packet
        .validate()
        .contains(&ReleaseNoteViolation::WhatsNewCardBlocksWorkflow));
}

#[test]
fn tampering_a_card_to_non_reopenable_is_rejected() {
    let mut packet = packet();
    packet.notes[0].whats_new_card.reopenable = false;
    assert!(packet
        .validate()
        .contains(&ReleaseNoteViolation::WhatsNewCardNotReopenable));
}

#[test]
fn tampered_consumer_verdict_is_rejected() {
    let mut packet = seeded_m5_release_note_evidence_set_security_and_migration();
    let idx = packet
        .consumers
        .iter()
        .position(|c| c.is_action_required())
        .unwrap();
    packet.consumers[idx].gate_decision = DescriptorGate::Governed;
    packet.consumers[idx].note_readiness = NoteReadiness::Informational;
    assert!(packet
        .validate()
        .contains(&ReleaseNoteViolation::ConsumerVerdictDrift));
}

#[test]
fn tampered_change_class_derivation_is_rejected() {
    let mut packet = packet();
    // Flip a docs-only note to breaking without recomputing: the derived gate now drifts.
    let idx = packet
        .notes
        .iter()
        .position(|n| n.change_class == ChangeClass::DocsOnly)
        .unwrap();
    packet.notes[idx].change_class = ChangeClass::Breaking;
    let violations = packet.validate();
    assert!(
        violations.contains(&ReleaseNoteViolation::NoteDerivationDrift)
            || violations.contains(&ReleaseNoteViolation::SummaryDrift),
        "{violations:?}"
    );
}

#[test]
fn export_carries_no_raw_material() {
    for packet in [
        seeded_m5_release_note_evidence_set(),
        seeded_m5_release_note_evidence_set_dismissed(),
        seeded_m5_release_note_evidence_set_docs_only(),
        seeded_m5_release_note_evidence_set_security_and_migration(),
    ] {
        assert!(packet.conformance.export_carries_no_raw_material);
        assert!(!packet
            .export_safe_json()
            .to_ascii_lowercase()
            .contains("bearer_token"));
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
