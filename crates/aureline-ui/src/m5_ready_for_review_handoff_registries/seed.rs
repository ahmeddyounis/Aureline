//! Canonical seed builders for the M5 ready-for-review-handoff-sheet and publish-action registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean ready-for-review-handoff and publish-action entries
//! are built so the one typed ready-for-review handoff sheet resolving per tracked item, sheets never letting a
//! local-only draft or queued publish read as a provider-committed update, offline / policy-blocked / partially-writable
//! publish-actions never hidden behind a green summary, the canonical / accessible / audit resolution forms, and the
//! complete changed-scope / checks-test-state / linked-review / comment-draft / attached-evidence / provider-mutation-list
//! publish-action object are proven across the work-item-detail, review-detail, Git / worktree, ready-for-review-handoff,
//! provider-handoff, and support surfaces without any hand-copied per-item assumption, local-shown-as-provider update,
//! incomplete object, hidden offline / policy-blocked state, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_READY_FOR_REVIEW_HANDOFF_REGISTRIES_PACKET_ID: &str =
    "m5-ready-for-review-handoff-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn review_pack_record(
    input: M5ReadyForReviewHandoffEntryResolutionInput,
) -> M5ResolvedReadyForReviewHandoffEntry {
    resolve_review_pack_record_entry(input).expect("seed line-review_pack_record entry resolves")
}

fn downgrade(
    input: M5ReadyForReviewRelationEntryResolutionInput,
) -> M5ResolvedReadyForReviewRelationEntry {
    resolve_review_pack_result_entry(input).expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5ReadyForReviewHandoffResolutionForm> {
    M5ReadyForReviewHandoffResolutionForm::ALL.to_vec()
}

// -- Clean line-review_pack_record entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_review_pack_record_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5ChangeIntentRole,
    report_section: M5ReadyForReviewHandoffKind,
    surface_context: M5ReadyForReviewHandoffSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5ReadyForReviewHandoffEntryResolutionInput {
    M5ReadyForReviewHandoffEntryResolutionInput {
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

fn review_pack_record_changed_files_scope_clean() -> M5ResolvedReadyForReviewHandoffEntry {
    review_pack_record(clean_review_pack_record_base(
        "review_pack_record:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.review_pack_record.core_team_canary",
        M5ChangeIntentRole::ProviderOwnershipDisclosure,
        M5ReadyForReviewHandoffKind::ChangedFilesScope,
        M5ReadyForReviewHandoffSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn review_pack_record_pull_request_scope_clean() -> M5ResolvedReadyForReviewHandoffEntry {
    review_pack_record(clean_review_pack_record_base(
        "review_pack_record:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.review_pack_record.pull_request_scope",
        M5ChangeIntentRole::LocalVersusProviderStateDisclosure,
        M5ReadyForReviewHandoffKind::PullRequestScope,
        M5ReadyForReviewHandoffSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn review_pack_record_base_head_range_scope_clean() -> M5ResolvedReadyForReviewHandoffEntry {
    review_pack_record(clean_review_pack_record_base(
        "review_pack_record:program-governance:extension-author",
        "launch.line.extension-author",
        "line.review_pack_record.base_head_range_scope",
        M5ChangeIntentRole::LinkedEngineeringIdentityDisclosure,
        M5ReadyForReviewHandoffKind::BaseHeadRangeScope,
        M5ReadyForReviewHandoffSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn review_pack_record_worktree_uncommitted_scope_clean() -> M5ResolvedReadyForReviewHandoffEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_review_pack_record_base(
        "review_pack_record:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.review_pack_record.worktree_uncommitted_scope",
        M5ChangeIntentRole::ProviderOwnershipDisclosure,
        M5ReadyForReviewHandoffKind::WorktreeUncommittedScope,
        M5ReadyForReviewHandoffSurfaceContext::ExecutiveSteeringSurface,
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
    review_pack_record(base)
}

fn review_pack_record_full_tree_scope_clean() -> M5ResolvedReadyForReviewHandoffEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_review_pack_record_base(
        "review_pack_record:support:public-preview",
        "launch.line.public-preview",
        "line.review_pack_record.full_tree_scope",
        M5ChangeIntentRole::LocalVersusProviderStateDisclosure,
        M5ReadyForReviewHandoffKind::FullTreeScope,
        M5ReadyForReviewHandoffSurfaceContext::SupportOrExportForm,
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
    review_pack_record(base)
}

fn review_pack_record_saved_pack_snapshot_scope_clean() -> M5ResolvedReadyForReviewHandoffEntry {
    review_pack_record(clean_review_pack_record_base(
        "review_pack_record:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.review_pack_record.saved_pack_snapshot_scope",
        M5ChangeIntentRole::SideEffectDisclosure,
        M5ReadyForReviewHandoffKind::SavedPackSnapshotScope,
        M5ReadyForReviewHandoffSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded line-review_pack_record entries ---------------------------------------------------------

/// Degraded review_pack_record entry: the resolved review_pack_record object is incomplete — the bundle IDs are unstated.
fn review_pack_record_object_incomplete() -> M5ResolvedReadyForReviewHandoffEntry {
    let mut base = clean_review_pack_record_base(
        "review_pack_record:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.review_pack_record.core_team_canary",
        M5ChangeIntentRole::ProviderOwnershipDisclosure,
        M5ReadyForReviewHandoffKind::ChangedFilesScope,
        M5ReadyForReviewHandoffSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    review_pack_record(base)
}

/// Degraded review_pack_record entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn review_pack_record_widen_fold() -> M5ResolvedReadyForReviewHandoffEntry {
    let mut base = clean_review_pack_record_base(
        "review_pack_record:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.review_pack_record.pull_request_scope",
        M5ChangeIntentRole::LocalVersusProviderStateDisclosure,
        M5ReadyForReviewHandoffKind::PullRequestScope,
        M5ReadyForReviewHandoffSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    review_pack_record(base)
}

/// Degraded review_pack_record entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn review_pack_record_unbound() -> M5ResolvedReadyForReviewHandoffEntry {
    let mut base = clean_review_pack_record_base(
        "review_pack_record:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.review_pack_record.saved_pack_snapshot_scope",
        M5ChangeIntentRole::SideEffectDisclosure,
        M5ReadyForReviewHandoffKind::SavedPackSnapshotScope,
        M5ReadyForReviewHandoffSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    review_pack_record(base)
}

/// Degraded review_pack_record entry: the canonical registry token name is unstated.
fn review_pack_record_token_unstated() -> M5ResolvedReadyForReviewHandoffEntry {
    let mut base = clean_review_pack_record_base(
        "review_pack_record:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5ChangeIntentRole::LinkedEngineeringIdentityDisclosure,
        M5ReadyForReviewHandoffKind::BaseHeadRangeScope,
        M5ReadyForReviewHandoffSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    review_pack_record(base)
}

// -- Clean line-downgrade-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_downgrade_base(
    entry_id: &str,
    comparison_ref: &str,
    token_name: &str,
    semantic_role: M5ChangeIntentRole,
    comparison_scope: M5ReadyForReviewRelationScope,
    surface_context: M5ReadyForReviewHandoffSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5ReadyForReviewRelationEntryResolutionInput {
    M5ReadyForReviewRelationEntryResolutionInput {
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
        keeps_review_pack_result_visible: true,
        comparison_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn downgrade_dogfood_ring_clean() -> M5ResolvedReadyForReviewRelationEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5ChangeIntentRole::ProviderOwnershipDisclosure,
        M5ReadyForReviewRelationScope::EvaluatedScopeBinding,
        M5ReadyForReviewHandoffSurfaceContext::ShiproomSurface,
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

fn downgrade_rehearsal_currency_clean() -> M5ResolvedReadyForReviewRelationEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.base_head_range_scope",
        M5ChangeIntentRole::LinkedEngineeringIdentityDisclosure,
        M5ReadyForReviewRelationScope::PackVersionDigestBinding,
        M5ReadyForReviewHandoffSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedReadyForReviewRelationEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.saved_pack_snapshot_scope",
        M5ChangeIntentRole::SideEffectDisclosure,
        M5ReadyForReviewRelationScope::DivergenceLabelBinding,
        M5ReadyForReviewHandoffSurfaceContext::ReleaseCenterSurface,
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
fn downgrade_support_ahead() -> M5ResolvedReadyForReviewRelationEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.full_tree_scope",
        M5ChangeIntentRole::LocalVersusProviderStateDisclosure,
        M5ReadyForReviewRelationScope::EvaluatedScopeBinding,
        M5ReadyForReviewHandoffSurfaceContext::ShiproomSurface,
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
fn downgrade_form_incomplete() -> M5ResolvedReadyForReviewRelationEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.saved_pack_snapshot_scope",
        M5ChangeIntentRole::SideEffectDisclosure,
        M5ReadyForReviewRelationScope::DivergenceLabelBinding,
        M5ReadyForReviewHandoffSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5ReadyForReviewHandoffResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn comparison_review_pack_result_unclassified() -> M5ResolvedReadyForReviewRelationEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.worktree_uncommitted_scope",
        M5ChangeIntentRole::ProviderOwnershipDisclosure,
        M5ReadyForReviewRelationScope::ChangeIntentResultUnclassified,
        M5ReadyForReviewHandoffSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5ReadyForReviewHandoffRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5ChangeIntentDowngradeTrigger>,
    review_pack_record_entries: Vec<M5ResolvedReadyForReviewHandoffEntry>,
    review_pack_result_entries: Vec<M5ResolvedReadyForReviewRelationEntry>,
) -> M5ReadyForReviewHandoffRegistriesRow {
    M5ReadyForReviewHandoffRegistriesRow {
        consumer_surface,
        qualification: M5ChangeIntentQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        capture_lifecycle_stages: M5ChangeIntentClassificationStage::ALL.to_vec(),
        required_labels: M5ChangeIntentRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5ChangeIntentAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5ReadyForReviewHandoffAnatomyPart::ALL.to_vec(),
        export_fields: M5ReadyForReviewHandoffExportField::ALL.to_vec(),
        downgrade_triggers,
        review_pack_record_entries,
        review_pack_result_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_READY_FOR_REVIEW_HANDOFF_REGISTRIES_SCHEMA_REF,
            M5_CHANGE_INTENT_DOMAIN_SCHEMA_REF,
            M5_READY_FOR_REVIEW_HANDOFF_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_review_pack_result_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5ReadyForReviewHandoffRegistriesRow> {
    use M5ChangeIntentConsumerSurface as C;
    use M5ChangeIntentDowngradeTrigger as D;

    vec![
        base_row(
            C::WorkItemDetail,
            "Work-item-detail owner",
            "Work-item detail resolves a tracked work item to one typed ready-for-review handoff sheet — its changed scope, checks / test state, linked review object, comment draft, attached evidence, provider mutation list, and export-versus-publish actions, summary-first — from the shared registry and proves the commit-state disclosure for that item; a sheet dropping its changed scope or linked review and a publish-action that would let a local-only draft or queued publish read as a provider-committed update degrade honestly instead of implying provider acceptance the target has not given",
            "ready-for-review:m5-work-item-detail:001",
            vec![
                D::LinkedEngineeringIdentityUnstated,
                D::LocalHandoffShownAsProviderCommitted,
                D::ChangeIntentMatrixStale,
            ],
            vec![
                review_pack_record_changed_files_scope_clean(),
                review_pack_record_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::ReviewDetail,
            "Review-detail owner",
            "Review detail resolves the same ready-for-review handoff sheet from the tracked item and shows the checks / test state, the linked review object, and the attached evidence bound to their commit state; a sheet letting a queued or local publish read as provider-accepted and a summary-first-ordering gap are caught before a green summary can hide them, so review detail renders the same handoff truth as work-item detail without contradiction",
            "ready-for-review:m5-review-detail:001",
            vec![
                D::LinkedEngineeringIdentityUnstated,
                D::ValidationEvidenceUnstated,
                D::ChangeIntentMatrixStale,
            ],
            vec![review_pack_record_pull_request_scope_clean(), review_pack_record_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::SupportExportPacket,
            "Support owner",
            "Support resolves the handoff sheet's commit state while keeping the changed scope / linked review object / attached evidence and the queue-for-publish / export-local-packet attribution bound to the export, and reports the commit-state disclosure; a sheet that is a hand-copied per-item assumption and a publish-action on an unclassified binding degrade honestly so the changed scope, linked review, and attached evidence are never dropped on export or reopen",
            "ready-for-review:m5-support:001",
            vec![
                D::ProviderOwnershipUnstated,
                D::BlockerStateUnstated,
                D::ChangeIntentMatrixStale,
            ],
            vec![
                review_pack_record_worktree_uncommitted_scope_clean(),
                review_pack_record_unbound(),
            ],
            vec![comparison_review_pack_result_unclassified()],
        ),
        base_row(
            C::LinkedChangePanel,
            "Linked-change-panel owner",
            "The linked-change panel surface renders the same handoff sheet's linked review object and provider mutation list bound to their commit state — publish now, queue for publish, or export a local packet — from the registry so the publish modes can no longer be flattened into one generic action, and a target that is offline, policy-blocked, or only partially writable stays visible and actionable instead of implying provider acceptance; an unstated commit state on a sheet is caught before it can drift",
            "ready-for-review:m5-linked-change-panel:001",
            vec![
                D::ProviderOwnershipUnstated,
                D::LocalVersusProviderStateUnstated,
                D::ChangeIntentMatrixStale,
            ],
            vec![
                review_pack_record_base_head_range_scope_clean(),
                review_pack_record_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::ReadyForReviewHandoff,
            "Ready-for-review-handoff owner",
            "The ready-for-review handoff sheet renders the same resolved sheet and publish-action truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied sheet, letting users compare publish-now, queue-for-publish, and export-local-packet outcomes from one sheet; the publish-action state and the checks / test state stay inspectable off-renderer so a local-only draft or queued publish never reads as a provider-committed update",
            "ready-for-review:m5-ready-for-review-handoff:001",
            vec![
                D::LocalVersusProviderStateUnstated,
                D::ProviderOwnershipUnstated,
                D::ChangeIntentMatrixStale,
            ],
            vec![review_pack_record_saved_pack_snapshot_scope_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::HelpDocs,
            "Help-docs owner",
            "The help / docs feed carries the same resolved handoff sheet and publish-action truth, so a dropped evidence field, an unstated commit state, a local-only draft masquerading as a provider-committed update, or an offline / policy-blocked / partially-writable target shown as accepted is visible in evidence — a publish-mode change or a commit-state change — rather than hidden behind a green summary",
            "ready-for-review:m5-help-docs:001",
            vec![
                D::ProviderOwnershipUnstated,
                D::LocalHandoffShownAsProviderCommitted,
                D::ChangeIntentMatrixStale,
            ],
            vec![review_pack_record_full_tree_scope_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5ReadyForReviewHandoffRegistriesGovernanceReview {
    M5ReadyForReviewHandoffRegistriesGovernanceReview {
        review_pack_record_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_review_pack_record_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        review_pack_result_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        review_pack_record_or_downgrade_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5ReadyForReviewHandoffRegistriesConsumerProjection {
    M5ReadyForReviewHandoffRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5ReadyForReviewHandoffRegistriesProofFreshness {
    M5ReadyForReviewHandoffRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ReadyForReviewHandoffRegistriesReleasePosture {
    M5ReadyForReviewHandoffRegistriesReleasePosture {
        proof_packet_ref: M5_READY_FOR_REVIEW_HANDOFF_REGISTRIES_ARTIFACT_REF.to_owned(),
        line_audit_ref: M5_READY_FOR_REVIEW_HANDOFF_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_READY_FOR_REVIEW_HANDOFF_REGISTRIES_SCHEMA_REF,
        M5_READY_FOR_REVIEW_HANDOFF_REGISTRIES_DOC_REF,
        M5_CHANGE_INTENT_MATRIX_SCHEMA_REF,
        M5_CHANGE_INTENT_MATRIX_DOC_REF,
        M5_CHANGE_INTENT_DOMAIN_SCHEMA_REF,
        M5_READY_FOR_REVIEW_HANDOFF_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 ready-for-review-handoff-sheet and publish-action registries packet.
pub fn seeded_m5_ready_for_review_handoff_registries() -> M5ReadyForReviewHandoffRegistriesPacket {
    M5ReadyForReviewHandoffRegistriesPacket::new(
        M5ReadyForReviewHandoffRegistriesPacketInput {
            packet_id: M5_READY_FOR_REVIEW_HANDOFF_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 ready-for-review-handoff-sheet and publish-action registries emitting one reusable machine-readable ready-for-review handoff sheet per tracked work item — one typed field per sheet section, summary-first: the changed scope, the checks / test state, the linked review object, the comment draft, the attached evidence, the provider mutation list, and the export-versus-publish actions — each bound to one commit state with its lineage, so a handoff never drops its changed scope / linked review / attached evidence and no local-only draft or queued publish reads as a provider-committed update, with canonical / accessible / audit resolution-form coverage, and a machine-readable publish-action object (publish now, queue for publish, or export a local packet) that keeps each publish outcome a visible, typed action — so a handoff never implies provider acceptance when the target is offline, policy-blocked, or only partially writable — across work-item detail, review detail, Git / worktree, provider-handoff, help / docs, and support / export surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5ReadyForReviewHandoffRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the work-item-detail row is held at Beta pending ready-for-review-handoff parity on every
/// commit state; every row stays visible and every example stays honest.
pub fn seeded_m5_ready_for_review_handoff_registries_review_pack_record_beta_narrowed(
) -> M5ReadyForReviewHandoffRegistriesPacket {
    let mut packet = seeded_m5_ready_for_review_handoff_registries();
    packet.packet_id =
        "m5-ready-for-review-handoff-registries:ready-for-review-handoff-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ChangeIntentConsumerSurface::WorkItemDetail)
        .expect("work-item-detail row present");
    row.qualification = M5ChangeIntentQualificationClass::Beta;
    packet
}

/// Narrowed variant: the review-detail row is narrowed to Preview pending publish-action parity on
/// every publish-mode binding; every row stays visible and every example stays honest.
pub fn seeded_m5_ready_for_review_handoff_registries_review_pack_result_preview_narrowed(
) -> M5ReadyForReviewHandoffRegistriesPacket {
    let mut packet = seeded_m5_ready_for_review_handoff_registries();
    packet.packet_id = "m5-ready-for-review-handoff-registries:relation-preview:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ChangeIntentConsumerSurface::ReviewDetail)
        .expect("review-detail row present");
    row.qualification = M5ChangeIntentQualificationClass::Preview;
    packet
}
