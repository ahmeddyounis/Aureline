//! Canonical seed builder for the M5 badge vocabulary.
//!
//! This builder is the single producer of the checked-in badge-vocabulary packet, the
//! published inventory, the release parity proof, the Markdown drawer catalog, and the
//! consumer-render fixtures. The headless emitter and the inline tests both call it so the
//! in-code packet, the artifacts, and the fixtures never drift. Every badge label, summary,
//! and drawer is generated from the controlled enums, so the vocabulary is always derivable
//! from the descriptors Aureline ships.

use super::*;

/// Stable packet id for the canonical badge-vocabulary packet.
pub const M5_BADGE_VOCABULARY_PACKET_ID: &str = "m5-badge-vocabulary:stable:0001";

/// Evaluation / mint timestamp for the canonical packet.
const SEED_EVALUATED_AT: &str = "2026-07-06T00:00:00Z";

/// The canonical badge vocabulary every public-truth surface renders.
pub fn seeded_m5_badge_vocabulary() -> M5BadgeVocabulary {
    M5BadgeVocabulary::canonical(
        M5_BADGE_VOCABULARY_PACKET_ID,
        "M5 badge vocabulary and explanation drawers",
        SEED_EVALUATED_AT,
        SEED_EVALUATED_AT,
    )
}
