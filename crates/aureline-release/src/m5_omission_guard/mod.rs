//! One weaker-evidence-state vocabulary and the no-silent-omission rule every public-truth
//! consumer must obey.
//!
//! The [descriptor object](crate::m5_descriptor_object) lane freezes the typed provenance /
//! freshness / qualification / client-scope state a claimed M5 artifact carries, the
//! [claim-narrowing](crate::m5_claim_narrowing) lane derives the one degraded-claim state that
//! condition implies, and the [descriptor-join](crate::m5_descriptor_join) lane proves the truth
//! survives copy/export. This lane fixes the remaining failure mode: a consumer that *omits* a
//! weaker state — quietly dropping a Mirrored / Offline / Side-loaded origin, a `not_provided`
//! value, or partial / stale evidence — so a narrowed surface reads cleaner than it is.
//!
//! It does two things. First, it freezes a single [`WeakerEvidenceState`] vocabulary: every
//! negative, partial, or non-authoritative origin/evidence condition resolves to exactly one
//! stable token, one user-facing label, and one explanation message id, so the state reads
//! identically wherever it is surfaced — release center, Help/About, marketplace, docs/help,
//! certification, evaluation packs, support exports, and companion handoffs. The authoritative
//! `Official` origin is part of that same vocabulary: the absence of weakening is *stated*, never
//! left blank. Second, it derives — never hand-authors — the set of present states from a
//! descriptor's own facets and projects that exact set onto every [`PublicTruthConsumer`]. The
//! [`OmissionGuardReview`] is the rule: a consumer can neither drop a present state (silent
//! omission) nor invent an absent one, and the per-state label and explanation must be identical
//! across every consumer. Because the present set is always non-empty and is keyed off the same
//! facets the claim-narrowing runtime reads, weakening is present on a surface *exactly* when the
//! shared claim is narrowed — mirror / offline / side-loaded / not-provided states can never
//! disappear when inconvenient.
//!
//! The [`M5OmissionGuardRegistry`] is the one inspectable, serde-serializable truth packet every
//! consumer reads; it carries metadata and refs only — no credential bodies or raw provider
//! payloads.
//!
//! - Packet schema:
//!   [`schemas/provenance/m5-omission-guard.schema.json`](../../../../../schemas/provenance/m5-omission-guard.schema.json)
//! - Contract doc:
//!   [`docs/public-truth/m5-omission-guard.md`](../../../../../docs/public-truth/m5-omission-guard.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_community_limited_case, seeded_m5_omission_guard_registry, seeded_mirrored_case,
    seeded_not_provided_blocked_case, seeded_official_case, seeded_offline_case,
    seeded_partial_evidence_case, seeded_scoped_client_case, seeded_side_loaded_case,
    seeded_stale_case, M5_OMISSION_GUARD_REGISTRY_ID,
};

use serde::{Deserialize, Serialize};

use crate::m5_claim_narrowing::{ClaimNarrowingCase, NarrowedClaimState};
use crate::m5_descriptor_badge::{PublicTruthConsumer, M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX};
use crate::m5_descriptor_object::{
    ArtifactBinding, AuthorityClass, DescriptorFacet, DescriptorObject, EvidenceState,
    FreshnessState, HandoffRequirement, ProvenanceClass, SignatureState,
};

/// Record-kind tag carried by an [`OmissionGuardCase`].
pub const M5_OMISSION_GUARD_RECORD_KIND: &str = "m5_omission_guard_case";

/// Record-kind tag carried by [`M5OmissionGuardRegistry`].
pub const M5_OMISSION_GUARD_REGISTRY_RECORD_KIND: &str = "m5_omission_guard_registry";

/// Schema version for the omission-guard case and registry.
pub const M5_OMISSION_GUARD_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the omission-guard schema.
pub const M5_OMISSION_GUARD_SCHEMA_REF: &str = "schemas/provenance/m5-omission-guard.schema.json";

/// Repo-relative path of the published omission-guard registry inventory.
pub const M5_OMISSION_GUARD_REGISTRY_REF: &str = "artifacts/public-truth/m5-omission-guard.json";

/// Repo-relative path of the release-grade omission-guard parity proof.
pub const M5_OMISSION_GUARD_PROOF_REF: &str =
    "artifacts/release/m5-descriptor-parity-proof/omission-guard.json";

/// Repo-relative path of the omission-guard contract doc.
pub const M5_OMISSION_GUARD_DOC_REF: &str = "docs/public-truth/m5-omission-guard.md";

/// Repo-relative directory of the omission-guard consumer fixtures.
pub const M5_OMISSION_GUARD_FIXTURE_DIR: &str = "fixtures/public-truth/m5-badge-consumers/";

/// One controlled weaker-evidence state a claimed M5 surface can carry. Each value is the single
/// vocabulary entry for a negative, partial, or non-authoritative origin/evidence condition the
/// no-silent-omission rule forbids any consumer from dropping. [`Official`](Self::Official) is the
/// authoritative anchor — present when the origin is first-party signed — so a clean surface still
/// *states* that it is official rather than rendering nothing. Every other value is weakening.
/// Declaration order is the canonical render order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WeakerEvidenceState {
    /// First-party signed origin — the authoritative anchor; not a weakening.
    Official,
    /// A vendor-distributed origin.
    Vendor,
    /// A community-distributed origin.
    Community,
    /// A mirror of an upstream origin.
    Mirrored,
    /// An offline bundle, distributed out of band.
    Offline,
    /// A side-loaded artifact, installed outside the governed channel.
    SideLoaded,
    /// A signature that is present but not verified / attestation-only / unsigned / invalid.
    Unverified,
    /// Evidence is partial or limited in scope.
    Partial,
    /// Evidence exists but a retest is pending before it can be relied on.
    RetestPending,
    /// Freshness or evidence has gone stale and needs refreshing.
    Stale,
    /// A freshness window has expired.
    Expired,
    /// Required freshness evidence is missing.
    Missing,
    /// The client scope cannot carry full desktop authority.
    ScopedClient,
    /// The action requires a handoff to a more capable client.
    HandoffRequired,
    /// A required origin, signature, evidence, or authority value was not provided.
    NotProvided,
}

impl WeakerEvidenceState {
    /// Every state, in canonical render order.
    pub const ALL: [Self; 15] = [
        Self::Official,
        Self::Vendor,
        Self::Community,
        Self::Mirrored,
        Self::Offline,
        Self::SideLoaded,
        Self::Unverified,
        Self::Partial,
        Self::RetestPending,
        Self::Stale,
        Self::Expired,
        Self::Missing,
        Self::ScopedClient,
        Self::HandoffRequired,
        Self::NotProvided,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Official => "official",
            Self::Vendor => "vendor",
            Self::Community => "community",
            Self::Mirrored => "mirrored",
            Self::Offline => "offline",
            Self::SideLoaded => "side_loaded",
            Self::Unverified => "unverified",
            Self::Partial => "partial",
            Self::RetestPending => "retest_pending",
            Self::Stale => "stale",
            Self::Expired => "expired",
            Self::Missing => "missing",
            Self::ScopedClient => "scoped_client",
            Self::HandoffRequired => "handoff_required",
            Self::NotProvided => "not_provided",
        }
    }

    /// The one user-facing label this state renders as, identical across every consumer.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Official => "Official",
            Self::Vendor => "Vendor",
            Self::Community => "Community",
            Self::Mirrored => "Mirrored",
            Self::Offline => "Offline",
            Self::SideLoaded => "Side-loaded",
            Self::Unverified => "Unverified",
            Self::Partial => "Partial",
            Self::RetestPending => "Retest pending",
            Self::Stale => "Stale",
            Self::Expired => "Expired",
            Self::Missing => "Missing",
            Self::ScopedClient => "Scoped client",
            Self::HandoffRequired => "Handoff required",
            Self::NotProvided => "Not provided",
        }
    }

    /// True when this state represents weakened evidence — everything except
    /// [`Official`](Self::Official).
    pub const fn is_weakening(self) -> bool {
        !matches!(self, Self::Official)
    }

    /// The facet:token pairs in `descriptor` that put this state in the present set. An empty
    /// result means the state is absent. Naming the sources is what keeps the derivation auditable.
    fn sources(self, descriptor: &DescriptorObject) -> Vec<String> {
        let provenance = &descriptor.provenance;
        let freshness = &descriptor.freshness;
        let qualification = &descriptor.qualification;
        let client = &descriptor.client_scope;
        let mut out = Vec::new();
        let mut push = |facet: DescriptorFacet, token: &str| out.push(facet_token(facet, token));
        match self {
            Self::Official => {
                if provenance.source_class.is_authoritative() {
                    push(
                        DescriptorFacet::SourceClass,
                        provenance.source_class.as_str(),
                    );
                }
            }
            Self::Vendor => {
                if matches!(provenance.source_class, ProvenanceClass::Vendor) {
                    push(
                        DescriptorFacet::SourceClass,
                        provenance.source_class.as_str(),
                    );
                }
            }
            Self::Community => {
                if matches!(provenance.source_class, ProvenanceClass::Community) {
                    push(
                        DescriptorFacet::SourceClass,
                        provenance.source_class.as_str(),
                    );
                }
            }
            Self::Mirrored => {
                if matches!(provenance.source_class, ProvenanceClass::Mirror) {
                    push(
                        DescriptorFacet::SourceClass,
                        provenance.source_class.as_str(),
                    );
                }
            }
            Self::Offline => {
                if matches!(provenance.source_class, ProvenanceClass::OfflineBundle) {
                    push(
                        DescriptorFacet::SourceClass,
                        provenance.source_class.as_str(),
                    );
                }
            }
            Self::SideLoaded => {
                if matches!(provenance.source_class, ProvenanceClass::SideLoaded) {
                    push(
                        DescriptorFacet::SourceClass,
                        provenance.source_class.as_str(),
                    );
                }
            }
            Self::Unverified => {
                if matches!(
                    provenance.signature_state,
                    SignatureState::SignedUnverified
                        | SignatureState::AttestationOnly
                        | SignatureState::Unsigned
                        | SignatureState::SignatureInvalid
                ) {
                    push(
                        DescriptorFacet::SignatureState,
                        provenance.signature_state.as_str(),
                    );
                }
            }
            Self::Partial => {
                if matches!(
                    freshness.evidence_state,
                    EvidenceState::Limited | EvidenceState::Partial
                ) {
                    push(
                        DescriptorFacet::FreshnessEvidence,
                        freshness.evidence_state.as_str(),
                    );
                }
                if matches!(
                    qualification.evidence_state,
                    EvidenceState::Limited | EvidenceState::Partial
                ) {
                    push(
                        DescriptorFacet::QualificationEvidence,
                        qualification.evidence_state.as_str(),
                    );
                }
            }
            Self::RetestPending => {
                if matches!(freshness.evidence_state, EvidenceState::RetestPending) {
                    push(
                        DescriptorFacet::FreshnessEvidence,
                        freshness.evidence_state.as_str(),
                    );
                }
                if matches!(qualification.evidence_state, EvidenceState::RetestPending) {
                    push(
                        DescriptorFacet::QualificationEvidence,
                        qualification.evidence_state.as_str(),
                    );
                }
            }
            Self::Stale => {
                if matches!(freshness.freshness_state, FreshnessState::Stale) {
                    push(
                        DescriptorFacet::FreshnessState,
                        freshness.freshness_state.as_str(),
                    );
                }
                if matches!(freshness.evidence_state, EvidenceState::EvidenceStale) {
                    push(
                        DescriptorFacet::FreshnessEvidence,
                        freshness.evidence_state.as_str(),
                    );
                }
                if matches!(qualification.evidence_state, EvidenceState::EvidenceStale) {
                    push(
                        DescriptorFacet::QualificationEvidence,
                        qualification.evidence_state.as_str(),
                    );
                }
            }
            Self::Expired => {
                if matches!(freshness.freshness_state, FreshnessState::Expired) {
                    push(
                        DescriptorFacet::FreshnessState,
                        freshness.freshness_state.as_str(),
                    );
                }
            }
            Self::Missing => {
                if matches!(freshness.freshness_state, FreshnessState::Missing) {
                    push(
                        DescriptorFacet::FreshnessState,
                        freshness.freshness_state.as_str(),
                    );
                }
            }
            Self::ScopedClient => {
                if !client.client_kind.is_full_authority() {
                    push(DescriptorFacet::ClientKind, client.client_kind.as_str());
                }
                if matches!(
                    client.authority_class,
                    AuthorityClass::ScopedAuthority
                        | AuthorityClass::ReferenceOnly
                        | AuthorityClass::HandoffOnly
                ) {
                    push(
                        DescriptorFacet::AuthorityClass,
                        client.authority_class.as_str(),
                    );
                }
            }
            Self::HandoffRequired => {
                if matches!(
                    client.handoff_requirement,
                    HandoffRequirement::DesktopHandoffRequired
                        | HandoffRequirement::ConsoleHandoffRequired
                ) {
                    push(
                        DescriptorFacet::HandoffRequirement,
                        client.handoff_requirement.as_str(),
                    );
                }
            }
            Self::NotProvided => {
                if matches!(provenance.source_class, ProvenanceClass::NotProvided) {
                    push(
                        DescriptorFacet::SourceClass,
                        provenance.source_class.as_str(),
                    );
                }
                if matches!(provenance.signature_state, SignatureState::NotProvided) {
                    push(
                        DescriptorFacet::SignatureState,
                        provenance.signature_state.as_str(),
                    );
                }
                if matches!(freshness.evidence_state, EvidenceState::NotProvided) {
                    push(
                        DescriptorFacet::FreshnessEvidence,
                        freshness.evidence_state.as_str(),
                    );
                }
                if matches!(qualification.evidence_state, EvidenceState::NotProvided) {
                    push(
                        DescriptorFacet::QualificationEvidence,
                        qualification.evidence_state.as_str(),
                    );
                }
                if matches!(client.authority_class, AuthorityClass::NotProvided) {
                    push(
                        DescriptorFacet::AuthorityClass,
                        client.authority_class.as_str(),
                    );
                }
                if matches!(client.handoff_requirement, HandoffRequirement::NotProvided) {
                    push(
                        DescriptorFacet::HandoffRequirement,
                        client.handoff_requirement.as_str(),
                    );
                }
            }
        }
        out
    }
}

/// One present weaker-evidence state as it renders on a surface: the state, its single shared
/// label, whether it is a weakening, the facet:token sources that put it in the present set, and
/// the explanation message id every surface resolves it to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderedState {
    /// The controlled state.
    pub state: WeakerEvidenceState,
    /// The single user-facing label, identical across consumers.
    pub label: String,
    /// True when the state is a weakening (everything except `official`).
    pub is_weakening: bool,
    /// The descriptor facet:token pairs that put this state in the present set.
    pub sourced_from: Vec<String>,
    /// Stable explanation message id; prefixed [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub explanation_message_id: String,
}

impl RenderedState {
    /// Builds a rendered state from its controlled state and the facet:token sources behind it.
    fn new(state: WeakerEvidenceState, sourced_from: Vec<String>) -> Self {
        Self {
            state,
            label: state.label().to_owned(),
            is_weakening: state.is_weakening(),
            sourced_from,
            explanation_message_id: state_explanation_id(state),
        }
    }
}

/// One public-truth consumer's projection of the present weaker-evidence states. The
/// no-silent-omission rule requires [`rendered_states`](Self::rendered_states) to equal the case's
/// present set exactly — the consumer can neither drop a present state nor invent an absent one,
/// and every label and explanation must match the shared vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerStateProjection {
    /// The consumer rendering the states.
    pub consumer: PublicTruthConsumer,
    /// Reviewer-facing consumer label.
    pub consumer_label: String,
    /// Where on the surface the states render.
    pub surface_field: String,
    /// The present states this consumer renders, in canonical order.
    pub rendered_states: Vec<RenderedState>,
    /// True when this consumer omits no present state.
    pub omits_no_present_state: bool,
    /// Stable status message id; prefixed [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub status_message_id: String,
}

/// The per-case no-silent-omission review. Every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OmissionGuardReview {
    /// The present set is never empty — a surface always states at least its origin.
    pub present_set_non_empty: bool,
    /// The present set is derived from the descriptor, not hand-authored.
    pub present_set_derived_from_descriptor: bool,
    /// Every public-truth consumer is projected.
    pub all_consumers_projected: bool,
    /// No consumer drops a present state.
    pub no_consumer_omits_present_state: bool,
    /// No consumer invents a state that is not present.
    pub no_consumer_adds_absent_state: bool,
    /// Every consumer renders the shared label for each state.
    pub labels_identical_across_consumers: bool,
    /// Every consumer resolves the shared explanation for each state.
    pub explanations_identical_across_consumers: bool,
    /// Weakening is present exactly when the shared claim is narrowed.
    pub weakening_aligns_with_claim_narrowing: bool,
}

impl OmissionGuardReview {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.present_set_non_empty
            && self.present_set_derived_from_descriptor
            && self.all_consumers_projected
            && self.no_consumer_omits_present_state
            && self.no_consumer_adds_absent_state
            && self.labels_identical_across_consumers
            && self.explanations_identical_across_consumers
            && self.weakening_aligns_with_claim_narrowing
    }
}

/// One descriptor condition projected into the no-silent-omission guard: the canonical set of
/// present weaker-evidence states derived from the descriptor's facets, the per-consumer
/// projections that must each render that exact set, and the review that proves no consumer can
/// silently hide a present state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OmissionGuardCase {
    /// Record kind; must equal [`M5_OMISSION_GUARD_RECORD_KIND`].
    pub record_kind: String,
    /// Stable case id.
    pub case_id: String,
    /// Reviewer-facing case label.
    pub case_label: String,
    /// The descriptor identity this case projects.
    pub descriptor_id: String,
    /// Reviewer-facing descriptor label.
    pub descriptor_label: String,
    /// The typed artifact binding this case projects.
    pub artifact_ref: ArtifactBinding,
    /// The descriptor object this case reads — the full provenance / freshness / qualification /
    /// client-scope truth the present set is derived from.
    pub descriptor: DescriptorObject,
    /// The canonical present states, in render order, derived from the descriptor.
    pub present_states: Vec<RenderedState>,
    /// True when at least one present state is a weakening.
    pub weakening_present: bool,
    /// The current controlled claim state, derived from the shared claim-narrowing runtime.
    pub claim_state: NarrowedClaimState,
    /// Per-consumer projections, in [`PublicTruthConsumer::ALL`] order.
    pub consumer_projections: Vec<ConsumerStateProjection>,
    /// The per-case no-silent-omission review.
    pub guard: OmissionGuardReview,
    /// Stable message id for the case explanation drawer; prefixed
    /// [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub explanation_drawer_message_id: String,
}

impl OmissionGuardCase {
    /// Builds a case from a descriptor object, deriving the present states, the per-consumer
    /// projections, the claim state, and the review from the descriptor's own facets so the
    /// surfaces are always generated from one source rather than hand-authored.
    pub fn from_descriptor(case_id: &str, case_label: &str, descriptor: DescriptorObject) -> Self {
        let present_states = derive_present_states(&descriptor);
        let weakening_present = present_states.iter().any(|r| r.is_weakening);
        let claim_state = derive_claim_state(&descriptor);
        let consumer_projections = derive_consumer_projections(&present_states);
        let guard = derive_case_guard(
            &descriptor,
            &present_states,
            weakening_present,
            claim_state,
            &consumer_projections,
        );
        Self {
            record_kind: M5_OMISSION_GUARD_RECORD_KIND.to_owned(),
            case_id: case_id.to_owned(),
            case_label: case_label.to_owned(),
            descriptor_id: descriptor.descriptor_id.clone(),
            descriptor_label: descriptor.descriptor_label.clone(),
            artifact_ref: descriptor.artifact_ref.clone(),
            present_states,
            weakening_present,
            claim_state,
            consumer_projections,
            guard,
            explanation_drawer_message_id: format!(
                "{M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX}omission.drawer"
            ),
            descriptor,
        }
    }

    /// True when no weakening state is present — a fully-official surface.
    pub fn is_fully_official(&self) -> bool {
        !self.weakening_present
    }

    /// Deterministic export-safe JSON for the case.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only case fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("omission-guard case serializes")
    }

    /// Validates the case's invariants: the present set is derived from the descriptor and never
    /// empty, no consumer omits or invents a state, every label / explanation matches the shared
    /// vocabulary, weakening aligns with the shared claim runtime, message ids carry the lane
    /// prefix, and the export carries no raw material.
    pub fn validate(&self) -> Vec<M5OmissionGuardViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_OMISSION_GUARD_RECORD_KIND {
            out.push(M5OmissionGuardViolation::WrongRecordKind);
        }
        if self.case_id.trim().is_empty()
            || self.case_label.trim().is_empty()
            || self.descriptor_id.trim().is_empty()
            || self.descriptor_label.trim().is_empty()
        {
            out.push(M5OmissionGuardViolation::MissingIdentity);
        }

        // The embedded descriptor must itself be self-consistent.
        if !self.descriptor.validate().is_empty() {
            out.push(M5OmissionGuardViolation::DescriptorInvalid);
        }

        // The case key fields must mirror the embedded descriptor — identity and binding preserved.
        if self.descriptor_id != self.descriptor.descriptor_id
            || self.descriptor_label != self.descriptor.descriptor_label
            || self.artifact_ref != self.descriptor.artifact_ref
        {
            out.push(M5OmissionGuardViolation::DescriptorBindingMismatch);
        }

        // The present set must be derived from the descriptor, in canonical order, and non-empty.
        let expected_present = derive_present_states(&self.descriptor);
        if self.present_states != expected_present {
            out.push(M5OmissionGuardViolation::PresentStateDrift);
        }
        if self.present_states.is_empty() {
            out.push(M5OmissionGuardViolation::PresentSetEmpty);
        }
        for rendered in &self.present_states {
            if rendered.label != rendered.state.label()
                || rendered.is_weakening != rendered.state.is_weakening()
                || rendered.explanation_message_id != state_explanation_id(rendered.state)
            {
                out.push(M5OmissionGuardViolation::VocabularyDrift);
            }
        }
        if self.weakening_present != self.present_states.iter().any(|r| r.is_weakening) {
            out.push(M5OmissionGuardViolation::WeakeningFlagDrift);
        }

        // The claim state must be derived from the shared runtime and stay aligned with weakening.
        if self.claim_state != derive_claim_state(&self.descriptor) {
            out.push(M5OmissionGuardViolation::ClaimStateDrift);
        }
        if self.weakening_present == self.claim_state.is_fully_supported() {
            out.push(M5OmissionGuardViolation::ClaimAlignmentBroken);
        }

        // Every consumer is projected, in canonical order, and renders the exact present set.
        let projected: Vec<PublicTruthConsumer> = self
            .consumer_projections
            .iter()
            .map(|p| p.consumer)
            .collect();
        if projected != PublicTruthConsumer::ALL.to_vec() {
            out.push(M5OmissionGuardViolation::ConsumerSetMismatch);
        }
        for projection in &self.consumer_projections {
            // The core guard: a consumer must render every present state and no other.
            if omits_any(&self.present_states, &projection.rendered_states) {
                out.push(M5OmissionGuardViolation::SilentOmission);
            }
            if omits_any(&projection.rendered_states, &self.present_states) {
                out.push(M5OmissionGuardViolation::StateInvented);
            }
            if projection.omits_no_present_state
                == omits_any(&self.present_states, &projection.rendered_states)
            {
                out.push(M5OmissionGuardViolation::GuardReviewFailed);
            }
            for rendered in &projection.rendered_states {
                if rendered.label != rendered.state.label()
                    || rendered.explanation_message_id != state_explanation_id(rendered.state)
                {
                    out.push(M5OmissionGuardViolation::VocabularyDrift);
                }
            }
        }

        if self.consumer_projections != derive_consumer_projections(&self.present_states) {
            out.push(M5OmissionGuardViolation::ConsumerProjectionDrift);
        }

        let expected_guard = derive_case_guard(
            &self.descriptor,
            &self.present_states,
            self.weakening_present,
            self.claim_state,
            &self.consumer_projections,
        );
        if self.guard != expected_guard || !self.guard.all_hold() {
            out.push(M5OmissionGuardViolation::GuardReviewFailed);
        }

        if !message_ids_prefixed(self) {
            out.push(M5OmissionGuardViolation::UnprefixedMessageId);
        }
        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("omission-guard case serializes"),
        ) {
            out.push(M5OmissionGuardViolation::RawMaterialInExport);
        }
        out
    }
}

/// Builds a facet:token source string.
fn facet_token(facet: DescriptorFacet, token: &str) -> String {
    format!("{}:{}", facet.as_str(), token)
}

/// The stable explanation message id for a state.
fn state_explanation_id(state: WeakerEvidenceState) -> String {
    format!(
        "{M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX}omission.state.{}.explanation",
        state.as_str()
    )
}

/// Derives the present weaker-evidence states for a descriptor, in canonical order. A state is
/// present exactly when the descriptor's facets source it; the origin always sources exactly one
/// state, so the result is never empty.
fn derive_present_states(descriptor: &DescriptorObject) -> Vec<RenderedState> {
    WeakerEvidenceState::ALL
        .iter()
        .filter_map(|&state| {
            let sourced_from = state.sources(descriptor);
            if sourced_from.is_empty() {
                None
            } else {
                Some(RenderedState::new(state, sourced_from))
            }
        })
        .collect()
}

/// Derives the per-consumer projections — one per [`PublicTruthConsumer`], each rendering the same
/// present set with the same labels and explanations.
fn derive_consumer_projections(present: &[RenderedState]) -> Vec<ConsumerStateProjection> {
    PublicTruthConsumer::ALL
        .iter()
        .map(|&consumer| ConsumerStateProjection {
            consumer,
            consumer_label: consumer.label().to_owned(),
            surface_field: format!("{}::weaker_evidence_states", consumer.as_str()),
            rendered_states: present.to_vec(),
            omits_no_present_state: true,
            status_message_id: format!(
                "{M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX}omission.consumer.{}.status",
                consumer.as_str()
            ),
        })
        .collect()
}

/// Derives the per-case review from the descriptor, the present set, and the projections.
fn derive_case_guard(
    descriptor: &DescriptorObject,
    present: &[RenderedState],
    weakening_present: bool,
    claim_state: NarrowedClaimState,
    projections: &[ConsumerStateProjection],
) -> OmissionGuardReview {
    let projected: Vec<PublicTruthConsumer> = projections.iter().map(|p| p.consumer).collect();
    OmissionGuardReview {
        present_set_non_empty: !present.is_empty(),
        present_set_derived_from_descriptor: present == derive_present_states(descriptor),
        all_consumers_projected: projected == PublicTruthConsumer::ALL.to_vec(),
        no_consumer_omits_present_state: projections
            .iter()
            .all(|p| !omits_any(present, &p.rendered_states)),
        no_consumer_adds_absent_state: projections
            .iter()
            .all(|p| !omits_any(&p.rendered_states, present)),
        labels_identical_across_consumers: projections
            .iter()
            .all(|p| p.rendered_states.iter().all(|r| r.label == r.state.label())),
        explanations_identical_across_consumers: projections.iter().all(|p| {
            p.rendered_states
                .iter()
                .all(|r| r.explanation_message_id == state_explanation_id(r.state))
        }),
        weakening_aligns_with_claim_narrowing: weakening_present
            != claim_state.is_fully_supported(),
    }
}

/// True when `subset` contains a state that `superset` does not — i.e. `superset` omits a state.
fn omits_any(subset: &[RenderedState], superset: &[RenderedState]) -> bool {
    subset
        .iter()
        .any(|s| !superset.iter().any(|o| o.state == s.state))
}

/// Derives the current claim state from the shared claim-narrowing runtime so the guard can never
/// disagree with the interactive consumers about whether a surface is narrowed.
fn derive_claim_state(descriptor: &DescriptorObject) -> NarrowedClaimState {
    ClaimNarrowingCase::from_descriptor(
        "omission-guard:derivation",
        "Omission-guard derivation",
        descriptor.clone(),
    )
    .canonical_claim_state
}

/// True when every message id the case carries is prefixed with the lane prefix.
fn message_ids_prefixed(case: &OmissionGuardCase) -> bool {
    let prefixed = |s: &str| s.starts_with(M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX);
    prefixed(&case.explanation_drawer_message_id)
        && case
            .present_states
            .iter()
            .all(|r| prefixed(&r.explanation_message_id))
        && case.consumer_projections.iter().all(|p| {
            prefixed(&p.status_message_id)
                && p.rendered_states
                    .iter()
                    .all(|r| prefixed(&r.explanation_message_id))
        })
}

/// One frozen entry in the shared weaker-evidence-state vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OmissionStateEntry {
    /// Stable state token.
    pub state: String,
    /// The single user-facing label.
    pub label: String,
    /// True when the state is a weakening.
    pub is_weakening: bool,
    /// Stable explanation message id.
    pub explanation_message_id: String,
}

/// Self-describing controlled-vocabulary set so the registry resolves every token a case carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OmissionGuardVocabulary {
    /// Every weaker-evidence state with its label and explanation id.
    pub states: Vec<OmissionStateEntry>,
    /// Consumer tokens.
    pub consumers: Vec<String>,
    /// Claim-state tokens.
    pub claim_states: Vec<String>,
    /// Descriptor-facet tokens.
    pub facets: Vec<String>,
}

impl OmissionGuardVocabulary {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            states: WeakerEvidenceState::ALL
                .iter()
                .map(|&s| OmissionStateEntry {
                    state: s.as_str().to_owned(),
                    label: s.label().to_owned(),
                    is_weakening: s.is_weakening(),
                    explanation_message_id: state_explanation_id(s),
                })
                .collect(),
            consumers: PublicTruthConsumer::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            claim_states: NarrowedClaimState::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            facets: DescriptorFacet::ALL
                .iter()
                .map(|f| f.as_str().to_owned())
                .collect(),
        }
    }

    /// True when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Conformance review for the omission-guard registry. Every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OmissionGuardConformance {
    /// Every case is self-consistent.
    pub cases_validate: bool,
    /// Every present set is derived from its descriptor, never hand-authored.
    pub present_sets_derived_from_descriptors: bool,
    /// Every consumer renders the same labels and explanations — one vocabulary everywhere.
    pub one_vocabulary_across_consumers: bool,
    /// No consumer silently omits a present state, anywhere.
    pub no_consumer_silently_omits: bool,
    /// Mirror / offline / side-loaded origins are first-class: each is exercised and surfaced.
    pub mirror_offline_side_loaded_first_class: bool,
    /// A `not_provided` value is never hidden: surfaced wherever present, and exercised.
    pub not_provided_never_hidden: bool,
    /// Partial / limited evidence is surfaced, and exercised.
    pub partial_states_surfaced: bool,
    /// The authoritative origin still renders an explicit `official` anchor.
    pub official_anchor_explicit: bool,
    /// Weakening is present exactly when the shared claim is narrowed.
    pub weakening_aligns_with_claim_narrowing: bool,
    /// The controlled vocabularies match the canonical frozen tokens.
    pub controlled_enums_frozen: bool,
    /// Every public-truth consumer reads this one guard runtime.
    pub shared_across_consumers: bool,
    /// The export carries no raw provider material.
    pub export_carries_no_raw_material: bool,
}

impl OmissionGuardConformance {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.cases_validate
            && self.present_sets_derived_from_descriptors
            && self.one_vocabulary_across_consumers
            && self.no_consumer_silently_omits
            && self.mirror_offline_side_loaded_first_class
            && self.not_provided_never_hidden
            && self.partial_states_surfaced
            && self.official_anchor_explicit
            && self.weakening_aligns_with_claim_narrowing
            && self.controlled_enums_frozen
            && self.shared_across_consumers
            && self.export_carries_no_raw_material
    }
}

/// Roll-up counts over the registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OmissionGuardSummary {
    /// Total cases.
    pub total_cases: u32,
    /// Cases that carry at least one weakening state.
    pub cases_with_weakening: u32,
    /// Cases that stand fully official (no weakening).
    pub fully_official_cases: u32,
    /// Total state renderings across every case and consumer.
    pub total_state_renderings: u32,
    /// Distinct weaker-evidence states exercised across every case.
    pub distinct_states_exercised: u32,
}

/// Constructor input for [`M5OmissionGuardRegistry::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5OmissionGuardRegistryInput {
    /// Stable registry id.
    pub registry_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The omission-guard cases this registry publishes.
    pub cases: Vec<OmissionGuardCase>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// The one inspectable, serde-serializable omission-guard truth packet every consumer reads: the
/// cases, the shared weaker-evidence-state vocabulary, the consumers that read the runtime, a
/// conformance review, and a roll-up summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OmissionGuardRegistry {
    /// Record kind; must equal [`M5_OMISSION_GUARD_REGISTRY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_OMISSION_GUARD_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable registry id.
    pub registry_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The omission-guard cases this registry publishes.
    pub cases: Vec<OmissionGuardCase>,
    /// The controlled vocabulary every case shares.
    pub vocabulary: OmissionGuardVocabulary,
    /// The public-truth consumers that read this guard runtime.
    pub consumers: Vec<String>,
    /// Conformance review block.
    pub conformance: OmissionGuardConformance,
    /// Roll-up counts.
    pub summary: OmissionGuardSummary,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5OmissionGuardRegistry {
    /// Builds a registry from its cases, deriving the vocabulary, consumer list, conformance
    /// review, and summary.
    pub fn new(input: M5OmissionGuardRegistryInput) -> Self {
        let cases = input.cases;
        let consumers: Vec<String> = PublicTruthConsumer::ALL
            .iter()
            .map(|c| c.as_str().to_owned())
            .collect();
        let conformance = derive_registry_conformance(&cases);
        let summary = derive_summary(&cases);
        Self {
            record_kind: M5_OMISSION_GUARD_REGISTRY_RECORD_KIND.to_owned(),
            schema_version: M5_OMISSION_GUARD_SCHEMA_VERSION,
            registry_id: input.registry_id,
            report_label: input.report_label,
            cases,
            vocabulary: OmissionGuardVocabulary::canonical(),
            consumers,
            conformance,
            summary,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Finds a case by id.
    pub fn case(&self, case_id: &str) -> Option<&OmissionGuardCase> {
        self.cases.iter().find(|c| c.case_id == case_id)
    }

    /// Deterministic export-safe JSON for the registry.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("omission-guard registry serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 no-silent-omission guard parity\n\n");
        out.push_str(&format!("- Registry: `{}`\n", self.registry_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Cases: {}\n", self.cases.len()));
        out.push_str(&format!("- Minted: `{}`\n", self.minted_at));
        out.push_str(
            "- One vocabulary across: release center, Help/About, marketplace, docs/help, certification, evaluation packs, support, companion\n",
        );

        out.push_str("\n## Vocabulary\n\n");
        out.push_str("| State | Label | Weakening |\n");
        out.push_str("|-------|-------|-----------|\n");
        for entry in &self.vocabulary.states {
            out.push_str(&format!(
                "| `{}` | {} | {} |\n",
                entry.state,
                entry.label,
                if entry.is_weakening { "yes" } else { "anchor" }
            ));
        }

        out.push_str("\n## Cases\n\n");
        out.push_str("| Case | Descriptor | Present states | Claim state | Weakening |\n");
        out.push_str("|------|------------|----------------|-------------|-----------|\n");
        for case in &self.cases {
            let states: Vec<&str> = case
                .present_states
                .iter()
                .map(|r| r.state.as_str())
                .collect();
            out.push_str(&format!(
                "| `{}` | `{}` | {} | `{}` | {} |\n",
                case.case_id,
                case.descriptor_id,
                states.join(", "),
                case.claim_state.as_str(),
                if case.weakening_present { "yes" } else { "no" }
            ));
        }

        out.push_str("\n## Consumer parity\n\n");
        for case in &self.cases {
            out.push_str(&format!("### `{}`\n\n", case.case_id));
            out.push_str("Present states (identical on every consumer):\n\n");
            for rendered in &case.present_states {
                out.push_str(&format!(
                    "- `{}` ({}) — from {}\n",
                    rendered.state.as_str(),
                    rendered.label,
                    rendered.sourced_from.join(", ")
                ));
            }
            out.push_str("\n| Consumer | States rendered | Omits none |\n");
            out.push_str("|----------|-----------------|------------|\n");
            for projection in &case.consumer_projections {
                out.push_str(&format!(
                    "| `{}` | {} | {} |\n",
                    projection.consumer.as_str(),
                    projection.rendered_states.len(),
                    if projection.omits_no_present_state {
                        "yes"
                    } else {
                        "NO"
                    }
                ));
            }
            out.push('\n');
        }
        out
    }

    /// Validates the registry's invariants.
    pub fn validate(&self) -> Vec<M5OmissionGuardViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_OMISSION_GUARD_REGISTRY_RECORD_KIND {
            out.push(M5OmissionGuardViolation::WrongRecordKind);
        }
        if self.schema_version != M5_OMISSION_GUARD_SCHEMA_VERSION {
            out.push(M5OmissionGuardViolation::WrongSchemaVersion);
        }
        if self.registry_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            out.push(M5OmissionGuardViolation::MissingIdentity);
        }
        if self.cases.is_empty() {
            out.push(M5OmissionGuardViolation::RegistryHasNoCases);
        }
        let mut seen = std::collections::BTreeSet::new();
        for case in &self.cases {
            if !seen.insert(case.case_id.clone()) {
                out.push(M5OmissionGuardViolation::DuplicateCaseId);
            }
            out.extend(case.validate());
        }
        if !self.vocabulary.matches_canonical() {
            out.push(M5OmissionGuardViolation::VocabularyMismatch);
        }
        let expected_consumers: Vec<String> = PublicTruthConsumer::ALL
            .iter()
            .map(|c| c.as_str().to_owned())
            .collect();
        if self.consumers != expected_consumers {
            out.push(M5OmissionGuardViolation::ConsumerSetMismatch);
        }
        if self.conformance != derive_registry_conformance(&self.cases)
            || !self.conformance.all_hold()
        {
            out.push(M5OmissionGuardViolation::ConformanceReviewFailed);
        }
        if self.summary != derive_summary(&self.cases) {
            out.push(M5OmissionGuardViolation::SummaryMismatch);
        }
        out
    }
}

/// Derives the roll-up summary from the cases.
fn derive_summary(cases: &[OmissionGuardCase]) -> OmissionGuardSummary {
    let mut distinct = std::collections::BTreeSet::new();
    for case in cases {
        for rendered in &case.present_states {
            distinct.insert(rendered.state.as_str());
        }
    }
    OmissionGuardSummary {
        total_cases: cases.len() as u32,
        cases_with_weakening: cases.iter().filter(|c| c.weakening_present).count() as u32,
        fully_official_cases: cases.iter().filter(|c| c.is_fully_official()).count() as u32,
        total_state_renderings: cases
            .iter()
            .map(|c| {
                c.consumer_projections
                    .iter()
                    .map(|p| p.rendered_states.len() as u32)
                    .sum::<u32>()
            })
            .sum(),
        distinct_states_exercised: distinct.len() as u32,
    }
}

/// True when a descriptor carries a `not_provided` value on any guarded facet.
fn descriptor_has_not_provided(descriptor: &DescriptorObject) -> bool {
    matches!(
        descriptor.provenance.source_class,
        ProvenanceClass::NotProvided
    ) || matches!(
        descriptor.provenance.signature_state,
        SignatureState::NotProvided
    ) || matches!(
        descriptor.freshness.evidence_state,
        EvidenceState::NotProvided
    ) || matches!(
        descriptor.qualification.evidence_state,
        EvidenceState::NotProvided
    ) || matches!(
        descriptor.client_scope.authority_class,
        AuthorityClass::NotProvided
    ) || matches!(
        descriptor.client_scope.handoff_requirement,
        HandoffRequirement::NotProvided
    )
}

/// True when a case's present set contains a given state.
fn case_has_state(case: &OmissionGuardCase, state: WeakerEvidenceState) -> bool {
    case.present_states.iter().any(|r| r.state == state)
}

/// Derives the registry conformance review from its cases.
fn derive_registry_conformance(cases: &[OmissionGuardCase]) -> OmissionGuardConformance {
    let cases_validate = !cases.is_empty() && cases.iter().all(|c| c.validate().is_empty());

    let present_sets_derived = cases
        .iter()
        .all(|c| c.present_states == derive_present_states(&c.descriptor));

    let one_vocabulary = cases.iter().all(|c| {
        let projected: Vec<PublicTruthConsumer> =
            c.consumer_projections.iter().map(|p| p.consumer).collect();
        projected == PublicTruthConsumer::ALL.to_vec()
            && c.consumer_projections.iter().all(|p| {
                p.consumer_label == p.consumer.label()
                    && p.rendered_states.iter().all(|r| {
                        r.label == r.state.label()
                            && r.explanation_message_id == state_explanation_id(r.state)
                    })
            })
    });

    let no_silent_omission = cases.iter().all(|c| {
        !c.present_states.is_empty()
            && c.consumer_projections
                .iter()
                .all(|p| !omits_any(&c.present_states, &p.rendered_states))
    });

    // Mirror / offline / side-loaded must each be exercised and surface wherever present.
    let mirror_offline_side_loaded_first_class = cases
        .iter()
        .any(|c| case_has_state(c, WeakerEvidenceState::Mirrored))
        && cases
            .iter()
            .any(|c| case_has_state(c, WeakerEvidenceState::Offline))
        && cases
            .iter()
            .any(|c| case_has_state(c, WeakerEvidenceState::SideLoaded));

    // A not_provided value must surface as the NotProvided state, and the registry must exercise it.
    let not_provided_never_hidden = cases.iter().all(|c| {
        !descriptor_has_not_provided(&c.descriptor)
            || case_has_state(c, WeakerEvidenceState::NotProvided)
    }) && cases
        .iter()
        .any(|c| case_has_state(c, WeakerEvidenceState::NotProvided));

    let partial_states_surfaced = cases
        .iter()
        .any(|c| case_has_state(c, WeakerEvidenceState::Partial));

    // The authoritative origin must render an explicit official anchor, and a fully-official case
    // must exist so the anchor is exercised without any weakening masking it.
    let official_anchor_explicit = cases.iter().all(|c| {
        !c.descriptor.provenance.source_class.is_authoritative()
            || case_has_state(c, WeakerEvidenceState::Official)
    }) && cases.iter().any(|c| c.is_fully_official());

    let weakening_aligns = cases
        .iter()
        .all(|c| c.weakening_present != c.claim_state.is_fully_supported());

    let export_clean = cases.iter().all(|c| {
        !json_contains_forbidden_material(
            &serde_json::to_value(c).expect("omission-guard case serializes"),
        )
    });

    OmissionGuardConformance {
        cases_validate,
        present_sets_derived_from_descriptors: present_sets_derived,
        one_vocabulary_across_consumers: one_vocabulary,
        no_consumer_silently_omits: no_silent_omission,
        mirror_offline_side_loaded_first_class,
        not_provided_never_hidden,
        partial_states_surfaced,
        official_anchor_explicit,
        weakening_aligns_with_claim_narrowing: weakening_aligns,
        controlled_enums_frozen: OmissionGuardVocabulary::canonical().matches_canonical(),
        shared_across_consumers: true,
        export_carries_no_raw_material: export_clean,
    }
}

/// Validation failures for the omission-guard lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OmissionGuardViolation {
    /// The record kind is wrong.
    WrongRecordKind,
    /// The schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is empty.
    MissingIdentity,
    /// The embedded descriptor object is itself invalid.
    DescriptorInvalid,
    /// The case key fields disagree with the embedded descriptor's identity or binding.
    DescriptorBindingMismatch,
    /// The present set drifted from a fresh derivation.
    PresentStateDrift,
    /// The present set is empty — a surface would render nothing.
    PresentSetEmpty,
    /// A rendered state's label, weakening flag, or explanation drifted from the vocabulary.
    VocabularyDrift,
    /// The weakening flag drifted from the present set.
    WeakeningFlagDrift,
    /// The claim state drifted from the shared claim-narrowing runtime.
    ClaimStateDrift,
    /// Weakening presence and the shared claim state disagree.
    ClaimAlignmentBroken,
    /// A consumer dropped a present state — a silent omission.
    SilentOmission,
    /// A consumer rendered a state that is not present.
    StateInvented,
    /// The per-consumer projections drifted from a fresh derivation.
    ConsumerProjectionDrift,
    /// The consumer set does not match the canonical consumers.
    ConsumerSetMismatch,
    /// A message id is missing the lane prefix.
    UnprefixedMessageId,
    /// The registry publishes no cases.
    RegistryHasNoCases,
    /// Two cases share a case id.
    DuplicateCaseId,
    /// The controlled-vocabulary set does not match the canonical tokens.
    VocabularyMismatch,
    /// A conformance-review flag does not hold or drifted.
    ConformanceReviewFailed,
    /// A per-case guard flag does not hold or drifted.
    GuardReviewFailed,
    /// The summary did not match the computed roll-up.
    SummaryMismatch,
    /// The export contains raw provider material.
    RawMaterialInExport,
}

impl M5OmissionGuardViolation {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::DescriptorInvalid => "descriptor_invalid",
            Self::DescriptorBindingMismatch => "descriptor_binding_mismatch",
            Self::PresentStateDrift => "present_state_drift",
            Self::PresentSetEmpty => "present_set_empty",
            Self::VocabularyDrift => "vocabulary_drift",
            Self::WeakeningFlagDrift => "weakening_flag_drift",
            Self::ClaimStateDrift => "claim_state_drift",
            Self::ClaimAlignmentBroken => "claim_alignment_broken",
            Self::SilentOmission => "silent_omission",
            Self::StateInvented => "state_invented",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::ConsumerSetMismatch => "consumer_set_mismatch",
            Self::UnprefixedMessageId => "unprefixed_message_id",
            Self::RegistryHasNoCases => "registry_has_no_cases",
            Self::DuplicateCaseId => "duplicate_case_id",
            Self::VocabularyMismatch => "vocabulary_mismatch",
            Self::ConformanceReviewFailed => "conformance_review_failed",
            Self::GuardReviewFailed => "guard_review_failed",
            Self::SummaryMismatch => "summary_mismatch",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Keys whose presence would mean an export leaked raw material. Mirrors the redaction posture of
/// the upstream descriptor lanes.
const FORBIDDEN_KEY_SUBSTRINGS: [&str; 6] = [
    "credential",
    "secret",
    "password",
    "api_key",
    "raw_payload",
    "bearer_token",
];

/// Scans a serialized value for forbidden material. Returns true when a key (case-insensitive)
/// contains a forbidden substring.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Object(map) => map.iter().any(|(key, child)| {
            let lower = key.to_ascii_lowercase();
            FORBIDDEN_KEY_SUBSTRINGS
                .iter()
                .any(|needle| lower.contains(needle))
                || json_contains_forbidden_material(child)
        }),
        serde_json::Value::Array(items) => items.iter().any(json_contains_forbidden_material),
        _ => false,
    }
}
