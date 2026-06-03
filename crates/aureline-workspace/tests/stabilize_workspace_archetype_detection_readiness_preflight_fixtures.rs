//! Replay gate for the workspace archetype readiness preflight record.
//!
//! Each fixture under
//! `fixtures/ux/m4/stabilize-workspace-archetype-detection-readiness-preflight/`
//! carries a checked-in `WorkspaceArchetypeReadinessPreflightRecord`. This gate
//! re-projects the corpus, asserts every fixture matches the live builder, and
//! validates the M04-190 invariants.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    workspace_archetype_readiness_preflight_corpus, ContinueWithoutClass, DetectionOutcome,
    MixedWorkspaceBoundaryChoice, RouteSwitchOption, WorkspaceArchetypeReadinessPreflightRecord,
    WORKSPACE_ARCHETYPE_READINESS_PREFLIGHT_RECORD_KIND,
    WORKSPACE_ARCHETYPE_READINESS_PREFLIGHT_SCHEMA_VERSION,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PreflightFixture {
    #[serde(flatten)]
    record: WorkspaceArchetypeReadinessPreflightRecord,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/ux/m4/stabilize-workspace-archetype-detection-readiness-preflight")
}

fn load_fixtures() -> Vec<(String, PreflightFixture)> {
    let dir = fixtures_dir();
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: PreflightFixture = serde_json::from_str(&raw)
            .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()));
        out.push((path.display().to_string(), fixture));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    assert!(!out.is_empty(), "expected at least one preflight fixture");
    out
}

#[test]
fn corpus_replays_each_fixture_exactly() {
    let corpus = workspace_archetype_readiness_preflight_corpus();
    let fixtures = load_fixtures();

    assert_eq!(
        corpus.len(),
        fixtures.len(),
        "corpus and fixture count must match"
    );

    let mut corpus_by_id: std::collections::HashMap<String, _> =
        std::collections::HashMap::with_capacity(corpus.len());
    for scenario in &corpus {
        corpus_by_id.insert(scenario.scenario_id.to_string(), scenario.record());
    }

    for (path, fixture) in &fixtures {
        let record_id = &fixture.record.record_id;
        let projected = corpus_by_id
            .get(record_id)
            .unwrap_or_else(|| panic!("corpus missing scenario {record_id}"));
        assert_eq!(
            *projected, fixture.record,
            "projection drifted from checked-in record for fixture {path}"
        );

        // Re-serializing and re-projecting must be idempotent.
        let roundtrip: WorkspaceArchetypeReadinessPreflightRecord =
            serde_json::from_str(&serde_json::to_string(projected).expect("record serializes"))
                .expect("record round-trips");
        assert_eq!(roundtrip, *projected, "record must round-trip for {path}");
    }
}

#[test]
fn every_fixture_is_well_formed_and_contract_valid() {
    for (name, fixture) in load_fixtures() {
        let record = &fixture.record;
        assert_eq!(
            record.record_kind, WORKSPACE_ARCHETYPE_READINESS_PREFLIGHT_RECORD_KIND,
            "fixture {name} must carry correct record kind"
        );
        assert_eq!(
            record.schema_version, WORKSPACE_ARCHETYPE_READINESS_PREFLIGHT_SCHEMA_VERSION,
            "fixture {name} must carry correct schema version"
        );
        assert!(
            record.is_contract_valid(),
            "fixture {name} must be contract valid: {:?}",
            record.contract_findings()
        );
        assert!(
            !record.auto_install_allowed,
            "fixture {name} must not allow auto-install"
        );
        assert!(
            !record.auto_trust_allowed,
            "fixture {name} must not allow auto-trust"
        );
        assert!(
            !record.hidden_setup_executed,
            "fixture {name} must not have hidden setup"
        );
        assert!(
            !record.trust_widened,
            "fixture {name} must not have widened trust"
        );
    }
}

#[test]
fn corpus_covers_all_required_detection_outcomes() {
    let fixtures = load_fixtures();
    let outcomes: Vec<DetectionOutcome> = fixtures
        .iter()
        .map(|(_, f)| f.record.detection_outcome)
        .collect();
    for required in [
        DetectionOutcome::CertifiedArchetypeMatch,
        DetectionOutcome::ProbableArchetype,
        DetectionOutcome::MixedOrAmbiguousWorkspace,
        DetectionOutcome::UnknownOrGenericWorkspace,
        DetectionOutcome::RestrictedOrPolicyBlocked,
        DetectionOutcome::MissingPrerequisite,
    ] {
        assert!(
            outcomes.contains(&required),
            "corpus must include detection outcome {required:?}"
        );
    }
}

#[test]
fn certified_and_probable_fixtures_carry_evidence_freshness() {
    for (name, fixture) in load_fixtures() {
        let record = &fixture.record;
        if matches!(
            record.detection_outcome,
            DetectionOutcome::CertifiedArchetypeMatch | DetectionOutcome::ProbableArchetype
        ) {
            assert!(
                !record.evidence_freshness.is_empty(),
                "fixture {name} must carry evidence freshness"
            );
        }
    }
}

#[test]
fn mixed_workspace_fixtures_have_all_boundary_choices() {
    for (name, fixture) in load_fixtures() {
        let record = &fixture.record;
        if record.detection_outcome == DetectionOutcome::MixedOrAmbiguousWorkspace {
            for required in [
                MixedWorkspaceBoundaryChoice::OpenWholeRepo,
                MixedWorkspaceBoundaryChoice::OpenProbableProject,
                MixedWorkspaceBoundaryChoice::OpenCurrentFolderOnly,
                MixedWorkspaceBoundaryChoice::CreateWorksetOrSlice,
            ] {
                assert!(
                    record.boundary_choices.contains(&required),
                    "fixture {name} must include boundary choice {}",
                    required.as_str()
                );
            }
        }
    }
}

#[test]
fn restricted_and_missing_prerequisite_offer_open_minimal() {
    for (name, fixture) in load_fixtures() {
        let record = &fixture.record;
        if matches!(
            record.detection_outcome,
            DetectionOutcome::RestrictedOrPolicyBlocked | DetectionOutcome::MissingPrerequisite
        ) {
            assert!(
                record
                    .switch_options
                    .contains(&RouteSwitchOption::OpenMinimal),
                "fixture {name} must offer OpenMinimal"
            );
        }
    }
}

#[test]
fn readiness_tasks_carry_source_signal_refs() {
    for (name, fixture) in load_fixtures() {
        let record = &fixture.record;
        for task in record.readiness_buckets.all_tasks() {
            assert!(
                !task.source_signal_refs.is_empty(),
                "fixture {name}: task {} must have source_signal_refs",
                task.task_ref
            );
        }
    }
}

#[test]
fn same_weight_bypasses_present_when_setup_recommended() {
    for (name, fixture) in load_fixtures() {
        let record = &fixture.record;
        if record.readiness_buckets.has_any_task() || !record.recommendation_refs.is_empty() {
            for required in [
                ContinueWithoutClass::SetUpLater,
                ContinueWithoutClass::OpenMinimal,
                ContinueWithoutClass::DismissRecommendation,
            ] {
                assert!(
                    record.same_weight_bypass_actions.contains(&required),
                    "fixture {name} must include same-weight bypass {}",
                    required.as_str()
                );
            }
        }
    }
}

#[test]
fn support_export_lines_render_all_sections() {
    for (name, fixture) in load_fixtures() {
        let record = &fixture.record;
        let lines = record.support_export_lines();
        assert!(
            lines
                .iter()
                .any(|line| line.starts_with("workspace_archetype_readiness_preflight:")),
            "fixture {name} must render header"
        );
        assert!(
            lines.iter().any(|line| line.starts_with("detection:")),
            "fixture {name} must render detection"
        );
        assert!(
            lines.iter().any(|line| line.starts_with("readiness:")),
            "fixture {name} must render readiness"
        );
        assert!(
            lines.iter().any(|line| line.starts_with("route:")),
            "fixture {name} must render route"
        );
        assert!(
            lines.iter().any(|line| line.starts_with("safety:")),
            "fixture {name} must render safety"
        );
    }
}
