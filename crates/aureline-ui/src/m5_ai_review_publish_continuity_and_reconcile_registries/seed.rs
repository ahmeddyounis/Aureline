//! Canonical seed builders for the M5 line-publish_later_draft and line-downgrade-packet registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean line-publish_later_draft and line-downgrade-packet entries
//! are built so the one typed line-publish_later_draft object resolving per line, lines never widening without
//! preserving rollback and diagnostics, partner / public support language never running ahead of line proof,
//! the canonical / accessible / audit resolution forms, and the complete line-identity / known-limits-ledger /
//! rollback-target / rehearsal-currency / readiness-signoff / support-language / last-widening-revision
//! line-downgrade object are proven across the shiproom, release-center, executive-steering,
//! program-governance, diagnostics, and support surfaces without any hand-copied per-line assumption,
//! widen-without-rollback, incomplete object, hidden line downgrade, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_AI_REVIEW_PUBLISH_CONTINUITY_AND_RECONCILE_REGISTRIES_PACKET_ID: &str =
    "m5-ai-review-publish-continuity-and-reconcile-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn publish_later_draft(
    input: M5PublishLaterDraftEntryResolutionInput,
) -> M5ResolvedPublishLaterDraftEntry {
    resolve_publish_later_draft_entry(input).expect("seed line-publish_later_draft entry resolves")
}

fn downgrade(
    input: M5CompareReconcileReviewEntryResolutionInput,
) -> M5ResolvedCompareReconcileReviewEntry {
    resolve_compare_reconcile_review_entry(input)
        .expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5PublishLaterDraftResolutionForm> {
    M5PublishLaterDraftResolutionForm::ALL.to_vec()
}

// -- Clean line-publish_later_draft entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_publish_later_draft_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5AiReviewAssistRole,
    report_section: M5PublishLaterDraftKind,
    surface_context: M5PublishLaterDraftSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5PublishLaterDraftEntryResolutionInput {
    M5PublishLaterDraftEntryResolutionInput {
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

fn publish_later_draft_provider_write_missing_clean() -> M5ResolvedPublishLaterDraftEntry {
    publish_later_draft(clean_publish_later_draft_base(
        "publish_later_draft:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.publish_later_draft.core_team_canary",
        M5AiReviewAssistRole::FindingClassification,
        M5PublishLaterDraftKind::ProviderWriteMissing,
        M5PublishLaterDraftSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn publish_later_draft_kept_local_draft_clean() -> M5ResolvedPublishLaterDraftEntry {
    publish_later_draft(clean_publish_later_draft_base(
        "publish_later_draft:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.publish_later_draft.kept_local_draft",
        M5AiReviewAssistRole::AnalyzedScopeDisclosure,
        M5PublishLaterDraftKind::KeptLocalDraft,
        M5PublishLaterDraftSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn publish_later_draft_exported_fallback_clean() -> M5ResolvedPublishLaterDraftEntry {
    publish_later_draft(clean_publish_later_draft_base(
        "publish_later_draft:program-governance:extension-author",
        "launch.line.extension-author",
        "line.publish_later_draft.exported_fallback",
        M5AiReviewAssistRole::PublishDestinationDisclosure,
        M5PublishLaterDraftKind::ExportedFallback,
        M5PublishLaterDraftSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn publish_later_draft_copied_forward_clean() -> M5ResolvedPublishLaterDraftEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_publish_later_draft_base(
        "publish_later_draft:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.publish_later_draft.copied_forward",
        M5AiReviewAssistRole::FindingClassification,
        M5PublishLaterDraftKind::CopiedForward,
        M5PublishLaterDraftSurfaceContext::ExecutiveSteeringSurface,
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
    publish_later_draft(base)
}

fn publish_later_draft_publish_later_queued_clean() -> M5ResolvedPublishLaterDraftEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_publish_later_draft_base(
        "publish_later_draft:support:public-preview",
        "launch.line.public-preview",
        "line.publish_later_draft.publish_later_queued",
        M5AiReviewAssistRole::AnalyzedScopeDisclosure,
        M5PublishLaterDraftKind::PublishLaterQueued,
        M5PublishLaterDraftSurfaceContext::SupportOrExportForm,
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
    publish_later_draft(base)
}

fn publish_later_draft_reconnect_repair_pending_clean() -> M5ResolvedPublishLaterDraftEntry {
    publish_later_draft(clean_publish_later_draft_base(
        "publish_later_draft:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.publish_later_draft.reconnect_repair_pending",
        M5AiReviewAssistRole::LocalVersusProviderState,
        M5PublishLaterDraftKind::ReconnectRepairPending,
        M5PublishLaterDraftSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded line-publish_later_draft entries ---------------------------------------------------------

/// Degraded publish_later_draft entry: the resolved publish_later_draft object is incomplete — the bundle IDs are unstated.
fn publish_later_draft_object_incomplete() -> M5ResolvedPublishLaterDraftEntry {
    let mut base = clean_publish_later_draft_base(
        "publish_later_draft:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.publish_later_draft.core_team_canary",
        M5AiReviewAssistRole::FindingClassification,
        M5PublishLaterDraftKind::ProviderWriteMissing,
        M5PublishLaterDraftSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    publish_later_draft(base)
}

/// Degraded publish_later_draft entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn publish_later_draft_widen_fold() -> M5ResolvedPublishLaterDraftEntry {
    let mut base = clean_publish_later_draft_base(
        "publish_later_draft:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.publish_later_draft.kept_local_draft",
        M5AiReviewAssistRole::AnalyzedScopeDisclosure,
        M5PublishLaterDraftKind::KeptLocalDraft,
        M5PublishLaterDraftSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    publish_later_draft(base)
}

/// Degraded publish_later_draft entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn publish_later_draft_unbound() -> M5ResolvedPublishLaterDraftEntry {
    let mut base = clean_publish_later_draft_base(
        "publish_later_draft:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.publish_later_draft.reconnect_repair_pending",
        M5AiReviewAssistRole::LocalVersusProviderState,
        M5PublishLaterDraftKind::ReconnectRepairPending,
        M5PublishLaterDraftSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    publish_later_draft(base)
}

/// Degraded publish_later_draft entry: the canonical registry token name is unstated.
fn publish_later_draft_token_unstated() -> M5ResolvedPublishLaterDraftEntry {
    let mut base = clean_publish_later_draft_base(
        "publish_later_draft:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5AiReviewAssistRole::PublishDestinationDisclosure,
        M5PublishLaterDraftKind::ExportedFallback,
        M5PublishLaterDraftSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    publish_later_draft(base)
}

// -- Clean line-downgrade-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_downgrade_base(
    entry_id: &str,
    comparison_ref: &str,
    token_name: &str,
    semantic_role: M5AiReviewAssistRole,
    comparison_scope: M5CompareReconcileReviewScope,
    surface_context: M5PublishLaterDraftSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5CompareReconcileReviewEntryResolutionInput {
    M5CompareReconcileReviewEntryResolutionInput {
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
        keeps_compare_reconcile_review_visible: true,
        comparison_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn downgrade_dogfood_ring_clean() -> M5ResolvedCompareReconcileReviewEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5AiReviewAssistRole::FindingClassification,
        M5CompareReconcileReviewScope::ReconciledPublishReady,
        M5PublishLaterDraftSurfaceContext::ShiproomSurface,
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

fn downgrade_rehearsal_currency_clean() -> M5ResolvedCompareReconcileReviewEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.exported_fallback",
        M5AiReviewAssistRole::PublishDestinationDisclosure,
        M5CompareReconcileReviewScope::TargetDiffDriftReconcile,
        M5PublishLaterDraftSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedCompareReconcileReviewEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.reconnect_repair_pending",
        M5AiReviewAssistRole::LocalVersusProviderState,
        M5CompareReconcileReviewScope::ProviderEditRaceReconcile,
        M5PublishLaterDraftSurfaceContext::ReleaseCenterSurface,
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
fn downgrade_support_ahead() -> M5ResolvedCompareReconcileReviewEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.publish_later_queued",
        M5AiReviewAssistRole::AnalyzedScopeDisclosure,
        M5CompareReconcileReviewScope::ReconciledPublishReady,
        M5PublishLaterDraftSurfaceContext::ShiproomSurface,
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
fn downgrade_form_incomplete() -> M5ResolvedCompareReconcileReviewEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.reconnect_repair_pending",
        M5AiReviewAssistRole::LocalVersusProviderState,
        M5CompareReconcileReviewScope::ProviderEditRaceReconcile,
        M5PublishLaterDraftSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5PublishLaterDraftResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn comparison_reconcile_scope_unclassified() -> M5ResolvedCompareReconcileReviewEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.copied_forward",
        M5AiReviewAssistRole::FindingClassification,
        M5CompareReconcileReviewScope::ReconcileScopeUnclassified,
        M5PublishLaterDraftSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5AiReviewPublishContinuityAndReconcileRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5AiReviewAssistDowngradeTrigger>,
    publish_later_draft_entries: Vec<M5ResolvedPublishLaterDraftEntry>,
    compare_reconcile_review_entries: Vec<M5ResolvedCompareReconcileReviewEntry>,
) -> M5AiReviewPublishContinuityAndReconcileRegistriesRow {
    M5AiReviewPublishContinuityAndReconcileRegistriesRow {
        consumer_surface,
        qualification: M5AiReviewAssistQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        capture_lifecycle_stages: M5AiReviewAssistClassificationStage::ALL.to_vec(),
        required_labels: M5AiReviewAssistRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5AiReviewAssistAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5PublishLaterDraftAnatomyPart::ALL.to_vec(),
        export_fields: M5PublishLaterDraftExportField::ALL.to_vec(),
        downgrade_triggers,
        publish_later_draft_entries,
        compare_reconcile_review_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_AI_REVIEW_PUBLISH_CONTINUITY_AND_RECONCILE_REGISTRIES_SCHEMA_REF,
            M5_AI_REVIEW_PUBLISH_SHEET_DOMAIN_SCHEMA_REF,
            M5_AI_REVIEW_SCOPE_SELECTOR_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_compare_reconcile_review_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5AiReviewPublishContinuityAndReconcileRegistriesRow> {
    use M5AiReviewAssistConsumerSurface as C;
    use M5AiReviewAssistDowngradeTrigger as D;

    vec![
        base_row(
            C::ReviewDetail,
            "Review-detail owner",
            "Review detail resolves an AI review finding to one durable publish-later draft — its publish-continuity state (provider-write-missing, kept-local-draft, exported-fallback, copied-forward, publish-later-queued, reconnect-repair-pending), the remote object identity it targets, the expected freshness floor, the target scope, the intended actor, and the conflict policy — from the shared registry and proves the compare-reconcile-review that keeps the path publish-ready; a draft missing its remote target identity and a reconcile that would silently overwrite a drifted diff degrade honestly instead of letting a local draft read as provider-committed",
            "retirement:m5-release-center:001",
            vec![
                D::PublishModeUnstated,
                D::PublishDestinationHidden,
                D::AiReviewAssistMatrixStale,
            ],
            vec![
                publish_later_draft_provider_write_missing_clean(),
                publish_later_draft_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::AiReviewPanel,
            "AI-review-panel owner",
            "The AI review panel resolves the keep-local, export, and copy-forward continuity states and the compare-reconcile-review (a provider-edit-race reconcile) while keeping the conflict policy and freshness floor visible; a draft that would wear a provider-committed badge without an accepted provider mutation and a resolution-form gap on a reconcile decision are caught before anything reads as committed",
            "retirement:m5-help-docs:001",
            vec![
                D::PublishModeUnstated,
                D::PublishExportFallbackMissing,
                D::AiReviewAssistMatrixStale,
            ],
            vec![publish_later_draft_kept_local_draft_clean(), publish_later_draft_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::SupportExportPacket,
            "Support owner",
            "Support / export resolves the same local-draft packet — remote object identity, expected freshness floor, target scope, intended actor, and conflict policy — keeping it reopenable after reconnect or auth repair, and reports the compare-reconcile-review outcome; a draft that is a hand-copied per-entry assumption and a decision on an unclassified reconcile scope degrade honestly",
            "retirement:m5-support:001",
            vec![
                D::AnalyzedScopeUnstated,
                D::LifecycleStateMissing,
                D::AiReviewAssistMatrixStale,
            ],
            vec![
                publish_later_draft_copied_forward_clean(),
                publish_later_draft_unbound(),
            ],
            vec![comparison_reconcile_scope_unclassified()],
        ),
        base_row(
            C::FindingRow,
            "Finding-row owner",
            "The finding row resolves the target-scope field and the target-diff-drift-reconcile decision bound to the registry so a materially drifted target diff forces compare / reconcile rather than a silent last-writer-wins publish; an unstated registry token on a publish-later draft is caught before its target identity can drift",
            "retirement:m5-marketplace-registry:001",
            vec![
                D::AnalyzedScopeUnstated,
                D::FindingClassBadgeMissing,
                D::AiReviewAssistMatrixStale,
            ],
            vec![
                publish_later_draft_exported_fallback_clean(),
                publish_later_draft_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::PendingReviewTray,
            "Pending-review-tray owner",
            "The pending-review tray renders the same resolved publish-later-draft and compare-reconcile-review truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied pending table; the local draft's remote object identity, freshness floor, and conflict policy stay inspectable off-renderer so a user can reopen a deferred draft safely after reconnect or auth repair without any provider-committed badge appearing",
            "retirement:m5-install-update:001",
            vec![
                D::FindingClassBadgeMissing,
                D::AnalyzedScopeUnstated,
                D::AiReviewAssistMatrixStale,
            ],
            vec![publish_later_draft_reconnect_repair_pending_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::ProviderPublishReview,
            "Provider-publish-review owner",
            "The provider publish-review feed carries the same resolved publish-later-draft and compare-reconcile-review truth, so a hand-copied constant, an unstated registry token, a local draft wearing a provider-committed badge without an accepted mutation, or a material drift committed as a silent last-writer-wins publish is visible in evidence — publish-ready, target-diff-drift reconcile, or provider-edit-race reconcile — rather than hidden behind a screenshot",
            "retirement:m5-partner-procurement:001",
            vec![
                D::AnalyzedScopeUnstated,
                D::PublishDestinationHidden,
                D::AiReviewAssistMatrixStale,
            ],
            vec![publish_later_draft_publish_later_queued_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5AiReviewPublishContinuityAndReconcileRegistriesGovernanceReview {
    M5AiReviewPublishContinuityAndReconcileRegistriesGovernanceReview {
        publish_later_draft_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_publish_later_draft_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        compare_reconcile_review_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        publish_later_draft_or_downgrade_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5AiReviewPublishContinuityAndReconcileRegistriesConsumerProjection {
    M5AiReviewPublishContinuityAndReconcileRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5AiReviewPublishContinuityAndReconcileRegistriesProofFreshness {
    M5AiReviewPublishContinuityAndReconcileRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5AiReviewPublishContinuityAndReconcileRegistriesReleasePosture {
    M5AiReviewPublishContinuityAndReconcileRegistriesReleasePosture {
        proof_packet_ref: M5_AI_REVIEW_PUBLISH_CONTINUITY_AND_RECONCILE_REGISTRIES_ARTIFACT_REF
            .to_owned(),
        line_audit_ref: M5_AI_REVIEW_PUBLISH_CONTINUITY_AND_RECONCILE_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_AI_REVIEW_PUBLISH_CONTINUITY_AND_RECONCILE_REGISTRIES_SCHEMA_REF,
        M5_AI_REVIEW_PUBLISH_CONTINUITY_AND_RECONCILE_REGISTRIES_DOC_REF,
        M5_AI_REVIEW_ASSIST_MATRIX_SCHEMA_REF,
        M5_AI_REVIEW_ASSIST_MATRIX_DOC_REF,
        M5_AI_REVIEW_PUBLISH_SHEET_DOMAIN_SCHEMA_REF,
        M5_AI_REVIEW_SCOPE_SELECTOR_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 post-launch public-proof-ledger and claim-history-diff registries packet.
pub fn seeded_m5_ai_review_publish_continuity_and_reconcile_registries(
) -> M5AiReviewPublishContinuityAndReconcileRegistriesPacket {
    M5AiReviewPublishContinuityAndReconcileRegistriesPacket::new(
        M5AiReviewPublishContinuityAndReconcileRegistriesPacketInput {
            packet_id: M5_AI_REVIEW_PUBLISH_CONTINUITY_AND_RECONCILE_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 publish-later-draft and compare-reconcile-review registries recording one durable local draft per AI review finding that targets a provider object — one typed field per section: its publish-continuity state (provider-write-missing, kept-local-draft, exported-fallback, copied-forward, publish-later-queued, reconnect-repair-pending), the remote object identity it targets, the expected freshness floor, the target scope, the intended actor, and the conflict policy — each bound to one object-class identity, so a finding kept local, exported, or copied forward never wears a provider-committed badge, with canonical / accessible / audit resolution-form coverage, and a machine-readable compare-reconcile-review (reconciled-publish-ready, target-diff-drift-reconcile, or provider-edit-race-reconcile) that forces compare / reconcile when provider-side edits race the local draft or the target diff drifted materially instead of a silent last-writer-wins publish, and preserves the same local-draft packet — remote object identity, freshness floor, target scope, intended actor, and conflict policy — in local history and support / export so a draft can be reopened safely after reconnect or auth repair, across review, AI, provider, and support / export surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5AiReviewPublishContinuityAndReconcileRegistriesVocabularySet::canonical(),
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
pub fn seeded_m5_ai_review_publish_continuity_and_reconcile_registries_publish_later_draft_beta_narrowed(
) -> M5AiReviewPublishContinuityAndReconcileRegistriesPacket {
    let mut packet = seeded_m5_ai_review_publish_continuity_and_reconcile_registries();
    packet.packet_id =
        "m5-ai-review-publish-continuity-and-reconcile-registries:snapshot-descriptor-beta:0001"
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
pub fn seeded_m5_ai_review_publish_continuity_and_reconcile_registries_compare_reconcile_review_preview_narrowed(
) -> M5AiReviewPublishContinuityAndReconcileRegistriesPacket {
    let mut packet = seeded_m5_ai_review_publish_continuity_and_reconcile_registries();
    packet.packet_id =
        "m5-ai-review-publish-continuity-and-reconcile-registries:compare-reconcile-review-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5AiReviewAssistConsumerSurface::AiReviewPanel)
        .expect("release-center row present");
    row.qualification = M5AiReviewAssistQualificationClass::Preview;
    packet
}
