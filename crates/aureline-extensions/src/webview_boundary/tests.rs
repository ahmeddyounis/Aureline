//! Unit and fixture coverage for extension webview boundary audits.

use super::{
    audit_extension_webview_boundary_rows, evaluate_extension_webview_boundary_row,
    project_extension_webview_boundary_support_export,
    project_extension_webview_boundary_support_row, seeded_extension_webview_boundary_audit_packet,
    validate_extension_webview_boundary_packet, validate_extension_webview_boundary_support_export,
    ExtensionBrowserHandoffPostureClass, ExtensionEmbeddedSurfaceClass,
    ExtensionHostAuthorityScopeClass, ExtensionHostChromeControlClass, ExtensionInheritanceClass,
    ExtensionNativeApprovalBoundaryClass, ExtensionSurfaceTrustClass,
    ExtensionWebviewBoundaryAuditPacket, ExtensionWebviewBoundaryDefectKind,
    ExtensionWebviewBoundaryInput, ExtensionWebviewBoundarySupportExport,
    EXTENSION_WEBVIEW_BOUNDARY_SHARED_CONTRACT_REF,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/extensions/m3/webview_boundary_audit"
);

fn load_json<T: serde::de::DeserializeOwned>(filename: &str) -> T {
    let path = format!("{}/{}", FIXTURE_DIR, filename);
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn seeded_packet_has_zero_defects_and_required_surface_coverage() {
    let packet = seeded_extension_webview_boundary_audit_packet();
    validate_extension_webview_boundary_packet(&packet).expect("seeded packet must validate");
    assert_eq!(
        packet.shared_contract_ref,
        EXTENSION_WEBVIEW_BOUNDARY_SHARED_CONTRACT_REF
    );
    assert_eq!(packet.summary.row_count, 6);
    assert_eq!(packet.summary.defect_count, 0);
    assert_eq!(packet.summary.conformant_row_count, 6);
    assert_eq!(packet.summary.safe_browser_baseline_required_row_count, 5);
    assert_eq!(packet.summary.safe_browser_baseline_satisfied_row_count, 5);
    assert_eq!(packet.summary.trust_class_parity_row_count, 6);
    assert_eq!(packet.summary.host_owned_native_approval_row_count, 6);

    for required in [
        ExtensionEmbeddedSurfaceClass::ExtensionWebviewPanel,
        ExtensionEmbeddedSurfaceClass::HostedDashboard,
        ExtensionEmbeddedSurfaceClass::ProviderAuthSurface,
        ExtensionEmbeddedSurfaceClass::BrowserRuntimeBridge,
        ExtensionEmbeddedSurfaceClass::DocumentationProviderPane,
    ] {
        assert!(
            packet.summary.surface_classes_present.contains(&required),
            "seeded packet must cover {required:?}"
        );
    }
}

#[test]
fn support_export_replays_same_visible_findings_without_private_material() {
    let packet = seeded_extension_webview_boundary_audit_packet();
    let export = project_extension_webview_boundary_support_export(
        &packet,
        "extension-webview-boundary:support-export:default",
        "2026-05-16T00:00:00Z",
    );
    validate_extension_webview_boundary_support_export(&packet, &export)
        .expect("support export must validate");
    assert!(export.raw_private_material_excluded);
    assert_eq!(export.support_rows.len(), packet.rows.len());
    for row in &packet.rows {
        let support = export
            .support_rows
            .iter()
            .find(|support| support.row_ref == row.row_id)
            .unwrap_or_else(|| panic!("missing support row for {}", row.row_id));
        assert_eq!(
            support.visible_boundary_finding_refs,
            row.visible_boundary_finding_refs
        );
        assert_eq!(support.host_chrome_trust_class, row.host_chrome_trust_class);
        assert_eq!(
            support.browser_handoff_posture_class,
            row.browser_handoff_posture_class
        );
    }
}

#[test]
fn checked_packet_fixture_matches_seeded_builder() {
    let from_file: ExtensionWebviewBoundaryAuditPacket = load_json("audit_packet.json");
    let from_code = seeded_extension_webview_boundary_audit_packet();
    assert_eq!(from_file, from_code);
}

#[test]
fn checked_support_export_fixture_matches_seeded_builder() {
    let from_file: ExtensionWebviewBoundarySupportExport = load_json("support_export.json");
    let packet = seeded_extension_webview_boundary_audit_packet();
    let from_code = project_extension_webview_boundary_support_export(
        &packet,
        "extension-webview-boundary:support-export:default",
        "2026-05-16T00:00:00Z",
    );
    assert_eq!(from_file, from_code);
}

#[test]
fn input_fixtures_project_to_checked_packet() {
    let inputs: Vec<ExtensionWebviewBoundaryInput> = load_json("inputs.json");
    let rows = inputs
        .into_iter()
        .map(evaluate_extension_webview_boundary_row)
        .collect();
    let from_inputs = ExtensionWebviewBoundaryAuditPacket::from_rows(
        "extension-webview-boundary:audit:beta:default",
        "2026-05-16T00:00:00Z",
        rows,
    );
    let checked: ExtensionWebviewBoundaryAuditPacket = load_json("audit_packet.json");
    assert_eq!(from_inputs, checked);
}

#[test]
fn validator_flags_missing_owner_origin_chrome() {
    let mut packet = seeded_extension_webview_boundary_audit_packet();
    packet.rows[0].publisher_label.clear();
    let support_rows = packet
        .rows
        .iter()
        .map(project_extension_webview_boundary_support_row)
        .collect::<Vec<_>>();
    let defects = audit_extension_webview_boundary_rows(&packet.rows, &support_rows);
    assert!(defects
        .iter()
        .any(|defect| defect.defect_kind
            == ExtensionWebviewBoundaryDefectKind::OwnerOriginChromeMissing));
}

#[test]
fn validator_flags_risky_provider_flow_without_system_browser_default() {
    let mut packet = seeded_extension_webview_boundary_audit_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface_class == ExtensionEmbeddedSurfaceClass::ProviderAuthSurface)
        .expect("seeded auth row exists");
    row.browser_handoff_posture_class = ExtensionBrowserHandoffPostureClass::InProductOnly;
    row.safe_browser_baseline_satisfied = false;
    let support_rows = packet
        .rows
        .iter()
        .map(project_extension_webview_boundary_support_row)
        .collect::<Vec<_>>();
    let defects = audit_extension_webview_boundary_rows(&packet.rows, &support_rows);
    assert!(defects.iter().any(|defect| defect.defect_kind
        == ExtensionWebviewBoundaryDefectKind::RiskySurfaceWithoutSafeBrowserBaseline));
}

#[test]
fn validator_flags_missing_browser_handoff_packet_on_system_browser_row() {
    let mut packet = seeded_extension_webview_boundary_audit_packet();
    packet.rows[1].browser_handoff_packet_ref = None;
    let support_rows = packet
        .rows
        .iter()
        .map(project_extension_webview_boundary_support_row)
        .collect::<Vec<_>>();
    let defects = audit_extension_webview_boundary_rows(&packet.rows, &support_rows);
    assert!(defects.iter().any(|defect| defect.defect_kind
        == ExtensionWebviewBoundaryDefectKind::BrowserHandoffPacketMissing));
}

#[test]
fn validator_flags_embedded_native_approval_attempt_and_unbounded_authority() {
    let mut packet = seeded_extension_webview_boundary_audit_packet();
    packet.rows[0].native_approval_boundary_class =
        ExtensionNativeApprovalBoundaryClass::EmbeddedSurfaceAttemptedApproval;
    packet.rows[0].host_authority_scope_class =
        ExtensionHostAuthorityScopeClass::UnboundedHostAuthority;
    let support_rows = packet
        .rows
        .iter()
        .map(project_extension_webview_boundary_support_row)
        .collect::<Vec<_>>();
    let defects = audit_extension_webview_boundary_rows(&packet.rows, &support_rows);
    assert!(defects.iter().any(|defect| defect.defect_kind
        == ExtensionWebviewBoundaryDefectKind::EmbeddedNativeApprovalAttempt));
    assert!(defects
        .iter()
        .any(|defect| defect.defect_kind
            == ExtensionWebviewBoundaryDefectKind::UnboundedHostAuthority));
}

#[test]
fn validator_flags_trust_and_support_export_parity_drift() {
    let mut packet = seeded_extension_webview_boundary_audit_packet();
    packet.rows[0].embedded_content_trust_class = ExtensionSurfaceTrustClass::Untrusted;
    let mut support_rows = packet
        .rows
        .iter()
        .map(project_extension_webview_boundary_support_row)
        .collect::<Vec<_>>();
    support_rows[0].publisher_label = "Different publisher".to_owned();
    let defects = audit_extension_webview_boundary_rows(&packet.rows, &support_rows);
    assert!(defects
        .iter()
        .any(|defect| defect.defect_kind
            == ExtensionWebviewBoundaryDefectKind::TrustClassParityDrift));
    assert!(defects
        .iter()
        .any(|defect| defect.defect_kind
            == ExtensionWebviewBoundaryDefectKind::SupportExportParityDrift));
}

#[test]
fn validator_flags_missing_chrome_and_appearance_disclosure() {
    let mut packet = seeded_extension_webview_boundary_audit_packet();
    packet.rows[0]
        .host_chrome_controls
        .retain(|control| *control != ExtensionHostChromeControlClass::OriginLabel);
    packet.rows[0].appearance_inheritance.theme_class = ExtensionInheritanceClass::NotDisclosed;
    let support_rows = packet
        .rows
        .iter()
        .map(project_extension_webview_boundary_support_row)
        .collect::<Vec<_>>();
    let defects = audit_extension_webview_boundary_rows(&packet.rows, &support_rows);
    assert!(defects
        .iter()
        .any(|defect| defect.defect_kind
            == ExtensionWebviewBoundaryDefectKind::HostChromeControlMissing));
    assert!(defects.iter().any(|defect| defect.defect_kind
        == ExtensionWebviewBoundaryDefectKind::AppearanceInheritanceDisclosureMissing));
}

#[test]
fn support_export_validation_blocks_raw_private_material() {
    let packet = seeded_extension_webview_boundary_audit_packet();
    let mut export = project_extension_webview_boundary_support_export(
        &packet,
        "extension-webview-boundary:support-export:default",
        "2026-05-16T00:00:00Z",
    );
    export.raw_private_material_excluded = false;
    let defects = validate_extension_webview_boundary_support_export(&packet, &export)
        .expect_err("raw private material export must fail");
    assert!(defects.iter().any(|defect| defect.defect_kind
        == ExtensionWebviewBoundaryDefectKind::RawPrivateMaterialExported));
}
