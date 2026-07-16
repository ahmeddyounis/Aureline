//! Canonical seed builders for the M5 line-review_scope_selector and line-downgrade-packet registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean line-review_scope_selector and line-downgrade-packet entries
//! are built so the one typed line-review_scope_selector object resolving per line, lines never widening without
//! preserving rollback and diagnostics, partner / public support language never running ahead of line proof,
//! the canonical / accessible / audit resolution forms, and the complete line-identity / known-limits-ledger /
//! rollback-target / rehearsal-currency / readiness-signoff / support-language / last-widening-revision
//! line-downgrade object are proven across the shiproom, release-center, executive-steering,
//! program-governance, diagnostics, and support surfaces without any hand-copied per-line assumption,
//! widen-without-rollback, incomplete object, hidden line downgrade, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_AI_REVIEW_SCOPE_SELECTOR_AND_RERUN_STATE_REGISTRIES_PACKET_ID: &str =
    "m5-ai-review-scope-selector-and-rerun-state-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn review_scope_selector(
    input: M5ReviewScopeSelectorEntryResolutionInput,
) -> M5ResolvedReviewScopeSelectorEntry {
    resolve_review_scope_selector_entry(input)
        .expect("seed line-review_scope_selector entry resolves")
}

fn downgrade(input: M5RerunFreshnessDiffEntryResolutionInput) -> M5ResolvedRerunFreshnessDiffEntry {
    resolve_rerun_freshness_diff_entry(input).expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5ReviewScopeSelectorResolutionForm> {
    M5ReviewScopeSelectorResolutionForm::ALL.to_vec()
}

// -- Clean line-review_scope_selector entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_review_scope_selector_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5AiReviewAssistRole,
    report_section: M5ReviewScopeSelectorKind,
    surface_context: M5ReviewScopeSelectorSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5ReviewScopeSelectorEntryResolutionInput {
    M5ReviewScopeSelectorEntryResolutionInput {
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

fn review_scope_selector_selected_diff_clean() -> M5ResolvedReviewScopeSelectorEntry {
    review_scope_selector(clean_review_scope_selector_base(
        "review_scope_selector:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.review_scope_selector.core_team_canary",
        M5AiReviewAssistRole::FindingClassification,
        M5ReviewScopeSelectorKind::SelectedDiff,
        M5ReviewScopeSelectorSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn review_scope_selector_uncommitted_changes_clean() -> M5ResolvedReviewScopeSelectorEntry {
    review_scope_selector(clean_review_scope_selector_base(
        "review_scope_selector:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.review_scope_selector.uncommitted_changes",
        M5AiReviewAssistRole::AnalyzedScopeDisclosure,
        M5ReviewScopeSelectorKind::UncommittedChanges,
        M5ReviewScopeSelectorSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn review_scope_selector_pull_merge_request_clean() -> M5ResolvedReviewScopeSelectorEntry {
    review_scope_selector(clean_review_scope_selector_base(
        "review_scope_selector:program-governance:extension-author",
        "launch.line.extension-author",
        "line.review_scope_selector.pull_merge_request",
        M5AiReviewAssistRole::PublishDestinationDisclosure,
        M5ReviewScopeSelectorKind::PullMergeRequest,
        M5ReviewScopeSelectorSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn review_scope_selector_base_head_range_clean() -> M5ResolvedReviewScopeSelectorEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_review_scope_selector_base(
        "review_scope_selector:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.review_scope_selector.base_head_range",
        M5AiReviewAssistRole::FindingClassification,
        M5ReviewScopeSelectorKind::BaseHeadRange,
        M5ReviewScopeSelectorSurfaceContext::ExecutiveSteeringSurface,
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
    review_scope_selector(base)
}

fn review_scope_selector_staged_changes_clean() -> M5ResolvedReviewScopeSelectorEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_review_scope_selector_base(
        "review_scope_selector:support:public-preview",
        "launch.line.public-preview",
        "line.review_scope_selector.staged_changes",
        M5AiReviewAssistRole::AnalyzedScopeDisclosure,
        M5ReviewScopeSelectorKind::StagedChanges,
        M5ReviewScopeSelectorSurfaceContext::SupportOrExportForm,
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
    review_scope_selector(base)
}

fn review_scope_selector_saved_review_snapshot_clean() -> M5ResolvedReviewScopeSelectorEntry {
    review_scope_selector(clean_review_scope_selector_base(
        "review_scope_selector:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.review_scope_selector.saved_review_snapshot",
        M5AiReviewAssistRole::LocalVersusProviderState,
        M5ReviewScopeSelectorKind::SavedReviewSnapshot,
        M5ReviewScopeSelectorSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded line-review_scope_selector entries ---------------------------------------------------------

/// Degraded review_scope_selector entry: the resolved review_scope_selector object is incomplete — the bundle IDs are unstated.
fn review_scope_selector_object_incomplete() -> M5ResolvedReviewScopeSelectorEntry {
    let mut base = clean_review_scope_selector_base(
        "review_scope_selector:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.review_scope_selector.core_team_canary",
        M5AiReviewAssistRole::FindingClassification,
        M5ReviewScopeSelectorKind::SelectedDiff,
        M5ReviewScopeSelectorSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    review_scope_selector(base)
}

/// Degraded review_scope_selector entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn review_scope_selector_widen_fold() -> M5ResolvedReviewScopeSelectorEntry {
    let mut base = clean_review_scope_selector_base(
        "review_scope_selector:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.review_scope_selector.uncommitted_changes",
        M5AiReviewAssistRole::AnalyzedScopeDisclosure,
        M5ReviewScopeSelectorKind::UncommittedChanges,
        M5ReviewScopeSelectorSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    review_scope_selector(base)
}

/// Degraded review_scope_selector entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn review_scope_selector_unbound() -> M5ResolvedReviewScopeSelectorEntry {
    let mut base = clean_review_scope_selector_base(
        "review_scope_selector:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.review_scope_selector.saved_review_snapshot",
        M5AiReviewAssistRole::LocalVersusProviderState,
        M5ReviewScopeSelectorKind::SavedReviewSnapshot,
        M5ReviewScopeSelectorSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    review_scope_selector(base)
}

/// Degraded review_scope_selector entry: the canonical registry token name is unstated.
fn review_scope_selector_token_unstated() -> M5ResolvedReviewScopeSelectorEntry {
    let mut base = clean_review_scope_selector_base(
        "review_scope_selector:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5AiReviewAssistRole::PublishDestinationDisclosure,
        M5ReviewScopeSelectorKind::PullMergeRequest,
        M5ReviewScopeSelectorSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    review_scope_selector(base)
}

// -- Clean line-downgrade-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_downgrade_base(
    entry_id: &str,
    comparison_ref: &str,
    token_name: &str,
    semantic_role: M5AiReviewAssistRole,
    comparison_scope: M5RerunFreshnessDiffScope,
    surface_context: M5ReviewScopeSelectorSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5RerunFreshnessDiffEntryResolutionInput {
    M5RerunFreshnessDiffEntryResolutionInput {
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
        keeps_rerun_freshness_diff_visible: true,
        comparison_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn downgrade_dogfood_ring_clean() -> M5ResolvedRerunFreshnessDiffEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5AiReviewAssistRole::FindingClassification,
        M5RerunFreshnessDiffScope::AnalyzedDiffChanged,
        M5ReviewScopeSelectorSurfaceContext::ShiproomSurface,
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

fn downgrade_rehearsal_currency_clean() -> M5ResolvedRerunFreshnessDiffEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.pull_merge_request",
        M5AiReviewAssistRole::PublishDestinationDisclosure,
        M5RerunFreshnessDiffScope::BaseHeadContextShifted,
        M5ReviewScopeSelectorSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedRerunFreshnessDiffEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.saved_review_snapshot",
        M5AiReviewAssistRole::LocalVersusProviderState,
        M5RerunFreshnessDiffScope::SavedSnapshotMismatch,
        M5ReviewScopeSelectorSurfaceContext::ReleaseCenterSurface,
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
fn downgrade_support_ahead() -> M5ResolvedRerunFreshnessDiffEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.staged_changes",
        M5AiReviewAssistRole::AnalyzedScopeDisclosure,
        M5RerunFreshnessDiffScope::AnalyzedDiffChanged,
        M5ReviewScopeSelectorSurfaceContext::ShiproomSurface,
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
fn downgrade_form_incomplete() -> M5ResolvedRerunFreshnessDiffEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.saved_review_snapshot",
        M5AiReviewAssistRole::LocalVersusProviderState,
        M5RerunFreshnessDiffScope::SavedSnapshotMismatch,
        M5ReviewScopeSelectorSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5ReviewScopeSelectorResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn comparison_rerun_freshness_unclassified() -> M5ResolvedRerunFreshnessDiffEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.base_head_range",
        M5AiReviewAssistRole::FindingClassification,
        M5RerunFreshnessDiffScope::RerunFreshnessUnclassified,
        M5ReviewScopeSelectorSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5AiReviewScopeSelectorAndRerunStateRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5AiReviewAssistDowngradeTrigger>,
    review_scope_selector_entries: Vec<M5ResolvedReviewScopeSelectorEntry>,
    rerun_freshness_diff_entries: Vec<M5ResolvedRerunFreshnessDiffEntry>,
) -> M5AiReviewScopeSelectorAndRerunStateRegistriesRow {
    M5AiReviewScopeSelectorAndRerunStateRegistriesRow {
        consumer_surface,
        qualification: M5AiReviewAssistQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        capture_lifecycle_stages: M5AiReviewAssistClassificationStage::ALL.to_vec(),
        required_labels: M5AiReviewAssistRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5AiReviewAssistAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5ReviewScopeSelectorAnatomyPart::ALL.to_vec(),
        export_fields: M5ReviewScopeSelectorExportField::ALL.to_vec(),
        downgrade_triggers,
        review_scope_selector_entries,
        rerun_freshness_diff_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_AI_REVIEW_SCOPE_SELECTOR_AND_RERUN_STATE_REGISTRIES_SCHEMA_REF,
            M5_AI_REVIEW_SCOPE_SELECTOR_DOMAIN_SCHEMA_REF,
            M5_AI_REVIEW_RESOLUTION_MEMORY_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_rerun_freshness_diff_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5AiReviewScopeSelectorAndRerunStateRegistriesRow> {
    use M5AiReviewAssistConsumerSurface as C;
    use M5AiReviewAssistDowngradeTrigger as D;

    vec![
        base_row(
            C::ReviewDetail,
            "Review-detail owner",
            "Review detail resolves an AI review run to one typed review-scope-selector state — the analyzed scope (selected diff), base / head context, repo-instruction / check-pack source, freshness, and in-scope rerun action — from the shared registry and proves the analyzed-diff-changed freshness diff for that finding; a scope state missing its base / head joins and a freshness diff that keeps a stale finding looking current degrade honestly instead of leaving a finding to read as fresh",
            "retirement:m5-release-center:001",
            vec![
                D::PublishModeUnstated,
                D::PublishDestinationHidden,
                D::AiReviewAssistMatrixStale,
            ],
            vec![
                review_scope_selector_selected_diff_clean(),
                review_scope_selector_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::AiReviewPanel,
            "AI-review-panel owner",
            "The AI review panel resolves the in-scope rerun action and the base-head-context-shifted freshness diff while keeping the active drift reason visible; a rerun widening its scope without preserved prior lineage and a resolution-form gap on a freshness diff are caught before a rerun can reintroduce a falsely-fresh finding",
            "retirement:m5-help-docs:001",
            vec![
                D::PublishModeUnstated,
                D::PublishExportFallbackMissing,
                D::AiReviewAssistMatrixStale,
            ],
            vec![review_scope_selector_uncommitted_changes_clean(), review_scope_selector_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::SupportExportPacket,
            "Support owner",
            "Support / export resolves the repo-instruction / check-pack source field while keeping its scope claim matched to the analyzed diff and reports the rerun-freshness-diff outcome; a scope entry that is a hand-copied per-entry assumption and a diff on an unclassified freshness drift degrade honestly",
            "retirement:m5-support:001",
            vec![
                D::AnalyzedScopeUnstated,
                D::LifecycleStateMissing,
                D::AiReviewAssistMatrixStale,
            ],
            vec![
                review_scope_selector_base_head_range_clean(),
                review_scope_selector_unbound(),
            ],
            vec![comparison_rerun_freshness_unclassified()],
        ),
        base_row(
            C::FindingRow,
            "Finding-row owner",
            "The finding row resolves the analyzed-scope field and the saved-snapshot-mismatch freshness diff bound to the registry so a prior finding can no longer read as current once its saved review snapshot no longer matches the target; an unstated registry token on a scope entry is caught before it can drift",
            "retirement:m5-marketplace-registry:001",
            vec![
                D::AnalyzedScopeUnstated,
                D::FindingClassBadgeMissing,
                D::AiReviewAssistMatrixStale,
            ],
            vec![
                review_scope_selector_pull_merge_request_clean(),
                review_scope_selector_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::ReviewScopeSelector,
            "Review-scope-selector owner",
            "The review scope selector renders the same resolved review-scope-selector and rerun-freshness-diff truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied scope table; the in-scope rerun action and the base-head-context-shifted freshness diff stay inspectable off-renderer so a rerun always re-resolves current scope before new output is shown",
            "retirement:m5-install-update:001",
            vec![
                D::FindingClassBadgeMissing,
                D::AnalyzedScopeUnstated,
                D::AiReviewAssistMatrixStale,
            ],
            vec![review_scope_selector_saved_review_snapshot_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::ProviderPublishReview,
            "Provider-publish-review owner",
            "The provider publish-review feed carries the same resolved review-scope-selector and rerun-freshness-diff truth, so a hand-copied constant, an unstated registry token, a scope state widening beyond its selected diff without preserved lineage, or a stale finding shown as current is visible in evidence — an analyzed-diff change, a base / head-context shift, or a saved-snapshot mismatch — rather than hidden behind a screenshot",
            "retirement:m5-partner-procurement:001",
            vec![
                D::AnalyzedScopeUnstated,
                D::PublishDestinationHidden,
                D::AiReviewAssistMatrixStale,
            ],
            vec![review_scope_selector_staged_changes_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5AiReviewScopeSelectorAndRerunStateRegistriesGovernanceReview {
    M5AiReviewScopeSelectorAndRerunStateRegistriesGovernanceReview {
        review_scope_selector_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_review_scope_selector_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        rerun_freshness_diff_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        review_scope_selector_or_downgrade_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5AiReviewScopeSelectorAndRerunStateRegistriesConsumerProjection {
    M5AiReviewScopeSelectorAndRerunStateRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5AiReviewScopeSelectorAndRerunStateRegistriesProofFreshness {
    M5AiReviewScopeSelectorAndRerunStateRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5AiReviewScopeSelectorAndRerunStateRegistriesReleasePosture {
    M5AiReviewScopeSelectorAndRerunStateRegistriesReleasePosture {
        proof_packet_ref: M5_AI_REVIEW_SCOPE_SELECTOR_AND_RERUN_STATE_REGISTRIES_ARTIFACT_REF
            .to_owned(),
        line_audit_ref: M5_AI_REVIEW_SCOPE_SELECTOR_AND_RERUN_STATE_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_AI_REVIEW_SCOPE_SELECTOR_AND_RERUN_STATE_REGISTRIES_SCHEMA_REF,
        M5_AI_REVIEW_SCOPE_SELECTOR_AND_RERUN_STATE_REGISTRIES_DOC_REF,
        M5_AI_REVIEW_ASSIST_MATRIX_SCHEMA_REF,
        M5_AI_REVIEW_ASSIST_MATRIX_DOC_REF,
        M5_AI_REVIEW_SCOPE_SELECTOR_DOMAIN_SCHEMA_REF,
        M5_AI_REVIEW_RESOLUTION_MEMORY_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 post-launch public-proof-ledger and claim-history-diff registries packet.
pub fn seeded_m5_ai_review_scope_selector_and_rerun_state_registries(
) -> M5AiReviewScopeSelectorAndRerunStateRegistriesPacket {
    M5AiReviewScopeSelectorAndRerunStateRegistriesPacket::new(
        M5AiReviewScopeSelectorAndRerunStateRegistriesPacketInput {
            packet_id: M5_AI_REVIEW_SCOPE_SELECTOR_AND_RERUN_STATE_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 review-scope-selector-state and rerun/outdated-freshness registries emitting one machine-readable scope-selector state per AI review run — one typed field per section: the analyzed review scope (selected diff, uncommitted changes, pull / merge request, base..head range, staged changes, saved review snapshot), the base / head context, the repo-instruction / enabled-check-pack source, the freshness and in-scope rerun action, and the retained-versus-re-resolved lineage — each bound to one object-class identity, so a finding never hides whether it came from selected lines, local uncommitted changes, or a hosted review object, with canonical / accessible / audit resolution-form coverage, and a machine-readable rerun-freshness diff (analyzed-diff-changed, base-head-context-shifted, or saved-snapshot-mismatch) that turns a changed diff scope or shifted base / head into a visible, typed freshness event marking the prior finding outdated / rerun-recommended rather than a silent mutation across review, AI, provider, and support / export surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5AiReviewScopeSelectorAndRerunStateRegistriesVocabularySet::canonical(),
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
pub fn seeded_m5_ai_review_scope_selector_and_rerun_state_registries_review_scope_selector_beta_narrowed(
) -> M5AiReviewScopeSelectorAndRerunStateRegistriesPacket {
    let mut packet = seeded_m5_ai_review_scope_selector_and_rerun_state_registries();
    packet.packet_id =
        "m5-ai-review-scope-selector-and-rerun-state-registries:snapshot-descriptor-beta:0001"
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
pub fn seeded_m5_ai_review_scope_selector_and_rerun_state_registries_rerun_freshness_diff_preview_narrowed(
) -> M5AiReviewScopeSelectorAndRerunStateRegistriesPacket {
    let mut packet = seeded_m5_ai_review_scope_selector_and_rerun_state_registries();
    packet.packet_id =
        "m5-ai-review-scope-selector-and-rerun-state-registries:rerun-freshness-diff-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5AiReviewAssistConsumerSurface::AiReviewPanel)
        .expect("release-center row present");
    row.qualification = M5AiReviewAssistQualificationClass::Preview;
    packet
}
