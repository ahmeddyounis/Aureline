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
pub const M5_REVIEW_PACK_RECORD_AND_RESULT_REGISTRIES_PACKET_ID: &str =
    "m5-review-pack-record-and-result-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn review_pack_record(
    input: M5ReviewPackRecordEntryResolutionInput,
) -> M5ResolvedReviewPackRecordEntry {
    resolve_review_pack_record_entry(input).expect("seed line-review_pack_record entry resolves")
}

fn downgrade(input: M5ReviewPackResultEntryResolutionInput) -> M5ResolvedReviewPackResultEntry {
    resolve_review_pack_result_entry(input).expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5ReviewPackRecordResolutionForm> {
    M5ReviewPackRecordResolutionForm::ALL.to_vec()
}

// -- Clean line-review_pack_record entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_review_pack_record_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5ReviewPackRole,
    report_section: M5ReviewPackRecordKind,
    surface_context: M5ReviewPackRecordSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5ReviewPackRecordEntryResolutionInput {
    M5ReviewPackRecordEntryResolutionInput {
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

fn review_pack_record_changed_files_scope_clean() -> M5ResolvedReviewPackRecordEntry {
    review_pack_record(clean_review_pack_record_base(
        "review_pack_record:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.review_pack_record.core_team_canary",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5ReviewPackRecordKind::ChangedFilesScope,
        M5ReviewPackRecordSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn review_pack_record_pull_request_scope_clean() -> M5ResolvedReviewPackRecordEntry {
    review_pack_record(clean_review_pack_record_base(
        "review_pack_record:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.review_pack_record.pull_request_scope",
        M5ReviewPackRole::PackVersionAndDigestDisclosure,
        M5ReviewPackRecordKind::PullRequestScope,
        M5ReviewPackRecordSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn review_pack_record_base_head_range_scope_clean() -> M5ResolvedReviewPackRecordEntry {
    review_pack_record(clean_review_pack_record_base(
        "review_pack_record:program-governance:extension-author",
        "launch.line.extension-author",
        "line.review_pack_record.base_head_range_scope",
        M5ReviewPackRole::OwnerProvenanceDisclosure,
        M5ReviewPackRecordKind::BaseHeadRangeScope,
        M5ReviewPackRecordSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn review_pack_record_worktree_uncommitted_scope_clean() -> M5ResolvedReviewPackRecordEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_review_pack_record_base(
        "review_pack_record:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.review_pack_record.worktree_uncommitted_scope",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5ReviewPackRecordKind::WorktreeUncommittedScope,
        M5ReviewPackRecordSurfaceContext::ExecutiveSteeringSurface,
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

fn review_pack_record_full_tree_scope_clean() -> M5ResolvedReviewPackRecordEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_review_pack_record_base(
        "review_pack_record:support:public-preview",
        "launch.line.public-preview",
        "line.review_pack_record.full_tree_scope",
        M5ReviewPackRole::PackVersionAndDigestDisclosure,
        M5ReviewPackRecordKind::FullTreeScope,
        M5ReviewPackRecordSurfaceContext::SupportOrExportForm,
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

fn review_pack_record_saved_pack_snapshot_scope_clean() -> M5ResolvedReviewPackRecordEntry {
    review_pack_record(clean_review_pack_record_base(
        "review_pack_record:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.review_pack_record.saved_pack_snapshot_scope",
        M5ReviewPackRole::LocalVersusProviderParityDisclosure,
        M5ReviewPackRecordKind::SavedPackSnapshotScope,
        M5ReviewPackRecordSurfaceContext::ReleaseCenterSurface,
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
fn review_pack_record_object_incomplete() -> M5ResolvedReviewPackRecordEntry {
    let mut base = clean_review_pack_record_base(
        "review_pack_record:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.review_pack_record.core_team_canary",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5ReviewPackRecordKind::ChangedFilesScope,
        M5ReviewPackRecordSurfaceContext::ShiproomSurface,
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
fn review_pack_record_widen_fold() -> M5ResolvedReviewPackRecordEntry {
    let mut base = clean_review_pack_record_base(
        "review_pack_record:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.review_pack_record.pull_request_scope",
        M5ReviewPackRole::PackVersionAndDigestDisclosure,
        M5ReviewPackRecordKind::PullRequestScope,
        M5ReviewPackRecordSurfaceContext::ReleaseCenterSurface,
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
fn review_pack_record_unbound() -> M5ResolvedReviewPackRecordEntry {
    let mut base = clean_review_pack_record_base(
        "review_pack_record:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.review_pack_record.saved_pack_snapshot_scope",
        M5ReviewPackRole::LocalVersusProviderParityDisclosure,
        M5ReviewPackRecordKind::SavedPackSnapshotScope,
        M5ReviewPackRecordSurfaceContext::ExecutiveSteeringSurface,
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
fn review_pack_record_token_unstated() -> M5ResolvedReviewPackRecordEntry {
    let mut base = clean_review_pack_record_base(
        "review_pack_record:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5ReviewPackRole::OwnerProvenanceDisclosure,
        M5ReviewPackRecordKind::BaseHeadRangeScope,
        M5ReviewPackRecordSurfaceContext::ProgramGovernanceSurface,
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
    semantic_role: M5ReviewPackRole,
    comparison_scope: M5ReviewPackResultScope,
    surface_context: M5ReviewPackRecordSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5ReviewPackResultEntryResolutionInput {
    M5ReviewPackResultEntryResolutionInput {
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

fn downgrade_dogfood_ring_clean() -> M5ResolvedReviewPackResultEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5ReviewPackResultScope::EvaluatedScopeBinding,
        M5ReviewPackRecordSurfaceContext::ShiproomSurface,
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

fn downgrade_rehearsal_currency_clean() -> M5ResolvedReviewPackResultEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.base_head_range_scope",
        M5ReviewPackRole::OwnerProvenanceDisclosure,
        M5ReviewPackResultScope::PackVersionDigestBinding,
        M5ReviewPackRecordSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedReviewPackResultEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.saved_pack_snapshot_scope",
        M5ReviewPackRole::LocalVersusProviderParityDisclosure,
        M5ReviewPackResultScope::DivergenceLabelBinding,
        M5ReviewPackRecordSurfaceContext::ReleaseCenterSurface,
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
fn downgrade_support_ahead() -> M5ResolvedReviewPackResultEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.full_tree_scope",
        M5ReviewPackRole::PackVersionAndDigestDisclosure,
        M5ReviewPackResultScope::EvaluatedScopeBinding,
        M5ReviewPackRecordSurfaceContext::ShiproomSurface,
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
fn downgrade_form_incomplete() -> M5ResolvedReviewPackResultEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.saved_pack_snapshot_scope",
        M5ReviewPackRole::LocalVersusProviderParityDisclosure,
        M5ReviewPackResultScope::DivergenceLabelBinding,
        M5ReviewPackRecordSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5ReviewPackRecordResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn comparison_review_pack_result_unclassified() -> M5ResolvedReviewPackResultEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.worktree_uncommitted_scope",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5ReviewPackResultScope::ReviewPackResultUnclassified,
        M5ReviewPackRecordSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5ReviewPackRecordAndResultRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5ReviewPackDowngradeTrigger>,
    review_pack_record_entries: Vec<M5ResolvedReviewPackRecordEntry>,
    review_pack_result_entries: Vec<M5ResolvedReviewPackResultEntry>,
) -> M5ReviewPackRecordAndResultRegistriesRow {
    M5ReviewPackRecordAndResultRegistriesRow {
        consumer_surface,
        qualification: M5ReviewPackQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        capture_lifecycle_stages: M5ReviewPackClassificationStage::ALL.to_vec(),
        required_labels: M5ReviewPackRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5ReviewPackAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5ReviewPackRecordAnatomyPart::ALL.to_vec(),
        export_fields: M5ReviewPackRecordExportField::ALL.to_vec(),
        downgrade_triggers,
        review_pack_record_entries,
        review_pack_result_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_REVIEW_PACK_RECORD_AND_RESULT_REGISTRIES_SCHEMA_REF,
            M5_REVIEW_PACK_DOMAIN_SCHEMA_REF,
            M5_REVIEW_PACK_RESULT_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_review_pack_result_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5ReviewPackRecordAndResultRegistriesRow> {
    use M5ReviewPackConsumerSurface as C;
    use M5ReviewPackDowngradeTrigger as D;

    vec![
        base_row(
            C::ReviewDetail,
            "Review-detail owner",
            "Review detail resolves a repo-defined review pack to one typed review-pack-record object — the pack version and content digest, the scope selector, the target diff identity, the worktree / base revision, and the evaluator outcome — from the shared registry and proves the evaluated-scope-binding result for that pack; a record missing its pack digest and a result that would let a local parity estimate read as provider-authoritative mergeability degrade honestly instead of leaving a stale pack to read as a fresh, authoritative review result",
            "review-pack:m5-review-detail:001",
            vec![
                D::PackVersionDigestUnstated,
                D::UnevaluatedCheckHiddenBehindGreenSummary,
                D::ReviewPackMatrixStale,
            ],
            vec![
                review_pack_record_changed_files_scope_clean(),
                review_pack_record_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::AiReviewPanel,
            "AI-review owner",
            "The AI review panel resolves the pack-version-and-digest binding and the divergence-label result while keeping the analyzed scope and the pack version the review ran under visible; a record widening a local estimate into provider-authoritative mergeability and a resolution-form gap on a result are caught before a green summary can reintroduce an authoritative reading, and an AI review can never run under an undisclosed pack version",
            "review-pack:m5-ai-review:001",
            vec![
                D::PackVersionDigestUnstated,
                D::PackVersionOrDigestDropped,
                D::ReviewPackMatrixStale,
            ],
            vec![review_pack_record_pull_request_scope_clean(), review_pack_record_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::SupportExportPacket,
            "Support owner",
            "Support resolves the evaluator result class while keeping the pack version / digest and comment / summary template attribution bound to the export, and reports the divergence-label result; a record that is a hand-copied per-entry assumption and a result on an unclassified evaluator binding degrade honestly so pack identity and evaluator lineage are never dropped on export or reopen",
            "review-pack:m5-support:001",
            vec![
                D::ParityStateUnstated,
                D::PackFreshnessUnstated,
                D::ReviewPackMatrixStale,
            ],
            vec![
                review_pack_record_worktree_uncommitted_scope_clean(),
                review_pack_record_unbound(),
            ],
            vec![comparison_review_pack_result_unclassified()],
        ),
        base_row(
            C::ReviewPackSummary,
            "Review-pack-summary owner",
            "The review-pack summary resolves the scope selector and the pack-freshness result — stale-pack, partial-scope, or slice-omitted — bound to the registry so a stale or partially evaluated pack can no longer read as a fresh, full-coverage review result; an unstated pack version / digest on a record is caught before it can drift",
            "review-pack:m5-review-pack-summary:001",
            vec![
                D::ParityStateUnstated,
                D::EvaluatorResultClassUnstated,
                D::ReviewPackMatrixStale,
            ],
            vec![
                review_pack_record_base_head_range_scope_clean(),
                review_pack_record_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::LocalCiParityStrip,
            "Local-CI-parity owner",
            "The local-CI parity strip renders the same resolved review-pack-record and review-pack-result truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied table; the ci-only / not-evaluated-here / provider-unavailable label and the evaluated-scope-binding result stay inspectable off-renderer so a local parity estimate never reads as provider-authoritative mergeability",
            "review-pack:m5-local-ci-parity:001",
            vec![
                D::EvaluatorResultClassUnstated,
                D::ParityStateUnstated,
                D::ReviewPackMatrixStale,
            ],
            vec![review_pack_record_saved_pack_snapshot_scope_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::ProviderHandoff,
            "Provider-handoff owner",
            "The provider handoff feed carries the same resolved review-pack-record and review-pack-result truth, so a dropped pack version / digest, an unstated evaluator result class, a local estimate masquerading as provider-authoritative, or a stale-pack result shown as current is visible in evidence — an evaluated-scope change, a pack-version-and-digest change, or a divergence-label change — rather than hidden behind a green summary",
            "review-pack:m5-provider-handoff:001",
            vec![
                D::ParityStateUnstated,
                D::UnevaluatedCheckHiddenBehindGreenSummary,
                D::ReviewPackMatrixStale,
            ],
            vec![review_pack_record_full_tree_scope_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5ReviewPackRecordAndResultRegistriesGovernanceReview {
    M5ReviewPackRecordAndResultRegistriesGovernanceReview {
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

fn consumer_projection() -> M5ReviewPackRecordAndResultRegistriesConsumerProjection {
    M5ReviewPackRecordAndResultRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5ReviewPackRecordAndResultRegistriesProofFreshness {
    M5ReviewPackRecordAndResultRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ReviewPackRecordAndResultRegistriesReleasePosture {
    M5ReviewPackRecordAndResultRegistriesReleasePosture {
        proof_packet_ref: M5_REVIEW_PACK_RECORD_AND_RESULT_REGISTRIES_ARTIFACT_REF.to_owned(),
        line_audit_ref: M5_REVIEW_PACK_RECORD_AND_RESULT_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_REVIEW_PACK_RECORD_AND_RESULT_REGISTRIES_SCHEMA_REF,
        M5_REVIEW_PACK_RECORD_AND_RESULT_REGISTRIES_DOC_REF,
        M5_REVIEW_PACK_MATRIX_SCHEMA_REF,
        M5_REVIEW_PACK_MATRIX_DOC_REF,
        M5_REVIEW_PACK_DOMAIN_SCHEMA_REF,
        M5_REVIEW_PACK_RESULT_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 review-pack-record and review-pack-result registries packet.
pub fn seeded_m5_review_pack_record_and_result_registries(
) -> M5ReviewPackRecordAndResultRegistriesPacket {
    M5ReviewPackRecordAndResultRegistriesPacket::new(
        M5ReviewPackRecordAndResultRegistriesPacketInput {
            packet_id: M5_REVIEW_PACK_RECORD_AND_RESULT_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 review-pack-record and review-pack-result registries emitting one machine-readable review-pack-record per repo-defined review pack — one typed field per record section: the pack version and content digest, the scope selector, the target diff identity, the worktree / base revision, and the evaluator outcome — each bound to one pack identity with its evaluator lineage, so a review result never drops its pack version / digest or template attribution and no local parity estimate reads as provider-authoritative mergeability, with canonical / accessible / audit resolution-form coverage, and a machine-readable review-pack-result (evaluated-scope-binding, pack-version-and-digest-binding, or divergence-label-binding) that turns a changed evaluated scope, pack digest, or divergence label into a visible, typed evaluator event — partial-scope, slice-omitted, stale-pack, ci-only, or provider-unavailable — rather than a green summary across review, AI-review, provider-handoff, and support / export surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5ReviewPackRecordAndResultRegistriesVocabularySet::canonical(),
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
pub fn seeded_m5_review_pack_record_and_result_registries_review_pack_record_beta_narrowed(
) -> M5ReviewPackRecordAndResultRegistriesPacket {
    let mut packet = seeded_m5_review_pack_record_and_result_registries();
    packet.packet_id =
        "m5-review-pack-record-and-result-registries:review-pack-record-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ReviewPackConsumerSurface::ReviewDetail)
        .expect("review-detail row present");
    row.qualification = M5ReviewPackQualificationClass::Beta;
    packet
}

/// Narrowed variant: the AI-review row is narrowed to Preview pending review-pack-result parity on every
/// evaluator binding; every row stays visible and every example stays honest.
pub fn seeded_m5_review_pack_record_and_result_registries_review_pack_result_preview_narrowed(
) -> M5ReviewPackRecordAndResultRegistriesPacket {
    let mut packet = seeded_m5_review_pack_record_and_result_registries();
    packet.packet_id =
        "m5-review-pack-record-and-result-registries:review-pack-result-preview:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ReviewPackConsumerSurface::AiReviewPanel)
        .expect("AI-review row present");
    row.qualification = M5ReviewPackQualificationClass::Preview;
    packet
}
