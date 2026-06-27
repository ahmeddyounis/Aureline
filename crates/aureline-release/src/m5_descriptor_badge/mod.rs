//! The canonical M5 provenance / freshness / qualification / client-scope descriptor
//! and badge runtime.
//!
//! Claimed M5 release, ecosystem, docs, and companion surfaces all need to render the
//! same public-truth state: where an artifact came from, how fresh the evidence behind
//! it is, what support class it qualifies for, and which client scope it runs in. Before
//! this lane that vocabulary was split across local enums, prose, and ad hoc badges, so
//! a surface could quietly claim more than its evidence supports. This module freezes the
//! one shared runtime those surfaces consume.
//!
//! Four [descriptor families](DescriptorFamily) are reusable objects, each with a stable
//! [enum value vocabulary](DescriptorContract::value_tokens), a [badge family](BadgeFamily),
//! an explanation-drawer message id, a named first consumer, and the proof packet that
//! keeps it current:
//!
//! - [provenance](DescriptorFamily::Provenance) — source/origin
//!   ([`ProvenanceClass`]); `mirror`, `offline_bundle`, `side_loaded`, and `not_provided`
//!   are first-class tokens so a weaker origin can never disappear into omission;
//! - [freshness](DescriptorFamily::Freshness) — evidence currency
//!   ([`FreshnessState`]); stale evidence narrows, expired/missing evidence blocks;
//! - [qualification](DescriptorFamily::Qualification) — support class
//!   ([`QualificationClass`]); the claim a surface wants to keep;
//! - [client-scope](DescriptorFamily::ClientScope) — surface scope ([`ClientScope`]); a
//!   narrowed client can never imply authority or capability parity it does not have.
//!
//! Each [`PublicTruthConsumer`] surface binds the descriptor families it renders. The
//! matrix *derives*, per consumer, the exact [coverage gaps](DescriptorGap), a
//! [gate decision](DescriptorGate) the release/public-truth automation reads, and an
//! effective [qualification](QualificationClass): a stale descriptor proof deterministically
//! auto-narrows the consumers that bind it below Stable, and a missing/expired proof — or a
//! descriptor family the matrix does not govern at all — blocks them from Stable promotion,
//! with the gap named rather than hidden. The published [downgrade rules](DowngradeRule)
//! freeze how each non-authoritative provenance origin, non-current freshness state, and
//! narrowed client scope narrows or blocks a claim.
//!
//! The [`M5DescriptorBadgeMatrix`] is the one inspectable, serde-serializable truth packet
//! every consumer reads. The [`DescriptorDisclosure`] records that the release center,
//! Help/About, marketplace, docs/help, support exports, and companion handoffs all consume
//! *this* runtime rather than parallel badge or copy vocabularies. The packet carries
//! metadata and refs only: no credential bodies or raw provider payloads.
//!
//! - Packet schema:
//!   [`schemas/provenance/m5-descriptor-badge-matrix.schema.json`](../../../../../schemas/provenance/m5-descriptor-badge-matrix.schema.json)
//! - Contract doc:
//!   [`docs/public-truth/m5-descriptor-badge-matrix.md`](../../../../../docs/public-truth/m5-descriptor-badge-matrix.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_descriptor_contract, seeded_m5_descriptor_badge_matrix,
    seeded_m5_descriptor_badge_matrix_missing_proof_blocked,
    seeded_m5_descriptor_badge_matrix_stale_proof_narrowed, M5_DESCRIPTOR_BADGE_MATRIX_PACKET_ID,
};

use serde::{Deserialize, Serialize};

/// Record-kind tag carried by [`M5DescriptorBadgeMatrix`].
pub const M5_DESCRIPTOR_BADGE_RECORD_KIND: &str = "m5_descriptor_badge_matrix";

/// Schema version for the descriptor/badge matrix packet.
pub const M5_DESCRIPTOR_BADGE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the matrix packet schema.
pub const M5_DESCRIPTOR_BADGE_SCHEMA_REF: &str =
    "schemas/provenance/m5-descriptor-badge-matrix.schema.json";

/// Repo-relative path of the published matrix inventory.
pub const M5_DESCRIPTOR_BADGE_REF: &str = "artifacts/public-truth/m5-descriptor-badge-matrix.json";

/// Repo-relative path of the release-grade descriptor parity proof.
pub const M5_DESCRIPTOR_BADGE_PROOF_REF: &str =
    "artifacts/release/m5-descriptor-parity-proof/descriptor-badge-matrix.json";

/// Repo-relative path of the descriptor/badge governance matrix doc.
pub const M5_DESCRIPTOR_BADGE_GOVERNANCE_REF: &str =
    "artifacts/public-truth/m5-descriptor-badge-governance.md";

/// Repo-relative path of the descriptor/badge contract doc.
pub const M5_DESCRIPTOR_BADGE_DOC_REF: &str = "docs/public-truth/m5-descriptor-badge-matrix.md";

/// Repo-relative directory of the consumer drill fixtures.
pub const M5_DESCRIPTOR_BADGE_FIXTURE_DIR: &str = "fixtures/public-truth/m5-badge-consumers/";

/// Prefix every descriptor-lane message id carries so consumers can route it.
pub const M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX: &str = "public_truth_descriptor.";

/// One reusable descriptor family — a shared public-truth object with a stable enum value
/// vocabulary and a badge family. Binding a consumer surface to a family is what makes
/// that surface's claim depend on the family's proof staying current.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DescriptorFamily {
    /// Source / origin of an artifact or claim.
    Provenance,
    /// Currency of the evidence behind a claim.
    Freshness,
    /// Support class a surface qualifies for.
    Qualification,
    /// Client scope a surface runs in.
    ClientScope,
}

impl DescriptorFamily {
    /// Every descriptor family, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Provenance,
        Self::Freshness,
        Self::Qualification,
        Self::ClientScope,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Provenance => "provenance",
            Self::Freshness => "freshness",
            Self::Qualification => "qualification",
            Self::ClientScope => "client_scope",
        }
    }

    /// The badge family this descriptor renders through.
    pub const fn badge_family(self) -> BadgeFamily {
        match self {
            Self::Provenance => BadgeFamily::ProvenanceBadge,
            Self::Freshness => BadgeFamily::FreshnessBadge,
            Self::Qualification => BadgeFamily::QualificationBadge,
            Self::ClientScope => BadgeFamily::ClientScopeBadge,
        }
    }

    /// Reviewer-facing family label.
    pub const fn descriptor_label(self) -> &'static str {
        match self {
            Self::Provenance => "Provenance / source-origin descriptor",
            Self::Freshness => "Evidence-freshness descriptor",
            Self::Qualification => "Qualification / support-class descriptor",
            Self::ClientScope => "Client-scope descriptor",
        }
    }

    /// Stable descriptor object id, for cross-referencing the descriptor object.
    pub const fn descriptor_object_id(self) -> &'static str {
        match self {
            Self::Provenance => "m5-provenance-descriptor:stable:0001",
            Self::Freshness => "m5-freshness-descriptor:stable:0001",
            Self::Qualification => "m5-qualification-descriptor:stable:0001",
            Self::ClientScope => "m5-client-scope-descriptor:stable:0001",
        }
    }

    /// Repo-relative schema that is this descriptor object's source of truth.
    pub const fn schema_ref(self) -> &'static str {
        match self {
            Self::Provenance => "schemas/provenance/m5-provenance-descriptor.schema.json",
            Self::Freshness => "schemas/provenance/m5-freshness-descriptor.schema.json",
            Self::Qualification => "schemas/provenance/m5-qualification-descriptor.schema.json",
            Self::ClientScope => "schemas/provenance/m5-client-scope-descriptor.schema.json",
        }
    }

    /// Repo-relative proof packet that keeps this descriptor object current. The standalone
    /// descriptor artifact is the checked-in proof of the descriptor's frozen vocabulary.
    pub const fn proof_packet_ref(self) -> &'static str {
        match self {
            Self::Provenance => "artifacts/public-truth/descriptors/m5-provenance-descriptor.json",
            Self::Freshness => "artifacts/public-truth/descriptors/m5-freshness-descriptor.json",
            Self::Qualification => {
                "artifacts/public-truth/descriptors/m5-qualification-descriptor.json"
            }
            Self::ClientScope => {
                "artifacts/public-truth/descriptors/m5-client-scope-descriptor.json"
            }
        }
    }

    /// The first consumer surface that reads this descriptor object.
    pub const fn first_consumer(self) -> PublicTruthConsumer {
        match self {
            Self::Provenance => PublicTruthConsumer::HelpAbout,
            Self::Freshness => PublicTruthConsumer::ReleaseCenter,
            Self::Qualification => PublicTruthConsumer::ReleaseCenter,
            Self::ClientScope => PublicTruthConsumer::CompanionHandoff,
        }
    }

    /// Owner role accountable for keeping this descriptor object current.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::Provenance => "release_provenance_owner",
            Self::Freshness => "release_freshness_owner",
            Self::Qualification => "release_qualification_owner",
            Self::ClientScope => "companion_scope_owner",
        }
    }

    /// The stable enum value vocabulary for this descriptor object.
    pub fn value_tokens(self) -> Vec<&'static str> {
        match self {
            Self::Provenance => ProvenanceClass::ALL.iter().map(|c| c.as_str()).collect(),
            Self::Freshness => FreshnessState::ALL.iter().map(|c| c.as_str()).collect(),
            Self::Qualification => QualificationClass::ALL.iter().map(|c| c.as_str()).collect(),
            Self::ClientScope => ClientScope::ALL.iter().map(|c| c.as_str()).collect(),
        }
    }
}

/// One stable badge family. Each maps 1:1 to a [`DescriptorFamily`] so a surface renders
/// exactly one badge vocabulary per descriptor rather than hand-authoring copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeFamily {
    /// Provenance / source-origin badges.
    ProvenanceBadge,
    /// Evidence-freshness badges.
    FreshnessBadge,
    /// Qualification / support-class badges.
    QualificationBadge,
    /// Client-scope badges.
    ClientScopeBadge,
}

impl BadgeFamily {
    /// Every badge family, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ProvenanceBadge,
        Self::FreshnessBadge,
        Self::QualificationBadge,
        Self::ClientScopeBadge,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProvenanceBadge => "provenance_badge",
            Self::FreshnessBadge => "freshness_badge",
            Self::QualificationBadge => "qualification_badge",
            Self::ClientScopeBadge => "client_scope_badge",
        }
    }

    /// The descriptor family this badge family renders.
    pub const fn descriptor_family(self) -> DescriptorFamily {
        match self {
            Self::ProvenanceBadge => DescriptorFamily::Provenance,
            Self::FreshnessBadge => DescriptorFamily::Freshness,
            Self::QualificationBadge => DescriptorFamily::Qualification,
            Self::ClientScopeBadge => DescriptorFamily::ClientScope,
        }
    }
}

/// Provenance / source-origin vocabulary. Declaration order is most→least authoritative;
/// every origin below [`FirstPartySigned`](Self::FirstPartySigned) narrows a claim, and
/// [`NotProvided`](Self::NotProvided) blocks Stable. The weaker origins are first-class
/// tokens so a mirror, offline, side-loaded, or absent origin can never disappear into
/// omission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceClass {
    /// First-party, signed and attested by the project's release identity.
    FirstPartySigned,
    /// Authored by a known vendor / partner under a governed agreement.
    Vendor,
    /// Community-contributed, reviewed but not first-party.
    Community,
    /// Served from a current mirror copy of a first-party artifact.
    Mirror,
    /// Installed from an offline bundle whose origin is recorded but not live-verified.
    OfflineBundle,
    /// Side-loaded outside the governed channel.
    SideLoaded,
    /// Origin is not provided — recorded explicitly, never left blank.
    NotProvided,
}

impl ProvenanceClass {
    /// Every provenance class, in declaration order (most→least authoritative).
    pub const ALL: [Self; 7] = [
        Self::FirstPartySigned,
        Self::Vendor,
        Self::Community,
        Self::Mirror,
        Self::OfflineBundle,
        Self::SideLoaded,
        Self::NotProvided,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartySigned => "first_party_signed",
            Self::Vendor => "vendor",
            Self::Community => "community",
            Self::Mirror => "mirror",
            Self::OfflineBundle => "offline_bundle",
            Self::SideLoaded => "side_loaded",
            Self::NotProvided => "not_provided",
        }
    }

    /// True for the one origin that can carry an authoritative Stable claim.
    pub const fn is_authoritative(self) -> bool {
        matches!(self, Self::FirstPartySigned)
    }
}

/// Evidence-freshness vocabulary. Declaration order is most→least fresh. `current` keeps a
/// claim; `stale` narrows it; `expired` and `missing` block Stable promotion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessState {
    /// Evidence is within its freshness window.
    Current,
    /// Evidence has fallen outside its freshness window; consumers narrow.
    Stale,
    /// Evidence has passed its hard expiry; consumers block.
    Expired,
    /// No usable evidence exists; consumers block.
    Missing,
}

impl FreshnessState {
    /// Every freshness state, in declaration order (most→least fresh).
    pub const ALL: [Self; 4] = [Self::Current, Self::Stale, Self::Expired, Self::Missing];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Expired => "expired",
            Self::Missing => "missing",
        }
    }
}

/// Qualification / support-class vocabulary. Declaration order is most→least permissive;
/// the matrix narrows a claimed class down this ladder when evidence thins out, and floors
/// blocked consumers at [`Unavailable`](Self::Unavailable).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationClass {
    /// Stable, fully governed public claim.
    Stable,
    /// Beta, narrowed below Stable.
    Beta,
    /// Preview.
    Preview,
    /// Experimental, behind an explicit gate.
    Experimental,
    /// Deprecated; superseded but still documented.
    Deprecated,
    /// Unavailable / held from public claim.
    Unavailable,
}

impl QualificationClass {
    /// Every qualification class, in declaration order (most→least permissive).
    pub const ALL: [Self; 6] = [
        Self::Stable,
        Self::Beta,
        Self::Preview,
        Self::Experimental,
        Self::Deprecated,
        Self::Unavailable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Deprecated => "deprecated",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Client-scope vocabulary. Declaration order is most→least capable; only
/// [`DesktopFull`](Self::DesktopFull) carries full authority, and every narrower scope
/// narrows a claim so it can never imply authority or capability parity it lacks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientScope {
    /// The full desktop product surface with full authority.
    DesktopFull,
    /// A companion surface with bounded, host-relayed scope.
    CompanionScoped,
    /// A mobile companion surface.
    MobileCompanion,
    /// An embedded panel hosted inside another surface.
    EmbeddedPanel,
    /// A browser reference / read-only surface.
    BrowserReference,
    /// A surface that only creates or opens a desktop handoff.
    HandoffOnly,
}

impl ClientScope {
    /// Every client scope, in declaration order (most→least capable).
    pub const ALL: [Self; 6] = [
        Self::DesktopFull,
        Self::CompanionScoped,
        Self::MobileCompanion,
        Self::EmbeddedPanel,
        Self::BrowserReference,
        Self::HandoffOnly,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompanionScoped => "companion_scoped",
            Self::MobileCompanion => "mobile_companion",
            Self::EmbeddedPanel => "embedded_panel",
            Self::BrowserReference => "browser_reference",
            Self::HandoffOnly => "handoff_only",
        }
    }

    /// True for the one scope that can carry full authority / capability parity.
    pub const fn is_full_authority(self) -> bool {
        matches!(self, Self::DesktopFull)
    }
}

/// One public-truth consumer surface that reads the descriptor runtime. Naming each surface
/// is what proves the release center, Help/About, marketplace, docs/help, support exports,
/// and companion handoffs all read the same machine-readable runtime rather than parallel
/// badge or copy vocabularies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicTruthConsumer {
    /// The release center / public-truth automation.
    ReleaseCenter,
    /// The Help/About panel.
    HelpAbout,
    /// The marketplace / ecosystem listing surface.
    Marketplace,
    /// The docs / help reference surface.
    DocsHelp,
    /// The certification surface.
    Certification,
    /// Private evaluation packs.
    EvaluationPacks,
    /// Support exports / bundles.
    SupportExport,
    /// Companion handoff surfaces.
    CompanionHandoff,
}

impl PublicTruthConsumer {
    /// Every consumer, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ReleaseCenter,
        Self::HelpAbout,
        Self::Marketplace,
        Self::DocsHelp,
        Self::Certification,
        Self::EvaluationPacks,
        Self::SupportExport,
        Self::CompanionHandoff,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "release_center",
            Self::HelpAbout => "help_about",
            Self::Marketplace => "marketplace",
            Self::DocsHelp => "docs_help",
            Self::Certification => "certification",
            Self::EvaluationPacks => "evaluation_packs",
            Self::SupportExport => "support_export",
            Self::CompanionHandoff => "companion_handoff",
        }
    }

    /// Reviewer-facing consumer label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "Release center",
            Self::HelpAbout => "Help / About",
            Self::Marketplace => "Marketplace / ecosystem",
            Self::DocsHelp => "Docs / Help",
            Self::Certification => "Certification",
            Self::EvaluationPacks => "Evaluation packs",
            Self::SupportExport => "Support export",
            Self::CompanionHandoff => "Companion handoff",
        }
    }

    /// Owner role accountable for keeping this consumer's binding current.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "release_center_owner",
            Self::HelpAbout => "help_about_owner",
            Self::Marketplace => "marketplace_owner",
            Self::DocsHelp => "docs_help_owner",
            Self::Certification => "certification_owner",
            Self::EvaluationPacks => "evaluation_pack_owner",
            Self::SupportExport => "support_export_owner",
            Self::CompanionHandoff => "companion_owner",
        }
    }
}

/// Release-gate decision the release/public-truth automation reads for a consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DescriptorGate {
    /// The consumer maps every bound descriptor to a current proof; full claim stands.
    Governed,
    /// A bound descriptor's proof is stale; the claim auto-narrows below Stable.
    Narrowed,
    /// A bound descriptor's proof is expired/missing or the family is unmapped; blocked.
    Blocked,
}

impl DescriptorGate {
    /// Every gate decision, in declaration order.
    pub const ALL: [Self; 3] = [Self::Governed, Self::Narrowed, Self::Blocked];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Governed => "governed",
            Self::Narrowed => "narrowed",
            Self::Blocked => "blocked",
        }
    }

    /// True when the gate blocks Stable promotion.
    pub const fn blocks(self) -> bool {
        matches!(self, Self::Blocked)
    }
}

/// Coverage status of a consumer's descriptor bindings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerStatus {
    /// Every bound descriptor maps to a current proof.
    Mapped,
    /// At least one bound descriptor's proof is stale (narrowed).
    Provisional,
    /// At least one bound descriptor is expired/missing/unmapped (blocked).
    Unmapped,
}

impl ConsumerStatus {
    /// Every status, in declaration order.
    pub const ALL: [Self; 3] = [Self::Mapped, Self::Provisional, Self::Unmapped];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Mapped => "mapped",
            Self::Provisional => "provisional",
            Self::Unmapped => "unmapped",
        }
    }

    /// Traffic-light signal for this status.
    pub const fn signal(self) -> DescriptorSignal {
        match self {
            Self::Mapped => DescriptorSignal::Green,
            Self::Provisional => DescriptorSignal::Yellow,
            Self::Unmapped => DescriptorSignal::Red,
        }
    }
}

/// Traffic-light signal mirroring a [`ConsumerStatus`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DescriptorSignal {
    /// Fully governed.
    Green,
    /// Narrowed below the claim.
    Yellow,
    /// Blocked from Stable promotion.
    Red,
}

impl DescriptorSignal {
    /// Every signal, in declaration order.
    pub const ALL: [Self; 3] = [Self::Green, Self::Yellow, Self::Red];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }
}

/// The kind of coverage gap on a consumer's bound descriptor family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DescriptorGapKind {
    /// The consumer binds a descriptor family the matrix does not govern.
    DescriptorMappingMissing,
    /// A bound descriptor's proof is stale.
    ProofStale,
    /// A bound descriptor's proof is expired.
    ProofExpired,
    /// A bound descriptor's proof is missing.
    ProofMissing,
}

impl DescriptorGapKind {
    /// Every gap kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DescriptorMappingMissing,
        Self::ProofStale,
        Self::ProofExpired,
        Self::ProofMissing,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DescriptorMappingMissing => "descriptor_mapping_missing",
            Self::ProofStale => "proof_stale",
            Self::ProofExpired => "proof_expired",
            Self::ProofMissing => "proof_missing",
        }
    }

    /// True when this gap blocks Stable promotion (vs only narrowing it).
    pub const fn is_blocking(self) -> bool {
        matches!(
            self,
            Self::DescriptorMappingMissing | Self::ProofExpired | Self::ProofMissing
        )
    }
}

/// The effect a [`DowngradeRule`] applies when its trigger holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeEffect {
    /// Narrow the effective claim to at most the rule's floor.
    Narrow,
    /// Block the claim from Stable promotion entirely.
    Block,
}

impl DowngradeEffect {
    /// Every effect, in declaration order.
    pub const ALL: [Self; 2] = [Self::Narrow, Self::Block];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Narrow => "narrow",
            Self::Block => "block",
        }
    }
}

/// One frozen downgrade rule: how a non-authoritative descriptor value narrows or blocks a
/// claim. The rule set is the published downgrade vocabulary; the conformance review proves
/// it covers every non-authoritative origin, non-current freshness state, and narrowed
/// client scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DowngradeRule {
    /// Stable rule id, unique within the packet.
    pub rule_id: String,
    /// The descriptor family the trigger value belongs to.
    pub trigger_family: DescriptorFamily,
    /// The descriptor value token that triggers the rule.
    pub trigger_token: String,
    /// What the rule does.
    pub effect: DowngradeEffect,
    /// The lowest qualification a narrowing rule floors the claim at (the floor for a
    /// blocking rule is [`QualificationClass::Unavailable`]).
    pub effective_floor: QualificationClass,
    /// Stable message id; prefixed [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub rationale_message_id: String,
}

impl DowngradeRule {
    fn new(
        trigger_family: DescriptorFamily,
        trigger_token: &str,
        effect: DowngradeEffect,
        effective_floor: QualificationClass,
    ) -> Self {
        Self {
            rule_id: format!("{}.{trigger_token}", trigger_family.as_str()),
            trigger_family,
            trigger_token: trigger_token.to_owned(),
            effect,
            effective_floor,
            rationale_message_id: format!(
                "{}downgrade.{}.{trigger_token}",
                M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX,
                trigger_family.as_str()
            ),
        }
    }
}

/// Builds the canonical downgrade-rule set from the frozen vocabularies. A weaker origin,
/// a non-current freshness state, or a narrower client scope each narrows the claim;
/// absent provenance and expired/missing evidence block it.
pub fn canonical_downgrade_rules() -> Vec<DowngradeRule> {
    let mut rules = Vec::new();

    // Provenance: anything below first-party-signed narrows; absent origin blocks.
    for class in ProvenanceClass::ALL {
        if class.is_authoritative() {
            continue;
        }
        let (effect, floor) = if matches!(class, ProvenanceClass::NotProvided) {
            (DowngradeEffect::Block, QualificationClass::Unavailable)
        } else {
            (DowngradeEffect::Narrow, QualificationClass::Beta)
        };
        rules.push(DowngradeRule::new(
            DescriptorFamily::Provenance,
            class.as_str(),
            effect,
            floor,
        ));
    }

    // Freshness: stale narrows; expired/missing block.
    for state in FreshnessState::ALL {
        let (effect, floor) = match state {
            FreshnessState::Current => continue,
            FreshnessState::Stale => (DowngradeEffect::Narrow, QualificationClass::Beta),
            FreshnessState::Expired | FreshnessState::Missing => {
                (DowngradeEffect::Block, QualificationClass::Unavailable)
            }
        };
        rules.push(DowngradeRule::new(
            DescriptorFamily::Freshness,
            state.as_str(),
            effect,
            floor,
        ));
    }

    // Client scope: any scope below full desktop narrows so it cannot imply parity.
    for scope in ClientScope::ALL {
        if scope.is_full_authority() {
            continue;
        }
        rules.push(DowngradeRule::new(
            DescriptorFamily::ClientScope,
            scope.as_str(),
            DowngradeEffect::Narrow,
            QualificationClass::Beta,
        ));
    }

    rules
}

/// One descriptor object's contract: its badge family, frozen value vocabulary, owner,
/// schema, first consumer, proof packet, and the freshness of that proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorContract {
    /// The descriptor family.
    pub family: DescriptorFamily,
    /// Stable descriptor object id.
    pub descriptor_object_id: String,
    /// Reviewer-facing descriptor label.
    pub descriptor_label: String,
    /// The badge family this descriptor renders through.
    pub badge_family: BadgeFamily,
    /// The frozen stable enum value vocabulary.
    pub value_tokens: Vec<String>,
    /// The badge tokens — identical to [`Self::value_tokens`] so a badge resolves to a value.
    pub badge_tokens: Vec<String>,
    /// Stable message id of the badge explanation drawer; prefixed
    /// [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub explanation_drawer_message_id: String,
    /// Owner role accountable for keeping this descriptor current.
    pub owner_role: String,
    /// The first consumer surface that reads this descriptor.
    pub first_consumer: PublicTruthConsumer,
    /// Repo-relative source-of-truth schema.
    pub schema_ref: String,
    /// Repo-relative proof packet that keeps this descriptor current.
    pub proof_packet_ref: String,
    /// Freshness of the descriptor's proof.
    pub proof_freshness: FreshnessState,
    /// Stable message id; prefixed [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

impl DescriptorContract {
    /// Builds a descriptor contract for a family at a given proof freshness, deriving every
    /// field from the family so a contract can never cite a ref that drifts from it.
    pub fn for_family(family: DescriptorFamily, proof_freshness: FreshnessState) -> Self {
        let value_tokens: Vec<String> = family
            .value_tokens()
            .iter()
            .map(|s| (*s).to_owned())
            .collect();
        Self {
            family,
            descriptor_object_id: family.descriptor_object_id().to_owned(),
            descriptor_label: family.descriptor_label().to_owned(),
            badge_family: family.badge_family(),
            badge_tokens: value_tokens.clone(),
            value_tokens,
            explanation_drawer_message_id: format!(
                "{}drawer.{}",
                M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX,
                family.as_str()
            ),
            owner_role: family.owner_role().to_owned(),
            first_consumer: family.first_consumer(),
            schema_ref: family.schema_ref().to_owned(),
            proof_packet_ref: family.proof_packet_ref().to_owned(),
            proof_freshness,
            detail_message_id: format!(
                "{}descriptor.{}",
                M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX,
                family.as_str()
            ),
        }
    }

    /// Validates the descriptor contract's invariants: every derived field matches the
    /// family, the badge family maps back, the value/badge vocabularies match, and the
    /// message ids carry the lane prefix.
    pub fn validate(&self) -> Vec<M5DescriptorBadgeViolation> {
        let mut out = Vec::new();
        let canonical: Vec<String> = self
            .family
            .value_tokens()
            .iter()
            .map(|s| (*s).to_owned())
            .collect();
        if self.descriptor_object_id != self.family.descriptor_object_id()
            || self.badge_family != self.family.badge_family()
            || self.schema_ref != self.family.schema_ref()
            || self.proof_packet_ref != self.family.proof_packet_ref()
            || self.first_consumer != self.family.first_consumer()
            || self.value_tokens != canonical
        {
            out.push(M5DescriptorBadgeViolation::DescriptorContractFieldMismatch);
        }
        if self.badge_family.descriptor_family() != self.family
            || self.badge_tokens != self.value_tokens
        {
            out.push(M5DescriptorBadgeViolation::BadgeFamilyMismatch);
        }
        if self.descriptor_label.trim().is_empty() || self.owner_role.trim().is_empty() {
            out.push(M5DescriptorBadgeViolation::MissingIdentity);
        }
        if !self
            .detail_message_id
            .starts_with(M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX)
            || !self
                .explanation_drawer_message_id
                .starts_with(M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX)
        {
            out.push(M5DescriptorBadgeViolation::UnprefixedMessageId);
        }
        out
    }
}

/// One coverage gap on a claimed consumer: a bound descriptor family the matrix does not
/// govern, or a bound family whose proof is stale, expired, or missing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorGap {
    /// Consumer this gap applies to.
    pub consumer: PublicTruthConsumer,
    /// The bound descriptor family the gap concerns.
    pub family: DescriptorFamily,
    /// The kind of gap.
    pub gap_kind: DescriptorGapKind,
    /// Stable message id; prefixed [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub cause_message_id: String,
}

/// Derived verdict for a consumer, computed from its gaps.
struct ConsumerVerdict {
    status: ConsumerStatus,
    signal: DescriptorSignal,
    gate: DescriptorGate,
    effective_qualification: QualificationClass,
}

/// Restrictiveness rank of a qualification class, from the canonical `ALL` ordering (least
/// restrictive first) so the matrix reuses the shipped support-class ladder.
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

fn derive_consumer_verdict(claimed: QualificationClass, gaps: &[DescriptorGap]) -> ConsumerVerdict {
    let any_blocking = gaps.iter().any(|g| g.gap_kind.is_blocking());
    let any_narrowing = gaps.iter().any(|g| !g.gap_kind.is_blocking());

    let status = if any_blocking {
        ConsumerStatus::Unmapped
    } else if any_narrowing {
        ConsumerStatus::Provisional
    } else {
        ConsumerStatus::Mapped
    };

    let gate = if any_blocking {
        DescriptorGate::Blocked
    } else if any_narrowing {
        DescriptorGate::Narrowed
    } else {
        DescriptorGate::Governed
    };

    let effective_qualification = match gate {
        DescriptorGate::Governed => claimed,
        DescriptorGate::Blocked => QualificationClass::Unavailable,
        // A stale descriptor proof always narrows the claim to at least Beta — deterministic
        // and never a quiet Stable claim over thinned evidence.
        DescriptorGate::Narrowed => more_restrictive(claimed, QualificationClass::Beta),
    };

    ConsumerVerdict {
        status,
        signal: status.signal(),
        gate,
        effective_qualification,
    }
}

/// One claimed public-truth consumer surface: the descriptor families it binds and the
/// verdict derived from those descriptors' proof freshness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerBinding {
    /// The consumer surface.
    pub consumer: PublicTruthConsumer,
    /// Reviewer-facing consumer label.
    pub consumer_label: String,
    /// Owner role accountable for keeping this consumer's binding current.
    pub owner_role: String,
    /// Public qualification the consumer wants to keep.
    pub claimed_qualification: QualificationClass,
    /// The descriptor families this consumer binds.
    pub bound_families: Vec<DescriptorFamily>,
    /// The badge families this consumer's bound descriptors render, in family order.
    pub covered_badge_families: Vec<BadgeFamily>,
    /// Effective qualification after the gate applies.
    pub effective_qualification: QualificationClass,
    /// Coverage status.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Release-gate decision the release/public-truth automation reads.
    pub gate_decision: DescriptorGate,
    /// Exact coverage gaps for this consumer.
    pub gaps: Vec<DescriptorGap>,
    /// Stable message id for the status; prefixed [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub status_message_id: String,
    /// Stable message id for the gate; prefixed [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub gate_message_id: String,
}

impl ConsumerBinding {
    /// Builds a consumer binding from its claimed qualification and bound families; gaps and
    /// verdict are recomputed from the descriptor contracts when the packet is assembled.
    pub fn new(
        consumer: PublicTruthConsumer,
        claimed_qualification: QualificationClass,
        bound_families: &[DescriptorFamily],
    ) -> Self {
        Self {
            consumer,
            consumer_label: consumer.label().to_owned(),
            owner_role: consumer.owner_role().to_owned(),
            claimed_qualification,
            bound_families: bound_families.to_vec(),
            covered_badge_families: Vec::new(),
            effective_qualification: claimed_qualification,
            status: ConsumerStatus::Mapped,
            signal: DescriptorSignal::Green,
            gate_decision: DescriptorGate::Governed,
            gaps: Vec::new(),
            status_message_id: format!(
                "{}{}.status",
                M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX,
                consumer.as_str()
            ),
            gate_message_id: format!(
                "{}{}.gate",
                M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX,
                consumer.as_str()
            ),
        }
    }

    /// Recomputes the gaps, covered badge families, and verdict from the descriptor
    /// contracts, so a consumer's claim is always generated from the same checked-in
    /// descriptor proofs the matrix ships rather than a hand-maintained status.
    pub fn recompute(&mut self, descriptors: &[DescriptorContract]) {
        let mut gaps = Vec::new();
        let consumer = self.consumer;
        let mut push_gap = |family: DescriptorFamily, kind: DescriptorGapKind| {
            gaps.push(DescriptorGap {
                consumer,
                family,
                gap_kind: kind,
                cause_message_id: format!(
                    "{}{}.{}.{}.gap",
                    M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX,
                    consumer.as_str(),
                    family.as_str(),
                    kind.as_str()
                ),
            });
        };

        for &family in &self.bound_families {
            match descriptors.iter().find(|c| c.family == family) {
                None => push_gap(family, DescriptorGapKind::DescriptorMappingMissing),
                Some(contract) => match contract.proof_freshness {
                    FreshnessState::Current => {}
                    FreshnessState::Stale => push_gap(family, DescriptorGapKind::ProofStale),
                    FreshnessState::Expired => push_gap(family, DescriptorGapKind::ProofExpired),
                    FreshnessState::Missing => push_gap(family, DescriptorGapKind::ProofMissing),
                },
            }
        }

        gaps.sort_by(|a, b| {
            a.family
                .as_str()
                .cmp(b.family.as_str())
                .then(a.gap_kind.as_str().cmp(b.gap_kind.as_str()))
        });
        self.gaps = gaps;

        let mut families = self.bound_families.clone();
        families.sort_by_key(family_rank);
        families.dedup();
        self.bound_families = families.clone();
        self.covered_badge_families = families.iter().map(|f| f.badge_family()).collect();

        let verdict = derive_consumer_verdict(self.claimed_qualification, &self.gaps);
        self.status = verdict.status;
        self.signal = verdict.signal;
        self.gate_decision = verdict.gate;
        self.effective_qualification = verdict.effective_qualification;
    }

    /// True when the consumer is blocked from Stable promotion.
    pub fn is_blocked(&self) -> bool {
        self.gate_decision.blocks()
    }

    /// True when the consumer auto-narrowed below its claim.
    pub fn is_narrowed(&self) -> bool {
        matches!(self.gate_decision, DescriptorGate::Narrowed)
    }

    /// True when the consumer is fully governed for Stable promotion.
    pub fn is_governed(&self) -> bool {
        matches!(self.gate_decision, DescriptorGate::Governed)
    }

    /// Validates the consumer's static invariants (identity, bound families, message ids).
    fn validate_static(&self) -> Vec<M5DescriptorBadgeViolation> {
        let mut out = Vec::new();
        if self.consumer_label.trim().is_empty() || self.owner_role.trim().is_empty() {
            out.push(M5DescriptorBadgeViolation::MissingIdentity);
        }
        if self.bound_families.is_empty() {
            out.push(M5DescriptorBadgeViolation::ConsumerBindsNoDescriptors);
        }
        if !self
            .status_message_id
            .starts_with(M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX)
            || !self
                .gate_message_id
                .starts_with(M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX)
        {
            out.push(M5DescriptorBadgeViolation::UnprefixedMessageId);
        }
        for gap in &self.gaps {
            if !gap
                .cause_message_id
                .starts_with(M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX)
            {
                out.push(M5DescriptorBadgeViolation::UnprefixedMessageId);
            }
        }
        out
    }
}

/// Position of a descriptor family in the canonical ordering.
fn family_rank(family: &DescriptorFamily) -> usize {
    DescriptorFamily::ALL
        .iter()
        .position(|f| f == family)
        .unwrap_or(DescriptorFamily::ALL.len())
}

/// Compact matrix summary — the scoreboard every consumer surface reads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorBadgeSummary {
    /// Total governed descriptor objects.
    pub total_descriptors: u32,
    /// Total badge families.
    pub total_badge_families: u32,
    /// Total downgrade rules.
    pub total_downgrade_rules: u32,
    /// Total claimed consumers.
    pub total_consumers: u32,
    /// Consumers governed at their full claim.
    pub governed_consumer_count: u32,
    /// Consumers that auto-narrowed below their claim.
    pub narrowed_consumer_count: u32,
    /// Consumers blocked from Stable promotion.
    pub blocked_consumer_count: u32,
    /// Descriptors whose proof is current.
    pub current_descriptor_count: u32,
    /// Descriptors whose proof is stale.
    pub stale_descriptor_count: u32,
    /// Descriptors whose proof is expired.
    pub expired_descriptor_count: u32,
    /// Descriptors whose proof is missing.
    pub missing_descriptor_count: u32,
    /// True when at least one consumer is blocked from Stable promotion.
    pub blocks_stable_promotion: bool,
}

/// Packet-level release gate aggregating the per-consumer gates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorReleaseGate {
    /// True when at least one consumer is blocked from Stable promotion.
    pub blocks_stable_promotion: bool,
    /// Sorted consumer tokens blocked from Stable promotion.
    pub blocked_consumers: Vec<String>,
    /// Sorted consumer tokens that auto-narrowed below their claim.
    pub narrowed_consumers: Vec<String>,
    /// Sorted consumer tokens fully governed for Stable promotion.
    pub governed_consumers: Vec<String>,
    /// Stable message id; prefixed [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub gate_message_id: String,
}

/// Which public-truth surfaces consume the one descriptor runtime. Every flag must hold so
/// no surface maintains a parallel badge or copy vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorDisclosure {
    /// The release center consumes the runtime.
    pub release_center_consumes_runtime: bool,
    /// The Help/About panel consumes the runtime.
    pub help_about_consumes_runtime: bool,
    /// The marketplace / ecosystem surface consumes the runtime.
    pub marketplace_consumes_runtime: bool,
    /// The docs / help surface consumes the runtime.
    pub docs_help_consumes_runtime: bool,
    /// Support exports consume the runtime.
    pub support_export_consumes_runtime: bool,
    /// Companion handoffs consume the runtime.
    pub companion_handoff_consumes_runtime: bool,
}

impl DescriptorDisclosure {
    /// The canonical disclosure: every surface consumes the runtime.
    pub const fn all_surfaces() -> Self {
        Self {
            release_center_consumes_runtime: true,
            help_about_consumes_runtime: true,
            marketplace_consumes_runtime: true,
            docs_help_consumes_runtime: true,
            support_export_consumes_runtime: true,
            companion_handoff_consumes_runtime: true,
        }
    }

    /// True when every surface consumes the runtime.
    pub const fn all_consume(&self) -> bool {
        self.release_center_consumes_runtime
            && self.help_about_consumes_runtime
            && self.marketplace_consumes_runtime
            && self.docs_help_consumes_runtime
            && self.support_export_consumes_runtime
            && self.companion_handoff_consumes_runtime
    }
}

/// Self-describing controlled-vocabulary set so the packet resolves every token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorVocabulary {
    /// Descriptor-family tokens.
    pub descriptor_families: Vec<String>,
    /// Badge-family tokens.
    pub badge_families: Vec<String>,
    /// Provenance-class tokens.
    pub provenance_classes: Vec<String>,
    /// Freshness-state tokens.
    pub freshness_states: Vec<String>,
    /// Qualification-class tokens.
    pub qualification_classes: Vec<String>,
    /// Client-scope tokens.
    pub client_scopes: Vec<String>,
    /// Consumer tokens.
    pub consumers: Vec<String>,
    /// Gate-decision tokens.
    pub gate_decisions: Vec<String>,
    /// Signal tokens.
    pub signals: Vec<String>,
    /// Consumer-status tokens.
    pub consumer_statuses: Vec<String>,
    /// Gap-kind tokens.
    pub gap_kinds: Vec<String>,
    /// Downgrade-effect tokens.
    pub downgrade_effects: Vec<String>,
}

impl DescriptorVocabulary {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        let tokens = |slice: &[&str]| slice.iter().map(|s| (*s).to_owned()).collect();
        Self {
            descriptor_families: DescriptorFamily::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            badge_families: BadgeFamily::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            provenance_classes: ProvenanceClass::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            freshness_states: FreshnessState::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            qualification_classes: QualificationClass::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            client_scopes: ClientScope::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            consumers: PublicTruthConsumer::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            gate_decisions: DescriptorGate::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            signals: DescriptorSignal::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            consumer_statuses: ConsumerStatus::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            gap_kinds: DescriptorGapKind::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            downgrade_effects: tokens(
                &DowngradeEffect::ALL
                    .iter()
                    .map(|c| c.as_str())
                    .collect::<Vec<_>>(),
            ),
        }
    }

    /// True when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Conformance review. Every flag is a hard invariant; all must hold.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorConformance {
    /// Every descriptor family has a governed descriptor contract.
    pub every_family_has_a_descriptor: bool,
    /// Every badge family maps to a governed descriptor.
    pub every_badge_family_maps_to_a_descriptor: bool,
    /// Every claimed consumer binds at least one descriptor family.
    pub every_consumer_binds_at_least_one_descriptor: bool,
    /// Every consumer maps to current descriptors or auto-narrows/blocks.
    pub every_consumer_maps_to_descriptors_or_narrows: bool,
    /// A stale descriptor proof narrows the consumers that bind it deterministically.
    pub stale_proof_narrows_deterministically: bool,
    /// An expired/missing/unmapped descriptor blocks Stable promotion for its consumers.
    pub missing_descriptor_blocks_stable_promotion: bool,
    /// Exact coverage gaps are named per consumer.
    pub exact_gaps_named_per_consumer: bool,
    /// Mirror/offline/side-loaded/not-provided origins are first-class, never omitted.
    pub weaker_origins_never_omitted: bool,
    /// Every non-authoritative descriptor value has a downgrade rule.
    pub downgrade_rules_cover_every_weaker_value: bool,
    /// Release center, Help/About, marketplace, docs/help, support, companion read one runtime.
    pub surfaces_consume_one_runtime: bool,
    /// The matrix is generated from the same checked-in descriptor proofs.
    pub generated_from_checked_in_descriptors: bool,
    /// The export carries no raw provider material.
    pub export_carries_no_raw_material: bool,
}

impl DescriptorConformance {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.every_family_has_a_descriptor
            && self.every_badge_family_maps_to_a_descriptor
            && self.every_consumer_binds_at_least_one_descriptor
            && self.every_consumer_maps_to_descriptors_or_narrows
            && self.stale_proof_narrows_deterministically
            && self.missing_descriptor_blocks_stable_promotion
            && self.exact_gaps_named_per_consumer
            && self.weaker_origins_never_omitted
            && self.downgrade_rules_cover_every_weaker_value
            && self.surfaces_consume_one_runtime
            && self.generated_from_checked_in_descriptors
            && self.export_carries_no_raw_material
    }
}

/// Constructor input for [`M5DescriptorBadgeMatrix::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DescriptorBadgeMatrixInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the packet was computed as-of.
    pub evaluated_at: String,
    /// The governed descriptor contracts.
    pub descriptors: Vec<DescriptorContract>,
    /// The claimed consumer bindings (gaps/verdict are recomputed from the descriptors).
    pub consumer_bindings: Vec<ConsumerBinding>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 descriptor/badge matrix: the canonical provenance, freshness,
/// qualification, and client-scope descriptor and badge runtime, plus the qualification of
/// every claimed public-truth consumer against it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DescriptorBadgeMatrix {
    /// Record kind; must equal [`M5_DESCRIPTOR_BADGE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DESCRIPTOR_BADGE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the packet was computed as-of.
    pub evaluated_at: String,
    /// The governed descriptor contracts.
    pub descriptors: Vec<DescriptorContract>,
    /// The frozen downgrade rules.
    pub downgrade_rules: Vec<DowngradeRule>,
    /// The claimed consumer bindings with their derived verdicts.
    pub consumer_bindings: Vec<ConsumerBinding>,
    /// Compact matrix summary.
    pub summary: DescriptorBadgeSummary,
    /// Which surfaces consume the runtime.
    pub disclosure: DescriptorDisclosure,
    /// Packet-level release gate.
    pub release_gate: DescriptorReleaseGate,
    /// Controlled-vocabulary set.
    pub vocabulary: DescriptorVocabulary,
    /// Conformance review block.
    pub conformance: DescriptorConformance,
    /// Cross-refs to the descriptor proof packets this matrix governs.
    pub source_proof_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DescriptorBadgeMatrix {
    /// Builds a matrix from seed input, recomputing each consumer's verdict and deriving the
    /// downgrade rules, summary, release gate, and conformance review from the descriptors.
    pub fn new(input: M5DescriptorBadgeMatrixInput) -> Self {
        let descriptors = input.descriptors;
        let mut consumer_bindings = input.consumer_bindings;
        for binding in &mut consumer_bindings {
            binding.recompute(&descriptors);
        }
        let downgrade_rules = canonical_downgrade_rules();
        let summary = derive_summary(&descriptors, &downgrade_rules, &consumer_bindings);
        let release_gate = derive_release_gate(&consumer_bindings);
        let conformance = derive_conformance(&descriptors, &downgrade_rules, &consumer_bindings);
        let mut source_proof_refs: Vec<String> = descriptors
            .iter()
            .map(|d| d.proof_packet_ref.clone())
            .collect();
        source_proof_refs.sort();
        Self {
            record_kind: M5_DESCRIPTOR_BADGE_RECORD_KIND.to_owned(),
            schema_version: M5_DESCRIPTOR_BADGE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            report_label: input.report_label,
            evaluated_at: input.evaluated_at,
            descriptors,
            downgrade_rules,
            consumer_bindings,
            summary,
            disclosure: DescriptorDisclosure::all_surfaces(),
            release_gate,
            vocabulary: DescriptorVocabulary::canonical(),
            conformance,
            source_proof_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// True when the release/public-truth automation must hold Stable promotion.
    pub fn blocks_stable_promotion(&self) -> bool {
        self.release_gate.blocks_stable_promotion
    }

    /// Finds a descriptor contract by family.
    pub fn descriptor(&self, family: DescriptorFamily) -> Option<&DescriptorContract> {
        self.descriptors.iter().find(|c| c.family == family)
    }

    /// Finds a consumer binding by consumer.
    pub fn consumer_binding(&self, consumer: PublicTruthConsumer) -> Option<&ConsumerBinding> {
        self.consumer_bindings
            .iter()
            .find(|b| b.consumer == consumer)
    }

    /// Validates the matrix's invariants.
    pub fn validate(&self) -> Vec<M5DescriptorBadgeViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_DESCRIPTOR_BADGE_RECORD_KIND {
            out.push(M5DescriptorBadgeViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DESCRIPTOR_BADGE_SCHEMA_VERSION {
            out.push(M5DescriptorBadgeViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.evaluated_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            out.push(M5DescriptorBadgeViolation::MissingIdentity);
        }

        // Every descriptor contract must be self-consistent; duplicate families are rejected.
        let mut seen_families = std::collections::BTreeSet::new();
        for contract in &self.descriptors {
            if !seen_families.insert(contract.family) {
                out.push(M5DescriptorBadgeViolation::DuplicateDescriptor);
            }
            out.extend(contract.validate());
        }
        for family in DescriptorFamily::ALL {
            if !self.descriptors.iter().any(|c| c.family == family) {
                out.push(M5DescriptorBadgeViolation::FamilyNotGoverned);
            }
        }
        for badge in BadgeFamily::ALL {
            if !self.descriptors.iter().any(|c| c.badge_family == badge) {
                out.push(M5DescriptorBadgeViolation::BadgeFamilyMismatch);
            }
        }

        if self.downgrade_rules != canonical_downgrade_rules() {
            out.push(M5DescriptorBadgeViolation::DowngradeRulesDrift);
        }

        if self.consumer_bindings.is_empty() {
            out.push(M5DescriptorBadgeViolation::PacketHasNoConsumers);
        }
        let mut seen_consumers = std::collections::BTreeSet::new();
        for binding in &self.consumer_bindings {
            if !seen_consumers.insert(binding.consumer) {
                out.push(M5DescriptorBadgeViolation::DuplicateConsumer);
            }
            out.extend(binding.validate_static());
            // The stored verdict must match a fresh recompute from the descriptor proofs.
            let mut probe = binding.clone();
            probe.recompute(&self.descriptors);
            if probe.gaps != binding.gaps
                || probe.covered_badge_families != binding.covered_badge_families
                || probe.status != binding.status
                || probe.signal != binding.signal
                || probe.gate_decision != binding.gate_decision
                || probe.effective_qualification != binding.effective_qualification
            {
                out.push(M5DescriptorBadgeViolation::ConsumerVerdictDrift);
            }
        }

        if self.summary
            != derive_summary(
                &self.descriptors,
                &self.downgrade_rules,
                &self.consumer_bindings,
            )
        {
            out.push(M5DescriptorBadgeViolation::SummaryDrift);
        }
        if self.release_gate != derive_release_gate(&self.consumer_bindings) {
            out.push(M5DescriptorBadgeViolation::ReleaseGateAggregateMismatch);
        }
        if !self.disclosure.all_consume() {
            out.push(M5DescriptorBadgeViolation::DisclosureIncomplete);
        }
        if !self.vocabulary.matches_canonical() {
            out.push(M5DescriptorBadgeViolation::VocabularyMismatch);
        }
        if self.conformance
            != derive_conformance(
                &self.descriptors,
                &self.downgrade_rules,
                &self.consumer_bindings,
            )
            || !self.conformance.all_hold()
        {
            out.push(M5DescriptorBadgeViolation::ConformanceReviewFailed);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 descriptor badge serializes"),
        ) {
            out.push(M5DescriptorBadgeViolation::RawMaterialInExport);
        }

        out
    }

    /// Deterministic export-safe JSON for the packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 descriptor badge serializes")
    }

    /// Deterministic Markdown governance matrix for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Descriptor / Badge Governance Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!(
            "- Descriptors: {} ({} current, {} stale, {} expired, {} missing)\n",
            self.summary.total_descriptors,
            self.summary.current_descriptor_count,
            self.summary.stale_descriptor_count,
            self.summary.expired_descriptor_count,
            self.summary.missing_descriptor_count
        ));
        out.push_str(&format!(
            "- Consumers: {} ({} governed, {} narrowed, {} blocked)\n",
            self.summary.total_consumers,
            self.summary.governed_consumer_count,
            self.summary.narrowed_consumer_count,
            self.summary.blocked_consumer_count
        ));
        out.push_str(&format!(
            "- Downgrade rules: {}\n",
            self.summary.total_downgrade_rules
        ));
        out.push_str(&format!(
            "- Release gate: {}\n",
            if self.summary.blocks_stable_promotion {
                "blocked"
            } else {
                "pass"
            }
        ));
        out.push_str(
            "- Consumed by: release center, Help/About, marketplace, docs/help, support, companion\n",
        );

        out.push_str("\n## Descriptor objects and badge families\n\n");
        out.push_str(
            "| Descriptor | Badge family | First consumer | Owner | Schema | Proof | Freshness |\n",
        );
        out.push_str(
            "|------------|--------------|----------------|-------|--------|-------|-----------|\n",
        );
        for d in &self.descriptors {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | {} | `{}` | `{}` | `{}` |\n",
                d.family.as_str(),
                d.badge_family.as_str(),
                d.first_consumer.as_str(),
                d.owner_role,
                d.schema_ref,
                d.proof_packet_ref,
                d.proof_freshness.as_str()
            ));
        }

        out.push_str("\n## Descriptor value vocabularies\n\n");
        for d in &self.descriptors {
            out.push_str(&format!(
                "- `{}`: {}\n",
                d.family.as_str(),
                d.value_tokens
                    .iter()
                    .map(|t| format!("`{t}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        out.push_str("\n## Downgrade rules\n\n");
        out.push_str("| Trigger family | Trigger value | Effect | Floor |\n");
        out.push_str("|----------------|---------------|--------|-------|\n");
        for r in &self.downgrade_rules {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` |\n",
                r.trigger_family.as_str(),
                r.trigger_token,
                r.effect.as_str(),
                r.effective_floor.as_str()
            ));
        }

        out.push_str("\n## Public-truth consumers\n\n");
        out.push_str("| Consumer | Status | Claim → effective | Gate | Binds |\n");
        out.push_str("|----------|--------|-------------------|------|-------|\n");
        for b in &self.consumer_bindings {
            let bound: Vec<&str> = b.bound_families.iter().map(|f| f.as_str()).collect();
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` → `{}` | `{}` | {} |\n",
                b.consumer.as_str(),
                b.status.as_str(),
                b.claimed_qualification.as_str(),
                b.effective_qualification.as_str(),
                b.gate_decision.as_str(),
                bound.join(", ")
            ));
            for gap in &b.gaps {
                out.push_str(&format!(
                    "| | | gap: `{}` on `{}` | | |\n",
                    gap.gap_kind.as_str(),
                    gap.family.as_str()
                ));
            }
        }
        out
    }
}

/// Derives the matrix summary from the descriptors, rules, and consumers.
fn derive_summary(
    descriptors: &[DescriptorContract],
    downgrade_rules: &[DowngradeRule],
    consumers: &[ConsumerBinding],
) -> DescriptorBadgeSummary {
    let descriptor_count = |state: FreshnessState| -> u32 {
        descriptors
            .iter()
            .filter(|d| d.proof_freshness == state)
            .count() as u32
    };
    let blocked = consumers.iter().filter(|c| c.is_blocked()).count() as u32;
    let badge_families: std::collections::BTreeSet<BadgeFamily> =
        descriptors.iter().map(|d| d.badge_family).collect();
    DescriptorBadgeSummary {
        total_descriptors: descriptors.len() as u32,
        total_badge_families: badge_families.len() as u32,
        total_downgrade_rules: downgrade_rules.len() as u32,
        total_consumers: consumers.len() as u32,
        governed_consumer_count: consumers.iter().filter(|c| c.is_governed()).count() as u32,
        narrowed_consumer_count: consumers.iter().filter(|c| c.is_narrowed()).count() as u32,
        blocked_consumer_count: blocked,
        current_descriptor_count: descriptor_count(FreshnessState::Current),
        stale_descriptor_count: descriptor_count(FreshnessState::Stale),
        expired_descriptor_count: descriptor_count(FreshnessState::Expired),
        missing_descriptor_count: descriptor_count(FreshnessState::Missing),
        blocks_stable_promotion: blocked > 0,
    }
}

/// Derives the aggregate release gate from the per-consumer gates.
fn derive_release_gate(consumers: &[ConsumerBinding]) -> DescriptorReleaseGate {
    let pick = |f: &dyn Fn(&ConsumerBinding) -> bool| -> Vec<String> {
        let mut tokens: Vec<String> = consumers
            .iter()
            .filter(|c| f(c))
            .map(|c| c.consumer.as_str().to_owned())
            .collect();
        tokens.sort();
        tokens
    };
    let blocked = pick(&|c| c.is_blocked());
    DescriptorReleaseGate {
        blocks_stable_promotion: !blocked.is_empty(),
        blocked_consumers: blocked,
        narrowed_consumers: pick(&|c| c.is_narrowed()),
        governed_consumers: pick(&|c| c.is_governed()),
        gate_message_id: format!("{}release_gate", M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX),
    }
}

/// Derives the conformance review, so the stored block reflects the actual packet.
fn derive_conformance(
    descriptors: &[DescriptorContract],
    downgrade_rules: &[DowngradeRule],
    consumers: &[ConsumerBinding],
) -> DescriptorConformance {
    let every_family = DescriptorFamily::ALL
        .iter()
        .all(|f| descriptors.iter().any(|d| d.family == *f));

    let every_badge = BadgeFamily::ALL
        .iter()
        .all(|b| descriptors.iter().any(|d| d.badge_family == *b));

    let every_consumer_binds =
        !consumers.is_empty() && consumers.iter().all(|c| !c.bound_families.is_empty());

    // Every consumer maps to current descriptors (governed) or auto-narrows/blocks via a
    // named gap — there is never a consumer with a stale/missing bound descriptor governed.
    let maps_or_narrows = consumers.iter().all(|c| match c.gate_decision {
        DescriptorGate::Governed => c.gaps.is_empty(),
        DescriptorGate::Narrowed | DescriptorGate::Blocked => !c.gaps.is_empty(),
    });

    let freshness_of = |family: DescriptorFamily| -> Option<FreshnessState> {
        descriptors
            .iter()
            .find(|d| d.family == family)
            .map(|d| d.proof_freshness)
    };

    // A stale descriptor proof narrows every consumer that binds it, unless a failing
    // descriptor already blocks that consumer.
    let stale_narrows = consumers.iter().all(|c| {
        let binds_stale = c
            .bound_families
            .iter()
            .any(|f| freshness_of(*f) == Some(FreshnessState::Stale));
        let binds_failing = c.bound_families.iter().any(|f| {
            !matches!(
                freshness_of(*f),
                Some(FreshnessState::Current) | Some(FreshnessState::Stale)
            )
        });
        !binds_stale || binds_failing || c.is_narrowed()
    });

    // An expired/missing/unmapped descriptor blocks every consumer that binds it.
    let missing_blocks = consumers.iter().all(|c| {
        let binds_failing = c.bound_families.iter().any(|f| {
            !matches!(
                freshness_of(*f),
                Some(FreshnessState::Current) | Some(FreshnessState::Stale)
            )
        });
        !binds_failing || c.is_blocked()
    });

    let gaps_named = consumers.iter().all(|c| {
        c.gaps.iter().all(|g| {
            g.cause_message_id
                .starts_with(M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX)
                && g.consumer == c.consumer
        })
    });

    // Mirror/offline/side-loaded/not-provided are first-class tokens in the provenance
    // vocabulary — never collapsed into omission.
    let provenance_tokens: Vec<&str> = DescriptorFamily::Provenance.value_tokens();
    let weaker_origins_present = ["mirror", "offline_bundle", "side_loaded", "not_provided"]
        .iter()
        .all(|needle| provenance_tokens.contains(needle));

    // Every non-authoritative descriptor value has a downgrade rule.
    let mut weaker_values: Vec<(DescriptorFamily, &str)> = Vec::new();
    for class in ProvenanceClass::ALL {
        if !class.is_authoritative() {
            weaker_values.push((DescriptorFamily::Provenance, class.as_str()));
        }
    }
    for state in FreshnessState::ALL {
        if !matches!(state, FreshnessState::Current) {
            weaker_values.push((DescriptorFamily::Freshness, state.as_str()));
        }
    }
    for scope in ClientScope::ALL {
        if !scope.is_full_authority() {
            weaker_values.push((DescriptorFamily::ClientScope, scope.as_str()));
        }
    }
    let downgrade_covers = weaker_values.iter().all(|(family, token)| {
        downgrade_rules
            .iter()
            .any(|r| r.trigger_family == *family && r.trigger_token == *token)
    });

    let generated = consumers.iter().all(|c| {
        let mut probe = c.clone();
        probe.recompute(descriptors);
        probe.gaps == c.gaps
            && probe.status == c.status
            && probe.gate_decision == c.gate_decision
            && probe.effective_qualification == c.effective_qualification
    });

    DescriptorConformance {
        every_family_has_a_descriptor: every_family,
        every_badge_family_maps_to_a_descriptor: every_badge,
        every_consumer_binds_at_least_one_descriptor: every_consumer_binds,
        every_consumer_maps_to_descriptors_or_narrows: maps_or_narrows,
        stale_proof_narrows_deterministically: stale_narrows,
        missing_descriptor_blocks_stable_promotion: missing_blocks,
        exact_gaps_named_per_consumer: gaps_named,
        weaker_origins_never_omitted: weaker_origins_present,
        downgrade_rules_cover_every_weaker_value: downgrade_covers,
        surfaces_consume_one_runtime: true,
        generated_from_checked_in_descriptors: generated,
        export_carries_no_raw_material: true,
    }
}

/// Validation failures for the descriptor/badge lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DescriptorBadgeViolation {
    /// The packet record kind is wrong.
    WrongRecordKind,
    /// The packet schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is empty.
    MissingIdentity,
    /// A descriptor contract cites a field that does not match its family.
    DescriptorContractFieldMismatch,
    /// A badge family does not map back to its descriptor family.
    BadgeFamilyMismatch,
    /// Two descriptor contracts name the same family.
    DuplicateDescriptor,
    /// A descriptor family has no governed contract.
    FamilyNotGoverned,
    /// The downgrade rule set drifted from the canonical rules.
    DowngradeRulesDrift,
    /// The packet declares no claimed consumers.
    PacketHasNoConsumers,
    /// Two consumers share a consumer token.
    DuplicateConsumer,
    /// A claimed consumer binds no descriptors.
    ConsumerBindsNoDescriptors,
    /// A consumer's stored verdict drifted from a fresh recompute.
    ConsumerVerdictDrift,
    /// The matrix summary disagrees with the descriptors/consumers.
    SummaryDrift,
    /// The aggregate release gate disagrees with the per-consumer gates.
    ReleaseGateAggregateMismatch,
    /// A disclosure surface does not consume the runtime.
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

impl M5DescriptorBadgeViolation {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::DescriptorContractFieldMismatch => "descriptor_contract_field_mismatch",
            Self::BadgeFamilyMismatch => "badge_family_mismatch",
            Self::DuplicateDescriptor => "duplicate_descriptor",
            Self::FamilyNotGoverned => "family_not_governed",
            Self::DowngradeRulesDrift => "downgrade_rules_drift",
            Self::PacketHasNoConsumers => "packet_has_no_consumers",
            Self::DuplicateConsumer => "duplicate_consumer",
            Self::ConsumerBindsNoDescriptors => "consumer_binds_no_descriptors",
            Self::ConsumerVerdictDrift => "consumer_verdict_drift",
            Self::SummaryDrift => "summary_drift",
            Self::ReleaseGateAggregateMismatch => "release_gate_aggregate_mismatch",
            Self::DisclosureIncomplete => "disclosure_incomplete",
            Self::VocabularyMismatch => "vocabulary_mismatch",
            Self::ConformanceReviewFailed => "conformance_review_failed",
            Self::UnprefixedMessageId => "unprefixed_message_id",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Keys whose presence would mean an export leaked raw material. Mirrors the redaction
/// posture of the upstream release and support lanes.
const FORBIDDEN_KEY_SUBSTRINGS: [&str; 6] = [
    "credential",
    "secret",
    "password",
    "api_key",
    "raw_payload",
    "bearer_token",
];

/// Scans a serialized packet for forbidden material. Returns true when a key
/// (case-insensitive) contains a forbidden substring.
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
