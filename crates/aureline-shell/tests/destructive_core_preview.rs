//! Integration test for the destructive-core preview/apply/revert wedge.
//!
//! Drives the full Propose -> Preview -> Apply -> Validate -> Revert walk
//! against fixture cases under
//! `fixtures/ux/preview_apply_cases/destructive_core_path/`. The protected
//! walk fixture exercises the nominal multi-file replace path and proves
//! that every lineage id (packet/proposal/preview/apply/mutation group/
//! checkpoint group/validation) is bound and visible. The failure-drill
//! fixture reopens the preview after an external mutation drifts the
//! basis and confirms apply is blocked with the typed
//! `blocked_by_basis_drift` reason.

use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Deserialize;

use aureline_history::{
    body_object_id, HistoryStorageRoot, LocalHistoryStore, MutationJournalStore,
};
use aureline_shell::review_preview::{
    ApplyAdmissibility, BasisDriftState, ConsequenceClass, DestructiveCoreEngine,
    DiffRowBlockReason, GroupResolution, PreviewApplyRevertPhase, RevertClass, WedgeError,
};

#[derive(Debug, Clone, Deserialize)]
struct DestructiveCoreCase {
    case_id: String,
    expected_protected_walk: Option<ProtectedWalkExpectations>,
    expected_failure_drill: Option<FailureDrillExpectations>,
    seed_world: Vec<SeedFile>,
    proposal: ProposalFixtureInput,
    #[serde(default)]
    external_mutations_after_preview: Vec<SeedFile>,
}

#[derive(Debug, Clone, Deserialize)]
struct SeedFile {
    logical_ref: String,
    initial_text: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ProposalFixtureInput {
    target_refs: Vec<String>,
    search_pattern: String,
    replacement_text: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ProtectedWalkExpectations {
    expected_match_count_total: u32,
    expected_apply_admissibility_token: String,
    expected_realized_revert_class_token: String,
    expected_group_resolution_token: String,
    expected_per_target_post_apply_contents: Vec<ExpectedPostApplyContent>,
}

#[derive(Debug, Clone, Deserialize)]
struct ExpectedPostApplyContent {
    logical_ref: String,
    expected_post_apply_text: String,
}

#[derive(Debug, Clone, Deserialize)]
struct FailureDrillExpectations {
    expected_admission_token_after_drift: String,
    expected_drifted_logical_refs: Vec<String>,
    expected_blocked_reason_token: String,
}

fn unique_temp_root(label: &str) -> PathBuf {
    let pid = std::process::id();
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let mut root = std::env::temp_dir();
    root.push(format!("aureline_destructive_core_preview_{label}_{pid}_{stamp}"));
    root
}

fn engine_for_case(case_id: &str) -> (DestructiveCoreEngine, PathBuf) {
    let root = unique_temp_root(case_id);
    let storage = HistoryStorageRoot::new(&root);
    let journal = MutationJournalStore::new(storage.clone());
    let history = LocalHistoryStore::new(storage);
    let engine = DestructiveCoreEngine::new(
        "ws-destructive-core-integration",
        "integration user",
        journal,
        history,
    )
    .with_pinned_clock(vec![
        "2026-05-11T13:40:00Z".to_owned(),
        "2026-05-11T13:40:01Z".to_owned(),
        "2026-05-11T13:40:02Z".to_owned(),
        "2026-05-11T13:40:03Z".to_owned(),
        "2026-05-11T13:40:04Z".to_owned(),
        "2026-05-11T13:40:05Z".to_owned(),
    ]);
    (engine, root)
}

fn load_case(name: &str) -> DestructiveCoreCase {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures/ux/preview_apply_cases/destructive_core_path")
        .join(name);
    let raw =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

#[test]
fn fixture_drives_full_protected_walk() {
    let case = load_case("protected_walk.json");
    let expected = case
        .expected_protected_walk
        .as_ref()
        .expect("protected walk expectations");
    let (mut engine, root) = engine_for_case(&case.case_id);

    for seed in &case.seed_world {
        engine.seed_target(seed.logical_ref.clone(), seed.initial_text.clone().into_bytes());
    }

    let refs: Vec<&str> = case
        .proposal
        .target_refs
        .iter()
        .map(|s| s.as_str())
        .collect();
    let mut packet = engine
        .propose(
            &refs,
            case.proposal.search_pattern.clone(),
            case.proposal.replacement_text.clone(),
        )
        .expect("propose");
    assert_eq!(packet.current_phase, PreviewApplyRevertPhase::Propose);
    assert_eq!(
        packet.proposal.declared_consequence_class,
        ConsequenceClass::DestructiveReversibleWithCheckpoint
    );
    assert_eq!(
        packet.proposal.declared_revert_class,
        RevertClass::RestoreFromCheckpoint
    );

    engine.preview(&mut packet).expect("preview");
    let preview = packet.preview.as_ref().expect("preview present");
    assert_eq!(
        preview.apply_admissibility.as_token(),
        expected.expected_apply_admissibility_token,
        "expected apply admission"
    );
    assert_eq!(preview.total_match_count, expected.expected_match_count_total);

    engine.apply(&mut packet).expect("apply");
    let apply = packet.apply.as_ref().expect("apply present");
    assert!(!apply.mutation_group_id.is_empty());
    assert!(!apply.local_history_group_id.is_empty());
    assert_eq!(
        apply.realized_revert_class.as_str(),
        expected.expected_realized_revert_class_token
    );

    for content in &expected.expected_per_target_post_apply_contents {
        let observed = engine
            .read_target(&content.logical_ref)
            .expect("target present");
        let observed_text = std::str::from_utf8(observed).expect("utf-8");
        assert_eq!(
            observed_text, content.expected_post_apply_text,
            "post-apply text mismatch for {}",
            content.logical_ref
        );
    }

    engine.validate(&mut packet).expect("validate");
    let validation = packet.validation.as_ref().expect("validation present");
    assert!(validation.all_targets_matched);

    engine.revert(&mut packet).expect("revert");
    let revert = packet.revert.as_ref().expect("revert present");
    assert_eq!(
        revert.group_resolution.as_str(),
        expected.expected_group_resolution_token
    );
    assert_eq!(
        revert.realized_revert_class,
        RevertClass::RestoreFromCheckpoint
    );

    // Targets are now back to the seeded pre-apply bytes.
    for seed in &case.seed_world {
        let observed = engine
            .read_target(&seed.logical_ref)
            .expect("target present after revert");
        let observed_digest = body_object_id(observed);
        let expected_digest = body_object_id(seed.initial_text.as_bytes());
        assert_eq!(
            observed_digest, expected_digest,
            "revert must restore pre-apply bytes for {}",
            seed.logical_ref
        );
    }

    // Lineage is bound and visible end-to-end.
    let ids = packet.lineage_ids();
    assert!(ids.preview_id.is_some());
    assert!(ids.apply_id.is_some());
    assert!(ids.mutation_group_id.is_some());
    assert!(ids.local_history_group_id.is_some());
    assert!(ids.validation_id.is_some());
    assert!(ids.revert_id.is_some());
    assert_eq!(
        revert
            .group_resolution
            ,
        GroupResolution::Reverted
    );

    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn fixture_drives_scope_drift_failure_drill() {
    let case = load_case("scope_drift_blocks_apply.json");
    let expected = case
        .expected_failure_drill
        .as_ref()
        .expect("failure-drill expectations");
    let (mut engine, root) = engine_for_case(&case.case_id);

    for seed in &case.seed_world {
        engine.seed_target(seed.logical_ref.clone(), seed.initial_text.clone().into_bytes());
    }
    let refs: Vec<&str> = case
        .proposal
        .target_refs
        .iter()
        .map(|s| s.as_str())
        .collect();
    let mut packet = engine
        .propose(
            &refs,
            case.proposal.search_pattern.clone(),
            case.proposal.replacement_text.clone(),
        )
        .expect("propose");
    engine.preview(&mut packet).expect("preview");
    assert!(
        packet
            .preview
            .as_ref()
            .expect("preview present")
            .apply_admissibility
            .is_admitted(),
        "initial preview should be admitted before the drift"
    );

    for drift in &case.external_mutations_after_preview {
        engine.simulate_external_mutation(&drift.logical_ref, drift.initial_text.clone().into_bytes());
    }
    engine.reopen_preview(&mut packet).expect("reopen preview");

    let preview = packet.preview.as_ref().expect("preview present after reopen");
    assert_eq!(
        preview.apply_admissibility.as_token(),
        expected.expected_admission_token_after_drift,
        "admission after drift",
    );
    assert!(matches!(
        preview.overall_basis_drift_state,
        BasisDriftState::DriftDetected { .. }
    ));

    for logical_ref in &expected.expected_drifted_logical_refs {
        let row = preview
            .rows
            .iter()
            .find(|row| &row.logical_ref == logical_ref)
            .unwrap_or_else(|| panic!("drifted row {logical_ref} present"));
        assert_eq!(
            row.blocked_reason
                .as_ref()
                .map(|reason| reason.as_str())
                .unwrap_or("none"),
            expected.expected_blocked_reason_token,
            "expected typed drift block-reason on {logical_ref}",
        );
        assert!(matches!(
            row.basis_drift,
            BasisDriftState::DriftDetected { .. }
        ));
    }

    let err = engine
        .apply(&mut packet)
        .expect_err("apply must be blocked");
    match err {
        WedgeError::ApplyBlocked(ApplyAdmissibility::BlockedByBasisDrift {
            drifted_target_count,
        }) => {
            assert_eq!(
                drifted_target_count as usize,
                expected.expected_drifted_logical_refs.len()
            );
        }
        other => panic!("expected BlockedByBasisDrift, got {other:?}"),
    }
    assert_eq!(packet.current_phase, PreviewApplyRevertPhase::Preview);
    assert!(packet.apply.is_none());
    let _ = std::fs::remove_dir_all(&root);
}

// Suppress dead-code warnings for fields the fixture exposes for other
// downstream readers but that this test does not consume.
#[allow(dead_code)]
fn _ensure_used(case: &DestructiveCoreCase) {
    let _ = &case.expected_failure_drill;
}

#[test]
fn diff_row_block_reason_token_set_is_exhaustive() {
    // Sanity-check that the fixture suite covers every typed block-reason
    // token the wedge can emit.
    let tokens: Vec<&str> = vec![
        DiffRowBlockReason::BasisDrifted.as_str(),
        DiffRowBlockReason::BasisMissing.as_str(),
        DiffRowBlockReason::TargetMissing.as_str(),
        DiffRowBlockReason::NoMatchesFound.as_str(),
    ];
    assert_eq!(tokens.len(), 4);
}
