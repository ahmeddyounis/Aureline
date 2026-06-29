//! Canonical seed builders for the M5 descriptor-join registry.
//!
//! These builders are the single producer of the checked-in descriptor-join registry, the
//! release-grade parity proof, and the per-state carrier fixtures. The headless emitter and the
//! inline tests both call them so the in-code joins, the artifacts, and the fixtures never drift.
//! Each join is built from the *same* descriptor condition the
//! [claim-narrowing](crate::m5_claim_narrowing) lane uses, so the export/support/admin carriers
//! and the interactive consumer surfaces stay in lockstep: a stale or narrowed descriptor narrows
//! every carrier exactly as it narrows every interactive consumer.

use super::*;

use crate::m5_claim_narrowing::{
    seeded_evidence_stale_case, seeded_fully_supported_case, seeded_limited_case,
    seeded_retest_pending_case, seeded_unsupported_case, seeded_unsupported_client_case,
};

/// Stable registry id for the canonical descriptor-join registry.
pub const M5_DESCRIPTOR_JOIN_REGISTRY_ID: &str = "m5-descriptor-join-registry:stable:0001";

/// Mint timestamp for the canonical joins.
const SEED_MINTED_AT: &str = "2026-07-06T00:00:00Z";

const REDACTION_CLASS: &str = "metadata_safe_default";

/// A fully-governed condition joined into copy-safe carriers: clean provenance, current evidence,
/// full authority. No descriptor narrows it, so every carrier stands fully supported at Stable.
pub fn seeded_fully_supported_join() -> DescriptorJoin {
    DescriptorJoin::from_descriptor(
        "descriptor-join:fully-supported",
        "Fully-supported release export join",
        seeded_fully_supported_case().descriptor,
    )
}

/// A limited-evidence condition (unsigned community origin, limited qualification evidence) joined
/// into copy-safe carriers. The three downgrade reasons stay attributable on every carrier.
pub fn seeded_limited_join() -> DescriptorJoin {
    DescriptorJoin::from_descriptor(
        "descriptor-join:limited",
        "Limited-evidence marketplace export join",
        seeded_limited_case().descriptor,
    )
}

/// A retest-pending qualification condition joined into copy-safe carriers.
pub fn seeded_retest_pending_join() -> DescriptorJoin {
    DescriptorJoin::from_descriptor(
        "descriptor-join:retest-pending",
        "Retest-pending docs export join",
        seeded_retest_pending_case().descriptor,
    )
}

/// A stale-evidence condition joined into copy-safe carriers. The stale freshness reason stays
/// attributable across export packet, support bundle, admin report, and copy-safe summary.
pub fn seeded_evidence_stale_join() -> DescriptorJoin {
    DescriptorJoin::from_descriptor(
        "descriptor-join:evidence-stale",
        "Stale-evidence evaluation-pack export join",
        seeded_evidence_stale_case().descriptor,
    )
}

/// A narrowed-client condition (scoped companion that must hand off to the desktop) joined into
/// copy-safe carriers. The narrowed client cannot read as full authority on any carrier.
pub fn seeded_unsupported_client_join() -> DescriptorJoin {
    DescriptorJoin::from_descriptor(
        "descriptor-join:unsupported-client",
        "Companion-scope handoff export join",
        seeded_unsupported_client_case().descriptor,
    )
}

/// A blocked, side-loaded condition (no provided origin or signature, missing freshness evidence,
/// browser-reference only) joined into copy-safe carriers. Every absent value stays explicit as an
/// attributable downgrade reason while the carriers all read unsupported / unavailable.
pub fn seeded_unsupported_join() -> DescriptorJoin {
    DescriptorJoin::from_descriptor(
        "descriptor-join:unsupported",
        "Side-loaded blocked export join",
        seeded_unsupported_case().descriptor,
    )
}

/// The canonical descriptor-join registry: the six seed joins spanning every degraded-claim state,
/// the controlled vocabulary, the consumer set, the conformance review, and the summary.
pub fn seeded_m5_descriptor_join_registry() -> M5DescriptorJoinRegistry {
    M5DescriptorJoinRegistry::new(M5DescriptorJoinRegistryInput {
        registry_id: M5_DESCRIPTOR_JOIN_REGISTRY_ID.to_owned(),
        report_label: "M5 descriptor export/support/admin join parity".to_owned(),
        joins: vec![
            seeded_fully_supported_join(),
            seeded_limited_join(),
            seeded_retest_pending_join(),
            seeded_evidence_stale_join(),
            seeded_unsupported_client_join(),
            seeded_unsupported_join(),
        ],
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_MINTED_AT.to_owned(),
    })
}
