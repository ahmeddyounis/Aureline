//! Chrome projection for the request-workspace alpha contract.
//!
//! The truth source is the canonical
//! [`aureline_runtime::RequestWorkspaceAlphaRecord`]. This module wraps that
//! record in the panel-shaped projection chrome inspectors render: one panel
//! header per request workspace row, structured request / environment /
//! credential / side-effect / response blocks, and one banner row per
//! send-inspector banner.
//!
//! UI / CLI parity invariant: the panel projection MUST iterate the
//! canonical send-inspector report verbatim. It does not invent banners,
//! drop side-effect rows, re-derive readiness, or paraphrase tokens. The
//! integration test in
//! [`/crates/aureline-runtime/tests/request_workspace_alpha.rs`](../../../../crates/aureline-runtime/tests/request_workspace_alpha.rs)
//! and the headless inspector binary
//! [`aureline_shell_request_workspace`](../bin/aureline_shell_request_workspace.rs)
//! both consume the same record, so the UI inspector cannot render a
//! different banner set, side-effect list, or readiness state than the CLI.

use serde::{Deserialize, Serialize};

use aureline_runtime::{
    ExpectedSideEffectRow, RequestWorkspaceAlphaRecord, RequestWorkspaceSupportExport,
    SendInspectorBanner, SendInspectorReport, REQUEST_WORKSPACE_ALPHA_SCHEMA_VERSION,
};

pub use aureline_runtime::{
    seeded_request_workspace_record as seeded_record,
    seeded_request_workspace_support_export as seeded_support_export,
    seeded_send_inspector_report as seeded_send_inspector_report,
    RequestWorkspaceSeededScenario,
};

/// Stable record kind for [`RequestWorkspacePanelProjection`] payloads.
pub const REQUEST_WORKSPACE_PANEL_PROJECTION_RECORD_KIND: &str =
    "shell_request_workspace_panel_projection_record";

/// Chrome panel projection for one request-workspace row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestWorkspacePanelProjection {
    /// Stable record kind.
    pub record_kind: String,
    /// Request-workspace schema version mirrored from the runtime record.
    pub schema_version: u32,
    /// Source request-workspace ref.
    pub request_workspace_ref: String,
    /// Source request id.
    pub request_id: String,
    /// Canonical execution-context ref.
    pub execution_context_ref: String,
    /// Stable target-class token.
    pub target_class_token: String,
    /// Stable method token.
    pub method_token: String,
    /// True when the inspector chrome MUST render the boundary cue.
    pub boundary_cue_visible: bool,
    /// True when the inspector chrome MUST render a review-required banner.
    pub requires_review_before_dispatch: bool,
    /// True when the inspector chrome MUST block the send action.
    pub blocks_dispatch: bool,
    /// Stable readiness token.
    pub readiness_token: String,
    /// Side-effect rows in canonical render order. Chrome MUST NOT invent
    /// or drop entries.
    pub expected_side_effects: Vec<ExpectedSideEffectRow>,
    /// Banner rows in canonical render order.
    pub banners: Vec<SendInspectorBanner>,
    /// Underlying canonical send-inspector report. Chrome MAY embed this
    /// verbatim in a "raw record" surface for support / replay flows.
    pub send_inspector_report: SendInspectorReport,
}

impl RequestWorkspacePanelProjection {
    /// Project a chrome panel from one canonical record.
    pub fn from_record(record: &RequestWorkspaceAlphaRecord) -> Self {
        let report = record.send_inspector_report();
        Self {
            record_kind: REQUEST_WORKSPACE_PANEL_PROJECTION_RECORD_KIND.to_owned(),
            schema_version: REQUEST_WORKSPACE_ALPHA_SCHEMA_VERSION,
            request_workspace_ref: record.request_workspace_ref.clone(),
            request_id: report.request_id.clone(),
            execution_context_ref: report.execution_context_ref.clone(),
            target_class_token: report.target_class_token.clone(),
            method_token: report.method_token.clone(),
            boundary_cue_visible: report.boundary_cue_visible,
            requires_review_before_dispatch: report.requires_review_before_dispatch,
            blocks_dispatch: report.blocks_dispatch,
            readiness_token: report.readiness_token.clone(),
            expected_side_effects: report.expected_side_effects.clone(),
            banners: report.banners.clone(),
            send_inspector_report: report,
        }
    }
}

/// Bundle chrome panel projections into the canonical support-export
/// wrapper. The wrapper still rides on the runtime
/// [`RequestWorkspaceSupportExport`] shape so reviewer / support consumers
/// do not have to parse a shell-side dialect.
pub fn panel_support_export(
    manifest_id: impl Into<String>,
    generated_at: impl Into<String>,
    records: Vec<RequestWorkspaceAlphaRecord>,
) -> RequestWorkspaceSupportExport {
    RequestWorkspaceSupportExport::from_records(manifest_id, generated_at, records)
}

/// Render an export-safe plaintext block for one support-export packet.
/// Both UI inspector chrome and the headless inspector binary delegate to
/// this helper so reviewer / support evidence is character-identical.
pub fn render_support_export_plaintext(export: &RequestWorkspaceSupportExport) -> String {
    export.render_plaintext()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_runtime::{
        RequestWorkspaceSeededScenario, SendInspectorReadiness, TargetClass,
    };

    #[test]
    fn local_read_only_panel_matches_runtime_report() {
        let record = seeded_record(RequestWorkspaceSeededScenario::LocalReadOnlyGet);
        let panel = RequestWorkspacePanelProjection::from_record(&record);
        assert_eq!(panel.readiness_token, "ready_to_send");
        assert!(!panel.boundary_cue_visible);
        assert_eq!(panel.target_class_token, TargetClass::LocalHost.as_str());
        assert_eq!(
            panel.expected_side_effects.len(),
            record.expected_side_effects.len()
        );
    }

    #[test]
    fn managed_delete_panel_blocks_dispatch_with_schema_stale_banner() {
        let record = seeded_record(RequestWorkspaceSeededScenario::ManagedDeleteMissingSchema);
        let panel = RequestWorkspacePanelProjection::from_record(&record);
        assert!(panel.blocks_dispatch);
        assert_eq!(
            panel.send_inspector_report.readiness,
            SendInspectorReadiness::BlockedSchemaStale
        );
        assert!(panel
            .banners
            .iter()
            .any(|banner| banner.banner_kind == "schema_stale_blocked"));
    }

    #[test]
    fn panel_banner_set_matches_canonical_report_verbatim() {
        // UI / CLI parity: the panel projection must iterate the canonical
        // report's banner list verbatim. The panel does not reorder, drop,
        // or invent entries.
        for scenario in RequestWorkspaceSeededScenario::ALL {
            let record = seeded_record(scenario);
            let panel = RequestWorkspacePanelProjection::from_record(&record);
            assert_eq!(panel.banners, panel.send_inspector_report.banners);
            assert_eq!(
                panel.expected_side_effects,
                panel.send_inspector_report.expected_side_effects
            );
        }
    }

    #[test]
    fn panel_support_export_plaintext_is_deterministic() {
        let export = seeded_support_export(
            "request-workspace-alpha:shell:test",
            "2026-05-15T00:00:00Z",
        );
        let first = render_support_export_plaintext(&export);
        let second = render_support_export_plaintext(&export);
        assert_eq!(first, second);
    }
}
