use super::*;

const PACKET_ID: &str = "package-certification:stable:0001";

fn trust_review() -> PackageComponentCertificationTrustReview {
    canonical_trust_review()
}

fn consumer_projection() -> PackageComponentCertificationConsumerProjection {
    canonical_consumer_projection()
}

fn proof_freshness() -> PackageComponentCertificationProofFreshness {
    PackageComponentCertificationProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<PackageComponentCertificationDowngradeTrigger> {
    vec![
        PackageComponentCertificationDowngradeTrigger::ProofStale,
        PackageComponentCertificationDowngradeTrigger::ManifestScopePartial,
        PackageComponentCertificationDowngradeTrigger::RegistryFreshnessStale,
        PackageComponentCertificationDowngradeTrigger::AuthStateUnsatisfied,
        PackageComponentCertificationDowngradeTrigger::LockfileImpactUnavailable,
    ]
}

fn source_contract_refs() -> Vec<String> {
    canonical_source_contract_refs()
}

fn row_refs(components: &[M5PackageComponent]) -> Vec<String> {
    let mut refs = vec![M5_PACKAGE_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF.to_owned()];
    for component in components {
        let schema = certification_component_canonical_schema_ref(*component).to_owned();
        if !refs.contains(&schema) {
            refs.push(schema);
        }
    }
    refs
}

fn axis_note(axis: PackageComponentCertificationAxis, narrowed: bool) -> String {
    let base = match axis {
        PackageComponentCertificationAxis::Visual => {
            "Visual rendering carries the controlled component truth"
        }
        PackageComponentCertificationAxis::Keyboard => {
            "Keyboard reach and operation carry the controlled component truth"
        }
        PackageComponentCertificationAxis::ScreenReader => {
            "Screen-reader labelling carries the controlled component truth"
        }
        PackageComponentCertificationAxis::CliExport => {
            "CLI and export forms carry the controlled component truth"
        }
        PackageComponentCertificationAxis::DegradedState => {
            "Degraded manifest-scope, registry, auth, lockfile, or rollback state is disclosed"
        }
        PackageComponentCertificationAxis::ScopeAndSourceProvenance => {
            "The manifest-scope, registry-source, script-risk, and lockfile-churn distinction stays explicit; certified never implies safe one-click"
        }
    };
    if narrowed {
        format!("{base} — narrowed here with an honest fallback disclosed")
    } else {
        base.to_owned()
    }
}

fn axis_outcomes(
    narrowed_axis: Option<PackageComponentCertificationAxis>,
) -> Vec<PackageComponentCertAxisOutcome> {
    PackageComponentCertificationAxis::ALL
        .iter()
        .map(|axis| {
            let narrowed = narrowed_axis == Some(*axis);
            PackageComponentCertAxisOutcome {
                axis: *axis,
                state: if narrowed {
                    PackageComponentAxisCertificationState::NarrowedCertified
                } else {
                    PackageComponentAxisCertificationState::Certified
                },
                note: axis_note(*axis, narrowed),
            }
        })
        .collect()
}

/// Builds one certified surface row, deriving status from its claims and narrowing so
/// the fixture stays self-consistent.
#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    surface: M5PackageComponentCertifiedSurface,
    components: Vec<M5PackageComponent>,
    claimed: PackageComponentClaimTier,
    certified: PackageComponentClaimTier,
    narrowed_axis: Option<PackageComponentCertificationAxis>,
    trigger: Option<PackageComponentCertificationDowngradeTrigger>,
) -> PackageComponentCertifiedSurfaceRow {
    let narrowed_axes = narrowed_axis.map(|axis| vec![axis]).unwrap_or_default();
    let component_truth_preserved = true;
    let status = derive_package_component_surface_claim_status(
        claimed,
        certified,
        component_truth_preserved,
        !narrowed_axes.is_empty(),
    );
    let refs = row_refs(&components);
    PackageComponentCertifiedSurfaceRow {
        row_id: row_id.to_owned(),
        surface,
        components_present: components,
        claimed_claim: claimed,
        certified_claim: certified,
        status,
        axis_outcomes: axis_outcomes(narrowed_axis),
        narrowed_axes,
        downgrade_trigger: trigger,
        component_truth_preserved,
        keyboard_label: format!(
            "{}: focusable, Enter opens review, Space toggles manifest scope",
            surface.as_str()
        ),
        screen_reader_label: format!(
            "{} package surface, certified {}",
            surface.as_str(),
            certified.as_str()
        ),
        cli_enum_token: format!("{}:{}", surface.as_str(), status.as_str()),
        export_enum_token: status.as_str().to_owned(),
        explanation_field: format!(
            "{} presents controlled component truth; claim certified as {}",
            surface.as_str(),
            certified.as_str()
        ),
        source_contract_refs: refs,
    }
}

/// The canonical eight-surface set: four green, four narrowed, all eight components.
fn surface_rows() -> Vec<PackageComponentCertifiedSurfaceRow> {
    use M5PackageComponent as C;
    use M5PackageComponentCertifiedSurface as S;
    use PackageComponentCertificationAxis as Axis;
    use PackageComponentCertificationDowngradeTrigger as Trigger;
    use PackageComponentClaimTier as Tier;
    vec![
        row(
            "cert:package-explorer",
            S::PackageExplorerSurface,
            vec![C::PackageExplorerRow, C::ManifestScopeSwitcher],
            Tier::FullReviewableManagement,
            Tier::FullReviewableManagement,
            None,
            None,
        ),
        row(
            "cert:search-detail",
            S::DependencySearchDetailSurface,
            vec![C::RegistryOrMirrorRow],
            Tier::FullReviewableManagement,
            Tier::FullReviewableManagement,
            None,
            None,
        ),
        row(
            "cert:help",
            S::HelpPackageSurface,
            vec![C::GroupedUpdatePlanner],
            Tier::FullReviewableManagement,
            Tier::FullReviewableManagement,
            None,
            None,
        ),
        row(
            "cert:cli",
            S::CliHeadless,
            vec![C::ScriptRiskNotice, C::RollbackCheckpointStrip],
            Tier::FullReviewableManagement,
            Tier::FullReviewableManagement,
            None,
            None,
        ),
        row(
            "cert:install-review",
            S::InstallReviewSheetSurface,
            vec![C::InstallReviewSheet, C::LockfileImpactCard],
            Tier::FullReviewableManagement,
            Tier::LockfileImpactUnknown,
            Some(Axis::DegradedState),
            Some(Trigger::LockfileImpactUnavailable),
        ),
        row(
            "cert:support-export",
            S::SupportExport,
            vec![C::RegistryOrMirrorRow, C::RollbackCheckpointStrip],
            Tier::FullReviewableManagement,
            Tier::AuthRequiredReadOnly,
            Some(Axis::DegradedState),
            Some(Trigger::AuthStateUnsatisfied),
        ),
        row(
            "cert:exported-packet",
            S::ExportedPackageReviewPacket,
            vec![C::PackageExplorerRow, C::LockfileImpactCard],
            Tier::FullReviewableManagement,
            Tier::MirrorOrOfflineSourced,
            Some(Axis::ScopeAndSourceProvenance),
            Some(Trigger::RegistryFreshnessStale),
        ),
        row(
            "cert:diagnostics",
            S::Diagnostics,
            vec![C::ManifestScopeSwitcher, C::ScriptRiskNotice],
            Tier::FullReviewableManagement,
            Tier::ManifestRangeScoped,
            Some(Axis::DegradedState),
            Some(Trigger::ManifestScopePartial),
        ),
    ]
}

fn packet_with(
    rows: Vec<PackageComponentCertifiedSurfaceRow>,
) -> PackageComponentCertificationPacket {
    let summary = PackageComponentCertificationSummary::from_rows(&rows);
    PackageComponentCertificationPacket::new(PackageComponentCertificationPacketInput {
        packet_id: PACKET_ID.to_owned(),
        certification_label: "Package-management component surface certification".to_owned(),
        surface_rows: rows,
        summary,
        downgrade_triggers: downgrade_triggers(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "manifest_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

fn packet() -> PackageComponentCertificationPacket {
    packet_with(surface_rows())
}

#[test]
fn certification_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn canonical_seed_is_four_green_four_narrowed() {
    let packet = packet();
    assert_eq!(packet.summary.certified_count, 4);
    assert_eq!(packet.summary.narrowed_count, 4);
    assert_eq!(packet.summary.blocked_count, 0);
    assert!(packet.summary.all_rows_preserve_component_truth);
    assert!(packet.summary.all_surfaces_covered);
    assert!(packet.summary.all_components_covered);
}

#[test]
fn every_row_status_is_consistent() {
    for row in surface_rows() {
        assert!(
            row.status_is_consistent(),
            "row status not consistent: {}",
            row.row_id
        );
        assert!(row.covers_all_axes(), "row missing axes: {}", row.row_id);
        assert!(
            row.narrowed_axes_consistent(),
            "row narrowed axes inconsistent: {}",
            row.row_id
        );
        assert!(
            row.parity_fields_present(),
            "row missing parity fields: {}",
            row.row_id
        );
    }
}

#[test]
fn derive_status_maps_claims_to_status() {
    use PackageComponentClaimTier as Tier;
    assert_eq!(
        derive_package_component_surface_claim_status(
            Tier::FullReviewableManagement,
            Tier::FullReviewableManagement,
            true,
            false
        ),
        PackageComponentSurfaceClaimStatus::CertifiedParity
    );
    assert_eq!(
        derive_package_component_surface_claim_status(
            Tier::FullReviewableManagement,
            Tier::MirrorOrOfflineSourced,
            true,
            false
        ),
        PackageComponentSurfaceClaimStatus::NarrowedParity
    );
    assert_eq!(
        derive_package_component_surface_claim_status(
            Tier::FullReviewableManagement,
            Tier::FullReviewableManagement,
            true,
            true
        ),
        PackageComponentSurfaceClaimStatus::NarrowedParity
    );
    assert_eq!(
        derive_package_component_surface_claim_status(
            Tier::FullReviewableManagement,
            Tier::FullReviewableManagement,
            false,
            false
        ),
        PackageComponentSurfaceClaimStatus::ParityBlocked
    );
}

// --- AC2: certification never overstates management scope ----------------------

#[test]
fn certified_claim_exceeds_claimed_fails() {
    let mut packet = packet();
    packet.surface_rows[0].claimed_claim = PackageComponentClaimTier::MirrorOrOfflineSourced;
    packet.surface_rows[0].certified_claim = PackageComponentClaimTier::FullReviewableManagement;
    assert!(packet
        .validate()
        .contains(&PackageComponentCertificationViolation::CertifiedClaimExceedsClaimed));
}

#[test]
fn certified_never_implies_safe_one_click() {
    // A fully-certified (green) surface and a narrowed surface both keep the
    // scope-and-source-provenance axis explicit, proving the axis is orthogonal to the
    // overall certified status: certification never implies safe one-click.
    let rows = surface_rows();
    let green = rows
        .iter()
        .find(|r| r.surface == M5PackageComponentCertifiedSurface::PackageExplorerSurface)
        .expect("package explorer row present");
    assert!(green.status.is_green());
    let green_provenance = green
        .axis_outcomes
        .iter()
        .find(|o| o.axis == PackageComponentCertificationAxis::ScopeAndSourceProvenance)
        .expect("green row scores the provenance axis");
    assert_eq!(
        green_provenance.state,
        PackageComponentAxisCertificationState::Certified
    );
    assert!(!green_provenance.note.trim().is_empty());

    let narrowed = rows
        .iter()
        .find(|r| r.surface == M5PackageComponentCertifiedSurface::ExportedPackageReviewPacket)
        .expect("exported packet row present");
    let narrowed_provenance = narrowed
        .axis_outcomes
        .iter()
        .find(|o| o.axis == PackageComponentCertificationAxis::ScopeAndSourceProvenance)
        .expect("narrowed row scores the provenance axis");
    assert_eq!(
        narrowed_provenance.state,
        PackageComponentAxisCertificationState::NarrowedCertified
    );
}

#[test]
fn status_mismatch_fails() {
    let mut packet = packet();
    packet.surface_rows[0].status = PackageComponentSurfaceClaimStatus::NarrowedParity;
    assert!(packet
        .validate()
        .contains(&PackageComponentCertificationViolation::StatusMismatch));
}

#[test]
fn narrowed_axes_inconsistent_fails() {
    let mut packet = packet();
    // Push an axis into narrowed_axes without marking its outcome narrowed.
    packet.surface_rows[0]
        .narrowed_axes
        .push(PackageComponentCertificationAxis::Keyboard);
    assert!(packet
        .validate()
        .contains(&PackageComponentCertificationViolation::NarrowedAxesInconsistent));
}

#[test]
fn narrowing_without_trigger_fails() {
    let mut packet = packet();
    let index = packet
        .surface_rows
        .iter()
        .position(|r| !r.narrowed_axes.is_empty())
        .expect("narrowed row present");
    packet.surface_rows[index].downgrade_trigger = None;
    assert!(packet
        .validate()
        .contains(&PackageComponentCertificationViolation::NarrowingWithoutTrigger));
}

// --- Delta: certification never drops component truth --------------------------

#[test]
fn component_truth_dropped_fails() {
    let mut packet = packet();
    packet.surface_rows[0].component_truth_preserved = false;
    // Keep status consistent so only the truth-dropped rule fires cleanly.
    packet.surface_rows[0].status = PackageComponentSurfaceClaimStatus::ParityBlocked;
    packet.summary = PackageComponentCertificationSummary::from_rows(&packet.surface_rows);
    assert!(packet
        .validate()
        .contains(&PackageComponentCertificationViolation::PackageComponentTruthDropped));
}

// --- AC1: parity across keyboard / screen-reader / CLI / export ----------------

#[test]
fn keyboard_label_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].keyboard_label = String::new();
    assert!(packet
        .validate()
        .contains(&PackageComponentCertificationViolation::KeyboardLabelMissing));
}

#[test]
fn screen_reader_label_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].screen_reader_label = String::new();
    assert!(packet
        .validate()
        .contains(&PackageComponentCertificationViolation::ScreenReaderLabelMissing));
}

#[test]
fn cli_enum_token_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].cli_enum_token = String::new();
    assert!(packet
        .validate()
        .contains(&PackageComponentCertificationViolation::CliEnumTokenMissing));
}

#[test]
fn export_enum_token_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].export_enum_token = String::new();
    assert!(packet
        .validate()
        .contains(&PackageComponentCertificationViolation::ExportEnumTokenMissing));
}

#[test]
fn explanation_field_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].explanation_field = String::new();
    assert!(packet
        .validate()
        .contains(&PackageComponentCertificationViolation::ExplanationFieldMissing));
}

#[test]
fn axis_coverage_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].axis_outcomes.truncate(3);
    assert!(packet
        .validate()
        .contains(&PackageComponentCertificationViolation::AxisCoverageMissing));
}

#[test]
fn axis_note_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].axis_outcomes[0].note = String::new();
    assert!(packet
        .validate()
        .contains(&PackageComponentCertificationViolation::AxisNoteMissing));
}

#[test]
fn components_missing_on_row_fails() {
    let mut packet = packet();
    packet.surface_rows[0].components_present.clear();
    assert!(packet
        .validate()
        .contains(&PackageComponentCertificationViolation::ComponentsMissingOnRow));
}

#[test]
fn canonical_contract_reference_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].source_contract_refs =
        vec![M5_PACKAGE_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF.to_owned()];
    assert!(packet
        .validate()
        .contains(&PackageComponentCertificationViolation::CanonicalContractReferenceMissing));
}

// --- Coverage -----------------------------------------------------------------

#[test]
fn surface_coverage_missing_fails() {
    let mut rows = surface_rows();
    rows.retain(|r| r.surface != M5PackageComponentCertifiedSurface::Diagnostics);
    let packet = packet_with(rows);
    assert!(packet
        .validate()
        .contains(&PackageComponentCertificationViolation::SurfaceCoverageMissing));
}

#[test]
fn component_coverage_missing_fails() {
    let mut rows = surface_rows();
    // Remove the grouped-update planner from every surface that presents it.
    for row in &mut rows {
        row.components_present
            .retain(|c| *c != M5PackageComponent::GroupedUpdatePlanner);
    }
    let packet = packet_with(rows);
    assert!(packet
        .validate()
        .contains(&PackageComponentCertificationViolation::ComponentCoverageMissing));
}

// --- Structural ---------------------------------------------------------------

#[test]
fn summary_mismatch_fails() {
    let mut packet = packet();
    packet.summary.certified_count += 1;
    assert!(packet
        .validate()
        .contains(&PackageComponentCertificationViolation::SummaryMismatch));
}

#[test]
fn row_incomplete_fails() {
    let mut packet = packet();
    packet.surface_rows[0].row_id = String::new();
    assert!(packet
        .validate()
        .contains(&PackageComponentCertificationViolation::RowIncomplete));
}

#[test]
fn missing_rows_fails() {
    let mut packet = packet();
    packet.surface_rows.clear();
    packet.summary = PackageComponentCertificationSummary::from_rows(&packet.surface_rows);
    assert!(packet
        .validate()
        .contains(&PackageComponentCertificationViolation::SurfaceRowsMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&PackageComponentCertificationViolation::MissingSourceContracts));
}

#[test]
fn downgrade_triggers_missing_fails() {
    let mut packet = packet();
    packet.downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&PackageComponentCertificationViolation::DowngradeTriggersMissing));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.certified_never_implies_safe_one_click = false;
    assert!(packet
        .validate()
        .contains(&PackageComponentCertificationViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .narrowed_surfaces_visibly_labelled = false;
    assert!(packet
        .validate()
        .contains(&PackageComponentCertificationViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&PackageComponentCertificationViolation::ProofFreshnessIncomplete));
}

// --- Auto-narrowing (AC2 release automation) ----------------------------------

#[test]
fn apply_downgrade_narrows_stale_scope_surface() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[PackageComponentCertObservation {
        surface: M5PackageComponentCertifiedSurface::PackageExplorerSurface,
        scope_and_registry_fresh: false,
        component_truth_preserved: true,
    }]);
    let row = packet
        .surface_rows
        .iter()
        .find(|r| r.surface == M5PackageComponentCertifiedSurface::PackageExplorerSurface)
        .expect("package explorer row present");
    assert_eq!(
        row.status,
        PackageComponentSurfaceClaimStatus::NarrowedParity
    );
    assert_eq!(
        row.certified_claim,
        PackageComponentClaimTier::MirrorOrOfflineSourced
    );
    assert!(row
        .narrowed_axes
        .contains(&PackageComponentCertificationAxis::ScopeAndSourceProvenance));
    assert_eq!(
        row.downgrade_trigger,
        Some(PackageComponentCertificationDowngradeTrigger::RegistryFreshnessStale)
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.summary.certified_count, 3);
    assert_eq!(packet.summary.narrowed_count, 5);
}

#[test]
fn apply_downgrade_blocks_flattened_surface() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[PackageComponentCertObservation {
        surface: M5PackageComponentCertifiedSurface::SupportExport,
        scope_and_registry_fresh: true,
        component_truth_preserved: false,
    }]);
    let row = packet
        .surface_rows
        .iter()
        .find(|r| r.surface == M5PackageComponentCertifiedSurface::SupportExport)
        .expect("support export row present");
    assert_eq!(
        row.status,
        PackageComponentSurfaceClaimStatus::ParityBlocked
    );
    assert!(packet
        .validate()
        .contains(&PackageComponentCertificationViolation::PackageComponentTruthDropped));
    assert_eq!(packet.summary.blocked_count, 1);
    assert!(!packet.summary.all_rows_preserve_component_truth);
}

// --- Rendering ----------------------------------------------------------------

#[test]
fn markdown_summary_lists_surfaces() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Certified surfaces"));
    assert!(summary.contains("package_explorer_surface"));
    assert!(summary.contains("exported_package_review_packet"));
    assert!(summary.contains("certified_parity"));
    assert!(summary.contains("narrowed_parity"));
}

#[test]
fn matrix_csv_has_header_and_rows() {
    let csv = packet().render_matrix_csv();
    assert!(csv.starts_with(
        "row_id,surface,claimed_claim,certified_claim,status,narrowed_axes,component_truth_preserved\n"
    ));
    assert!(csv.contains("cert:package-explorer,package_explorer_surface"));
    assert!(csv.contains("scope_and_source_provenance"));
}

// --- Checked artifacts --------------------------------------------------------

#[test]
fn checked_support_export_validates() {
    let packet = current_package_component_certification_export()
        .expect("checked package certification export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-package-management-component-certification/registry_freshness_stale_auto_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-package-management-component-certification/search_detail_and_cli_narrowed.json"
        )),
    ] {
        let packet: PackageComponentCertificationPacket =
            serde_json::from_str(raw).expect("fixture parses as certification packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

// --- Fixture builders ---------------------------------------------------------

fn fixture_registry_freshness_stale_auto_narrowed() -> PackageComponentCertificationPacket {
    let mut packet = packet();
    packet.packet_id =
        "package-certification:fixture:registry-freshness-stale-auto-narrowed".to_owned();
    packet.certification_label =
        "Package surface certification: registry freshness stale, claim auto-narrowed".to_owned();
    packet.apply_downgrade_automation(&[PackageComponentCertObservation {
        surface: M5PackageComponentCertifiedSurface::PackageExplorerSurface,
        scope_and_registry_fresh: false,
        component_truth_preserved: true,
    }]);
    packet
}

fn fixture_search_detail_and_cli_narrowed() -> PackageComponentCertificationPacket {
    let mut packet = packet();
    packet.packet_id = "package-certification:fixture:search-detail-and-cli-narrowed".to_owned();
    packet.certification_label =
        "Package surface certification: search detail and CLI scope narrowed".to_owned();
    packet.apply_downgrade_automation(&[
        PackageComponentCertObservation {
            surface: M5PackageComponentCertifiedSurface::DependencySearchDetailSurface,
            scope_and_registry_fresh: false,
            component_truth_preserved: true,
        },
        PackageComponentCertObservation {
            surface: M5PackageComponentCertifiedSurface::CliHeadless,
            scope_and_registry_fresh: false,
            component_truth_preserved: true,
        },
    ]);
    packet
}

/// Regenerates the checked-in release proof (support export, matrix, report) and
/// fixtures.
///
/// Gated behind `GEN_PACKAGE_COMPONENT_CERTIFICATION_ARTIFACTS` so it never writes
/// during a normal test run.
#[test]
fn regenerate_package_component_certification_artifacts() {
    if std::env::var("GEN_PACKAGE_COMPONENT_CERTIFICATION_ARTIFACTS").is_err() {
        return;
    }

    let manifest = env!("CARGO_MANIFEST_DIR");
    let root = format!("{manifest}/../..");

    let canonical = packet();
    assert!(
        canonical.validate().is_empty(),
        "{:?}",
        canonical.validate()
    );

    let release_dir = format!("{root}/artifacts/release/m5-package-management-certification-proof");
    std::fs::create_dir_all(&release_dir).expect("create release proof dir");
    std::fs::write(
        format!("{release_dir}/support_export.json"),
        format!("{}\n", canonical.export_safe_json()),
    )
    .expect("write release support export");
    std::fs::write(
        format!("{release_dir}/matrix.csv"),
        canonical.render_matrix_csv(),
    )
    .expect("write release matrix csv");
    std::fs::write(
        format!("{release_dir}/report.md"),
        canonical.render_markdown_summary(),
    )
    .expect("write release report");

    let fixture_dir = format!("{root}/fixtures/ui/m5-package-management-component-certification");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");
    for (name, fixture) in [
        (
            "registry_freshness_stale_auto_narrowed.json",
            fixture_registry_freshness_stale_auto_narrowed(),
        ),
        (
            "search_detail_and_cli_narrowed.json",
            fixture_search_detail_and_cli_narrowed(),
        ),
    ] {
        assert!(
            fixture.validate().is_empty(),
            "{name}: {:?}",
            fixture.validate()
        );
        std::fs::write(
            format!("{fixture_dir}/{name}"),
            format!("{}\n", fixture.export_safe_json()),
        )
        .expect("write fixture");
    }
}
