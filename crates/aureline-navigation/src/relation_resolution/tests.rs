use super::*;
use crate::target_model::RelationKind;

fn def_candidate(target_id: &'static str) -> NavigationTarget {
    candidate(Seed {
        target_id,
        relation: RelationKind::Definition,
        provider: ProviderClass::LanguageServer,
        proof: ProofClass::DirectSemantic,
        confidence: NavigationConfidence::Exact,
        freshness: FreshnessClass::AuthoritativeLive,
        ambiguity: AmbiguityClass::Unambiguous,
        scope: ScopeCompleteness::CompleteForDeclaredScope,
        generated: GeneratedOrExternalState::AuthoredSource,
        downgrades: &[],
        summary: "Definition.",
    })
}

#[test]
fn canonical_set_validates_and_freezes() {
    let set = relation_resolution_set();
    set.validate().expect("canonical corpus validates");
    assert!(set.all_invariants_hold());
    assert!(set.is_support_export_safe());
    assert_eq!(set.scenarios.len(), 7);
    assert!(!set.invariants.is_empty());
}

#[test]
fn single_exact_definition_resolves_with_no_alias() {
    let set = relation_resolution_set();
    let resolution = &set
        .scenario("resolution.definition_single_exact")
        .expect("scenario present")
        .resolution;
    assert_eq!(
        resolution.disposition,
        ResolutionDisposition::ResolvedSingle
    );
    assert_eq!(resolution.aliasing_posture, AliasingPosture::NoAlias);
    assert_eq!(
        resolution.navigated_relation,
        Some(RelationKind::Definition)
    );
    assert!(resolution.is_silent_alias_free());
}

#[test]
fn declaration_does_not_select_definition() {
    let set = relation_resolution_set();
    let resolution = &set
        .scenario("resolution.declaration_distinct_from_definition")
        .expect("scenario present")
        .resolution;
    let selected = resolution.selected_target.as_ref().expect("a target");
    assert_eq!(selected.relation_kind(), RelationKind::Declaration);
    assert_ne!(selected.relation_kind(), RelationKind::Definition);
    assert_eq!(resolution.aliasing_posture, AliasingPosture::NoAlias);
}

#[test]
fn multiple_implementations_open_disambiguation() {
    let set = relation_resolution_set();
    let resolution = &set
        .scenario("resolution.implementation_multi_disambiguation")
        .expect("scenario present")
        .resolution;
    assert_eq!(
        resolution.disposition,
        ResolutionDisposition::OpenedDisambiguation
    );
    assert!(resolution.selected_target.is_none());
    let disambiguation = resolution
        .disambiguation_set
        .as_ref()
        .expect("a disambiguation set");
    assert_eq!(disambiguation.candidate_target_refs.len(), 3);
    assert!(disambiguation
        .downgrade_reasons
        .contains(&DowngradeReason::AmbiguousCandidates));
}

#[test]
fn missing_declaration_provider_discloses_fallback_without_relabeling() {
    let set = relation_resolution_set();
    let resolution = &set
        .scenario("resolution.declaration_discloses_fallback")
        .expect("scenario present")
        .resolution;
    assert_eq!(resolution.requested_relation, RelationKind::Declaration);
    assert_eq!(
        resolution.aliasing_posture,
        AliasingPosture::DisclosedFallback
    );
    // The navigated relation kind is preserved as the real (definition) kind.
    assert_eq!(
        resolution.navigated_relation,
        Some(RelationKind::Definition)
    );
    assert!(resolution
        .downgrade_reasons
        .contains(&DowngradeReason::MissingProvider));
    assert!(!resolution.fallback_notes.is_empty());
    assert!(resolution.is_silent_alias_free());
}

#[test]
fn lexical_definition_is_disclosed_not_semantic() {
    let set = relation_resolution_set();
    let resolution = &set
        .scenario("resolution.definition_lexical_fallback_disclosed")
        .expect("scenario present")
        .resolution;
    assert_eq!(resolution.proof_class, ProofClass::LexicalFallback);
    assert!(resolution
        .downgrade_reasons
        .contains(&DowngradeReason::LexicalFallbackOnly));
}

#[test]
fn unavailable_never_aliases() {
    let set = relation_resolution_set();
    let resolution = &set
        .scenario("resolution.implementation_unavailable")
        .expect("scenario present")
        .resolution;
    assert_eq!(resolution.disposition, ResolutionDisposition::Unavailable);
    assert_eq!(resolution.aliasing_posture, AliasingPosture::NoResolution);
    assert!(resolution.selected_target.is_none());
    assert!(resolution.navigated_relation.is_none());
    assert!(!resolution.fallback_notes.is_empty());
}

#[test]
fn fallback_disambiguation_when_multiple_related_candidates() {
    // Two definition candidates, but the command requested a declaration with no
    // declaration provider: a disclosed-fallback disambiguation set, never a guess.
    let req = request(
        "req.decl.fallback_multi",
        NavigationCommand::GoToDeclaration,
        "symbol.overloaded",
        &[RelationKind::Definition],
        &[ProviderClass::ProjectGraph],
        vec![def_candidate("def.one"), def_candidate("def.two")],
    );
    let resolution = resolve_navigation(&req);
    assert_eq!(
        resolution.disposition,
        ResolutionDisposition::OpenedDisambiguation
    );
    assert_eq!(
        resolution.aliasing_posture,
        AliasingPosture::DisclosedFallback
    );
    assert!(resolution.navigated_relation.is_none());
    assert!(resolution
        .downgrade_reasons
        .contains(&DowngradeReason::MissingProvider));
    assert!(resolution.is_silent_alias_free());
}

#[test]
fn provider_can_resolve_but_no_target_is_unavailable_not_fallback() {
    // A declaration provider exists but returns nothing, while a definition
    // candidate is present: the resolver must NOT fall back to the definition.
    let req = request(
        "req.decl.present_def",
        NavigationCommand::GoToDeclaration,
        "symbol.weird",
        &[RelationKind::Declaration, RelationKind::Definition],
        &[ProviderClass::LanguageServer],
        vec![def_candidate("def.weird")],
    );
    let resolution = resolve_navigation(&req);
    assert_eq!(resolution.disposition, ResolutionDisposition::Unavailable);
    assert_eq!(resolution.aliasing_posture, AliasingPosture::NoResolution);
    assert!(resolution.navigated_relation.is_none());
}

#[test]
fn resolution_round_trips_through_json() {
    let set = relation_resolution_set();
    let json = serde_json::to_string(&set).expect("serializes");
    let round_trip: RelationResolutionSet = serde_json::from_str(&json).expect("deserializes");
    assert_eq!(round_trip, set);
}

#[test]
fn unavailable_proof_class_candidates_are_not_admissible() {
    let mut unavailable = def_candidate("def.dead");
    unavailable.proof_class = ProofClass::Unavailable;
    let req = request(
        "req.def.dead",
        NavigationCommand::GoToDefinition,
        "symbol.dead",
        &[RelationKind::Definition],
        &[ProviderClass::LanguageServer],
        vec![unavailable],
    );
    let resolution = resolve_navigation(&req);
    assert_eq!(resolution.disposition, ResolutionDisposition::Unavailable);
}
