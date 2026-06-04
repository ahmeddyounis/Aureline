//! Protected checks for scanner import parity across review, CLI, support, and release packets.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_language::DiagnosticAnchorRemapStateClass;
use aureline_runtime::scanner_import::{
    materialize_sarif_import_session, materialize_structured_scanner_import_session,
    ScannerBaselineFamilyStateClass, ScannerDeltaCompatibilityClass, ScannerDeltaCounts,
    ScannerFindingDeltaState, ScannerFindingFidelityClass, ScannerFindingTruthClass,
    ScannerImportDeploymentProfileClass, ScannerImportFreshnessClass, ScannerImportRequest,
    ScannerImportSourceClass, ScannerParityComparisonStateClass, ScannerRawPayloadBacklinkPolicy,
    ScannerSourceFormatClass,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct StableScannerParityFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    request: ScannerImportRequest,
    expect: StableScannerParityExpect,
}

#[derive(Debug, Deserialize)]
struct StableScannerParityExpect {
    source_class: String,
    deployment_profile_class: String,
    signer_verified: bool,
    public_fallback_blocked: bool,
    raw_payload_ref: String,
    unsupported_field_ref: String,
    delta_counts: ScannerDeltaCounts,
    parity_source_tokens: Vec<String>,
    parity_comparison_tokens: Vec<String>,
    silent_fix_all_blocked: bool,
}

fn repo_fixture_dir(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("repo root")
        .join("fixtures")
        .join("quality")
        .join(relative)
}

fn load_fixture(relative_dir: &str, name: &str) -> String {
    let path = repo_fixture_dir(relative_dir).join(name);
    fs::read_to_string(&path).unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()))
}

fn load_sarif_request() -> ScannerImportRequest {
    serde_json::from_str(&load_fixture("sarif_alpha", "import_request.json"))
        .expect("parse SARIF import request")
}

fn load_structured_request() -> ScannerImportRequest {
    serde_json::from_str(&load_fixture(
        "scanner_import_beta",
        "structured_import_request.json",
    ))
    .expect("parse structured import request")
}

fn load_stable_parity_fixture() -> StableScannerParityFixture {
    serde_json::from_str(&load_fixture(
        "m4/sarif-and-scanner-parity",
        "stable_signed_mirror_import_request.json",
    ))
    .expect("parse stable scanner parity fixture")
}

fn token_set(values: impl IntoIterator<Item = String>) -> BTreeSet<String> {
    values.into_iter().collect()
}

#[test]
fn sarif_import_projects_review_cli_support_and_release_labels() {
    let session = materialize_sarif_import_session(
        load_sarif_request(),
        &load_fixture("sarif_alpha", "scanner_output.sarif.json"),
    )
    .expect("SARIF import materializes");

    assert_eq!(session.findings.len(), 5);
    assert!(session.findings.iter().all(|finding| finding.read_only));
    assert_eq!(session.review_packet.finding_rows.len(), 5);
    assert_eq!(
        session.review_packet.delta_counts,
        session.delta_packet.delta_counts
    );
    assert!(session.review_packet.finding_rows.iter().any(|row| {
        row.truth_class == ScannerFindingTruthClass::LocallyConfirmed
            && row.read_only
            && row.import_freshness_class == ScannerImportFreshnessClass::ImportedSnapshot
    }));

    let cli = session.cli_projection();
    assert_eq!(cli.source_format_class, ScannerSourceFormatClass::Sarif21);
    assert_eq!(cli.imported_finding_count, 5);
    assert_eq!(cli.locally_confirmed_count, 1);
    assert_eq!(cli.read_only_count, 5);
    assert!(cli.exact_delta_claim_blocked);

    let support = session.support_export(None);
    assert_eq!(support.imported_finding_count, 5);
    assert_eq!(support.locally_confirmed_count, 1);
    assert_eq!(
        support.raw_payload_backlink_policy,
        ScannerRawPayloadBacklinkPolicy::OpaqueRefsOnly
    );
    assert!(!support.payload_backlinks_redacted);
    assert!(support.rows.iter().any(|row| {
        row.delta_state_class == ScannerFindingDeltaState::Unmapped
            && row.remap_state_class == DiagnosticAnchorRemapStateClass::Unmapped
    }));

    let release = session.release_packet("release:candidate:quality:scanner");
    assert_eq!(
        release.compatibility_class,
        ScannerDeltaCompatibilityClass::BlockedAnchorMappingUncertain
    );
    assert_eq!(release.active_suppression_or_waiver_count, 2);
    assert_eq!(release.release_visible_baseline_count, 2);
    assert!(release.imported_findings_read_only);
    assert!(release.exact_delta_claim_blocked);
    assert!(release.parity_note.contains("imported labels"));
}

#[test]
fn structured_scanner_payload_uses_same_delta_and_packet_model() {
    let session = materialize_structured_scanner_import_session(
        load_structured_request(),
        &load_fixture("scanner_import_beta", "structured_scanner_output.json"),
    )
    .expect("structured scanner import materializes");

    assert_eq!(
        session.run_descriptors[0].source_format_class,
        ScannerSourceFormatClass::StructuredScannerJson
    );
    assert_eq!(session.delta_packet.delta_counts.new_count, 1);
    assert_eq!(session.delta_packet.delta_counts.resolved_count, 1);
    assert_eq!(session.delta_packet.delta_counts.persisting_count, 1);
    assert_eq!(session.delta_packet.delta_counts.suppressed_count, 1);
    assert_eq!(session.delta_packet.delta_counts.waived_count, 1);
    assert_eq!(session.delta_packet.delta_counts.unmapped_count, 1);

    let fidelity_states = session
        .review_packet
        .finding_rows
        .iter()
        .map(|row| row.fidelity_state_class)
        .collect::<BTreeSet<_>>();
    assert!(fidelity_states.contains(&ScannerFindingFidelityClass::RemappedContextual));
    assert!(fidelity_states.contains(&ScannerFindingFidelityClass::LocallyConfirmed));
    assert!(fidelity_states.contains(&ScannerFindingFidelityClass::UnmappedAnchor));

    let serialized_support =
        serde_json::to_string(&session.support_export(None)).expect("support export serializes");
    assert!(!serialized_support.contains("src/payments"));
    assert!(!serialized_support.contains("package-lock"));
    assert!(!serialized_support.contains("message:structured"));
}

#[test]
fn stale_redacted_and_mismatched_imports_fail_safely() {
    let payload = load_fixture("scanner_import_beta", "structured_scanner_output.json");

    let mut stale_request = load_structured_request();
    stale_request.import_freshness_class = ScannerImportFreshnessClass::StaleImportedSnapshot;
    let stale = materialize_structured_scanner_import_session(stale_request, &payload)
        .expect("stale structured import materializes");
    assert!(stale.blocks_exact_delta_claims());
    assert!(stale
        .review_packet
        .finding_rows
        .iter()
        .any(|row| row.fidelity_state_class == ScannerFindingFidelityClass::StaleImported));

    let mut redacted_request = load_structured_request();
    redacted_request.raw_payload_backlink_policy =
        ScannerRawPayloadBacklinkPolicy::RedactedByPolicy;
    let redacted = materialize_structured_scanner_import_session(redacted_request, &payload)
        .expect("redacted structured import materializes");
    let redacted_support = redacted.support_export(None);
    assert!(redacted_support.payload_backlinks_redacted);
    assert_eq!(
        redacted_support.raw_payload_refs,
        vec!["raw_payload:redacted_by_policy"]
    );
    assert!(redacted_support.rows.iter().all(|row| {
        row.fidelity_state_class == ScannerFindingFidelityClass::RedactedPayload
            && row.raw_payload_ref == "raw_payload:redacted_by_policy"
    }));

    let mut mismatch_request = load_structured_request();
    mismatch_request.rule_pack.baseline_family_state_class =
        ScannerBaselineFamilyStateClass::IncompatibleRulePack;
    let mismatch = materialize_structured_scanner_import_session(mismatch_request, &payload)
        .expect("mismatched structured import materializes");
    assert_eq!(
        mismatch.delta_packet.compatibility_class,
        ScannerDeltaCompatibilityClass::BlockedRulePackMismatch
    );
    assert!(mismatch
        .review_packet
        .finding_rows
        .iter()
        .all(|row| row.fidelity_state_class == ScannerFindingFidelityClass::BaselineMismatch));
    assert!(
        mismatch
            .release_packet("release:candidate:quality:scanner")
            .exact_delta_claim_blocked
    );
}

#[test]
fn signed_mirror_import_builds_distinct_ci_local_parity_view() {
    let fixture = load_stable_parity_fixture();
    assert_eq!(fixture.record_kind, "sarif_and_scanner_parity_stable_case");
    assert_eq!(fixture.schema_version, 1);
    assert!(!fixture.case_name.trim().is_empty());
    assert!(!fixture.scenario.trim().is_empty());

    let session = materialize_structured_scanner_import_session(
        fixture.request,
        &load_fixture("scanner_import_beta", "structured_scanner_output.json"),
    )
    .expect("stable signed mirror import materializes");

    assert_eq!(session.source_class.as_str(), fixture.expect.source_class);
    assert_eq!(
        session.deployment_profile_class.as_str(),
        fixture.expect.deployment_profile_class
    );
    assert_eq!(
        session.signer.as_ref().map(|signer| signer.verified),
        Some(fixture.expect.signer_verified)
    );
    assert_eq!(
        session.raw_payload_refs,
        vec![fixture.expect.raw_payload_ref]
    );
    assert_eq!(
        session.unsupported_field_refs,
        vec![fixture.expect.unsupported_field_ref]
    );
    assert_eq!(
        session.delta_packet.delta_counts,
        fixture.expect.delta_counts
    );

    let release = session.release_packet("release:candidate:quality:scanner");
    assert_eq!(
        release.source_class,
        ScannerImportSourceClass::ManagedPipeline
    );
    assert_eq!(
        release.deployment_profile_class,
        ScannerImportDeploymentProfileClass::MirrorOnly
    );
    assert_eq!(
        release.public_fallback_blocked,
        fixture.expect.public_fallback_blocked
    );
    assert!(release.imported_findings_read_only);
    assert!(release.exact_delta_claim_blocked);

    let parity = session.ci_local_parity_view();
    assert_eq!(
        parity.compatibility_class,
        ScannerDeltaCompatibilityClass::BlockedAnchorMappingUncertain
    );
    assert_eq!(
        parity.silent_fix_all_blocked,
        fixture.expect.silent_fix_all_blocked
    );
    assert!(parity.local_confirmation_required_before_mutation);
    assert_eq!(
        token_set(
            parity
                .source_rows
                .iter()
                .map(|row| row.source_kind.as_str().to_owned())
        ),
        token_set(fixture.expect.parity_source_tokens)
    );
    assert_eq!(
        token_set(
            parity
                .comparisons
                .iter()
                .map(|row| row.comparison_state_class.as_str().to_owned())
        ),
        token_set(fixture.expect.parity_comparison_tokens)
    );
    assert!(parity.comparisons.iter().any(|row| {
        row.comparison_state_class == ScannerParityComparisonStateClass::Comparable
            && row.local_confirmation_ref.is_some()
    }));
    assert!(parity.comparisons.iter().any(|row| {
        row.comparison_state_class == ScannerParityComparisonStateClass::ParityGapAnchorMapping
    }));
}
