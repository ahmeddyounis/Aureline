//! Start Center workspace-template bundle alpha projection.
//!
//! This module turns the checked-in alpha workspace-template bundles into the
//! compact Start Center rows that disclose author / source class, support
//! class, target runtime, side effects, trust posture, and open-without-starter
//! bypass routes **before** a template is used. The same projection backs the
//! deterministic CLI / headless plaintext export so docs, support packets, and
//! scripted entry surfaces read one bundle truth.

use std::fmt;

use aureline_workspace::{
    project_workspace_template_bundle, WorkspaceTemplateBundleError,
    WorkspaceTemplateBundleProjection,
};

const ALPHA_BUNDLES: &[(&str, &str)] = &[
    (
        "fixtures/workspace/m3/template_bundle/first_party_local_starter.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/workspace/m3/template_bundle/first_party_local_starter.json"
        )),
    ),
    (
        "fixtures/workspace/m3/template_bundle/community_uncertified_starter.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/workspace/m3/template_bundle/community_uncertified_starter.json"
        )),
    ),
    (
        "fixtures/workspace/m3/template_bundle/managed_cloud_starter.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/workspace/m3/template_bundle/managed_cloud_starter.json"
        )),
    ),
];

/// Start Center row for one alpha workspace-template bundle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartCenterTemplateBundleRow {
    /// Stable bundle id quoted by support and CLI surfaces.
    pub bundle_id: String,
    /// Bound template id from the signed manifest.
    pub bound_template_id: String,
    /// Bound template version from the signed manifest.
    pub bound_template_version: String,
    /// Display label rendered on the card.
    pub display_label: String,
    /// Short reviewable summary rendered under the title.
    pub summary: String,
    /// Source class shown next to the title.
    pub source_class: String,
    /// Distribution channel for the bound source.
    pub source_distribution_class: String,
    /// Signature state shown next to the source.
    pub signature_state: String,
    /// Publisher label shown next to the signature.
    pub publisher_label: String,
    /// Support class shown on the row.
    pub support_class: String,
    /// Lifecycle class re-exported from the manifest.
    pub lifecycle_class: String,
    /// Runtime scope shown next to the support class.
    pub runtime_scope_class: String,
    /// Host boundary class shown next to the runtime scope.
    pub host_boundary_class: String,
    /// Supported ecosystem class tokens.
    pub supported_ecosystems: Vec<String>,
    /// Supported platform class tokens.
    pub supported_platforms: Vec<String>,
    /// Side-effect class summary.
    pub side_effect_summary: String,
    /// Side-effect notes carried on the bundle.
    pub side_effect_notes: Vec<String>,
    /// Trust posture class.
    pub trust_posture_class: String,
    /// Egress posture class.
    pub egress_posture_class: String,
    /// Trust notes shown next to the source class.
    pub trust_notes: Vec<String>,
    /// Open-without-starter bypass route ids advertised at equal weight.
    pub bypass_route_ids: Vec<String>,
    /// Always `equal_weight_with_apply`.
    pub bypass_continuity_class: String,
    /// Consumer surfaces that read this bundle.
    pub consumer_surfaces: Vec<String>,
    /// Support-export packet refs.
    pub support_export_refs: Vec<String>,
}

impl From<WorkspaceTemplateBundleProjection> for StartCenterTemplateBundleRow {
    fn from(projection: WorkspaceTemplateBundleProjection) -> Self {
        let side_effect_summary = format!(
            "egress={} ext={} remote={} managed={} cred={}",
            projection.required_network_egress_class,
            projection.required_extension_install_class,
            projection.required_remote_provisioning_class,
            projection.required_managed_service_class,
            projection.required_credential_provisioning_class,
        );
        Self {
            bundle_id: projection.bundle_id,
            bound_template_id: projection.bound_template_id,
            bound_template_version: projection.bound_template_version,
            display_label: projection.display_label,
            summary: projection.summary,
            source_class: projection.source_class,
            source_distribution_class: projection.source_distribution_class,
            signature_state: projection.signature_state,
            publisher_label: projection.publisher_label,
            support_class: projection.support_class,
            lifecycle_class: projection.lifecycle_class,
            runtime_scope_class: projection.runtime_scope_class,
            host_boundary_class: projection.host_boundary_class,
            supported_ecosystems: projection.supported_ecosystems,
            supported_platforms: projection.supported_platforms,
            side_effect_summary,
            side_effect_notes: projection.side_effect_notes,
            trust_posture_class: projection.trust_posture_class,
            egress_posture_class: projection.egress_posture_class,
            trust_notes: projection.trust_notes,
            bypass_route_ids: projection.open_without_starter_route_ids,
            bypass_continuity_class: projection.bypass_continuity_class,
            consumer_surfaces: projection.consumer_surfaces,
            support_export_refs: projection.support_export_refs,
        }
    }
}

/// Error returned when the Start Center cannot project an alpha bundle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartCenterTemplateBundleError {
    source_ref: &'static str,
    message: String,
}

impl StartCenterTemplateBundleError {
    /// Returns the artifact path that failed to project.
    pub const fn source_ref(&self) -> &'static str {
        self.source_ref
    }

    /// Returns the parse or validation failure.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for StartCenterTemplateBundleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.source_ref, self.message)
    }
}

impl std::error::Error for StartCenterTemplateBundleError {}

/// Builds Start Center template-bundle rows from the checked-in alpha fixtures.
///
/// # Errors
///
/// Returns [`StartCenterTemplateBundleError`] when any bundle fails to parse or
/// validate against the alpha contract.
pub fn build_alpha_template_bundle_rows(
) -> Result<Vec<StartCenterTemplateBundleRow>, StartCenterTemplateBundleError> {
    let mut rows = Vec::with_capacity(ALPHA_BUNDLES.len());
    for (source_ref, payload) in ALPHA_BUNDLES {
        let projection = project_workspace_template_bundle(payload)
            .map_err(|err| projection_error(source_ref, err))?;
        rows.push(StartCenterTemplateBundleRow::from(projection));
    }
    Ok(rows)
}

/// Renders the alpha bundle projection as deterministic plaintext for CLI /
/// headless / docs consumers.
///
/// # Errors
///
/// Returns [`StartCenterTemplateBundleError`] when a bundle cannot be projected.
pub fn render_alpha_template_bundle_plaintext() -> Result<String, StartCenterTemplateBundleError> {
    let rows = build_alpha_template_bundle_rows()?;
    let mut lines = vec![
        "Workspace template bundle alpha".to_string(),
        "bundle_id | source/signature | support | runtime/host | egress | bypass_routes"
            .to_string(),
    ];
    for row in rows {
        lines.push(format!(
            "{} | {}/{} | {} | {}/{} | {} | {}",
            row.bundle_id,
            row.source_class,
            row.signature_state,
            row.support_class,
            row.runtime_scope_class,
            row.host_boundary_class,
            row.side_effect_summary,
            row.bypass_route_ids.join(",")
        ));
    }
    lines.push(String::new());
    Ok(lines.join("\n"))
}

fn projection_error(
    source_ref: &'static str,
    err: WorkspaceTemplateBundleError,
) -> StartCenterTemplateBundleError {
    StartCenterTemplateBundleError {
        source_ref,
        message: err.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha_bundle_rows_project() {
        let rows = build_alpha_template_bundle_rows().expect("alpha bundles must project");
        assert_eq!(rows.len(), 3);
        let bundle_ids: Vec<&str> = rows.iter().map(|row| row.bundle_id.as_str()).collect();
        assert!(bundle_ids.contains(&"workspace_template_bundle_alpha:typescript_web_vite_local"));
        assert!(
            bundle_ids.contains(&"workspace_template_bundle_alpha:community_python_data_starter")
        );
        assert!(
            bundle_ids.contains(&"workspace_template_bundle_alpha:managed_cloud_remote_service")
        );
        for row in &rows {
            assert_eq!(row.bypass_continuity_class, "equal_weight_with_apply");
            assert!(
                row.consumer_surfaces.iter().any(|s| s == "start_center"),
                "every bundle row must keep start_center wired"
            );
        }
    }

    #[test]
    fn alpha_bundle_plaintext_is_deterministic() {
        let first = render_alpha_template_bundle_plaintext().expect("plaintext renders");
        let second = render_alpha_template_bundle_plaintext().expect("plaintext renders");
        assert_eq!(first, second);
        assert!(first.contains("Workspace template bundle alpha"));
        assert!(first.contains("bypass.create_empty_workspace"));
    }
}
