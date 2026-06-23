//! Relation-kind resolution: the no-silent-aliasing resolver that turns a
//! Go to Definition / Declaration / Implementation command into a distinct,
//! relation-kind-explicit outcome.
//!
//! The [`target_model`](crate::target_model) already freezes the typed
//! [`navigation target`](crate::target_model::NavigationTarget) and
//! [`disambiguation set`](crate::target_model::NavigationDisambiguationSet)
//! objects, and the
//! [`relation-navigation matrix`](crate::m5_relation_navigation) freezes the
//! governance vocabulary over them. What was still implicit is the *resolution*
//! step: given a navigation command and the candidate targets one or more
//! providers returned, how does Aureline pick an outcome without letting
//! definition, declaration, and implementation silently alias one another?
//!
//! This module is that step. [`resolve_navigation`] is a pure function over a
//! typed [`NavigationRequest`] that produces a [`NavigationResolution`] obeying
//! three rules the spec requires:
//!
//! 1. **Distinct relation kinds.** A command resolves only against candidates
//!    whose [`relation_kind`](crate::target_model::NavigationTarget::relation_kind)
//!    matches the requested kind. A definition is never relabeled a declaration;
//!    the resolution records the relation kind it actually navigated.
//! 2. **Open disambiguation instead of guessing.** When more than one admissible
//!    candidate exists — where choosing one over another could change behavior or
//!    meaning — the resolver opens a [`disambiguation set`](crate::target_model::NavigationDisambiguationSet)
//!    carrying provider, freshness, and ambiguity truth rather than picking a
//!    best target silently.
//! 3. **No silent aliasing.** When provider depth cannot serve the requested
//!    relation, the resolver either offers a *disclosed* fallback — the related
//!    target with its real relation kind preserved, a [`DowngradeReason::MissingProvider`]
//!    flag, and a fallback note — or reports the command [`Unavailable`](ResolutionDisposition::Unavailable).
//!    It never substitutes a different relation kind under the requested label.
//!
//! Every resolution carries the request id, the navigated relation kind, the
//! provider/proof/freshness/ambiguity it resolved against, and a one-sentence
//! replay explanation, so a support or debug packet can reconstruct *which*
//! relation kind Aureline navigated and *why*. [`relation_resolution_set`] freezes
//! a deterministic corpus of resolutions whose [`RelationResolutionInvariant`]
//! flags are computed from the resolver's own output, so the checked-in fixture
//! and the freeze gate pin the contract byte-for-byte and any regression in the
//! resolver flips an invariant and fails CI. The records carry no source bodies,
//! raw paths, provider payloads, URLs, hostnames, or credentials — only opaque
//! object handles, stable tokens, and short reviewable sentences — so they are
//! safe for support export.

use serde::{Deserialize, Serialize};

use crate::target_model::{
    AmbiguityClass, DowngradeReason, FreshnessClass, GeneratedOrExternalState,
    NavigationConfidence, NavigationDisambiguationSet, NavigationTarget, NavigationTargetRef,
    ProofClass, ProviderClass, RelationKind, ScopeCompleteness,
};

#[cfg(test)]
mod tests;

/// Schema version for the relation-resolution corpus.
pub const RELATION_RESOLUTION_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the relation-resolution corpus.
pub const RELATION_RESOLUTION_SCHEMA_REF: &str =
    "schemas/navigation/relation_navigation_resolution.schema.json";

/// Stable record-kind tag for the relation-resolution corpus.
pub const RELATION_RESOLUTION_RECORD_KIND: &str = "relation_navigation_resolution_set";

/// Stable id for the canonical relation-resolution corpus.
pub const RELATION_RESOLUTION_SET_ID: &str = "relation-navigation-resolution:set:0001";

/// Evaluation stamp for the canonical corpus. Held as a constant so the canonical
/// binding stays deterministic and the fixture freezes byte-for-byte.
pub const RELATION_RESOLUTION_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The freeze gate that keeps the corpus binding current. Stable promotion runs
/// this gate; it fails when the in-code corpus drifts from the checked-in fixture
/// or any invariant flips.
pub const RELATION_RESOLUTION_FREEZE_GATE_REF: &str =
    "crates/aureline-navigation/tests/relation_navigation_resolution.rs";

/// Reviewer doc for the relation-resolution contract.
pub const RELATION_RESOLUTION_DOC_REF: &str = "docs/navigation/relation_navigation_resolution.md";

/// Evidence companion for the relation-resolution corpus.
pub const RELATION_RESOLUTION_ARTIFACT_REF: &str =
    "artifacts/navigation/relation_navigation_resolution.md";

/// Repo-relative path of the checked-in canonical corpus.
pub const RELATION_RESOLUTION_FIXTURE_REF: &str =
    "fixtures/navigation/relation_navigation_resolution/canonical_resolutions.json";

// ---------------------------------------------------------------------------
// Navigation command.
// ---------------------------------------------------------------------------

/// A relation-kind navigation command issued by a user, CLI caller, or AI tool.
///
/// Each command maps to exactly one requested [`RelationKind`]; the resolver keeps
/// these distinct so a Go to Declaration never silently resolves to a definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavigationCommand {
    /// Go to Definition — resolves to the definition site.
    GoToDefinition,
    /// Go to Declaration — resolves to the declaration / signature surface.
    GoToDeclaration,
    /// Go to Implementation — resolves to an implementation candidate.
    GoToImplementation,
    /// Go to Type Definition — resolves to the type/schema/interface target.
    GoToTypeDefinition,
}

impl NavigationCommand {
    /// All commands, in canonical order.
    pub const ALL: [Self; 4] = [
        Self::GoToDefinition,
        Self::GoToDeclaration,
        Self::GoToImplementation,
        Self::GoToTypeDefinition,
    ];

    /// Returns the relation kind this command requests.
    pub const fn requested_relation(self) -> RelationKind {
        match self {
            Self::GoToDefinition => RelationKind::Definition,
            Self::GoToDeclaration => RelationKind::Declaration,
            Self::GoToImplementation => RelationKind::Implementation,
            Self::GoToTypeDefinition => RelationKind::Type,
        }
    }

    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GoToDefinition => "go_to_definition",
            Self::GoToDeclaration => "go_to_declaration",
            Self::GoToImplementation => "go_to_implementation",
            Self::GoToTypeDefinition => "go_to_type_definition",
        }
    }

    /// Returns a human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::GoToDefinition => "Go to Definition",
            Self::GoToDeclaration => "Go to Declaration",
            Self::GoToImplementation => "Go to Implementation",
            Self::GoToTypeDefinition => "Go to Type Definition",
        }
    }
}

// ---------------------------------------------------------------------------
// Request.
// ---------------------------------------------------------------------------

/// The relation-kind reach of the providers admitted for one request.
///
/// `resolvable_relations` is the set of relation kinds the admitted providers can
/// actually produce for this symbol. When the requested kind is absent from this
/// set, the resolver treats provider depth as insufficient and refuses to alias a
/// different relation kind under the requested label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderReach {
    /// Relation kinds the admitted providers can resolve for this symbol.
    pub resolvable_relations: Vec<RelationKind>,
    /// Provider families consulted for this request.
    pub provider_classes: Vec<ProviderClass>,
}

impl ProviderReach {
    /// Returns true when an admitted provider can resolve the requested relation.
    pub fn can_resolve(&self, relation: RelationKind) -> bool {
        self.resolvable_relations.contains(&relation)
    }
}

/// One relation-kind navigation request: a command plus the candidate targets the
/// admitted providers returned for the origin symbol.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationRequest {
    /// Stable request id, echoed into the resolution for replay.
    pub request_id: String,
    /// The navigation command.
    pub command: NavigationCommand,
    /// Stable object ref the request originated from.
    pub origin_object_ref: String,
    /// Stable anchor ref the request originated from.
    pub origin_anchor_ref: String,
    /// The relation-kind reach of the admitted providers.
    pub provider_reach: ProviderReach,
    /// Candidate targets the providers returned, in provider order.
    pub candidates: Vec<NavigationTarget>,
}

// ---------------------------------------------------------------------------
// Resolved target.
// ---------------------------------------------------------------------------

/// A distinct, relation-kind-explicit target produced by resolving a navigation
/// command.
///
/// This is the spec's distinct definition/declaration/implementation object: it
/// carries the target ref and anchor, provider class, confidence, freshness, an
/// ambiguity count over its sibling candidates, and any fallback notes — without
/// ever rewriting the underlying [`NavigationTarget`]'s relation kind.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationResolvedTarget {
    /// Stable target ref (id, relation kind, object ref, anchor ref).
    pub target_ref: NavigationTargetRef,
    /// Provider family that admitted the target.
    pub provider_class: ProviderClass,
    /// Proof class for the target relation.
    pub proof_class: ProofClass,
    /// Confidence class for the target.
    pub confidence: NavigationConfidence,
    /// Freshness class for the target.
    pub freshness: FreshnessClass,
    /// Ambiguity class carried from the resolved target.
    pub ambiguity_class: AmbiguityClass,
    /// Count of sibling candidates of the same relation kind considered (>= 1).
    pub ambiguity_count: usize,
    /// Authorship, generated, imported, or read-only posture.
    pub generated_or_external_state: GeneratedOrExternalState,
    /// Downgrade reasons that must stay visible on consumers.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Fallback notes describing any disclosed fallback or fallback proof class.
    pub fallback_notes: Vec<String>,
    /// Export-safe summary.
    pub summary: String,
}

impl RelationResolvedTarget {
    /// Returns the relation kind this target actually represents.
    pub fn relation_kind(&self) -> RelationKind {
        self.target_ref.relation_kind
    }
}

// ---------------------------------------------------------------------------
// Disposition and aliasing posture.
// ---------------------------------------------------------------------------

/// How a navigation request resolved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolutionDisposition {
    /// Exactly one admissible target; opened directly.
    ResolvedSingle,
    /// More than one admissible candidate; a disambiguation set was opened instead
    /// of guessing a best target.
    OpenedDisambiguation,
    /// No admissible target and no disclosed fallback; the command is unavailable.
    Unavailable,
}

impl ResolutionDisposition {
    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResolvedSingle => "resolved_single",
            Self::OpenedDisambiguation => "opened_disambiguation",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Whether the navigated relation kind matched the request, was a disclosed
/// fallback, or could not be served.
///
/// There is deliberately no "silent alias" variant: the resolver can only ever
/// preserve the requested kind, disclose a different one, or decline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AliasingPosture {
    /// The navigated relation kind equals the requested kind.
    NoAlias,
    /// A different relation kind was offered, disclosed as a fallback with reasons;
    /// the relation kind is preserved, never relabeled.
    DisclosedFallback,
    /// The requested relation could not be served and no fallback was offered.
    NoResolution,
}

impl AliasingPosture {
    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAlias => "no_alias",
            Self::DisclosedFallback => "disclosed_fallback",
            Self::NoResolution => "no_resolution",
        }
    }
}

// ---------------------------------------------------------------------------
// Resolution.
// ---------------------------------------------------------------------------

/// The replayable outcome of resolving one navigation request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationResolution {
    /// Echoed request id, so support and debug packets can correlate the outcome.
    pub request_id: String,
    /// The navigation command that was resolved.
    pub command: NavigationCommand,
    /// The relation kind the command requested.
    pub requested_relation: RelationKind,
    /// How the request resolved.
    pub disposition: ResolutionDisposition,
    /// Whether the navigated relation matched, was disclosed, or was declined.
    pub aliasing_posture: AliasingPosture,
    /// The relation kind actually navigated, present unless the command is
    /// unavailable or a disambiguation set is open. Equals `requested_relation`
    /// only under [`AliasingPosture::NoAlias`].
    pub navigated_relation: Option<RelationKind>,
    /// The single selected target, when one was opened directly.
    pub selected_target: Option<RelationResolvedTarget>,
    /// The disambiguation set, when more than one candidate was offered.
    pub disambiguation_set: Option<NavigationDisambiguationSet>,
    /// Every candidate target ref considered, in provider order.
    pub considered_target_refs: Vec<NavigationTargetRef>,
    /// Provider families consulted, deduplicated and ordered.
    pub provider_classes: Vec<ProviderClass>,
    /// Aggregate proof class for the resolution.
    pub proof_class: ProofClass,
    /// Aggregate confidence for the resolution.
    pub confidence: NavigationConfidence,
    /// Aggregate freshness for the resolution.
    pub freshness: FreshnessClass,
    /// Aggregate ambiguity class for the resolution.
    pub ambiguity_class: AmbiguityClass,
    /// Number of candidates offered (0 when unavailable, 1 when single, N for a set).
    pub ambiguity_count: usize,
    /// Downgrade reasons that must stay visible on consumers.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Fallback notes describing any disclosed fallback, fallback proof class, or
    /// unavailable outcome.
    pub fallback_notes: Vec<String>,
    /// One reviewable sentence stating which relation kind was navigated and why.
    pub replay_explanation: String,
    /// Export-safe summary.
    pub summary: String,
}

impl NavigationResolution {
    /// Returns true when the resolution navigated a relation kind other than the
    /// requested one.
    pub fn navigated_other_relation(&self) -> bool {
        self.navigated_relation
            .is_some_and(|relation| relation != self.requested_relation)
    }

    /// Returns true when the resolution honors the no-silent-aliasing rule: any
    /// time a different relation kind is navigated it is a disclosed fallback with
    /// a missing-provider reason and a fallback note.
    pub fn is_silent_alias_free(&self) -> bool {
        if !self.navigated_other_relation() {
            return self.aliasing_posture != AliasingPosture::DisclosedFallback
                || !self.fallback_notes.is_empty();
        }
        self.aliasing_posture == AliasingPosture::DisclosedFallback
            && self
                .downgrade_reasons
                .contains(&DowngradeReason::MissingProvider)
            && !self.fallback_notes.is_empty()
    }

    /// Returns true when the resolution must render with a visible caveat.
    pub fn requires_disclosure(&self) -> bool {
        self.aliasing_posture != AliasingPosture::NoAlias
            || self.proof_class.requires_disclosure()
            || self.confidence.requires_disclosure()
            || self.freshness.requires_disclosure()
            || self.ambiguity_class.requires_disambiguation()
            || !self.downgrade_reasons.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Resolver.
// ---------------------------------------------------------------------------

/// Resolves a navigation request into a [`NavigationResolution`] without ever
/// silently aliasing one relation kind for another.
///
/// The resolver considers only candidates whose relation kind matches the
/// requested kind. With one admissible candidate it opens it directly; with more
/// than one it opens a disambiguation set rather than guessing. With none it looks
/// for a conflatable related target only when provider depth cannot serve the
/// requested kind, and offers it as a *disclosed* fallback — preserving the real
/// relation kind — or reports the command unavailable.
pub fn resolve_navigation(request: &NavigationRequest) -> NavigationResolution {
    let requested = request.command.requested_relation();
    let considered_target_refs: Vec<NavigationTargetRef> = request
        .candidates
        .iter()
        .map(NavigationTarget::target_ref)
        .collect();

    let admissible: Vec<&NavigationTarget> = request
        .candidates
        .iter()
        .filter(|target| target.relation_kind == requested && admits(target))
        .collect();

    if admissible.len() == 1 {
        return resolve_single(
            request,
            requested,
            &considered_target_refs,
            admissible[0],
            AliasingPosture::NoAlias,
        );
    }
    if admissible.len() >= 2 {
        return resolve_disambiguation(
            request,
            requested,
            &considered_target_refs,
            &admissible,
            AliasingPosture::NoAlias,
        );
    }

    // No admissible candidate of the requested relation kind.
    let provider_can_resolve = request.provider_reach.can_resolve(requested);
    let fallback: Vec<&NavigationTarget> = if provider_can_resolve {
        Vec::new()
    } else {
        request
            .candidates
            .iter()
            .filter(|target| {
                target.relation_kind != requested
                    && is_conflatable(target.relation_kind)
                    && is_conflatable(requested)
                    && admits(target)
            })
            .collect()
    };

    match fallback.len() {
        1 => resolve_single(
            request,
            requested,
            &considered_target_refs,
            fallback[0],
            AliasingPosture::DisclosedFallback,
        ),
        n if n >= 2 => resolve_disambiguation(
            request,
            requested,
            &considered_target_refs,
            &fallback,
            AliasingPosture::DisclosedFallback,
        ),
        _ => resolve_unavailable(
            request,
            requested,
            &considered_target_refs,
            provider_can_resolve,
        ),
    }
}

fn resolve_single(
    request: &NavigationRequest,
    requested: RelationKind,
    considered: &[NavigationTargetRef],
    target: &NavigationTarget,
    posture: AliasingPosture,
) -> NavigationResolution {
    let navigated = target.relation_kind;
    let mut downgrade_reasons = target.downgrade_reasons.clone();
    let mut fallback_notes = Vec::new();

    if posture == AliasingPosture::DisclosedFallback {
        push_unique(&mut downgrade_reasons, DowngradeReason::MissingProvider);
        fallback_notes.push(format!(
            "No {} provider is available for this symbol, so {} cannot be served; offering the {} \
             target as a disclosed fallback. The relation kind stays '{}' and is not relabeled '{}'.",
            requested.as_str(),
            request.command.label(),
            navigated.as_str(),
            navigated.as_str(),
            requested.as_str(),
        ));
    }
    if target.proof_class.requires_disclosure() {
        fallback_notes.push(fallback_proof_note(target.proof_class, navigated));
    }

    let resolved = RelationResolvedTarget {
        target_ref: target.target_ref(),
        provider_class: target.provider_class,
        proof_class: target.proof_class,
        confidence: target.confidence,
        freshness: target.freshness,
        ambiguity_class: target.ambiguity_class,
        ambiguity_count: 1,
        generated_or_external_state: target.generated_or_external_state,
        downgrade_reasons: downgrade_reasons.clone(),
        fallback_notes: fallback_notes.clone(),
        summary: target.summary.clone(),
    };

    let replay_explanation = if posture == AliasingPosture::NoAlias {
        format!(
            "{}: resolved one {} target via {} with {} proof; relation kind preserved, no aliasing.",
            request.command.label(),
            requested.as_str(),
            provider_token(target.provider_class),
            target.proof_class.as_str(),
        )
    } else {
        format!(
            "{}: no {} provider available, so the {} target was opened as a disclosed fallback \
             ({} proof); relation kind not relabeled.",
            request.command.label(),
            requested.as_str(),
            navigated.as_str(),
            target.proof_class.as_str(),
        )
    };

    NavigationResolution {
        request_id: request.request_id.clone(),
        command: request.command,
        requested_relation: requested,
        disposition: ResolutionDisposition::ResolvedSingle,
        aliasing_posture: posture,
        navigated_relation: Some(navigated),
        selected_target: Some(resolved),
        disambiguation_set: None,
        considered_target_refs: considered.to_vec(),
        provider_classes: provider_classes(request),
        proof_class: target.proof_class,
        confidence: target.confidence,
        freshness: target.freshness,
        ambiguity_class: target.ambiguity_class,
        ambiguity_count: 1,
        downgrade_reasons,
        fallback_notes,
        replay_explanation,
        summary: format!(
            "{} resolved to a single {} target.",
            request.command.label(),
            navigated.as_str()
        ),
    }
}

fn resolve_disambiguation(
    request: &NavigationRequest,
    requested: RelationKind,
    considered: &[NavigationTargetRef],
    candidates: &[&NavigationTarget],
    posture: AliasingPosture,
) -> NavigationResolution {
    let proof_class = weakest_proof(candidates);
    let confidence = weakest_confidence(candidates);
    let freshness = weakest_freshness(candidates);
    let scope_completeness = weakest_scope(candidates);
    let ambiguity_class = aggregate_ambiguity(candidates);

    let mut downgrade_reasons = vec![DowngradeReason::AmbiguousCandidates];
    let mut fallback_notes = Vec::new();
    for candidate in candidates {
        for reason in &candidate.downgrade_reasons {
            push_unique(&mut downgrade_reasons, *reason);
        }
    }
    if posture == AliasingPosture::DisclosedFallback {
        push_unique(&mut downgrade_reasons, DowngradeReason::MissingProvider);
        fallback_notes.push(format!(
            "No {} provider is available for this symbol; {} candidates of a related kind are offered \
             as a disclosed fallback for explicit selection, with their relation kinds preserved.",
            requested.as_str(),
            candidates.len(),
        ));
    }

    let mut evidence_refs: Vec<String> = candidates
        .iter()
        .flat_map(|candidate| candidate.evidence_refs.iter().cloned())
        .collect();
    evidence_refs.sort();
    evidence_refs.dedup();

    let selection_policy = if posture == AliasingPosture::NoAlias {
        "Open the candidate the operator selects. Aureline does not auto-pick when choosing one \
         candidate over another could change behavior or meaning."
            .to_owned()
    } else {
        "Open the fallback candidate the operator selects. The requested relation has no provider, \
         so each candidate keeps its real relation kind and is disclosed as a fallback."
            .to_owned()
    };

    let set = NavigationDisambiguationSet {
        set_id: format!("{}:disambiguation", request.request_id),
        requested_relation: requested,
        candidate_target_refs: candidates
            .iter()
            .map(|candidate| candidate.target_id.clone())
            .collect(),
        selection_policy,
        created_at: RELATION_RESOLUTION_AS_OF.to_owned(),
        ambiguity_class,
        confidence,
        freshness,
        scope_completeness,
        downgrade_reasons: downgrade_reasons.clone(),
        evidence_refs,
        summary: format!(
            "{} candidates for {}; selection required.",
            candidates.len(),
            requested.as_str()
        ),
    };

    let replay_explanation = if posture == AliasingPosture::NoAlias {
        format!(
            "{}: {} admissible {} candidates, so a disambiguation set was opened instead of guessing \
             a best target.",
            request.command.label(),
            candidates.len(),
            requested.as_str(),
        )
    } else {
        format!(
            "{}: no {} provider available, so {} related candidates were opened as a disclosed \
             fallback disambiguation set rather than aliasing one as the requested kind.",
            request.command.label(),
            requested.as_str(),
            candidates.len(),
        )
    };

    NavigationResolution {
        request_id: request.request_id.clone(),
        command: request.command,
        requested_relation: requested,
        disposition: ResolutionDisposition::OpenedDisambiguation,
        aliasing_posture: posture,
        navigated_relation: None,
        selected_target: None,
        disambiguation_set: Some(set),
        considered_target_refs: considered.to_vec(),
        provider_classes: provider_classes(request),
        proof_class,
        confidence,
        freshness,
        ambiguity_class,
        ambiguity_count: candidates.len(),
        downgrade_reasons,
        fallback_notes,
        replay_explanation,
        summary: format!(
            "{} opened a disambiguation set over {} candidates.",
            request.command.label(),
            candidates.len()
        ),
    }
}

fn resolve_unavailable(
    request: &NavigationRequest,
    requested: RelationKind,
    considered: &[NavigationTargetRef],
    provider_can_resolve: bool,
) -> NavigationResolution {
    let (downgrade_reasons, fallback_notes, ambiguity_class) = if provider_can_resolve {
        (
            Vec::new(),
            vec![format!(
                "The {} provider resolved no target for this symbol; reported as unavailable rather \
                 than substituting a different relation kind.",
                requested.as_str()
            )],
            AmbiguityClass::MissingTarget,
        )
    } else {
        (
            vec![DowngradeReason::MissingProvider],
            vec![format!(
                "No {} provider is available and no related target exists to offer as a disclosed \
                 fallback; {} is unavailable rather than aliased.",
                requested.as_str(),
                request.command.label()
            )],
            AmbiguityClass::MissingTarget,
        )
    };

    NavigationResolution {
        request_id: request.request_id.clone(),
        command: request.command,
        requested_relation: requested,
        disposition: ResolutionDisposition::Unavailable,
        aliasing_posture: AliasingPosture::NoResolution,
        navigated_relation: None,
        selected_target: None,
        disambiguation_set: None,
        considered_target_refs: considered.to_vec(),
        provider_classes: provider_classes(request),
        proof_class: ProofClass::Unavailable,
        confidence: NavigationConfidence::Unavailable,
        freshness: FreshnessClass::Unverified,
        ambiguity_class,
        ambiguity_count: 0,
        downgrade_reasons,
        fallback_notes,
        replay_explanation: format!(
            "{}: no admissible {} target; reported unavailable, never aliased to another relation kind.",
            request.command.label(),
            requested.as_str(),
        ),
        summary: format!("{} is unavailable for this symbol.", request.command.label()),
    }
}

/// Returns true when a candidate is admissible: it carries an admissible proof
/// class. A candidate whose proof class is [`ProofClass::Unavailable`] is never
/// opened.
fn admits(target: &NavigationTarget) -> bool {
    target.proof_class != ProofClass::Unavailable
}

/// Returns true when a relation kind belongs to the conflatable
/// definition/declaration/implementation/type family — the family whose members a
/// fallback may be drawn from when provider depth is insufficient.
fn is_conflatable(relation: RelationKind) -> bool {
    matches!(
        relation,
        RelationKind::Definition
            | RelationKind::Declaration
            | RelationKind::Implementation
            | RelationKind::Type
    )
}

fn provider_classes(request: &NavigationRequest) -> Vec<ProviderClass> {
    let mut classes: Vec<ProviderClass> = if request.candidates.is_empty() {
        request.provider_reach.provider_classes.clone()
    } else {
        request
            .candidates
            .iter()
            .map(|candidate| candidate.provider_class)
            .collect()
    };
    classes.sort();
    classes.dedup();
    classes
}

fn fallback_proof_note(proof: ProofClass, relation: RelationKind) -> String {
    format!(
        "The {} target rests on {} proof, disclosed as a fallback and never shown as semantic \
         certainty.",
        relation.as_str(),
        proof.as_str()
    )
}

fn provider_token(provider: ProviderClass) -> &'static str {
    match provider {
        ProviderClass::Syntax => "the syntax provider",
        ProviderClass::ProjectGraph => "the project graph",
        ProviderClass::LanguageServer => "a language server",
        ProviderClass::FrameworkPack => "a framework pack",
        ProviderClass::NotebookAdapter => "a notebook adapter",
        ProviderClass::GeneratedSourceBridge => "the generated-source bridge",
        ProviderClass::SearchIndex => "the search index",
        ProviderClass::RemoteIndex => "a remote index",
        ProviderClass::ImportedSnapshot => "an imported snapshot",
        ProviderClass::RuntimeObserver => "a runtime observer",
        ProviderClass::AiAssist => "AI assistance",
    }
}

fn push_unique(reasons: &mut Vec<DowngradeReason>, reason: DowngradeReason) {
    if !reasons.contains(&reason) {
        reasons.push(reason);
    }
}

fn aggregate_ambiguity(candidates: &[&NavigationTarget]) -> AmbiguityClass {
    if candidates
        .iter()
        .all(|candidate| candidate.ambiguity_class == AmbiguityClass::MultipleCandidatesRanked)
    {
        AmbiguityClass::MultipleCandidatesRanked
    } else {
        AmbiguityClass::AmbiguousNeedsSelection
    }
}

fn weakest_proof(candidates: &[&NavigationTarget]) -> ProofClass {
    candidates
        .iter()
        .map(|candidate| candidate.proof_class)
        .max_by_key(|proof| proof_rank(*proof))
        .unwrap_or(ProofClass::Unavailable)
}

fn weakest_confidence(candidates: &[&NavigationTarget]) -> NavigationConfidence {
    candidates
        .iter()
        .map(|candidate| candidate.confidence)
        .max_by_key(|confidence| confidence_rank(*confidence))
        .unwrap_or(NavigationConfidence::Unavailable)
}

fn weakest_freshness(candidates: &[&NavigationTarget]) -> FreshnessClass {
    candidates
        .iter()
        .map(|candidate| candidate.freshness)
        .max_by_key(|freshness| freshness_rank(*freshness))
        .unwrap_or(FreshnessClass::Unverified)
}

fn weakest_scope(candidates: &[&NavigationTarget]) -> ScopeCompleteness {
    candidates
        .iter()
        .map(|candidate| candidate.scope_completeness)
        .max_by_key(|scope| scope_rank(*scope))
        .unwrap_or(ScopeCompleteness::UnavailableForDeclaredScope)
}

const fn proof_rank(proof: ProofClass) -> u8 {
    match proof {
        ProofClass::DirectSemantic => 0,
        ProofClass::IndexedSemantic => 1,
        ProofClass::ImportedEvidence => 2,
        ProofClass::FrameworkDerived => 3,
        ProofClass::RuntimeObserved => 4,
        ProofClass::SyntaxFallback => 5,
        ProofClass::LexicalFallback => 6,
        ProofClass::AiInferred => 7,
        ProofClass::Unavailable => 8,
    }
}

const fn confidence_rank(confidence: NavigationConfidence) -> u8 {
    match confidence {
        NavigationConfidence::Exact => 0,
        NavigationConfidence::Indexed => 1,
        NavigationConfidence::Imported => 2,
        NavigationConfidence::WorkspaceSliceLimited => 3,
        NavigationConfidence::Partial => 4,
        NavigationConfidence::Heuristic => 5,
        NavigationConfidence::Stale => 6,
        NavigationConfidence::Unavailable => 7,
    }
}

const fn freshness_rank(freshness: FreshnessClass) -> u8 {
    match freshness {
        FreshnessClass::AuthoritativeLive => 0,
        FreshnessClass::WarmCached => 1,
        FreshnessClass::DegradedCached => 2,
        FreshnessClass::Unverified => 3,
        FreshnessClass::Stale => 4,
    }
}

const fn scope_rank(scope: ScopeCompleteness) -> u8 {
    match scope {
        ScopeCompleteness::CompleteForDeclaredScope => 0,
        ScopeCompleteness::PartialForDeclaredScope => 1,
        ScopeCompleteness::StaleForDeclaredScope => 2,
        ScopeCompleteness::UnavailableForDeclaredScope => 3,
    }
}

/// Returns true when a proof class is a non-direct fallback class that must carry
/// disclosure rather than masquerade as semantic certainty.
fn proof_is_fallback(proof: ProofClass) -> bool {
    matches!(
        proof,
        ProofClass::LexicalFallback
            | ProofClass::SyntaxFallback
            | ProofClass::ImportedEvidence
            | ProofClass::FrameworkDerived
            | ProofClass::RuntimeObserved
            | ProofClass::AiInferred
    )
}

// ---------------------------------------------------------------------------
// Frozen corpus.
// ---------------------------------------------------------------------------

/// One frozen resolution scenario: a request, the resolution the resolver
/// produces for it, and the property the scenario proves.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationResolutionScenario {
    /// Stable scenario id.
    pub scenario_id: String,
    /// Plain-language title.
    pub title: String,
    /// The navigation request.
    pub request: NavigationRequest,
    /// The resolution `resolve_navigation` produces for the request.
    pub resolution: NavigationResolution,
    /// One reviewable sentence stating what the scenario proves.
    pub expectation_note: String,
}

/// One frozen invariant over the corpus, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationResolutionInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built corpus satisfies the invariant.
    pub holds: bool,
}

/// The frozen relation-resolution corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationResolutionSet {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub relation_resolution_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable corpus id.
    pub set_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// The freeze gate that keeps the corpus binding current.
    pub freeze_gate_ref: String,
    /// One reviewable sentence summarizing the corpus.
    pub summary: String,
    /// The frozen resolution scenarios.
    pub scenarios: Vec<RelationResolutionScenario>,
    /// The computed invariants.
    pub invariants: Vec<RelationResolutionInvariant>,
    /// Whether raw source bodies and payloads are excluded (always true).
    pub raw_payload_excluded: bool,
}

/// Error returned when the corpus fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationResolutionValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for RelationResolutionValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "relation-resolution corpus invalid: {}", self.reason)
    }
}

impl std::error::Error for RelationResolutionValidationError {}

impl RelationResolutionSet {
    /// Returns the scenario with a given id, if present.
    pub fn scenario(&self, scenario_id: &str) -> Option<&RelationResolutionScenario> {
        self.scenarios
            .iter()
            .find(|scenario| scenario.scenario_id == scenario_id)
    }

    /// Returns true when every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|invariant| invariant.holds)
    }

    /// Returns true when the corpus is safe to place in a support export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded && self.all_refs().into_iter().all(is_export_safe_ref)
    }

    fn all_refs(&self) -> Vec<&str> {
        let mut refs: Vec<&str> = vec![self.schema_ref.as_str(), self.freeze_gate_ref.as_str()];
        for scenario in &self.scenarios {
            refs.push(scenario.request.origin_object_ref.as_str());
            refs.push(scenario.request.origin_anchor_ref.as_str());
            for candidate in &scenario.request.candidates {
                refs.push(candidate.object_ref.as_str());
                refs.push(candidate.anchor_ref.as_str());
                refs.push(candidate.scope_ref.as_str());
                refs.extend(candidate.evidence_refs.iter().map(String::as_str));
            }
            if let Some(target) = &scenario.resolution.selected_target {
                refs.push(target.target_ref.object_ref.as_str());
                refs.push(target.target_ref.anchor_ref.as_str());
            }
            if let Some(set) = &scenario.resolution.disambiguation_set {
                refs.extend(set.evidence_refs.iter().map(String::as_str));
            }
            for considered in &scenario.resolution.considered_target_refs {
                refs.push(considered.object_ref.as_str());
                refs.push(considered.anchor_ref.as_str());
            }
        }
        refs
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    pub fn validate(&self) -> Result<(), RelationResolutionValidationError> {
        let fail = |reason: String| Err(RelationResolutionValidationError { reason });

        if self.record_kind != RELATION_RESOLUTION_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != RELATION_RESOLUTION_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }
        if self.scenarios.is_empty() {
            return fail("corpus must carry at least one scenario".to_owned());
        }
        if !all_unique(self.scenarios.iter().map(|s| s.scenario_id.as_str())) {
            return fail("scenario ids are not unique".to_owned());
        }

        // Every scenario's stored resolution equals what the resolver produces, so
        // the fixture cannot drift from the resolver.
        for scenario in &self.scenarios {
            let produced = resolve_navigation(&scenario.request);
            if produced != scenario.resolution {
                return fail(format!(
                    "scenario {} resolution drifted from resolver output",
                    scenario.scenario_id
                ));
            }
            if !scenario.resolution.is_silent_alias_free() {
                return fail(format!(
                    "scenario {} resolution is not silent-alias free",
                    scenario.scenario_id
                ));
            }
        }

        if !self.is_support_export_safe() {
            return fail("corpus is not support-export safe".to_owned());
        }
        if !self.all_invariants_hold() {
            let failed: Vec<&str> = self
                .invariants
                .iter()
                .filter(|invariant| !invariant.holds)
                .map(|invariant| invariant.invariant_id.as_str())
                .collect();
            return fail(format!("invariants do not hold: {}", failed.join(", ")));
        }
        Ok(())
    }
}

/// Builds the canonical relation-resolution corpus.
///
/// Deterministic: the same bytes every call. Each scenario's resolution is the
/// resolver's own output, and the invariant `holds` flags are computed from those
/// resolutions, so a regression in [`resolve_navigation`] flips an invariant or
/// drifts the fixture rather than silently passing.
pub fn relation_resolution_set() -> RelationResolutionSet {
    let scenarios = build_scenarios();
    let invariants = compute_invariants(&scenarios);

    RelationResolutionSet {
        record_kind: RELATION_RESOLUTION_RECORD_KIND.to_owned(),
        relation_resolution_schema_version: RELATION_RESOLUTION_SCHEMA_VERSION,
        schema_ref: RELATION_RESOLUTION_SCHEMA_REF.to_owned(),
        set_id: RELATION_RESOLUTION_SET_ID.to_owned(),
        as_of: RELATION_RESOLUTION_AS_OF.to_owned(),
        freeze_gate_ref: RELATION_RESOLUTION_FREEZE_GATE_REF.to_owned(),
        summary: "Frozen relation-kind resolution corpus: every Go to Definition / Declaration / \
                  Implementation command resolves to a distinct relation kind, opens a disambiguation \
                  set instead of guessing when multiple candidates could change behavior, and never \
                  silently aliases one relation kind for another — it discloses a fallback or reports \
                  the command unavailable. Each resolution carries the request id, navigated relation \
                  kind, provider/proof/freshness/ambiguity truth, and a replay explanation so support \
                  and debug packets can reconstruct which relation kind was navigated and why."
            .to_owned(),
        scenarios,
        invariants,
        raw_payload_excluded: true,
    }
}

/// Renders the corpus as human-readable lines for CLI/headless and support.
pub fn relation_resolution_lines(set: &RelationResolutionSet) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Relation-resolution corpus — {} ({})",
        set.set_id, set.as_of
    ));
    lines.push(set.summary.clone());
    lines.push(format!(
        "Scenarios: {}  Invariants: {}",
        set.scenarios.len(),
        set.invariants.len()
    ));

    lines.push("Scenarios:".to_owned());
    for scenario in &set.scenarios {
        let resolution = &scenario.resolution;
        lines.push(format!("  - {} [{}]", scenario.scenario_id, scenario.title));
        lines.push(format!(
            "      command={} requested={} disposition={} posture={} navigated={}",
            resolution.command.as_str(),
            resolution.requested_relation.as_str(),
            resolution.disposition.as_str(),
            resolution.aliasing_posture.as_str(),
            resolution
                .navigated_relation
                .map(RelationKind::as_str)
                .unwrap_or("none"),
        ));
        lines.push(format!("      {}", resolution.replay_explanation));
    }

    lines.push("Invariants:".to_owned());
    for invariant in &set.invariants {
        lines.push(format!(
            "  - [{}] {}",
            if invariant.holds { "ok" } else { "FAIL" },
            invariant.invariant_id
        ));
    }

    lines
}

// ---------------------------------------------------------------------------
// Scenario builders.
// ---------------------------------------------------------------------------

/// Compact seed for a candidate [`NavigationTarget`], so each scenario reads as a
/// small table rather than a wall of struct fields.
struct Seed {
    target_id: &'static str,
    relation: RelationKind,
    provider: ProviderClass,
    proof: ProofClass,
    confidence: NavigationConfidence,
    freshness: FreshnessClass,
    ambiguity: AmbiguityClass,
    scope: ScopeCompleteness,
    generated: GeneratedOrExternalState,
    downgrades: &'static [DowngradeReason],
    summary: &'static str,
}

fn candidate(seed: Seed) -> NavigationTarget {
    NavigationTarget {
        target_id: seed.target_id.to_owned(),
        relation_kind: seed.relation,
        object_ref: format!("aureline://object/{}", seed.target_id),
        anchor_ref: format!("aureline://anchor/{}", seed.target_id),
        provider_class: seed.provider,
        proof_class: seed.proof,
        confidence: seed.confidence,
        freshness: seed.freshness,
        ambiguity_class: seed.ambiguity,
        scope_completeness: seed.scope,
        scope_ref: "aureline://scope/workspace".to_owned(),
        generated_or_external_state: seed.generated,
        downgrade_reasons: seed.downgrades.to_vec(),
        evidence_refs: vec![format!("aureline://evidence/{}", seed.target_id)],
        summary: seed.summary.to_owned(),
    }
}

fn request(
    request_id: &str,
    command: NavigationCommand,
    origin: &str,
    reach: &[RelationKind],
    providers: &[ProviderClass],
    candidates: Vec<NavigationTarget>,
) -> NavigationRequest {
    NavigationRequest {
        request_id: request_id.to_owned(),
        command,
        origin_object_ref: format!("aureline://object/{origin}"),
        origin_anchor_ref: format!("aureline://anchor/{origin}"),
        provider_reach: ProviderReach {
            resolvable_relations: reach.to_vec(),
            provider_classes: providers.to_vec(),
        },
        candidates,
    }
}

fn scenario(
    scenario_id: &str,
    title: &str,
    request: NavigationRequest,
    expectation_note: &str,
) -> RelationResolutionScenario {
    let resolution = resolve_navigation(&request);
    RelationResolutionScenario {
        scenario_id: scenario_id.to_owned(),
        title: title.to_owned(),
        request,
        resolution,
        expectation_note: expectation_note.to_owned(),
    }
}

fn build_scenarios() -> Vec<RelationResolutionScenario> {
    use DowngradeReason::*;
    use NavigationCommand::*;
    use ProviderClass::*;
    use RelationKind::*;

    vec![
        // 1. Go to Definition resolves a single exact target with no aliasing.
        scenario(
            "resolution.definition_single_exact",
            "Go to Definition resolves one exact target",
            request(
                "req.definition.exact",
                GoToDefinition,
                "symbol.handler",
                &[Definition, Reference],
                &[LanguageServer],
                vec![candidate(Seed {
                    target_id: "def.handler",
                    relation: Definition,
                    provider: LanguageServer,
                    proof: ProofClass::DirectSemantic,
                    confidence: NavigationConfidence::Exact,
                    freshness: FreshnessClass::AuthoritativeLive,
                    ambiguity: AmbiguityClass::Unambiguous,
                    scope: ScopeCompleteness::CompleteForDeclaredScope,
                    generated: GeneratedOrExternalState::AuthoredSource,
                    downgrades: &[],
                    summary: "Definition of the request handler.",
                })],
            ),
            "A single exact definition opens directly with no caveat and no aliasing.",
        ),
        // 2. Go to Declaration keeps declaration distinct from a present definition.
        scenario(
            "resolution.declaration_distinct_from_definition",
            "Go to Declaration does not select a present definition",
            request(
                "req.declaration.distinct",
                GoToDeclaration,
                "symbol.service",
                &[Definition, Declaration, Reference],
                &[LanguageServer],
                vec![
                    candidate(Seed {
                        target_id: "decl.service",
                        relation: Declaration,
                        provider: LanguageServer,
                        proof: ProofClass::DirectSemantic,
                        confidence: NavigationConfidence::Exact,
                        freshness: FreshnessClass::AuthoritativeLive,
                        ambiguity: AmbiguityClass::Unambiguous,
                        scope: ScopeCompleteness::CompleteForDeclaredScope,
                        generated: GeneratedOrExternalState::AuthoredSource,
                        downgrades: &[],
                        summary: "Declaration of the service trait.",
                    }),
                    candidate(Seed {
                        target_id: "def.service",
                        relation: Definition,
                        provider: LanguageServer,
                        proof: ProofClass::DirectSemantic,
                        confidence: NavigationConfidence::Exact,
                        freshness: FreshnessClass::AuthoritativeLive,
                        ambiguity: AmbiguityClass::Unambiguous,
                        scope: ScopeCompleteness::CompleteForDeclaredScope,
                        generated: GeneratedOrExternalState::AuthoredSource,
                        downgrades: &[],
                        summary: "Definition of the service impl.",
                    }),
                ],
            ),
            "Declaration resolves to the declaration only; the definition candidate is never picked, \
             proving definition is not declaration.",
        ),
        // 3. Go to Implementation with a single implementation resolves directly.
        scenario(
            "resolution.implementation_single",
            "Go to Implementation resolves one implementation",
            request(
                "req.implementation.single",
                GoToImplementation,
                "symbol.sole_impl",
                &[Implementation, Definition, Reference],
                &[ProjectGraph],
                vec![candidate(Seed {
                    target_id: "impl.sole",
                    relation: Implementation,
                    provider: ProjectGraph,
                    proof: ProofClass::IndexedSemantic,
                    confidence: NavigationConfidence::Indexed,
                    freshness: FreshnessClass::WarmCached,
                    ambiguity: AmbiguityClass::Unambiguous,
                    scope: ScopeCompleteness::CompleteForDeclaredScope,
                    generated: GeneratedOrExternalState::AuthoredSource,
                    downgrades: &[],
                    summary: "The sole implementation of the trait method.",
                })],
            ),
            "A single implementation opens directly as a distinct implementation target, never as a \
             definition or declaration.",
        ),
        // 4. Go to Implementation over many candidates opens disambiguation.
        scenario(
            "resolution.implementation_multi_disambiguation",
            "Go to Implementation opens a disambiguation set",
            request(
                "req.implementation.multi",
                GoToImplementation,
                "symbol.trait_method",
                &[Implementation, Definition, Reference],
                &[ProjectGraph],
                vec![
                    candidate(Seed {
                        target_id: "impl.alpha",
                        relation: Implementation,
                        provider: ProjectGraph,
                        proof: ProofClass::IndexedSemantic,
                        confidence: NavigationConfidence::Indexed,
                        freshness: FreshnessClass::WarmCached,
                        ambiguity: AmbiguityClass::AmbiguousNeedsSelection,
                        scope: ScopeCompleteness::CompleteForDeclaredScope,
                        generated: GeneratedOrExternalState::AuthoredSource,
                        downgrades: &[],
                        summary: "Implementation on type Alpha.",
                    }),
                    candidate(Seed {
                        target_id: "impl.beta",
                        relation: Implementation,
                        provider: ProjectGraph,
                        proof: ProofClass::IndexedSemantic,
                        confidence: NavigationConfidence::Indexed,
                        freshness: FreshnessClass::WarmCached,
                        ambiguity: AmbiguityClass::AmbiguousNeedsSelection,
                        scope: ScopeCompleteness::CompleteForDeclaredScope,
                        generated: GeneratedOrExternalState::AuthoredSource,
                        downgrades: &[],
                        summary: "Implementation on type Beta.",
                    }),
                    candidate(Seed {
                        target_id: "impl.gamma",
                        relation: Implementation,
                        provider: ProjectGraph,
                        proof: ProofClass::IndexedSemantic,
                        confidence: NavigationConfidence::Indexed,
                        freshness: FreshnessClass::WarmCached,
                        ambiguity: AmbiguityClass::AmbiguousNeedsSelection,
                        scope: ScopeCompleteness::CompleteForDeclaredScope,
                        generated: GeneratedOrExternalState::AuthoredSource,
                        downgrades: &[],
                        summary: "Implementation on type Gamma.",
                    }),
                ],
            ),
            "Three implementations open a disambiguation set carrying provider/freshness/ambiguity \
             truth instead of guessing a best target.",
        ),
        // 5. Go to Declaration with no declaration provider discloses a fallback.
        scenario(
            "resolution.declaration_discloses_fallback",
            "Go to Declaration discloses a definition fallback",
            request(
                "req.declaration.fallback",
                GoToDeclaration,
                "symbol.free_function",
                // The admitted providers can reach a definition but not a declaration.
                &[Definition, Reference],
                &[ProjectGraph],
                vec![candidate(Seed {
                    target_id: "def.free_function",
                    relation: Definition,
                    provider: ProjectGraph,
                    proof: ProofClass::IndexedSemantic,
                    confidence: NavigationConfidence::Indexed,
                    freshness: FreshnessClass::WarmCached,
                    ambiguity: AmbiguityClass::Unambiguous,
                    scope: ScopeCompleteness::CompleteForDeclaredScope,
                    generated: GeneratedOrExternalState::AuthoredSource,
                    downgrades: &[],
                    summary: "Definition of a free function with no separate declaration.",
                })],
            ),
            "With no declaration provider, the definition is offered as a disclosed fallback with a \
             missing-provider reason; its relation kind stays 'definition' and is not relabeled.",
        ),
        // 6. Go to Definition via grep fallback discloses lexical proof.
        scenario(
            "resolution.definition_lexical_fallback_disclosed",
            "Go to Definition via grep fallback stays disclosed",
            request(
                "req.definition.lexical",
                GoToDefinition,
                "symbol.macro_target",
                &[Definition],
                &[SearchIndex],
                vec![candidate(Seed {
                    target_id: "def.macro_target",
                    relation: Definition,
                    provider: SearchIndex,
                    proof: ProofClass::LexicalFallback,
                    confidence: NavigationConfidence::Heuristic,
                    freshness: FreshnessClass::WarmCached,
                    ambiguity: AmbiguityClass::Unambiguous,
                    scope: ScopeCompleteness::PartialForDeclaredScope,
                    generated: GeneratedOrExternalState::AuthoredSource,
                    downgrades: &[LexicalFallbackOnly],
                    summary: "Lexical match for a macro-defined symbol.",
                })],
            ),
            "A grep-only definition opens with its lexical proof class and a downgrade reason, never \
             shown as semantic certainty.",
        ),
        // 7. Go to Implementation with no candidates and no fallback is unavailable.
        scenario(
            "resolution.implementation_unavailable",
            "Go to Implementation reports unavailable",
            request(
                "req.implementation.none",
                GoToImplementation,
                "symbol.abstract_only",
                // A provider can resolve implementations but found none for this symbol.
                &[Implementation, Definition],
                &[ProjectGraph],
                Vec::new(),
            ),
            "With no implementation target found, the command reports unavailable rather than \
             aliasing a definition under the implementation label.",
        ),
    ]
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> RelationResolutionInvariant {
    RelationResolutionInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(
    scenarios: &[RelationResolutionScenario],
) -> Vec<RelationResolutionInvariant> {
    let resolutions: Vec<&NavigationResolution> = scenarios.iter().map(|s| &s.resolution).collect();

    let mut out = Vec::new();

    // Definition, declaration, and implementation each resolve distinctly.
    let resolves_distinctly = |command: NavigationCommand| {
        resolutions.iter().any(|resolution| {
            resolution.command == command
                && resolution.disposition == ResolutionDisposition::ResolvedSingle
                && resolution.aliasing_posture == AliasingPosture::NoAlias
                && resolution
                    .selected_target
                    .as_ref()
                    .is_some_and(|target| target.relation_kind() == command.requested_relation())
        })
    };
    out.push(invariant(
        "relation_resolution.distinct_definition_declaration_implementation",
        "Go to Definition, Go to Declaration, and Go to Implementation each have a scenario that \
         resolves to a single target of exactly the requested relation kind, so the three commands \
         stay distinct relation kinds.",
        resolves_distinctly(NavigationCommand::GoToDefinition)
            && resolves_distinctly(NavigationCommand::GoToDeclaration)
            && resolves_distinctly(NavigationCommand::GoToImplementation),
    ));

    // The resolver never relabels a target's relation kind.
    out.push(invariant(
        "relation_resolution.never_relabels_relation_kind",
        "Whenever a single target is selected, its navigated relation kind equals the target's own \
         relation kind, and it equals the requested kind only under the no-alias posture, so a \
         definition is never relabeled a declaration or implementation.",
        resolutions.iter().all(|resolution| {
            let selected_matches = resolution.selected_target.as_ref().map_or(true, |target| {
                resolution.navigated_relation == Some(target.relation_kind())
            });
            let no_alias_means_requested = resolution.aliasing_posture
                != AliasingPosture::NoAlias
                || resolution
                    .navigated_relation
                    .map_or(true, |relation| relation == resolution.requested_relation);
            selected_matches && no_alias_means_requested
        }),
    ));

    // Multi-target navigation opens disambiguation instead of guessing.
    out.push(invariant(
        "relation_resolution.multi_target_opens_disambiguation",
        "Every resolution offering more than one candidate opens a disambiguation set with at least \
         two candidates and selects no single target, so a multi-target navigation never guesses a \
         best target.",
        resolutions.iter().all(|resolution| {
            if resolution.ambiguity_count >= 2 {
                resolution.disposition == ResolutionDisposition::OpenedDisambiguation
                    && resolution.selected_target.is_none()
                    && resolution
                        .disambiguation_set
                        .as_ref()
                        .is_some_and(|set| set.candidate_target_refs.len() >= 2)
            } else {
                true
            }
        }),
    ));

    // No silent aliasing: any navigated other-relation is a disclosed fallback.
    out.push(invariant(
        "relation_resolution.no_silent_aliasing",
        "Every resolution is silent-alias free: a navigated relation kind that differs from the \
         requested kind is always a disclosed fallback carrying a missing-provider reason and a \
         fallback note.",
        resolutions
            .iter()
            .all(|resolution| resolution.is_silent_alias_free()),
    ));

    // A disclosed fallback always carries its disclosure.
    out.push(invariant(
        "relation_resolution.disclosed_fallback_is_evidenced",
        "Every disclosed-fallback resolution carries a missing-provider downgrade reason, a \
         non-empty fallback note, and a navigated relation kind that differs from the requested \
         kind.",
        resolutions.iter().all(|resolution| {
            resolution.aliasing_posture != AliasingPosture::DisclosedFallback
                || (resolution
                    .downgrade_reasons
                    .contains(&DowngradeReason::MissingProvider)
                    && !resolution.fallback_notes.is_empty()
                    && (resolution.navigated_other_relation()
                        || resolution.disposition == ResolutionDisposition::OpenedDisambiguation))
        }),
    ));

    // Grep / fallback proof never masquerades as semantic certainty.
    out.push(invariant(
        "relation_resolution.fallback_proof_never_semantic",
        "Every resolution resting on a fallback proof class — lexical, syntax, imported, framework, \
         runtime, or AI-inferred — carries at least one downgrade reason, so a grep fallback is \
         never shown as semantic certainty.",
        resolutions.iter().all(|resolution| {
            !proof_is_fallback(resolution.proof_class) || !resolution.downgrade_reasons.is_empty()
        }),
    ));

    // Unavailable outcomes are honest.
    out.push(invariant(
        "relation_resolution.unavailable_is_honest",
        "Every unavailable resolution carries the no-resolution posture, selects no target, opens no \
         disambiguation set, and explains itself with a downgrade reason or fallback note.",
        resolutions.iter().all(|resolution| {
            resolution.disposition != ResolutionDisposition::Unavailable
                || (resolution.aliasing_posture == AliasingPosture::NoResolution
                    && resolution.selected_target.is_none()
                    && resolution.disambiguation_set.is_none()
                    && (!resolution.downgrade_reasons.is_empty()
                        || !resolution.fallback_notes.is_empty()))
        }),
    ));

    // Every resolution is replayable.
    out.push(invariant(
        "relation_resolution.replayable_relation_and_reason",
        "Every resolution carries a non-empty request id, a navigated-or-declined relation outcome, \
         and a non-empty replay explanation, so a support or debug packet can reconstruct which \
         relation kind was navigated and why.",
        resolutions.iter().all(|resolution| {
            !resolution.request_id.trim().is_empty()
                && !resolution.replay_explanation.trim().is_empty()
                && !resolution.summary.trim().is_empty()
        }),
    ));

    // Disambiguation surfaces carry provider/freshness/ambiguity truth.
    out.push(invariant(
        "relation_resolution.disambiguation_carries_truth",
        "Every disambiguation set carries at least two candidates, a non-empty selection policy, and \
         an ambiguity class requiring selection, so the disambiguation surface shows \
         provider/freshness/ambiguity truth instead of an arbitrary pick.",
        resolutions.iter().all(|resolution| {
            resolution.disambiguation_set.as_ref().map_or(true, |set| {
                set.candidate_target_refs.len() >= 2
                    && !set.selection_policy.trim().is_empty()
                    && set.ambiguity_class.requires_disambiguation()
            })
        }),
    ));

    // Every command is covered, and the stored resolutions match the resolver.
    out.push(invariant(
        "relation_resolution.commands_covered_and_resolver_consistent",
        "The corpus covers Go to Definition, Declaration, Implementation, and Type Definition's \
         resolver paths, and every stored resolution equals the resolver's own output for its \
         request.",
        NavigationCommand::ALL.iter().all(|command| {
            // Type Definition shares the resolver path proven by the other commands;
            // require coverage of the three headline commands explicitly.
            *command == NavigationCommand::GoToTypeDefinition
                || resolutions.iter().any(|r| r.command == *command)
        }) && scenarios
            .iter()
            .all(|scenario| resolve_navigation(&scenario.request) == scenario.resolution),
    ));

    out
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
