//! Canonical seed builders for the M5 claim-narrowing registry.
//!
//! These builders are the single producer of the checked-in claim-narrowing registry, the
//! release-grade parity proof, and the per-state consumer fixtures. The headless emitter and the
//! inline tests both call them so the in-code cases, the artifacts, and the fixtures never drift.
//! The six seed cases each isolate one controlled degraded-claim state — fully supported, limited,
//! retest pending, evidence stale, unsupported client, and unsupported — so the registry exercises
//! every state in the frozen vocabulary while every consumer surface converges on the same state.

use super::*;

use crate::m5_descriptor_object::{
    ArtifactBinding, AuthorityClass, ClientScope, ClientScopeSubDescriptor, DescriptorObjectInput,
    EvidenceState, FreshnessState, FreshnessSubDescriptor, HandoffRequirement, ProvenanceClass,
    ProvenanceSubDescriptor, QualificationSubDescriptor, SignatureState,
};

/// Stable registry id for the canonical claim-narrowing registry.
pub const M5_CLAIM_NARROWING_REGISTRY_ID: &str = "m5-claim-narrowing-registry:stable:0001";

/// Mint timestamp for the canonical cases.
const SEED_MINTED_AT: &str = "2026-07-06T00:00:00Z";

const REDACTION_CLASS: &str = "metadata_safe_default";

/// Builds a descriptor object for a claim-narrowing case from its four sub-descriptors. The
/// helper keeps every seed case's clean facets identical so each case isolates exactly the
/// degraded facets it means to demonstrate.
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
    client_scope: ClientScopeSubDescriptor,
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
        client_scope,
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_MINTED_AT.to_owned(),
    })
}

/// The clean provenance sub-descriptor: first-party signed and attested.
fn clean_provenance() -> ProvenanceSubDescriptor {
    ProvenanceSubDescriptor {
        source_class: ProvenanceClass::FirstPartySigned,
        signature_state: SignatureState::SignedAttested,
    }
}

/// The clean freshness sub-descriptor: current and complete.
fn clean_freshness() -> FreshnessSubDescriptor {
    FreshnessSubDescriptor {
        freshness_state: FreshnessState::Current,
        evidence_state: EvidenceState::Complete,
    }
}

/// The clean qualification sub-descriptor: a Stable claim with complete evidence.
fn clean_qualification() -> QualificationSubDescriptor {
    QualificationSubDescriptor {
        support_class: QualificationClass::Stable,
        evidence_state: EvidenceState::Complete,
    }
}

/// The clean client-scope sub-descriptor: full desktop authority, no handoff.
fn clean_client_scope() -> ClientScopeSubDescriptor {
    ClientScopeSubDescriptor {
        client_kind: ClientScope::DesktopFull,
        authority_class: AuthorityClass::FullAuthority,
        handoff_requirement: HandoffRequirement::NotRequired,
    }
}

/// A fully-governed condition: clean provenance, current evidence, full authority. No descriptor
/// narrows it, so every consumer surface stands fully supported at Stable.
pub fn seeded_fully_supported_case() -> ClaimNarrowingCase {
    ClaimNarrowingCase::from_descriptor(
        "claim-narrowing:fully-supported",
        "Fully-supported release claim",
        case_descriptor(
            "m5-descriptor-object:release-artifact-graph:stable:0001",
            "Release artifact-graph descriptor",
            "release-artifact-graph:0001",
            "release_artifact_graph",
            "build_provenance",
            clean_provenance(),
            clean_freshness(),
            clean_qualification(),
            clean_client_scope(),
        ),
    )
}

/// A condition whose evidence and provenance are present but limited: an unsigned community
/// origin and limited qualification evidence, otherwise clean and on the full desktop client. It
/// narrows to `limited` on every surface.
pub fn seeded_limited_case() -> ClaimNarrowingCase {
    ClaimNarrowingCase::from_descriptor(
        "claim-narrowing:limited",
        "Limited-evidence marketplace claim",
        case_descriptor(
            "m5-descriptor-object:marketplace-extension:limited:0001",
            "Marketplace extension descriptor",
            "marketplace-extension:0042",
            "marketplace_extension",
            "extension_listing",
            ProvenanceSubDescriptor {
                source_class: ProvenanceClass::Community,
                signature_state: SignatureState::Unsigned,
            },
            clean_freshness(),
            QualificationSubDescriptor {
                support_class: QualificationClass::Stable,
                evidence_state: EvidenceState::Limited,
            },
            clean_client_scope(),
        ),
    )
}

/// A condition whose qualification evidence is awaiting a retest: clean everywhere else and on
/// the full desktop client, so the single retest-pending facet drives the state.
pub fn seeded_retest_pending_case() -> ClaimNarrowingCase {
    ClaimNarrowingCase::from_descriptor(
        "claim-narrowing:retest-pending",
        "Retest-pending docs claim",
        case_descriptor(
            "m5-descriptor-object:docs-reference:retest:0001",
            "Docs reference descriptor",
            "docs-reference:5001",
            "docs_reference",
            "reference_page",
            clean_provenance(),
            clean_freshness(),
            QualificationSubDescriptor {
                support_class: QualificationClass::Stable,
                evidence_state: EvidenceState::RetestPending,
            },
            clean_client_scope(),
        ),
    )
}

/// A condition whose freshness window lapsed: stale evidence, clean elsewhere and on the full
/// desktop client. Stale evidence narrows every surface to `evidence_stale`.
pub fn seeded_evidence_stale_case() -> ClaimNarrowingCase {
    ClaimNarrowingCase::from_descriptor(
        "claim-narrowing:evidence-stale",
        "Stale-evidence evaluation-pack claim",
        case_descriptor(
            "m5-descriptor-object:evaluation-pack:stale:0001",
            "Evaluation-pack descriptor",
            "evaluation-pack:7001",
            "evaluation_pack",
            "claim_summary",
            clean_provenance(),
            FreshnessSubDescriptor {
                freshness_state: FreshnessState::Stale,
                evidence_state: EvidenceState::Complete,
            },
            clean_qualification(),
            clean_client_scope(),
        ),
    )
}

/// A condition whose client scope cannot carry the claimed capability: a scoped companion that
/// must hand off to the desktop, clean provenance and current evidence. The narrowed client
/// drives `unsupported_client`.
pub fn seeded_unsupported_client_case() -> ClaimNarrowingCase {
    ClaimNarrowingCase::from_descriptor(
        "claim-narrowing:unsupported-client",
        "Companion-scope handoff claim",
        case_descriptor(
            "m5-descriptor-object:companion-action:scoped:0001",
            "Companion action descriptor",
            "companion-action:8001",
            "companion_action",
            "handoff_summary",
            clean_provenance(),
            clean_freshness(),
            clean_qualification(),
            ClientScopeSubDescriptor {
                client_kind: ClientScope::CompanionScoped,
                authority_class: AuthorityClass::ScopedAuthority,
                handoff_requirement: HandoffRequirement::DesktopHandoffRequired,
            },
        ),
    )
}

/// A condition whose absent provenance and missing evidence block the claim: a side-loaded
/// artifact with no provided origin or signature, missing freshness evidence, running
/// browser-reference only. The blockers hold the claim at `unsupported` / Unavailable while every
/// absent value stays explicit.
pub fn seeded_unsupported_case() -> ClaimNarrowingCase {
    ClaimNarrowingCase::from_descriptor(
        "claim-narrowing:unsupported",
        "Side-loaded blocked claim",
        case_descriptor(
            "m5-descriptor-object:sideloaded-doc:blocked:0001",
            "Side-loaded docs reference descriptor",
            "docs-reference:9001",
            "docs_reference",
            "browser_reference",
            ProvenanceSubDescriptor {
                source_class: ProvenanceClass::NotProvided,
                signature_state: SignatureState::NotProvided,
            },
            FreshnessSubDescriptor {
                freshness_state: FreshnessState::Missing,
                evidence_state: EvidenceState::NotProvided,
            },
            QualificationSubDescriptor {
                support_class: QualificationClass::Beta,
                evidence_state: EvidenceState::RetestPending,
            },
            ClientScopeSubDescriptor {
                client_kind: ClientScope::BrowserReference,
                authority_class: AuthorityClass::ReferenceOnly,
                handoff_requirement: HandoffRequirement::ConsoleHandoffRequired,
            },
        ),
    )
}

/// The canonical claim-narrowing registry: the six seed cases spanning every degraded-claim
/// state, the controlled vocabulary, the consumer set, the conformance review, and the summary.
pub fn seeded_m5_claim_narrowing_registry() -> M5ClaimNarrowingRegistry {
    M5ClaimNarrowingRegistry::new(M5ClaimNarrowingRegistryInput {
        registry_id: M5_CLAIM_NARROWING_REGISTRY_ID.to_owned(),
        report_label: "M5 claim-narrowing parity across public-truth consumers".to_owned(),
        cases: vec![
            seeded_fully_supported_case(),
            seeded_limited_case(),
            seeded_retest_pending_case(),
            seeded_evidence_stale_case(),
            seeded_unsupported_client_case(),
            seeded_unsupported_case(),
        ],
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_MINTED_AT.to_owned(),
    })
}
