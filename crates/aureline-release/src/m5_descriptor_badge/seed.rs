//! Canonical seed builders for the M5 descriptor/badge matrix.
//!
//! These builders are the single producer of the checked-in matrix packet, the published
//! inventory, the standalone descriptor artifacts, the Markdown governance matrix, and the
//! stale / missing / unmapped consumer drill fixtures. The headless emitter and the inline
//! tests both call them so the in-code packet, the artifacts, and the fixtures never drift.
//! Every builder derives each consumer's verdict from the same checked-in descriptor
//! contracts, so the qualification is always generated from the descriptor proofs Aureline
//! ships: the canonical packet is all-governed; the drills perturb one descriptor's proof
//! freshness (or drop its contract) and let the derivation recompute each consumer's status,
//! gate, effective qualification, and named gaps.

use super::*;

/// Stable packet id for the canonical (all-governed) matrix packet.
pub const M5_DESCRIPTOR_BADGE_MATRIX_PACKET_ID: &str = "m5-descriptor-badge-matrix:stable:0001";

/// Evaluation / mint timestamp for the canonical packet — a date at which every descriptor's
/// proof is current.
const SEED_EVALUATED_AT: &str = "2026-07-06T00:00:00Z";

const REDACTION_CLASS: &str = "metadata_safe_default";

/// The descriptor the stale drill perturbs. It is bound by most consumers but not the
/// marketplace or docs references, so the drill narrows exactly the consumers that depend on
/// freshness.
const STALE_DRILL_FAMILY: DescriptorFamily = DescriptorFamily::Freshness;

/// The descriptor the missing drill perturbs. It is bound by the release center,
/// marketplace, certification, support, and companion consumers, so the drill blocks exactly
/// the consumers that depend on client scope.
const FAILING_DRILL_FAMILY: DescriptorFamily = DescriptorFamily::ClientScope;

/// The claimed consumer surfaces and the descriptor families each binds. Together the
/// bindings cover every descriptor family.
const CONSUMER_DEFS: [(PublicTruthConsumer, QualificationClass, &[DescriptorFamily]); 8] = [
    (
        PublicTruthConsumer::ReleaseCenter,
        QualificationClass::Stable,
        &[
            DescriptorFamily::Provenance,
            DescriptorFamily::Freshness,
            DescriptorFamily::Qualification,
            DescriptorFamily::ClientScope,
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
    ),
    (
        PublicTruthConsumer::Marketplace,
        QualificationClass::Stable,
        &[
            DescriptorFamily::Provenance,
            DescriptorFamily::Qualification,
            DescriptorFamily::ClientScope,
        ],
    ),
    (
        PublicTruthConsumer::DocsHelp,
        QualificationClass::Stable,
        &[
            DescriptorFamily::Provenance,
            DescriptorFamily::Qualification,
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
    ),
    (
        PublicTruthConsumer::EvaluationPacks,
        QualificationClass::Stable,
        &[
            DescriptorFamily::Provenance,
            DescriptorFamily::Freshness,
            DescriptorFamily::Qualification,
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
    ),
    (
        PublicTruthConsumer::CompanionHandoff,
        QualificationClass::Stable,
        &[
            DescriptorFamily::Freshness,
            DescriptorFamily::Qualification,
            DescriptorFamily::ClientScope,
        ],
    ),
];

/// Builds one descriptor contract at a given proof freshness.
pub fn seeded_descriptor_contract(
    family: DescriptorFamily,
    proof_freshness: FreshnessState,
) -> DescriptorContract {
    DescriptorContract::for_family(family, proof_freshness)
}

/// Builds the canonical descriptor contracts with every proof current.
fn canonical_descriptors() -> Vec<DescriptorContract> {
    DescriptorFamily::ALL
        .iter()
        .map(|family| DescriptorContract::for_family(*family, FreshnessState::Current))
        .collect()
}

/// Marks one descriptor's proof at the given freshness state.
fn with_descriptor_state(
    mut descriptors: Vec<DescriptorContract>,
    family: DescriptorFamily,
    state: FreshnessState,
) -> Vec<DescriptorContract> {
    for contract in &mut descriptors {
        if contract.family == family {
            contract.proof_freshness = state;
        }
    }
    descriptors
}

/// Builds the claimed consumer bindings; gaps and verdict are recomputed in the packet.
fn consumer_bindings() -> Vec<ConsumerBinding> {
    CONSUMER_DEFS
        .iter()
        .map(|(consumer, claimed, bound)| ConsumerBinding::new(*consumer, *claimed, bound))
        .collect()
}

/// Assembles a packet from the given descriptor contracts.
fn assemble_packet(
    packet_id: &str,
    report_label: &str,
    descriptors: Vec<DescriptorContract>,
) -> M5DescriptorBadgeMatrix {
    M5DescriptorBadgeMatrix::new(M5DescriptorBadgeMatrixInput {
        packet_id: packet_id.to_owned(),
        report_label: report_label.to_owned(),
        evaluated_at: SEED_EVALUATED_AT.to_owned(),
        descriptors,
        consumer_bindings: consumer_bindings(),
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_EVALUATED_AT.to_owned(),
    })
}

/// The canonical, all-governed descriptor/badge matrix.
pub fn seeded_m5_descriptor_badge_matrix() -> M5DescriptorBadgeMatrix {
    assemble_packet(
        M5_DESCRIPTOR_BADGE_MATRIX_PACKET_ID,
        "M5 descriptor / badge matrix",
        canonical_descriptors(),
    )
}

/// Drill: one descriptor's proof is stale, so the consumers that bind it auto-narrow below
/// Stable.
pub fn seeded_m5_descriptor_badge_matrix_stale_proof_narrowed() -> M5DescriptorBadgeMatrix {
    let descriptors = with_descriptor_state(
        canonical_descriptors(),
        STALE_DRILL_FAMILY,
        FreshnessState::Stale,
    );
    assemble_packet(
        "m5-descriptor-badge-matrix:drill-stale:0001",
        "M5 descriptor / badge matrix — stale-proof drill",
        descriptors,
    )
}

/// Drill: one descriptor's proof is missing, so the consumers that bind it are blocked from
/// Stable promotion.
pub fn seeded_m5_descriptor_badge_matrix_missing_proof_blocked() -> M5DescriptorBadgeMatrix {
    let descriptors = with_descriptor_state(
        canonical_descriptors(),
        FAILING_DRILL_FAMILY,
        FreshnessState::Missing,
    );
    assemble_packet(
        "m5-descriptor-badge-matrix:drill-missing:0001",
        "M5 descriptor / badge matrix — missing-proof drill",
        descriptors,
    )
}
