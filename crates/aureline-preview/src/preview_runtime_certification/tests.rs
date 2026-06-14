use super::*;

const PACKET_ID: &str = "m5-preview-runtime-certification:stable:0001";

fn ev(refs: &[&str]) -> Vec<String> {
    refs.iter().map(|r| (*r).to_owned()).collect()
}

fn proof(lane: CertificationLane, status: LaneProofStatus, id: &str) -> LaneProof {
    let last_refresh = match status {
        LaneProofStatus::Current | LaneProofStatus::Stale => {
            Some(if status == LaneProofStatus::Stale {
                "2026-05-01T00:00:00Z".to_owned()
            } else {
                "2026-06-07T00:00:00Z".to_owned()
            })
        }
        LaneProofStatus::Missing | LaneProofStatus::NotApplicable => None,
    };
    LaneProof {
        lane,
        status,
        source_lane_ref: lane.canonical_schema_ref().to_owned(),
        evidence_ref: format!("evidence:lane:{}:{id}", lane.as_str()),
        last_refresh,
    }
}

fn lane_refs(lanes: &[CertificationLane]) -> Vec<String> {
    lanes
        .iter()
        .map(|lane| lane.canonical_schema_ref().to_owned())
        .collect()
}

fn certified_row(
    row_id: &str,
    surface: PreviewSurface,
    label: &str,
    summary: &str,
    required: &[CertificationLane],
    claimed: CertificationClass,
    claims_write_capable: bool,
) -> CertificationRow {
    let lane_proofs = required
        .iter()
        .map(|lane| proof(*lane, LaneProofStatus::Current, row_id))
        .collect();
    CertificationRow {
        row_id: row_id.to_owned(),
        surface,
        claimed_surface_label: label.to_owned(),
        required_lanes: required.to_vec(),
        lane_proofs,
        claimed_certification: claimed,
        effective_certification: claimed,
        promotion_blocked: false,
        claims_write_capable,
        narrow_trigger: None,
        degraded_label: None,
        label_summary: summary.to_owned(),
        observed_at: "2026-06-07T00:00:00Z".to_owned(),
        evidence_refs: ev(&[&format!("evidence:row:{row_id}")]),
        source_lane_refs: lane_refs(required),
    }
}

fn rows() -> Vec<CertificationRow> {
    let mut rows = vec![
        certified_row(
            "cert-row:source-first-framework:0001",
            PreviewSurface::SourceFirstFrameworkPreview,
            "Source-first framework preview rendered from the canonical source",
            "Every release-required lane is currently proven; the source-first framework preview is release-certified",
            &[
                CertificationLane::SourceFirstPreview,
                CertificationLane::InspectToSourceFidelity,
                CertificationLane::RoundTripHonesty,
                CertificationLane::DriftRecovery,
            ],
            CertificationClass::Certified,
            true,
        ),
        certified_row(
            "cert-row:visual-surface-mapping:0001",
            PreviewSurface::VisualSurfaceMapping,
            "Component/DOM/widget mapping inspector over a source-bound design render",
            "Source-first preview, inspect-to-source, and round-trip proof are current; the mapping surface is certified at beta depth",
            &[
                CertificationLane::SourceFirstPreview,
                CertificationLane::InspectToSourceFidelity,
                CertificationLane::RoundTripHonesty,
            ],
            CertificationClass::Beta,
            true,
        ),
        certified_row(
            "cert-row:browser-runtime-inspection:0001",
            PreviewSurface::BrowserRuntimeInspection,
            "Browser-runtime DOM/CSS/network/storage inspection over an attached tab",
            "Browser-runtime inspection, inspect-to-source, provider conformance, and drift drills are current; this inspect-only row is certified at beta depth",
            &[
                CertificationLane::BrowserRuntimeInspection,
                CertificationLane::InspectToSourceFidelity,
                CertificationLane::ProviderConformance,
                CertificationLane::DriftRecovery,
            ],
            CertificationClass::Beta,
            false,
        ),
        certified_row(
            "cert-row:device-or-simulator:0001",
            PreviewSurface::DeviceOrSimulatorPreview,
            "Simulator preview tethered over a workspace-bound transport",
            "Source-first preview, round-trip, drift drills, and provider conformance are current; the device preview is certified at beta depth",
            &[
                CertificationLane::SourceFirstPreview,
                CertificationLane::RoundTripHonesty,
                CertificationLane::DriftRecovery,
                CertificationLane::ProviderConformance,
            ],
            CertificationClass::Beta,
            true,
        ),
        certified_row(
            "cert-row:embedded-webview:0001",
            PreviewSurface::EmbeddedWebviewPreview,
            "Embedded webview preview hosted in the shell with inspect-only depth",
            "Source-first preview, browser-runtime inspection, and inspect-to-source proof are current; the embedded inspect-only row is certified at preview depth",
            &[
                CertificationLane::SourceFirstPreview,
                CertificationLane::BrowserRuntimeInspection,
                CertificationLane::InspectToSourceFidelity,
            ],
            CertificationClass::Preview,
            false,
        ),
        certified_row(
            "cert-row:visual-edit-transform:0001",
            PreviewSurface::VisualEditTransform,
            "Visual-edit transform that previews the real source diff before commit",
            "Source-first preview, round-trip honesty, and inspect-to-source proof are current; the write-capable visual-edit row is release-certified",
            &[
                CertificationLane::SourceFirstPreview,
                CertificationLane::RoundTripHonesty,
                CertificationLane::InspectToSourceFidelity,
            ],
            CertificationClass::Certified,
            true,
        ),
    ];

    {
        let required = [
            CertificationLane::SourceFirstPreview,
            CertificationLane::BrowserRuntimeInspection,
            CertificationLane::RoundTripHonesty,
            CertificationLane::DriftRecovery,
        ];
        rows.push(CertificationRow {
            row_id: "cert-row:full-stack-loop:0001".to_owned(),
            surface: PreviewSurface::FullStackPreviewLoop,
            claimed_surface_label: "Full-stack preview loop spanning client and server".to_owned(),
            required_lanes: required.to_vec(),
            lane_proofs: vec![
                proof(
                    CertificationLane::SourceFirstPreview,
                    LaneProofStatus::Stale,
                    "full-stack-loop:0001",
                ),
                proof(
                    CertificationLane::BrowserRuntimeInspection,
                    LaneProofStatus::Current,
                    "full-stack-loop:0001",
                ),
                proof(
                    CertificationLane::RoundTripHonesty,
                    LaneProofStatus::Current,
                    "full-stack-loop:0001",
                ),
                proof(
                    CertificationLane::DriftRecovery,
                    LaneProofStatus::Current,
                    "full-stack-loop:0001",
                ),
            ],
            claimed_certification: CertificationClass::Beta,
            effective_certification: CertificationClass::Held,
            promotion_blocked: true,
            claims_write_capable: false,
            narrow_trigger: Some(CertificationDowngradeTrigger::StaleSourceMap),
            degraded_label: Some(
                "The source map backing this full-stack loop went stale; the row is held below beta and promotion is blocked until the source map is refreshed"
                    .to_owned(),
            ),
            label_summary: "Full-stack preview loop with a stale source map; source-first preview proof is stale, so the claim is narrowed and promotion is blocked".to_owned(),
            observed_at: "2026-06-07T00:00:00Z".to_owned(),
            evidence_refs: ev(&["evidence:row:cert-row:full-stack-loop:0001"]),
            source_lane_refs: lane_refs(&required),
        });
    }

    {
        let required = [
            CertificationLane::SourceFirstPreview,
            CertificationLane::ProviderConformance,
        ];
        rows.push(CertificationRow {
            row_id: "cert-row:support-export:0001".to_owned(),
            surface: PreviewSurface::SupportExportProjection,
            claimed_surface_label: "Support/export projection of a claimed preview row".to_owned(),
            required_lanes: required.to_vec(),
            lane_proofs: vec![
                proof(
                    CertificationLane::SourceFirstPreview,
                    LaneProofStatus::Current,
                    "support-export:0001",
                ),
                proof(
                    CertificationLane::ProviderConformance,
                    LaneProofStatus::Missing,
                    "support-export:0001",
                ),
            ],
            claimed_certification: CertificationClass::Beta,
            effective_certification: CertificationClass::Blocked,
            promotion_blocked: true,
            claims_write_capable: false,
            narrow_trigger: Some(CertificationDowngradeTrigger::MissingLaneProof),
            degraded_label: Some(
                "No provider-conformance proof is on hand for this projected row; the claim is blocked until a conformance packet is published"
                    .to_owned(),
            ),
            label_summary: "Support/export projection missing provider-conformance proof; the claim is blocked from promotion".to_owned(),
            observed_at: "2026-06-07T00:00:00Z".to_owned(),
            evidence_refs: ev(&["evidence:row:cert-row:support-export:0001"]),
            source_lane_refs: lane_refs(&required),
        });
    }

    rows
}

fn guardrails() -> CertificationGuardrails {
    CertificationGuardrails {
        source_canonical_no_second_writable_model: true,
        runtime_state_never_hides_source_mapping_uncertainty: true,
        inspect_only_never_auto_upgraded_to_write: true,
        embedded_boundaries_not_blurred_into_product: true,
        claimed_rows_auto_narrow_without_current_proof: true,
        regressions_block_promotion_or_narrow: true,
        single_certification_result_no_manual_clone: true,
    }
}

fn consumer_projection() -> CertificationConsumerProjection {
    CertificationConsumerProjection {
        product_ingests_certification: true,
        docs_help_ingests_certification: true,
        diagnostics_ingests_certification: true,
        provider_conformance_ingests_certification: true,
        release_control_ingests_certification: true,
        narrowed_or_blocked_rows_labeled_below_current: true,
    }
}

fn evidence_freshness() -> CertificationEvidenceFreshness {
    CertificationEvidenceFreshness {
        evidence_freshness_slo_hours: 168,
        last_evidence_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        PREVIEW_RUNTIME_CERTIFICATION_SCHEMA_REF.to_owned(),
        PREVIEW_RUNTIME_CERTIFICATION_DOC_REF.to_owned(),
        PREVIEW_RUNTIME_CERTIFICATION_ARTIFACT_REF.to_owned(),
    ];
    for lane in CertificationLane::ALL {
        refs.push(lane.canonical_schema_ref().to_owned());
    }
    refs
}

fn packet() -> PreviewRuntimeCertificationPacket {
    PreviewRuntimeCertificationPacket::new(PreviewRuntimeCertificationPacketInput {
        packet_id: PACKET_ID.to_owned(),
        certification_label: "M5 Source-First Preview / Browser-Runtime Release Certification"
            .to_owned(),
        rows: rows(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        evidence_freshness: evidence_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

fn row_mut<'a>(
    packet: &'a mut PreviewRuntimeCertificationPacket,
    row_id: &str,
) -> &'a mut CertificationRow {
    packet
        .rows
        .iter_mut()
        .find(|r| r.row_id == row_id)
        .unwrap_or_else(|| panic!("row {row_id}"))
}

#[test]
fn packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn every_surface_is_present() {
    let surfaces = packet().represented_surfaces();
    for surface in PreviewSurface::ALL {
        assert!(
            surfaces.contains(&surface),
            "missing surface: {}",
            surface.as_str()
        );
    }
}

#[test]
fn every_lane_is_required_somewhere() {
    let lanes = packet().represented_lanes();
    for lane in CertificationLane::ALL {
        assert!(
            lanes.contains(&lane),
            "missing required lane: {}",
            lane.as_str()
        );
    }
}

#[test]
fn rollup_counts_are_correct() {
    let packet = packet();
    assert_eq!(packet.certified_row_count(), 2);
    assert_eq!(packet.narrowed_row_count(), 2);
    assert_eq!(packet.blocked_row_count(), 2);
    assert_eq!(packet.claimed_row_count(), 8);
}

#[test]
fn lane_proofs_bind_canonical_lane_schema() {
    for row in packet().rows {
        for proof in &row.lane_proofs {
            assert!(
                proof.binds_canonical_lane(),
                "proof for {} does not bind canonical schema",
                proof.lane.as_str()
            );
        }
    }
}

#[test]
fn missing_surface_fails() {
    let mut packet = packet();
    packet
        .rows
        .retain(|r| r.surface != PreviewSurface::VisualEditTransform);
    assert!(packet
        .validate()
        .contains(&PreviewRuntimeCertificationViolation::RequiredSurfaceMissing));
}

#[test]
fn missing_required_lane_fails() {
    let mut packet = packet();
    // DriftRecovery only appears on the framework/runtime/device/full-stack rows; drop
    // every row that requires it.
    packet
        .rows
        .retain(|r| !r.required_lanes.contains(&CertificationLane::DriftRecovery));
    assert!(packet
        .validate()
        .contains(&PreviewRuntimeCertificationViolation::RequiredLaneMissing));
}

#[test]
fn stale_required_lane_forces_narrowing() {
    let mut packet = packet();
    // Take the clean certified framework row and make one required lane go stale
    // without narrowing the claim: the gate must fire.
    let row = row_mut(&mut packet, "cert-row:source-first-framework:0001");
    row.lane_proofs[0].status = LaneProofStatus::Stale;
    row.lane_proofs[0].last_refresh = Some("2026-05-01T00:00:00Z".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&PreviewRuntimeCertificationViolation::RowNotNarrowedOnBlockingGap));
    assert!(violations.contains(&PreviewRuntimeCertificationViolation::PromotionGatingInconsistent));
}

#[test]
fn blocked_row_that_is_not_narrowed_fails() {
    let mut packet = packet();
    // Pretend the missing-proof support-export row is still fully claimed.
    let row = row_mut(&mut packet, "cert-row:support-export:0001");
    row.effective_certification = CertificationClass::Beta;
    row.promotion_blocked = false;
    row.narrow_trigger = None;
    row.degraded_label = None;
    let violations = packet.validate();
    assert!(violations.contains(&PreviewRuntimeCertificationViolation::RowNotNarrowedOnBlockingGap));
}

#[test]
fn clean_row_marked_blocked_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "cert-row:visual-edit-transform:0001").promotion_blocked = true;
    assert!(packet
        .validate()
        .contains(&PreviewRuntimeCertificationViolation::PromotionGatingInconsistent));
}

#[test]
fn narrowed_row_without_trigger_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "cert-row:full-stack-loop:0001").narrow_trigger = None;
    let violations = packet.validate();
    assert!(violations
        .contains(&PreviewRuntimeCertificationViolation::NarrowedRowMissingLabelOrTrigger));
}

#[test]
fn generic_degraded_label_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "cert-row:full-stack-loop:0001").degraded_label =
        Some("not certified".to_owned());
    let violations = packet.validate();
    assert!(violations
        .contains(&PreviewRuntimeCertificationViolation::NarrowedRowMissingLabelOrTrigger));
}

#[test]
fn row_missing_proof_for_required_lane_fails() {
    let mut packet = packet();
    // Drop a proof entry for a still-required lane.
    let row = row_mut(&mut packet, "cert-row:visual-surface-mapping:0001");
    row.lane_proofs
        .retain(|p| p.lane != CertificationLane::RoundTripHonesty);
    let violations = packet.validate();
    assert!(violations.contains(&PreviewRuntimeCertificationViolation::RowCoverageIncomplete));
}

#[test]
fn lane_proof_with_wrong_source_ref_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "cert-row:source-first-framework:0001").lane_proofs[0].source_lane_ref =
        "schemas/preview/wrong.schema.json".to_owned();
    assert!(packet
        .validate()
        .contains(&PreviewRuntimeCertificationViolation::LaneProofIncomplete));
}

#[test]
fn current_lane_proof_without_refresh_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "cert-row:source-first-framework:0001").lane_proofs[0].last_refresh = None;
    assert!(packet
        .validate()
        .contains(&PreviewRuntimeCertificationViolation::LaneProofIncomplete));
}

#[test]
fn write_capable_claim_without_round_trip_lane_fails() {
    let mut packet = packet();
    // Make the visual-edit row claim write capability but only require source-first.
    let row = row_mut(&mut packet, "cert-row:visual-edit-transform:0001");
    row.required_lanes = vec![CertificationLane::SourceFirstPreview];
    row.lane_proofs
        .retain(|p| p.lane == CertificationLane::SourceFirstPreview);
    assert!(packet
        .validate()
        .contains(&PreviewRuntimeCertificationViolation::WriteCapabilityUnbacked));
}

#[test]
fn write_capable_claim_on_narrowed_row_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "cert-row:full-stack-loop:0001").claims_write_capable = true;
    assert!(packet
        .validate()
        .contains(&PreviewRuntimeCertificationViolation::WriteCapabilityUnbacked));
}

#[test]
fn missing_narrowed_case_fails() {
    let mut packet = packet();
    // Remove both blocked rows so no gate demonstration remains.
    packet
        .rows
        .retain(|r| !r.promotion_blocked && !r.needs_narrowing());
    assert!(packet
        .validate()
        .contains(&PreviewRuntimeCertificationViolation::NarrowedRowCaseMissing));
}

#[test]
fn row_without_evidence_fails() {
    let mut packet = packet();
    packet.rows[0].evidence_refs.clear();
    assert!(packet
        .validate()
        .contains(&PreviewRuntimeCertificationViolation::RowEvidenceMissing));
}

#[test]
fn missing_base_source_contract_fails() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != PREVIEW_RUNTIME_CERTIFICATION_DOC_REF);
    assert!(packet
        .validate()
        .contains(&PreviewRuntimeCertificationViolation::MissingSourceContracts));
}

#[test]
fn missing_lane_source_contract_fails() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != CertificationLane::DriftRecovery.canonical_schema_ref());
    assert!(packet
        .validate()
        .contains(&PreviewRuntimeCertificationViolation::MissingSourceContracts));
}

#[test]
fn incomplete_guardrails_fail() {
    let mut packet = packet();
    packet.guardrails.regressions_block_promotion_or_narrow = false;
    assert!(packet
        .validate()
        .contains(&PreviewRuntimeCertificationViolation::GuardrailsIncomplete));
}

#[test]
fn incomplete_consumer_projection_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .narrowed_or_blocked_rows_labeled_below_current = false;
    assert!(packet
        .validate()
        .contains(&PreviewRuntimeCertificationViolation::ConsumerProjectionIncomplete));
}

#[test]
fn zero_freshness_slo_fails() {
    let mut packet = packet();
    packet.evidence_freshness.evidence_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&PreviewRuntimeCertificationViolation::EvidenceFreshnessIncomplete));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "wrong".to_owned();
    assert!(packet
        .validate()
        .contains(&PreviewRuntimeCertificationViolation::WrongRecordKind));
}

#[test]
fn chip_tokens_name_governed_chips() {
    let row = &packet().rows[0];
    let chips = row.chip_tokens();
    assert!(chips.contains("surface=source_first_framework_preview"));
    assert!(chips.contains("claim=certified"));
    assert!(chips.contains("effective=certified"));
    assert!(chips.contains("blocked=false"));
}

#[test]
fn markdown_summary_names_rows() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("M5 Source-First Preview / Browser-Runtime Release Certification"));
    assert!(summary.contains("cert-row:full-stack-loop:0001"));
    assert!(summary.contains("Degraded:"));
    assert!(summary.contains("lanes:"));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: PreviewRuntimeCertificationPacket =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn checked_support_export_matches_builder() {
    let checked = current_m5_preview_runtime_certification_export()
        .expect("checked preview runtime certification export validates");
    assert_eq!(checked, packet());
}
