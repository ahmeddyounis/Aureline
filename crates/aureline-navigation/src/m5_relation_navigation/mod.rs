//! Relation-navigation matrix: the frozen, typed contract for Aureline's
//! definition/declaration/implementation/reference/hierarchy/related-object and
//! rename-preview navigation truth.
//!
//! Aureline navigates relations, and relation kinds stay explicit and
//! trustworthy. A definition is not a declaration; a grep fallback never
//! masquerades as semantic certainty; implementation and hierarchy edges preserve
//! their proof class and ambiguity; related-object navigation stays
//! source-attributed; and a rename preview exposes blocked, generated, read-only,
//! and partial-scope candidates before any broad mutation. The objects that carry
//! that truth already exist: the [`navigation target`](crate::target_model::NavigationTarget),
//! [`reference occurrence`](crate::target_model::ReferenceOccurrence),
//! [`hierarchy edge`](crate::target_model::HierarchyEdge),
//! [`disambiguation / related-object set`](crate::target_model::NavigationDisambiguationSet),
//! and [`rename-preview set`](crate::target_model::RenamePreviewSet) all have typed
//! records and a boundary schema under `schemas/navigation/`. What was still
//! implicit was a single place that names the relation-navigation object
//! *families*, freezes their stable identifiers and required fields, pins one
//! controlled vocabulary across relation kinds, proof classes, access kinds,
//! ambiguity, freshness, partiality, generated/runtime labels, and rename omission
//! reasons, maps each object to the proof packet that keeps it current, and states
//! the invariants every relation-navigation surface must hold. This lane is that
//! place.
//!
//! The matrix does four things:
//!
//! 1. **Names the relation-navigation object families**
//!    ([`RelationNavObjectClass`]) and, for each, cites the canonical boundary
//!    schema(s) it binds, the crate module that already produces that truth, the
//!    required fields it must carry, the relation kinds it can represent, and the
//!    [`proof packet`](RelationNavObjectEntry::proof_packet_ref) that keeps it
//!    current — so search, graph, docs/help, editor, AI, review, and support
//!    surfaces point at the same object model rather than re-expressing
//!    definition/reference/hierarchy/rename truth ad hoc.
//! 2. **Freezes one qualification-state vocabulary** ([`RelationNavStateClass`])
//!    spanning exact/indexed semantic proof, disclosed lexical/syntax/framework/
//!    runtime/imported fallback, ambiguity and drift, partial and stale scope,
//!    generated and read-only boundaries, blocked rename, and the unavailable
//!    classes. Each state carries computed honesty flags and the upstream enum it
//!    derives from.
//! 3. **Defines the controlled vocabulary** ([`RelationNavVocabulary`]) the spec
//!    requires: relation kind, proof class, access kind, ambiguity, freshness,
//!    partiality, generated/runtime label, and rename omission reason. Each object
//!    declares which axes it binds.
//! 4. **Covers every consumer surface** ([`RelationNavConsumer`]): the search
//!    palette, editor assist, graph overlay, docs/help, AI context, review
//!    workspace, support export, CLI/headless, and shell continuity surfaces that
//!    render these objects.
//!
//! [`relation_navigation_matrix`] is the canonical binding: it builds the matrix
//! deterministically and computes each [`RelationNavInvariant`]'s `holds` flag from
//! the built objects and states, so the checked-in fixture and the freeze gate
//! freeze the contract byte-for-byte and an inconsistent edit flips an invariant
//! and fails CI. In particular [`RelationNavInvariant`]
//! `relation_nav.proof_packet_mapped` flips false the moment a claimed
//! relation-navigation object lacks a mapped proof packet, so stable promotion
//! cannot harden a relation-navigation claim without current proof. The record
//! carries no source bodies, raw paths, provider payloads, URLs, hostnames, or
//! credentials — only opaque object refs, stable tokens, and short reviewable
//! sentences — so it is safe for support export.

use serde::{Deserialize, Serialize};

use crate::target_model::{AccessKind, ProofClass, RelationKind, REQUIRED_RELATION_KINDS};

#[cfg(test)]
mod tests;

/// Schema version for the relation-navigation matrix.
pub const M5_RELATION_NAVIGATION_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the relation-navigation matrix.
pub const M5_RELATION_NAVIGATION_SCHEMA_REF: &str =
    "schemas/navigation/m5-relation-navigation.schema.json";

/// Stable record-kind tag for the relation-navigation matrix.
pub const M5_RELATION_NAVIGATION_RECORD_KIND: &str = "m5_relation_navigation_matrix";

/// Stable id for the canonical relation-navigation matrix.
pub const M5_RELATION_NAVIGATION_MATRIX_ID: &str = "m5-relation-navigation:matrix:0001";

/// Evaluation stamp for the canonical matrix. Held as a constant so the canonical
/// binding stays deterministic and the fixture freezes byte-for-byte.
pub const M5_RELATION_NAVIGATION_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The freeze gate that keeps the matrix binding current. Stable promotion runs
/// this gate; it fails when the in-code matrix drifts from the checked-in fixture
/// or any invariant flips.
pub const M5_RELATION_NAVIGATION_FREEZE_GATE_REF: &str =
    "crates/aureline-navigation/tests/m5_relation_navigation.rs";

// ---------------------------------------------------------------------------
// Relation-navigation object families.
// ---------------------------------------------------------------------------

/// The closed set of governed relation-navigation object families this matrix
/// freezes.
///
/// Each family is one governed navigation object. Adding a family is a breaking
/// change to the matrix; renaming one breaks every consumer that resolves an
/// object by token, so the tokens are frozen here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationNavObjectClass {
    /// The navigation target: the stable target a go-to, outline, breadcrumb,
    /// hierarchy, or search jump resolves to, with its relation kind and proof.
    NavigationTarget,
    /// The reference occurrence: one member of a find-references or evidence set,
    /// with its access kind, proof class, and scope completeness.
    ReferenceOccurrence,
    /// The hierarchy edge: one call/type/override/owner/route/docs edge that
    /// preserves its proof class, depth, ambiguity, and runtime/framework evidence.
    HierarchyEdge,
    /// The related-object relation: a source-attributed link to a related target —
    /// type, implementation, owner, route binding, doc, or generated pair — kept
    /// inspectable and disambiguable.
    RelatedObjectRelation,
    /// The rename-preview set: the candidate occurrences a rename would touch, with
    /// blocked, generated, read-only, and partial-scope candidates exposed before
    /// any broad mutation.
    RenamePreviewSet,
    /// The relation / fallback vocabulary: the dictionary of relation kinds, proof
    /// classes, and fallback/disclosure labels every other object resolves against.
    RelationFallbackVocabulary,
}

impl RelationNavObjectClass {
    /// All object families, in matrix order.
    pub const ALL: [Self; 6] = [
        Self::NavigationTarget,
        Self::ReferenceOccurrence,
        Self::HierarchyEdge,
        Self::RelatedObjectRelation,
        Self::RenamePreviewSet,
        Self::RelationFallbackVocabulary,
    ];

    /// Stable snake_case token for this family.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NavigationTarget => "navigation_target",
            Self::ReferenceOccurrence => "reference_occurrence",
            Self::HierarchyEdge => "hierarchy_edge",
            Self::RelatedObjectRelation => "related_object_relation",
            Self::RenamePreviewSet => "rename_preview_set",
            Self::RelationFallbackVocabulary => "relation_fallback_vocabulary",
        }
    }

    /// Stable object id, namespaced so it is unique across the product.
    pub fn object_id(self) -> String {
        format!("relation_nav_object.{}", self.as_str())
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NavigationTarget => "Navigation target",
            Self::ReferenceOccurrence => "Reference occurrence",
            Self::HierarchyEdge => "Hierarchy edge",
            Self::RelatedObjectRelation => "Related-object relation",
            Self::RenamePreviewSet => "Rename-preview set",
            Self::RelationFallbackVocabulary => "Relation / fallback vocabulary",
        }
    }

    /// Whether this family is a navigable object (versus the vocabulary
    /// dictionary), so per-row honesty invariants apply to it.
    pub const fn is_navigable_object(self) -> bool {
        !matches!(self, Self::RelationFallbackVocabulary)
    }
}

// ---------------------------------------------------------------------------
// Unified qualification-state vocabulary.
// ---------------------------------------------------------------------------

/// One shared qualification-state vocabulary spanning every relation-navigation
/// object.
///
/// The tokens span exact and indexed semantic proof, the disclosed fallback
/// classes (lexical, syntax, framework, runtime, imported), ambiguity and drift,
/// partial and stale scope, generated and read-only boundaries, blocked rename,
/// and the unavailable classes already frozen by [`crate::target_model`]. Each
/// [`RelationNavStateTerm`] in the matrix cites the upstream enum it derives from,
/// so this vocabulary never silently diverges from the objects it summarizes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationNavStateClass {
    /// Proven exactly against current source by a direct semantic provider.
    ExactSemantic,
    /// Proven from a current index or graph for the declared scope.
    IndexedSemantic,
    /// Imported from a snapshot, docs pack, provider overlay, or generated-source
    /// lineage, disclosed as such.
    ImportedSnapshot,
    /// Lexical / grep fallback, disclosed and never shown as semantic certainty.
    LexicalFallbackDisclosed,
    /// Syntax-tree-only fallback, disclosed.
    SyntaxFallbackDisclosed,
    /// Framework, route, or generator-metadata derived, disclosed.
    FrameworkDerivedDisclosed,
    /// Runtime trace, debugger, or observed-dispatch derived, disclosed.
    RuntimeObservedDisclosed,
    /// Multiple candidates require explicit selection.
    AmbiguousNeedsSelection,
    /// Multiple candidates are ranked but still inspectable.
    MultipleCandidatesRanked,
    /// A previous target drifted and cannot auto-open safely.
    DriftedNeedsReview,
    /// Result set is partial for the declared scope.
    PartialScope,
    /// Result set is stale for the declared scope.
    StaleScope,
    /// Resolved across a generated or paired-artifact boundary, disclosed.
    GeneratedBoundaryDisclosed,
    /// Resolved into read-only, protected, or external-dependency source.
    ReadOnlyProtected,
    /// A rename candidate is blocked and held for review before any broad mutation.
    RenameBlockedPendingReview,
    /// Target is missing from the current scope.
    MissingTarget,
    /// Workset, branch, policy, docs pack, or remote shard hides the target.
    ScopeUnavailable,
    /// No admissible proof exists for the requested relation.
    Unavailable,
}

impl RelationNavStateClass {
    /// All states, in vocabulary order.
    pub const ALL: [Self; 18] = [
        Self::ExactSemantic,
        Self::IndexedSemantic,
        Self::ImportedSnapshot,
        Self::LexicalFallbackDisclosed,
        Self::SyntaxFallbackDisclosed,
        Self::FrameworkDerivedDisclosed,
        Self::RuntimeObservedDisclosed,
        Self::AmbiguousNeedsSelection,
        Self::MultipleCandidatesRanked,
        Self::DriftedNeedsReview,
        Self::PartialScope,
        Self::StaleScope,
        Self::GeneratedBoundaryDisclosed,
        Self::ReadOnlyProtected,
        Self::RenameBlockedPendingReview,
        Self::MissingTarget,
        Self::ScopeUnavailable,
        Self::Unavailable,
    ];

    /// Stable snake_case token for this state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactSemantic => "exact_semantic",
            Self::IndexedSemantic => "indexed_semantic",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::LexicalFallbackDisclosed => "lexical_fallback_disclosed",
            Self::SyntaxFallbackDisclosed => "syntax_fallback_disclosed",
            Self::FrameworkDerivedDisclosed => "framework_derived_disclosed",
            Self::RuntimeObservedDisclosed => "runtime_observed_disclosed",
            Self::AmbiguousNeedsSelection => "ambiguous_needs_selection",
            Self::MultipleCandidatesRanked => "multiple_candidates_ranked",
            Self::DriftedNeedsReview => "drifted_needs_review",
            Self::PartialScope => "partial_scope",
            Self::StaleScope => "stale_scope",
            Self::GeneratedBoundaryDisclosed => "generated_boundary_disclosed",
            Self::ReadOnlyProtected => "read_only_protected",
            Self::RenameBlockedPendingReview => "rename_blocked_pending_review",
            Self::MissingTarget => "missing_target",
            Self::ScopeUnavailable => "scope_unavailable",
            Self::Unavailable => "unavailable",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ExactSemantic => "Exact semantic",
            Self::IndexedSemantic => "Indexed semantic",
            Self::ImportedSnapshot => "Imported snapshot",
            Self::LexicalFallbackDisclosed => "Lexical fallback (disclosed)",
            Self::SyntaxFallbackDisclosed => "Syntax fallback (disclosed)",
            Self::FrameworkDerivedDisclosed => "Framework-derived (disclosed)",
            Self::RuntimeObservedDisclosed => "Runtime-observed (disclosed)",
            Self::AmbiguousNeedsSelection => "Ambiguous — needs selection",
            Self::MultipleCandidatesRanked => "Multiple candidates ranked",
            Self::DriftedNeedsReview => "Drifted — needs review",
            Self::PartialScope => "Partial scope",
            Self::StaleScope => "Stale scope",
            Self::GeneratedBoundaryDisclosed => "Generated boundary (disclosed)",
            Self::ReadOnlyProtected => "Read-only / protected",
            Self::RenameBlockedPendingReview => "Rename blocked — pending review",
            Self::MissingTarget => "Missing target",
            Self::ScopeUnavailable => "Scope unavailable",
            Self::Unavailable => "Unavailable",
        }
    }

    /// Whether this state must render with a visible caveat: anything other than
    /// exact or indexed semantic proof cannot be shown as an unquestioned success.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::ExactSemantic | Self::IndexedSemantic)
    }

    /// Whether this state is a fallback / non-direct-semantic proof class — the
    /// class that must never masquerade as semantic certainty.
    pub const fn is_fallback_proof(self) -> bool {
        matches!(
            self,
            Self::ImportedSnapshot
                | Self::LexicalFallbackDisclosed
                | Self::SyntaxFallbackDisclosed
                | Self::FrameworkDerivedDisclosed
                | Self::RuntimeObservedDisclosed
        )
    }

    /// Whether this state is an ambiguity / drift class that requires selection or
    /// review before auto-open.
    pub const fn is_ambiguity(self) -> bool {
        matches!(
            self,
            Self::AmbiguousNeedsSelection
                | Self::MultipleCandidatesRanked
                | Self::DriftedNeedsReview
        )
    }

    /// Whether this state is a rename-omission class: a candidate the rename
    /// preview must expose as blocked, generated, read-only, or partial before any
    /// broad mutation.
    pub const fn is_rename_omission(self) -> bool {
        matches!(
            self,
            Self::GeneratedBoundaryDisclosed
                | Self::ReadOnlyProtected
                | Self::RenameBlockedPendingReview
                | Self::PartialScope
        )
    }

    /// The upstream `target_model` enum variant this state derives from, for
    /// provenance.
    fn derived_from_refs(self) -> Vec<String> {
        let module = "crates/aureline-navigation/src/target_model/mod.rs";
        let refs: Vec<String> = match self {
            Self::ExactSemantic => vec![
                format!("{module}#ProofClass::DirectSemantic"),
                format!("{module}#NavigationConfidence::Exact"),
            ],
            Self::IndexedSemantic => vec![
                format!("{module}#ProofClass::IndexedSemantic"),
                format!("{module}#NavigationConfidence::Indexed"),
            ],
            Self::ImportedSnapshot => vec![
                format!("{module}#ProofClass::ImportedEvidence"),
                format!("{module}#NavigationConfidence::Imported"),
            ],
            Self::LexicalFallbackDisclosed => vec![
                format!("{module}#ProofClass::LexicalFallback"),
                format!("{module}#DowngradeReason::LexicalFallbackOnly"),
            ],
            Self::SyntaxFallbackDisclosed => vec![
                format!("{module}#ProofClass::SyntaxFallback"),
                format!("{module}#DowngradeReason::SyntaxFallbackOnly"),
            ],
            Self::FrameworkDerivedDisclosed => vec![
                format!("{module}#ProofClass::FrameworkDerived"),
                format!("{module}#DowngradeReason::RuntimeOrFrameworkOnly"),
            ],
            Self::RuntimeObservedDisclosed => vec![
                format!("{module}#ProofClass::RuntimeObserved"),
                format!("{module}#DowngradeReason::RuntimeOrFrameworkOnly"),
            ],
            Self::AmbiguousNeedsSelection => {
                vec![format!("{module}#AmbiguityClass::AmbiguousNeedsSelection")]
            }
            Self::MultipleCandidatesRanked => {
                vec![format!("{module}#AmbiguityClass::MultipleCandidatesRanked")]
            }
            Self::DriftedNeedsReview => {
                vec![format!("{module}#AmbiguityClass::DriftedNeedsReview")]
            }
            Self::PartialScope => {
                vec![format!(
                    "{module}#ScopeCompleteness::PartialForDeclaredScope"
                )]
            }
            Self::StaleScope => vec![
                format!("{module}#ScopeCompleteness::StaleForDeclaredScope"),
                format!("{module}#FreshnessClass::Stale"),
            ],
            Self::GeneratedBoundaryDisclosed => vec![
                format!("{module}#GeneratedOrExternalState::GeneratedSource"),
                format!("{module}#DowngradeReason::GeneratedBoundary"),
            ],
            Self::ReadOnlyProtected => vec![
                format!("{module}#GeneratedOrExternalState::ReadOnlySource"),
                format!("{module}#GeneratedOrExternalState::ExternalDependency"),
            ],
            Self::RenameBlockedPendingReview => vec![
                format!("{module}#RenameApplyPosture::BlockedPendingScopeReview"),
                format!("{module}#RenameApplyPosture::BlockedPendingPolicyOrProtectedReview"),
            ],
            Self::MissingTarget => vec![format!("{module}#AmbiguityClass::MissingTarget")],
            Self::ScopeUnavailable => vec![
                format!("{module}#AmbiguityClass::ScopeUnavailable"),
                format!("{module}#ScopeCompleteness::UnavailableForDeclaredScope"),
            ],
            Self::Unavailable => vec![format!("{module}#ProofClass::Unavailable")],
        };
        refs
    }
}

// ---------------------------------------------------------------------------
// Controlled vocabulary axes.
// ---------------------------------------------------------------------------

/// The named controlled-vocabulary axes this matrix defines and each object
/// declares it binds.
///
/// These are exactly the vocabularies the contract requires: relation kind, proof
/// class, access kind, ambiguity, freshness, partiality, generated/runtime label,
/// and rename omission reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationNavVocabulary {
    /// The relation kind the navigation represents (definition vs declaration vs
    /// implementation vs reference, etc.).
    #[serde(rename = "relation_kind")]
    RelationKindAxis,
    /// Why the relation exists and how strong the evidence is.
    #[serde(rename = "proof_class")]
    ProofClassAxis,
    /// How a reference occurrence touches the symbol (read/write/call/inherit/…).
    #[serde(rename = "access_kind")]
    AccessKindAxis,
    /// Whether the relation is unambiguous or needs selection / review.
    Ambiguity,
    /// Whether the evidence is live, warm, degraded, stale, or unverified.
    Freshness,
    /// Whether the result set is complete, partial, stale, or unavailable for the
    /// declared scope.
    Partiality,
    /// Whether the source is authored, generated, external, read-only, or imported.
    GeneratedRuntimeLabel,
    /// Why a rename candidate is omitted, blocked, or held for review.
    RenameOmissionReason,
}

impl RelationNavVocabulary {
    /// All controlled-vocabulary axes, in order.
    pub const ALL: [Self; 8] = [
        Self::RelationKindAxis,
        Self::ProofClassAxis,
        Self::AccessKindAxis,
        Self::Ambiguity,
        Self::Freshness,
        Self::Partiality,
        Self::GeneratedRuntimeLabel,
        Self::RenameOmissionReason,
    ];

    /// Stable snake_case token for this axis.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RelationKindAxis => "relation_kind",
            Self::ProofClassAxis => "proof_class",
            Self::AccessKindAxis => "access_kind",
            Self::Ambiguity => "ambiguity",
            Self::Freshness => "freshness",
            Self::Partiality => "partiality",
            Self::GeneratedRuntimeLabel => "generated_runtime_label",
            Self::RenameOmissionReason => "rename_omission_reason",
        }
    }
}

// ---------------------------------------------------------------------------
// Consumer surfaces.
// ---------------------------------------------------------------------------

/// The surfaces that render a relation-navigation object instead of restating
/// definition/reference/hierarchy/rename truth ad hoc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationNavConsumer {
    /// The unified search / navigation palette.
    SearchPalette,
    /// The editor go-to, peek, references, and rename assist micro-surfaces.
    EditorAssist,
    /// The graph / topology overlay.
    GraphOverlay,
    /// Docs, Help, and About truth surfaces.
    DocsHelp,
    /// AI context picker, composer, and tool-call evidence.
    AiContext,
    /// Review workspace and hosted review evidence.
    ReviewWorkspace,
    /// Support bundle / export packet.
    SupportExport,
    /// CLI, SDK, and headless inspection.
    CliHeadless,
    /// Shell breadcrumbs, outline, bookmarks, history, and peek continuity.
    ShellContinuity,
}

impl RelationNavConsumer {
    /// All consumer surfaces, in order.
    pub const ALL: [Self; 9] = [
        Self::SearchPalette,
        Self::EditorAssist,
        Self::GraphOverlay,
        Self::DocsHelp,
        Self::AiContext,
        Self::ReviewWorkspace,
        Self::SupportExport,
        Self::CliHeadless,
        Self::ShellContinuity,
    ];

    /// Stable snake_case token for this consumer.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchPalette => "search_palette",
            Self::EditorAssist => "editor_assist",
            Self::GraphOverlay => "graph_overlay",
            Self::DocsHelp => "docs_help",
            Self::AiContext => "ai_context",
            Self::ReviewWorkspace => "review_workspace",
            Self::SupportExport => "support_export",
            Self::CliHeadless => "cli_headless",
            Self::ShellContinuity => "shell_continuity",
        }
    }
}

/// Redaction posture applied to a relation-navigation object on export surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationNavRedactionClass {
    /// Metadata-safe default — the export default for navigation surfaces.
    MetadataSafeDefault,
    /// Summary text and stable refs only, never source bodies.
    SummaryAndRefsOnly,
    /// Operator-only restricted projection.
    OperatorOnlyRestricted,
    /// Internal-support restricted projection.
    InternalSupportRestricted,
}

// ---------------------------------------------------------------------------
// Record structs.
// ---------------------------------------------------------------------------

/// One `(token, label)` definition in the shared vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationNavTokenDef {
    /// Stable token.
    pub token: String,
    /// Human-readable label.
    pub label: String,
}

/// The controlled-vocabulary token sets and bound source schemas this matrix
/// freezes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationNavSharedVocabulary {
    /// Relation kinds (`relation_kind` axis).
    pub relation_kinds: Vec<RelationNavTokenDef>,
    /// Proof classes (`proof_class`).
    pub proof_classes: Vec<RelationNavTokenDef>,
    /// Access kinds (`access_kind`).
    pub access_kinds: Vec<RelationNavTokenDef>,
    /// Ambiguity classes (`ambiguity`).
    pub ambiguity_classes: Vec<RelationNavTokenDef>,
    /// Freshness classes (`freshness`).
    pub freshness_classes: Vec<RelationNavTokenDef>,
    /// Partiality / scope-completeness classes (`partiality`).
    pub partiality_classes: Vec<RelationNavTokenDef>,
    /// Generated / runtime / external labels (`generated_runtime_label`).
    pub generated_runtime_labels: Vec<RelationNavTokenDef>,
    /// Rename omission reasons (`rename_omission_reason`).
    pub rename_omission_reasons: Vec<RelationNavTokenDef>,
    /// Redaction classes governing export.
    pub redaction_classes: Vec<RelationNavTokenDef>,
    /// Consumer classes that render these objects.
    pub consumer_classes: Vec<RelationNavTokenDef>,
    /// The boundary schemas this matrix binds as truth sources.
    pub source_schema_refs: Vec<String>,
}

/// One state in the unified qualification vocabulary, with provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationNavStateTerm {
    /// The state.
    pub state: RelationNavStateClass,
    /// Stable token (equals `state.as_str()`), surfaced for reuse by consumers.
    pub token: String,
    /// Human-readable label.
    pub label: String,
    /// Whether this state must render with a visible caveat.
    pub requires_disclosure: bool,
    /// Whether this state is a fallback proof class that must never masquerade as
    /// semantic certainty.
    pub is_fallback_proof: bool,
    /// Whether this state is an ambiguity / drift class.
    pub is_ambiguity: bool,
    /// Whether this state is a rename-omission class.
    pub is_rename_omission: bool,
    /// The upstream `target_model` enum variant this state derives from.
    pub derived_from_refs: Vec<String>,
}

/// One required field a relation-navigation object must carry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationNavFieldDef {
    /// Stable field id (matches the producing `target_model` struct field).
    pub field_id: String,
    /// Human-readable label.
    pub label: String,
    /// Whether the field is required on every instance of the object.
    pub required: bool,
}

/// One relation-navigation object-family entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationNavObjectEntry {
    /// The object family.
    pub object: RelationNavObjectClass,
    /// Stable, namespaced object id.
    pub object_id: String,
    /// Human-readable label.
    pub label: String,
    /// One reviewable sentence describing the object.
    pub summary: String,
    /// The canonical boundary schema(s) this object binds.
    pub canonical_schema_refs: Vec<String>,
    /// The crate module(s) that already produce this truth.
    pub produced_by_refs: Vec<String>,
    /// The proof packet (contract, fixture, or evidence) that keeps this object
    /// current. Stable promotion fails when this is empty.
    pub proof_packet_ref: String,
    /// The consumers that render this object.
    pub consumed_by: Vec<RelationNavConsumer>,
    /// The relation kinds this object can represent.
    pub relation_kinds: Vec<RelationKind>,
    /// The qualification states from the unified vocabulary this object can show.
    pub applicable_states: Vec<RelationNavStateClass>,
    /// The controlled-vocabulary axes this object binds.
    pub controlled_vocabularies: Vec<RelationNavVocabulary>,
    /// The required fields this object must carry.
    pub required_fields: Vec<RelationNavFieldDef>,
    /// Whether this object always carries an explicit proof class (no relation is
    /// shown without one).
    pub proof_class_required: bool,
    /// Whether this object is source-attributed — it names a stable source anchor
    /// or evidence ref for every relation rather than asserting an unsourced link.
    pub carries_source_attribution: bool,
    /// The field that carries that source attribution, if any.
    pub source_attribution_field: Option<String>,
    /// The default redaction posture on export.
    pub default_redaction: RelationNavRedactionClass,
    /// Whether the object is locally inspectable (never console-only / portal-only).
    pub locally_inspectable: bool,
    /// Whether the object is typed (never reduced to a prose-only or toast-only view).
    pub typed_not_prose_only: bool,
    /// One reviewable sentence stating the object's relation-kind honesty rule.
    pub boundary_note: String,
}

impl RelationNavObjectEntry {
    /// Whether the object binds the named controlled-vocabulary axis.
    pub fn binds(&self, vocab: RelationNavVocabulary) -> bool {
        self.controlled_vocabularies.contains(&vocab)
    }

    /// Whether the object can show a given qualification state.
    pub fn can_show(&self, state: RelationNavStateClass) -> bool {
        self.applicable_states.contains(&state)
    }

    /// Whether the object can represent a given relation kind.
    pub fn represents(&self, kind: RelationKind) -> bool {
        self.relation_kinds.contains(&kind)
    }
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationNavInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built matrix satisfies the invariant.
    pub holds: bool,
}

/// The frozen relation-navigation matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationNavigationMatrix {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_relation_navigation_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable matrix id.
    pub matrix_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// The freeze gate that keeps the matrix binding current.
    pub freeze_gate_ref: String,
    /// One reviewable sentence summarizing the matrix.
    pub summary: String,
    /// The controlled-vocabulary token sets and bound source schemas.
    pub shared_vocabulary: RelationNavSharedVocabulary,
    /// The unified qualification-state vocabulary.
    pub state_vocabulary: Vec<RelationNavStateTerm>,
    /// The relation-navigation object-family entries.
    pub objects: Vec<RelationNavObjectEntry>,
    /// The computed invariants.
    pub invariants: Vec<RelationNavInvariant>,
    /// Whether raw source bodies and payloads are excluded (always true).
    pub raw_payload_excluded: bool,
}

/// Error returned when the matrix fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationNavMatrixValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for RelationNavMatrixValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "relation-navigation matrix invalid: {}", self.reason)
    }
}

impl std::error::Error for RelationNavMatrixValidationError {}

impl RelationNavigationMatrix {
    /// Returns the entry for an object family, if present.
    pub fn object(&self, object: RelationNavObjectClass) -> Option<&RelationNavObjectEntry> {
        self.objects.iter().find(|o| o.object == object)
    }

    /// Returns the state term for a state, if present.
    pub fn state_term(&self, state: RelationNavStateClass) -> Option<&RelationNavStateTerm> {
        self.state_vocabulary.iter().find(|t| t.state == state)
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

    /// Every ref string carried by the matrix, for export-safety auditing.
    fn all_refs(&self) -> impl Iterator<Item = &str> {
        let from_shared = self
            .shared_vocabulary
            .source_schema_refs
            .iter()
            .map(String::as_str);
        let from_states = self
            .state_vocabulary
            .iter()
            .flat_map(|t| t.derived_from_refs.iter().map(String::as_str));
        let from_objects = self.objects.iter().flat_map(|o| {
            o.canonical_schema_refs
                .iter()
                .map(String::as_str)
                .chain(o.produced_by_refs.iter().map(String::as_str))
                .chain(std::iter::once(o.proof_packet_ref.as_str()))
        });
        let from_gate = std::iter::once(self.freeze_gate_ref.as_str());
        from_shared
            .chain(from_states)
            .chain(from_objects)
            .chain(from_gate)
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    /// Complements the computed [`RelationNavInvariant`]s with the uniqueness and
    /// completeness checks a consumer relies on.
    pub fn validate(&self) -> Result<(), RelationNavMatrixValidationError> {
        let fail = |reason: String| Err(RelationNavMatrixValidationError { reason });

        if self.record_kind != M5_RELATION_NAVIGATION_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_RELATION_NAVIGATION_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }

        // Every object family and state is present exactly once.
        for object in RelationNavObjectClass::ALL {
            if self.objects.iter().filter(|o| o.object == object).count() != 1 {
                return fail(format!(
                    "object {} not present exactly once",
                    object.as_str()
                ));
            }
        }
        for state in RelationNavStateClass::ALL {
            if self
                .state_vocabulary
                .iter()
                .filter(|t| t.state == state)
                .count()
                != 1
            {
                return fail(format!("state {} not present exactly once", state.as_str()));
            }
        }

        // Stable ids and tokens are unique.
        if !all_unique(self.objects.iter().map(|o| o.object_id.as_str())) {
            return fail("object ids are not unique".to_owned());
        }
        if !all_unique(self.state_vocabulary.iter().map(|t| t.token.as_str())) {
            return fail("state tokens are not unique".to_owned());
        }

        // Per-object structural floor: typed, evidenced, fielded, proven.
        for entry in &self.objects {
            if entry.object_id != entry.object.object_id() {
                return fail(format!("object id mismatch for {}", entry.object.as_str()));
            }
            if entry.canonical_schema_refs.is_empty() {
                return fail(format!("object {} cites no schema", entry.object.as_str()));
            }
            if entry.produced_by_refs.is_empty() {
                return fail(format!("object {} has no producer", entry.object.as_str()));
            }
            if entry.proof_packet_ref.is_empty() {
                return fail(format!(
                    "object {} has no mapped proof packet",
                    entry.object.as_str()
                ));
            }
            if entry.applicable_states.is_empty() {
                return fail(format!(
                    "object {} declares no states",
                    entry.object.as_str()
                ));
            }
            if entry.controlled_vocabularies.is_empty() {
                return fail(format!(
                    "object {} binds no controlled vocabulary",
                    entry.object.as_str()
                ));
            }
            if entry.required_fields.is_empty() {
                return fail(format!(
                    "object {} declares no required fields",
                    entry.object.as_str()
                ));
            }
            if entry.relation_kinds.is_empty() {
                return fail(format!(
                    "object {} represents no relation kind",
                    entry.object.as_str()
                ));
            }
            for state in &entry.applicable_states {
                if self.state_term(*state).is_none() {
                    return fail(format!(
                        "object {} references undefined state {}",
                        entry.object.as_str(),
                        state.as_str()
                    ));
                }
            }
        }

        if !self.is_support_export_safe() {
            return fail("matrix is not support-export safe".to_owned());
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

/// Builds the canonical relation-navigation matrix.
///
/// Deterministic: the same bytes every call. The invariant `holds` flags are
/// computed from the built objects and states, so an inconsistent edit flips an
/// invariant rather than silently passing.
pub fn relation_navigation_matrix() -> RelationNavigationMatrix {
    let state_vocabulary = build_state_vocabulary();
    let objects = build_objects();
    let shared_vocabulary = build_shared_vocabulary(&objects);
    let invariants = compute_invariants(&objects, &state_vocabulary);

    RelationNavigationMatrix {
        record_kind: M5_RELATION_NAVIGATION_RECORD_KIND.to_owned(),
        m5_relation_navigation_schema_version: M5_RELATION_NAVIGATION_SCHEMA_VERSION,
        schema_ref: M5_RELATION_NAVIGATION_SCHEMA_REF.to_owned(),
        matrix_id: M5_RELATION_NAVIGATION_MATRIX_ID.to_owned(),
        as_of: M5_RELATION_NAVIGATION_AS_OF.to_owned(),
        freeze_gate_ref: M5_RELATION_NAVIGATION_FREEZE_GATE_REF.to_owned(),
        summary: "One frozen, typed matrix for Aureline's relation-kind navigation — navigation \
                  targets, reference occurrences, hierarchy edges, related-object relations, and \
                  rename-preview sets — across the search palette, editor assist, graph overlay, \
                  docs/help, AI context, review workspace, support export, CLI/headless, and shell \
                  continuity surfaces, with each object mapped to the proof packet that keeps it \
                  current. Relation kinds stay explicit and trustworthy: a definition is not a \
                  declaration, grep fallback never masquerades as semantic certainty, implementation \
                  and hierarchy edges preserve proof class and ambiguity, related-object navigation \
                  stays source-attributed, and rename preview exposes blocked, generated, read-only, \
                  and partial-scope candidates before any broad mutation."
            .to_owned(),
        shared_vocabulary,
        state_vocabulary,
        objects,
        invariants,
        raw_payload_excluded: true,
    }
}

fn build_state_vocabulary() -> Vec<RelationNavStateTerm> {
    RelationNavStateClass::ALL
        .iter()
        .map(|state| RelationNavStateTerm {
            state: *state,
            token: state.as_str().to_owned(),
            label: state.label().to_owned(),
            requires_disclosure: state.requires_disclosure(),
            is_fallback_proof: state.is_fallback_proof(),
            is_ambiguity: state.is_ambiguity(),
            is_rename_omission: state.is_rename_omission(),
            derived_from_refs: state.derived_from_refs(),
        })
        .collect()
}

fn field(field_id: &str, label: &str, required: bool) -> RelationNavFieldDef {
    RelationNavFieldDef {
        field_id: field_id.to_owned(),
        label: label.to_owned(),
        required,
    }
}

fn strvec(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn build_objects() -> Vec<RelationNavObjectEntry> {
    use RelationNavConsumer::*;
    use RelationNavStateClass::*;
    use RelationNavVocabulary::*;

    vec![
        RelationNavObjectEntry {
            object: RelationNavObjectClass::NavigationTarget,
            object_id: RelationNavObjectClass::NavigationTarget.object_id(),
            label: RelationNavObjectClass::NavigationTarget.label().to_owned(),
            summary: "The stable target a go-to, outline, breadcrumb, hierarchy, or search jump \
                      resolves to: a stable id, relation kind, object and anchor refs, provider \
                      family, proof class, confidence, freshness, ambiguity, scope completeness, \
                      authorship posture, and the downgrade reasons that must stay visible."
                .to_owned(),
            canonical_schema_refs: strvec(&["schemas/navigation/navigation_target.schema.json"]),
            produced_by_refs: strvec(&[
                "crates/aureline-navigation/src/target_model/mod.rs",
            ]),
            proof_packet_ref: "docs/navigation/m3/navigation_target_beta_contract.md".to_owned(),
            consumed_by: vec![
                SearchPalette,
                EditorAssist,
                GraphOverlay,
                AiContext,
                ReviewWorkspace,
                SupportExport,
                CliHeadless,
                ShellContinuity,
            ],
            relation_kinds: vec![
                RelationKind::Definition,
                RelationKind::Declaration,
                RelationKind::Implementation,
                RelationKind::Type,
            ],
            applicable_states: vec![
                ExactSemantic,
                IndexedSemantic,
                ImportedSnapshot,
                LexicalFallbackDisclosed,
                SyntaxFallbackDisclosed,
                FrameworkDerivedDisclosed,
                RuntimeObservedDisclosed,
                AmbiguousNeedsSelection,
                MultipleCandidatesRanked,
                DriftedNeedsReview,
                PartialScope,
                StaleScope,
                GeneratedBoundaryDisclosed,
                ReadOnlyProtected,
                MissingTarget,
                ScopeUnavailable,
                Unavailable,
            ],
            controlled_vocabularies: vec![
                RelationKindAxis,
                ProofClassAxis,
                Ambiguity,
                Freshness,
                Partiality,
                GeneratedRuntimeLabel,
            ],
            required_fields: vec![
                field("target_id", "Target id", true),
                field("relation_kind", "Relation kind", true),
                field("object_ref", "Object ref", true),
                field("anchor_ref", "Anchor ref", true),
                field("provider_class", "Provider class", true),
                field("proof_class", "Proof class", true),
                field("confidence", "Confidence", true),
                field("freshness", "Freshness", true),
                field("ambiguity_class", "Ambiguity class", true),
                field("scope_completeness", "Scope completeness", true),
                field("scope_ref", "Scope ref", true),
                field("generated_or_external_state", "Authorship posture", true),
                field("downgrade_reasons", "Downgrade reasons", true),
                field("evidence_refs", "Evidence refs", true),
                field("summary", "Export-safe summary", true),
            ],
            proof_class_required: true,
            carries_source_attribution: true,
            source_attribution_field: Some("anchor_ref".to_owned()),
            default_redaction: RelationNavRedactionClass::MetadataSafeDefault,
            locally_inspectable: true,
            typed_not_prose_only: true,
            boundary_note: "The target names its relation kind explicitly — a definition is never \
                            relabeled a declaration — and carries a proof class, so a lexical or \
                            syntax fallback is disclosed rather than shown as semantic certainty."
                .to_owned(),
        },
        RelationNavObjectEntry {
            object: RelationNavObjectClass::ReferenceOccurrence,
            object_id: RelationNavObjectClass::ReferenceOccurrence.object_id(),
            label: RelationNavObjectClass::ReferenceOccurrence.label().to_owned(),
            summary: "One member of a find-references, rename-preview, or evidence set: a stable \
                      occurrence id, the target it references, a source anchor, its access kind, \
                      scope, authorship posture, proof class, confidence, freshness, and scope \
                      completeness — so read/write/test-only/generated occurrences are never hidden \
                      inside an undifferentiated count."
                .to_owned(),
            canonical_schema_refs: strvec(&["schemas/navigation/semantic_result_ref.schema.json"]),
            produced_by_refs: strvec(&[
                "crates/aureline-navigation/src/target_model/mod.rs",
            ]),
            proof_packet_ref:
                "docs/navigation/semantic_navigation_and_rename_contract.md".to_owned(),
            consumed_by: vec![
                SearchPalette,
                EditorAssist,
                GraphOverlay,
                AiContext,
                ReviewWorkspace,
                SupportExport,
                CliHeadless,
            ],
            relation_kinds: vec![RelationKind::Reference, RelationKind::Call],
            applicable_states: vec![
                ExactSemantic,
                IndexedSemantic,
                ImportedSnapshot,
                LexicalFallbackDisclosed,
                SyntaxFallbackDisclosed,
                PartialScope,
                StaleScope,
                GeneratedBoundaryDisclosed,
                ReadOnlyProtected,
                ScopeUnavailable,
                Unavailable,
            ],
            controlled_vocabularies: vec![
                RelationKindAxis,
                ProofClassAxis,
                AccessKindAxis,
                Freshness,
                Partiality,
                GeneratedRuntimeLabel,
            ],
            required_fields: vec![
                field("occurrence_id", "Occurrence id", true),
                field("target_ref", "Target ref", true),
                field("anchor_ref", "Anchor ref", true),
                field("access_kind", "Access kind", true),
                field("scope_ref", "Scope ref", true),
                field("generated_or_external_state", "Authorship posture", true),
                field("proof_class", "Proof class", true),
                field("confidence", "Confidence", true),
                field("freshness", "Freshness", true),
                field("scope_completeness", "Scope completeness", true),
                field("downgrade_reasons", "Downgrade reasons", true),
                field("evidence_refs", "Evidence refs", true),
                field("summary", "Export-safe summary", true),
            ],
            proof_class_required: true,
            carries_source_attribution: true,
            source_attribution_field: Some("anchor_ref".to_owned()),
            default_redaction: RelationNavRedactionClass::MetadataSafeDefault,
            locally_inspectable: true,
            typed_not_prose_only: true,
            boundary_note: "Each occurrence carries its access kind and proof class, so a grep \
                            fallback occurrence stays labeled as lexical and a test-only or \
                            generated occurrence is never silently folded into a production count."
                .to_owned(),
        },
        RelationNavObjectEntry {
            object: RelationNavObjectClass::HierarchyEdge,
            object_id: RelationNavObjectClass::HierarchyEdge.object_id(),
            label: RelationNavObjectClass::HierarchyEdge.label().to_owned(),
            summary: "One call, runtime-call, inherit, implement, override, framework-binding, \
                      owner, or documented-by edge: a stable edge id, source and target refs, edge \
                      kind, proof class, depth, scope completeness, freshness, confidence, and any \
                      runtime/framework evidence refs — so an observed-dispatch edge is never shown \
                      as a static semantic fact."
                .to_owned(),
            canonical_schema_refs: strvec(&["schemas/navigation/semantic_result_ref.schema.json"]),
            produced_by_refs: strvec(&[
                "crates/aureline-navigation/src/target_model/mod.rs",
            ]),
            proof_packet_ref:
                "fixtures/navigation/m3/target_accuracy/hierarchy_framework_runtime_edges.yaml"
                    .to_owned(),
            consumed_by: vec![
                GraphOverlay,
                EditorAssist,
                AiContext,
                ReviewWorkspace,
                SupportExport,
                CliHeadless,
            ],
            relation_kinds: vec![
                RelationKind::Call,
                RelationKind::Implementation,
                RelationKind::Type,
                RelationKind::RouteBinding,
                RelationKind::OwnerLink,
                RelationKind::DocLink,
            ],
            applicable_states: vec![
                ExactSemantic,
                IndexedSemantic,
                ImportedSnapshot,
                FrameworkDerivedDisclosed,
                RuntimeObservedDisclosed,
                AmbiguousNeedsSelection,
                MultipleCandidatesRanked,
                PartialScope,
                StaleScope,
                ScopeUnavailable,
                Unavailable,
            ],
            controlled_vocabularies: vec![
                RelationKindAxis,
                ProofClassAxis,
                Ambiguity,
                Freshness,
                Partiality,
            ],
            required_fields: vec![
                field("edge_id", "Edge id", true),
                field("source_ref", "Source ref", true),
                field("target_ref", "Target ref", true),
                field("edge_kind", "Edge kind", true),
                field("proof_class", "Proof class", true),
                field("depth", "Depth", true),
                field("scope_completeness", "Scope completeness", true),
                field("freshness", "Freshness", true),
                field("confidence", "Confidence", true),
                field(
                    "runtime_or_framework_evidence_refs",
                    "Runtime / framework evidence refs",
                    false,
                ),
                field("downgrade_reasons", "Downgrade reasons", true),
                field("summary", "Export-safe summary", true),
            ],
            proof_class_required: true,
            carries_source_attribution: true,
            source_attribution_field: Some("source_ref".to_owned()),
            default_redaction: RelationNavRedactionClass::MetadataSafeDefault,
            locally_inspectable: true,
            typed_not_prose_only: true,
            boundary_note: "Every edge preserves its proof class, depth, and ambiguity; a \
                            framework-derived or runtime-observed edge carries its evidence refs and \
                            is disclosed, never collapsed into an unqualified static-call claim."
                .to_owned(),
        },
        RelationNavObjectEntry {
            object: RelationNavObjectClass::RelatedObjectRelation,
            object_id: RelationNavObjectClass::RelatedObjectRelation.object_id(),
            label: RelationNavObjectClass::RelatedObjectRelation.label().to_owned(),
            summary: "A source-attributed link to a related target — its type, an implementation, \
                      an owner or steward, a route binding, a doc anchor, or a generated pair — \
                      expressed as a disambiguation set over candidate target refs with a selection \
                      policy, ambiguity class, freshness, and scope completeness."
                .to_owned(),
            canonical_schema_refs: strvec(&[
                "schemas/navigation/navigation_artifacts.schema.json",
                "schemas/navigation/semantic_result_ref.schema.json",
            ]),
            produced_by_refs: strvec(&[
                "crates/aureline-navigation/src/target_model/mod.rs",
            ]),
            proof_packet_ref:
                "fixtures/navigation/m3/target_accuracy/generated_boundary_disambiguation.yaml"
                    .to_owned(),
            consumed_by: vec![
                SearchPalette,
                EditorAssist,
                GraphOverlay,
                DocsHelp,
                AiContext,
                ReviewWorkspace,
                SupportExport,
                CliHeadless,
            ],
            relation_kinds: vec![
                RelationKind::Type,
                RelationKind::Implementation,
                RelationKind::RouteBinding,
                RelationKind::OwnerLink,
                RelationKind::DocLink,
            ],
            applicable_states: vec![
                ExactSemantic,
                IndexedSemantic,
                ImportedSnapshot,
                FrameworkDerivedDisclosed,
                AmbiguousNeedsSelection,
                MultipleCandidatesRanked,
                DriftedNeedsReview,
                PartialScope,
                StaleScope,
                GeneratedBoundaryDisclosed,
                ReadOnlyProtected,
                MissingTarget,
                ScopeUnavailable,
                Unavailable,
            ],
            controlled_vocabularies: vec![
                RelationKindAxis,
                ProofClassAxis,
                Ambiguity,
                Freshness,
                Partiality,
                GeneratedRuntimeLabel,
            ],
            required_fields: vec![
                field("set_id", "Relation set id", true),
                field("requested_relation", "Requested relation kind", true),
                field("candidate_target_refs", "Candidate target refs", true),
                field("selection_policy", "Selection policy", true),
                field("created_at", "Created at", true),
                field("ambiguity_class", "Ambiguity class", true),
                field("confidence", "Confidence", true),
                field("freshness", "Freshness", true),
                field("scope_completeness", "Scope completeness", true),
                field("downgrade_reasons", "Downgrade reasons", true),
                field("evidence_refs", "Evidence refs", true),
                field("summary", "Export-safe summary", true),
            ],
            proof_class_required: true,
            carries_source_attribution: true,
            source_attribution_field: Some("evidence_refs".to_owned()),
            default_redaction: RelationNavRedactionClass::MetadataSafeDefault,
            locally_inspectable: true,
            typed_not_prose_only: true,
            boundary_note: "Related-object navigation names its candidate target refs and evidence, \
                            so the link is source-attributed and disambiguable rather than an \
                            unsourced suggestion, and a generated or imported relation is disclosed."
                .to_owned(),
        },
        RelationNavObjectEntry {
            object: RelationNavObjectClass::RenamePreviewSet,
            object_id: RelationNavObjectClass::RenamePreviewSet.object_id(),
            label: RelationNavObjectClass::RenamePreviewSet.label().to_owned(),
            summary: "The candidate occurrences a rename would touch: the root target, the \
                      candidate occurrence refs, the blocked refs, conflict notes, sparse/partial \
                      reasons, generated-scope notes, a changed/unresolved/generated/protected/\
                      skipped count summary, an apply posture, and a redaction class — so blocked, \
                      generated, read-only, and partial-scope candidates are exposed before any \
                      broad mutation."
                .to_owned(),
            canonical_schema_refs: strvec(&["schemas/navigation/rename_preview.schema.json"]),
            produced_by_refs: strvec(&[
                "crates/aureline-navigation/src/target_model/mod.rs",
            ]),
            proof_packet_ref:
                "fixtures/navigation/m3/target_accuracy/rename_conflicts_partial_scope.yaml"
                    .to_owned(),
            consumed_by: vec![
                EditorAssist,
                SearchPalette,
                AiContext,
                ReviewWorkspace,
                SupportExport,
                CliHeadless,
            ],
            relation_kinds: vec![RelationKind::Reference, RelationKind::Definition],
            applicable_states: vec![
                ExactSemantic,
                IndexedSemantic,
                LexicalFallbackDisclosed,
                AmbiguousNeedsSelection,
                PartialScope,
                StaleScope,
                GeneratedBoundaryDisclosed,
                ReadOnlyProtected,
                RenameBlockedPendingReview,
                ScopeUnavailable,
                Unavailable,
            ],
            controlled_vocabularies: vec![
                RelationKindAxis,
                ProofClassAxis,
                Freshness,
                Partiality,
                GeneratedRuntimeLabel,
                RenameOmissionReason,
            ],
            required_fields: vec![
                field("rename_preview_id", "Rename preview id", true),
                field("root_target_ref", "Root target ref", true),
                field("candidate_occurrence_refs", "Candidate occurrence refs", true),
                field("blocked_refs", "Blocked refs", true),
                field("conflict_notes", "Conflict notes", true),
                field("sparse_or_partial_reasons", "Sparse / partial reasons", true),
                field("generated_scope_notes", "Generated-scope notes", true),
                field("count_summary", "Count summary", true),
                field("proof_class", "Proof class", true),
                field("confidence", "Confidence", true),
                field("freshness", "Freshness", true),
                field("scope_completeness", "Scope completeness", true),
                field("apply_posture", "Apply posture", true),
                field("redaction_class", "Redaction class", true),
                field("evidence_refs", "Evidence refs", true),
                field("summary", "Export-safe summary", true),
            ],
            proof_class_required: true,
            carries_source_attribution: true,
            source_attribution_field: Some("candidate_occurrence_refs".to_owned()),
            default_redaction: RelationNavRedactionClass::SummaryAndRefsOnly,
            locally_inspectable: true,
            typed_not_prose_only: true,
            boundary_note: "The preview surfaces blocked, generated, read-only, conflicting, and \
                            partial-scope candidates and an apply posture that blocks direct apply \
                            until reviewed, so no broad rename runs on an unproven or partial set."
                .to_owned(),
        },
        RelationNavObjectEntry {
            object: RelationNavObjectClass::RelationFallbackVocabulary,
            object_id: RelationNavObjectClass::RelationFallbackVocabulary.object_id(),
            label: RelationNavObjectClass::RelationFallbackVocabulary.label().to_owned(),
            summary: "The dictionary every other object resolves against: the closed relation-kind \
                      set, the proof classes (direct/indexed semantic versus lexical, syntax, \
                      imported, framework, runtime, and unavailable fallback), the access kinds, \
                      ambiguity, freshness, partiality, generated/runtime labels, and rename \
                      omission reasons — so relation kinds and fallback disclosure mean the same \
                      thing on every surface."
                .to_owned(),
            canonical_schema_refs: strvec(&["schemas/navigation/navigation_target.schema.json"]),
            produced_by_refs: strvec(&[
                "crates/aureline-navigation/src/target_model/mod.rs",
            ]),
            proof_packet_ref: "docs/navigation/m3/navigation_target_beta_contract.md".to_owned(),
            consumed_by: vec![
                SearchPalette,
                EditorAssist,
                GraphOverlay,
                DocsHelp,
                AiContext,
                ReviewWorkspace,
                SupportExport,
                CliHeadless,
                ShellContinuity,
            ],
            relation_kinds: REQUIRED_RELATION_KINDS.to_vec(),
            applicable_states: RelationNavStateClass::ALL.to_vec(),
            controlled_vocabularies: RelationNavVocabulary::ALL.to_vec(),
            required_fields: vec![
                field("relation_kind", "Relation kind axis", true),
                field("proof_class", "Proof class axis", true),
                field("access_kind", "Access kind axis", true),
                field("ambiguity_class", "Ambiguity axis", true),
                field("freshness", "Freshness axis", true),
                field("scope_completeness", "Partiality axis", true),
                field("generated_or_external_state", "Generated / runtime label axis", true),
                field("rename_apply_posture", "Rename omission reason axis", true),
            ],
            proof_class_required: true,
            carries_source_attribution: false,
            source_attribution_field: None,
            default_redaction: RelationNavRedactionClass::MetadataSafeDefault,
            locally_inspectable: true,
            typed_not_prose_only: true,
            boundary_note: "The vocabulary is closed and shared: definition, declaration, \
                            implementation, reference, type, call, route-binding, owner-link, and \
                            doc-link are distinct tokens, and every fallback proof class is named so \
                            it can be disclosed rather than hidden."
                .to_owned(),
        },
    ]
}

fn build_shared_vocabulary(objects: &[RelationNavObjectEntry]) -> RelationNavSharedVocabulary {
    let def = |token: &str, label: &str| RelationNavTokenDef {
        token: token.to_owned(),
        label: label.to_owned(),
    };

    // Relation kinds and access kinds are derived from the live `target_model`
    // enums so the matrix can never silently diverge from the object model.
    let relation_kinds = REQUIRED_RELATION_KINDS
        .iter()
        .map(|k| def(k.as_str(), relation_kind_label(*k)))
        .collect();

    let proof_classes = PROOF_CLASS_ORDER
        .iter()
        .map(|p| def(p.as_str(), proof_class_label(*p)))
        .collect();

    let access_kinds = ACCESS_KIND_ORDER
        .iter()
        .map(|a| def(a.as_str(), access_kind_label(*a)))
        .collect();

    // The bound source schemas are exactly the union of every object's cited
    // schema, plus the rename-preview and breadcrumb schemas the continuity lane
    // leans on.
    let mut source_schema_refs: Vec<String> = objects
        .iter()
        .flat_map(|o| o.canonical_schema_refs.iter().cloned())
        .chain(std::iter::once(
            "schemas/navigation/breadcrumb_segment.schema.json".to_owned(),
        ))
        .collect();
    source_schema_refs.sort();
    source_schema_refs.dedup();

    RelationNavSharedVocabulary {
        relation_kinds,
        proof_classes,
        access_kinds,
        ambiguity_classes: vec![
            def("unambiguous", "Unambiguous"),
            def("ambiguous_needs_selection", "Ambiguous — needs selection"),
            def("multiple_candidates_ranked", "Multiple candidates ranked"),
            def("drifted_needs_review", "Drifted — needs review"),
            def("missing_target", "Missing target"),
            def("scope_unavailable", "Scope unavailable"),
        ],
        freshness_classes: vec![
            def("authoritative_live", "Authoritative live"),
            def("warm_cached", "Warm cached"),
            def("degraded_cached", "Degraded cached"),
            def("stale", "Stale"),
            def("unverified", "Unverified"),
        ],
        partiality_classes: vec![
            def("complete_for_declared_scope", "Complete for declared scope"),
            def("partial_for_declared_scope", "Partial for declared scope"),
            def("stale_for_declared_scope", "Stale for declared scope"),
            def(
                "unavailable_for_declared_scope",
                "Unavailable for declared scope",
            ),
        ],
        generated_runtime_labels: vec![
            def("authored_source", "Authored source"),
            def("generated_source", "Generated source"),
            def("external_dependency", "External dependency"),
            def("read_only_source", "Read-only source"),
            def("imported_snapshot", "Imported snapshot"),
        ],
        rename_omission_reasons: vec![
            def(
                "blocked_by_policy_or_protected",
                "Blocked by policy or protected source",
            ),
            def(
                "blocked_generated_or_paired",
                "Blocked generated or paired artifact",
            ),
            def("blocked_read_only", "Blocked read-only source"),
            def(
                "blocked_missing_anchor",
                "Blocked — candidate could not be anchored",
            ),
            def(
                "blocked_pending_scope_review",
                "Blocked pending scope review",
            ),
            def(
                "blocked_pending_refresh",
                "Blocked pending provider / index refresh",
            ),
            def(
                "conflict_shadowing_or_alias",
                "Conflict — shadowing or alias ambiguity",
            ),
            def("sparse_or_partial_scope", "Sparse or partial scope"),
            def(
                "inspect_only_unavailable",
                "Inspect-only — apply unavailable",
            ),
        ],
        redaction_classes: vec![
            def("metadata_safe_default", "Metadata-safe default"),
            def("summary_and_refs_only", "Summary and refs only"),
            def("operator_only_restricted", "Operator-only restricted"),
            def("internal_support_restricted", "Internal-support restricted"),
        ],
        consumer_classes: RelationNavConsumer::ALL
            .iter()
            .map(|c| def(c.as_str(), consumer_label(*c)))
            .collect(),
        source_schema_refs,
    }
}

/// The proof classes in matrix order (strongest first).
const PROOF_CLASS_ORDER: [ProofClass; 9] = [
    ProofClass::DirectSemantic,
    ProofClass::IndexedSemantic,
    ProofClass::LexicalFallback,
    ProofClass::SyntaxFallback,
    ProofClass::ImportedEvidence,
    ProofClass::FrameworkDerived,
    ProofClass::RuntimeObserved,
    ProofClass::AiInferred,
    ProofClass::Unavailable,
];

/// The access kinds in matrix order.
const ACCESS_KIND_ORDER: [AccessKind; 8] = [
    AccessKind::Read,
    AccessKind::Write,
    AccessKind::Call,
    AccessKind::Inherit,
    AccessKind::Import,
    AccessKind::Export,
    AccessKind::TestOnly,
    AccessKind::Generated,
];

fn relation_kind_label(kind: RelationKind) -> &'static str {
    match kind {
        RelationKind::Definition => "Definition",
        RelationKind::Declaration => "Declaration",
        RelationKind::Implementation => "Implementation",
        RelationKind::Reference => "Reference",
        RelationKind::Type => "Type",
        RelationKind::Call => "Call",
        RelationKind::RouteBinding => "Route binding",
        RelationKind::OwnerLink => "Owner link",
        RelationKind::DocLink => "Doc link",
    }
}

fn proof_class_label(proof: ProofClass) -> &'static str {
    match proof {
        ProofClass::DirectSemantic => "Direct semantic",
        ProofClass::IndexedSemantic => "Indexed semantic",
        ProofClass::LexicalFallback => "Lexical fallback",
        ProofClass::SyntaxFallback => "Syntax fallback",
        ProofClass::ImportedEvidence => "Imported evidence",
        ProofClass::FrameworkDerived => "Framework derived",
        ProofClass::RuntimeObserved => "Runtime observed",
        ProofClass::AiInferred => "AI inferred",
        ProofClass::Unavailable => "Unavailable",
    }
}

fn access_kind_label(access: AccessKind) -> &'static str {
    match access {
        AccessKind::Read => "Read",
        AccessKind::Write => "Write",
        AccessKind::Call => "Call",
        AccessKind::Inherit => "Inherit",
        AccessKind::Import => "Import",
        AccessKind::Export => "Export",
        AccessKind::TestOnly => "Test-only",
        AccessKind::Generated => "Generated",
    }
}

fn consumer_label(consumer: RelationNavConsumer) -> &'static str {
    match consumer {
        RelationNavConsumer::SearchPalette => "Search palette",
        RelationNavConsumer::EditorAssist => "Editor assist",
        RelationNavConsumer::GraphOverlay => "Graph overlay",
        RelationNavConsumer::DocsHelp => "Docs / Help",
        RelationNavConsumer::AiContext => "AI context",
        RelationNavConsumer::ReviewWorkspace => "Review workspace",
        RelationNavConsumer::SupportExport => "Support export",
        RelationNavConsumer::CliHeadless => "CLI / headless",
        RelationNavConsumer::ShellContinuity => "Shell continuity",
    }
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> RelationNavInvariant {
    RelationNavInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(
    objects: &[RelationNavObjectEntry],
    states: &[RelationNavStateTerm],
) -> Vec<RelationNavInvariant> {
    use RelationNavObjectClass::*;
    use RelationNavStateClass::*;
    use RelationNavVocabulary::*;

    let object = |class: RelationNavObjectClass| objects.iter().find(|o| o.object == class);
    let navigable = || objects.iter().filter(|o| o.object.is_navigable_object());

    let mut out = Vec::new();

    // Every object points at a canonical object and a producer.
    out.push(invariant(
        "relation_nav.canonical_object_identity",
        "Every relation-navigation object cites at least one canonical boundary schema and at least \
         one producing crate module, so search/graph/docs/editor point at the same objects.",
        objects
            .iter()
            .all(|o| !o.canonical_schema_refs.is_empty() && !o.produced_by_refs.is_empty()),
    ));

    // Release-automation binding: every object maps to a proof packet. A claimed
    // relation-navigation object with no mapped proof row flips this false and
    // fails promotion.
    out.push(invariant(
        "relation_nav.proof_packet_mapped",
        "Every relation-navigation object maps to a non-empty proof packet that keeps it current, \
         so stable promotion fails when a claimed relation-navigation surface lacks a mapped proof \
         row.",
        objects.iter().all(|o| !o.proof_packet_ref.is_empty()),
    ));

    // Definition is not declaration.
    out.push(invariant(
        "relation_nav.definition_distinct_from_declaration",
        "The relation-kind vocabulary keeps definition and declaration as distinct tokens, and the \
         navigation target can represent both, so a definition jump is never relabeled a \
         declaration.",
        RelationKind::Definition.as_str() != RelationKind::Declaration.as_str()
            && object(NavigationTarget).is_some_and(|o| {
                o.represents(RelationKind::Definition) && o.represents(RelationKind::Declaration)
            }),
    ));

    // Grep fallback never masquerades as semantic certainty.
    out.push(invariant(
        "relation_nav.fallback_never_masquerades",
        "Every navigable object that can show a fallback proof state binds the proof-class \
         vocabulary, and every fallback state requires disclosure, so a lexical/syntax/imported/\
         framework/runtime result is labeled rather than shown as semantic certainty.",
        states
            .iter()
            .all(|t| !t.is_fallback_proof || t.requires_disclosure)
            && navigable().all(|o| {
                let shows_fallback = o.applicable_states.iter().any(|s| s.is_fallback_proof());
                !shows_fallback || o.binds(ProofClassAxis)
            }),
    ));

    // Hierarchy edges preserve proof class and ambiguity.
    out.push(invariant(
        "relation_nav.hierarchy_preserves_proof_and_ambiguity",
        "The hierarchy-edge object binds the proof-class and ambiguity vocabularies and can show \
         ambiguous, partial, framework-derived, and runtime-observed states, so an implementation \
         or hierarchy edge preserves its proof class and ambiguity.",
        object(HierarchyEdge).is_some_and(|o| {
            o.binds(ProofClassAxis)
                && o.binds(Ambiguity)
                && o.can_show(AmbiguousNeedsSelection)
                && o.can_show(PartialScope)
                && o.can_show(FrameworkDerivedDisclosed)
                && o.can_show(RuntimeObservedDisclosed)
        }),
    ));

    // Related-object navigation stays source-attributed.
    out.push(invariant(
        "relation_nav.related_object_source_attributed",
        "The related-object relation carries source attribution and a named attribution field and \
         binds the proof-class vocabulary, so a related-object link is sourced and disambiguable \
         rather than an unsourced suggestion.",
        object(RelatedObjectRelation).is_some_and(|o| {
            o.carries_source_attribution
                && o.source_attribution_field.is_some()
                && o.binds(ProofClassAxis)
        }),
    ));

    // Rename preview exposes blocked / generated / read-only / partial candidates.
    out.push(invariant(
        "relation_nav.rename_preview_exposes_blocked",
        "The rename-preview set binds the rename-omission-reason and generated/runtime-label \
         vocabularies, can show blocked, generated, read-only, and partial-scope states, and \
         declares blocked-refs, generated-scope-notes, and a count summary, so blocked and \
         partial-scope candidates are exposed before any broad mutation.",
        object(RenamePreviewSet).is_some_and(|o| {
            o.binds(RenameOmissionReason)
                && o.binds(GeneratedRuntimeLabel)
                && o.can_show(RenameBlockedPendingReview)
                && o.can_show(GeneratedBoundaryDisclosed)
                && o.can_show(ReadOnlyProtected)
                && o.can_show(PartialScope)
                && has_field(o, "blocked_refs")
                && has_field(o, "generated_scope_notes")
                && has_field(o, "count_summary")
        }),
    ));

    // The relation-kind vocabulary is complete.
    out.push(invariant(
        "relation_nav.relation_kind_vocabulary_complete",
        "The relation / fallback vocabulary enumerates the full relation-kind set — definition, \
         declaration, implementation, reference, type, call, route-binding, owner-link, and \
         doc-link — and binds the relation-kind and proof-class axes.",
        object(RelationFallbackVocabulary).is_some_and(|o| {
            REQUIRED_RELATION_KINDS.iter().all(|k| o.represents(*k))
                && o.relation_kinds.len() == REQUIRED_RELATION_KINDS.len()
                && o.binds(RelationKindAxis)
                && o.binds(ProofClassAxis)
        }),
    ));

    // Every navigable object carries an explicit proof class.
    out.push(invariant(
        "relation_nav.every_object_carries_proof_class",
        "Every object binds the proof-class vocabulary and is marked proof-class-required, so no \
         relation is ever shown without an explicit proof class.",
        objects
            .iter()
            .all(|o| o.binds(ProofClassAxis) && o.proof_class_required),
    ));

    // Every named controlled vocabulary is bound by some object.
    out.push(invariant(
        "relation_nav.controlled_vocabulary_complete",
        "Each of the eight named controlled vocabularies — relation kind, proof class, access kind, \
         ambiguity, freshness, partiality, generated/runtime label, and rename omission reason — is \
         bound by at least one object.",
        RelationNavVocabulary::ALL
            .iter()
            .all(|v| objects.iter().any(|o| o.binds(*v))),
    ));

    // Docs/help/search/editor/support consumers all share the object model.
    out.push(invariant(
        "relation_nav.consumers_share_object_model",
        "Every consumer surface — search palette, editor assist, graph overlay, docs/help, AI \
         context, review workspace, support export, CLI/headless, and shell continuity — renders at \
         least one object, so each points at the shared model rather than re-expressing relation \
         truth ad hoc.",
        RelationNavConsumer::ALL
            .iter()
            .all(|c| objects.iter().any(|o| o.consumed_by.contains(c))),
    ));

    // Stable ids and tokens defined once and unique.
    out.push(invariant(
        "relation_nav.stable_ids_unique",
        "Object ids and state tokens are each defined once and unique, so consumers can resolve an \
         object or state by a stable token.",
        all_unique(objects.iter().map(|o| o.object_id.as_str()))
            && all_unique(states.iter().map(|t| t.token.as_str())),
    ));

    // Every object family is present.
    out.push(invariant(
        "relation_nav.all_objects_present",
        "Every governed relation-navigation object family in the matrix is present exactly once.",
        RelationNavObjectClass::ALL
            .iter()
            .all(|class| objects.iter().filter(|o| o.object == *class).count() == 1),
    ));

    // Typed, never prose-only.
    out.push(invariant(
        "relation_nav.typed_not_prose_only",
        "Every object is typed and locally inspectable: it carries state terms, required fields, \
         relation kinds, and schema refs and is never reduced to a prose-only view.",
        objects.iter().all(|o| {
            o.typed_not_prose_only
                && o.locally_inspectable
                && !o.applicable_states.is_empty()
                && !o.required_fields.is_empty()
                && !o.relation_kinds.is_empty()
                && !o.canonical_schema_refs.is_empty()
        }),
    ));

    out
}

fn has_field(entry: &RelationNavObjectEntry, field_id: &str) -> bool {
    entry.required_fields.iter().any(|f| f.field_id == field_id)
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the matrix as human-readable lines for CLI/headless and support.
pub fn relation_navigation_lines(matrix: &RelationNavigationMatrix) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Relation-navigation matrix — {} ({})",
        matrix.matrix_id, matrix.as_of
    ));
    lines.push(matrix.summary.clone());
    lines.push(format!(
        "Objects: {}  States: {}  Invariants: {}",
        matrix.objects.len(),
        matrix.state_vocabulary.len(),
        matrix.invariants.len(),
    ));

    lines.push("Objects:".to_owned());
    for o in &matrix.objects {
        let kinds: Vec<&str> = o.relation_kinds.iter().map(|k| k.as_str()).collect();
        let vocab: Vec<&str> = o
            .controlled_vocabularies
            .iter()
            .map(|v| v.as_str())
            .collect();
        let states: Vec<&str> = o.applicable_states.iter().map(|s| s.as_str()).collect();
        lines.push(format!(
            "  - {} [{}] source_attributed={} proof_class_required={}",
            o.object.as_str(),
            o.object_id,
            o.carries_source_attribution,
            o.proof_class_required,
        ));
        lines.push(format!("      {}", o.summary));
        lines.push(format!("      relation_kinds: {}", kinds.join(", ")));
        lines.push(format!("      vocabularies: {}", vocab.join(", ")));
        lines.push(format!("      states: {}", states.join(", ")));
        lines.push(format!(
            "      schemas: {}",
            o.canonical_schema_refs.join(", ")
        ));
        lines.push(format!("      proof: {}", o.proof_packet_ref));
    }

    lines.push("Invariants:".to_owned());
    for i in &matrix.invariants {
        lines.push(format!(
            "  - [{}] {}",
            if i.holds { "ok" } else { "FAIL" },
            i.invariant_id
        ));
    }

    lines
}
