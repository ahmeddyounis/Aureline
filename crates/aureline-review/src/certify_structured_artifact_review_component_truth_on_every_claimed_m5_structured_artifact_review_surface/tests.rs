use super::*;

const PACKET_ID: &str = "structured-artifact-certification:stable:0001";

fn trust_review() -> StructuredArtifactCertificationTrustReview {
    canonical_trust_review()
}

fn consumer_projection() -> StructuredArtifactCertificationConsumerProjection {
    canonical_consumer_projection()
}

fn proof_freshness() -> StructuredArtifactCertificationProofFreshness {
    StructuredArtifactCertificationProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<StructuredArtifactCertificationDowngradeTrigger> {
    vec![
        StructuredArtifactCertificationDowngradeTrigger::ProofStale,
        StructuredArtifactCertificationDowngradeTrigger::ParserSchemaUncertain,
        StructuredArtifactCertificationDowngradeTrigger::RenderTrustUnavailable,
        StructuredArtifactCertificationDowngradeTrigger::WriteBackSafetyUnavailable,
        StructuredArtifactCertificationDowngradeTrigger::MetadataWithheldRedaction,
    ]
}

fn source_contract_refs() -> Vec<String> {
    canonical_source_contract_refs()
}

fn row_refs(components: &[M5ArtifactComponent]) -> Vec<String> {
    let mut refs =
        vec![M5_STRUCTURED_ARTIFACT_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF.to_owned()];
    for component in components {
        let schema = certification_component_canonical_schema_ref(*component).to_owned();
        if !refs.contains(&schema) {
            refs.push(schema);
        }
    }
    refs
}

fn axis_note(axis: StructuredArtifactCertificationAxis, narrowed: bool) -> String {
    let base = match axis {
        StructuredArtifactCertificationAxis::Visual => "Visual rendering carries the controlled component truth",
        StructuredArtifactCertificationAxis::Keyboard => "Keyboard reach and operation carry the controlled component truth",
        StructuredArtifactCertificationAxis::ScreenReader => "Screen-reader labelling carries the controlled component truth",
        StructuredArtifactCertificationAxis::CliExport => "CLI and export forms carry the controlled component truth",
        StructuredArtifactCertificationAxis::DegradedState => "Degraded parser/schema, render-trust, write-back, or metadata state is disclosed",
        StructuredArtifactCertificationAxis::StructuredFidelityProvenance => "The structured-versus-raw and render-trust distinction stays explicit; certified never implies full fidelity",
    };
    if narrowed {
        format!("{base} — narrowed here with an honest fallback disclosed")
    } else {
        base.to_owned()
    }
}

fn axis_outcomes(
    narrowed_axis: Option<StructuredArtifactCertificationAxis>,
) -> Vec<StructuredArtifactCertAxisOutcome> {
    StructuredArtifactCertificationAxis::ALL
        .iter()
        .map(|axis| {
            let narrowed = narrowed_axis == Some(*axis);
            StructuredArtifactCertAxisOutcome {
                axis: *axis,
                state: if narrowed {
                    StructuredArtifactAxisCertificationState::NarrowedCertified
                } else {
                    StructuredArtifactAxisCertificationState::Certified
                },
                note: axis_note(*axis, narrowed),
            }
        })
        .collect()
}

/// Builds one certified surface row, deriving status from its claims and narrowing
/// so the fixture stays self-consistent.
#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    surface: M5StructuredArtifactCertifiedSurface,
    components: Vec<M5ArtifactComponent>,
    claimed: ArtifactReviewClaimTier,
    certified: ArtifactReviewClaimTier,
    narrowed_axis: Option<StructuredArtifactCertificationAxis>,
    trigger: Option<StructuredArtifactCertificationDowngradeTrigger>,
) -> StructuredArtifactCertifiedSurfaceRow {
    let narrowed_axes = narrowed_axis.map(|axis| vec![axis]).unwrap_or_default();
    let component_truth_preserved = true;
    let status = derive_structured_artifact_surface_claim_status(
        claimed,
        certified,
        component_truth_preserved,
        !narrowed_axes.is_empty(),
    );
    let refs = row_refs(&components);
    StructuredArtifactCertifiedSurfaceRow {
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
            "{}: focusable, Enter opens compare, Space toggles diff mode",
            surface.as_str()
        ),
        screen_reader_label: format!(
            "{} structured-artifact surface, certified {}",
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

/// The canonical eight-surface set: four green, four narrowed, all nine components.
fn surface_rows() -> Vec<StructuredArtifactCertifiedSurfaceRow> {
    use ArtifactReviewClaimTier as Tier;
    use M5ArtifactComponent as C;
    use M5StructuredArtifactCertifiedSurface as S;
    use StructuredArtifactCertificationAxis as Axis;
    use StructuredArtifactCertificationDowngradeTrigger as Trigger;
    vec![
        row(
            "cert:diff-toolbar",
            S::DiffToolbarSurface,
            vec![C::ArtifactIdentityBar, C::DiffModeSwitcher],
            Tier::FullStructuredFidelity,
            Tier::FullStructuredFidelity,
            None,
            None,
        ),
        row(
            "cert:merge-sheet",
            S::MergeSheetSurface,
            vec![C::MergeDecisionRow, C::GeneratedArtifactNotice],
            Tier::FullStructuredFidelity,
            Tier::FullStructuredFidelity,
            None,
            None,
        ),
        row(
            "cert:help",
            S::HelpArtifactSurface,
            vec![C::CompareSummaryCard],
            Tier::FullStructuredFidelity,
            Tier::FullStructuredFidelity,
            None,
            None,
        ),
        row(
            "cert:cli",
            S::CliHeadless,
            vec![C::StructureRow, C::RedactionOrTrustBadgeSet],
            Tier::FullStructuredFidelity,
            Tier::FullStructuredFidelity,
            None,
            None,
        ),
        row(
            "cert:workspace",
            S::ReviewWorkspaceSurface,
            vec![C::RenderedCompareViewer, C::MediaMetadataRail],
            Tier::FullStructuredFidelity,
            Tier::StructuredCompareOnly,
            Some(Axis::DegradedState),
            Some(Trigger::WriteBackSafetyUnavailable),
        ),
        row(
            "cert:support-export",
            S::SupportExport,
            vec![C::ArtifactIdentityBar, C::RedactionOrTrustBadgeSet],
            Tier::FullStructuredFidelity,
            Tier::MetadataWithheld,
            Some(Axis::DegradedState),
            Some(Trigger::MetadataWithheldRedaction),
        ),
        row(
            "cert:exported-packet",
            S::ExportedArtifactPacket,
            vec![C::StructureRow, C::CompareSummaryCard],
            Tier::FullStructuredFidelity,
            Tier::RawFallbackDisclosed,
            Some(Axis::StructuredFidelityProvenance),
            Some(Trigger::RenderTrustUnavailable),
        ),
        row(
            "cert:diagnostics",
            S::Diagnostics,
            vec![C::MergeDecisionRow, C::MediaMetadataRail],
            Tier::FullStructuredFidelity,
            Tier::PartialStructure,
            Some(Axis::DegradedState),
            Some(Trigger::ProofStale),
        ),
    ]
}

fn packet_with(
    rows: Vec<StructuredArtifactCertifiedSurfaceRow>,
) -> StructuredArtifactCertificationPacket {
    let summary = StructuredArtifactCertificationSummary::from_rows(&rows);
    StructuredArtifactCertificationPacket::new(StructuredArtifactCertificationPacketInput {
        packet_id: PACKET_ID.to_owned(),
        certification_label: "Structured-artifact review-component surface certification"
            .to_owned(),
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

fn packet() -> StructuredArtifactCertificationPacket {
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
    }
}

#[test]
fn derive_status_maps_claims_to_status() {
    use ArtifactReviewClaimTier as Tier;
    assert_eq!(
        derive_structured_artifact_surface_claim_status(
            Tier::FullStructuredFidelity,
            Tier::FullStructuredFidelity,
            true,
            false
        ),
        StructuredArtifactSurfaceClaimStatus::CertifiedParity
    );
    assert_eq!(
        derive_structured_artifact_surface_claim_status(
            Tier::FullStructuredFidelity,
            Tier::RawFallbackDisclosed,
            true,
            false
        ),
        StructuredArtifactSurfaceClaimStatus::NarrowedParity
    );
    assert_eq!(
        derive_structured_artifact_surface_claim_status(
            Tier::FullStructuredFidelity,
            Tier::FullStructuredFidelity,
            true,
            true
        ),
        StructuredArtifactSurfaceClaimStatus::NarrowedParity
    );
    assert_eq!(
        derive_structured_artifact_surface_claim_status(
            Tier::FullStructuredFidelity,
            Tier::FullStructuredFidelity,
            false,
            false
        ),
        StructuredArtifactSurfaceClaimStatus::ParityBlocked
    );
}

// --- AC2: certification never overstates structured fidelity ------------------

#[test]
fn certified_claim_exceeds_claimed_fails() {
    let mut packet = packet();
    packet.surface_rows[0].claimed_claim = ArtifactReviewClaimTier::RawFallbackDisclosed;
    packet.surface_rows[0].certified_claim = ArtifactReviewClaimTier::FullStructuredFidelity;
    assert!(packet
        .validate()
        .contains(&StructuredArtifactCertificationViolation::CertifiedClaimExceedsClaimed));
}

#[test]
fn certified_never_implies_full_fidelity() {
    // A fully-certified (green) surface and a narrowed surface both keep the
    // structured-fidelity-provenance axis explicit, proving the axis is orthogonal
    // to the overall certified status: certification never implies full fidelity.
    let rows = surface_rows();
    let green = rows
        .iter()
        .find(|r| r.surface == M5StructuredArtifactCertifiedSurface::DiffToolbarSurface)
        .expect("diff toolbar row present");
    assert!(green.status.is_green());
    let green_provenance = green
        .axis_outcomes
        .iter()
        .find(|o| o.axis == StructuredArtifactCertificationAxis::StructuredFidelityProvenance)
        .expect("green row scores the provenance axis");
    assert_eq!(
        green_provenance.state,
        StructuredArtifactAxisCertificationState::Certified
    );
    assert!(!green_provenance.note.trim().is_empty());

    let narrowed = rows
        .iter()
        .find(|r| r.surface == M5StructuredArtifactCertifiedSurface::ExportedArtifactPacket)
        .expect("exported packet row present");
    let narrowed_provenance = narrowed
        .axis_outcomes
        .iter()
        .find(|o| o.axis == StructuredArtifactCertificationAxis::StructuredFidelityProvenance)
        .expect("narrowed row scores the provenance axis");
    assert_eq!(
        narrowed_provenance.state,
        StructuredArtifactAxisCertificationState::NarrowedCertified
    );
}

#[test]
fn status_mismatch_fails() {
    let mut packet = packet();
    packet.surface_rows[0].status = StructuredArtifactSurfaceClaimStatus::NarrowedParity;
    assert!(packet
        .validate()
        .contains(&StructuredArtifactCertificationViolation::StatusMismatch));
}

#[test]
fn narrowed_axes_inconsistent_fails() {
    let mut packet = packet();
    // Push an axis into narrowed_axes without marking its outcome narrowed.
    packet.surface_rows[0]
        .narrowed_axes
        .push(StructuredArtifactCertificationAxis::Keyboard);
    assert!(packet
        .validate()
        .contains(&StructuredArtifactCertificationViolation::NarrowedAxesInconsistent));
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
        .contains(&StructuredArtifactCertificationViolation::NarrowingWithoutTrigger));
}

// --- Delta: certification never drops component truth --------------------------

#[test]
fn component_truth_dropped_fails() {
    let mut packet = packet();
    packet.surface_rows[0].component_truth_preserved = false;
    // Keep status consistent so only the truth-dropped rule fires cleanly.
    packet.surface_rows[0].status = StructuredArtifactSurfaceClaimStatus::ParityBlocked;
    packet.summary = StructuredArtifactCertificationSummary::from_rows(&packet.surface_rows);
    assert!(packet.validate().contains(
        &StructuredArtifactCertificationViolation::StructuredArtifactComponentTruthDropped
    ));
}

// --- AC1: parity across keyboard / screen-reader / CLI / export ----------------

#[test]
fn keyboard_label_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].keyboard_label = String::new();
    assert!(packet
        .validate()
        .contains(&StructuredArtifactCertificationViolation::KeyboardLabelMissing));
}

#[test]
fn screen_reader_label_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].screen_reader_label = String::new();
    assert!(packet
        .validate()
        .contains(&StructuredArtifactCertificationViolation::ScreenReaderLabelMissing));
}

#[test]
fn cli_enum_token_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].cli_enum_token = String::new();
    assert!(packet
        .validate()
        .contains(&StructuredArtifactCertificationViolation::CliEnumTokenMissing));
}

#[test]
fn export_enum_token_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].export_enum_token = String::new();
    assert!(packet
        .validate()
        .contains(&StructuredArtifactCertificationViolation::ExportEnumTokenMissing));
}

#[test]
fn explanation_field_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].explanation_field = String::new();
    assert!(packet
        .validate()
        .contains(&StructuredArtifactCertificationViolation::ExplanationFieldMissing));
}

#[test]
fn axis_coverage_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].axis_outcomes.truncate(3);
    assert!(packet
        .validate()
        .contains(&StructuredArtifactCertificationViolation::AxisCoverageMissing));
}

#[test]
fn axis_note_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].axis_outcomes[0].note = String::new();
    assert!(packet
        .validate()
        .contains(&StructuredArtifactCertificationViolation::AxisNoteMissing));
}

#[test]
fn components_missing_on_row_fails() {
    let mut packet = packet();
    packet.surface_rows[0].components_present.clear();
    assert!(packet
        .validate()
        .contains(&StructuredArtifactCertificationViolation::ComponentsMissingOnRow));
}

#[test]
fn canonical_contract_reference_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].source_contract_refs =
        vec![M5_STRUCTURED_ARTIFACT_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF.to_owned()];
    assert!(packet
        .validate()
        .contains(&StructuredArtifactCertificationViolation::CanonicalContractReferenceMissing));
}

// --- Coverage -----------------------------------------------------------------

#[test]
fn surface_coverage_missing_fails() {
    let mut rows = surface_rows();
    rows.retain(|r| r.surface != M5StructuredArtifactCertifiedSurface::Diagnostics);
    let packet = packet_with(rows);
    assert!(packet
        .validate()
        .contains(&StructuredArtifactCertificationViolation::SurfaceCoverageMissing));
}

#[test]
fn component_coverage_missing_fails() {
    let mut rows = surface_rows();
    // Remove the diff-mode switcher from every surface that presents it.
    for row in &mut rows {
        row.components_present
            .retain(|c| *c != M5ArtifactComponent::DiffModeSwitcher);
    }
    let packet = packet_with(rows);
    assert!(packet
        .validate()
        .contains(&StructuredArtifactCertificationViolation::ComponentCoverageMissing));
}

// --- Structural ---------------------------------------------------------------

#[test]
fn summary_mismatch_fails() {
    let mut packet = packet();
    packet.summary.certified_count += 1;
    assert!(packet
        .validate()
        .contains(&StructuredArtifactCertificationViolation::SummaryMismatch));
}

#[test]
fn row_incomplete_fails() {
    let mut packet = packet();
    packet.surface_rows[0].row_id = String::new();
    assert!(packet
        .validate()
        .contains(&StructuredArtifactCertificationViolation::RowIncomplete));
}

#[test]
fn missing_rows_fails() {
    let mut packet = packet();
    packet.surface_rows.clear();
    packet.summary = StructuredArtifactCertificationSummary::from_rows(&packet.surface_rows);
    assert!(packet
        .validate()
        .contains(&StructuredArtifactCertificationViolation::SurfaceRowsMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&StructuredArtifactCertificationViolation::MissingSourceContracts));
}

#[test]
fn downgrade_triggers_missing_fails() {
    let mut packet = packet();
    packet.downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&StructuredArtifactCertificationViolation::DowngradeTriggersMissing));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.certified_never_implies_full_fidelity = false;
    assert!(packet
        .validate()
        .contains(&StructuredArtifactCertificationViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .narrowed_surfaces_visibly_labelled = false;
    assert!(packet
        .validate()
        .contains(&StructuredArtifactCertificationViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&StructuredArtifactCertificationViolation::ProofFreshnessIncomplete));
}

// --- Auto-narrowing (AC2 release automation) ----------------------------------

#[test]
fn apply_downgrade_narrows_stale_fidelity_surface() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[StructuredArtifactCertObservation {
        surface: M5StructuredArtifactCertifiedSurface::DiffToolbarSurface,
        structured_fidelity_fresh: false,
        component_truth_preserved: true,
    }]);
    let row = packet
        .surface_rows
        .iter()
        .find(|r| r.surface == M5StructuredArtifactCertifiedSurface::DiffToolbarSurface)
        .expect("diff toolbar row present");
    assert_eq!(
        row.status,
        StructuredArtifactSurfaceClaimStatus::NarrowedParity
    );
    assert_eq!(
        row.certified_claim,
        ArtifactReviewClaimTier::RawFallbackDisclosed
    );
    assert!(row
        .narrowed_axes
        .contains(&StructuredArtifactCertificationAxis::StructuredFidelityProvenance));
    assert_eq!(
        row.downgrade_trigger,
        Some(StructuredArtifactCertificationDowngradeTrigger::RenderTrustUnavailable)
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.summary.certified_count, 3);
    assert_eq!(packet.summary.narrowed_count, 5);
}

#[test]
fn apply_downgrade_blocks_flattened_surface() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[StructuredArtifactCertObservation {
        surface: M5StructuredArtifactCertifiedSurface::SupportExport,
        structured_fidelity_fresh: true,
        component_truth_preserved: false,
    }]);
    let row = packet
        .surface_rows
        .iter()
        .find(|r| r.surface == M5StructuredArtifactCertifiedSurface::SupportExport)
        .expect("support export row present");
    assert_eq!(
        row.status,
        StructuredArtifactSurfaceClaimStatus::ParityBlocked
    );
    assert!(packet.validate().contains(
        &StructuredArtifactCertificationViolation::StructuredArtifactComponentTruthDropped
    ));
    assert_eq!(packet.summary.blocked_count, 1);
    assert!(!packet.summary.all_rows_preserve_component_truth);
}

// --- Rendering ----------------------------------------------------------------

#[test]
fn markdown_summary_lists_surfaces() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Certified surfaces"));
    assert!(summary.contains("diff_toolbar_surface"));
    assert!(summary.contains("exported_artifact_packet"));
    assert!(summary.contains("certified_parity"));
    assert!(summary.contains("narrowed_parity"));
}

#[test]
fn matrix_csv_has_header_and_rows() {
    let csv = packet().render_matrix_csv();
    assert!(csv.starts_with(
        "row_id,surface,claimed_claim,certified_claim,status,narrowed_axes,component_truth_preserved\n"
    ));
    assert!(csv.contains("cert:diff-toolbar,diff_toolbar_surface"));
    assert!(csv.contains("structured_fidelity_provenance"));
}

// --- Checked artifacts --------------------------------------------------------

#[test]
fn checked_support_export_validates() {
    let packet = current_structured_artifact_certification_export()
        .expect("checked structured-artifact certification export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-structured-artifact-review-component-certification/render_trust_stale_auto_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-structured-artifact-review-component-certification/merge_sheet_and_cli_narrowed.json"
        )),
    ] {
        let packet: StructuredArtifactCertificationPacket =
            serde_json::from_str(raw).expect("fixture parses as certification packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

// --- Fixture builders ---------------------------------------------------------

fn fixture_render_trust_stale_auto_narrowed() -> StructuredArtifactCertificationPacket {
    let mut packet = packet();
    packet.packet_id =
        "structured-artifact-certification:fixture:render-trust-stale-auto-narrowed".to_owned();
    packet.certification_label =
        "Structured-artifact surface certification: render trust stale, claim auto-narrowed"
            .to_owned();
    packet.apply_downgrade_automation(&[StructuredArtifactCertObservation {
        surface: M5StructuredArtifactCertifiedSurface::DiffToolbarSurface,
        structured_fidelity_fresh: false,
        component_truth_preserved: true,
    }]);
    packet
}

fn fixture_merge_sheet_and_cli_narrowed() -> StructuredArtifactCertificationPacket {
    let mut packet = packet();
    packet.packet_id =
        "structured-artifact-certification:fixture:merge-sheet-and-cli-narrowed".to_owned();
    packet.certification_label =
        "Structured-artifact surface certification: merge sheet and CLI structured fidelity narrowed"
            .to_owned();
    packet.apply_downgrade_automation(&[
        StructuredArtifactCertObservation {
            surface: M5StructuredArtifactCertifiedSurface::MergeSheetSurface,
            structured_fidelity_fresh: false,
            component_truth_preserved: true,
        },
        StructuredArtifactCertObservation {
            surface: M5StructuredArtifactCertifiedSurface::CliHeadless,
            structured_fidelity_fresh: false,
            component_truth_preserved: true,
        },
    ]);
    packet
}

/// Regenerates the checked-in support export, summary, release proof, and fixtures.
///
/// Gated behind `GEN_STRUCTURED_ARTIFACT_CERTIFICATION_ARTIFACTS` so it never writes
/// during a normal test run.
#[test]
fn regenerate_structured_artifact_certification_artifacts() {
    if std::env::var("GEN_STRUCTURED_ARTIFACT_CERTIFICATION_ARTIFACTS").is_err() {
        return;
    }

    let manifest = env!("CARGO_MANIFEST_DIR");
    let root = format!("{manifest}/../..");
    let module = "certify_structured_artifact_review_component_truth_on_every_claimed_m5_structured_artifact_review_surface";

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
        format!("{root}/artifacts/release/m5-structured-artifact-review-certification-proof");
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
        format!("{root}/fixtures/ui/m5-structured-artifact-review-component-certification");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");
    for (name, fixture) in [
        (
            "render_trust_stale_auto_narrowed.json",
            fixture_render_trust_stale_auto_narrowed(),
        ),
        (
            "merge_sheet_and_cli_narrowed.json",
            fixture_merge_sheet_and_cli_narrowed(),
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
