//! Unit tests for the M5 breakpoint-spec set.

use super::*;

#[test]
fn canonical_set_validates_and_is_export_safe() {
    let set = m5_breakpoint_spec_set();
    set.validate().expect("canonical set validates");
    assert!(set.is_support_export_safe());
    assert!(set.all_invariants_hold());
    assert!(set.raw_payload_excluded);
}

#[test]
fn canonical_set_round_trips_through_serde() {
    let set = m5_breakpoint_spec_set();
    let json = serde_json::to_string(&set).expect("serializes");
    let back: BreakpointSpecSet = serde_json::from_str(&json).expect("round-trips");
    assert_eq!(set, back);
}

#[test]
fn every_verification_and_mapping_state_is_materialized() {
    let set = m5_breakpoint_spec_set();
    for state in BreakpointVerificationState::ALL {
        assert!(
            set.in_verification_state(state).is_some(),
            "missing breakpoint for verification state {}",
            state.as_str()
        );
    }
    for mapping in BreakpointMappingState::ALL {
        assert!(
            set.in_mapping_state(mapping).is_some(),
            "missing breakpoint for mapping state {}",
            mapping.as_str()
        );
    }
}

#[test]
fn every_scope_is_materialized() {
    let set = m5_breakpoint_spec_set();
    for scope in BreakpointScopeClass::ALL {
        assert!(
            set.breakpoints.iter().any(|b| b.scope == scope),
            "missing breakpoint for scope {}",
            scope.as_str()
        );
    }
}

#[test]
fn only_verified_exact_non_replay_shows_clean_confirmed() {
    let set = m5_breakpoint_spec_set();
    for bp in &set.breakpoints {
        let expected = bp.verification().is_bound()
            && bp.mapping().preserves_exact_location()
            && !bp.scope.is_replay_only();
        assert_eq!(
            bp.pill.shows_clean_confirmed, expected,
            "breakpoint {} clean-confirmed flag must equal its derivation",
            bp.breakpoint_id
        );
        if bp.pill.shows_clean_confirmed {
            assert!(!bp.pill.requires_disclosure);
        } else {
            assert!(bp.pill.requires_disclosure);
        }
    }
}

#[test]
fn unbound_misaligned_replay_and_blocked_never_show_green() {
    let set = m5_breakpoint_spec_set();
    for bp in &set.breakpoints {
        let dishonest_green = bp.pill.shows_clean_confirmed
            && (bp.verification() != BreakpointVerificationState::Verified
                || bp.mapping() != BreakpointMappingState::Exact
                || bp.scope.is_replay_only());
        assert!(
            !dishonest_green,
            "breakpoint {} renders a green icon that hides a caveat",
            bp.breakpoint_id
        );
    }
}

#[test]
fn lost_source_identity_degrades_to_needs_remap_not_silence() {
    let set = m5_breakpoint_spec_set();
    let lost: Vec<_> = set
        .breakpoints
        .iter()
        .filter(|b| b.mapping_provenance == BreakpointMappingProvenance::SourceIdentityLost)
        .collect();
    assert!(!lost.is_empty(), "a lost-identity case must exist");
    for bp in lost {
        assert_eq!(
            bp.mapping(),
            BreakpointMappingState::NeedsRemap,
            "lost-identity breakpoint {} must degrade to needs-remap, not vanish",
            bp.breakpoint_id
        );
    }
    // And a needs-remap state only ever comes from a lost source identity.
    for bp in &set.breakpoints {
        if bp.mapping() == BreakpointMappingState::NeedsRemap {
            assert_eq!(
                bp.mapping_provenance,
                BreakpointMappingProvenance::SourceIdentityLost,
                "needs-remap breakpoint {} must trace to a lost source identity",
                bp.breakpoint_id
            );
        }
    }
}

#[test]
fn lexical_fallback_is_never_presented_as_exact() {
    let set = m5_breakpoint_spec_set();
    let fallback: Vec<_> = set
        .breakpoints
        .iter()
        .filter(|b| b.mapping_provenance == BreakpointMappingProvenance::LexicalFallback)
        .collect();
    assert!(!fallback.is_empty(), "a lexical-fallback case must exist");
    for bp in fallback {
        assert_ne!(bp.mapping(), BreakpointMappingState::Exact);
        assert!(!bp.mapping_provenance_is_semantic);
        assert!(bp.pill.requires_disclosure);
    }
}

#[test]
fn notebook_breakpoints_keep_stable_cell_identity() {
    let set = m5_breakpoint_spec_set();
    let notebook: Vec<_> = set
        .breakpoints
        .iter()
        .filter(|b| b.scope == BreakpointScopeClass::NotebookCell)
        .collect();
    assert!(
        !notebook.is_empty(),
        "a notebook-scoped breakpoint must exist"
    );
    for bp in notebook {
        let anchor = bp
            .notebook_anchor
            .as_ref()
            .unwrap_or_else(|| panic!("notebook breakpoint {} has an anchor", bp.breakpoint_id));
        assert!(!anchor.cell_id.is_empty());
        if bp.mapping() != BreakpointMappingState::Exact {
            assert!(
                !bp.pill.shows_clean_confirmed,
                "remapped notebook breakpoint {} must not be drawn as exact",
                bp.breakpoint_id
            );
        }
    }
}

#[test]
fn replay_breakpoints_keep_frame_identity_and_stay_replay_only() {
    let set = m5_breakpoint_spec_set();
    let replay: Vec<_> = set
        .breakpoints
        .iter()
        .filter(|b| b.scope == BreakpointScopeClass::ReplayTimeline)
        .collect();
    assert!(!replay.is_empty(), "a replay-scoped breakpoint must exist");
    for bp in replay {
        let anchor = bp
            .replay_anchor
            .as_ref()
            .unwrap_or_else(|| panic!("replay breakpoint {} has an anchor", bp.breakpoint_id));
        assert!(!anchor.timeline_ref.is_empty());
        assert!(bp.pill.is_replay_only);
        assert!(!bp.pill.shows_clean_confirmed);
    }
}

#[test]
fn pill_label_discloses_caveats() {
    let set = m5_breakpoint_spec_set();
    for bp in &set.breakpoints {
        if bp.mapping() == BreakpointMappingState::Misaligned {
            assert!(bp.pill.label.contains("relocated"));
        }
        if bp.mapping() == BreakpointMappingState::NeedsRemap {
            assert!(bp.pill.label.contains("needs remap"));
        }
        if bp.scope.is_replay_only() {
            assert!(bp.pill.label.contains("replay-only"));
        }
    }
}

#[test]
fn payloads_are_consistent() {
    let set = m5_breakpoint_spec_set();
    for bp in &set.breakpoints {
        assert!(
            bp.payload.is_consistent(),
            "breakpoint {} payload flags disagree with its digests",
            bp.breakpoint_id
        );
    }
}

#[test]
fn tampering_with_a_pill_fails_validation() {
    let mut set = m5_breakpoint_spec_set();
    // Force an unbound breakpoint to claim a clean confirmed stop.
    let bp = set
        .breakpoints
        .iter_mut()
        .find(|b| b.verification() == BreakpointVerificationState::Unbound)
        .expect("an unbound breakpoint exists");
    bp.pill.shows_clean_confirmed = true;
    bp.pill.requires_disclosure = false;
    assert!(
        set.validate().is_err(),
        "an unbound breakpoint claiming a clean confirmed stop must fail validation"
    );
}

#[test]
fn tampering_with_needs_remap_provenance_fails_validation() {
    let mut set = m5_breakpoint_spec_set();
    // Flip a needs-remap breakpoint's provenance away from the lost-identity cause.
    let bp = set
        .breakpoints
        .iter_mut()
        .find(|b| b.mapping() == BreakpointMappingState::NeedsRemap)
        .expect("a needs-remap breakpoint exists");
    bp.mapping_provenance = BreakpointMappingProvenance::StableSourceId;
    bp.mapping_provenance_token = BreakpointMappingProvenance::StableSourceId
        .as_str()
        .to_owned();
    bp.mapping_provenance_is_semantic = true;
    bp.mapping_provenance_requires_disclosure = false;
    assert!(
        set.validate().is_err(),
        "a needs-remap state without a lost source identity must fail validation"
    );
}

#[test]
fn dropping_a_notebook_anchor_fails_validation() {
    let mut set = m5_breakpoint_spec_set();
    let bp = set
        .breakpoints
        .iter_mut()
        .find(|b| b.scope == BreakpointScopeClass::NotebookCell)
        .expect("a notebook breakpoint exists");
    bp.notebook_anchor = None;
    assert!(
        set.validate().is_err(),
        "a notebook breakpoint without a cell anchor must fail validation"
    );
}

#[test]
fn lines_projection_covers_breakpoints_and_invariants() {
    let set = m5_breakpoint_spec_set();
    let lines = m5_breakpoint_spec_lines(&set);
    assert!(lines.iter().any(|l| l.contains("Breakpoints:")));
    assert!(lines.iter().any(|l| l.contains("Invariants:")));
    assert!(lines.iter().any(|l| l.contains("clean_confirmed=true")));
    assert!(lines.iter().any(|l| l.contains("clean_confirmed=false")));
    assert!(lines.iter().any(|l| l.contains("needs_remap=true")));
    assert!(lines.iter().any(|l| l.contains("replay_only=true")));
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
    assert!(unique(BreakpointKindClass::ALL.map(|k| k.as_str())));
    assert!(unique(BreakpointEnablement::ALL.map(|e| e.as_str())));
    assert!(unique(BreakpointVerificationState::ALL.map(|v| v.as_str())));
    assert!(unique(BreakpointMappingState::ALL.map(|m| m.as_str())));
    assert!(unique(BreakpointScopeClass::ALL.map(|s| s.as_str())));
    assert!(unique(BreakpointMappingProvenance::ALL.map(|p| p.as_str())));
}
