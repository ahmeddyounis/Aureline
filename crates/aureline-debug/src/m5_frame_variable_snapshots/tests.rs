//! Unit tests for the M5 frame-mapping and variable/watch snapshot set.

use super::*;

#[test]
fn canonical_set_validates_and_is_export_safe() {
    let set = m5_frame_variable_snapshot_set();
    set.validate().expect("canonical set validates");
    assert!(set.is_support_export_safe());
    assert!(set.all_invariants_hold());
    assert!(set.raw_payload_excluded);
}

#[test]
fn canonical_set_round_trips_through_serde() {
    let set = m5_frame_variable_snapshot_set();
    let json = serde_json::to_string(&set).expect("serializes");
    let back: FrameVariableSnapshotSet = serde_json::from_str(&json).expect("round-trips");
    assert_eq!(set, back);
}

#[test]
fn every_fidelity_and_disclosure_is_materialized() {
    let set = m5_frame_variable_snapshot_set();
    for fidelity in FrameMappingFidelity::ALL {
        assert!(
            set.frame_in_fidelity(fidelity).is_some(),
            "missing frame for fidelity {}",
            fidelity.as_str()
        );
    }
    for disclosure in ValueDisclosure::ALL {
        assert!(
            set.snapshot_in_disclosure(disclosure).is_some(),
            "missing snapshot for disclosure {}",
            disclosure.as_str()
        );
    }
}

#[test]
fn both_entry_kinds_are_materialized() {
    let set = m5_frame_variable_snapshot_set();
    for kind in SnapshotEntryKind::ALL {
        assert!(
            set.snapshots.iter().any(|s| s.entry_kind == kind),
            "missing snapshot for entry kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn only_exact_with_verified_build_shows_precise_source_link() {
    let set = m5_frame_variable_snapshot_set();
    for fr in &set.frames {
        let expected = fr.fidelity().preserves_exact_source()
            && fr.build_identity.match_state.proves_exact_build();
        assert_eq!(
            fr.pill.shows_exact_source_link, expected,
            "frame {} precise-link flag must equal its derivation",
            fr.frame_id
        );
        assert_eq!(
            fr.pill.requires_disclosure,
            !fr.pill.shows_exact_source_link
        );
    }
}

#[test]
fn approximate_symbol_only_unmapped_and_mismatch_never_show_precise_link() {
    let set = m5_frame_variable_snapshot_set();
    for fr in &set.frames {
        let dishonest = fr.pill.shows_exact_source_link
            && (fr.fidelity() != FrameMappingFidelity::Exact
                || fr.build_identity.match_state != BuildMatchClass::ExactBuildVerified);
        assert!(
            !dishonest,
            "frame {} shows a precise source link that hides a caveat",
            fr.frame_id
        );
    }
}

#[test]
fn current_frame_identity_is_preserved_per_thread() {
    let set = m5_frame_variable_snapshot_set();
    let mut threads: Vec<(&str, &str)> = Vec::new();
    for fr in &set.frames {
        let key = (fr.session_id.as_str(), fr.thread_id.as_str());
        if !threads.contains(&key) {
            threads.push(key);
        }
    }
    assert!(threads.len() >= 2, "multiple threads must be materialized");
    for (s, t) in threads {
        let current = set
            .frames
            .iter()
            .filter(|f| f.session_id == s && f.thread_id == t && f.is_current_frame)
            .count();
        assert_eq!(
            current, 1,
            "thread {s}/{t} must have exactly one current frame"
        );
    }
    // The selected frame is tracked distinctly from the current frame.
    assert!(set
        .frames
        .iter()
        .any(|f| f.is_selected_frame && !f.is_current_frame));
}

#[test]
fn lost_mapping_degrades_to_explicit_unmapped() {
    let set = m5_frame_variable_snapshot_set();
    let unresolved: Vec<_> = set
        .frames
        .iter()
        .filter(|f| f.mapping_provenance == FrameMappingProvenance::Unresolved)
        .collect();
    assert!(!unresolved.is_empty(), "an unresolved frame must exist");
    for fr in unresolved {
        assert_eq!(
            fr.fidelity(),
            FrameMappingFidelity::Unmapped,
            "unresolved frame {} must degrade to an explicit unmapped frame",
            fr.frame_id
        );
    }
    for fr in &set.frames {
        if fr.fidelity() == FrameMappingFidelity::Unmapped {
            assert_eq!(
                fr.mapping_provenance,
                FrameMappingProvenance::Unresolved,
                "unmapped frame {} must trace to an unresolved provenance",
                fr.frame_id
            );
        }
    }
}

#[test]
fn source_map_provenance_always_discloses() {
    let set = m5_frame_variable_snapshot_set();
    let sourcemap: Vec<_> = set
        .frames
        .iter()
        .filter(|f| f.mapping_provenance.is_source_map())
        .collect();
    assert!(!sourcemap.is_empty(), "a source-map frame must exist");
    for fr in sourcemap {
        assert!(fr.mapping_provenance_requires_disclosure);
        assert!(
            fr.pill.label.contains("source-map"),
            "source-map frame {} must disclose its provenance in the pill",
            fr.frame_id
        );
    }
}

#[test]
fn async_boundary_stays_visible() {
    let set = m5_frame_variable_snapshot_set();
    let boundary: Vec<_> = set.frames.iter().filter(|f| f.is_async_boundary).collect();
    assert!(!boundary.is_empty(), "an async-boundary frame must exist");
    for fr in boundary {
        assert!(fr.continuity.is_async_boundary());
        assert!(
            fr.pill.label.contains("async boundary"),
            "async-boundary frame {} must disclose its boundary",
            fr.frame_id
        );
    }
    // A contiguous frame never claims a boundary.
    for fr in &set.frames {
        if fr.continuity == FrameContinuityClass::Contiguous {
            assert!(!fr.is_async_boundary);
        }
    }
}

#[test]
fn value_implies_live_authority_only_when_truly_live() {
    let set = m5_frame_variable_snapshot_set();
    for sn in &set.snapshots {
        let is_live = sn.disclosure_class() == ValueDisclosure::Live;
        assert_eq!(sn.disclosure.implies_live_authority, is_live);
        assert_eq!(sn.disclosure.is_live_read, is_live);
        assert_eq!(sn.disclosure.requires_disclosure, !is_live);
    }
    // At least one captured or stale value exists and never implies live authority.
    assert!(set.snapshots.iter().any(|s| {
        matches!(
            s.disclosure_class(),
            ValueDisclosure::Captured | ValueDisclosure::Stale
        ) && !s.disclosure.implies_live_authority
    }));
}

#[test]
fn unavailable_snapshots_name_a_reason_and_withhold_body() {
    let set = m5_frame_variable_snapshot_set();
    let unavailable: Vec<_> = set
        .snapshots
        .iter()
        .filter(|s| s.disclosure_class() == ValueDisclosure::Unavailable)
        .collect();
    assert!(
        !unavailable.is_empty(),
        "an unavailable snapshot must exist"
    );
    for sn in unavailable {
        assert!(sn.unavailable_reason.is_some());
        assert_eq!(
            sn.unavailable_reason_token.as_deref(),
            sn.unavailable_reason.map(VariableUnavailableReason::as_str)
        );
        assert!(sn.value_repr_digest.is_none());
        assert!(!sn.disclosure.value_body_present);
    }
    // A non-unavailable snapshot carries no reason.
    for sn in &set.snapshots {
        if sn.disclosure_class() != ValueDisclosure::Unavailable {
            assert!(sn.unavailable_reason.is_none());
        }
    }
}

#[test]
fn redacted_snapshots_withhold_the_value_body() {
    let set = m5_frame_variable_snapshot_set();
    let redacted: Vec<_> = set
        .snapshots
        .iter()
        .filter(|s| s.redaction.is_redacted())
        .collect();
    assert!(!redacted.is_empty(), "a redacted snapshot must exist");
    for sn in redacted {
        assert_eq!(sn.disclosure_class(), ValueDisclosure::Redacted);
        assert!(sn.value_repr_digest.is_none());
        assert!(sn.disclosure.is_redacted);
        assert!(!sn.disclosure.value_body_present);
    }
}

#[test]
fn variables_and_watches_share_one_vocabulary() {
    let set = m5_frame_variable_snapshot_set();
    for sn in &set.snapshots {
        match sn.entry_kind {
            SnapshotEntryKind::Watch => {
                assert!(sn.scope.is_watch_scope());
                assert!(sn.watch_expression_digest.is_some());
            }
            SnapshotEntryKind::Variable => {
                assert!(!sn.scope.is_watch_scope());
                assert!(sn.watch_expression_digest.is_none());
            }
        }
        // Both kinds draw their disclosure from the same shared vocabulary.
        assert!(ValueDisclosure::ALL.contains(&sn.disclosure_class()));
    }
}

#[test]
fn notebook_and_replay_inspectors_reuse_the_snapshot_vocabulary() {
    let set = m5_frame_variable_snapshot_set();
    assert!(set
        .snapshots
        .iter()
        .any(|s| s.capture_context.notebook_cell_ref.is_some()));
    assert!(set
        .snapshots
        .iter()
        .any(|s| s.capture_context.replay_capture_ref.is_some()));
    for sn in &set.snapshots {
        if sn.capture_context.notebook_cell_ref.is_some()
            || sn.capture_context.replay_capture_ref.is_some()
        {
            assert!(ValueDisclosure::ALL.contains(&sn.disclosure_class()));
        }
    }
}

#[test]
fn lazy_loadable_and_truncation_are_disclosed() {
    let set = m5_frame_variable_snapshot_set();
    assert!(
        set.snapshots.iter().any(|s| s.lazy_loadable),
        "a lazy-loadable snapshot must exist"
    );
    assert!(
        set.snapshots.iter().any(|s| s.truncation.is_truncated),
        "a truncated snapshot must exist"
    );
    for sn in &set.snapshots {
        assert!(sn.truncation.is_consistent());
        assert!(sn.type_shape.is_consistent());
    }
}

#[test]
fn tampering_with_a_frame_pill_fails_validation() {
    let mut set = m5_frame_variable_snapshot_set();
    // Force an unmapped frame to claim a precise source link.
    let fr = set
        .frames
        .iter_mut()
        .find(|f| f.fidelity() == FrameMappingFidelity::Unmapped)
        .expect("an unmapped frame exists");
    fr.pill.shows_exact_source_link = true;
    fr.pill.requires_disclosure = false;
    assert!(
        set.validate().is_err(),
        "an unmapped frame claiming a precise source link must fail validation"
    );
}

#[test]
fn tampering_with_a_live_disclosure_fails_validation() {
    let mut set = m5_frame_variable_snapshot_set();
    // Force a stale value to claim a live read.
    let sn = set
        .snapshots
        .iter_mut()
        .find(|s| s.disclosure_class() == ValueDisclosure::Stale)
        .expect("a stale snapshot exists");
    sn.disclosure.is_live_read = true;
    sn.disclosure.implies_live_authority = true;
    assert!(
        set.validate().is_err(),
        "a stale value claiming a live read must fail validation"
    );
}

#[test]
fn removing_an_unavailable_reason_fails_validation() {
    let mut set = m5_frame_variable_snapshot_set();
    let sn = set
        .snapshots
        .iter_mut()
        .find(|s| s.disclosure_class() == ValueDisclosure::Unavailable)
        .expect("an unavailable snapshot exists");
    sn.unavailable_reason = None;
    sn.unavailable_reason_token = None;
    assert!(
        set.validate().is_err(),
        "an unavailable value without a reason must fail validation"
    );
}

#[test]
fn a_second_current_frame_in_one_thread_fails_validation() {
    let mut set = m5_frame_variable_snapshot_set();
    // Mark a non-current frame on the main thread as current too.
    let main_thread = set.frames[0].thread_id.clone();
    let fr = set
        .frames
        .iter_mut()
        .find(|f| f.thread_id == main_thread && !f.is_current_frame)
        .expect("a non-current main-thread frame exists");
    fr.is_current_frame = true;
    assert!(
        set.validate().is_err(),
        "two current frames in one thread must fail validation"
    );
}

#[test]
fn lines_projection_covers_frames_snapshots_and_invariants() {
    let set = m5_frame_variable_snapshot_set();
    let lines = m5_frame_variable_snapshot_lines(&set);
    assert!(lines.iter().any(|l| l.contains("Frames:")));
    assert!(lines.iter().any(|l| l.contains("Snapshots:")));
    assert!(lines.iter().any(|l| l.contains("Invariants:")));
    assert!(lines.iter().any(|l| l.contains("exact_link=true")));
    assert!(lines.iter().any(|l| l.contains("exact_link=false")));
    assert!(lines.iter().any(|l| l.contains("disclosure=redacted")));
    assert!(lines.iter().any(|l| l.contains("async_boundary=true")));
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
    assert!(unique(FrameMappingFidelity::ALL.map(|f| f.as_str())));
    assert!(unique(FrameMappingProvenance::ALL.map(|p| p.as_str())));
    assert!(unique(BuildMatchClass::ALL.map(|b| b.as_str())));
    assert!(unique(FrameContinuityClass::ALL.map(|c| c.as_str())));
    assert!(unique(VariableFreshnessState::ALL.map(|v| v.as_str())));
    assert!(unique(ValueDisclosure::ALL.map(|d| d.as_str())));
    assert!(unique(ValueRedactionClass::ALL.map(|r| r.as_str())));
    assert!(unique(VariableUnavailableReason::ALL.map(|r| r.as_str())));
    assert!(unique(VariableScopeClass::ALL.map(|s| s.as_str())));
    assert!(unique(SnapshotEntryKind::ALL.map(|k| k.as_str())));
    assert!(unique(ValueShapeClass::ALL.map(|s| s.as_str())));
}
