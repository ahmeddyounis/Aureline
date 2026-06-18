//! Fixture-driven coverage for the M5 adapter hierarchy negotiation baseline:
//! per-ecosystem capability negotiation, ordered native-first resolution, the
//! explicit fallback-reason packet, named unsupported capabilities, surfaced
//! capability drift, and the disclosure surfaces that keep a lower-priority
//! adapter from silently displacing native/BSP/BEP truth.

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_adapter_hierarchy_negotiation_input, AdapterNegotiationBaseline,
    AdapterNegotiationBaselineInput, BuildTestAdapterCapabilityState, BuildTestInteropConfidence,
    CapabilityNegotiation, DisclosureSurface, Ecosystem, NegotiatedCapability,
    ADAPTER_NEGOTIATION_BASELINE_ARTIFACT_REF, ADAPTER_NEGOTIATION_DOC_REF,
    ADAPTER_NEGOTIATION_ENVELOPE_SCHEMA_REF, ADAPTER_NEGOTIATION_FIXTURE_DIR,
    ADAPTER_NEGOTIATION_POLICY_SCHEMA_REF, ADAPTER_NEGOTIATION_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct NegotiationFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    mutation: String,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    expected_finding_kinds: Vec<String>,
    ecosystem_tokens: Vec<String>,
    selected_source_kind_tokens: Vec<String>,
    fallback_class_tokens: Vec<String>,
    drift_class_tokens: Vec<String>,
    disclosure_surface_tokens: Vec<String>,
    support_export_safe: bool,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root canonicalizes")
}

fn assert_exists(rel: &str) {
    let path = repo_root().join(rel);
    assert!(
        path.exists(),
        "expected path to exist on disk: {} ({})",
        rel,
        path.display()
    );
}

fn load_fixture(file_name: &str) -> NegotiationFixture {
    let path = repo_root()
        .join(ADAPTER_NEGOTIATION_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

/// Mirrors the mutations applied by the `dump_m5_adapter_hierarchy_negotiation`
/// regenerator, so the checked-in fixtures stay bit-for-bit derivable.
fn mutated_input(mutation: &str) -> AdapterNegotiationBaselineInput {
    let mut input = current_stable_adapter_hierarchy_negotiation_input();
    match mutation {
        "none" => {}
        "lower_priority_displaces_higher" => {
            let gradle = input
                .resolutions
                .iter_mut()
                .find(|r| r.ecosystem == Ecosystem::GradleJvm)
                .expect("seed has the gradle resolution");
            let native = gradle
                .candidate_ladder
                .iter_mut()
                .find(|c| c.priority_rank == 1)
                .expect("seed has the native rung");
            native.available = true;
            native.capabilities = vec![CapabilityNegotiation {
                capability: NegotiatedCapability::LifecycleEvents,
                state: BuildTestAdapterCapabilityState::Negotiated,
                capability_packet_ref: "capability-packet:gradle_jvm:native:lifecycle_events"
                    .to_owned(),
                note: "native is reachable".to_owned(),
            }];
        }
        "heuristic_overclaims_confidence" => {
            let generic = input
                .resolutions
                .iter_mut()
                .find(|r| r.ecosystem == Ecosystem::Generic)
                .expect("seed has the generic resolution");
            generic.confidence = BuildTestInteropConfidence::High;
        }
        "fallback_not_downgraded" => {
            let pytest = input
                .resolutions
                .iter_mut()
                .find(|r| r.ecosystem == Ecosystem::PythonPytest)
                .expect("seed has the pytest resolution");
            pytest.downgraded = false;
        }
        "unsupported_capability_unnamed" => {
            let pytest = input
                .resolutions
                .iter_mut()
                .find(|r| r.ecosystem == Ecosystem::PythonPytest)
                .expect("seed has the pytest resolution");
            pytest
                .unsupported_capabilities
                .retain(|c| *c != NegotiatedCapability::TargetGraph);
        }
        "skip_reason_missing" => {
            let bazel = input
                .resolutions
                .iter_mut()
                .find(|r| r.ecosystem == Ecosystem::Bazel)
                .expect("seed has the bazel resolution");
            let native = bazel
                .candidate_ladder
                .iter_mut()
                .find(|c| c.priority_rank == 1)
                .expect("seed has the native rung");
            native.skip_reason = None;
            bazel
                .fallback_reasons
                .retain(|r| r.adapter_id != "adapter:bazel:native");
        }
        "drift_not_visible" => {
            input.drift_signals[0].visible_before_trust_loss = false;
        }
        "disclosure_surface_missing" => {
            input
                .disclosure_surfaces
                .retain(|b| b.surface != DisclosureSurface::AiEvidence);
        }
        other => panic!("unknown mutation {other}"),
    }
    input
}

fn assert_token_set(observed: Vec<&str>, expected: &[String], label: &str) {
    let mut observed = observed;
    observed.sort_unstable();
    let mut expected: Vec<&str> = expected.iter().map(String::as_str).collect();
    expected.sort_unstable();
    assert_eq!(observed, expected, "{label} token set drift");
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "m5_adapter_hierarchy_negotiation_case",
        "fixture {file_name} declares unexpected record_kind"
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.case_name.trim().is_empty() && !fixture.scenario.trim().is_empty(),
        "fixture must describe its case and scenario"
    );

    let baseline = AdapterNegotiationBaseline::materialize(mutated_input(&fixture.mutation));
    assert_eq!(
        baseline.promotion_state.as_str(),
        fixture.expect.promotion_state,
        "fixture {} promotion drift",
        fixture.case_name
    );
    assert_eq!(
        baseline.validation_findings.len(),
        fixture.expect.validation_finding_count,
        "fixture {} finding count drift; got {:?}",
        fixture.case_name,
        baseline
            .validation_findings
            .iter()
            .map(|f| f.finding_kind.as_str())
            .collect::<Vec<_>>()
    );
    let observed_kinds: Vec<&str> = baseline
        .validation_findings
        .iter()
        .map(|f| f.finding_kind.as_str())
        .collect();
    assert_eq!(
        observed_kinds,
        fixture
            .expect
            .expected_finding_kinds
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        "fixture {} finding kinds drift",
        fixture.case_name
    );
    assert_token_set(
        baseline.ecosystem_tokens(),
        &fixture.expect.ecosystem_tokens,
        "ecosystem",
    );
    assert_token_set(
        baseline.selected_source_kind_tokens(),
        &fixture.expect.selected_source_kind_tokens,
        "selected source kind",
    );
    assert_token_set(
        baseline.fallback_class_tokens(),
        &fixture.expect.fallback_class_tokens,
        "fallback class",
    );
    assert_token_set(
        baseline.drift_class_tokens(),
        &fixture.expect.drift_class_tokens,
        "drift class",
    );
    assert_token_set(
        baseline.disclosure_surface_tokens(),
        &fixture.expect.disclosure_surface_tokens,
        "disclosure surface",
    );

    let export = baseline.support_export(
        format!("support-export:{}", fixture.case_name),
        "2026-06-17T00:01:00Z",
    );
    assert_eq!(
        export.is_export_safe(),
        fixture.expect.support_export_safe,
        "fixture {} support-export safety drift",
        fixture.case_name
    );
}

#[test]
fn schema_doc_fixture_and_artifact_exist_on_disk() {
    assert_exists(ADAPTER_NEGOTIATION_SCHEMA_REF);
    assert_exists(ADAPTER_NEGOTIATION_ENVELOPE_SCHEMA_REF);
    assert_exists(ADAPTER_NEGOTIATION_POLICY_SCHEMA_REF);
    assert_exists(ADAPTER_NEGOTIATION_DOC_REF);
    assert_exists(ADAPTER_NEGOTIATION_FIXTURE_DIR);
    assert_exists(ADAPTER_NEGOTIATION_BASELINE_ARTIFACT_REF);
}

#[test]
fn checked_in_baseline_validates_clean() {
    let path = repo_root().join(ADAPTER_NEGOTIATION_BASELINE_ARTIFACT_REF);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("baseline artifact {path:?} must read: {err}"));
    let baseline: AdapterNegotiationBaseline = serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("baseline artifact {path:?} must parse: {err}"));
    assert!(
        baseline.validate().is_empty(),
        "checked-in baseline must validate without findings: {:?}",
        baseline.validate()
    );
    assert_eq!(baseline.promotion_state.as_str(), "stable");
}

#[test]
fn baseline_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn lower_priority_displaces_higher_blocks_stable() {
    assert_fixture_matches("lower_priority_displaces_higher_blocks_stable.json");
}

#[test]
fn heuristic_overclaims_confidence_blocks_stable() {
    assert_fixture_matches("heuristic_overclaims_confidence_blocks_stable.json");
}

#[test]
fn fallback_not_downgraded_blocks_stable() {
    assert_fixture_matches("fallback_not_downgraded_blocks_stable.json");
}

#[test]
fn unsupported_capability_unnamed_blocks_stable() {
    assert_fixture_matches("unsupported_capability_unnamed_blocks_stable.json");
}

#[test]
fn skip_reason_missing_blocks_stable() {
    assert_fixture_matches("skip_reason_missing_blocks_stable.json");
}

#[test]
fn drift_not_visible_blocks_stable() {
    assert_fixture_matches("drift_not_visible_blocks_stable.json");
}

#[test]
fn disclosure_surface_missing_blocks_stable() {
    assert_fixture_matches("disclosure_surface_missing_blocks_stable.json");
}
