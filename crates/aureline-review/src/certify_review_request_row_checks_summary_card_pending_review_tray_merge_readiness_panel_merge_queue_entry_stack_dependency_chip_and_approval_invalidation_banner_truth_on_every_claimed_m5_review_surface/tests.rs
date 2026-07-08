use super::*;

const PACKET_ID: &str = "review-component-certification:stable:0001";

fn trust_review() -> ReviewComponentCertificationTrustReview {
    canonical_trust_review()
}

fn consumer_projection() -> ReviewComponentCertificationConsumerProjection {
    canonical_consumer_projection()
}

fn proof_freshness() -> ReviewComponentCertificationProofFreshness {
    ReviewComponentCertificationProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<ReviewComponentCertificationDowngradeTrigger> {
    vec![
        ReviewComponentCertificationDowngradeTrigger::ProofStale,
        ReviewComponentCertificationDowngradeTrigger::ProviderFreshnessStale,
        ReviewComponentCertificationDowngradeTrigger::QueueAuthorityLocalEstimate,
        ReviewComponentCertificationDowngradeTrigger::ApprovalLineageMissing,
        ReviewComponentCertificationDowngradeTrigger::StackDriftUnresolved,
    ]
}

fn source_contract_refs() -> Vec<String> {
    canonical_source_contract_refs()
}

fn row_refs(components: &[M5ReviewComponent]) -> Vec<String> {
    let mut refs = vec![M5_REVIEW_COMPONENT_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF.to_owned()];
    for component in components {
        let schema = certification_component_canonical_schema_ref(*component).to_owned();
        if !refs.contains(&schema) {
            refs.push(schema);
        }
    }
    refs
}

fn axis_note(axis: ReviewComponentCertificationAxis, narrowed: bool) -> String {
    let base = match axis {
        ReviewComponentCertificationAxis::Visual => "Visual rendering carries the controlled component truth",
        ReviewComponentCertificationAxis::Keyboard => "Keyboard reach and operation carry the controlled component truth",
        ReviewComponentCertificationAxis::ScreenReader => "Screen-reader labelling carries the controlled component truth",
        ReviewComponentCertificationAxis::CliExport => "CLI and export forms carry the controlled component truth",
        ReviewComponentCertificationAxis::DegradedState => "Degraded provider, queue, approval, or stack state is disclosed",
        ReviewComponentCertificationAxis::ProviderLocalProvenance => "The provider-backed versus local distinction stays explicit; certified never implies fresh",
    };
    if narrowed {
        format!("{base} — narrowed here with an honest fallback disclosed")
    } else {
        base.to_owned()
    }
}

fn axis_outcomes(
    narrowed_axis: Option<ReviewComponentCertificationAxis>,
) -> Vec<ReviewComponentCertAxisOutcome> {
    ReviewComponentCertificationAxis::ALL
        .iter()
        .map(|axis| {
            let narrowed = narrowed_axis == Some(*axis);
            ReviewComponentCertAxisOutcome {
                axis: *axis,
                state: if narrowed {
                    ReviewComponentAxisCertificationState::NarrowedCertified
                } else {
                    ReviewComponentAxisCertificationState::Certified
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
    surface: M5ReviewComponentCertifiedSurface,
    components: Vec<M5ReviewComponent>,
    claimed: ReviewComponentClaimTier,
    certified: ReviewComponentClaimTier,
    narrowed_axis: Option<ReviewComponentCertificationAxis>,
    trigger: Option<ReviewComponentCertificationDowngradeTrigger>,
) -> ReviewComponentCertifiedSurfaceRow {
    let narrowed_axes = narrowed_axis.map(|axis| vec![axis]).unwrap_or_default();
    let component_truth_preserved = true;
    let status = derive_review_component_surface_claim_status(
        claimed,
        certified,
        component_truth_preserved,
        !narrowed_axes.is_empty(),
    );
    let refs = row_refs(&components);
    ReviewComponentCertifiedSurfaceRow {
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
            "{}: focusable, Enter opens review, Space toggles detail",
            surface.as_str()
        ),
        screen_reader_label: format!(
            "{} review surface, certified {}",
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

/// The canonical eight-surface set: four green, four narrowed, all seven components.
fn surface_rows() -> Vec<ReviewComponentCertifiedSurfaceRow> {
    use M5ReviewComponent as C;
    use M5ReviewComponentCertifiedSurface as S;
    use ReviewComponentCertificationAxis as Axis;
    use ReviewComponentCertificationDowngradeTrigger as Trigger;
    use ReviewComponentClaimTier as Tier;
    vec![
        row(
            "cert:desktop-list",
            S::DesktopReviewList,
            vec![C::ReviewRequestRow, C::ChecksSummaryCard],
            Tier::ProviderBacked,
            Tier::ProviderBacked,
            None,
            None,
        ),
        row(
            "cert:detail-pane",
            S::ReviewDetailPane,
            vec![C::MergeReadinessPanel, C::StackDependencyChip],
            Tier::ProviderBacked,
            Tier::ProviderBacked,
            None,
            None,
        ),
        row(
            "cert:companion-queue",
            S::CompanionReviewQueue,
            vec![C::MergeQueueEntry, C::PendingReviewTray],
            Tier::ProviderBacked,
            Tier::EstimateOnly,
            Some(Axis::DegradedState),
            Some(Trigger::QueueAuthorityLocalEstimate),
        ),
        row(
            "cert:help",
            S::HelpReviewSurface,
            vec![C::ApprovalInvalidationBanner],
            Tier::ProviderBacked,
            Tier::ProviderBacked,
            None,
            None,
        ),
        row(
            "cert:support-export",
            S::SupportExport,
            vec![C::ReviewRequestRow, C::ApprovalInvalidationBanner],
            Tier::ProviderBacked,
            Tier::ApprovalUnverified,
            Some(Axis::DegradedState),
            Some(Trigger::ApprovalLineageMissing),
        ),
        row(
            "cert:exported-packet",
            S::ExportedReviewPacket,
            vec![C::ChecksSummaryCard, C::MergeReadinessPanel],
            Tier::ProviderBacked,
            Tier::LocallyReviewable,
            Some(Axis::ProviderLocalProvenance),
            Some(Trigger::ProviderFreshnessStale),
        ),
        row(
            "cert:cli",
            S::CliHeadless,
            vec![C::StackDependencyChip, C::MergeQueueEntry],
            Tier::ProviderBacked,
            Tier::ProviderBacked,
            None,
            None,
        ),
        row(
            "cert:diagnostics",
            S::Diagnostics,
            vec![C::PendingReviewTray, C::ChecksSummaryCard],
            Tier::ProviderBacked,
            Tier::LocallyReviewable,
            Some(Axis::DegradedState),
            Some(Trigger::ProofStale),
        ),
    ]
}

fn packet_with(
    rows: Vec<ReviewComponentCertifiedSurfaceRow>,
) -> ReviewComponentCertificationPacket {
    let summary = ReviewComponentCertificationSummary::from_rows(&rows);
    ReviewComponentCertificationPacket::new(ReviewComponentCertificationPacketInput {
        packet_id: PACKET_ID.to_owned(),
        certification_label: "Review-component surface certification".to_owned(),
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

fn packet() -> ReviewComponentCertificationPacket {
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
    use ReviewComponentClaimTier as Tier;
    assert_eq!(
        derive_review_component_surface_claim_status(
            Tier::ProviderBacked,
            Tier::ProviderBacked,
            true,
            false
        ),
        ReviewComponentSurfaceClaimStatus::CertifiedParity
    );
    assert_eq!(
        derive_review_component_surface_claim_status(
            Tier::ProviderBacked,
            Tier::LocallyReviewable,
            true,
            false
        ),
        ReviewComponentSurfaceClaimStatus::NarrowedParity
    );
    assert_eq!(
        derive_review_component_surface_claim_status(
            Tier::ProviderBacked,
            Tier::ProviderBacked,
            true,
            true
        ),
        ReviewComponentSurfaceClaimStatus::NarrowedParity
    );
    assert_eq!(
        derive_review_component_surface_claim_status(
            Tier::ProviderBacked,
            Tier::ProviderBacked,
            false,
            false
        ),
        ReviewComponentSurfaceClaimStatus::ParityBlocked
    );
}

// --- AC2: certification never overstates provider-backed truth ----------------

#[test]
fn certified_claim_exceeds_claimed_fails() {
    let mut packet = packet();
    packet.surface_rows[0].claimed_claim = ReviewComponentClaimTier::LocallyReviewable;
    packet.surface_rows[0].certified_claim = ReviewComponentClaimTier::ProviderBacked;
    assert!(packet
        .validate()
        .contains(&ReviewComponentCertificationViolation::CertifiedClaimExceedsClaimed));
}

#[test]
fn certified_never_implies_provider_fresh() {
    // A fully-certified (green) surface and a narrowed surface both keep the
    // provider/local-provenance axis explicit, proving the axis is orthogonal to
    // the overall certified status: certification never implies provider freshness.
    let rows = surface_rows();
    let green = rows
        .iter()
        .find(|r| r.surface == M5ReviewComponentCertifiedSurface::DesktopReviewList)
        .expect("desktop list row present");
    assert!(green.status.is_green());
    let green_provenance = green
        .axis_outcomes
        .iter()
        .find(|o| o.axis == ReviewComponentCertificationAxis::ProviderLocalProvenance)
        .expect("green row scores the provenance axis");
    assert_eq!(
        green_provenance.state,
        ReviewComponentAxisCertificationState::Certified
    );
    assert!(!green_provenance.note.trim().is_empty());

    let narrowed = rows
        .iter()
        .find(|r| r.surface == M5ReviewComponentCertifiedSurface::ExportedReviewPacket)
        .expect("exported packet row present");
    let narrowed_provenance = narrowed
        .axis_outcomes
        .iter()
        .find(|o| o.axis == ReviewComponentCertificationAxis::ProviderLocalProvenance)
        .expect("narrowed row scores the provenance axis");
    assert_eq!(
        narrowed_provenance.state,
        ReviewComponentAxisCertificationState::NarrowedCertified
    );
}

#[test]
fn status_mismatch_fails() {
    let mut packet = packet();
    packet.surface_rows[0].status = ReviewComponentSurfaceClaimStatus::NarrowedParity;
    assert!(packet
        .validate()
        .contains(&ReviewComponentCertificationViolation::StatusMismatch));
}

#[test]
fn narrowed_axes_inconsistent_fails() {
    let mut packet = packet();
    // Push an axis into narrowed_axes without marking its outcome narrowed.
    packet.surface_rows[0]
        .narrowed_axes
        .push(ReviewComponentCertificationAxis::Keyboard);
    assert!(packet
        .validate()
        .contains(&ReviewComponentCertificationViolation::NarrowedAxesInconsistent));
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
        .contains(&ReviewComponentCertificationViolation::NarrowingWithoutTrigger));
}

// --- Delta: certification never drops component truth --------------------------

#[test]
fn component_truth_dropped_fails() {
    let mut packet = packet();
    packet.surface_rows[0].component_truth_preserved = false;
    // Keep status consistent so only the truth-dropped rule fires cleanly.
    packet.surface_rows[0].status = ReviewComponentSurfaceClaimStatus::ParityBlocked;
    packet.summary = ReviewComponentCertificationSummary::from_rows(&packet.surface_rows);
    assert!(packet
        .validate()
        .contains(&ReviewComponentCertificationViolation::ReviewComponentTruthDropped));
}

// --- AC1: parity across keyboard / screen-reader / CLI / export ----------------

#[test]
fn keyboard_label_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].keyboard_label = String::new();
    assert!(packet
        .validate()
        .contains(&ReviewComponentCertificationViolation::KeyboardLabelMissing));
}

#[test]
fn screen_reader_label_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].screen_reader_label = String::new();
    assert!(packet
        .validate()
        .contains(&ReviewComponentCertificationViolation::ScreenReaderLabelMissing));
}

#[test]
fn cli_enum_token_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].cli_enum_token = String::new();
    assert!(packet
        .validate()
        .contains(&ReviewComponentCertificationViolation::CliEnumTokenMissing));
}

#[test]
fn export_enum_token_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].export_enum_token = String::new();
    assert!(packet
        .validate()
        .contains(&ReviewComponentCertificationViolation::ExportEnumTokenMissing));
}

#[test]
fn explanation_field_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].explanation_field = String::new();
    assert!(packet
        .validate()
        .contains(&ReviewComponentCertificationViolation::ExplanationFieldMissing));
}

#[test]
fn axis_coverage_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].axis_outcomes.truncate(3);
    assert!(packet
        .validate()
        .contains(&ReviewComponentCertificationViolation::AxisCoverageMissing));
}

#[test]
fn axis_note_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].axis_outcomes[0].note = String::new();
    assert!(packet
        .validate()
        .contains(&ReviewComponentCertificationViolation::AxisNoteMissing));
}

#[test]
fn components_missing_on_row_fails() {
    let mut packet = packet();
    packet.surface_rows[0].components_present.clear();
    assert!(packet
        .validate()
        .contains(&ReviewComponentCertificationViolation::ComponentsMissingOnRow));
}

#[test]
fn canonical_contract_reference_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].source_contract_refs =
        vec![M5_REVIEW_COMPONENT_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF.to_owned()];
    assert!(packet
        .validate()
        .contains(&ReviewComponentCertificationViolation::CanonicalContractReferenceMissing));
}

// --- Coverage -----------------------------------------------------------------

#[test]
fn surface_coverage_missing_fails() {
    let mut rows = surface_rows();
    rows.retain(|r| r.surface != M5ReviewComponentCertifiedSurface::Diagnostics);
    let packet = packet_with(rows);
    assert!(packet
        .validate()
        .contains(&ReviewComponentCertificationViolation::SurfaceCoverageMissing));
}

#[test]
fn component_coverage_missing_fails() {
    let mut rows = surface_rows();
    // Remove the stack-dependency chip from every surface that presents it.
    for row in &mut rows {
        row.components_present
            .retain(|c| *c != M5ReviewComponent::StackDependencyChip);
    }
    let packet = packet_with(rows);
    assert!(packet
        .validate()
        .contains(&ReviewComponentCertificationViolation::ComponentCoverageMissing));
}

// --- Structural ---------------------------------------------------------------

#[test]
fn summary_mismatch_fails() {
    let mut packet = packet();
    packet.summary.certified_count += 1;
    assert!(packet
        .validate()
        .contains(&ReviewComponentCertificationViolation::SummaryMismatch));
}

#[test]
fn row_incomplete_fails() {
    let mut packet = packet();
    packet.surface_rows[0].row_id = String::new();
    assert!(packet
        .validate()
        .contains(&ReviewComponentCertificationViolation::RowIncomplete));
}

#[test]
fn missing_rows_fails() {
    let mut packet = packet();
    packet.surface_rows.clear();
    packet.summary = ReviewComponentCertificationSummary::from_rows(&packet.surface_rows);
    assert!(packet
        .validate()
        .contains(&ReviewComponentCertificationViolation::SurfaceRowsMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&ReviewComponentCertificationViolation::MissingSourceContracts));
}

#[test]
fn downgrade_triggers_missing_fails() {
    let mut packet = packet();
    packet.downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&ReviewComponentCertificationViolation::DowngradeTriggersMissing));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.certified_never_implies_fresh = false;
    assert!(packet
        .validate()
        .contains(&ReviewComponentCertificationViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .narrowed_surfaces_visibly_labelled = false;
    assert!(packet
        .validate()
        .contains(&ReviewComponentCertificationViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&ReviewComponentCertificationViolation::ProofFreshnessIncomplete));
}

// --- Auto-narrowing (AC2 release automation) ----------------------------------

#[test]
fn apply_downgrade_narrows_stale_provider_surface() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[ReviewComponentCertObservation {
        surface: M5ReviewComponentCertifiedSurface::DesktopReviewList,
        provider_fresh: false,
        component_truth_preserved: true,
    }]);
    let row = packet
        .surface_rows
        .iter()
        .find(|r| r.surface == M5ReviewComponentCertifiedSurface::DesktopReviewList)
        .expect("desktop list row present");
    assert_eq!(
        row.status,
        ReviewComponentSurfaceClaimStatus::NarrowedParity
    );
    assert_eq!(
        row.certified_claim,
        ReviewComponentClaimTier::LocallyReviewable
    );
    assert!(row
        .narrowed_axes
        .contains(&ReviewComponentCertificationAxis::ProviderLocalProvenance));
    assert_eq!(
        row.downgrade_trigger,
        Some(ReviewComponentCertificationDowngradeTrigger::ProviderFreshnessStale)
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.summary.certified_count, 3);
    assert_eq!(packet.summary.narrowed_count, 5);
}

#[test]
fn apply_downgrade_blocks_flattened_surface() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[ReviewComponentCertObservation {
        surface: M5ReviewComponentCertifiedSurface::SupportExport,
        provider_fresh: true,
        component_truth_preserved: false,
    }]);
    let row = packet
        .surface_rows
        .iter()
        .find(|r| r.surface == M5ReviewComponentCertifiedSurface::SupportExport)
        .expect("support export row present");
    assert_eq!(row.status, ReviewComponentSurfaceClaimStatus::ParityBlocked);
    assert!(packet
        .validate()
        .contains(&ReviewComponentCertificationViolation::ReviewComponentTruthDropped));
    assert_eq!(packet.summary.blocked_count, 1);
    assert!(!packet.summary.all_rows_preserve_component_truth);
}

// --- Rendering ----------------------------------------------------------------

#[test]
fn markdown_summary_lists_surfaces() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Certified surfaces"));
    assert!(summary.contains("desktop_review_list"));
    assert!(summary.contains("exported_review_packet"));
    assert!(summary.contains("certified_parity"));
    assert!(summary.contains("narrowed_parity"));
}

#[test]
fn matrix_csv_has_header_and_rows() {
    let csv = packet().render_matrix_csv();
    assert!(csv.starts_with(
        "row_id,surface,claimed_claim,certified_claim,status,narrowed_axes,component_truth_preserved\n"
    ));
    assert!(csv.contains("cert:desktop-list,desktop_review_list"));
    assert!(csv.contains("provider_local_provenance"));
}

// --- Checked artifacts --------------------------------------------------------

#[test]
fn checked_support_export_validates() {
    let packet = current_review_component_certification_export()
        .expect("checked review-component certification export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-review-request-check-queue-component-certification/provider_freshness_stale_auto_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-review-request-check-queue-component-certification/detail_pane_and_cli_narrowed.json"
        )),
    ] {
        let packet: ReviewComponentCertificationPacket =
            serde_json::from_str(raw).expect("fixture parses as certification packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

// --- Fixture builders ---------------------------------------------------------

fn fixture_provider_freshness_stale_auto_narrowed() -> ReviewComponentCertificationPacket {
    let mut packet = packet();
    packet.packet_id =
        "review-component-certification:fixture:provider-freshness-stale-auto-narrowed".to_owned();
    packet.certification_label =
        "Review-component surface certification: provider freshness stale, claim auto-narrowed"
            .to_owned();
    packet.apply_downgrade_automation(&[ReviewComponentCertObservation {
        surface: M5ReviewComponentCertifiedSurface::DesktopReviewList,
        provider_fresh: false,
        component_truth_preserved: true,
    }]);
    packet
}

fn fixture_detail_pane_and_cli_narrowed() -> ReviewComponentCertificationPacket {
    let mut packet = packet();
    packet.packet_id =
        "review-component-certification:fixture:detail-pane-and-cli-narrowed".to_owned();
    packet.certification_label =
        "Review-component surface certification: detail pane and CLI provider freshness narrowed"
            .to_owned();
    packet.apply_downgrade_automation(&[
        ReviewComponentCertObservation {
            surface: M5ReviewComponentCertifiedSurface::ReviewDetailPane,
            provider_fresh: false,
            component_truth_preserved: true,
        },
        ReviewComponentCertObservation {
            surface: M5ReviewComponentCertifiedSurface::CliHeadless,
            provider_fresh: false,
            component_truth_preserved: true,
        },
    ]);
    packet
}

/// Regenerates the checked-in support export, summary, release proof, and fixtures.
///
/// Gated behind `GEN_REVIEW_COMPONENT_CERTIFICATION_ARTIFACTS` so it never writes
/// during a normal test run.
#[test]
fn regenerate_review_component_certification_artifacts() {
    if std::env::var("GEN_REVIEW_COMPONENT_CERTIFICATION_ARTIFACTS").is_err() {
        return;
    }

    let manifest = env!("CARGO_MANIFEST_DIR");
    let root = format!("{manifest}/../..");
    let module = "certify_review_request_row_checks_summary_card_pending_review_tray_merge_readiness_panel_merge_queue_entry_stack_dependency_chip_and_approval_invalidation_banner_truth_on_every_claimed_m5_review_surface";

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
        format!("{root}/artifacts/release/m5-review-request-check-queue-certification-proof");
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
        format!("{root}/fixtures/ui/m5-review-request-check-queue-component-certification");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");
    for (name, fixture) in [
        (
            "provider_freshness_stale_auto_narrowed.json",
            fixture_provider_freshness_stale_auto_narrowed(),
        ),
        (
            "detail_pane_and_cli_narrowed.json",
            fixture_detail_pane_and_cli_narrowed(),
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
