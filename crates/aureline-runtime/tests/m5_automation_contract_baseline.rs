//! Fixture-driven coverage for the M5 automation contract baseline: the checked-in
//! packet matches the seed bit-for-bit, the worked-example recipe-macro fixtures
//! deserialize into their frozen Rust types, and every baseline mutation fixture
//! reproduces the fail-closed promotion state and findings the gate enforces.

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_automation_contract_baseline_input, seeded_automation_contract_baseline_packet,
    AutomationContractBaselinePacket, AutomationObjectFamily, AutomationSafetyLabelId,
    DryRunExplainPacket, MacroSession, ParameterReviewSheet, RecipeBuilderSession, SafetyLabelKind,
    AUTOMATION_CONTRACT_BASELINE_PACKET_ARTIFACT_REF,
};
use serde::Deserialize;

const ARTIFACT_DIR: &str = "artifacts/m5/automation/automation-contract-baseline";
const RECIPE_MACRO_FIXTURE_DIR: &str = "fixtures/automation/m5/recipe-macro";
const BASELINE_FIXTURE_DIR: &str = "fixtures/automation/m5/automation-contract-baseline";

/// Each baseline mutation fixture is (file-name, mutation).
const CASES: [(&str, &str); 7] = [
    ("baseline_stable.json", "none"),
    (
        "missing_object_family_blocks_stable.json",
        "missing_object_family",
    ),
    (
        "family_missing_evidence_hook_blocks_stable.json",
        "family_missing_evidence_hook",
    ),
    (
        "family_missing_consumer_surface_blocks_stable.json",
        "family_missing_consumer_surface",
    ),
    (
        "safety_label_set_incomplete_blocks_stable.json",
        "safety_label_set_incomplete",
    ),
    (
        "safety_label_miscategorized_blocks_stable.json",
        "safety_label_miscategorized",
    ),
    (
        "invariant_violated_blocks_stable.json",
        "invariant_violated",
    ),
];

#[derive(Debug, Deserialize)]
struct CaseFixture {
    case_name: String,
    mutation: String,
    expect: CaseExpect,
}

#[derive(Debug, Deserialize)]
struct CaseExpect {
    promotion_state: String,
    validation_finding_count: usize,
    expected_finding_kinds: Vec<String>,
    family_tokens: Vec<String>,
    safety_label_tokens: Vec<String>,
    is_stable: bool,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> T {
    let body = std::fs::read_to_string(path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn mutated(mutation: &str) -> AutomationContractBaselinePacket {
    let mut input = current_automation_contract_baseline_input();
    match mutation {
        "none" => {}
        "missing_object_family" => {
            input
                .object_families
                .retain(|binding| binding.family != AutomationObjectFamily::MacroRecorder);
        }
        "family_missing_evidence_hook" => {
            input
                .object_families
                .iter_mut()
                .find(|binding| binding.family == AutomationObjectFamily::RecipeBuilder)
                .expect("recipe builder present")
                .evidence_hook_refs
                .clear();
        }
        "family_missing_consumer_surface" => {
            input
                .object_families
                .iter_mut()
                .find(|binding| binding.family == AutomationObjectFamily::RunHistory)
                .expect("run history present")
                .consumer_surfaces
                .clear();
        }
        "safety_label_set_incomplete" => {
            input
                .safety_labels
                .retain(|label| label.label_id != AutomationSafetyLabelId::NetworkCall);
        }
        "safety_label_miscategorized" => {
            input
                .safety_labels
                .iter_mut()
                .find(|label| label.label_id == AutomationSafetyLabelId::WritesFiles)
                .expect("writes_files present")
                .label_kind = SafetyLabelKind::AdmissibilityCue;
        }
        "invariant_violated" => {
            input
                .invariants
                .reruns_reresolve_current_context_never_replay_stale_authority = false;
        }
        other => panic!("unknown mutation {other}"),
    }
    AutomationContractBaselinePacket::materialize(input)
}

#[test]
fn checked_in_packet_matches_seed() {
    let on_disk: AutomationContractBaselinePacket =
        read_json(&repo_root().join(AUTOMATION_CONTRACT_BASELINE_PACKET_ARTIFACT_REF));
    let seed = seeded_automation_contract_baseline_packet();
    assert_eq!(
        on_disk, seed,
        "checked-in packet drifted from the seed; rerun the dump example to regenerate"
    );
}

#[test]
fn baseline_mutation_fixtures_match_seed_derived_packets() {
    let root = repo_root();
    for (file_name, mutation) in CASES {
        let fixture: CaseFixture = read_json(&root.join(BASELINE_FIXTURE_DIR).join(file_name));
        assert_eq!(fixture.mutation, mutation);
        let packet = mutated(mutation);

        assert_eq!(
            packet.promotion_state.as_str(),
            fixture.expect.promotion_state,
            "promotion state drift for {}",
            fixture.case_name
        );
        assert_eq!(
            packet.validation_findings.len(),
            fixture.expect.validation_finding_count,
            "finding count drift for {}",
            fixture.case_name
        );
        let kinds: Vec<String> = packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str().to_owned())
            .collect();
        assert_eq!(
            kinds, fixture.expect.expected_finding_kinds,
            "finding kinds drift for {}",
            fixture.case_name
        );
        assert_eq!(
            packet.family_tokens(),
            fixture.expect.family_tokens,
            "family tokens drift for {}",
            fixture.case_name
        );
        assert_eq!(
            packet.safety_label_tokens(),
            fixture.expect.safety_label_tokens,
            "safety label tokens drift for {}",
            fixture.case_name
        );
        assert_eq!(
            packet.is_stable(),
            fixture.expect.is_stable,
            "is_stable drift for {}",
            fixture.case_name
        );
    }
}

#[test]
fn baseline_stable_fixture_promotes_stable() {
    let root = repo_root();
    let fixture: CaseFixture =
        read_json(&root.join(BASELINE_FIXTURE_DIR).join("baseline_stable.json"));
    assert_eq!(fixture.expect.promotion_state, "stable");
    assert!(fixture.expect.is_stable);
    assert!(fixture.expect.expected_finding_kinds.is_empty());

    // Every other case must block stable.
    for (file_name, _) in CASES
        .iter()
        .filter(|(name, _)| *name != "baseline_stable.json")
    {
        let case: CaseFixture = read_json(&root.join(BASELINE_FIXTURE_DIR).join(file_name));
        assert_eq!(
            case.expect.promotion_state, "blocks_stable",
            "{} must block stable",
            file_name
        );
        assert!(!case.expect.is_stable);
    }
}

#[test]
fn recipe_macro_worked_examples_deserialize() {
    let root = repo_root();
    let dir = root.join(RECIPE_MACRO_FIXTURE_DIR);

    let preview_ready: RecipeBuilderSession =
        read_json(&dir.join("recipe_builder_session_preview_ready.json"));
    assert_eq!(preview_ready.record_kind, "recipe_builder_session_record");
    assert_eq!(preview_ready.step_drafts.len(), 2);

    let blocked: RecipeBuilderSession = read_json(&dir.join("recipe_builder_session_blocked.json"));
    assert_eq!(blocked.validation_findings.len(), 1);

    let sheet: ParameterReviewSheet = read_json(&dir.join("parameter_review_sheet.json"));
    assert_eq!(sheet.record_kind, "parameter_review_sheet_record");
    assert_eq!(sheet.unresolved_required_count, 0);

    let dry_run: DryRunExplainPacket = read_json(&dir.join("dry_run_explain_packet.json"));
    assert_eq!(dry_run.record_kind, "dry_run_explain_packet_record");
    assert_eq!(dry_run.step_explanations.len(), 2);

    let macro_promotable: MacroSession =
        read_json(&dir.join("macro_session_stopped_promotable.json"));
    assert_eq!(macro_promotable.record_kind, "macro_session_record");
    assert!(macro_promotable.resulting_macro_manifest_ref.is_some());

    let macro_discarded: MacroSession = read_json(&dir.join("macro_session_discarded.json"));
    assert!(macro_discarded.resulting_macro_manifest_ref.is_none());
}

#[test]
fn checked_in_compact_matches_seed() {
    let root = repo_root();
    let on_disk = std::fs::read_to_string(root.join(ARTIFACT_DIR).join("compact.txt"))
        .expect("read compact.txt");
    let expected = format!(
        "{}\n",
        seeded_automation_contract_baseline_packet()
            .compact_lines()
            .join("\n")
    );
    assert_eq!(on_disk, expected, "compact.txt drifted from the seed");
}
