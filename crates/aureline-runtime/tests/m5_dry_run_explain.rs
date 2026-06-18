//! Fixture-driven coverage for the M5 dry-run/explain preview object and its
//! first consumers: the checked-in packet matches the seed bit-for-bit, the
//! worked-example preview export round-trips into an equal preview, the survival
//! demonstration proves the preview result lives on through run history and
//! support, the blocked preview keeps its denying gate visible, and every
//! mutation fixture reproduces the fail-closed promotion state the gate enforces.

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_dry_run_explain_first_consumers_input, seeded_blocked_preview,
    seeded_dry_run_explain_consumer_preview, seeded_dry_run_explain_first_consumers_packet,
    AutomationBaselinePromotionState, DryRunExplainConsumerBinding, DryRunExplainExport,
    DryRunExplainFirstConsumersPacket, DryRunExplainPreview, DryRunOutcomeClass,
    DryRunSideEffectClass, PreviewedAction, RecipeBuilderEntrypoint,
    DRY_RUN_EXPLAIN_FIRST_CONSUMERS_PACKET_ARTIFACT_REF,
};
use serde::Deserialize;

const FIXTURE_DIR: &str = "fixtures/automation/m5/side-effect-preview";

/// Each mutation fixture is (file-name, mutation).
const CASES: [(&str, &str); 7] = [
    ("dry_run_explain_stable.json", "none"),
    (
        "missing_entrypoint_blocks_stable.json",
        "missing_entrypoint",
    ),
    (
        "predicted_write_not_declared_blocks_stable.json",
        "predicted_write_not_declared",
    ),
    (
        "mutating_action_mislabeled_read_only_blocks_stable.json",
        "mutating_action_mislabeled_read_only",
    ),
    (
        "outcome_projection_inconsistent_blocks_stable.json",
        "outcome_projection_inconsistent",
    ),
    (
        "safety_label_projection_inconsistent_blocks_stable.json",
        "safety_label_projection_inconsistent",
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
    entrypoint_tokens: Vec<String>,
    is_stable: bool,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> T {
    let body = std::fs::read_to_string(path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn action_mut<'a>(preview: &'a mut DryRunExplainPreview, step_id: &str) -> &'a mut PreviewedAction {
    preview
        .actions
        .iter_mut()
        .find(|action| action.step_id == step_id)
        .expect("action present")
}

fn replace_binding(
    input: &mut aureline_runtime::DryRunExplainFirstConsumersInput,
    entrypoint: RecipeBuilderEntrypoint,
    preview: &DryRunExplainPreview,
) {
    input
        .consumer_bindings
        .retain(|binding| binding.entrypoint != entrypoint);
    input
        .consumer_bindings
        .push(DryRunExplainConsumerBinding::from_preview(preview));
}

fn mutated(mutation: &str) -> DryRunExplainFirstConsumersPacket {
    let mut input = current_dry_run_explain_first_consumers_input();
    match mutation {
        "none" => {}
        "missing_entrypoint" => {
            input
                .consumer_bindings
                .retain(|binding| binding.entrypoint != RecipeBuilderEntrypoint::Package);
        }
        "predicted_write_not_declared" => {
            let mut preview =
                seeded_dry_run_explain_consumer_preview(RecipeBuilderEntrypoint::Notebook);
            action_mut(&mut preview, "step:write-export")
                .predicted_writes
                .clear();
            replace_binding(&mut input, RecipeBuilderEntrypoint::Notebook, &preview);
        }
        "mutating_action_mislabeled_read_only" => {
            let mut preview =
                seeded_dry_run_explain_consumer_preview(RecipeBuilderEntrypoint::Package);
            action_mut(&mut preview, "step:resolve-update").side_effect_class =
                DryRunSideEffectClass::ReadOnlyInspection;
            replace_binding(&mut input, RecipeBuilderEntrypoint::Package, &preview);
        }
        "outcome_projection_inconsistent" => {
            input
                .consumer_bindings
                .iter_mut()
                .find(|binding| binding.entrypoint == RecipeBuilderEntrypoint::RequestApi)
                .expect("request binding")
                .packet_record
                .dry_run_outcome_class = DryRunOutcomeClass::WouldApply;
        }
        "safety_label_projection_inconsistent" => {
            input
                .consumer_bindings
                .iter_mut()
                .find(|binding| binding.entrypoint == RecipeBuilderEntrypoint::Notebook)
                .expect("notebook binding")
                .packet_record
                .aggregate_safety_labels
                .pop();
        }
        "invariant_violated" => {
            input.invariants.predicted_writes_are_explicit_before_apply = false;
        }
        other => panic!("unknown mutation {other}"),
    }
    DryRunExplainFirstConsumersPacket::materialize(input)
}

#[test]
fn checked_in_packet_matches_the_seed() {
    let seeded = seeded_dry_run_explain_first_consumers_packet();
    let artifact: DryRunExplainFirstConsumersPacket =
        read_json(&repo_root().join(DRY_RUN_EXPLAIN_FIRST_CONSUMERS_PACKET_ARTIFACT_REF));
    assert_eq!(
        artifact, seeded,
        "checked-in packet must be bit-for-bit derivable from the seed; regenerate the dump"
    );
    assert!(seeded.is_stable());
}

#[test]
fn every_entrypoint_binds_a_side_effect_bearing_preview() {
    let packet = seeded_dry_run_explain_first_consumers_packet();
    for entrypoint in RecipeBuilderEntrypoint::ALL {
        let binding = packet
            .binding(entrypoint)
            .unwrap_or_else(|| panic!("missing binding for {}", entrypoint.as_str()));
        assert!(!binding.previewed_actions.is_empty());
        for action in &binding.previewed_actions {
            // A predicted write declares what it writes; no action hides as read-only.
            if action.side_effect_class == DryRunSideEffectClass::PredictedWrite {
                assert!(!action.predicted_writes.is_empty());
            }
            assert!(action.side_effect_consistent());
        }
        // The frozen packet projects one step per live action and quotes the
        // recomputed outcome and label union.
        assert_eq!(
            binding.packet_record.step_explanations.len(),
            binding.previewed_actions.len()
        );
        assert_eq!(
            binding.packet_record.dry_run_outcome_class,
            binding.recomputed_outcome()
        );
        assert_eq!(
            binding.packet_record.aggregate_safety_labels,
            binding.recomputed_labels()
        );
    }
}

#[test]
fn worked_example_export_round_trips() {
    let export: DryRunExplainExport = read_json(
        &repo_root()
            .join(FIXTURE_DIR)
            .join("preview_export_roundtrip.json"),
    );
    let imported = export.import();
    let reexported = imported.export(export.export_id.clone(), export.exported_at.clone());
    assert_eq!(
        reexported, export,
        "import then export must reproduce the checked-in export verbatim"
    );
    assert!(export.side_effects_preserved());
}

#[test]
fn preview_survives_history_and_support() {
    #[derive(Debug, Deserialize)]
    struct Survival {
        outcome_preserved: bool,
        digest_preserved: bool,
        side_effects_preserved: bool,
        initial_outcome: String,
        history_outcome: String,
    }
    let demo: Survival = read_json(
        &repo_root()
            .join(FIXTURE_DIR)
            .join("preview_survives_history_and_support.json"),
    );
    assert!(demo.outcome_preserved);
    assert!(demo.digest_preserved);
    assert!(demo.side_effects_preserved);
    assert_eq!(demo.initial_outcome, demo.history_outcome);
}

#[test]
fn blocked_preview_keeps_its_gate_visible() {
    use aureline_runtime::DryRunExplainPacket;
    let packet: DryRunExplainPacket = read_json(
        &repo_root()
            .join(FIXTURE_DIR)
            .join("blocked_preview_packet.json"),
    );
    // The incident remote action is denied at a trust gate, and the preview says so.
    assert_eq!(
        packet.dry_run_outcome_class,
        DryRunOutcomeClass::WouldBeDeniedAtGate
    );
    // The denied outcome matches the seeded preview's derivation.
    assert_eq!(
        packet.dry_run_outcome_class,
        seeded_blocked_preview().dry_run_outcome_class()
    );
}

#[test]
fn mutation_fixtures_reproduce_promotion_states() {
    for (file_name, mutation) in CASES {
        let fixture: CaseFixture = read_json(&repo_root().join(FIXTURE_DIR).join(file_name));
        assert_eq!(fixture.mutation, mutation);
        let packet = mutated(mutation);
        assert_eq!(
            packet.promotion_state.as_str(),
            fixture.expect.promotion_state,
            "{} promotion mismatch",
            fixture.case_name
        );
        assert_eq!(
            packet.validation_findings.len(),
            fixture.expect.validation_finding_count,
            "{} finding count mismatch",
            fixture.case_name
        );
        let kinds: Vec<String> = packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str().to_owned())
            .collect();
        assert_eq!(
            kinds, fixture.expect.expected_finding_kinds,
            "{} finding kinds mismatch",
            fixture.case_name
        );
        assert_eq!(packet.entrypoint_tokens(), fixture.expect.entrypoint_tokens);
        assert_eq!(packet.is_stable(), fixture.expect.is_stable);
        if file_name == "dry_run_explain_stable.json" {
            assert_eq!(
                packet.promotion_state,
                AutomationBaselinePromotionState::Stable
            );
        } else {
            assert_eq!(
                packet.promotion_state,
                AutomationBaselinePromotionState::BlocksStable
            );
        }
    }
}
