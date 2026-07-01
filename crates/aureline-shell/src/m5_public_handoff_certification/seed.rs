//! Canonical seed builders for the M5 public-handoff certification.
//!
//! These builders are the single producer of the checked-in packet, dashboard, and
//! support-export artifacts plus the narrowed fixtures. The headless emitter and the
//! inline tests both call them so the in-code certification, the artifacts, and the
//! fixtures never drift. The certified rows are pulled straight from the frozen
//! public-handoff matrix's seeded packet, so the certification cannot certify an
//! object the matrix does not freeze.

use super::*;
use crate::freeze_the_m5_public_handoff_and_capture_boundary_matrix::{
    seeded_m5_public_handoff_matrix, M5HandoffObjectRow, M5_PUBLIC_HANDOFF_MATRIX_PACKET_ID,
};

/// Deterministic generated-at value carried by the seeded packet.
const GENERATED_AT: &str = "2026-06-30T00:00:00Z";

/// Deterministic last-proof-refresh value carried by the seeded rows.
const PROOF_REFRESH: &str = "2026-06-30T00:00:00Z";

/// Frozen, representative exact-build identity ref used by the seed.
///
/// A live runtime stamps [`aureline_build_info::exact_build_identity_ref`] here; the
/// seed uses a fixed value so the checked-in fixtures stay reproducible.
pub const SEED_BUILD_IDENTITY_REF: &str =
    "build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2";

/// Frozen, representative release-channel class used by the seed.
pub const SEED_RELEASE_CHANNEL_CLASS: &str = "stable";

/// The proof, boundary, and redaction posture seeded for one governed object.
struct ProofSpec {
    freshness: HandoffNoticeFreshnessState,
    boundary_honesty: BoundaryHonestyState,
    redaction_readiness: RedactionReadinessState,
    waiver: Option<HandoffCertificationWaiver>,
    narrowing_reason: Option<&'static str>,
}

impl ProofSpec {
    /// A fully proven posture: proven-current, honestly disclosed, proven redaction.
    fn proven() -> Self {
        Self {
            freshness: HandoffNoticeFreshnessState::ProvenCurrent,
            boundary_honesty: BoundaryHonestyState::HonestlyDisclosed,
            redaction_readiness: RedactionReadinessState::Proven,
            waiver: None,
            narrowing_reason: None,
        }
    }
}

/// Builds the embedded-boundary labeling-sync waiver carried by the seed.
fn embedded_boundary_label_sync_waiver() -> HandoffCertificationWaiver {
    HandoffCertificationWaiver {
        waiver_id: "waiver:embedded-boundary-label-sync:0001".to_owned(),
        object_kind: M5HandoffObjectKind::EmbeddedAuthBoundary,
        reason: "The embedded webview and the system-browser handoff both label the external \
                 origin and route trust class, but the exact chrome wording is being unified in \
                 the next cross-surface boundary-copy sync. The difference is disclosed, never \
                 hidden, and no surface impersonates native chrome."
            .to_owned(),
        owner_role: "Browser/auth boundary owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// Returns the seeded proof posture for one governed object.
fn proof_spec(kind: M5HandoffObjectKind) -> ProofSpec {
    match kind {
        M5HandoffObjectKind::DevicePermissionBoundary => ProofSpec {
            narrowing_reason: Some(
                "The device/mic permission boundary is qualified at Beta in the frozen \
                 public-handoff matrix; capture ships with a disclosed Beta posture and is \
                 narrowed below a Stable public claim.",
            ),
            ..ProofSpec::proven()
        },
        M5HandoffObjectKind::EmbeddedAuthBoundary => ProofSpec {
            boundary_honesty: BoundaryHonestyState::DisclosedGap,
            waiver: Some(embedded_boundary_label_sync_waiver()),
            narrowing_reason: Some(
                "The embedded webview / auth boundary is qualified at Beta; the embedded surface \
                 and the system-browser handoff disclose a known chrome-wording gap that is \
                 waivered pending the next cross-surface boundary-copy sync.",
            ),
            ..ProofSpec::proven()
        },
        _ => ProofSpec::proven(),
    }
}

/// Builds one certification row from a frozen matrix row and a proof posture.
fn row_from_matrix(matrix_row: &M5HandoffObjectRow, spec: ProofSpec) -> HandoffCertificationRow {
    let mut row = HandoffCertificationRow {
        object_kind: matrix_row.object_kind,
        matrix_qualification: matrix_row.qualification,
        owner_role: matrix_row.owner_role.clone(),
        certified_surface: certified_surface_label(matrix_row.object_kind).to_owned(),
        proof_packet_refs: matrix_row.required_proof_packet_refs.clone(),
        last_proof_refresh: PROOF_REFRESH.to_owned(),
        disclosure_freshness: spec.freshness,
        boundary_honesty: spec.boundary_honesty,
        redaction_readiness: spec.redaction_readiness,
        consumer_surfaces: matrix_row.consumer_surfaces.clone(),
        applicable_downgrade_triggers: matrix_row.downgrade_triggers.clone(),
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: HandoffCertStatus::Green,
        stale_proof_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.stale_proof_causes = row.recompute_causes();
    row
}

/// Builds the certification rows for the canonical seed, one per matrix object.
fn seeded_rows() -> Vec<HandoffCertificationRow> {
    seeded_m5_public_handoff_matrix()
        .object_rows
        .iter()
        .map(|matrix_row| row_from_matrix(matrix_row, proof_spec(matrix_row.object_kind)))
        .collect()
}

/// Builds the canonical M5 public-handoff certification packet.
///
/// This is the single producer of the checked-in packet, dashboard, and
/// support-export artifacts. Six rows certify green; the device/mic permission
/// boundary and the embedded webview/auth boundary auto-narrow to yellow from their
/// frozen Beta qualification (the embedded boundary also discloses a waivered
/// boundary-labeling gap), and no row is blocked — so the packet is clean and every
/// row is publishable.
pub fn seeded_public_handoff_certification_packet() -> PublicHandoffCertificationPacket {
    build_public_handoff_certification_packet(PublicHandoffCertificationInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_PUBLIC_HANDOFF_MATRIX_PACKET_ID.to_owned(),
        rows: seeded_rows(),
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds a variant where the embedded webview / auth boundary hides a native-chrome
/// impersonation, proving an undisclosed impersonation blocks promotion (red) rather
/// than passing on behavior alone.
pub fn seeded_public_handoff_certification_packet_embedded_impersonation_blocked(
) -> PublicHandoffCertificationPacket {
    let rows = seeded_m5_public_handoff_matrix()
        .object_rows
        .iter()
        .map(|matrix_row| {
            let mut spec = proof_spec(matrix_row.object_kind);
            if matrix_row.object_kind == M5HandoffObjectKind::EmbeddedAuthBoundary {
                spec.boundary_honesty = BoundaryHonestyState::UndisclosedImpersonation;
                spec.waiver = None;
            }
            row_from_matrix(matrix_row, spec)
        })
        .collect();
    build_public_handoff_certification_packet(PublicHandoffCertificationInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_PUBLIC_HANDOFF_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds a variant where the reproduction packet would share raw sensitive
/// material, proving an unsafe redaction posture blocks promotion (red) before the
/// packet can keep a public claim.
pub fn seeded_public_handoff_certification_packet_repro_redaction_unsafe(
) -> PublicHandoffCertificationPacket {
    let rows = seeded_m5_public_handoff_matrix()
        .object_rows
        .iter()
        .map(|matrix_row| {
            let mut spec = proof_spec(matrix_row.object_kind);
            if matrix_row.object_kind == M5HandoffObjectKind::ReproductionPacket {
                spec.redaction_readiness = RedactionReadinessState::UnsafeMaterial;
                spec.narrowing_reason = Some(
                    "The reproduction packet would share before its redaction preview is \
                     confirmed, so raw paths/hostnames/tokens would leave; the row blocks before \
                     keeping a public claim.",
                );
            }
            row_from_matrix(matrix_row, spec)
        })
        .collect();
    build_public_handoff_certification_packet(PublicHandoffCertificationInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_PUBLIC_HANDOFF_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds a variant where the release/service-health notice runs on stale proof with
/// no waiver, proving stale proof auto-narrows a Stable row to blocked (red) before
/// it can keep its notice claim.
pub fn seeded_public_handoff_certification_packet_service_health_stale(
) -> PublicHandoffCertificationPacket {
    let rows = seeded_m5_public_handoff_matrix()
        .object_rows
        .iter()
        .map(|matrix_row| {
            let mut spec = proof_spec(matrix_row.object_kind);
            if matrix_row.object_kind == M5HandoffObjectKind::ServiceHealthNotice {
                spec.freshness = HandoffNoticeFreshnessState::Stale;
                spec.narrowing_reason = Some(
                    "The service-health notice proof has gone stale past its freshness floor and \
                     is not waivered, so the row blocks before keeping a Stable notice claim.",
                );
            }
            row_from_matrix(matrix_row, spec)
        })
        .collect();
    build_public_handoff_certification_packet(PublicHandoffCertificationInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_PUBLIC_HANDOFF_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}
