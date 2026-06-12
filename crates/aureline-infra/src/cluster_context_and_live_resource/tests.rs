//! Unit coverage for cluster-context and live-resource packets.

use super::*;
use crate::target_context_and_control_plane_boundary::EnvironmentCompleteness;

fn context() -> EnvironmentContext {
    EnvironmentContext {
        context_id: "envctx:k8s:payments-prod:checkout".to_string(),
        provider: "kubernetes".to_string(),
        account_subscription_project: "project:payments-prod".to_string(),
        cluster: Some("cluster:payments-us-1".to_string()),
        namespace: Some("namespace:checkout".to_string()),
        region_zone: Some("us-west-2".to_string()),
        tenant: Some("tenant:payments".to_string()),
        workspace_root: "workspace:aureline".to_string(),
        branch_worktree_or_commit: "commit:9f4c1d0".to_string(),
        execution_context_profile: "exec-profile:managed-ops".to_string(),
        toolchain_cli_identity: "kubectl:v1.30.2".to_string(),
        credential_handle_class: "managed_connector_handle".to_string(),
        issuance_source: "managed_control_plane".to_string(),
        expiry: Some("2026-06-04T19:40:00Z".to_string()),
        write_scope: "namespace_bound_pending_approval".to_string(),
        observed_at: "2026-06-04T19:09:30Z".to_string(),
        completeness: EnvironmentCompleteness::Complete,
        ambient_context_prohibited: true,
        high_risk: true,
    }
}

fn strip() -> ClusterContextStrip {
    ClusterContextStrip {
        strip_id: "strip:checkout".to_string(),
        context_ref: "envctx:k8s:payments-prod:checkout".to_string(),
        provider: "kubernetes".to_string(),
        account_subscription: "project:payments-prod".to_string(),
        project: Some("project:payments-prod".to_string()),
        cluster: Some("cluster:payments-us-1".to_string()),
        namespace: Some("namespace:checkout".to_string()),
        region: Some("us-west-2".to_string()),
        tenant: Some("tenant:payments".to_string()),
        execution_origin: "managed agent".to_string(),
        credential_class: "managed_connector_handle".to_string(),
    }
}

fn all_mode_views() -> Vec<TruthModeView> {
    vec![
        view(TruthMode::Desired, FreshnessLabel::CurrentSnapshot, false),
        view(TruthMode::Rendered, FreshnessLabel::CurrentSnapshot, false),
        view(TruthMode::Plan, FreshnessLabel::CurrentSnapshot, false),
        view(TruthMode::Live, FreshnessLabel::Live, true),
        view(TruthMode::ProviderOverlay, FreshnessLabel::Partial, false),
    ]
}

fn view(mode: TruthMode, freshness: FreshnessLabel, mutation_capable: bool) -> TruthModeView {
    TruthModeView {
        truth_mode: mode,
        source_label: "source".to_string(),
        source_ref: "ref:source".to_string(),
        freshness,
        observed_at: Some("2026-06-04T19:09:30Z".to_string()),
        blended_with_other_modes: false,
        mutation_capable,
    }
}

fn projection(surface: OpsSurface, tool: OpsToolKind) -> OpsSurfaceProjection {
    OpsSurfaceProjection {
        surface,
        tool_kind: tool,
        context_strip: strip(),
        truth_mode_views: all_mode_views(),
        preserves_truth_vocabulary: true,
        uses_shared_packet: true,
    }
}

fn handoff_truth() -> ConsoleHandoffTruth {
    ConsoleHandoffTruth {
        surface: OpsSurface::BrowserConsoleHandoff,
        tool_kind: OpsToolKind::Kubernetes,
        aureline_is_authoritative: false,
        handoff: ControlPlaneHandoff {
            handoff_id: "handoff:console:checkout".to_string(),
            destination: "Provider console".to_string(),
            target_context_ref: "envctx:k8s:payments-prod:checkout".to_string(),
            connector_class: ConnectorClass::ProviderConsoleOverlay,
            explicit_handoff_destination: true,
            not_substitute_truth: true,
            return_or_revocation_path: "aureline://return/handoff/console".to_string(),
            audit_ref: "audit:handoff:console".to_string(),
        },
    }
}

fn packet() -> ClusterLiveResourcePacket {
    ClusterLiveResourcePacket {
        record_kind: CLUSTER_LIVE_RESOURCE_PACKET_RECORD_KIND.to_string(),
        schema_version: CLUSTER_LIVE_RESOURCE_SCHEMA_VERSION,
        packet_id: "infra-cluster:payments-prod:checkout".to_string(),
        captured_at: "2026-06-04T19:10:00Z".to_string(),
        environment_context: context(),
        surface_projections: vec![
            projection(OpsSurface::TerraformPlanReview, OpsToolKind::Terraform),
            projection(OpsSurface::KubernetesResourceView, OpsToolKind::Kubernetes),
            projection(OpsSurface::ClusterLiveResource, OpsToolKind::Kubernetes),
            projection(
                OpsSurface::IncidentRunbookStep,
                OpsToolKind::IncidentAdjacent,
            ),
            projection(
                OpsSurface::SupportRunbookExport,
                OpsToolKind::IncidentAdjacent,
            ),
            OpsSurfaceProjection {
                truth_mode_views: vec![
                    view(TruthMode::ProviderOverlay, FreshnessLabel::Partial, false),
                    view(TruthMode::Live, FreshnessLabel::CurrentSnapshot, false),
                ],
                ..projection(OpsSurface::BrowserConsoleHandoff, OpsToolKind::Kubernetes)
            },
        ],
        action_gates: vec![MutatingActionGate {
            gate_id: "gate:apply".to_string(),
            surface: OpsSurface::TerraformPlanReview,
            tool_kind: OpsToolKind::Terraform,
            action_kind: ActionKind::Mutate,
            context_ref: "envctx:k8s:payments-prod:checkout".to_string(),
            requires_reviewed_preview: true,
            preview_ref: Some("preview:apply".to_string()),
            target_preview_ref: Some("target:apply".to_string()),
            source_of_truth_posture: "plan is source of truth".to_string(),
            handoff_ref: None,
            approved: true,
        }],
        console_handoffs: vec![handoff_truth()],
        support_summary: "Stable cluster-context packet.".to_string(),
    }
}

#[test]
fn valid_packet_passes() {
    let report = packet().validate();
    assert!(report.passed, "expected pass: {:#?}", report.findings);
    assert_eq!(report.truth_modes.len(), 5);
    assert!(report.tool_kinds.contains(&OpsToolKind::Terraform));
    assert!(report.tool_kinds.contains(&OpsToolKind::Kubernetes));
    assert!(report.tool_kinds.contains(&OpsToolKind::IncidentAdjacent));
}

#[test]
fn blended_view_is_rejected() {
    let mut pkt = packet();
    pkt.surface_projections[1].truth_mode_views[0].blended_with_other_modes = true;
    let report = pkt.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|f| f.check_id == "blended_truth_mode"));
}

#[test]
fn non_live_mutation_capable_is_rejected() {
    let mut pkt = packet();
    pkt.surface_projections[0].truth_mode_views[0].mutation_capable = true;
    let report = pkt.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|f| f.check_id == "mutation_mode"));
}

#[test]
fn stale_live_view_cannot_mutate() {
    let mut pkt = packet();
    // Live view in the cluster surface goes stale but is still mutation-capable.
    let live = pkt.surface_projections[2]
        .truth_mode_views
        .iter_mut()
        .find(|v| v.truth_mode == TruthMode::Live)
        .unwrap();
    live.freshness = FreshnessLabel::Stale;
    let report = pkt.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|f| f.check_id == "mutation_freshness"));
}

#[test]
fn missing_truth_mode_is_rejected() {
    let mut pkt = packet();
    pkt.surface_projections[0].truth_mode_views.pop();
    let report = pkt.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|f| f.check_id == "truth_mode_coverage"));
}

#[test]
fn boundary_action_without_preview_is_rejected() {
    let mut pkt = packet();
    pkt.action_gates[0].preview_ref = None;
    pkt.action_gates[0].handoff_ref = None;
    let report = pkt.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|f| f.check_id == "gate_preview_or_handoff"));
}

#[test]
fn high_risk_mutation_requires_preview_ref() {
    let mut pkt = packet();
    // Keep a handoff so the preview-or-handoff rule passes, but drop the preview.
    pkt.action_gates[0].preview_ref = None;
    pkt.action_gates[0].handoff_ref = Some("handoff:apply".to_string());
    let report = pkt.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|f| f.check_id == "gate_high_risk_preview"));
}

#[test]
fn console_launch_requires_handoff_ref() {
    let mut pkt = packet();
    pkt.action_gates.push(MutatingActionGate {
        gate_id: "gate:console".to_string(),
        surface: OpsSurface::BrowserConsoleHandoff,
        tool_kind: OpsToolKind::Kubernetes,
        action_kind: ActionKind::BrowserConsoleLaunch,
        context_ref: "envctx:k8s:payments-prod:checkout".to_string(),
        requires_reviewed_preview: true,
        preview_ref: Some("preview:console".to_string()),
        target_preview_ref: Some("target:console".to_string()),
        source_of_truth_posture: "console is vendor authoritative".to_string(),
        handoff_ref: None,
        approved: false,
    });
    let report = pkt.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|f| f.check_id == "gate_console_handoff"));
}

#[test]
fn handoff_claiming_authority_is_rejected() {
    let mut pkt = packet();
    pkt.console_handoffs[0].aureline_is_authoritative = true;
    let report = pkt.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|f| f.check_id == "handoff_authority"));
}

#[test]
fn wrong_target_strip_is_rejected() {
    let mut pkt = packet();
    pkt.surface_projections[1].context_strip.cluster = Some("cluster:other".to_string());
    let report = pkt.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|f| f.check_id == "strip_identity"));
}

#[test]
fn ambient_context_must_be_prohibited() {
    let mut pkt = packet();
    pkt.environment_context.ambient_context_prohibited = false;
    let report = pkt.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|f| f.check_id == "ambient_context"));
}
