use super::*;

const PACKET_ID: &str = "signed-template-registry:stable:0001";
const REGISTRY_LABEL: &str =
    "Signed Template Registry, Provenance/Mirror, and Template-Health Rows";

const OFFICIAL_ROW: &str = "registry-row:official.rust.cli:2026.04";
const ORG_MIRROR_ROW: &str = "registry-row:org_mirror.ts_web:2026.04";
const COMMUNITY_ROW: &str = "registry-row:community.python.data:2026.03";
const REPO_LOCAL_ROW: &str = "registry-row:repo_local.web.service:2026.05";

fn proof_freshness() -> SignedTemplateRegistryProofFreshness {
    SignedTemplateRegistryProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> SignedTemplateRegistryPacket {
    canonical_signed_template_registry(
        PACKET_ID.to_owned(),
        REGISTRY_LABEL.to_owned(),
        "2026-06-07T00:00:00Z".to_owned(),
        proof_freshness(),
    )
}

fn row<'a>(
    packet: &'a SignedTemplateRegistryPacket,
    row_id: &str,
) -> &'a SignedTemplateRegistryRow {
    packet
        .rows
        .iter()
        .find(|row| row.row_id == row_id)
        .unwrap_or_else(|| panic!("missing row {row_id}"))
}

#[test]
fn signed_template_registry_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn canonical_registry_covers_provenance_and_mirror_spectrum() {
    let packet = packet();
    let origins: Vec<TemplateRegistryOriginClass> =
        packet.rows.iter().map(|row| row.origin_class).collect();
    for required in [
        TemplateRegistryOriginClass::OfficialOrigin,
        TemplateRegistryOriginClass::OrgMirror,
        TemplateRegistryOriginClass::CommunityOrigin,
        TemplateRegistryOriginClass::RepoLocalGenerator,
    ] {
        assert!(
            origins.contains(&required),
            "missing origin {}",
            required.as_str()
        );
    }
}

#[test]
fn stale_mirror_row_is_not_admitted_in_canonical_registry() {
    let packet = packet();
    let mirror = row(&packet, ORG_MIRROR_ROW);
    assert_eq!(
        mirror.declared_freshness_class,
        TemplateFreshnessClass::MirrorStale
    );
    assert!(!mirror.admitted_for_generation);
}

#[test]
fn rows_empty_fails_validation() {
    let mut packet = packet();
    packet.rows.clear();
    assert!(packet
        .validate()
        .contains(&SignedTemplateRegistryViolation::RowsEmpty));
}

#[test]
fn mirror_missing_freshness_ref_fails() {
    let mut packet = packet();
    let mirror = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == ORG_MIRROR_ROW)
        .unwrap();
    mirror.mirror_freshness_ref = None;
    assert!(packet
        .validate()
        .contains(&SignedTemplateRegistryViolation::MirrorProvenanceIncomplete));
}

#[test]
fn official_origin_with_non_core_trust_fails() {
    let mut packet = packet();
    let official = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == OFFICIAL_ROW)
        .unwrap();
    official.trust_source_class = TemplateTrustSourceClass::CommunityChannelSignature;
    assert!(packet
        .validate()
        .contains(&SignedTemplateRegistryViolation::TrustSourceMismatch));
}

#[test]
fn repo_local_certification_inflation_fails() {
    let mut packet = packet();
    let local = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == REPO_LOCAL_ROW)
        .unwrap();
    local.certification_class = TemplateCertificationClass::CoreCertified;
    assert!(packet
        .validate()
        .contains(&SignedTemplateRegistryViolation::LocalCertificationInflated));
}

#[test]
fn non_local_row_missing_trust_root_fails() {
    let mut packet = packet();
    let official = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == OFFICIAL_ROW)
        .unwrap();
    official.trust_root_ref = None;
    assert!(packet
        .validate()
        .contains(&SignedTemplateRegistryViolation::TrustRootMissing));
}

#[test]
fn blocking_health_admitted_fails() {
    let mut packet = packet();
    let community = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == COMMUNITY_ROW)
        .unwrap();
    community.health_state_class = TemplateHealthStateClass::ValidationFailedBlocksStarter;
    // admitted_for_generation stays true.
    assert!(packet
        .validate()
        .contains(&SignedTemplateRegistryViolation::BlockingHealthAdmitted));
}

#[test]
fn missing_health_check_refs_fails() {
    let mut packet = packet();
    packet.rows[0].health_check_refs.clear();
    assert!(packet
        .validate()
        .contains(&SignedTemplateRegistryViolation::HealthCheckRefsMissing));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.rows[0].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&SignedTemplateRegistryViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = packet();
    packet.rows[0].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&SignedTemplateRegistryViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&SignedTemplateRegistryViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.no_trust_inferred_from_mirror_location = false;
    assert!(packet
        .validate()
        .contains(&SignedTemplateRegistryViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_projection.blocked_rows_labeled_not_hidden = false;
    assert!(packet
        .validate()
        .contains(&SignedTemplateRegistryViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&SignedTemplateRegistryViolation::ProofFreshnessIncomplete));
}

#[test]
fn failed_signature_blocks_a_row() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[SignedTemplateRegistryRowObservation {
        row_id: OFFICIAL_ROW.to_owned(),
        signature_valid: false,
        trust_root_resolved: true,
        mirror_fresh: true,
        health_current: true,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let official = row(&packet, OFFICIAL_ROW);
    assert_eq!(
        official.health_state_class,
        TemplateHealthStateClass::SignatureOrTrustFailedBlocksStarter
    );
    assert!(!official.admitted_for_generation);
    assert_eq!(official.support_class, TemplateSupportClass::SupportUnknown);
    assert!(official
        .downgrade_triggers
        .contains(&TemplateRegistryDowngradeTrigger::SignatureUnverified));
    // A blocked-but-labeled row keeps the export valid.
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn stale_mirror_withholds_admission() {
    let mut packet = packet();
    // Start from a row that is admitted, then make its mirror stale.
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == ORG_MIRROR_ROW)
        .unwrap()
        .admitted_for_generation = true;
    packet.apply_downgrade_automation(&[SignedTemplateRegistryRowObservation {
        row_id: ORG_MIRROR_ROW.to_owned(),
        signature_valid: true,
        trust_root_resolved: true,
        mirror_fresh: false,
        health_current: true,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let mirror = row(&packet, ORG_MIRROR_ROW);
    assert!(!mirror.admitted_for_generation);
    assert_eq!(
        mirror.declared_freshness_class,
        TemplateFreshnessClass::MirrorStale
    );
    assert!(mirror
        .downgrade_triggers
        .contains(&TemplateRegistryDowngradeTrigger::MirrorStale));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn stale_health_narrows_a_healthy_row() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[SignedTemplateRegistryRowObservation {
        row_id: OFFICIAL_ROW.to_owned(),
        signature_valid: true,
        trust_root_resolved: true,
        mirror_fresh: true,
        health_current: false,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let official = row(&packet, OFFICIAL_ROW);
    assert_eq!(
        official.health_state_class,
        TemplateHealthStateClass::StaleButInspectable
    );
    assert!(official
        .downgrade_triggers
        .contains(&TemplateRegistryDowngradeTrigger::HealthCheckStale));
}

#[test]
fn stale_proof_withholds_admission() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[SignedTemplateRegistryRowObservation {
        row_id: COMMUNITY_ROW.to_owned(),
        signature_valid: true,
        trust_root_resolved: true,
        mirror_fresh: true,
        health_current: true,
        proof_fresh: false,
        upstream_narrowed: false,
    }]);
    let community = row(&packet, COMMUNITY_ROW);
    assert!(!community.admitted_for_generation);
    assert!(community
        .downgrade_triggers
        .contains(&TemplateRegistryDowngradeTrigger::ProofStale));
}

#[test]
fn markdown_summary_lists_every_row() {
    let summary = packet().render_markdown_summary();
    for row in &packet().rows {
        assert!(
            summary.contains(&row.template_id),
            "summary missing template {}",
            row.template_id
        );
    }
    assert!(summary.contains("mirror_stale"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_signed_template_registry_export()
        .expect("checked signed template-registry export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_canonical_builder() {
    let checked = current_signed_template_registry_export()
        .expect("checked signed template-registry export validates");
    assert_eq!(checked, packet());
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/templates/m5/implement_the_signed_template_registry_provenance_or_mirror_support_and_template_health_rows/health_stale_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/templates/m5/implement_the_signed_template_registry_provenance_or_mirror_support_and_template_health_rows/signature_failed_blocked.json"
        )),
    ] {
        let packet: SignedTemplateRegistryPacket =
            serde_json::from_str(raw).expect("fixture parses as signed registry packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
