//! Canonical seed builders for the M5 descriptor-certification packet.
//!
//! These builders are the single producer of the checked-in certification packet, the published
//! inventory, the release-grade parity proof (and its Markdown report), and the stale / missing
//! drill fixtures. The headless emitter and the inline tests both call them so the in-code packet,
//! the artifacts, and the fixtures never drift. Every builder derives each consumer's verdict from
//! the same certified runtime lanes, so the qualification is always generated from the parity
//! proofs Aureline ships: the canonical packet certifies every lane current; the drills perturb one
//! lane's parity-proof freshness and let the derivation recompute each consumer's status, gate,
//! effective qualification, and named gaps.

use super::*;

/// Stable packet id for the canonical (all-current) certification packet.
pub const M5_DESCRIPTOR_CERTIFICATION_PACKET_ID: &str = "m5-descriptor-certification:stable:0001";

/// Evaluation / mint timestamp for the canonical packet — a date at which every lane's parity
/// proof is current.
const SEED_EVALUATED_AT: &str = "2026-07-06T00:00:00Z";

const REDACTION_CLASS: &str = "metadata_safe_default";

/// The lane the stale drill perturbs. It is read by the export-carrier consumers (release center,
/// certification, evaluation packs, support exports, companion handoffs) but not the docs or
/// marketplace references, so the drill narrows exactly the consumers that depend on it.
const STALE_DRILL_LANE: RuntimeLane = RuntimeLane::DescriptorJoin;

/// The lane the failing drill perturbs. It is read by the client-scope consumers (release center,
/// Help/About, marketplace, certification, support exports, companion handoffs), so the drill
/// blocks exactly the consumers that depend on client scope.
const FAILING_DRILL_LANE: RuntimeLane = RuntimeLane::ClientScopeCard;

/// The claimed consumer surfaces, the descriptor families each binds, and the runtime lanes each
/// reads. Together the bindings cover every descriptor family and the reads cover every lane.
const CONSUMER_DEFS: [(
    PublicTruthConsumer,
    QualificationClass,
    &[DescriptorFamily],
    &[RuntimeLane],
); 8] = [
    (
        PublicTruthConsumer::ReleaseCenter,
        QualificationClass::Stable,
        &[
            DescriptorFamily::Provenance,
            DescriptorFamily::Freshness,
            DescriptorFamily::Qualification,
            DescriptorFamily::ClientScope,
        ],
        &[
            RuntimeLane::DescriptorObject,
            RuntimeLane::DescriptorBadgeMatrix,
            RuntimeLane::BadgeVocabulary,
            RuntimeLane::ClaimNarrowing,
            RuntimeLane::DescriptorJoin,
            RuntimeLane::OmissionGuard,
            RuntimeLane::ClientScopeCard,
        ],
    ),
    (
        PublicTruthConsumer::HelpAbout,
        QualificationClass::Stable,
        &[
            DescriptorFamily::Provenance,
            DescriptorFamily::Freshness,
            DescriptorFamily::Qualification,
        ],
        &[
            RuntimeLane::DescriptorObject,
            RuntimeLane::DescriptorBadgeMatrix,
            RuntimeLane::BadgeVocabulary,
            RuntimeLane::ClaimNarrowing,
            RuntimeLane::OmissionGuard,
            RuntimeLane::ClientScopeCard,
        ],
    ),
    (
        PublicTruthConsumer::Marketplace,
        QualificationClass::Stable,
        &[
            DescriptorFamily::Provenance,
            DescriptorFamily::Qualification,
            DescriptorFamily::ClientScope,
        ],
        &[
            RuntimeLane::DescriptorObject,
            RuntimeLane::DescriptorBadgeMatrix,
            RuntimeLane::BadgeVocabulary,
            RuntimeLane::ClaimNarrowing,
            RuntimeLane::OmissionGuard,
            RuntimeLane::ClientScopeCard,
        ],
    ),
    (
        PublicTruthConsumer::DocsHelp,
        QualificationClass::Stable,
        &[
            DescriptorFamily::Provenance,
            DescriptorFamily::Qualification,
        ],
        &[
            RuntimeLane::DescriptorObject,
            RuntimeLane::DescriptorBadgeMatrix,
            RuntimeLane::BadgeVocabulary,
            RuntimeLane::ClaimNarrowing,
            RuntimeLane::OmissionGuard,
        ],
    ),
    (
        PublicTruthConsumer::Certification,
        QualificationClass::Stable,
        &[
            DescriptorFamily::Provenance,
            DescriptorFamily::Freshness,
            DescriptorFamily::Qualification,
            DescriptorFamily::ClientScope,
        ],
        &[
            RuntimeLane::DescriptorObject,
            RuntimeLane::DescriptorBadgeMatrix,
            RuntimeLane::BadgeVocabulary,
            RuntimeLane::ClaimNarrowing,
            RuntimeLane::DescriptorJoin,
            RuntimeLane::OmissionGuard,
            RuntimeLane::ClientScopeCard,
        ],
    ),
    (
        PublicTruthConsumer::EvaluationPacks,
        QualificationClass::Stable,
        &[
            DescriptorFamily::Provenance,
            DescriptorFamily::Freshness,
            DescriptorFamily::Qualification,
        ],
        &[
            RuntimeLane::DescriptorObject,
            RuntimeLane::DescriptorBadgeMatrix,
            RuntimeLane::BadgeVocabulary,
            RuntimeLane::ClaimNarrowing,
            RuntimeLane::DescriptorJoin,
            RuntimeLane::OmissionGuard,
        ],
    ),
    (
        PublicTruthConsumer::SupportExport,
        QualificationClass::Stable,
        &[
            DescriptorFamily::Provenance,
            DescriptorFamily::Freshness,
            DescriptorFamily::Qualification,
            DescriptorFamily::ClientScope,
        ],
        &[
            RuntimeLane::DescriptorObject,
            RuntimeLane::DescriptorBadgeMatrix,
            RuntimeLane::BadgeVocabulary,
            RuntimeLane::ClaimNarrowing,
            RuntimeLane::DescriptorJoin,
            RuntimeLane::OmissionGuard,
            RuntimeLane::ClientScopeCard,
        ],
    ),
    (
        PublicTruthConsumer::CompanionHandoff,
        QualificationClass::Stable,
        &[
            DescriptorFamily::Freshness,
            DescriptorFamily::Qualification,
            DescriptorFamily::ClientScope,
        ],
        &[
            RuntimeLane::DescriptorObject,
            RuntimeLane::DescriptorBadgeMatrix,
            RuntimeLane::BadgeVocabulary,
            RuntimeLane::ClaimNarrowing,
            RuntimeLane::DescriptorJoin,
            RuntimeLane::ClientScopeCard,
        ],
    ),
];

/// Builds the canonical certified lanes with every parity proof current.
fn canonical_lanes() -> Vec<CertifiedLane> {
    RuntimeLane::ALL
        .iter()
        .map(|lane| CertifiedLane::for_lane(*lane, FreshnessState::Current))
        .collect()
}

/// Marks one lane's parity proof at the given freshness state.
fn with_lane_state(
    mut lanes: Vec<CertifiedLane>,
    lane: RuntimeLane,
    state: FreshnessState,
) -> Vec<CertifiedLane> {
    for certified in &mut lanes {
        if certified.lane == lane {
            *certified = CertifiedLane::for_lane(lane, state);
        }
    }
    lanes
}

/// Builds the claimed consumer certifications; gaps and verdict are recomputed in the packet.
fn consumer_certifications() -> Vec<CertifiedConsumer> {
    CONSUMER_DEFS
        .iter()
        .map(|(consumer, claimed, bound, lanes)| {
            CertifiedConsumer::new(*consumer, *claimed, bound, lanes)
        })
        .collect()
}

/// Assembles a packet from the given certified lanes.
fn assemble_packet(
    packet_id: &str,
    report_label: &str,
    lanes: Vec<CertifiedLane>,
) -> M5DescriptorCertification {
    M5DescriptorCertification::new(M5DescriptorCertificationInput {
        packet_id: packet_id.to_owned(),
        report_label: report_label.to_owned(),
        evaluated_at: SEED_EVALUATED_AT.to_owned(),
        lanes,
        consumers: consumer_certifications(),
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_EVALUATED_AT.to_owned(),
    })
}

/// The canonical, all-current descriptor-certification packet: every runtime lane certified at a
/// current parity proof, so every consumer stands fully certified at Stable.
pub fn seeded_m5_descriptor_certification() -> M5DescriptorCertification {
    assemble_packet(
        M5_DESCRIPTOR_CERTIFICATION_PACKET_ID,
        "M5 descriptor / badge certification",
        canonical_lanes(),
    )
}

/// Drill: one lane's parity proof is stale, so the consumers that read it auto-narrow below Stable.
pub fn seeded_m5_descriptor_certification_stale_proof_narrowed() -> M5DescriptorCertification {
    let lanes = with_lane_state(canonical_lanes(), STALE_DRILL_LANE, FreshnessState::Stale);
    assemble_packet(
        "m5-descriptor-certification:drill-stale:0001",
        "M5 descriptor / badge certification — stale-proof drill",
        lanes,
    )
}

/// Drill: one lane's parity proof is missing, so the consumers that read it are blocked from Stable
/// promotion.
pub fn seeded_m5_descriptor_certification_missing_proof_blocked() -> M5DescriptorCertification {
    let lanes = with_lane_state(
        canonical_lanes(),
        FAILING_DRILL_LANE,
        FreshnessState::Missing,
    );
    assemble_packet(
        "m5-descriptor-certification:drill-missing:0001",
        "M5 descriptor / badge certification — missing-proof drill",
        lanes,
    )
}
