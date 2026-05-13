use crate::diagnostics::{
    DiagnosticAnchor, DiagnosticBus, DiagnosticBusSnapshotRequest, DiagnosticEnvelope,
    DiagnosticEvidenceRef, DiagnosticEvidenceRoleClass, DiagnosticFreshness, DiagnosticScope,
    DiagnosticSourceDescriptor, DIAGNOSTIC_BUS_SCHEMA_VERSION,
};
use crate::lsp_router::{FreshnessClass, HealthState, RedactionClass, SupportClass};

use super::super::records::{
    PythonInterpreterReadinessClass, PythonInterpreterSelectionStateClass, PythonWorkspaceContext,
};
use super::records::{
    PythonQualityAggregateCounts, PythonQualityDiagnosticSeed, PythonQualityExecutionTaskHook,
    PythonQualityPreviewRequirementClass, PythonQualityRerunPostureClass,
    PythonQualitySeedSnapshot, PythonQualitySnapshot, PythonQualitySnapshotRequest,
    PythonQualityTaskHookSeed, PythonQualityToolStatusRow, PYTHON_QUALITY_ALPHA_SCHEMA_VERSION,
};

/// Fixture-backed Python formatter, linter, and test-adapter quality alpha.
#[derive(Debug, Clone)]
pub struct PythonQualityWedge {
    seed: PythonQualitySeedSnapshot,
}

impl PythonQualityWedge {
    /// Builds a Python quality surface from a protected seed snapshot.
    pub fn new(seed: PythonQualitySeedSnapshot) -> Self {
        Self { seed }
    }

    /// Returns the protected seed backing this quality surface.
    pub const fn seed(&self) -> &PythonQualitySeedSnapshot {
        &self.seed
    }

    /// Builds a deterministic snapshot with normalized diagnostics and task hooks.
    pub fn snapshot(&self, request: PythonQualitySnapshotRequest) -> PythonQualitySnapshot {
        let mut bus = DiagnosticBus::new();
        for tool in &self.seed.tool_rows {
            bus.ingest_provider_availability(tool.diagnostic_provider_row());
        }
        for diagnostic_seed in &self.seed.diagnostic_seeds {
            bus.publish(self.diagnostic_from_seed(diagnostic_seed, &request));
        }

        let diagnostic_bus_snapshot = bus.snapshot(DiagnosticBusSnapshotRequest {
            snapshot_id: format!("diagnostic_bus:{}", request.snapshot_id),
            workspace_id: self.seed.workspace_context.workspace_id.clone(),
            collection_id: request.collection_id.clone(),
            captured_at: request.captured_at.clone(),
        });
        let task_hooks = self
            .seed
            .task_hook_seeds
            .iter()
            .map(|hook_seed| self.task_hook_from_seed(hook_seed, &request))
            .collect::<Vec<_>>();
        let aggregate_counts = PythonQualityAggregateCounts::from_parts(
            &diagnostic_bus_snapshot,
            &self.seed.tool_rows,
            &task_hooks,
        );

        PythonQualitySnapshot {
            record_kind: PythonQualitySnapshot::RECORD_KIND.into(),
            schema_version: PYTHON_QUALITY_ALPHA_SCHEMA_VERSION,
            snapshot_id: request.snapshot_id,
            language_id: self.seed.language_id.clone(),
            workspace_context: self.seed.workspace_context.clone(),
            effective_quality_profile_ref: self.seed.effective_quality_profile_ref.clone(),
            quality_profile_summary: self.seed.quality_profile_summary.clone(),
            diagnostic_bus_snapshot,
            tool_rows: self.seed.tool_rows.clone(),
            task_hooks,
            aggregate_counts,
            redaction_class: RedactionClass::MetadataSafeDefault,
            captured_at: request.captured_at,
            export_safe_summary: format!(
                "Python quality snapshot preserves formatter, linter, test-adapter, and interpreter provenance for {}.",
                self.seed.workspace_context.scope_label
            ),
        }
    }

    fn diagnostic_from_seed(
        &self,
        seed: &PythonQualityDiagnosticSeed,
        request: &PythonQualitySnapshotRequest,
    ) -> DiagnosticEnvelope {
        DiagnosticEnvelope {
            record_kind: DiagnosticEnvelope::RECORD_KIND.into(),
            diagnostic_bus_schema_version: DIAGNOSTIC_BUS_SCHEMA_VERSION,
            diagnostic_id: seed.diagnostic_id.clone(),
            collection_id: request.collection_id.clone(),
            workspace_id: self.seed.workspace_context.workspace_id.clone(),
            source: DiagnosticSourceDescriptor {
                source_descriptor_id: seed.source_descriptor_id.clone(),
                source_family: seed.source_family,
                evidence_plane_class: seed.evidence_plane_class,
                origin_class: seed.origin_class,
                producer_ref: seed.producer_ref.clone(),
                producer_version_ref: seed.producer_version_ref.clone(),
                provider_id: seed.provider_id.clone(),
                router_host_ref: None,
                locality_class: seed.locality_class,
                support_class: seed.support_class,
                summary: seed.source_summary.clone(),
            },
            severity_class: seed.severity_class,
            rule_id_ref: seed.rule_id_ref.clone(),
            category_ref: seed.category_ref.clone(),
            freshness: DiagnosticFreshness {
                freshness_class: seed.freshness_class,
                observed_at: request.captured_at.clone(),
                epoch_ref: seed.epoch_ref.clone(),
                invalidation_ref: seed.invalidation_ref.clone(),
                summary: seed.freshness_summary.clone(),
            },
            scope: DiagnosticScope {
                scope_claim_class: seed.scope_claim_class,
                completeness_class: seed.completeness_class,
                scope_limit_classes: seed.scope_limit_classes.clone(),
                target_ref: seed.target_ref.clone(),
                execution_context_id: self.seed.workspace_context.execution_context_id.clone(),
                summary: seed.scope_summary.clone(),
            },
            anchor: DiagnosticAnchor {
                anchor_family_id: seed.anchor_family_id.clone(),
                current_anchor_ref: seed.current_anchor_ref.clone(),
                path_ref: seed.path_ref.clone(),
                remap_state_class: seed.remap_state_class,
                summary: seed.anchor_summary.clone(),
            },
            evidence_refs: vec![DiagnosticEvidenceRef {
                evidence_ref: format!(
                    "evidence:python-quality:{}",
                    sanitize_id(&seed.diagnostic_id)
                ),
                evidence_role_class: DiagnosticEvidenceRoleClass::PrimarySource,
                summary: format!(
                    "{} diagnostic is normalized from the Python quality adapter.",
                    seed.diagnostic_id
                ),
            }],
            provider_status_refs: seed.provider_id.iter().cloned().collect(),
            router_decision_ref: None,
            redaction_class: RedactionClass::MetadataSafeDefault,
            captured_at: request.captured_at.clone(),
            export_safe_summary: seed.diagnostic_summary.clone(),
        }
    }

    fn task_hook_from_seed(
        &self,
        seed: &PythonQualityTaskHookSeed,
        request: &PythonQualitySnapshotRequest,
    ) -> PythonQualityExecutionTaskHook {
        let tool = self.seed.tool(&seed.provider_id);
        let rerun_posture_class = rerun_posture(
            tool,
            seed.preview_requirement_class,
            &self.seed.workspace_context,
        );
        let provider_health_class = tool
            .map(|tool| tool.health_state)
            .unwrap_or(HealthState::Unavailable);
        let provider_freshness_class = tool
            .map(|tool| tool.freshness_class)
            .unwrap_or(FreshnessClass::Unverified);
        let support_class = tool
            .map(|tool| tool.support_class)
            .unwrap_or(SupportClass::Unsupported);
        let interpreter_context = &self.seed.workspace_context.interpreter_context;

        PythonQualityExecutionTaskHook {
            record_kind: PythonQualityExecutionTaskHook::RECORD_KIND.into(),
            schema_version: PYTHON_QUALITY_ALPHA_SCHEMA_VERSION,
            task_hook_id: seed.task_hook_id.clone(),
            tool_kind_class: seed.tool_kind_class,
            action_class: seed.action_class,
            trigger_class: seed.trigger_class,
            provider_id: seed.provider_id.clone(),
            canonical_command_id: seed.canonical_command_id.clone(),
            canonical_verb: seed.canonical_verb.clone(),
            execution_context_id: self.seed.workspace_context.execution_context_id.clone(),
            interpreter_ref: interpreter_context.interpreter_ref.clone(),
            interpreter_selection_state_class: interpreter_context.selection_state_class,
            interpreter_readiness_class: interpreter_context.readiness_class,
            target_ref: seed.target_ref.clone(),
            build_target_id: seed.build_target_id.clone(),
            diagnostic_collection_id: request.collection_id.clone(),
            source_diagnostic_refs: seed.source_diagnostic_refs.clone(),
            task_event_trace_ref: seed.task_event_trace_ref.clone(),
            normalized_task_event_refs: seed.normalized_task_event_refs.clone(),
            rerun_posture_class,
            preview_requirement_class: seed.preview_requirement_class,
            safety_class: seed.safety_class,
            provider_health_class,
            provider_freshness_class,
            support_class,
            redaction_class: RedactionClass::MetadataSafeDefault,
            captured_at: request.captured_at.clone(),
            export_safe_summary: format!(
                "{} Rerun posture is {:?}.",
                seed.summary, rerun_posture_class
            ),
        }
    }
}

fn rerun_posture(
    tool: Option<&PythonQualityToolStatusRow>,
    preview_requirement_class: PythonQualityPreviewRequirementClass,
    context: &PythonWorkspaceContext,
) -> PythonQualityRerunPostureClass {
    if interpreter_blocks_rerun(context) {
        return PythonQualityRerunPostureClass::BlockedInterpreterUnavailable;
    }

    let Some(tool) = tool else {
        return PythonQualityRerunPostureClass::BlockedToolUnavailable;
    };

    if tool.health_state == HealthState::PolicyBlocked {
        PythonQualityRerunPostureClass::BlockedByPolicy
    } else if tool.is_unavailable() {
        PythonQualityRerunPostureClass::BlockedToolUnavailable
    } else if tool.support_class == SupportClass::InspectOnly {
        PythonQualityRerunPostureClass::InspectOnlyEvidence
    } else if preview_requirement_class.requires_preview() {
        PythonQualityRerunPostureClass::RerunnableAfterPreviewReview
    } else {
        PythonQualityRerunPostureClass::RerunnableFromExecutionPlane
    }
}

fn interpreter_blocks_rerun(context: &PythonWorkspaceContext) -> bool {
    context.interpreter_context.selection_state_class
        != PythonInterpreterSelectionStateClass::Selected
        || context.interpreter_context.readiness_class
            == PythonInterpreterReadinessClass::Unavailable
}

fn sanitize_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}
