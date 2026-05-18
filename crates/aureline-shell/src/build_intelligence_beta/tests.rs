use super::*;

use aureline_runtime::{
    AdapterHealthReason, AdapterHealthState, AdapterHealthStrip, AdapterIdentity,
    ArtifactSourceClass, BuildIntelligenceAction, BuildIntelligenceActionClass,
    BuildIntelligenceLaneType, BuildIntelligenceReceipt, BuildIntelligenceRunConfigCard,
    BuildIntelligenceSupportExport, BuildIntelligenceTargetRow, DiscoveryDiffReview,
    HighTrustActionPosture, ImportedLiveState, RefreshLineage, TargetExactnessStatus,
};

fn action(class: BuildIntelligenceActionClass, suffix: &str) -> BuildIntelligenceAction {
    BuildIntelligenceAction::enabled(class, format!("action:shell:{suffix}"))
}

fn lineage(refresh: &str, previous: Option<&str>) -> RefreshLineage {
    RefreshLineage::new(
        refresh.to_owned(),
        previous.map(str::to_owned),
        "2026-05-18T14:00:00Z",
        Some("2026-05-18T14:00:05Z".to_owned()),
    )
    .with_refs(
        Some(format!("snapshot:{refresh}")),
        Some(format!("raw:{refresh}")),
        Some(format!("inspection:{refresh}")),
        None,
    )
}

#[test]
fn panel_preserves_health_target_receipt_and_diff_truth() {
    let native_strip = AdapterHealthStrip::new(
        "strip:native",
        "workspace:shell",
        BuildIntelligenceLaneType::NativeAdapter,
        AdapterIdentity::new(
            "adapter:cargo",
            "Cargo adapter",
            Some("1.0".to_owned()),
            None,
            None,
        ),
        AdapterHealthState::Healthy,
        None,
        Some("2026-05-18T14:00:05Z".to_owned()),
        ImportedLiveState::LiveWorkspaceInspection,
        lineage("refresh:current", Some("refresh:previous")),
        action(BuildIntelligenceActionClass::RefreshDiscovery, "refresh"),
        action(BuildIntelligenceActionClass::OpenDetails, "details"),
    );
    let managed_strip = AdapterHealthStrip::new(
        "strip:managed",
        "workspace:shell",
        BuildIntelligenceLaneType::StructuredProtocol,
        AdapterIdentity::new(
            "adapter:managed",
            "Managed workspace protocol",
            Some("2.0".to_owned()),
            Some("aureline-managed".to_owned()),
            Some("1".to_owned()),
        ),
        AdapterHealthState::Partial,
        Some(AdapterHealthReason::ControlPlaneOutage),
        Some("2026-05-18T13:59:00Z".to_owned()),
        ImportedLiveState::MixedLiveAndImported,
        lineage("refresh:current", Some("refresh:previous")),
        action(
            BuildIntelligenceActionClass::RefreshDiscovery,
            "managed-refresh",
        ),
        action(BuildIntelligenceActionClass::OpenDetails, "managed-details"),
    )
    .with_continuation_actions(
        Some(action(
            BuildIntelligenceActionClass::ContinueLocal,
            "continue-local",
        )),
        Some(action(
            BuildIntelligenceActionClass::InspectOnly,
            "inspect-only",
        )),
    );
    let previous_row = BuildIntelligenceTargetRow::new(
        "row:web:previous",
        "target:web",
        "web",
        BuildIntelligenceLaneType::NativeAdapter,
        "strip:native",
        TargetExactnessStatus::Exact,
        ImportedLiveState::LiveWorkspaceInspection,
        "current live workspace inspection",
        lineage("refresh:previous", None),
    );
    let current_row = BuildIntelligenceTargetRow::new(
        "row:web:current",
        "target:web",
        "web:test",
        BuildIntelligenceLaneType::HeuristicFallback,
        "strip:managed",
        TargetExactnessStatus::Heuristic,
        ImportedLiveState::HeuristicInference,
        "heuristic fallback; review before rerun",
        lineage("refresh:current", Some("refresh:previous")),
    );
    let card = BuildIntelligenceRunConfigCard::from_target_row(
        "card:web",
        "task.run.web",
        &current_row,
        HighTrustActionPosture::ReviewBeforeDispatch,
    );
    let receipt = BuildIntelligenceReceipt::from_target_row(
        "receipt:web",
        "task.run.web",
        "run:web:1",
        &current_row,
        &managed_strip,
        "managed/workspace",
        ArtifactSourceClass::HeuristicInference,
        None,
        "heuristic receipt; not a current live adapter result",
        HighTrustActionPosture::ReviewBeforeDispatch,
    );
    let diff = DiscoveryDiffReview::between(
        "diff:web",
        "workspace:shell",
        "refresh:previous",
        "refresh:current",
        "2026-05-18T14:01:00Z",
        &[previous_row],
        std::slice::from_ref(&current_row),
    );
    let export = BuildIntelligenceSupportExport::new(
        "support:build-intelligence:shell",
        "workspace:shell",
        "2026-05-18T14:02:00Z",
        vec![native_strip, managed_strip],
        vec![current_row],
        vec![card],
        vec![receipt],
        vec![diff],
    );
    let panel = BuildIntelligenceBetaPanel::from_support_export(&export);

    assert_eq!(panel.record_kind, BUILD_INTELLIGENCE_BETA_PANEL_RECORD_KIND);
    assert_eq!(panel.health_rows.len(), 2);
    assert_eq!(panel.target_rows.len(), 1);
    assert_eq!(panel.receipt_rows.len(), 1);
    assert_eq!(panel.diff_rows.len(), 1);
    let managed = panel
        .health_rows
        .iter()
        .find(|row| row.strip_id == "strip:managed")
        .expect("managed health row");
    assert_eq!(
        managed.health_reason_token.as_deref(),
        Some("control_plane_outage")
    );
    assert_eq!(
        managed.continue_local_action_ref.as_deref(),
        Some("action:shell:continue-local")
    );

    let plaintext = panel.render_plaintext();
    assert!(plaintext.contains("lane=heuristic_fallback"));
    assert!(plaintext.contains("provenance=heuristic_inference"));
    assert!(plaintext.contains("posture=review_before_dispatch"));
    assert!(plaintext.contains("newly_heuristic=1"));
    assert!(!plaintext.contains("/Users/"));
}
