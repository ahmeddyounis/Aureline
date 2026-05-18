//! Projections from launch-language records into the shared navigation target model.
//!
//! These helpers keep TS/JS and Python hover/navigation alpha records from
//! becoming a private semantic payload. Definition, reference, and rename
//! consumers can use the shared `aureline-navigation` model before rendering
//! UI, CLI/headless output, review packets, AI context, or support exports.

use aureline_navigation::target_model::{
    AccessKind, AmbiguityClass, DowngradeReason, ExportRedactionClass, FreshnessClass,
    GeneratedOrExternalState, NavigationConfidence, NavigationTarget, NavigationTargetCountSummary,
    ProofClass, ProviderClass, ReferenceOccurrence, RelationKind, RenameApplyPosture,
    RenamePreviewSet, ScopeCompleteness,
};

use crate::{
    lsp_router::{
        FreshnessClass as RouterFreshnessClass, HealthState, ProviderFamily, RedactionClass,
        ScopeLimitClass,
    },
    PythonProviderSnapshot, PythonReferenceSetRecord, PythonRelationClass,
    PythonRenamePreviewCompletenessClass, PythonRenamePreviewRecord, PythonResultConfidenceClass,
    PythonSemanticResultIdentityClass, PythonSemanticResultRecord, PythonSourceAnchorKindClass,
    TsJsProviderSnapshot, TsJsReferenceSetRecord, TsJsRelationClass,
    TsJsRenamePreviewCompletenessClass, TsJsRenamePreviewRecord, TsJsResultConfidenceClass,
    TsJsSemanticResultIdentityClass, TsJsSemanticResultRecord, TsJsSourceAnchorKindClass,
};

/// Projects a TS/JS semantic result into a shared [`NavigationTarget`].
pub fn tsjs_navigation_target(record: &TsJsSemanticResultRecord) -> NavigationTarget {
    let proof_class = proof_class_for_tsjs(record);
    let confidence = tsjs_confidence(record.result_confidence_class);
    let freshness = freshness(record.provider_snapshot.freshness_class);
    let scope_completeness = tsjs_scope_completeness(record.completeness_class);
    NavigationTarget {
        target_id: record.semantic_result_id.clone(),
        relation_kind: relation_kind_for_tsjs_identity(record.semantic_result_identity_class),
        object_ref: record
            .source_anchor
            .symbol_ref
            .clone()
            .unwrap_or_else(|| record.semantic_result_id.clone()),
        anchor_ref: record.source_anchor.source_anchor_ref.clone(),
        provider_class: provider_class(record.provider_snapshot.provider_family),
        proof_class,
        confidence,
        freshness,
        ambiguity_class: ambiguity_class(
            record.ambiguity_descriptor.disambiguation_required,
            record.ambiguity_descriptor.ambiguous_candidate_count,
        ),
        scope_completeness,
        scope_ref: record.scope_descriptor.covered_scope_ref.clone(),
        generated_or_external_state: generated_state_for_tsjs_anchor(
            record.source_anchor.source_anchor_kind_class,
            record.relation_class,
        ),
        downgrade_reasons: semantic_downgrade_reasons(
            proof_class,
            confidence,
            freshness,
            scope_completeness,
            record.provider_snapshot.provider_health_class,
            &record.scope_descriptor.scope_limit_classes,
            record.ambiguity_descriptor.disambiguation_required,
            matches!(
                record.source_anchor.source_anchor_kind_class,
                TsJsSourceAnchorKindClass::GeneratedLineageAnchor
                    | TsJsSourceAnchorKindClass::ProviderOverlayAnchor
            ),
        ),
        evidence_refs: semantic_evidence_refs(
            &record.evidence_binding.source_evidence_refs,
            &record.evidence_binding.scope_caveat_refs,
            &record.router_decision_id,
        ),
        summary: record.export_safe_summary.clone(),
    }
}

/// Projects a TS/JS reference set into shared [`ReferenceOccurrence`] rows.
pub fn tsjs_reference_occurrences(record: &TsJsReferenceSetRecord) -> Vec<ReferenceOccurrence> {
    record
        .occurrence_results
        .iter()
        .map(|occurrence| {
            let proof_class = proof_class_for_tsjs(occurrence);
            let confidence = tsjs_confidence(occurrence.result_confidence_class);
            let freshness = freshness(occurrence.provider_snapshot.freshness_class);
            let scope_completeness = tsjs_scope_completeness(occurrence.completeness_class);
            ReferenceOccurrence {
                occurrence_id: occurrence.semantic_result_id.clone(),
                target_ref: record.target_symbol_ref.clone(),
                anchor_ref: occurrence.source_anchor.source_anchor_ref.clone(),
                access_kind: access_kind_for_tsjs_relation(occurrence.relation_class),
                scope_ref: occurrence.scope_descriptor.covered_scope_ref.clone(),
                generated_or_external_state: generated_state_for_tsjs_anchor(
                    occurrence.source_anchor.source_anchor_kind_class,
                    occurrence.relation_class,
                ),
                proof_class,
                confidence,
                freshness,
                scope_completeness,
                downgrade_reasons: semantic_downgrade_reasons(
                    proof_class,
                    confidence,
                    freshness,
                    scope_completeness,
                    occurrence.provider_snapshot.provider_health_class,
                    &occurrence.scope_descriptor.scope_limit_classes,
                    occurrence.ambiguity_descriptor.disambiguation_required,
                    matches!(
                        occurrence.source_anchor.source_anchor_kind_class,
                        TsJsSourceAnchorKindClass::GeneratedLineageAnchor
                            | TsJsSourceAnchorKindClass::ProviderOverlayAnchor
                    ),
                ),
                evidence_refs: semantic_evidence_refs(
                    &occurrence.evidence_binding.source_evidence_refs,
                    &occurrence.evidence_binding.scope_caveat_refs,
                    &occurrence.router_decision_id,
                ),
                summary: occurrence.export_safe_summary.clone(),
            }
        })
        .collect()
}

/// Projects a TS/JS rename preview into a shared [`RenamePreviewSet`].
pub fn tsjs_rename_preview_set(record: &TsJsRenamePreviewRecord) -> RenamePreviewSet {
    let proof_class = proof_class_for_tsjs_provider(&record.provider_snapshot);
    let confidence = tsjs_rename_confidence(record.preview_completeness_class);
    let freshness = freshness(record.provider_snapshot.freshness_class);
    let scope_completeness = tsjs_rename_scope_completeness(record.preview_completeness_class);
    RenamePreviewSet {
        rename_preview_id: record.rename_preview_id.clone(),
        root_target_ref: record.target_semantic_result_ref.clone(),
        candidate_occurrence_refs: affected_result_refs_from_tsjs_preview(record),
        blocked_refs: blocked_refs_from_tsjs_preview(record),
        conflict_notes: record
            .warning_rows
            .iter()
            .map(|warning| warning.summary.clone())
            .collect(),
        sparse_or_partial_reasons: record
            .affected_scope_rows
            .iter()
            .filter(|row| !row.coverage_limit_classes.is_empty())
            .map(|row| row.caveat_summary.clone())
            .collect(),
        generated_scope_notes: generated_notes_from_tsjs_preview(record),
        count_summary: NavigationTargetCountSummary {
            changed_count: record.count_summary.changed_count,
            unresolved_count: record.count_summary.unresolved_count,
            generated_count: record.count_summary.generated_count,
            protected_count: record.count_summary.protected_count,
            skipped_count: record.count_summary.skipped_count,
        },
        proof_class,
        confidence,
        freshness,
        scope_completeness,
        apply_posture: rename_apply_posture(record.apply_posture_class.into()),
        redaction_class: redaction_class(record.redaction_class),
        evidence_refs: rename_evidence_refs(
            &record.evidence_binding.source_evidence_refs,
            &record.evidence_binding.scope_caveat_refs,
            &record.router_decision_id,
        ),
        summary: record.export_safe_summary.clone(),
    }
}

/// Projects a Python semantic result into a shared [`NavigationTarget`].
pub fn python_navigation_target(record: &PythonSemanticResultRecord) -> NavigationTarget {
    let proof_class = proof_class_for_python(record);
    let confidence = python_confidence(record.result_confidence_class);
    let freshness = freshness(record.provider_snapshot.freshness_class);
    let scope_completeness = python_scope_completeness(record.completeness_class);
    NavigationTarget {
        target_id: record.semantic_result_id.clone(),
        relation_kind: relation_kind_for_python_identity(record.semantic_result_identity_class),
        object_ref: record
            .source_anchor
            .symbol_ref
            .clone()
            .unwrap_or_else(|| record.semantic_result_id.clone()),
        anchor_ref: record.source_anchor.source_anchor_ref.clone(),
        provider_class: provider_class(record.provider_snapshot.provider_family),
        proof_class,
        confidence,
        freshness,
        ambiguity_class: ambiguity_class(
            record.ambiguity_descriptor.disambiguation_required,
            record.ambiguity_descriptor.ambiguous_candidate_count,
        ),
        scope_completeness,
        scope_ref: record.scope_descriptor.covered_scope_ref.clone(),
        generated_or_external_state: generated_state_for_python_anchor(
            record.source_anchor.source_anchor_kind_class,
            record.relation_class,
        ),
        downgrade_reasons: semantic_downgrade_reasons(
            proof_class,
            confidence,
            freshness,
            scope_completeness,
            record.provider_snapshot.provider_health_class,
            &record.scope_descriptor.scope_limit_classes,
            record.ambiguity_descriptor.disambiguation_required,
            matches!(
                record.source_anchor.source_anchor_kind_class,
                PythonSourceAnchorKindClass::GeneratedLineageAnchor
                    | PythonSourceAnchorKindClass::ProviderOverlayAnchor
            ),
        ),
        evidence_refs: semantic_evidence_refs(
            &record.evidence_binding.source_evidence_refs,
            &record.evidence_binding.scope_caveat_refs,
            &record.router_decision_id,
        ),
        summary: record.export_safe_summary.clone(),
    }
}

/// Projects a Python reference set into shared [`ReferenceOccurrence`] rows.
pub fn python_reference_occurrences(record: &PythonReferenceSetRecord) -> Vec<ReferenceOccurrence> {
    record
        .occurrence_results
        .iter()
        .map(|occurrence| {
            let proof_class = proof_class_for_python(occurrence);
            let confidence = python_confidence(occurrence.result_confidence_class);
            let freshness = freshness(occurrence.provider_snapshot.freshness_class);
            let scope_completeness = python_scope_completeness(occurrence.completeness_class);
            ReferenceOccurrence {
                occurrence_id: occurrence.semantic_result_id.clone(),
                target_ref: record.target_symbol_ref.clone(),
                anchor_ref: occurrence.source_anchor.source_anchor_ref.clone(),
                access_kind: access_kind_for_python_relation(occurrence.relation_class),
                scope_ref: occurrence.scope_descriptor.covered_scope_ref.clone(),
                generated_or_external_state: generated_state_for_python_anchor(
                    occurrence.source_anchor.source_anchor_kind_class,
                    occurrence.relation_class,
                ),
                proof_class,
                confidence,
                freshness,
                scope_completeness,
                downgrade_reasons: semantic_downgrade_reasons(
                    proof_class,
                    confidence,
                    freshness,
                    scope_completeness,
                    occurrence.provider_snapshot.provider_health_class,
                    &occurrence.scope_descriptor.scope_limit_classes,
                    occurrence.ambiguity_descriptor.disambiguation_required,
                    matches!(
                        occurrence.source_anchor.source_anchor_kind_class,
                        PythonSourceAnchorKindClass::GeneratedLineageAnchor
                            | PythonSourceAnchorKindClass::ProviderOverlayAnchor
                    ),
                ),
                evidence_refs: semantic_evidence_refs(
                    &occurrence.evidence_binding.source_evidence_refs,
                    &occurrence.evidence_binding.scope_caveat_refs,
                    &occurrence.router_decision_id,
                ),
                summary: occurrence.export_safe_summary.clone(),
            }
        })
        .collect()
}

/// Projects a Python rename preview into a shared [`RenamePreviewSet`].
pub fn python_rename_preview_set(record: &PythonRenamePreviewRecord) -> RenamePreviewSet {
    let proof_class = proof_class_for_python_provider(&record.provider_snapshot);
    let confidence = python_rename_confidence(record.preview_completeness_class);
    let freshness = freshness(record.provider_snapshot.freshness_class);
    let scope_completeness = python_rename_scope_completeness(record.preview_completeness_class);
    RenamePreviewSet {
        rename_preview_id: record.rename_preview_id.clone(),
        root_target_ref: record.target_semantic_result_ref.clone(),
        candidate_occurrence_refs: affected_result_refs_from_python_preview(record),
        blocked_refs: blocked_refs_from_python_preview(record),
        conflict_notes: record
            .warning_rows
            .iter()
            .map(|warning| warning.summary.clone())
            .collect(),
        sparse_or_partial_reasons: record
            .affected_scope_rows
            .iter()
            .filter(|row| !row.coverage_limit_classes.is_empty())
            .map(|row| row.caveat_summary.clone())
            .collect(),
        generated_scope_notes: generated_notes_from_python_preview(record),
        count_summary: NavigationTargetCountSummary {
            changed_count: record.count_summary.changed_count,
            unresolved_count: record.count_summary.unresolved_count,
            generated_count: record.count_summary.generated_count,
            protected_count: record.count_summary.protected_count,
            skipped_count: record.count_summary.skipped_count,
        },
        proof_class,
        confidence,
        freshness,
        scope_completeness,
        apply_posture: rename_apply_posture(record.apply_posture_class.into()),
        redaction_class: redaction_class(record.redaction_class),
        evidence_refs: rename_evidence_refs(
            &record.evidence_binding.source_evidence_refs,
            &record.evidence_binding.scope_caveat_refs,
            &record.router_decision_id,
        ),
        summary: record.export_safe_summary.clone(),
    }
}

fn relation_kind_for_tsjs_identity(identity: TsJsSemanticResultIdentityClass) -> RelationKind {
    match identity {
        TsJsSemanticResultIdentityClass::Definition => RelationKind::Definition,
        TsJsSemanticResultIdentityClass::Reference
        | TsJsSemanticResultIdentityClass::ImportedOrGeneratedReference => RelationKind::Reference,
    }
}

fn relation_kind_for_python_identity(identity: PythonSemanticResultIdentityClass) -> RelationKind {
    match identity {
        PythonSemanticResultIdentityClass::Definition => RelationKind::Definition,
        PythonSemanticResultIdentityClass::Reference
        | PythonSemanticResultIdentityClass::ImportedOrGeneratedReference => {
            RelationKind::Reference
        }
    }
}

fn access_kind_for_tsjs_relation(relation: TsJsRelationClass) -> AccessKind {
    match relation {
        TsJsRelationClass::ReadReference => AccessKind::Read,
        TsJsRelationClass::WriteReference => AccessKind::Write,
        TsJsRelationClass::CallReference => AccessKind::Call,
        TsJsRelationClass::ImportOrExportReference => AccessKind::Import,
        TsJsRelationClass::GeneratedOrFrameworkReference => AccessKind::Generated,
        TsJsRelationClass::DefinitionTarget | TsJsRelationClass::NotApplicable => AccessKind::Read,
    }
}

fn access_kind_for_python_relation(relation: PythonRelationClass) -> AccessKind {
    match relation {
        PythonRelationClass::ReadReference => AccessKind::Read,
        PythonRelationClass::WriteReference => AccessKind::Write,
        PythonRelationClass::CallReference => AccessKind::Call,
        PythonRelationClass::ImportOrExportReference => AccessKind::Import,
        PythonRelationClass::GeneratedOrFrameworkReference => AccessKind::Generated,
        PythonRelationClass::DefinitionTarget | PythonRelationClass::NotApplicable => {
            AccessKind::Read
        }
    }
}

fn provider_class(provider: ProviderFamily) -> ProviderClass {
    match provider {
        ProviderFamily::Syntax => ProviderClass::Syntax,
        ProviderFamily::ProjectGraph => ProviderClass::ProjectGraph,
        ProviderFamily::LanguageServer => ProviderClass::LanguageServer,
        ProviderFamily::FrameworkPack => ProviderClass::FrameworkPack,
        ProviderFamily::NotebookAdapter => ProviderClass::NotebookAdapter,
        ProviderFamily::GeneratedSourceBridge => ProviderClass::GeneratedSourceBridge,
        ProviderFamily::AiAssist => ProviderClass::AiAssist,
    }
}

fn proof_class_for_tsjs(record: &TsJsSemanticResultRecord) -> ProofClass {
    if record.result_confidence_class == TsJsResultConfidenceClass::HeuristicallyMapped {
        return ProofClass::SyntaxFallback;
    }
    if record.relation_class == TsJsRelationClass::GeneratedOrFrameworkReference {
        return ProofClass::FrameworkDerived;
    }
    proof_class_for_tsjs_provider(&record.provider_snapshot)
}

fn proof_class_for_python(record: &PythonSemanticResultRecord) -> ProofClass {
    if record.result_confidence_class == PythonResultConfidenceClass::HeuristicallyMapped {
        return ProofClass::SyntaxFallback;
    }
    if record.relation_class == PythonRelationClass::GeneratedOrFrameworkReference {
        return ProofClass::FrameworkDerived;
    }
    proof_class_for_python_provider(&record.provider_snapshot)
}

fn proof_class_for_tsjs_provider(provider: &TsJsProviderSnapshot) -> ProofClass {
    provider_class(provider.provider_family).default_proof_class()
}

fn proof_class_for_python_provider(provider: &PythonProviderSnapshot) -> ProofClass {
    provider_class(provider.provider_family).default_proof_class()
}

fn tsjs_confidence(confidence: TsJsResultConfidenceClass) -> NavigationConfidence {
    match confidence {
        TsJsResultConfidenceClass::Exact => NavigationConfidence::Exact,
        TsJsResultConfidenceClass::Indexed => NavigationConfidence::Indexed,
        TsJsResultConfidenceClass::Partial => NavigationConfidence::Partial,
        TsJsResultConfidenceClass::Unavailable => NavigationConfidence::Unavailable,
        TsJsResultConfidenceClass::HeuristicallyMapped => NavigationConfidence::Heuristic,
        TsJsResultConfidenceClass::WorkspaceSliceLimited => {
            NavigationConfidence::WorkspaceSliceLimited
        }
    }
}

fn python_confidence(confidence: PythonResultConfidenceClass) -> NavigationConfidence {
    match confidence {
        PythonResultConfidenceClass::Exact => NavigationConfidence::Exact,
        PythonResultConfidenceClass::Indexed => NavigationConfidence::Indexed,
        PythonResultConfidenceClass::Partial => NavigationConfidence::Partial,
        PythonResultConfidenceClass::Unavailable => NavigationConfidence::Unavailable,
        PythonResultConfidenceClass::HeuristicallyMapped => NavigationConfidence::Heuristic,
        PythonResultConfidenceClass::WorkspaceSliceLimited => {
            NavigationConfidence::WorkspaceSliceLimited
        }
    }
}

fn freshness(freshness: RouterFreshnessClass) -> FreshnessClass {
    match freshness {
        RouterFreshnessClass::AuthoritativeLive => FreshnessClass::AuthoritativeLive,
        RouterFreshnessClass::WarmCached => FreshnessClass::WarmCached,
        RouterFreshnessClass::DegradedCached => FreshnessClass::DegradedCached,
        RouterFreshnessClass::Stale => FreshnessClass::Stale,
        RouterFreshnessClass::Unverified => FreshnessClass::Unverified,
    }
}

fn tsjs_scope_completeness(completeness: crate::TsJsCompletenessClass) -> ScopeCompleteness {
    match completeness {
        crate::TsJsCompletenessClass::CompleteForDeclaredScope => {
            ScopeCompleteness::CompleteForDeclaredScope
        }
        crate::TsJsCompletenessClass::PartialForDeclaredScope => {
            ScopeCompleteness::PartialForDeclaredScope
        }
        crate::TsJsCompletenessClass::StaleForDeclaredScope => {
            ScopeCompleteness::StaleForDeclaredScope
        }
        crate::TsJsCompletenessClass::UnavailableForDeclaredScope => {
            ScopeCompleteness::UnavailableForDeclaredScope
        }
    }
}

fn python_scope_completeness(completeness: crate::PythonCompletenessClass) -> ScopeCompleteness {
    match completeness {
        crate::PythonCompletenessClass::CompleteForDeclaredScope => {
            ScopeCompleteness::CompleteForDeclaredScope
        }
        crate::PythonCompletenessClass::PartialForDeclaredScope => {
            ScopeCompleteness::PartialForDeclaredScope
        }
        crate::PythonCompletenessClass::StaleForDeclaredScope => {
            ScopeCompleteness::StaleForDeclaredScope
        }
        crate::PythonCompletenessClass::UnavailableForDeclaredScope => {
            ScopeCompleteness::UnavailableForDeclaredScope
        }
    }
}

fn ambiguity_class(disambiguation_required: bool, candidate_count: u32) -> AmbiguityClass {
    if disambiguation_required {
        AmbiguityClass::AmbiguousNeedsSelection
    } else if candidate_count > 1 {
        AmbiguityClass::MultipleCandidatesRanked
    } else {
        AmbiguityClass::Unambiguous
    }
}

fn generated_state_for_tsjs_anchor(
    anchor_kind: TsJsSourceAnchorKindClass,
    relation: TsJsRelationClass,
) -> GeneratedOrExternalState {
    match (anchor_kind, relation) {
        (TsJsSourceAnchorKindClass::GeneratedLineageAnchor, _)
        | (_, TsJsRelationClass::GeneratedOrFrameworkReference) => {
            GeneratedOrExternalState::GeneratedSource
        }
        (TsJsSourceAnchorKindClass::ProviderOverlayAnchor, _) => {
            GeneratedOrExternalState::ExternalDependency
        }
        (TsJsSourceAnchorKindClass::UnresolvedAnchor, _) => {
            GeneratedOrExternalState::ImportedSnapshot
        }
        (TsJsSourceAnchorKindClass::WorkspaceSourceAnchor, _) => {
            GeneratedOrExternalState::AuthoredSource
        }
    }
}

fn generated_state_for_python_anchor(
    anchor_kind: PythonSourceAnchorKindClass,
    relation: PythonRelationClass,
) -> GeneratedOrExternalState {
    match (anchor_kind, relation) {
        (PythonSourceAnchorKindClass::GeneratedLineageAnchor, _)
        | (_, PythonRelationClass::GeneratedOrFrameworkReference) => {
            GeneratedOrExternalState::GeneratedSource
        }
        (PythonSourceAnchorKindClass::ProviderOverlayAnchor, _) => {
            GeneratedOrExternalState::ExternalDependency
        }
        (PythonSourceAnchorKindClass::UnresolvedAnchor, _) => {
            GeneratedOrExternalState::ImportedSnapshot
        }
        (PythonSourceAnchorKindClass::WorkspaceSourceAnchor, _) => {
            GeneratedOrExternalState::AuthoredSource
        }
    }
}

fn semantic_downgrade_reasons(
    proof_class: ProofClass,
    confidence: NavigationConfidence,
    freshness: FreshnessClass,
    scope_completeness: ScopeCompleteness,
    health: HealthState,
    scope_limits: &[ScopeLimitClass],
    disambiguation_required: bool,
    generated_or_imported: bool,
) -> Vec<DowngradeReason> {
    let mut reasons = Vec::new();
    if proof_class == ProofClass::SyntaxFallback {
        push_unique(&mut reasons, DowngradeReason::SyntaxFallbackOnly);
    }
    if matches!(
        proof_class,
        ProofClass::FrameworkDerived | ProofClass::RuntimeObserved
    ) {
        push_unique(&mut reasons, DowngradeReason::RuntimeOrFrameworkOnly);
    }
    if confidence.requires_disclosure() {
        match confidence {
            NavigationConfidence::WorkspaceSliceLimited | NavigationConfidence::Partial => {
                push_unique(&mut reasons, DowngradeReason::SparseWorkset)
            }
            NavigationConfidence::Heuristic => {
                push_unique(&mut reasons, DowngradeReason::SyntaxFallbackOnly)
            }
            NavigationConfidence::Imported => {
                push_unique(&mut reasons, DowngradeReason::GeneratedBoundary)
            }
            NavigationConfidence::Stale => push_unique(&mut reasons, DowngradeReason::StaleShard),
            NavigationConfidence::Unavailable => {
                push_unique(&mut reasons, DowngradeReason::ProviderUnavailable)
            }
            NavigationConfidence::Exact | NavigationConfidence::Indexed => {}
        }
    }
    if freshness.requires_disclosure() {
        match freshness {
            FreshnessClass::Stale => push_unique(&mut reasons, DowngradeReason::StaleShard),
            FreshnessClass::Unverified => {
                push_unique(&mut reasons, DowngradeReason::GeneratedBoundary)
            }
            FreshnessClass::DegradedCached => {
                push_unique(&mut reasons, DowngradeReason::ProviderUnavailable)
            }
            FreshnessClass::AuthoritativeLive | FreshnessClass::WarmCached => {}
        }
    }
    if scope_completeness.requires_disclosure() || !scope_limits.is_empty() {
        push_unique(&mut reasons, DowngradeReason::SparseWorkset);
    }
    if !matches!(health, HealthState::Ready) {
        push_unique(&mut reasons, DowngradeReason::ProviderUnavailable);
    }
    if disambiguation_required {
        push_unique(&mut reasons, DowngradeReason::AmbiguousCandidates);
    }
    if generated_or_imported {
        push_unique(&mut reasons, DowngradeReason::GeneratedBoundary);
    }
    if scope_limits
        .iter()
        .any(|limit| matches!(limit, ScopeLimitClass::PolicyNarrowed))
    {
        push_unique(&mut reasons, DowngradeReason::PolicyLimited);
    }
    reasons
}

fn semantic_evidence_refs(
    source_evidence_refs: &[String],
    scope_caveat_refs: &[String],
    router_decision_id: &str,
) -> Vec<String> {
    let mut refs = Vec::new();
    push_unique_string(&mut refs, router_decision_id);
    for evidence_ref in source_evidence_refs {
        push_unique_string(&mut refs, evidence_ref);
    }
    for caveat_ref in scope_caveat_refs {
        push_unique_string(&mut refs, caveat_ref);
    }
    refs
}

fn rename_evidence_refs(
    source_evidence_refs: &[String],
    scope_caveat_refs: &[String],
    router_decision_id: &str,
) -> Vec<String> {
    semantic_evidence_refs(source_evidence_refs, scope_caveat_refs, router_decision_id)
}

fn tsjs_rename_confidence(
    completeness: TsJsRenamePreviewCompletenessClass,
) -> NavigationConfidence {
    match completeness {
        TsJsRenamePreviewCompletenessClass::FullWorkspaceComplete
        | TsJsRenamePreviewCompletenessClass::CompleteForRequestedScope => {
            NavigationConfidence::Exact
        }
        TsJsRenamePreviewCompletenessClass::PartialDueToWorkspaceSlice => {
            NavigationConfidence::WorkspaceSliceLimited
        }
        TsJsRenamePreviewCompletenessClass::PartialDueToIndexOrProvider => {
            NavigationConfidence::Partial
        }
        TsJsRenamePreviewCompletenessClass::PartialDueToImportedOrGeneratedBoundaries => {
            NavigationConfidence::Imported
        }
        TsJsRenamePreviewCompletenessClass::StaleRequiresRefresh => NavigationConfidence::Stale,
        TsJsRenamePreviewCompletenessClass::UnavailableBlocked => NavigationConfidence::Unavailable,
    }
}

fn python_rename_confidence(
    completeness: PythonRenamePreviewCompletenessClass,
) -> NavigationConfidence {
    match completeness {
        PythonRenamePreviewCompletenessClass::FullWorkspaceComplete
        | PythonRenamePreviewCompletenessClass::CompleteForRequestedScope => {
            NavigationConfidence::Exact
        }
        PythonRenamePreviewCompletenessClass::PartialDueToWorkspaceSlice => {
            NavigationConfidence::WorkspaceSliceLimited
        }
        PythonRenamePreviewCompletenessClass::PartialDueToIndexOrProvider => {
            NavigationConfidence::Partial
        }
        PythonRenamePreviewCompletenessClass::PartialDueToImportedOrGeneratedBoundaries => {
            NavigationConfidence::Imported
        }
        PythonRenamePreviewCompletenessClass::StaleRequiresRefresh => NavigationConfidence::Stale,
        PythonRenamePreviewCompletenessClass::UnavailableBlocked => {
            NavigationConfidence::Unavailable
        }
    }
}

fn tsjs_rename_scope_completeness(
    completeness: TsJsRenamePreviewCompletenessClass,
) -> ScopeCompleteness {
    match completeness {
        TsJsRenamePreviewCompletenessClass::FullWorkspaceComplete
        | TsJsRenamePreviewCompletenessClass::CompleteForRequestedScope => {
            ScopeCompleteness::CompleteForDeclaredScope
        }
        TsJsRenamePreviewCompletenessClass::StaleRequiresRefresh => {
            ScopeCompleteness::StaleForDeclaredScope
        }
        TsJsRenamePreviewCompletenessClass::UnavailableBlocked => {
            ScopeCompleteness::UnavailableForDeclaredScope
        }
        TsJsRenamePreviewCompletenessClass::PartialDueToWorkspaceSlice
        | TsJsRenamePreviewCompletenessClass::PartialDueToIndexOrProvider
        | TsJsRenamePreviewCompletenessClass::PartialDueToImportedOrGeneratedBoundaries => {
            ScopeCompleteness::PartialForDeclaredScope
        }
    }
}

fn python_rename_scope_completeness(
    completeness: PythonRenamePreviewCompletenessClass,
) -> ScopeCompleteness {
    match completeness {
        PythonRenamePreviewCompletenessClass::FullWorkspaceComplete
        | PythonRenamePreviewCompletenessClass::CompleteForRequestedScope => {
            ScopeCompleteness::CompleteForDeclaredScope
        }
        PythonRenamePreviewCompletenessClass::StaleRequiresRefresh => {
            ScopeCompleteness::StaleForDeclaredScope
        }
        PythonRenamePreviewCompletenessClass::UnavailableBlocked => {
            ScopeCompleteness::UnavailableForDeclaredScope
        }
        PythonRenamePreviewCompletenessClass::PartialDueToWorkspaceSlice
        | PythonRenamePreviewCompletenessClass::PartialDueToIndexOrProvider
        | PythonRenamePreviewCompletenessClass::PartialDueToImportedOrGeneratedBoundaries => {
            ScopeCompleteness::PartialForDeclaredScope
        }
    }
}

fn rename_apply_posture(token: RenameApplyPostureToken) -> RenameApplyPosture {
    match token {
        RenameApplyPostureToken::ReadyForApplyAfterPreview => {
            RenameApplyPosture::ReadyForApplyAfterPreview
        }
        RenameApplyPostureToken::BlockedPendingScopeReview => {
            RenameApplyPosture::BlockedPendingScopeReview
        }
        RenameApplyPostureToken::BlockedPendingRefresh => RenameApplyPosture::BlockedPendingRefresh,
        RenameApplyPostureToken::BlockedPendingPolicyOrProtectedReview => {
            RenameApplyPosture::BlockedPendingPolicyOrProtectedReview
        }
        RenameApplyPostureToken::InspectOnlyUnavailable => {
            RenameApplyPosture::InspectOnlyUnavailable
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum RenameApplyPostureToken {
    ReadyForApplyAfterPreview,
    BlockedPendingScopeReview,
    BlockedPendingRefresh,
    BlockedPendingPolicyOrProtectedReview,
    InspectOnlyUnavailable,
}

impl From<crate::TsJsApplyPostureClass> for RenameApplyPostureToken {
    fn from(value: crate::TsJsApplyPostureClass) -> Self {
        match value {
            crate::TsJsApplyPostureClass::ReadyForApplyAfterPreview => {
                Self::ReadyForApplyAfterPreview
            }
            crate::TsJsApplyPostureClass::BlockedPendingScopeReview => {
                Self::BlockedPendingScopeReview
            }
            crate::TsJsApplyPostureClass::BlockedPendingRefresh => Self::BlockedPendingRefresh,
            crate::TsJsApplyPostureClass::BlockedPendingPolicyOrProtectedReview => {
                Self::BlockedPendingPolicyOrProtectedReview
            }
            crate::TsJsApplyPostureClass::InspectOnlyUnavailable => Self::InspectOnlyUnavailable,
        }
    }
}

impl From<crate::PythonApplyPostureClass> for RenameApplyPostureToken {
    fn from(value: crate::PythonApplyPostureClass) -> Self {
        match value {
            crate::PythonApplyPostureClass::ReadyForApplyAfterPreview => {
                Self::ReadyForApplyAfterPreview
            }
            crate::PythonApplyPostureClass::BlockedPendingScopeReview => {
                Self::BlockedPendingScopeReview
            }
            crate::PythonApplyPostureClass::BlockedPendingRefresh => Self::BlockedPendingRefresh,
            crate::PythonApplyPostureClass::BlockedPendingPolicyOrProtectedReview => {
                Self::BlockedPendingPolicyOrProtectedReview
            }
            crate::PythonApplyPostureClass::InspectOnlyUnavailable => Self::InspectOnlyUnavailable,
        }
    }
}

fn redaction_class(redaction: RedactionClass) -> ExportRedactionClass {
    match redaction {
        RedactionClass::MetadataSafeDefault => ExportRedactionClass::MetadataSafeDefault,
        RedactionClass::OperatorOnlyRestricted => ExportRedactionClass::OperatorOnlyRestricted,
        RedactionClass::InternalSupportRestricted => {
            ExportRedactionClass::InternalSupportRestricted
        }
        RedactionClass::SigningEvidenceOnly => ExportRedactionClass::SigningEvidenceOnly,
    }
}

fn affected_result_refs_from_tsjs_preview(record: &TsJsRenamePreviewRecord) -> Vec<String> {
    unique_strings(
        record
            .affected_scope_rows
            .iter()
            .flat_map(|row| row.affected_result_refs.iter().cloned()),
    )
}

fn affected_result_refs_from_python_preview(record: &PythonRenamePreviewRecord) -> Vec<String> {
    unique_strings(
        record
            .affected_scope_rows
            .iter()
            .flat_map(|row| row.affected_result_refs.iter().cloned()),
    )
}

fn blocked_refs_from_tsjs_preview(record: &TsJsRenamePreviewRecord) -> Vec<String> {
    let mut refs = record
        .warning_rows
        .iter()
        .flat_map(|warning| warning.affected_result_refs.iter().cloned())
        .collect::<Vec<_>>();
    add_synthetic_blocked_refs(
        &mut refs,
        &record.rename_preview_id,
        record.count_summary.unresolved_count,
        record.count_summary.generated_count,
        record.count_summary.protected_count,
        record.count_summary.skipped_count,
    );
    unique_strings(refs)
}

fn blocked_refs_from_python_preview(record: &PythonRenamePreviewRecord) -> Vec<String> {
    let mut refs = record
        .warning_rows
        .iter()
        .flat_map(|warning| warning.affected_result_refs.iter().cloned())
        .collect::<Vec<_>>();
    add_synthetic_blocked_refs(
        &mut refs,
        &record.rename_preview_id,
        record.count_summary.unresolved_count,
        record.count_summary.generated_count,
        record.count_summary.protected_count,
        record.count_summary.skipped_count,
    );
    unique_strings(refs)
}

fn generated_notes_from_tsjs_preview(record: &TsJsRenamePreviewRecord) -> Vec<String> {
    let mut notes = record
        .warning_rows
        .iter()
        .filter(|warning| {
            matches!(
                warning.warning_class,
                crate::TsJsRenameWarningClass::GeneratedReferenceWouldChange
                    | crate::TsJsRenameWarningClass::ImportedAnchorUnverified
            )
        })
        .map(|warning| warning.summary.clone())
        .collect::<Vec<_>>();
    if notes.is_empty() && record.count_summary.generated_count > 0 {
        notes.push(format!(
            "{} includes generated candidates that require lineage review.",
            record.rename_preview_id
        ));
    }
    notes
}

fn generated_notes_from_python_preview(record: &PythonRenamePreviewRecord) -> Vec<String> {
    let mut notes = record
        .warning_rows
        .iter()
        .filter(|warning| {
            matches!(
                warning.warning_class,
                crate::PythonRenameWarningClass::GeneratedReferenceWouldChange
                    | crate::PythonRenameWarningClass::ImportedAnchorUnverified
            )
        })
        .map(|warning| warning.summary.clone())
        .collect::<Vec<_>>();
    if notes.is_empty() && record.count_summary.generated_count > 0 {
        notes.push(format!(
            "{} includes generated candidates that require lineage review.",
            record.rename_preview_id
        ));
    }
    notes
}

fn add_synthetic_blocked_refs(
    refs: &mut Vec<String>,
    preview_id: &str,
    unresolved_count: usize,
    generated_count: usize,
    protected_count: usize,
    skipped_count: usize,
) {
    if unresolved_count > 0 {
        refs.push(format!("{preview_id}:blocked:unresolved"));
    }
    if generated_count > 0 {
        refs.push(format!("{preview_id}:blocked:generated"));
    }
    if protected_count > 0 {
        refs.push(format!("{preview_id}:blocked:protected"));
    }
    if skipped_count > 0 {
        refs.push(format!("{preview_id}:blocked:skipped"));
    }
}

fn push_unique(reasons: &mut Vec<DowngradeReason>, reason: DowngradeReason) {
    if !reasons.contains(&reason) {
        reasons.push(reason);
    }
}

fn push_unique_string(strings: &mut Vec<String>, value: &str) {
    if !strings.iter().any(|existing| existing == value) {
        strings.push(value.to_owned());
    }
}

fn unique_strings(values: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut unique = Vec::new();
    for value in values {
        if !unique.contains(&value) {
            unique.push(value);
        }
    }
    unique
}
