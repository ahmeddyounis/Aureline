//! Protected tests for finalized Support Center and Diagnostics Center surfaces.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_support::finalize_support_center_surfaces_performance_inspector_language_service::{
    load_diagnostics_center_record, DiagnosticsCenterEvaluator, DiagnosticsCenterRecord,
    HealthStateClass, OutageScopeClass, ServiceFamilyClass, DIAGNOSTICS_CENTER_ARTIFACT_REF,
    DIAGNOSTICS_CENTER_DOC_REF, DIAGNOSTICS_CENTER_FIXTURE_DIR, DIAGNOSTICS_CENTER_SCHEMA_REF,
    DIAGNOSTICS_CENTER_SCHEMA_VERSION,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Manifest {
    scenarios: Vec<ManifestScenario>,
}

#[derive(Debug, Deserialize)]
struct ManifestScenario {
    scenario_id: String,
    file: String,
    expected_outage_scope: String,
    #[serde(default)]
    expected_all_healthy: bool,
    #[serde(default)]
    expected_continuity_note_present: bool,
    #[serde(default)]
    expected_quarantine_ref_present: bool,
    #[serde(default)]
    expected_index_freshness: Option<String>,
    #[serde(default)]
    expected_raw_prompts_excluded: bool,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join(DIAGNOSTICS_CENTER_FIXTURE_DIR)
}

fn load_manifest() -> Manifest {
    let path = fixture_dir().join("manifest.yaml");
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_yaml::from_str(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

struct LoadedScenario {
    scenario_id: String,
    record: DiagnosticsCenterRecord,
    expected: ManifestScenario,
}

fn load_scenarios() -> Vec<LoadedScenario> {
    let manifest = load_manifest();
    manifest
        .scenarios
        .into_iter()
        .map(|scenario| {
            let path = fixture_dir().join(&scenario.file);
            let yaml =
                std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
            let record = load_diagnostics_center_record(&yaml)
                .unwrap_or_else(|err| panic!("parse {path:?}: {err}"));
            LoadedScenario {
                scenario_id: scenario.scenario_id.clone(),
                record,
                expected: scenario,
            }
        })
        .collect()
}

#[test]
fn fixture_corpus_validates_and_covers_all_service_families() {
    let evaluator = DiagnosticsCenterEvaluator::new();
    let scenarios = load_scenarios();
    assert_eq!(scenarios.len(), 6);

    for scenario in &scenarios {
        let report = evaluator.validate_record(&scenario.record);
        assert!(
            report.is_valid(),
            "{}: validation failed: {:?}",
            scenario.scenario_id,
            report.violations
        );

        assert_eq!(
            scenario.record.schema_version, DIAGNOSTICS_CENTER_SCHEMA_VERSION,
            "{}: schema_version must be 1",
            scenario.scenario_id
        );
        assert!(
            scenario.record.raw_private_material_excluded,
            "{}: raw_private_material_excluded must be true",
            scenario.scenario_id
        );
        assert!(
            !scenario.record.exact_build_identity_ref.is_empty(),
            "{}: exact_build_identity_ref must be non-empty",
            scenario.scenario_id
        );
        assert!(
            !scenario.record.recovery_hooks.is_empty(),
            "{}: at least one recovery hook is required",
            scenario.scenario_id
        );

        // All required service families must be present.
        let families: BTreeSet<ServiceFamilyClass> = scenario
            .record
            .health_feed_items
            .iter()
            .map(|item| item.service_family)
            .collect();
        for required in ServiceFamilyClass::REQUIRED {
            assert!(
                families.contains(&required),
                "{}: missing service family {}",
                scenario.scenario_id,
                required.as_str()
            );
        }

        // Every health-feed item must have at least one diagnostics action.
        for item in &scenario.record.health_feed_items {
            assert!(
                !item.diagnostics_actions.is_empty(),
                "{}: item {} must have at least one diagnostics action",
                scenario.scenario_id,
                item.item_id
            );
        }
    }
}

#[test]
fn outage_scope_matches_expected_and_partial_outage_preserves_healthy_subsystems() {
    let scenarios = load_scenarios();

    for scenario in &scenarios {
        let actual = scenario.record.outage_scope.as_str();
        assert_eq!(
            actual, scenario.expected.expected_outage_scope,
            "{}: outage_scope mismatch",
            scenario.scenario_id
        );

        if scenario.record.outage_scope == OutageScopeClass::PartialService {
            let healthy_count = scenario
                .record
                .health_feed_items
                .iter()
                .filter(|item| item.health_state == HealthStateClass::Healthy)
                .count();
            assert!(
                healthy_count > 0,
                "{}: partial-service outage must keep at least one subsystem explicitly healthy",
                scenario.scenario_id
            );
        }

        if scenario.expected.expected_all_healthy {
            assert!(
                scenario
                    .record
                    .health_feed_items
                    .iter()
                    .all(|item| item.health_state == HealthStateClass::Healthy),
                "{}: expected all healthy",
                scenario.scenario_id
            );
        }
    }
}

#[test]
fn continuity_notes_present_when_required() {
    let scenarios = load_scenarios();

    for scenario in &scenarios {
        if scenario.expected.expected_continuity_note_present {
            let notes = scenario
                .record
                .health_feed_items
                .iter()
                .filter(|item| {
                    item.local_only_continuity_note_required
                        && item.local_only_continuity_note.is_some()
                })
                .count();
            assert!(
                notes > 0,
                "{}: expected continuity note present",
                scenario.scenario_id
            );
        }
    }
}

#[test]
fn performance_inspector_budget_and_waiver_hooks() {
    let scenarios = load_scenarios();

    for scenario in &scenarios {
        let inspector = &scenario.record.performance_inspector;
        assert!(
            inspector.p50_budget_ms > 0,
            "{}: p50 budget must be positive",
            scenario.scenario_id
        );
        assert!(
            inspector.p95_budget_ms > 0,
            "{}: p95 budget must be positive",
            scenario.scenario_id
        );

        if scenario.scenario_id == "performance_p95_breach" {
            assert!(
                !inspector.within_budget,
                "{}: expected within_budget false",
                scenario.scenario_id
            );
            assert!(
                !inspector.waiver_hooks.is_empty(),
                "{}: expected waiver hooks present",
                scenario.scenario_id
            );
        } else {
            assert!(
                inspector.within_budget,
                "{}: expected within_budget true",
                scenario.scenario_id
            );
        }
    }
}

#[test]
fn language_service_quarantine_state_detected() {
    let scenarios = load_scenarios();

    for scenario in &scenarios {
        let dashboard = &scenario.record.language_service_dashboard;
        if scenario.expected.expected_quarantine_ref_present {
            assert!(
                dashboard.quarantined_provider_present,
                "{}: expected quarantined provider present",
                scenario.scenario_id
            );
            assert!(
                dashboard
                    .provider_availability_rows
                    .iter()
                    .any(|row| row.quarantine_ref.is_some()),
                "{}: expected at least one quarantine ref",
                scenario.scenario_id
            );
        }
    }
}

#[test]
fn index_health_freshness_matches_expected() {
    let scenarios = load_scenarios();

    for scenario in &scenarios {
        if let Some(ref expected) = scenario.expected.expected_index_freshness {
            let actual = scenario.record.index_health_view.index_freshness.as_str();
            assert_eq!(
                actual, expected,
                "{}: index freshness mismatch",
                scenario.scenario_id
            );
        }
    }
}

#[test]
fn ai_evidence_excludes_raw_material() {
    let scenarios = load_scenarios();

    for scenario in &scenarios {
        let ai = &scenario.record.ai_evidence_inspector;
        assert!(
            ai.raw_prompts_excluded,
            "{}: raw_prompts_excluded must be true",
            scenario.scenario_id
        );
        assert!(
            ai.raw_provider_payloads_excluded,
            "{}: raw_provider_payloads_excluded must be true",
            scenario.scenario_id
        );

        if scenario.expected.expected_raw_prompts_excluded {
            assert!(
                ai.redaction_class_token == "metadata_safe_default",
                "{}: expected metadata_safe_default redaction",
                scenario.scenario_id
            );
        }
    }
}

#[test]
fn support_center_cards_derive_from_health_feed_identically() {
    let evaluator = DiagnosticsCenterEvaluator::new();
    let scenarios = load_scenarios();

    for scenario in &scenarios {
        let cards = evaluator.support_center_cards(&scenario.record, "2026-06-02T12:00:00Z");
        assert_eq!(
            cards.len(),
            scenario.record.health_feed_items.len(),
            "{}: card count must match health-feed item count",
            scenario.scenario_id
        );

        for (card, item) in cards.iter().zip(&scenario.record.health_feed_items) {
            assert_eq!(
                card.health_feed_item_id, item.item_id,
                "{}: card must reference health-feed item",
                scenario.scenario_id
            );
            assert_eq!(
                card.health_state, item.health_state,
                "{}: card health state must match health-feed item",
                scenario.scenario_id
            );
            assert!(
                card.read_only,
                "{}: support center card must be read-only by default",
                scenario.scenario_id
            );
            assert!(
                !card.actions.is_empty(),
                "{}: card must have at least one action",
                scenario.scenario_id
            );
        }
    }
}

#[test]
fn diagnostics_center_projects_the_shared_feed_contract() {
    let scenarios = load_scenarios();

    for scenario in &scenarios {
        let feed = scenario.record.shared_service_health_feed();
        let report = feed.validate();
        assert!(
            report.passed,
            "{}: shared feed projection failed: {:?}",
            scenario.scenario_id,
            report.findings
        );
        assert_eq!(
            feed.items.len(),
            scenario.record.health_feed_items.len(),
            "{}: shared feed item count drift",
            scenario.scenario_id
        );
    }
}

#[test]
fn diagnostics_export_packet_is_export_safe_and_preserves_exact_build_identity() {
    let evaluator = DiagnosticsCenterEvaluator::new();
    let scenarios = load_scenarios();

    let packet = evaluator
        .support_packet(
            "support.packet.diagnostics_center",
            "2026-06-02T12:00:00Z",
            &scenarios
                .iter()
                .map(|s| s.record.clone())
                .collect::<Vec<_>>(),
        )
        .expect("support packet must build from valid scenarios");

    assert!(packet.is_export_safe());
    assert!(packet.raw_private_material_excluded);
    assert!(packet.ambient_authority_excluded);
    assert!(packet.all_rows_export_safe);
    assert_eq!(packet.rows.len(), scenarios.len());

    for row in &packet.rows {
        assert!(
            !row.exact_build_identity_ref.is_empty(),
            "exact_build_identity_ref must be non-empty in export"
        );
        assert!(
            row.raw_private_material_excluded,
            "raw_private_material_excluded must be true in export"
        );
    }
}

#[test]
fn evaluator_refuses_record_with_missing_service_family() {
    let evaluator = DiagnosticsCenterEvaluator::new();
    let mut scenarios = load_scenarios();
    let mut record = scenarios.remove(0).record;

    // Remove all performance health-feed items to create a gap.
    record
        .health_feed_items
        .retain(|item| item.service_family != ServiceFamilyClass::Performance);

    let report = evaluator.validate_record(&record);
    assert!(!report.is_valid());
    assert!(report
        .violations
        .iter()
        .any(|v| v.check_id == "diagnostics_center.missing_service_family"));
}

#[test]
fn evaluator_refuses_record_without_raw_private_material_excluded() {
    let evaluator = DiagnosticsCenterEvaluator::new();
    let mut scenarios = load_scenarios();
    let mut record = scenarios.remove(0).record;

    record.raw_private_material_excluded = false;

    let report = evaluator.validate_record(&record);
    assert!(!report.is_valid());
    assert!(report
        .violations
        .iter()
        .any(|v| v.check_id == "diagnostics_center.raw_private_material_not_excluded"));
}

#[test]
fn checked_in_artifacts_and_docs_exist_for_reviewers() {
    let root = repo_root();
    for rel in [
        DIAGNOSTICS_CENTER_SCHEMA_REF,
        DIAGNOSTICS_CENTER_DOC_REF,
        DIAGNOSTICS_CENTER_ARTIFACT_REF,
    ] {
        assert!(root.join(rel).is_file(), "{rel} must exist");
    }
}
