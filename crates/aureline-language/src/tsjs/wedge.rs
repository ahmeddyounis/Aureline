use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use crate::lsp_router::{
    CapabilityClass, CompletenessClass, CoordinateTranslationRequirementClass, FreshnessClass,
    HealthState, LaneClass, LanguageServerHostStatus, LspRouter, PlacementPreferenceClass,
    ProviderFamily, ProviderKind, ProviderStackRow, RedactionClass, RequestedAuthorityFloorClass,
    RouterDecisionRecord, RouterRequest, RouterRequestContext, RoutingContext, ScopeClaimClass,
    ScopeLimitClass, SurfaceClass,
};

use super::records::{
    TsJsAmbiguityDescriptor, TsJsAnchorRef, TsJsAnswerLayerClass, TsJsApplyPostureClass,
    TsJsCheckpointClass, TsJsCompletenessClass, TsJsGeneratedOrExternalStateClass, TsJsHoverRecord,
    TsJsInlineVisibilityClass, TsJsLaunchWedgeSnapshot, TsJsProviderSnapshot,
    TsJsReferenceCountSummary, TsJsReferenceSetRecord, TsJsRelationClass,
    TsJsRenameAffectedScopeRow, TsJsRenameCountSummary, TsJsRenameCoverageLimitClass,
    TsJsRenameEvidenceBinding, TsJsRenamePreviewCompletenessClass, TsJsRenamePreviewRecord,
    TsJsRenameWarningClass, TsJsRenameWarningRow, TsJsResultConfidenceClass, TsJsRollbackPathClass,
    TsJsScopeDescriptor, TsJsSemanticEvidenceBinding, TsJsSemanticResultIdentityClass,
    TsJsSemanticResultRecord, TsJsSourceAnchor, TsJsSymbolSeed, TSJS_NAV_ALPHA_SCHEMA_VERSION,
};

/// Error returned when a TS/JS assistance request cannot be built.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TsJsNavigationError {
    /// The requested symbol is not in the protected snapshot.
    SymbolNotFound {
        /// Missing symbol ref.
        symbol_ref: String,
    },
}

impl fmt::Display for TsJsNavigationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SymbolNotFound { symbol_ref } => {
                write!(formatter, "TS/JS symbol {symbol_ref} was not found")
            }
        }
    }
}

impl Error for TsJsNavigationError {}

/// Fixture-backed TS/JS hover, navigation, references, and rename-preview alpha.
#[derive(Debug, Clone)]
pub struct TsJsLaunchWedge {
    snapshot: TsJsLaunchWedgeSnapshot,
    router: LspRouter,
}

impl TsJsLaunchWedge {
    /// Builds a TS/JS launch-wedge assistance surface from a protected snapshot.
    pub fn new(snapshot: TsJsLaunchWedgeSnapshot) -> Self {
        Self {
            snapshot,
            router: LspRouter::new(),
        }
    }

    /// Returns the protected snapshot backing this assistance surface.
    pub const fn snapshot(&self) -> &TsJsLaunchWedgeSnapshot {
        &self.snapshot
    }

    /// Builds a hover record for the requested symbol.
    pub fn hover(
        &self,
        symbol_ref: &str,
        host_statuses: &[LanguageServerHostStatus],
    ) -> Result<TsJsHoverRecord, TsJsNavigationError> {
        let symbol = self.symbol(symbol_ref)?;
        let decision = self.route(
            symbol_ref,
            SurfaceClass::Hover,
            CapabilityClass::Hover,
            host_statuses,
        );
        let selected_host = selected_host(&decision, host_statuses);
        let provider_snapshot = self.provider_snapshot(&decision, selected_host);
        let scope_descriptor = self.scope_descriptor(&decision, selected_host);
        let answer_layer_class = answer_layer(&decision, selected_host);
        let fallback_summary = if answer_layer_class.is_fallback() {
            decision.decision_outcome.fallback_summary.clone()
        } else {
            "No fallback was used; hover is served by the selected language provider.".into()
        };
        let hover_detail = if answer_layer_class.is_fallback() {
            format!(
                "Syntax-local symbol `{}` ({}) in {}.",
                symbol.display_name,
                symbol.kind_class.as_str(),
                symbol.definition_anchor.workspace_relative_path
            )
        } else {
            symbol.hover_detail.clone()
        };

        Ok(TsJsHoverRecord {
            record_kind: TsJsHoverRecord::RECORD_KIND.into(),
            schema_version: TSJS_NAV_ALPHA_SCHEMA_VERSION,
            hover_id: format!("tsjs:hover:{}", sanitize_id(symbol_ref)),
            target_symbol_ref: symbol.symbol_ref.clone(),
            display_label: symbol.display_name.clone(),
            answer_layer_class,
            provider_snapshot,
            scope_descriptor,
            router_decision_id: decision.router_decision_id.clone(),
            hover_label: symbol.hover_label.clone(),
            hover_summary: symbol.hover_summary.clone(),
            hover_detail,
            fallback_summary,
            degraded_state_class: decision.decision_outcome.degraded_state_class,
            policy_context: self.snapshot.workspace_context.policy_context(),
            redaction_class: RedactionClass::MetadataSafeDefault,
            captured_at: self.snapshot.captured_at.clone(),
            export_safe_summary: format!(
                "Hover for {} is answered by {:?} with scope label {}.",
                symbol.display_name,
                answer_layer_class,
                self.snapshot.workspace_context.scope_label
            ),
        })
    }

    /// Builds a definition navigation result for the requested symbol.
    pub fn definition(
        &self,
        symbol_ref: &str,
        host_statuses: &[LanguageServerHostStatus],
    ) -> Result<TsJsSemanticResultRecord, TsJsNavigationError> {
        let symbol = self.symbol(symbol_ref)?;
        let decision = self.route(
            symbol_ref,
            SurfaceClass::Definition,
            CapabilityClass::Definition,
            host_statuses,
        );
        let selected_host = selected_host(&decision, host_statuses);
        let provider_snapshot = self.provider_snapshot(&decision, selected_host);
        let scope_descriptor = self.scope_descriptor(&decision, selected_host);
        let semantic_state = semantic_state(&decision, selected_host);
        let result_id = definition_result_id(symbol_ref);

        Ok(self.semantic_result(SemanticResultInput {
            semantic_result_id: result_id,
            symbol,
            anchor: &symbol.definition_anchor,
            semantic_result_identity_class: TsJsSemanticResultIdentityClass::Definition,
            relation_class: TsJsRelationClass::DefinitionTarget,
            provider_snapshot,
            scope_descriptor,
            semantic_state,
            decision: &decision,
            summary:
                "Definition target remains inspectable with provider, freshness, and scope labels.",
        }))
    }

    /// Builds a reference set for the requested symbol.
    pub fn references(
        &self,
        symbol_ref: &str,
        host_statuses: &[LanguageServerHostStatus],
    ) -> Result<TsJsReferenceSetRecord, TsJsNavigationError> {
        let symbol = self.symbol(symbol_ref)?;
        let decision = self.route(
            symbol_ref,
            SurfaceClass::Reference,
            CapabilityClass::Reference,
            host_statuses,
        );
        let selected_host = selected_host(&decision, host_statuses);
        let provider_snapshot = self.provider_snapshot(&decision, selected_host);
        let scope_descriptor = self.scope_descriptor(&decision, selected_host);
        let semantic_state = semantic_state(&decision, selected_host);
        let materialized_occurrences =
            self.materialized_reference_occurrences(symbol, &decision, selected_host);
        let occurrence_results = materialized_occurrences
            .iter()
            .map(|occurrence| {
                let identity_class = if occurrence.generated_or_external_state_class
                    == TsJsGeneratedOrExternalStateClass::AuthoredSource
                {
                    TsJsSemanticResultIdentityClass::Reference
                } else {
                    TsJsSemanticResultIdentityClass::ImportedOrGeneratedReference
                };
                self.semantic_result(SemanticResultInput {
                    semantic_result_id: reference_result_id(symbol_ref, &occurrence.occurrence_ref),
                    symbol,
                    anchor: &occurrence.anchor,
                    semantic_result_identity_class: identity_class,
                    relation_class: relation_for_occurrence(occurrence),
                    provider_snapshot: provider_snapshot.clone(),
                    scope_descriptor: scope_descriptor.clone(),
                    semantic_state,
                    decision: &decision,
                    summary: occurrence.summary.as_str(),
                })
            })
            .collect::<Vec<_>>();
        let count_summary = reference_count_summary(symbol, &occurrence_results);

        Ok(TsJsReferenceSetRecord {
            record_kind: TsJsReferenceSetRecord::RECORD_KIND.into(),
            schema_version: TSJS_NAV_ALPHA_SCHEMA_VERSION,
            reference_set_id: format!("tsjs:references:{}", sanitize_id(symbol_ref)),
            target_symbol_ref: symbol.symbol_ref.clone(),
            occurrence_results,
            scope_descriptor,
            provider_snapshot,
            router_decision_id: decision.router_decision_id.clone(),
            count_summary,
            degraded_state_class: decision.decision_outcome.degraded_state_class,
            captured_at: self.snapshot.captured_at.clone(),
            export_safe_summary: format!(
                "Reference set for {} materialized {} of {} known occurrences.",
                symbol.display_name,
                materialized_occurrences.len(),
                symbol.occurrences.len()
            ),
        })
    }

    /// Builds a preview-only rename packet for the requested symbol.
    pub fn rename_preview(
        &self,
        symbol_ref: &str,
        requested_new_name_ref: &str,
        host_statuses: &[LanguageServerHostStatus],
    ) -> Result<TsJsRenamePreviewRecord, TsJsNavigationError> {
        let symbol = self.symbol(symbol_ref)?;
        let decision = self.route(
            symbol_ref,
            SurfaceClass::Rename,
            CapabilityClass::Rename,
            host_statuses,
        );
        let selected_host = selected_host(&decision, host_statuses);
        let provider_snapshot = self.provider_snapshot(&decision, selected_host);
        let target_semantic_result_ref = definition_result_id(symbol_ref);
        let rename_occurrences =
            self.materialized_rename_occurrences(symbol, &decision, selected_host);
        let count_summary = rename_count_summary(symbol, &rename_occurrences);
        let preview_completeness_class =
            rename_preview_completeness(&decision, selected_host, &count_summary);
        let apply_posture_class = rename_apply_posture(preview_completeness_class, &count_summary);
        let affected_result_refs = rename_occurrences
            .iter()
            .map(|occurrence| reference_result_id(symbol_ref, &occurrence.occurrence_ref))
            .collect::<Vec<_>>();
        let coverage_limits =
            rename_coverage_limits(&decision, selected_host, preview_completeness_class);
        let affected_scope_rows = vec![TsJsRenameAffectedScopeRow {
            requested_scope_class: self.snapshot.workspace_context.requested_scope_class,
            materialized_scope_class: materialized_rename_scope(&decision, selected_host),
            coverage_limit_classes: coverage_limits.clone(),
            affected_result_refs: affected_result_refs.clone(),
            omitted_result_count: count_summary.skipped_count + count_summary.protected_count,
            caveat_summary: rename_scope_summary(preview_completeness_class, &self.snapshot),
        }];
        let warning_rows = rename_warning_rows(symbol, &affected_result_refs, &count_summary);
        let rename_preview_id = format!(
            "tsjs:rename-preview:{}:{}",
            sanitize_id(symbol_ref),
            sanitize_id(requested_new_name_ref)
        );
        let checkpoint_descriptor =
            checkpoint_descriptor(apply_posture_class, &rename_preview_id, symbol_ref);
        let evidence_binding = TsJsRenameEvidenceBinding {
            durable_preview_id: rename_preview_id.clone(),
            result_provenance_ref: Some(format!(
                "lang:result:tsjs:rename-preview:{}",
                sanitize_id(symbol_ref)
            )),
            refactor_preview_ref: Some(format!(
                "editor:refactor:preview:tsjs:{}",
                sanitize_id(symbol_ref)
            )),
            review_packet_ref: Some(format!(
                "review:packet:tsjs:rename-preview:{}",
                sanitize_id(symbol_ref)
            )),
            ai_citation_anchor_ref: Some(format!(
                "docs:anchor:ai:tsjs:rename-preview:{}",
                sanitize_id(symbol_ref)
            )),
            support_export_ref: Some(format!(
                "support:tsjs:rename-preview:{}",
                sanitize_id(symbol_ref)
            )),
            source_evidence_refs: vec![
                decision.router_decision_id.clone(),
                provider_snapshot.provider_id.clone(),
                symbol.definition_anchor.source_anchor_ref.clone(),
            ],
            scope_caveat_refs: scope_caveat_refs(
                &self.snapshot.workspace_context,
                &coverage_limits,
            ),
        };

        Ok(TsJsRenamePreviewRecord {
            record_kind: TsJsRenamePreviewRecord::RECORD_KIND.into(),
            rename_preview_schema_version: TSJS_NAV_ALPHA_SCHEMA_VERSION,
            rename_preview_id,
            target_semantic_result_ref,
            requested_new_name_ref: requested_new_name_ref.to_owned(),
            preview_completeness_class,
            apply_posture_class,
            count_summary,
            affected_scope_rows,
            warning_rows,
            checkpoint_descriptor,
            provider_snapshot,
            current_epoch_bindings: self.snapshot.current_epoch_bindings.clone(),
            evidence_binding,
            policy_context: self.snapshot.workspace_context.policy_context(),
            redaction_class: redaction_for_preview(preview_completeness_class),
            router_decision_id: decision.router_decision_id.clone(),
            captured_at: self.snapshot.captured_at.clone(),
            export_safe_summary: format!(
                "Rename preview for {} is {:?} in {}.",
                symbol.display_name,
                preview_completeness_class,
                self.snapshot.workspace_context.scope_label
            ),
        })
    }

    fn symbol(&self, symbol_ref: &str) -> Result<&TsJsSymbolSeed, TsJsNavigationError> {
        self.snapshot
            .symbol(symbol_ref)
            .ok_or_else(|| TsJsNavigationError::SymbolNotFound {
                symbol_ref: symbol_ref.to_owned(),
            })
    }

    fn route(
        &self,
        symbol_ref: &str,
        surface_class: SurfaceClass,
        capability_class: CapabilityClass,
        host_statuses: &[LanguageServerHostStatus],
    ) -> RouterDecisionRecord {
        let context = &self.snapshot.workspace_context;
        let coordinate_translation_requirement_class =
            if capability_class == CapabilityClass::Rename {
                CoordinateTranslationRequirementClass::RequiredForMutation
            } else {
                CoordinateTranslationRequirementClass::RequiredBeforeResult
            };
        let request = RouterRequest {
            language_id: self.snapshot.language_id.clone(),
            request_context: RouterRequestContext {
                requested_surface_class: surface_class,
                requested_capability_class: capability_class,
                requested_authority_floor_class:
                    RequestedAuthorityFloorClass::AuthoritativePreferred,
                requested_scope_claim_class: context.requested_scope_class,
                requested_subject_ref: symbol_ref.to_owned(),
                placement_preference_class: PlacementPreferenceClass::MatchSubjectLocation,
                coordinate_translation_requirement_class,
                policy_epoch: context.policy_epoch.clone(),
                trust_state: context.trust_state,
                execution_context_id: context.execution_context_id.clone(),
            },
            routing_context: RoutingContext {
                workspace_id: context.workspace_id.clone(),
                workset_id: context.workset_id.clone(),
                workspace_root_ref: context.workspace_root_ref.clone(),
                subject_root_ref: context.subject_root_ref.clone(),
                package_root_ref: Some(context.package_root_ref.clone()),
                config_root_ref: Some(context.config_root_ref.clone()),
                lane_class: LaneClass::LocalOnly,
                target_summary: format!("TS/JS assistance is scoped to {}.", context.scope_label),
                toolchain_summary: format!(
                    "TS/JS semantics are anchored by {}.",
                    context.execution_context_id
                ),
            },
            captured_at: self.snapshot.captured_at.clone(),
        };

        self.router.route(request, host_statuses)
    }

    fn provider_snapshot(
        &self,
        decision: &RouterDecisionRecord,
        selected_host: Option<&LanguageServerHostStatus>,
    ) -> TsJsProviderSnapshot {
        if let Some(host) = selected_host {
            return TsJsProviderSnapshot {
                provider_id: host.identity.provider_id.clone(),
                provider_family: ProviderFamily::LanguageServer,
                provider_display_label: host.identity.server_label.clone(),
                provider_health_class: host.health_state,
                freshness_class: host.freshness_class,
                locality_class: host.identity.locality_class,
                host_identity_ref: Some(host.identity.host_instance_id.clone()),
                current_epoch_bindings: self.snapshot.current_epoch_bindings.clone(),
                summary: host.health_summary.clone(),
            };
        }

        let row = selected_provider_row(decision).unwrap_or_else(|| {
            decision
                .provider_stack_rows
                .first()
                .expect("router emits at least one provider row")
        });
        TsJsProviderSnapshot {
            provider_id: row.provider_id.clone(),
            provider_family: provider_family_for_kind(row.provider_kind),
            provider_display_label: row.provider_display_label.clone(),
            provider_health_class: row.health_state,
            freshness_class: row.freshness_class,
            locality_class: row.locality_class,
            host_identity_ref: None,
            current_epoch_bindings: self.snapshot.current_epoch_bindings.clone(),
            summary: row.summary.clone(),
        }
    }

    fn scope_descriptor(
        &self,
        decision: &RouterDecisionRecord,
        selected_host: Option<&LanguageServerHostStatus>,
    ) -> TsJsScopeDescriptor {
        let context = &self.snapshot.workspace_context;
        if selected_provider_is_syntax(decision) {
            return TsJsScopeDescriptor {
                requested_scope_class: context.requested_scope_class,
                materialized_scope_class: ScopeClaimClass::SingleFile,
                scope_limit_classes: vec![ScopeLimitClass::SingleFileOnly],
                covered_scope_ref: symbol_file_scope_ref(decision),
                omitted_scope_ref: context.omitted_scope_ref.clone(),
                caveat_summary: "Language service is unavailable or incomplete; the result is limited to file-local syntax/text fallback.".into(),
            };
        }

        if let Some(host) = selected_host {
            let mut limits = host.scope_limit_classes.clone();
            if host.completeness_class != CompletenessClass::CompleteForClaimedScope
                && limits.is_empty()
            {
                limits.push(ScopeLimitClass::ActiveWorksetOnly);
            }
            let caveat_summary = if limits.is_empty() {
                format!("Result covers the requested {} scope.", context.scope_label)
            } else {
                format!(
                    "Result is limited to {} and must disclose omitted scope before broad rename or review.",
                    context.scope_label
                )
            };
            return TsJsScopeDescriptor {
                requested_scope_class: context.requested_scope_class,
                materialized_scope_class: host.scope_claim_class,
                scope_limit_classes: limits,
                covered_scope_ref: context.covered_scope_ref.clone(),
                omitted_scope_ref: if host.completeness_class
                    == CompletenessClass::CompleteForClaimedScope
                {
                    None
                } else {
                    context.omitted_scope_ref.clone()
                },
                caveat_summary,
            };
        }

        TsJsScopeDescriptor {
            requested_scope_class: context.requested_scope_class,
            materialized_scope_class: context.materialized_scope_class,
            scope_limit_classes: context.scope_limit_classes.clone(),
            covered_scope_ref: context.covered_scope_ref.clone(),
            omitted_scope_ref: context.omitted_scope_ref.clone(),
            caveat_summary: format!("Result inherits the fixture scope {}.", context.scope_label),
        }
    }

    fn semantic_result(&self, input: SemanticResultInput<'_>) -> TsJsSemanticResultRecord {
        let SemanticResultInput {
            semantic_result_id,
            symbol,
            anchor,
            semantic_result_identity_class,
            relation_class,
            provider_snapshot,
            scope_descriptor,
            semantic_state,
            decision,
            summary,
        } = input;
        let source_anchor = TsJsSourceAnchor::from_anchor(anchor, &symbol.symbol_ref);
        let evidence_binding = TsJsSemanticEvidenceBinding {
            durable_result_id: semantic_result_id.clone(),
            result_provenance_ref: Some(format!(
                "lang:result:tsjs:{}",
                sanitize_id(&semantic_result_id)
            )),
            navigation_artifact_ref: Some(format!(
                "nav:history:tsjs:{}",
                sanitize_id(&semantic_result_id)
            )),
            review_packet_ref: None,
            ai_citation_anchor_ref: Some(format!(
                "docs:anchor:ai:tsjs:{}",
                sanitize_id(&semantic_result_id)
            )),
            support_export_ref: Some(format!(
                "support:tsjs:navigation:{}",
                sanitize_id(&semantic_result_id)
            )),
            source_evidence_refs: vec![
                decision.router_decision_id.clone(),
                provider_snapshot.provider_id.clone(),
                anchor.source_anchor_ref.clone(),
            ],
            scope_caveat_refs: scope_limit_caveat_refs(&scope_descriptor),
        };
        TsJsSemanticResultRecord {
            record_kind: TsJsSemanticResultRecord::RECORD_KIND.into(),
            semantic_result_ref_schema_version: TSJS_NAV_ALPHA_SCHEMA_VERSION,
            semantic_result_id: semantic_result_id.clone(),
            semantic_result_identity_class,
            relation_class,
            source_anchor,
            provider_snapshot,
            result_confidence_class: semantic_state.result_confidence_class,
            completeness_class: semantic_state.completeness_class,
            inline_visibility_class: semantic_state.inline_visibility_class,
            scope_descriptor,
            ambiguity_descriptor: TsJsAmbiguityDescriptor {
                ambiguous_candidate_count: 0,
                selected_candidate_count: 1,
                disambiguation_required: false,
                summary: "A single fixture-backed TS/JS target was selected.".into(),
            },
            evidence_binding,
            current_epoch_bindings: self.snapshot.current_epoch_bindings.clone(),
            policy_context: self.snapshot.workspace_context.policy_context(),
            redaction_class: redaction_for_semantic_state(semantic_state),
            router_decision_id: decision.router_decision_id.clone(),
            captured_at: self.snapshot.captured_at.clone(),
            export_safe_summary: format!("{} {}", summary, semantic_result_id),
        }
    }

    fn materialized_reference_occurrences<'a>(
        &self,
        symbol: &'a TsJsSymbolSeed,
        decision: &RouterDecisionRecord,
        selected_host: Option<&LanguageServerHostStatus>,
    ) -> Vec<&'a super::records::TsJsOccurrenceSeed> {
        let complete_semantic = selected_host.is_some_and(host_claims_complete_semantics);
        if complete_semantic {
            return symbol.reference_occurrences().collect();
        }

        if selected_provider_is_syntax(decision) {
            let definition_file = &symbol.definition_anchor.canonical_file_ref;
            return symbol
                .reference_occurrences()
                .filter(|occurrence| occurrence.anchor.canonical_file_ref == *definition_file)
                .collect();
        }

        symbol
            .reference_occurrences()
            .filter(|occurrence| occurrence.in_current_workset)
            .collect()
    }

    fn materialized_rename_occurrences<'a>(
        &self,
        symbol: &'a TsJsSymbolSeed,
        decision: &RouterDecisionRecord,
        selected_host: Option<&LanguageServerHostStatus>,
    ) -> Vec<&'a super::records::TsJsOccurrenceSeed> {
        let complete_semantic = selected_host.is_some_and(host_claims_complete_semantics);
        let definition_file = &symbol.definition_anchor.canonical_file_ref;

        symbol
            .reference_occurrences()
            .filter(|occurrence| occurrence.rename_writable_authored_candidate())
            .filter(|occurrence| {
                if complete_semantic {
                    true
                } else if selected_provider_is_syntax(decision) {
                    occurrence.anchor.canonical_file_ref == *definition_file
                } else {
                    occurrence.in_current_workset
                }
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
struct SemanticState {
    result_confidence_class: TsJsResultConfidenceClass,
    completeness_class: TsJsCompletenessClass,
    inline_visibility_class: TsJsInlineVisibilityClass,
}

#[derive(Debug)]
struct SemanticResultInput<'a> {
    semantic_result_id: String,
    symbol: &'a TsJsSymbolSeed,
    anchor: &'a TsJsAnchorRef,
    semantic_result_identity_class: TsJsSemanticResultIdentityClass,
    relation_class: TsJsRelationClass,
    provider_snapshot: TsJsProviderSnapshot,
    scope_descriptor: TsJsScopeDescriptor,
    semantic_state: SemanticState,
    decision: &'a RouterDecisionRecord,
    summary: &'a str,
}

fn selected_host<'a>(
    decision: &RouterDecisionRecord,
    host_statuses: &'a [LanguageServerHostStatus],
) -> Option<&'a LanguageServerHostStatus> {
    host_statuses
        .iter()
        .find(|host| host.identity.provider_id == decision.decision_outcome.selected_provider_id)
}

fn selected_provider_row(decision: &RouterDecisionRecord) -> Option<&ProviderStackRow> {
    decision
        .provider_stack_rows
        .iter()
        .find(|row| row.provider_id == decision.decision_outcome.selected_provider_id)
}

fn selected_provider_is_syntax(decision: &RouterDecisionRecord) -> bool {
    selected_provider_row(decision)
        .is_some_and(|row| row.provider_kind == ProviderKind::SyntaxParser)
}

fn host_claims_complete_semantics(host: &LanguageServerHostStatus) -> bool {
    host.health_state == HealthState::Ready
        && host.freshness_class == FreshnessClass::AuthoritativeLive
        && host.completeness_class == CompletenessClass::CompleteForClaimedScope
        && host.scope_limit_classes.is_empty()
}

fn answer_layer(
    decision: &RouterDecisionRecord,
    selected_host: Option<&LanguageServerHostStatus>,
) -> TsJsAnswerLayerClass {
    if selected_provider_is_syntax(decision) {
        TsJsAnswerLayerClass::Layer1SyntaxStructure
    } else if selected_host.is_some() {
        TsJsAnswerLayerClass::Layer2CompatibilityBreadth
    } else {
        TsJsAnswerLayerClass::Layer1SyntaxStructure
    }
}

fn semantic_state(
    decision: &RouterDecisionRecord,
    selected_host: Option<&LanguageServerHostStatus>,
) -> SemanticState {
    if let Some(host) = selected_host {
        if host_claims_complete_semantics(host) {
            return SemanticState {
                result_confidence_class: TsJsResultConfidenceClass::Exact,
                completeness_class: TsJsCompletenessClass::CompleteForDeclaredScope,
                inline_visibility_class: TsJsInlineVisibilityClass::InlineAuthoritativeAllowed,
            };
        }
        return SemanticState {
            result_confidence_class: TsJsResultConfidenceClass::WorkspaceSliceLimited,
            completeness_class: TsJsCompletenessClass::PartialForDeclaredScope,
            inline_visibility_class: TsJsInlineVisibilityClass::InlineCaveatedAllowed,
        };
    }

    if selected_provider_is_syntax(decision) {
        return SemanticState {
            result_confidence_class: TsJsResultConfidenceClass::HeuristicallyMapped,
            completeness_class: TsJsCompletenessClass::PartialForDeclaredScope,
            inline_visibility_class: TsJsInlineVisibilityClass::InlineCaveatedAllowed,
        };
    }

    SemanticState {
        result_confidence_class: TsJsResultConfidenceClass::Unavailable,
        completeness_class: TsJsCompletenessClass::UnavailableForDeclaredScope,
        inline_visibility_class: TsJsInlineVisibilityClass::InlineUnavailable,
    }
}

fn relation_for_occurrence(occurrence: &super::records::TsJsOccurrenceSeed) -> TsJsRelationClass {
    if occurrence.generated_or_external_state_class
        == TsJsGeneratedOrExternalStateClass::GeneratedSource
    {
        TsJsRelationClass::GeneratedOrFrameworkReference
    } else {
        occurrence.access_kind_class.relation_class()
    }
}

fn provider_family_for_kind(provider_kind: ProviderKind) -> ProviderFamily {
    match provider_kind {
        ProviderKind::SyntaxParser => ProviderFamily::Syntax,
        ProviderKind::LanguageServer => ProviderFamily::LanguageServer,
        ProviderKind::FrameworkPack => ProviderFamily::FrameworkPack,
        ProviderKind::GeneratedSourceBridge => ProviderFamily::GeneratedSourceBridge,
        ProviderKind::ProjectGraph => ProviderFamily::ProjectGraph,
        ProviderKind::AiAssist => ProviderFamily::AiAssist,
        ProviderKind::NativeAnalyzer
        | ProviderKind::DebugAdapter
        | ProviderKind::FormatterAdapter
        | ProviderKind::LinterAdapter
        | ProviderKind::TestAdapter
        | ProviderKind::BuildAdapter => ProviderFamily::LanguageServer,
    }
}

fn symbol_file_scope_ref(decision: &RouterDecisionRecord) -> String {
    format!(
        "scope:file:{}",
        sanitize_id(&decision.request_context.requested_subject_ref)
    )
}

fn reference_count_summary(
    symbol: &TsJsSymbolSeed,
    occurrence_results: &[TsJsSemanticResultRecord],
) -> TsJsReferenceCountSummary {
    let materialized_count = occurrence_results.len();
    let generated_count = occurrence_results
        .iter()
        .filter(|result| result.relation_class == TsJsRelationClass::GeneratedOrFrameworkReference)
        .count();
    TsJsReferenceCountSummary {
        total_count: symbol.occurrences.len(),
        materialized_count,
        omitted_count: symbol.occurrences.len().saturating_sub(materialized_count),
        generated_count,
        readonly_count: 0,
    }
}

fn rename_count_summary(
    symbol: &TsJsSymbolSeed,
    rename_occurrences: &[&super::records::TsJsOccurrenceSeed],
) -> TsJsRenameCountSummary {
    let changed_count = rename_occurrences.len();
    let changed_files = rename_occurrences
        .iter()
        .map(|occurrence| occurrence.anchor.canonical_file_ref.as_str())
        .collect::<BTreeSet<_>>();
    let protected_count = symbol
        .occurrences
        .iter()
        .filter(|occurrence| {
            occurrence.rename_candidate
                && (occurrence.readonly
                    || occurrence.generated_or_external_state_class
                        == TsJsGeneratedOrExternalStateClass::ReadOnlySource)
        })
        .count();
    let generated_count = symbol
        .occurrences
        .iter()
        .filter(|occurrence| {
            occurrence.rename_candidate
                && occurrence.generated_or_external_state_class
                    == TsJsGeneratedOrExternalStateClass::GeneratedSource
        })
        .count();
    let total_candidates = symbol
        .occurrences
        .iter()
        .filter(|occurrence| occurrence.rename_candidate)
        .count();
    let accounted = changed_count + protected_count;
    TsJsRenameCountSummary {
        changed_count,
        unresolved_count: 0,
        generated_count,
        protected_count,
        skipped_count: total_candidates.saturating_sub(accounted),
        changed_file_count: changed_files.len(),
        changed_symbol_count: usize::from(changed_count > 0),
    }
}

fn rename_preview_completeness(
    decision: &RouterDecisionRecord,
    selected_host: Option<&LanguageServerHostStatus>,
    count_summary: &TsJsRenameCountSummary,
) -> TsJsRenamePreviewCompletenessClass {
    if count_summary.changed_count == 0 {
        return TsJsRenamePreviewCompletenessClass::UnavailableBlocked;
    }
    if selected_host.is_some_and(host_claims_complete_semantics)
        && count_summary.protected_count == 0
        && count_summary.skipped_count == 0
    {
        return TsJsRenamePreviewCompletenessClass::CompleteForRequestedScope;
    }
    if selected_provider_is_syntax(decision) {
        TsJsRenamePreviewCompletenessClass::PartialDueToIndexOrProvider
    } else if selected_host.is_some_and(|host| !host.scope_limit_classes.is_empty()) {
        TsJsRenamePreviewCompletenessClass::PartialDueToWorkspaceSlice
    } else if count_summary.generated_count > 0 {
        TsJsRenamePreviewCompletenessClass::PartialDueToImportedOrGeneratedBoundaries
    } else {
        TsJsRenamePreviewCompletenessClass::PartialDueToWorkspaceSlice
    }
}

fn rename_apply_posture(
    preview_completeness_class: TsJsRenamePreviewCompletenessClass,
    count_summary: &TsJsRenameCountSummary,
) -> TsJsApplyPostureClass {
    if preview_completeness_class == TsJsRenamePreviewCompletenessClass::UnavailableBlocked {
        TsJsApplyPostureClass::InspectOnlyUnavailable
    } else if count_summary.protected_count > 0 {
        TsJsApplyPostureClass::BlockedPendingPolicyOrProtectedReview
    } else if preview_completeness_class
        == TsJsRenamePreviewCompletenessClass::CompleteForRequestedScope
    {
        TsJsApplyPostureClass::ReadyForApplyAfterPreview
    } else if preview_completeness_class
        == TsJsRenamePreviewCompletenessClass::PartialDueToIndexOrProvider
    {
        TsJsApplyPostureClass::BlockedPendingRefresh
    } else {
        TsJsApplyPostureClass::BlockedPendingScopeReview
    }
}

fn materialized_rename_scope(
    decision: &RouterDecisionRecord,
    selected_host: Option<&LanguageServerHostStatus>,
) -> ScopeClaimClass {
    if selected_provider_is_syntax(decision) {
        ScopeClaimClass::SingleFile
    } else {
        selected_host
            .map(|host| host.scope_claim_class)
            .unwrap_or(ScopeClaimClass::Unavailable)
    }
}

fn rename_coverage_limits(
    decision: &RouterDecisionRecord,
    selected_host: Option<&LanguageServerHostStatus>,
    preview_completeness_class: TsJsRenamePreviewCompletenessClass,
) -> Vec<TsJsRenameCoverageLimitClass> {
    if preview_completeness_class == TsJsRenamePreviewCompletenessClass::CompleteForRequestedScope {
        return Vec::new();
    }
    if selected_provider_is_syntax(decision) {
        return vec![
            TsJsRenameCoverageLimitClass::ProviderUnavailable,
            TsJsRenameCoverageLimitClass::SemanticIndexPartial,
        ];
    }

    let mut limits = selected_host
        .map(|host| host.scope_limit_classes.as_slice())
        .unwrap_or(&[])
        .iter()
        .map(|limit| match limit {
            ScopeLimitClass::ActiveWorksetOnly
            | ScopeLimitClass::SingleFileOnly
            | ScopeLimitClass::UnloadedRootsOmitted => {
                TsJsRenameCoverageLimitClass::WorkspaceSliceLimited
            }
            ScopeLimitClass::GeneratedOverlayOnly | ScopeLimitClass::GeneratedCandidatesOmitted => {
                TsJsRenameCoverageLimitClass::GeneratedLineageUnresolved
            }
            ScopeLimitClass::PolicyNarrowed => TsJsRenameCoverageLimitClass::PolicyNarrowed,
            ScopeLimitClass::RemoteShardUnreachable => {
                TsJsRenameCoverageLimitClass::RemoteShardUnreachable
            }
            ScopeLimitClass::NotebookCellProjectionOnly
            | ScopeLimitClass::CrossCellContextUnavailable
            | ScopeLimitClass::DiffOrReviewSliceOnly => {
                TsJsRenameCoverageLimitClass::SemanticIndexPartial
            }
        })
        .collect::<Vec<_>>();
    if limits.is_empty() {
        limits.push(TsJsRenameCoverageLimitClass::SemanticIndexPartial);
    }
    limits.sort_by_key(|limit| format!("{limit:?}"));
    limits.dedup();
    limits
}

fn rename_warning_rows(
    symbol: &TsJsSymbolSeed,
    affected_result_refs: &[String],
    count_summary: &TsJsRenameCountSummary,
) -> Vec<TsJsRenameWarningRow> {
    let mut rows = Vec::new();
    if count_summary.generated_count > 0 {
        let generated_refs = symbol
            .occurrences
            .iter()
            .filter(|occurrence| {
                occurrence.rename_candidate
                    && occurrence.generated_or_external_state_class
                        == TsJsGeneratedOrExternalStateClass::GeneratedSource
            })
            .map(|occurrence| reference_result_id(&symbol.symbol_ref, &occurrence.occurrence_ref))
            .collect::<Vec<_>>();
        rows.push(TsJsRenameWarningRow {
            warning_class: TsJsRenameWarningClass::GeneratedReferenceWouldChange,
            warning_count: count_summary.generated_count,
            affected_result_refs: generated_refs,
            summary: "Generated or paired occurrences are visible in the preview and are not silently mutated.".into(),
        });
    }
    if count_summary.protected_count > 0 {
        rows.push(TsJsRenameWarningRow {
            warning_class: TsJsRenameWarningClass::ProtectedOrReadOnlyTarget,
            warning_count: count_summary.protected_count,
            affected_result_refs: affected_result_refs.to_vec(),
            summary: "Read-only or protected occurrences require review before apply.".into(),
        });
    }
    if count_summary.skipped_count > 0 {
        rows.push(TsJsRenameWarningRow {
            warning_class: TsJsRenameWarningClass::WorkspaceSliceLimited,
            warning_count: count_summary.skipped_count,
            affected_result_refs: affected_result_refs.to_vec(),
            summary: "Some rename candidates are outside the materialized scope or excluded by fallback limits.".into(),
        });
    }
    rows
}

fn checkpoint_descriptor(
    apply_posture_class: TsJsApplyPostureClass,
    rename_preview_id: &str,
    symbol_ref: &str,
) -> super::records::TsJsRenameCheckpointDescriptor {
    match apply_posture_class {
        TsJsApplyPostureClass::ReadyForApplyAfterPreview => {
            super::records::TsJsRenameCheckpointDescriptor {
                checkpoint_class: TsJsCheckpointClass::CheckpointCaptured,
                checkpoint_ref: Some(format!("checkpoint:tsjs:rename:{}", sanitize_id(symbol_ref))),
                rollback_ref: Some(format!("rollback:tsjs:rename:{}", sanitize_id(symbol_ref))),
                rollback_path_class: TsJsRollbackPathClass::ExactUndoViaLocalHistoryCheckpoint,
                summary: "Local history checkpoint is available before apply.".into(),
            }
        }
        TsJsApplyPostureClass::InspectOnlyUnavailable => {
            super::records::TsJsRenameCheckpointDescriptor {
                checkpoint_class: TsJsCheckpointClass::CheckpointNotRequiredInspectOnly,
                checkpoint_ref: None,
                rollback_ref: None,
                rollback_path_class: TsJsRollbackPathClass::NoSafeRollbackAvailable,
                summary: "Preview is inspect-only, so no mutation checkpoint is claimed.".into(),
            }
        }
        _ => super::records::TsJsRenameCheckpointDescriptor {
            checkpoint_class: TsJsCheckpointClass::CheckpointRequiredNotCaptured,
            checkpoint_ref: None,
            rollback_ref: None,
            rollback_path_class: TsJsRollbackPathClass::ManualReviewRequiredNoAutomaticPath,
            summary: format!("{rename_preview_id} must be reviewed before a checkpointed apply path is available."),
        },
    }
}

fn rename_scope_summary(
    preview_completeness_class: TsJsRenamePreviewCompletenessClass,
    snapshot: &TsJsLaunchWedgeSnapshot,
) -> String {
    match preview_completeness_class {
        TsJsRenamePreviewCompletenessClass::CompleteForRequestedScope => format!(
            "Preview is complete for {}.",
            snapshot.workspace_context.scope_label
        ),
        TsJsRenamePreviewCompletenessClass::PartialDueToWorkspaceSlice => format!(
            "Preview is limited to {}; omitted roots remain visible as scope caveats.",
            snapshot.workspace_context.scope_label
        ),
        TsJsRenamePreviewCompletenessClass::PartialDueToIndexOrProvider => {
            "Preview is a lower-authority file-local/text fallback because semantic providers are unavailable or incomplete.".into()
        }
        TsJsRenamePreviewCompletenessClass::PartialDueToImportedOrGeneratedBoundaries => {
            "Preview includes generated or imported boundaries and requires review before apply.".into()
        }
        TsJsRenamePreviewCompletenessClass::StaleRequiresRefresh => {
            "Preview requires a provider refresh before apply.".into()
        }
        TsJsRenamePreviewCompletenessClass::UnavailableBlocked => {
            "Rename preview is unavailable for mutation and can only explain why.".into()
        }
        TsJsRenamePreviewCompletenessClass::FullWorkspaceComplete => {
            "Preview covers the whole admitted workspace.".into()
        }
    }
}

fn redaction_for_semantic_state(semantic_state: SemanticState) -> RedactionClass {
    if semantic_state.result_confidence_class.requires_disclosure()
        || semantic_state.completeness_class.requires_disclosure()
    {
        RedactionClass::InternalSupportRestricted
    } else {
        RedactionClass::MetadataSafeDefault
    }
}

fn redaction_for_preview(
    preview_completeness_class: TsJsRenamePreviewCompletenessClass,
) -> RedactionClass {
    if preview_completeness_class.blocks_direct_apply() {
        RedactionClass::InternalSupportRestricted
    } else {
        RedactionClass::MetadataSafeDefault
    }
}

fn scope_limit_caveat_refs(scope_descriptor: &TsJsScopeDescriptor) -> Vec<String> {
    let mut refs = scope_descriptor
        .scope_limit_classes
        .iter()
        .map(|limit| format!("scope:caveat:{limit:?}"))
        .collect::<Vec<_>>();
    if let Some(omitted_scope_ref) = &scope_descriptor.omitted_scope_ref {
        refs.push(omitted_scope_ref.clone());
    }
    refs
}

fn scope_caveat_refs(
    context: &super::records::TsJsWorkspaceContext,
    coverage_limits: &[TsJsRenameCoverageLimitClass],
) -> Vec<String> {
    let mut refs = context.omitted_scope_refs.clone();
    refs.extend(
        coverage_limits
            .iter()
            .map(|limit| format!("scope:caveat:{limit:?}")),
    );
    refs.sort();
    refs.dedup();
    refs
}

fn definition_result_id(symbol_ref: &str) -> String {
    format!(
        "nav:semantic:result:tsjs:definition:{}",
        sanitize_id(symbol_ref)
    )
}

fn reference_result_id(symbol_ref: &str, occurrence_ref: &str) -> String {
    format!(
        "nav:semantic:result:tsjs:reference:{}:{}",
        sanitize_id(symbol_ref),
        sanitize_id(occurrence_ref)
    )
}

fn sanitize_id(value: &str) -> String {
    let sanitized = value
        .trim()
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>();
    sanitized.trim_matches('-').to_owned()
}
