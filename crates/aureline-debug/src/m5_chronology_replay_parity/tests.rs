//! Unit tests for the M5 chronology/replay/parity set.

use super::*;

#[test]
fn canonical_set_validates_and_is_export_safe() {
    let set = m5_chronology_replay_parity_set();
    set.validate().expect("canonical set validates");
    assert!(set.is_support_export_safe());
    assert!(set.all_invariants_hold());
    assert!(set.raw_payload_excluded);
}

#[test]
fn canonical_set_round_trips_through_serde() {
    let set = m5_chronology_replay_parity_set();
    let json = serde_json::to_string(&set).expect("serializes");
    let back: ChronologyReplayParitySet = serde_json::from_str(&json).expect("round-trips");
    assert_eq!(set, back);
}

#[test]
fn every_support_class_is_materialized_across_chronology() {
    let set = m5_chronology_replay_parity_set();
    for class in DebugSupportClass::ALL {
        assert!(
            set.chronology_in_support_class(class).is_some(),
            "missing chronology for support class {}",
            class.as_str()
        );
    }
}

#[test]
fn one_support_vocabulary_is_shared_across_surfaces() {
    let set = m5_chronology_replay_parity_set();
    // Live debug / chronology, replay, and notebook all carry support classes from the one
    // vocabulary, and each has a supported member.
    assert!(set
        .chronology_capabilities
        .iter()
        .any(|c| c.support_class == DebugSupportClass::Supported));
    assert!(set
        .replay_sessions
        .iter()
        .any(|r| r.support_class == DebugSupportClass::Supported));
    assert!(set
        .notebook_kernels
        .iter()
        .any(|k| k.support_class == DebugSupportClass::Supported));
}

#[test]
fn unsupported_backends_inherit_no_claims() {
    let set = m5_chronology_replay_parity_set();
    let inert: Vec<_> = set
        .chronology_capabilities
        .iter()
        .filter(|c| c.support_pill.is_inert)
        .collect();
    assert!(
        !inert.is_empty(),
        "an inert chronology descriptor must exist"
    );
    for c in inert {
        assert!(c.supported_verbs.is_empty());
        assert!(!c.support_pill.time_travel_available);
        assert!(!c.recorded_scope.records_history());
    }
    for k in &set.notebook_kernels {
        if k.support_pill.is_inert {
            assert!(k.supported_verbs.is_empty());
        }
    }
}

#[test]
fn time_travel_verbs_are_backed_only_when_replayable() {
    let set = m5_chronology_replay_parity_set();
    let pairs = set
        .chronology_capabilities
        .iter()
        .map(|c| (&c.supported_verbs, c.support_pill.time_travel_available))
        .chain(
            set.replay_sessions
                .iter()
                .map(|r| (&r.supported_verbs, r.support_pill.time_travel_available)),
        );
    for (verbs, available) in pairs {
        for v in verbs {
            if v.requires_time_travel() {
                assert!(
                    available,
                    "time-travel verb backed without a replayable timeline"
                );
            }
        }
        assert_eq!(available, !verbs.is_empty());
    }
}

#[test]
fn replay_sessions_are_inspect_only_and_capture_bound() {
    let set = m5_chronology_replay_parity_set();
    assert!(!set.replay_sessions.is_empty());
    for r in &set.replay_sessions {
        assert!(r.inspect_only, "replay session must be inspect-only");
        assert!(r.capture.is_fully_bound());
        assert!(
            set.chronology(&r.source_chronology_ref).is_some(),
            "replay session {} sources from a chronology descriptor in the set",
            r.replay_session_id
        );
    }
}

#[test]
fn timeline_bookmarks_bind_to_one_capture_and_survive_export() {
    let set = m5_chronology_replay_parity_set();
    assert!(!set.timeline_bookmarks.is_empty());
    for b in &set.timeline_bookmarks {
        assert!(b.capture.is_fully_bound());
        assert!(b.survives_support_export);
        assert!(b.survives_restore_review);
        let rs = set
            .replay_session(&b.replay_session_ref)
            .expect("bookmark resolves to a replay session");
        assert!(
            rs.capture.same_as(&b.capture),
            "bookmark {} capture identity must match its replay session",
            b.bookmark_id
        );
    }
}

#[test]
fn every_restart_consequence_itemizes_the_five_subjects() {
    let set = m5_chronology_replay_parity_set();
    assert!(!set.restart_consequences.is_empty());
    for c in &set.restart_consequences {
        assert!(
            c.itemizes_every_subject(),
            "consequence {} must itemize every subject",
            c.consequence_id
        );
        for subject in ConsequenceSubject::ALL {
            assert!(
                c.disposition_for(subject).is_some(),
                "consequence {} must name subject {}",
                c.consequence_id,
                subject.as_str()
            );
        }
    }
}

#[test]
fn restart_consequences_cover_notebook_debug_and_replay() {
    let set = m5_chronology_replay_parity_set();
    let has = |t: ConsequenceTrigger| set.restart_consequences.iter().any(|c| c.trigger == t);
    // notebook
    assert!(
        has(ConsequenceTrigger::KernelRestart) || has(ConsequenceTrigger::TransportLostReconnect)
    );
    // debug
    assert!(has(ConsequenceTrigger::SessionRestart) || has(ConsequenceTrigger::Reconnect));
    // replay
    assert!(has(ConsequenceTrigger::ReplayReacquire));
}

#[test]
fn the_full_disposition_and_trigger_vocabularies_are_materialized() {
    let set = m5_chronology_replay_parity_set();
    for d in ConsequenceDisposition::ALL {
        assert!(
            set.restart_consequences
                .iter()
                .flat_map(|c| c.entries.iter())
                .any(|e| e.disposition == d),
            "disposition {} is not materialized",
            d.as_str()
        );
    }
    for t in ConsequenceTrigger::ALL {
        assert!(
            set.restart_consequences.iter().any(|c| c.trigger == t),
            "trigger {} is not materialized",
            t.as_str()
        );
    }
}

#[test]
fn a_cell_link_renders_exact_only_when_exact_and_supported() {
    let set = m5_chronology_replay_parity_set();
    for l in &set.cell_frame_links {
        assert_eq!(
            l.renders_exact_link,
            l.fidelity.is_exact() && l.support_class.permits_use(),
            "link {} exact-link flag must match its fidelity and support",
            l.link_id
        );
    }
    // An exact, supported link exists, and a degraded one is never drawn exact.
    assert!(set.cell_frame_links.iter().any(|l| l.renders_exact_link));
    assert!(set
        .cell_frame_links
        .iter()
        .any(|l| !l.fidelity.is_exact() && !l.renders_exact_link));
}

#[test]
fn the_full_cell_link_fidelity_vocabulary_is_materialized() {
    let set = m5_chronology_replay_parity_set();
    for f in CellLinkFidelity::ALL {
        assert!(
            set.cell_frame_links.iter().any(|l| l.fidelity == f),
            "cell-link fidelity {} is not materialized",
            f.as_str()
        );
    }
}

#[test]
fn notebook_cross_references_resolve() {
    let set = m5_chronology_replay_parity_set();
    for l in &set.cell_frame_links {
        assert!(
            set.notebook_kernel(&l.kernel_ref).is_some(),
            "cell-frame link {} resolves to a kernel",
            l.link_id
        );
    }
    for k in &set.notebook_kernels {
        assert!(
            set.restart_consequence(&k.restart_consequence_ref)
                .is_some(),
            "kernel {} resolves its restart consequence",
            k.kernel_id
        );
    }
}

#[test]
fn timeline_state_and_support_class_vocabularies_are_complete() {
    let set = m5_chronology_replay_parity_set();
    // Every timeline state is materialized somewhere across the descriptors.
    let all_states: std::collections::BTreeSet<TimelineState> = set
        .chronology_capabilities
        .iter()
        .map(|c| c.timeline_state)
        .chain(set.replay_sessions.iter().map(|r| r.timeline_state))
        .chain(set.notebook_kernels.iter().map(|k| k.timeline_state))
        .collect();
    for state in TimelineState::ALL {
        assert!(
            all_states.contains(&state),
            "timeline state {} is not materialized",
            state.as_str()
        );
    }
}

#[test]
fn tampering_with_a_support_pill_fails_validation() {
    let mut set = m5_chronology_replay_parity_set();
    // Force an unavailable chronology descriptor to claim time travel is available.
    let c = set
        .chronology_capabilities
        .iter_mut()
        .find(|c| c.support_pill.is_inert)
        .expect("an inert descriptor exists");
    c.support_pill.time_travel_available = true;
    assert!(
        set.validate().is_err(),
        "an inert descriptor claiming time travel must fail validation"
    );
}

#[test]
fn granting_an_inert_backend_verbs_fails_validation() {
    let mut set = m5_chronology_replay_parity_set();
    let c = set
        .chronology_capabilities
        .iter_mut()
        .find(|c| c.support_pill.is_inert)
        .expect("an inert descriptor exists");
    c.supported_verbs = vec![CapabilityVerb::ReverseStep];
    assert!(
        set.validate().is_err(),
        "an inert descriptor granted a verb must fail validation"
    );
}

#[test]
fn flattening_a_restart_consequence_fails_validation() {
    let mut set = m5_chronology_replay_parity_set();
    // Drop all but one subject — a flattened, banner-style consequence.
    let c = set
        .restart_consequences
        .first_mut()
        .expect("a consequence exists");
    c.entries.truncate(1);
    assert!(
        set.validate().is_err(),
        "a restart consequence that no longer itemizes every subject must fail validation"
    );
}

#[test]
fn drawing_a_stale_link_exact_fails_validation() {
    let mut set = m5_chronology_replay_parity_set();
    let l = set
        .cell_frame_links
        .iter_mut()
        .find(|l| !l.fidelity.is_exact())
        .expect("a non-exact link exists");
    l.renders_exact_link = true;
    assert!(
        set.validate().is_err(),
        "a non-exact link drawn exact must fail validation"
    );
}

#[test]
fn making_a_replay_session_mutable_fails_validation() {
    let mut set = m5_chronology_replay_parity_set();
    let r = set
        .replay_sessions
        .first_mut()
        .expect("a replay session exists");
    r.inspect_only = false;
    assert!(
        set.validate().is_err(),
        "a mutable replay session must fail validation"
    );
}

#[test]
fn orphaning_a_bookmark_from_its_capture_fails_validation() {
    let mut set = m5_chronology_replay_parity_set();
    let b = set
        .timeline_bookmarks
        .first_mut()
        .expect("a bookmark exists");
    b.capture.capture_id = "debug.capture:foreign:9999".to_owned();
    assert!(
        set.validate().is_err(),
        "a bookmark whose capture no longer matches its replay session must fail validation"
    );
}

#[test]
fn lines_projection_covers_every_section() {
    let set = m5_chronology_replay_parity_set();
    let lines = m5_chronology_replay_parity_lines(&set);
    assert!(lines.iter().any(|l| l.contains("Chronology capabilities:")));
    assert!(lines.iter().any(|l| l.contains("Replay sessions:")));
    assert!(lines.iter().any(|l| l.contains("Timeline bookmarks:")));
    assert!(lines.iter().any(|l| l.contains("Notebook kernels:")));
    assert!(lines.iter().any(|l| l.contains("Cell-frame links:")));
    assert!(lines
        .iter()
        .any(|l| l.contains("Restart/reconnect consequences:")));
    assert!(lines.iter().any(|l| l.contains("Invariants:")));
    assert!(lines.iter().any(|l| l.contains("support=supported")));
    assert!(lines.iter().any(|l| l.contains("support=policy_blocked")));
    assert!(lines.iter().any(|l| l.contains("-> preserved")));
    assert!(lines.iter().any(|l| l.contains("-> lost")));
}

#[test]
fn enum_tokens_are_stable_and_unique() {
    fn unique<const N: usize>(tokens: [&str; N]) -> bool {
        tokens
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            == N
    }
    assert!(unique(RuntimeBackendFamily::ALL.map(|f| f.as_str())));
    assert!(unique(DebugSupportClass::ALL.map(|s| s.as_str())));
    assert!(unique(TimelineState::ALL.map(|t| t.as_str())));
    assert!(unique(CapabilityVerb::ALL.map(|v| v.as_str())));
    assert!(unique(RecordedScope::ALL.map(|r| r.as_str())));
    assert!(unique(NotebookParityClass::ALL.map(|p| p.as_str())));
    assert!(unique(BookmarkKind::ALL.map(|k| k.as_str())));
    assert!(unique(CellLinkFidelity::ALL.map(|f| f.as_str())));
    assert!(unique(ConsequenceTrigger::ALL.map(|t| t.as_str())));
    assert!(unique(ConsequenceSubject::ALL.map(|s| s.as_str())));
    assert!(unique(ConsequenceDisposition::ALL.map(|d| d.as_str())));
}
