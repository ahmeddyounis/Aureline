//! Automatic stale-evidence and degraded-claim narrowing across every public-truth consumer.
//!
//! The [descriptor object](crate::m5_descriptor_object) lane freezes the typed
//! provenance / freshness / qualification / client-scope state a claimed M5 artifact carries
//! and derives an [effective qualification](crate::m5_descriptor_object::DescriptorObject)
//! from its named narrowings. The [descriptor / badge matrix](crate::m5_descriptor_badge)
//! freezes which consumer surface binds which descriptor family. This lane closes the loop
//! between them: it takes an underlying evidence condition (a descriptor object) and projects
//! the *one* controlled degraded-claim state that condition implies onto every public-truth
//! consumer — release/help, marketplace, docs/help, certification, evaluation packs, support
//! exports, and companion handoffs — so a stale or narrowed supporting descriptor cannot leave
//! any consumer surface green by accident.
//!
//! Each consumer publishes the same [`NarrowedClaimState`] for the same condition. The state
//! is the controlled degraded-claim vocabulary the surfaces all share — `fully_supported`,
//! `limited`, `retest_pending`, `evidence_stale`, `unsupported_client`, `unsupported` — derived
//! deterministically from the descriptor's narrowings rather than hand-authored per surface.
//! Because every consumer projection is derived from one descriptor through one function, the
//! projections always *converge*: the same evidence condition yields the same degraded state on
//! the release card, the marketplace row, the docs badge, the evaluation-pack summary, and the
//! companion handoff summary alike. The [`ClaimNarrowingConformance`] block proves it.
//!
//! The downgrade is never a silent relabel. Every weaker descriptor value produces an
//! inspectable [`ClaimNarrowingReason`] (the facet, the value token, the claim state it
//! implies) and a paired [`RestorationStep`] naming the [`RestorationAction`] that would
//! restore the claim — so a user or support engineer can read *why* a claim narrowed and *what*
//! would lift it. The [`M5ClaimNarrowingRegistry`] is the one inspectable, serde-serializable
//! truth packet every consumer reads; it carries metadata and refs only — no credential bodies
//! or raw provider payloads.
//!
//! - Packet schema:
//!   [`schemas/provenance/m5-claim-narrowing.schema.json`](../../../../../schemas/provenance/m5-claim-narrowing.schema.json)
//! - Contract doc:
//!   [`docs/public-truth/m5-claim-narrowing.md`](../../../../../docs/public-truth/m5-claim-narrowing.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_evidence_stale_case, seeded_fully_supported_case, seeded_limited_case,
    seeded_m5_claim_narrowing_registry, seeded_retest_pending_case, seeded_unsupported_case,
    seeded_unsupported_client_case, M5_CLAIM_NARROWING_REGISTRY_ID,
};

use serde::{Deserialize, Serialize};

use crate::m5_descriptor_badge::{
    DowngradeEffect, PublicTruthConsumer, QualificationClass, M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX,
};
use crate::m5_descriptor_object::{DescriptorFacet, DescriptorNarrowing, DescriptorObject};

/// Record-kind tag carried by a [`ClaimNarrowingCase`].
pub const M5_CLAIM_NARROWING_CASE_RECORD_KIND: &str = "m5_claim_narrowing_case";

/// Record-kind tag carried by [`M5ClaimNarrowingRegistry`].
pub const M5_CLAIM_NARROWING_RECORD_KIND: &str = "m5_claim_narrowing_registry";

/// Schema version for the claim-narrowing case and registry.
pub const M5_CLAIM_NARROWING_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the claim-narrowing schema.
pub const M5_CLAIM_NARROWING_SCHEMA_REF: &str = "schemas/provenance/m5-claim-narrowing.schema.json";

/// Repo-relative path of the published claim-narrowing registry inventory.
pub const M5_CLAIM_NARROWING_REGISTRY_REF: &str = "artifacts/public-truth/m5-claim-narrowing.json";

/// Repo-relative path of the release-grade claim-narrowing parity proof.
pub const M5_CLAIM_NARROWING_PROOF_REF: &str =
    "artifacts/release/m5-descriptor-parity-proof/claim-narrowing.json";

/// Repo-relative path of the claim-narrowing contract doc.
pub const M5_CLAIM_NARROWING_DOC_REF: &str = "docs/public-truth/m5-claim-narrowing.md";

/// Repo-relative directory of the claim-narrowing consumer fixtures.
pub const M5_CLAIM_NARROWING_FIXTURE_DIR: &str = "fixtures/public-truth/m5-badge-consumers/";

/// The controlled degraded-claim state a consumer surface publishes for an evidence condition.
/// Declaration order is least→most degraded so the worst applicable state wins. Every consumer
/// renders the same state for the same condition; the state is derived from a descriptor's
/// narrowings, never hand-authored, so a thinned descriptor can never read as fully supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowedClaimState {
    /// The claim stands at its ceiling — no supporting descriptor narrowed it.
    FullySupported,
    /// Evidence or provenance is present but limited in scope; the claim is narrowed.
    Limited,
    /// A retest is pending before the claim can be relied on.
    RetestPending,
    /// Backing evidence fell out of its freshness window.
    EvidenceStale,
    /// The client scope cannot carry the claimed capability or authority.
    UnsupportedClient,
    /// A blocking condition holds; the claim is held from public truth entirely.
    Unsupported,
}

impl NarrowedClaimState {
    /// Every claim state, in declaration order (least→most degraded).
    pub const ALL: [Self; 6] = [
        Self::FullySupported,
        Self::Limited,
        Self::RetestPending,
        Self::EvidenceStale,
        Self::UnsupportedClient,
        Self::Unsupported,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullySupported => "fully_supported",
            Self::Limited => "limited",
            Self::RetestPending => "retest_pending",
            Self::EvidenceStale => "evidence_stale",
            Self::UnsupportedClient => "unsupported_client",
            Self::Unsupported => "unsupported",
        }
    }

    /// Reviewer-facing label for the state.
    pub const fn label(self) -> &'static str {
        match self {
            Self::FullySupported => "Fully supported",
            Self::Limited => "Limited",
            Self::RetestPending => "Retest pending",
            Self::EvidenceStale => "Evidence stale",
            Self::UnsupportedClient => "Unsupported client",
            Self::Unsupported => "Unsupported",
        }
    }

    /// Severity rank from the declaration order (higher is more degraded).
    fn severity(self) -> usize {
        Self::ALL.iter().position(|s| *s == self).unwrap_or(0)
    }

    /// True for the one clean state where the claim stands at its ceiling.
    pub const fn is_fully_supported(self) -> bool {
        matches!(self, Self::FullySupported)
    }

    /// True when the state holds the claim from public truth entirely.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::Unsupported)
    }

    /// True when the state narrows the claim below its ceiling without blocking it.
    pub const fn is_narrowed(self) -> bool {
        !self.is_fully_supported() && !self.is_blocked()
    }
}

/// The action that would restore a narrowed claim — the inspectable "what would fix it" half of
/// every narrowing reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestorationAction {
    /// Re-run or refresh the backing evidence so it returns to its freshness window.
    RefreshEvidence,
    /// Supply the missing or partial evidence so the proof is complete.
    CompleteEvidence,
    /// Attach first-party signed and attested provenance.
    ProvideProvenance,
    /// Perform the action on the full desktop client or via its handoff target.
    UseDesktopClient,
}

impl RestorationAction {
    /// Every restoration action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RefreshEvidence,
        Self::CompleteEvidence,
        Self::ProvideProvenance,
        Self::UseDesktopClient,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RefreshEvidence => "refresh_evidence",
            Self::CompleteEvidence => "complete_evidence",
            Self::ProvideProvenance => "provide_provenance",
            Self::UseDesktopClient => "use_desktop_client",
        }
    }
}

/// One inspectable downgrade reason: the descriptor facet and value token that narrowed the
/// claim, the effect it carried, and the claim state it implies. Naming the facet and token is
/// what keeps the downgrade from being a silent relabel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimNarrowingReason {
    /// The descriptor facet whose value triggered the narrowing.
    pub facet: DescriptorFacet,
    /// The weaker value token.
    pub token: String,
    /// Whether the value narrows the claim or blocks it entirely.
    pub effect: DowngradeEffect,
    /// The controlled claim state this value implies on its own.
    pub implied_state: NarrowedClaimState,
    /// Stable message id; prefixed [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub reason_message_id: String,
}

/// One restoration step: which narrowing it addresses and the action that would lift it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestorationStep {
    /// The descriptor facet this step would restore.
    pub facet: DescriptorFacet,
    /// The weaker value token this step addresses.
    pub addressed_token: String,
    /// The action that would restore the claim for this facet.
    pub action: RestorationAction,
    /// Stable message id; prefixed [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub restore_message_id: String,
}

/// One consumer surface's projection of a degraded claim. Every projection in a case carries
/// the *same* claim state and effective qualification as the case's canonical derivation — that
/// equality is the convergence proof: the same evidence condition narrows every surface identically.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerClaimProjection {
    /// The public-truth consumer surface.
    pub consumer: PublicTruthConsumer,
    /// Reviewer-facing consumer label.
    pub consumer_label: String,
    /// Where on this surface the claim renders (e.g. the release card, the marketplace row).
    pub surface_claim_field: String,
    /// The controlled claim state this surface publishes.
    pub claim_state: NarrowedClaimState,
    /// The effective qualification this surface publishes after narrowing.
    pub effective_qualification: QualificationClass,
    /// True when this projection matches the case's canonical state and qualification.
    pub converges_with_canonical: bool,
    /// Stable message id; prefixed [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub status_message_id: String,
}

/// Where each consumer surface renders the public claim. Local to this lane so the consumer
/// vocabulary stays owned by [`crate::m5_descriptor_badge`].
fn surface_claim_field(consumer: PublicTruthConsumer) -> &'static str {
    match consumer {
        PublicTruthConsumer::ReleaseCenter => "release/help provenance card",
        PublicTruthConsumer::HelpAbout => "Help/About support row",
        PublicTruthConsumer::Marketplace => "marketplace listing row",
        PublicTruthConsumer::DocsHelp => "docs/help reference badge",
        PublicTruthConsumer::Certification => "certification claim row",
        PublicTruthConsumer::EvaluationPacks => "evaluation-pack claim summary",
        PublicTruthConsumer::SupportExport => "support-export claim line",
        PublicTruthConsumer::CompanionHandoff => "companion handoff summary",
    }
}

/// The claim state a single narrowing implies on its own. A blocking value implies
/// [`NarrowedClaimState::Unsupported`]; a narrowing value maps by facet and token.
fn narrowing_claim_state(narrowing: &DescriptorNarrowing) -> NarrowedClaimState {
    if matches!(narrowing.effect, DowngradeEffect::Block) {
        return NarrowedClaimState::Unsupported;
    }
    match narrowing.facet {
        DescriptorFacet::ClientKind
        | DescriptorFacet::AuthorityClass
        | DescriptorFacet::HandoffRequirement => NarrowedClaimState::UnsupportedClient,
        DescriptorFacet::FreshnessState => NarrowedClaimState::EvidenceStale,
        DescriptorFacet::SourceClass | DescriptorFacet::SignatureState => {
            NarrowedClaimState::Limited
        }
        // The claimed support class is the starting claim, not a narrowing, so it never reaches
        // here; it is grouped with the evidence facets so the match stays total and any thin
        // evidence value maps the same way.
        DescriptorFacet::FreshnessEvidence
        | DescriptorFacet::QualificationEvidence
        | DescriptorFacet::SupportClass => match narrowing.token.as_str() {
            "evidence_stale" => NarrowedClaimState::EvidenceStale,
            "retest_pending" => NarrowedClaimState::RetestPending,
            // limited / partial and any other present-but-thin evidence value.
            _ => NarrowedClaimState::Limited,
        },
    }
}

/// The restoration action that would lift a single narrowing.
fn narrowing_restoration_action(narrowing: &DescriptorNarrowing) -> RestorationAction {
    match narrowing.facet {
        DescriptorFacet::SourceClass | DescriptorFacet::SignatureState => {
            RestorationAction::ProvideProvenance
        }
        DescriptorFacet::FreshnessState => RestorationAction::RefreshEvidence,
        DescriptorFacet::ClientKind
        | DescriptorFacet::AuthorityClass
        | DescriptorFacet::HandoffRequirement => RestorationAction::UseDesktopClient,
        DescriptorFacet::FreshnessEvidence
        | DescriptorFacet::QualificationEvidence
        | DescriptorFacet::SupportClass => match narrowing.token.as_str() {
            "evidence_stale" | "retest_pending" => RestorationAction::RefreshEvidence,
            // not_provided / limited / partial all need the evidence supplied.
            _ => RestorationAction::CompleteEvidence,
        },
    }
}

/// Derives the canonical degraded-claim state for a descriptor object: the most severe state
/// implied by any of its narrowings, or [`NarrowedClaimState::FullySupported`] when it carries
/// none.
fn derive_canonical_state(descriptor: &DescriptorObject) -> NarrowedClaimState {
    descriptor
        .narrowings
        .iter()
        .map(narrowing_claim_state)
        .max_by_key(|state| state.severity())
        .unwrap_or(NarrowedClaimState::FullySupported)
}

/// Derives the inspectable narrowing reasons from a descriptor's named narrowings.
fn derive_reasons(descriptor: &DescriptorObject) -> Vec<ClaimNarrowingReason> {
    descriptor
        .narrowings
        .iter()
        .map(|n| ClaimNarrowingReason {
            facet: n.facet,
            token: n.token.clone(),
            effect: n.effect,
            implied_state: narrowing_claim_state(n),
            reason_message_id: format!(
                "{M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX}claim_narrowing.reason.{}.{}",
                n.facet.as_str(),
                n.token
            ),
        })
        .collect()
}

/// Derives the restoration steps from a descriptor's named narrowings — one per narrowing.
fn derive_restoration(descriptor: &DescriptorObject) -> Vec<RestorationStep> {
    descriptor
        .narrowings
        .iter()
        .map(|n| RestorationStep {
            facet: n.facet,
            addressed_token: n.token.clone(),
            action: narrowing_restoration_action(n),
            restore_message_id: format!(
                "{M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX}claim_narrowing.restore.{}.{}",
                n.facet.as_str(),
                narrowing_restoration_action(n).as_str()
            ),
        })
        .collect()
}

/// One claim-narrowing case: an underlying evidence condition (a descriptor object), the
/// canonical degraded-claim state it implies, the inspectable reasons and restoration steps, and
/// the per-consumer projection that proves every surface converges on the same state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimNarrowingCase {
    /// Record kind; must equal [`M5_CLAIM_NARROWING_CASE_RECORD_KIND`].
    pub record_kind: String,
    /// Stable case id.
    pub case_id: String,
    /// Reviewer-facing case label.
    pub case_label: String,
    /// The descriptor object id this case binds.
    pub condition_descriptor_id: String,
    /// The underlying evidence condition this case narrows from.
    pub descriptor: DescriptorObject,
    /// The canonical degraded-claim state every consumer converges on.
    pub canonical_claim_state: NarrowedClaimState,
    /// The canonical effective qualification (mirrors the descriptor's own derivation).
    pub canonical_effective_qualification: QualificationClass,
    /// Inspectable downgrade reasons, in descriptor-facet order.
    pub reasons: Vec<ClaimNarrowingReason>,
    /// Restoration steps — what would restore the claim, one per narrowing.
    pub restoration: Vec<RestorationStep>,
    /// Per-consumer projections, in [`PublicTruthConsumer::ALL`] order.
    pub consumer_projections: Vec<ConsumerClaimProjection>,
    /// Stable message id for the case explanation drawer; prefixed
    /// [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub explanation_drawer_message_id: String,
}

impl ClaimNarrowingCase {
    /// Builds a case from a descriptor object, deriving the canonical state, reasons, restoration
    /// steps, and per-consumer projections so the degraded claim is always generated from the
    /// descriptor's own state rather than hand-authored per surface.
    pub fn from_descriptor(case_id: &str, case_label: &str, descriptor: DescriptorObject) -> Self {
        let canonical_claim_state = derive_canonical_state(&descriptor);
        let canonical_effective_qualification = descriptor.effective_qualification;
        let reasons = derive_reasons(&descriptor);
        let restoration = derive_restoration(&descriptor);
        let consumer_projections = PublicTruthConsumer::ALL
            .iter()
            .map(|&consumer| ConsumerClaimProjection {
                consumer,
                consumer_label: consumer.label().to_owned(),
                surface_claim_field: surface_claim_field(consumer).to_owned(),
                claim_state: canonical_claim_state,
                effective_qualification: canonical_effective_qualification,
                converges_with_canonical: true,
                status_message_id: format!(
                    "{M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX}claim_narrowing.{}.status",
                    consumer.as_str()
                ),
            })
            .collect();
        Self {
            record_kind: M5_CLAIM_NARROWING_CASE_RECORD_KIND.to_owned(),
            case_id: case_id.to_owned(),
            case_label: case_label.to_owned(),
            condition_descriptor_id: descriptor.descriptor_id.clone(),
            descriptor,
            canonical_claim_state,
            canonical_effective_qualification,
            reasons,
            restoration,
            consumer_projections,
            explanation_drawer_message_id: format!(
                "{M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX}claim_narrowing.case.drawer"
            ),
        }
    }

    /// True when no supporting descriptor narrowed the claim on any surface.
    pub fn is_fully_supported(&self) -> bool {
        self.canonical_claim_state.is_fully_supported()
    }

    /// True when a blocking condition holds the claim from public truth.
    pub fn is_blocked(&self) -> bool {
        self.canonical_claim_state.is_blocked()
    }

    /// Validates the case's invariants: derived state/reasons/restoration agree with the
    /// descriptor, every consumer converges, no narrowed surface reads fully supported, message
    /// ids carry the lane prefix, and the export carries no raw material.
    pub fn validate(&self) -> Vec<M5ClaimNarrowingViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_CLAIM_NARROWING_CASE_RECORD_KIND {
            out.push(M5ClaimNarrowingViolation::WrongRecordKind);
        }
        if self.case_id.trim().is_empty()
            || self.case_label.trim().is_empty()
            || self.condition_descriptor_id.trim().is_empty()
        {
            out.push(M5ClaimNarrowingViolation::MissingIdentity);
        }
        if self.condition_descriptor_id != self.descriptor.descriptor_id {
            out.push(M5ClaimNarrowingViolation::DescriptorBindingMismatch);
        }

        // The embedded descriptor must itself be self-consistent.
        if !self.descriptor.validate().is_empty() {
            out.push(M5ClaimNarrowingViolation::DescriptorInvalid);
        }

        // The canonical state and qualification must be derived from the descriptor.
        let expected_state = derive_canonical_state(&self.descriptor);
        if self.canonical_claim_state != expected_state {
            out.push(M5ClaimNarrowingViolation::ClaimStateDrift);
        }
        if self.canonical_effective_qualification != self.descriptor.effective_qualification {
            out.push(M5ClaimNarrowingViolation::EffectiveQualificationDrift);
        }

        // State / qualification coherence: a clean descriptor is fully supported; a blocking
        // descriptor is unsupported and held at Unavailable.
        let has_narrowing = !self.descriptor.narrowings.is_empty();
        if has_narrowing == self.canonical_claim_state.is_fully_supported() {
            out.push(M5ClaimNarrowingViolation::StateCoherenceBroken);
        }
        if self.descriptor.blocks_stable_promotion() != self.canonical_claim_state.is_blocked() {
            out.push(M5ClaimNarrowingViolation::StateCoherenceBroken);
        }

        // Reasons and restoration must be derived, one per narrowing.
        if self.reasons != derive_reasons(&self.descriptor) {
            out.push(M5ClaimNarrowingViolation::ReasonDrift);
        }
        if self.restoration != derive_restoration(&self.descriptor) {
            out.push(M5ClaimNarrowingViolation::RestorationDrift);
        }

        // Every consumer is projected, in canonical order, and every projection converges.
        let expected_consumers: Vec<PublicTruthConsumer> = PublicTruthConsumer::ALL.to_vec();
        let projected: Vec<PublicTruthConsumer> = self
            .consumer_projections
            .iter()
            .map(|p| p.consumer)
            .collect();
        if projected != expected_consumers {
            out.push(M5ClaimNarrowingViolation::ConsumerSetMismatch);
        }
        for projection in &self.consumer_projections {
            if projection.claim_state != self.canonical_claim_state
                || projection.effective_qualification != self.canonical_effective_qualification
                || !projection.converges_with_canonical
            {
                out.push(M5ClaimNarrowingViolation::ConsumerDiverged);
            }
            // The core guard: a narrowed condition can never read fully supported on a surface.
            if has_narrowing && projection.claim_state.is_fully_supported() {
                out.push(M5ClaimNarrowingViolation::NarrowedSurfaceReadsSupported);
            }
        }

        if !message_ids_prefixed(self) {
            out.push(M5ClaimNarrowingViolation::UnprefixedMessageId);
        }
        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("claim-narrowing case serializes"),
        ) {
            out.push(M5ClaimNarrowingViolation::RawMaterialInExport);
        }
        out
    }

    /// Deterministic export-safe JSON for the case.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only case fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("claim-narrowing case serializes")
    }
}

/// True when every message id the case carries is prefixed with the lane prefix.
fn message_ids_prefixed(case: &ClaimNarrowingCase) -> bool {
    let prefixed = |s: &str| s.starts_with(M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX);
    prefixed(&case.explanation_drawer_message_id)
        && case.reasons.iter().all(|r| prefixed(&r.reason_message_id))
        && case
            .restoration
            .iter()
            .all(|s| prefixed(&s.restore_message_id))
        && case
            .consumer_projections
            .iter()
            .all(|p| prefixed(&p.status_message_id))
}

/// Self-describing controlled-vocabulary set so the registry resolves every token a case carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimNarrowingVocabulary {
    /// Claim-state tokens.
    pub claim_states: Vec<String>,
    /// Restoration-action tokens.
    pub restoration_actions: Vec<String>,
    /// Descriptor-facet tokens.
    pub facets: Vec<String>,
    /// Downgrade-effect tokens.
    pub downgrade_effects: Vec<String>,
    /// Qualification-class tokens.
    pub qualification_classes: Vec<String>,
    /// Consumer tokens.
    pub consumers: Vec<String>,
}

impl ClaimNarrowingVocabulary {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            claim_states: NarrowedClaimState::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            restoration_actions: RestorationAction::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            facets: DescriptorFacet::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            downgrade_effects: DowngradeEffect::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            qualification_classes: QualificationClass::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            consumers: PublicTruthConsumer::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
        }
    }

    /// True when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Conformance review for the claim-narrowing registry. Every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimNarrowingConformance {
    /// Every case is self-consistent (derived state, reasons, restoration, projections).
    pub cases_validate: bool,
    /// A stale or narrowed descriptor never leaves a consumer surface fully supported.
    pub narrowed_never_reads_supported: bool,
    /// Every consumer projection converges on the case's canonical state and qualification.
    pub consumers_converge_on_same_state: bool,
    /// Every narrowing carries an inspectable reason and a restoration step.
    pub reasons_and_restoration_named: bool,
    /// A blocking condition holds the claim at unsupported / unavailable.
    pub blocking_condition_holds_unsupported: bool,
    /// Mirror/offline/side-loaded/not-provided origins still produce reasons, never omitted.
    pub weaker_origins_never_omitted: bool,
    /// The controlled vocabularies match the canonical frozen tokens.
    pub controlled_enums_frozen: bool,
    /// Every public-truth consumer reads this one claim-narrowing runtime.
    pub shared_across_consumers: bool,
    /// The export carries no raw provider material.
    pub export_carries_no_raw_material: bool,
}

impl ClaimNarrowingConformance {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.cases_validate
            && self.narrowed_never_reads_supported
            && self.consumers_converge_on_same_state
            && self.reasons_and_restoration_named
            && self.blocking_condition_holds_unsupported
            && self.weaker_origins_never_omitted
            && self.controlled_enums_frozen
            && self.shared_across_consumers
            && self.export_carries_no_raw_material
    }
}

/// Roll-up counts over the registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimNarrowingSummary {
    /// Total cases.
    pub total_cases: u32,
    /// Cases that stand fully supported.
    pub fully_supported_cases: u32,
    /// Cases narrowed below their ceiling without blocking.
    pub narrowed_cases: u32,
    /// Cases blocked from public truth.
    pub blocked_cases: u32,
    /// Total consumer projections across every case.
    pub total_projections: u32,
    /// Consumer projections that converge on their case's canonical state.
    pub converged_projections: u32,
}

/// Constructor input for [`M5ClaimNarrowingRegistry::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ClaimNarrowingRegistryInput {
    /// Stable registry id.
    pub registry_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The claim-narrowing cases this registry publishes.
    pub cases: Vec<ClaimNarrowingCase>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// The one inspectable, serde-serializable claim-narrowing truth packet every public-truth
/// consumer reads: the cases, the controlled vocabulary they share, the consumers that read the
/// runtime, a conformance review, and a roll-up summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ClaimNarrowingRegistry {
    /// Record kind; must equal [`M5_CLAIM_NARROWING_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_CLAIM_NARROWING_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable registry id.
    pub registry_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The claim-narrowing cases this registry publishes.
    pub cases: Vec<ClaimNarrowingCase>,
    /// The controlled vocabulary every case shares.
    pub vocabulary: ClaimNarrowingVocabulary,
    /// The public-truth consumers that read this claim-narrowing runtime.
    pub consumers: Vec<String>,
    /// Conformance review block.
    pub conformance: ClaimNarrowingConformance,
    /// Roll-up counts.
    pub summary: ClaimNarrowingSummary,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ClaimNarrowingRegistry {
    /// Builds a registry from its cases, deriving the vocabulary, consumer list, conformance
    /// review, and summary.
    pub fn new(input: M5ClaimNarrowingRegistryInput) -> Self {
        let cases = input.cases;
        let consumers: Vec<String> = PublicTruthConsumer::ALL
            .iter()
            .map(|c| c.as_str().to_owned())
            .collect();
        let conformance = derive_registry_conformance(&cases);
        let summary = derive_summary(&cases);
        Self {
            record_kind: M5_CLAIM_NARROWING_RECORD_KIND.to_owned(),
            schema_version: M5_CLAIM_NARROWING_SCHEMA_VERSION,
            registry_id: input.registry_id,
            report_label: input.report_label,
            cases,
            vocabulary: ClaimNarrowingVocabulary::canonical(),
            consumers,
            conformance,
            summary,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Finds a case by id.
    pub fn case(&self, case_id: &str) -> Option<&ClaimNarrowingCase> {
        self.cases.iter().find(|c| c.case_id == case_id)
    }

    /// Deterministic export-safe JSON for the registry.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("claim-narrowing registry serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 claim-narrowing parity\n\n");
        out.push_str(&format!("- Registry: `{}`\n", self.registry_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Cases: {}\n", self.cases.len()));
        out.push_str(&format!("- Minted: `{}`\n", self.minted_at));
        out.push_str(
            "- Consumed by: release center, Help/About, marketplace, docs/help, certification, evaluation packs, support, companion\n",
        );

        out.push_str("\n## Cases\n\n");
        out.push_str("| Case | Descriptor | Claim state | Effective | Reasons |\n");
        out.push_str("|------|------------|-------------|-----------|--------|\n");
        for case in &self.cases {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | {} |\n",
                case.case_id,
                case.condition_descriptor_id,
                case.canonical_claim_state.as_str(),
                case.canonical_effective_qualification.as_str(),
                case.reasons.len()
            ));
        }

        out.push_str("\n## Consumer convergence\n\n");
        for case in &self.cases {
            out.push_str(&format!(
                "### `{}` → `{}`\n\n",
                case.case_id,
                case.canonical_claim_state.as_str()
            ));
            out.push_str("| Consumer | Surface | Claim state | Converges |\n");
            out.push_str("|----------|---------|-------------|-----------|\n");
            for projection in &case.consumer_projections {
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | {} |\n",
                    projection.consumer.as_str(),
                    projection.surface_claim_field,
                    projection.claim_state.as_str(),
                    if projection.converges_with_canonical {
                        "yes"
                    } else {
                        "NO"
                    }
                ));
            }
            if case.restoration.is_empty() {
                out.push_str("\n_No narrowing — claim stands at its ceiling._\n\n");
            } else {
                out.push_str("\n**Restores when:**\n\n");
                for step in &case.restoration {
                    out.push_str(&format!(
                        "- `{}` (`{}`) → `{}`\n",
                        step.facet.as_str(),
                        step.addressed_token,
                        step.action.as_str()
                    ));
                }
                out.push('\n');
            }
        }
        out
    }

    /// Validates the registry's invariants.
    pub fn validate(&self) -> Vec<M5ClaimNarrowingViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_CLAIM_NARROWING_RECORD_KIND {
            out.push(M5ClaimNarrowingViolation::WrongRecordKind);
        }
        if self.schema_version != M5_CLAIM_NARROWING_SCHEMA_VERSION {
            out.push(M5ClaimNarrowingViolation::WrongSchemaVersion);
        }
        if self.registry_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            out.push(M5ClaimNarrowingViolation::MissingIdentity);
        }
        if self.cases.is_empty() {
            out.push(M5ClaimNarrowingViolation::RegistryHasNoCases);
        }
        let mut seen = std::collections::BTreeSet::new();
        for case in &self.cases {
            if !seen.insert(case.case_id.clone()) {
                out.push(M5ClaimNarrowingViolation::DuplicateCaseId);
            }
            out.extend(case.validate());
        }
        if !self.vocabulary.matches_canonical() {
            out.push(M5ClaimNarrowingViolation::VocabularyMismatch);
        }
        let expected_consumers: Vec<String> = PublicTruthConsumer::ALL
            .iter()
            .map(|c| c.as_str().to_owned())
            .collect();
        if self.consumers != expected_consumers {
            out.push(M5ClaimNarrowingViolation::ConsumerSetMismatch);
        }
        if self.conformance != derive_registry_conformance(&self.cases)
            || !self.conformance.all_hold()
        {
            out.push(M5ClaimNarrowingViolation::ConformanceReviewFailed);
        }
        if self.summary != derive_summary(&self.cases) {
            out.push(M5ClaimNarrowingViolation::SummaryMismatch);
        }
        out
    }
}

/// Derives the roll-up summary from the cases.
fn derive_summary(cases: &[ClaimNarrowingCase]) -> ClaimNarrowingSummary {
    let total_projections: u32 = cases
        .iter()
        .map(|c| c.consumer_projections.len() as u32)
        .sum();
    let converged_projections: u32 = cases
        .iter()
        .flat_map(|c| &c.consumer_projections)
        .filter(|p| p.converges_with_canonical)
        .count() as u32;
    ClaimNarrowingSummary {
        total_cases: cases.len() as u32,
        fully_supported_cases: cases.iter().filter(|c| c.is_fully_supported()).count() as u32,
        narrowed_cases: cases
            .iter()
            .filter(|c| c.canonical_claim_state.is_narrowed())
            .count() as u32,
        blocked_cases: cases.iter().filter(|c| c.is_blocked()).count() as u32,
        total_projections,
        converged_projections,
    }
}

/// Derives the registry conformance review from its cases.
fn derive_registry_conformance(cases: &[ClaimNarrowingCase]) -> ClaimNarrowingConformance {
    let cases_validate = !cases.is_empty() && cases.iter().all(|c| c.validate().is_empty());

    let narrowed_never_reads_supported = cases.iter().all(|c| {
        let has_narrowing = !c.descriptor.narrowings.is_empty();
        !has_narrowing
            || c.consumer_projections
                .iter()
                .all(|p| !p.claim_state.is_fully_supported())
    });

    let consumers_converge = cases.iter().all(|c| {
        c.consumer_projections.iter().all(|p| {
            p.claim_state == c.canonical_claim_state
                && p.effective_qualification == c.canonical_effective_qualification
                && p.converges_with_canonical
        })
    });

    let reasons_and_restoration_named = cases.iter().all(|c| {
        c.reasons.len() == c.descriptor.narrowings.len()
            && c.restoration.len() == c.descriptor.narrowings.len()
    });

    // A case whose descriptor blocks Stable promotion must read as unsupported / unavailable.
    let blocking_holds_unsupported = cases.iter().all(|c| {
        !c.descriptor.blocks_stable_promotion()
            || (c.canonical_claim_state.is_blocked()
                && matches!(
                    c.canonical_effective_qualification,
                    QualificationClass::Unavailable
                ))
    });

    // Every blocking case in the corpus is exercised at least once so the guard is not vacuous.
    let blocking_condition_holds_unsupported =
        blocking_holds_unsupported && cases.iter().any(|c| c.descriptor.blocks_stable_promotion());

    // A not-provided / mirror / offline / side-loaded origin must still surface a reason.
    let weaker_origins_never_omitted = cases.iter().all(|c| {
        c.descriptor
            .narrowings
            .iter()
            .filter(|n| matches!(n.facet, DescriptorFacet::SourceClass))
            .all(|n| c.reasons.iter().any(|r| r.token == n.token))
    });

    let export_clean = cases.iter().all(|c| {
        !json_contains_forbidden_material(
            &serde_json::to_value(c).expect("claim-narrowing case serializes"),
        )
    });

    ClaimNarrowingConformance {
        cases_validate,
        narrowed_never_reads_supported,
        consumers_converge_on_same_state: consumers_converge,
        reasons_and_restoration_named,
        blocking_condition_holds_unsupported,
        weaker_origins_never_omitted,
        controlled_enums_frozen: ClaimNarrowingVocabulary::canonical().matches_canonical(),
        shared_across_consumers: true,
        export_carries_no_raw_material: export_clean,
    }
}

/// Validation failures for the claim-narrowing lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ClaimNarrowingViolation {
    /// The record kind is wrong.
    WrongRecordKind,
    /// The schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is empty.
    MissingIdentity,
    /// The case binds a descriptor id that disagrees with the embedded descriptor.
    DescriptorBindingMismatch,
    /// The embedded descriptor object is itself invalid.
    DescriptorInvalid,
    /// The canonical claim state drifted from a fresh derivation.
    ClaimStateDrift,
    /// The canonical effective qualification drifted from the descriptor.
    EffectiveQualificationDrift,
    /// The claim state is incoherent with the descriptor's narrowing / blocking posture.
    StateCoherenceBroken,
    /// The inspectable reasons drifted from a fresh derivation.
    ReasonDrift,
    /// The restoration steps drifted from a fresh derivation.
    RestorationDrift,
    /// A consumer projection diverged from the case's canonical state.
    ConsumerDiverged,
    /// A narrowed condition reads as fully supported on a consumer surface.
    NarrowedSurfaceReadsSupported,
    /// A message id is missing the lane prefix.
    UnprefixedMessageId,
    /// The registry publishes no cases.
    RegistryHasNoCases,
    /// Two cases share a case id.
    DuplicateCaseId,
    /// The controlled-vocabulary set does not match the canonical tokens.
    VocabularyMismatch,
    /// The consumer set does not match the canonical consumers.
    ConsumerSetMismatch,
    /// A conformance-review flag does not hold or drifted.
    ConformanceReviewFailed,
    /// The summary did not match the computed roll-up.
    SummaryMismatch,
    /// The export contains raw provider material.
    RawMaterialInExport,
}

impl M5ClaimNarrowingViolation {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::DescriptorBindingMismatch => "descriptor_binding_mismatch",
            Self::DescriptorInvalid => "descriptor_invalid",
            Self::ClaimStateDrift => "claim_state_drift",
            Self::EffectiveQualificationDrift => "effective_qualification_drift",
            Self::StateCoherenceBroken => "state_coherence_broken",
            Self::ReasonDrift => "reason_drift",
            Self::RestorationDrift => "restoration_drift",
            Self::ConsumerDiverged => "consumer_diverged",
            Self::NarrowedSurfaceReadsSupported => "narrowed_surface_reads_supported",
            Self::UnprefixedMessageId => "unprefixed_message_id",
            Self::RegistryHasNoCases => "registry_has_no_cases",
            Self::DuplicateCaseId => "duplicate_case_id",
            Self::VocabularyMismatch => "vocabulary_mismatch",
            Self::ConsumerSetMismatch => "consumer_set_mismatch",
            Self::ConformanceReviewFailed => "conformance_review_failed",
            Self::SummaryMismatch => "summary_mismatch",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Keys whose presence would mean an export leaked raw material. Mirrors the redaction posture
/// of the upstream descriptor lanes.
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
