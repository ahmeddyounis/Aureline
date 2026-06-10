use super::*;

const PACKET_ID: &str = "local-model-pack:stable:0001";

fn proof_stale_to(narrowed_to: M5AiWorkflowQualificationClass) -> LocalPackDowngradeRule {
    LocalPackDowngradeRule {
        trigger: M5AiWorkflowDowngradeTrigger::ProofStale,
        narrowed_to,
        auto_enforced: true,
        rationale: "Stale proof narrows the public claim".to_owned(),
    }
}

fn direct_verified_pack() -> LocalModelPackRow {
    LocalModelPackRow {
        pack_id: "small-instruct".to_owned(),
        model_id: "local-pack-small".to_owned(),
        publisher_id: "aureline-models".to_owned(),
        pack_version: "1.4.0".to_owned(),
        claimed_qualification: M5AiWorkflowQualificationClass::Stable,
        install_state: LocalPackInstallState::Verified,
        provenance: PackProvenanceClass::SignedPublisherVerified,
        source_channel: PackSourceChannelClass::DirectVendorDownload,
        hardware_fit: HardwareFitClass::FitsComfortably,
        footprint_tier: PackFootprintTierClass::Small,
        accelerator: PackAcceleratorClass::CpuOnly,
        provenance_label: "Signed and verified by the publisher".to_owned(),
        downgrade_rules: vec![
            proof_stale_to(M5AiWorkflowQualificationClass::Beta),
            LocalPackDowngradeRule {
                trigger: M5AiWorkflowDowngradeTrigger::PolicyBlocked,
                narrowed_to: M5AiWorkflowQualificationClass::Unavailable,
                auto_enforced: true,
                rationale: "A policy block makes the pack unavailable".to_owned(),
            },
        ],
        evidence_packet_refs: vec!["evidence:local-pack-small:m5".to_owned()],
    }
}

fn mirror_installed_pack() -> LocalModelPackRow {
    LocalModelPackRow {
        pack_id: "mid-mirror".to_owned(),
        model_id: "local-pack-mid".to_owned(),
        publisher_id: "aureline-models".to_owned(),
        pack_version: "2.1.0".to_owned(),
        claimed_qualification: M5AiWorkflowQualificationClass::Beta,
        install_state: LocalPackInstallState::Installed,
        provenance: PackProvenanceClass::SignedKeyPinned,
        source_channel: PackSourceChannelClass::ConfiguredMirror,
        hardware_fit: HardwareFitClass::FitsConstrained,
        footprint_tier: PackFootprintTierClass::Medium,
        accelerator: PackAcceleratorClass::GpuOptional,
        provenance_label: "Pulled from the configured mirror; key-pinned".to_owned(),
        downgrade_rules: vec![proof_stale_to(M5AiWorkflowQualificationClass::Preview)],
        evidence_packet_refs: vec!["evidence:local-pack-mid:m5".to_owned()],
    }
}

fn quarantined_pack() -> LocalModelPackRow {
    LocalModelPackRow {
        pack_id: "tampered-large".to_owned(),
        model_id: "local-pack-large".to_owned(),
        publisher_id: "unknown-publisher".to_owned(),
        pack_version: "0.9.0".to_owned(),
        claimed_qualification: M5AiWorkflowQualificationClass::Held,
        install_state: LocalPackInstallState::Quarantined,
        provenance: PackProvenanceClass::SignatureMismatch,
        source_channel: PackSourceChannelClass::OfflineBundleImport,
        hardware_fit: HardwareFitClass::UnknownUnverified,
        footprint_tier: PackFootprintTierClass::Large,
        accelerator: PackAcceleratorClass::GpuRequired,
        provenance_label: "Signature did not verify; held in quarantine".to_owned(),
        downgrade_rules: vec![proof_stale_to(M5AiWorkflowQualificationClass::Unavailable)],
        evidence_packet_refs: vec![],
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        LOCAL_MODEL_PACK_SCHEMA_REF.to_owned(),
        LOCAL_MODEL_PACK_DOC_REF.to_owned(),
        PROVIDER_MODEL_REGISTRY_SCHEMA_REF.to_owned(),
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF.to_owned(),
    ]
}

fn proof_freshness() -> LocalModelPackProofFreshness {
    LocalModelPackProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> LocalModelPackInstallPacket {
    LocalModelPackInstallPacket::new(LocalModelPackInstallPacketInput {
        packet_id: PACKET_ID.to_owned(),
        catalogue_label: "Local Model Pack Install And Provenance".to_owned(),
        packs: vec![
            direct_verified_pack(),
            mirror_installed_pack(),
            quarantined_pack(),
        ],
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

#[test]
fn local_model_pack_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn install_state_active_and_held_partition() {
    assert!(LocalPackInstallState::Verified.is_active_install());
    assert!(LocalPackInstallState::Installed.is_active_install());
    assert!(LocalPackInstallState::Quarantined.is_held_aside());
    assert!(LocalPackInstallState::NotInstalled.is_held_aside());
    assert!(!LocalPackInstallState::Acquiring.is_active_install());
    assert!(!LocalPackInstallState::Acquiring.is_held_aside());
}

#[test]
fn counts_match_fixture() {
    let packet = packet();
    assert_eq!(packet.installed_pack_count(), 2);
    assert_eq!(packet.verified_pack_count(), 1);
    assert_eq!(packet.offline_or_mirror_pack_count(), 2);
    assert_eq!(packet.quarantined_pack_count(), 1);
}

#[test]
fn no_packs_fails() {
    let mut packet = packet();
    packet.packs.clear();
    assert!(packet
        .validate()
        .contains(&LocalModelPackViolation::NoPacks));
}

#[test]
fn duplicate_pack_fails() {
    let mut packet = packet();
    let first = packet.packs[0].clone();
    packet.packs.push(first);
    assert!(packet
        .validate()
        .contains(&LocalModelPackViolation::DuplicatePack));
}

#[test]
fn pack_row_incomplete_fails() {
    let mut packet = packet();
    packet.packs[0].provenance_label.clear();
    assert!(packet
        .validate()
        .contains(&LocalModelPackViolation::PackRowIncomplete));
}

#[test]
fn active_install_without_verified_provenance_fails() {
    let mut packet = packet();
    // An installed pack may not run on an unverified provenance chain.
    packet.packs[1].provenance = PackProvenanceClass::ChecksumOnlyUnsigned;
    assert!(packet
        .validate()
        .contains(&LocalModelPackViolation::ActiveInstallWithoutVerifiedProvenance));
}

#[test]
fn failed_provenance_not_quarantined_fails() {
    let mut packet = packet();
    // A signature mismatch must be held aside, never shown as installed.
    packet.packs[0].provenance = PackProvenanceClass::SignatureMismatch;
    packet.packs[0].install_state = LocalPackInstallState::Installed;
    let violations = packet.validate();
    assert!(violations.contains(&LocalModelPackViolation::FailedProvenanceNotQuarantined));
}

#[test]
fn active_install_hardware_unfit_fails() {
    let mut packet = packet();
    // A pack that does not fit the device may not be presented as installed.
    packet.packs[0].hardware_fit = HardwareFitClass::InsufficientMemory;
    assert!(packet
        .validate()
        .contains(&LocalModelPackViolation::ActiveInstallHardwareUnfit));
}

#[test]
fn undisclosed_hardware_fit_fails() {
    let mut packet = packet();
    // A claimed pack may not hide its hardware fit. Move it out of active
    // install so the unfit check does not mask the disclosure check.
    packet.packs[1].install_state = LocalPackInstallState::NotInstalled;
    packet.packs[1].hardware_fit = HardwareFitClass::UnknownUnverified;
    assert!(packet
        .validate()
        .contains(&LocalModelPackViolation::UndisclosedHardwareFit));
}

#[test]
fn offline_or_mirror_without_verified_provenance_fails() {
    let mut packet = packet();
    // A claimed mirror pack, even when not actively installed, must stay signed.
    packet.packs[1].install_state = LocalPackInstallState::NotInstalled;
    packet.packs[1].provenance = PackProvenanceClass::ChecksumOnlyUnsigned;
    assert!(packet
        .validate()
        .contains(&LocalModelPackViolation::OfflineOrMirrorWithoutVerifiedProvenance));
}

#[test]
fn claimed_pack_missing_evidence_fails() {
    let mut packet = packet();
    packet.packs[0].evidence_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&LocalModelPackViolation::ClaimedPackMissingEvidence));
}

#[test]
fn downgrade_rules_missing_fails() {
    let mut packet = packet();
    packet.packs[0].downgrade_rules.clear();
    assert!(packet
        .validate()
        .contains(&LocalModelPackViolation::DowngradeRulesMissing));
}

#[test]
fn downgrade_rule_missing_proof_stale_fails() {
    let mut packet = packet();
    packet.packs[0]
        .downgrade_rules
        .retain(|rule| rule.trigger != M5AiWorkflowDowngradeTrigger::ProofStale);
    assert!(packet
        .validate()
        .contains(&LocalModelPackViolation::DowngradeRuleMissingProofStale));
}

#[test]
fn downgrade_rule_not_narrowing_fails() {
    let mut packet = packet();
    // Narrowing "to" Stable from a Stable claim does not narrow.
    packet.packs[0].downgrade_rules[0].narrowed_to = M5AiWorkflowQualificationClass::Stable;
    assert!(packet
        .validate()
        .contains(&LocalModelPackViolation::DowngradeRuleNotNarrowing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&LocalModelPackViolation::MissingSourceContracts));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&LocalModelPackViolation::ProofFreshnessIncomplete));
}

#[test]
fn raw_boundary_material_in_export_fails() {
    let mut packet = packet();
    // A raw mirror URL must never cross the support boundary.
    packet.packs[1].provenance_label = "https://mirror.example/pack.bin".to_owned();
    assert!(packet
        .validate()
        .contains(&LocalModelPackViolation::RawBoundaryMaterialInExport));
}

#[test]
fn narrowed_qualification_applies_matching_rule() {
    let pack = mirror_installed_pack();
    assert_eq!(
        pack.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProofStale),
        M5AiWorkflowQualificationClass::Preview
    );
    // A trigger with no rule leaves the claim unchanged.
    assert_eq!(
        pack.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProviderUnavailable),
        M5AiWorkflowQualificationClass::Beta
    );
}

#[test]
fn render_inspector_lists_every_posture() {
    let card = mirror_installed_pack().render_inspector();
    assert!(card.contains("mid-mirror"));
    assert!(card.contains("installed"));
    assert!(card.contains("signed_key_pinned"));
    assert!(card.contains("configured_mirror"));
    assert!(card.contains("fits_constrained"));
    assert!(card.contains("gpu_optional"));
}

#[test]
fn markdown_summary_lists_every_pack() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("Pack inspectors"));
    for pack in &packet().packs {
        assert!(summary.contains(&pack.pack_id), "missing {}", pack.pack_id);
    }
}

#[test]
fn offline_held_pack_fixture_validates() {
    let packet: LocalModelPackInstallPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/implement_local_model_pack_install_provenance_hardware_fit_checks_and_offline_or_mirror_support/offline_mirror_with_held_quarantine.json"
    )))
    .expect("offline/mirror held fixture parses");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    // The quarantined pack is held, not claimed, and narrows to unavailable.
    let held: Vec<&LocalModelPackRow> = packet
        .packs
        .iter()
        .filter(|pack| pack.install_state == LocalPackInstallState::Quarantined)
        .collect();
    assert_eq!(held.len(), 1);
    for pack in held {
        assert!(!pack.is_claimed());
        assert_eq!(
            pack.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProofStale),
            M5AiWorkflowQualificationClass::Unavailable
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_local_model_pack_install_export()
        .expect("checked local-model pack export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
    assert!(!packet.packs.is_empty());
    assert!(packet.installed_pack_count() >= 1);
    assert!(packet.offline_or_mirror_pack_count() >= 1);
}
