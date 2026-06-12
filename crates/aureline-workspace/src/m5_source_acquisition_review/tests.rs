use super::*;

fn packet() -> M5SourceAcquisitionReviewPacket {
    current_m5_source_acquisition_review_packet().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_SOURCE_ACQUISITION_REVIEW_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, M5_SOURCE_ACQUISITION_REVIEW_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_matches_recomputed() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_verb_is_exercised_and_stays_distinct() {
    // Clone, open, import, and resume remain distinct verbs, and each appears.
    let packet = packet();
    for verb in EntryVerb::ALL {
        assert!(
            packet.sheets_with_verb(verb).next().is_some(),
            "verb {} is not exercised by any sheet",
            verb.as_str()
        );
    }
    for sheet in &packet.sheets {
        assert!(
            sheet.verb_is_canonical(),
            "sheet {} records a verb that is not canonical for its source kind",
            sheet.sheet_id
        );
    }
}

#[test]
fn clone_is_never_rewritten_to_open_on_existing_local_copy() {
    // The guardrail: a clone with a local copy already present stays a clone.
    let packet = packet();
    let shallow = packet
        .sheet("m5-acq:remote-shallow-clone")
        .expect("shallow clone sheet present");
    assert_eq!(shallow.entry_verb, EntryVerb::Clone);
    assert!(shallow.local_copy_present);
    assert!(shallow.verb_locked);
    assert!(shallow.verb_is_canonical());
}

#[test]
fn import_is_never_rewritten_to_resume_on_resumable_bundle() {
    // The guardrail: a handoff bundle that looks resumable stays an import.
    let packet = packet();
    let handoff = packet
        .sheet("m5-acq:companion-handoff-import")
        .expect("companion handoff sheet present");
    assert_eq!(handoff.entry_verb, EntryVerb::Import);
    assert_eq!(handoff.source_kind, SourceKind::HandoffBundle);
    assert!(handoff.verb_locked);
}

#[test]
fn no_follow_up_ever_runs_implicitly() {
    let packet = packet();
    for sheet in &packet.sheets {
        assert!(
            !sheet.has_implicit_follow_up(),
            "sheet {} previews a follow-up that runs implicitly",
            sheet.sheet_id
        );
        for item in &sheet.follow_up_queue {
            assert!(!item.runs_implicitly, "item must not run implicitly");
        }
    }
}

#[test]
fn every_topology_cue_kind_is_exercised() {
    let packet = packet();
    for kind in TopologyCueKind::ALL {
        let exercised = packet
            .sheets
            .iter()
            .flat_map(|s| s.topology_cues.iter())
            .any(|cue| cue.cue_kind == kind);
        assert!(exercised, "topology cue {} is not exercised", kind.as_str());
    }
}

#[test]
fn every_follow_up_kind_is_exercised() {
    let packet = packet();
    for kind in FollowUpKind::ALL {
        let exercised = packet
            .sheets
            .iter()
            .flat_map(|s| s.follow_up_queue.iter())
            .any(|item| item.item_kind == kind);
        assert!(exercised, "follow-up {} is not exercised", kind.as_str());
    }
}

#[test]
fn applicable_cues_offer_a_recovery_and_absent_cues_do_not() {
    let packet = packet();
    for sheet in &packet.sheets {
        for cue in &sheet.topology_cues {
            if cue.state.applies() {
                assert!(
                    cue.recovery_action.is_offered(),
                    "applicable cue {} on {} must offer a recovery",
                    cue.cue_kind.as_str(),
                    sheet.sheet_id
                );
            } else {
                assert!(
                    !cue.recovery_action.is_offered(),
                    "absent cue {} on {} must not offer a recovery",
                    cue.cue_kind.as_str(),
                    sheet.sheet_id
                );
            }
        }
    }
}

#[test]
fn blocking_cues_remain_recoverable() {
    // A cue that blocks first-useful-work is never a dead end.
    let packet = packet();
    for sheet in &packet.sheets {
        for cue in sheet.blocking_cues() {
            assert!(
                cue.recoverable && cue.recovery_action.is_offered(),
                "blocking cue {} on {} must be recoverable",
                cue.cue_kind.as_str(),
                sheet.sheet_id
            );
        }
    }
}

#[test]
fn every_sheet_carries_provenance_for_diagnostics() {
    let packet = packet();
    for sheet in &packet.sheets {
        assert!(
            sheet.provenance_complete(),
            "sheet {} lacks source-locator or checkout-plan provenance",
            sheet.sheet_id
        );
    }
}

#[test]
fn review_requirement_matches_gate_and_local_open_is_safe() {
    let packet = packet();
    for sheet in &packet.sheets {
        assert_eq!(
            sheet.review_required_before_acquisition,
            sheet.computed_review_required(),
            "sheet {} review requirement diverges from the gate",
            sheet.sheet_id
        );
    }
    // The clean local open requires no review — the sheet is not a blanket gate.
    let local = packet
        .sheet("m5-acq:local-folder-open")
        .expect("local open sheet present");
    assert!(!local.review_required_before_acquisition);
    assert_eq!(local.expected_cost_band, CostBand::LocalNoFetch);
    assert!(!local.has_applicable_cue());
    // Every network-fetch or non-open sheet requires review before acquisition.
    for sheet in &packet.sheets {
        if sheet.entry_verb != EntryVerb::Open || sheet.expected_cost_band.implies_network_fetch() {
            assert!(
                sheet.review_required_before_acquisition,
                "sheet {} should require review before acquisition",
                sheet.sheet_id
            );
        }
    }
}

#[test]
fn export_projection_reflects_sheets() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.sheets.len(), packet.sheets.len());
    assert!(projection.all_sheets_consistent);
    assert_eq!(
        projection.sheets_requiring_review,
        packet.summary.sheets_requiring_review
    );
    for (row, sheet) in projection.sheets.iter().zip(&packet.sheets) {
        assert_eq!(row.sheet_id, sheet.sheet_id);
        assert_eq!(row.entry_verb, sheet.entry_verb);
        assert!(row.verb_canonical_and_locked);
    }
}

#[test]
fn all_sheets_consistent_holds() {
    assert!(packet().all_sheets_consistent());
}

#[test]
fn validate_flags_a_silently_rewritten_verb() {
    let mut packet = packet();
    // Rewrite a clone's verb to open without changing the remote source kind.
    let sheet = packet
        .sheets
        .iter_mut()
        .find(|s| s.entry_verb == EntryVerb::Clone)
        .expect("a clone sheet exists");
    sheet.entry_verb = EntryVerb::Open;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5SourceAcquisitionReviewViolation::VerbNotCanonical { .. }
    )));
}

#[test]
fn validate_flags_an_implicit_follow_up() {
    let mut packet = packet();
    let sheet = packet
        .sheets
        .iter_mut()
        .find(|s| !s.follow_up_queue.is_empty())
        .expect("a sheet with follow-ups exists");
    sheet.follow_up_queue[0].runs_implicitly = true;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5SourceAcquisitionReviewViolation::ImplicitFollowUp { .. }
    )));
}

#[test]
fn validate_flags_a_review_requirement_drift() {
    let mut packet = packet();
    let sheet = packet
        .sheet("m5-acq:local-folder-open")
        .cloned()
        .expect("local open sheet present");
    let idx = packet
        .sheets
        .iter()
        .position(|s| s.sheet_id == sheet.sheet_id)
        .unwrap();
    packet.sheets[idx].review_required_before_acquisition = true;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5SourceAcquisitionReviewViolation::ReviewRequirementMismatch { .. }
    )));
}

#[test]
fn constants_point_at_checked_in_paths() {
    assert_eq!(
        M5_SOURCE_ACQUISITION_REVIEW_PATH,
        "artifacts/workspace/m5/m5-source-acquisition-review.json"
    );
    assert_eq!(
        M5_SOURCE_ACQUISITION_REVIEW_SCHEMA_REF,
        "schemas/workspace/m5-source-acquisition-review.schema.json"
    );
    assert_eq!(
        M5_SOURCE_ACQUISITION_REVIEW_DOC_REF,
        "docs/workspace/m5/m5-source-acquisition-review.md"
    );
}
