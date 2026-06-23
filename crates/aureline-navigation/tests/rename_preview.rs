//! Freeze gate for the rename-preview corpus.
//!
//! The checked-in fixture
//! `fixtures/navigation/governed_rename_preview/canonical_previews.json` is the
//! published corpus. This gate rebuilds the corpus in code and asserts it equals the fixture
//! after a serialize round-trip, so the governed rename-preview contract cannot drift
//! from the published artifact without failing CI. It also re-proves that every stored
//! preview equals the builder's own output, that the corpus is support-export safe,
//! that every preview groups candidates into the editable and held sets, reconciles
//! change-versus-held counts, keeps omitted candidates visible, enforces the
//! inspect-before-mutate apply gate, and that every frozen invariant holds. This test
//! runs under `cargo test --workspace`, so stable promotion cannot harden a rename
//! surface without current proof.

use std::path::{Path, PathBuf};

use aureline_navigation::rename_preview::{
    build_rename_preview, rename_preview_set, RenameApplyPrecondition, RenameCandidateGroupKind,
    RenamePreviewGovernanceSet, RENAME_GROUP_ORDER, RENAME_PREVIEW_RECORD_KIND,
    RENAME_PREVIEW_SCHEMA_REF,
};
use aureline_navigation::target_model::{RelationKind, RenameApplyPosture};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/navigation/governed_rename_preview/canonical_previews.json")
}

fn load_fixture() -> RenamePreviewGovernanceSet {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_corpus_matches_checked_in_fixture() {
    let built = rename_preview_set();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code rename-preview corpus drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-navigation --example dump_rename_preview`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, RENAME_PREVIEW_RECORD_KIND);
    assert_eq!(fixture.schema_ref, RENAME_PREVIEW_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: RenamePreviewGovernanceSet =
        serde_json::from_str(&serde_json::to_string(&fixture).expect("serializes"))
            .expect("round-trips");
    assert_eq!(roundtrip, fixture);
}

#[test]
fn every_frozen_invariant_holds() {
    let fixture = load_fixture();
    assert!(!fixture.invariants.is_empty());
    for invariant in &fixture.invariants {
        assert!(
            invariant.holds,
            "frozen invariant must hold: {}",
            invariant.invariant_id
        );
    }
    assert!(fixture.all_invariants_hold());
}

#[test]
fn every_stored_preview_equals_builder_output() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let produced = build_rename_preview(&scenario.input);
        assert_eq!(
            produced, scenario.preview,
            "scenario {} drifted from the builder",
            scenario.scenario_id
        );
        assert_eq!(produced.root_relation_kind, RelationKind::Definition);
    }
}

#[test]
fn every_preview_groups_candidates_disjointly_in_order() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let preview = &scenario.preview;
        // Every candidate lands in exactly the group its precedence selects.
        for candidate in &scenario.input.candidates {
            let kind = candidate.group_kind();
            let group = preview
                .group(kind)
                .unwrap_or_else(|| panic!("group missing for {}", kind.as_str()));
            assert!(
                group.candidate_refs.contains(&candidate.candidate_id),
                "candidate {} not grouped under its precedence",
                candidate.candidate_id
            );
        }
        // Groups stay in canonical order.
        let order = |group_kind| {
            RENAME_GROUP_ORDER
                .iter()
                .position(|candidate| *candidate == group_kind)
                .unwrap()
        };
        for pair in preview.groups.windows(2) {
            assert!(order(pair[0].group_kind) < order(pair[1].group_kind));
        }
        // No candidate disappears.
        let grouped: usize = preview.groups.iter().map(|g| g.candidate_refs.len()).sum();
        assert_eq!(grouped, scenario.input.candidates.len());
    }
}

#[test]
fn change_versus_held_counts_reconcile() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let preview = &scenario.preview;
        assert!(preview.totals.reconciles());
        assert_eq!(
            preview.totals.will_change_count + preview.totals.held_count(),
            preview.totals.total_count
        );
        let grouped: usize = preview.groups.iter().map(|g| g.counts.total_count).sum();
        assert_eq!(grouped, preview.totals.total_count);
        for group in &preview.groups {
            assert!(group.counts.reconciles());
        }
    }
}

#[test]
fn omitted_candidates_remain_visible_with_reasons() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let preview = &scenario.preview;
        for candidate in &scenario.input.candidates {
            let kind = candidate.group_kind();
            if kind == RenameCandidateGroupKind::Editable {
                continue;
            }
            let group = preview.group(kind).expect("held group present");
            assert!(
                group.candidate_refs.contains(&candidate.candidate_id),
                "held candidate {} disappeared",
                candidate.candidate_id
            );
            assert!(
                !group.omission_reasons.is_empty(),
                "held group {} carries no omission reason",
                kind.as_str()
            );
        }
    }
}

#[test]
fn inspect_before_mutate_is_always_enforced() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let gate = &scenario.preview.apply_gate;
        assert!(gate.inspect_before_mutate_required);
        assert!(gate.blind_apply_blocked);
        assert!(gate.omitted_candidates_remain_visible);
        assert!(gate.redacted_candidates_remain_visible);
        assert!(!gate.undo_checkpoint_ref.is_empty());
        // Apply is allowed only when ready-after-preview.
        assert_eq!(
            gate.apply_allowed_after_preview,
            gate.apply_posture == RenameApplyPosture::ReadyForApplyAfterPreview
        );
        // Only the editable group mutates.
        for group in &scenario.preview.groups {
            assert_eq!(
                group.mutates_on_apply,
                group.group_kind == RenameCandidateGroupKind::Editable
            );
        }
    }
}

#[test]
fn apply_posture_never_claims_safe_while_blocking() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let preview = &scenario.preview;
        let blocking = preview.totals.blocked_count > 0 || preview.totals.conflict_count > 0;
        if blocking {
            assert!(
                preview.apply_gate.apply_posture.blocks_apply(),
                "scenario {} has blocking candidates but a non-blocking posture",
                scenario.scenario_id
            );
            assert!(!preview.apply_gate.apply_allowed_after_preview);
        }
    }
}

#[test]
fn preview_set_projection_matches_governed_preview() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let preview = &scenario.preview;
        assert_eq!(
            preview.preview_set.apply_posture,
            preview.apply_gate.apply_posture
        );
        assert_eq!(
            preview.preview_set.count_summary.changed_count,
            preview.totals.will_change_count
        );
        assert_eq!(
            preview.preview_set.blocked_refs.len(),
            preview.totals.held_count()
        );
    }
}

#[test]
fn corpus_covers_postures_and_preconditions() {
    let fixture = load_fixture();
    let postures = [
        RenameApplyPosture::ReadyForApplyAfterPreview,
        RenameApplyPosture::BlockedPendingScopeReview,
        RenameApplyPosture::BlockedPendingRefresh,
        RenameApplyPosture::BlockedPendingPolicyOrProtectedReview,
        RenameApplyPosture::InspectOnlyUnavailable,
    ];
    for posture in postures {
        assert!(
            fixture
                .scenarios
                .iter()
                .any(|s| s.preview.apply_gate.apply_posture == posture),
            "no scenario covers posture {posture:?}"
        );
    }
    for precondition in RenameApplyPrecondition::ALL {
        assert!(
            fixture.scenarios.iter().any(|s| s
                .preview
                .apply_gate
                .preconditions
                .contains(&precondition)),
            "no scenario covers precondition {}",
            precondition.as_str()
        );
    }
}
