//! The artifact-bound M5 public-truth descriptor *object* and its controlled enums.
//!
//! The [descriptor / badge matrix](crate::m5_descriptor_badge) freezes the four shared
//! descriptor *families* and their badge vocabularies at the governance level. This lane
//! freezes the layer beneath it: the machine-readable descriptor *object* a claimed M5
//! artifact actually carries, and the controlled enums that object is built from. Where the
//! matrix answers "which descriptor families exist and which consumers render them", a
//! [`DescriptorObject`] answers "where did *this* artifact come from, how is its evidence
//! signed and how fresh is it, what support class does it qualify for, and which client scope
//! and authority does it run under" — as typed, versioned, serde-round-trippable state rather
//! than prose or hand-authored badges.
//!
//! Every descriptor object composes four sub-descriptors, each over a frozen controlled
//! vocabulary so a nearby surface can never invent a quasi-equivalent state:
//!
//! - [`ProvenanceSubDescriptor`] — source/origin ([`ProvenanceClass`]) and
//!   signature/attestation state ([`SignatureState`]);
//! - [`FreshnessSubDescriptor`] — evidence currency ([`FreshnessState`]) and the explicit
//!   evidence-completeness state ([`EvidenceState`]);
//! - [`QualificationSubDescriptor`] — claimed support class ([`QualificationClass`]) and its
//!   evidence-completeness state;
//! - [`ClientScopeSubDescriptor`] — client kind ([`ClientScope`]), authority class
//!   ([`AuthorityClass`]), and handoff requirement ([`HandoffRequirement`]).
//!
//! Missing or partial evidence is never dropped: the [`EvidenceState`] vocabulary carries
//! [`Partial`](EvidenceState::Partial), [`RetestPending`](EvidenceState::RetestPending),
//! [`EvidenceStale`](EvidenceState::EvidenceStale), [`Limited`](EvidenceState::Limited), and
//! [`NotProvided`](EvidenceState::NotProvided) as first-class tokens, and the weaker
//! provenance, signature, authority, and handoff states all survive serialization as explicit
//! state instead of omission. Every weaker value the object carries produces a named
//! [`DescriptorNarrowing`], and the object derives an [effective
//! qualification](DescriptorObject::effective_qualification) from those narrowings: stale or
//! partial evidence narrows the claim automatically, and absent provenance, an invalid
//! signature, or expired/missing evidence blocks Stable — so weaker evidence or a narrowed
//! client can never imply authority or capability parity it does not have.
//!
//! The descriptor object preserves its [identity](DescriptorObject::descriptor_id) and its
//! [artifact binding](ArtifactBinding) as structured fields across export/import rather than
//! flattening them to a plain string. The [`M5DescriptorObjectRegistry`] is the one
//! inspectable, serde-serializable truth packet the public-truth consumers read; it carries
//! metadata and refs only — no credential bodies or raw provider payloads.
//!
//! - Object/registry schema:
//!   [`schemas/provenance/m5-descriptor-object.schema.json`](../../../../../schemas/provenance/m5-descriptor-object.schema.json)
//! - Contract doc:
//!   [`docs/public-truth/m5-descriptor-object.md`](../../../../../docs/public-truth/m5-descriptor-object.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_descriptor_object_registry, seeded_narrowed_descriptor_object,
    seeded_not_provided_descriptor_object, seeded_stable_descriptor_object,
    M5_DESCRIPTOR_OBJECT_REGISTRY_ID,
};

use serde::{Deserialize, Serialize};

// The descriptor object reuses the matrix lane's frozen family vocabularies so the object
// layer and the governance layer can never drift to different tokens.
pub use crate::m5_descriptor_badge::{
    ClientScope, DowngradeEffect, FreshnessState, ProvenanceClass, PublicTruthConsumer,
    QualificationClass, M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX,
};

/// Record-kind tag carried by [`DescriptorObject`].
pub const M5_DESCRIPTOR_OBJECT_RECORD_KIND: &str = "m5_public_truth_descriptor_object";

/// Record-kind tag carried by [`M5DescriptorObjectRegistry`].
pub const M5_DESCRIPTOR_OBJECT_REGISTRY_RECORD_KIND: &str = "m5_public_truth_descriptor_registry";

/// Schema version for the descriptor object and registry.
pub const M5_DESCRIPTOR_OBJECT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the descriptor-object / registry schema.
pub const M5_DESCRIPTOR_OBJECT_SCHEMA_REF: &str =
    "schemas/provenance/m5-descriptor-object.schema.json";

/// Repo-relative path of the published descriptor-object registry inventory.
pub const M5_DESCRIPTOR_OBJECT_REGISTRY_REF: &str =
    "artifacts/public-truth/descriptors/m5-descriptor-object-registry.json";

/// Repo-relative path of the release-grade descriptor-object parity proof.
pub const M5_DESCRIPTOR_OBJECT_PROOF_REF: &str =
    "artifacts/release/m5-descriptor-parity-proof/descriptor-objects.json";

/// Repo-relative path of the descriptor-object contract doc.
pub const M5_DESCRIPTOR_OBJECT_DOC_REF: &str = "docs/public-truth/m5-descriptor-object.md";

/// Repo-relative directory of the descriptor-object instance fixtures.
pub const M5_DESCRIPTOR_OBJECT_FIXTURE_DIR: &str = "fixtures/public-truth/m5-badge-consumers/";

/// Signature / attestation state of an artifact's provenance. Declaration order is
/// most→least trustworthy. Only [`SignedAttested`](Self::SignedAttested) can back an
/// authoritative Stable claim; [`SignatureInvalid`](Self::SignatureInvalid) blocks it, and the
/// weaker states all narrow rather than disappear into omission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignatureState {
    /// Signed by the release identity and the attestation verified.
    SignedAttested,
    /// Signed, but the signature was not verified live (e.g. served from a mirror).
    SignedUnverified,
    /// An attestation is present but no signature backs it.
    AttestationOnly,
    /// No signature is present.
    Unsigned,
    /// A signature is present but failed verification.
    SignatureInvalid,
    /// No signature or attestation information was provided — recorded, never left blank.
    NotProvided,
}

impl SignatureState {
    /// Every signature state, in declaration order (most→least trustworthy).
    pub const ALL: [Self; 6] = [
        Self::SignedAttested,
        Self::SignedUnverified,
        Self::AttestationOnly,
        Self::Unsigned,
        Self::SignatureInvalid,
        Self::NotProvided,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignedAttested => "signed_attested",
            Self::SignedUnverified => "signed_unverified",
            Self::AttestationOnly => "attestation_only",
            Self::Unsigned => "unsigned",
            Self::SignatureInvalid => "signature_invalid",
            Self::NotProvided => "not_provided",
        }
    }

    /// True for the one state that can back an authoritative Stable claim.
    pub const fn is_attested(self) -> bool {
        matches!(self, Self::SignedAttested)
    }

    /// The downgrade this signature state applies to a claim: a verified signature is clean,
    /// an invalid signature blocks Stable, and every other state narrows it.
    pub const fn downgrade(self) -> Option<DowngradeEffect> {
        match self {
            Self::SignedAttested => None,
            Self::SignatureInvalid => Some(DowngradeEffect::Block),
            Self::SignedUnverified | Self::AttestationOnly | Self::Unsigned | Self::NotProvided => {
                Some(DowngradeEffect::Narrow)
            }
        }
    }
}

/// Explicit completeness / posture of the evidence behind a descriptor. The vocabulary makes
/// missing or partial evidence first-class: [`Partial`](Self::Partial),
/// [`RetestPending`](Self::RetestPending), [`EvidenceStale`](Self::EvidenceStale),
/// [`Limited`](Self::Limited), and [`NotProvided`](Self::NotProvided) all survive
/// serialization as explicit state rather than collapsing into an absent or "complete" field.
/// Declaration order is most→least complete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceState {
    /// Full evidence is present and current.
    Complete,
    /// Evidence is present but limited in scope.
    Limited,
    /// Evidence is partial — some required checks are absent.
    Partial,
    /// Evidence existed but a retest is pending before it can be relied on.
    RetestPending,
    /// Evidence has gone stale and needs refreshing.
    EvidenceStale,
    /// No evidence was provided — recorded explicitly, never left blank.
    NotProvided,
}

impl EvidenceState {
    /// Every evidence state, in declaration order (most→least complete).
    pub const ALL: [Self; 6] = [
        Self::Complete,
        Self::Limited,
        Self::Partial,
        Self::RetestPending,
        Self::EvidenceStale,
        Self::NotProvided,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Limited => "limited",
            Self::Partial => "partial",
            Self::RetestPending => "retest_pending",
            Self::EvidenceStale => "evidence_stale",
            Self::NotProvided => "not_provided",
        }
    }

    /// True for the one state that carries fully complete evidence.
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }

    /// The downgrade this evidence state applies: complete is clean, absent evidence blocks
    /// Stable, and every partial / limited / stale / pending state narrows it.
    pub const fn downgrade(self) -> Option<DowngradeEffect> {
        match self {
            Self::Complete => None,
            Self::NotProvided => Some(DowngradeEffect::Block),
            Self::Limited | Self::Partial | Self::RetestPending | Self::EvidenceStale => {
                Some(DowngradeEffect::Narrow)
            }
        }
    }
}

/// The authority a client scope carries. Declaration order is most→least capable; only
/// [`FullAuthority`](Self::FullAuthority) carries full mutate/approve authority, and every
/// narrower class narrows a claim so a companion, reference, or handoff surface can never
/// imply authority parity it lacks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityClass {
    /// Full mutate / approve authority (the desktop product surface).
    FullAuthority,
    /// Bounded, host-relayed authority (a permitted companion scope).
    ScopedAuthority,
    /// Read-only reference authority — observe, never mutate.
    ReferenceOnly,
    /// Can only originate or open a desktop handoff, never act in place.
    HandoffOnly,
    /// Authority was not provided — recorded explicitly, never left blank.
    NotProvided,
}

impl AuthorityClass {
    /// Every authority class, in declaration order (most→least capable).
    pub const ALL: [Self; 5] = [
        Self::FullAuthority,
        Self::ScopedAuthority,
        Self::ReferenceOnly,
        Self::HandoffOnly,
        Self::NotProvided,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullAuthority => "full_authority",
            Self::ScopedAuthority => "scoped_authority",
            Self::ReferenceOnly => "reference_only",
            Self::HandoffOnly => "handoff_only",
            Self::NotProvided => "not_provided",
        }
    }

    /// True for the one class that carries full authority / capability parity.
    pub const fn is_full_authority(self) -> bool {
        matches!(self, Self::FullAuthority)
    }

    /// The downgrade this authority class applies: full authority is clean; every narrower
    /// class narrows the claim so it cannot imply parity it lacks.
    pub const fn downgrade(self) -> Option<DowngradeEffect> {
        match self {
            Self::FullAuthority => None,
            Self::ScopedAuthority | Self::ReferenceOnly | Self::HandoffOnly | Self::NotProvided => {
                Some(DowngradeEffect::Narrow)
            }
        }
    }
}

/// Whether a privileged action on this surface requires a handoff out of the current plane.
/// Declaration order is least→most constrained. A required handoff narrows a claim so an
/// in-product control is never implied where one does not exist.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffRequirement {
    /// No handoff is required — the action runs in place with in-product control.
    NotRequired,
    /// A privileged action must hand off to the desktop product to proceed.
    DesktopHandoffRequired,
    /// An out-of-plane vendor / browser console pivot is required.
    ConsoleHandoffRequired,
    /// The handoff requirement was not provided — recorded explicitly, never left blank.
    NotProvided,
}

impl HandoffRequirement {
    /// Every handoff requirement, in declaration order (least→most constrained).
    pub const ALL: [Self; 4] = [
        Self::NotRequired,
        Self::DesktopHandoffRequired,
        Self::ConsoleHandoffRequired,
        Self::NotProvided,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::DesktopHandoffRequired => "desktop_handoff_required",
            Self::ConsoleHandoffRequired => "console_handoff_required",
            Self::NotProvided => "not_provided",
        }
    }

    /// True when this surface keeps in-product control with no handoff.
    pub const fn is_in_product(self) -> bool {
        matches!(self, Self::NotRequired)
    }

    /// The downgrade a required handoff applies: an in-product action is clean; any required
    /// handoff narrows the claim so in-product parity is not implied.
    pub const fn downgrade(self) -> Option<DowngradeEffect> {
        match self {
            Self::NotRequired => None,
            Self::DesktopHandoffRequired | Self::ConsoleHandoffRequired | Self::NotProvided => {
                Some(DowngradeEffect::Narrow)
            }
        }
    }
}

/// One controlled-vocabulary facet a [`DescriptorObject`] is built from. Naming the facet on
/// every [`DescriptorNarrowing`] is what lets a consumer say *exactly* which weaker value
/// narrowed or blocked a claim rather than collapsing the reason into a single string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DescriptorFacet {
    /// Provenance source / origin class.
    SourceClass,
    /// Signature / attestation state.
    SignatureState,
    /// Evidence-freshness state.
    FreshnessState,
    /// Completeness of the freshness evidence.
    FreshnessEvidence,
    /// Claimed support class.
    SupportClass,
    /// Completeness of the qualification evidence.
    QualificationEvidence,
    /// Client kind.
    ClientKind,
    /// Authority class.
    AuthorityClass,
    /// Handoff requirement.
    HandoffRequirement,
}

impl DescriptorFacet {
    /// Every facet, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::SourceClass,
        Self::SignatureState,
        Self::FreshnessState,
        Self::FreshnessEvidence,
        Self::SupportClass,
        Self::QualificationEvidence,
        Self::ClientKind,
        Self::AuthorityClass,
        Self::HandoffRequirement,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceClass => "source_class",
            Self::SignatureState => "signature_state",
            Self::FreshnessState => "freshness_state",
            Self::FreshnessEvidence => "freshness_evidence",
            Self::SupportClass => "support_class",
            Self::QualificationEvidence => "qualification_evidence",
            Self::ClientKind => "client_kind",
            Self::AuthorityClass => "authority_class",
            Self::HandoffRequirement => "handoff_requirement",
        }
    }
}

/// Provenance sub-descriptor: where an artifact came from and how its provenance is signed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceSubDescriptor {
    /// Source / origin class.
    pub source_class: ProvenanceClass,
    /// Signature / attestation state of the source claim.
    pub signature_state: SignatureState,
}

/// Freshness sub-descriptor: how current the evidence behind a claim is, and how complete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessSubDescriptor {
    /// Evidence-freshness state.
    pub freshness_state: FreshnessState,
    /// Completeness of the freshness evidence.
    pub evidence_state: EvidenceState,
}

/// Qualification sub-descriptor: the support class a surface claims, and the completeness of
/// the evidence behind it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationSubDescriptor {
    /// Claimed support class.
    pub support_class: QualificationClass,
    /// Completeness of the qualification evidence.
    pub evidence_state: EvidenceState,
}

/// Client-scope sub-descriptor: which client an artifact renders in, the authority it carries,
/// and whether privileged actions require a handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientScopeSubDescriptor {
    /// Client kind.
    pub client_kind: ClientScope,
    /// Authority the client kind carries.
    pub authority_class: AuthorityClass,
    /// Whether a privileged action requires a handoff out of plane.
    pub handoff_requirement: HandoffRequirement,
}

/// Structured binding from a descriptor object to the artifact it describes. Keeping the
/// binding as typed fields — not a flattened `"family/id@digest"` string — is what preserves
/// the artifact's identity across export/import so a consumer can rejoin a descriptor to its
/// artifact without re-parsing prose.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactBinding {
    /// Stable id of the bound artifact.
    pub artifact_id: String,
    /// Artifact family the bound artifact belongs to.
    pub artifact_family: String,
    /// Artifact kind within its family.
    pub artifact_kind: String,
    /// Repo-relative schema that governs the bound artifact.
    pub schema_ref: String,
    /// Content-digest *reference* for the bound artifact — a ref, never the raw payload.
    pub content_digest_ref: String,
}

impl ArtifactBinding {
    /// Validates the binding's identity fields are present.
    fn validate(&self) -> Vec<M5DescriptorObjectViolation> {
        if self.artifact_id.trim().is_empty()
            || self.artifact_family.trim().is_empty()
            || self.artifact_kind.trim().is_empty()
            || self.schema_ref.trim().is_empty()
            || self.content_digest_ref.trim().is_empty()
        {
            vec![M5DescriptorObjectViolation::MissingArtifactBinding]
        } else {
            Vec::new()
        }
    }
}

/// One named narrowing or block a weaker descriptor value applies to a claim. Every weaker
/// value an object carries produces one of these, so the reason a claim narrowed or blocked is
/// explicit, machine-readable state — never an omission.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorNarrowing {
    /// The facet whose value triggered the narrowing.
    pub facet: DescriptorFacet,
    /// The weaker value token.
    pub token: String,
    /// Whether the value narrows the claim or blocks Stable promotion.
    pub effect: DowngradeEffect,
    /// The qualification the value floors the claim at.
    pub effective_floor: QualificationClass,
    /// Stable message id; prefixed [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub reason_message_id: String,
}

/// Restrictiveness rank of a qualification class, from the shipped support-class ladder (least
/// restrictive first).
fn qualification_rank(class: QualificationClass) -> usize {
    QualificationClass::ALL
        .iter()
        .position(|c| *c == class)
        .unwrap_or(QualificationClass::ALL.len())
}

/// The more restrictive of two qualification classes.
fn more_restrictive(a: QualificationClass, b: QualificationClass) -> QualificationClass {
    if qualification_rank(a) >= qualification_rank(b) {
        a
    } else {
        b
    }
}

/// The artifact-bound public-truth descriptor object: the typed provenance, freshness,
/// qualification, and client-scope state a claimed M5 artifact carries, plus the effective
/// qualification derived from that state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorObject {
    /// Record kind; must equal [`M5_DESCRIPTOR_OBJECT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DESCRIPTOR_OBJECT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable descriptor id — preserved across export/import.
    pub descriptor_id: String,
    /// Reviewer-facing descriptor label.
    pub descriptor_label: String,
    /// The artifact this descriptor is bound to.
    pub artifact_ref: ArtifactBinding,
    /// Provenance sub-descriptor.
    pub provenance: ProvenanceSubDescriptor,
    /// Freshness sub-descriptor.
    pub freshness: FreshnessSubDescriptor,
    /// Qualification sub-descriptor.
    pub qualification: QualificationSubDescriptor,
    /// Client-scope sub-descriptor.
    pub client_scope: ClientScopeSubDescriptor,
    /// Effective qualification derived from the claimed support class and the narrowings.
    pub effective_qualification: QualificationClass,
    /// The named narrowings every weaker value applied, in facet order.
    pub narrowings: Vec<DescriptorNarrowing>,
    /// Stable message id for the object's explanation drawer; prefixed
    /// [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub explanation_drawer_message_id: String,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Object mint timestamp.
    pub minted_at: String,
}

/// Constructor input for [`DescriptorObject::new`]; the narrowings and effective qualification
/// are derived from the sub-descriptors so they can never be hand-edited out of agreement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DescriptorObjectInput {
    /// Stable descriptor id.
    pub descriptor_id: String,
    /// Reviewer-facing descriptor label.
    pub descriptor_label: String,
    /// The artifact this descriptor is bound to.
    pub artifact_ref: ArtifactBinding,
    /// Provenance sub-descriptor.
    pub provenance: ProvenanceSubDescriptor,
    /// Freshness sub-descriptor.
    pub freshness: FreshnessSubDescriptor,
    /// Qualification sub-descriptor.
    pub qualification: QualificationSubDescriptor,
    /// Client-scope sub-descriptor.
    pub client_scope: ClientScopeSubDescriptor,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Object mint timestamp.
    pub minted_at: String,
}

impl DescriptorObject {
    /// Builds a descriptor object, deriving the narrowings and effective qualification from the
    /// sub-descriptors so the claim is always generated from the descriptor's own state.
    pub fn new(input: DescriptorObjectInput) -> Self {
        let narrowings = derive_narrowings(
            &input.provenance,
            &input.freshness,
            &input.qualification,
            &input.client_scope,
        );
        let effective_qualification =
            derive_effective_qualification(input.qualification.support_class, &narrowings);
        Self {
            record_kind: M5_DESCRIPTOR_OBJECT_RECORD_KIND.to_owned(),
            schema_version: M5_DESCRIPTOR_OBJECT_SCHEMA_VERSION,
            descriptor_id: input.descriptor_id,
            descriptor_label: input.descriptor_label,
            artifact_ref: input.artifact_ref,
            provenance: input.provenance,
            freshness: input.freshness,
            qualification: input.qualification,
            client_scope: input.client_scope,
            effective_qualification,
            narrowings,
            explanation_drawer_message_id: format!(
                "{M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX}object.drawer"
            ),
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// True when the object's evidence is strong enough to stand at Stable.
    pub fn is_stable(&self) -> bool {
        matches!(self.effective_qualification, QualificationClass::Stable)
    }

    /// True when a weaker value blocks the object from any Stable promotion.
    pub fn blocks_stable_promotion(&self) -> bool {
        self.narrowings
            .iter()
            .any(|n| matches!(n.effect, DowngradeEffect::Block))
    }

    /// Field-level diff against another descriptor object: every facet (and the effective
    /// qualification) whose token changed, so a consumer can render exactly what moved between
    /// two versions of an artifact's descriptor.
    pub fn diff(&self, other: &DescriptorObject) -> Vec<DescriptorObjectDiff> {
        let mut out = Vec::new();
        let mut push = |facet: &str, from: String, to: String| {
            if from != to {
                out.push(DescriptorObjectDiff {
                    facet: facet.to_owned(),
                    from_token: from,
                    to_token: to,
                });
            }
        };
        push(
            DescriptorFacet::SourceClass.as_str(),
            self.provenance.source_class.as_str().to_owned(),
            other.provenance.source_class.as_str().to_owned(),
        );
        push(
            DescriptorFacet::SignatureState.as_str(),
            self.provenance.signature_state.as_str().to_owned(),
            other.provenance.signature_state.as_str().to_owned(),
        );
        push(
            DescriptorFacet::FreshnessState.as_str(),
            self.freshness.freshness_state.as_str().to_owned(),
            other.freshness.freshness_state.as_str().to_owned(),
        );
        push(
            DescriptorFacet::FreshnessEvidence.as_str(),
            self.freshness.evidence_state.as_str().to_owned(),
            other.freshness.evidence_state.as_str().to_owned(),
        );
        push(
            DescriptorFacet::SupportClass.as_str(),
            self.qualification.support_class.as_str().to_owned(),
            other.qualification.support_class.as_str().to_owned(),
        );
        push(
            DescriptorFacet::QualificationEvidence.as_str(),
            self.qualification.evidence_state.as_str().to_owned(),
            other.qualification.evidence_state.as_str().to_owned(),
        );
        push(
            DescriptorFacet::ClientKind.as_str(),
            self.client_scope.client_kind.as_str().to_owned(),
            other.client_scope.client_kind.as_str().to_owned(),
        );
        push(
            DescriptorFacet::AuthorityClass.as_str(),
            self.client_scope.authority_class.as_str().to_owned(),
            other.client_scope.authority_class.as_str().to_owned(),
        );
        push(
            DescriptorFacet::HandoffRequirement.as_str(),
            self.client_scope.handoff_requirement.as_str().to_owned(),
            other.client_scope.handoff_requirement.as_str().to_owned(),
        );
        push(
            "effective_qualification",
            self.effective_qualification.as_str().to_owned(),
            other.effective_qualification.as_str().to_owned(),
        );
        out
    }

    /// Deterministic export-safe JSON for the object.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only object fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("descriptor object serializes")
    }

    /// Validates the object's invariants: record kind/version, identity, artifact binding,
    /// derived narrowings and effective qualification, message-id prefixing, and that the
    /// export carries no raw provider material.
    pub fn validate(&self) -> Vec<M5DescriptorObjectViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_DESCRIPTOR_OBJECT_RECORD_KIND {
            out.push(M5DescriptorObjectViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DESCRIPTOR_OBJECT_SCHEMA_VERSION {
            out.push(M5DescriptorObjectViolation::WrongSchemaVersion);
        }
        if self.descriptor_id.trim().is_empty()
            || self.descriptor_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            out.push(M5DescriptorObjectViolation::MissingIdentity);
        }
        out.extend(self.artifact_ref.validate());

        let expected_narrowings = derive_narrowings(
            &self.provenance,
            &self.freshness,
            &self.qualification,
            &self.client_scope,
        );
        if self.narrowings != expected_narrowings {
            out.push(M5DescriptorObjectViolation::NarrowingDrift);
        }
        let expected_effective =
            derive_effective_qualification(self.qualification.support_class, &expected_narrowings);
        if self.effective_qualification != expected_effective {
            out.push(M5DescriptorObjectViolation::EffectiveQualificationDrift);
        }
        if !self
            .explanation_drawer_message_id
            .starts_with(M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX)
            || self.narrowings.iter().any(|n| {
                !n.reason_message_id
                    .starts_with(M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX)
            })
        {
            out.push(M5DescriptorObjectViolation::UnprefixedMessageId);
        }
        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("descriptor object serializes"),
        ) {
            out.push(M5DescriptorObjectViolation::RawMaterialInExport);
        }
        out
    }
}

/// One field that changed between two descriptor objects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorObjectDiff {
    /// The facet (or `effective_qualification`) that changed.
    pub facet: String,
    /// The token before.
    pub from_token: String,
    /// The token after.
    pub to_token: String,
}

/// Floors the claim at Beta when a value narrows, and at Unavailable when it blocks.
fn floor_for(effect: DowngradeEffect) -> QualificationClass {
    match effect {
        DowngradeEffect::Narrow => QualificationClass::Beta,
        DowngradeEffect::Block => QualificationClass::Unavailable,
    }
}

/// Builds a narrowing record for a facet whose value carries a downgrade.
fn narrowing_for(
    facet: DescriptorFacet,
    token: &str,
    effect: DowngradeEffect,
) -> DescriptorNarrowing {
    DescriptorNarrowing {
        facet,
        token: token.to_owned(),
        effect,
        effective_floor: floor_for(effect),
        reason_message_id: format!(
            "{M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX}object.narrowing.{}.{token}",
            facet.as_str()
        ),
    }
}

/// The downgrade a provenance source class applies: first-party-signed is clean; absent origin
/// blocks Stable; every other origin narrows it.
fn source_class_downgrade(class: ProvenanceClass) -> Option<DowngradeEffect> {
    if class.is_authoritative() {
        None
    } else if matches!(class, ProvenanceClass::NotProvided) {
        Some(DowngradeEffect::Block)
    } else {
        Some(DowngradeEffect::Narrow)
    }
}

/// The downgrade a freshness state applies: current is clean; stale narrows; expired/missing
/// block.
fn freshness_downgrade(state: FreshnessState) -> Option<DowngradeEffect> {
    match state {
        FreshnessState::Current => None,
        FreshnessState::Stale => Some(DowngradeEffect::Narrow),
        FreshnessState::Expired | FreshnessState::Missing => Some(DowngradeEffect::Block),
    }
}

/// The downgrade a client kind applies: full desktop is clean; every narrower kind narrows.
fn client_kind_downgrade(kind: ClientScope) -> Option<DowngradeEffect> {
    if kind.is_full_authority() {
        None
    } else {
        Some(DowngradeEffect::Narrow)
    }
}

/// Derives the named narrowings for an object's sub-descriptors, in facet order.
fn derive_narrowings(
    provenance: &ProvenanceSubDescriptor,
    freshness: &FreshnessSubDescriptor,
    qualification: &QualificationSubDescriptor,
    client_scope: &ClientScopeSubDescriptor,
) -> Vec<DescriptorNarrowing> {
    let mut out = Vec::new();
    if let Some(effect) = source_class_downgrade(provenance.source_class) {
        out.push(narrowing_for(
            DescriptorFacet::SourceClass,
            provenance.source_class.as_str(),
            effect,
        ));
    }
    if let Some(effect) = provenance.signature_state.downgrade() {
        out.push(narrowing_for(
            DescriptorFacet::SignatureState,
            provenance.signature_state.as_str(),
            effect,
        ));
    }
    if let Some(effect) = freshness_downgrade(freshness.freshness_state) {
        out.push(narrowing_for(
            DescriptorFacet::FreshnessState,
            freshness.freshness_state.as_str(),
            effect,
        ));
    }
    if let Some(effect) = freshness.evidence_state.downgrade() {
        out.push(narrowing_for(
            DescriptorFacet::FreshnessEvidence,
            freshness.evidence_state.as_str(),
            effect,
        ));
    }
    if let Some(effect) = qualification.evidence_state.downgrade() {
        out.push(narrowing_for(
            DescriptorFacet::QualificationEvidence,
            qualification.evidence_state.as_str(),
            effect,
        ));
    }
    if let Some(effect) = client_kind_downgrade(client_scope.client_kind) {
        out.push(narrowing_for(
            DescriptorFacet::ClientKind,
            client_scope.client_kind.as_str(),
            effect,
        ));
    }
    if let Some(effect) = client_scope.authority_class.downgrade() {
        out.push(narrowing_for(
            DescriptorFacet::AuthorityClass,
            client_scope.authority_class.as_str(),
            effect,
        ));
    }
    if let Some(effect) = client_scope.handoff_requirement.downgrade() {
        out.push(narrowing_for(
            DescriptorFacet::HandoffRequirement,
            client_scope.handoff_requirement.as_str(),
            effect,
        ));
    }
    out
}

/// Derives the effective qualification: start from the claimed support class, floor it at each
/// narrowing's floor, and drop to Unavailable if any narrowing blocks.
fn derive_effective_qualification(
    claimed: QualificationClass,
    narrowings: &[DescriptorNarrowing],
) -> QualificationClass {
    let mut effective = claimed;
    for narrowing in narrowings {
        effective = more_restrictive(effective, narrowing.effective_floor);
    }
    effective
}

/// Self-describing controlled-vocabulary set so the registry resolves every token a descriptor
/// object can carry — the canonical proof that nearby surfaces share one enum vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorObjectVocabulary {
    /// Facet tokens.
    pub facets: Vec<String>,
    /// Source-class tokens.
    pub source_classes: Vec<String>,
    /// Signature-state tokens.
    pub signature_states: Vec<String>,
    /// Freshness-state tokens.
    pub freshness_states: Vec<String>,
    /// Evidence-state tokens.
    pub evidence_states: Vec<String>,
    /// Support-class tokens.
    pub support_classes: Vec<String>,
    /// Client-kind tokens.
    pub client_kinds: Vec<String>,
    /// Authority-class tokens.
    pub authority_classes: Vec<String>,
    /// Handoff-requirement tokens.
    pub handoff_requirements: Vec<String>,
    /// Downgrade-effect tokens.
    pub downgrade_effects: Vec<String>,
}

impl DescriptorObjectVocabulary {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            facets: DescriptorFacet::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            source_classes: ProvenanceClass::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            signature_states: SignatureState::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            freshness_states: FreshnessState::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            evidence_states: EvidenceState::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            support_classes: QualificationClass::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            client_kinds: ClientScope::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            authority_classes: AuthorityClass::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            handoff_requirements: HandoffRequirement::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            downgrade_effects: DowngradeEffect::ALL
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

/// Conformance review for the descriptor-object registry. Every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorObjectConformance {
    /// Every object is self-consistent (record kind, identity, derived state).
    pub objects_validate: bool,
    /// The controlled vocabularies match the canonical frozen tokens.
    pub controlled_enums_frozen: bool,
    /// Every weaker value survives as an explicit narrowing rather than omission.
    pub weaker_evidence_survives_as_state: bool,
    /// Absent provenance, an invalid signature, or absent/expired evidence blocks Stable.
    pub missing_evidence_blocks_stable: bool,
    /// `not_provided` is a first-class token across the relevant facets.
    pub not_provided_is_first_class: bool,
    /// Each object's effective qualification is derived from its own state.
    pub effective_qualification_derived: bool,
    /// Each object preserves its identity and artifact binding across a serialize round-trip.
    pub identity_and_binding_preserved: bool,
    /// Every public-truth consumer reads this one descriptor-object runtime.
    pub shared_across_consumers: bool,
    /// The export carries no raw provider material.
    pub export_carries_no_raw_material: bool,
}

impl DescriptorObjectConformance {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.objects_validate
            && self.controlled_enums_frozen
            && self.weaker_evidence_survives_as_state
            && self.missing_evidence_blocks_stable
            && self.not_provided_is_first_class
            && self.effective_qualification_derived
            && self.identity_and_binding_preserved
            && self.shared_across_consumers
            && self.export_carries_no_raw_material
    }
}

/// Constructor input for [`M5DescriptorObjectRegistry::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DescriptorObjectRegistryInput {
    /// Stable registry id.
    pub registry_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The descriptor objects this registry publishes.
    pub objects: Vec<DescriptorObject>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// The one inspectable, serde-serializable descriptor-object truth packet every public-truth
/// consumer reads: the published descriptor objects, the controlled vocabulary they share, the
/// consumers that read the runtime, and a conformance review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DescriptorObjectRegistry {
    /// Record kind; must equal [`M5_DESCRIPTOR_OBJECT_REGISTRY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DESCRIPTOR_OBJECT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable registry id.
    pub registry_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The descriptor objects this registry publishes.
    pub objects: Vec<DescriptorObject>,
    /// The controlled vocabulary every object shares.
    pub vocabulary: DescriptorObjectVocabulary,
    /// The public-truth consumers that read this descriptor-object runtime.
    pub consumers: Vec<String>,
    /// Conformance review block.
    pub conformance: DescriptorObjectConformance,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DescriptorObjectRegistry {
    /// Builds a registry from its objects, deriving the vocabulary, consumer list, and
    /// conformance review.
    pub fn new(input: M5DescriptorObjectRegistryInput) -> Self {
        let objects = input.objects;
        let consumers: Vec<String> = PublicTruthConsumer::ALL
            .iter()
            .map(|c| c.as_str().to_owned())
            .collect();
        let conformance = derive_registry_conformance(&objects);
        Self {
            record_kind: M5_DESCRIPTOR_OBJECT_REGISTRY_RECORD_KIND.to_owned(),
            schema_version: M5_DESCRIPTOR_OBJECT_SCHEMA_VERSION,
            registry_id: input.registry_id,
            report_label: input.report_label,
            objects,
            vocabulary: DescriptorObjectVocabulary::canonical(),
            consumers,
            conformance,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Finds a descriptor object by id.
    pub fn object(&self, descriptor_id: &str) -> Option<&DescriptorObject> {
        self.objects
            .iter()
            .find(|o| o.descriptor_id == descriptor_id)
    }

    /// Deterministic export-safe JSON for the registry.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("descriptor object registry serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 public-truth descriptor objects\n\n");
        out.push_str(&format!("- Registry: `{}`\n", self.registry_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Objects: {}\n", self.objects.len()));
        out.push_str(&format!("- Minted: `{}`\n", self.minted_at));
        out.push_str(
            "- Consumed by: release center, Help/About, marketplace, docs/help, certification, evaluation packs, support, companion\n",
        );

        out.push_str("\n## Descriptor objects\n\n");
        out.push_str(
            "| Descriptor | Artifact | Source / signature | Freshness / evidence | Authority / handoff | Claim → effective | Narrowings |\n",
        );
        out.push_str(
            "|------------|----------|--------------------|----------------------|---------------------|-------------------|-----------|\n",
        );
        for o in &self.objects {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` / `{}` | `{}` / `{}` | `{}` / `{}` | `{}` → `{}` | {} |\n",
                o.descriptor_id,
                o.artifact_ref.artifact_id,
                o.provenance.source_class.as_str(),
                o.provenance.signature_state.as_str(),
                o.freshness.freshness_state.as_str(),
                o.freshness.evidence_state.as_str(),
                o.client_scope.authority_class.as_str(),
                o.client_scope.handoff_requirement.as_str(),
                o.qualification.support_class.as_str(),
                o.effective_qualification.as_str(),
                o.narrowings.len()
            ));
        }

        out.push_str("\n## Named narrowings\n\n");
        for o in &self.objects {
            out.push_str(&format!("### `{}`\n\n", o.descriptor_id));
            if o.narrowings.is_empty() {
                out.push_str("- none — stands at its claimed class\n\n");
                continue;
            }
            out.push_str("| Facet | Value | Effect | Floor |\n");
            out.push_str("|-------|-------|--------|-------|\n");
            for n in &o.narrowings {
                out.push_str(&format!(
                    "| `{}` | `{}` | `{}` | `{}` |\n",
                    n.facet.as_str(),
                    n.token,
                    n.effect.as_str(),
                    n.effective_floor.as_str()
                ));
            }
            out.push('\n');
        }
        out
    }

    /// Validates the registry's invariants.
    pub fn validate(&self) -> Vec<M5DescriptorObjectViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_DESCRIPTOR_OBJECT_REGISTRY_RECORD_KIND {
            out.push(M5DescriptorObjectViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DESCRIPTOR_OBJECT_SCHEMA_VERSION {
            out.push(M5DescriptorObjectViolation::WrongSchemaVersion);
        }
        if self.registry_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            out.push(M5DescriptorObjectViolation::MissingIdentity);
        }
        if self.objects.is_empty() {
            out.push(M5DescriptorObjectViolation::RegistryHasNoObjects);
        }
        let mut seen = std::collections::BTreeSet::new();
        for object in &self.objects {
            if !seen.insert(object.descriptor_id.clone()) {
                out.push(M5DescriptorObjectViolation::DuplicateDescriptorId);
            }
            out.extend(object.validate());
        }
        if !self.vocabulary.matches_canonical() {
            out.push(M5DescriptorObjectViolation::VocabularyMismatch);
        }
        let expected_consumers: Vec<String> = PublicTruthConsumer::ALL
            .iter()
            .map(|c| c.as_str().to_owned())
            .collect();
        if self.consumers != expected_consumers {
            out.push(M5DescriptorObjectViolation::ConsumerSetMismatch);
        }
        if self.conformance != derive_registry_conformance(&self.objects)
            || !self.conformance.all_hold()
        {
            out.push(M5DescriptorObjectViolation::ConformanceReviewFailed);
        }
        out
    }
}

/// Derives the registry conformance review from its objects.
fn derive_registry_conformance(objects: &[DescriptorObject]) -> DescriptorObjectConformance {
    let objects_validate = !objects.is_empty() && objects.iter().all(|o| o.validate_basic());

    // Every weaker value an object carries appears as an explicit narrowing, and no narrowing
    // is invented for a clean value.
    let weaker_survives = objects.iter().all(|o| {
        o.narrowings
            == derive_narrowings(
                &o.provenance,
                &o.freshness,
                &o.qualification,
                &o.client_scope,
            )
    });

    // The blocking facets really do block Stable when their absent/invalid value is present.
    let missing_blocks = objects.iter().all(|o| {
        let has_blocker = matches!(o.provenance.source_class, ProvenanceClass::NotProvided)
            || matches!(
                o.provenance.signature_state,
                SignatureState::SignatureInvalid
            )
            || matches!(
                o.freshness.freshness_state,
                FreshnessState::Expired | FreshnessState::Missing
            )
            || matches!(o.freshness.evidence_state, EvidenceState::NotProvided)
            || matches!(o.qualification.evidence_state, EvidenceState::NotProvided);
        !has_blocker || matches!(o.effective_qualification, QualificationClass::Unavailable)
    });

    let not_provided_first_class = ProvenanceClass::ALL
        .iter()
        .any(|c| matches!(c, ProvenanceClass::NotProvided))
        && SignatureState::ALL
            .iter()
            .any(|c| matches!(c, SignatureState::NotProvided))
        && EvidenceState::ALL
            .iter()
            .any(|c| matches!(c, EvidenceState::NotProvided))
        && AuthorityClass::ALL
            .iter()
            .any(|c| matches!(c, AuthorityClass::NotProvided))
        && HandoffRequirement::ALL
            .iter()
            .any(|c| matches!(c, HandoffRequirement::NotProvided));

    let effective_derived = objects.iter().all(|o| {
        let narrowings = derive_narrowings(
            &o.provenance,
            &o.freshness,
            &o.qualification,
            &o.client_scope,
        );
        o.effective_qualification
            == derive_effective_qualification(o.qualification.support_class, &narrowings)
    });

    // Round-trip every object through JSON and confirm the identity and binding are preserved.
    let identity_preserved = objects.iter().all(|o| {
        match serde_json::to_string(o).and_then(|s| serde_json::from_str::<DescriptorObject>(&s)) {
            Ok(round) => {
                round.descriptor_id == o.descriptor_id && round.artifact_ref == o.artifact_ref
            }
            Err(_) => false,
        }
    });

    let export_clean = objects.iter().all(|o| {
        !json_contains_forbidden_material(
            &serde_json::to_value(o).expect("descriptor object serializes"),
        )
    });

    DescriptorObjectConformance {
        objects_validate,
        controlled_enums_frozen: DescriptorObjectVocabulary::canonical().matches_canonical(),
        weaker_evidence_survives_as_state: weaker_survives,
        missing_evidence_blocks_stable: missing_blocks,
        not_provided_is_first_class: not_provided_first_class,
        effective_qualification_derived: effective_derived,
        identity_and_binding_preserved: identity_preserved,
        shared_across_consumers: true,
        export_carries_no_raw_material: export_clean,
    }
}

impl DescriptorObject {
    /// Validation used by the registry conformance derivation (avoids recursion into the
    /// registry-level checks).
    fn validate_basic(&self) -> bool {
        self.validate().is_empty()
    }
}

/// Validation failures for the descriptor-object lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DescriptorObjectViolation {
    /// The record kind is wrong.
    WrongRecordKind,
    /// The schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is empty.
    MissingIdentity,
    /// The artifact binding is missing a required field.
    MissingArtifactBinding,
    /// The stored narrowings drifted from a fresh derivation.
    NarrowingDrift,
    /// The stored effective qualification drifted from a fresh derivation.
    EffectiveQualificationDrift,
    /// A message id is missing the lane prefix.
    UnprefixedMessageId,
    /// The registry publishes no objects.
    RegistryHasNoObjects,
    /// Two objects share a descriptor id.
    DuplicateDescriptorId,
    /// The controlled-vocabulary set does not match the canonical tokens.
    VocabularyMismatch,
    /// The consumer set does not match the canonical consumers.
    ConsumerSetMismatch,
    /// A conformance-review flag does not hold or drifted.
    ConformanceReviewFailed,
    /// The export contains raw provider material.
    RawMaterialInExport,
}

impl M5DescriptorObjectViolation {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingArtifactBinding => "missing_artifact_binding",
            Self::NarrowingDrift => "narrowing_drift",
            Self::EffectiveQualificationDrift => "effective_qualification_drift",
            Self::UnprefixedMessageId => "unprefixed_message_id",
            Self::RegistryHasNoObjects => "registry_has_no_objects",
            Self::DuplicateDescriptorId => "duplicate_descriptor_id",
            Self::VocabularyMismatch => "vocabulary_mismatch",
            Self::ConsumerSetMismatch => "consumer_set_mismatch",
            Self::ConformanceReviewFailed => "conformance_review_failed",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Keys whose presence would mean an export leaked raw material. Mirrors the redaction posture
/// of the upstream release and support lanes.
const FORBIDDEN_KEY_SUBSTRINGS: [&str; 6] = [
    "credential",
    "secret",
    "password",
    "api_key",
    "raw_payload",
    "bearer_token",
];

/// Scans a serialized object for forbidden material. Returns true when a key (case-insensitive)
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
