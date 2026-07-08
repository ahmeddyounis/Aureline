use super::*;

const PACKET_ID: &str = "git-history-certification:stable:0001";

fn trust_review() -> GitHistoryCertificationTrustReview {
    canonical_trust_review()
}

fn consumer_projection() -> GitHistoryCertificationConsumerProjection {
    canonical_consumer_projection()
}

fn proof_freshness() -> GitHistoryCertificationProofFreshness {
    GitHistoryCertificationProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-07-08T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<GitHistoryCertificationDowngradeTrigger> {
    vec![
        GitHistoryCertificationDowngradeTrigger::ProofStale,
        GitHistoryCertificationDowngradeTrigger::ProviderReviewStateStale,
        GitHistoryCertificationDowngradeTrigger::RepoTopologyPartial,
        GitHistoryCertificationDowngradeTrigger::CheckpointRecoveryUnavailable,
        GitHistoryCertificationDowngradeTrigger::OfflineLocalOnly,
    ]
}

fn source_contract_refs() -> Vec<String> {
    canonical_source_contract_refs()
}

fn row_refs(components: &[M5GitHistoryComponent]) -> Vec<String> {
    let mut refs = vec![M5_GIT_HISTORY_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF.to_owned()];
    for component in components {
        let schema = certification_component_canonical_schema_ref(*component).to_owned();
        if !refs.contains(&schema) {
            refs.push(schema);
        }
    }
    refs
}

fn axis_note(axis: GitHistoryCertificationAxis, narrowed: bool) -> String {
    let base = match axis {
        GitHistoryCertificationAxis::Visual => {
            "Visual rendering carries the controlled component truth"
        }
        GitHistoryCertificationAxis::Keyboard => {
            "Keyboard reach and operation carry the controlled component truth"
        }
        GitHistoryCertificationAxis::ScreenReader => {
            "Screen-reader labelling carries the controlled component truth"
        }
        GitHistoryCertificationAxis::CliExport => {
            "CLI and export forms carry the controlled component truth"
        }
        GitHistoryCertificationAxis::DegradedState => {
            "Degraded repo topology, checkpoint, or provider-recovery state is disclosed"
        }
        GitHistoryCertificationAxis::LocalRecoveryProvenance => {
            "The local-recovery versus provider distinction stays explicit; certified never implies fresh"
        }
    };
    if narrowed {
        format!("{base} — narrowed here with an honest fallback disclosed")
    } else {
        base.to_owned()
    }
}

fn axis_outcomes(
    narrowed_axis: Option<GitHistoryCertificationAxis>,
) -> Vec<GitHistoryCertAxisOutcome> {
    GitHistoryCertificationAxis::ALL
        .iter()
        .map(|axis| {
            let narrowed = narrowed_axis == Some(*axis);
            GitHistoryCertAxisOutcome {
                axis: *axis,
                state: if narrowed {
                    GitHistoryAxisCertificationState::NarrowedCertified
                } else {
                    GitHistoryAxisCertificationState::Certified
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
    surface: M5GitHistoryCertifiedSurface,
    components: Vec<M5GitHistoryComponent>,
    claimed: GitHistoryClaimTier,
    certified: GitHistoryClaimTier,
    narrowed_axis: Option<GitHistoryCertificationAxis>,
    trigger: Option<GitHistoryCertificationDowngradeTrigger>,
) -> GitHistoryCertifiedSurfaceRow {
    let narrowed_axes = narrowed_axis.map(|axis| vec![axis]).unwrap_or_default();
    let component_truth_preserved = true;
    let status = derive_git_history_surface_claim_status(
        claimed,
        certified,
        component_truth_preserved,
        !narrowed_axes.is_empty(),
    );
    let refs = row_refs(&components);
    GitHistoryCertifiedSurfaceRow {
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
            "{}: focusable, Enter opens the component, Space toggles detail",
            surface.as_str()
        ),
        screen_reader_label: format!(
            "{} Git-history surface, certified {}",
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

/// The canonical eight-surface set: four green, four narrowed, all twelve components.
fn surface_rows() -> Vec<GitHistoryCertifiedSurfaceRow> {
    use GitHistoryCertificationAxis as Axis;
    use GitHistoryCertificationDowngradeTrigger as Trigger;
    use GitHistoryClaimTier as Tier;
    use M5GitHistoryCertifiedSurface as S;
    use M5GitHistoryComponent as C;
    vec![
        row(
            "cert:history-sidebar",
            S::HistorySidebar,
            vec![C::CommitGraphHeader, C::HistoryGraphRow],
            Tier::RecoverableInProduct,
            Tier::RecoverableInProduct,
            None,
            None,
        ),
        row(
            "cert:review-workspace",
            S::ReviewWorkspace,
            vec![C::BranchComparisonChip, C::WorktreeRow],
            Tier::RecoverableInProduct,
            Tier::RecoverableInProduct,
            None,
            None,
        ),
        row(
            "cert:help",
            S::HelpGitSurface,
            vec![C::StashEntry],
            Tier::RecoverableInProduct,
            Tier::RecoverableInProduct,
            None,
            None,
        ),
        row(
            "cert:cli",
            S::CliHeadless,
            vec![C::SequenceEditorHeader, C::RebaseTodoRow],
            Tier::RecoverableInProduct,
            Tier::RecoverableInProduct,
            None,
            None,
        ),
        row(
            "cert:risky-mutation-sheet",
            S::RiskyMutationSheet,
            vec![C::CherryPickRevertReviewSheet, C::ForcePushReviewDialog],
            Tier::RecoverableInProduct,
            Tier::LocallyRecoverable,
            Some(Axis::DegradedState),
            Some(Trigger::ProviderReviewStateStale),
        ),
        row(
            "cert:exported-recovery-packet",
            S::ExportedRecoveryPacket,
            vec![C::ReflogRecoveryBanner, C::ConflictCheckpointCard],
            Tier::RecoverableInProduct,
            Tier::LocalContinueOnly,
            Some(Axis::LocalRecoveryProvenance),
            Some(Trigger::OfflineLocalOnly),
        ),
        row(
            "cert:support-export",
            S::SupportExport,
            vec![C::PatchApplyReviewSheet],
            Tier::RecoverableInProduct,
            Tier::ReflogOnlyRecovery,
            Some(Axis::DegradedState),
            Some(Trigger::CheckpointRecoveryUnavailable),
        ),
        row(
            "cert:diagnostics",
            S::Diagnostics,
            vec![C::ConflictCheckpointCard, C::WorktreeRow],
            Tier::RecoverableInProduct,
            Tier::PartialHistoryOnly,
            Some(Axis::DegradedState),
            Some(Trigger::RepoTopologyPartial),
        ),
    ]
}

fn packet_with(rows: Vec<GitHistoryCertifiedSurfaceRow>) -> GitHistoryCertificationPacket {
    let summary = GitHistoryCertificationSummary::from_rows(&rows);
    GitHistoryCertificationPacket::new(GitHistoryCertificationPacketInput {
        packet_id: PACKET_ID.to_owned(),
        certification_label: "Git-history surface certification".to_owned(),
        surface_rows: rows,
        summary,
        downgrade_triggers: downgrade_triggers(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-08T00:00:00Z".to_owned(),
    })
}

fn packet() -> GitHistoryCertificationPacket {
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
    use GitHistoryClaimTier as Tier;
    assert_eq!(
        derive_git_history_surface_claim_status(
            Tier::RecoverableInProduct,
            Tier::RecoverableInProduct,
            true,
            false
        ),
        GitHistorySurfaceClaimStatus::CertifiedParity
    );
    assert_eq!(
        derive_git_history_surface_claim_status(
            Tier::RecoverableInProduct,
            Tier::LocallyRecoverable,
            true,
            false
        ),
        GitHistorySurfaceClaimStatus::NarrowedParity
    );
    assert_eq!(
        derive_git_history_surface_claim_status(
            Tier::RecoverableInProduct,
            Tier::RecoverableInProduct,
            true,
            true
        ),
        GitHistorySurfaceClaimStatus::NarrowedParity
    );
    assert_eq!(
        derive_git_history_surface_claim_status(
            Tier::RecoverableInProduct,
            Tier::RecoverableInProduct,
            false,
            false
        ),
        GitHistorySurfaceClaimStatus::ParityBlocked
    );
}

// --- AC2: certification never overstates recoverable-in-product truth ----------

#[test]
fn certified_claim_exceeds_claimed_fails() {
    let mut packet = packet();
    packet.surface_rows[0].claimed_claim = GitHistoryClaimTier::LocallyRecoverable;
    packet.surface_rows[0].certified_claim = GitHistoryClaimTier::RecoverableInProduct;
    assert!(packet
        .validate()
        .contains(&GitHistoryCertificationViolation::CertifiedClaimExceedsClaimed));
}

#[test]
fn certified_never_implies_provider_fresh() {
    // A fully-certified (green) surface and a narrowed surface both keep the
    // local-recovery-provenance axis explicit, proving the axis is orthogonal to the
    // overall certified status: certification never implies provider freshness.
    let rows = surface_rows();
    let green = rows
        .iter()
        .find(|r| r.surface == M5GitHistoryCertifiedSurface::HistorySidebar)
        .expect("history sidebar row present");
    assert!(green.status.is_green());
    let green_provenance = green
        .axis_outcomes
        .iter()
        .find(|o| o.axis == GitHistoryCertificationAxis::LocalRecoveryProvenance)
        .expect("green row scores the provenance axis");
    assert_eq!(
        green_provenance.state,
        GitHistoryAxisCertificationState::Certified
    );
    assert!(!green_provenance.note.trim().is_empty());

    let narrowed = rows
        .iter()
        .find(|r| r.surface == M5GitHistoryCertifiedSurface::ExportedRecoveryPacket)
        .expect("exported recovery packet row present");
    let narrowed_provenance = narrowed
        .axis_outcomes
        .iter()
        .find(|o| o.axis == GitHistoryCertificationAxis::LocalRecoveryProvenance)
        .expect("narrowed row scores the provenance axis");
    assert_eq!(
        narrowed_provenance.state,
        GitHistoryAxisCertificationState::NarrowedCertified
    );
}

#[test]
fn status_mismatch_fails() {
    let mut packet = packet();
    packet.surface_rows[0].status = GitHistorySurfaceClaimStatus::NarrowedParity;
    assert!(packet
        .validate()
        .contains(&GitHistoryCertificationViolation::StatusMismatch));
}

#[test]
fn narrowed_axes_inconsistent_fails() {
    let mut packet = packet();
    // Push an axis into narrowed_axes without marking its outcome narrowed.
    packet.surface_rows[0]
        .narrowed_axes
        .push(GitHistoryCertificationAxis::Keyboard);
    assert!(packet
        .validate()
        .contains(&GitHistoryCertificationViolation::NarrowedAxesInconsistent));
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
        .contains(&GitHistoryCertificationViolation::NarrowingWithoutTrigger));
}

// --- Delta: certification never drops component truth ---------------------------

#[test]
fn component_truth_dropped_fails() {
    let mut packet = packet();
    packet.surface_rows[0].component_truth_preserved = false;
    // Keep status consistent so only the truth-dropped rule fires cleanly.
    packet.surface_rows[0].status = GitHistorySurfaceClaimStatus::ParityBlocked;
    packet.summary = GitHistoryCertificationSummary::from_rows(&packet.surface_rows);
    assert!(packet
        .validate()
        .contains(&GitHistoryCertificationViolation::GitHistoryComponentTruthDropped));
}

// --- AC1: parity across keyboard / screen-reader / CLI / export -----------------

#[test]
fn keyboard_label_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].keyboard_label = String::new();
    assert!(packet
        .validate()
        .contains(&GitHistoryCertificationViolation::KeyboardLabelMissing));
}

#[test]
fn screen_reader_label_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].screen_reader_label = String::new();
    assert!(packet
        .validate()
        .contains(&GitHistoryCertificationViolation::ScreenReaderLabelMissing));
}

#[test]
fn cli_enum_token_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].cli_enum_token = String::new();
    assert!(packet
        .validate()
        .contains(&GitHistoryCertificationViolation::CliEnumTokenMissing));
}

#[test]
fn export_enum_token_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].export_enum_token = String::new();
    assert!(packet
        .validate()
        .contains(&GitHistoryCertificationViolation::ExportEnumTokenMissing));
}

#[test]
fn explanation_field_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].explanation_field = String::new();
    assert!(packet
        .validate()
        .contains(&GitHistoryCertificationViolation::ExplanationFieldMissing));
}

#[test]
fn axis_coverage_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].axis_outcomes.truncate(3);
    assert!(packet
        .validate()
        .contains(&GitHistoryCertificationViolation::AxisCoverageMissing));
}

#[test]
fn axis_note_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].axis_outcomes[0].note = String::new();
    assert!(packet
        .validate()
        .contains(&GitHistoryCertificationViolation::AxisNoteMissing));
}

#[test]
fn components_missing_on_row_fails() {
    let mut packet = packet();
    packet.surface_rows[0].components_present.clear();
    assert!(packet
        .validate()
        .contains(&GitHistoryCertificationViolation::ComponentsMissingOnRow));
}

#[test]
fn canonical_contract_reference_missing_fails() {
    let mut packet = packet();
    packet.surface_rows[0].source_contract_refs =
        vec![M5_GIT_HISTORY_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF.to_owned()];
    assert!(packet
        .validate()
        .contains(&GitHistoryCertificationViolation::CanonicalContractReferenceMissing));
}

// --- Coverage ------------------------------------------------------------------

#[test]
fn surface_coverage_missing_fails() {
    let mut rows = surface_rows();
    rows.retain(|r| r.surface != M5GitHistoryCertifiedSurface::Diagnostics);
    let packet = packet_with(rows);
    assert!(packet
        .validate()
        .contains(&GitHistoryCertificationViolation::SurfaceCoverageMissing));
}

#[test]
fn component_coverage_missing_fails() {
    let mut rows = surface_rows();
    // Remove the force-push review dialog from every surface that presents it.
    for row in &mut rows {
        row.components_present
            .retain(|c| *c != M5GitHistoryComponent::ForcePushReviewDialog);
    }
    let packet = packet_with(rows);
    assert!(packet
        .validate()
        .contains(&GitHistoryCertificationViolation::ComponentCoverageMissing));
}

// --- Structural ----------------------------------------------------------------

#[test]
fn summary_mismatch_fails() {
    let mut packet = packet();
    packet.summary.certified_count += 1;
    assert!(packet
        .validate()
        .contains(&GitHistoryCertificationViolation::SummaryMismatch));
}

#[test]
fn row_incomplete_fails() {
    let mut packet = packet();
    packet.surface_rows[0].row_id = String::new();
    assert!(packet
        .validate()
        .contains(&GitHistoryCertificationViolation::RowIncomplete));
}

#[test]
fn missing_rows_fails() {
    let mut packet = packet();
    packet.surface_rows.clear();
    packet.summary = GitHistoryCertificationSummary::from_rows(&packet.surface_rows);
    assert!(packet
        .validate()
        .contains(&GitHistoryCertificationViolation::SurfaceRowsMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&GitHistoryCertificationViolation::MissingSourceContracts));
}

#[test]
fn downgrade_triggers_missing_fails() {
    let mut packet = packet();
    packet.downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&GitHistoryCertificationViolation::DowngradeTriggersMissing));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.certified_never_implies_fresh = false;
    assert!(packet
        .validate()
        .contains(&GitHistoryCertificationViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .narrowed_surfaces_visibly_labelled = false;
    assert!(packet
        .validate()
        .contains(&GitHistoryCertificationViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&GitHistoryCertificationViolation::ProofFreshnessIncomplete));
}

// --- Auto-narrowing (AC2 release automation) -----------------------------------

#[test]
fn apply_downgrade_narrows_stale_provider_surface() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[GitHistoryCertObservation {
        surface: M5GitHistoryCertifiedSurface::HistorySidebar,
        provider_recovery_fresh: false,
        component_truth_preserved: true,
    }]);
    let row = packet
        .surface_rows
        .iter()
        .find(|r| r.surface == M5GitHistoryCertifiedSurface::HistorySidebar)
        .expect("history sidebar row present");
    assert_eq!(row.status, GitHistorySurfaceClaimStatus::NarrowedParity);
    assert_eq!(row.certified_claim, GitHistoryClaimTier::LocallyRecoverable);
    assert!(row
        .narrowed_axes
        .contains(&GitHistoryCertificationAxis::LocalRecoveryProvenance));
    assert_eq!(
        row.downgrade_trigger,
        Some(GitHistoryCertificationDowngradeTrigger::ProviderReviewStateStale)
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.summary.certified_count, 3);
    assert_eq!(packet.summary.narrowed_count, 5);
}

#[test]
fn apply_downgrade_blocks_flattened_surface() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[GitHistoryCertObservation {
        surface: M5GitHistoryCertifiedSurface::SupportExport,
        provider_recovery_fresh: true,
        component_truth_preserved: false,
    }]);
    let row = packet
        .surface_rows
        .iter()
        .find(|r| r.surface == M5GitHistoryCertifiedSurface::SupportExport)
        .expect("support export row present");
    assert_eq!(row.status, GitHistorySurfaceClaimStatus::ParityBlocked);
    assert!(packet
        .validate()
        .contains(&GitHistoryCertificationViolation::GitHistoryComponentTruthDropped));
    assert_eq!(packet.summary.blocked_count, 1);
    assert!(!packet.summary.all_rows_preserve_component_truth);
}

// --- Rendering -----------------------------------------------------------------

#[test]
fn markdown_summary_lists_surfaces() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Certified surfaces"));
    assert!(summary.contains("history_sidebar"));
    assert!(summary.contains("exported_recovery_packet"));
    assert!(summary.contains("certified_parity"));
    assert!(summary.contains("narrowed_parity"));
}

#[test]
fn matrix_csv_has_header_and_rows() {
    let csv = packet().render_matrix_csv();
    assert!(csv.starts_with(
        "row_id,surface,claimed_claim,certified_claim,status,narrowed_axes,component_truth_preserved\n"
    ));
    assert!(csv.contains("cert:history-sidebar,history_sidebar"));
    assert!(csv.contains("local_recovery_provenance"));
}

// --- Checked artifacts ---------------------------------------------------------

#[test]
fn checked_support_export_validates() {
    let packet = current_git_history_certification_export()
        .expect("checked git-history certification export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-git-history-surface-certification/provider_review_state_stale_auto_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-git-history-surface-certification/risky_mutation_and_exported_recovery_narrowed.json"
        )),
    ] {
        let packet: GitHistoryCertificationPacket =
            serde_json::from_str(raw).expect("fixture parses as certification packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

// --- Fixture builders ----------------------------------------------------------

fn fixture_provider_review_state_stale_auto_narrowed() -> GitHistoryCertificationPacket {
    let mut packet = packet();
    packet.packet_id =
        "git-history-certification:fixture:provider-review-state-stale-auto-narrowed".to_owned();
    packet.certification_label =
        "Git-history surface certification: provider-linked recovery stale, claim auto-narrowed"
            .to_owned();
    packet.apply_downgrade_automation(&[GitHistoryCertObservation {
        surface: M5GitHistoryCertifiedSurface::HistorySidebar,
        provider_recovery_fresh: false,
        component_truth_preserved: true,
    }]);
    packet
}

fn fixture_risky_mutation_and_exported_recovery_narrowed() -> GitHistoryCertificationPacket {
    let mut packet = packet();
    packet.packet_id =
        "git-history-certification:fixture:risky-mutation-and-exported-recovery-narrowed"
            .to_owned();
    packet.certification_label =
        "Git-history surface certification: review workspace and CLI provider recovery narrowed"
            .to_owned();
    packet.apply_downgrade_automation(&[
        GitHistoryCertObservation {
            surface: M5GitHistoryCertifiedSurface::ReviewWorkspace,
            provider_recovery_fresh: false,
            component_truth_preserved: true,
        },
        GitHistoryCertObservation {
            surface: M5GitHistoryCertifiedSurface::CliHeadless,
            provider_recovery_fresh: false,
            component_truth_preserved: true,
        },
    ]);
    packet
}

/// Regenerates the checked-in release proof (support export, matrix, report) and
/// fixtures.
///
/// Gated behind `GEN_GIT_HISTORY_CERTIFICATION_ARTIFACTS` so it never writes during a
/// normal test run.
#[test]
fn regenerate_git_history_certification_artifacts() {
    if std::env::var("GEN_GIT_HISTORY_CERTIFICATION_ARTIFACTS").is_err() {
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

    let release_dir =
        format!("{root}/artifacts/release/m5-git-history-surface-certification-proof");
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

    let fixture_dir = format!("{root}/fixtures/ui/m5-git-history-surface-certification");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");
    for (name, fixture) in [
        (
            "provider_review_state_stale_auto_narrowed.json",
            fixture_provider_review_state_stale_auto_narrowed(),
        ),
        (
            "risky_mutation_and_exported_recovery_narrowed.json",
            fixture_risky_mutation_and_exported_recovery_narrowed(),
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
