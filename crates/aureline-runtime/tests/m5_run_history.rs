//! Fixture-driven coverage for the M5 run-history / evidence-panel object and its
//! first consumers: the checked-in packet matches the seed bit-for-bit, every
//! entrypoint binds a panel whose evidence rows quote the recomputed rerun
//! resolution, the worked-example export round-trips into an equal entry, the
//! survival demonstration proves identity and rerun survive history and support,
//! the imported row keeps offering no rerun, and every mutation fixture reproduces
//! the fail-closed promotion state the gate enforces.

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_run_history_first_consumers_input, seeded_run_history_entry, seeded_run_history_panel,
    AutomationBaselinePromotionState, CurrentPolicyBlocker, OpenAsRecipeActionClass,
    RecipeBuilderEntrypoint, RerunActionClass, RunHistoryConsumerBinding, RunHistoryEntry,
    RunHistoryEvidenceExport, RunHistoryEvidenceRow, RunHistoryFirstConsumersInput,
    RunHistoryFirstConsumersPacket, RUN_HISTORY_FIRST_CONSUMERS_PACKET_ARTIFACT_REF,
};
use serde::Deserialize;

const FIXTURE_DIR: &str = "fixtures/automation/m5/run-history-evidence";

/// Each mutation fixture is (file-name, mutation).
const CASES: [(&str, &str); 8] = [
    ("run_history_stable.json", "none"),
    (
        "missing_entrypoint_blocks_stable.json",
        "missing_entrypoint",
    ),
    (
        "rerun_implies_cached_approval_blocks_stable.json",
        "rerun_implies_cached_approval",
    ),
    (
        "macro_offers_external_rerun_blocks_stable.json",
        "macro_offers_external_rerun",
    ),
    (
        "capability_laundered_into_recipe_blocks_stable.json",
        "capability_laundered_into_recipe",
    ),
    (
        "raw_secret_material_in_history_blocks_stable.json",
        "raw_secret_material_in_history",
    ),
    (
        "evidence_row_projection_inconsistent_blocks_stable.json",
        "evidence_row_projection_inconsistent",
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

fn rebuild_binding(
    input: &mut RunHistoryFirstConsumersInput,
    entrypoint: RecipeBuilderEntrypoint,
    entries: Vec<RunHistoryEntry>,
) {
    input
        .consumer_bindings
        .retain(|binding| binding.entrypoint != entrypoint);
    input
        .consumer_bindings
        .push(RunHistoryConsumerBinding::from_entries(
            entrypoint,
            entries,
            "mutated panel",
        ));
}

fn binding_mut(
    input: &mut RunHistoryFirstConsumersInput,
    entrypoint: RecipeBuilderEntrypoint,
) -> &mut RunHistoryConsumerBinding {
    input
        .consumer_bindings
        .iter_mut()
        .find(|binding| binding.entrypoint == entrypoint)
        .expect("entrypoint present")
}

fn mutated(mutation: &str) -> RunHistoryFirstConsumersPacket {
    let mut input = current_run_history_first_consumers_input();
    match mutation {
        "none" => {}
        "missing_entrypoint" => {
            input
                .consumer_bindings
                .retain(|binding| binding.entrypoint != RecipeBuilderEntrypoint::Package);
        }
        "rerun_implies_cached_approval" => {
            let mut entry = seeded_run_history_entry(RecipeBuilderEntrypoint::RequestApi);
            entry
                .current_policy_blockers
                .push(CurrentPolicyBlocker::NoBlockerPresent);
            rebuild_binding(&mut input, RecipeBuilderEntrypoint::RequestApi, vec![entry]);
        }
        "macro_offers_external_rerun" => {
            let mut entries = seeded_run_history_panel(RecipeBuilderEntrypoint::Notebook);
            entries[1].current_policy_blockers =
                vec![CurrentPolicyBlocker::ExtensionOrExternalRunnerUnavailable];
            rebuild_binding(&mut input, RecipeBuilderEntrypoint::Notebook, entries);
        }
        "capability_laundered_into_recipe" => {
            let mut entry = seeded_run_history_entry(RecipeBuilderEntrypoint::TaskTestDebug);
            entry.open_as_recipe_action_class = OpenAsRecipeActionClass::AdmissibleMacroPromotable;
            rebuild_binding(
                &mut input,
                RecipeBuilderEntrypoint::TaskTestDebug,
                vec![entry],
            );
        }
        "raw_secret_material_in_history" => {
            let mut entry = seeded_run_history_entry(RecipeBuilderEntrypoint::RequestApi);
            entry.secret_reference_refs = vec!["raw:plaintext-token".to_owned()];
            rebuild_binding(&mut input, RecipeBuilderEntrypoint::RequestApi, vec![entry]);
        }
        "evidence_row_projection_inconsistent" => {
            binding_mut(&mut input, RecipeBuilderEntrypoint::Notebook).evidence_rows[0]
                .rerun_action_class = RerunActionClass::BlockedReplayWindowExpired;
        }
        "invariant_violated" => {
            input.invariants.raw_secrets_never_appear_in_history = false;
        }
        other => panic!("unknown mutation {other}"),
    }
    RunHistoryFirstConsumersPacket::materialize(input)
}

#[test]
fn checked_in_packet_matches_the_seed() {
    let seeded = aureline_runtime::seeded_run_history_first_consumers_packet();
    let artifact: RunHistoryFirstConsumersPacket =
        read_json(&repo_root().join(RUN_HISTORY_FIRST_CONSUMERS_PACKET_ARTIFACT_REF));
    assert_eq!(
        artifact, seeded,
        "checked-in packet must be bit-for-bit derivable from the seed; regenerate the dump"
    );
    assert!(seeded.is_stable());
}

#[test]
fn every_entrypoint_binds_an_attributable_panel() {
    let packet = aureline_runtime::seeded_run_history_first_consumers_packet();
    for entrypoint in RecipeBuilderEntrypoint::ALL {
        let binding = packet
            .binding(entrypoint)
            .unwrap_or_else(|| panic!("missing binding for {}", entrypoint.as_str()));
        assert!(!binding.entries.is_empty());
        assert_eq!(binding.evidence_rows.len(), binding.entries.len());
        for (entry, row) in binding.entries.iter().zip(&binding.evidence_rows) {
            assert!(!entry.run_identity.run_id.is_empty());
            assert!(entry.rerun_consistent());
            assert!(entry.open_as_recipe_consistent());
            assert!(entry.secret_references_opaque());
            assert!(entry.retention_consistent());
            // The evidence row quotes the recomputed rerun resolution.
            assert_eq!(row.rerun_action_class, entry.resolved_rerun_class());
            assert_eq!(row.rerun_admissible, entry.rerun_admissible());
            // An imported row never offers a rerun.
            if entry.imported {
                assert_eq!(
                    row.rerun_action_class,
                    RerunActionClass::BlockedImportedRecord
                );
            }
        }
    }
}

#[test]
fn worked_example_export_round_trips() {
    let export: RunHistoryEvidenceExport = read_json(
        &repo_root()
            .join(FIXTURE_DIR)
            .join("run_history_export_roundtrip.json"),
    );
    let imported = export.import();
    let reexported = imported.export(export.export_id.clone(), export.exported_at.clone());
    assert_eq!(
        reexported, export,
        "import then export must reproduce the checked-in export verbatim"
    );
    assert!(export.identity_and_rerun_preserved());
}

#[test]
fn imported_row_blocks_rerun() {
    let row: RunHistoryEvidenceRow = read_json(
        &repo_root()
            .join(FIXTURE_DIR)
            .join("imported_row_blocks_rerun.json"),
    );
    assert!(row.imported);
    assert_eq!(
        row.rerun_action_class,
        RerunActionClass::BlockedImportedRecord
    );
    assert!(!row.rerun_admissible);
}

#[test]
fn rerun_survives_history_and_support() {
    #[derive(Debug, Deserialize)]
    struct Survival {
        rerun_preserved: bool,
        digest_preserved: bool,
        rerun_resolution_is_fresh: bool,
        identity_and_rerun_preserved: bool,
        initial_rerun: String,
        history_rerun: String,
    }
    let demo: Survival = read_json(
        &repo_root()
            .join(FIXTURE_DIR)
            .join("rerun_survives_history_and_support.json"),
    );
    assert!(demo.rerun_preserved);
    assert!(demo.digest_preserved);
    assert!(demo.rerun_resolution_is_fresh);
    assert!(demo.identity_and_rerun_preserved);
    assert_eq!(demo.initial_rerun, demo.history_rerun);
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
        if file_name == "run_history_stable.json" {
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
