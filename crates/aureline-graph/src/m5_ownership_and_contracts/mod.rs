//! Canonical M5 ownership-and-contracts packet: the honest answer the M5 graph surfaces give to
//! *who owns this, and what kind of source produced that answer?*
//!
//! Where [`crate::m5_workset_scope`] answers *what slice am I looking at?*,
//! [`crate::m5_topology_identity`] answers *which exact graph object is this?*,
//! [`crate::m5_impact_query`] answers *is this empty impact answer safe?*, and
//! [`crate::m5_graph_governance`] freezes *which depth claim a lane may publish*, this packet
//! answers the question review hints, explainer cards, onboarding context, AI ownership
//! suggestions, and support all ask of an ownership answer: *is this curated truth, policy-derived
//! truth, imported provider metadata, or merely a heuristic guess?* It carries one
//! [`OwnershipDescriptor`] per ownership or contract fact — each with the subject it attaches to,
//! its distinct human [`OwnershipRole`] (owner, reviewer, maintainer, support contact, or
//! change-control link), its [`OwnershipSourceClass`], a freshness and confidence token, a
//! [`OwnershipVisibility`] scope, an optional source reason, and the descriptor ids it supersedes
//! — plus one [`OwnershipConsumerBinding`] per surface that carries the same answer beyond a single
//! panel render.
//!
//! Four invariants hold across the packet:
//!
//! - **Source kind stays visible.** Every non-[`OwnershipSourceClass::Curated`] descriptor carries
//!   an explicit [`OwnershipDescriptor::source_reason`], so an imported or heuristic ownership hint
//!   never reads as curated first-party truth.
//! - **Inference never overwrites curated truth.** A heuristic or imported descriptor may not
//!   supersede a curated or policy-derived descriptor merely because it is newer or easier to
//!   compute; doing so fails validation.
//! - **Roles stay distinct.** Owner, reviewer, maintainer, support contact, and change-control
//!   links are separate [`OwnershipRole`]s rather than one generic owner field, and a change-control
//!   link carries its url distinctly.
//! - **Exports never leak restricted metadata.** Every binding declares the visibility ceiling it
//!   may carry, the support-export binding carries every export-safe descriptor and no private one,
//!   and the export projection redacts private descriptors entirely, so private or policy-scoped
//!   ownership never widens past its declared scope.
//!
//! The packet reuses the stable topology identity space ([`TopologyNodeKind`]) and the active scope
//! snapshot ([`TopologyScopeAnchor`]) rather than minting one-off ownership strings per surface,
//! binds upstream to the canonical graph-depth governance matrix, the workset-scope packet, and the
//! topology-identity packet it extends, and stamps every consumer binding with the active scope
//! snapshot so replay can reconstruct the exact slice the user queried.
//!
//! The packet is checked in at `artifacts/graph/m5/m5-ownership-and-contracts.json` and embedded
//! here. It is metadata-only: every field is a typed state, a count, a label, or an opaque ref, and
//! it carries no credential bodies, raw provider payloads, or graph node contents.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_topology_identity::{TopologyNodeKind, TopologyScopeAnchor};

/// Supported M5 ownership-and-contracts packet schema version.
pub const M5_OWNERSHIP_CONTRACTS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_OWNERSHIP_CONTRACTS_RECORD_KIND: &str = "m5_ownership_and_contracts_packet";

/// Repo-relative path to the checked-in packet.
pub const M5_OWNERSHIP_CONTRACTS_PATH: &str = "artifacts/graph/m5/m5-ownership-and-contracts.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_OWNERSHIP_CONTRACTS_SCHEMA_REF: &str =
    "schemas/graph/m5-ownership-and-contracts.schema.json";

/// Repo-relative path to the companion document.
pub const M5_OWNERSHIP_CONTRACTS_DOC_REF: &str = "docs/graph/m5/m5-ownership-and-contracts.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_OWNERSHIP_CONTRACTS_FIXTURE_DIR: &str = "fixtures/graph/m5/m5-ownership-and-contracts";

/// Repo-relative path to the canonical graph-depth governance matrix this packet extends.
pub const M5_OWNERSHIP_CONTRACTS_GOVERNANCE_MATRIX_REF: &str =
    "artifacts/graph/m5/m5-graph-governance.json";

/// Repo-relative path to the canonical workset-scope packet this packet is bound to.
pub const M5_OWNERSHIP_CONTRACTS_SCOPE_PACKET_REF: &str =
    "artifacts/graph/m5/m5-workset-scope.json";

/// Repo-relative path to the canonical topology-identity packet whose id space this packet reuses.
pub const M5_OWNERSHIP_CONTRACTS_TOPOLOGY_PACKET_REF: &str =
    "artifacts/graph/m5/m5-topology-identity.json";

/// Embedded checked-in packet JSON.
pub const M5_OWNERSHIP_CONTRACTS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/graph/m5/m5-ownership-and-contracts.json"
));

/// What kind of source produced an ownership or contract fact, keeping curated first-party truth
/// distinct from policy-derived, imported, and heuristic answers.
///
/// The ordering is a precedence: [`OwnershipSourceClass::Curated`] is the most authoritative and
/// [`OwnershipSourceClass::Heuristic`] the least, so a derived hint never silently outranks curated
/// truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnershipSourceClass {
    /// Curated first-party metadata supplied by a maintainer annotation.
    Curated,
    /// Ownership derived from a policy such as a CODEOWNERS rule.
    PolicyDerived,
    /// Metadata imported from a connected provider rather than indexed locally.
    Imported,
    /// Ownership inferred by a heuristic or AI producer; a hint, not declared truth.
    Heuristic,
}

impl OwnershipSourceClass {
    /// Every source class, in declaration (precedence) order.
    pub const ALL: [Self; 4] = [
        Self::Curated,
        Self::PolicyDerived,
        Self::Imported,
        Self::Heuristic,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Curated => "curated",
            Self::PolicyDerived => "policy_derived",
            Self::Imported => "imported",
            Self::Heuristic => "heuristic",
        }
    }

    /// Precedence rank; lower is more authoritative.
    pub const fn precedence_rank(self) -> u8 {
        match self {
            Self::Curated => 0,
            Self::PolicyDerived => 1,
            Self::Imported => 2,
            Self::Heuristic => 3,
        }
    }

    /// Whether this class is authoritative truth (curated or policy-derived) rather than a derived
    /// hint.
    pub const fn is_authoritative(self) -> bool {
        matches!(self, Self::Curated | Self::PolicyDerived)
    }

    /// Whether this class is an imported or heuristic answer that must never overwrite curated or
    /// policy-backed truth.
    pub const fn is_inferred_or_imported(self) -> bool {
        matches!(self, Self::Imported | Self::Heuristic)
    }

    /// Whether this class must carry an explicit source reason; only curated truth is exempt.
    pub const fn requires_source_reason(self) -> bool {
        !matches!(self, Self::Curated)
    }

    /// Whether this class outranks another (is strictly more authoritative).
    pub const fn outranks(self, other: Self) -> bool {
        self.precedence_rank() < other.precedence_rank()
    }
}

/// The distinct human role an ownership descriptor names, kept separate so the surfaces never
/// collapse owner, reviewer, maintainer, support contact, and change-control links into one generic
/// owner field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnershipRole {
    /// The accountable owner of the subject.
    Owner,
    /// A designated reviewer for changes to the subject.
    Reviewer,
    /// A maintainer responsible for ongoing upkeep.
    Maintainer,
    /// A support contact for questions about the subject.
    SupportContact,
    /// A change-control link such as a CODEOWNERS rule or change policy.
    ChangeControl,
}

impl OwnershipRole {
    /// Every role, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Owner,
        Self::Reviewer,
        Self::Maintainer,
        Self::SupportContact,
        Self::ChangeControl,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Owner => "owner",
            Self::Reviewer => "reviewer",
            Self::Maintainer => "maintainer",
            Self::SupportContact => "support_contact",
            Self::ChangeControl => "change_control",
        }
    }

    /// Whether this role is a change-control link that carries a url distinctly.
    pub const fn is_change_control(self) -> bool {
        matches!(self, Self::ChangeControl)
    }
}

/// The visibility scope a descriptor may be shown within, keeping private or policy-scoped ownership
/// from widening into exports or public-facing surfaces beyond its declared scope.
///
/// The ordering is a restrictiveness: [`OwnershipVisibility::Public`] is the least restricted and
/// [`OwnershipVisibility::Private`] the most.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnershipVisibility {
    /// Shown anywhere, including public issue reports.
    Public,
    /// Shown in-product and in enterprise/support exports, but not public-facing surfaces.
    Internal,
    /// Restricted; shown only in-product to authorized users and never exported.
    Private,
}

impl OwnershipVisibility {
    /// Every visibility, in declaration (restrictiveness) order.
    pub const ALL: [Self; 3] = [Self::Public, Self::Internal, Self::Private];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Internal => "internal",
            Self::Private => "private",
        }
    }

    /// Restrictiveness rank; higher is more restricted.
    pub const fn restrictiveness_rank(self) -> u8 {
        match self {
            Self::Public => 0,
            Self::Internal => 1,
            Self::Private => 2,
        }
    }

    /// Whether a descriptor at this visibility may appear in an export (support or enterprise);
    /// private descriptors never may.
    pub const fn is_export_safe(self) -> bool {
        matches!(self, Self::Public | Self::Internal)
    }

    /// Whether a descriptor at this visibility may appear on a public-facing surface.
    pub const fn is_public_safe(self) -> bool {
        matches!(self, Self::Public)
    }

    /// Whether a descriptor at this visibility fits within a binding capped at `ceiling`.
    pub const fn fits_within(self, ceiling: Self) -> bool {
        self.restrictiveness_rank() <= ceiling.restrictiveness_rank()
    }
}

/// A surface that carries an ownership answer beyond the panel that first rendered it.
///
/// The closed vocabulary is exhaustive: every M5 surface that shows ownership plus the durable
/// support-export bundle, so an ownership answer is never trapped in one render.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnershipConsumerSurface {
    /// The review-explanation ownership hint.
    ReviewHint,
    /// The architecture explainer card.
    ExplainerCard,
    /// Onboarding context for a new contributor.
    OnboardingContext,
    /// An AI ownership suggestion.
    AiOwnershipSuggestion,
    /// The support/export bundle.
    SupportExport,
}

impl OwnershipConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ReviewHint,
        Self::ExplainerCard,
        Self::OnboardingContext,
        Self::AiOwnershipSuggestion,
        Self::SupportExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewHint => "review_hint",
            Self::ExplainerCard => "explainer_card",
            Self::OnboardingContext => "onboarding_context",
            Self::AiOwnershipSuggestion => "ai_ownership_suggestion",
            Self::SupportExport => "support_export",
        }
    }

    /// Whether this is the durable support-export surface that must carry every export-safe answer.
    pub const fn is_support_export(self) -> bool {
        matches!(self, Self::SupportExport)
    }
}

/// One ownership or contract fact: who fills a role for a subject, and what kind of source said so.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OwnershipDescriptor {
    /// Stable descriptor id inside the packet.
    pub descriptor_id: String,
    /// Canonical, stable topology node id the ownership attaches to.
    pub subject_id: String,
    /// Node kind of the subject.
    pub subject_kind: TopologyNodeKind,
    /// The distinct human role this descriptor names.
    pub role: OwnershipRole,
    /// What kind of source produced this ownership fact.
    pub source_class: OwnershipSourceClass,
    /// Explicit source reason; required for every non-curated source class.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_reason: Option<String>,
    /// Redaction-aware display label for the party (a team handle, contact, or rule label).
    pub party_label: String,
    /// Visibility scope this descriptor may be shown within.
    pub visibility: OwnershipVisibility,
    /// Freshness token copied from the graph object.
    pub freshness: String,
    /// Confidence token copied from the graph object.
    pub confidence: String,
    /// Change-control link url; required for the change-control role and forbidden otherwise.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_control_url: Option<String>,
    /// Descriptor ids this descriptor supersedes; an inferred descriptor may not supersede curated
    /// or policy-derived truth.
    #[serde(default)]
    pub supersedes: Vec<String>,
    /// Export-safe, copy-safe permalink that embeds the canonical descriptor id.
    pub export_permalink: String,
}

impl OwnershipDescriptor {
    /// Whether a non-curated descriptor carries an explicit source reason.
    pub fn source_is_labeled(&self) -> bool {
        if !self.source_class.requires_source_reason() {
            return true;
        }
        self.source_reason
            .as_ref()
            .is_some_and(|reason| !reason.trim().is_empty())
    }

    /// Whether the change-control link is shaped correctly: present for the change-control role and
    /// absent otherwise.
    pub fn change_control_link_is_well_formed(&self) -> bool {
        let has_link = self
            .change_control_url
            .as_ref()
            .is_some_and(|url| !url.trim().is_empty());
        if self.role.is_change_control() {
            has_link
        } else {
            self.change_control_url.is_none()
        }
    }

    /// Whether the permalink is non-empty and embeds the canonical descriptor id.
    pub fn permalink_is_export_safe(&self) -> bool {
        !self.export_permalink.trim().is_empty()
            && self.export_permalink.contains(&self.descriptor_id)
    }

    /// Whether this descriptor may appear in an export.
    pub const fn is_export_safe(&self) -> bool {
        self.visibility.is_export_safe()
    }
}

/// One surface bound to the active scope snapshot, carrying a set of ownership descriptors forward.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OwnershipConsumerBinding {
    /// Stable binding id inside the packet.
    pub binding_id: String,
    /// Surface this binding carries descriptors into.
    pub surface: OwnershipConsumerSurface,
    /// Snapshot id this surface is bound to; must equal the active snapshot id.
    pub snapshot_id: String,
    /// Scope id this surface renders; must equal the active scope id.
    pub scope_id: String,
    /// Visibility ceiling this surface may carry; a carried descriptor may not exceed it.
    pub max_visibility: OwnershipVisibility,
    /// Whether this surface preserves the descriptors' source-class labels rather than flattening
    /// them; must be true so an inferred hint never reads as curated truth downstream.
    pub preserves_source_labels: bool,
    /// Canonical descriptor ids this surface carries; every id must be declared in the packet.
    #[serde(default)]
    pub carries_descriptor_ids: Vec<String>,
    /// Ref to the surface artifact that ingests these descriptors.
    pub consumer_ref: String,
    /// Reviewer-facing note.
    pub note: String,
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5OwnershipContractSummary {
    /// Total declared descriptors.
    pub descriptor_count: usize,
    /// Total consumer bindings.
    pub consumer_binding_count: usize,
    /// Number of distinct surfaces bound.
    pub surface_count: usize,
    /// Descriptors sourced as `curated`.
    pub curated_count: usize,
    /// Descriptors sourced as `policy_derived`.
    pub policy_derived_count: usize,
    /// Descriptors sourced as `imported`.
    pub imported_count: usize,
    /// Descriptors sourced as `heuristic`.
    pub heuristic_count: usize,
    /// Descriptors with the `owner` role.
    pub owner_count: usize,
    /// Descriptors with the `reviewer` role.
    pub reviewer_count: usize,
    /// Descriptors with the `maintainer` role.
    pub maintainer_count: usize,
    /// Descriptors with the `support_contact` role.
    pub support_contact_count: usize,
    /// Descriptors with the `change_control` role.
    pub change_control_count: usize,
    /// Descriptors at `public` visibility.
    pub public_count: usize,
    /// Descriptors at `internal` visibility.
    pub internal_count: usize,
    /// Descriptors at `private` visibility.
    pub private_count: usize,
    /// Descriptors that are export-safe (public or internal).
    pub export_safe_descriptor_count: usize,
    /// Descriptors carrying an explicit source reason.
    pub descriptors_with_source_reason: usize,
    /// Total supersedes links across every descriptor.
    pub supersedes_link_count: usize,
}

/// A redaction-safe export row projected from one ownership descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OwnershipContractExportRow {
    /// Canonical descriptor id.
    pub descriptor_id: String,
    /// Canonical subject id.
    pub subject_id: String,
    /// Role token.
    pub role: String,
    /// Source-class token.
    pub source_class: String,
    /// Visibility token.
    pub visibility: String,
    /// Redaction-aware party label.
    pub party_label: String,
    /// Change-control link url, if any.
    pub change_control_url: Option<String>,
    /// Freshness token.
    pub freshness: String,
    /// Confidence token.
    pub confidence: String,
    /// Explicit source reason, if any.
    pub source_reason: Option<String>,
    /// Export-safe permalink that points at the exact descriptor.
    pub permalink: String,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet — the ownership index downstream surfaces
/// render instead of re-describing ownership by hand. Private descriptors are redacted entirely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OwnershipContractExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Active snapshot id every binding is stamped with.
    pub snapshot_id: String,
    /// Active scope id.
    pub scope_id: String,
    /// Active scope-mode token.
    pub scope_mode: String,
    /// Projected export-safe descriptor rows.
    pub descriptors: Vec<M5OwnershipContractExportRow>,
    /// Count of private descriptors withheld from the export.
    pub redacted_private_count: usize,
    /// Whether no inferred or imported descriptor overwrites curated or policy-backed truth.
    pub curated_truth_preserved: bool,
    /// Whether every non-curated descriptor carries an explicit source reason.
    pub all_inferred_descriptors_labeled: bool,
    /// Whether every binding preserves the descriptors' source-class labels.
    pub source_labels_preserved_everywhere: bool,
    /// Whether every export-safe descriptor is carried by the support-export binding.
    pub every_export_safe_descriptor_in_support_export: bool,
    /// Whether the support-export binding carries no private descriptor.
    pub no_private_in_support_export: bool,
}

/// The typed M5 ownership-and-contracts packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5OwnershipContractPacket {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the canonical graph-depth governance matrix this packet extends.
    pub governance_matrix_ref: String,
    /// Ref to the canonical workset-scope packet this packet is bound to.
    pub scope_packet_ref: String,
    /// Ref to the canonical topology-identity packet whose id space this packet reuses.
    pub topology_packet_ref: String,
    /// Ref to the graph-conformance suite backing the packet.
    pub conformance_ref: String,
    /// Ref binding this packet into the release-evidence surface.
    pub release_evidence_ref: String,
    /// Ref binding this packet into the help/service-health surface.
    pub help_surface_ref: String,
    /// Ref binding this packet into the docs-badge surface.
    pub docs_badge_ref: String,
    /// Ref binding this packet into the support-export surface.
    pub support_export_ref: String,
    /// Closed source-class vocabulary.
    pub source_classes: Vec<OwnershipSourceClass>,
    /// Closed role vocabulary.
    pub roles: Vec<OwnershipRole>,
    /// Closed visibility vocabulary.
    pub visibilities: Vec<OwnershipVisibility>,
    /// Closed consumer-surface vocabulary.
    pub consumer_surfaces: Vec<OwnershipConsumerSurface>,
    /// The active scope snapshot every binding is stamped with.
    pub active_scope: TopologyScopeAnchor,
    /// Declared ownership descriptors.
    #[serde(default)]
    pub descriptors: Vec<OwnershipDescriptor>,
    /// Consumer bindings, one per surface.
    #[serde(default)]
    pub consumer_bindings: Vec<OwnershipConsumerBinding>,
    /// Summary counts.
    pub summary: M5OwnershipContractSummary,
}

impl M5OwnershipContractPacket {
    /// Returns the descriptor for a descriptor id.
    pub fn descriptor(&self, descriptor_id: &str) -> Option<&OwnershipDescriptor> {
        self.descriptors
            .iter()
            .find(|d| d.descriptor_id == descriptor_id)
    }

    /// Returns the binding for a surface.
    pub fn consumer_binding(
        &self,
        surface: OwnershipConsumerSurface,
    ) -> Option<&OwnershipConsumerBinding> {
        self.consumer_bindings.iter().find(|b| b.surface == surface)
    }

    /// Returns the export-safe permalink for a descriptor id.
    pub fn permalink_for_descriptor(&self, descriptor_id: &str) -> Option<&str> {
        self.descriptor(descriptor_id)
            .map(|d| d.export_permalink.as_str())
    }

    /// Returns the most authoritative declared descriptor for a subject and role.
    ///
    /// Authority follows [`OwnershipSourceClass`] precedence, so curated truth wins over a
    /// policy-derived, imported, or heuristic answer for the same subject and role.
    pub fn authoritative_descriptor(
        &self,
        subject_id: &str,
        role: OwnershipRole,
    ) -> Option<&OwnershipDescriptor> {
        self.descriptors
            .iter()
            .filter(|d| d.subject_id == subject_id && d.role == role)
            .min_by_key(|d| d.source_class.precedence_rank())
    }

    /// Whether every non-curated descriptor carries an explicit source reason.
    pub fn all_inferred_descriptors_labeled(&self) -> bool {
        self.descriptors
            .iter()
            .all(OwnershipDescriptor::source_is_labeled)
    }

    /// Whether no inferred or imported descriptor supersedes curated or policy-backed truth.
    ///
    /// This is the headline guardrail: a heuristic or imported ownership answer may not overwrite
    /// curated or policy-derived truth merely because it is newer or easier to compute.
    pub fn inference_never_overwrites_curated(&self) -> bool {
        self.descriptors.iter().all(|descriptor| {
            !descriptor.source_class.is_inferred_or_imported()
                || descriptor.supersedes.iter().all(|superseded_id| {
                    match self.descriptor(superseded_id) {
                        Some(target) => !target.source_class.is_authoritative(),
                        None => true,
                    }
                })
        })
    }

    /// Whether every binding preserves the descriptors' source-class labels.
    pub fn source_labels_preserved_everywhere(&self) -> bool {
        self.consumer_bindings
            .iter()
            .all(|b| b.preserves_source_labels)
    }

    /// Whether every export-safe descriptor is carried by the support-export binding.
    pub fn every_export_safe_descriptor_in_support_export(&self) -> bool {
        let Some(binding) = self.consumer_binding(OwnershipConsumerSurface::SupportExport) else {
            return self.descriptors.iter().all(|d| !d.is_export_safe());
        };
        let carried: BTreeSet<&str> = binding
            .carries_descriptor_ids
            .iter()
            .map(String::as_str)
            .collect();
        self.descriptors
            .iter()
            .filter(|d| d.is_export_safe())
            .all(|d| carried.contains(d.descriptor_id.as_str()))
    }

    /// Whether the support-export binding carries no private descriptor.
    pub fn no_private_in_support_export(&self) -> bool {
        let Some(binding) = self.consumer_binding(OwnershipConsumerSurface::SupportExport) else {
            return true;
        };
        binding
            .carries_descriptor_ids
            .iter()
            .all(|id| match self.descriptor(id) {
                Some(d) => d.visibility.is_export_safe(),
                None => true,
            })
    }

    /// Recomputes the summary block from the descriptors and bindings.
    pub fn computed_summary(&self) -> M5OwnershipContractSummary {
        let source_count = |class: OwnershipSourceClass| {
            self.descriptors
                .iter()
                .filter(|d| d.source_class == class)
                .count()
        };
        let role_count =
            |role: OwnershipRole| self.descriptors.iter().filter(|d| d.role == role).count();
        let visibility_count = |visibility: OwnershipVisibility| {
            self.descriptors
                .iter()
                .filter(|d| d.visibility == visibility)
                .count()
        };
        let distinct_surfaces: BTreeSet<OwnershipConsumerSurface> =
            self.consumer_bindings.iter().map(|b| b.surface).collect();
        M5OwnershipContractSummary {
            descriptor_count: self.descriptors.len(),
            consumer_binding_count: self.consumer_bindings.len(),
            surface_count: distinct_surfaces.len(),
            curated_count: source_count(OwnershipSourceClass::Curated),
            policy_derived_count: source_count(OwnershipSourceClass::PolicyDerived),
            imported_count: source_count(OwnershipSourceClass::Imported),
            heuristic_count: source_count(OwnershipSourceClass::Heuristic),
            owner_count: role_count(OwnershipRole::Owner),
            reviewer_count: role_count(OwnershipRole::Reviewer),
            maintainer_count: role_count(OwnershipRole::Maintainer),
            support_contact_count: role_count(OwnershipRole::SupportContact),
            change_control_count: role_count(OwnershipRole::ChangeControl),
            public_count: visibility_count(OwnershipVisibility::Public),
            internal_count: visibility_count(OwnershipVisibility::Internal),
            private_count: visibility_count(OwnershipVisibility::Private),
            export_safe_descriptor_count: self
                .descriptors
                .iter()
                .filter(|d| d.is_export_safe())
                .count(),
            descriptors_with_source_reason: self
                .descriptors
                .iter()
                .filter(|d| {
                    d.source_reason
                        .as_ref()
                        .is_some_and(|r| !r.trim().is_empty())
                })
                .count(),
            supersedes_link_count: self.descriptors.iter().map(|d| d.supersedes.len()).sum(),
        }
    }

    /// Produces the ownership index downstream surfaces — release evidence, help/service-health,
    /// docs badges, review hints, explainer cards, onboarding context, AI ownership suggestions,
    /// and support exports — render instead of re-describing ownership by hand. Private descriptors
    /// are redacted entirely.
    pub fn export_projection(&self) -> M5OwnershipContractExportProjection {
        let descriptors = self
            .descriptors
            .iter()
            .filter(|d| d.is_export_safe())
            .map(|d| M5OwnershipContractExportRow {
                descriptor_id: d.descriptor_id.clone(),
                subject_id: d.subject_id.clone(),
                role: d.role.as_str().to_owned(),
                source_class: d.source_class.as_str().to_owned(),
                visibility: d.visibility.as_str().to_owned(),
                party_label: d.party_label.clone(),
                change_control_url: d.change_control_url.clone(),
                freshness: d.freshness.clone(),
                confidence: d.confidence.clone(),
                source_reason: d.source_reason.clone(),
                permalink: d.export_permalink.clone(),
                summary: format!(
                    "{} {} for {} ({}, {}): {} [{}/{}]",
                    d.source_class.as_str(),
                    d.role.as_str(),
                    d.subject_id,
                    d.visibility.as_str(),
                    d.party_label,
                    d.descriptor_id,
                    d.freshness,
                    d.confidence
                ),
            })
            .collect();
        M5OwnershipContractExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            snapshot_id: self.active_scope.snapshot_id.clone(),
            scope_id: self.active_scope.scope_id.clone(),
            scope_mode: self.active_scope.scope_mode.as_str().to_owned(),
            descriptors,
            redacted_private_count: self
                .descriptors
                .iter()
                .filter(|d| !d.is_export_safe())
                .count(),
            curated_truth_preserved: self.inference_never_overwrites_curated(),
            all_inferred_descriptors_labeled: self.all_inferred_descriptors_labeled(),
            source_labels_preserved_everywhere: self.source_labels_preserved_everywhere(),
            every_export_safe_descriptor_in_support_export: self
                .every_export_safe_descriptor_in_support_export(),
            no_private_in_support_export: self.no_private_in_support_export(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5OwnershipContractViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_anchor(&mut violations);
        self.validate_descriptors(&mut violations);
        self.validate_bindings(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(M5OwnershipContractViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5OwnershipContractViolation>) {
        if self.schema_version != M5_OWNERSHIP_CONTRACTS_SCHEMA_VERSION {
            violations.push(M5OwnershipContractViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_OWNERSHIP_CONTRACTS_RECORD_KIND {
            violations.push(M5OwnershipContractViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("governance_matrix_ref", &self.governance_matrix_ref),
            ("scope_packet_ref", &self.scope_packet_ref),
            ("topology_packet_ref", &self.topology_packet_ref),
            ("conformance_ref", &self.conformance_ref),
            ("release_evidence_ref", &self.release_evidence_ref),
            ("help_surface_ref", &self.help_surface_ref),
            ("docs_badge_ref", &self.docs_badge_ref),
            ("support_export_ref", &self.support_export_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5OwnershipContractViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        // The packet must bind upstream to the canonical governance matrix, workset-scope packet,
        // and topology-identity packet it extends, so ownership has one provenance root.
        if self.governance_matrix_ref != M5_OWNERSHIP_CONTRACTS_GOVERNANCE_MATRIX_REF {
            violations.push(M5OwnershipContractViolation::GovernanceMatrixRefMismatch);
        }
        if self.scope_packet_ref != M5_OWNERSHIP_CONTRACTS_SCOPE_PACKET_REF {
            violations.push(M5OwnershipContractViolation::ScopePacketRefMismatch);
        }
        if self.topology_packet_ref != M5_OWNERSHIP_CONTRACTS_TOPOLOGY_PACKET_REF {
            violations.push(M5OwnershipContractViolation::TopologyPacketRefMismatch);
        }
        for (field, ok) in [
            (
                "source_classes",
                self.source_classes == OwnershipSourceClass::ALL.to_vec(),
            ),
            ("roles", self.roles == OwnershipRole::ALL.to_vec()),
            (
                "visibilities",
                self.visibilities == OwnershipVisibility::ALL.to_vec(),
            ),
            (
                "consumer_surfaces",
                self.consumer_surfaces == OwnershipConsumerSurface::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5OwnershipContractViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_anchor(&self, violations: &mut Vec<M5OwnershipContractViolation>) {
        for (field, value) in [
            ("snapshot_id", &self.active_scope.snapshot_id),
            ("scope_id", &self.active_scope.scope_id),
            ("taken_as_of", &self.active_scope.taken_as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5OwnershipContractViolation::EmptyField {
                    id: "<active_scope>".to_owned(),
                    field_name: field,
                });
            }
        }
    }

    fn validate_descriptors(&self, violations: &mut Vec<M5OwnershipContractViolation>) {
        let declared: BTreeSet<&str> = self
            .descriptors
            .iter()
            .map(|d| d.descriptor_id.as_str())
            .collect();

        let mut seen_ids = BTreeSet::new();
        for descriptor in &self.descriptors {
            if !seen_ids.insert(descriptor.descriptor_id.clone()) {
                violations.push(M5OwnershipContractViolation::DuplicateDescriptorId {
                    descriptor_id: descriptor.descriptor_id.clone(),
                });
            }
            for (field, value) in [
                ("descriptor_id", &descriptor.descriptor_id),
                ("subject_id", &descriptor.subject_id),
                ("party_label", &descriptor.party_label),
                ("freshness", &descriptor.freshness),
                ("confidence", &descriptor.confidence),
                ("export_permalink", &descriptor.export_permalink),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5OwnershipContractViolation::EmptyField {
                        id: descriptor.descriptor_id.clone(),
                        field_name: field,
                    });
                }
            }
            // A non-curated descriptor must say where its answer came from, so an imported or
            // heuristic hint never reads as curated first-party truth.
            if !descriptor.source_is_labeled() {
                violations.push(M5OwnershipContractViolation::MissingSourceReason {
                    descriptor_id: descriptor.descriptor_id.clone(),
                    source_class: descriptor.source_class.as_str(),
                });
            }
            // Change-control links are kept distinct: present for the change-control role and
            // absent otherwise.
            if !descriptor.change_control_link_is_well_formed() {
                if descriptor.role.is_change_control() {
                    violations.push(M5OwnershipContractViolation::ChangeControlWithoutLink {
                        descriptor_id: descriptor.descriptor_id.clone(),
                    });
                } else {
                    violations.push(M5OwnershipContractViolation::NonChangeControlWithLink {
                        descriptor_id: descriptor.descriptor_id.clone(),
                        role: descriptor.role.as_str(),
                    });
                }
            }
            if !descriptor.permalink_is_export_safe() {
                violations.push(M5OwnershipContractViolation::UnsafeDescriptorPermalink {
                    descriptor_id: descriptor.descriptor_id.clone(),
                });
            }
            self.validate_supersedes(descriptor, &declared, violations);
        }
    }

    fn validate_supersedes(
        &self,
        descriptor: &OwnershipDescriptor,
        declared: &BTreeSet<&str>,
        violations: &mut Vec<M5OwnershipContractViolation>,
    ) {
        for superseded_id in &descriptor.supersedes {
            if superseded_id == &descriptor.descriptor_id {
                violations.push(M5OwnershipContractViolation::SelfSupersede {
                    descriptor_id: descriptor.descriptor_id.clone(),
                });
                continue;
            }
            if !declared.contains(superseded_id.as_str()) {
                violations.push(M5OwnershipContractViolation::UnresolvedSupersedesRef {
                    descriptor_id: descriptor.descriptor_id.clone(),
                    superseded_id: superseded_id.clone(),
                });
                continue;
            }
            // The headline guardrail: an inferred or imported descriptor may not overwrite curated
            // or policy-backed truth merely because it is newer or easier to compute.
            if descriptor.source_class.is_inferred_or_imported() {
                if let Some(target) = self.descriptor(superseded_id) {
                    if target.source_class.is_authoritative() {
                        violations.push(M5OwnershipContractViolation::InferenceOverwritesCurated {
                            descriptor_id: descriptor.descriptor_id.clone(),
                            superseded_id: superseded_id.clone(),
                            source_class: descriptor.source_class.as_str(),
                            superseded_source_class: target.source_class.as_str(),
                        });
                    }
                }
            }
        }
    }

    fn validate_bindings(&self, violations: &mut Vec<M5OwnershipContractViolation>) {
        let snapshot_id = &self.active_scope.snapshot_id;
        let scope_id = &self.active_scope.scope_id;

        let mut seen_ids = BTreeSet::new();
        let mut seen_surfaces = BTreeSet::new();
        for binding in &self.consumer_bindings {
            if !seen_ids.insert(binding.binding_id.clone()) {
                violations.push(M5OwnershipContractViolation::DuplicateBindingId {
                    binding_id: binding.binding_id.clone(),
                });
            }
            if !seen_surfaces.insert(binding.surface) {
                violations.push(M5OwnershipContractViolation::DuplicateSurfaceBinding {
                    surface: binding.surface.as_str(),
                });
            }
            for (field, value) in [
                ("binding_id", &binding.binding_id),
                ("snapshot_id", &binding.snapshot_id),
                ("scope_id", &binding.scope_id),
                ("consumer_ref", &binding.consumer_ref),
                ("note", &binding.note),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5OwnershipContractViolation::EmptyField {
                        id: binding.binding_id.clone(),
                        field_name: field,
                    });
                }
            }
            // Every surface must preserve source-class labels so an inferred hint never reads as
            // curated truth once it leaves the originating panel.
            if !binding.preserves_source_labels {
                violations.push(M5OwnershipContractViolation::SourceLabelsNotPreserved {
                    binding_id: binding.binding_id.clone(),
                });
            }
            // Every binding must be stamped with the active snapshot and scope so support export
            // and replay can reconstruct the slice the user queried.
            if &binding.snapshot_id != snapshot_id {
                violations.push(M5OwnershipContractViolation::SnapshotBindingMismatch {
                    binding_id: binding.binding_id.clone(),
                });
            }
            if &binding.scope_id != scope_id {
                violations.push(M5OwnershipContractViolation::ScopeIdMismatch {
                    binding_id: binding.binding_id.clone(),
                });
            }
            for descriptor_id in &binding.carries_descriptor_ids {
                let Some(descriptor) = self.descriptor(descriptor_id) else {
                    violations.push(M5OwnershipContractViolation::UnresolvedDescriptorRef {
                        binding_id: binding.binding_id.clone(),
                        descriptor_id: descriptor_id.clone(),
                    });
                    continue;
                };
                // A binding may not carry a descriptor more restricted than its declared ceiling,
                // so private or policy-scoped ownership never widens past its declared scope.
                if !descriptor.visibility.fits_within(binding.max_visibility) {
                    violations.push(M5OwnershipContractViolation::VisibilityExceedsBinding {
                        binding_id: binding.binding_id.clone(),
                        descriptor_id: descriptor_id.clone(),
                        visibility: descriptor.visibility.as_str(),
                        max_visibility: binding.max_visibility.as_str(),
                    });
                }
            }
        }

        // Every surface must carry a binding so no consumer leaves its handoff implicit.
        for surface in OwnershipConsumerSurface::ALL {
            if !seen_surfaces.contains(&surface) {
                violations.push(M5OwnershipContractViolation::MissingSurfaceBinding {
                    surface: surface.as_str(),
                });
            }
        }

        self.validate_support_export(violations);
    }

    fn validate_support_export(&self, violations: &mut Vec<M5OwnershipContractViolation>) {
        let Some(binding) = self.consumer_binding(OwnershipConsumerSurface::SupportExport) else {
            return;
        };
        let carried: BTreeSet<&str> = binding
            .carries_descriptor_ids
            .iter()
            .map(String::as_str)
            .collect();
        // Guardrail: every export-safe descriptor must be carried by the durable support-export
        // surface, so support and enterprise review can cite it without a private dashboard lookup.
        for descriptor in &self.descriptors {
            if descriptor.is_export_safe() && !carried.contains(descriptor.descriptor_id.as_str()) {
                violations.push(
                    M5OwnershipContractViolation::ExportSafeDescriptorMissingFromSupportExport {
                        descriptor_id: descriptor.descriptor_id.clone(),
                    },
                );
            }
            // Out-of-scope guardrail: a private descriptor must never widen into the export.
            if !descriptor.is_export_safe() && carried.contains(descriptor.descriptor_id.as_str()) {
                violations.push(
                    M5OwnershipContractViolation::PrivateDescriptorInSupportExport {
                        descriptor_id: descriptor.descriptor_id.clone(),
                    },
                );
            }
        }
    }
}

/// A validation violation for the M5 ownership-and-contracts packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5OwnershipContractViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Row or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// The packet does not bind to the canonical governance matrix.
    GovernanceMatrixRefMismatch,
    /// The packet does not bind to the canonical workset-scope packet.
    ScopePacketRefMismatch,
    /// The packet does not bind to the canonical topology-identity packet.
    TopologyPacketRefMismatch,
    /// A descriptor id appears more than once.
    DuplicateDescriptorId {
        /// Duplicate descriptor id.
        descriptor_id: String,
    },
    /// A non-curated descriptor carries no explicit source reason.
    MissingSourceReason {
        /// Descriptor id.
        descriptor_id: String,
        /// Source-class token.
        source_class: &'static str,
    },
    /// A change-control descriptor carries no link url.
    ChangeControlWithoutLink {
        /// Descriptor id.
        descriptor_id: String,
    },
    /// A non-change-control descriptor carries a change-control link url.
    NonChangeControlWithLink {
        /// Descriptor id.
        descriptor_id: String,
        /// Role token.
        role: &'static str,
    },
    /// A descriptor carries a permalink that is empty or does not embed its id.
    UnsafeDescriptorPermalink {
        /// Descriptor id.
        descriptor_id: String,
    },
    /// A descriptor supersedes itself.
    SelfSupersede {
        /// Descriptor id.
        descriptor_id: String,
    },
    /// A descriptor supersedes an id not declared in the packet.
    UnresolvedSupersedesRef {
        /// Descriptor id.
        descriptor_id: String,
        /// Unresolved superseded id.
        superseded_id: String,
    },
    /// An inferred or imported descriptor supersedes curated or policy-backed truth.
    InferenceOverwritesCurated {
        /// Inferred or imported descriptor id.
        descriptor_id: String,
        /// Superseded authoritative descriptor id.
        superseded_id: String,
        /// Inferred descriptor's source class.
        source_class: &'static str,
        /// Superseded descriptor's source class.
        superseded_source_class: &'static str,
    },
    /// A binding does not preserve the descriptors' source-class labels.
    SourceLabelsNotPreserved {
        /// Binding id.
        binding_id: String,
    },
    /// A binding id appears more than once.
    DuplicateBindingId {
        /// Duplicate binding id.
        binding_id: String,
    },
    /// A surface carries more than one binding.
    DuplicateSurfaceBinding {
        /// Surface token.
        surface: &'static str,
    },
    /// A surface has no binding.
    MissingSurfaceBinding {
        /// Surface token.
        surface: &'static str,
    },
    /// A binding is not stamped with the active snapshot id.
    SnapshotBindingMismatch {
        /// Binding id.
        binding_id: String,
    },
    /// A binding renders a scope id other than the active scope.
    ScopeIdMismatch {
        /// Binding id.
        binding_id: String,
    },
    /// A binding carries a descriptor id not declared in the packet.
    UnresolvedDescriptorRef {
        /// Binding id.
        binding_id: String,
        /// Unresolved descriptor id.
        descriptor_id: String,
    },
    /// A binding carries a descriptor more restricted than its declared visibility ceiling.
    VisibilityExceedsBinding {
        /// Binding id.
        binding_id: String,
        /// Carried descriptor id.
        descriptor_id: String,
        /// Descriptor visibility token.
        visibility: &'static str,
        /// Binding ceiling token.
        max_visibility: &'static str,
    },
    /// An export-safe descriptor is not carried by the support-export binding.
    ExportSafeDescriptorMissingFromSupportExport {
        /// Descriptor id.
        descriptor_id: String,
    },
    /// A private descriptor is carried by the support-export binding.
    PrivateDescriptorInSupportExport {
        /// Descriptor id.
        descriptor_id: String,
    },
    /// The summary counts disagree with the packet body.
    SummaryMismatch,
}

impl fmt::Display for M5OwnershipContractViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::GovernanceMatrixRefMismatch => write!(
                f,
                "packet governance_matrix_ref must be the canonical graph-depth governance matrix"
            ),
            Self::ScopePacketRefMismatch => write!(
                f,
                "packet scope_packet_ref must be the canonical workset-scope packet"
            ),
            Self::TopologyPacketRefMismatch => write!(
                f,
                "packet topology_packet_ref must be the canonical topology-identity packet"
            ),
            Self::DuplicateDescriptorId { descriptor_id } => {
                write!(f, "duplicate descriptor id {descriptor_id}")
            }
            Self::MissingSourceReason {
                descriptor_id,
                source_class,
            } => write!(
                f,
                "descriptor {descriptor_id} is {source_class} but carries no explicit source_reason"
            ),
            Self::ChangeControlWithoutLink { descriptor_id } => write!(
                f,
                "descriptor {descriptor_id} is a change_control role but carries no change_control_url"
            ),
            Self::NonChangeControlWithLink {
                descriptor_id,
                role,
            } => write!(
                f,
                "descriptor {descriptor_id} is a {role} role but carries a change_control_url"
            ),
            Self::UnsafeDescriptorPermalink { descriptor_id } => write!(
                f,
                "descriptor {descriptor_id} has an empty permalink or one that does not embed its id"
            ),
            Self::SelfSupersede { descriptor_id } => {
                write!(f, "descriptor {descriptor_id} supersedes itself")
            }
            Self::UnresolvedSupersedesRef {
                descriptor_id,
                superseded_id,
            } => write!(
                f,
                "descriptor {descriptor_id} supersedes {superseded_id} that is not declared in the packet"
            ),
            Self::InferenceOverwritesCurated {
                descriptor_id,
                superseded_id,
                source_class,
                superseded_source_class,
            } => write!(
                f,
                "{source_class} descriptor {descriptor_id} may not supersede {superseded_source_class} descriptor {superseded_id}"
            ),
            Self::SourceLabelsNotPreserved { binding_id } => write!(
                f,
                "binding {binding_id} does not preserve source-class labels"
            ),
            Self::DuplicateBindingId { binding_id } => {
                write!(f, "duplicate binding id {binding_id}")
            }
            Self::DuplicateSurfaceBinding { surface } => {
                write!(f, "duplicate binding for surface {surface}")
            }
            Self::MissingSurfaceBinding { surface } => {
                write!(f, "missing binding for surface {surface}")
            }
            Self::SnapshotBindingMismatch { binding_id } => write!(
                f,
                "binding {binding_id} is not stamped with the active snapshot id"
            ),
            Self::ScopeIdMismatch { binding_id } => write!(
                f,
                "binding {binding_id} renders a scope other than the active scope"
            ),
            Self::UnresolvedDescriptorRef {
                binding_id,
                descriptor_id,
            } => write!(
                f,
                "binding {binding_id} carries descriptor {descriptor_id} that is not declared in the packet"
            ),
            Self::VisibilityExceedsBinding {
                binding_id,
                descriptor_id,
                visibility,
                max_visibility,
            } => write!(
                f,
                "binding {binding_id} carries {visibility} descriptor {descriptor_id} beyond its {max_visibility} ceiling"
            ),
            Self::ExportSafeDescriptorMissingFromSupportExport { descriptor_id } => write!(
                f,
                "export-safe descriptor {descriptor_id} is not carried by the support-export binding"
            ),
            Self::PrivateDescriptorInSupportExport { descriptor_id } => write!(
                f,
                "private descriptor {descriptor_id} may not be carried by the support-export binding"
            ),
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the packet body")
            }
        }
    }
}

impl Error for M5OwnershipContractViolation {}

/// Loads the embedded M5 ownership-and-contracts packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5OwnershipContractPacket`].
pub fn current_m5_ownership_and_contracts_packet(
) -> Result<M5OwnershipContractPacket, serde_json::Error> {
    serde_json::from_str(M5_OWNERSHIP_CONTRACTS_JSON)
}

#[cfg(test)]
mod tests;
