//! Provider-overlay disclosure and vendor-console handoff continuity packets.
//!
//! This module binds infrastructure provider overlays to explicit vendor-console
//! handoffs and return-safe breadcrumbs. It ensures code, incident, preview,
//! route, and infrastructure surfaces all disclose the same provider-owned
//! overlay truth, the same control-plane boundary, and the same return anchor
//! instead of treating browser escape hatches as hidden or generic fallbacks.

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{
    source_intelligence_and_resource_relationships::{
        InfrastructureFamily, TruthLayer, SOURCE_INTELLIGENCE_OBJECT_PACKET_RECORD_KIND,
    },
    target_context_and_control_plane_boundary::{
        validate_control_plane_handoff, ControlPlaneHandoff, ControlPlaneHandoffReason,
        EnvironmentCompleteness, EnvironmentContext, FreshnessLabel, InfraBoundaryFinding,
        InfraBoundaryFindingSeverity, CONTROL_PLANE_BOUNDARY_PACKET_RECORD_KIND,
    },
};

/// Schema version for provider-overlay continuity packets.
pub const PROVIDER_OVERLAY_HANDOFF_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind discriminator for [`ProviderOverlayHandoffContinuityPacket`].
pub const PROVIDER_OVERLAY_HANDOFF_PACKET_RECORD_KIND: &str =
    "infra_provider_overlay_and_vendor_console_handoff_continuity_packet";

/// JSON Schema reference for packet interchange.
pub const PROVIDER_OVERLAY_HANDOFF_SCHEMA_REF: &str =
    "schemas/infra/provider-overlay-and-vendor-console-handoff-continuity.schema.json";

/// Reviewer-facing documentation reference.
pub const PROVIDER_OVERLAY_HANDOFF_DOC_REF: &str =
    "docs/infra/provider-overlay-and-vendor-console-handoff-continuity.md";

/// Fixture corpus directory for continuity qualification and downgrade drills.
pub const PROVIDER_OVERLAY_HANDOFF_FIXTURE_DIR: &str =
    "fixtures/infra/provider-overlay-and-vendor-console-handoff-continuity";

/// Checked support-export artifact for the qualified continuity packet.
pub const PROVIDER_OVERLAY_HANDOFF_ARTIFACT_REF: &str =
    "artifacts/infra/provider-overlay-and-vendor-console-handoff-continuity/support_export.json";

/// Surface that must preserve the same overlay and handoff disclosure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverlayContinuitySurface {
    /// File, manifest, or code-local breadcrumb surface.
    CodeBreadcrumb,
    /// Incident workspace or runbook surface.
    IncidentWorkspace,
    /// Preview route or preview diagnostics surface.
    PreviewRoute,
    /// Route explorer or endpoint relationship surface.
    RouteExplorer,
    /// Infrastructure resource panel or graph surface.
    InfrastructurePanel,
}

/// Explicit disclosure row for one provider-owned overlay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderOverlayDisclosureRow {
    /// Stable overlay-row id.
    pub overlay_row_id: String,
    /// Infrastructure family the overlay belongs to.
    pub family: InfrastructureFamily,
    /// Provider-owned overlay object ref from the shared object packet.
    pub overlay_object_ref: String,
    /// Canonical repo-owned or locally observed object ref the overlay enriches.
    pub canonical_object_ref: String,
    /// Relation ref that proves the overlay enriches the canonical object.
    pub overlay_relation_ref: String,
    /// Canonical truth layer preserved alongside the overlay.
    pub canonical_truth_layer: TruthLayer,
    /// Shared target context the row belongs to.
    pub context_ref: String,
    /// Stable reason the overlay remains explicit instead of canonicalized.
    pub handoff_reason: ControlPlaneHandoffReason,
    /// Freshness posture carried by the overlay row.
    pub freshness: FreshnessLabel,
    /// Human-facing source label shown with the overlay row.
    pub source_label: String,
    /// True when the row states that the visible data is provider-owned overlay truth.
    pub declares_provider_owned_truth: bool,
    /// True when canonical repo-owned or local truth remains visible next to the overlay.
    pub canonical_truth_visible: bool,
    /// Handoff ref used when the overlay requires a provider-owned page.
    pub handoff_ref: String,
    /// Surfaces that must render this overlay with the same disclosure.
    pub visible_surfaces: Vec<OverlayContinuitySurface>,
    /// Export-safe row summary.
    pub support_summary: String,
}

/// Surface binding that proves overlay and handoff continuity stays stable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverlayContinuitySurfaceBinding {
    /// Surface covered by the binding.
    pub surface: OverlayContinuitySurface,
    /// Shared target context shown by the surface.
    pub context_ref: String,
    /// Overlay rows rendered by the surface.
    pub overlay_row_refs: Vec<String>,
    /// Handoff rows reachable from the surface.
    pub handoff_refs: Vec<String>,
    /// True when the provider-overlay badge stays visible.
    pub shows_provider_overlay_badge: bool,
    /// True when the surface names the canonical truth the overlay enriches.
    pub shows_canonical_truth_reference: bool,
    /// True when the control-plane boundary stays visible.
    pub shows_control_plane_boundary: bool,
    /// True when the handoff reason stays visible.
    pub shows_handoff_reason: bool,
    /// True when the surface shows the structured return anchor.
    pub shows_return_anchor: bool,
    /// True when the surface shows return-safe breadcrumbs.
    pub shows_breadcrumbs: bool,
    /// True when target identity is preserved verbatim across the surface.
    pub preserves_target_identity: bool,
    /// True when returning rehydrates the same target context instead of a generic shell.
    pub return_rehydrates_same_context: bool,
    /// True when the surface reads the shared packet directly.
    pub uses_shared_packet: bool,
    /// Export-safe surface summary.
    pub support_summary: String,
}

/// Canonical packet for provider-overlay and vendor-console handoff continuity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderOverlayHandoffContinuityPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Source-intelligence packet family that froze object ids and relation refs.
    pub source_intelligence_packet_ref: String,
    /// Boundary packet family that froze target-context and handoff vocabulary.
    pub boundary_packet_ref: String,
    /// Shared environment contexts used by overlays and handoffs.
    pub environment_contexts: Vec<EnvironmentContext>,
    /// Provider-overlay disclosure rows.
    pub overlay_rows: Vec<ProviderOverlayDisclosureRow>,
    /// Vendor-console or provider-owned handoff rows.
    pub handoff_rows: Vec<ControlPlaneHandoff>,
    /// Surface bindings that preserve identical disclosure.
    pub surface_bindings: Vec<OverlayContinuitySurfaceBinding>,
    /// Export-safe packet summary.
    pub support_summary: String,
}

impl ProviderOverlayHandoffContinuityPacket {
    /// Validates the packet against provider-overlay continuity invariants.
    pub fn validate(&self) -> ProviderOverlayHandoffContinuityValidationReport {
        validate_provider_overlay_handoff_packet(self)
    }
}

/// Validation report emitted for provider-overlay continuity packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderOverlayHandoffContinuityValidationReport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version validated.
    pub schema_version: u32,
    /// Packet id validated.
    pub packet_id: String,
    /// True when no error-severity finding was emitted.
    pub passed: bool,
    /// Context ids covered by the packet.
    pub context_ids: BTreeSet<String>,
    /// Overlay rows covered by the packet.
    pub overlay_row_ids: BTreeSet<String>,
    /// Handoff ids covered by the packet.
    pub handoff_ids: BTreeSet<String>,
    /// Surface bindings covered by the packet.
    pub surfaces: BTreeSet<OverlayContinuitySurface>,
    /// Findings emitted during validation.
    pub findings: Vec<InfraBoundaryFinding>,
}

/// Validates one provider-overlay continuity packet.
pub fn validate_provider_overlay_handoff_packet(
    packet: &ProviderOverlayHandoffContinuityPacket,
) -> ProviderOverlayHandoffContinuityValidationReport {
    let mut findings = Vec::new();
    let mut context_ids = BTreeSet::new();
    let mut overlay_row_ids = BTreeSet::new();
    let mut handoff_ids = BTreeSet::new();
    let mut surfaces = BTreeSet::new();
    let mut context_by_id = BTreeMap::new();

    if packet.record_kind != PROVIDER_OVERLAY_HANDOFF_PACKET_RECORD_KIND {
        findings.push(error(
            "record_kind",
            "Packet record_kind is not the provider-overlay continuity discriminator.",
        ));
    }
    if packet.schema_version != PROVIDER_OVERLAY_HANDOFF_SCHEMA_VERSION {
        findings.push(error(
            "schema_version",
            "Packet schema_version does not match this crate.",
        ));
    }
    if packet.source_intelligence_packet_ref != SOURCE_INTELLIGENCE_OBJECT_PACKET_RECORD_KIND {
        findings.push(error(
            "source_intelligence_packet_ref",
            "Provider-overlay continuity packet must cite the shared infrastructure object packet.",
        ));
    }
    if packet.boundary_packet_ref != CONTROL_PLANE_BOUNDARY_PACKET_RECORD_KIND {
        findings.push(error(
            "boundary_packet_ref",
            "Provider-overlay continuity packet must cite the shared infra boundary packet.",
        ));
    }
    if packet.support_summary.trim().is_empty() {
        findings.push(error(
            "support_summary",
            "Provider-overlay continuity packet is missing a support summary.",
        ));
    }

    for context in &packet.environment_contexts {
        if !context_ids.insert(context.context_id.clone()) {
            findings.push(error(
                "duplicate_context",
                "Provider-overlay continuity packet repeats the same environment context.",
            ));
        }
        if !context.ambient_context_prohibited {
            findings.push(error(
                "ambient_context",
                "Provider-overlay continuity environment context allows ambient inheritance.",
            ));
        }
        if context.completeness == EnvironmentCompleteness::Incomplete {
            findings.push(error(
                "context_completeness",
                "Provider-overlay continuity environment context is incomplete.",
            ));
        }
        context_by_id.insert(context.context_id.as_str(), context);
    }

    let mut overlay_refs_by_surface = BTreeMap::<OverlayContinuitySurface, BTreeSet<&str>>::new();
    let mut handoff_refs_by_surface = BTreeMap::<OverlayContinuitySurface, BTreeSet<&str>>::new();
    let mut overlay_handoff_refs = BTreeSet::new();

    for row in &packet.overlay_rows {
        if !overlay_row_ids.insert(row.overlay_row_id.clone()) {
            findings.push(error(
                "duplicate_overlay_row",
                "Provider-overlay continuity packet repeats the same overlay row.",
            ));
        }
        if row.overlay_object_ref.trim().is_empty()
            || row.canonical_object_ref.trim().is_empty()
            || row.overlay_relation_ref.trim().is_empty()
            || row.source_label.trim().is_empty()
            || row.support_summary.trim().is_empty()
        {
            findings.push(error(
                "overlay_row_shape",
                "Provider-overlay row is missing required identity or summary fields.",
            ));
        }
        if row.overlay_object_ref == row.canonical_object_ref {
            findings.push(error(
                "overlay_row_shape",
                "Provider-overlay row points at the same overlay and canonical object ref.",
            ));
        }
        if row.canonical_truth_layer == TruthLayer::ProviderOverlay {
            findings.push(error(
                "overlay_truth_disclosure",
                "Provider-overlay row does not preserve a non-overlay canonical truth layer.",
            ));
        }
        if !row.declares_provider_owned_truth || !row.canonical_truth_visible {
            findings.push(error(
                "overlay_truth_disclosure",
                "Provider-overlay row blurs provider-owned overlay truth with canonical truth.",
            ));
        }
        if !context_ids.contains(&row.context_ref) {
            findings.push(error(
                "overlay_context",
                "Provider-overlay row points at an unknown target context.",
            ));
        }
        if row.visible_surfaces.is_empty() {
            findings.push(error(
                "overlay_surface_coverage",
                "Provider-overlay row does not declare any visible surfaces.",
            ));
        }
        for surface in &row.visible_surfaces {
            overlay_refs_by_surface
                .entry(*surface)
                .or_default()
                .insert(row.overlay_row_id.as_str());
        }
        overlay_handoff_refs.insert(row.handoff_ref.as_str());
    }

    for handoff in &packet.handoff_rows {
        if !handoff_ids.insert(handoff.handoff_id.clone()) {
            findings.push(error(
                "duplicate_handoff",
                "Provider-overlay continuity packet repeats the same handoff id.",
            ));
        }
        let Some(context) = context_by_id.get(handoff.target_context_ref.as_str()) else {
            findings.push(error(
                "handoff_context",
                "Provider-overlay continuity handoff points at an unknown target context.",
            ));
            continue;
        };
        findings.extend(validate_control_plane_handoff(handoff, context));
    }

    for row in &packet.overlay_rows {
        if !handoff_ids.contains(row.handoff_ref.as_str()) {
            findings.push(error(
                "overlay_handoff_ref",
                "Provider-overlay row points at an unknown handoff ref.",
            ));
        }
    }

    for binding in &packet.surface_bindings {
        if !surfaces.insert(binding.surface) {
            findings.push(error(
                "duplicate_surface_binding",
                "Provider-overlay continuity packet repeats the same surface binding.",
            ));
        }
        if !context_ids.contains(&binding.context_ref) {
            findings.push(error(
                "surface_context",
                "Surface binding points at an unknown target context.",
            ));
        }
        if !binding.uses_shared_packet
            || !binding.shows_provider_overlay_badge
            || !binding.shows_canonical_truth_reference
            || !binding.shows_control_plane_boundary
            || !binding.shows_handoff_reason
            || !binding.shows_return_anchor
            || !binding.shows_breadcrumbs
            || !binding.preserves_target_identity
            || !binding.return_rehydrates_same_context
        {
            findings.push(error(
                "surface_binding_disclosure",
                "Surface binding hides required overlay, boundary, target, or return disclosure.",
            ));
        }
        if binding.overlay_row_refs.is_empty() || binding.handoff_refs.is_empty() {
            findings.push(error(
                "surface_binding_refs",
                "Surface binding is missing overlay-row refs or handoff refs.",
            ));
        }
        if binding.support_summary.trim().is_empty() {
            findings.push(error(
                "surface_binding_summary",
                "Surface binding is missing a support summary.",
            ));
        }
        for overlay_ref in &binding.overlay_row_refs {
            if !overlay_row_ids.contains(overlay_ref) {
                findings.push(error(
                    "surface_binding_overlay_ref",
                    "Surface binding points at an unknown overlay row.",
                ));
            } else {
                handoff_refs_by_surface.entry(binding.surface).or_default();
            }
        }
        for handoff_ref in &binding.handoff_refs {
            if !handoff_ids.contains(handoff_ref) {
                findings.push(error(
                    "surface_binding_handoff_ref",
                    "Surface binding points at an unknown handoff ref.",
                ));
            } else {
                handoff_refs_by_surface
                    .entry(binding.surface)
                    .or_default()
                    .insert(handoff_ref.as_str());
            }
        }
    }

    for required in [
        OverlayContinuitySurface::CodeBreadcrumb,
        OverlayContinuitySurface::IncidentWorkspace,
        OverlayContinuitySurface::PreviewRoute,
        OverlayContinuitySurface::RouteExplorer,
        OverlayContinuitySurface::InfrastructurePanel,
    ] {
        if !surfaces.contains(&required) {
            findings.push(error(
                "surface_coverage",
                "Provider-overlay continuity packet is missing a required consumer surface.",
            ));
        }
    }

    for (surface, expected_overlay_refs) in overlay_refs_by_surface {
        let Some(binding) = packet
            .surface_bindings
            .iter()
            .find(|binding| binding.surface == surface)
        else {
            findings.push(error(
                "surface_coverage",
                "A declared provider-overlay surface has no matching surface binding.",
            ));
            continue;
        };
        let actual_overlay_refs: BTreeSet<_> = binding
            .overlay_row_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !expected_overlay_refs.is_subset(&actual_overlay_refs) {
            findings.push(error(
                "surface_overlay_parity",
                "Surface binding does not render every provider-overlay row declared for that surface.",
            ));
        }
    }

    for handoff_ref in overlay_handoff_refs {
        let covered_by_surface = packet.surface_bindings.iter().any(|binding| {
            binding
                .handoff_refs
                .iter()
                .any(|binding_handoff_ref| binding_handoff_ref == handoff_ref)
        });
        if !covered_by_surface {
            findings.push(error(
                "handoff_surface_parity",
                "Provider-overlay handoff is not reachable from any disclosed consumer surface.",
            ));
        }
    }

    let passed = findings
        .iter()
        .all(|finding| finding.severity != InfraBoundaryFindingSeverity::Error);
    ProviderOverlayHandoffContinuityValidationReport {
        record_kind:
            "infra_provider_overlay_and_vendor_console_handoff_continuity_validation_report"
                .to_string(),
        schema_version: PROVIDER_OVERLAY_HANDOFF_SCHEMA_VERSION,
        packet_id: packet.packet_id.clone(),
        passed,
        context_ids,
        overlay_row_ids,
        handoff_ids,
        surfaces,
        findings,
    }
}

fn error(check_id: &str, message: &str) -> InfraBoundaryFinding {
    InfraBoundaryFinding {
        severity: InfraBoundaryFindingSeverity::Error,
        check_id: check_id.to_string(),
        message: message.to_string(),
    }
}

/// Returns a deterministic qualified continuity packet for tests and fixtures.
pub fn seeded_provider_overlay_handoff_packet() -> ProviderOverlayHandoffContinuityPacket {
    use crate::target_context_and_control_plane_boundary::{
        ActionPosture, ControlPlaneAuthorityBoundary, ControlPlaneBreadcrumb,
        ControlPlaneHandoffDestinationClass, ControlPlaneReturnAnchor, ControlPlaneReturnSurface,
        ControlPlaneTargetIdentity,
    };

    fn context() -> EnvironmentContext {
        EnvironmentContext {
            context_id: "ctx:kubernetes".to_string(),
            provider: "kubernetes".to_string(),
            account_subscription_project: "payments-prod".to_string(),
            cluster: Some("cluster/payments-prod".to_string()),
            namespace: Some("payments".to_string()),
            region_zone: Some("us-west-2".to_string()),
            tenant: Some("tenant-prod".to_string()),
            workspace_root: "workspace://checkout".to_string(),
            branch_worktree_or_commit: "refs/heads/main".to_string(),
            execution_context_profile: "exec.local.checkout".to_string(),
            toolchain_cli_identity: "helm@3.16.1".to_string(),
            credential_handle_class: "delegated_read_only".to_string(),
            issuance_source: "workspace-secret-broker".to_string(),
            expiry: Some("2026-06-12T21:00:00Z".to_string()),
            write_scope: "read_only".to_string(),
            observed_at: "2026-06-12T20:00:00Z".to_string(),
            completeness: EnvironmentCompleteness::Complete,
            ambient_context_prohibited: true,
            high_risk: false,
        }
    }

    fn target_identity(target_label: &str, stable_target_ref: &str) -> ControlPlaneTargetIdentity {
        ControlPlaneTargetIdentity {
            target_label: target_label.to_string(),
            provider: "kubernetes".to_string(),
            account_subscription_project: "payments-prod".to_string(),
            cluster: Some("cluster/payments-prod".to_string()),
            namespace_or_scope: Some("payments".to_string()),
            region_zone: Some("us-west-2".to_string()),
            tenant: Some("tenant-prod".to_string()),
            stable_target_ref: stable_target_ref.to_string(),
            provider_handle_ref: Some(format!("provider://{stable_target_ref}")),
        }
    }

    fn boundary(label: &str) -> ControlPlaneAuthorityBoundary {
        ControlPlaneAuthorityBoundary {
            boundary_label: label.to_string(),
            aureline_posture: ActionPosture::HandoffOnly,
            destination_authority_label: "Vendor-owned rollout and route authority".to_string(),
            control_plane_boundary_disclosed: true,
            vendor_owned_authority: true,
            overlay_cannot_become_canonical_truth: true,
        }
    }

    fn return_anchor(
        surface: ControlPlaneReturnSurface,
        object_ref: &str,
        route_or_view_ref: &str,
        restore_action_label: &str,
    ) -> ControlPlaneReturnAnchor {
        ControlPlaneReturnAnchor {
            anchor_id: format!("anchor:{object_ref}"),
            surface,
            context_ref: "ctx:kubernetes".to_string(),
            primary_object_ref: object_ref.to_string(),
            route_or_view_ref: route_or_view_ref.to_string(),
            restore_action_label: restore_action_label.to_string(),
            generic_shell_reopen_forbidden: true,
        }
    }

    fn breadcrumbs(
        entries: &[(u32, &str, ControlPlaneReturnSurface, &str)],
    ) -> Vec<ControlPlaneBreadcrumb> {
        entries
            .iter()
            .map(
                |(position, label, surface, object_ref)| ControlPlaneBreadcrumb {
                    breadcrumb_id: format!("crumb:{position}:{object_ref}"),
                    position: *position,
                    label: (*label).to_string(),
                    surface: *surface,
                    context_ref: "ctx:kubernetes".to_string(),
                    object_ref: (*object_ref).to_string(),
                },
            )
            .collect()
    }

    fn handoff(
        handoff_id: &str,
        destination: &str,
        reason: ControlPlaneHandoffReason,
        destination_class: ControlPlaneHandoffDestinationClass,
        target_label: &str,
        stable_target_ref: &str,
        boundary_label: &str,
        return_anchor: ControlPlaneReturnAnchor,
        breadcrumbs: Vec<ControlPlaneBreadcrumb>,
    ) -> ControlPlaneHandoff {
        ControlPlaneHandoff {
            handoff_id: handoff_id.to_string(),
            destination: destination.to_string(),
            handoff_reason: reason,
            destination_class,
            target_context_ref: "ctx:kubernetes".to_string(),
            connector_class: crate::ConnectorClass::ProviderConsoleOverlay,
            target_identity: target_identity(target_label, stable_target_ref),
            authority_boundary: boundary(boundary_label),
            explicit_handoff_destination: true,
            not_substitute_truth: true,
            return_or_revocation_path: format!("aureline://return/{handoff_id}"),
            return_anchor,
            breadcrumbs,
            audit_ref: format!("audit:{handoff_id}"),
        }
    }

    fn overlay_row(
        overlay_row_id: &str,
        canonical_object_ref: &str,
        canonical_truth_layer: TruthLayer,
        handoff_reason: ControlPlaneHandoffReason,
        freshness: FreshnessLabel,
        source_label: &str,
        handoff_ref: &str,
        visible_surfaces: &[OverlayContinuitySurface],
        support_summary: &str,
    ) -> ProviderOverlayDisclosureRow {
        ProviderOverlayDisclosureRow {
            overlay_row_id: overlay_row_id.to_string(),
            family: InfrastructureFamily::KubernetesHelm,
            overlay_object_ref: "obj:k8s:overlay".to_string(),
            canonical_object_ref: canonical_object_ref.to_string(),
            overlay_relation_ref: "rel:k8s:overlay_of".to_string(),
            canonical_truth_layer,
            context_ref: "ctx:kubernetes".to_string(),
            handoff_reason,
            freshness,
            source_label: source_label.to_string(),
            declares_provider_owned_truth: true,
            canonical_truth_visible: true,
            handoff_ref: handoff_ref.to_string(),
            visible_surfaces: visible_surfaces.to_vec(),
            support_summary: support_summary.to_string(),
        }
    }

    fn surface_binding(
        surface: OverlayContinuitySurface,
        overlay_row_refs: &[&str],
        handoff_refs: &[&str],
        support_summary: &str,
    ) -> OverlayContinuitySurfaceBinding {
        OverlayContinuitySurfaceBinding {
            surface,
            context_ref: "ctx:kubernetes".to_string(),
            overlay_row_refs: overlay_row_refs
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
            handoff_refs: handoff_refs
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
            shows_provider_overlay_badge: true,
            shows_canonical_truth_reference: true,
            shows_control_plane_boundary: true,
            shows_handoff_reason: true,
            shows_return_anchor: true,
            shows_breadcrumbs: true,
            preserves_target_identity: true,
            return_rehydrates_same_context: true,
            uses_shared_packet: true,
            support_summary: support_summary.to_string(),
        }
    }

    ProviderOverlayHandoffContinuityPacket {
        record_kind: PROVIDER_OVERLAY_HANDOFF_PACKET_RECORD_KIND.to_string(),
        schema_version: PROVIDER_OVERLAY_HANDOFF_SCHEMA_VERSION,
        packet_id: "infra-provider-overlay-continuity:checkout".to_string(),
        captured_at: "2026-06-12T20:15:00Z".to_string(),
        source_intelligence_packet_ref: SOURCE_INTELLIGENCE_OBJECT_PACKET_RECORD_KIND.to_string(),
        boundary_packet_ref: CONTROL_PLANE_BOUNDARY_PACKET_RECORD_KIND.to_string(),
        environment_contexts: vec![context()],
        overlay_rows: vec![
            overlay_row(
                "overlay-row:route-status",
                "obj:k8s:observed",
                TruthLayer::ObservedLive,
                ControlPlaneHandoffReason::PreviewOrRouteContinuation,
                FreshnessLabel::Partial,
                "Provider route status overlay",
                "handoff:route-status",
                &[
                    OverlayContinuitySurface::CodeBreadcrumb,
                    OverlayContinuitySurface::PreviewRoute,
                    OverlayContinuitySurface::RouteExplorer,
                    OverlayContinuitySurface::InfrastructurePanel,
                ],
                "Route status overlay stays explicit and links back to the same cluster target.",
            ),
            overlay_row(
                "overlay-row:incident-rollout",
                "obj:k8s:observed",
                TruthLayer::ObservedLive,
                ControlPlaneHandoffReason::IncidentRunbookContinuation,
                FreshnessLabel::Live,
                "Provider rollout event overlay",
                "handoff:incident-rollout",
                &[
                    OverlayContinuitySurface::IncidentWorkspace,
                    OverlayContinuitySurface::InfrastructurePanel,
                ],
                "Incident rollout overlay stays explicit and preserves the incident return path.",
            ),
            overlay_row(
                "overlay-row:policy-annotation",
                "obj:k8s:planned",
                TruthLayer::PlannedValidated,
                ControlPlaneHandoffReason::ProviderOverlayInspection,
                FreshnessLabel::CurrentSnapshot,
                "Provider policy annotation overlay",
                "handoff:policy-annotation",
                &[
                    OverlayContinuitySurface::CodeBreadcrumb,
                    OverlayContinuitySurface::InfrastructurePanel,
                ],
                "Policy annotation overlay stays explicit and keeps the manifest-side breadcrumb trail.",
            ),
        ],
        handoff_rows: vec![
            handoff(
                "handoff:route-status",
                "Provider route detail page",
                ControlPlaneHandoffReason::PreviewOrRouteContinuation,
                crate::target_context_and_control_plane_boundary::ControlPlaneHandoffDestinationClass::ProviderOwnedPage,
                "checkout route",
                "obj:k8s:observed",
                "Provider route detail owns the live route overlay and any follow-up mutation.",
                return_anchor(
                    crate::target_context_and_control_plane_boundary::ControlPlaneReturnSurface::PreviewRoute,
                    "obj:k8s:observed",
                    "preview://checkout-route",
                    "Return to preview route",
                ),
                breadcrumbs(&[
                    (
                        1,
                        "checkout manifest",
                        crate::target_context_and_control_plane_boundary::ControlPlaneReturnSurface::CodeBreadcrumb,
                        "obj:k8s:authored",
                    ),
                    (
                        2,
                        "route explorer",
                        crate::target_context_and_control_plane_boundary::ControlPlaneReturnSurface::RouteExplorer,
                        "obj:k8s:observed",
                    ),
                    (
                        3,
                        "preview route",
                        crate::target_context_and_control_plane_boundary::ControlPlaneReturnSurface::PreviewRoute,
                        "obj:k8s:observed",
                    ),
                ]),
            ),
            handoff(
                "handoff:incident-rollout",
                "Vendor rollout console",
                ControlPlaneHandoffReason::IncidentRunbookContinuation,
                crate::target_context_and_control_plane_boundary::ControlPlaneHandoffDestinationClass::VendorConsole,
                "checkout rollout",
                "obj:k8s:observed",
                "Vendor rollout console owns live rollout controls during the incident step.",
                return_anchor(
                    crate::target_context_and_control_plane_boundary::ControlPlaneReturnSurface::IncidentWorkspace,
                    "obj:k8s:observed",
                    "incident://checkout-rollout",
                    "Return to incident workspace",
                ),
                breadcrumbs(&[
                    (
                        1,
                        "incident workspace",
                        crate::target_context_and_control_plane_boundary::ControlPlaneReturnSurface::IncidentWorkspace,
                        "obj:k8s:observed",
                    ),
                    (
                        2,
                        "infrastructure panel",
                        crate::target_context_and_control_plane_boundary::ControlPlaneReturnSurface::InfrastructurePanel,
                        "obj:k8s:observed",
                    ),
                ]),
            ),
            handoff(
                "handoff:policy-annotation",
                "Provider policy annotation page",
                ControlPlaneHandoffReason::ProviderOverlayInspection,
                crate::target_context_and_control_plane_boundary::ControlPlaneHandoffDestinationClass::ProviderRunbookPage,
                "checkout policy annotation",
                "obj:k8s:planned",
                "Provider policy annotation page supplies overlay detail but does not replace manifest truth.",
                return_anchor(
                    crate::target_context_and_control_plane_boundary::ControlPlaneReturnSurface::CodeBreadcrumb,
                    "obj:k8s:planned",
                    "code://k8s/checkout-policy",
                    "Return to code breadcrumb",
                ),
                breadcrumbs(&[
                    (
                        1,
                        "checkout manifest",
                        crate::target_context_and_control_plane_boundary::ControlPlaneReturnSurface::CodeBreadcrumb,
                        "obj:k8s:authored",
                    ),
                    (
                        2,
                        "planned policy view",
                        crate::target_context_and_control_plane_boundary::ControlPlaneReturnSurface::InfrastructurePanel,
                        "obj:k8s:planned",
                    ),
                ]),
            ),
        ],
        surface_bindings: vec![
            surface_binding(
                OverlayContinuitySurface::CodeBreadcrumb,
                &["overlay-row:route-status", "overlay-row:policy-annotation"],
                &["handoff:route-status", "handoff:policy-annotation"],
                "Code breadcrumbs preserve overlay badges, canonical truth, and return-safe handoffs.",
            ),
            surface_binding(
                OverlayContinuitySurface::IncidentWorkspace,
                &["overlay-row:incident-rollout"],
                &["handoff:incident-rollout"],
                "Incident workspace preserves the same rollout overlay and handoff return anchor.",
            ),
            surface_binding(
                OverlayContinuitySurface::PreviewRoute,
                &["overlay-row:route-status"],
                &["handoff:route-status"],
                "Preview route preserves route overlay truth and the same return-safe handoff.",
            ),
            surface_binding(
                OverlayContinuitySurface::RouteExplorer,
                &["overlay-row:route-status"],
                &["handoff:route-status"],
                "Route explorer preserves route overlay truth and the same vendor-page handoff.",
            ),
            surface_binding(
                OverlayContinuitySurface::InfrastructurePanel,
                &[
                    "overlay-row:route-status",
                    "overlay-row:incident-rollout",
                    "overlay-row:policy-annotation",
                ],
                &[
                    "handoff:route-status",
                    "handoff:incident-rollout",
                    "handoff:policy-annotation",
                ],
                "Infrastructure panel keeps all overlay rows and handoffs explicit instead of flattening them.",
            ),
        ],
        support_summary:
            "Provider overlays remain explicit, vendor-console handoffs remain attributable, and return paths preserve exact target context across code, incident, preview, route, and infrastructure surfaces."
                .to_string(),
    }
}
