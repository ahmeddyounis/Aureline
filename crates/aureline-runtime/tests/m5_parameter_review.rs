//! Fixture-driven coverage for the M5 parameter-review object and its first
//! consumers: the checked-in packet matches the seed bit-for-bit, the
//! worked-example sheet export round-trips into an equal sheet, the rerun
//! demonstration proves provenance and redaction posture survive, the secret
//! reference sheet holds the secret as a reference, and every mutation fixture
//! reproduces the fail-closed promotion state the gate enforces.

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_parameter_review_first_consumers_input, seeded_consumer_sheet,
    seeded_parameter_review_first_consumers_packet, AutomationBaselinePromotionState,
    ParameterFieldType, ParameterReviewConsumerBinding, ParameterReviewExport,
    ParameterReviewFirstConsumersPacket, ParameterReviewVerdictClass, ParameterSourceLayer,
    ParameterValueState, RecipeBuilderEntrypoint, SaveToScope,
    PARAMETER_REVIEW_FIRST_CONSUMERS_PACKET_ARTIFACT_REF,
};
use serde::Deserialize;

const FIXTURE_DIR: &str = "fixtures/automation/m5/parameter-review";

/// Each mutation fixture is (file-name, mutation).
const CASES: [(&str, &str); 7] = [
    ("parameter_review_stable.json", "none"),
    (
        "missing_entrypoint_blocks_stable.json",
        "missing_entrypoint",
    ),
    ("raw_secret_blocks_stable.json", "raw_secret"),
    (
        "save_scope_not_allowed_blocks_stable.json",
        "save_scope_not_allowed",
    ),
    (
        "source_layer_unspecified_blocks_stable.json",
        "source_layer_unspecified",
    ),
    (
        "sheet_projection_inconsistent_blocks_stable.json",
        "sheet_projection_inconsistent",
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

fn mutated(mutation: &str) -> ParameterReviewFirstConsumersPacket {
    let mut input = current_parameter_review_first_consumers_input();
    match mutation {
        "none" => {}
        "missing_entrypoint" => {
            input
                .consumer_bindings
                .retain(|binding| binding.entrypoint != RecipeBuilderEntrypoint::Package);
        }
        "raw_secret" => {
            let mut sheet = seeded_consumer_sheet(RecipeBuilderEntrypoint::RequestApi);
            let token = sheet
                .parameters
                .iter_mut()
                .find(|parameter| parameter.parameter_name == "bearer_token")
                .expect("bearer_token");
            token.secret_reference = None;
            token.value_state = ParameterValueState::DefaultValue;
            input
                .consumer_bindings
                .retain(|binding| binding.entrypoint != RecipeBuilderEntrypoint::RequestApi);
            input
                .consumer_bindings
                .push(ParameterReviewConsumerBinding::from_builder(&sheet));
        }
        "save_scope_not_allowed" => {
            let mut sheet = seeded_consumer_sheet(RecipeBuilderEntrypoint::Notebook);
            sheet
                .parameters
                .iter_mut()
                .find(|parameter| parameter.parameter_name == "output_dir")
                .expect("output_dir")
                .chosen_save_scope = SaveToScope::User;
            input
                .consumer_bindings
                .retain(|binding| binding.entrypoint != RecipeBuilderEntrypoint::Notebook);
            input
                .consumer_bindings
                .push(ParameterReviewConsumerBinding::from_builder(&sheet));
        }
        "source_layer_unspecified" => {
            let mut sheet = seeded_consumer_sheet(RecipeBuilderEntrypoint::Incident);
            sheet
                .parameters
                .iter_mut()
                .find(|parameter| parameter.parameter_name == "incident_ref")
                .expect("incident_ref")
                .source_layer = ParameterSourceLayer::UnspecifiedGenericControl;
            input
                .consumer_bindings
                .retain(|binding| binding.entrypoint != RecipeBuilderEntrypoint::Incident);
            input
                .consumer_bindings
                .push(ParameterReviewConsumerBinding::from_builder(&sheet));
        }
        "sheet_projection_inconsistent" => {
            input
                .consumer_bindings
                .iter_mut()
                .find(|binding| binding.entrypoint == RecipeBuilderEntrypoint::Notebook)
                .expect("notebook binding")
                .sheet_record
                .rows[0]
                .verdict_class = ParameterReviewVerdictClass::Blocked;
        }
        "invariant_violated" => {
            input.invariants.secret_values_are_references_not_raw = false;
        }
        other => panic!("unknown mutation {other}"),
    }
    ParameterReviewFirstConsumersPacket::materialize(input)
}

#[test]
fn checked_in_packet_matches_the_seed() {
    let seeded = seeded_parameter_review_first_consumers_packet();
    let artifact: ParameterReviewFirstConsumersPacket =
        read_json(&repo_root().join(PARAMETER_REVIEW_FIRST_CONSUMERS_PACKET_ARTIFACT_REF));
    assert_eq!(
        artifact, seeded,
        "checked-in packet must be bit-for-bit derivable from the seed; regenerate the dump"
    );
    assert!(seeded.is_stable());
}

#[test]
fn every_entrypoint_binds_a_typed_provenance_bearing_sheet() {
    let packet = seeded_parameter_review_first_consumers_packet();
    for entrypoint in RecipeBuilderEntrypoint::ALL {
        let binding = packet
            .binding(entrypoint)
            .unwrap_or_else(|| panic!("missing binding for {}", entrypoint.as_str()));
        assert!(!binding.reviewed_parameters.is_empty());
        for parameter in &binding.reviewed_parameters {
            // Every parameter is typed with an explicit source layer.
            assert!(parameter.source_layer.explicit_inspection_kind().is_some());
            // Secret-bearing fields hold a reference, never a raw value.
            if parameter.field_type == ParameterFieldType::SecretReference {
                assert!(parameter.secret_reference.is_some());
            }
            // Save scope is explicit and in the allowed set.
            assert!(parameter.save_scope_allowed());
        }
        // The frozen projection is aligned with the live parameters.
        assert_eq!(
            binding.sheet_record.rows.len(),
            binding.reviewed_parameters.len()
        );
    }
}

#[test]
fn worked_example_export_round_trips() {
    let export: ParameterReviewExport = read_json(
        &repo_root()
            .join(FIXTURE_DIR)
            .join("sheet_export_roundtrip.json"),
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
fn rerun_preserves_provenance_and_redaction() {
    #[derive(Debug, Deserialize)]
    struct RerunDemo {
        source_layers_preserved: bool,
        redaction_preserved: bool,
        provenance_preserved: bool,
        initial_source_layers: Vec<String>,
        rerun_source_layers: Vec<String>,
    }
    let demo: RerunDemo = read_json(
        &repo_root()
            .join(FIXTURE_DIR)
            .join("rerun_preserves_provenance.json"),
    );
    assert!(demo.source_layers_preserved);
    assert!(demo.redaction_preserved);
    assert!(demo.provenance_preserved);
    assert_eq!(demo.initial_source_layers, demo.rerun_source_layers);
}

#[test]
fn secret_reference_sheet_holds_a_reference() {
    use aureline_runtime::ParameterReviewSheet;
    let sheet: ParameterReviewSheet = read_json(
        &repo_root()
            .join(FIXTURE_DIR)
            .join("secret_reference_held_sheet.json"),
    );
    // The package registry token reads as held behind a broker handle.
    assert!(sheet
        .rows
        .iter()
        .any(|row| row.verdict_class == ParameterReviewVerdictClass::SensitiveHeldForReview));
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
        if file_name == "parameter_review_stable.json" {
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
