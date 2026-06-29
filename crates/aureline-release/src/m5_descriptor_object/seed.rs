//! Canonical seed builders for the M5 descriptor-object registry.
//!
//! These builders are the single producer of the checked-in descriptor-object registry, the
//! release-grade parity proof, and the descriptor-object instance fixtures. The headless
//! emitter and the inline tests both call them so the in-code objects, the artifacts, and the
//! fixtures never drift. The three seed objects span the range the lane must keep first-class:
//! a fully-governed Stable object, an object whose weaker-but-present evidence auto-narrows it
//! to Beta, and an object whose absent provenance and evidence block Stable — each carrying its
//! weaker values as explicit narrowings rather than dropping them.

use super::*;

/// Stable registry id for the canonical descriptor-object registry.
pub const M5_DESCRIPTOR_OBJECT_REGISTRY_ID: &str = "m5-descriptor-object-registry:stable:0001";

/// Mint timestamp for the canonical objects.
const SEED_MINTED_AT: &str = "2026-07-06T00:00:00Z";

const REDACTION_CLASS: &str = "metadata_safe_default";

/// A fully-governed, first-party-signed Stable descriptor object: clean provenance, current
/// and complete evidence, full desktop authority, and no handoff. It carries no narrowings, so
/// its effective qualification stands at Stable.
pub fn seeded_stable_descriptor_object() -> DescriptorObject {
    DescriptorObject::new(DescriptorObjectInput {
        descriptor_id: "m5-descriptor-object:release-artifact-graph:stable:0001".to_owned(),
        descriptor_label: "Release artifact-graph descriptor".to_owned(),
        artifact_ref: ArtifactBinding {
            artifact_id: "release-artifact-graph:0001".to_owned(),
            artifact_family: "release_artifact_graph".to_owned(),
            artifact_kind: "build_provenance".to_owned(),
            schema_ref: "schemas/release/artifact_graph.schema.json".to_owned(),
            content_digest_ref: "digest-ref:release-artifact-graph:0001".to_owned(),
        },
        provenance: ProvenanceSubDescriptor {
            source_class: ProvenanceClass::FirstPartySigned,
            signature_state: SignatureState::SignedAttested,
        },
        freshness: FreshnessSubDescriptor {
            freshness_state: FreshnessState::Current,
            evidence_state: EvidenceState::Complete,
        },
        qualification: QualificationSubDescriptor {
            support_class: QualificationClass::Stable,
            evidence_state: EvidenceState::Complete,
        },
        client_scope: ClientScopeSubDescriptor {
            client_kind: ClientScope::DesktopFull,
            authority_class: AuthorityClass::FullAuthority,
            handoff_requirement: HandoffRequirement::NotRequired,
        },
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_MINTED_AT.to_owned(),
    })
}

/// A descriptor object whose weaker-but-present evidence auto-narrows it below Stable: a
/// community mirror served unverified, stale-but-partial evidence, a scoped companion that
/// requires a desktop handoff. Every weaker value survives as a named narrowing, and the
/// effective qualification floors at Beta.
pub fn seeded_narrowed_descriptor_object() -> DescriptorObject {
    DescriptorObject::new(DescriptorObjectInput {
        descriptor_id: "m5-descriptor-object:companion-extension:narrowed:0001".to_owned(),
        descriptor_label: "Companion marketplace extension descriptor".to_owned(),
        artifact_ref: ArtifactBinding {
            artifact_id: "marketplace-extension:0042".to_owned(),
            artifact_family: "marketplace_extension".to_owned(),
            artifact_kind: "companion_panel".to_owned(),
            schema_ref: "schemas/marketplace/extension_listing.schema.json".to_owned(),
            content_digest_ref: "digest-ref:marketplace-extension:0042".to_owned(),
        },
        provenance: ProvenanceSubDescriptor {
            source_class: ProvenanceClass::Mirror,
            signature_state: SignatureState::SignedUnverified,
        },
        freshness: FreshnessSubDescriptor {
            freshness_state: FreshnessState::Stale,
            evidence_state: EvidenceState::Partial,
        },
        qualification: QualificationSubDescriptor {
            support_class: QualificationClass::Stable,
            evidence_state: EvidenceState::Limited,
        },
        client_scope: ClientScopeSubDescriptor {
            client_kind: ClientScope::CompanionScoped,
            authority_class: AuthorityClass::ScopedAuthority,
            handoff_requirement: HandoffRequirement::DesktopHandoffRequired,
        },
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_MINTED_AT.to_owned(),
    })
}

/// A descriptor object whose absent provenance and evidence block Stable: a side-loaded
/// artifact with no provided origin or signature, missing freshness evidence, retest pending,
/// running browser-reference only. The blockers floor the effective qualification at
/// Unavailable while every absent value stays explicit rather than disappearing.
pub fn seeded_not_provided_descriptor_object() -> DescriptorObject {
    DescriptorObject::new(DescriptorObjectInput {
        descriptor_id: "m5-descriptor-object:sideloaded-doc:blocked:0001".to_owned(),
        descriptor_label: "Side-loaded docs reference descriptor".to_owned(),
        artifact_ref: ArtifactBinding {
            artifact_id: "docs-reference:9001".to_owned(),
            artifact_family: "docs_reference".to_owned(),
            artifact_kind: "browser_reference".to_owned(),
            schema_ref: "schemas/docs/reference_page.schema.json".to_owned(),
            content_digest_ref: "digest-ref:docs-reference:9001".to_owned(),
        },
        provenance: ProvenanceSubDescriptor {
            source_class: ProvenanceClass::NotProvided,
            signature_state: SignatureState::NotProvided,
        },
        freshness: FreshnessSubDescriptor {
            freshness_state: FreshnessState::Missing,
            evidence_state: EvidenceState::NotProvided,
        },
        qualification: QualificationSubDescriptor {
            support_class: QualificationClass::Beta,
            evidence_state: EvidenceState::RetestPending,
        },
        client_scope: ClientScopeSubDescriptor {
            client_kind: ClientScope::BrowserReference,
            authority_class: AuthorityClass::ReferenceOnly,
            handoff_requirement: HandoffRequirement::ConsoleHandoffRequired,
        },
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_MINTED_AT.to_owned(),
    })
}

/// The canonical descriptor-object registry: the three seed objects, the controlled
/// vocabulary, the consumer set, and the conformance review.
pub fn seeded_m5_descriptor_object_registry() -> M5DescriptorObjectRegistry {
    M5DescriptorObjectRegistry::new(M5DescriptorObjectRegistryInput {
        registry_id: M5_DESCRIPTOR_OBJECT_REGISTRY_ID.to_owned(),
        report_label: "M5 public-truth descriptor objects".to_owned(),
        objects: vec![
            seeded_stable_descriptor_object(),
            seeded_narrowed_descriptor_object(),
            seeded_not_provided_descriptor_object(),
        ],
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_MINTED_AT.to_owned(),
    })
}
