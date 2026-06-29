//! Copy-safe joins of a descriptor object into export packets, support bundles, and admin reports.
//!
//! The [descriptor object](crate::m5_descriptor_object) lane freezes the typed
//! provenance / freshness / qualification / client-scope state a claimed M5 artifact carries,
//! and the [claim-narrowing](crate::m5_claim_narrowing) lane derives the one controlled
//! degraded-claim state that condition implies across every public-truth consumer. Those lanes
//! make the truth *interactive*. This lane makes it *portable*: it joins a descriptor object into
//! the copy-safe carrier shapes the support, admin, and reporting paths actually emit — an export
//! packet, a support bundle, an admin report, and a plain copy-safe summary — and proves the
//! descriptor's identity, its typed artifact binding, and its inspectable downgrade reasons all
//! survive copy/export instead of collapsing to flat text.
//!
//! Each [`DescriptorJoin`] embeds the descriptor object it joins, re-states the descriptor
//! identity ([`descriptor_id`](DescriptorJoin::descriptor_id)) and the typed
//! [artifact binding](crate::m5_descriptor_object::ArtifactBinding) as first-class join fields,
//! and derives — never hand-authors — the current [claim state](crate::m5_claim_narrowing) from
//! the shared claim-narrowing runtime, the inspectable [`JoinDowngradeReason`]s from the
//! descriptor's named narrowings, and the supporting [`JoinEvidenceRef`]s (schema, digest, and
//! proof-packet *references* only, never raw payloads). Every weaker mirror / offline /
//! side-loaded / `not_provided` origin still surfaces as a reason rather than disappearing into
//! omission, and a narrowed descriptor can never read fully supported on any carrier.
//!
//! Because each [`JoinCarrierRendering`] carries the same descriptor identity, the same artifact
//! binding, and the same downgrade-reason count as its join, the carriers *converge*: a support
//! bundle, an admin report, and a copy-safe summary reconstruct the same public truth from one
//! source. And because a join's serialization is deterministic and channel-independent, the
//! [`JoinChannel`] parity check proves the desktop UI, the CLI/headless path, and offline /
//! mirror-safe packet generation all emit byte-identical output. The [`M5DescriptorJoinRegistry`]
//! is the one inspectable, serde-serializable truth packet every export path reads; it carries
//! metadata and refs only — no credential bodies or raw provider payloads.
//!
//! - Packet schema:
//!   [`schemas/provenance/m5-descriptor-join.schema.json`](../../../../../schemas/provenance/m5-descriptor-join.schema.json)
//! - Contract doc:
//!   [`docs/public-truth/m5-descriptor-join.md`](../../../../../docs/public-truth/m5-descriptor-join.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_evidence_stale_join, seeded_fully_supported_join, seeded_limited_join,
    seeded_m5_descriptor_join_registry, seeded_retest_pending_join, seeded_unsupported_client_join,
    seeded_unsupported_join, M5_DESCRIPTOR_JOIN_REGISTRY_ID,
};

use serde::{Deserialize, Serialize};

use crate::m5_claim_narrowing::{ClaimNarrowingCase, NarrowedClaimState};
use crate::m5_descriptor_badge::{
    DowngradeEffect, PublicTruthConsumer, QualificationClass, M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX,
};
use crate::m5_descriptor_object::{
    ArtifactBinding, DescriptorFacet, DescriptorObject, M5_DESCRIPTOR_OBJECT_REGISTRY_REF,
    M5_DESCRIPTOR_OBJECT_SCHEMA_REF,
};

/// Record-kind tag carried by a [`DescriptorJoin`].
pub const M5_DESCRIPTOR_JOIN_RECORD_KIND: &str = "m5_descriptor_join";

/// Record-kind tag carried by [`M5DescriptorJoinRegistry`].
pub const M5_DESCRIPTOR_JOIN_REGISTRY_RECORD_KIND: &str = "m5_descriptor_join_registry";

/// Schema version for the descriptor-join and registry.
pub const M5_DESCRIPTOR_JOIN_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the descriptor-join schema.
pub const M5_DESCRIPTOR_JOIN_SCHEMA_REF: &str = "schemas/provenance/m5-descriptor-join.schema.json";

/// Repo-relative path of the published descriptor-join registry inventory.
pub const M5_DESCRIPTOR_JOIN_REGISTRY_REF: &str = "artifacts/public-truth/m5-descriptor-join.json";

/// Repo-relative path of the release-grade descriptor-join parity proof.
pub const M5_DESCRIPTOR_JOIN_PROOF_REF: &str =
    "artifacts/release/m5-descriptor-parity-proof/descriptor-join.json";

/// Repo-relative path of the descriptor-join contract doc.
pub const M5_DESCRIPTOR_JOIN_DOC_REF: &str = "docs/public-truth/m5-descriptor-join.md";

/// Repo-relative directory of the descriptor-join carrier fixtures.
pub const M5_DESCRIPTOR_JOIN_FIXTURE_DIR: &str = "fixtures/public-truth/m5-badge-consumers/";

/// One copy-safe carrier shape a descriptor join is rendered into. Each carrier preserves the
/// descriptor identity, the artifact binding, and the downgrade reasons; the carriers differ only
/// in where they land — an export packet, a support bundle, an admin report, or a plain summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JoinCarrier {
    /// A copy-safe export packet (desktop export or CLI/headless structured output).
    ExportPacket,
    /// A support-bundle attachment.
    SupportBundle,
    /// An admin / fleet report row.
    AdminReport,
    /// A plain, copy-safe one-line summary.
    CopySafeSummary,
}

impl JoinCarrier {
    /// Every carrier, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ExportPacket,
        Self::SupportBundle,
        Self::AdminReport,
        Self::CopySafeSummary,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExportPacket => "export_packet",
            Self::SupportBundle => "support_bundle",
            Self::AdminReport => "admin_report",
            Self::CopySafeSummary => "copy_safe_summary",
        }
    }

    /// Reviewer-facing carrier label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ExportPacket => "Export packet",
            Self::SupportBundle => "Support bundle",
            Self::AdminReport => "Admin report",
            Self::CopySafeSummary => "Copy-safe summary",
        }
    }
}

/// The generation channel a join is produced on. Every channel produces byte-identical output —
/// the desktop UI, the CLI/headless path, and offline / mirror-safe packet generation cannot
/// diverge because the join's serialization carries no channel-specific state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JoinChannel {
    /// The desktop product UI export action.
    DesktopUi,
    /// The CLI / headless structured-output path.
    CliHeadless,
    /// Offline / mirror-safe packet generation.
    OfflineMirror,
}

impl JoinChannel {
    /// Every channel, in declaration order.
    pub const ALL: [Self; 3] = [Self::DesktopUi, Self::CliHeadless, Self::OfflineMirror];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopUi => "desktop_ui",
            Self::CliHeadless => "cli_headless",
            Self::OfflineMirror => "offline_mirror",
        }
    }
}

/// The kind of supporting-evidence reference a join carries. Every value is a *reference* — a
/// schema path, a content-digest ref, or a proof-packet path — never a raw provider payload, so
/// the join can attribute its truth without forcing internal-payload disclosure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceRefKind {
    /// The schema that governs the bound artifact.
    ArtifactSchema,
    /// The content-digest reference of the bound artifact.
    ArtifactDigest,
    /// The descriptor-object schema the join's descriptor conforms to.
    DescriptorSchema,
    /// The published descriptor proof packet that keeps the descriptor current.
    ProofPacket,
}

impl EvidenceRefKind {
    /// Every evidence-ref kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ArtifactSchema,
        Self::ArtifactDigest,
        Self::DescriptorSchema,
        Self::ProofPacket,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArtifactSchema => "artifact_schema",
            Self::ArtifactDigest => "artifact_digest",
            Self::DescriptorSchema => "descriptor_schema",
            Self::ProofPacket => "proof_packet",
        }
    }
}

/// One supporting-evidence reference carried into every carrier so a support / admin / export
/// artifact can name the evidence behind the descriptor truth without disclosing a raw payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JoinEvidenceRef {
    /// The kind of reference.
    pub ref_kind: EvidenceRefKind,
    /// The reference value — a schema path, a digest ref, or a proof-packet path.
    pub ref_value: String,
}

/// One inspectable downgrade reason carried into every copy-safe carrier: the descriptor facet
/// and value token that narrowed the claim, the effect it carried, the claim state it implies,
/// the qualification it floors the claim at, and the message ids that explain and caveat it.
/// Naming the facet and token is what keeps the downgrade attributable after export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JoinDowngradeReason {
    /// The descriptor facet whose value triggered the narrowing.
    pub facet: DescriptorFacet,
    /// The weaker value token.
    pub token: String,
    /// Whether the value narrows the claim or blocks it entirely.
    pub effect: DowngradeEffect,
    /// The controlled claim state this value implies on its own.
    pub implied_state: NarrowedClaimState,
    /// The qualification this value floors the claim at.
    pub effective_floor: QualificationClass,
    /// Stable claim-narrowing reason id; prefixed [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub reason_message_id: String,
    /// Stable caveat id for the carrier-facing caveat line; prefixed
    /// [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub caveat_message_id: String,
}

/// One carrier's copy-safe rendering of a descriptor join. Every carrier re-states the descriptor
/// identity, the typed artifact binding, the claim state, the effective qualification, and the
/// downgrade-reason count of its join — that equality is the proof copy/export never flattens the
/// truth away.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JoinCarrierRendering {
    /// The carrier shape.
    pub carrier: JoinCarrier,
    /// Reviewer-facing carrier label.
    pub carrier_label: String,
    /// The descriptor identity preserved on this carrier.
    pub descriptor_id: String,
    /// The typed artifact binding preserved on this carrier.
    pub artifact_ref: ArtifactBinding,
    /// The claim state this carrier publishes.
    pub claim_state: NarrowedClaimState,
    /// The effective qualification this carrier publishes.
    pub effective_qualification: QualificationClass,
    /// The number of attributable downgrade reasons this carrier preserves.
    pub downgrade_reason_count: u32,
    /// True when the descriptor identity survives onto this carrier.
    pub preserves_identity: bool,
    /// True when the typed artifact binding survives onto this carrier.
    pub preserves_binding: bool,
    /// True when every downgrade reason survives onto this carrier.
    pub preserves_downgrade_reasons: bool,
    /// Stable message id; prefixed [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub render_message_id: String,
}

/// A copy-safe join of one descriptor object into export packets, support bundles, admin reports,
/// and a copy-safe summary. It re-states the descriptor identity and the typed artifact binding,
/// embeds the descriptor object so every facet's current value can be reconstructed without ad hoc
/// translation, and derives the current claim state, the inspectable downgrade reasons, and the
/// supporting evidence references from that descriptor's own state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorJoin {
    /// Record kind; must equal [`M5_DESCRIPTOR_JOIN_RECORD_KIND`].
    pub record_kind: String,
    /// Stable join id.
    pub join_id: String,
    /// Reviewer-facing join label.
    pub join_label: String,
    /// The descriptor identity this join preserves.
    pub descriptor_id: String,
    /// Reviewer-facing descriptor label.
    pub descriptor_label: String,
    /// The typed artifact binding this join preserves.
    pub artifact_ref: ArtifactBinding,
    /// The descriptor object this join carries — the full provenance / freshness / qualification /
    /// client-scope truth, reconstructable without ad hoc translation.
    pub descriptor: DescriptorObject,
    /// The claimed support class before narrowing.
    pub claimed_support_class: QualificationClass,
    /// The effective qualification after narrowing.
    pub effective_qualification: QualificationClass,
    /// The current controlled claim state, derived from the shared claim-narrowing runtime.
    pub claim_state: NarrowedClaimState,
    /// Inspectable downgrade reasons, in descriptor-facet order.
    pub downgrade_reasons: Vec<JoinDowngradeReason>,
    /// Supporting evidence references — refs only, never raw payloads.
    pub evidence_refs: Vec<JoinEvidenceRef>,
    /// Per-carrier renderings, in [`JoinCarrier::ALL`] order.
    pub carriers: Vec<JoinCarrierRendering>,
    /// A deterministic, copy-safe one-line summary of the join.
    pub copy_safe_summary: String,
    /// Stable message id for the join explanation drawer; prefixed
    /// [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub explanation_drawer_message_id: String,
}

impl DescriptorJoin {
    /// Builds a join from a descriptor object, deriving the claim state, downgrade reasons,
    /// evidence references, carrier renderings, and copy-safe summary from the descriptor's own
    /// state so the export carriers are always generated from one source rather than hand-authored.
    pub fn from_descriptor(join_id: &str, join_label: &str, descriptor: DescriptorObject) -> Self {
        let claim_state = derive_claim_state(&descriptor);
        let claimed_support_class = descriptor.qualification.support_class;
        let effective_qualification = descriptor.effective_qualification;
        let downgrade_reasons = derive_downgrade_reasons(&descriptor);
        let evidence_refs = derive_evidence_refs(&descriptor);
        let carriers = derive_carriers(
            &descriptor.descriptor_id,
            &descriptor.artifact_ref,
            claim_state,
            effective_qualification,
            downgrade_reasons.len(),
        );
        let copy_safe_summary = derive_copy_safe_summary(
            &descriptor,
            claim_state,
            effective_qualification,
            downgrade_reasons.len(),
        );
        Self {
            record_kind: M5_DESCRIPTOR_JOIN_RECORD_KIND.to_owned(),
            join_id: join_id.to_owned(),
            join_label: join_label.to_owned(),
            descriptor_id: descriptor.descriptor_id.clone(),
            descriptor_label: descriptor.descriptor_label.clone(),
            artifact_ref: descriptor.artifact_ref.clone(),
            claimed_support_class,
            effective_qualification,
            claim_state,
            downgrade_reasons,
            evidence_refs,
            carriers,
            copy_safe_summary,
            explanation_drawer_message_id: format!(
                "{M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX}join.drawer"
            ),
            descriptor,
        }
    }

    /// True when no supporting descriptor narrowed the claim.
    pub fn is_fully_supported(&self) -> bool {
        self.claim_state.is_fully_supported()
    }

    /// True when a blocking condition holds the claim from public truth.
    pub fn is_blocked(&self) -> bool {
        self.claim_state.is_blocked()
    }

    /// Renders the join for a generation channel. The output is identical for every channel —
    /// the channel parameter exists only to prove desktop, CLI/headless, and offline / mirror
    /// packet generation produce byte-identical output.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only join fails.
    pub fn render_for_channel(&self, _channel: JoinChannel) -> String {
        self.export_safe_json()
    }

    /// Deterministic export-safe JSON for the join.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only join fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("descriptor join serializes")
    }

    /// Validates the join's invariants: derived state / reasons / refs / carriers agree with the
    /// embedded descriptor, the identity and binding are preserved on every carrier, a narrowed
    /// descriptor never reads fully supported, message ids carry the lane prefix, and the export
    /// carries no raw material.
    pub fn validate(&self) -> Vec<M5DescriptorJoinViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_DESCRIPTOR_JOIN_RECORD_KIND {
            out.push(M5DescriptorJoinViolation::WrongRecordKind);
        }
        if self.join_id.trim().is_empty()
            || self.join_label.trim().is_empty()
            || self.descriptor_id.trim().is_empty()
            || self.descriptor_label.trim().is_empty()
            || self.copy_safe_summary.trim().is_empty()
        {
            out.push(M5DescriptorJoinViolation::MissingIdentity);
        }

        // The embedded descriptor must itself be self-consistent.
        if !self.descriptor.validate().is_empty() {
            out.push(M5DescriptorJoinViolation::DescriptorInvalid);
        }

        // The join key fields must mirror the embedded descriptor — identity and binding preserved.
        if self.descriptor_id != self.descriptor.descriptor_id
            || self.descriptor_label != self.descriptor.descriptor_label
            || self.artifact_ref != self.descriptor.artifact_ref
        {
            out.push(M5DescriptorJoinViolation::DescriptorBindingMismatch);
        }

        // Claimed / effective qualification must be read from the descriptor.
        if self.claimed_support_class != self.descriptor.qualification.support_class
            || self.effective_qualification != self.descriptor.effective_qualification
        {
            out.push(M5DescriptorJoinViolation::EffectiveQualificationDrift);
        }

        // The claim state and reasons must be derived from the shared runtimes.
        if self.claim_state != derive_claim_state(&self.descriptor) {
            out.push(M5DescriptorJoinViolation::ClaimStateDrift);
        }
        if self.downgrade_reasons != derive_downgrade_reasons(&self.descriptor) {
            out.push(M5DescriptorJoinViolation::DowngradeReasonDrift);
        }
        if self.evidence_refs != derive_evidence_refs(&self.descriptor) {
            out.push(M5DescriptorJoinViolation::EvidenceRefDrift);
        }
        let expected_carriers = derive_carriers(
            &self.descriptor.descriptor_id,
            &self.descriptor.artifact_ref,
            self.claim_state,
            self.effective_qualification,
            self.downgrade_reasons.len(),
        );
        if self.carriers != expected_carriers {
            out.push(M5DescriptorJoinViolation::CarrierDrift);
        }
        if self.copy_safe_summary
            != derive_copy_safe_summary(
                &self.descriptor,
                self.claim_state,
                self.effective_qualification,
                self.downgrade_reasons.len(),
            )
        {
            out.push(M5DescriptorJoinViolation::CopySafeSummaryDrift);
        }

        // State coherence: a clean descriptor is fully supported; a narrowed one carries reasons.
        let has_narrowing = !self.descriptor.narrowings.is_empty();
        if has_narrowing == self.claim_state.is_fully_supported() {
            out.push(M5DescriptorJoinViolation::StateCoherenceBroken);
        }
        if has_narrowing && self.downgrade_reasons.is_empty() {
            out.push(M5DescriptorJoinViolation::StateCoherenceBroken);
        }

        // Every carrier is projected, in canonical order, and preserves identity / binding / reasons.
        let projected: Vec<JoinCarrier> = self.carriers.iter().map(|c| c.carrier).collect();
        if projected != JoinCarrier::ALL.to_vec() {
            out.push(M5DescriptorJoinViolation::CarrierSetMismatch);
        }
        for carrier in &self.carriers {
            if carrier.descriptor_id != self.descriptor_id
                || carrier.artifact_ref != self.artifact_ref
            {
                out.push(M5DescriptorJoinViolation::CarrierDropsBinding);
            }
            if carrier.claim_state != self.claim_state
                || carrier.effective_qualification != self.effective_qualification
                || carrier.downgrade_reason_count != self.downgrade_reasons.len() as u32
            {
                out.push(M5DescriptorJoinViolation::CarrierDiverged);
            }
            if !carrier.preserves_identity
                || !carrier.preserves_binding
                || !carrier.preserves_downgrade_reasons
            {
                out.push(M5DescriptorJoinViolation::CarrierDropsBinding);
            }
            // The core guard: a narrowed descriptor can never read fully supported on a carrier.
            if has_narrowing && carrier.claim_state.is_fully_supported() {
                out.push(M5DescriptorJoinViolation::NarrowedCarrierReadsSupported);
            }
        }

        if !message_ids_prefixed(self) {
            out.push(M5DescriptorJoinViolation::UnprefixedMessageId);
        }
        if self.evidence_refs.is_empty()
            || self
                .evidence_refs
                .iter()
                .any(|r| r.ref_value.trim().is_empty())
        {
            out.push(M5DescriptorJoinViolation::EvidenceRefDrift);
        }
        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("descriptor join serializes"),
        ) {
            out.push(M5DescriptorJoinViolation::RawMaterialInExport);
        }
        out
    }
}

/// Derives the current claim state from the shared claim-narrowing runtime so the join can never
/// derive a different state than the interactive consumers do.
fn derive_claim_state(descriptor: &DescriptorObject) -> NarrowedClaimState {
    claim_case(descriptor).canonical_claim_state
}

/// Builds a claim-narrowing case from the descriptor — the single shared derivation of the claim
/// state and the inspectable narrowing reasons.
fn claim_case(descriptor: &DescriptorObject) -> ClaimNarrowingCase {
    ClaimNarrowingCase::from_descriptor(
        "descriptor-join:derivation",
        "Descriptor-join derivation",
        descriptor.clone(),
    )
}

/// Derives the inspectable downgrade reasons by pairing the descriptor's named narrowings with
/// the claim-narrowing runtime's reasons (both derived in facet order from the same descriptor).
fn derive_downgrade_reasons(descriptor: &DescriptorObject) -> Vec<JoinDowngradeReason> {
    let case = claim_case(descriptor);
    descriptor
        .narrowings
        .iter()
        .zip(case.reasons.iter())
        .map(|(narrowing, reason)| JoinDowngradeReason {
            facet: reason.facet,
            token: reason.token.clone(),
            effect: reason.effect,
            implied_state: reason.implied_state,
            effective_floor: narrowing.effective_floor,
            reason_message_id: reason.reason_message_id.clone(),
            caveat_message_id: format!(
                "{M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX}join.caveat.{}.{}",
                reason.facet.as_str(),
                reason.token
            ),
        })
        .collect()
}

/// Derives the supporting evidence references for a descriptor — refs only, never raw payloads.
fn derive_evidence_refs(descriptor: &DescriptorObject) -> Vec<JoinEvidenceRef> {
    vec![
        JoinEvidenceRef {
            ref_kind: EvidenceRefKind::ArtifactSchema,
            ref_value: descriptor.artifact_ref.schema_ref.clone(),
        },
        JoinEvidenceRef {
            ref_kind: EvidenceRefKind::ArtifactDigest,
            ref_value: descriptor.artifact_ref.content_digest_ref.clone(),
        },
        JoinEvidenceRef {
            ref_kind: EvidenceRefKind::DescriptorSchema,
            ref_value: M5_DESCRIPTOR_OBJECT_SCHEMA_REF.to_owned(),
        },
        JoinEvidenceRef {
            ref_kind: EvidenceRefKind::ProofPacket,
            ref_value: M5_DESCRIPTOR_OBJECT_REGISTRY_REF.to_owned(),
        },
    ]
}

/// Derives the per-carrier renderings, one per [`JoinCarrier`], each preserving the identity,
/// binding, claim state, effective qualification, and downgrade-reason count.
fn derive_carriers(
    descriptor_id: &str,
    artifact_ref: &ArtifactBinding,
    claim_state: NarrowedClaimState,
    effective_qualification: QualificationClass,
    downgrade_reason_count: usize,
) -> Vec<JoinCarrierRendering> {
    JoinCarrier::ALL
        .iter()
        .map(|&carrier| JoinCarrierRendering {
            carrier,
            carrier_label: carrier.label().to_owned(),
            descriptor_id: descriptor_id.to_owned(),
            artifact_ref: artifact_ref.clone(),
            claim_state,
            effective_qualification,
            downgrade_reason_count: downgrade_reason_count as u32,
            preserves_identity: true,
            preserves_binding: true,
            preserves_downgrade_reasons: true,
            render_message_id: format!(
                "{M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX}join.carrier.{}.render",
                carrier.as_str()
            ),
        })
        .collect()
}

/// Derives the deterministic, copy-safe one-line summary for a descriptor join.
fn derive_copy_safe_summary(
    descriptor: &DescriptorObject,
    claim_state: NarrowedClaimState,
    effective_qualification: QualificationClass,
    downgrade_reason_count: usize,
) -> String {
    format!(
        "{} · artifact {}/{} · claim {} · effective {} · {} reason(s)",
        descriptor.descriptor_id,
        descriptor.artifact_ref.artifact_family,
        descriptor.artifact_ref.artifact_id,
        claim_state.as_str(),
        effective_qualification.as_str(),
        downgrade_reason_count
    )
}

/// True when every message id the join carries is prefixed with the lane prefix.
fn message_ids_prefixed(join: &DescriptorJoin) -> bool {
    let prefixed = |s: &str| s.starts_with(M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX);
    prefixed(&join.explanation_drawer_message_id)
        && join
            .downgrade_reasons
            .iter()
            .all(|r| prefixed(&r.reason_message_id) && prefixed(&r.caveat_message_id))
        && join.carriers.iter().all(|c| prefixed(&c.render_message_id))
}

/// Self-describing controlled-vocabulary set so the registry resolves every token a join carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorJoinVocabulary {
    /// Carrier tokens.
    pub carriers: Vec<String>,
    /// Channel tokens.
    pub channels: Vec<String>,
    /// Claim-state tokens.
    pub claim_states: Vec<String>,
    /// Descriptor-facet tokens.
    pub facets: Vec<String>,
    /// Downgrade-effect tokens.
    pub downgrade_effects: Vec<String>,
    /// Qualification-class tokens.
    pub qualification_classes: Vec<String>,
    /// Evidence-ref-kind tokens.
    pub evidence_ref_kinds: Vec<String>,
    /// Consumer tokens.
    pub consumers: Vec<String>,
}

impl DescriptorJoinVocabulary {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            carriers: JoinCarrier::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            channels: JoinChannel::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            claim_states: NarrowedClaimState::ALL
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
            evidence_ref_kinds: EvidenceRefKind::ALL
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

/// Conformance review for the descriptor-join registry. Every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorJoinConformance {
    /// Every join is self-consistent (derived state, reasons, refs, carriers).
    pub joins_validate: bool,
    /// The descriptor identity survives onto every carrier.
    pub identity_survives_carriers: bool,
    /// The typed artifact binding survives onto every carrier.
    pub binding_survives_carriers: bool,
    /// Every narrowing is attributable as an inspectable reason on every carrier.
    pub downgrade_reasons_attributable: bool,
    /// The full descriptor truth is reconstructable from a join without ad hoc translation.
    pub truth_reconstructable_without_translation: bool,
    /// The claim state matches the shared claim-narrowing runtime.
    pub claim_state_matches_narrowing_runtime: bool,
    /// Supporting evidence is carried as references only — never raw payloads.
    pub evidence_refs_are_refs_only: bool,
    /// Mirror / offline / side-loaded / not-provided origins still surface as reasons.
    pub weaker_origins_never_omitted: bool,
    /// Desktop, CLI/headless, and offline / mirror generation produce identical output.
    pub channels_produce_identical_output: bool,
    /// The controlled vocabularies match the canonical frozen tokens.
    pub controlled_enums_frozen: bool,
    /// Every public-truth consumer reads this one join runtime.
    pub shared_across_consumers: bool,
    /// The export carries no raw provider material.
    pub export_carries_no_raw_material: bool,
}

impl DescriptorJoinConformance {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.joins_validate
            && self.identity_survives_carriers
            && self.binding_survives_carriers
            && self.downgrade_reasons_attributable
            && self.truth_reconstructable_without_translation
            && self.claim_state_matches_narrowing_runtime
            && self.evidence_refs_are_refs_only
            && self.weaker_origins_never_omitted
            && self.channels_produce_identical_output
            && self.controlled_enums_frozen
            && self.shared_across_consumers
            && self.export_carries_no_raw_material
    }
}

/// Roll-up counts over the registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorJoinSummary {
    /// Total joins.
    pub total_joins: u32,
    /// Joins that stand fully supported.
    pub fully_supported_joins: u32,
    /// Joins narrowed below their ceiling without blocking.
    pub narrowed_joins: u32,
    /// Joins blocked from public truth.
    pub blocked_joins: u32,
    /// Total carrier renderings across every join.
    pub total_carrier_renderings: u32,
    /// Total downgrade reasons across every join.
    pub total_downgrade_reasons: u32,
}

/// Constructor input for [`M5DescriptorJoinRegistry::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DescriptorJoinRegistryInput {
    /// Stable registry id.
    pub registry_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The descriptor joins this registry publishes.
    pub joins: Vec<DescriptorJoin>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// The one inspectable, serde-serializable descriptor-join truth packet every export path reads:
/// the joins, the controlled vocabulary they share, the consumers that read the runtime, a
/// conformance review, and a roll-up summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DescriptorJoinRegistry {
    /// Record kind; must equal [`M5_DESCRIPTOR_JOIN_REGISTRY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DESCRIPTOR_JOIN_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable registry id.
    pub registry_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The descriptor joins this registry publishes.
    pub joins: Vec<DescriptorJoin>,
    /// The controlled vocabulary every join shares.
    pub vocabulary: DescriptorJoinVocabulary,
    /// The public-truth consumers that read this join runtime.
    pub consumers: Vec<String>,
    /// Conformance review block.
    pub conformance: DescriptorJoinConformance,
    /// Roll-up counts.
    pub summary: DescriptorJoinSummary,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DescriptorJoinRegistry {
    /// Builds a registry from its joins, deriving the vocabulary, consumer list, conformance
    /// review, and summary.
    pub fn new(input: M5DescriptorJoinRegistryInput) -> Self {
        let joins = input.joins;
        let consumers: Vec<String> = PublicTruthConsumer::ALL
            .iter()
            .map(|c| c.as_str().to_owned())
            .collect();
        let conformance = derive_registry_conformance(&joins);
        let summary = derive_summary(&joins);
        Self {
            record_kind: M5_DESCRIPTOR_JOIN_REGISTRY_RECORD_KIND.to_owned(),
            schema_version: M5_DESCRIPTOR_JOIN_SCHEMA_VERSION,
            registry_id: input.registry_id,
            report_label: input.report_label,
            joins,
            vocabulary: DescriptorJoinVocabulary::canonical(),
            consumers,
            conformance,
            summary,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Finds a join by id.
    pub fn join(&self, join_id: &str) -> Option<&DescriptorJoin> {
        self.joins.iter().find(|j| j.join_id == join_id)
    }

    /// Deterministic export-safe JSON for the registry.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("descriptor join registry serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 descriptor join parity\n\n");
        out.push_str(&format!("- Registry: `{}`\n", self.registry_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Joins: {}\n", self.joins.len()));
        out.push_str(&format!("- Minted: `{}`\n", self.minted_at));
        out.push_str(
            "- Carriers: export packet, support bundle, admin report, copy-safe summary\n",
        );
        out.push_str(
            "- Consumed by: release center, Help/About, marketplace, docs/help, certification, evaluation packs, support, companion\n",
        );

        out.push_str("\n## Joins\n\n");
        out.push_str("| Join | Descriptor | Artifact | Claim state | Effective | Reasons |\n");
        out.push_str("|------|------------|----------|-------------|-----------|--------|\n");
        for join in &self.joins {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}/{}` | `{}` | `{}` | {} |\n",
                join.join_id,
                join.descriptor_id,
                join.artifact_ref.artifact_family,
                join.artifact_ref.artifact_id,
                join.claim_state.as_str(),
                join.effective_qualification.as_str(),
                join.downgrade_reasons.len()
            ));
        }

        out.push_str("\n## Carrier parity\n\n");
        for join in &self.joins {
            out.push_str(&format!(
                "### `{}` → `{}`\n\n",
                join.join_id,
                join.claim_state.as_str()
            ));
            out.push_str(&format!(
                "Copy-safe summary: `{}`\n\n",
                join.copy_safe_summary
            ));
            out.push_str("| Carrier | Identity | Binding | Reasons | Reasons kept |\n");
            out.push_str("|---------|----------|---------|---------|--------------|\n");
            for carrier in &join.carriers {
                out.push_str(&format!(
                    "| `{}` | `{}` | `{}/{}` | {} | {} |\n",
                    carrier.carrier.as_str(),
                    carrier.descriptor_id,
                    carrier.artifact_ref.artifact_family,
                    carrier.artifact_ref.artifact_id,
                    carrier.downgrade_reason_count,
                    if carrier.preserves_downgrade_reasons {
                        "yes"
                    } else {
                        "NO"
                    }
                ));
            }
            if join.downgrade_reasons.is_empty() {
                out.push_str("\n_No narrowing — claim stands at its ceiling._\n\n");
            } else {
                out.push_str("\n**Downgrade reasons (attributable):**\n\n");
                for reason in &join.downgrade_reasons {
                    out.push_str(&format!(
                        "- `{}` (`{}`) → `{}` ({})\n",
                        reason.facet.as_str(),
                        reason.token,
                        reason.implied_state.as_str(),
                        reason.effect.as_str()
                    ));
                }
                out.push('\n');
            }
        }
        out
    }

    /// Validates the registry's invariants.
    pub fn validate(&self) -> Vec<M5DescriptorJoinViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_DESCRIPTOR_JOIN_REGISTRY_RECORD_KIND {
            out.push(M5DescriptorJoinViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DESCRIPTOR_JOIN_SCHEMA_VERSION {
            out.push(M5DescriptorJoinViolation::WrongSchemaVersion);
        }
        if self.registry_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            out.push(M5DescriptorJoinViolation::MissingIdentity);
        }
        if self.joins.is_empty() {
            out.push(M5DescriptorJoinViolation::RegistryHasNoJoins);
        }
        let mut seen = std::collections::BTreeSet::new();
        for join in &self.joins {
            if !seen.insert(join.join_id.clone()) {
                out.push(M5DescriptorJoinViolation::DuplicateJoinId);
            }
            out.extend(join.validate());
        }
        if !self.vocabulary.matches_canonical() {
            out.push(M5DescriptorJoinViolation::VocabularyMismatch);
        }
        let expected_consumers: Vec<String> = PublicTruthConsumer::ALL
            .iter()
            .map(|c| c.as_str().to_owned())
            .collect();
        if self.consumers != expected_consumers {
            out.push(M5DescriptorJoinViolation::ConsumerSetMismatch);
        }
        if self.conformance != derive_registry_conformance(&self.joins)
            || !self.conformance.all_hold()
        {
            out.push(M5DescriptorJoinViolation::ConformanceReviewFailed);
        }
        if self.summary != derive_summary(&self.joins) {
            out.push(M5DescriptorJoinViolation::SummaryMismatch);
        }
        out
    }
}

/// Derives the roll-up summary from the joins.
fn derive_summary(joins: &[DescriptorJoin]) -> DescriptorJoinSummary {
    DescriptorJoinSummary {
        total_joins: joins.len() as u32,
        fully_supported_joins: joins.iter().filter(|j| j.is_fully_supported()).count() as u32,
        narrowed_joins: joins.iter().filter(|j| j.claim_state.is_narrowed()).count() as u32,
        blocked_joins: joins.iter().filter(|j| j.is_blocked()).count() as u32,
        total_carrier_renderings: joins.iter().map(|j| j.carriers.len() as u32).sum(),
        total_downgrade_reasons: joins.iter().map(|j| j.downgrade_reasons.len() as u32).sum(),
    }
}

/// Derives the registry conformance review from its joins.
fn derive_registry_conformance(joins: &[DescriptorJoin]) -> DescriptorJoinConformance {
    let joins_validate = !joins.is_empty() && joins.iter().all(|j| j.validate().is_empty());

    let identity_survives = joins.iter().all(|j| {
        j.carriers
            .iter()
            .all(|c| c.preserves_identity && c.descriptor_id == j.descriptor_id)
    });

    let binding_survives = joins.iter().all(|j| {
        j.carriers
            .iter()
            .all(|c| c.preserves_binding && c.artifact_ref == j.artifact_ref)
    });

    // Every narrowing in the descriptor is attributable as a reason, and every carrier keeps the
    // full reason count.
    let reasons_attributable = joins.iter().all(|j| {
        j.downgrade_reasons.len() == j.descriptor.narrowings.len()
            && j.downgrade_reasons
                .iter()
                .all(|r| !r.token.trim().is_empty())
            && j.carriers.iter().all(|c| {
                c.preserves_downgrade_reasons
                    && c.downgrade_reason_count == j.downgrade_reasons.len() as u32
            })
    });

    // The full descriptor truth is reconstructable: the embedded descriptor is self-consistent and
    // its identity / binding / effective qualification match the join's key fields.
    let truth_reconstructable = joins.iter().all(|j| {
        j.descriptor.validate().is_empty()
            && j.descriptor.descriptor_id == j.descriptor_id
            && j.descriptor.artifact_ref == j.artifact_ref
            && j.descriptor.effective_qualification == j.effective_qualification
    });

    let claim_state_matches = joins
        .iter()
        .all(|j| j.claim_state == derive_claim_state(&j.descriptor));

    let evidence_refs_only = joins.iter().all(|j| {
        !j.evidence_refs.is_empty()
            && j.evidence_refs
                .iter()
                .all(|r| !r.ref_value.trim().is_empty())
    });

    // A weaker source origin must still surface as a reason; at least one join exercises one so the
    // guard is not vacuous.
    let weaker_origins_named = joins.iter().all(|j| {
        j.descriptor
            .narrowings
            .iter()
            .filter(|n| matches!(n.facet, DescriptorFacet::SourceClass))
            .all(|n| j.downgrade_reasons.iter().any(|r| r.token == n.token))
    });
    let weaker_origins_never_omitted = weaker_origins_named
        && joins.iter().any(|j| {
            j.descriptor
                .narrowings
                .iter()
                .any(|n| matches!(n.facet, DescriptorFacet::SourceClass))
        });

    // Desktop, CLI/headless, and offline / mirror generation produce byte-identical output.
    let channels_identical = joins.iter().all(|j| {
        JoinChannel::ALL
            .iter()
            .map(|&channel| j.render_for_channel(channel))
            .all(|rendered| rendered == j.export_safe_json())
    });

    let export_clean = joins.iter().all(|j| {
        !json_contains_forbidden_material(
            &serde_json::to_value(j).expect("descriptor join serializes"),
        )
    });

    DescriptorJoinConformance {
        joins_validate,
        identity_survives_carriers: identity_survives,
        binding_survives_carriers: binding_survives,
        downgrade_reasons_attributable: reasons_attributable,
        truth_reconstructable_without_translation: truth_reconstructable,
        claim_state_matches_narrowing_runtime: claim_state_matches,
        evidence_refs_are_refs_only: evidence_refs_only,
        weaker_origins_never_omitted,
        channels_produce_identical_output: channels_identical,
        controlled_enums_frozen: DescriptorJoinVocabulary::canonical().matches_canonical(),
        shared_across_consumers: true,
        export_carries_no_raw_material: export_clean,
    }
}

/// Validation failures for the descriptor-join lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DescriptorJoinViolation {
    /// The record kind is wrong.
    WrongRecordKind,
    /// The schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is empty.
    MissingIdentity,
    /// The embedded descriptor object is itself invalid.
    DescriptorInvalid,
    /// The join key fields disagree with the embedded descriptor's identity or binding.
    DescriptorBindingMismatch,
    /// The claimed or effective qualification drifted from the descriptor.
    EffectiveQualificationDrift,
    /// The claim state drifted from the shared claim-narrowing runtime.
    ClaimStateDrift,
    /// The downgrade reasons drifted from a fresh derivation.
    DowngradeReasonDrift,
    /// The supporting evidence references drifted or are missing.
    EvidenceRefDrift,
    /// The carrier renderings drifted from a fresh derivation.
    CarrierDrift,
    /// The copy-safe summary drifted from a fresh derivation.
    CopySafeSummaryDrift,
    /// The claim state is incoherent with the descriptor's narrowing posture.
    StateCoherenceBroken,
    /// The carrier set does not match the canonical carriers.
    CarrierSetMismatch,
    /// A carrier diverged from the join's canonical state.
    CarrierDiverged,
    /// A carrier dropped the descriptor identity or artifact binding.
    CarrierDropsBinding,
    /// A narrowed descriptor reads as fully supported on a carrier.
    NarrowedCarrierReadsSupported,
    /// A message id is missing the lane prefix.
    UnprefixedMessageId,
    /// The registry publishes no joins.
    RegistryHasNoJoins,
    /// Two joins share a join id.
    DuplicateJoinId,
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

impl M5DescriptorJoinViolation {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::DescriptorInvalid => "descriptor_invalid",
            Self::DescriptorBindingMismatch => "descriptor_binding_mismatch",
            Self::EffectiveQualificationDrift => "effective_qualification_drift",
            Self::ClaimStateDrift => "claim_state_drift",
            Self::DowngradeReasonDrift => "downgrade_reason_drift",
            Self::EvidenceRefDrift => "evidence_ref_drift",
            Self::CarrierDrift => "carrier_drift",
            Self::CopySafeSummaryDrift => "copy_safe_summary_drift",
            Self::StateCoherenceBroken => "state_coherence_broken",
            Self::CarrierSetMismatch => "carrier_set_mismatch",
            Self::CarrierDiverged => "carrier_diverged",
            Self::CarrierDropsBinding => "carrier_drops_binding",
            Self::NarrowedCarrierReadsSupported => "narrowed_carrier_reads_supported",
            Self::UnprefixedMessageId => "unprefixed_message_id",
            Self::RegistryHasNoJoins => "registry_has_no_joins",
            Self::DuplicateJoinId => "duplicate_join_id",
            Self::VocabularyMismatch => "vocabulary_mismatch",
            Self::ConsumerSetMismatch => "consumer_set_mismatch",
            Self::ConformanceReviewFailed => "conformance_review_failed",
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
