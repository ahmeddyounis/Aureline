//! Canonical seed builders for the M5 update / support-lifecycle certification.
//!
//! These builders are the single producer of the checked-in certification packet, the published
//! inventory, the rendered certification document, the machine-readable grid CSV, the release-grade
//! parity proof (and its Markdown report), and the per-state drill fixtures. The headless emitter
//! and the inline tests both call them so the in-code packet, the artifacts, and the fixtures never
//! drift.
//!
//! The certification is a pure projection of the [governance
//! matrix](crate::m5_update_lifecycle): every variant is built by qualifying the same claimed
//! channel × profile grid against one of the three seeded governance packets. The canonical packet
//! qualifies against the all-current matrix, so every claim stands at its claimed qualification; the
//! drills qualify against the matrix's stale-proof and missing-proof variants and let the projection
//! recompute each claim's outcome, gate, and effective qualification.

use super::*;

use crate::m5_update_lifecycle::{
    seeded_m5_update_lifecycle, seeded_m5_update_lifecycle_missing_proof_blocked,
    seeded_m5_update_lifecycle_stale_proof_narrowed,
};

/// Stable packet id for the canonical certification packet.
pub const M5_UPDATE_LIFECYCLE_CERTIFICATION_PACKET_ID: &str =
    "m5-update-lifecycle-certification:stable:0001";

const REDACTION_CLASS: &str = "metadata_safe_default";

/// The claimed M5 channels, their claimed qualification, and the deployment profiles each is claimed
/// on. Every channel is claimed on both deployment profiles, so the grid is the full claimed
/// channel × profile product.
const CLAIMED_CHANNELS: [(ChannelScope, QualificationClass); 5] = [
    (ChannelScope::Stable, QualificationClass::Stable),
    (ChannelScope::Beta, QualificationClass::Beta),
    (ChannelScope::Preview, QualificationClass::Preview),
    (ChannelScope::Nightly, QualificationClass::Experimental),
    (ChannelScope::Lts, QualificationClass::Stable),
];

/// The claimed channel × profile grid: every channel on every deployment profile.
fn claimed_grid() -> Vec<(ChannelScope, DeploymentProfile, QualificationClass)> {
    let mut out = Vec::new();
    for (channel, claimed) in CLAIMED_CHANNELS {
        for profile in DeploymentProfile::ALL {
            out.push((channel, profile, claimed));
        }
    }
    out
}

/// Assembles a certification packet by projecting a governance matrix onto the claimed grid.
fn assemble(
    packet_id: &str,
    report_label: &str,
    governance: &M5UpdateLifecycleGovernance,
) -> M5UpdateLifecycleCertification {
    M5UpdateLifecycleCertification::from_governance(
        governance,
        M5UpdateLifecycleCertificationInput {
            packet_id: packet_id.to_owned(),
            report_label: report_label.to_owned(),
            claims: claimed_grid(),
            redaction_class_token: REDACTION_CLASS.to_owned(),
            minted_at: governance.minted_at.clone(),
        },
    )
}

/// The canonical certification: every claimed channel / profile qualifies against the all-current
/// governance matrix, so every applicable dimension is certified and every claim stands at its
/// claimed qualification.
pub fn seeded_m5_update_lifecycle_certification() -> M5UpdateLifecycleCertification {
    assemble(
        M5_UPDATE_LIFECYCLE_CERTIFICATION_PACKET_ID,
        "M5 update / support-lifecycle certification",
        &seeded_m5_update_lifecycle(),
    )
}

/// Drill: the governance matrix's change-impact proof is stale, so every claimed channel the
/// change-impact facet scopes to narrows on the update-communication dimension while the channels it
/// does not scope to (LTS) stay certified — proving narrowing is per claim, not behind a generic
/// stable label.
pub fn seeded_m5_update_lifecycle_certification_stale_proof_narrowed(
) -> M5UpdateLifecycleCertification {
    assemble(
        "m5-update-lifecycle-certification:drill-stale:0001",
        "M5 update / support-lifecycle certification — stale-proof drill",
        &seeded_m5_update_lifecycle_stale_proof_narrowed(),
    )
}

/// Drill: the governance matrix's service-health proof is missing, so every claimed channel /
/// profile blocks on the stale-data-behavior dimension and the consumers that surface it block from
/// Stable promotion.
pub fn seeded_m5_update_lifecycle_certification_missing_proof_blocked(
) -> M5UpdateLifecycleCertification {
    assemble(
        "m5-update-lifecycle-certification:drill-missing:0001",
        "M5 update / support-lifecycle certification — missing-proof drill",
        &seeded_m5_update_lifecycle_missing_proof_blocked(),
    )
}
