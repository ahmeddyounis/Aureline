use super::*;

const PACKET_ID: &str = "protected-path-governance-certification:stable:0001";

fn trust_review() -> GovernanceComponentCertificationTrustReview {
    canonical_trust_review()
}

fn consumer_projection() -> GovernanceComponentCertificationConsumerProjection {
    canonical_consumer_projection()
}

fn proof_freshness() -> GovernanceComponentCertificationProofFreshness {
    GovernanceComponentCertificationProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<GovernanceComponentCertificationDowngradeTrigger> {
    vec![
        GovernanceComponentCertificationDowngradeTrigger::ProofStale,
        GovernanceComponentCertificationDowngradeTrigger::ProviderEnforcementAdvisoryOrStale,
        GovernanceComponentCertificationDowngradeTrigger::OwnerBackupCoverageMissing,
        GovernanceComponentCertificationDowngradeTrigger::ApproverStateWaivedOrExpired,
        GovernanceComponentCertificationDowngradeTrigger::ReviewPackStale,
        GovernanceComponentCertificationDowngradeTrigger::PublicSurfaceEvidenceMissing,
    ]
}

fn source_contract_refs() -> Vec<String> {
    canonical_source_contract_refs()
}

fn row_refs(components: &[M5GovernanceComponent]) -> Vec<String> {
    let mut refs =
        vec![M5_GOVERNANCE_COMPONENT_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF.to_owned()];
    for component in components {
        let schema = certification_component_canonical_schema_ref(*component).to_owned();
        if !refs.contains(&schema) {
            refs.push(schema);
        }
    }
    refs
}

fn axis_note(axis: GovernanceComponentCertificationAxis, narrowed: bool) -> String {
    let base = match axis {
        GovernanceComponentCertificationAxis::Visual => {
            "Visual rendering carries the controlled component truth"
        }
        GovernanceComponentCertificationAxis::Keyboard => {
            "Keyboard reach and operation carry the controlled component truth"
        }
        GovernanceComponentCertificationAxis::ScreenReader => {
            "Screen-reader labelling carries the controlled component truth"
        }
        GovernanceComponentCertificationAxis::CliExport => {
            "CLI and export forms carry the controlled component truth"
        }
        GovernanceComponentCertificationAxis::DegradedState => {
            "Degraded enforcement, owner-coverage, approver, review-pack, or public-surface state is disclosed"
        }
        GovernanceComponentCertificationAxis::EnforcementOwnershipProvenance => {
            "The advisory-versus-authoritative, owner-source, and public-surface distinction stays explicit; certified never implies provider authority"
        }
    };
    if narrowed {
        format!("{base} — narrowed here with an honest fallback disclosed")
    } else {
        base.to_owned()
    }
}

fn axis_outcomes(
    narrowed_axis: Option<GovernanceComponentCertificationAxis>,
) -> Vec<GovernanceComponentCertAxisOutcome> {
    GovernanceComponentCertificationAxis::ALL
        .iter()
        .map(|axis| {
            let narrowed = narrowed_axis == Some(*axis);
            GovernanceComponentCertAxisOutcome {
                axis: *axis,
                state: if narrowed {
                    GovernanceComponentAxisCertificationState::NarrowedCertified
                } else {
                    GovernanceComponentAxisCertificationState::Certified
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
    surface: M5GovernanceCertifiedSurface,
    components: Vec<M5GovernanceComponent>,
    claimed: GovernanceComponentClaimTier,
    certified: GovernanceComponentClaimTier,
    narrowed_axis: Option<GovernanceComponentCertificationAxis>,
    trigger: Option<GovernanceComponentCertificationDowngradeTrigger>,
) -> GovernanceComponentCertifiedSurfaceRow {
    let narrowed_axes = narrowed_axis.map(|axis| vec![axis]).unwrap_or_default();
    let component_truth_preserved = true;
    let status = derive_governance_component_surface_claim_status(
        claimed,
        certified,
        component_truth_preserved,
        !narrowed_axes.is_empty(),
    );
    let refs = row_refs(&components);
    GovernanceComponentCertifiedSurfaceRow {
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
            "{}: focusable, Enter opens governance detail, Space toggles enforcement view",
            surface.as_str()
        ),
        screen_reader_label: format!(
            "{} governed-review surface, certified {}",
            surface.as_str(),
            certified.as_str()
        ),
        cli_enum_token: format!("{}:{}", surface.as_str(), status.as_str()),
        export_enum_token: status.as_str().to_owned(),
        explanation_field: format!(
            "{} presents controlled governance component truth; claim certified as {}",
            surface.as_str(),
            certified.as_str()
        ),
        source_contract_refs: refs,
    }
}

/// The canonical eight-surface set: four green, four narrowed, all eight components.
fn surface_rows() -> Vec<GovernanceComponentCertifiedSurfaceRow> {
    use GovernanceComponentCertificationAxis as Axis;
    use GovernanceComponentCertificationDowngradeTrigger as Trigger;
    use GovernanceComponentClaimTier as Tier;
    use M5GovernanceCertifiedSurface as S;
    use M5GovernanceComponent as C;
    vec![
        row(
            "cert:review-workspace",
            S::ReviewWorkspaceSurface,
            vec![C::ProtectedPathRow, C::OwnershipCard],
            Tier::FullGovernedAuthority,
            Tier::FullGovernedAuthority,
            None,
            None,
        ),
        row(
            "cert:merge-queue",
            S::MergeQueueSurface,
            vec![C::MergeControlBanner, C::MergeReadinessStrip],
            Tier::FullGovernedAuthority,
            Tier::FullGovernedAuthority,
            None,
            None,
        ),
        row(
            "cert:help",
            S::HelpGovernanceSurface,
            vec![C::ApproverMatrix],
            Tier::FullGovernedAuthority,
            Tier::FullGovernedAuthority,
            None,
            None,
        ),
        row(
            "cert:cli",
            S::CliHeadless,
            vec![C::DriRegistryRow, C::PublicSurfaceDiffCard],
            Tier::FullGovernedAuthority,
            Tier::FullGovernedAuthority,
            None,
            None,
        ),
        row(
            "cert:release-center",
            S::ReleaseCenterSurface,
            vec![C::PublicSurfaceDiffCard, C::ReviewPackSummary],
            Tier::FullGovernedAuthority,
            Tier::PublicSurfaceEvidenceWithheld,
            Some(Axis::DegradedState),
            Some(Trigger::PublicSurfaceEvidenceMissing),
        ),
        row(
            "cert:support-export",
            S::SupportExport,
            vec![C::OwnershipCard, C::DriRegistryRow],
            Tier::FullGovernedAuthority,
            Tier::OwnerBackupCoverageMissing,
            Some(Axis::DegradedState),
            Some(Trigger::OwnerBackupCoverageMissing),
        ),
        row(
            "cert:exported-packet",
            S::ExportedGovernancePacket,
            vec![C::ProtectedPathRow, C::ApproverMatrix],
            Tier::FullGovernedAuthority,
            Tier::AdvisoryEnforcementOnly,
            Some(Axis::EnforcementOwnershipProvenance),
            Some(Trigger::ProviderEnforcementAdvisoryOrStale),
        ),
        row(
            "cert:shiproom",
            S::ShiproomSurface,
            vec![C::MergeControlBanner, C::ReviewPackSummary],
            Tier::FullGovernedAuthority,
            Tier::ReviewPackStaleDisclosed,
            Some(Axis::DegradedState),
            Some(Trigger::ReviewPackStale),
        ),
    ]
}

fn packet_with(
    rows: Vec<GovernanceComponentCertifiedSurfaceRow>,
) -> GovernanceComponentCertificationPacket {
    let summary = GovernanceComponentCertificationSummary::from_rows(&rows);
    GovernanceComponentCertificationPacket::new(GovernanceComponentCertificationPacketInput {
        packet_id: PACKET_ID.to_owned(),
        certification_label: "Protected-path governance-component surface certification".to_owned(),
        surface_rows: rows,
        summary,
        downgrade_triggers: downgrade_triggers(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

fn packet() -> GovernanceComponentCertificationPacket {
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
            "row parity fields missing: {}",
            row.row_id
        );
    }
}

#[test]
fn derive_status_maps_claims_to_status() {
    use GovernanceComponentClaimTier as Tier;
    assert_eq!(
        derive_governance_component_surface_claim_status(
            Tier::FullGovernedAuthority,
            Tier::FullGovernedAuthority,
            true,
            false
        ),
        GovernanceComponentSurfaceClaimStatus::CertifiedParity
    );
    assert_eq!(
        derive_governance_component_surface_claim_status(
            Tier::FullGovernedAuthority,
            Tier::AdvisoryEnforcementOnly,
            true,
            false
        ),
        GovernanceComponentSurfaceClaimStatus::NarrowedParity
    );
    assert_eq!(
        derive_governance_component_surface_claim_status(
            Tier::FullGovernedAuthority,
            Tier::FullGovernedAuthority,
            true,
            true
        ),
        GovernanceComponentSurfaceClaimStatus::NarrowedParity
    );
    assert_eq!(
        derive_governance_component_surface_claim_status(
            Tier::FullGovernedAuthority,
            Tier::FullGovernedAuthority,
            false,
            false
        ),
        GovernanceComponentSurfaceClaimStatus::ParityBlocked
    );
}

// --- AC: certification never overstates governed authority --------------------

#[test]
fn certified_claim_exceeds_claimed_fails() {
    let mut packet = packet();
    packet.surface_rows[0].claimed_claim = GovernanceComponentClaimTier::AdvisoryEnforcementOnly;
    packet.surface_rows[0].certified_claim = GovernanceComponentClaimTier::FullGovernedAuthority;
    assert!(packet
        .validate()
        .contains(&GovernanceComponentCertificationViolation::CertifiedClaimExceedsClaimed));
}

#[test]
fn certified_never_implies_provider_authority() {
    // A fully-certified (green) surface and a narrowed surface both keep the
    // enforcement-ownership-provenance axis explicit, proving the axis is orthogonal
    // to the overall certified status: certification never implies provider authority.
    let rows = surface_rows();
    let green = rows
        .iter()
        .find(|r| r.surface == M5GovernanceCertifiedSurface::ReviewWorkspaceSurface)
        .expect("review workspace row present");
    assert!(green.status.is_green());
    let green_provenance = green
        .axis_outcomes
        .iter()
        .find(|o| o.axis == GovernanceComponentCertificationAxis::EnforcementOwnershipProvenance)
        .expect("green row scores the provenance axis");
    assert_eq!(
        green_provenance.state,
        GovernanceComponentAxisCertificationState::Certified
    );
    assert!(!green_provenance.note.trim().is_empty());

    let narrowed = rows
        .iter()
        .find(|r| r.surface == M5GovernanceCertifiedSurface::ExportedGovernancePacket)
        .expect("exported packet row present");
    let narrowed_provenance = narrowed
        .axis_outcomes
        .iter()
        .find(|o| o.axis == GovernanceComponentCertificationAxis::EnforcementOwnershipProvenance)
        .expect("narrowed row scores the provenance axis");
    assert_eq!(
        narrowed_provenance.state,
        GovernanceComponentAxisCertificationState::NarrowedCertified
    );
}

#[test]
fn status_mismatch_fails() {
    let mut packet = packet();
    packet.surface_rows[0].status = GovernanceComponentSurfaceClaimStatus::NarrowedParity;
    assert!(packet
        .validate()
        .contains(&GovernanceComponentCertificationViolation::StatusMismatch));
}

#[test]
fn narrowed_axes_inconsistent_fails() {
    let mut packet = packet();
    // Push an axis into narrowed_axes without marking its outcome narrowed.
    packet.surface_rows[0]
        .narrowed_axes
        .push(GovernanceComponentCertificationAxis::Keyboard);
    assert!(packet
        .validate()
        .contains(&GovernanceComponentCertificationViolation::NarrowedAxesInconsistent));
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
        .contains(&GovernanceComponentCertificationViolation::NarrowingWithoutTrigger));
}

// --- Delta: certification never drops component truth --------------------------

#[test]
fn component_truth_dropped_fails() {
    let mut packet = packet();
    packet.surface_rows[0].component_truth_preserved = false;
    // Keep status consistent so only the truth-dropped rule fires cleanly.
    packet.surface_rows[0].status = GovernanceComponentSurfaceClaimStatus::ParityBlocked;
    packet.summary = GovernanceComponentCertificationSummary::from_rows(&packet.surface_rows);
    assert!(packet
        .validate()
        .contains(&GovernanceComponentCertificationViolation::GovernanceComponentTruthDropped));
}

// --- AC: parity across keyboard / screen-reader / CLI / export -----------------

#[test]
fn keyboard_label_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].keyboard_label = String::new();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentCertificationViolation::KeyboardLabelMissing));
}

#[test]
fn screen_reader_label_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].screen_reader_label = String::new();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentCertificationViolation::ScreenReaderLabelMissing));
}

#[test]
fn cli_enum_token_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].cli_enum_token = String::new();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentCertificationViolation::CliEnumTokenMissing));
}

#[test]
fn export_enum_token_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].export_enum_token = String::new();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentCertificationViolation::ExportEnumTokenMissing));
}

#[test]
fn explanation_field_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].explanation_field = String::new();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentCertificationViolation::ExplanationFieldMissing));
}

#[test]
fn axis_coverage_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].axis_outcomes.truncate(3);
    assert!(packet
        .validate()
        .contains(&GovernanceComponentCertificationViolation::AxisCoverageMissing));
}

#[test]
fn axis_note_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].axis_outcomes[0].note = String::new();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentCertificationViolation::AxisNoteMissing));
}

#[test]
fn components_missing_on_row_fails() {
    let mut packet = packet();
    packet.surface_rows[0].components_present.clear();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentCertificationViolation::ComponentsMissingOnRow));
}

#[test]
fn canonical_contract_reference_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].source_contract_refs =
        vec![M5_GOVERNANCE_COMPONENT_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF.to_owned()];
    assert!(packet
        .validate()
        .contains(&GovernanceComponentCertificationViolation::CanonicalContractReferenceMissing));
}

// --- Coverage -----------------------------------------------------------------

#[test]
fn surface_coverage_missing_fails() {
    let mut rows = surface_rows();
    rows.retain(|r| r.surface != M5GovernanceCertifiedSurface::ShiproomSurface);
    let packet = packet_with(rows);
    assert!(packet
        .validate()
        .contains(&GovernanceComponentCertificationViolation::SurfaceCoverageMissing));
}

#[test]
fn component_coverage_missing_fails() {
    let mut rows = surface_rows();
    // Remove the merge-readiness strip from every surface that presents it.
    for row in &mut rows {
        row.components_present
            .retain(|c| *c != M5GovernanceComponent::MergeReadinessStrip);
    }
    let packet = packet_with(rows);
    assert!(packet
        .validate()
        .contains(&GovernanceComponentCertificationViolation::ComponentCoverageMissing));
}

// --- Structural ---------------------------------------------------------------

#[test]
fn summary_mismatch_fails() {
    let mut packet = packet();
    packet.summary.certified_count += 1;
    assert!(packet
        .validate()
        .contains(&GovernanceComponentCertificationViolation::SummaryMismatch));
}

#[test]
fn row_incomplete_fails() {
    let mut packet = packet();
    packet.surface_rows[0].row_id = String::new();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentCertificationViolation::RowIncomplete));
}

#[test]
fn missing_rows_fails() {
    let mut packet = packet();
    packet.surface_rows.clear();
    packet.summary = GovernanceComponentCertificationSummary::from_rows(&packet.surface_rows);
    assert!(packet
        .validate()
        .contains(&GovernanceComponentCertificationViolation::SurfaceRowsMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentCertificationViolation::MissingSourceContracts));
}

#[test]
fn downgrade_triggers_missing_fails() {
    let mut packet = packet();
    packet.downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentCertificationViolation::DowngradeTriggersMissing));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .trust_review
        .certified_never_implies_provider_authoritative_enforcement = false;
    assert!(packet
        .validate()
        .contains(&GovernanceComponentCertificationViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .narrowed_surfaces_visibly_labelled = false;
    assert!(packet
        .validate()
        .contains(&GovernanceComponentCertificationViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&GovernanceComponentCertificationViolation::ProofFreshnessIncomplete));
}

// --- Auto-narrowing (release automation) --------------------------------------

#[test]
fn apply_downgrade_narrows_stale_governance_surface() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[GovernanceComponentCertObservation {
        surface: M5GovernanceCertifiedSurface::ReviewWorkspaceSurface,
        governance_truth_fresh: false,
        component_truth_preserved: true,
    }]);
    let row = packet
        .surface_rows
        .iter()
        .find(|r| r.surface == M5GovernanceCertifiedSurface::ReviewWorkspaceSurface)
        .expect("review workspace row present");
    assert_eq!(
        row.status,
        GovernanceComponentSurfaceClaimStatus::NarrowedParity
    );
    assert_eq!(
        row.certified_claim,
        GovernanceComponentClaimTier::AdvisoryEnforcementOnly
    );
    assert!(row
        .narrowed_axes
        .contains(&GovernanceComponentCertificationAxis::EnforcementOwnershipProvenance));
    assert_eq!(
        row.downgrade_trigger,
        Some(GovernanceComponentCertificationDowngradeTrigger::ProviderEnforcementAdvisoryOrStale)
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.summary.certified_count, 3);
    assert_eq!(packet.summary.narrowed_count, 5);
}

#[test]
fn apply_downgrade_blocks_flattened_surface() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[GovernanceComponentCertObservation {
        surface: M5GovernanceCertifiedSurface::SupportExport,
        governance_truth_fresh: true,
        component_truth_preserved: false,
    }]);
    let row = packet
        .surface_rows
        .iter()
        .find(|r| r.surface == M5GovernanceCertifiedSurface::SupportExport)
        .expect("support export row present");
    assert_eq!(
        row.status,
        GovernanceComponentSurfaceClaimStatus::ParityBlocked
    );
    assert!(packet
        .validate()
        .contains(&GovernanceComponentCertificationViolation::GovernanceComponentTruthDropped));
    assert_eq!(packet.summary.blocked_count, 1);
    assert!(!packet.summary.all_rows_preserve_component_truth);
}

// --- Rendering ----------------------------------------------------------------

#[test]
fn markdown_summary_lists_surfaces() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Certified surfaces"));
    assert!(summary.contains("review_workspace_surface"));
    assert!(summary.contains("exported_governance_packet"));
    assert!(summary.contains("certified_parity"));
    assert!(summary.contains("narrowed_parity"));
}

#[test]
fn matrix_csv_has_header_and_rows() {
    let csv = packet().render_matrix_csv();
    assert!(csv.starts_with(
        "row_id,surface,claimed_claim,certified_claim,status,narrowed_axes,component_truth_preserved\n"
    ));
    assert!(csv.contains("cert:review-workspace,review_workspace_surface"));
    assert!(csv.contains("enforcement_ownership_provenance"));
}

// --- Checked artifacts --------------------------------------------------------

#[test]
fn checked_support_export_validates() {
    let packet = current_governance_component_certification_export()
        .expect("checked governance certification export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-protected-path-governance-component-certification/enforcement_stale_auto_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-protected-path-governance-component-certification/merge_queue_and_cli_narrowed.json"
        )),
    ] {
        let packet: GovernanceComponentCertificationPacket =
            serde_json::from_str(raw).expect("fixture parses as certification packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

// --- Fixture builders ---------------------------------------------------------

fn fixture_enforcement_stale_auto_narrowed() -> GovernanceComponentCertificationPacket {
    let mut packet = packet();
    packet.packet_id =
        "protected-path-governance-certification:fixture:enforcement-stale-auto-narrowed"
            .to_owned();
    packet.certification_label =
        "Protected-path governance surface certification: provider enforcement stale, claim auto-narrowed"
            .to_owned();
    packet.apply_downgrade_automation(&[GovernanceComponentCertObservation {
        surface: M5GovernanceCertifiedSurface::ReviewWorkspaceSurface,
        governance_truth_fresh: false,
        component_truth_preserved: true,
    }]);
    packet
}

fn fixture_merge_queue_and_cli_narrowed() -> GovernanceComponentCertificationPacket {
    let mut packet = packet();
    packet.packet_id =
        "protected-path-governance-certification:fixture:merge-queue-and-cli-narrowed".to_owned();
    packet.certification_label =
        "Protected-path governance surface certification: merge queue and CLI enforcement narrowed"
            .to_owned();
    packet.apply_downgrade_automation(&[
        GovernanceComponentCertObservation {
            surface: M5GovernanceCertifiedSurface::MergeQueueSurface,
            governance_truth_fresh: false,
            component_truth_preserved: true,
        },
        GovernanceComponentCertObservation {
            surface: M5GovernanceCertifiedSurface::CliHeadless,
            governance_truth_fresh: false,
            component_truth_preserved: true,
        },
    ]);
    packet
}

/// Regenerates the checked-in support export, summary, release proof, and fixtures.
///
/// Gated behind `GEN_GOVERNANCE_COMPONENT_CERTIFICATION_ARTIFACTS` so it never writes
/// during a normal test run.
#[test]
fn regenerate_governance_component_certification_artifacts() {
    if std::env::var("GEN_GOVERNANCE_COMPONENT_CERTIFICATION_ARTIFACTS").is_err() {
        return;
    }

    let manifest = env!("CARGO_MANIFEST_DIR");
    let root = format!("{manifest}/../..");
    let module = "certify_protected_path_row_ownership_card_approver_matrix_review_pack_summary_public_surface_diff_card_merge_control_banner_dri_registry_row_and_merge_readiness_strip_truth_on_every_claimed_m5_governed_review_and_release_surface";

    let canonical = packet();
    assert!(
        canonical.validate().is_empty(),
        "{:?}",
        canonical.validate()
    );

    let artifact_dir = format!("{root}/artifacts/review/m5/{module}");
    std::fs::create_dir_all(&artifact_dir).expect("create artifact dir");
    std::fs::write(
        format!("{artifact_dir}/support_export.json"),
        format!("{}\n", canonical.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(
        format!("{root}/artifacts/review/m5/{module}.md"),
        canonical.render_markdown_summary(),
    )
    .expect("write summary");

    let release_dir =
        format!("{root}/artifacts/release/m5-protected-path-governance-certification-proof");
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

    let fixture_dir =
        format!("{root}/fixtures/ui/m5-protected-path-governance-component-certification");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");
    for (name, fixture) in [
        (
            "enforcement_stale_auto_narrowed.json",
            fixture_enforcement_stale_auto_narrowed(),
        ),
        (
            "merge_queue_and_cli_narrowed.json",
            fixture_merge_queue_and_cli_narrowed(),
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
