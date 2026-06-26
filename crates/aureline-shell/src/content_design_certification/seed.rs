//! Canonical seed builders for the M5 content-design certification.
//!
//! These builders are the single producer of the checked-in packet, dashboard,
//! and support-export artifacts plus the narrowed fixtures. The headless emitter
//! and the inline tests both call them so the in-code certification, the
//! artifacts, and the fixtures never drift. The certified rows are pulled straight
//! from the frozen content-wording matrix's seeded packet, so the certification
//! cannot certify an object the matrix does not freeze.

use super::*;
use crate::freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix::{
    seeded_m5_content_wording_matrix, M5ContentObjectRow, M5_CONTENT_WORDING_MATRIX_PACKET_ID,
};

/// Deterministic generated-at value carried by the seeded packet.
const GENERATED_AT: &str = "2026-06-26T00:00:00Z";

/// Deterministic last-proof-refresh value carried by the seeded rows.
const PROOF_REFRESH: &str = "2026-06-26T00:00:00Z";

/// Frozen, representative exact-build identity ref used by the seed.
///
/// A live runtime stamps [`aureline_build_info::exact_build_identity_ref`] here;
/// the seed uses a fixed value so the checked-in fixtures stay reproducible.
pub const SEED_BUILD_IDENTITY_REF: &str =
    "build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2";

/// Frozen, representative release-channel class used by the seed.
pub const SEED_RELEASE_CHANNEL_CLASS: &str = "stable";

/// The proof, parity, and metadata posture seeded for one governed object.
struct ProofSpec {
    freshness: ContentFreshnessState,
    copy_parity: CopyParityState,
    metadata_state: ContentOpsMetadataState,
    waiver: Option<ContentCertificationWaiver>,
    narrowing_reason: Option<&'static str>,
}

impl ProofSpec {
    /// A fully proven posture: proven-current, in-parity, complete metadata.
    fn proven() -> Self {
        Self {
            freshness: ContentFreshnessState::ProvenCurrent,
            copy_parity: CopyParityState::InParity,
            metadata_state: ContentOpsMetadataState::Complete,
            waiver: None,
            narrowing_reason: None,
        }
    }
}

/// Builds the commercial-boundary copy-sync waiver carried by the seed.
fn boundary_copy_sync_waiver() -> ContentCertificationWaiver {
    ContentCertificationWaiver {
        waiver_id: "waiver:content-boundary-copy-sync:0001".to_owned(),
        object_kind: M5ContentObjectKind::CommercialBoundaryWording,
        reason: "The marketplace upgrade prompt and Help/About disclose the same hosted/managed \
                 boundary, but the exact phrasing is being unified in the next cross-surface copy \
                 sync. The difference is disclosed, never hidden."
            .to_owned(),
        owner_role: "Commercial boundary owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// Returns the seeded proof posture for one governed object.
fn proof_spec(kind: M5ContentObjectKind) -> ProofSpec {
    match kind {
        M5ContentObjectKind::AiCopyGuardrail => ProofSpec {
            narrowing_reason: Some(
                "AI copy guardrail is qualified at Beta in the frozen content matrix; AI wording \
                 ships with a disclosed Low confidence / Review required posture and is narrowed \
                 below a Stable wording claim.",
            ),
            ..ProofSpec::proven()
        },
        M5ContentObjectKind::CommercialBoundaryWording => ProofSpec {
            copy_parity: CopyParityState::DisclosedDrift,
            waiver: Some(boundary_copy_sync_waiver()),
            narrowing_reason: Some(
                "Commercial-boundary wording is qualified at Beta; the marketplace upgrade prompt \
                 and Help/About disclose a known hosted/managed boundary phrasing difference that \
                 is waivered pending the next cross-surface copy sync.",
            ),
            ..ProofSpec::proven()
        },
        _ => ProofSpec::proven(),
    }
}

/// Builds one certification row from a frozen matrix row and a proof posture.
fn row_from_matrix(matrix_row: &M5ContentObjectRow, spec: ProofSpec) -> ContentCertificationRow {
    let mut row = ContentCertificationRow {
        object_kind: matrix_row.object_kind,
        matrix_qualification: matrix_row.qualification,
        owner_role: matrix_row.owner_role.clone(),
        protected_concept: protected_concept_label(matrix_row.object_kind).to_owned(),
        proof_packet_refs: matrix_row.required_proof_packet_refs.clone(),
        last_proof_refresh: PROOF_REFRESH.to_owned(),
        proof_freshness: spec.freshness,
        copy_parity: spec.copy_parity,
        metadata_state: spec.metadata_state,
        consumer_surfaces: matrix_row.consumer_surfaces.clone(),
        applicable_downgrade_triggers: matrix_row.downgrade_triggers.clone(),
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: ContentRowStatus::Green,
        stale_proof_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.stale_proof_causes = row.recompute_causes();
    row
}

/// Builds the certification rows for the canonical seed, one per matrix object.
fn seeded_rows() -> Vec<ContentCertificationRow> {
    seeded_m5_content_wording_matrix()
        .object_rows
        .iter()
        .map(|matrix_row| row_from_matrix(matrix_row, proof_spec(matrix_row.object_kind)))
        .collect()
}

/// Builds the canonical M5 content-design certification packet.
///
/// This is the single producer of the checked-in packet, dashboard, and
/// support-export artifacts. Six rows certify green; the AI copy guardrail and
/// commercial-boundary rows auto-narrow to yellow from their frozen Beta
/// qualification (the boundary row also discloses a waivered copy drift), and no
/// row is blocked — so the packet is clean and every row is publishable.
pub fn seeded_content_design_certification_packet() -> ContentDesignCertificationPacket {
    build_content_design_certification_packet(ContentDesignCertificationInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_CONTENT_WORDING_MATRIX_PACKET_ID.to_owned(),
        rows: seeded_rows(),
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds a variant where the AI copy guardrail hides a wording overclaim,
/// proving an undisclosed drift blocks promotion (red) rather than passing on
/// behavior alone.
pub fn seeded_content_design_certification_packet_ai_overclaim_blocked(
) -> ContentDesignCertificationPacket {
    let rows = seeded_m5_content_wording_matrix()
        .object_rows
        .iter()
        .map(|matrix_row| {
            let mut spec = proof_spec(matrix_row.object_kind);
            if matrix_row.object_kind == M5ContentObjectKind::AiCopyGuardrail {
                spec.copy_parity = CopyParityState::UndisclosedDrift;
            }
            row_from_matrix(matrix_row, spec)
        })
        .collect();
    build_content_design_certification_packet(ContentDesignCertificationInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_CONTENT_WORDING_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds a variant where the content-ops metadata row runs on stale proof with
/// no waiver, proving stale proof auto-narrows a Stable row to blocked (red)
/// before it can keep its wording claim.
pub fn seeded_content_design_certification_packet_content_ops_stale(
) -> ContentDesignCertificationPacket {
    let rows = seeded_m5_content_wording_matrix()
        .object_rows
        .iter()
        .map(|matrix_row| {
            let mut spec = proof_spec(matrix_row.object_kind);
            if matrix_row.object_kind == M5ContentObjectKind::ContentOpsArtifact {
                spec.freshness = ContentFreshnessState::Stale;
                spec.narrowing_reason = Some(
                    "Content-ops metadata proof has gone stale past its freshness floor and is not \
                     waivered, so the row blocks before keeping a Stable wording claim.",
                );
            }
            row_from_matrix(matrix_row, spec)
        })
        .collect();
    build_content_design_certification_packet(ContentDesignCertificationInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_CONTENT_WORDING_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}
