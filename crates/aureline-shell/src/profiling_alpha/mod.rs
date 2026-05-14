//! First shell projection for profile, trace-bundle, replay, and comparison truth.
//!
//! The surface consumes [`aureline_runtime::RuntimeEvidenceAlphaPacket`] and
//! projects it into rows a profiler/support pane can render without inventing
//! local mapping-quality, replay, redaction, retention, or comparison labels.

use serde::{Deserialize, Serialize};

use aureline_runtime::{
    ComparisonClass, ReplayLaneState, RuntimeEvidenceAlphaPacket, RuntimeEvidenceSupportExport,
    TraceRedactionMode, TraceRetentionClass,
};

/// Stable record-kind tag for [`ProfilingTraceReplayAlphaSurface`].
pub const PROFILING_TRACE_REPLAY_ALPHA_SURFACE_RECORD_KIND: &str =
    "profiling_trace_replay_alpha_surface";

/// Schema version for the shell projection record.
pub const PROFILING_TRACE_REPLAY_ALPHA_SURFACE_SCHEMA_VERSION: u32 = 1;

/// Scope notice shown by first consumers so the import/view-only lane is not overstated.
pub const PROFILING_TRACE_REPLAY_ALPHA_SCOPE_NOTICE: &str =
    "Profile, trace-bundle, replay-capability, and comparison alpha surface. \
     Replay is import/view-only in this baseline; flamegraph, call-tree, timeline, and \
     support/export actions inspect evidence but do not control live reverse execution.";

/// Stable actions exposed by the alpha profiling projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfilingTraceActionKind {
    /// Opens the derived flamegraph view.
    OpenFlamegraph,
    /// Opens the trace-bundle manifest.
    OpenTraceManifest,
    /// Opens the support/export projection for the current evidence packet.
    ExportSupportPacket,
    /// Reserved live replay action disabled by import/view-only truth.
    ReservedStartLiveReplay,
}

impl ProfilingTraceActionKind {
    /// Stable token used by action rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenFlamegraph => "open_flamegraph",
            Self::OpenTraceManifest => "open_trace_manifest",
            Self::ExportSupportPacket => "export_support_packet",
            Self::ReservedStartLiveReplay => "reserved_start_live_replay",
        }
    }

    /// Reviewer-facing label used by shell rows.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OpenFlamegraph => "Open flamegraph",
            Self::OpenTraceManifest => "Open trace manifest",
            Self::ExportSupportPacket => "Export support packet",
            Self::ReservedStartLiveReplay => "Start live replay (reserved)",
        }
    }
}

/// One action row on the alpha profiling projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfilingTraceAction {
    /// Action kind.
    pub kind: ProfilingTraceActionKind,
    /// Stable action token.
    pub action_token: String,
    /// Reviewer-facing label.
    pub label: String,
    /// True when this action is enabled.
    pub is_live: bool,
    /// Disabled reason for reserved or blocked actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
}

impl ProfilingTraceAction {
    fn live(kind: ProfilingTraceActionKind) -> Self {
        Self {
            kind,
            action_token: kind.as_str().to_owned(),
            label: kind.label().to_owned(),
            is_live: true,
            disabled_reason: None,
        }
    }

    fn reserved_live_replay(lane_state: ReplayLaneState) -> Self {
        Self {
            kind: ProfilingTraceActionKind::ReservedStartLiveReplay,
            action_token: ProfilingTraceActionKind::ReservedStartLiveReplay
                .as_str()
                .to_owned(),
            label: ProfilingTraceActionKind::ReservedStartLiveReplay
                .label()
                .to_owned(),
            is_live: false,
            disabled_reason: Some(format!(
                "Replay lane is {}; live reverse execution is not claimed.",
                lane_state.as_str()
            )),
        }
    }
}

/// Profile-session summary row for the shell projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileSessionSurfaceSummary {
    /// Profile-session id.
    pub profile_session_id: String,
    /// Capture-mode token from the runtime packet.
    pub capture_mode_token: String,
    /// Capture-mode label from the runtime packet.
    pub capture_mode_label: String,
    /// Capture-source token from the runtime packet.
    pub capture_source_token: String,
    /// Capture-source label from the runtime packet.
    pub capture_source_label: String,
    /// Execution context id shared by the evidence.
    pub execution_context_id: String,
    /// Exact-build identity ref shared by the evidence.
    pub exact_build_identity_ref: String,
    /// Target label from the runtime packet.
    pub target_label: String,
    /// Overhead token from the runtime packet.
    pub overhead_token: String,
    /// Mapping-quality token from the runtime packet.
    pub mapping_quality_token: String,
    /// Mapping-quality label from the runtime packet.
    pub mapping_quality_label: String,
}

/// Trace-bundle summary row for the shell projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceBundleSurfaceSummary {
    /// Trace-bundle id.
    pub bundle_id: String,
    /// Raw bundle ref retained by the trace object service.
    pub raw_bundle_ref: String,
    /// Immutability state from the trace manifest.
    pub immutability_state: String,
    /// Derived view tokens available for inspection.
    pub derived_view_tokens: Vec<String>,
    /// Redaction token from the trace manifest.
    pub redaction_token: String,
    /// Redaction label from the trace manifest.
    pub redaction_label: String,
    /// Retention token from the trace manifest.
    pub retention_token: String,
    /// Retention label from the trace manifest.
    pub retention_label: String,
    /// Number of digest entries on the manifest.
    pub digest_count: usize,
}

/// Replay summary row for the shell projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayCapabilitySurfaceSummary {
    /// Replay descriptor id.
    pub descriptor_id: String,
    /// Backend family token.
    pub backend_family: String,
    /// Lane-state token.
    pub lane_state_token: String,
    /// Lane-state label.
    pub lane_state_label: String,
    /// Reverse-step support token.
    pub reverse_step_state: String,
    /// Frame-inspection support token.
    pub frame_inspection_state: String,
    /// Data-inspection support token.
    pub data_inspection_state: String,
    /// Determinism caveats copied from the runtime packet.
    pub determinism_caveats: Vec<String>,
    /// True when the projection claims live replay controls.
    pub live_replay_claimed: bool,
}

/// Comparison summary row for the shell projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComparisonSurfaceSummary {
    /// Comparison packet id.
    pub comparison_packet_id: String,
    /// Workload id.
    pub workload_id: String,
    /// Corpus or archetype token.
    pub corpus_archetype: String,
    /// Source-class token.
    pub source_class_token: String,
    /// Comparison-class token.
    pub comparison_class_token: String,
    /// Comparison-class label.
    pub comparison_class_label: String,
    /// Sample count disclosed by the packet.
    pub sample_count: u32,
    /// Variance window label.
    pub variance_window_label: String,
    /// Confounders shown before any regression claim.
    pub confounders: Vec<String>,
}

/// Support/export summary row for the shell projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeEvidenceSupportSurfaceSummary {
    /// Support-export id.
    pub export_id: String,
    /// Support-pack item id.
    pub support_pack_item_id: String,
    /// Mapping-quality token preserved in support/export.
    pub mapping_quality_token: String,
    /// Comparison-class token preserved in support/export.
    pub comparison_class_token: String,
    /// Redaction token preserved in support/export.
    pub redaction_token: String,
    /// Retention token preserved in support/export.
    pub retention_token: String,
    /// Replay lane token preserved in support/export.
    pub replay_lane_token: String,
    /// True when raw payloads are exported.
    pub raw_payload_exported: bool,
    /// True when replay is import/view-only.
    pub import_view_only: bool,
}

impl RuntimeEvidenceSupportSurfaceSummary {
    fn from_export(export: &RuntimeEvidenceSupportExport) -> Self {
        Self {
            export_id: export.export_id.clone(),
            support_pack_item_id: export.support_pack_item_id.clone(),
            mapping_quality_token: export.mapping_quality_state.as_str().to_owned(),
            comparison_class_token: export.comparison_class.as_str().to_owned(),
            redaction_token: export.redaction_mode.as_str().to_owned(),
            retention_token: export.retention_class.as_str().to_owned(),
            replay_lane_token: export.replay_lane_state.as_str().to_owned(),
            raw_payload_exported: export.raw_payload_exported,
            import_view_only: export.import_view_only,
        }
    }
}

/// First shell projection over one runtime-evidence alpha packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfilingTraceReplayAlphaSurface {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version for this shell projection.
    pub schema_version: u32,
    /// Reviewer-facing scope notice.
    pub scope_notice: String,
    /// Profile-session summary.
    pub profile_session: ProfileSessionSurfaceSummary,
    /// Trace-bundle summary.
    pub trace_bundle: TraceBundleSurfaceSummary,
    /// Replay-capability summary.
    pub replay_capability: ReplayCapabilitySurfaceSummary,
    /// Comparison-class summary.
    pub comparison: ComparisonSurfaceSummary,
    /// Support/export summary.
    pub support_export: RuntimeEvidenceSupportSurfaceSummary,
    /// Action rows available to the shell surface.
    pub actions: Vec<ProfilingTraceAction>,
}

impl ProfilingTraceReplayAlphaSurface {
    /// Projects a runtime-evidence packet into the shell-facing alpha surface.
    pub fn from_packet(packet: &RuntimeEvidenceAlphaPacket) -> Self {
        let profile = &packet.profile_session;
        let trace = &packet.trace_bundle;
        let replay = &packet.replay_capability;
        let comparison = &packet.comparison;

        Self {
            record_kind: PROFILING_TRACE_REPLAY_ALPHA_SURFACE_RECORD_KIND.into(),
            schema_version: PROFILING_TRACE_REPLAY_ALPHA_SURFACE_SCHEMA_VERSION,
            scope_notice: PROFILING_TRACE_REPLAY_ALPHA_SCOPE_NOTICE.into(),
            profile_session: ProfileSessionSurfaceSummary {
                profile_session_id: profile.profile_session_id.clone(),
                capture_mode_token: profile.capture.capture_mode.as_str().to_owned(),
                capture_mode_label: profile.capture.capture_mode.label().to_owned(),
                capture_source_token: profile.capture.capture_source.as_str().to_owned(),
                capture_source_label: profile.capture.capture_source.label().to_owned(),
                execution_context_id: profile.execution_context_id.clone(),
                exact_build_identity_ref: profile
                    .exact_build_identity
                    .exact_build_identity_ref
                    .clone(),
                target_label: profile.target.target_label.clone(),
                overhead_token: profile.overhead_class.as_str().to_owned(),
                mapping_quality_token: profile.mapping_quality.state.as_str().to_owned(),
                mapping_quality_label: profile.mapping_quality.state.label().to_owned(),
            },
            trace_bundle: TraceBundleSurfaceSummary {
                bundle_id: trace.bundle_id.clone(),
                raw_bundle_ref: trace.raw_bundle.raw_bundle_ref.clone(),
                immutability_state: trace.immutability.immutability_state.clone(),
                derived_view_tokens: trace
                    .derived_views
                    .iter()
                    .map(|view| view.view_kind.as_str().to_owned())
                    .collect(),
                redaction_token: trace.redaction.redaction_mode.as_str().to_owned(),
                redaction_label: trace.redaction.redaction_mode.label().to_owned(),
                retention_token: trace.retention.retention_class.as_str().to_owned(),
                retention_label: trace.retention.retention_class.label().to_owned(),
                digest_count: trace.digest_set.len(),
            },
            replay_capability: ReplayCapabilitySurfaceSummary {
                descriptor_id: replay.descriptor_id.clone(),
                backend_family: replay.backend.backend_family.clone(),
                lane_state_token: replay.lane_state.as_str().to_owned(),
                lane_state_label: replay.lane_state.label().to_owned(),
                reverse_step_state: replay.support_matrix.reverse_step.state.as_str().to_owned(),
                frame_inspection_state: replay
                    .support_matrix
                    .frame_inspection
                    .state
                    .as_str()
                    .to_owned(),
                data_inspection_state: replay
                    .support_matrix
                    .data_inspection
                    .state
                    .as_str()
                    .to_owned(),
                determinism_caveats: replay.determinism_caveats.clone(),
                live_replay_claimed: replay.lane_state.claims_live_replay(),
            },
            comparison: ComparisonSurfaceSummary {
                comparison_packet_id: comparison.comparison_packet_id.clone(),
                workload_id: comparison.workload_id.clone(),
                corpus_archetype: comparison.corpus_archetype.clone(),
                source_class_token: comparison.source_class.as_str().to_owned(),
                comparison_class_token: comparison.comparison_class.as_str().to_owned(),
                comparison_class_label: comparison.comparison_class.label().to_owned(),
                sample_count: comparison.variance_window.sample_count,
                variance_window_label: comparison.variance_window.window_label.clone(),
                confounders: comparison.confounders.clone(),
            },
            support_export: RuntimeEvidenceSupportSurfaceSummary::from_export(
                &packet.support_export,
            ),
            actions: vec![
                ProfilingTraceAction::live(ProfilingTraceActionKind::OpenFlamegraph),
                ProfilingTraceAction::live(ProfilingTraceActionKind::OpenTraceManifest),
                ProfilingTraceAction::live(ProfilingTraceActionKind::ExportSupportPacket),
                ProfilingTraceAction::reserved_live_replay(replay.lane_state),
            ],
        }
    }

    /// Returns true when the projection did not overclaim live replay or comparison truth.
    pub fn preserves_runtime_truth(&self) -> bool {
        !self.replay_capability.live_replay_claimed
            && self.replay_capability.lane_state_token == ReplayLaneState::ImportViewOnly.as_str()
            && self.comparison.comparison_class_token
                == ComparisonClass::ImportViewOnlyNotComparable.as_str()
            && self.support_export.redaction_token
                == TraceRedactionMode::LocalOnlyRawRetained.as_str()
            && self.support_export.retention_token
                == TraceRetentionClass::LocalRotationSevenDays.as_str()
            && self.support_export.import_view_only
            && !self.support_export.raw_payload_exported
    }

    /// Returns the action row for the requested kind.
    pub fn find_action(&self, kind: ProfilingTraceActionKind) -> Option<&ProfilingTraceAction> {
        self.actions.iter().find(|action| action.kind == kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projection_preserves_runtime_evidence_truth_without_live_replay_claim() {
        let packet = RuntimeEvidenceAlphaPacket::import_view_only_baseline();
        let surface = ProfilingTraceReplayAlphaSurface::from_packet(&packet);

        assert!(surface.preserves_runtime_truth());
        assert_eq!(
            surface.profile_session.execution_context_id,
            packet.profile_session.execution_context_id
        );
        assert_eq!(
            surface.profile_session.exact_build_identity_ref,
            packet
                .profile_session
                .exact_build_identity
                .exact_build_identity_ref
        );
        assert_eq!(
            surface.profile_session.mapping_quality_token,
            packet.profile_session.mapping_quality.state.as_str()
        );
        assert_eq!(
            surface.comparison.comparison_class_token,
            packet.comparison.comparison_class.as_str()
        );
        assert_eq!(
            surface.support_export.redaction_token,
            packet.support_export.redaction_mode.as_str()
        );
        let live_replay = surface
            .find_action(ProfilingTraceActionKind::ReservedStartLiveReplay)
            .expect("reserved live replay action exists");
        assert!(!live_replay.is_live);
        assert!(live_replay
            .disabled_reason
            .as_deref()
            .expect("disabled reason")
            .contains("import_view_only"));
    }
}
