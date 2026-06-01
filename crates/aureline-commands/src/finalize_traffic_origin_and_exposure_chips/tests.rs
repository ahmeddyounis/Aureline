use super::*;

const PACKET_ID: &str = "traffic-origin-exposure-chips-finalization:stable:0001";

fn tunnel_record() -> TunnelExplainabilityRecord {
    TunnelExplainabilityRecord {
        tunnel_ref: "tunnel-ref:dev-tunnel:stable:0001".to_owned(),
        tunnel_kind: TunnelKindClass::DevTunnel,
        target_port_ref: "port-ref:dev-tunnel-target:stable:0001".to_owned(),
        traffic_origin: TrafficOriginClass::TunnelIngress,
        exposure: ExposureChipClass::TunnelExposed,
        approval_scope_ref: "approval-scope:dev-tunnel:stable:0001".to_owned(),
        approval_owner_label: "Workspace operator".to_owned(),
        drift_forces_reapproval: true,
        disclosed_in_chip: true,
        disclosed_in_preview: true,
        disclosed_in_support_export: true,
    }
}

fn port_record() -> PortExplainabilityRecord {
    PortExplainabilityRecord {
        port_ref: "port-ref:forwarded:stable:0001".to_owned(),
        protocol: PortProtocolClass::Http,
        exposure: ExposureChipClass::PortForwarded,
        traffic_origin: TrafficOriginClass::PortForwardIngress,
        disclosed_in_chip: true,
        disclosed_in_support_export: true,
    }
}

fn publish_target_record() -> PublishTargetExplainabilityRecord {
    PublishTargetExplainabilityRecord {
        publish_target_ref: "publish-target-ref:static-host:stable:0001".to_owned(),
        target_class: PublishTargetClass::StaticHost,
        exposure: ExposureChipClass::PubliclyPublished,
        traffic_origin: TrafficOriginClass::PublishTargetRelay,
        approval_scope_ref: "approval-scope:static-host:stable:0001".to_owned(),
        spend_posture_token: "entitlement_band".to_owned(),
        disclosed_in_chip: true,
        disclosed_in_support_export: true,
    }
}

fn chip_surface_rows() -> Vec<TrafficOriginChipRow> {
    CommandSurfaceClass::required_coverage()
        .into_iter()
        .map(|surface_class| TrafficOriginChipRow {
            surface_class,
            chip_visible: true,
            discloses_origin_class: true,
            discloses_exposure_class: true,
            discloses_network_explainability: true,
            policy_checked: true,
            no_authority_widening: true,
            qualification: SurfaceQualificationClass::Stable,
            claimed_stable: true,
        })
        .collect()
}

fn evidence_export() -> CommandContractEvidenceExport {
    CommandContractEvidenceExport {
        evidence_id: "command-evidence:traffic-origin-exposure-chips:stable:0001".to_owned(),
        json_export_ref: FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_ARTIFACT_REF.to_owned(),
        markdown_summary_ref: FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_SUMMARY_REF.to_owned(),
        admin_inspector_ref: "admin-inspector:traffic-origin-exposure-chips:stable:0001"
            .to_owned(),
        support_export_ref: "support-export:traffic-origin-exposure-chips:stable:0001".to_owned(),
        rollback_lineage_refs: vec![
            "rollback-checkpoint:traffic-origin-exposure-chips:0001".to_owned(),
        ],
        export_lineage_refs: vec![
            "export-lineage:traffic-origin-exposure-chips:beta:0001".to_owned(),
        ],
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_DOC_REF.to_owned(),
        FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_SCHEMA_REF.to_owned(),
        FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_DESCRIPTOR_CONTRACT_REF.to_owned(),
        FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_PARITY_CONTRACT_REF.to_owned(),
    ]
}

fn input() -> FinalizeTrafficOriginExposureChipsPacketInput {
    FinalizeTrafficOriginExposureChipsPacketInput {
        packet_id: PACKET_ID.to_owned(),
        command_family_id: "cmd-family:network.tunnel_open".to_owned(),
        display_label: "Network Tunnel and Port Forward (traffic-origin exposure chips)"
            .to_owned(),
        claimed_stable: true,
        policy_epoch_ref: "policy-epoch:traffic-origin:0001".to_owned(),
        contract_refs: StableContractRefs::canonical(),
        traffic_origin_classes: vec![
            TrafficOriginClass::TunnelIngress,
            TrafficOriginClass::PortForwardIngress,
            TrafficOriginClass::PublishTargetRelay,
        ],
        exposure_classes: vec![
            ExposureChipClass::TunnelExposed,
            ExposureChipClass::PortForwarded,
            ExposureChipClass::PubliclyPublished,
        ],
        tunnel_records: vec![tunnel_record()],
        port_records: vec![port_record()],
        publish_target_records: vec![publish_target_record()],
        chip_surface_rows: chip_surface_rows(),
        evidence_export: evidence_export(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-01T00:00:00Z".to_owned(),
    }
}

fn packet() -> FinalizeTrafficOriginExposureChipsPacket {
    FinalizeTrafficOriginExposureChipsPacket::new(input())
}

#[test]
fn packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn wrong_record_kind_is_rejected() {
    let mut packet = packet();
    packet.record_kind = "not_the_packet".to_owned();
    assert!(packet
        .validate()
        .contains(&FinalizeTrafficOriginExposureChipsViolation::WrongRecordKind));
}

#[test]
fn missing_identity_is_rejected() {
    let mut packet = packet();
    packet.command_family_id = String::new();
    assert!(packet
        .validate()
        .contains(&FinalizeTrafficOriginExposureChipsViolation::MissingIdentity));
}

#[test]
fn non_canonical_contract_refs_are_rejected() {
    let mut packet = packet();
    packet.contract_refs.result_packet_schema_ref =
        "schemas/commands/some_other_result.schema.json".to_owned();
    assert!(packet
        .validate()
        .contains(&FinalizeTrafficOriginExposureChipsViolation::ContractRefsNotCanonical));
}

#[test]
fn missing_source_contracts_are_rejected() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != FINALIZE_TRAFFIC_ORIGIN_EXPOSURE_CHIPS_PARITY_CONTRACT_REF);
    assert!(packet
        .validate()
        .contains(&FinalizeTrafficOriginExposureChipsViolation::MissingSourceContracts));
}

#[test]
fn empty_traffic_origin_classes_are_rejected() {
    let mut packet = packet();
    packet.traffic_origin_classes.clear();
    assert!(packet
        .validate()
        .contains(&FinalizeTrafficOriginExposureChipsViolation::TrafficOriginChipsMissing));
}

#[test]
fn empty_exposure_classes_are_rejected() {
    let mut packet = packet();
    packet.exposure_classes.clear();
    assert!(packet
        .validate()
        .contains(&FinalizeTrafficOriginExposureChipsViolation::ExposureChipsMissing));
}

#[test]
fn tunnel_record_missing_drift_guard_is_rejected() {
    let mut packet = packet();
    packet.tunnel_records[0].drift_forces_reapproval = false;
    assert!(packet
        .validate()
        .contains(&FinalizeTrafficOriginExposureChipsViolation::TunnelRecordGuardsBroken));
}

#[test]
fn tunnel_record_not_disclosed_in_preview_is_rejected() {
    let mut packet = packet();
    packet.tunnel_records[0].disclosed_in_preview = false;
    assert!(packet
        .validate()
        .contains(&FinalizeTrafficOriginExposureChipsViolation::TunnelRecordGuardsBroken));
}

#[test]
fn tunnel_record_not_disclosed_in_chip_is_rejected() {
    let mut packet = packet();
    packet.tunnel_records[0].disclosed_in_chip = false;
    assert!(packet
        .validate()
        .contains(&FinalizeTrafficOriginExposureChipsViolation::TunnelRecordGuardsBroken));
}

#[test]
fn tunnel_record_missing_approval_scope_ref_is_rejected() {
    let mut packet = packet();
    packet.tunnel_records[0].approval_scope_ref = String::new();
    assert!(packet
        .validate()
        .contains(&FinalizeTrafficOriginExposureChipsViolation::TunnelRecordGuardsBroken));
}

#[test]
fn port_record_not_disclosed_in_chip_is_rejected() {
    let mut packet = packet();
    packet.port_records[0].disclosed_in_chip = false;
    assert!(packet
        .validate()
        .contains(&FinalizeTrafficOriginExposureChipsViolation::PortRecordGuardsBroken));
}

#[test]
fn port_record_not_disclosed_in_support_export_is_rejected() {
    let mut packet = packet();
    packet.port_records[0].disclosed_in_support_export = false;
    assert!(packet
        .validate()
        .contains(&FinalizeTrafficOriginExposureChipsViolation::PortRecordGuardsBroken));
}

#[test]
fn publish_target_record_missing_approval_scope_ref_is_rejected() {
    let mut packet = packet();
    packet.publish_target_records[0].approval_scope_ref = String::new();
    assert!(packet
        .validate()
        .contains(
            &FinalizeTrafficOriginExposureChipsViolation::PublishTargetRecordGuardsBroken
        ));
}

#[test]
fn publish_target_record_not_disclosed_in_chip_is_rejected() {
    let mut packet = packet();
    packet.publish_target_records[0].disclosed_in_chip = false;
    assert!(packet
        .validate()
        .contains(
            &FinalizeTrafficOriginExposureChipsViolation::PublishTargetRecordGuardsBroken
        ));
}

#[test]
fn chip_surface_coverage_is_required() {
    let mut packet = packet();
    packet
        .chip_surface_rows
        .retain(|row| row.surface_class != CommandSurfaceClass::AiTool);
    assert!(packet
        .validate()
        .contains(&FinalizeTrafficOriginExposureChipsViolation::ChipSurfaceCoverageMissing));
}

#[test]
fn stable_surface_dropping_origin_class_disclosure_is_rejected() {
    let mut packet = packet();
    packet.chip_surface_rows[0].discloses_origin_class = false;
    assert!(packet
        .validate()
        .contains(&FinalizeTrafficOriginExposureChipsViolation::ChipSurfaceParityBroken));
}

#[test]
fn stable_surface_dropping_exposure_class_disclosure_is_rejected() {
    let mut packet = packet();
    packet.chip_surface_rows[0].discloses_exposure_class = false;
    assert!(packet
        .validate()
        .contains(&FinalizeTrafficOriginExposureChipsViolation::ChipSurfaceParityBroken));
}

#[test]
fn stable_surface_dropping_network_explainability_is_rejected() {
    let mut packet = packet();
    packet.chip_surface_rows[0].discloses_network_explainability = false;
    assert!(packet
        .validate()
        .contains(&FinalizeTrafficOriginExposureChipsViolation::ChipSurfaceParityBroken));
}

#[test]
fn narrowed_surface_may_not_claim_stable() {
    let mut packet = packet();
    packet.chip_surface_rows[0].qualification = SurfaceQualificationClass::Beta;
    assert!(packet
        .validate()
        .contains(&FinalizeTrafficOriginExposureChipsViolation::UnqualifiedSurfaceClaimsStable));
}

#[test]
fn narrowed_surface_below_stable_validates() {
    let mut packet = packet();
    if let Some(row) = packet
        .chip_surface_rows
        .iter_mut()
        .find(|row| row.surface_class == CommandSurfaceClass::Voice)
    {
        row.qualification = SurfaceQualificationClass::NotApplicable;
        row.claimed_stable = false;
        row.chip_visible = false;
        row.discloses_origin_class = false;
        row.discloses_exposure_class = false;
        row.discloses_network_explainability = false;
    }
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_evidence_id_is_rejected() {
    let mut packet = packet();
    packet.evidence_export.evidence_id = String::new();
    assert!(packet
        .validate()
        .contains(&FinalizeTrafficOriginExposureChipsViolation::EvidenceExportRefsMissing));
}

#[test]
fn raw_material_is_rejected() {
    let mut packet = packet();
    packet.tunnel_records[0].tunnel_ref =
        "reachable via ssh-tunnel://internal.host/dev".to_owned();
    assert!(packet
        .validate()
        .contains(&FinalizeTrafficOriginExposureChipsViolation::RawMaterialInExport));
}

#[test]
fn exposure_chip_external_reachability() {
    assert!(ExposureChipClass::TunnelExposed.is_externally_reachable());
    assert!(ExposureChipClass::PubliclyPublished.is_externally_reachable());
    assert!(ExposureChipClass::ProviderManaged.is_externally_reachable());
    assert!(ExposureChipClass::EnterpriseGateway.is_externally_reachable());
    assert!(!ExposureChipClass::Unexposed.is_externally_reachable());
    assert!(!ExposureChipClass::LocalhostOnly.is_externally_reachable());
    assert!(!ExposureChipClass::PortForwarded.is_externally_reachable());
}

#[test]
fn checked_artifact_validates() {
    let packet = current_traffic_origin_exposure_chips_export()
        .expect("checked traffic-origin exposure-chips export validates");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
#[ignore = "run manually to regenerate the checked artifact"]
fn emit_artifact() {
    let packet = packet();
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    let dir = format!(
        "{root}/artifacts/commands/m4/finalize_traffic_origin_and_exposure_chips"
    );
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        format!("{dir}/support_export.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .unwrap();
    std::fs::write(
        format!("{dir}/summary.md"),
        packet.render_markdown_summary(),
    )
    .unwrap();
    let fixture_dir = format!(
        "{root}/fixtures/commands/m4/finalize_traffic_origin_and_exposure_chips"
    );
    std::fs::create_dir_all(&fixture_dir).unwrap();
    std::fs::write(
        format!(
            "{fixture_dir}/finalize_traffic_origin_and_exposure_chips_packet.json"
        ),
        format!("{}\n", packet.export_safe_json()),
    )
    .unwrap();
}
