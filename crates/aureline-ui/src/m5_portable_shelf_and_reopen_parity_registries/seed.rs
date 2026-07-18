//! Canonical seed builders for the M5 review-pack-record and review-pack-result registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean review-pack-record and review-pack-result entries
//! are built so the one typed review-pack-record object resolving per pack, records never widening a local
//! parity estimate into provider-authoritative mergeability, divergence labels never hidden behind a green
//! summary, the canonical / accessible / audit resolution forms, and the complete evaluated-scope / pack
//! version-and-digest / target-diff / worktree-base / evaluator-outcome review-pack-result object are proven
//! across the review-detail, AI-review, review-pack-summary, local-CI-parity, provider-handoff, and support
//! surfaces without any hand-copied per-pack assumption, estimate-shown-as-authoritative, incomplete object,
//! hidden divergence label, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_PORTABLE_SHELF_RECORD_AND_RESULT_REGISTRIES_PACKET_ID: &str =
    "m5-portable-shelf-and-reopen-parity-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn review_pack_record(
    input: M5PortableShelfRecordEntryResolutionInput,
) -> M5ResolvedPortableShelfRecordEntry {
    resolve_review_pack_record_entry(input).expect("seed line-review_pack_record entry resolves")
}

fn downgrade(
    input: M5PortableShelfResultEntryResolutionInput,
) -> M5ResolvedPortableShelfResultEntry {
    resolve_review_pack_result_entry(input).expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5PortableShelfRecordResolutionForm> {
    M5PortableShelfRecordResolutionForm::ALL.to_vec()
}

// -- Clean line-review_pack_record entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_review_pack_record_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5ChangeOrchestrationRole,
    report_section: M5PortableShelfRecordKind,
    surface_context: M5PortableShelfRecordSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5PortableShelfRecordEntryResolutionInput {
    M5PortableShelfRecordEntryResolutionInput {
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

fn review_pack_record_changed_files_scope_clean() -> M5ResolvedPortableShelfRecordEntry {
    review_pack_record(clean_review_pack_record_base(
        "review_pack_record:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.review_pack_record.core_team_canary",
        M5ChangeOrchestrationRole::SelectedChangeObjectDisclosure,
        M5PortableShelfRecordKind::ChangedFilesScope,
        M5PortableShelfRecordSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn review_pack_record_pull_request_scope_clean() -> M5ResolvedPortableShelfRecordEntry {
    review_pack_record(clean_review_pack_record_base(
        "review_pack_record:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.review_pack_record.pull_request_scope",
        M5ChangeOrchestrationRole::WorktreeBindingDisclosure,
        M5PortableShelfRecordKind::PullRequestScope,
        M5PortableShelfRecordSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn review_pack_record_base_head_range_scope_clean() -> M5ResolvedPortableShelfRecordEntry {
    review_pack_record(clean_review_pack_record_base(
        "review_pack_record:program-governance:extension-author",
        "launch.line.extension-author",
        "line.review_pack_record.base_head_range_scope",
        M5ChangeOrchestrationRole::StackMembershipDisclosure,
        M5PortableShelfRecordKind::BaseHeadRangeScope,
        M5PortableShelfRecordSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn review_pack_record_worktree_uncommitted_scope_clean() -> M5ResolvedPortableShelfRecordEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_review_pack_record_base(
        "review_pack_record:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.review_pack_record.worktree_uncommitted_scope",
        M5ChangeOrchestrationRole::SelectedChangeObjectDisclosure,
        M5PortableShelfRecordKind::WorktreeUncommittedScope,
        M5PortableShelfRecordSurfaceContext::ExecutiveSteeringSurface,
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

fn review_pack_record_full_tree_scope_clean() -> M5ResolvedPortableShelfRecordEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_review_pack_record_base(
        "review_pack_record:support:public-preview",
        "launch.line.public-preview",
        "line.review_pack_record.full_tree_scope",
        M5ChangeOrchestrationRole::WorktreeBindingDisclosure,
        M5PortableShelfRecordKind::FullTreeScope,
        M5PortableShelfRecordSurfaceContext::SupportOrExportForm,
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

fn review_pack_record_saved_pack_snapshot_scope_clean() -> M5ResolvedPortableShelfRecordEntry {
    review_pack_record(clean_review_pack_record_base(
        "review_pack_record:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.review_pack_record.saved_pack_snapshot_scope",
        M5ChangeOrchestrationRole::LandingStateDisclosure,
        M5PortableShelfRecordKind::SavedPackSnapshotScope,
        M5PortableShelfRecordSurfaceContext::ReleaseCenterSurface,
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
fn review_pack_record_object_incomplete() -> M5ResolvedPortableShelfRecordEntry {
    let mut base = clean_review_pack_record_base(
        "review_pack_record:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.review_pack_record.core_team_canary",
        M5ChangeOrchestrationRole::SelectedChangeObjectDisclosure,
        M5PortableShelfRecordKind::ChangedFilesScope,
        M5PortableShelfRecordSurfaceContext::ShiproomSurface,
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
fn review_pack_record_widen_fold() -> M5ResolvedPortableShelfRecordEntry {
    let mut base = clean_review_pack_record_base(
        "review_pack_record:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.review_pack_record.pull_request_scope",
        M5ChangeOrchestrationRole::WorktreeBindingDisclosure,
        M5PortableShelfRecordKind::PullRequestScope,
        M5PortableShelfRecordSurfaceContext::ReleaseCenterSurface,
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
fn review_pack_record_unbound() -> M5ResolvedPortableShelfRecordEntry {
    let mut base = clean_review_pack_record_base(
        "review_pack_record:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.review_pack_record.saved_pack_snapshot_scope",
        M5ChangeOrchestrationRole::LandingStateDisclosure,
        M5PortableShelfRecordKind::SavedPackSnapshotScope,
        M5PortableShelfRecordSurfaceContext::ExecutiveSteeringSurface,
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
fn review_pack_record_token_unstated() -> M5ResolvedPortableShelfRecordEntry {
    let mut base = clean_review_pack_record_base(
        "review_pack_record:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5ChangeOrchestrationRole::StackMembershipDisclosure,
        M5PortableShelfRecordKind::BaseHeadRangeScope,
        M5PortableShelfRecordSurfaceContext::ProgramGovernanceSurface,
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
    semantic_role: M5ChangeOrchestrationRole,
    comparison_scope: M5PortableShelfResultScope,
    surface_context: M5PortableShelfRecordSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5PortableShelfResultEntryResolutionInput {
    M5PortableShelfResultEntryResolutionInput {
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

fn downgrade_dogfood_ring_clean() -> M5ResolvedPortableShelfResultEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5ChangeOrchestrationRole::SelectedChangeObjectDisclosure,
        M5PortableShelfResultScope::EvaluatedScopeBinding,
        M5PortableShelfRecordSurfaceContext::ShiproomSurface,
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

fn downgrade_rehearsal_currency_clean() -> M5ResolvedPortableShelfResultEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.base_head_range_scope",
        M5ChangeOrchestrationRole::StackMembershipDisclosure,
        M5PortableShelfResultScope::PackVersionDigestBinding,
        M5PortableShelfRecordSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedPortableShelfResultEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.saved_pack_snapshot_scope",
        M5ChangeOrchestrationRole::LandingStateDisclosure,
        M5PortableShelfResultScope::DivergenceLabelBinding,
        M5PortableShelfRecordSurfaceContext::ReleaseCenterSurface,
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
fn downgrade_support_ahead() -> M5ResolvedPortableShelfResultEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.full_tree_scope",
        M5ChangeOrchestrationRole::WorktreeBindingDisclosure,
        M5PortableShelfResultScope::EvaluatedScopeBinding,
        M5PortableShelfRecordSurfaceContext::ShiproomSurface,
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
fn downgrade_form_incomplete() -> M5ResolvedPortableShelfResultEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.saved_pack_snapshot_scope",
        M5ChangeOrchestrationRole::LandingStateDisclosure,
        M5PortableShelfResultScope::DivergenceLabelBinding,
        M5PortableShelfRecordSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5PortableShelfRecordResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn comparison_review_pack_result_unclassified() -> M5ResolvedPortableShelfResultEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.worktree_uncommitted_scope",
        M5ChangeOrchestrationRole::SelectedChangeObjectDisclosure,
        M5PortableShelfResultScope::ChangeObjectResultUnclassified,
        M5PortableShelfRecordSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5PortableShelfRecordAndResultRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5ChangeOrchestrationDowngradeTrigger>,
    review_pack_record_entries: Vec<M5ResolvedPortableShelfRecordEntry>,
    review_pack_result_entries: Vec<M5ResolvedPortableShelfResultEntry>,
) -> M5PortableShelfRecordAndResultRegistriesRow {
    M5PortableShelfRecordAndResultRegistriesRow {
        consumer_surface,
        qualification: M5ChangeOrchestrationQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        capture_lifecycle_stages: M5ChangeOrchestrationClassificationStage::ALL.to_vec(),
        required_labels: M5ChangeOrchestrationRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5ChangeOrchestrationAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5PortableShelfRecordAnatomyPart::ALL.to_vec(),
        export_fields: M5PortableShelfRecordExportField::ALL.to_vec(),
        downgrade_triggers,
        review_pack_record_entries,
        review_pack_result_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_PORTABLE_SHELF_RECORD_AND_RESULT_REGISTRIES_SCHEMA_REF,
            M5_PORTABLE_SHELF_DOMAIN_SCHEMA_REF,
            M5_PORTABLE_SHELF_REOPEN_PARITY_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_review_pack_result_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5PortableShelfRecordAndResultRegistriesRow> {
    use M5ChangeOrchestrationConsumerSurface as C;
    use M5ChangeOrchestrationDowngradeTrigger as D;

    vec![
        base_row(
            C::ChangeObjectDetail,
            "Change-object-detail owner",
            "The change-object detail resolves the portable shelf / bundle for a selected change from the shared registry — the bundle ID, the diff refs, the evidence refs, the review-pack version, the redaction profile, and the import / reopen status — and keeps the reopen-parity posture of that shelf bound separately; a portable shelf missing its bundle ID or diff / evidence refs and a reopen parity that would reopen an imported shelf without naming its handoff channel, reopen state, reopened truth posture, or preserved diff / evidence identity degrade honestly instead of letting a local imported shelf read as a live provider object",
            "worktree:m5-change-object-detail:001",
            vec![
                D::StackMembershipSourceUnstated,
                D::CrossWorktreeWriteWithoutSelectedChangeObject,
                D::ChangeOrchestrationMatrixStale,
            ],
            vec![
                review_pack_record_changed_files_scope_clean(),
                review_pack_record_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::PatchStackQueue,
            "Patch-stack-queue owner",
            "The patch-stack / queue resolves the portable shelf / bundle for each stack member's change object — bundle ID, diff refs, evidence refs, review-pack version, and import / reopen status — so which member is exported, which is imported as local-only, and which reopened provider-linked are visible before any landing rather than after; a shelf that would land from a stale imported estimate and a member whose diff / evidence identity was dropped on reopen are caught before a green summary can hide them, so no member is silently reordered, collapsed, or retargeted and nothing lands from an overclaimed imported shelf",
            "worktree:m5-patch-stack-queue:001",
            vec![
                D::StackMembershipSourceUnstated,
                D::ValidationFreshnessUnstated,
                D::ChangeOrchestrationMatrixStale,
            ],
            vec![review_pack_record_pull_request_scope_clean(), review_pack_record_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::SupportExportPacket,
            "Support owner",
            "Support resolves each portable shelf's bundle ID, diff refs, evidence refs, and review-pack version while keeping the reopen-parity posture bound separately to the export; a portable shelf that is a hand-copied per-entry assumption and a reopen parity left unclassified degrade honestly so the diff / evidence identity, its recovery checkpoint, and its export-safe evidence are never dropped on export or reopen — a user can reopen the exported bundle offline or through a support escalation instead of losing the diff / evidence identity when a provider link goes stale or a redaction profile applies",
            "worktree:m5-support:001",
            vec![
                D::SelectedChangeObjectUnstated,
                D::LandingStateUnstated,
                D::ChangeOrchestrationMatrixStale,
            ],
            vec![
                review_pack_record_worktree_uncommitted_scope_clean(),
                review_pack_record_unbound(),
            ],
            vec![comparison_review_pack_result_unclassified()],
        ),
        base_row(
            C::StackEditReviewSheet,
            "Stack-edit-review-sheet owner",
            "The stack-edit / review sheet renders the same resolved portable-shelf and reopen-parity truth bound to the registry — bundle ID, diff refs, evidence refs, review-pack version, and redaction profile — so a shelf's diff / evidence identity and reopen state can no longer be flattened into one generic badge; a portable shelf with an unstated bundle ID or diff / evidence refs is caught before Aureline reopens anything but an explicitly identified, identity-preserved shelf",
            "worktree:m5-portable-shelf:001",
            vec![
                D::SelectedChangeObjectUnstated,
                D::WorktreeBindingUnstated,
                D::ChangeOrchestrationMatrixStale,
            ],
            vec![
                review_pack_record_base_head_range_scope_clean(),
                review_pack_record_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::ProviderMergeQueue,
            "Provider-merge-queue owner",
            "The provider merge queue renders the same resolved portable-shelf and reopen-parity truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied sheet; the reopen-parity posture and the reopened truth (local-only, provider-linked, stale, or redacted) stay inspectable off-renderer so an imported shelf stays blocked from reading as provider-authoritative for background agents and broad automation unless the user explicitly reopens and confirms it, and a stale provider link never reads as current hosted truth",
            "worktree:m5-provider-merge-queue:001",
            vec![
                D::WorktreeBindingUnstated,
                D::SelectedChangeObjectUnstated,
                D::ChangeOrchestrationMatrixStale,
            ],
            vec![review_pack_record_saved_pack_snapshot_scope_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::HelpDocs,
            "Help-docs owner",
            "The help / docs feed carries the same resolved portable-shelf and reopen-parity truth, so a dropped bundle ID, an unstated diff / evidence ref, a diff / evidence identity shown as preserved when it was lost, or an imported shelf reopened by background automation as provider-authoritative is visible in evidence — a handoff channel, a reopen state, or a reopened truth posture — rather than hidden behind a green summary",
            "worktree:m5-help-docs:001",
            vec![
                D::SelectedChangeObjectUnstated,
                D::CrossWorktreeWriteWithoutSelectedChangeObject,
                D::ChangeOrchestrationMatrixStale,
            ],
            vec![review_pack_record_full_tree_scope_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5PortableShelfRecordAndResultRegistriesGovernanceReview {
    M5PortableShelfRecordAndResultRegistriesGovernanceReview {
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

fn consumer_projection() -> M5PortableShelfRecordAndResultRegistriesConsumerProjection {
    M5PortableShelfRecordAndResultRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5PortableShelfRecordAndResultRegistriesProofFreshness {
    M5PortableShelfRecordAndResultRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5PortableShelfRecordAndResultRegistriesReleasePosture {
    M5PortableShelfRecordAndResultRegistriesReleasePosture {
        proof_packet_ref: M5_PORTABLE_SHELF_RECORD_AND_RESULT_REGISTRIES_ARTIFACT_REF.to_owned(),
        line_audit_ref: M5_PORTABLE_SHELF_RECORD_AND_RESULT_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_PORTABLE_SHELF_RECORD_AND_RESULT_REGISTRIES_SCHEMA_REF,
        M5_PORTABLE_SHELF_RECORD_AND_RESULT_REGISTRIES_DOC_REF,
        M5_CHANGE_ORCHESTRATION_MATRIX_SCHEMA_REF,
        M5_CHANGE_ORCHESTRATION_MATRIX_DOC_REF,
        M5_PORTABLE_SHELF_DOMAIN_SCHEMA_REF,
        M5_PORTABLE_SHELF_REOPEN_PARITY_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 worktree-manager-row and cleanup-preview registries packet.
pub fn seeded_m5_portable_shelf_and_reopen_parity_registries(
) -> M5PortableShelfRecordAndResultRegistriesPacket {
    M5PortableShelfRecordAndResultRegistriesPacket::new(
        M5PortableShelfRecordAndResultRegistriesPacketInput {
            packet_id: M5_PORTABLE_SHELF_RECORD_AND_RESULT_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 portable-shelf and reopen-parity registries emitting one durable machine-readable portable shelf / bundle per change object or landing candidate — one typed field per section: the bundle ID, the diff refs, the evidence refs, the review-pack version, the redaction profile, and the import / reopen status — with the reopen-parity posture of that shelf kept separate, so stacked work survives browser handoff, offline follow-up, support escalation, incident bridge, and review export without requiring one specific code host or cloud service and no member is silently reordered, collapsed, or retargeted, with canonical / accessible / audit resolution-form coverage, and a machine-readable reopen parity that never lets an imported shelf overclaim current hosted truth — it names the handoff channel, the reopen state, the reopened truth posture (local-only, provider-linked, stale, or redacted), and whether the diff / evidence identity is preserved, so an imported shelf stays blocked from reading as a live provider object for background agents and broad automation unless the user explicitly reopens and confirms it, and nothing lands from an overclaimed imported shelf — rather than a green summary across Git, patch-stack / queue, review, provider-landing, help / docs, and support / export surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5PortableShelfRecordAndResultRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the review-detail row is held at Beta pending review-pack-record parity on every pack
/// version / digest; every row stays visible and every example stays honest.
pub fn seeded_m5_portable_shelf_and_reopen_parity_registries_review_pack_record_beta_narrowed(
) -> M5PortableShelfRecordAndResultRegistriesPacket {
    let mut packet = seeded_m5_portable_shelf_and_reopen_parity_registries();
    packet.packet_id =
        "m5-portable-shelf-and-reopen-parity-registries:manager-row-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| {
            row.consumer_surface == M5ChangeOrchestrationConsumerSurface::ChangeObjectDetail
        })
        .expect("review-detail row present");
    row.qualification = M5ChangeOrchestrationQualificationClass::Beta;
    packet
}

/// Narrowed variant: the AI-review row is narrowed to Preview pending review-pack-result parity on every
/// evaluator binding; every row stays visible and every example stays honest.
pub fn seeded_m5_portable_shelf_and_reopen_parity_registries_review_pack_result_preview_narrowed(
) -> M5PortableShelfRecordAndResultRegistriesPacket {
    let mut packet = seeded_m5_portable_shelf_and_reopen_parity_registries();
    packet.packet_id =
        "m5-portable-shelf-and-reopen-parity-registries:cleanup-preview:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ChangeOrchestrationConsumerSurface::PatchStackQueue)
        .expect("AI-review row present");
    row.qualification = M5ChangeOrchestrationQualificationClass::Preview;
    packet
}
