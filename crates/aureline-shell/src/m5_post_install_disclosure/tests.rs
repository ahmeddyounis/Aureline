use super::*;

fn panel_of(
    set: &M5PostInstallDisclosurePanelSet,
    family: DisclosureArtifactFamily,
) -> &PostInstallDisclosureRecord {
    set.panels
        .iter()
        .find(|panel| panel.artifact_family() == family)
        .unwrap_or_else(|| panic!("panel for {} present", family.as_str()))
}

#[test]
fn seeded_panel_set_validates() {
    let set = seeded_m5_post_install_disclosure_panel_set();
    assert!(set.validate().is_empty(), "{:?}", set.validate());
    assert_eq!(set.packet_id, M5_POST_INSTALL_DISCLOSURE_PANEL_SET_ID);
}

#[test]
fn panel_set_covers_every_family() {
    let set = seeded_m5_post_install_disclosure_panel_set();
    for family in DisclosureArtifactFamily::ALL {
        assert!(
            set.panels.iter().any(|p| p.artifact_family() == family),
            "missing family {}",
            family.as_str()
        );
    }
    assert!(set.coverage.is_complete());
    assert_eq!(set.panels.len(), DisclosureArtifactFamily::ALL.len());
}

#[test]
fn every_panel_validates_on_its_own() {
    let set = seeded_m5_post_install_disclosure_panel_set();
    for panel in &set.panels {
        assert!(
            panel.validate().is_empty(),
            "panel {} failed: {:?}",
            panel.disclosure_id,
            panel.validate()
        );
    }
}

#[test]
fn missing_data_row_omission_fails() {
    // The side-loaded extension reports several missing axes; dropping its rows
    // must be caught as a hidden gap.
    let mut set = seeded_m5_post_install_disclosure_panel_set();
    let panel = set
        .panels
        .iter_mut()
        .find(|p| p.artifact_family() == DisclosureArtifactFamily::ExtensionFrameworkPack)
        .expect("extension panel present");
    panel.visible_cues.missing_or_partial_data.clear();
    assert!(panel
        .validate()
        .contains(&PostInstallDisclosureViolation::MissingDataRowOmitted));
}

#[test]
fn source_label_must_match_class() {
    let mut panel = panel_of(
        &seeded_m5_post_install_disclosure_panel_set(),
        DisclosureArtifactFamily::DesktopBuildInstaller,
    )
    .clone();
    panel.source.source_label = SourceLabel::Mirrored;
    assert!(panel
        .validate()
        .contains(&PostInstallDisclosureViolation::SourceLabelMismatch));
}

#[test]
fn mirrored_without_snapshot_fails() {
    let mut panel = panel_of(
        &seeded_m5_post_install_disclosure_panel_set(),
        DisclosureArtifactFamily::MirroredOfflineArtifact,
    )
    .clone();
    panel.verification.revocation_snapshot_ref = None;
    assert!(panel
        .validate()
        .contains(&PostInstallDisclosureViolation::SourceEvidenceMissing));
}

#[test]
fn attached_sbom_requires_format_label() {
    let mut panel = panel_of(
        &seeded_m5_post_install_disclosure_panel_set(),
        DisclosureArtifactFamily::DesktopBuildInstaller,
    )
    .clone();
    panel.notice_inventory.sbom_formats.clear();
    assert!(panel
        .validate()
        .contains(&PostInstallDisclosureViolation::SbomFormatLabelMissing));
}

#[test]
fn sbom_format_without_attachment_fails() {
    let mut panel = panel_of(
        &seeded_m5_post_install_disclosure_panel_set(),
        DisclosureArtifactFamily::ExtensionFrameworkPack,
    )
    .clone();
    // Side-loaded extension has sbom_missing; declaring a format is unbacked.
    panel.notice_inventory.sbom_formats = vec![SbomFormat::SpdxJson];
    assert!(panel
        .validate()
        .contains(&PostInstallDisclosureViolation::SbomFormatLabelUnbacked));
}

#[test]
fn required_access_point_missing_fails() {
    let mut panel = panel_of(
        &seeded_m5_post_install_disclosure_panel_set(),
        DisclosureArtifactFamily::DesktopBuildInstaller,
    )
    .clone();
    panel
        .access_points
        .retain(|point| point.access_point_class != AccessPointClass::About);
    assert!(panel
        .validate()
        .contains(&PostInstallDisclosureViolation::RequiredAccessPointMissing));
}

#[test]
fn generated_artifact_requires_lineage() {
    let mut panel = panel_of(
        &seeded_m5_post_install_disclosure_panel_set(),
        DisclosureArtifactFamily::GeneratedExportArtifact,
    )
    .clone();
    panel.artifact.generated_artifact_lineage_ref = None;
    assert!(panel
        .validate()
        .contains(&PostInstallDisclosureViolation::GeneratedLineageMissing));
}

#[test]
fn invalid_disclosure_id_fails() {
    let mut panel = panel_of(
        &seeded_m5_post_install_disclosure_panel_set(),
        DisclosureArtifactFamily::DesktopBuildInstaller,
    )
    .clone();
    panel.disclosure_id = "Desktop Official Build".to_owned();
    assert!(panel
        .validate()
        .contains(&PostInstallDisclosureViolation::InvalidDisclosureId));
}

#[test]
fn disclosure_id_validator_accepts_and_rejects() {
    assert!(is_valid_disclosure_id(
        "post_install_disclosure:desktop.official.signed_stable"
    ));
    assert!(is_valid_disclosure_id("post_install_disclosure:a"));
    assert!(!is_valid_disclosure_id("post_install_disclosure:"));
    assert!(!is_valid_disclosure_id("post_install_disclosure:Desktop"));
    assert!(!is_valid_disclosure_id("post_install_disclosure:a..b"));
    assert!(!is_valid_disclosure_id("post_install_disclosure:.a"));
    assert!(!is_valid_disclosure_id("desktop.official"));
}

#[test]
fn coverage_drift_fails() {
    let mut set = seeded_m5_post_install_disclosure_panel_set();
    set.coverage.covers_generated_export_artifact = false;
    assert!(set
        .validate()
        .contains(&PostInstallDisclosureViolation::CoverageDrift));
}

#[test]
fn dropping_a_family_fails_coverage() {
    let mut panels = seeded_post_install_panels();
    panels.retain(|p| p.artifact_family() != DisclosureArtifactFamily::GeneratedExportArtifact);
    let set = M5PostInstallDisclosurePanelSet::new(M5PostInstallDisclosurePanelSetInput {
        packet_id: "m5-post-install-disclosure-panels:partial:0001".to_owned(),
        panel_set_label: "partial".to_owned(),
        panels,
        honesty_invariants: seeded_m5_post_install_disclosure_panel_set().honesty_invariants,
        consumer_projection: seeded_m5_post_install_disclosure_panel_set().consumer_projection,
        source_contract_refs: seeded_m5_post_install_disclosure_panel_set().source_contract_refs,
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-30T00:00:00Z".to_owned(),
    });
    assert!(set
        .validate()
        .contains(&PostInstallDisclosureViolation::FamilyCoverageIncomplete));
}

#[test]
fn honesty_invariant_unmet_fails() {
    let mut set = seeded_m5_post_install_disclosure_panel_set();
    set.honesty_invariants.missing_data_visible_not_omitted = false;
    assert!(set
        .validate()
        .contains(&PostInstallDisclosureViolation::HonestyInvariantUnmet));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut set = seeded_m5_post_install_disclosure_panel_set();
    set.consumer_projection
        .marketplace_or_package_detail_shows_provenance_for_packs = false;
    assert!(set
        .validate()
        .contains(&PostInstallDisclosureViolation::ConsumerProjectionIncomplete));
}

#[test]
fn missing_source_contracts_fails() {
    let mut set = seeded_m5_post_install_disclosure_panel_set();
    set.source_contract_refs.clear();
    assert!(set
        .validate()
        .contains(&PostInstallDisclosureViolation::MissingSourceContracts));
}

#[test]
fn narrowed_signature_revoked_validates_and_narrows() {
    let panel = seeded_post_install_product_build_signature_revoked();
    assert!(panel.validate().is_empty(), "{:?}", panel.validate());
    assert_eq!(
        panel.verification.signature_state,
        SignatureState::SignatureRevoked
    );
    assert_eq!(
        panel.verification.revocation_state,
        RevocationState::RevokedOrYanked
    );
    assert!(panel
        .visible_cues
        .missing_or_partial_data
        .iter()
        .any(|row| row.data_class == DataClass::RevocationSnapshot));
}

#[test]
fn narrowed_generated_export_sbom_not_provided_validates() {
    let panel = seeded_post_install_generated_export_sbom_not_provided();
    assert!(panel.validate().is_empty(), "{:?}", panel.validate());
    assert_eq!(panel.notice_inventory.sbom_state, SbomState::SbomMissing);
    assert!(panel
        .visible_cues
        .missing_or_partial_data
        .iter()
        .any(|row| row.data_class == DataClass::Sbom
            && row.missing_state == MissingState::NotProvided));
}

#[test]
fn panel_csv_has_a_row_per_panel() {
    let set = seeded_m5_post_install_disclosure_panel_set();
    let csv = set.render_panel_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + set.panels.len());
    assert!(lines[0].starts_with("family,subject_kind,source_class,"));
    for family in DisclosureArtifactFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing {}",
            family.as_str()
        );
    }
}

#[test]
fn markdown_summary_lists_every_family() {
    let summary = seeded_m5_post_install_disclosure_panel_set().render_markdown_summary();
    for family in DisclosureArtifactFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing {}",
            family.as_str()
        );
    }
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_post_install_disclosure_panel_set().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
}

#[test]
fn checked_panel_set_validates() {
    let set =
        current_stable_m5_post_install_disclosure_panel_set().expect("checked panel set validates");
    assert_eq!(set.packet_id, M5_POST_INSTALL_DISCLOSURE_PANEL_SET_ID);
}

#[test]
fn checked_panel_set_matches_seed() {
    let from_disk =
        current_stable_m5_post_install_disclosure_panel_set().expect("checked panel set validates");
    assert_eq!(
        from_disk,
        seeded_m5_post_install_disclosure_panel_set(),
        "checked panel-set export drifted from the seed builder"
    );
}

#[test]
fn checked_narrowed_fixtures_match_seed_builders() {
    let revoked: PostInstallDisclosureRecord = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/help/post-install-disclosure/product_build_signature_revoked.json"
    )))
    .expect("revoked fixture parses");
    assert_eq!(
        revoked,
        seeded_post_install_product_build_signature_revoked()
    );
    assert!(revoked.validate().is_empty(), "{:?}", revoked.validate());

    let sbom_missing: PostInstallDisclosureRecord = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/help/post-install-disclosure/generated_export_sbom_not_provided.json"
    )))
    .expect("sbom-missing fixture parses");
    assert_eq!(
        sbom_missing,
        seeded_post_install_generated_export_sbom_not_provided()
    );
    assert!(
        sbom_missing.validate().is_empty(),
        "{:?}",
        sbom_missing.validate()
    );
}
