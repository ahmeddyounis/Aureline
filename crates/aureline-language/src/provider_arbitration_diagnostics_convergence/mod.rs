//! Provider-arbitration and diagnostics convergence layer.
//!
//! This module converges compiler, LSP, linter, framework, runtime, and policy
//! diagnostics into one typed model with explicit source, confidence, freshness,
//! and suppression class. It surfaces provider arbitration, partiality, and
//! semantic-to-text downgrade labels across Problems, editor gutters, hovers,
//! code actions, search, review, AI evidence, and support exports.
//!
//! Quick-fix and suppression flows preserve originating provider/evidence and
//! block broad apply when arbitration or stale-state risk exists. The module
//! emits a machine-readable diagnostics-convergence packet that stable
//! docs/help, shiproom dashboards, and support bundles ingest directly.
//!
//! The diagnostic source taxonomy and clustered-diagnostics model preserve
//! compiler/build, LSP, linter/formatter, framework/schema, runtime/test/debug,
//! and policy/trust provenance, freshness epoch, and run lineage even when rows
//! are deduped in compact Problems views.

mod records;

#[cfg(test)]
mod tests;

pub use records::{
    BatchFixScopeClass, ConvergedDiagnosticCluster, ConvergenceAggregateCounts,
    ConvergenceDisplayStateClass, ConvergenceInspector, ConvergencePacketAggregateCounts,
    ConvergenceValidationDefect, ConvergenceValidationReport,
    ProviderArbitrationDiagnosticsConvergencePacket,
    ProviderArbitrationDiagnosticsConvergenceSchemaVersion, ProviderClaimRow, QuickFixSafetyClass,
    SeverityConvergenceClass, SuppressionClass, CONVERGED_DIAGNOSTIC_CLUSTER_RECORD_KIND,
    CONVERGENCE_PACKET_RECORD_KIND, PROVIDER_ARBITRATION_DIAGNOSTICS_CONVERGENCE_SCHEMA_VERSION,
};

/// Repository-relative documentation ref for the convergence contract.
pub const PROVIDER_ARBITRATION_DIAGNOSTICS_CONVERGENCE_DOC_REF: &str =
    "docs/help/language/provider-arbitration-diagnostics-convergence.md";

/// Repository-relative schema ref for the convergence packet.
pub const PROVIDER_ARBITRATION_DIAGNOSTICS_CONVERGENCE_SCHEMA_REF: &str =
    "schemas/language/provider-arbitration-diagnostics-convergence.schema.json";

/// Repository-relative artifact ref for the human-readable convergence report.
pub const PROVIDER_ARBITRATION_DIAGNOSTICS_CONVERGENCE_ARTIFACT_REF: &str =
    "artifacts/language/m4/provider-arbitration-diagnostics-convergence.md";

/// Directory containing the checked-in diagnostics convergence corpus.
pub const PROVIDER_ARBITRATION_DIAGNOSTICS_CONVERGENCE_CORPUS_DIR: &str =
    "fixtures/language/m4/provider_arbitration_diagnostics_convergence";

const CORPUS_COMPILER_LSP_AGREE_PATH: &str =
    "fixtures/language/m4/provider_arbitration_diagnostics_convergence/compiler_lsp_agree_exact.yaml";
const CORPUS_COMPILER_LSP_DISAGREE_SEVERITY_PATH: &str =
    "fixtures/language/m4/provider_arbitration_diagnostics_convergence/compiler_lsp_disagree_severity.yaml";
const CORPUS_LINTER_FRAMEWORK_CONFLICT_PATH: &str =
    "fixtures/language/m4/provider_arbitration_diagnostics_convergence/linter_framework_conflict.yaml";
const CORPUS_STALE_RUNTIME_EVIDENCE_PATH: &str =
    "fixtures/language/m4/provider_arbitration_diagnostics_convergence/stale_runtime_evidence.yaml";
const CORPUS_POLICY_OVERRIDE_PATH: &str =
    "fixtures/language/m4/provider_arbitration_diagnostics_convergence/policy_override.yaml";
const CORPUS_IMPORTED_SCAN_WITH_LIVE_CONFLICT_PATH: &str =
    "fixtures/language/m4/provider_arbitration_diagnostics_convergence/imported_scan_with_live_conflict.yaml";
const CORPUS_BATCH_FIX_BLOCKED_BY_ARBITRATION_PATH: &str =
    "fixtures/language/m4/provider_arbitration_diagnostics_convergence/batch_fix_blocked_by_arbitration.yaml";
const CORPUS_SUPPRESSION_PRESERVES_PROVIDER_PATH: &str =
    "fixtures/language/m4/provider_arbitration_diagnostics_convergence/suppression_preserves_provider.yaml";

const CURRENT_DIAGNOSTICS_CONVERGENCE_FIXTURES: &[(&str, &str)] = &[
    (
        CORPUS_COMPILER_LSP_AGREE_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m4/provider_arbitration_diagnostics_convergence/compiler_lsp_agree_exact.yaml"
        )),
    ),
    (
        CORPUS_COMPILER_LSP_DISAGREE_SEVERITY_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m4/provider_arbitration_diagnostics_convergence/compiler_lsp_disagree_severity.yaml"
        )),
    ),
    (
        CORPUS_LINTER_FRAMEWORK_CONFLICT_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m4/provider_arbitration_diagnostics_convergence/linter_framework_conflict.yaml"
        )),
    ),
    (
        CORPUS_STALE_RUNTIME_EVIDENCE_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m4/provider_arbitration_diagnostics_convergence/stale_runtime_evidence.yaml"
        )),
    ),
    (
        CORPUS_POLICY_OVERRIDE_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m4/provider_arbitration_diagnostics_convergence/policy_override.yaml"
        )),
    ),
    (
        CORPUS_IMPORTED_SCAN_WITH_LIVE_CONFLICT_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m4/provider_arbitration_diagnostics_convergence/imported_scan_with_live_conflict.yaml"
        )),
    ),
    (
        CORPUS_BATCH_FIX_BLOCKED_BY_ARBITRATION_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m4/provider_arbitration_diagnostics_convergence/batch_fix_blocked_by_arbitration.yaml"
        )),
    ),
    (
        CORPUS_SUPPRESSION_PRESERVES_PROVIDER_PATH,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/language/m4/provider_arbitration_diagnostics_convergence/suppression_preserves_provider.yaml"
        )),
    ),
];

/// Loads one converged diagnostic cluster from YAML.
pub fn load_convergence_cluster(
    yaml: &str,
) -> Result<ConvergedDiagnosticCluster, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads the current checked-in diagnostics convergence corpus.
pub fn current_diagnostics_convergence_corpus(
) -> Result<Vec<ConvergedDiagnosticCluster>, serde_yaml::Error> {
    CURRENT_DIAGNOSTICS_CONVERGENCE_FIXTURES
        .iter()
        .map(|(_fixture_ref, yaml)| serde_yaml::from_str::<ConvergedDiagnosticCluster>(yaml))
        .collect::<Result<Vec<_>, _>>()
}

/// Returns fixture refs included in the checked-in convergence corpus.
pub fn current_diagnostics_convergence_fixture_refs() -> impl Iterator<Item = &'static str> {
    CURRENT_DIAGNOSTICS_CONVERGENCE_FIXTURES
        .iter()
        .map(|(fixture_ref, _)| *fixture_ref)
}

/// Builds a convergence packet from the checked-in corpus.
pub fn current_diagnostics_convergence_packet(
) -> Result<ProviderArbitrationDiagnosticsConvergencePacket, serde_yaml::Error> {
    let clusters = current_diagnostics_convergence_corpus()?;
    let aggregate_counts =
        ProviderArbitrationDiagnosticsConvergencePacket::build_aggregate_counts(&clusters);
    Ok(ProviderArbitrationDiagnosticsConvergencePacket {
        record_kind: CONVERGENCE_PACKET_RECORD_KIND.to_owned(),
        schema_version: PROVIDER_ARBITRATION_DIAGNOSTICS_CONVERGENCE_SCHEMA_VERSION,
        packet_id: "provider_arbitration_diagnostics_convergence:m4:current".to_owned(),
        captured_at: "2026-06-02T22:18:40Z".to_owned(),
        doc_ref: PROVIDER_ARBITRATION_DIAGNOSTICS_CONVERGENCE_DOC_REF.to_owned(),
        schema_ref: PROVIDER_ARBITRATION_DIAGNOSTICS_CONVERGENCE_SCHEMA_REF.to_owned(),
        clusters,
        aggregate_counts,
        raw_payload_excluded: true,
        raw_private_material_excluded: true,
        export_safe_summary:
            "Provider-arbitration diagnostics convergence packet for M4 stable language intelligence."
                .to_owned(),
    })
}
