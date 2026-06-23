//! Relation-navigation qualification: the certification lane that auto-narrows a
//! claimed search/graph/docs/editor relation-navigation claim when its proof is
//! stale or failing.
//!
//! The [`m5_relation_navigation`](crate::m5_relation_navigation) matrix freezes
//! the relation-navigation object model, and each sibling lane
//! ([`relation_resolution`](crate::relation_resolution),
//! [`reference_panes`](crate::reference_panes),
//! [`hierarchy_views`](crate::hierarchy_views),
//! [`related_object_navigation`](crate::related_object_navigation),
//! [`rename_preview`](crate::rename_preview), and
//! [`relation_continuity`](crate::relation_continuity)) produces the typed truth.
//! What was still implicit was a governed place that *certifies* each claimed M5
//! relation-navigation family on every claimed surface and degrades the claim
//! automatically when the proof behind it goes stale or fails. This lane is that
//! place.
//!
//! The certification does five things:
//!
//! 1. **Names the certified relation-navigation families**
//!    ([`RelationNavQualificationFamily`]): definition/declaration/implementation
//!    target-kind honesty, references/access-kind truth, hierarchy proof classes,
//!    related-object attribution, rename-preview completeness, and
//!    continuity/replay fidelity. Each family binds the matrix object(s) it
//!    certifies, the producing module(s), the canonical boundary schema(s), and
//!    the proof packet plus freeze gate that keep it current.
//! 2. **Publishes qualification rows** ([`RelationNavQualificationRow`]): one per
//!    `(family, claimed surface)` pair across the search/navigation, graph/topology,
//!    docs/help, and editor-assist surfaces, each carrying its proof state, proof
//!    freshness, and a *computed* [`ClaimState`].
//! 3. **Auto-narrows stale or failing claims** ([`narrow_claim`]): a row's claim
//!    state is derived purely from its proof state and freshness, so a claim cannot
//!    stay green when its proof is stale, unverified, missing, or failing — it
//!    narrows to a disclosed posture or withdraws entirely, with a visible reason.
//! 4. **Emits release evidence rows** ([`ReleaseEvidenceRow`]): explicit per-family
//!    rows for definition/declaration/implementation honesty, references/access-kind
//!    truth, hierarchy proof classes, related-object attribution, rename-preview
//!    completeness, and continuity/replay fidelity, so a release packet states each
//!    relation-navigation guarantee and whether it currently holds.
//! 5. **Projects to consumer surfaces** ([`QualificationConsumer`]): About, Help,
//!    search/navigation, support, compatibility, release-truth, and public-truth
//!    surfaces consume the same qualification state rather than restating
//!    relation-navigation quality claims by hand.
//!
//! [`relation_navigation_qualification`] builds the canonical, all-green
//! certification deterministically and computes each
//! [`RelationNavQualificationInvariant`]'s `holds` flag from the built rows, so the
//! checked-in fixture and the freeze gate freeze the contract byte-for-byte and an
//! inconsistent edit flips an invariant and fails CI. [`certify`] builds a live
//! certification from per-family proof postures, so feeding a stale or failing
//! posture narrows the affected surface claim automatically — the invariant
//! `relation_nav_qual.narrowing_applied` proves the narrowing is actually applied,
//! and `relation_nav_qual.no_green_claim_without_current_proof` proves no row stays
//! green without current, passing proof. The record carries no source bodies, raw
//! paths, provider payloads, URLs, hostnames, or credentials — only opaque object
//! refs, stable tokens, and short reviewable sentences — so it is safe for support
//! export.

use serde::{Deserialize, Serialize};

use crate::m5_relation_navigation::RelationNavObjectClass;
use crate::target_model::{FreshnessClass, RelationKind};

#[cfg(test)]
mod tests;

/// Schema version for the relation-navigation qualification certification.
pub const RELATION_NAV_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the relation-navigation qualification certification.
pub const RELATION_NAV_QUALIFICATION_SCHEMA_REF: &str =
    "schemas/navigation/relation_navigation_qualification.schema.json";

/// Stable record-kind tag for the relation-navigation qualification certification.
pub const RELATION_NAV_QUALIFICATION_RECORD_KIND: &str =
    "relation_navigation_qualification_certification";

/// Stable id for the canonical relation-navigation qualification certification.
pub const RELATION_NAV_QUALIFICATION_CERTIFICATION_ID: &str =
    "relation-navigation-qualification:certification:0001";

/// Evaluation stamp for the canonical certification. Held as a constant so the
/// canonical binding stays deterministic and the fixture freezes byte-for-byte.
pub const RELATION_NAV_QUALIFICATION_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The freeze gate that keeps the certification binding current. Stable promotion
/// runs this gate; it fails when the in-code certification drifts from the
/// checked-in fixture or any invariant flips.
pub const RELATION_NAV_QUALIFICATION_FREEZE_GATE_REF: &str =
    "crates/aureline-navigation/tests/relation_navigation_qualification.rs";

/// The checked-in canonical certification fixture.
pub const RELATION_NAV_QUALIFICATION_FIXTURE_REF: &str =
    "fixtures/navigation/relation_navigation_qualification/canonical_certification.json";

/// The contract narrative document.
pub const RELATION_NAV_QUALIFICATION_DOC_REF: &str =
    "docs/navigation/relation_navigation_qualification.md";

/// The human-readable evidence companion artifact.
pub const RELATION_NAV_QUALIFICATION_ARTIFACT_REF: &str =
    "artifacts/navigation/relation_navigation_qualification.md";

// ---------------------------------------------------------------------------
// Certified relation-navigation families.
// ---------------------------------------------------------------------------

/// The closed set of relation-navigation families this lane certifies.
///
/// Each family is one governed relation-navigation guarantee. Adding a family is a
/// breaking change to the certification; renaming one breaks every consumer that
/// resolves a family by token, so the tokens are frozen here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationNavQualificationFamily {
    /// Definition/declaration/implementation target-kind honesty: a definition jump
    /// is never relabeled a declaration and a fallback is disclosed, not shown as
    /// semantic certainty.
    TargetKindHonesty,
    /// Find-references access-kind truth: read/write/call/test-only/generated
    /// occurrences keep their access kind and proof class.
    ReferenceAccessKindTruth,
    /// Call/type/override/ownership hierarchy proof classes: each edge preserves its
    /// proof class and ambiguity.
    HierarchyProofClasses,
    /// Related-object attribution: every related-object link stays source-attributed
    /// and disambiguable.
    RelatedObjectAttribution,
    /// Rename-preview completeness: blocked/generated/read-only/partial-scope
    /// candidates are exposed before any broad mutation.
    RenamePreviewCompleteness,
    /// Continuity/replay fidelity: peek/reveal/split/history entries preserve
    /// relation kind and target truth across replay and drift.
    ContinuityReplayFidelity,
}

impl RelationNavQualificationFamily {
    /// All families, in certification order.
    pub const ALL: [Self; 6] = [
        Self::TargetKindHonesty,
        Self::ReferenceAccessKindTruth,
        Self::HierarchyProofClasses,
        Self::RelatedObjectAttribution,
        Self::RenamePreviewCompleteness,
        Self::ContinuityReplayFidelity,
    ];

    /// The five families a release evidence packet must name explicitly per the
    /// qualification contract.
    pub const NAMED_RELEASE_EVIDENCE_FAMILIES: [Self; 5] = [
        Self::TargetKindHonesty,
        Self::ReferenceAccessKindTruth,
        Self::HierarchyProofClasses,
        Self::RelatedObjectAttribution,
        Self::RenamePreviewCompleteness,
    ];

    /// Stable snake_case token for this family.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetKindHonesty => "target_kind_honesty",
            Self::ReferenceAccessKindTruth => "reference_access_kind_truth",
            Self::HierarchyProofClasses => "hierarchy_proof_classes",
            Self::RelatedObjectAttribution => "related_object_attribution",
            Self::RenamePreviewCompleteness => "rename_preview_completeness",
            Self::ContinuityReplayFidelity => "continuity_replay_fidelity",
        }
    }

    /// Stable family id, namespaced so it is unique across the product.
    pub fn family_id(self) -> String {
        format!("relation_nav_qual_family.{}", self.as_str())
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::TargetKindHonesty => "Target-kind honesty",
            Self::ReferenceAccessKindTruth => "References / access-kind truth",
            Self::HierarchyProofClasses => "Hierarchy proof classes",
            Self::RelatedObjectAttribution => "Related-object attribution",
            Self::RenamePreviewCompleteness => "Rename-preview completeness",
            Self::ContinuityReplayFidelity => "Continuity / replay fidelity",
        }
    }

    /// The claimed surfaces this family backs, in surface order. The certification
    /// publishes one qualification row per `(family, surface)` here.
    pub fn claimed_surfaces(self) -> Vec<ClaimedSurface> {
        use ClaimedSurface::*;
        match self {
            Self::TargetKindHonesty => {
                vec![SearchNavigation, EditorAssist, GraphTopology, DocsHelp]
            }
            Self::ReferenceAccessKindTruth => vec![SearchNavigation, EditorAssist, GraphTopology],
            Self::HierarchyProofClasses => vec![GraphTopology, EditorAssist, DocsHelp],
            Self::RelatedObjectAttribution => {
                vec![SearchNavigation, GraphTopology, DocsHelp, EditorAssist]
            }
            Self::RenamePreviewCompleteness => vec![EditorAssist, SearchNavigation],
            Self::ContinuityReplayFidelity => vec![EditorAssist, SearchNavigation, DocsHelp],
        }
    }

    /// The matrix object id(s) this family certifies, so the certification is bound
    /// to the relation-navigation matrix rather than asserting unsourced guarantees.
    fn certified_object_refs(self) -> Vec<String> {
        use RelationNavObjectClass::*;
        let classes: &[RelationNavObjectClass] = match self {
            Self::TargetKindHonesty => &[NavigationTarget, RelationFallbackVocabulary],
            Self::ReferenceAccessKindTruth => &[ReferenceOccurrence],
            Self::HierarchyProofClasses => &[HierarchyEdge],
            Self::RelatedObjectAttribution => &[RelatedObjectRelation],
            Self::RenamePreviewCompleteness => &[RenamePreviewSet],
            Self::ContinuityReplayFidelity => &[NavigationTarget, ReferenceOccurrence],
        };
        classes.iter().map(|c| c.object_id()).collect()
    }

    /// The crate module(s) that already produce this family's truth.
    fn produced_by_refs(self) -> Vec<String> {
        let module = |name: &str| format!("crates/aureline-navigation/src/{name}/mod.rs");
        match self {
            Self::TargetKindHonesty => vec![module("relation_resolution"), module("target_model")],
            Self::ReferenceAccessKindTruth => {
                vec![module("reference_panes"), module("target_model")]
            }
            Self::HierarchyProofClasses => vec![module("hierarchy_views")],
            Self::RelatedObjectAttribution => vec![module("related_object_navigation")],
            Self::RenamePreviewCompleteness => vec![module("rename_preview")],
            Self::ContinuityReplayFidelity => vec![
                module("relation_continuity"),
                module("bookmark_history_and_drift_continuity"),
            ],
        }
    }

    /// The canonical boundary schema(s) this family binds.
    fn canonical_schema_refs(self) -> Vec<String> {
        let schema = |name: &str| format!("schemas/navigation/{name}.schema.json");
        match self {
            Self::TargetKindHonesty => vec![
                schema("navigation_target"),
                schema("relation_navigation_resolution"),
            ],
            Self::ReferenceAccessKindTruth => vec![schema("reference_panes")],
            Self::HierarchyProofClasses => vec![schema("hierarchy_views")],
            Self::RelatedObjectAttribution => vec![schema("related_object_navigation")],
            Self::RenamePreviewCompleteness => vec![schema("governed_rename_preview")],
            Self::ContinuityReplayFidelity => vec![schema("relation_continuity")],
        }
    }

    /// The proof packet (the sibling lane's canonical fixture) that keeps this family
    /// current. Stable promotion fails when this is empty.
    fn proof_packet_ref(self) -> String {
        let fixture = |dir: &str, file: &str| format!("fixtures/navigation/{dir}/{file}.json");
        match self {
            Self::TargetKindHonesty => {
                fixture("relation_navigation_resolution", "canonical_resolutions")
            }
            Self::ReferenceAccessKindTruth => fixture("reference_panes", "canonical_panes"),
            Self::HierarchyProofClasses => fixture("hierarchy_views", "canonical_views"),
            Self::RelatedObjectAttribution => {
                fixture("related_object_navigation", "canonical_links")
            }
            Self::RenamePreviewCompleteness => {
                fixture("governed_rename_preview", "canonical_previews")
            }
            Self::ContinuityReplayFidelity => {
                fixture("relation_continuity", "canonical_continuity")
            }
        }
    }

    /// The sibling lane freeze gate that re-checks this family's proof under
    /// `cargo test --workspace`.
    fn freeze_gate_ref(self) -> String {
        let gate = |name: &str| format!("crates/aureline-navigation/tests/{name}.rs");
        match self {
            Self::TargetKindHonesty => gate("relation_navigation_resolution"),
            Self::ReferenceAccessKindTruth => gate("reference_panes"),
            Self::HierarchyProofClasses => gate("hierarchy_views"),
            Self::RelatedObjectAttribution => gate("related_object_navigation"),
            Self::RenamePreviewCompleteness => gate("rename_preview"),
            Self::ContinuityReplayFidelity => gate("relation_continuity"),
        }
    }

    /// The relation kinds this family certifies.
    fn relation_kinds(self) -> Vec<RelationKind> {
        use RelationKind::*;
        match self {
            Self::TargetKindHonesty => vec![Definition, Declaration, Implementation, Type],
            Self::ReferenceAccessKindTruth => vec![Reference, Call],
            Self::HierarchyProofClasses => {
                vec![Call, Implementation, Type, RouteBinding, OwnerLink, DocLink]
            }
            Self::RelatedObjectAttribution => {
                vec![Type, Implementation, RouteBinding, OwnerLink, DocLink]
            }
            Self::RenamePreviewCompleteness => vec![Reference, Definition],
            Self::ContinuityReplayFidelity => {
                vec![Definition, Declaration, Implementation, Reference]
            }
        }
    }

    /// One reviewable sentence naming what this family certifies on its surfaces.
    fn claim_subject(self) -> &'static str {
        match self {
            Self::TargetKindHonesty => {
                "Definition, declaration, and implementation target-kind honesty"
            }
            Self::ReferenceAccessKindTruth => "Find-references access-kind truth",
            Self::HierarchyProofClasses => {
                "Call, type, override, and ownership hierarchy proof classes"
            }
            Self::RelatedObjectAttribution => "Related-object source attribution",
            Self::RenamePreviewCompleteness => {
                "Rename-preview blocked, generated, read-only, and partial-scope completeness"
            }
            Self::ContinuityReplayFidelity => {
                "Peek, reveal, split, and history continuity and replay fidelity"
            }
        }
    }

    /// One reviewable sentence stating the relation-kind honesty rule this family
    /// protects — the track invariant a release packet must be able to cite.
    fn evidence_claim(self) -> &'static str {
        match self {
            Self::TargetKindHonesty => {
                "A definition jump is never relabeled a declaration and a grep fallback is \
                 disclosed, never shown as semantic certainty."
            }
            Self::ReferenceAccessKindTruth => {
                "Read, write, call, test-only, and generated occurrences keep their access kind and \
                 proof class and are never folded into one undifferentiated count."
            }
            Self::HierarchyProofClasses => {
                "Direct, transitive, inferred, and runtime-observed edges preserve their proof class \
                 and ambiguity, so an observed-dispatch edge is never shown as a static fact."
            }
            Self::RelatedObjectAttribution => {
                "Every related-object link is source-attributed and disambiguable, so a framework \
                 guess never poses as a graph-proven fact."
            }
            Self::RenamePreviewCompleteness => {
                "Blocked, generated, read-only, and partial-scope candidates are exposed before any \
                 broad mutation."
            }
            Self::ContinuityReplayFidelity => {
                "Peek, reveal, split, and history entries preserve relation kind and target truth \
                 across replay and drift, never silently retargeting to a nearby guess."
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Claimed surfaces.
// ---------------------------------------------------------------------------

/// The claimed M5 profiles whose relation-navigation claim this lane governs and
/// narrows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimedSurface {
    /// The unified search and navigation palette.
    SearchNavigation,
    /// The graph / topology overlay.
    GraphTopology,
    /// Docs, Help, and About truth surfaces.
    DocsHelp,
    /// The editor go-to, peek, references, hierarchy, and rename assist surfaces.
    EditorAssist,
}

impl ClaimedSurface {
    /// All claimed surfaces, in order.
    pub const ALL: [Self; 4] = [
        Self::SearchNavigation,
        Self::GraphTopology,
        Self::DocsHelp,
        Self::EditorAssist,
    ];

    /// Stable snake_case token for this surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchNavigation => "search_navigation",
            Self::GraphTopology => "graph_topology",
            Self::DocsHelp => "docs_help",
            Self::EditorAssist => "editor_assist",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SearchNavigation => "search / navigation",
            Self::GraphTopology => "graph / topology",
            Self::DocsHelp => "docs / help",
            Self::EditorAssist => "editor assist",
        }
    }
}

// ---------------------------------------------------------------------------
// Proof state and freshness.
// ---------------------------------------------------------------------------

/// The pass/fail state of the proof packet behind a qualification row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofState {
    /// The freeze gate that proves this family is passing.
    Passing,
    /// The proof is in progress or awaiting a refresh; it has not failed but is not
    /// yet current.
    Pending,
    /// The freeze gate that proves this family is failing.
    Failing,
    /// No proof packet is wired for this family on this surface.
    Missing,
}

impl ProofState {
    /// All proof states, in order.
    pub const ALL: [Self; 4] = [Self::Passing, Self::Pending, Self::Failing, Self::Missing];

    /// Stable snake_case token for this state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passing => "passing",
            Self::Pending => "pending",
            Self::Failing => "failing",
            Self::Missing => "missing",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Passing => "Passing",
            Self::Pending => "Pending",
            Self::Failing => "Failing",
            Self::Missing => "Missing",
        }
    }
}

/// The freshness of the proof packet behind a qualification row, mirroring the
/// upstream [`FreshnessClass`] so the certification never diverges from the object
/// model's freshness vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofFreshness {
    /// Revalidated against the current workspace epoch.
    Live,
    /// Warm cache known to be current enough for a navigation claim.
    Warm,
    /// Cache usable only with a downgrade disclosure.
    Degraded,
    /// Past its freshness floor.
    Stale,
    /// Not verified against the current workspace.
    Unverified,
}

impl ProofFreshness {
    /// All freshness classes, in order.
    pub const ALL: [Self; 5] = [
        Self::Live,
        Self::Warm,
        Self::Degraded,
        Self::Stale,
        Self::Unverified,
    ];

    /// Stable snake_case token for this freshness class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Warm => "warm",
            Self::Degraded => "degraded",
            Self::Stale => "stale",
            Self::Unverified => "unverified",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Live => "Live",
            Self::Warm => "Warm",
            Self::Degraded => "Degraded",
            Self::Stale => "Stale",
            Self::Unverified => "Unverified",
        }
    }

    /// Whether this freshness is current enough to support a green claim.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Live | Self::Warm)
    }

    /// Whether this freshness requires a visible disclosure caveat.
    pub const fn requires_disclosure(self) -> bool {
        matches!(self, Self::Degraded | Self::Stale | Self::Unverified)
    }

    /// The upstream [`FreshnessClass`] this class derives from, for provenance.
    pub const fn freshness_class(self) -> FreshnessClass {
        match self {
            Self::Live => FreshnessClass::AuthoritativeLive,
            Self::Warm => FreshnessClass::WarmCached,
            Self::Degraded => FreshnessClass::DegradedCached,
            Self::Stale => FreshnessClass::Stale,
            Self::Unverified => FreshnessClass::Unverified,
        }
    }

    /// The provenance ref naming the upstream [`FreshnessClass`] variant this class
    /// derives from, so the certification's freshness vocabulary never silently
    /// diverges from the object model.
    pub fn derived_from_ref(self) -> String {
        let variant = match self {
            Self::Live => "AuthoritativeLive",
            Self::Warm => "WarmCached",
            Self::Degraded => "DegradedCached",
            Self::Stale => "Stale",
            Self::Unverified => "Unverified",
        };
        format!("crates/aureline-navigation/src/target_model/mod.rs#FreshnessClass::{variant}")
    }
}

// ---------------------------------------------------------------------------
// Claim states.
// ---------------------------------------------------------------------------

/// The computed, possibly-narrowed state of a relation-navigation claim.
///
/// A claim state is never authored directly: it is derived from the proof state and
/// freshness by [`narrow_claim`], so a claim cannot stay green when its proof is
/// stale, unverified, missing, or failing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimState {
    /// Proof is passing and current; the full claim stands.
    Qualified,
    /// Proof is passing but degraded; the claim stands only with a downgrade
    /// disclosure.
    NarrowedDisclosed,
    /// Proof is passing but stale; the claim is narrowed to a captured, last-known
    /// posture rather than a live guarantee.
    NarrowedStale,
    /// Proof is unverified, pending, or unmapped; the claim is withdrawn until proof
    /// refreshes.
    WithdrawnPendingProof,
    /// Proof is failing; the claim is withdrawn and the surface degrades to a
    /// disclosed / inspect-only posture.
    WithdrawnFailing,
}

impl ClaimState {
    /// All claim states, in severity order (least to most severe).
    pub const ALL: [Self; 5] = [
        Self::Qualified,
        Self::NarrowedDisclosed,
        Self::NarrowedStale,
        Self::WithdrawnPendingProof,
        Self::WithdrawnFailing,
    ];

    /// Stable snake_case token for this claim state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::NarrowedStale => "narrowed_stale",
            Self::WithdrawnPendingProof => "withdrawn_pending_proof",
            Self::WithdrawnFailing => "withdrawn_failing",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Qualified => "Qualified",
            Self::NarrowedDisclosed => "Narrowed — disclosed",
            Self::NarrowedStale => "Narrowed — stale",
            Self::WithdrawnPendingProof => "Withdrawn — pending proof",
            Self::WithdrawnFailing => "Withdrawn — failing",
        }
    }

    /// Severity rank, so a surface claim can be aggregated to its worst row.
    pub const fn severity(self) -> u8 {
        match self {
            Self::Qualified => 0,
            Self::NarrowedDisclosed => 1,
            Self::NarrowedStale => 2,
            Self::WithdrawnPendingProof => 3,
            Self::WithdrawnFailing => 4,
        }
    }

    /// Whether the claim stays fully green.
    pub const fn is_green(self) -> bool {
        matches!(self, Self::Qualified)
    }

    /// Whether the claim is narrowed but still presentable with disclosure.
    pub const fn is_narrowed(self) -> bool {
        matches!(self, Self::NarrowedDisclosed | Self::NarrowedStale)
    }

    /// Whether the claim is withdrawn entirely.
    pub const fn is_withdrawn(self) -> bool {
        matches!(self, Self::WithdrawnPendingProof | Self::WithdrawnFailing)
    }

    /// Whether the claim requires a visible caveat (anything but qualified).
    pub const fn requires_disclosure(self) -> bool {
        !self.is_green()
    }
}

/// Derives the claim state from a row's proof state and freshness.
///
/// This is the core of the lane: a claim is green only when its proof is passing
/// *and* current. Failing proof withdraws the claim; missing, pending, or unverified
/// proof withdraws it pending a refresh; degraded or stale proof narrows it to a
/// disclosed posture.
pub fn narrow_claim(proof: ProofState, freshness: ProofFreshness) -> ClaimState {
    match proof {
        ProofState::Failing => ClaimState::WithdrawnFailing,
        ProofState::Missing | ProofState::Pending => ClaimState::WithdrawnPendingProof,
        ProofState::Passing => match freshness {
            ProofFreshness::Live | ProofFreshness::Warm => ClaimState::Qualified,
            ProofFreshness::Degraded => ClaimState::NarrowedDisclosed,
            ProofFreshness::Stale => ClaimState::NarrowedStale,
            ProofFreshness::Unverified => ClaimState::WithdrawnPendingProof,
        },
    }
}

/// The reviewable reason a claim narrowed or withdrew, or `None` when it stays
/// qualified.
fn narrowing_reason(proof: ProofState, freshness: ProofFreshness) -> Option<String> {
    let state = narrow_claim(proof, freshness);
    if state.is_green() {
        return None;
    }
    Some(format!(
        "proof_state={} and freshness={} narrow the claim to {}",
        proof.as_str(),
        freshness.as_str(),
        state.as_str()
    ))
}

/// The disclosure caveat that must render for a narrowed or withdrawn claim, or
/// `None` when it stays qualified.
fn disclosure_note(state: ClaimState) -> Option<String> {
    let note = match state {
        ClaimState::Qualified => return None,
        ClaimState::NarrowedDisclosed => {
            "Proof is current but degraded; the claim renders only with a downgrade disclosure."
        }
        ClaimState::NarrowedStale => {
            "Proof is past its freshness floor; the claim is narrowed to a captured, last-known \
             posture rather than a live guarantee."
        }
        ClaimState::WithdrawnPendingProof => {
            "Proof is unverified, pending, or unmapped; the claim is withdrawn until proof refreshes."
        }
        ClaimState::WithdrawnFailing => {
            "Proof is failing; the claim is withdrawn and the surface degrades to a disclosed / \
             inspect-only posture."
        }
    };
    Some(note.to_owned())
}

// ---------------------------------------------------------------------------
// Consumer surfaces.
// ---------------------------------------------------------------------------

/// The surfaces that consume the qualification state instead of restating
/// relation-navigation quality claims by hand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationConsumer {
    /// The About truth surface.
    About,
    /// The Help center.
    Help,
    /// The search / navigation palette qualification banner.
    SearchNavigation,
    /// The support bundle / export packet.
    Support,
    /// The compatibility matrix.
    Compatibility,
    /// The release-truth / shiproom packet.
    ReleaseTruth,
    /// The public-truth comparison surface.
    PublicTruth,
}

impl QualificationConsumer {
    /// All consumer surfaces, in order.
    pub const ALL: [Self; 7] = [
        Self::About,
        Self::Help,
        Self::SearchNavigation,
        Self::Support,
        Self::Compatibility,
        Self::ReleaseTruth,
        Self::PublicTruth,
    ];

    /// Stable snake_case token for this consumer.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::About => "about",
            Self::Help => "help",
            Self::SearchNavigation => "search_navigation",
            Self::Support => "support",
            Self::Compatibility => "compatibility",
            Self::ReleaseTruth => "release_truth",
            Self::PublicTruth => "public_truth",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::About => "About",
            Self::Help => "Help",
            Self::SearchNavigation => "Search / navigation",
            Self::Support => "Support export",
            Self::Compatibility => "Compatibility",
            Self::ReleaseTruth => "Release truth",
            Self::PublicTruth => "Public truth",
        }
    }

    /// One reviewable sentence describing how this surface consumes the state.
    fn summary(self) -> &'static str {
        match self {
            Self::About => {
                "About reads the certification's family claim states directly so the relation-\
                 navigation guarantees it shows reflect current proof."
            }
            Self::Help => {
                "Help renders the per-family qualification rows and their narrowing reasons rather \
                 than restating relation-navigation quality by hand."
            }
            Self::SearchNavigation => {
                "The search / navigation palette banners the claim state for the surface it is on, \
                 so a narrowed or withdrawn claim is visible at the point of use."
            }
            Self::Support => {
                "Support export embeds the full certification, including narrowed and withdrawn rows \
                 with their reasons, with no source bodies."
            }
            Self::Compatibility => {
                "The compatibility matrix reads each family's claim state so a stale or failing \
                 relation-navigation family narrows the compatibility row automatically."
            }
            Self::ReleaseTruth => {
                "The release-truth packet consumes the release evidence rows so promotion sees each \
                 relation-navigation guarantee and whether it currently holds."
            }
            Self::PublicTruth => {
                "Public truth surfaces the family claim states so a withdrawn claim is never \
                 published as green."
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Inputs.
// ---------------------------------------------------------------------------

/// The live proof posture of one certified family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FamilyProofPosture {
    /// The family this posture applies to.
    pub family: RelationNavQualificationFamily,
    /// The pass/fail state of the family's proof packet.
    pub proof_state: ProofState,
    /// The freshness of the family's proof packet.
    pub proof_freshness: ProofFreshness,
}

/// The input to [`certify`]: the evaluation stamp and per-family proof postures.
///
/// Families absent from `postures` default to a passing, live posture, so a partial
/// input degrades only the families it names.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationNavQualificationInput {
    /// The evaluation stamp.
    pub as_of: String,
    /// The per-family proof postures.
    pub postures: Vec<FamilyProofPosture>,
}

impl RelationNavQualificationInput {
    /// Resolves the posture for a family, defaulting to passing + live.
    fn posture_for(&self, family: RelationNavQualificationFamily) -> (ProofState, ProofFreshness) {
        self.postures
            .iter()
            .find(|p| p.family == family)
            .map(|p| (p.proof_state, p.proof_freshness))
            .unwrap_or((ProofState::Passing, ProofFreshness::Live))
    }
}

/// The default, all-green input: every family passing and live.
pub fn default_qualification_input() -> RelationNavQualificationInput {
    RelationNavQualificationInput {
        as_of: RELATION_NAV_QUALIFICATION_AS_OF.to_owned(),
        postures: RelationNavQualificationFamily::ALL
            .iter()
            .map(|family| FamilyProofPosture {
                family: *family,
                proof_state: ProofState::Passing,
                proof_freshness: ProofFreshness::Live,
            })
            .collect(),
    }
}

// ---------------------------------------------------------------------------
// Record structs.
// ---------------------------------------------------------------------------

/// One certified relation-navigation family entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationNavQualificationFamilyEntry {
    /// The family.
    pub family: RelationNavQualificationFamily,
    /// Stable, namespaced family id.
    pub family_id: String,
    /// Human-readable label.
    pub label: String,
    /// One reviewable sentence describing the family.
    pub summary: String,
    /// The matrix object id(s) this family certifies.
    pub certified_object_refs: Vec<String>,
    /// The crate module(s) that produce this family's truth.
    pub produced_by_refs: Vec<String>,
    /// The canonical boundary schema(s) this family binds.
    pub canonical_schema_refs: Vec<String>,
    /// The proof packet that keeps this family current. Promotion fails when empty.
    pub proof_packet_ref: String,
    /// The sibling lane freeze gate that re-checks this family's proof.
    pub freeze_gate_ref: String,
    /// The relation kinds this family certifies.
    pub relation_kinds: Vec<RelationKind>,
    /// The claimed surfaces this family backs.
    pub claimed_surfaces: Vec<ClaimedSurface>,
    /// One reviewable sentence stating the relation-kind honesty rule this family
    /// protects.
    pub honesty_rule: String,
    /// Whether a release evidence packet must name this family explicitly.
    pub is_release_evidence_family: bool,
}

/// One qualification row: a `(family, claimed surface)` pair with a computed claim
/// state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationNavQualificationRow {
    /// Stable, namespaced row id.
    pub row_id: String,
    /// The certified family.
    pub family: RelationNavQualificationFamily,
    /// The claimed surface this row governs.
    pub claimed_surface: ClaimedSurface,
    /// One reviewable sentence stating the claim this row certifies.
    pub claim_text: String,
    /// The proof packet that backs this row.
    pub proof_packet_ref: String,
    /// The freeze gate that re-checks this row.
    pub freeze_gate_ref: String,
    /// The pass/fail state of the proof.
    pub proof_state: ProofState,
    /// The freshness of the proof.
    pub proof_freshness: ProofFreshness,
    /// The computed claim state, derived from proof state and freshness.
    pub claim_state: ClaimState,
    /// The reviewable reason the claim narrowed or withdrew, if any.
    pub narrowing_reason: Option<String>,
    /// The disclosure caveat that must render for a narrowed or withdrawn claim, if
    /// any.
    pub disclosure_note: Option<String>,
}

/// One release evidence row: a per-family guarantee a release packet states and
/// whether it currently holds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseEvidenceRow {
    /// Stable, namespaced evidence id.
    pub evidence_id: String,
    /// The family this row attests.
    pub family: RelationNavQualificationFamily,
    /// One reviewable sentence stating the guarantee.
    pub evidence_claim: String,
    /// The proof packet that backs this guarantee.
    pub proof_packet_ref: String,
    /// The freeze gate that re-checks this guarantee.
    pub freeze_gate_ref: String,
    /// The pass/fail state of the proof.
    pub proof_state: ProofState,
    /// The freshness of the proof.
    pub proof_freshness: ProofFreshness,
    /// The computed claim state for the family.
    pub claim_state: ClaimState,
    /// Whether the guarantee currently holds (claim state is qualified).
    pub holds: bool,
}

/// One consumer-surface projection: how a surface consumes the qualification state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationConsumerProjection {
    /// The consumer.
    pub consumer: QualificationConsumer,
    /// Human-readable label.
    pub label: String,
    /// One reviewable sentence describing how it consumes the state.
    pub summary: String,
    /// The family tokens this surface projects.
    pub surfaced_family_tokens: Vec<String>,
    /// Whether the surface must visibly show narrowed and withdrawn claims.
    pub highlights_narrowed: bool,
    /// Whether the surface consumes the shared qualification state (always true).
    pub consumes_shared_state: bool,
    /// Whether the surface restates claims manually (always false).
    pub restates_manually: bool,
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationNavQualificationInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built certification satisfies the invariant.
    pub holds: bool,
}

/// The relation-navigation qualification certification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationNavQualificationCertification {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub relation_navigation_qualification_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable certification id.
    pub certification_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// The freeze gate that keeps this certification current.
    pub freeze_gate_ref: String,
    /// One reviewable sentence summarizing the certification.
    pub summary: String,
    /// The certified relation-navigation family entries.
    pub families: Vec<RelationNavQualificationFamilyEntry>,
    /// The qualification rows.
    pub rows: Vec<RelationNavQualificationRow>,
    /// The release evidence rows.
    pub release_evidence: Vec<ReleaseEvidenceRow>,
    /// The consumer-surface projections.
    pub consumer_projections: Vec<QualificationConsumerProjection>,
    /// The computed invariants.
    pub invariants: Vec<RelationNavQualificationInvariant>,
    /// Whether every claim row is qualified (a green certification).
    pub all_claims_qualified: bool,
    /// Whether raw source bodies and payloads are excluded (always true).
    pub raw_payload_excluded: bool,
}

/// Error returned when the certification fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationNavQualificationValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for RelationNavQualificationValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "relation-navigation qualification certification invalid: {}",
            self.reason
        )
    }
}

impl std::error::Error for RelationNavQualificationValidationError {}

impl RelationNavQualificationCertification {
    /// Returns the entry for a family, if present.
    pub fn family(
        &self,
        family: RelationNavQualificationFamily,
    ) -> Option<&RelationNavQualificationFamilyEntry> {
        self.families.iter().find(|f| f.family == family)
    }

    /// Returns the rows that govern a claimed surface.
    pub fn rows_for_surface(
        &self,
        surface: ClaimedSurface,
    ) -> impl Iterator<Item = &RelationNavQualificationRow> {
        self.rows
            .iter()
            .filter(move |r| r.claimed_surface == surface)
    }

    /// The aggregate claim state for a surface: the most-severe row claim state, or
    /// [`ClaimState::Qualified`] when the surface has no rows.
    pub fn surface_claim_state(&self, surface: ClaimedSurface) -> ClaimState {
        self.rows_for_surface(surface)
            .map(|r| r.claim_state)
            .max_by_key(|s| s.severity())
            .unwrap_or(ClaimState::Qualified)
    }

    /// The aggregate claim state for every claimed surface, in surface order.
    pub fn surface_claim_states(&self) -> Vec<(ClaimedSurface, ClaimState)> {
        ClaimedSurface::ALL
            .iter()
            .map(|s| (*s, self.surface_claim_state(*s)))
            .collect()
    }

    /// The rows whose claim narrowed (still presentable with disclosure).
    pub fn narrowed_rows(&self) -> impl Iterator<Item = &RelationNavQualificationRow> {
        self.rows.iter().filter(|r| r.claim_state.is_narrowed())
    }

    /// The rows whose claim withdrew entirely.
    pub fn withdrawn_rows(&self) -> impl Iterator<Item = &RelationNavQualificationRow> {
        self.rows.iter().filter(|r| r.claim_state.is_withdrawn())
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Whether the record is safe to place in a support export: raw payloads are
    /// excluded and every ref is a repo-relative object ref, never a URL, host,
    /// credential, or absolute path.
    pub fn is_support_export_safe(&self) -> bool {
        if !self.raw_payload_excluded {
            return false;
        }
        self.all_refs().all(is_export_safe_ref)
    }

    /// Every ref string carried by the certification, for export-safety auditing.
    fn all_refs(&self) -> impl Iterator<Item = &str> {
        let from_families = self.families.iter().flat_map(|f| {
            f.produced_by_refs
                .iter()
                .map(String::as_str)
                .chain(f.canonical_schema_refs.iter().map(String::as_str))
                .chain(std::iter::once(f.proof_packet_ref.as_str()))
                .chain(std::iter::once(f.freeze_gate_ref.as_str()))
        });
        let from_rows = self.rows.iter().flat_map(|r| {
            std::iter::once(r.proof_packet_ref.as_str())
                .chain(std::iter::once(r.freeze_gate_ref.as_str()))
        });
        let from_evidence = self.release_evidence.iter().flat_map(|e| {
            std::iter::once(e.proof_packet_ref.as_str())
                .chain(std::iter::once(e.freeze_gate_ref.as_str()))
        });
        let from_gate = std::iter::once(self.freeze_gate_ref.as_str());
        from_families
            .chain(from_rows)
            .chain(from_evidence)
            .chain(from_gate)
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    pub fn validate(&self) -> Result<(), RelationNavQualificationValidationError> {
        let fail = |reason: String| Err(RelationNavQualificationValidationError { reason });

        if self.record_kind != RELATION_NAV_QUALIFICATION_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != RELATION_NAV_QUALIFICATION_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }

        // Every family is present exactly once with a mapped proof packet and gate.
        for family in RelationNavQualificationFamily::ALL {
            let matches: Vec<_> = self
                .families
                .iter()
                .filter(|f| f.family == family)
                .collect();
            if matches.len() != 1 {
                return fail(format!(
                    "family {} not present exactly once",
                    family.as_str()
                ));
            }
            let entry = matches[0];
            if entry.family_id != family.family_id() {
                return fail(format!("family id mismatch for {}", family.as_str()));
            }
            if entry.certified_object_refs.is_empty() {
                return fail(format!(
                    "family {} certifies no matrix object",
                    family.as_str()
                ));
            }
            if entry.produced_by_refs.is_empty() {
                return fail(format!("family {} has no producer", family.as_str()));
            }
            if entry.canonical_schema_refs.is_empty() {
                return fail(format!("family {} cites no schema", family.as_str()));
            }
            if entry.proof_packet_ref.is_empty() || entry.freeze_gate_ref.is_empty() {
                return fail(format!(
                    "family {} has no mapped proof packet or freeze gate",
                    family.as_str()
                ));
            }
            if entry.relation_kinds.is_empty() {
                return fail(format!(
                    "family {} certifies no relation kind",
                    family.as_str()
                ));
            }
            if entry.claimed_surfaces.is_empty() {
                return fail(format!("family {} backs no surface", family.as_str()));
            }
        }

        // Stable ids are unique.
        if !all_unique(self.families.iter().map(|f| f.family_id.as_str())) {
            return fail("family ids are not unique".to_owned());
        }
        if !all_unique(self.rows.iter().map(|r| r.row_id.as_str())) {
            return fail("row ids are not unique".to_owned());
        }
        if !all_unique(self.release_evidence.iter().map(|e| e.evidence_id.as_str())) {
            return fail("evidence ids are not unique".to_owned());
        }

        // Every row applies the narrowing function and discloses when not green.
        for row in &self.rows {
            let expected = narrow_claim(row.proof_state, row.proof_freshness);
            if row.claim_state != expected {
                return fail(format!(
                    "row {} claim_state {} does not match narrow_claim {}",
                    row.row_id,
                    row.claim_state.as_str(),
                    expected.as_str()
                ));
            }
            if row.claim_state.is_green() {
                if row.narrowing_reason.is_some() || row.disclosure_note.is_some() {
                    return fail(format!("qualified row {} carries a caveat", row.row_id));
                }
            } else if row.narrowing_reason.is_none() || row.disclosure_note.is_none() {
                return fail(format!(
                    "narrowed row {} lacks a reason or disclosure",
                    row.row_id
                ));
            }
        }

        // Release evidence names the five required families and is consistent.
        for family in RelationNavQualificationFamily::NAMED_RELEASE_EVIDENCE_FAMILIES {
            if !self.release_evidence.iter().any(|e| e.family == family) {
                return fail(format!(
                    "release evidence missing required family {}",
                    family.as_str()
                ));
            }
        }
        for evidence in &self.release_evidence {
            let expected = narrow_claim(evidence.proof_state, evidence.proof_freshness);
            if evidence.claim_state != expected {
                return fail(format!(
                    "evidence {} claim_state inconsistent with proof",
                    evidence.evidence_id
                ));
            }
            if evidence.holds != evidence.claim_state.is_green() {
                return fail(format!(
                    "evidence {} holds flag inconsistent with claim_state",
                    evidence.evidence_id
                ));
            }
        }

        // Every consumer is present once and consumes the shared state.
        for consumer in QualificationConsumer::ALL {
            let matches = self
                .consumer_projections
                .iter()
                .filter(|p| p.consumer == consumer)
                .count();
            if matches != 1 {
                return fail(format!(
                    "consumer {} not present exactly once",
                    consumer.as_str()
                ));
            }
        }
        for projection in &self.consumer_projections {
            if !projection.consumes_shared_state || projection.restates_manually {
                return fail(format!(
                    "consumer {} must consume the shared state and not restate manually",
                    projection.consumer.as_str()
                ));
            }
            if projection.surfaced_family_tokens.is_empty() {
                return fail(format!(
                    "consumer {} surfaces no family",
                    projection.consumer.as_str()
                ));
            }
        }

        if self.all_claims_qualified != self.rows.iter().all(|r| r.claim_state.is_green()) {
            return fail("all_claims_qualified flag inconsistent with rows".to_owned());
        }

        if !self.is_support_export_safe() {
            return fail("certification is not support-export safe".to_owned());
        }
        if !self.all_invariants_hold() {
            let failed: Vec<&str> = self
                .invariants
                .iter()
                .filter(|i| !i.holds)
                .map(|i| i.invariant_id.as_str())
                .collect();
            return fail(format!("invariants do not hold: {}", failed.join(", ")));
        }
        Ok(())
    }
}

fn all_unique<'a>(iter: impl Iterator<Item = &'a str>) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    iter.into_iter().all(|item| seen.insert(item))
}

/// Whether a ref is safe to export: a repo-relative object ref or opaque
/// `aureline://` handle, never a URL, host, credential, or absolute path.
fn is_export_safe_ref(r: &str) -> bool {
    if r.is_empty() || r.starts_with('/') || (r.contains("://") && !r.starts_with("aureline://")) {
        return false;
    }
    r.starts_with("schemas/")
        || r.starts_with("crates/")
        || r.starts_with("artifacts/")
        || r.starts_with("fixtures/")
        || r.starts_with("docs/")
        || r.starts_with("aureline://")
}

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// Builds the canonical, all-green relation-navigation qualification certification.
///
/// Deterministic: the same bytes every call. Every family is passing and live, so
/// every claim is qualified and every invariant holds.
pub fn relation_navigation_qualification() -> RelationNavQualificationCertification {
    certify(&default_qualification_input())
}

/// Builds a certification from per-family proof postures.
///
/// Each row's and evidence row's claim state is derived from its family's posture by
/// [`narrow_claim`], so a stale or failing posture narrows the affected surface
/// claim automatically.
pub fn certify(input: &RelationNavQualificationInput) -> RelationNavQualificationCertification {
    let families = build_families();
    let rows = build_rows(input);
    let release_evidence = build_release_evidence(input);
    let consumer_projections = build_consumer_projections();
    let all_claims_qualified = rows.iter().all(|r| r.claim_state.is_green());
    let invariants = compute_invariants(&families, &rows, &release_evidence, &consumer_projections);

    RelationNavQualificationCertification {
        record_kind: RELATION_NAV_QUALIFICATION_RECORD_KIND.to_owned(),
        relation_navigation_qualification_schema_version: RELATION_NAV_QUALIFICATION_SCHEMA_VERSION,
        schema_ref: RELATION_NAV_QUALIFICATION_SCHEMA_REF.to_owned(),
        certification_id: RELATION_NAV_QUALIFICATION_CERTIFICATION_ID.to_owned(),
        as_of: input.as_of.clone(),
        freeze_gate_ref: RELATION_NAV_QUALIFICATION_FREEZE_GATE_REF.to_owned(),
        summary: "One frozen certification that binds Aureline's relation-kind navigation and \
                  rename-preview truth into M5 promotion: definition/declaration/implementation \
                  target-kind honesty, references/access-kind truth, hierarchy proof classes, \
                  related-object attribution, rename-preview completeness, and continuity/replay \
                  fidelity are each certified on the search/navigation, graph/topology, docs/help, \
                  and editor-assist surfaces, with every claim derived from its proof state and \
                  freshness so a stale or failing proof narrows or withdraws the affected claim \
                  automatically. About, Help, search/navigation, support, compatibility, release-\
                  truth, and public-truth surfaces consume this certification instead of restating \
                  relation-navigation quality by hand."
            .to_owned(),
        families,
        rows,
        release_evidence,
        consumer_projections,
        invariants,
        all_claims_qualified,
        raw_payload_excluded: true,
    }
}

fn build_families() -> Vec<RelationNavQualificationFamilyEntry> {
    RelationNavQualificationFamily::ALL
        .iter()
        .map(|family| {
            let summary = format!(
                "{} certified across {} so the claim narrows automatically when its proof is stale \
                 or failing.",
                family.claim_subject(),
                surface_list(&family.claimed_surfaces()),
            );
            RelationNavQualificationFamilyEntry {
                family: *family,
                family_id: family.family_id(),
                label: family.label().to_owned(),
                summary,
                certified_object_refs: family.certified_object_refs(),
                produced_by_refs: family.produced_by_refs(),
                canonical_schema_refs: family.canonical_schema_refs(),
                proof_packet_ref: family.proof_packet_ref(),
                freeze_gate_ref: family.freeze_gate_ref(),
                relation_kinds: family.relation_kinds(),
                claimed_surfaces: family.claimed_surfaces(),
                honesty_rule: family.evidence_claim().to_owned(),
                is_release_evidence_family: true,
            }
        })
        .collect()
}

fn build_rows(input: &RelationNavQualificationInput) -> Vec<RelationNavQualificationRow> {
    let mut rows = Vec::new();
    for family in RelationNavQualificationFamily::ALL {
        let (proof_state, proof_freshness) = input.posture_for(family);
        let claim_state = narrow_claim(proof_state, proof_freshness);
        for surface in family.claimed_surfaces() {
            rows.push(RelationNavQualificationRow {
                row_id: format!(
                    "relation_nav_qual_row.{}.{}",
                    family.as_str(),
                    surface.as_str()
                ),
                family,
                claimed_surface: surface,
                claim_text: format!(
                    "{} stays trustworthy on the {} surface and narrows automatically when its \
                     proof is stale or failing.",
                    family.claim_subject(),
                    surface.label(),
                ),
                proof_packet_ref: family.proof_packet_ref(),
                freeze_gate_ref: family.freeze_gate_ref(),
                proof_state,
                proof_freshness,
                claim_state,
                narrowing_reason: narrowing_reason(proof_state, proof_freshness),
                disclosure_note: disclosure_note(claim_state),
            });
        }
    }
    rows
}

fn build_release_evidence(input: &RelationNavQualificationInput) -> Vec<ReleaseEvidenceRow> {
    RelationNavQualificationFamily::ALL
        .iter()
        .map(|family| {
            let (proof_state, proof_freshness) = input.posture_for(*family);
            let claim_state = narrow_claim(proof_state, proof_freshness);
            ReleaseEvidenceRow {
                evidence_id: format!("relation_nav_qual_evidence.{}", family.as_str()),
                family: *family,
                evidence_claim: format!("{}: {}", family.label(), family.evidence_claim()),
                proof_packet_ref: family.proof_packet_ref(),
                freeze_gate_ref: family.freeze_gate_ref(),
                proof_state,
                proof_freshness,
                claim_state,
                holds: claim_state.is_green(),
            }
        })
        .collect()
}

fn build_consumer_projections() -> Vec<QualificationConsumerProjection> {
    let all_family_tokens: Vec<String> = RelationNavQualificationFamily::ALL
        .iter()
        .map(|f| f.as_str().to_owned())
        .collect();
    QualificationConsumer::ALL
        .iter()
        .map(|consumer| QualificationConsumerProjection {
            consumer: *consumer,
            label: consumer.label().to_owned(),
            summary: consumer.summary().to_owned(),
            surfaced_family_tokens: all_family_tokens.clone(),
            highlights_narrowed: true,
            consumes_shared_state: true,
            restates_manually: false,
        })
        .collect()
}

fn surface_list(surfaces: &[ClaimedSurface]) -> String {
    surfaces
        .iter()
        .map(|s| s.label())
        .collect::<Vec<_>>()
        .join(", ")
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> RelationNavQualificationInvariant {
    RelationNavQualificationInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(
    families: &[RelationNavQualificationFamilyEntry],
    rows: &[RelationNavQualificationRow],
    release_evidence: &[ReleaseEvidenceRow],
    consumer_projections: &[QualificationConsumerProjection],
) -> Vec<RelationNavQualificationInvariant> {
    let mut out = Vec::new();

    // Every certified family is present exactly once.
    out.push(invariant(
        "relation_nav_qual.every_family_certified",
        "Every certified relation-navigation family — target-kind honesty, references/access-kind \
         truth, hierarchy proof classes, related-object attribution, rename-preview completeness, \
         and continuity/replay fidelity — is present exactly once.",
        RelationNavQualificationFamily::ALL
            .iter()
            .all(|f| families.iter().filter(|e| e.family == *f).count() == 1),
    ));

    // Every family is bound to a matrix object, a producer, and a schema.
    out.push(invariant(
        "relation_nav_qual.family_binds_matrix_object",
        "Every family cites at least one relation-navigation matrix object id, a producing crate \
         module, and a canonical boundary schema, so the certification is bound to the object model \
         rather than asserting unsourced guarantees.",
        families.iter().all(|f| {
            !f.certified_object_refs.is_empty()
                && f.certified_object_refs
                    .iter()
                    .all(|r| r.starts_with("relation_nav_object."))
                && !f.produced_by_refs.is_empty()
                && !f.canonical_schema_refs.is_empty()
        }),
    ));

    // Release-automation binding: every family maps to a proof packet and gate.
    out.push(invariant(
        "relation_nav_qual.every_family_maps_proof",
        "Every family maps to a non-empty proof packet and freeze gate, so stable promotion fails \
         when a claimed relation-navigation family lacks current proof.",
        families
            .iter()
            .all(|f| !f.proof_packet_ref.is_empty() && !f.freeze_gate_ref.is_empty()),
    ));

    // The narrowing function is actually applied to every row.
    out.push(invariant(
        "relation_nav_qual.narrowing_applied",
        "Every qualification row's claim state equals the narrowing of its proof state and \
         freshness, so a claim can never be authored green independently of its proof.",
        rows.iter()
            .all(|r| r.claim_state == narrow_claim(r.proof_state, r.proof_freshness)),
    ));

    // No row stays green without current, passing proof.
    out.push(invariant(
        "relation_nav_qual.no_green_claim_without_current_proof",
        "No qualification row is qualified unless its proof is passing and its freshness is current, \
         so a stale, unverified, pending, missing, or failing proof never leaves a claim green.",
        rows.iter().all(|r| {
            !r.claim_state.is_green()
                || (r.proof_state == ProofState::Passing && r.proof_freshness.is_current())
        }),
    ));

    // Narrowed and withdrawn rows always disclose; qualified rows never do.
    out.push(invariant(
        "relation_nav_qual.narrowed_rows_disclose",
        "Every narrowed or withdrawn row carries a narrowing reason and a disclosure note, and \
         every qualified row carries neither, so a downgraded claim always explains itself.",
        rows.iter().all(|r| {
            if r.claim_state.is_green() {
                r.narrowing_reason.is_none() && r.disclosure_note.is_none()
            } else {
                r.narrowing_reason.is_some() && r.disclosure_note.is_some()
            }
        }),
    ));

    // Every claimed surface is governed by at least one row.
    out.push(invariant(
        "relation_nav_qual.every_surface_governed",
        "Every claimed surface — search/navigation, graph/topology, docs/help, and editor assist — \
         is governed by at least one qualification row.",
        ClaimedSurface::ALL
            .iter()
            .all(|s| rows.iter().any(|r| r.claimed_surface == *s)),
    ));

    // Release evidence names the five required families and is consistent.
    out.push(invariant(
        "relation_nav_qual.release_evidence_covers_named_families",
        "Release evidence includes an explicit, proof-consistent row for definition/declaration/\
         implementation honesty, references/access-kind truth, hierarchy proof classes, related-\
         object attribution, and rename-preview completeness.",
        RelationNavQualificationFamily::NAMED_RELEASE_EVIDENCE_FAMILIES
            .iter()
            .all(|f| release_evidence.iter().any(|e| e.family == *f))
            && release_evidence.iter().all(|e| {
                e.claim_state == narrow_claim(e.proof_state, e.proof_freshness)
                    && e.holds == e.claim_state.is_green()
            }),
    ));

    // Every consumer surface consumes the shared state without restating manually.
    out.push(invariant(
        "relation_nav_qual.consumers_share_state",
        "Every consumer surface — About, Help, search/navigation, support, compatibility, release-\
         truth, and public-truth — consumes the shared qualification state and does not restate \
         relation-navigation quality claims manually.",
        QualificationConsumer::ALL.iter().all(|c| {
            consumer_projections.iter().any(|p| {
                p.consumer == *c
                    && p.consumes_shared_state
                    && !p.restates_manually
                    && !p.surfaced_family_tokens.is_empty()
            })
        }),
    ));

    // Stable ids are unique.
    out.push(invariant(
        "relation_nav_qual.stable_ids_unique",
        "Family ids, row ids, and evidence ids are each defined once and unique, so a consumer can \
         resolve any of them by a stable token.",
        all_unique(families.iter().map(|f| f.family_id.as_str()))
            && all_unique(rows.iter().map(|r| r.row_id.as_str()))
            && all_unique(release_evidence.iter().map(|e| e.evidence_id.as_str())),
    ));

    out
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the certification as human-readable lines for CLI/headless and support.
pub fn relation_navigation_qualification_lines(
    cert: &RelationNavQualificationCertification,
) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Relation-navigation qualification — {} ({})",
        cert.certification_id, cert.as_of
    ));
    lines.push(cert.summary.clone());
    lines.push(format!(
        "Families: {}  Rows: {}  Release evidence: {}  Consumers: {}  Invariants: {}  \
         all_qualified={}",
        cert.families.len(),
        cert.rows.len(),
        cert.release_evidence.len(),
        cert.consumer_projections.len(),
        cert.invariants.len(),
        cert.all_claims_qualified,
    ));

    lines.push("Surface claims:".to_owned());
    for (surface, state) in cert.surface_claim_states() {
        lines.push(format!("  - {} => {}", surface.as_str(), state.as_str()));
    }

    lines.push("Rows:".to_owned());
    for r in &cert.rows {
        lines.push(format!(
            "  - {} [{}/{}] => {}",
            r.row_id,
            r.proof_state.as_str(),
            r.proof_freshness.as_str(),
            r.claim_state.as_str(),
        ));
        if let Some(reason) = &r.narrowing_reason {
            lines.push(format!("      {reason}"));
        }
    }

    lines.push("Release evidence:".to_owned());
    for e in &cert.release_evidence {
        lines.push(format!(
            "  - {} holds={} => {}",
            e.evidence_id,
            e.holds,
            e.claim_state.as_str()
        ));
    }

    lines.push("Invariants:".to_owned());
    for i in &cert.invariants {
        lines.push(format!(
            "  - [{}] {}",
            if i.holds { "ok" } else { "FAIL" },
            i.invariant_id
        ));
    }

    lines
}
