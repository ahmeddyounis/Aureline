//! Fixture-driven coverage for the M5 recipe-builder object and its first
//! consumers: the checked-in packet matches the seed bit-for-bit, the
//! worked-example builder export round-trips into an equal builder, the reorder
//! demonstration proves drag and keyboard converge, and every mutation fixture
//! reproduces the fail-closed promotion state the gate enforces.

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_recipe_builder_first_consumers_input, seeded_blocked_recipe_builder,
    seeded_recipe_builder_first_consumers_packet, AutomationBaselinePromotionState,
    RecipeBuilderConsumerBinding, RecipeBuilderEntrypoint, RecipeBuilderExport,
    RecipeBuilderFirstConsumersPacket, RecipeBuilderStateClass,
    RECIPE_BUILDER_FIRST_CONSUMERS_PACKET_ARTIFACT_REF,
};
use serde::Deserialize;

const FIXTURE_DIR: &str = "fixtures/automation/m5/recipe-builder";

/// Each mutation fixture is (file-name, mutation).
const CASES: [(&str, &str); 6] = [
    ("first_consumers_stable.json", "none"),
    (
        "missing_entrypoint_blocks_stable.json",
        "missing_entrypoint",
    ),
    (
        "non_declarative_manifest_blocks_stable.json",
        "non_declarative_manifest",
    ),
    (
        "ui_only_step_not_blocked_blocks_stable.json",
        "ui_only_step_not_blocked",
    ),
    (
        "cli_docs_parity_broken_blocks_stable.json",
        "cli_docs_parity_broken",
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

fn mutated(mutation: &str) -> RecipeBuilderFirstConsumersPacket {
    let mut input = current_recipe_builder_first_consumers_input();
    match mutation {
        "none" => {}
        "missing_entrypoint" => {
            input
                .consumer_bindings
                .retain(|binding| binding.entrypoint != RecipeBuilderEntrypoint::Package);
        }
        "non_declarative_manifest" => {
            input
                .consumer_bindings
                .iter_mut()
                .find(|binding| binding.entrypoint == RecipeBuilderEntrypoint::Notebook)
                .expect("notebook binding")
                .session_record
                .manifest_target_schema_ref =
                "schemas/automation/shell_script.schema.json".to_owned();
        }
        "ui_only_step_not_blocked" => {
            let mut forged =
                RecipeBuilderConsumerBinding::from_builder(&seeded_blocked_recipe_builder());
            forged.builder_state_class = RecipeBuilderStateClass::PreviewReady;
            forged.session_record.builder_state_class = RecipeBuilderStateClass::PreviewReady;
            input
                .consumer_bindings
                .retain(|binding| binding.entrypoint != RecipeBuilderEntrypoint::TaskTestDebug);
            input.consumer_bindings.push(forged);
        }
        "cli_docs_parity_broken" => {
            input
                .consumer_bindings
                .iter_mut()
                .find(|binding| binding.entrypoint == RecipeBuilderEntrypoint::Notebook)
                .expect("notebook binding")
                .copy_cli_lines[0] = "aureline command run wrong.verb".to_owned();
        }
        "invariant_violated" => {
            input
                .invariants
                .builder_reuses_command_truth_not_private_form_state = false;
        }
        other => panic!("unknown mutation {other}"),
    }
    RecipeBuilderFirstConsumersPacket::materialize(input)
}

#[test]
fn checked_in_packet_matches_the_seed() {
    let seeded = seeded_recipe_builder_first_consumers_packet();
    let artifact: RecipeBuilderFirstConsumersPacket =
        read_json(&repo_root().join(RECIPE_BUILDER_FIRST_CONSUMERS_PACKET_ARTIFACT_REF));
    assert_eq!(
        artifact, seeded,
        "checked-in packet must be bit-for-bit derivable from the seed; regenerate the dump"
    );
    assert!(seeded.is_stable());
}

#[test]
fn every_entrypoint_binds_a_reusing_builder() {
    let packet = seeded_recipe_builder_first_consumers_packet();
    for entrypoint in RecipeBuilderEntrypoint::ALL {
        let binding = packet
            .binding(entrypoint)
            .unwrap_or_else(|| panic!("missing binding for {}", entrypoint.as_str()));
        // The binding reuses command truth: its session record is non-empty and
        // every step keeps its command identity.
        assert!(!binding.session_record.step_drafts.is_empty());
        for draft in &binding.session_record.step_drafts {
            assert!(!draft.command_id.is_empty());
            assert!(!draft.canonical_verb.is_empty());
        }
        // Copy-CLI / open-docs parity holds per step.
        assert_eq!(
            binding.copy_cli_lines.len(),
            binding.session_record.step_drafts.len()
        );
        assert_eq!(
            binding.open_docs_anchors.len(),
            binding.session_record.step_drafts.len()
        );
    }
}

#[test]
fn worked_example_export_round_trips() {
    let export: RecipeBuilderExport = read_json(
        &repo_root()
            .join(FIXTURE_DIR)
            .join("builder_export_roundtrip.json"),
    );
    let imported = export.import();
    let reexported = imported.export(export.export_id.clone(), export.exported_at.clone());
    assert_eq!(
        reexported, export,
        "import then export must reproduce the checked-in export verbatim"
    );
    assert!(export.provenance_preserved());
}

#[test]
fn reorder_demonstration_converges() {
    #[derive(Debug, Deserialize)]
    struct ReorderDemo {
        orders_match: bool,
        step_identity_preserved: bool,
        drag_result_order: Vec<String>,
        keyboard_result_order: Vec<String>,
    }
    let demo: ReorderDemo = read_json(
        &repo_root()
            .join(FIXTURE_DIR)
            .join("reorder_preserves_identity.json"),
    );
    assert!(demo.orders_match);
    assert!(demo.step_identity_preserved);
    assert_eq!(demo.drag_result_order, demo.keyboard_result_order);
}

#[test]
fn blocked_builder_session_stays_visible() {
    use aureline_runtime::RecipeBuilderSession;
    let session: RecipeBuilderSession = read_json(
        &repo_root()
            .join(FIXTURE_DIR)
            .join("blocked_builder_session.json"),
    );
    assert_eq!(
        session.builder_state_class,
        RecipeBuilderStateClass::Blocked
    );
    assert!(session
        .validation_findings
        .iter()
        .any(|finding| finding.severity == "blocker"));
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
        if file_name == "first_consumers_stable.json" {
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
