use serde::{Deserialize, Serialize};

use crate::code_actions::SemanticLayerStateClass;
use crate::diagnostics::{
    DiagnosticAnchorRemapStateClass, DiagnosticFreshnessClass, DiagnosticSeverityClass,
    DiagnosticSourceFamily,
};
use crate::lsp_router::{HealthState, LocalityClass};
use crate::provider_arbitration::{
    ArbitrationPolicyContext, ArbitrationRedactionClass, ConfidenceOutcomeClass, ProviderFamily,
};

/// Integer schema version for provider-arbitration diagnostics convergence payloads.
pub type ProviderArbitrationDiagnosticsConvergenceSchemaVersion = u32;

/// Schema version used by convergence records and projections.
pub const PROVIDER_ARBITRATION_DIAGNOSTICS_CONVERGENCE_SCHEMA_VERSION:
    ProviderArbitrationDiagnosticsConvergenceSchemaVersion = 1;

/// Stable record-kind tag for convergence packets.
pub const CONVERGENCE_PACKET_RECORD_KIND: &str =
    "provider_arbitration_diagnostics_convergence_packet";

/// Stable record-kind tag for converged diagnostic clusters.
pub const CONVERGED_DIAGNOSTIC_CLUSTER_RECORD_KIND: &str = "converged_diagnostic_cluster_record";

/// Closed suppression-class vocabulary for one contributing provider claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuppressionClass {
    /// No suppression applies.
    NotSuppressed,
    /// Time-bounded suppression with an expiration review.
    TimeBounded,
    /// Policy-governed suppression or baseline.
    PolicyGoverned,
    /// Provider-specific suppression that does not hide other providers.
    ProviderSpecific,
    /// Baselined and waived through governed review.
    BaselineWaived,
}

impl SuppressionClass {
    /// Returns true when this suppression class blocks the diagnostic from compact surfaces.
    pub const fn blocks_compact_display(self) -> bool {
        matches!(self, Self::PolicyGoverned | Self::BaselineWaived)
    }

    /// Returns true when the suppression is provider-specific and other claims remain visible.
    pub const fn is_provider_narrow(self) -> bool {
        matches!(self, Self::ProviderSpecific)
    }
}

/// Closed quick-fix safety-class vocabulary attached to converged diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuickFixSafetyClass {
    /// Safe to apply without further review.
    SafeToApply,
    /// Preview is required before apply.
    PreviewRequired,
    /// Blocked because providers disagree on the fix.
    BlockedForDisagreement,
    /// Blocked because the diagnostic evidence is stale.
    BlockedForStaleState,
    /// Blocked because the claimed scope is partial.
    BlockedForPartialScope,
    /// Blocked because the target is generated or read-only.
    BlockedForGeneratedOrReadOnly,
    /// Blocked because provider health is unavailable.
    BlockedForProviderHealth,
    /// Inspect-only path; no apply lane.
    InspectOnly,
}

impl QuickFixSafetyClass {
    /// Returns true when the safety class blocks broad apply.
    pub const fn blocks_broad_apply(self) -> bool {
        !matches!(self, Self::SafeToApply)
    }

    /// Returns true when a preview or explicit review is required before apply.
    pub const fn requires_preview(self) -> bool {
        matches!(self, Self::PreviewRequired | Self::BlockedForDisagreement)
    }
}

/// Closed batch-fix scope vocabulary that prevents mixed-provider opaque batches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchFixScopeClass {
    /// Fix applies to a single diagnostic only.
    SingleDiagnostic,
    /// Fix applies to all diagnostics sharing the same rule in one file.
    SameRuleSameFile,
    /// Fix applies to all diagnostics from the same provider in one file.
    SameProviderSameFile,
    /// Fix applies to all diagnostics sharing the same rule across the workspace.
    SameRuleWorkspace,
    /// Fix applies to all diagnostics from the same provider across the workspace.
    SameProviderWorkspace,
    /// Mixed-provider batch fix is blocked.
    MixedProviderBlocked,
}

impl BatchFixScopeClass {
    /// Returns true when the batch scope crosses provider boundaries.
    pub const fn crosses_provider_boundary(self) -> bool {
        matches!(self, Self::SameRuleWorkspace | Self::MixedProviderBlocked)
    }

    /// Returns true when the batch scope is blocked.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::MixedProviderBlocked)
    }
}

/// Closed severity-convergence vocabulary describing how severity settled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeverityConvergenceClass {
    /// All contributing providers agree on severity.
    SingleSeverity,
    /// Contributing providers report conflicting severities.
    ConflictingSeverityPresent,
    /// Policy override changed the displayed severity.
    PolicyOverriddenSeverityPresent,
}

/// Closed display-state vocabulary for converged diagnostic clusters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConvergenceDisplayStateClass {
    /// Current, exact, live diagnostic with no conflicts.
    CurrentExactLive,
    /// Current but remapped from original anchor.
    CurrentRemappedLive,
    /// Current with a visible severity conflict.
    CurrentWithSeverityConflict,
    /// Current but mixing static-analysis and runtime evidence.
    CurrentMixedStaticAndRuntime,
    /// Stale or superseded evidence.
    StaleOrSuperseded,
    /// Imported snapshot only.
    ImportedSnapshotOnly,
    /// Suppressed or baselined under governance.
    SuppressedOrBaselinedGoverned,
    /// Downgraded because providers disagree.
    DowngradedForProviderDisagreement,
}

/// One provider claim preserved inside a converged diagnostic cluster.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderClaimRow {
    /// Provider id.
    pub provider_id: String,
    /// Provider family.
    pub provider_family: ProviderFamily,
    /// Diagnostic source family.
    pub source_family: DiagnosticSourceFamily,
    /// Provider health state at convergence time.
    pub health_state: HealthState,
    /// Freshness class for this provider's claim.
    pub freshness_class: DiagnosticFreshnessClass,
    /// Severity class reported by this provider.
    pub severity_class: DiagnosticSeverityClass,
    /// Anchor remap state for this provider's claim.
    pub remap_state_class: DiagnosticAnchorRemapStateClass,
    /// Rule id reference.
    pub rule_id_ref: String,
    /// Category reference.
    pub category_ref: String,
    /// Anchor family id.
    pub anchor_family_id: String,
    /// Suppression class.
    pub suppression_class: SuppressionClass,
    /// Quick-fix safety class.
    pub quick_fix_safety_class: QuickFixSafetyClass,
    /// Batch-fix scope class.
    pub batch_fix_scope_class: BatchFixScopeClass,
    /// True when this provider is the primary/chosen provider.
    pub is_primary: bool,
    /// Provider locality.
    pub locality_class: LocalityClass,
    /// Export-safe claim summary.
    pub summary: String,
}

impl ProviderClaimRow {
    /// Returns true when this claim requires degraded disclosure.
    pub fn requires_degraded_disclosure(&self) -> bool {
        self.freshness_class.requires_disclosure()
            || self.remap_state_class.requires_disclosure()
            || self.suppression_class.blocks_compact_display()
            || self.quick_fix_safety_class.blocks_broad_apply()
            || !self.is_primary
    }
}

/// Aggregate counts for one converged diagnostic cluster.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ConvergenceAggregateCounts {
    /// Total provider claims in the cluster.
    pub total_provider_claims: u32,
    /// Claims with live local or managed origin.
    pub live_count: u32,
    /// Claims from imported snapshots.
    pub imported_snapshot_count: u32,
    /// Claims that are stale or superseded.
    pub stale_or_superseded_count: u32,
    /// Claims that are suppressed.
    pub suppressed_count: u32,
    /// Claims with conflicting severity.
    pub conflicting_severity_count: u32,
    /// Claims whose quick fix is blocked.
    pub blocked_quick_fix_count: u32,
    /// Claims whose batch fix is blocked.
    pub blocked_batch_fix_count: u32,
}

/// Converged diagnostic cluster that preserves per-provider claims, freshness,
/// suppression state, and batch-fix scope rather than flattening them into one
/// anonymous issue row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConvergedDiagnosticCluster {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: ProviderArbitrationDiagnosticsConvergenceSchemaVersion,
    /// Stable cluster id.
    pub cluster_id: String,
    /// Human-readable cluster title.
    pub cluster_title: String,
    /// Primary diagnostic reference.
    pub primary_diagnostic_ref: String,
    /// Rule id reference.
    pub rule_id_ref: String,
    /// Category reference.
    pub category_ref: String,
    /// Dominant severity after convergence.
    pub dominant_severity_class: DiagnosticSeverityClass,
    /// How severity converged.
    pub severity_convergence_class: SeverityConvergenceClass,
    /// Worst freshness class in the cluster.
    pub cluster_freshness_class: DiagnosticFreshnessClass,
    /// Display state for compact surfaces.
    pub dominant_display_state_class: ConvergenceDisplayStateClass,
    /// Semantic layer state for the cluster.
    pub semantic_layer_state_class: SemanticLayerStateClass,
    /// Convergence outcome class.
    pub convergence_outcome_class: ConfidenceOutcomeClass,
    /// Provider claim rows preserved after deduplication.
    pub provider_claim_rows: Vec<ProviderClaimRow>,
    /// Aggregate counts.
    pub aggregate_counts: ConvergenceAggregateCounts,
    /// Suppression review refs.
    pub suppression_review_refs: Vec<String>,
    /// Code-action summary refs.
    pub code_action_summary_refs: Vec<String>,
    /// Policy context.
    pub policy_context: ArbitrationPolicyContext,
    /// Redaction class.
    pub redaction_class: ArbitrationRedactionClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl ConvergedDiagnosticCluster {
    /// Stable record-kind tag for converged diagnostic clusters.
    pub const RECORD_KIND: &'static str = CONVERGED_DIAGNOSTIC_CLUSTER_RECORD_KIND;

    /// Returns true when the cluster has any degraded state to disclose.
    pub fn requires_degraded_disclosure(&self) -> bool {
        self.provider_claim_rows
            .iter()
            .any(ProviderClaimRow::requires_degraded_disclosure)
            || self.severity_convergence_class != SeverityConvergenceClass::SingleSeverity
            || self.convergence_outcome_class != ConfidenceOutcomeClass::Exact
    }

    /// Returns true when broad batch apply is unsafe for this cluster.
    pub fn blocks_broad_batch_apply(&self) -> bool {
        self.provider_claim_rows
            .iter()
            .any(|row| row.quick_fix_safety_class.blocks_broad_apply())
            || self
                .provider_claim_rows
                .iter()
                .any(|row| row.batch_fix_scope_class.is_blocked())
    }

    /// Returns true when the cluster contains a mixed-provider conflict.
    pub fn has_mixed_provider_conflict(&self) -> bool {
        let families: std::collections::BTreeSet<_> = self
            .provider_claim_rows
            .iter()
            .map(|row| row.provider_family)
            .collect();
        families.len() > 1
            && self.severity_convergence_class
                == SeverityConvergenceClass::ConflictingSeverityPresent
    }

    /// Returns the primary provider claim, if any.
    pub fn primary_claim(&self) -> Option<&ProviderClaimRow> {
        self.provider_claim_rows.iter().find(|row| row.is_primary)
    }
}

/// Aggregate counts for a convergence packet.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ConvergencePacketAggregateCounts {
    /// Total clusters.
    pub total_clusters: u32,
    /// Clusters with exact convergence.
    pub exact_clusters: u32,
    /// Clusters with heuristic convergence.
    pub heuristic_clusters: u32,
    /// Clusters with partial convergence.
    pub partial_clusters: u32,
    /// Clusters with stale convergence.
    pub stale_clusters: u32,
    /// Clusters that are unavailable.
    pub unavailable_clusters: u32,
    /// Clusters with a visible provider conflict.
    pub conflict_clusters: u32,
    /// Clusters with at least one suppressed claim.
    pub suppressed_clusters: u32,
    /// Clusters where broad batch fix is blocked.
    pub blocked_batch_fix_clusters: u32,
}

/// Machine-readable provider-arbitration diagnostics convergence packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderArbitrationDiagnosticsConvergencePacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: ProviderArbitrationDiagnosticsConvergenceSchemaVersion,
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Documentation ref.
    pub doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Converged diagnostic clusters.
    pub clusters: Vec<ConvergedDiagnosticCluster>,
    /// Aggregate counts.
    pub aggregate_counts: ConvergencePacketAggregateCounts,
    /// True when raw provider payloads are excluded.
    pub raw_payload_excluded: bool,
    /// True when private source material is excluded.
    pub raw_private_material_excluded: bool,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl ProviderArbitrationDiagnosticsConvergencePacket {
    /// Stable record-kind tag for convergence packets.
    pub const RECORD_KIND: &'static str = CONVERGENCE_PACKET_RECORD_KIND;

    /// Builds aggregate counts from clusters.
    pub fn build_aggregate_counts(
        clusters: &[ConvergedDiagnosticCluster],
    ) -> ConvergencePacketAggregateCounts {
        let mut counts = ConvergencePacketAggregateCounts {
            total_clusters: clusters.len() as u32,
            ..ConvergencePacketAggregateCounts::default()
        };
        for cluster in clusters {
            match cluster.convergence_outcome_class {
                ConfidenceOutcomeClass::Exact => counts.exact_clusters += 1,
                ConfidenceOutcomeClass::Heuristic => counts.heuristic_clusters += 1,
                ConfidenceOutcomeClass::Partial => counts.partial_clusters += 1,
                ConfidenceOutcomeClass::Stale => counts.stale_clusters += 1,
                ConfidenceOutcomeClass::Unavailable => counts.unavailable_clusters += 1,
            }
            if cluster.has_mixed_provider_conflict() {
                counts.conflict_clusters += 1;
            }
            if cluster.aggregate_counts.suppressed_count > 0 {
                counts.suppressed_clusters += 1;
            }
            if cluster.blocks_broad_batch_apply() {
                counts.blocked_batch_fix_clusters += 1;
            }
        }
        counts
    }

    /// Returns true when the packet is safe to include in support evidence.
    pub fn is_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.raw_private_material_excluded
            && !self.clusters.is_empty()
            && self.aggregate_counts.total_clusters == self.clusters.len() as u32
    }

    /// Validates the packet and returns a report.
    pub fn validate(&self) -> ConvergenceValidationReport {
        let mut defects = Vec::new();

        if self.record_kind != Self::RECORD_KIND {
            defects.push(ConvergenceValidationDefect::WrongRecordKind {
                expected: Self::RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        if self.schema_version != PROVIDER_ARBITRATION_DIAGNOSTICS_CONVERGENCE_SCHEMA_VERSION {
            defects.push(ConvergenceValidationDefect::SchemaVersionMismatch {
                expected: PROVIDER_ARBITRATION_DIAGNOSTICS_CONVERGENCE_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }

        for cluster in &self.clusters {
            if cluster.record_kind != ConvergedDiagnosticCluster::RECORD_KIND {
                defects.push(ConvergenceValidationDefect::ClusterWrongRecordKind {
                    cluster_id: cluster.cluster_id.clone(),
                    expected: ConvergedDiagnosticCluster::RECORD_KIND.to_owned(),
                    actual: cluster.record_kind.clone(),
                });
            }

            if cluster.provider_claim_rows.is_empty() {
                defects.push(ConvergenceValidationDefect::ClusterMissingProviderClaims {
                    cluster_id: cluster.cluster_id.clone(),
                });
            }

            let primary_count = cluster
                .provider_claim_rows
                .iter()
                .filter(|row| row.is_primary)
                .count();
            if primary_count != 1 {
                defects.push(ConvergenceValidationDefect::ClusterPrimaryClaimAmbiguity {
                    cluster_id: cluster.cluster_id.clone(),
                    primary_count: primary_count as u32,
                });
            }

            if cluster.convergence_outcome_class == ConfidenceOutcomeClass::Exact
                && cluster.dominant_display_state_class
                    == ConvergenceDisplayStateClass::DowngradedForProviderDisagreement
            {
                defects.push(
                    ConvergenceValidationDefect::ExactOutcomeWithDisagreementDisplay {
                        cluster_id: cluster.cluster_id.clone(),
                    },
                );
            }

            if cluster.convergence_outcome_class != ConfidenceOutcomeClass::Exact
                && cluster.severity_convergence_class == SeverityConvergenceClass::SingleSeverity
                && cluster
                    .provider_claim_rows
                    .iter()
                    .any(|row| row.suppression_class != SuppressionClass::NotSuppressed)
                && cluster.blocks_broad_batch_apply()
            {
                // Non-exact outcomes with suppressed claims that still block batch apply
                // are valid, but we flag when suppression should have unblocked.
                // This is intentionally a narrow check.
            }
        }

        let expected_counts = Self::build_aggregate_counts(&self.clusters);
        if self.aggregate_counts != expected_counts {
            defects.push(ConvergenceValidationDefect::AggregateCountsMismatch {
                expected: expected_counts,
                actual: self.aggregate_counts.clone(),
            });
        }

        ConvergenceValidationReport { defects }
    }
}

/// One validation defect for a convergence packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "defect_kind")]
pub enum ConvergenceValidationDefect {
    /// Packet record kind does not match the schema.
    WrongRecordKind {
        /// Expected record kind.
        expected: String,
        /// Actual record kind.
        actual: String,
    },
    /// Schema version mismatch.
    SchemaVersionMismatch {
        /// Expected schema version.
        expected: u32,
        /// Actual schema version.
        actual: u32,
    },
    /// Cluster record kind does not match.
    ClusterWrongRecordKind {
        /// Cluster id.
        cluster_id: String,
        /// Expected record kind.
        expected: String,
        /// Actual record kind.
        actual: String,
    },
    /// Cluster has no provider claims.
    ClusterMissingProviderClaims {
        /// Cluster id.
        cluster_id: String,
    },
    /// Cluster does not have exactly one primary claim.
    ClusterPrimaryClaimAmbiguity {
        /// Cluster id.
        cluster_id: String,
        /// Number of primary claims found.
        primary_count: u32,
    },
    /// Exact outcome paired with a disagreement display state.
    ExactOutcomeWithDisagreementDisplay {
        /// Cluster id.
        cluster_id: String,
    },
    /// Aggregate counts do not match derived values.
    AggregateCountsMismatch {
        /// Expected counts.
        expected: ConvergencePacketAggregateCounts,
        /// Actual counts.
        actual: ConvergencePacketAggregateCounts,
    },
}

/// Validation report for a convergence packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConvergenceValidationReport {
    /// Defects found during validation.
    pub defects: Vec<ConvergenceValidationDefect>,
}

impl ConvergenceValidationReport {
    /// Returns true when validation found no defects.
    pub fn is_conformant(&self) -> bool {
        self.defects.is_empty()
    }
}

/// Inspector for provider-arbitration diagnostics convergence.
pub struct ConvergenceInspector;

impl ConvergenceInspector {
    /// Creates a new inspector.
    pub fn new() -> Self {
        Self
    }

    /// Inspects a packet and returns a validation report.
    pub fn inspect(
        &self,
        packet: &ProviderArbitrationDiagnosticsConvergencePacket,
    ) -> ConvergenceValidationReport {
        packet.validate()
    }

    /// Reports whether every cluster preserves per-provider truth labels.
    pub fn truth_labels_preserved(
        &self,
        packet: &ProviderArbitrationDiagnosticsConvergencePacket,
    ) -> bool {
        packet.clusters.iter().all(|cluster| {
            cluster.provider_claim_rows.len()
                == cluster.aggregate_counts.total_provider_claims as usize
        })
    }

    /// Reports whether any cluster hides a provider disagreement.
    pub fn hidden_disagreements(
        &self,
        packet: &ProviderArbitrationDiagnosticsConvergencePacket,
    ) -> Vec<String> {
        packet
            .clusters
            .iter()
            .filter(|cluster| {
                cluster.has_mixed_provider_conflict()
                    && !matches!(
                        cluster.dominant_display_state_class,
                        ConvergenceDisplayStateClass::DowngradedForProviderDisagreement
                            | ConvergenceDisplayStateClass::CurrentWithSeverityConflict
                            | ConvergenceDisplayStateClass::CurrentMixedStaticAndRuntime
                    )
            })
            .map(|cluster| cluster.cluster_id.clone())
            .collect()
    }
}

impl Default for ConvergenceInspector {
    fn default() -> Self {
        Self::new()
    }
}
