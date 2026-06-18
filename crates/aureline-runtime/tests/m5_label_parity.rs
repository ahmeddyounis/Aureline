//! Fixture-driven coverage for the M5 automation-label parity packet: the
//! checked-in packet matches the seed bit-for-bit, every claimed command projects
//! its label set to every surface with canonical stable ids and display tokens,
//! and every mutation fixture reproduces the fail-closed promotion state the gate
//! enforces.

use std::path::{Path, PathBuf};

use aureline_runtime::{
    canonical_safety_labels, current_label_parity_input, AutomationBaselinePromotionState,
    AutomationSafetyLabelId, LabelParityInput, LabelParityPacket, LabelSurfaceClass,
    ProjectedLabel, LABEL_PARITY_PACKET_ARTIFACT_REF,
};
use serde::Deserialize;

const FIXTURE_DIR: &str = "fixtures/automation/m5/label-parity";

/// Each mutation fixture is (file-name, mutation).
const CASES: [(&str, &str); 7] = [
    ("label_parity_stable.json", "none"),
    (
        "missing_surface_projection_blocks_stable.json",
        "missing_surface",
    ),
    (
        "surface_label_drift_blocks_stable.json",
        "surface_label_drift",
    ),
    (
        "synonym_display_token_blocks_stable.json",
        "synonym_display_token",
    ),
    (
        "effect_disclosure_dropped_blocks_stable.json",
        "effect_disclosure_dropped",
    ),
    (
        "stable_id_not_preserved_blocks_stable.json",
        "stable_id_not_preserved",
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
    command_verbs: Vec<String>,
    is_stable: bool,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> T {
    let body = std::fs::read_to_string(path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn projection_mut(
    input: &mut LabelParityInput,
    command_index: usize,
    surface: LabelSurfaceClass,
) -> &mut aureline_runtime::SurfaceLabelProjection {
    input.command_rows[command_index]
        .surface_projections
        .iter_mut()
        .find(|projection| projection.surface == surface)
        .expect("surface projection present")
}

fn mutated(mutation: &str) -> LabelParityPacket {
    let mut input = current_label_parity_input();
    match mutation {
        "none" => {}
        "missing_surface" => {
            input.command_rows[0]
                .surface_projections
                .retain(|projection| projection.surface != LabelSurfaceClass::DocsHelp);
        }
        "surface_label_drift" => {
            projection_mut(&mut input, 2, LabelSurfaceClass::CommandPaletteRow)
                .projected_labels
                .push(ProjectedLabel::canonical(
                    AutomationSafetyLabelId::MacroSafe,
                ));
        }
        "synonym_display_token" => {
            let projection = projection_mut(&mut input, 0, LabelSurfaceClass::ReleasePublicTruth);
            let label = projection
                .projected_labels
                .iter_mut()
                .find(|label| label.label_id == AutomationSafetyLabelId::WritesFiles)
                .expect("writes_files label");
            label.display_token = "Writes to disk".to_owned();
        }
        "effect_disclosure_dropped" => {
            projection_mut(&mut input, 0, LabelSurfaceClass::DocsHelp)
                .projected_labels
                .retain(|label| label.label_id != AutomationSafetyLabelId::WritesFiles);
        }
        "stable_id_not_preserved" => {
            projection_mut(&mut input, 4, LabelSurfaceClass::SupportExport)
                .preserves_stable_ids_on_downgrade = false;
        }
        "invariant_violated" => {
            input.invariants.no_surface_invents_synonyms = false;
        }
        other => panic!("unknown mutation {other}"),
    }
    LabelParityPacket::materialize(input)
}

#[test]
fn checked_in_packet_matches_the_seed() {
    let seeded = LabelParityPacket::materialize(current_label_parity_input());
    let artifact: LabelParityPacket =
        read_json(&repo_root().join(LABEL_PARITY_PACKET_ARTIFACT_REF));
    assert_eq!(
        artifact, seeded,
        "checked-in packet must be bit-for-bit derivable from the seed; regenerate the dump"
    );
    assert!(seeded.is_stable());
}

#[test]
fn vocabulary_is_the_frozen_set() {
    let packet = LabelParityPacket::materialize(current_label_parity_input());
    assert_eq!(packet.vocabulary, canonical_safety_labels());
    // Every label appears as a source label on at least one command.
    for label in AutomationSafetyLabelId::ALL {
        assert!(
            packet
                .command_rows
                .iter()
                .any(|row| row.source_labels.contains(&label)),
            "no command claims {}",
            label.as_str()
        );
    }
}

#[test]
fn every_command_projects_to_every_surface() {
    let packet = LabelParityPacket::materialize(current_label_parity_input());
    for row in &packet.command_rows {
        for surface in LabelSurfaceClass::ALL {
            let projection = row.projection(surface).unwrap_or_else(|| {
                panic!("missing {} on {}", surface.as_str(), row.canonical_verb)
            });
            // Same stable-id set as the command source; no synonyms; states preserved.
            let mut projected = projection.stable_id_tokens();
            projected.sort();
            let mut source = row.source_stable_id_tokens();
            source.sort();
            assert_eq!(
                projected,
                source,
                "{} {}",
                row.canonical_verb,
                surface.as_str()
            );
            for label in &projection.projected_labels {
                assert!(label.stable_id_matches());
                assert!(label.display_token_matches());
            }
            assert!(projection.preserves_stable_ids());
        }
    }
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
        let verbs: Vec<String> = packet
            .command_verbs()
            .into_iter()
            .map(str::to_owned)
            .collect();
        assert_eq!(verbs, fixture.expect.command_verbs);
        assert_eq!(packet.is_stable(), fixture.expect.is_stable);
        if file_name == "label_parity_stable.json" {
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
