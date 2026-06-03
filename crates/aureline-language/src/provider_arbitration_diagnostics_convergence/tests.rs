use super::*;

use crate::code_actions::SemanticLayerStateClass;
use crate::diagnostics::{
    DiagnosticAnchorRemapStateClass, DiagnosticFreshnessClass, DiagnosticSeverityClass,
    DiagnosticSourceFamily,
};
use crate::lsp_router::{HealthState, LocalityClass};
use crate::provider_arbitration::{
    ArbitrationPolicyContext, ArbitrationRedactionClass, ConfidenceOutcomeClass, ProviderFamily,
};

#[test]
fn quick_fix_safety_blocks_broad_apply() {
    assert!(!QuickFixSafetyClass::SafeToApply.blocks_broad_apply());
    assert!(QuickFixSafetyClass::PreviewRequired.blocks_broad_apply());
    assert!(QuickFixSafetyClass::BlockedForDisagreement.blocks_broad_apply());
    assert!(QuickFixSafetyClass::BlockedForStaleState.blocks_broad_apply());
    assert!(QuickFixSafetyClass::BlockedForPartialScope.blocks_broad_apply());
    assert!(QuickFixSafetyClass::BlockedForGeneratedOrReadOnly.blocks_broad_apply());
    assert!(QuickFixSafetyClass::BlockedForProviderHealth.blocks_broad_apply());
    assert!(QuickFixSafetyClass::InspectOnly.blocks_broad_apply());
}

#[test]
fn batch_fix_scope_crosses_provider_boundary() {
    assert!(!BatchFixScopeClass::SingleDiagnostic.crosses_provider_boundary());
    assert!(!BatchFixScopeClass::SameRuleSameFile.crosses_provider_boundary());
    assert!(!BatchFixScopeClass::SameProviderSameFile.crosses_provider_boundary());
    assert!(BatchFixScopeClass::SameRuleWorkspace.crosses_provider_boundary());
    assert!(!BatchFixScopeClass::SameProviderWorkspace.crosses_provider_boundary());
    assert!(BatchFixScopeClass::MixedProviderBlocked.crosses_provider_boundary());
}

#[test]
fn suppression_blocks_compact_display() {
    assert!(!SuppressionClass::NotSuppressed.blocks_compact_display());
    assert!(!SuppressionClass::TimeBounded.blocks_compact_display());
    assert!(SuppressionClass::PolicyGoverned.blocks_compact_display());
    assert!(!SuppressionClass::ProviderSpecific.blocks_compact_display());
    assert!(SuppressionClass::BaselineWaived.blocks_compact_display());
}

#[test]
fn cluster_requires_degraded_disclosure_for_exact() {
    let cluster = ConvergedDiagnosticCluster {
        record_kind: CONVERGED_DIAGNOSTIC_CLUSTER_RECORD_KIND.to_owned(),
        schema_version: PROVIDER_ARBITRATION_DIAGNOSTICS_CONVERGENCE_SCHEMA_VERSION,
        cluster_id: "cluster:test:exact".to_owned(),
        cluster_title: "Test exact cluster".to_owned(),
        primary_diagnostic_ref: "diag:1".to_owned(),
        rule_id_ref: "rule:test".to_owned(),
        category_ref: "category:test".to_owned(),
        dominant_severity_class: DiagnosticSeverityClass::Error,
        severity_convergence_class: SeverityConvergenceClass::SingleSeverity,
        cluster_freshness_class: DiagnosticFreshnessClass::Current,
        dominant_display_state_class: ConvergenceDisplayStateClass::CurrentExactLive,
        semantic_layer_state_class: SemanticLayerStateClass::SemanticCurrentExact,
        convergence_outcome_class: ConfidenceOutcomeClass::Exact,
        provider_claim_rows: vec![ProviderClaimRow {
            provider_id: "provider:lsp".to_owned(),
            provider_family: ProviderFamily::LanguageServer,
            source_family: DiagnosticSourceFamily::LanguageServer,
            health_state: HealthState::Ready,
            freshness_class: DiagnosticFreshnessClass::Current,
            severity_class: DiagnosticSeverityClass::Error,
            remap_state_class: DiagnosticAnchorRemapStateClass::Exact,
            rule_id_ref: "rule:test".to_owned(),
            category_ref: "category:test".to_owned(),
            anchor_family_id: "anchor:1".to_owned(),
            suppression_class: SuppressionClass::NotSuppressed,
            quick_fix_safety_class: QuickFixSafetyClass::SafeToApply,
            batch_fix_scope_class: BatchFixScopeClass::SameRuleSameFile,
            is_primary: true,
            locality_class: LocalityClass::LocalInProcess,
            summary: "Primary LSP claim.".to_owned(),
        }],
        aggregate_counts: ConvergenceAggregateCounts {
            total_provider_claims: 1,
            live_count: 1,
            ..ConvergenceAggregateCounts::default()
        },
        suppression_review_refs: Vec::new(),
        code_action_summary_refs: Vec::new(),
        policy_context: ArbitrationPolicyContext {
            policy_epoch: "epoch:1".to_owned(),
            trust_state: crate::provider_arbitration::ArbitrationTrustState::Trusted,
            execution_context_id: "ctx:1".to_owned(),
        },
        redaction_class: ArbitrationRedactionClass::MetadataSafeDefault,
        captured_at: "2026-06-02T22:18:40Z".to_owned(),
        export_safe_summary: "Test cluster.".to_owned(),
    };
    assert!(!cluster.requires_degraded_disclosure());
    assert!(!cluster.blocks_broad_batch_apply());
    assert!(!cluster.has_mixed_provider_conflict());
}

#[test]
fn cluster_blocks_broad_batch_for_disagreement() {
    let cluster = ConvergedDiagnosticCluster {
        record_kind: CONVERGED_DIAGNOSTIC_CLUSTER_RECORD_KIND.to_owned(),
        schema_version: PROVIDER_ARBITRATION_DIAGNOSTICS_CONVERGENCE_SCHEMA_VERSION,
        cluster_id: "cluster:test:disagree".to_owned(),
        cluster_title: "Test disagreement cluster".to_owned(),
        primary_diagnostic_ref: "diag:2".to_owned(),
        rule_id_ref: "rule:test".to_owned(),
        category_ref: "category:test".to_owned(),
        dominant_severity_class: DiagnosticSeverityClass::Warning,
        severity_convergence_class: SeverityConvergenceClass::ConflictingSeverityPresent,
        cluster_freshness_class: DiagnosticFreshnessClass::Current,
        dominant_display_state_class: ConvergenceDisplayStateClass::CurrentWithSeverityConflict,
        semantic_layer_state_class: SemanticLayerStateClass::SemanticCurrentExact,
        convergence_outcome_class: ConfidenceOutcomeClass::Heuristic,
        provider_claim_rows: vec![
            ProviderClaimRow {
                provider_id: "provider:lsp".to_owned(),
                provider_family: ProviderFamily::LanguageServer,
                source_family: DiagnosticSourceFamily::LanguageServer,
                health_state: HealthState::Ready,
                freshness_class: DiagnosticFreshnessClass::Current,
                severity_class: DiagnosticSeverityClass::Error,
                remap_state_class: DiagnosticAnchorRemapStateClass::Exact,
                rule_id_ref: "rule:test".to_owned(),
                category_ref: "category:test".to_owned(),
                anchor_family_id: "anchor:2".to_owned(),
                suppression_class: SuppressionClass::NotSuppressed,
                quick_fix_safety_class: QuickFixSafetyClass::SafeToApply,
                batch_fix_scope_class: BatchFixScopeClass::SameRuleSameFile,
                is_primary: true,
                locality_class: LocalityClass::LocalInProcess,
                summary: "Primary LSP claim.".to_owned(),
            },
            ProviderClaimRow {
                provider_id: "provider:compiler".to_owned(),
                provider_family: ProviderFamily::Syntax,
                source_family: DiagnosticSourceFamily::CompilerOrBuild,
                health_state: HealthState::Ready,
                freshness_class: DiagnosticFreshnessClass::Current,
                severity_class: DiagnosticSeverityClass::Warning,
                remap_state_class: DiagnosticAnchorRemapStateClass::Exact,
                rule_id_ref: "rule:test".to_owned(),
                category_ref: "category:test".to_owned(),
                anchor_family_id: "anchor:2".to_owned(),
                suppression_class: SuppressionClass::NotSuppressed,
                quick_fix_safety_class: QuickFixSafetyClass::BlockedForDisagreement,
                batch_fix_scope_class: BatchFixScopeClass::MixedProviderBlocked,
                is_primary: false,
                locality_class: LocalityClass::LocalInProcess,
                summary: "Compiler claim disagrees on severity.".to_owned(),
            },
        ],
        aggregate_counts: ConvergenceAggregateCounts {
            total_provider_claims: 2,
            live_count: 2,
            conflicting_severity_count: 1,
            blocked_quick_fix_count: 1,
            blocked_batch_fix_count: 1,
            ..ConvergenceAggregateCounts::default()
        },
        suppression_review_refs: Vec::new(),
        code_action_summary_refs: Vec::new(),
        policy_context: ArbitrationPolicyContext {
            policy_epoch: "epoch:1".to_owned(),
            trust_state: crate::provider_arbitration::ArbitrationTrustState::Trusted,
            execution_context_id: "ctx:1".to_owned(),
        },
        redaction_class: ArbitrationRedactionClass::MetadataSafeDefault,
        captured_at: "2026-06-02T22:18:40Z".to_owned(),
        export_safe_summary: "Test disagreement cluster.".to_owned(),
    };
    assert!(cluster.requires_degraded_disclosure());
    assert!(cluster.blocks_broad_batch_apply());
    assert!(cluster.has_mixed_provider_conflict());
}

#[test]
fn packet_validation_is_conformant_for_valid_packet() {
    let cluster = ConvergedDiagnosticCluster {
        record_kind: CONVERGED_DIAGNOSTIC_CLUSTER_RECORD_KIND.to_owned(),
        schema_version: PROVIDER_ARBITRATION_DIAGNOSTICS_CONVERGENCE_SCHEMA_VERSION,
        cluster_id: "cluster:test:valid".to_owned(),
        cluster_title: "Valid cluster".to_owned(),
        primary_diagnostic_ref: "diag:1".to_owned(),
        rule_id_ref: "rule:test".to_owned(),
        category_ref: "category:test".to_owned(),
        dominant_severity_class: DiagnosticSeverityClass::Error,
        severity_convergence_class: SeverityConvergenceClass::SingleSeverity,
        cluster_freshness_class: DiagnosticFreshnessClass::Current,
        dominant_display_state_class: ConvergenceDisplayStateClass::CurrentExactLive,
        semantic_layer_state_class: SemanticLayerStateClass::SemanticCurrentExact,
        convergence_outcome_class: ConfidenceOutcomeClass::Exact,
        provider_claim_rows: vec![ProviderClaimRow {
            provider_id: "provider:lsp".to_owned(),
            provider_family: ProviderFamily::LanguageServer,
            source_family: DiagnosticSourceFamily::LanguageServer,
            health_state: HealthState::Ready,
            freshness_class: DiagnosticFreshnessClass::Current,
            severity_class: DiagnosticSeverityClass::Error,
            remap_state_class: DiagnosticAnchorRemapStateClass::Exact,
            rule_id_ref: "rule:test".to_owned(),
            category_ref: "category:test".to_owned(),
            anchor_family_id: "anchor:1".to_owned(),
            suppression_class: SuppressionClass::NotSuppressed,
            quick_fix_safety_class: QuickFixSafetyClass::SafeToApply,
            batch_fix_scope_class: BatchFixScopeClass::SameRuleSameFile,
            is_primary: true,
            locality_class: LocalityClass::LocalInProcess,
            summary: "Primary claim.".to_owned(),
        }],
        aggregate_counts: ConvergenceAggregateCounts {
            total_provider_claims: 1,
            live_count: 1,
            ..ConvergenceAggregateCounts::default()
        },
        suppression_review_refs: Vec::new(),
        code_action_summary_refs: Vec::new(),
        policy_context: ArbitrationPolicyContext {
            policy_epoch: "epoch:1".to_owned(),
            trust_state: crate::provider_arbitration::ArbitrationTrustState::Trusted,
            execution_context_id: "ctx:1".to_owned(),
        },
        redaction_class: ArbitrationRedactionClass::MetadataSafeDefault,
        captured_at: "2026-06-02T22:18:40Z".to_owned(),
        export_safe_summary: "Valid cluster.".to_owned(),
    };

    let packet = ProviderArbitrationDiagnosticsConvergencePacket {
        record_kind: CONVERGENCE_PACKET_RECORD_KIND.to_owned(),
        schema_version: PROVIDER_ARBITRATION_DIAGNOSTICS_CONVERGENCE_SCHEMA_VERSION,
        packet_id: "packet:test:valid".to_owned(),
        captured_at: "2026-06-02T22:18:40Z".to_owned(),
        doc_ref: PROVIDER_ARBITRATION_DIAGNOSTICS_CONVERGENCE_DOC_REF.to_owned(),
        schema_ref: PROVIDER_ARBITRATION_DIAGNOSTICS_CONVERGENCE_SCHEMA_REF.to_owned(),
        aggregate_counts: ProviderArbitrationDiagnosticsConvergencePacket::build_aggregate_counts(
            &[cluster.clone()],
        ),
        clusters: vec![cluster],
        raw_payload_excluded: true,
        raw_private_material_excluded: true,
        export_safe_summary: "Valid test packet.".to_owned(),
    };

    let report = packet.validate();
    assert!(report.is_conformant());
    assert!(packet.is_export_safe());
}

#[test]
fn inspector_detects_hidden_disagreements() {
    let cluster = ConvergedDiagnosticCluster {
        record_kind: CONVERGED_DIAGNOSTIC_CLUSTER_RECORD_KIND.to_owned(),
        schema_version: PROVIDER_ARBITRATION_DIAGNOSTICS_CONVERGENCE_SCHEMA_VERSION,
        cluster_id: "cluster:test:hidden".to_owned(),
        cluster_title: "Hidden disagreement".to_owned(),
        primary_diagnostic_ref: "diag:3".to_owned(),
        rule_id_ref: "rule:test".to_owned(),
        category_ref: "category:test".to_owned(),
        dominant_severity_class: DiagnosticSeverityClass::Error,
        severity_convergence_class: SeverityConvergenceClass::ConflictingSeverityPresent,
        cluster_freshness_class: DiagnosticFreshnessClass::Current,
        dominant_display_state_class: ConvergenceDisplayStateClass::CurrentExactLive,
        semantic_layer_state_class: SemanticLayerStateClass::SemanticCurrentExact,
        convergence_outcome_class: ConfidenceOutcomeClass::Heuristic,
        provider_claim_rows: vec![
            ProviderClaimRow {
                provider_id: "provider:lsp".to_owned(),
                provider_family: ProviderFamily::LanguageServer,
                source_family: DiagnosticSourceFamily::LanguageServer,
                health_state: HealthState::Ready,
                freshness_class: DiagnosticFreshnessClass::Current,
                severity_class: DiagnosticSeverityClass::Error,
                remap_state_class: DiagnosticAnchorRemapStateClass::Exact,
                rule_id_ref: "rule:test".to_owned(),
                category_ref: "category:test".to_owned(),
                anchor_family_id: "anchor:3".to_owned(),
                suppression_class: SuppressionClass::NotSuppressed,
                quick_fix_safety_class: QuickFixSafetyClass::SafeToApply,
                batch_fix_scope_class: BatchFixScopeClass::SameRuleSameFile,
                is_primary: true,
                locality_class: LocalityClass::LocalInProcess,
                summary: "LSP claim.".to_owned(),
            },
            ProviderClaimRow {
                provider_id: "provider:compiler".to_owned(),
                provider_family: ProviderFamily::Syntax,
                source_family: DiagnosticSourceFamily::CompilerOrBuild,
                health_state: HealthState::Ready,
                freshness_class: DiagnosticFreshnessClass::Current,
                severity_class: DiagnosticSeverityClass::Warning,
                remap_state_class: DiagnosticAnchorRemapStateClass::Exact,
                rule_id_ref: "rule:test".to_owned(),
                category_ref: "category:test".to_owned(),
                anchor_family_id: "anchor:3".to_owned(),
                suppression_class: SuppressionClass::NotSuppressed,
                quick_fix_safety_class: QuickFixSafetyClass::SafeToApply,
                batch_fix_scope_class: BatchFixScopeClass::SameRuleSameFile,
                is_primary: false,
                locality_class: LocalityClass::LocalInProcess,
                summary: "Compiler claim.".to_owned(),
            },
        ],
        aggregate_counts: ConvergenceAggregateCounts {
            total_provider_claims: 2,
            live_count: 2,
            conflicting_severity_count: 1,
            ..ConvergenceAggregateCounts::default()
        },
        suppression_review_refs: Vec::new(),
        code_action_summary_refs: Vec::new(),
        policy_context: ArbitrationPolicyContext {
            policy_epoch: "epoch:1".to_owned(),
            trust_state: crate::provider_arbitration::ArbitrationTrustState::Trusted,
            execution_context_id: "ctx:1".to_owned(),
        },
        redaction_class: ArbitrationRedactionClass::MetadataSafeDefault,
        captured_at: "2026-06-02T22:18:40Z".to_owned(),
        export_safe_summary: "Hidden disagreement cluster.".to_owned(),
    };

    let packet = ProviderArbitrationDiagnosticsConvergencePacket {
        record_kind: CONVERGENCE_PACKET_RECORD_KIND.to_owned(),
        schema_version: PROVIDER_ARBITRATION_DIAGNOSTICS_CONVERGENCE_SCHEMA_VERSION,
        packet_id: "packet:test:hidden".to_owned(),
        captured_at: "2026-06-02T22:18:40Z".to_owned(),
        doc_ref: PROVIDER_ARBITRATION_DIAGNOSTICS_CONVERGENCE_DOC_REF.to_owned(),
        schema_ref: PROVIDER_ARBITRATION_DIAGNOSTICS_CONVERGENCE_SCHEMA_REF.to_owned(),
        aggregate_counts: ProviderArbitrationDiagnosticsConvergencePacket::build_aggregate_counts(
            &[cluster.clone()],
        ),
        clusters: vec![cluster],
        raw_payload_excluded: true,
        raw_private_material_excluded: true,
        export_safe_summary: "Hidden disagreement packet.".to_owned(),
    };

    let inspector = ConvergenceInspector::new();
    let hidden = inspector.hidden_disagreements(&packet);
    assert_eq!(hidden.len(), 1);
    assert_eq!(hidden[0], "cluster:test:hidden");
}
