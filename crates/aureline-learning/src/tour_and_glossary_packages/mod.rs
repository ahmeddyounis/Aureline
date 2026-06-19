//! Versioned glossary-pack and tour-package content manifests for the M5
//! feature families, with stable target refs, prerequisites, citations, locale
//! overlays, freshness state, and offline/mirror parity.
//!
//! Where [`crate::qualify_learning_mode_guided_tours_and_teaching_sessions`]
//! attaches *qualification verdicts* to opaque pack refs and
//! [`crate::m5_feature_family_learning_rails`] bundles those qualification
//! records per family, this module owns the **content packages themselves**: the
//! glossary entries and tour steps that point at stable product objects rather
//! than brittle screen coordinates, the version/revision identity each package
//! carries, the locale overlays that localize display copy without disturbing
//! target identity or citations, and the freshness/offline/mirror posture that
//! keeps a cached or mirrored package visibly distinct from current live help.
//!
//! ## What a package carries
//!
//! - **Stable target refs.** Every [`GlossaryEntryRecord`] and [`TourStepRecord`]
//!   references one or more [`StableTargetRef`] — a command id, file/symbol/docs
//!   object id, graph node id, or surface object id — so a tour step survives a
//!   layout change instead of chasing pixel coordinates. The taxonomy aligns with
//!   the public single-package contract at
//!   [`TOUR_PACKAGE_CONTRACT_SCHEMA_REF`].
//! - **Versioned identity.** [`PackageVersion`] pins a version ref and a revision
//!   ref so a package can be reopened, mirrored, or exported and matched back to
//!   the exact content it shipped.
//! - **Prerequisites.** A tour package declares the glossary pack and any earlier
//!   packages it leans on; prerequisites resolve within the manifest and are
//!   cycle-checked.
//! - **Citations.** Each entry and step carries a [`CitationProof`] so localized
//!   or mirrored copies never lose the authoritative command/docs anchors they
//!   cite.
//! - **Locale overlays.** A [`LocaleOverlay`] localizes display labels for the
//!   same entry/step ids and is forbidden from touching target refs or citations,
//!   so target identity survives translation.
//! - **Freshness + offline/mirror parity.** [`FreshnessState`] and the reused
//!   [`MirrorParityPosture`] keep cached, mirrored, local-only, and stale packages
//!   explicitly disclosed — a cached package never masquerades as current live
//!   knowledge.
//!
//! ## Invariants enforced
//!
//! - **No brittle-coordinate-only steps.** A tour step with no stable target
//!   narrows below Stable and fails validation.
//! - **Named scope widening.** When a step widens scope (e.g. from a file to its
//!   folder), [`ScopeWidening`] must name the from/to scope and a reason.
//! - **Localization preserves identity.** A locale overlay that does not preserve
//!   target identity or citations narrows below Stable and fails validation.
//! - **Cached never masquerades as live.** A non-live [`FreshnessState`] must be
//!   explicitly disclosed and agree with the mirror-parity freshness label.
//! - **Prerequisites resolve.** A prerequisite ref into the manifest's own id
//!   namespace must resolve to a present package, with no cycles.
//!
//! ## Canonical truth source
//!
//! [`seeded_m5_tour_and_glossary_packages`] produces the canonical manifest.
//! Help/About, docs/migration, support export, and release surfaces ingest it
//! rather than rephrasing package provenance by hand.
//!
//! - Schema: [`M5_TOUR_AND_GLOSSARY_SCHEMA_REF`]
//! - Fixture: [`M5_TOUR_AND_GLOSSARY_FIXTURE_REF`]
//! - Artifact: [`M5_TOUR_AND_GLOSSARY_ARTIFACT_REF`]
//! - Doc: [`M5_TOUR_AND_GLOSSARY_DOC_REF`]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::m5_feature_family_learning_rails::{M5LearningSurfaceFamily, MirrorParityPosture};
use crate::qualify_learning_mode_guided_tours_and_teaching_sessions::{
    CitationProof, ExplainApplyClass, QualificationVerdict,
};

// ── Schema-version and record-kind constants ─────────────────────────────────

/// Integer schema version for the tour/glossary package records. Bumped only on
/// breaking payload changes; additive-optional fields do not bump it.
pub const M5_TOUR_AND_GLOSSARY_SCHEMA_VERSION: u32 = 1;

/// Record kind for [`GlossaryPack`].
pub const GLOSSARY_PACK_RECORD_KIND: &str = "glossary_pack_record";

/// Record kind for [`TourPackage`].
pub const TOUR_PACKAGE_RECORD_KIND: &str = "tour_package_record";

/// Record kind for [`M5TourAndGlossaryPackageManifest`].
pub const M5_TOUR_AND_GLOSSARY_MANIFEST_RECORD_KIND: &str =
    "m5_tour_and_glossary_package_manifest_record";

// ── Canonical path constants ──────────────────────────────────────────────────

/// Repository-relative path to the boundary schema.
pub const M5_TOUR_AND_GLOSSARY_SCHEMA_REF: &str =
    "schemas/help/m5-tour-and-glossary-packages.schema.json";

/// Repository-relative path to the canonical manifest fixture.
pub const M5_TOUR_AND_GLOSSARY_FIXTURE_REF: &str =
    "fixtures/help/m5/tour-and-glossary-packages/m5_tour_and_glossary_packages.json";

/// Repository-relative path to the proof artifact.
pub const M5_TOUR_AND_GLOSSARY_ARTIFACT_REF: &str =
    "artifacts/ux/m5/tour-package-proof/implement-versioned-glossary-pack-and-tour-package-manifests.md";

/// Repository-relative path to the public doc.
pub const M5_TOUR_AND_GLOSSARY_DOC_REF: &str = "docs/help/m5/tour-and-glossary-packages.md";

/// Repository-relative path to the related public single-package tour contract
/// whose stable-target taxonomy this module's records reuse.
pub const TOUR_PACKAGE_CONTRACT_SCHEMA_REF: &str = "schemas/help/tour_package.schema.json";

// ── Stable target ref ───────────────────────────────────────────────────────

/// The kind of stable product object a step or glossary entry points at.
///
/// Tour steps reference these stable objects instead of brittle screen
/// coordinates, so a step survives layout, theme, and window changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetKind {
    /// A command id from the command registry.
    CommandId,
    /// A stable file object id.
    FileObjectId,
    /// A stable symbol object id.
    SymbolObjectId,
    /// A docs/help node id.
    DocsNodeId,
    /// A semantic/command graph node id.
    GraphNodeId,
    /// A stable UI surface object id (panel, view, region).
    SurfaceObjectId,
}

impl TargetKind {
    /// Stable string token for records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandId => "command_id",
            Self::FileObjectId => "file_object_id",
            Self::SymbolObjectId => "symbol_object_id",
            Self::DocsNodeId => "docs_node_id",
            Self::GraphNodeId => "graph_node_id",
            Self::SurfaceObjectId => "surface_object_id",
        }
    }
}

/// A reference to one stable product object.
///
/// `target_id` is an opaque, stable identifier — never a raw URL, raw absolute
/// path, or pixel coordinate.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct StableTargetRef {
    /// The kind of object this ref points at.
    pub target_kind: TargetKind,
    /// Opaque, stable id of the object.
    pub target_id: String,
}

// ── Source class ──────────────────────────────────────────────────────────────

/// Where a package's content originates.
///
/// Mirrors the source vocabulary of the public single-package tour contract at
/// [`TOUR_PACKAGE_CONTRACT_SCHEMA_REF`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceClass {
    /// Content sourced from the project's own docs.
    ProjectDocs,
    /// Content sourced from a mirrored copy of official docs.
    MirroredOfficialDocs,
    /// Content sourced from a curated knowledge pack.
    CuratedKnowledgePack,
    /// Content sourced from the semantic graph.
    SemanticGraph,
}

impl SourceClass {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectDocs => "project_docs",
            Self::MirroredOfficialDocs => "mirrored_official_docs",
            Self::CuratedKnowledgePack => "curated_knowledge_pack",
            Self::SemanticGraph => "semantic_graph",
        }
    }
}

// ── Freshness state ─────────────────────────────────────────────────────────

/// How current a package's content is relative to the live authoritative source.
///
/// A non-live state MUST be explicitly disclosed so a cached, mirrored, or
/// imported package never masquerades as current live knowledge. The token set
/// matches [`MirrorParityPosture::freshness_label`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessState {
    /// Served from the installed, current authoritative revision.
    LiveAuthoritative,
    /// Served from a mirror, disclosed as such.
    MirrorSyncedDisclosed,
    /// Served from a cached revision, freshness disclosed.
    CachedDisclosed,
    /// Available locally only; not yet mirror-synced.
    LocalOnlyDisclosed,
    /// Known stale; disclosed rather than hidden.
    StaleDisclosed,
}

impl FreshnessState {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveAuthoritative => "live_authoritative",
            Self::MirrorSyncedDisclosed => "mirror_synced_disclosed",
            Self::CachedDisclosed => "cached_disclosed",
            Self::LocalOnlyDisclosed => "local_only_disclosed",
            Self::StaleDisclosed => "stale_disclosed",
        }
    }

    /// Whether this state represents the current live authoritative revision.
    pub const fn is_live(self) -> bool {
        matches!(self, Self::LiveAuthoritative)
    }

    /// Whether this state must be explicitly disclosed to the user.
    ///
    /// Every non-live state requires disclosure so a cached or mirrored package
    /// stays visibly distinct from live help.
    pub const fn requires_disclosure(self) -> bool {
        !self.is_live()
    }

    /// Whether this state qualifies a package Stable on freshness grounds.
    ///
    /// Live and disclosed mirror-synced content are Stable; cached, local-only,
    /// and stale content are honest but narrowed below Stable.
    pub const fn qualifies_stable(self) -> bool {
        matches!(self, Self::LiveAuthoritative | Self::MirrorSyncedDisclosed)
    }
}

// ── Package version ─────────────────────────────────────────────────────────

/// Version and revision identity for a package.
///
/// Both refs are opaque, stable ids. The version ref names the published version;
/// the revision ref pins the exact content revision so an exported or mirrored
/// package can be matched back to what it shipped.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageVersion {
    /// Opaque published-version id.
    pub version_ref: String,
    /// Opaque exact-content-revision id.
    pub revision_ref: String,
}

// ── Locale overlay ────────────────────────────────────────────────────────────

/// A localization overlay for one locale.
///
/// An overlay supplies localized display-label refs for the same entry/step ids
/// the base package defines. It MUST NOT carry target refs or citation refs:
/// localization changes display copy only, never target identity or the
/// authoritative anchors a package cites.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleOverlay {
    /// BCP-47 locale tag (e.g. `fr-FR`).
    pub locale_tag: String,
    /// Localized display-label refs keyed by the base entry/step id.
    pub localized_label_refs: BTreeMap<String, String>,
    /// Whether the overlay preserves target identity (carries no target refs).
    /// MUST be true.
    pub preserves_target_identity: bool,
    /// Whether the overlay preserves citations (carries no citation refs). MUST
    /// be true.
    pub preserves_citations: bool,
    /// Named reason when the overlay fails to preserve identity or citations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
}

impl LocaleOverlay {
    /// Returns true when the overlay preserves both target identity and
    /// citations.
    pub fn qualifies_stable(&self) -> bool {
        self.preserves_target_identity && self.preserves_citations
    }
}

// ── Glossary entry ──────────────────────────────────────────────────────────

/// One term in a glossary pack.
///
/// Each entry names a term, points at the stable objects the term refers to, and
/// cites at least one authoritative anchor. Display copy is referenced by opaque
/// label/definition refs so locale overlays can localize it without touching
/// targets or citations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GlossaryEntryRecord {
    /// Stable id for this entry, used as the locale-overlay key.
    pub entry_id: String,
    /// Short, stable token for the term.
    pub term_token: String,
    /// Opaque ref to the base-locale display label.
    pub label_ref: String,
    /// Opaque ref to the base-locale definition copy.
    pub definition_ref: String,
    /// Stable target refs the term points at (at least one).
    pub stable_targets: Vec<StableTargetRef>,
    /// Citation proof for the term.
    pub citation: CitationProof,
    /// Ids of glossary entries that should be understood first.
    #[serde(default)]
    pub prerequisite_entry_refs: Vec<String>,
}

// ── Scope widening ────────────────────────────────────────────────────────────

/// Whether a tour step widens the working scope, and how it is named.
///
/// When a step broadens scope (for example, from a single file to its folder, or
/// from one statement to a whole connection), the widening MUST be named with a
/// from/to scope and a reason, so a learner is never silently handed a wider
/// blast radius than the previous step implied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeWidening {
    /// Whether this step widens scope relative to the previous step.
    pub widens: bool,
    /// Opaque ref to the scope before this step.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_scope_ref: Option<String>,
    /// Opaque ref to the scope after this step.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_scope_ref: Option<String>,
    /// Named reason the scope widens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub named_reason: Option<String>,
}

impl ScopeWidening {
    /// A non-widening step.
    pub fn none() -> Self {
        Self {
            widens: false,
            from_scope_ref: None,
            to_scope_ref: None,
            named_reason: None,
        }
    }

    /// Returns true when the widening is adequately named.
    ///
    /// A non-widening step always qualifies. A widening step qualifies only when
    /// it names a from-scope, a to-scope, and a reason.
    pub fn qualifies_stable(&self) -> bool {
        if !self.widens {
            return true;
        }
        self.from_scope_ref.is_some() && self.to_scope_ref.is_some() && self.named_reason.is_some()
    }
}

// ── Tour step ─────────────────────────────────────────────────────────────────

/// One step of a tour package.
///
/// A step points at stable objects (never coordinates alone), names any scope
/// widening, keeps explain and apply separate, and cites its authoritative
/// anchors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TourStepRecord {
    /// Stable id for this step, used as the locale-overlay key.
    pub step_id: String,
    /// Zero-based position of the step within the tour.
    pub position_index: u32,
    /// Opaque ref to the base-locale step title.
    pub title_ref: String,
    /// Stable target refs the step points at (at least one — no coordinate-only
    /// steps).
    pub stable_targets: Vec<StableTargetRef>,
    /// Ids of steps that must complete before this one.
    #[serde(default)]
    pub prerequisite_step_refs: Vec<String>,
    /// Citation proof for the step.
    pub citation: CitationProof,
    /// Explain-vs-apply separation class for the step.
    pub explain_apply_class: ExplainApplyClass,
    /// Scope-widening declaration.
    pub scope_widening: ScopeWidening,
    /// Opaque refs to the step's success criteria.
    #[serde(default)]
    pub success_criteria_refs: Vec<String>,
}

impl TourStepRecord {
    /// Whether the step relies on brittle coordinates alone (no stable target).
    pub fn relies_on_coordinates_only(&self) -> bool {
        self.stable_targets.is_empty()
    }
}

// ── Glossary pack ───────────────────────────────────────────────────────────

/// A versioned glossary pack for one feature family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GlossaryPack {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque stable id for this pack.
    pub pack_id: String,
    /// Version and revision identity.
    pub version: PackageVersion,
    /// Feature family this pack serves.
    pub family: M5LearningSurfaceFamily,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Where the pack's content originates.
    pub source_class: SourceClass,
    /// Opaque ref to the source revision.
    pub source_ref: String,
    /// Freshness state of the pack's content.
    pub freshness_state: FreshnessState,
    /// Offline/mirror parity posture.
    pub mirror_parity: MirrorParityPosture,
    /// Base locale the entries are authored in.
    pub base_locale: String,
    /// Localization overlays for additional locales.
    #[serde(default)]
    pub locale_overlays: Vec<LocaleOverlay>,
    /// Ids of packages that should be understood first.
    #[serde(default)]
    pub prerequisite_pack_refs: Vec<String>,
    /// Glossary entries.
    pub entries: Vec<GlossaryEntryRecord>,
    /// Derived verdict.
    pub verdict: QualificationVerdict,
    /// Named narrowing reasons (empty when verdict is QualifiedStable).
    #[serde(default)]
    pub narrowing_reasons: Vec<String>,
}

impl GlossaryPack {
    /// Recomputes [`verdict`](Self::verdict) and
    /// [`narrowing_reasons`](Self::narrowing_reasons) from current evidence.
    pub fn sync_verdict(&mut self) {
        let (verdict, reasons) = derive_glossary_pack_verdict(self);
        self.verdict = verdict;
        self.narrowing_reasons = reasons;
    }

    /// The set of every stable target id this pack references.
    ///
    /// This fingerprint is invariant under localization, export, and reopen — it
    /// is how target identity is proved preserved across those operations.
    pub fn target_ref_fingerprint(&self) -> BTreeSet<String> {
        self.entries
            .iter()
            .flat_map(|e| e.stable_targets.iter().map(|t| t.target_id.clone()))
            .collect()
    }

    /// The set of every citation anchor this pack references.
    pub fn citation_ref_fingerprint(&self) -> BTreeSet<String> {
        self.entries
            .iter()
            .flat_map(|e| citation_refs(&e.citation))
            .collect()
    }

    /// The localized display-label map for `locale_tag`, if an overlay exists.
    ///
    /// The returned labels differ per locale; the pack's
    /// [`target_ref_fingerprint`](Self::target_ref_fingerprint) and
    /// [`citation_ref_fingerprint`](Self::citation_ref_fingerprint) do not.
    pub fn localized_labels(&self, locale_tag: &str) -> Option<&BTreeMap<String, String>> {
        self.locale_overlays
            .iter()
            .find(|o| o.locale_tag == locale_tag)
            .map(|o| &o.localized_label_refs)
    }
}

// ── Tour package ──────────────────────────────────────────────────────────────

/// A versioned tour package for one feature family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TourPackage {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque stable id for this package.
    pub package_id: String,
    /// Version and revision identity.
    pub version: PackageVersion,
    /// Feature family this package serves.
    pub family: M5LearningSurfaceFamily,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Where the package's content originates.
    pub source_class: SourceClass,
    /// Opaque ref to the source revision.
    pub source_ref: String,
    /// Freshness state of the package's content.
    pub freshness_state: FreshnessState,
    /// Offline/mirror parity posture.
    pub mirror_parity: MirrorParityPosture,
    /// Base locale the steps are authored in.
    pub base_locale: String,
    /// Localization overlays for additional locales.
    #[serde(default)]
    pub locale_overlays: Vec<LocaleOverlay>,
    /// Id of the glossary pack this tour leans on.
    pub glossary_pack_ref: String,
    /// Ids of packages that must be understood first (includes the glossary pack).
    #[serde(default)]
    pub prerequisite_package_refs: Vec<String>,
    /// Package-level explain-vs-apply posture.
    pub explain_apply_class: ExplainApplyClass,
    /// Ordered tour steps.
    pub steps: Vec<TourStepRecord>,
    /// Derived verdict.
    pub verdict: QualificationVerdict,
    /// Named narrowing reasons (empty when verdict is QualifiedStable).
    #[serde(default)]
    pub narrowing_reasons: Vec<String>,
}

impl TourPackage {
    /// Recomputes [`verdict`](Self::verdict) and
    /// [`narrowing_reasons`](Self::narrowing_reasons) from current evidence.
    pub fn sync_verdict(&mut self) {
        let (verdict, reasons) = derive_tour_package_verdict(self);
        self.verdict = verdict;
        self.narrowing_reasons = reasons;
    }

    /// The set of every stable target id this package references.
    pub fn target_ref_fingerprint(&self) -> BTreeSet<String> {
        self.steps
            .iter()
            .flat_map(|s| s.stable_targets.iter().map(|t| t.target_id.clone()))
            .collect()
    }

    /// The set of every citation anchor this package references.
    pub fn citation_ref_fingerprint(&self) -> BTreeSet<String> {
        self.steps
            .iter()
            .flat_map(|s| citation_refs(&s.citation))
            .collect()
    }

    /// The localized display-label map for `locale_tag`, if an overlay exists.
    pub fn localized_labels(&self, locale_tag: &str) -> Option<&BTreeMap<String, String>> {
        self.locale_overlays
            .iter()
            .find(|o| o.locale_tag == locale_tag)
            .map(|o| &o.localized_label_refs)
    }
}

/// Collects the citation anchor ids from a [`CitationProof`].
fn citation_refs(citation: &CitationProof) -> Vec<String> {
    citation
        .command_id_refs
        .iter()
        .chain(citation.docs_citation_anchor_refs.iter())
        .chain(citation.symbol_linked_refs.iter())
        .cloned()
        .collect()
}

// ── Verdict derivation ────────────────────────────────────────────────────────

/// Folds the freshness/mirror-parity posture shared by both package kinds.
fn fold_freshness_and_parity(
    label: &str,
    freshness: FreshnessState,
    parity: &MirrorParityPosture,
    verdict: &mut QualificationVerdict,
    reasons: &mut Vec<String>,
) {
    if !freshness.qualifies_stable() {
        *verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        reasons.push(format!(
            "{label}: freshness_not_live: {}",
            freshness.as_str()
        ));
    }
    if !parity.qualifies_stable() {
        *verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        if let Some(r) = &parity.narrowing_reason {
            reasons.push(format!("{label}: mirror_parity: {r}"));
        } else {
            reasons.push(format!("{label}: mirror_parity_inadequate"));
        }
    }
}

/// Folds the locale-overlay posture shared by both package kinds.
fn fold_locale_overlays(
    label: &str,
    overlays: &[LocaleOverlay],
    verdict: &mut QualificationVerdict,
    reasons: &mut Vec<String>,
) {
    for overlay in overlays {
        if !overlay.qualifies_stable() {
            *verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
            if let Some(r) = &overlay.narrowing_reason {
                reasons.push(format!("{label}: locale[{}]: {r}", overlay.locale_tag));
            } else {
                reasons.push(format!(
                    "{label}: locale[{}]_drops_identity_or_citations",
                    overlay.locale_tag
                ));
            }
        }
    }
}

/// Derives a glossary pack's verdict and narrowing reasons from its evidence.
///
/// A pack qualifies Stable only when its freshness is live (or disclosed
/// mirror-synced), its mirror parity holds, every entry carries at least one
/// stable target and a live citation, and every locale overlay preserves target
/// identity and citations.
pub fn derive_glossary_pack_verdict(pack: &GlossaryPack) -> (QualificationVerdict, Vec<String>) {
    let label = &pack.pack_id;
    let mut verdict = QualificationVerdict::QualifiedStable;
    let mut reasons: Vec<String> = Vec::new();

    fold_freshness_and_parity(
        label,
        pack.freshness_state,
        &pack.mirror_parity,
        &mut verdict,
        &mut reasons,
    );
    fold_locale_overlays(label, &pack.locale_overlays, &mut verdict, &mut reasons);

    for entry in &pack.entries {
        if entry.stable_targets.is_empty() {
            verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
            reasons.push(format!(
                "{label}: entry[{}]_no_stable_target",
                entry.entry_id
            ));
        }
        if !entry.citation.has_citation {
            verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
            reasons.push(format!("{label}: entry[{}]_no_citation", entry.entry_id));
        } else if !entry.citation.all_anchors_live_authoritative {
            verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
            reasons.push(format!(
                "{label}: entry[{}]_citation_not_live",
                entry.entry_id
            ));
        }
    }

    reasons.sort();
    reasons.dedup();
    (verdict, reasons)
}

/// Derives a tour package's verdict and narrowing reasons from its evidence.
///
/// A package qualifies Stable only when its freshness is live (or disclosed
/// mirror-synced), its mirror parity holds, every step points at a stable object
/// (no coordinate-only steps), every scope widening is named, no step conflates
/// explain and apply, and every locale overlay preserves identity and citations.
pub fn derive_tour_package_verdict(pkg: &TourPackage) -> (QualificationVerdict, Vec<String>) {
    let label = &pkg.package_id;
    let mut verdict = QualificationVerdict::QualifiedStable;
    let mut reasons: Vec<String> = Vec::new();

    fold_freshness_and_parity(
        label,
        pkg.freshness_state,
        &pkg.mirror_parity,
        &mut verdict,
        &mut reasons,
    );
    fold_locale_overlays(label, &pkg.locale_overlays, &mut verdict, &mut reasons);

    if !pkg.explain_apply_class.qualifies_stable() {
        verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        reasons.push(format!(
            "{label}: explain_apply_conflated: {}",
            pkg.explain_apply_class.as_str()
        ));
    }

    for step in &pkg.steps {
        if step.relies_on_coordinates_only() {
            verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
            reasons.push(format!(
                "{label}: step[{}]_coordinate_only_no_stable_target",
                step.step_id
            ));
        }
        if !step.scope_widening.qualifies_stable() {
            verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
            reasons.push(format!(
                "{label}: step[{}]_scope_widening_unnamed",
                step.step_id
            ));
        }
        if !step.explain_apply_class.qualifies_stable() {
            verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
            reasons.push(format!(
                "{label}: step[{}]_explain_apply_conflated",
                step.step_id
            ));
        }
        if !step.citation.has_citation {
            verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
            reasons.push(format!("{label}: step[{}]_no_citation", step.step_id));
        } else if !step.citation.all_anchors_live_authoritative {
            verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
            reasons.push(format!("{label}: step[{}]_citation_not_live", step.step_id));
        }
    }

    reasons.sort();
    reasons.dedup();
    (verdict, reasons)
}

// ── Manifest ──────────────────────────────────────────────────────────────────

/// The canonical manifest binding every versioned glossary pack and tour package
/// across the M5 feature families.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TourAndGlossaryPackageManifest {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque stable id for this manifest.
    pub manifest_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Schema, docs, and contract refs this manifest consumes.
    pub contract_refs: BTreeMap<String, String>,
    /// Versioned glossary packs.
    pub glossary_packs: Vec<GlossaryPack>,
    /// Versioned tour packages.
    pub tour_packages: Vec<TourPackage>,
    /// Overall derived verdict — the strictest verdict across all packages.
    pub overall_verdict: QualificationVerdict,
    /// Named narrowing reasons aggregated across packages (empty when
    /// overall_verdict is QualifiedStable).
    #[serde(default)]
    pub overall_narrowing_reasons: Vec<String>,
}

impl M5TourAndGlossaryPackageManifest {
    /// Recomputes every package verdict and the overall verdict from current
    /// evidence, writing them back.
    pub fn sync_verdicts(&mut self) {
        let mut overall = QualificationVerdict::QualifiedStable;
        let mut reasons: Vec<String> = Vec::new();
        for pack in &mut self.glossary_packs {
            pack.sync_verdict();
            overall = overall.meet(pack.verdict);
            reasons.extend(pack.narrowing_reasons.iter().cloned());
        }
        for pkg in &mut self.tour_packages {
            pkg.sync_verdict();
            overall = overall.meet(pkg.verdict);
            reasons.extend(pkg.narrowing_reasons.iter().cloned());
        }
        reasons.sort();
        reasons.dedup();
        self.overall_verdict = overall;
        self.overall_narrowing_reasons = reasons;
    }

    /// Returns the glossary pack with `pack_id`, if present.
    pub fn glossary_pack(&self, pack_id: &str) -> Option<&GlossaryPack> {
        self.glossary_packs.iter().find(|p| p.pack_id == pack_id)
    }

    /// Returns the tour package with `package_id`, if present.
    pub fn tour_package(&self, package_id: &str) -> Option<&TourPackage> {
        self.tour_packages
            .iter()
            .find(|p| p.package_id == package_id)
    }

    /// The set of every package id the manifest defines.
    pub fn known_package_ids(&self) -> BTreeSet<String> {
        self.glossary_packs
            .iter()
            .map(|p| p.pack_id.clone())
            .chain(self.tour_packages.iter().map(|p| p.package_id.clone()))
            .collect()
    }
}

/// Reopens a manifest from its exported JSON form.
///
/// This is the round-trip used to prove a package survives export and reopen
/// without losing citations or target identity: the reopened manifest is
/// structurally equal to the original, and so are its target/citation
/// fingerprints.
///
/// # Errors
///
/// Returns the underlying [`serde_json::Error`] when `json` is not a valid
/// serialized manifest.
pub fn reopen_manifest_from_json(
    json: &str,
) -> Result<M5TourAndGlossaryPackageManifest, serde_json::Error> {
    serde_json::from_str(json)
}

// ── Seeded corpus ─────────────────────────────────────────────────────────────

const GENERATED_AT: &str = "2026-06-19T13:30:00Z";

fn glossary_pack_id(family: M5LearningSurfaceFamily) -> String {
    format!("learning:m5:glossary_pack:{}:v1", family.as_str())
}

fn tour_package_id(family: M5LearningSurfaceFamily) -> String {
    format!("learning:m5:tour_package:{}:v1", family.as_str())
}

fn stable_parity(freshness: FreshnessState) -> MirrorParityPosture {
    MirrorParityPosture {
        available_offline: true,
        available_on_mirror: true,
        freshness_label: freshness.as_str().to_string(),
        explicit_freshness_disclosed: true,
        silent_dead_link_on_stale: false,
        narrowing_reason: None,
    }
}

fn local_only_parity() -> MirrorParityPosture {
    MirrorParityPosture {
        available_offline: true,
        available_on_mirror: false,
        freshness_label: FreshnessState::LocalOnlyDisclosed.as_str().to_string(),
        explicit_freshness_disclosed: true,
        silent_dead_link_on_stale: false,
        narrowing_reason: Some("learning_pack_not_yet_mirror_synced".to_string()),
    }
}

fn live_citation(commands: &[&str], anchors: &[&str]) -> CitationProof {
    CitationProof {
        has_citation: true,
        command_id_refs: commands.iter().map(|s| s.to_string()).collect(),
        docs_citation_anchor_refs: anchors.iter().map(|s| s.to_string()).collect(),
        symbol_linked_refs: vec![],
        all_anchors_live_authoritative: true,
        narrowing_reason: None,
    }
}

fn cached_citation(commands: &[&str], anchors: &[&str]) -> CitationProof {
    CitationProof {
        has_citation: true,
        command_id_refs: commands.iter().map(|s| s.to_string()).collect(),
        docs_citation_anchor_refs: anchors.iter().map(|s| s.to_string()).collect(),
        symbol_linked_refs: vec![],
        all_anchors_live_authoritative: false,
        narrowing_reason: Some("anchors_cached_not_live_authoritative".to_string()),
    }
}

fn version(family: M5LearningSurfaceFamily, kind: &str) -> PackageVersion {
    PackageVersion {
        version_ref: format!("ver:m5:{}:{}:1.0.0", kind, family.as_str()),
        revision_ref: format!("rev:m5:{}:{}:2026.06.19", kind, family.as_str()),
    }
}

/// Builds a two-locale overlay set for the given entry/step ids.
fn locale_overlays(ids: &[&str]) -> Vec<LocaleOverlay> {
    ["fr-FR", "ja-JP"]
        .into_iter()
        .map(|tag| {
            let localized_label_refs = ids
                .iter()
                .map(|id| (id.to_string(), format!("copy:{tag}:{id}")))
                .collect();
            LocaleOverlay {
                locale_tag: tag.to_string(),
                localized_label_refs,
                preserves_target_identity: true,
                preserves_citations: true,
                narrowing_reason: None,
            }
        })
        .collect()
}

struct GlossaryTerm<'a> {
    token: &'a str,
    targets: &'a [(TargetKind, &'a str)],
    commands: &'a [&'a str],
    anchors: &'a [&'a str],
    prerequisites: &'a [&'a str],
}

fn build_glossary_pack(
    family: M5LearningSurfaceFamily,
    source_class: SourceClass,
    freshness: FreshnessState,
    parity: MirrorParityPosture,
    terms: &[GlossaryTerm<'_>],
    live: bool,
) -> GlossaryPack {
    let pack_id = glossary_pack_id(family);
    let entries: Vec<GlossaryEntryRecord> = terms
        .iter()
        .map(|t| {
            let entry_id = format!("{pack_id}:entry:{}", t.token);
            let citation = if live {
                live_citation(t.commands, t.anchors)
            } else {
                cached_citation(t.commands, t.anchors)
            };
            GlossaryEntryRecord {
                entry_id: entry_id.clone(),
                term_token: t.token.to_string(),
                label_ref: format!("copy:base:{}:label", t.token),
                definition_ref: format!("copy:base:{}:definition", t.token),
                stable_targets: t
                    .targets
                    .iter()
                    .map(|(kind, id)| StableTargetRef {
                        target_kind: *kind,
                        target_id: id.to_string(),
                    })
                    .collect(),
                citation,
                prerequisite_entry_refs: t.prerequisites.iter().map(|s| s.to_string()).collect(),
            }
        })
        .collect();
    let entry_ids: Vec<&str> = entries.iter().map(|e| e.entry_id.as_str()).collect();

    let mut pack = GlossaryPack {
        record_kind: GLOSSARY_PACK_RECORD_KIND.to_string(),
        schema_version: M5_TOUR_AND_GLOSSARY_SCHEMA_VERSION,
        pack_id: pack_id.clone(),
        version: version(family, "glossary"),
        family,
        generated_at: GENERATED_AT.to_string(),
        source_class,
        source_ref: format!("source:m5:glossary:{}:v1", family.as_str()),
        freshness_state: freshness,
        mirror_parity: parity,
        base_locale: "en-US".to_string(),
        locale_overlays: locale_overlays(&entry_ids),
        prerequisite_pack_refs: vec![],
        entries,
        verdict: QualificationVerdict::QualifiedStable,
        narrowing_reasons: vec![],
    };
    pack.sync_verdict();
    pack
}

struct TourStepSpec<'a> {
    token: &'a str,
    targets: &'a [(TargetKind, &'a str)],
    commands: &'a [&'a str],
    anchors: &'a [&'a str],
    explain_apply: ExplainApplyClass,
    scope_widening: ScopeWidening,
}

fn build_tour_package(
    family: M5LearningSurfaceFamily,
    source_class: SourceClass,
    freshness: FreshnessState,
    parity: MirrorParityPosture,
    steps: &[TourStepSpec<'_>],
    live: bool,
) -> TourPackage {
    let package_id = tour_package_id(family);
    let glossary_ref = glossary_pack_id(family);
    let step_records: Vec<TourStepRecord> = steps
        .iter()
        .enumerate()
        .map(|(idx, s)| {
            let step_id = format!("{package_id}:step:{}", s.token);
            let citation = if live {
                live_citation(s.commands, s.anchors)
            } else {
                cached_citation(s.commands, s.anchors)
            };
            TourStepRecord {
                step_id: step_id.clone(),
                position_index: idx as u32,
                title_ref: format!("copy:base:{}:title", s.token),
                stable_targets: s
                    .targets
                    .iter()
                    .map(|(kind, id)| StableTargetRef {
                        target_kind: *kind,
                        target_id: id.to_string(),
                    })
                    .collect(),
                prerequisite_step_refs: vec![],
                citation,
                explain_apply_class: s.explain_apply,
                scope_widening: s.scope_widening.clone(),
                success_criteria_refs: vec![format!("criterion:{}:{}", family.as_str(), s.token)],
            }
        })
        .collect();
    let step_ids: Vec<&str> = step_records.iter().map(|s| s.step_id.as_str()).collect();

    let mut pkg = TourPackage {
        record_kind: TOUR_PACKAGE_RECORD_KIND.to_string(),
        schema_version: M5_TOUR_AND_GLOSSARY_SCHEMA_VERSION,
        package_id: package_id.clone(),
        version: version(family, "tour"),
        family,
        generated_at: GENERATED_AT.to_string(),
        source_class,
        source_ref: format!("source:m5:tour:{}:v1", family.as_str()),
        freshness_state: freshness,
        mirror_parity: parity,
        base_locale: "en-US".to_string(),
        locale_overlays: locale_overlays(&step_ids),
        glossary_pack_ref: glossary_ref.clone(),
        prerequisite_package_refs: vec![glossary_ref],
        explain_apply_class: ExplainApplyClass::ApplyRequiresApproval,
        steps: step_records,
        verdict: QualificationVerdict::QualifiedStable,
        narrowing_reasons: vec![],
    };
    pkg.sync_verdict();
    pkg
}

/// Returns the seeded tour/glossary package manifest covering every M5 feature
/// family.
///
/// Most families ship Stable, live-authoritative glossary packs and tour
/// packages with localized overlays and stable target refs. Two families
/// demonstrate the narrowing invariant honestly:
///
/// - `companion` ships from a cached revision (`cached_disclosed`), so its
///   packages narrow to Beta rather than masquerading as live.
/// - `preview` is not yet mirror-synced (`local_only_disclosed`), so its packages
///   narrow to Beta.
///
/// The `template_scaffold` tour includes a step that widens scope from a single
/// file to its folder and names the widening, proving the named-scope-widening
/// invariant on a Stable row.
pub fn seeded_m5_tour_and_glossary_packages() -> M5TourAndGlossaryPackageManifest {
    use M5LearningSurfaceFamily::*;
    use TargetKind::*;

    let mut glossary_packs = Vec::new();
    let mut tour_packages = Vec::new();

    // ── notebook ──
    glossary_packs.push(build_glossary_pack(
        Notebook,
        SourceClass::ProjectDocs,
        FreshnessState::LiveAuthoritative,
        stable_parity(FreshnessState::LiveAuthoritative),
        &[
            GlossaryTerm {
                token: "kernel_trust",
                targets: &[(CommandId, "cmd:notebook.run_cell")],
                commands: &["cmd:notebook.run_cell"],
                anchors: &["docs:anchor:notebook:kernel_trust"],
                prerequisites: &[],
            },
            GlossaryTerm {
                token: "cell_output",
                targets: &[(SurfaceObjectId, "surface:notebook:output_region")],
                commands: &["cmd:notebook.run_cell"],
                anchors: &["docs:anchor:notebook:execution_model"],
                prerequisites: &[],
            },
        ],
        true,
    ));
    tour_packages.push(build_tour_package(
        Notebook,
        SourceClass::ProjectDocs,
        FreshnessState::LiveAuthoritative,
        stable_parity(FreshnessState::LiveAuthoritative),
        &[
            TourStepSpec {
                token: "open_notebook",
                targets: &[(CommandId, "cmd:notebook.open")],
                commands: &["cmd:notebook.open"],
                anchors: &["docs:anchor:notebook:execution_model"],
                explain_apply: ExplainApplyClass::ReadOnly,
                scope_widening: ScopeWidening::none(),
            },
            TourStepSpec {
                token: "run_and_review",
                targets: &[
                    (CommandId, "cmd:notebook.run_cell"),
                    (SurfaceObjectId, "surface:notebook:output_region"),
                ],
                commands: &["cmd:notebook.run_cell"],
                anchors: &["docs:anchor:notebook:kernel_trust"],
                explain_apply: ExplainApplyClass::ApplyRequiresApproval,
                scope_widening: ScopeWidening::none(),
            },
        ],
        true,
    ));

    // ── request_workspace ──
    glossary_packs.push(build_glossary_pack(
        RequestWorkspace,
        SourceClass::ProjectDocs,
        FreshnessState::LiveAuthoritative,
        stable_parity(FreshnessState::LiveAuthoritative),
        &[GlossaryTerm {
            token: "auth_profile",
            targets: &[(DocsNodeId, "docs:node:request:auth_profiles")],
            commands: &["cmd:request.send"],
            anchors: &["docs:anchor:request:auth_profiles"],
            prerequisites: &[],
        }],
        true,
    ));
    tour_packages.push(build_tour_package(
        RequestWorkspace,
        SourceClass::ProjectDocs,
        FreshnessState::LiveAuthoritative,
        stable_parity(FreshnessState::LiveAuthoritative),
        &[TourStepSpec {
            token: "compose_and_send",
            targets: &[(CommandId, "cmd:request.send")],
            commands: &["cmd:request.send"],
            anchors: &["docs:anchor:request:auth_profiles"],
            explain_apply: ExplainApplyClass::ApplyRequiresApproval,
            scope_widening: ScopeWidening::none(),
        }],
        true,
    ));

    // ── database_workspace ──
    glossary_packs.push(build_glossary_pack(
        DatabaseWorkspace,
        SourceClass::ProjectDocs,
        FreshnessState::LiveAuthoritative,
        stable_parity(FreshnessState::LiveAuthoritative),
        &[GlossaryTerm {
            token: "statement_safety",
            targets: &[(CommandId, "cmd:database.run_statement")],
            commands: &["cmd:database.run_statement"],
            anchors: &["docs:anchor:database:statement_safety"],
            prerequisites: &[],
        }],
        true,
    ));
    tour_packages.push(build_tour_package(
        DatabaseWorkspace,
        SourceClass::ProjectDocs,
        FreshnessState::LiveAuthoritative,
        stable_parity(FreshnessState::LiveAuthoritative),
        &[TourStepSpec {
            token: "safe_run",
            targets: &[(CommandId, "cmd:database.run_statement")],
            commands: &["cmd:database.run_statement"],
            anchors: &["docs:anchor:database:statement_safety"],
            explain_apply: ExplainApplyClass::ApplyRequiresApproval,
            scope_widening: ScopeWidening::none(),
        }],
        true,
    ));

    // ── profiler_trace ──
    glossary_packs.push(build_glossary_pack(
        ProfilerTrace,
        SourceClass::SemanticGraph,
        FreshnessState::LiveAuthoritative,
        stable_parity(FreshnessState::LiveAuthoritative),
        &[GlossaryTerm {
            token: "flame_graph",
            targets: &[(GraphNodeId, "graph:node:trace:flame_graph")],
            commands: &["cmd:trace.open_flame_graph"],
            anchors: &["docs:anchor:trace:flame_graph"],
            prerequisites: &[],
        }],
        true,
    ));
    tour_packages.push(build_tour_package(
        ProfilerTrace,
        SourceClass::SemanticGraph,
        FreshnessState::LiveAuthoritative,
        stable_parity(FreshnessState::LiveAuthoritative),
        &[TourStepSpec {
            token: "capture_and_interpret",
            targets: &[
                (CommandId, "cmd:profiler.start_capture"),
                (GraphNodeId, "graph:node:trace:flame_graph"),
            ],
            commands: &["cmd:profiler.start_capture"],
            anchors: &["docs:anchor:profiler:capture_model"],
            explain_apply: ExplainApplyClass::ApplyRequiresApproval,
            scope_widening: ScopeWidening::none(),
        }],
        true,
    ));

    // ── docs_browser (mirror-synced, still Stable) ──
    glossary_packs.push(build_glossary_pack(
        DocsBrowser,
        SourceClass::MirroredOfficialDocs,
        FreshnessState::MirrorSyncedDisclosed,
        stable_parity(FreshnessState::MirrorSyncedDisclosed),
        &[GlossaryTerm {
            token: "offline_pack",
            targets: &[(DocsNodeId, "docs:node:docs_browser:offline_packs")],
            commands: &["cmd:docs.open_in_browser"],
            anchors: &["docs:anchor:docs_browser:offline_packs"],
            prerequisites: &[],
        }],
        true,
    ));
    tour_packages.push(build_tour_package(
        DocsBrowser,
        SourceClass::MirroredOfficialDocs,
        FreshnessState::MirrorSyncedDisclosed,
        stable_parity(FreshnessState::MirrorSyncedDisclosed),
        &[TourStepSpec {
            token: "open_and_cite",
            targets: &[(CommandId, "cmd:docs.open_in_browser")],
            commands: &["cmd:docs.open_in_browser"],
            anchors: &["docs:anchor:docs_browser:contract"],
            explain_apply: ExplainApplyClass::ReadOnly,
            scope_widening: ScopeWidening::none(),
        }],
        true,
    ));

    // ── preview (not yet mirror-synced → Beta) ──
    glossary_packs.push(build_glossary_pack(
        Preview,
        SourceClass::ProjectDocs,
        FreshnessState::LocalOnlyDisclosed,
        local_only_parity(),
        &[GlossaryTerm {
            token: "lineage",
            targets: &[(SurfaceObjectId, "surface:preview:lineage_trace")],
            commands: &["cmd:preview.open"],
            anchors: &["docs:anchor:preview:lineage"],
            prerequisites: &[],
        }],
        true,
    ));
    tour_packages.push(build_tour_package(
        Preview,
        SourceClass::ProjectDocs,
        FreshnessState::LocalOnlyDisclosed,
        local_only_parity(),
        &[TourStepSpec {
            token: "open_and_trace",
            targets: &[(CommandId, "cmd:preview.open")],
            commands: &["cmd:preview.open"],
            anchors: &["docs:anchor:preview:origin_model"],
            explain_apply: ExplainApplyClass::ReadOnly,
            scope_widening: ScopeWidening::none(),
        }],
        true,
    ));

    // ── template_scaffold (Stable, with a named scope-widening step) ──
    glossary_packs.push(build_glossary_pack(
        TemplateScaffold,
        SourceClass::CuratedKnowledgePack,
        FreshnessState::LiveAuthoritative,
        stable_parity(FreshnessState::LiveAuthoritative),
        &[GlossaryTerm {
            token: "planner",
            targets: &[(CommandId, "cmd:scaffold.plan")],
            commands: &["cmd:scaffold.plan"],
            anchors: &["docs:anchor:scaffold:planner_model"],
            prerequisites: &[],
        }],
        true,
    ));
    tour_packages.push(build_tour_package(
        TemplateScaffold,
        SourceClass::CuratedKnowledgePack,
        FreshnessState::LiveAuthoritative,
        stable_parity(FreshnessState::LiveAuthoritative),
        &[
            TourStepSpec {
                token: "plan",
                targets: &[(CommandId, "cmd:scaffold.plan")],
                commands: &["cmd:scaffold.plan"],
                anchors: &["docs:anchor:scaffold:planner_model"],
                explain_apply: ExplainApplyClass::ReadOnly,
                scope_widening: ScopeWidening::none(),
            },
            TourStepSpec {
                token: "review_and_apply",
                targets: &[
                    (CommandId, "cmd:scaffold.apply"),
                    (FileObjectId, "file:scaffold:target_folder"),
                ],
                commands: &["cmd:scaffold.apply"],
                anchors: &["docs:anchor:scaffold:lineage"],
                explain_apply: ExplainApplyClass::ApplyRequiresApproval,
                // Applying a scaffold widens scope from one planned file to the
                // whole target folder; this is named, not silent.
                scope_widening: ScopeWidening {
                    widens: true,
                    from_scope_ref: Some("scope:scaffold:single_planned_file".to_string()),
                    to_scope_ref: Some("scope:scaffold:target_folder".to_string()),
                    named_reason: Some("apply_writes_every_planned_file_in_folder".to_string()),
                },
            },
        ],
        true,
    ));

    // ── companion (cached revision → Beta) ──
    glossary_packs.push(build_glossary_pack(
        Companion,
        SourceClass::CuratedKnowledgePack,
        FreshnessState::CachedDisclosed,
        stable_parity(FreshnessState::CachedDisclosed),
        &[GlossaryTerm {
            token: "incident_handoff",
            targets: &[(DocsNodeId, "docs:node:incident:response_model")],
            commands: &["cmd:incident.acknowledge"],
            anchors: &["docs:anchor:incident:response_model"],
            prerequisites: &[],
        }],
        false,
    ));
    tour_packages.push(build_tour_package(
        Companion,
        SourceClass::CuratedKnowledgePack,
        FreshnessState::CachedDisclosed,
        stable_parity(FreshnessState::CachedDisclosed),
        &[TourStepSpec {
            token: "incident_flow",
            targets: &[(CommandId, "cmd:companion.open")],
            commands: &["cmd:companion.open"],
            anchors: &["docs:anchor:companion:surface_contract"],
            explain_apply: ExplainApplyClass::ApplyRequiresApproval,
            scope_widening: ScopeWidening::none(),
        }],
        false,
    ));

    // ── sync_offboarding ──
    glossary_packs.push(build_glossary_pack(
        SyncOffboarding,
        SourceClass::ProjectDocs,
        FreshnessState::LiveAuthoritative,
        stable_parity(FreshnessState::LiveAuthoritative),
        &[GlossaryTerm {
            token: "retention",
            targets: &[(DocsNodeId, "docs:node:sync:retention_model")],
            commands: &["cmd:sync.status"],
            anchors: &["docs:anchor:sync:retention_model"],
            prerequisites: &[],
        }],
        true,
    ));
    tour_packages.push(build_tour_package(
        SyncOffboarding,
        SourceClass::ProjectDocs,
        FreshnessState::LiveAuthoritative,
        stable_parity(FreshnessState::LiveAuthoritative),
        &[TourStepSpec {
            token: "export_and_offboard",
            targets: &[(CommandId, "cmd:offboarding.export_bundle")],
            commands: &["cmd:offboarding.export_bundle"],
            anchors: &["docs:anchor:offboarding:export_and_destroy"],
            explain_apply: ExplainApplyClass::ApplyRequiresApproval,
            scope_widening: ScopeWidening::none(),
        }],
        true,
    ));

    let mut contract_refs = BTreeMap::new();
    contract_refs.insert(
        "tour_and_glossary_schema".to_string(),
        M5_TOUR_AND_GLOSSARY_SCHEMA_REF.to_string(),
    );
    contract_refs.insert(
        "tour_package_contract_schema".to_string(),
        TOUR_PACKAGE_CONTRACT_SCHEMA_REF.to_string(),
    );
    contract_refs.insert(
        "artifact_doc".to_string(),
        M5_TOUR_AND_GLOSSARY_ARTIFACT_REF.to_string(),
    );
    contract_refs.insert(
        "public_doc".to_string(),
        M5_TOUR_AND_GLOSSARY_DOC_REF.to_string(),
    );
    contract_refs.insert(
        "canonical_fixture".to_string(),
        M5_TOUR_AND_GLOSSARY_FIXTURE_REF.to_string(),
    );

    let mut manifest = M5TourAndGlossaryPackageManifest {
        record_kind: M5_TOUR_AND_GLOSSARY_MANIFEST_RECORD_KIND.to_string(),
        schema_version: M5_TOUR_AND_GLOSSARY_SCHEMA_VERSION,
        manifest_id: "m5-tour-and-glossary-packages:manifest:2026.06.19-01".to_string(),
        generated_at: GENERATED_AT.to_string(),
        contract_refs,
        glossary_packs,
        tour_packages,
        overall_verdict: QualificationVerdict::QualifiedStable,
        overall_narrowing_reasons: vec![],
    };
    manifest.sync_verdicts();
    manifest
}

// ── Validation ────────────────────────────────────────────────────────────────

/// A typed validation error from [`validate_m5_tour_and_glossary_packages`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TourAndGlossaryValidationError {
    /// Opaque id of the package or record that failed.
    pub subject_id: String,
    /// Human-readable description of the failure.
    pub message: String,
}

impl std::fmt::Display for TourAndGlossaryValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.subject_id, self.message)
    }
}

/// Validates a manifest against the tour/glossary package invariants.
///
/// # Errors
///
/// Returns a non-empty `Vec` when any package's stored verdict diverges from the
/// verdict derived from its evidence; when a non-live freshness state is not
/// disclosed or disagrees with the mirror-parity label; when a glossary entry has
/// no stable target; when a tour step relies on coordinates alone or widens scope
/// without naming it; when a locale overlay fails to preserve target identity or
/// citations; when a bundle silently dead-links offline or on a mirror; or when a
/// prerequisite ref into the manifest's own namespace fails to resolve or forms a
/// cycle.
pub fn validate_m5_tour_and_glossary_packages(
    manifest: &M5TourAndGlossaryPackageManifest,
) -> Result<(), Vec<TourAndGlossaryValidationError>> {
    let mut errors: Vec<TourAndGlossaryValidationError> = Vec::new();
    let known_ids = manifest.known_package_ids();

    for pack in &manifest.glossary_packs {
        let subject = pack.pack_id.clone();

        let (derived, _) = derive_glossary_pack_verdict(pack);
        if derived != pack.verdict {
            errors.push(TourAndGlossaryValidationError {
                subject_id: subject.clone(),
                message: format!(
                    "stored verdict {:?} diverges from derived {:?}",
                    pack.verdict, derived
                ),
            });
        }

        check_freshness_parity(
            &subject,
            pack.freshness_state,
            &pack.mirror_parity,
            &mut errors,
        );
        check_locale_overlays(&subject, &pack.locale_overlays, &mut errors);
        check_prerequisites(
            &subject,
            &pack.prerequisite_pack_refs,
            &known_ids,
            &mut errors,
        );

        if pack.entries.is_empty() {
            errors.push(TourAndGlossaryValidationError {
                subject_id: subject.clone(),
                message: "glossary pack has no entries".to_string(),
            });
        }
        for entry in &pack.entries {
            if entry.stable_targets.is_empty() {
                errors.push(TourAndGlossaryValidationError {
                    subject_id: entry.entry_id.clone(),
                    message: "glossary entry has no stable target".to_string(),
                });
            }
            if !entry.citation.has_citation {
                errors.push(TourAndGlossaryValidationError {
                    subject_id: entry.entry_id.clone(),
                    message: "glossary entry is not citation-backed".to_string(),
                });
            }
        }
    }

    for pkg in &manifest.tour_packages {
        let subject = pkg.package_id.clone();

        let (derived, _) = derive_tour_package_verdict(pkg);
        if derived != pkg.verdict {
            errors.push(TourAndGlossaryValidationError {
                subject_id: subject.clone(),
                message: format!(
                    "stored verdict {:?} diverges from derived {:?}",
                    pkg.verdict, derived
                ),
            });
        }

        check_freshness_parity(
            &subject,
            pkg.freshness_state,
            &pkg.mirror_parity,
            &mut errors,
        );
        check_locale_overlays(&subject, &pkg.locale_overlays, &mut errors);
        check_prerequisites(
            &subject,
            &pkg.prerequisite_package_refs,
            &known_ids,
            &mut errors,
        );

        // The declared glossary pack must resolve.
        if manifest.glossary_pack(&pkg.glossary_pack_ref).is_none() {
            errors.push(TourAndGlossaryValidationError {
                subject_id: subject.clone(),
                message: format!(
                    "tour package references unknown glossary pack {}",
                    pkg.glossary_pack_ref
                ),
            });
        }

        if pkg.explain_apply_class == ExplainApplyClass::Conflated {
            errors.push(TourAndGlossaryValidationError {
                subject_id: subject.clone(),
                message: "tour package conflates explain/apply".to_string(),
            });
        }
        if pkg.steps.is_empty() {
            errors.push(TourAndGlossaryValidationError {
                subject_id: subject.clone(),
                message: "tour package has no steps".to_string(),
            });
        }
        for step in &pkg.steps {
            if step.relies_on_coordinates_only() {
                errors.push(TourAndGlossaryValidationError {
                    subject_id: step.step_id.clone(),
                    message: "tour step relies on coordinates alone (no stable target)".to_string(),
                });
            }
            if !step.scope_widening.qualifies_stable() {
                errors.push(TourAndGlossaryValidationError {
                    subject_id: step.step_id.clone(),
                    message: "tour step widens scope without naming from/to scope and reason"
                        .to_string(),
                });
            }
            if step.explain_apply_class == ExplainApplyClass::Conflated {
                errors.push(TourAndGlossaryValidationError {
                    subject_id: step.step_id.clone(),
                    message: "tour step conflates explain/apply".to_string(),
                });
            }
        }
    }

    if let Some(cycle) = detect_prerequisite_cycle(manifest) {
        errors.push(TourAndGlossaryValidationError {
            subject_id: cycle,
            message: "prerequisite cycle detected".to_string(),
        });
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn check_freshness_parity(
    subject: &str,
    freshness: FreshnessState,
    parity: &MirrorParityPosture,
    errors: &mut Vec<TourAndGlossaryValidationError>,
) {
    if parity.freshness_label != freshness.as_str() {
        errors.push(TourAndGlossaryValidationError {
            subject_id: subject.to_string(),
            message: format!(
                "freshness state {} disagrees with mirror-parity label {}",
                freshness.as_str(),
                parity.freshness_label
            ),
        });
    }
    if freshness.requires_disclosure() && !parity.explicit_freshness_disclosed {
        errors.push(TourAndGlossaryValidationError {
            subject_id: subject.to_string(),
            message: format!(
                "non-live freshness {} is not explicitly disclosed (would masquerade as live)",
                freshness.as_str()
            ),
        });
    }
    if parity.silent_dead_link_on_stale {
        errors.push(TourAndGlossaryValidationError {
            subject_id: subject.to_string(),
            message: "package shows a silent dead link when stale/offline".to_string(),
        });
    }
}

fn check_locale_overlays(
    subject: &str,
    overlays: &[LocaleOverlay],
    errors: &mut Vec<TourAndGlossaryValidationError>,
) {
    let mut seen = BTreeSet::new();
    for overlay in overlays {
        if !seen.insert(overlay.locale_tag.clone()) {
            errors.push(TourAndGlossaryValidationError {
                subject_id: subject.to_string(),
                message: format!("duplicate locale overlay {}", overlay.locale_tag),
            });
        }
        if !overlay.preserves_target_identity {
            errors.push(TourAndGlossaryValidationError {
                subject_id: subject.to_string(),
                message: format!(
                    "locale overlay {} drops target identity",
                    overlay.locale_tag
                ),
            });
        }
        if !overlay.preserves_citations {
            errors.push(TourAndGlossaryValidationError {
                subject_id: subject.to_string(),
                message: format!("locale overlay {} drops citations", overlay.locale_tag),
            });
        }
    }
}

fn check_prerequisites(
    subject: &str,
    prerequisite_refs: &[String],
    known_ids: &BTreeSet<String>,
    errors: &mut Vec<TourAndGlossaryValidationError>,
) {
    // Prerequisite refs that use the manifest's own id namespace MUST resolve to
    // a present package; external refs (a different namespace) are allowed.
    for prereq in prerequisite_refs {
        if prereq.starts_with("learning:m5:") && !known_ids.contains(prereq) {
            errors.push(TourAndGlossaryValidationError {
                subject_id: subject.to_string(),
                message: format!("unresolved prerequisite {prereq}"),
            });
        }
    }
}

/// Detects a cycle in the in-manifest prerequisite graph, returning the id of a
/// node on the cycle when one exists.
fn detect_prerequisite_cycle(manifest: &M5TourAndGlossaryPackageManifest) -> Option<String> {
    let mut edges: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let known = manifest.known_package_ids();
    for pack in &manifest.glossary_packs {
        edges.insert(
            pack.pack_id.clone(),
            pack.prerequisite_pack_refs
                .iter()
                .filter(|r| known.contains(*r))
                .cloned()
                .collect(),
        );
    }
    for pkg in &manifest.tour_packages {
        edges.insert(
            pkg.package_id.clone(),
            pkg.prerequisite_package_refs
                .iter()
                .filter(|r| known.contains(*r))
                .cloned()
                .collect(),
        );
    }

    // Iterative DFS with white/grey/black coloring.
    #[derive(Clone, Copy, PartialEq)]
    enum Color {
        White,
        Grey,
        Black,
    }
    let mut color: BTreeMap<String, Color> =
        edges.keys().map(|k| (k.clone(), Color::White)).collect();

    for start in edges.keys() {
        if color[start] != Color::White {
            continue;
        }
        let mut stack: Vec<(String, usize)> = vec![(start.clone(), 0)];
        color.insert(start.clone(), Color::Grey);
        while let Some((node, idx)) = stack.last().cloned() {
            let neighbors = edges.get(&node).cloned().unwrap_or_default();
            if idx < neighbors.len() {
                stack.last_mut().unwrap().1 += 1;
                let next = neighbors[idx].clone();
                match color.get(&next).copied().unwrap_or(Color::Black) {
                    Color::Grey => return Some(next),
                    Color::White => {
                        color.insert(next.clone(), Color::Grey);
                        stack.push((next, 0));
                    }
                    Color::Black => {}
                }
            } else {
                color.insert(node.clone(), Color::Black);
                stack.pop();
            }
        }
    }
    None
}

#[cfg(test)]
mod tests;
