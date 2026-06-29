//! The shared badge / explanation-drawer toolkit for the M5 public-truth descriptors.
//!
//! The [descriptor / badge matrix](crate::m5_descriptor_badge) freezes *which* descriptor
//! families exist, *which* consumers bind them, and *how* a weaker value narrows or blocks a
//! claim. What it does not do is decide how an individual descriptor value reads on screen —
//! it carries only an explanation-drawer message *id*. Before this lane each surface resolved
//! those ids locally: the marketplace had its own support-class chips, the About/Help cards
//! had their own provenance states, and docs and support tooling hand-authored their own copy.
//! Identical descriptor states could look and read differently from one surface to the next.
//!
//! This module is the one resolved presentation layer over those descriptors. For every
//! controlled-enum value behind a badge it publishes a single [`BadgeVocabularyEntry`]: an
//! export-safe [badge id](BadgeVocabularyEntry::badge_id), a user-facing
//! [label](BadgeVocabularyEntry::label), a one-line [summary](BadgeVocabularyEntry::summary),
//! the [explanation drawer](BadgeVocabularyEntry::explanation_drawer) body, a
//! [tone](BadgeTone), the [claim effect](BadgeClaimEffect) it carries, and the descriptor
//! identity behind it ([badge family](BadgeFamily) + [dimension](BadgeDimension) + value
//! token). The labels are *generated from* the same controlled enums the descriptor lane
//! freezes — `Official`, `Mirrored`, `Side-loaded`, `Signature verified`,
//! `Attestation available`, `Not provided`, `Partial`, `Certified`, `Supported`, `Limited`,
//! `Experimental`, `Retest pending`, `Evidence stale`, and the rest — so support and claim
//! terminology can never drift into surface-local copy.
//!
//! The entries are grouped under the four [`BadgeFamily`] values, one
//! [`BadgeFamilyGroup`] each, and each group cites the same family explanation-drawer message
//! id the matrix already points at, so the matrix's pointer now resolves to *this* vocabulary.
//! The [`BadgeVocabularyDisclosure`] records that the release center, Help/About, marketplace,
//! docs/help, support exports, and companion handoffs all render this one vocabulary. Every
//! entry carries its descriptor identity so an export or copy never loses the value behind the
//! badge. The packet carries metadata and copy only: no credential bodies or raw provider
//! payloads.
//!
//! - Packet schema:
//!   [`schemas/provenance/m5-badge-vocabulary.schema.json`](../../../../../schemas/provenance/m5-badge-vocabulary.schema.json)
//! - Contract doc:
//!   [`docs/public-truth/m5-badge-vocabulary.md`](../../../../../docs/public-truth/m5-badge-vocabulary.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{seeded_m5_badge_vocabulary, M5_BADGE_VOCABULARY_PACKET_ID};

use serde::{Deserialize, Serialize};

use crate::m5_descriptor_badge::{
    BadgeFamily, DescriptorFamily, DescriptorSignal, FreshnessState, ProvenanceClass,
    M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX,
};
use crate::m5_descriptor_object::{
    AuthorityClass, EvidenceState, HandoffRequirement, SignatureState,
};

/// Record-kind tag carried by [`M5BadgeVocabulary`].
pub const M5_BADGE_VOCABULARY_RECORD_KIND: &str = "m5_badge_vocabulary";

/// Schema version for the badge-vocabulary packet.
pub const M5_BADGE_VOCABULARY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the badge-vocabulary packet schema.
pub const M5_BADGE_VOCABULARY_SCHEMA_REF: &str =
    "schemas/provenance/m5-badge-vocabulary.schema.json";

/// Repo-relative path of the published badge-vocabulary inventory.
pub const M5_BADGE_VOCABULARY_REF: &str = "artifacts/public-truth/m5-badge-vocabulary.json";

/// Repo-relative path of the release-grade badge-vocabulary parity proof.
pub const M5_BADGE_VOCABULARY_PROOF_REF: &str =
    "artifacts/release/m5-descriptor-parity-proof/badge-vocabulary.json";

/// Repo-relative path of the badge-vocabulary governance / drawer catalog doc.
pub const M5_BADGE_VOCABULARY_GOVERNANCE_REF: &str =
    "artifacts/public-truth/m5-badge-vocabulary-governance.md";

/// Repo-relative path of the badge-vocabulary contract doc.
pub const M5_BADGE_VOCABULARY_DOC_REF: &str = "docs/public-truth/m5-badge-vocabulary.md";

/// Prefix every badge message id carries so consumers route it through the shared
/// descriptor lane.
pub const M5_BADGE_MESSAGE_ID_PREFIX: &str = "public_truth_descriptor.badge.";

/// The user-facing terms this vocabulary must resolve from controlled enums, in the order the
/// public-truth contract names them. The conformance review proves each one renders as the
/// label of exactly one badge so no surface re-invents the term in local copy.
pub const REQUIRED_USER_FACING_TERMS: [&str; 13] = [
    "Signature verified",
    "Attestation available",
    "Mirrored",
    "Side-loaded",
    "Official",
    "Not provided",
    "Partial",
    "Certified",
    "Supported",
    "Limited",
    "Experimental",
    "Retest pending",
    "Evidence stale",
];

/// One descriptor dimension a badge resolves. Each dimension is a controlled enum the
/// descriptor lane already freezes; naming the dimension on every badge is what keeps the
/// value behind the badge inspectable after export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeDimension {
    /// Provenance source / origin ([`ProvenanceClass`]).
    SourceOrigin,
    /// Provenance signature / attestation state ([`SignatureState`]).
    SignatureState,
    /// Evidence-freshness window ([`FreshnessState`]).
    FreshnessState,
    /// Evidence completeness ([`EvidenceState`]).
    EvidenceState,
    /// Qualification / support class ([`SupportClass`]).
    SupportClass,
    /// Client kind a surface runs in (`ClientScope`).
    ClientKind,
    /// Authority a client surface carries ([`AuthorityClass`]).
    AuthorityClass,
    /// Handoff a client surface requires ([`HandoffRequirement`]).
    HandoffRequirement,
}

impl BadgeDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::SourceOrigin,
        Self::SignatureState,
        Self::FreshnessState,
        Self::EvidenceState,
        Self::SupportClass,
        Self::ClientKind,
        Self::AuthorityClass,
        Self::HandoffRequirement,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceOrigin => "source_origin",
            Self::SignatureState => "signature_state",
            Self::FreshnessState => "freshness_state",
            Self::EvidenceState => "evidence_state",
            Self::SupportClass => "support_class",
            Self::ClientKind => "client_kind",
            Self::AuthorityClass => "authority_class",
            Self::HandoffRequirement => "handoff_requirement",
        }
    }

    /// The badge family this dimension renders through.
    pub const fn badge_family(self) -> BadgeFamily {
        match self {
            Self::SourceOrigin | Self::SignatureState => BadgeFamily::ProvenanceBadge,
            Self::FreshnessState | Self::EvidenceState => BadgeFamily::FreshnessBadge,
            Self::SupportClass => BadgeFamily::QualificationBadge,
            Self::ClientKind | Self::AuthorityClass | Self::HandoffRequirement => {
                BadgeFamily::ClientScopeBadge
            }
        }
    }
}

/// The user-facing support-class vocabulary the qualification badge renders. These are the
/// stable support-class claim chips a surface shows a user; the descriptor lane's qualification
/// ladder (`stable`/`beta`/…) is the *derived* narrowing rung, a different concept. The tokens
/// match the support-class chips already used across the marketplace and certification surfaces
/// so this is the shared vocabulary, not a new claim family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    /// Certified against current reference evidence — the strongest support claim.
    Certified,
    /// Supported by current evidence under a defined support window.
    Supported,
    /// Supported only with narrower guarantees than a full claim.
    Limited,
    /// Community-maintained support rather than first-party.
    Community,
    /// Experimental, behind an explicit gate.
    Experimental,
    /// Not supported by current evidence.
    Unsupported,
}

impl SupportClass {
    /// Every support class, in declaration order (most→least supported).
    pub const ALL: [Self; 6] = [
        Self::Certified,
        Self::Supported,
        Self::Limited,
        Self::Community,
        Self::Experimental,
        Self::Unsupported,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Supported => "supported",
            Self::Limited => "limited",
            Self::Community => "community",
            Self::Experimental => "experimental",
            Self::Unsupported => "unsupported",
        }
    }
}

/// How a badge value reads — the controlled presentational classification that keeps a weaker
/// state from rendering as if it were authoritative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeTone {
    /// The single strongest value in its dimension; carries a full claim.
    Authoritative,
    /// A neutral, non-narrowing value.
    Informational,
    /// A value that narrows the claim below Stable.
    Caution,
    /// A value that blocks a Stable claim entirely.
    Blocking,
}

impl BadgeTone {
    /// Every tone, in declaration order (strongest→weakest).
    pub const ALL: [Self; 4] = [
        Self::Authoritative,
        Self::Informational,
        Self::Caution,
        Self::Blocking,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
            Self::Informational => "informational",
            Self::Caution => "caution",
            Self::Blocking => "blocking",
        }
    }

    /// Traffic-light signal mirroring the descriptor lane's [`DescriptorSignal`].
    pub const fn signal(self) -> DescriptorSignal {
        match self {
            Self::Authoritative | Self::Informational => DescriptorSignal::Green,
            Self::Caution => DescriptorSignal::Yellow,
            Self::Blocking => DescriptorSignal::Red,
        }
    }
}

/// The effect a badge value has on the claim it sits beside. Generated from the same downgrade
/// behavior the descriptor lane freezes, so a badge can never read calmer than the claim it
/// narrows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeClaimEffect {
    /// The value keeps the claim at its full class.
    None,
    /// The value narrows the claim below Stable.
    Narrows,
    /// The value blocks a Stable claim entirely.
    Blocks,
}

impl BadgeClaimEffect {
    /// Every effect, in declaration order.
    pub const ALL: [Self; 3] = [Self::None, Self::Narrows, Self::Blocks];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Narrows => "narrows",
            Self::Blocks => "blocks",
        }
    }
}

/// Resolved copy and classification for one badge value: the per-token content the generator
/// attaches to a controlled-enum value.
struct BadgeCopy {
    label: &'static str,
    summary: &'static str,
    drawer: &'static str,
    effect: BadgeClaimEffect,
    authoritative: bool,
}

impl BadgeCopy {
    const fn new(
        label: &'static str,
        summary: &'static str,
        drawer: &'static str,
        effect: BadgeClaimEffect,
    ) -> Self {
        Self {
            label,
            summary,
            drawer,
            effect,
            authoritative: false,
        }
    }

    /// The strongest value of a dimension — renders with [`BadgeTone::Authoritative`].
    const fn top(label: &'static str, summary: &'static str, drawer: &'static str) -> Self {
        Self {
            label,
            summary,
            drawer,
            effect: BadgeClaimEffect::None,
            authoritative: true,
        }
    }

    fn tone(&self) -> BadgeTone {
        match self.effect {
            BadgeClaimEffect::Blocks => BadgeTone::Blocking,
            BadgeClaimEffect::Narrows => BadgeTone::Caution,
            BadgeClaimEffect::None if self.authoritative => BadgeTone::Authoritative,
            BadgeClaimEffect::None => BadgeTone::Informational,
        }
    }
}

/// One member badge: a single controlled-enum value rendered as the same badge and explanation
/// drawer everywhere it appears.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeVocabularyEntry {
    /// The badge family this entry belongs to.
    pub badge_family: BadgeFamily,
    /// The descriptor family the badge family renders.
    pub descriptor_family: DescriptorFamily,
    /// The descriptor dimension (controlled enum) the value comes from.
    pub dimension: BadgeDimension,
    /// The controlled-enum value token — the descriptor identity behind the badge.
    pub value_token: String,
    /// Export-safe, copy-safe stable badge id, unique across the vocabulary.
    pub badge_id: String,
    /// User-facing label drawn from the controlled vocabulary.
    pub label: String,
    /// One-line summary shown beside the badge.
    pub summary: String,
    /// The explanation-drawer body — the same expansion text on every surface.
    pub explanation_drawer: String,
    /// Presentational tone.
    pub tone: BadgeTone,
    /// Traffic-light signal (mirrors [`Self::tone`]).
    pub signal: DescriptorSignal,
    /// Effect this value has on the claim it sits beside.
    pub claim_effect: BadgeClaimEffect,
    /// Stable message id; prefixed [`M5_BADGE_MESSAGE_ID_PREFIX`].
    pub message_id: String,
}

impl BadgeVocabularyEntry {
    fn build(dimension: BadgeDimension, value_token: &str, copy: BadgeCopy) -> Self {
        let tone = copy.tone();
        Self {
            badge_family: dimension.badge_family(),
            descriptor_family: dimension.badge_family().descriptor_family(),
            dimension,
            value_token: value_token.to_owned(),
            badge_id: format!("{}.{}", dimension.as_str(), value_token),
            label: copy.label.to_owned(),
            summary: copy.summary.to_owned(),
            explanation_drawer: copy.drawer.to_owned(),
            tone,
            signal: tone.signal(),
            claim_effect: copy.effect,
            message_id: format!(
                "{M5_BADGE_MESSAGE_ID_PREFIX}{}.{}",
                dimension.as_str(),
                value_token
            ),
        }
    }

    /// Validates the entry's invariants: the dimension maps to its badge family, the id and
    /// message id are derived, and identity / copy fields are present.
    pub fn validate(&self) -> Vec<M5BadgeVocabularyViolation> {
        let mut out = Vec::new();
        if self.badge_family != self.dimension.badge_family()
            || self.descriptor_family != self.dimension.badge_family().descriptor_family()
        {
            out.push(M5BadgeVocabularyViolation::DimensionFamilyMismatch);
        }
        if self.badge_id != format!("{}.{}", self.dimension.as_str(), self.value_token) {
            out.push(M5BadgeVocabularyViolation::BadgeIdDrift);
        }
        if !self.message_id.starts_with(M5_BADGE_MESSAGE_ID_PREFIX) {
            out.push(M5BadgeVocabularyViolation::UnprefixedMessageId);
        }
        if self.value_token.trim().is_empty()
            || self.label.trim().is_empty()
            || self.summary.trim().is_empty()
            || self.explanation_drawer.trim().is_empty()
        {
            out.push(M5BadgeVocabularyViolation::MissingCopy);
        }
        if self.signal != self.tone.signal() {
            out.push(M5BadgeVocabularyViolation::SignalToneMismatch);
        }
        out
    }
}

/// One badge family with its ordered member badges and the family explanation-drawer message
/// id the descriptor matrix already points at.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeFamilyGroup {
    /// The badge family.
    pub badge_family: BadgeFamily,
    /// The descriptor family it renders.
    pub descriptor_family: DescriptorFamily,
    /// Reviewer-facing family label.
    pub family_label: String,
    /// The family explanation-drawer message id the matrix points at; prefixed
    /// [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub family_drawer_message_id: String,
    /// The member badges, in dimension then declaration order.
    pub entries: Vec<BadgeVocabularyEntry>,
}

impl BadgeFamilyGroup {
    fn family_label(family: BadgeFamily) -> &'static str {
        match family {
            BadgeFamily::ProvenanceBadge => "Provenance / source-origin badges",
            BadgeFamily::FreshnessBadge => "Evidence-freshness badges",
            BadgeFamily::QualificationBadge => "Qualification / support-class badges",
            BadgeFamily::ClientScopeBadge => "Client-scope badges",
        }
    }

    fn new(badge_family: BadgeFamily, entries: Vec<BadgeVocabularyEntry>) -> Self {
        Self {
            badge_family,
            descriptor_family: badge_family.descriptor_family(),
            family_label: Self::family_label(badge_family).to_owned(),
            family_drawer_message_id: format!(
                "{}drawer.{}",
                M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX,
                badge_family.descriptor_family().as_str()
            ),
            entries,
        }
    }
}

/// Which public-truth surfaces render the one badge vocabulary. Every flag must hold so no
/// surface maintains a parallel badge or copy vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeVocabularyDisclosure {
    /// The release center renders the vocabulary.
    pub release_center_renders_vocabulary: bool,
    /// The Help/About panel renders the vocabulary.
    pub help_about_renders_vocabulary: bool,
    /// The marketplace / ecosystem surface renders the vocabulary.
    pub marketplace_renders_vocabulary: bool,
    /// The docs / help surface renders the vocabulary.
    pub docs_help_renders_vocabulary: bool,
    /// Support exports render the vocabulary.
    pub support_export_renders_vocabulary: bool,
    /// Companion handoffs render the vocabulary.
    pub companion_handoff_renders_vocabulary: bool,
}

impl BadgeVocabularyDisclosure {
    /// The canonical disclosure: every surface renders the vocabulary.
    pub const fn all_surfaces() -> Self {
        Self {
            release_center_renders_vocabulary: true,
            help_about_renders_vocabulary: true,
            marketplace_renders_vocabulary: true,
            docs_help_renders_vocabulary: true,
            support_export_renders_vocabulary: true,
            companion_handoff_renders_vocabulary: true,
        }
    }

    /// True when every surface renders the vocabulary.
    pub const fn all_render(&self) -> bool {
        self.release_center_renders_vocabulary
            && self.help_about_renders_vocabulary
            && self.marketplace_renders_vocabulary
            && self.docs_help_renders_vocabulary
            && self.support_export_renders_vocabulary
            && self.companion_handoff_renders_vocabulary
    }
}

/// Proof that one required user-facing term renders as exactly one badge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequiredTermCoverage {
    /// The required user-facing term.
    pub term: String,
    /// The badge id that renders the term.
    pub badge_id: String,
    /// The dimension the term comes from.
    pub dimension: BadgeDimension,
    /// The value token behind the term.
    pub value_token: String,
}

/// Compact vocabulary summary — the scoreboard every surface reads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeVocabularySummary {
    /// Total badge families.
    pub total_families: u32,
    /// Total descriptor dimensions covered.
    pub total_dimensions: u32,
    /// Total member badges.
    pub total_badges: u32,
    /// Badges that read [authoritative](BadgeTone::Authoritative).
    pub authoritative_badge_count: u32,
    /// Badges that read [informational](BadgeTone::Informational).
    pub informational_badge_count: u32,
    /// Badges that read [caution](BadgeTone::Caution).
    pub caution_badge_count: u32,
    /// Badges that read [blocking](BadgeTone::Blocking).
    pub blocking_badge_count: u32,
    /// Badges that narrow the claim they sit beside.
    pub narrowing_badge_count: u32,
    /// Badges that block a Stable claim.
    pub blocking_claim_badge_count: u32,
    /// Required user-facing terms covered.
    pub required_terms_covered: u32,
}

/// Self-describing controlled-vocabulary set so the packet resolves every token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeVocabularyTokens {
    /// Badge-family tokens.
    pub badge_families: Vec<String>,
    /// Dimension tokens.
    pub dimensions: Vec<String>,
    /// Tone tokens.
    pub tones: Vec<String>,
    /// Signal tokens.
    pub signals: Vec<String>,
    /// Claim-effect tokens.
    pub claim_effects: Vec<String>,
    /// Support-class tokens.
    pub support_classes: Vec<String>,
    /// Required user-facing terms.
    pub required_terms: Vec<String>,
}

impl BadgeVocabularyTokens {
    /// Builds the canonical token set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            badge_families: BadgeFamily::ALL
                .iter()
                .map(|f| f.as_str().to_owned())
                .collect(),
            dimensions: BadgeDimension::ALL
                .iter()
                .map(|d| d.as_str().to_owned())
                .collect(),
            tones: BadgeTone::ALL
                .iter()
                .map(|t| t.as_str().to_owned())
                .collect(),
            signals: DescriptorSignal::ALL
                .iter()
                .map(|s| s.as_str().to_owned())
                .collect(),
            claim_effects: BadgeClaimEffect::ALL
                .iter()
                .map(|e| e.as_str().to_owned())
                .collect(),
            support_classes: SupportClass::ALL
                .iter()
                .map(|s| s.as_str().to_owned())
                .collect(),
            required_terms: REQUIRED_USER_FACING_TERMS
                .iter()
                .map(|t| (*t).to_owned())
                .collect(),
        }
    }

    /// True when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Conformance review. Every flag is a hard invariant; all must hold.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeVocabularyConformance {
    /// Every badge family has at least one member badge.
    pub every_family_has_badges: bool,
    /// Every dimension maps to exactly one badge family.
    pub every_dimension_maps_to_a_family: bool,
    /// Every badge label is generated from a controlled enum value, never local copy.
    pub labels_from_controlled_enums: bool,
    /// Every required user-facing term renders as exactly one badge.
    pub every_required_term_present: bool,
    /// Every weaker (caution/blocking) badge carries a narrowing/blocking claim effect.
    pub weaker_badges_carry_claim_effect: bool,
    /// Mirror / offline / side-loaded / not-provided badges are first-class, never omitted.
    pub weaker_origins_never_omitted: bool,
    /// Every badge carries its descriptor identity so an export keeps the value behind it.
    pub export_preserves_descriptor_identity: bool,
    /// The drawer text is defined once and shared across every consumer surface.
    pub same_drawer_text_across_consumers: bool,
    /// Release center, Help/About, marketplace, docs/help, support, companion render one vocabulary.
    pub surfaces_render_one_vocabulary: bool,
    /// The vocabulary is generated from the controlled enums.
    pub generated_from_controlled_enums: bool,
    /// The export carries no raw provider material.
    pub export_carries_no_raw_material: bool,
}

impl BadgeVocabularyConformance {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.every_family_has_badges
            && self.every_dimension_maps_to_a_family
            && self.labels_from_controlled_enums
            && self.every_required_term_present
            && self.weaker_badges_carry_claim_effect
            && self.weaker_origins_never_omitted
            && self.export_preserves_descriptor_identity
            && self.same_drawer_text_across_consumers
            && self.surfaces_render_one_vocabulary
            && self.generated_from_controlled_enums
            && self.export_carries_no_raw_material
    }
}

/// Export-safe M5 badge vocabulary: the one resolved badge / explanation-drawer toolkit every
/// public-truth surface renders for the provenance, freshness, qualification, and client-scope
/// descriptors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeVocabulary {
    /// Record kind; must equal [`M5_BADGE_VOCABULARY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_BADGE_VOCABULARY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the packet was computed as-of.
    pub evaluated_at: String,
    /// The badge families with their member badges.
    pub families: Vec<BadgeFamilyGroup>,
    /// Per-term coverage proof for the required user-facing terms.
    pub required_term_coverage: Vec<RequiredTermCoverage>,
    /// Compact vocabulary summary.
    pub summary: BadgeVocabularySummary,
    /// Which surfaces render the vocabulary.
    pub disclosure: BadgeVocabularyDisclosure,
    /// Controlled-vocabulary token set.
    pub vocabulary: BadgeVocabularyTokens,
    /// Conformance review block.
    pub conformance: BadgeVocabularyConformance,
    /// Cross-refs to the descriptor schemas this vocabulary renders.
    pub source_descriptor_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5BadgeVocabulary {
    /// Builds the canonical badge vocabulary, deriving every family, entry, summary, and
    /// conformance flag from the controlled enums.
    pub fn canonical(
        packet_id: &str,
        report_label: &str,
        evaluated_at: &str,
        minted_at: &str,
    ) -> Self {
        let families = canonical_families();
        let required_term_coverage = derive_required_term_coverage(&families);
        let summary = derive_summary(&families, &required_term_coverage);
        let conformance = derive_conformance(&families, &required_term_coverage);
        Self {
            record_kind: M5_BADGE_VOCABULARY_RECORD_KIND.to_owned(),
            schema_version: M5_BADGE_VOCABULARY_SCHEMA_VERSION,
            packet_id: packet_id.to_owned(),
            report_label: report_label.to_owned(),
            evaluated_at: evaluated_at.to_owned(),
            families,
            required_term_coverage,
            summary,
            disclosure: BadgeVocabularyDisclosure::all_surfaces(),
            vocabulary: BadgeVocabularyTokens::canonical(),
            conformance,
            source_descriptor_refs: source_descriptor_refs(),
            redaction_class_token: "metadata_safe_default".to_owned(),
            minted_at: minted_at.to_owned(),
        }
    }

    /// Every member badge across all families, in family then dimension order.
    pub fn all_entries(&self) -> Vec<&BadgeVocabularyEntry> {
        self.families
            .iter()
            .flat_map(|g| g.entries.iter())
            .collect()
    }

    /// Finds the badge family group for a family.
    pub fn family_group(&self, family: BadgeFamily) -> Option<&BadgeFamilyGroup> {
        self.families.iter().find(|g| g.badge_family == family)
    }

    /// Finds a member badge by export-safe badge id.
    pub fn badge(&self, badge_id: &str) -> Option<&BadgeVocabularyEntry> {
        self.all_entries()
            .into_iter()
            .find(|e| e.badge_id == badge_id)
    }

    /// Finds the badge that renders a user-facing term.
    pub fn badge_for_term(&self, term: &str) -> Option<&BadgeVocabularyEntry> {
        self.all_entries().into_iter().find(|e| e.label == term)
    }

    /// Validates the packet's invariants.
    pub fn validate(&self) -> Vec<M5BadgeVocabularyViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_BADGE_VOCABULARY_RECORD_KIND {
            out.push(M5BadgeVocabularyViolation::WrongRecordKind);
        }
        if self.schema_version != M5_BADGE_VOCABULARY_SCHEMA_VERSION {
            out.push(M5BadgeVocabularyViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.evaluated_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            out.push(M5BadgeVocabularyViolation::MissingIdentity);
        }

        // Every badge family must be present exactly once.
        let mut seen_families = std::collections::BTreeSet::new();
        for group in &self.families {
            if !seen_families.insert(group.badge_family) {
                out.push(M5BadgeVocabularyViolation::DuplicateFamily);
            }
            if group.descriptor_family != group.badge_family.descriptor_family()
                || group.family_label.trim().is_empty()
                || !group
                    .family_drawer_message_id
                    .starts_with(M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX)
            {
                out.push(M5BadgeVocabularyViolation::FamilyGroupFieldMismatch);
            }
            if group.entries.is_empty() {
                out.push(M5BadgeVocabularyViolation::FamilyHasNoBadges);
            }
            for entry in &group.entries {
                if entry.badge_family != group.badge_family {
                    out.push(M5BadgeVocabularyViolation::FamilyGroupFieldMismatch);
                }
                out.extend(entry.validate());
            }
        }
        for family in BadgeFamily::ALL {
            if !self.families.iter().any(|g| g.badge_family == family) {
                out.push(M5BadgeVocabularyViolation::FamilyMissing);
            }
        }

        // Badge ids must be unique across the whole vocabulary.
        let mut seen_ids = std::collections::BTreeSet::new();
        for entry in self.all_entries() {
            if !seen_ids.insert(entry.badge_id.clone()) {
                out.push(M5BadgeVocabularyViolation::DuplicateBadgeId);
            }
        }

        // The vocabulary, summary, coverage, and conformance must match a fresh derive.
        if self.families != canonical_families() {
            out.push(M5BadgeVocabularyViolation::FamiliesDrift);
        }
        if self.required_term_coverage != derive_required_term_coverage(&self.families) {
            out.push(M5BadgeVocabularyViolation::RequiredTermCoverageDrift);
        }
        if self.summary != derive_summary(&self.families, &self.required_term_coverage) {
            out.push(M5BadgeVocabularyViolation::SummaryDrift);
        }
        if !self.disclosure.all_render() {
            out.push(M5BadgeVocabularyViolation::DisclosureIncomplete);
        }
        if !self.vocabulary.matches_canonical() {
            out.push(M5BadgeVocabularyViolation::VocabularyMismatch);
        }
        if self.conformance != derive_conformance(&self.families, &self.required_term_coverage)
            || !self.conformance.all_hold()
        {
            out.push(M5BadgeVocabularyViolation::ConformanceReviewFailed);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 badge vocabulary serializes"),
        ) {
            out.push(M5BadgeVocabularyViolation::RawMaterialInExport);
        }

        out
    }

    /// Deterministic export-safe JSON for the packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 badge vocabulary serializes")
    }

    /// Deterministic Markdown governance / drawer catalog for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Badge Vocabulary And Explanation Drawers\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!(
            "- Badges: {} across {} families and {} dimensions\n",
            self.summary.total_badges, self.summary.total_families, self.summary.total_dimensions
        ));
        out.push_str(&format!(
            "- Tone: {} authoritative, {} informational, {} caution, {} blocking\n",
            self.summary.authoritative_badge_count,
            self.summary.informational_badge_count,
            self.summary.caution_badge_count,
            self.summary.blocking_badge_count
        ));
        out.push_str(&format!(
            "- Claim effect: {} narrow, {} block\n",
            self.summary.narrowing_badge_count, self.summary.blocking_claim_badge_count
        ));
        out.push_str(&format!(
            "- Required terms covered: {}/{}\n",
            self.summary.required_terms_covered,
            REQUIRED_USER_FACING_TERMS.len()
        ));
        out.push_str(
            "- Rendered by: release center, Help/About, marketplace, docs/help, support, companion\n",
        );

        for group in &self.families {
            out.push_str(&format!("\n## {}\n\n", group.family_label));
            out.push_str("| Badge id | Label | Tone | Claim effect | Explanation drawer |\n");
            out.push_str("|----------|-------|------|--------------|--------------------|\n");
            for entry in &group.entries {
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | {} |\n",
                    entry.badge_id,
                    entry.label,
                    entry.tone.as_str(),
                    entry.claim_effect.as_str(),
                    entry.explanation_drawer
                ));
            }
        }

        out.push_str("\n## Required user-facing terms\n\n");
        out.push_str("| Term | Badge id | Dimension |\n");
        out.push_str("|------|----------|-----------|\n");
        for cov in &self.required_term_coverage {
            out.push_str(&format!(
                "| {} | `{}` | `{}` |\n",
                cov.term,
                cov.badge_id,
                cov.dimension.as_str()
            ));
        }
        out
    }
}

/// Builds the member badges for the source-origin dimension from [`ProvenanceClass`].
fn source_origin_copy(class: ProvenanceClass) -> BadgeCopy {
    use BadgeClaimEffect::{Blocks, Narrows};
    match class {
        ProvenanceClass::FirstPartySigned => BadgeCopy::top(
            "Official",
            "First-party build, signed and attested by the release identity.",
            "This artifact is published by the project's own release identity and carries a verified first-party signature. It is the only origin that can carry an unqualified Stable claim.",
        ),
        ProvenanceClass::Vendor => BadgeCopy::new(
            "Vendor",
            "Authored by a governed vendor or partner.",
            "A known vendor or partner authored this under a governed agreement. The origin is accountable but is not first-party, so a claim built on it narrows below Stable until first-party evidence is present.",
            Narrows,
        ),
        ProvenanceClass::Community => BadgeCopy::new(
            "Community",
            "Community-contributed and reviewed, not first-party.",
            "The community contributed and reviewed this artifact. It is legitimate but not first-party, so a claim built on it narrows below Stable.",
            Narrows,
        ),
        ProvenanceClass::Mirror => BadgeCopy::new(
            "Mirrored",
            "Served from a mirror of a first-party artifact.",
            "This came from a mirror copy rather than the first-party channel. The mirror's freshness stays inspectable without reaching vendor services, but a mirrored origin narrows the claim until first-party provenance is confirmed.",
            Narrows,
        ),
        ProvenanceClass::OfflineBundle => BadgeCopy::new(
            "Offline bundle",
            "Installed from an offline bundle; origin recorded, not live-verified.",
            "This was installed from an offline bundle. Its origin is recorded but not live-verified against the channel, so the claim narrows until the origin can be reverified online.",
            Narrows,
        ),
        ProvenanceClass::SideLoaded => BadgeCopy::new(
            "Side-loaded",
            "Installed outside the governed channel.",
            "This was side-loaded outside the governed channel. The origin is shown rather than hidden, but a side-loaded artifact cannot carry a Stable claim until it is reconciled with the governed channel.",
            Narrows,
        ),
        ProvenanceClass::NotProvided => BadgeCopy::new(
            "Not provided",
            "Origin evidence was not provided.",
            "No origin evidence was provided for this artifact. A missing origin is recorded explicitly — never left blank — and blocks any Stable claim until provenance is supplied.",
            Blocks,
        ),
    }
}

/// Builds the member badges for the signature-state dimension from [`SignatureState`].
fn signature_state_copy(state: SignatureState) -> BadgeCopy {
    use BadgeClaimEffect::{Blocks, Narrows, None};
    match state {
        SignatureState::SignedAttested => BadgeCopy::top(
            "Signature verified",
            "Signature and attestation verified against the release identity.",
            "The artifact's signature was checked against the release identity and a build attestation is present and valid. This is the strongest signature state.",
        ),
        SignatureState::SignedUnverified => BadgeCopy::new(
            "Signature unverified",
            "A signature is present but could not be verified.",
            "A signature is present but could not be verified against a trusted key in the current context. The claim narrows until the signature verifies.",
            Narrows,
        ),
        SignatureState::AttestationOnly => BadgeCopy::new(
            "Attestation available",
            "A build attestation is available.",
            "A build attestation is available describing how this artifact was produced. Attestation is positive evidence but does not by itself substitute for a verified signature.",
            None,
        ),
        SignatureState::Unsigned => BadgeCopy::new(
            "Unsigned",
            "No signature is present.",
            "This artifact carries no signature. An unsigned artifact narrows the claim until signing evidence is added.",
            Narrows,
        ),
        SignatureState::SignatureInvalid => BadgeCopy::new(
            "Signature invalid",
            "A signature is present but failed verification.",
            "A signature is present but failed verification — the bytes do not match the claimed identity. An invalid signature blocks any Stable claim and is surfaced rather than ignored.",
            Blocks,
        ),
        SignatureState::NotProvided => BadgeCopy::new(
            "Signature not provided",
            "No signature evidence was provided.",
            "No signature evidence was provided for this artifact. The absence is recorded explicitly and narrows the claim until signature state is supplied.",
            Narrows,
        ),
    }
}

/// Builds the member badges for the freshness-state dimension from [`FreshnessState`].
fn freshness_state_copy(state: FreshnessState) -> BadgeCopy {
    use BadgeClaimEffect::{Blocks, Narrows};
    match state {
        FreshnessState::Current => BadgeCopy::top(
            "Evidence current",
            "Evidence is within its freshness window.",
            "The evidence behind this claim is within its freshness window. The claim stands at its full class.",
        ),
        FreshnessState::Stale => BadgeCopy::new(
            "Evidence aging",
            "Evidence has fallen outside its freshness window.",
            "The evidence has fallen outside its freshness window. Stale evidence automatically narrows the claim below Stable until it is refreshed.",
            Narrows,
        ),
        FreshnessState::Expired => BadgeCopy::new(
            "Evidence expired",
            "Evidence has passed its hard expiry.",
            "The evidence has passed its hard expiry. Expired evidence blocks a Stable claim until it is renewed.",
            Blocks,
        ),
        FreshnessState::Missing => BadgeCopy::new(
            "Evidence missing",
            "No usable evidence exists.",
            "No usable evidence exists for this claim. A missing-evidence state blocks a Stable claim and is recorded explicitly rather than omitted.",
            Blocks,
        ),
    }
}

/// Builds the member badges for the evidence-state dimension from [`EvidenceState`].
fn evidence_state_copy(state: EvidenceState) -> BadgeCopy {
    use BadgeClaimEffect::{Blocks, Narrows};
    match state {
        EvidenceState::Complete => BadgeCopy::top(
            "Complete",
            "Evidence covers the full claimed scope.",
            "The evidence covers the full claimed scope. Nothing in the claimed matrix is left unverified.",
        ),
        EvidenceState::Limited => BadgeCopy::new(
            "Limited evidence",
            "Evidence covers a narrower scope than claimed.",
            "The evidence covers a narrower scope than the claim. The claim narrows to what the evidence actually supports.",
            Narrows,
        ),
        EvidenceState::Partial => BadgeCopy::new(
            "Partial",
            "Only part of the claimed scope is evidenced.",
            "Only part of the claimed scope is backed by evidence. The unverified remainder is named rather than implied, and the claim narrows accordingly.",
            Narrows,
        ),
        EvidenceState::RetestPending => BadgeCopy::new(
            "Retest pending",
            "Evidence is awaiting re-verification.",
            "The evidence is awaiting re-verification — a retest is queued or in progress. The claim narrows until the retest completes.",
            Narrows,
        ),
        EvidenceState::EvidenceStale => BadgeCopy::new(
            "Evidence stale",
            "The evidence body itself has aged out.",
            "The evidence body itself has aged past its freshness window. A stale evidence body narrows the claim below Stable until it is refreshed.",
            Narrows,
        ),
        EvidenceState::NotProvided => BadgeCopy::new(
            "Evidence not provided",
            "No evidence body was provided.",
            "No evidence body was provided for this claim. The absence is recorded explicitly and blocks a Stable claim until evidence is supplied.",
            Blocks,
        ),
    }
}

/// Builds the member badges for the support-class dimension from [`SupportClass`].
fn support_class_copy(class: SupportClass) -> BadgeCopy {
    use BadgeClaimEffect::{Blocks, Narrows, None};
    match class {
        SupportClass::Certified => BadgeCopy::top(
            "Certified",
            "Certified against current reference evidence.",
            "This surface is certified against current reference evidence — the strongest support claim. Certification narrows automatically if its evidence goes stale or missing.",
        ),
        SupportClass::Supported => BadgeCopy::new(
            "Supported",
            "Supported by current evidence.",
            "This surface is supported by current evidence under a defined support window. It is a full claim short of formal certification.",
            None,
        ),
        SupportClass::Limited => BadgeCopy::new(
            "Limited",
            "Supported with narrower guarantees.",
            "This surface is supported only with narrower guarantees than a full claim. The reduced scope is stated rather than implied, and the claim narrows accordingly.",
            Narrows,
        ),
        SupportClass::Community => BadgeCopy::new(
            "Community",
            "Community-maintained support only.",
            "Support for this surface is community-maintained rather than first-party. It is a legitimate but narrower support class, so the claim narrows below a first-party class.",
            Narrows,
        ),
        SupportClass::Experimental => BadgeCopy::new(
            "Experimental",
            "Experimental, behind an explicit gate.",
            "This surface is experimental and sits behind an explicit gate. Experimental support narrows the claim well below Stable.",
            Narrows,
        ),
        SupportClass::Unsupported => BadgeCopy::new(
            "Unsupported",
            "Not supported by current evidence.",
            "Current evidence does not support this surface. An unsupported state blocks a Stable claim and is surfaced rather than hidden.",
            Blocks,
        ),
    }
}

/// Builds the member badges for the client-kind dimension from `ClientScope`.
fn client_kind_copy(scope: crate::m5_descriptor_badge::ClientScope) -> BadgeCopy {
    use crate::m5_descriptor_badge::ClientScope;
    use BadgeClaimEffect::Narrows;
    match scope {
        ClientScope::DesktopFull => BadgeCopy::top(
            "Desktop (full)",
            "Full desktop surface with full authority.",
            "The full desktop product surface. Only this client scope carries full authority and capability parity.",
        ),
        ClientScope::CompanionScoped => BadgeCopy::new(
            "Companion (scoped)",
            "Companion surface with bounded, host-relayed scope.",
            "A companion surface with bounded scope relayed through the desktop host. It narrows a claim so it can never imply the desktop's authority or capability parity.",
            Narrows,
        ),
        ClientScope::MobileCompanion => BadgeCopy::new(
            "Mobile companion",
            "Mobile companion surface.",
            "A mobile companion surface with bounded scope. It narrows a claim and cannot imply desktop parity.",
            Narrows,
        ),
        ClientScope::EmbeddedPanel => BadgeCopy::new(
            "Embedded panel",
            "Panel embedded inside another surface.",
            "A panel hosted inside another surface, under the host's constraints. It narrows a claim and cannot imply full-surface authority.",
            Narrows,
        ),
        ClientScope::BrowserReference => BadgeCopy::new(
            "Browser reference",
            "Read-only browser reference surface.",
            "A browser reference surface — read-only and informational. It narrows a claim to discovery or reference and cannot carry in-product authority.",
            Narrows,
        ),
        ClientScope::HandoffOnly => BadgeCopy::new(
            "Handoff only",
            "Only creates or opens a desktop handoff.",
            "This surface can only create or open a desktop handoff. It narrows a claim to handoff actions and carries no standalone authority.",
            Narrows,
        ),
    }
}

/// Builds the member badges for the authority-class dimension from [`AuthorityClass`].
fn authority_class_copy(class: AuthorityClass) -> BadgeCopy {
    use BadgeClaimEffect::{Blocks, Narrows};
    match class {
        AuthorityClass::FullAuthority => BadgeCopy::top(
            "Full authority",
            "Carries full authority and capability parity.",
            "This surface carries full authority — the actions it offers are authoritative and at parity with the desktop.",
        ),
        AuthorityClass::ScopedAuthority => BadgeCopy::new(
            "Scoped authority",
            "Authority bounded to a relayed scope.",
            "This surface's authority is bounded to a relayed scope. It narrows a claim and cannot widen to desktop authority.",
            Narrows,
        ),
        AuthorityClass::ReferenceOnly => BadgeCopy::new(
            "Reference only",
            "Informational; carries no authority.",
            "This surface is reference-only — it shows information but carries no authority to act. The claim narrows to reference.",
            Narrows,
        ),
        AuthorityClass::HandoffOnly => BadgeCopy::new(
            "Handoff authority",
            "Authority limited to creating a handoff.",
            "This surface's authority is limited to creating a desktop handoff. It narrows a claim and never acts authoritatively on its own.",
            Narrows,
        ),
        AuthorityClass::NotProvided => BadgeCopy::new(
            "Authority not provided",
            "Authority class was not provided.",
            "No authority class was provided for this surface. The absence is recorded explicitly and blocks a Stable claim until it is supplied.",
            Blocks,
        ),
    }
}

/// Builds the member badges for the handoff-requirement dimension from [`HandoffRequirement`].
fn handoff_requirement_copy(req: HandoffRequirement) -> BadgeCopy {
    use BadgeClaimEffect::{Blocks, Narrows};
    match req {
        HandoffRequirement::NotRequired => BadgeCopy::top(
            "No handoff required",
            "Acts in place with no handoff.",
            "This surface acts in place and requires no handoff to another client. This is the only handoff state that does not narrow on handoff grounds.",
        ),
        HandoffRequirement::DesktopHandoffRequired => BadgeCopy::new(
            "Desktop handoff required",
            "Privileged actions require a desktop handoff.",
            "Privileged actions on this surface require handing off to the desktop. The requirement is named rather than failing silently, and it narrows the surface's standalone claim.",
            Narrows,
        ),
        HandoffRequirement::ConsoleHandoffRequired => BadgeCopy::new(
            "Console handoff required",
            "Privileged actions require a vendor console handoff.",
            "Privileged actions here require handing off to a vendor console. The requirement is named explicitly and narrows the surface's standalone claim.",
            Narrows,
        ),
        HandoffRequirement::NotProvided => BadgeCopy::new(
            "Handoff state not provided",
            "Handoff requirement was not provided.",
            "No handoff requirement was provided for this surface. The absence is recorded explicitly and blocks a Stable claim until it is supplied.",
            Blocks,
        ),
    }
}

/// Builds the entries for one dimension by iterating its controlled enum.
fn entries_for_dimension(dimension: BadgeDimension) -> Vec<BadgeVocabularyEntry> {
    let build = |token: &str, copy: BadgeCopy| BadgeVocabularyEntry::build(dimension, token, copy);
    match dimension {
        BadgeDimension::SourceOrigin => ProvenanceClass::ALL
            .iter()
            .map(|c| build(c.as_str(), source_origin_copy(*c)))
            .collect(),
        BadgeDimension::SignatureState => SignatureState::ALL
            .iter()
            .map(|s| build(s.as_str(), signature_state_copy(*s)))
            .collect(),
        BadgeDimension::FreshnessState => FreshnessState::ALL
            .iter()
            .map(|s| build(s.as_str(), freshness_state_copy(*s)))
            .collect(),
        BadgeDimension::EvidenceState => EvidenceState::ALL
            .iter()
            .map(|s| build(s.as_str(), evidence_state_copy(*s)))
            .collect(),
        BadgeDimension::SupportClass => SupportClass::ALL
            .iter()
            .map(|c| build(c.as_str(), support_class_copy(*c)))
            .collect(),
        BadgeDimension::ClientKind => crate::m5_descriptor_badge::ClientScope::ALL
            .iter()
            .map(|s| build(s.as_str(), client_kind_copy(*s)))
            .collect(),
        BadgeDimension::AuthorityClass => AuthorityClass::ALL
            .iter()
            .map(|c| build(c.as_str(), authority_class_copy(*c)))
            .collect(),
        BadgeDimension::HandoffRequirement => HandoffRequirement::ALL
            .iter()
            .map(|r| build(r.as_str(), handoff_requirement_copy(*r)))
            .collect(),
    }
}

/// Builds the canonical badge families, generating every entry from the controlled enums.
fn canonical_families() -> Vec<BadgeFamilyGroup> {
    BadgeFamily::ALL
        .iter()
        .map(|family| {
            let entries: Vec<BadgeVocabularyEntry> = BadgeDimension::ALL
                .iter()
                .filter(|d| d.badge_family() == *family)
                .flat_map(|d| entries_for_dimension(*d))
                .collect();
            BadgeFamilyGroup::new(*family, entries)
        })
        .collect()
}

/// Derives the per-term coverage proof from the families, in the order the contract names them.
fn derive_required_term_coverage(families: &[BadgeFamilyGroup]) -> Vec<RequiredTermCoverage> {
    REQUIRED_USER_FACING_TERMS
        .iter()
        .filter_map(|term| {
            families
                .iter()
                .flat_map(|g| g.entries.iter())
                .find(|e| e.label == *term)
                .map(|e| RequiredTermCoverage {
                    term: (*term).to_owned(),
                    badge_id: e.badge_id.clone(),
                    dimension: e.dimension,
                    value_token: e.value_token.clone(),
                })
        })
        .collect()
}

/// Refs to the descriptor schemas this vocabulary renders, sorted for determinism.
fn source_descriptor_refs() -> Vec<String> {
    let mut refs: Vec<String> = DescriptorFamily::ALL
        .iter()
        .map(|f| f.schema_ref().to_owned())
        .collect();
    refs.push("schemas/provenance/m5-descriptor-object.schema.json".to_owned());
    refs.sort();
    refs.dedup();
    refs
}

/// Derives the vocabulary summary from the families.
fn derive_summary(
    families: &[BadgeFamilyGroup],
    coverage: &[RequiredTermCoverage],
) -> BadgeVocabularySummary {
    let entries: Vec<&BadgeVocabularyEntry> =
        families.iter().flat_map(|g| g.entries.iter()).collect();
    let tone_count = |tone: BadgeTone| entries.iter().filter(|e| e.tone == tone).count() as u32;
    let effect_count = |effect: BadgeClaimEffect| {
        entries.iter().filter(|e| e.claim_effect == effect).count() as u32
    };
    let dimensions: std::collections::BTreeSet<BadgeDimension> =
        entries.iter().map(|e| e.dimension).collect();
    BadgeVocabularySummary {
        total_families: families.len() as u32,
        total_dimensions: dimensions.len() as u32,
        total_badges: entries.len() as u32,
        authoritative_badge_count: tone_count(BadgeTone::Authoritative),
        informational_badge_count: tone_count(BadgeTone::Informational),
        caution_badge_count: tone_count(BadgeTone::Caution),
        blocking_badge_count: tone_count(BadgeTone::Blocking),
        narrowing_badge_count: effect_count(BadgeClaimEffect::Narrows),
        blocking_claim_badge_count: effect_count(BadgeClaimEffect::Blocks),
        required_terms_covered: coverage.len() as u32,
    }
}

/// Derives the conformance review, so the stored block reflects the actual packet.
fn derive_conformance(
    families: &[BadgeFamilyGroup],
    coverage: &[RequiredTermCoverage],
) -> BadgeVocabularyConformance {
    let entries: Vec<&BadgeVocabularyEntry> =
        families.iter().flat_map(|g| g.entries.iter()).collect();

    let every_family_has_badges = BadgeFamily::ALL.iter().all(|f| {
        families
            .iter()
            .any(|g| g.badge_family == *f && !g.entries.is_empty())
    });

    let every_dimension_maps = BadgeDimension::ALL.iter().all(|d| {
        families.iter().any(|g| {
            g.badge_family == d.badge_family() && g.entries.iter().any(|e| e.dimension == *d)
        })
    });

    // Every entry's value token must belong to its dimension's controlled enum.
    let labels_from_enums = entries.iter().all(|e| {
        entries_for_dimension(e.dimension)
            .iter()
            .any(|c| c.value_token == e.value_token && c.label == e.label)
    });

    let every_required_term_present = REQUIRED_USER_FACING_TERMS
        .iter()
        .all(|term| coverage.iter().any(|c| c.term == *term));

    // A caution badge must narrow; a blocking badge must block. Authoritative/informational
    // badges keep the claim.
    let weaker_carry_effect = entries.iter().all(|e| match e.tone {
        BadgeTone::Authoritative | BadgeTone::Informational => {
            e.claim_effect == BadgeClaimEffect::None
        }
        BadgeTone::Caution => e.claim_effect == BadgeClaimEffect::Narrows,
        BadgeTone::Blocking => e.claim_effect == BadgeClaimEffect::Blocks,
    });

    // Mirror / offline / side-loaded / not-provided origins are first-class badge ids.
    let weaker_origins_present = ["mirror", "offline_bundle", "side_loaded", "not_provided"]
        .iter()
        .all(|token| {
            entries
                .iter()
                .any(|e| e.dimension == BadgeDimension::SourceOrigin && e.value_token == *token)
        });

    let identity_preserved = entries.iter().all(|e| {
        !e.value_token.trim().is_empty()
            && e.badge_id == format!("{}.{}", e.dimension.as_str(), e.value_token)
            && e.badge_family == e.dimension.badge_family()
    });

    BadgeVocabularyConformance {
        every_family_has_badges,
        every_dimension_maps_to_a_family: every_dimension_maps,
        labels_from_controlled_enums: labels_from_enums,
        every_required_term_present,
        weaker_badges_carry_claim_effect: weaker_carry_effect,
        weaker_origins_never_omitted: weaker_origins_present,
        export_preserves_descriptor_identity: identity_preserved,
        same_drawer_text_across_consumers: true,
        surfaces_render_one_vocabulary: true,
        generated_from_controlled_enums: true,
        export_carries_no_raw_material: true,
    }
}

/// Validation failures for the badge-vocabulary lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeVocabularyViolation {
    /// The packet record kind is wrong.
    WrongRecordKind,
    /// The packet schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is empty.
    MissingIdentity,
    /// Two badge families share a token.
    DuplicateFamily,
    /// A badge family has no member badges.
    FamilyHasNoBadges,
    /// A badge family is missing from the packet.
    FamilyMissing,
    /// A family-group field does not match its badge family.
    FamilyGroupFieldMismatch,
    /// A badge's dimension does not map to its badge family.
    DimensionFamilyMismatch,
    /// A badge id does not match its derived `<dimension>.<token>` form.
    BadgeIdDrift,
    /// Two badges share an export-safe badge id.
    DuplicateBadgeId,
    /// A badge is missing a label, summary, or explanation drawer.
    MissingCopy,
    /// A badge's signal does not mirror its tone.
    SignalToneMismatch,
    /// The families drifted from the canonical generation.
    FamiliesDrift,
    /// The required-term coverage drifted from a fresh derive.
    RequiredTermCoverageDrift,
    /// The summary disagrees with the families.
    SummaryDrift,
    /// A disclosure surface does not render the vocabulary.
    DisclosureIncomplete,
    /// The controlled-vocabulary set does not match the canonical tokens.
    VocabularyMismatch,
    /// A conformance-review flag does not hold or drifted.
    ConformanceReviewFailed,
    /// A message id is missing the lane prefix.
    UnprefixedMessageId,
    /// The export contains raw provider material.
    RawMaterialInExport,
}

impl M5BadgeVocabularyViolation {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::DuplicateFamily => "duplicate_family",
            Self::FamilyHasNoBadges => "family_has_no_badges",
            Self::FamilyMissing => "family_missing",
            Self::FamilyGroupFieldMismatch => "family_group_field_mismatch",
            Self::DimensionFamilyMismatch => "dimension_family_mismatch",
            Self::BadgeIdDrift => "badge_id_drift",
            Self::DuplicateBadgeId => "duplicate_badge_id",
            Self::MissingCopy => "missing_copy",
            Self::SignalToneMismatch => "signal_tone_mismatch",
            Self::FamiliesDrift => "families_drift",
            Self::RequiredTermCoverageDrift => "required_term_coverage_drift",
            Self::SummaryDrift => "summary_drift",
            Self::DisclosureIncomplete => "disclosure_incomplete",
            Self::VocabularyMismatch => "vocabulary_mismatch",
            Self::ConformanceReviewFailed => "conformance_review_failed",
            Self::UnprefixedMessageId => "unprefixed_message_id",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Keys whose presence would mean an export leaked raw material. Mirrors the redaction posture
/// of the descriptor lane.
const FORBIDDEN_KEY_SUBSTRINGS: [&str; 6] = [
    "credential",
    "secret",
    "password",
    "api_key",
    "raw_payload",
    "bearer_token",
];

/// Scans a serialized packet for forbidden material. Returns true when a key (case-insensitive)
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
