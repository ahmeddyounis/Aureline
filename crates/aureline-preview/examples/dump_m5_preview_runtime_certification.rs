//! Conformance dump for the M5 source-first preview / browser-runtime release
//! certification packet.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_preview::preview_runtime_certification::*;
use aureline_preview::PreviewSurface;

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

/// A fully-proven, release-certified row.
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

    // Narrowed + blocked: a full-stack loop whose source map went stale, so the
    // source-first preview lane is stale, the claim narrows below beta, and promotion
    // blocks until the source map is refreshed.
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

    // Narrowed + blocked: a support/export projection whose provider-conformance proof
    // is missing entirely, so the claim is blocked outright.
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
        packet_id: "m5-preview-runtime-certification:stable:0001".to_owned(),
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

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = packet();

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );

    if which == "summary" {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
