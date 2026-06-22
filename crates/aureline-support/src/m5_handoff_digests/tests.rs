//! Unit tests for the handoff/digest builder: the storage-class distinction, the
//! reopen-safe-continuity rule, digest grouping by severity before chronology,
//! latest-update and blocker preservation, unresolved questions, scope/boundary
//! truth, roll-up, and export parity.

use super::*;

#[test]
fn set_validates_and_all_invariants_hold() {
    let set = handoff_digest_set();
    set.validate()
        .expect("canonical handoff/digest set validates");
    assert!(set.all_invariants_hold());
    assert!(!set.invariants.is_empty());
}

#[test]
fn set_is_deterministic() {
    assert_eq!(handoff_digest_set(), handoff_digest_set());
}

#[test]
fn set_is_support_export_safe() {
    let set = handoff_digest_set();
    assert!(set.raw_payload_excluded);
    assert!(set.is_support_export_safe());
}

#[test]
fn every_packet_is_present_once_and_binds_the_matrix() {
    let set = handoff_digest_set();
    let matrix = crate::m5_operator_surfaces::operator_surface_matrix();
    assert_eq!(set.packets.len(), PacketClass::ALL.len());
    for class in PacketClass::ALL {
        let packet = set.packet(class).expect("packet present");
        assert_eq!(packet.packet_id, class.packet_id());
        assert_eq!(packet.kind, class.kind());
        assert_eq!(packet.surface, class.surface());
        assert_eq!(packet.surface_id, packet.surface.surface_id());
        assert!(matrix.surface(packet.surface).is_some());
        assert!(!packet.object_groups.is_empty());
    }
}

#[test]
fn both_surfaces_are_proven() {
    let set = handoff_digest_set();
    for kind in ContinuityPacketKind::ALL {
        assert!(
            set.packets.iter().any(|p| p.kind == kind),
            "kind {} must appear",
            kind.as_str()
        );
    }
}

#[test]
fn storage_classes_are_distinct_and_never_flattened() {
    let set = handoff_digest_set();
    let evidence: Vec<&EvidenceItem> = set
        .packets
        .iter()
        .flat_map(|p| p.object_groups.iter())
        .flat_map(|g| g.evidence.iter())
        .collect();
    for sc in StorageClass::ALL {
        assert!(
            evidence.iter().any(|e| e.storage_class == sc),
            "storage class {} must appear so the distinction is proven",
            sc.as_str()
        );
    }
    // is_live / can_refresh are computed from the storage class — never set by hand.
    for ev in &evidence {
        assert_eq!(ev.is_live, ev.storage_class.is_live(), "{}", ev.evidence_id);
        assert_eq!(
            ev.can_refresh,
            ev.storage_class.can_refresh(),
            "{}",
            ev.evidence_id
        );
    }
    // The roll-up counts each class separately, never merged into one bucket.
    for packet in &set.packets {
        let r = &packet.roll_up;
        let total = r.live_link_count + r.cached_count + r.mirrored_count + r.snapshot_count;
        let evidence_count: u32 = packet
            .object_groups
            .iter()
            .map(|g| g.evidence.len() as u32)
            .sum();
        assert_eq!(total, evidence_count, "{}", packet.packet.as_str());
    }
    // Only a live link is live; a snapshot can never refresh.
    assert!(StorageClass::LiveLink.is_live());
    assert!(!StorageClass::Cached.is_live());
    assert!(!StorageClass::Snapshot.can_refresh());
}

#[test]
fn reopen_anchors_land_on_object_or_truthful_placeholder() {
    let set = handoff_digest_set();
    let anchors: Vec<&ReopenAnchor> = set
        .packets
        .iter()
        .flat_map(|p| {
            std::iter::once(&p.reopen_anchor)
                .chain(p.object_groups.iter().map(|g| &g.reopen_anchor))
        })
        .collect();
    for anchor in &anchors {
        assert_eq!(
            anchor.resolves_object,
            compute_resolves_object(anchor.anchor_class)
        );
        if anchor.anchor_class == ReopenAnchorClass::TruthfulPlaceholder {
            assert!(anchor.target_ref.is_empty());
            assert!(
                !anchor.placeholder_label.is_empty(),
                "placeholder must name the object"
            );
            assert!(!anchor.resolves_object);
        } else {
            assert!(anchor.target_ref.starts_with("aureline://"));
            assert!(anchor.placeholder_label.is_empty());
            assert!(anchor.resolves_object);
        }
    }
    // All four anchor classes are proven, including the placeholder.
    for class in ReopenAnchorClass::ALL {
        assert!(
            anchors.iter().any(|a| a.anchor_class == class),
            "anchor class {} must appear",
            class.as_str()
        );
    }
}

#[test]
fn digests_group_by_severity_before_chronology() {
    let set = handoff_digest_set();
    for packet in set
        .packets
        .iter()
        .filter(|p| p.kind == ContinuityPacketKind::ShiftDigest)
    {
        // Groups ordered by severity, most severe first.
        for w in packet.object_groups.windows(2) {
            assert!(
                w[0].severity.rank() >= w[1].severity.rank(),
                "{} groups must be severity-ordered",
                packet.packet.as_str()
            );
        }
        // Within each group, events are chronological and the group severity is the
        // most severe event.
        for g in &packet.object_groups {
            for w in g.events.windows(2) {
                assert!(
                    w[0].at <= w[1].at,
                    "{} events must be chronological",
                    g.group_id
                );
            }
            let max = g.events.iter().map(|e| e.severity).max_by_key(|s| s.rank());
            assert_eq!(
                max,
                Some(g.severity),
                "{} severity must be its worst event",
                g.group_id
            );
        }
    }
}

#[test]
fn latest_update_and_blockers_are_preserved() {
    let set = handoff_digest_set();
    for packet in &set.packets {
        for g in &packet.object_groups {
            let latest = g.events.iter().map(|e| e.at.as_str()).max().unwrap();
            assert_eq!(g.latest_update_at, latest, "{}", g.group_id);
            if g.blocker.requires_reason() {
                assert!(
                    !g.blocker_reason.is_empty(),
                    "{} blocked must name a reason",
                    g.group_id
                );
            }
        }
        let roll_latest = packet
            .object_groups
            .iter()
            .map(|g| g.latest_update_at.as_str())
            .max()
            .unwrap();
        assert_eq!(packet.roll_up.latest_update_at, roll_latest);
    }
}

#[test]
fn unresolved_questions_answer_what_to_do_next() {
    let set = handoff_digest_set();
    for packet in &set.packets {
        assert!(
            !packet.unresolved_questions.is_empty(),
            "{}",
            packet.packet.as_str()
        );
        for q in &packet.unresolved_questions {
            assert!(!q.owner.is_empty());
            assert!(q.linked_object_ref.starts_with("aureline://"));
            assert!(
                !q.next_safe_action.is_empty(),
                "{} must name a next safe action",
                q.question_id
            );
            if q.status.requires_reason() {
                assert!(
                    !q.blocker_reason.is_empty(),
                    "{} blocked must name a reason",
                    q.question_id
                );
            }
        }
    }
    // Every question status is proven.
    let statuses: Vec<QuestionStatus> = set
        .packets
        .iter()
        .flat_map(|p| p.unresolved_questions.iter().map(|q| q.status))
        .collect();
    for status in QuestionStatus::ALL {
        assert!(
            statuses.contains(&status),
            "status {} must appear",
            status.as_str()
        );
    }
}

#[test]
fn scope_and_boundary_truth_holds_for_every_packet() {
    let set = handoff_digest_set();
    for packet in &set.packets {
        assert_eq!(packet.share_posture.scope(), packet.scope);
        let gate = &packet.export_gate;
        assert_eq!(gate.scope, packet.scope);
        assert_eq!(gate.share_posture, packet.share_posture);
        assert_eq!(
            gate.requires_boundary_ack,
            packet.share_posture.requires_boundary_ack()
        );
        assert_eq!(gate.redaction_class, packet.default_redaction);
        assert!(!gate.crosses_on_share.is_empty());
        assert!(gate.raw_payload_excluded);
    }
    // A private, a workspace-shared, and an org-shared packet are all present.
    for posture in SharePosture::ALL {
        assert!(set.packets.iter().any(|p| p.share_posture == posture));
    }
}

#[test]
fn export_parity_and_storage_preservation_hold() {
    let set = handoff_digest_set();
    for packet in &set.packets {
        let exported = export_packet(packet);
        assert_eq!(
            exported,
            packet.export,
            "{} export parity",
            packet.packet.as_str()
        );
        assert_eq!(
            packet.export.live_vs_snapshot,
            LiveSnapshotClass::SnapshotOnly
        );
        // The frozen export preserves the storage distinction byte-for-byte.
        assert_eq!(packet.export.object_groups, packet.object_groups);
        assert_eq!(
            packet.export.unresolved_questions,
            packet.unresolved_questions
        );
        assert_eq!(packet.export.reopen_anchor, packet.reopen_anchor);
    }
}

#[test]
fn roll_up_answers_the_three_handoff_questions() {
    let set = handoff_digest_set();
    for packet in &set.packets {
        let r = &packet.roll_up;
        assert!(!r.what_changed.is_empty());
        assert!(!r.what_unresolved.is_empty());
        assert!(!r.next_safe_action.is_empty());
        assert!(r.headline.contains("What changed"));
        assert!(r.headline.contains("What remains unresolved"));
        assert!(r.headline.contains("Next safe action"));
        assert!(r.headline.contains("never flattened"));
        assert_eq!(r, &compute_roll_up(packet));
    }
}

#[test]
fn every_object_kind_and_severity_is_proven() {
    let set = handoff_digest_set();
    let groups: Vec<&ObjectGroup> = set
        .packets
        .iter()
        .flat_map(|p| p.object_groups.iter())
        .collect();
    for kind in ObjectKind::ALL {
        assert!(
            groups.iter().any(|g| g.object_kind == kind),
            "kind {} must appear",
            kind.as_str()
        );
    }
    for sev in SeverityClass::ALL {
        assert!(
            groups.iter().any(|g| g.severity == sev),
            "severity {} must appear",
            sev.as_str()
        );
    }
}

#[test]
fn local_safe_actions_are_offered_and_computed() {
    let set = handoff_digest_set();
    for packet in &set.packets {
        assert!(packet
            .actions
            .iter()
            .any(|a| a.action == ContinuityActionClass::ReopenAtAnchor && a.local_safe));
        assert!(packet
            .actions
            .iter()
            .any(|a| a.action == ContinuityActionClass::ExportSnapshot && a.local_safe));
        // share_packet is the only non-local-safe action.
        for a in &packet.actions {
            assert_eq!(a.local_safe, a.action.local_safe(), "{}", a.action.as_str());
            assert_eq!(
                a.routes_to_canonical_object,
                a.action.routes_to_canonical_object()
            );
        }
    }
}
