//! Canonical seed builders for the M5 line-publish_to_review_sheet and line-downgrade-packet registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean line-publish_to_review_sheet and line-downgrade-packet entries
//! are built so the one typed line-publish_to_review_sheet object resolving per line, lines never widening without
//! preserving rollback and diagnostics, partner / public support language never running ahead of line proof,
//! the canonical / accessible / audit resolution forms, and the complete line-identity / known-limits-ledger /
//! rollback-target / rehearsal-currency / readiness-signoff / support-language / last-widening-revision
//! line-downgrade object are proven across the shiproom, release-center, executive-steering,
//! program-governance, diagnostics, and support surfaces without any hand-copied per-line assumption,
//! widen-without-rollback, incomplete object, hidden line downgrade, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_AI_REVIEW_PUBLISH_SHEET_AND_SCOPE_DECISION_REGISTRIES_PACKET_ID: &str =
    "m5-ai-review-publish-sheet-and-scope-decision-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn publish_to_review_sheet(
    input: M5PublishToReviewSheetEntryResolutionInput,
) -> M5ResolvedPublishToReviewSheetEntry {
    resolve_publish_to_review_sheet_entry(input)
        .expect("seed line-publish_to_review_sheet entry resolves")
}

fn downgrade(
    input: M5PublishScopeDecisionEntryResolutionInput,
) -> M5ResolvedPublishScopeDecisionEntry {
    resolve_publish_scope_decision_entry(input).expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5PublishToReviewSheetResolutionForm> {
    M5PublishToReviewSheetResolutionForm::ALL.to_vec()
}

// -- Clean line-publish_to_review_sheet entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_publish_to_review_sheet_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5AiReviewAssistRole,
    report_section: M5PublishToReviewSheetKind,
    surface_context: M5PublishToReviewSheetSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5PublishToReviewSheetEntryResolutionInput {
    M5PublishToReviewSheetEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        line_binding_id: line_binding_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        report_section,
        surface_context,
        resolution_form_coverage: all_forms(),
        exact_repo_journey_rows: exact_repo_journey_rows.to_owned(),
        bundle_ids: bundle_ids.to_owned(),
        install_topology: install_topology.to_owned(),
        toolchain_envelope: toolchain_envelope.to_owned(),
        known_limits: known_limits.to_owned(),
        rollback_target: rollback_target.to_owned(),
        diagnostics_posture: diagnostics_posture.to_owned(),
        bound_to_registry: true,
        rollback_and_diagnostics_bounded: true,
        is_public_facing_line: false,
        support_language_matches_line_proof: true,
        proof_fresh: true,
    }
}

fn publish_to_review_sheet_local_draft_clean() -> M5ResolvedPublishToReviewSheetEntry {
    publish_to_review_sheet(clean_publish_to_review_sheet_base(
        "publish_to_review_sheet:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.publish_to_review_sheet.core_team_canary",
        M5AiReviewAssistRole::FindingClassification,
        M5PublishToReviewSheetKind::LocalDraft,
        M5PublishToReviewSheetSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn publish_to_review_sheet_publish_now_provider_comment_clean(
) -> M5ResolvedPublishToReviewSheetEntry {
    publish_to_review_sheet(clean_publish_to_review_sheet_base(
        "publish_to_review_sheet:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.publish_to_review_sheet.publish_now_provider_comment",
        M5AiReviewAssistRole::AnalyzedScopeDisclosure,
        M5PublishToReviewSheetKind::PublishNowProviderComment,
        M5PublishToReviewSheetSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn publish_to_review_sheet_publish_now_suggested_patch_clean() -> M5ResolvedPublishToReviewSheetEntry
{
    publish_to_review_sheet(clean_publish_to_review_sheet_base(
        "publish_to_review_sheet:program-governance:extension-author",
        "launch.line.extension-author",
        "line.publish_to_review_sheet.publish_now_suggested_patch",
        M5AiReviewAssistRole::PublishDestinationDisclosure,
        M5PublishToReviewSheetKind::PublishNowSuggestedPatch,
        M5PublishToReviewSheetSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn publish_to_review_sheet_publish_now_check_annotation_clean(
) -> M5ResolvedPublishToReviewSheetEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_publish_to_review_sheet_base(
        "publish_to_review_sheet:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.publish_to_review_sheet.publish_now_check_annotation",
        M5AiReviewAssistRole::FindingClassification,
        M5PublishToReviewSheetKind::PublishNowCheckAnnotation,
        M5PublishToReviewSheetSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.design-partner-preview-journeys",
        "bundle.ids.design-partner-0007",
        "install.topology.enrolled-design-partners",
        "toolchain.envelope.pinned-partner",
        "known-limits.published.design-partner",
        "rollback.target.partner-previous-preview",
        "diagnostics.posture.partner-telemetry",
    );
    base.is_public_facing_line = true;
    base.support_language_matches_line_proof = true;
    publish_to_review_sheet(base)
}

fn publish_to_review_sheet_open_in_provider_clean() -> M5ResolvedPublishToReviewSheetEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_publish_to_review_sheet_base(
        "publish_to_review_sheet:support:public-preview",
        "launch.line.public-preview",
        "line.publish_to_review_sheet.open_in_provider",
        M5AiReviewAssistRole::AnalyzedScopeDisclosure,
        M5PublishToReviewSheetKind::OpenInProvider,
        M5PublishToReviewSheetSurfaceContext::SupportOrExportForm,
        "repo.rows.public-preview-journeys",
        "bundle.ids.public-preview-0007",
        "install.topology.public-preview-ring",
        "toolchain.envelope.pinned-public",
        "known-limits.published.public-preview",
        "rollback.target.public-previous-stable",
        "diagnostics.posture.public-telemetry",
    );
    base.is_public_facing_line = true;
    base.support_language_matches_line_proof = true;
    publish_to_review_sheet(base)
}

fn publish_to_review_sheet_export_fallback_offline_clean() -> M5ResolvedPublishToReviewSheetEntry {
    publish_to_review_sheet(clean_publish_to_review_sheet_base(
        "publish_to_review_sheet:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.publish_to_review_sheet.export_fallback_offline",
        M5AiReviewAssistRole::LocalVersusProviderState,
        M5PublishToReviewSheetKind::ExportFallbackOffline,
        M5PublishToReviewSheetSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded line-publish_to_review_sheet entries ---------------------------------------------------------

/// Degraded publish_to_review_sheet entry: the resolved publish_to_review_sheet object is incomplete — the bundle IDs are unstated.
fn publish_to_review_sheet_object_incomplete() -> M5ResolvedPublishToReviewSheetEntry {
    let mut base = clean_publish_to_review_sheet_base(
        "publish_to_review_sheet:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.publish_to_review_sheet.core_team_canary",
        M5AiReviewAssistRole::FindingClassification,
        M5PublishToReviewSheetKind::LocalDraft,
        M5PublishToReviewSheetSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    publish_to_review_sheet(base)
}

/// Degraded publish_to_review_sheet entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn publish_to_review_sheet_widen_fold() -> M5ResolvedPublishToReviewSheetEntry {
    let mut base = clean_publish_to_review_sheet_base(
        "publish_to_review_sheet:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.publish_to_review_sheet.publish_now_provider_comment",
        M5AiReviewAssistRole::AnalyzedScopeDisclosure,
        M5PublishToReviewSheetKind::PublishNowProviderComment,
        M5PublishToReviewSheetSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    publish_to_review_sheet(base)
}

/// Degraded publish_to_review_sheet entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn publish_to_review_sheet_unbound() -> M5ResolvedPublishToReviewSheetEntry {
    let mut base = clean_publish_to_review_sheet_base(
        "publish_to_review_sheet:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.publish_to_review_sheet.export_fallback_offline",
        M5AiReviewAssistRole::LocalVersusProviderState,
        M5PublishToReviewSheetKind::ExportFallbackOffline,
        M5PublishToReviewSheetSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    publish_to_review_sheet(base)
}

/// Degraded publish_to_review_sheet entry: the canonical registry token name is unstated.
fn publish_to_review_sheet_token_unstated() -> M5ResolvedPublishToReviewSheetEntry {
    let mut base = clean_publish_to_review_sheet_base(
        "publish_to_review_sheet:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5AiReviewAssistRole::PublishDestinationDisclosure,
        M5PublishToReviewSheetKind::PublishNowSuggestedPatch,
        M5PublishToReviewSheetSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    publish_to_review_sheet(base)
}

// -- Clean line-downgrade-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_downgrade_base(
    entry_id: &str,
    comparison_ref: &str,
    token_name: &str,
    semantic_role: M5AiReviewAssistRole,
    comparison_scope: M5PublishScopeDecisionScope,
    surface_context: M5PublishToReviewSheetSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5PublishScopeDecisionEntryResolutionInput {
    M5PublishScopeDecisionEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        comparison_ref: comparison_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        comparison_scope,
        surface_context,
        resolution_form_coverage: all_forms(),
        resolved_line_identity: resolved_line_identity.to_owned(),
        known_limits_ledger: known_limits_ledger.to_owned(),
        rollback_target_reference: rollback_target_reference.to_owned(),
        rehearsal_currency_state: rehearsal_currency_state.to_owned(),
        readiness_signoff_state: readiness_signoff_state.to_owned(),
        support_language_reference: support_language_reference.to_owned(),
        last_widening_revision: last_widening_revision.to_owned(),
        keeps_publish_scope_decision_visible: true,
        comparison_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn downgrade_dogfood_ring_clean() -> M5ResolvedPublishScopeDecisionEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5AiReviewAssistRole::FindingClassification,
        M5PublishScopeDecisionScope::PublishScopeAllowed,
        M5PublishToReviewSheetSurfaceContext::ShiproomSurface,
        "line-id.core-team-canary-0007",
        "known-limits.ledger.canary",
        "rollback.target.ref.canary",
        "rehearsal.currency.dogfood-ring-current",
        "readiness.signoff.dogfood-reviewed",
        "support.language.canary-bound-to-proof",
        "widening.revision.0007",
    );
    base.support_language_present = true;
    base.support_language_bound_to_proof = true;
    downgrade(base)
}

fn downgrade_rehearsal_currency_clean() -> M5ResolvedPublishScopeDecisionEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.publish_now_suggested_patch",
        M5AiReviewAssistRole::PublishDestinationDisclosure,
        M5PublishScopeDecisionScope::PublishScopeDowngraded,
        M5PublishToReviewSheetSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedPublishScopeDecisionEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.export_fallback_offline",
        M5AiReviewAssistRole::LocalVersusProviderState,
        M5PublishScopeDecisionScope::PublishScopeBlocked,
        M5PublishToReviewSheetSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    ))
}

// -- Degraded line-downgrade-packet entries ----------------------------------------------------

/// Degraded downgrade entry: the downgrade would run partner / public support language ahead of line proof — a
/// support-language reference present but not bound to line proof reads as trustworthy when the line proof
/// does not yet back it.
fn downgrade_support_ahead() -> M5ResolvedPublishScopeDecisionEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.open_in_provider",
        M5AiReviewAssistRole::AnalyzedScopeDisclosure,
        M5PublishScopeDecisionScope::PublishScopeAllowed,
        M5PublishToReviewSheetSurfaceContext::ShiproomSurface,
        "line-id.public-preview-0007",
        "known-limits.ledger.public-preview",
        "rollback.target.ref.public-preview",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.public-preview-reviewed",
        "support.language.public-ahead-of-proof",
        "widening.revision.0007",
    );
    base.support_language_present = true;
    base.support_language_bound_to_proof = false;
    downgrade(base)
}

/// Degraded downgrade entry: the canonical / accessible / audit resolution-form coverage of the downgrade is
/// incomplete.
fn downgrade_form_incomplete() -> M5ResolvedPublishScopeDecisionEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.export_fallback_offline",
        M5AiReviewAssistRole::LocalVersusProviderState,
        M5PublishScopeDecisionScope::PublishScopeBlocked,
        M5PublishToReviewSheetSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5PublishToReviewSheetResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn comparison_publish_scope_unclassified() -> M5ResolvedPublishScopeDecisionEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.publish_now_check_annotation",
        M5AiReviewAssistRole::FindingClassification,
        M5PublishScopeDecisionScope::PublishScopeUnclassified,
        M5PublishToReviewSheetSurfaceContext::ExecutiveSteeringSurface,
        "line-id.design-partner-preview-0007",
        "known-limits.ledger.design-partner",
        "rollback.target.ref.design-partner",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.design-partner-reviewed",
        "support.language.design-partner-bound-to-proof",
        "widening.revision.0007",
    ))
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5AiReviewPublishSheetAndScopeDecisionRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5AiReviewAssistDowngradeTrigger>,
    publish_to_review_sheet_entries: Vec<M5ResolvedPublishToReviewSheetEntry>,
    publish_scope_decision_entries: Vec<M5ResolvedPublishScopeDecisionEntry>,
) -> M5AiReviewPublishSheetAndScopeDecisionRegistriesRow {
    M5AiReviewPublishSheetAndScopeDecisionRegistriesRow {
        consumer_surface,
        qualification: M5AiReviewAssistQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        capture_lifecycle_stages: M5AiReviewAssistClassificationStage::ALL.to_vec(),
        required_labels: M5AiReviewAssistRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5AiReviewAssistAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5PublishToReviewSheetAnatomyPart::ALL.to_vec(),
        export_fields: M5PublishToReviewSheetExportField::ALL.to_vec(),
        downgrade_triggers,
        publish_to_review_sheet_entries,
        publish_scope_decision_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_AI_REVIEW_PUBLISH_SHEET_AND_SCOPE_DECISION_REGISTRIES_SCHEMA_REF,
            M5_AI_REVIEW_PUBLISH_SHEET_DOMAIN_SCHEMA_REF,
            M5_AI_REVIEW_RESOLUTION_MEMORY_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_publish_scope_decision_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5AiReviewPublishSheetAndScopeDecisionRegistriesRow> {
    use M5AiReviewAssistConsumerSurface as C;
    use M5AiReviewAssistDowngradeTrigger as D;

    vec![
        base_row(
            C::ReviewDetail,
            "Review-detail owner",
            "Review detail resolves an AI review finding to one typed publish-to-review sheet — the target provider, thread or check-run target, outbound text preview, review artifact class (comment, suggested patch, or provider-specific check annotation), attribution state, redaction note, and publish / copy / export / cancel actions — from the shared registry and proves the publish-scope-decision that makes the publish path allowed; a sheet missing its outbound destination or text preview and a scope decision that flattens a provider write failure into a generic error degrade honestly instead of letting an outbound action look ready",
            "retirement:m5-release-center:001",
            vec![
                D::PublishModeUnstated,
                D::PublishDestinationHidden,
                D::AiReviewAssistMatrixStale,
            ],
            vec![
                publish_to_review_sheet_local_draft_clean(),
                publish_to_review_sheet_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::AiReviewPanel,
            "AI-review-panel owner",
            "The AI review panel resolves the publish, copy, export, and cancel actions and the publish-scope-decision (a blocked publish path) while keeping the active permission-scope reason visible; a publish sheet that would commit implicitly without an explicit outbound preview and a resolution-form gap on a scope decision are caught before anything becomes durable provider history",
            "retirement:m5-help-docs:001",
            vec![
                D::PublishModeUnstated,
                D::PublishExportFallbackMissing,
                D::AiReviewAssistMatrixStale,
            ],
            vec![publish_to_review_sheet_publish_now_provider_comment_clean(), publish_to_review_sheet_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::SupportExportPacket,
            "Support owner",
            "Support / export resolves the same publish packet — destination, attribution, and redaction state — keeping it auditable outside the live provider UI, and reports the publish-scope-decision outcome; a sheet that is a hand-copied per-entry assumption and a decision on an unclassified publish scope degrade honestly",
            "retirement:m5-support:001",
            vec![
                D::AnalyzedScopeUnstated,
                D::LifecycleStateMissing,
                D::AiReviewAssistMatrixStale,
            ],
            vec![
                publish_to_review_sheet_publish_now_check_annotation_clean(),
                publish_to_review_sheet_unbound(),
            ],
            vec![comparison_publish_scope_unclassified()],
        ),
        base_row(
            C::FindingRow,
            "Finding-row owner",
            "The finding row resolves the review-artifact-class field and the publish-scope-downgraded decision bound to the registry so a narrowed provider scope surfaces as an explicit publish-state explanation with copy / export fallback rather than a generic failure; an unstated registry token on a publish sheet is caught before it can drift",
            "retirement:m5-marketplace-registry:001",
            vec![
                D::AnalyzedScopeUnstated,
                D::FindingClassBadgeMissing,
                D::AiReviewAssistMatrixStale,
            ],
            vec![
                publish_to_review_sheet_publish_now_suggested_patch_clean(),
                publish_to_review_sheet_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::PublishToReviewSheet,
            "Publish-to-review-sheet owner",
            "The publish-to-review sheet renders the same resolved publish-to-review-sheet and publish-scope-decision truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied outbound table; the outbound destination and text preview stay inspectable off-renderer so a user always reviews exactly what leaves the client before it becomes durable provider history",
            "retirement:m5-install-update:001",
            vec![
                D::FindingClassBadgeMissing,
                D::AnalyzedScopeUnstated,
                D::AiReviewAssistMatrixStale,
            ],
            vec![publish_to_review_sheet_export_fallback_offline_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::ProviderPublishReview,
            "Provider-publish-review owner",
            "The provider publish-review feed carries the same resolved publish-to-review-sheet and publish-scope-decision truth, so a hand-copied constant, an unstated registry token, an outbound action publishing without an explicit destination and preview, or a provider write failure flattened into a generic error is visible in evidence — publish allowed, downgraded, or blocked — rather than hidden behind a screenshot",
            "retirement:m5-partner-procurement:001",
            vec![
                D::AnalyzedScopeUnstated,
                D::PublishDestinationHidden,
                D::AiReviewAssistMatrixStale,
            ],
            vec![publish_to_review_sheet_open_in_provider_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5AiReviewPublishSheetAndScopeDecisionRegistriesGovernanceReview {
    M5AiReviewPublishSheetAndScopeDecisionRegistriesGovernanceReview {
        publish_to_review_sheet_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_publish_to_review_sheet_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        publish_scope_decision_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        publish_to_review_sheet_or_downgrade_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5AiReviewPublishSheetAndScopeDecisionRegistriesConsumerProjection {
    M5AiReviewPublishSheetAndScopeDecisionRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5AiReviewPublishSheetAndScopeDecisionRegistriesProofFreshness {
    M5AiReviewPublishSheetAndScopeDecisionRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5AiReviewPublishSheetAndScopeDecisionRegistriesReleasePosture {
    M5AiReviewPublishSheetAndScopeDecisionRegistriesReleasePosture {
        proof_packet_ref: M5_AI_REVIEW_PUBLISH_SHEET_AND_SCOPE_DECISION_REGISTRIES_ARTIFACT_REF
            .to_owned(),
        line_audit_ref: M5_AI_REVIEW_PUBLISH_SHEET_AND_SCOPE_DECISION_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_AI_REVIEW_PUBLISH_SHEET_AND_SCOPE_DECISION_REGISTRIES_SCHEMA_REF,
        M5_AI_REVIEW_PUBLISH_SHEET_AND_SCOPE_DECISION_REGISTRIES_DOC_REF,
        M5_AI_REVIEW_ASSIST_MATRIX_SCHEMA_REF,
        M5_AI_REVIEW_ASSIST_MATRIX_DOC_REF,
        M5_AI_REVIEW_PUBLISH_SHEET_DOMAIN_SCHEMA_REF,
        M5_AI_REVIEW_RESOLUTION_MEMORY_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 post-launch public-proof-ledger and claim-history-diff registries packet.
pub fn seeded_m5_ai_review_publish_sheet_and_scope_decision_registries(
) -> M5AiReviewPublishSheetAndScopeDecisionRegistriesPacket {
    M5AiReviewPublishSheetAndScopeDecisionRegistriesPacket::new(
        M5AiReviewPublishSheetAndScopeDecisionRegistriesPacketInput {
            packet_id: M5_AI_REVIEW_PUBLISH_SHEET_AND_SCOPE_DECISION_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 publish-to-review-sheet and publish-scope-decision registries emitting one machine-readable publish-to-review sheet per outbound AI review publish action — one typed field per section: the target provider, the thread or check-run target, the outbound text preview, the review artifact class (comment, suggested patch, or provider-specific check annotation), the attribution state, the redaction note, and the publish / copy / export / cancel actions — each bound to one object-class identity, so an outbound action never publishes or merges implicitly and never hides whether output stays local, becomes a provider comment, a suggested patch, or a check annotation, with canonical / accessible / audit resolution-form coverage, and a machine-readable publish-scope-decision (publish-scope-allowed, publish-scope-downgraded, or publish-scope-blocked) that turns missing or narrowed provider scope into an explicit publish-state explanation with copy / export fallback rather than a generic publish failure, and preserves the same publish packet — attribution, destination, and redaction state — in local history and support / export so outbound review state stays auditable outside the live provider UI, across review, AI, provider, and support / export surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5AiReviewPublishSheetAndScopeDecisionRegistriesVocabularySet::canonical(),
            governance_review: governance_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            release_posture: release_posture(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Narrowed variant: the shiproom row is held at Beta pending public-proof-ledger parity on every proof source;
/// every row stays visible and every example stays honest.
pub fn seeded_m5_ai_review_publish_sheet_and_scope_decision_registries_publish_to_review_sheet_beta_narrowed(
) -> M5AiReviewPublishSheetAndScopeDecisionRegistriesPacket {
    let mut packet = seeded_m5_ai_review_publish_sheet_and_scope_decision_registries();
    packet.packet_id =
        "m5-ai-review-publish-sheet-and-scope-decision-registries:snapshot-descriptor-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5AiReviewAssistConsumerSurface::ReviewDetail)
        .expect("shiproom row present");
    row.qualification = M5AiReviewAssistQualificationClass::Beta;
    packet
}

/// Narrowed variant: the release-center row is narrowed to Preview pending claim-history-diff parity on every
/// diff scope; every row stays visible and every example stays honest.
pub fn seeded_m5_ai_review_publish_sheet_and_scope_decision_registries_publish_scope_decision_preview_narrowed(
) -> M5AiReviewPublishSheetAndScopeDecisionRegistriesPacket {
    let mut packet = seeded_m5_ai_review_publish_sheet_and_scope_decision_registries();
    packet.packet_id =
        "m5-ai-review-publish-sheet-and-scope-decision-registries:publish-scope-decision-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5AiReviewAssistConsumerSurface::AiReviewPanel)
        .expect("release-center row present");
    row.qualification = M5AiReviewAssistQualificationClass::Preview;
    packet
}
