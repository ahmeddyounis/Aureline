//! Chrome projection for the env-inspect contract.
//!
//! The truth source is the canonical
//! [`aureline_runtime::EnvInspectSnapshot`]. This module wraps that snapshot
//! in the panel-shaped projection chrome inspectors render: one panel header
//! per execution context, one section block per
//! [`aureline_runtime::EnvInspectSection`], and one row per
//! [`aureline_runtime::EnvInspectCoreField`].
//!
//! UI / CLI parity invariant: the panel projection MUST iterate the snapshot
//! verbatim. It does not invent sections, drop rows, re-derive degradation
//! severity, or paraphrase tokens. The integration test in
//! [`/crates/aureline-runtime/tests/env_inspect_beta.rs`](../../../../crates/aureline-runtime/tests/env_inspect_beta.rs)
//! and the headless inspector binary
//! [`aureline_shell_env_inspect`](../bin/aureline_shell_env_inspect.rs) both
//! consume the same snapshot, so the UI inspector cannot render a different
//! set of core fields or labels than the CLI.

use serde::{Deserialize, Serialize};

use aureline_runtime::{
    EnvInspectCoreField, EnvInspectDegradationLabel, EnvInspectDegradationSeverity,
    EnvInspectSection, EnvInspectSnapshot, EnvInspectSupportExport,
    ENV_INSPECT_SCHEMA_VERSION,
};

pub use aureline_runtime::{
    seeded_env_inspect_resolver as seeded_resolver,
    seeded_env_inspect_snapshot as seeded_snapshot,
    seeded_env_inspect_support_export as seeded_support_export, EnvInspectSeededScenario,
};

/// Stable record kind for [`EnvInspectPanelProjection`] payloads.
pub const ENV_INSPECT_PANEL_PROJECTION_RECORD_KIND: &str =
    "shell_env_inspect_panel_projection_record";

/// Stable record kind for [`EnvInspectPanelSection`] payloads.
pub const ENV_INSPECT_PANEL_SECTION_RECORD_KIND: &str =
    "shell_env_inspect_panel_section_record";

/// One row inside a panel section. The row carries the same `field_path`,
/// `label`, and `value_token` as the runtime core-field record so chrome
/// rendering MUST NOT invent its own labels.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvInspectPanelRow {
    /// Dotted field path into the canonical execution context.
    pub field_path: String,
    /// Reviewer-facing row label.
    pub label: String,
    /// Resolved value token. Missing when the resolver did not record one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_token: Option<String>,
}

impl EnvInspectPanelRow {
    fn from_core_field(field: &EnvInspectCoreField) -> Self {
        Self {
            field_path: field.field_path.clone(),
            label: field.label.clone(),
            value_token: field.value_token.clone(),
        }
    }
}

/// One section block rendered by the inspector panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvInspectPanelSection {
    /// Stable record kind.
    pub record_kind: String,
    /// Canonical section.
    pub section: EnvInspectSection,
    /// Stable section token.
    pub section_token: String,
    /// Reviewer-facing section label.
    pub section_label: String,
    /// Rows in canonical render order.
    pub rows: Vec<EnvInspectPanelRow>,
}

impl EnvInspectPanelSection {
    fn from_snapshot(snapshot: &EnvInspectSnapshot, section: EnvInspectSection) -> Self {
        let rows = snapshot
            .fields_for_section(section)
            .map(EnvInspectPanelRow::from_core_field)
            .collect();
        Self {
            record_kind: ENV_INSPECT_PANEL_SECTION_RECORD_KIND.to_owned(),
            section,
            section_token: section.as_str().to_owned(),
            section_label: section.label().to_owned(),
            rows,
        }
    }
}

/// One degradation banner rendered above the inspector body.
///
/// The banner carries the same severity band as the canonical degradation
/// label; chrome MUST NOT invent its own severity or paraphrase the label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvInspectPanelBanner {
    /// Severity band.
    pub severity: EnvInspectDegradationSeverity,
    /// Stable severity token.
    pub severity_token: String,
    /// Field path the banner applies to.
    pub field_path: String,
    /// Reviewer-facing label.
    pub label: String,
    /// Optional repair-hook reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repair_hook_ref: Option<String>,
}

impl EnvInspectPanelBanner {
    fn from_label(label: &EnvInspectDegradationLabel) -> Self {
        Self {
            severity: label.severity,
            severity_token: label.severity_token.clone(),
            field_path: label.field_path.clone(),
            label: label.label.clone(),
            repair_hook_ref: label.repair_hook_ref.clone(),
        }
    }
}

/// Chrome inspector panel projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvInspectPanelProjection {
    /// Stable record kind.
    pub record_kind: String,
    /// Env-inspect schema version mirrored from the runtime snapshot.
    pub schema_version: u32,
    /// Canonical execution-context id the panel renders.
    pub execution_context_ref: String,
    /// Lane the canonical context resolves onto.
    pub lane_token: String,
    /// Reviewer-facing lane label.
    pub lane_label: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Surface the canonical context was minted for.
    pub surface_token: String,
    /// True when the inspector chrome MUST render the boundary cue.
    pub boundary_cue_visible: bool,
    /// True when the inspector chrome MUST render a review-required banner.
    pub requires_review_before_dispatch: bool,
    /// True when the inspector chrome MUST block the dispatch action.
    pub blocks_dispatch: bool,
    /// Section blocks in canonical render order.
    pub sections: Vec<EnvInspectPanelSection>,
    /// Degradation banners, one per canonical degradation label.
    pub degradation_banners: Vec<EnvInspectPanelBanner>,
    /// Underlying canonical snapshot. Chrome MAY embed this verbatim in a
    /// "raw record" surface for support / replay flows.
    pub snapshot: EnvInspectSnapshot,
}

impl EnvInspectPanelProjection {
    /// Project a chrome panel from one canonical snapshot.
    pub fn from_snapshot(snapshot: EnvInspectSnapshot) -> Self {
        let sections: Vec<EnvInspectPanelSection> = EnvInspectSection::ORDER
            .iter()
            .map(|section| EnvInspectPanelSection::from_snapshot(&snapshot, *section))
            .collect();
        let degradation_banners = snapshot
            .degradation_labels
            .iter()
            .map(EnvInspectPanelBanner::from_label)
            .collect();
        Self {
            record_kind: ENV_INSPECT_PANEL_PROJECTION_RECORD_KIND.to_owned(),
            schema_version: ENV_INSPECT_SCHEMA_VERSION,
            execution_context_ref: snapshot.execution_context_id.clone(),
            lane_token: snapshot.lane_token.clone(),
            lane_label: snapshot.lane_label.clone(),
            workspace_id: snapshot.workspace_id.clone(),
            surface_token: snapshot.surface_token.clone(),
            boundary_cue_visible: snapshot.boundary_cue_visible,
            requires_review_before_dispatch: snapshot.requires_review_before_dispatch(),
            blocks_dispatch: snapshot.blocks_dispatch(),
            sections,
            degradation_banners,
            snapshot,
        }
    }

    /// Iterate the panel rows in canonical render order — the same order the
    /// CLI / headless surface emits.
    pub fn rows(&self) -> impl Iterator<Item = &EnvInspectPanelRow> {
        self.sections.iter().flat_map(|section| section.rows.iter())
    }

    /// Convenience accessor used by the support-export wrapper.
    pub fn snapshot(&self) -> &EnvInspectSnapshot {
        &self.snapshot
    }
}

/// Bundle a chrome panel projection into the canonical support-export
/// wrapper. The wrapper still rides on the runtime [`EnvInspectSupportExport`]
/// shape so reviewer / support consumers do not have to parse a shell-side
/// dialect.
pub fn panel_support_export(
    manifest_id: impl Into<String>,
    generated_at: impl Into<String>,
    panels: &[EnvInspectPanelProjection],
) -> EnvInspectSupportExport {
    let snapshots = panels
        .iter()
        .map(|panel| panel.snapshot.clone())
        .collect();
    EnvInspectSupportExport::new(manifest_id, generated_at, snapshots)
}

/// Render an export-safe plaintext block for one seeded snapshot. Both UI
/// inspector chrome and the headless inspector binary delegate to this
/// helper so reviewer / support evidence is character-identical.
pub fn render_snapshot_plaintext(snapshot: &EnvInspectSnapshot) -> String {
    snapshot.render_plaintext()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_runtime::{
        CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContextRequest,
        ExecutionContextResolver, ExecutionContextResolverConfig, IdentityMode, ScopeClass,
        TargetClass, TrustState,
    };

    fn baseline_config() -> ExecutionContextResolverConfig {
        ExecutionContextResolverConfig {
            workspace_id: "ws-shell-env-inspect".to_owned(),
            profile_id: Some("prof.shell-env-inspect".to_owned()),
            identity_mode: IdentityMode::AccountFreeLocal,
            policy_epoch: 1,
            workspace_default_target_class: TargetClass::LocalHost,
            workspace_default_working_directory: Some("/workspace".to_owned()),
            workspace_default_scope_class: ScopeClass::CurrentRoot,
            local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
            environment_capsule_ref: EnvironmentCapsuleRef {
                capsule_id: "caps:shell-env-inspect:seed".to_owned(),
                capsule_hash: "sha256:shell-seed".to_owned(),
                resolved_schema_version: "1".to_owned(),
                drift_state: CapsuleDriftState::InSync,
            },
            resolver_version: "shell-env-inspect-test-0".to_owned(),
        }
    }

    #[test]
    fn panel_projection_carries_every_canonical_section() {
        let mut resolver = ExecutionContextResolver::new(baseline_config());
        let context = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
            "terminal.open",
            TrustState::Trusted,
            "mono:0",
        ));
        let snapshot = EnvInspectSnapshot::from_context(&context);
        let panel = EnvInspectPanelProjection::from_snapshot(snapshot.clone());
        assert_eq!(panel.sections.len(), EnvInspectSection::ORDER.len());
        for (panel_section, canonical_section) in
            panel.sections.iter().zip(EnvInspectSection::ORDER.iter())
        {
            assert_eq!(panel_section.section, *canonical_section);
            assert!(!panel_section.rows.is_empty());
        }
        assert!(!panel.boundary_cue_visible);
        assert!(panel.degradation_banners.is_empty());
        assert!(!panel.requires_review_before_dispatch);
    }

    #[test]
    fn panel_rows_match_snapshot_core_fields_verbatim() {
        // UI / CLI parity: the panel projection must iterate the snapshot's
        // core-field rows verbatim. The rows do not reorder, drop, or invent
        // entries.
        let mut resolver = ExecutionContextResolver::new(baseline_config());
        let context = resolver.resolve(ExecutionContextRequest::task_seed(
            "task.run",
            TrustState::Trusted,
            "mono:0",
        ));
        let snapshot = EnvInspectSnapshot::from_context(&context);
        let panel = EnvInspectPanelProjection::from_snapshot(snapshot.clone());

        let panel_paths: Vec<&str> = panel.rows().map(|row| row.field_path.as_str()).collect();
        let snapshot_paths: Vec<&str> = snapshot
            .core_fields
            .iter()
            .map(|field| field.field_path.as_str())
            .collect();
        assert_eq!(panel_paths, snapshot_paths);
    }

    #[test]
    fn pending_trust_panel_lights_review_banner() {
        let mut resolver = ExecutionContextResolver::new(baseline_config());
        let context = resolver.resolve(ExecutionContextRequest::remote_attach_task_seed(
            "task.run.ssh_remote",
            TargetClass::SshRemote,
            TrustState::PendingEvaluation,
            "mono:0",
        ));
        let snapshot = EnvInspectSnapshot::from_context(&context);
        let panel = EnvInspectPanelProjection::from_snapshot(snapshot);
        assert!(panel.boundary_cue_visible);
        assert!(panel.requires_review_before_dispatch);
        assert!(!panel.degradation_banners.is_empty());
    }
}
