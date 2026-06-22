//! Support / export / provider-debug packet for editor-assist decisions.
//!
//! The canonical [editor-assist matrix](crate::m5_editor_assist) freezes the
//! *contract*: which assist channel is offered, narrowed, or blocked on each
//! surface, and the minimum fields each micro-surface record must carry into a
//! support export. This module produces the *realized support records* that
//! satisfy that contract for the four decisions support actually has to explain —
//! **completion, hint, hover, and peek** — so a user or operator can answer:
//!
//! - *why did this hint appear, and from which provider or fallback path?*
//! - *why is this completion list cached / local-word fallback instead of
//!   deterministic language intelligence?*
//! - *why did this hover or peek open on an approximate (wrong) anchor?*
//! - *why was this hover stale, or this completion pending?*
//! - *why did this surface narrow or block assist, and what is the next safe
//!   action?*
//!
//! Each [`AssistDecisionRecord`] pins, with stable ids, the originating
//! micro-surface identity ([`AssistDecisionRecord::subject_ref`]), the provider /
//! source path (provider id, [`AssistSourceLabelClass`], and
//! [`CompletionProviderPosture`]), the degraded / blocked reason
//! ([`AssistDegradeClass`] + [`NarrowReasonClass`]), the partial-index and
//! stale-doc state ([`IndexFreshnessClass`] + [`AssistContentStateClass`]), the
//! anchor mapping quality ([`MappingQualityClass`]), any additional-edit / import
//! side-effect cue ([`AdditionalEditCue`]), and a single derived **drift class**
//! ([`AssistDriftClass`]) — the one explainability category an operator filters by.
//!
//! On top of the records the packet projects a Project-Doctor-style view: a
//! per-drift-class rollup ([`AssistDriftRollup`]) and a per-surface rollup
//! ([`AssistSurfaceRollup`]), each carrying a stable correlation field id so the
//! same answer is reachable from a support bundle, local diagnostics, and the CLI
//! without a screenshot. Downstream Project Doctor and support-export surfaces
//! should consume this packet directly rather than inventing a second view of
//! completion / hint / hover / peek behavior.
//!
//! The packet is **redaction-safe by construction**: every field is a typed token,
//! an opaque ref, a count, or a bounded metadata sentence. It carries no source
//! text, prompt context, provider payloads, or credential bodies, and the
//! [`SupportExportContract`] names exactly the classes that never cross the
//! boundary. The build is deterministic and self-contained:
//! [`assist_support_packet`] seeds one frozen corpus, derives the rollups, and
//! evaluates every [`AssistSupportInvariant`] over its own data, so a structural
//! regression flips an invariant to `holds = false` rather than silently shipping.
//! The checked-in fixture plus the replay gate freeze the packet byte-for-byte.

use serde::{Deserialize, Serialize};

use crate::assist::AssistSourceLabelClass;
use crate::m5_completion_rows::{AdditionalEditCue, CompletionProviderPosture};
use crate::m5_constrained_assist::{NarrowReasonClass, NextSafeActionClass};
use crate::m5_editor_assist::{AssistDegradeClass, EditorSurfaceClass, MicroSurfaceKind};
use crate::m5_hover_peek::MappingQualityClass;

/// Schema version for the assist-support packet.
pub const M5_ASSIST_SUPPORT_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the assist-support packet.
pub const M5_ASSIST_SUPPORT_SCHEMA_REF: &str = "schemas/editor/m5-assist-support.schema.json";

/// Stable record-kind tag for the assist-support packet.
pub const M5_ASSIST_SUPPORT_RECORD_KIND: &str = "m5_assist_support_packet";

/// Stable id for the canonical assist-support packet.
pub const M5_ASSIST_SUPPORT_PACKET_ID: &str = "m5-assist-support:packet:0001";

/// Capture stamp for the canonical packet. Held as a constant so the projection
/// stays deterministic and the fixture freezes byte-for-byte.
pub const M5_ASSIST_SUPPORT_AS_OF: &str = "2026-06-22T00:00:00Z";

// ---------------------------------------------------------------------------
// Decision kind.
// ---------------------------------------------------------------------------

/// The editor-assist decisions this packet makes supportable. Scoped to the four
/// the spec names — completion, hint, hover, peek — so the explainer stays inside
/// support / provenance for those surfaces rather than becoming a generic editor
/// debug console.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistDecisionKind {
    /// A completion-list decision.
    Completion,
    /// A code-lens / inlay-hint decision.
    Hint,
    /// A hover / quick-info decision.
    Hover,
    /// A peek (definition / references / implementations / type / call-hierarchy)
    /// decision.
    Peek,
}

impl AssistDecisionKind {
    /// All decision kinds, in catalog order.
    pub const ALL: [Self; 4] = [Self::Completion, Self::Hint, Self::Hover, Self::Peek];

    /// Returns the stable schema token for this kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Completion => "completion",
            Self::Hint => "hint",
            Self::Hover => "hover",
            Self::Peek => "peek",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Completion => "Completion",
            Self::Hint => "Hint",
            Self::Hover => "Hover",
            Self::Peek => "Peek",
        }
    }

    /// Stable id prefix every decision record of this kind uses.
    pub const fn id_prefix(self) -> &'static str {
        match self {
            Self::Completion => "completion-decision:",
            Self::Hint => "hint-decision:",
            Self::Hover => "hover-decision:",
            Self::Peek => "peek-decision:",
        }
    }

    /// The originating micro-surface record kind whose identity a decision of this
    /// kind references, reusing the matrix identity vocabulary.
    pub const fn subject_kind(self) -> MicroSurfaceKind {
        match self {
            Self::Completion => MicroSurfaceKind::CompletionSession,
            Self::Hint => MicroSurfaceKind::HintDescriptor,
            Self::Hover | Self::Peek => MicroSurfaceKind::HoverPeekCard,
        }
    }
}

// ---------------------------------------------------------------------------
// Index freshness + content state.
// ---------------------------------------------------------------------------

/// Freshness of the semantic index backing a decision, surfaced so a partial /
/// rebuilding index is never confused with a fully-resolved one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IndexFreshnessClass {
    /// The semantic index is fully built for this file.
    Fresh,
    /// The semantic index is still building, so semantic results are partial.
    PartialBuilding,
    /// The index is built but a reindex is pending after an external change.
    ReindexPending,
    /// The file is not semantically indexed (large-file / restricted load).
    NotIndexed,
}

impl IndexFreshnessClass {
    /// All freshness classes, in catalog order.
    pub const ALL: [Self; 4] = [
        Self::Fresh,
        Self::PartialBuilding,
        Self::ReindexPending,
        Self::NotIndexed,
    ];

    /// Returns the stable schema token for this class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::PartialBuilding => "partial_building",
            Self::ReindexPending => "reindex_pending",
            Self::NotIndexed => "not_indexed",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Fresh => "Index fresh",
            Self::PartialBuilding => "Index building — partial",
            Self::ReindexPending => "Reindex pending",
            Self::NotIndexed => "Not indexed",
        }
    }

    /// Returns true when the index is fully built and authoritative.
    pub const fn is_fresh(self) -> bool {
        matches!(self, Self::Fresh)
    }
}

/// The content posture a decision reflects, so a stale, pending, snapshot, or
/// fallback result is never reported as a live authoritative one. Generalizes the
/// hover/peek inline-state vocabulary across all four decision kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistContentStateClass {
    /// Live, authoritative content from the current provider.
    Live,
    /// A previous result shown while a refresh is pending.
    StaleRefreshPending,
    /// Partial content while the semantic index is still building.
    PartialIndexPending,
    /// An imported snapshot rather than a live read (e.g. generated output).
    ImportedSnapshot,
    /// A labeled fallback from a different provider than the authoritative one.
    FallbackProvider,
    /// Content narrowed by policy (protected / restricted path).
    PolicyLimited,
    /// Suppressed because the file is in large-file / restricted mode.
    Suppressed,
}

impl AssistContentStateClass {
    /// All content states, in catalog order.
    pub const ALL: [Self; 7] = [
        Self::Live,
        Self::StaleRefreshPending,
        Self::PartialIndexPending,
        Self::ImportedSnapshot,
        Self::FallbackProvider,
        Self::PolicyLimited,
        Self::Suppressed,
    ];

    /// Returns the stable schema token for this state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::StaleRefreshPending => "stale_refresh_pending",
            Self::PartialIndexPending => "partial_index_pending",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::FallbackProvider => "fallback_provider",
            Self::PolicyLimited => "policy_limited",
            Self::Suppressed => "suppressed",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Live => "Live",
            Self::StaleRefreshPending => "Stale — refresh pending",
            Self::PartialIndexPending => "Partial — index building",
            Self::ImportedSnapshot => "Imported snapshot",
            Self::FallbackProvider => "Fallback provider",
            Self::PolicyLimited => "Policy-limited",
            Self::Suppressed => "Suppressed",
        }
    }

    /// Returns true when the content is live and authoritative.
    pub const fn is_live(self) -> bool {
        matches!(self, Self::Live)
    }
}

// ---------------------------------------------------------------------------
// Drift class — the explainability category.
// ---------------------------------------------------------------------------

/// The single explainability category a decision falls into: the headline answer
/// to "why did this assist surface behave this way?". One closed vocabulary so
/// support, diagnostics, and the CLI filter assist issues the same way.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistDriftClass {
    /// No drift: an authoritative provider answered with an exact anchor and live
    /// content. The baseline the other classes are explained against.
    NoDriftAuthoritative,
    /// The preferred provider drifted to a degraded posture, so a fallback path
    /// answered instead of the authoritative one.
    ProviderDrift,
    /// The result came from a cached or local-word / lexical fallback rather than
    /// deterministic language intelligence.
    CachedLocalWordFallback,
    /// The decision mapped to an approximate / heuristic / unresolved anchor rather
    /// than an exact symbol, so the target may be wrong.
    WrongAnchorMapping,
    /// A constrained-file state (generated, protected, projection, restricted)
    /// narrowed or blocked the decision.
    ConstrainedFileNarrowing,
    /// Semantic results are pending while the index finishes building.
    PartialIndexPending,
    /// The decision reflects a stale or imported-snapshot doc rather than a live
    /// read.
    StaleDocSnapshot,
}

impl AssistDriftClass {
    /// All drift classes, in catalog order.
    pub const ALL: [Self; 7] = [
        Self::NoDriftAuthoritative,
        Self::ProviderDrift,
        Self::CachedLocalWordFallback,
        Self::WrongAnchorMapping,
        Self::ConstrainedFileNarrowing,
        Self::PartialIndexPending,
        Self::StaleDocSnapshot,
    ];

    /// Returns the stable schema token for this class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoDriftAuthoritative => "no_drift_authoritative",
            Self::ProviderDrift => "provider_drift",
            Self::CachedLocalWordFallback => "cached_local_word_fallback",
            Self::WrongAnchorMapping => "wrong_anchor_mapping",
            Self::ConstrainedFileNarrowing => "constrained_file_narrowing",
            Self::PartialIndexPending => "partial_index_pending",
            Self::StaleDocSnapshot => "stale_doc_snapshot",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NoDriftAuthoritative => "Authoritative — no drift",
            Self::ProviderDrift => "Provider drift",
            Self::CachedLocalWordFallback => "Cached / local-word fallback",
            Self::WrongAnchorMapping => "Wrong-anchor mapping",
            Self::ConstrainedFileNarrowing => "Constrained-file narrowing",
            Self::PartialIndexPending => "Partial-index pending",
            Self::StaleDocSnapshot => "Stale / snapshot doc",
        }
    }

    /// One-line summary of what this class means for support.
    pub const fn summary(self) -> &'static str {
        match self {
            Self::NoDriftAuthoritative => {
                "An authoritative provider answered with an exact anchor and live content."
            }
            Self::ProviderDrift => {
                "The preferred provider drifted to a degraded posture; a fallback path answered."
            }
            Self::CachedLocalWordFallback => {
                "A cached or local-word / lexical fallback answered instead of deterministic language intelligence."
            }
            Self::WrongAnchorMapping => {
                "The decision resolved an approximate or heuristic anchor, so the target may be wrong."
            }
            Self::ConstrainedFileNarrowing => {
                "A constrained-file state narrowed or blocked the decision; writes or assist route elsewhere."
            }
            Self::PartialIndexPending => {
                "Semantic results are pending while the index finishes building."
            }
            Self::StaleDocSnapshot => {
                "The decision reflects a stale or imported-snapshot doc rather than a live read."
            }
        }
    }

    /// Returns true when this is the clean, fully-authoritative baseline.
    pub const fn is_clean(self) -> bool {
        matches!(self, Self::NoDriftAuthoritative)
    }
}

/// A closed-vocabulary catalog entry for the drift classes, so consumers render
/// the same tokens, labels, and summaries instead of forking copy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriftDescriptor {
    /// Stable drift-class token.
    pub class_token: String,
    /// Human-readable label.
    pub label: String,
    /// What the class means for support.
    pub summary: String,
}

// ---------------------------------------------------------------------------
// Decision record.
// ---------------------------------------------------------------------------

/// One supportable, redaction-safe assist decision: the full provenance of a
/// single completion / hint / hover / peek outcome, with a stable id and field id
/// so it correlates from a support bundle, local diagnostics, or the CLI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistDecisionRecord {
    /// Stable, kind-prefixed id for this decision record.
    pub decision_id: String,
    /// Stable dotted correlation field id (`assist_support.<kind>.<drift_class>`)
    /// operators filter and group by.
    pub field_id: String,
    /// Which assist decision this record explains.
    pub kind: AssistDecisionKind,
    /// The editor surface the decision happened on (matrix surface vocabulary).
    pub surface_class: EditorSurfaceClass,
    /// Opaque ref to the originating micro-surface record (session / card) whose
    /// identity this decision preserves. Carries no payload.
    pub subject_ref: String,
    /// Opaque provider identity (e.g. `provider:lsp.rust-analyzer`). A name, never
    /// a credential or payload.
    pub provider_id: String,
    /// Visible source-label class for the answering path.
    pub source_label_class: AssistSourceLabelClass,
    /// Provider posture surfaced with the decision.
    pub provider_posture: CompletionProviderPosture,
    /// Degraded-state class the decision was narrowed to (matrix vocabulary).
    pub degrade_state: AssistDegradeClass,
    /// Why the decision was narrowed or blocked, when it was.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrow_reason: Option<NarrowReasonClass>,
    /// How well the source anchor mapped to the resolved target.
    pub mapping_quality: MappingQualityClass,
    /// The content posture the decision reflects (live / stale / pending / …).
    pub content_state: AssistContentStateClass,
    /// Freshness of the semantic index backing the decision.
    pub index_freshness: IndexFreshnessClass,
    /// Additional-edit / import side-effect cue an acceptance would carry.
    pub side_effect_cue: AdditionalEditCue,
    /// The single derived explainability category for this decision.
    pub drift_class: AssistDriftClass,
    /// The next safe action offered to narrow or resolve the issue, when drifted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_safe_action: Option<NextSafeActionClass>,
    /// Canonical command id for [`Self::next_safe_action`], when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_action_command: Option<String>,
    /// Bounded, metadata-only explanation of the decision. No source text or
    /// prompt context.
    pub explanation: String,
    /// Whether this record is metadata-only and safe to export.
    pub redaction_safe: bool,
}

impl AssistDecisionRecord {
    /// Returns true when the decision is the clean, fully-authoritative baseline.
    pub fn is_clean(&self) -> bool {
        self.drift_class.is_clean()
    }
}

// ---------------------------------------------------------------------------
// Rollups (Project Doctor view).
// ---------------------------------------------------------------------------

/// Per-drift-class rollup: how many decisions fell into one explainability
/// category and which kinds and surfaces they touched.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistDriftRollup {
    /// The drift class this rollup aggregates.
    pub drift_class: AssistDriftClass,
    /// Stable correlation field id (`assist_support.drift.<token>`).
    pub field_id: String,
    /// Number of decisions in this class.
    pub count: usize,
    /// Decision kinds observed in this class, in catalog order.
    pub affected_kinds: Vec<AssistDecisionKind>,
    /// Surfaces observed in this class, in matrix order.
    pub affected_surfaces: Vec<EditorSurfaceClass>,
    /// What this class means for support.
    pub explanation: String,
}

/// Per-surface rollup: how many assist decisions on one surface stayed
/// authoritative versus drifted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistSurfaceRollup {
    /// The surface this rollup aggregates.
    pub surface_class: EditorSurfaceClass,
    /// Stable correlation field id (`assist_support.surface.<token>`).
    pub field_id: String,
    /// Total decisions observed on this surface.
    pub count: usize,
    /// Authoritative, no-drift decisions on this surface.
    pub clean_count: usize,
    /// Drifted decisions on this surface.
    pub drifted_count: usize,
}

// ---------------------------------------------------------------------------
// Support-export contract.
// ---------------------------------------------------------------------------

/// The redaction contract for the packet: exactly which fields cross the export
/// boundary and which evidence classes never do.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportContract {
    /// Export record-kind tag.
    pub record_kind: String,
    /// The per-decision fields exported into a support bundle.
    pub exported_fields: Vec<String>,
    /// Evidence classes that are never exported by default.
    pub redacted_classes: Vec<String>,
    /// Whether the export excludes raw payloads / credential bodies.
    pub raw_payload_excluded: bool,
    /// What the export carries and what it must never carry.
    pub note: String,
}

// ---------------------------------------------------------------------------
// Invariant.
// ---------------------------------------------------------------------------

/// One frozen invariant the packet must satisfy, with the result of evaluating it
/// over the packet's own data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistSupportInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// Human-readable statement of the invariant.
    pub statement: String,
    /// Whether the invariant holds on the built packet.
    pub holds: bool,
}

// ---------------------------------------------------------------------------
// Top-level packet.
// ---------------------------------------------------------------------------

/// The canonical, frozen, export-safe assist-support packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistSupportPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_assist_support_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Capture stamp.
    pub as_of: String,
    /// The closed drift-class vocabulary.
    pub drift_catalog: Vec<DriftDescriptor>,
    /// The seeded, frozen corpus of supportable assist decisions.
    pub decisions: Vec<AssistDecisionRecord>,
    /// Per-drift-class rollups (Project Doctor view).
    pub drift_rollups: Vec<AssistDriftRollup>,
    /// Per-surface rollups (Project Doctor view).
    pub surface_rollups: Vec<AssistSurfaceRollup>,
    /// The redaction / support-export contract.
    pub support_export: SupportExportContract,
    /// Frozen invariants and whether each holds on this packet.
    pub invariants: Vec<AssistSupportInvariant>,
    /// Whether the packet is metadata-safe for support export.
    pub raw_payload_excluded: bool,
    /// Human-readable summary.
    pub summary: String,
}

impl AssistSupportPacket {
    /// Returns true when every frozen invariant holds on this packet.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|invariant| invariant.holds)
    }

    /// Returns true when the packet is metadata-safe for support export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.support_export.raw_payload_excluded
            && self.schema_ref == M5_ASSIST_SUPPORT_SCHEMA_REF
            && self.record_kind == M5_ASSIST_SUPPORT_RECORD_KIND
            && self
                .decisions
                .iter()
                .all(|decision| decision.redaction_safe)
    }

    /// Returns the decisions of a given kind, in corpus order.
    pub fn decisions_for_kind(
        &self,
        kind: AssistDecisionKind,
    ) -> impl Iterator<Item = &AssistDecisionRecord> {
        self.decisions
            .iter()
            .filter(move |decision| decision.kind == kind)
    }

    /// Returns the decisions on a given surface, in corpus order.
    pub fn decisions_for_surface(
        &self,
        surface: EditorSurfaceClass,
    ) -> impl Iterator<Item = &AssistDecisionRecord> {
        self.decisions
            .iter()
            .filter(move |decision| decision.surface_class == surface)
    }

    /// Returns the rollup for a drift class, when present.
    pub fn drift_rollup(&self, drift_class: AssistDriftClass) -> Option<&AssistDriftRollup> {
        self.drift_rollups
            .iter()
            .find(|rollup| rollup.drift_class == drift_class)
    }
}

// ---------------------------------------------------------------------------
// Seed corpus.
// ---------------------------------------------------------------------------

/// The discriminating evidence for one seeded decision. Ids, field ids, command
/// ids, and the explanation are derived from this so the corpus stays consistent.
struct Seed {
    kind: AssistDecisionKind,
    surface: EditorSurfaceClass,
    provider_id: &'static str,
    source: AssistSourceLabelClass,
    posture: CompletionProviderPosture,
    degrade: AssistDegradeClass,
    narrow_reason: Option<NarrowReasonClass>,
    mapping: MappingQualityClass,
    content: AssistContentStateClass,
    index: IndexFreshnessClass,
    side_effect: AdditionalEditCue,
    drift: AssistDriftClass,
    next: Option<NextSafeActionClass>,
}

/// The frozen seed corpus. One representative decision per
/// kind × surface × drift combination worth explaining, spanning every claimed
/// constrained surface so the shared model is proved across surfaces.
fn seed_corpus() -> Vec<Seed> {
    use AssistContentStateClass as C;
    use AssistDecisionKind as K;
    use AssistDegradeClass as D;
    use AssistDriftClass as Drift;
    use AssistSourceLabelClass as Src;
    use CompletionProviderPosture as P;
    use EditorSurfaceClass as S;
    use IndexFreshnessClass as I;
    use MappingQualityClass as M;
    use NarrowReasonClass as N;
    use NextSafeActionClass as A;

    vec![
        // Clean baselines, one per kind.
        Seed {
            kind: K::Completion,
            surface: S::CodeFile,
            provider_id: "provider:lsp.rust-analyzer",
            source: Src::DeterministicLanguage,
            posture: P::FullSemantic,
            degrade: D::FullFidelity,
            narrow_reason: None,
            mapping: M::Exact,
            content: C::Live,
            index: I::Fresh,
            side_effect: AdditionalEditCue::None,
            drift: Drift::NoDriftAuthoritative,
            next: None,
        },
        Seed {
            kind: K::Hint,
            surface: S::CodeFile,
            provider_id: "provider:lsp.rust-analyzer",
            source: Src::DeterministicLanguage,
            posture: P::FullSemantic,
            degrade: D::FullFidelity,
            narrow_reason: None,
            mapping: M::Exact,
            content: C::Live,
            index: I::Fresh,
            side_effect: AdditionalEditCue::None,
            drift: Drift::NoDriftAuthoritative,
            next: None,
        },
        Seed {
            kind: K::Hover,
            surface: S::CodeFile,
            provider_id: "provider:lsp.rust-analyzer",
            source: Src::DeterministicLanguage,
            posture: P::FullSemantic,
            degrade: D::FullFidelity,
            narrow_reason: None,
            mapping: M::Exact,
            content: C::Live,
            index: I::Fresh,
            side_effect: AdditionalEditCue::None,
            drift: Drift::NoDriftAuthoritative,
            next: None,
        },
        Seed {
            kind: K::Peek,
            surface: S::CodeFile,
            provider_id: "provider:lsp.rust-analyzer",
            source: Src::DeterministicLanguage,
            posture: P::FullSemantic,
            degrade: D::FullFidelity,
            narrow_reason: None,
            mapping: M::Exact,
            content: C::Live,
            index: I::Fresh,
            side_effect: AdditionalEditCue::None,
            drift: Drift::NoDriftAuthoritative,
            next: None,
        },
        // Provider drift: preferred provider degraded, fallback answered.
        Seed {
            kind: K::Completion,
            surface: S::CodeFile,
            provider_id: "provider:lsp.rust-analyzer",
            source: Src::CachedFallback,
            posture: P::DegradedProvider,
            degrade: D::SourceLabeledFallback,
            narrow_reason: Some(N::ProviderDegradedFallback),
            mapping: M::Exact,
            content: C::FallbackProvider,
            index: I::ReindexPending,
            side_effect: AdditionalEditCue::None,
            drift: Drift::ProviderDrift,
            next: Some(A::ReconnectProvider),
        },
        // Cached / local-word fallback in large-file mode.
        Seed {
            kind: K::Completion,
            surface: S::LargeFileRestricted,
            provider_id: "provider:lexical.local-word",
            source: Src::CachedFallback,
            posture: P::LargeFileFallback,
            degrade: D::SuppressedLargeFile,
            narrow_reason: Some(N::SuppressedForSafety),
            mapping: M::Heuristic,
            content: C::Suppressed,
            index: I::NotIndexed,
            side_effect: AdditionalEditCue::None,
            drift: Drift::CachedLocalWordFallback,
            next: Some(A::OpenInFullEditor),
        },
        // Wrong-anchor mapping on a peek.
        Seed {
            kind: K::Peek,
            surface: S::CodeFile,
            provider_id: "provider:lsp.rust-analyzer",
            source: Src::DeterministicLanguage,
            posture: P::FullSemantic,
            degrade: D::SourceLabeledFallback,
            narrow_reason: Some(N::ProviderDegradedFallback),
            mapping: M::Approximate,
            content: C::Live,
            index: I::Fresh,
            side_effect: AdditionalEditCue::None,
            drift: Drift::WrongAnchorMapping,
            next: Some(A::OpenInFullEditor),
        },
        // Constrained-file narrowing: generated file blocks completion apply.
        Seed {
            kind: K::Completion,
            surface: S::GeneratedFile,
            provider_id: "provider:generator.codegen",
            source: Src::FrameworkProvider,
            posture: P::FullSemantic,
            degrade: D::ReadOnlyNoApply,
            narrow_reason: Some(N::WriteRoutesThroughGenerator),
            mapping: M::Exact,
            content: C::ImportedSnapshot,
            index: I::Fresh,
            side_effect: AdditionalEditCue::GeneratedOutputEffect,
            drift: Drift::ConstrainedFileNarrowing,
            next: Some(A::OpenGeneratorSource),
        },
        // Constrained-file narrowing: protected file requires approval.
        Seed {
            kind: K::Completion,
            surface: S::ProtectedFile,
            provider_id: "provider:lsp.rust-analyzer",
            source: Src::DeterministicLanguage,
            posture: P::RestrictedMode,
            degrade: D::ReadOnlyNoApply,
            narrow_reason: Some(N::WriteRequiresApproval),
            mapping: M::Exact,
            content: C::PolicyLimited,
            index: I::Fresh,
            side_effect: AdditionalEditCue::None,
            drift: Drift::ConstrainedFileNarrowing,
            next: Some(A::RequestApprovalReview),
        },
        // Constrained-file narrowing: notebook cross-cell hint fallback.
        Seed {
            kind: K::Hint,
            surface: S::NotebookCell,
            provider_id: "provider:notebook.kernel-introspection",
            source: Src::CachedFallback,
            posture: P::DegradedProvider,
            degrade: D::SourceLabeledFallback,
            narrow_reason: Some(N::ProviderDegradedFallback),
            mapping: M::Approximate,
            content: C::FallbackProvider,
            index: I::Fresh,
            side_effect: AdditionalEditCue::None,
            drift: Drift::ConstrainedFileNarrowing,
            next: Some(A::ViewOnlyNoAction),
        },
        // Constrained-file narrowing: request editor hover from schema.
        Seed {
            kind: K::Hover,
            surface: S::RequestEditor,
            provider_id: "provider:schema.request",
            source: Src::FrameworkProvider,
            posture: P::RestrictedMode,
            degrade: D::SourceLabeledFallback,
            narrow_reason: Some(N::SuppressedForSafety),
            mapping: M::Heuristic,
            content: C::PolicyLimited,
            index: I::NotIndexed,
            side_effect: AdditionalEditCue::None,
            drift: Drift::ConstrainedFileNarrowing,
            next: Some(A::ViewOnlyNoAction),
        },
        // Constrained-file narrowing: docs-code peek blocked.
        Seed {
            kind: K::Peek,
            surface: S::DocsCodeBlock,
            provider_id: "provider:lexical.detected-language",
            source: Src::CachedFallback,
            posture: P::RestrictedMode,
            degrade: D::BlockedUnavailable,
            narrow_reason: Some(N::SuppressedForSafety),
            mapping: M::Unresolved,
            content: C::Suppressed,
            index: I::NotIndexed,
            side_effect: AdditionalEditCue::None,
            drift: Drift::ConstrainedFileNarrowing,
            next: Some(A::OpenInFullEditor),
        },
        // Cached / local-word fallback: SQL dialect peek.
        Seed {
            kind: K::Peek,
            surface: S::SqlEditor,
            provider_id: "provider:sql.dialect-introspection",
            source: Src::CachedFallback,
            posture: P::OfflineCachedOnly,
            degrade: D::SourceLabeledFallback,
            narrow_reason: Some(N::ProviderDegradedFallback),
            mapping: M::Heuristic,
            content: C::FallbackProvider,
            index: I::ReindexPending,
            side_effect: AdditionalEditCue::None,
            drift: Drift::CachedLocalWordFallback,
            next: Some(A::ReconnectProvider),
        },
        // Partial-index pending: completion on a still-indexing file.
        Seed {
            kind: K::Completion,
            surface: S::PartialIndexState,
            provider_id: "provider:lsp.rust-analyzer",
            source: Src::DeterministicLanguage,
            posture: P::StalePartialIndex,
            degrade: D::PendingPartialIndex,
            narrow_reason: Some(N::IndexStillBuilding),
            mapping: M::Approximate,
            content: C::PartialIndexPending,
            index: I::PartialBuilding,
            side_effect: AdditionalEditCue::None,
            drift: Drift::PartialIndexPending,
            next: Some(A::WaitForIndex),
        },
        // Partial-index pending: hint on a still-indexing file.
        Seed {
            kind: K::Hint,
            surface: S::PartialIndexState,
            provider_id: "provider:lsp.rust-analyzer",
            source: Src::DeterministicLanguage,
            posture: P::StalePartialIndex,
            degrade: D::PendingPartialIndex,
            narrow_reason: Some(N::IndexStillBuilding),
            mapping: M::Approximate,
            content: C::PartialIndexPending,
            index: I::PartialBuilding,
            side_effect: AdditionalEditCue::None,
            drift: Drift::PartialIndexPending,
            next: Some(A::WaitForIndex),
        },
        // Stale-doc snapshot: hover showing a stale result while refreshing.
        Seed {
            kind: K::Hover,
            surface: S::CodeFile,
            provider_id: "provider:lsp.rust-analyzer",
            source: Src::DeterministicLanguage,
            posture: P::FullSemantic,
            degrade: D::FullFidelity,
            narrow_reason: None,
            mapping: M::Exact,
            content: C::StaleRefreshPending,
            index: I::ReindexPending,
            side_effect: AdditionalEditCue::None,
            drift: Drift::StaleDocSnapshot,
            next: Some(A::ReconnectProvider),
        },
        // Stale-doc snapshot: hover over imported generated output.
        Seed {
            kind: K::Hover,
            surface: S::GeneratedFile,
            provider_id: "provider:generator.codegen",
            source: Src::FrameworkProvider,
            posture: P::FullSemantic,
            degrade: D::FullFidelity,
            narrow_reason: None,
            mapping: M::Exact,
            content: C::ImportedSnapshot,
            index: I::Fresh,
            side_effect: AdditionalEditCue::None,
            drift: Drift::StaleDocSnapshot,
            next: Some(A::RegenerateFromSource),
        },
    ]
}

// ---------------------------------------------------------------------------
// Builder.
// ---------------------------------------------------------------------------

/// Builds the one canonical assist-support packet.
///
/// The build is deterministic and self-contained: it never opens a file, reads
/// the environment, or consults the clock. It expands the frozen seed corpus into
/// decision records (deriving ids, field ids, command ids, and bounded
/// explanations), aggregates the drift and surface rollups, declares the
/// support-export contract, and evaluates every frozen invariant over the
/// assembled data so the record's `invariants[].holds` reflect real checks.
pub fn assist_support_packet() -> AssistSupportPacket {
    let drift_catalog = build_drift_catalog();
    let decisions = build_decisions();
    let drift_rollups = build_drift_rollups(&decisions);
    let surface_rollups = build_surface_rollups(&decisions);
    let support_export = build_support_export_contract();

    let invariants = evaluate_invariants(&decisions, &drift_rollups, &surface_rollups);
    let qualified = invariants.iter().all(|invariant| invariant.holds);

    let drifted = decisions
        .iter()
        .filter(|decision| !decision.is_clean())
        .count();
    let summary = if qualified {
        format!(
            "Assist-support packet frozen: {decisions} decisions ({drifted} drifted) across \
             {kinds} kinds and {surfaces} surfaces, {drift} drift classes, all {invariants} \
             invariants hold.",
            decisions = decisions.len(),
            kinds = AssistDecisionKind::ALL.len(),
            surfaces = surface_rollups.len(),
            drift = drift_rollups.len(),
            invariants = invariants.len(),
        )
    } else {
        let failed: Vec<&str> = invariants
            .iter()
            .filter(|invariant| !invariant.holds)
            .map(|invariant| invariant.invariant_id.as_str())
            .collect();
        format!(
            "Assist-support packet is inconsistent: failing invariants {}.",
            failed.join(", ")
        )
    };

    AssistSupportPacket {
        record_kind: M5_ASSIST_SUPPORT_RECORD_KIND.to_owned(),
        m5_assist_support_schema_version: M5_ASSIST_SUPPORT_SCHEMA_VERSION,
        schema_ref: M5_ASSIST_SUPPORT_SCHEMA_REF.to_owned(),
        packet_id: M5_ASSIST_SUPPORT_PACKET_ID.to_owned(),
        as_of: M5_ASSIST_SUPPORT_AS_OF.to_owned(),
        drift_catalog,
        decisions,
        drift_rollups,
        surface_rollups,
        support_export,
        invariants,
        raw_payload_excluded: true,
        summary,
    }
}

fn build_drift_catalog() -> Vec<DriftDescriptor> {
    AssistDriftClass::ALL
        .iter()
        .map(|class| DriftDescriptor {
            class_token: class.as_str().to_owned(),
            label: class.label().to_owned(),
            summary: class.summary().to_owned(),
        })
        .collect()
}

fn build_decisions() -> Vec<AssistDecisionRecord> {
    seed_corpus()
        .into_iter()
        .enumerate()
        .map(|(index, seed)| build_decision(index, seed))
        .collect()
}

fn build_decision(index: usize, seed: Seed) -> AssistDecisionRecord {
    let decision_id = format!("{}{:04}", seed.kind.id_prefix(), index);
    let field_id = format!(
        "assist_support.{}.{}",
        seed.kind.as_str(),
        seed.drift.as_str()
    );
    let subject_ref = format!(
        "{}{}:{:04}",
        seed.kind.subject_kind().id_prefix(),
        seed.surface.as_str(),
        index
    );
    let next_action_command = seed.next.map(|action| action.command_id().to_owned());
    let explanation = build_explanation(&seed);

    AssistDecisionRecord {
        decision_id,
        field_id,
        kind: seed.kind,
        surface_class: seed.surface,
        subject_ref,
        provider_id: seed.provider_id.to_owned(),
        source_label_class: seed.source,
        provider_posture: seed.posture,
        degrade_state: seed.degrade,
        narrow_reason: seed.narrow_reason,
        mapping_quality: seed.mapping,
        content_state: seed.content,
        index_freshness: seed.index,
        side_effect_cue: seed.side_effect,
        drift_class: seed.drift,
        next_safe_action: seed.next,
        next_action_command,
        explanation,
        redaction_safe: true,
    }
}

fn build_explanation(seed: &Seed) -> String {
    let next = seed
        .next
        .map(|action| format!(" Next safe action: {}.", action.label()))
        .unwrap_or_default();
    match seed.drift {
        AssistDriftClass::NoDriftAuthoritative => format!(
            "{} resolved from the authoritative {} provider with an exact anchor and live content; no drift.",
            seed.kind.label(),
            seed.source.as_str(),
        ),
        AssistDriftClass::ProviderDrift => format!(
            "{} fell back because the preferred provider drifted to a degraded posture ({}).{}",
            seed.kind.label(),
            seed.posture.as_str(),
            next,
        ),
        AssistDriftClass::CachedLocalWordFallback => format!(
            "{} was served from a cached / local-word fallback ({}) rather than deterministic language intelligence.{}",
            seed.kind.label(),
            seed.source.as_str(),
            next,
        ),
        AssistDriftClass::WrongAnchorMapping => format!(
            "{} resolved an anchor of {} mapping quality rather than an exact match, so the target may be wrong.{}",
            seed.kind.label(),
            seed.mapping.as_str(),
            next,
        ),
        AssistDriftClass::ConstrainedFileNarrowing => format!(
            "{} was narrowed on the {} because {}.{}",
            seed.kind.label(),
            seed.surface.as_str(),
            seed.narrow_reason
                .map(|reason| reason.as_str())
                .unwrap_or("the file state constrains assist"),
            next,
        ),
        AssistDriftClass::PartialIndexPending => format!(
            "{} is pending while the semantic index finishes building on the {}.{}",
            seed.kind.label(),
            seed.surface.as_str(),
            next,
        ),
        AssistDriftClass::StaleDocSnapshot => format!(
            "{} reflects a {} snapshot rather than a live read.{}",
            seed.kind.label(),
            seed.content.as_str(),
            next,
        ),
    }
}

fn build_drift_rollups(decisions: &[AssistDecisionRecord]) -> Vec<AssistDriftRollup> {
    AssistDriftClass::ALL
        .iter()
        .filter_map(|class| {
            let matching: Vec<&AssistDecisionRecord> = decisions
                .iter()
                .filter(|decision| decision.drift_class == *class)
                .collect();
            if matching.is_empty() {
                return None;
            }
            let affected_kinds = AssistDecisionKind::ALL
                .iter()
                .copied()
                .filter(|kind| matching.iter().any(|decision| decision.kind == *kind))
                .collect();
            let affected_surfaces = EditorSurfaceClass::ALL
                .iter()
                .copied()
                .filter(|surface| {
                    matching
                        .iter()
                        .any(|decision| decision.surface_class == *surface)
                })
                .collect();
            Some(AssistDriftRollup {
                drift_class: *class,
                field_id: format!("assist_support.drift.{}", class.as_str()),
                count: matching.len(),
                affected_kinds,
                affected_surfaces,
                explanation: class.summary().to_owned(),
            })
        })
        .collect()
}

fn build_surface_rollups(decisions: &[AssistDecisionRecord]) -> Vec<AssistSurfaceRollup> {
    EditorSurfaceClass::ALL
        .iter()
        .filter_map(|surface| {
            let matching: Vec<&AssistDecisionRecord> = decisions
                .iter()
                .filter(|decision| decision.surface_class == *surface)
                .collect();
            if matching.is_empty() {
                return None;
            }
            let clean_count = matching
                .iter()
                .filter(|decision| decision.is_clean())
                .count();
            Some(AssistSurfaceRollup {
                surface_class: *surface,
                field_id: format!("assist_support.surface.{}", surface.as_str()),
                count: matching.len(),
                clean_count,
                drifted_count: matching.len() - clean_count,
            })
        })
        .collect()
}

fn build_support_export_contract() -> SupportExportContract {
    SupportExportContract {
        record_kind: "assist_decision_record".to_owned(),
        exported_fields: [
            "decision_id",
            "field_id",
            "kind",
            "surface_class",
            "subject_ref",
            "provider_id",
            "source_label_class",
            "provider_posture",
            "degrade_state",
            "narrow_reason",
            "mapping_quality",
            "content_state",
            "index_freshness",
            "side_effect_cue",
            "drift_class",
            "next_safe_action",
            "next_action_command",
        ]
        .iter()
        .map(|field| (*field).to_owned())
        .collect(),
        redacted_classes: [
            "source_text",
            "prompt_context",
            "provider_payload",
            "credential_body",
            "buffer_contents",
        ]
        .iter()
        .map(|class| (*class).to_owned())
        .collect(),
        raw_payload_excluded: true,
        note: "Exports identity, provider / source path, degraded reason, freshness, mapping, and \
               drift class only. Never exports source text, prompt context, raw provider payloads, \
               or credential bodies."
            .to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Invariant evaluation.
// ---------------------------------------------------------------------------

fn evaluate_invariants(
    decisions: &[AssistDecisionRecord],
    drift_rollups: &[AssistDriftRollup],
    surface_rollups: &[AssistSurfaceRollup],
) -> Vec<AssistSupportInvariant> {
    vec![
        AssistSupportInvariant {
            invariant_id: "every_decision_carries_stable_ids".to_owned(),
            statement:
                "Every decision carries a kind-prefixed decision id, a correlation field id, a subject ref, and a provider id."
                    .to_owned(),
            holds: every_decision_carries_stable_ids(decisions),
        },
        AssistSupportInvariant {
            invariant_id: "clean_baseline_is_fully_authoritative".to_owned(),
            statement:
                "A decision is no-drift if and only if it has a full-semantic posture, exact anchor, live content, full fidelity, no narrow reason, and no remediation route."
                    .to_owned(),
            holds: clean_baseline_is_fully_authoritative(decisions),
        },
        AssistSupportInvariant {
            invariant_id: "drift_class_matches_evidence".to_owned(),
            statement:
                "Each drift class is consistent with the decision's provider posture, source, mapping quality, content state, and constrained-surface evidence."
                    .to_owned(),
            holds: drift_class_matches_evidence(decisions),
        },
        AssistSupportInvariant {
            invariant_id: "drifted_decisions_offer_route_and_explanation".to_owned(),
            statement:
                "Every drifted decision carries a next-safe-action route with a command id and a non-empty explanation."
                    .to_owned(),
            holds: drifted_decisions_offer_route_and_explanation(decisions),
        },
        AssistSupportInvariant {
            invariant_id: "narrowing_iff_degraded".to_owned(),
            statement:
                "A decision carries a narrow reason exactly when its degraded-state class is not full fidelity."
                    .to_owned(),
            holds: narrowing_iff_degraded(decisions),
        },
        AssistSupportInvariant {
            invariant_id: "rollups_reconcile_with_decisions".to_owned(),
            statement:
                "Drift and surface rollups cover exactly the classes and surfaces present, and each dimension's counts sum to the decision total."
                    .to_owned(),
            holds: rollups_reconcile_with_decisions(decisions, drift_rollups, surface_rollups),
        },
        AssistSupportInvariant {
            invariant_id: "corpus_covers_kinds_and_constrained_surfaces".to_owned(),
            statement:
                "The corpus includes every decision kind and every claimed constrained surface so the shared model is proved across surfaces."
                    .to_owned(),
            holds: corpus_covers_kinds_and_constrained_surfaces(decisions),
        },
        AssistSupportInvariant {
            invariant_id: "redaction_safe_excludes_raw_payload".to_owned(),
            statement:
                "Every decision is metadata-only and the support-export contract excludes source text, prompt context, provider payloads, and credential bodies."
                    .to_owned(),
            holds: redaction_safe_excludes_raw_payload(decisions),
        },
    ]
}

fn every_decision_carries_stable_ids(decisions: &[AssistDecisionRecord]) -> bool {
    !decisions.is_empty()
        && decisions.iter().all(|decision| {
            decision.decision_id.starts_with(decision.kind.id_prefix())
                && !decision.field_id.is_empty()
                && !decision.subject_ref.is_empty()
                && !decision.provider_id.is_empty()
        })
}

fn clean_baseline_is_fully_authoritative(decisions: &[AssistDecisionRecord]) -> bool {
    let has_clean = decisions.iter().any(|decision| decision.is_clean());
    has_clean
        && decisions.iter().all(|decision| {
            let fully_authoritative = decision.provider_posture
                == CompletionProviderPosture::FullSemantic
                && decision.mapping_quality == MappingQualityClass::Exact
                && decision.content_state.is_live()
                && decision.degrade_state == AssistDegradeClass::FullFidelity
                && decision.narrow_reason.is_none()
                && decision.next_safe_action.is_none();
            decision.is_clean() == fully_authoritative
        })
}

fn drift_class_matches_evidence(decisions: &[AssistDecisionRecord]) -> bool {
    decisions.iter().all(|decision| match decision.drift_class {
        AssistDriftClass::NoDriftAuthoritative => {
            decision.content_state.is_live()
                && decision.provider_posture == CompletionProviderPosture::FullSemantic
        }
        AssistDriftClass::ProviderDrift => {
            decision.provider_posture.is_degraded()
                && decision.content_state == AssistContentStateClass::FallbackProvider
        }
        AssistDriftClass::CachedLocalWordFallback => {
            decision.source_label_class == AssistSourceLabelClass::CachedFallback
                || matches!(
                    decision.provider_posture,
                    CompletionProviderPosture::LargeFileFallback
                        | CompletionProviderPosture::OfflineCachedOnly
                )
        }
        AssistDriftClass::WrongAnchorMapping => decision.mapping_quality.requires_disclosure(),
        AssistDriftClass::ConstrainedFileNarrowing => {
            decision.surface_class.is_constrained() && decision.narrow_reason.is_some()
        }
        AssistDriftClass::PartialIndexPending => {
            decision.content_state == AssistContentStateClass::PartialIndexPending
                && decision.index_freshness == IndexFreshnessClass::PartialBuilding
        }
        AssistDriftClass::StaleDocSnapshot => matches!(
            decision.content_state,
            AssistContentStateClass::StaleRefreshPending
                | AssistContentStateClass::ImportedSnapshot
        ),
    })
}

fn drifted_decisions_offer_route_and_explanation(decisions: &[AssistDecisionRecord]) -> bool {
    decisions
        .iter()
        .filter(|decision| !decision.is_clean())
        .all(|decision| {
            decision.next_safe_action.is_some()
                && decision.next_action_command.is_some()
                && !decision.explanation.is_empty()
        })
}

fn narrowing_iff_degraded(decisions: &[AssistDecisionRecord]) -> bool {
    decisions.iter().all(|decision| {
        let degraded = decision.degrade_state != AssistDegradeClass::FullFidelity;
        decision.narrow_reason.is_some() == degraded
    })
}

fn rollups_reconcile_with_decisions(
    decisions: &[AssistDecisionRecord],
    drift_rollups: &[AssistDriftRollup],
    surface_rollups: &[AssistSurfaceRollup],
) -> bool {
    let drift_total: usize = drift_rollups.iter().map(|rollup| rollup.count).sum();
    let surface_total: usize = surface_rollups.iter().map(|rollup| rollup.count).sum();
    if drift_total != decisions.len() || surface_total != decisions.len() {
        return false;
    }
    // Every present drift class / surface has exactly one rollup, and no rollup is
    // empty.
    let drift_classes_present: std::collections::BTreeSet<&str> = decisions
        .iter()
        .map(|decision| decision.drift_class.as_str())
        .collect();
    let drift_classes_rolled: std::collections::BTreeSet<&str> = drift_rollups
        .iter()
        .map(|rollup| rollup.drift_class.as_str())
        .collect();
    let surfaces_present: std::collections::BTreeSet<&str> = decisions
        .iter()
        .map(|decision| decision.surface_class.as_str())
        .collect();
    let surfaces_rolled: std::collections::BTreeSet<&str> = surface_rollups
        .iter()
        .map(|rollup| rollup.surface_class.as_str())
        .collect();
    drift_classes_present == drift_classes_rolled
        && surfaces_present == surfaces_rolled
        && drift_rollups.iter().all(|rollup| rollup.count > 0)
        && surface_rollups.iter().all(|rollup| {
            rollup.count > 0 && rollup.clean_count + rollup.drifted_count == rollup.count
        })
}

fn corpus_covers_kinds_and_constrained_surfaces(decisions: &[AssistDecisionRecord]) -> bool {
    let kinds_covered = AssistDecisionKind::ALL
        .iter()
        .all(|kind| decisions.iter().any(|decision| decision.kind == *kind));
    let constrained_surfaces = [
        EditorSurfaceClass::NotebookCell,
        EditorSurfaceClass::RequestEditor,
        EditorSurfaceClass::SqlEditor,
        EditorSurfaceClass::DocsCodeBlock,
        EditorSurfaceClass::GeneratedFile,
        EditorSurfaceClass::ProtectedFile,
        EditorSurfaceClass::PartialIndexState,
        EditorSurfaceClass::LargeFileRestricted,
    ];
    let surfaces_covered = constrained_surfaces.iter().all(|surface| {
        decisions
            .iter()
            .any(|decision| decision.surface_class == *surface)
    });
    kinds_covered && surfaces_covered
}

fn redaction_safe_excludes_raw_payload(decisions: &[AssistDecisionRecord]) -> bool {
    let contract = build_support_export_contract();
    let required_redactions = [
        "source_text",
        "prompt_context",
        "provider_payload",
        "credential_body",
    ];
    decisions.iter().all(|decision| decision.redaction_safe)
        && contract.raw_payload_excluded
        && required_redactions.iter().all(|class| {
            contract
                .redacted_classes
                .iter()
                .any(|redacted| redacted == class)
        })
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the export-safe, human-readable lines for the assist-support packet.
///
/// This is the shared projection consumed by the headless CLI emitter, Project
/// Doctor, and support export, so they never clone packet text from each other.
pub fn assist_support_packet_lines(packet: &AssistSupportPacket) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Assist-support packet — {} ({})",
        packet.packet_id, packet.as_of
    ));
    lines.push(format!(
        "schema_ref={} version={}",
        packet.schema_ref, packet.m5_assist_support_schema_version
    ));

    lines.push("Decisions:".to_owned());
    for decision in &packet.decisions {
        lines.push(format!(
            "  {id} [{kind}/{surface}] drift={drift} source={source} posture={posture} \
             degrade={degrade} mapping={mapping} content={content} index={index}",
            id = decision.decision_id,
            kind = decision.kind.as_str(),
            surface = decision.surface_class.as_str(),
            drift = decision.drift_class.as_str(),
            source = decision.source_label_class.as_str(),
            posture = decision.provider_posture.as_str(),
            degrade = decision.degrade_state.as_str(),
            mapping = decision.mapping_quality.as_str(),
            content = decision.content_state.as_str(),
            index = decision.index_freshness.as_str(),
        ));
        lines.push(format!("    field_id={}", decision.field_id));
        lines.push(format!("    subject_ref={}", decision.subject_ref));
        if let Some(command) = &decision.next_action_command {
            lines.push(format!("    next_action_command={command}"));
        }
        lines.push(format!("    {}", decision.explanation));
    }

    lines.push("Drift rollups:".to_owned());
    for rollup in &packet.drift_rollups {
        lines.push(format!(
            "  {drift} count={count} field_id={field}",
            drift = rollup.drift_class.as_str(),
            count = rollup.count,
            field = rollup.field_id,
        ));
    }

    lines.push("Surface rollups:".to_owned());
    for rollup in &packet.surface_rollups {
        lines.push(format!(
            "  {surface} count={count} clean={clean} drifted={drifted} field_id={field}",
            surface = rollup.surface_class.as_str(),
            count = rollup.count,
            clean = rollup.clean_count,
            drifted = rollup.drifted_count,
            field = rollup.field_id,
        ));
    }

    lines.push(format!(
        "Support export: record_kind={kind} raw_payload_excluded={excluded} redacted=[{redacted}]",
        kind = packet.support_export.record_kind,
        excluded = packet.support_export.raw_payload_excluded,
        redacted = packet.support_export.redacted_classes.join(", "),
    ));

    lines.push("Invariants:".to_owned());
    for invariant in &packet.invariants {
        lines.push(format!(
            "  {id} holds={holds}",
            id = invariant.invariant_id,
            holds = invariant.holds,
        ));
    }

    lines.push(packet.summary.clone());
    lines
}

#[cfg(test)]
mod tests;
