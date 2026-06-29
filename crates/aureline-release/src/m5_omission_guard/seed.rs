//! Canonical seed builders for the M5 omission-guard registry.
//!
//! These builders are the single producer of the checked-in omission-guard registry, the
//! release-grade parity proof, and the per-condition consumer fixtures. The headless emitter and
//! the inline tests both call them so the in-code cases, the artifacts, and the fixtures never
//! drift. Each case is built from the *same* descriptor condition the
//! [claim-narrowing](crate::m5_claim_narrowing) lane derives a claim state from, so a Mirrored,
//! Offline, Side-loaded, `not_provided`, partial, or stale condition surfaces the same present
//! states on every consumer exactly when it narrows the shared claim. The mirror / offline /
//! side-loaded / partial conditions the upstream claim-narrowing seeds do not isolate are built
//! here so each spec-named weaker state is first-class and exercised.

use super::*;

use crate::m5_claim_narrowing::{
    seeded_evidence_stale_case, seeded_fully_supported_case, seeded_limited_case,
    seeded_unsupported_case, seeded_unsupported_client_case,
};
use crate::m5_descriptor_object::{
    ArtifactBinding, DescriptorObject, DescriptorObjectInput, EvidenceState,
    FreshnessSubDescriptor, ProvenanceClass, ProvenanceSubDescriptor, QualificationSubDescriptor,
    SignatureState,
};

/// Stable registry id for the canonical omission-guard registry.
pub const M5_OMISSION_GUARD_REGISTRY_ID: &str = "m5-omission-guard-registry:stable:0001";

/// Mint timestamp for the canonical cases.
const SEED_MINTED_AT: &str = "2026-07-06T00:00:00Z";

const REDACTION_CLASS: &str = "metadata_safe_default";

/// Builds a descriptor object for an omission-guard case from sub-descriptors that override the
/// clean baseline only where the case means to demonstrate a weaker state.
#[allow(clippy::too_many_arguments)]
fn case_descriptor(
    descriptor_id: &str,
    descriptor_label: &str,
    artifact_id: &str,
    artifact_family: &str,
    artifact_kind: &str,
    provenance: ProvenanceSubDescriptor,
    freshness: FreshnessSubDescriptor,
    qualification: QualificationSubDescriptor,
) -> DescriptorObject {
    DescriptorObject::new(DescriptorObjectInput {
        descriptor_id: descriptor_id.to_owned(),
        descriptor_label: descriptor_label.to_owned(),
        artifact_ref: ArtifactBinding {
            artifact_id: artifact_id.to_owned(),
            artifact_family: artifact_family.to_owned(),
            artifact_kind: artifact_kind.to_owned(),
            schema_ref: "schemas/release/artifact_graph.schema.json".to_owned(),
            content_digest_ref: format!("digest-ref:{artifact_id}"),
        },
        provenance,
        freshness,
        qualification,
        client_scope: seeded_fully_supported_case().descriptor.client_scope,
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_MINTED_AT.to_owned(),
    })
}

/// The clean freshness sub-descriptor: current and complete.
fn clean_freshness() -> FreshnessSubDescriptor {
    seeded_fully_supported_case().descriptor.freshness
}

/// The clean qualification sub-descriptor: a Stable claim with complete evidence.
fn clean_qualification() -> QualificationSubDescriptor {
    seeded_fully_supported_case().descriptor.qualification
}

/// A fully-official condition: first-party signed, current evidence, full authority. No weakening
/// is present, so every consumer renders only the explicit `official` anchor.
pub fn seeded_official_case() -> OmissionGuardCase {
    OmissionGuardCase::from_descriptor(
        "omission-guard:official",
        "Fully-official release surface",
        seeded_fully_supported_case().descriptor,
    )
}

/// A mirrored-origin condition: the artifact comes from a mirror of the upstream origin, otherwise
/// clean. The `mirrored` state surfaces on every consumer and can never be omitted.
pub fn seeded_mirrored_case() -> OmissionGuardCase {
    OmissionGuardCase::from_descriptor(
        "omission-guard:mirrored",
        "Mirrored-origin marketplace surface",
        case_descriptor(
            "m5-descriptor-object:marketplace-mirror:mirrored:0001",
            "Mirrored marketplace listing descriptor",
            "marketplace-extension:mirror:0001",
            "marketplace_extension",
            "extension_listing",
            ProvenanceSubDescriptor {
                source_class: ProvenanceClass::Mirror,
                signature_state: SignatureState::SignedAttested,
            },
            clean_freshness(),
            clean_qualification(),
        ),
    )
}

/// An offline-bundle condition: the artifact arrived as an out-of-band offline bundle, otherwise
/// clean. The `offline` state surfaces on every consumer and can never be omitted.
pub fn seeded_offline_case() -> OmissionGuardCase {
    OmissionGuardCase::from_descriptor(
        "omission-guard:offline",
        "Offline-bundle docs surface",
        case_descriptor(
            "m5-descriptor-object:docs-offline:offline:0001",
            "Offline docs bundle descriptor",
            "docs-reference:offline:0001",
            "docs_reference",
            "offline_bundle",
            ProvenanceSubDescriptor {
                source_class: ProvenanceClass::OfflineBundle,
                signature_state: SignatureState::SignedAttested,
            },
            clean_freshness(),
            clean_qualification(),
        ),
    )
}

/// A side-loaded condition: the artifact was installed outside the governed channel and is
/// unsigned. Both the `side_loaded` origin and the `unverified` signature surface on every
/// consumer.
pub fn seeded_side_loaded_case() -> OmissionGuardCase {
    OmissionGuardCase::from_descriptor(
        "omission-guard:side-loaded",
        "Side-loaded extension surface",
        case_descriptor(
            "m5-descriptor-object:sideloaded-extension:sideloaded:0001",
            "Side-loaded extension descriptor",
            "marketplace-extension:sideloaded:0001",
            "marketplace_extension",
            "extension_listing",
            ProvenanceSubDescriptor {
                source_class: ProvenanceClass::SideLoaded,
                signature_state: SignatureState::Unsigned,
            },
            clean_freshness(),
            clean_qualification(),
        ),
    )
}

/// A partial-evidence condition: first-party signed and current, but the qualification evidence is
/// partial. The `official` anchor and the `partial` state surface together on every consumer.
pub fn seeded_partial_evidence_case() -> OmissionGuardCase {
    OmissionGuardCase::from_descriptor(
        "omission-guard:partial-evidence",
        "Partial-evidence evaluation surface",
        case_descriptor(
            "m5-descriptor-object:evaluation-pack:partial:0001",
            "Partial-evidence evaluation-pack descriptor",
            "evaluation-pack:partial:0001",
            "evaluation_pack",
            "claim_summary",
            seeded_fully_supported_case().descriptor.provenance,
            clean_freshness(),
            QualificationSubDescriptor {
                support_class: clean_qualification().support_class,
                evidence_state: EvidenceState::Partial,
            },
        ),
    )
}

/// A community-origin, limited-evidence condition reused from the claim-narrowing lane. The
/// `community` origin, the `unverified` signature, and the `partial` evidence all surface.
pub fn seeded_community_limited_case() -> OmissionGuardCase {
    OmissionGuardCase::from_descriptor(
        "omission-guard:community-limited",
        "Community-origin limited surface",
        seeded_limited_case().descriptor,
    )
}

/// A stale-evidence condition reused from the claim-narrowing lane: first-party signed but the
/// freshness window lapsed. The `official` anchor and the `stale` state surface together.
pub fn seeded_stale_case() -> OmissionGuardCase {
    OmissionGuardCase::from_descriptor(
        "omission-guard:stale",
        "Stale-evidence release surface",
        seeded_evidence_stale_case().descriptor,
    )
}

/// A scoped-client condition reused from the claim-narrowing lane: first-party signed and current,
/// but a companion scope that must hand off to the desktop. The `official` anchor, the
/// `scoped_client` state, and the `handoff_required` state surface together.
pub fn seeded_scoped_client_case() -> OmissionGuardCase {
    OmissionGuardCase::from_descriptor(
        "omission-guard:scoped-client",
        "Companion-scope handoff surface",
        seeded_unsupported_client_case().descriptor,
    )
}

/// A blocked, not-provided condition reused from the claim-narrowing lane: no provided origin or
/// signature, missing freshness evidence, browser-reference only. Every absent value stays
/// explicit — `not_provided`, `missing`, `retest_pending`, `scoped_client`, and
/// `handoff_required` all surface rather than disappearing into omission.
pub fn seeded_not_provided_blocked_case() -> OmissionGuardCase {
    OmissionGuardCase::from_descriptor(
        "omission-guard:not-provided-blocked",
        "Side-loaded not-provided blocked surface",
        seeded_unsupported_case().descriptor,
    )
}

/// The canonical omission-guard registry: the nine seed cases spanning the official anchor and the
/// mirror / offline / side-loaded / unverified / partial / stale / scoped / handoff / not-provided
/// weaker states, the controlled vocabulary, the consumer set, the conformance review, and the
/// summary.
pub fn seeded_m5_omission_guard_registry() -> M5OmissionGuardRegistry {
    M5OmissionGuardRegistry::new(M5OmissionGuardRegistryInput {
        registry_id: M5_OMISSION_GUARD_REGISTRY_ID.to_owned(),
        report_label: "M5 no-silent-omission guard parity across public-truth consumers".to_owned(),
        cases: vec![
            seeded_official_case(),
            seeded_mirrored_case(),
            seeded_offline_case(),
            seeded_side_loaded_case(),
            seeded_partial_evidence_case(),
            seeded_community_limited_case(),
            seeded_stale_case(),
            seeded_scoped_client_case(),
            seeded_not_provided_blocked_case(),
        ],
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_MINTED_AT.to_owned(),
    })
}
