//! Canonical seed builders for the M5 review-pack-invalidation-record and review-pack-rerun-compare registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean review-pack-invalidation-record and review-pack-rerun-compare entries
//! are built so one typed review-pack-invalidation-record object resolving per stale transition — one per
//! invalidation cause: base-revision drift, head-revision drift, worktree-scope drift, pack-version drift,
//! pack-digest drift, and environment-capsule drift — records never widening a stale local parity estimate into
//! fresh provider-authoritative mergeability, divergence labels never hidden behind a green summary, the canonical
//! / accessible / audit resolution forms, and the complete previous-packet / current-packet / preserved-draft-evidence
//! rerun-compare object are proven across the review-detail, AI-review, review-pack-summary, local-CI-parity,
//! provider-handoff, and support surfaces without any hand-copied per-pack assumption,
//! stale-shown-as-fresh-authoritative, incomplete object, hidden divergence label, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_REVIEW_PACK_INVALIDATION_AND_RERUN_COMPARE_REGISTRIES_PACKET_ID: &str =
    "m5-review-pack-invalidation-and-rerun-compare-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn review_pack_invalidation_record(
    input: M5ReviewPackInvalidationRecordEntryResolutionInput,
) -> M5ResolvedReviewPackInvalidationRecordEntry {
    resolve_review_pack_invalidation_record_entry(input)
        .expect("seed line-review_pack_invalidation_record entry resolves")
}

fn downgrade(
    input: M5ReviewPackRerunCompareEntryResolutionInput,
) -> M5ResolvedReviewPackRerunCompareEntry {
    resolve_review_pack_rerun_compare_entry(input)
        .expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5ReviewPackInvalidationRecordResolutionForm> {
    M5ReviewPackInvalidationRecordResolutionForm::ALL.to_vec()
}

// -- Clean line-review_pack_invalidation_record entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_review_pack_invalidation_record_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5ReviewPackRole,
    report_section: M5ReviewPackInvalidationRecordKind,
    surface_context: M5ReviewPackInvalidationRecordSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5ReviewPackInvalidationRecordEntryResolutionInput {
    M5ReviewPackInvalidationRecordEntryResolutionInput {
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

fn review_pack_invalidation_record_base_revision_drift_clean(
) -> M5ResolvedReviewPackInvalidationRecordEntry {
    review_pack_invalidation_record(clean_review_pack_invalidation_record_base(
        "review_pack_invalidation_record:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.review_pack_invalidation_record.core_team_canary",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5ReviewPackInvalidationRecordKind::BaseRevisionDrift,
        M5ReviewPackInvalidationRecordSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn review_pack_invalidation_record_head_revision_drift_clean(
) -> M5ResolvedReviewPackInvalidationRecordEntry {
    review_pack_invalidation_record(clean_review_pack_invalidation_record_base(
        "review_pack_invalidation_record:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.review_pack_invalidation_record.head_revision_drift",
        M5ReviewPackRole::PackVersionAndDigestDisclosure,
        M5ReviewPackInvalidationRecordKind::HeadRevisionDrift,
        M5ReviewPackInvalidationRecordSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn review_pack_invalidation_record_worktree_scope_drift_clean(
) -> M5ResolvedReviewPackInvalidationRecordEntry {
    review_pack_invalidation_record(clean_review_pack_invalidation_record_base(
        "review_pack_invalidation_record:program-governance:extension-author",
        "launch.line.extension-author",
        "line.review_pack_invalidation_record.worktree_scope_drift",
        M5ReviewPackRole::OwnerProvenanceDisclosure,
        M5ReviewPackInvalidationRecordKind::WorktreeScopeDrift,
        M5ReviewPackInvalidationRecordSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn review_pack_invalidation_record_pack_version_drift_clean(
) -> M5ResolvedReviewPackInvalidationRecordEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_review_pack_invalidation_record_base(
        "review_pack_invalidation_record:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.review_pack_invalidation_record.pack_version_drift",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5ReviewPackInvalidationRecordKind::PackVersionDrift,
        M5ReviewPackInvalidationRecordSurfaceContext::ExecutiveSteeringSurface,
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
    review_pack_invalidation_record(base)
}

fn review_pack_invalidation_record_pack_digest_drift_clean(
) -> M5ResolvedReviewPackInvalidationRecordEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_review_pack_invalidation_record_base(
        "review_pack_invalidation_record:support:public-preview",
        "launch.line.public-preview",
        "line.review_pack_invalidation_record.pack_digest_drift",
        M5ReviewPackRole::PackVersionAndDigestDisclosure,
        M5ReviewPackInvalidationRecordKind::PackDigestDrift,
        M5ReviewPackInvalidationRecordSurfaceContext::SupportOrExportForm,
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
    review_pack_invalidation_record(base)
}

fn review_pack_invalidation_record_environment_capsule_drift_clean(
) -> M5ResolvedReviewPackInvalidationRecordEntry {
    review_pack_invalidation_record(clean_review_pack_invalidation_record_base(
        "review_pack_invalidation_record:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.review_pack_invalidation_record.environment_capsule_drift",
        M5ReviewPackRole::LocalVersusProviderParityDisclosure,
        M5ReviewPackInvalidationRecordKind::EnvironmentCapsuleDrift,
        M5ReviewPackInvalidationRecordSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded line-review_pack_invalidation_record entries ---------------------------------------------------------

/// Degraded review_pack_invalidation_record entry: the resolved review_pack_invalidation_record object is incomplete — the bundle IDs are unstated.
fn review_pack_invalidation_record_object_incomplete() -> M5ResolvedReviewPackInvalidationRecordEntry
{
    let mut base = clean_review_pack_invalidation_record_base(
        "review_pack_invalidation_record:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.review_pack_invalidation_record.core_team_canary",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5ReviewPackInvalidationRecordKind::BaseRevisionDrift,
        M5ReviewPackInvalidationRecordSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    review_pack_invalidation_record(base)
}

/// Degraded review_pack_invalidation_record entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn review_pack_invalidation_record_widen_fold() -> M5ResolvedReviewPackInvalidationRecordEntry {
    let mut base = clean_review_pack_invalidation_record_base(
        "review_pack_invalidation_record:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.review_pack_invalidation_record.head_revision_drift",
        M5ReviewPackRole::PackVersionAndDigestDisclosure,
        M5ReviewPackInvalidationRecordKind::HeadRevisionDrift,
        M5ReviewPackInvalidationRecordSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    review_pack_invalidation_record(base)
}

/// Degraded review_pack_invalidation_record entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn review_pack_invalidation_record_unbound() -> M5ResolvedReviewPackInvalidationRecordEntry {
    let mut base = clean_review_pack_invalidation_record_base(
        "review_pack_invalidation_record:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.review_pack_invalidation_record.environment_capsule_drift",
        M5ReviewPackRole::LocalVersusProviderParityDisclosure,
        M5ReviewPackInvalidationRecordKind::EnvironmentCapsuleDrift,
        M5ReviewPackInvalidationRecordSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    review_pack_invalidation_record(base)
}

/// Degraded review_pack_invalidation_record entry: the canonical registry token name is unstated.
fn review_pack_invalidation_record_token_unstated() -> M5ResolvedReviewPackInvalidationRecordEntry {
    let mut base = clean_review_pack_invalidation_record_base(
        "review_pack_invalidation_record:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5ReviewPackRole::OwnerProvenanceDisclosure,
        M5ReviewPackInvalidationRecordKind::WorktreeScopeDrift,
        M5ReviewPackInvalidationRecordSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    review_pack_invalidation_record(base)
}

// -- Clean line-downgrade-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_downgrade_base(
    entry_id: &str,
    comparison_ref: &str,
    token_name: &str,
    semantic_role: M5ReviewPackRole,
    comparison_scope: M5ReviewPackRerunCompareScope,
    surface_context: M5ReviewPackInvalidationRecordSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5ReviewPackRerunCompareEntryResolutionInput {
    M5ReviewPackRerunCompareEntryResolutionInput {
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
        keeps_review_pack_rerun_compare_visible: true,
        comparison_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn downgrade_dogfood_ring_clean() -> M5ResolvedReviewPackRerunCompareEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5ReviewPackRerunCompareScope::PreviousPacketBinding,
        M5ReviewPackInvalidationRecordSurfaceContext::ShiproomSurface,
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

fn downgrade_rehearsal_currency_clean() -> M5ResolvedReviewPackRerunCompareEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.worktree_scope_drift",
        M5ReviewPackRole::OwnerProvenanceDisclosure,
        M5ReviewPackRerunCompareScope::CurrentPacketBinding,
        M5ReviewPackInvalidationRecordSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedReviewPackRerunCompareEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.environment_capsule_drift",
        M5ReviewPackRole::LocalVersusProviderParityDisclosure,
        M5ReviewPackRerunCompareScope::PreservedDraftEvidenceBinding,
        M5ReviewPackInvalidationRecordSurfaceContext::ReleaseCenterSurface,
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
fn downgrade_support_ahead() -> M5ResolvedReviewPackRerunCompareEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.pack_digest_drift",
        M5ReviewPackRole::PackVersionAndDigestDisclosure,
        M5ReviewPackRerunCompareScope::PreviousPacketBinding,
        M5ReviewPackInvalidationRecordSurfaceContext::ShiproomSurface,
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
fn downgrade_form_incomplete() -> M5ResolvedReviewPackRerunCompareEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.environment_capsule_drift",
        M5ReviewPackRole::LocalVersusProviderParityDisclosure,
        M5ReviewPackRerunCompareScope::PreservedDraftEvidenceBinding,
        M5ReviewPackInvalidationRecordSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage =
        vec![M5ReviewPackInvalidationRecordResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn comparison_review_pack_rerun_compare_unclassified() -> M5ResolvedReviewPackRerunCompareEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.pack_version_drift",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5ReviewPackRerunCompareScope::ReviewPackRerunCompareUnclassified,
        M5ReviewPackInvalidationRecordSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5ReviewPackInvalidationAndRerunCompareRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5ReviewPackDowngradeTrigger>,
    review_pack_invalidation_record_entries: Vec<M5ResolvedReviewPackInvalidationRecordEntry>,
    review_pack_rerun_compare_entries: Vec<M5ResolvedReviewPackRerunCompareEntry>,
) -> M5ReviewPackInvalidationAndRerunCompareRegistriesRow {
    M5ReviewPackInvalidationAndRerunCompareRegistriesRow {
        consumer_surface,
        qualification: M5ReviewPackQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        capture_lifecycle_stages: M5ReviewPackClassificationStage::ALL.to_vec(),
        required_labels: M5ReviewPackRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5ReviewPackAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5ReviewPackInvalidationRecordAnatomyPart::ALL.to_vec(),
        export_fields: M5ReviewPackInvalidationRecordExportField::ALL.to_vec(),
        downgrade_triggers,
        review_pack_invalidation_record_entries,
        review_pack_rerun_compare_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_REVIEW_PACK_INVALIDATION_AND_RERUN_COMPARE_REGISTRIES_SCHEMA_REF,
            M5_REVIEW_PACK_DOMAIN_SCHEMA_REF,
            M5_REVIEW_PACK_RESULT_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_review_pack_rerun_compare_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5ReviewPackInvalidationAndRerunCompareRegistriesRow> {
    use M5ReviewPackConsumerSurface as C;
    use M5ReviewPackDowngradeTrigger as D;

    vec![
        base_row(
            C::ReviewDetail,
            "Review-detail owner",
            "Review detail resolves a base-revision drift to one typed review-pack-invalidation-record object — the invalidation cause, the pack version and content digest, the drifted base / head / worktree scope, the target diff identity, and the evaluator outcome — from the shared registry and proves the previous-packet rerun/compare binding for that pack; a record that cannot name its invalidation cause and a compare that would let a stale local parity estimate read as fresh provider-authoritative mergeability degrade honestly instead of leaving a stale pack to read as a fresh, authoritative review result",
            "review-pack:m5-review-detail:001",
            vec![
                D::PackVersionDigestUnstated,
                D::UnevaluatedCheckHiddenBehindGreenSummary,
                D::ReviewPackMatrixStale,
            ],
            vec![
                review_pack_invalidation_record_base_revision_drift_clean(),
                review_pack_invalidation_record_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::AiReviewPanel,
            "AI-review owner",
            "The AI review panel resolves the head-revision drift and its current-packet rerun/compare binding while keeping the analyzed scope and the pack version the review ran under visible; a record widening a stale local estimate into fresh provider-authoritative mergeability and a resolution-form gap on a compare are caught before a green summary can reintroduce an authoritative reading, so AI policy compliance never stays green under an undisclosed or drifted pack version",
            "review-pack:m5-ai-review:001",
            vec![
                D::PackVersionDigestUnstated,
                D::PackVersionOrDigestDropped,
                D::ReviewPackMatrixStale,
            ],
            vec![review_pack_invalidation_record_head_revision_drift_clean(), review_pack_invalidation_record_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::SupportExportPacket,
            "Support owner",
            "Support resolves the pack-version drift while keeping the pack version / digest and comment / summary template attribution bound to the export, and reports the preserved-draft-evidence rerun/compare binding so draft-only notes and local evidence carry forward marked stale rather than discarded; a record that is a hand-copied per-entry assumption and a compare on an unclassified rerun/compare binding degrade honestly so pack identity, the invalidation cause, and evaluator lineage are never dropped on export or reopen",
            "review-pack:m5-support:001",
            vec![
                D::ParityStateUnstated,
                D::PackFreshnessUnstated,
                D::ReviewPackMatrixStale,
            ],
            vec![
                review_pack_invalidation_record_pack_version_drift_clean(),
                review_pack_invalidation_record_unbound(),
            ],
            vec![comparison_review_pack_rerun_compare_unclassified()],
        ),
        base_row(
            C::ReviewPackSummary,
            "Review-pack-summary owner",
            "The review-pack summary resolves the worktree-scope drift and the stale transition it forces — stale-pack, partial-scope, or slice-omitted — bound to the registry so a stale or partially evaluated pack can no longer read as a fresh, full-coverage review result; a record that cannot name its invalidation cause is caught before it can silently drift",
            "review-pack:m5-review-pack-summary:001",
            vec![
                D::ParityStateUnstated,
                D::EvaluatorResultClassUnstated,
                D::ReviewPackMatrixStale,
            ],
            vec![
                review_pack_invalidation_record_worktree_scope_drift_clean(),
                review_pack_invalidation_record_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::LocalCiParityStrip,
            "Local-CI-parity owner",
            "The local-CI parity strip renders the same resolved review-pack-invalidation-record (environment-capsule drift) and review-pack-rerun-compare truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied table; the ci-only / not-evaluated-here / provider-unavailable label and the previous-versus-current compare binding stay inspectable off-renderer so a stale local parity estimate never reads as fresh provider-authoritative mergeability",
            "review-pack:m5-local-ci-parity:001",
            vec![
                D::EvaluatorResultClassUnstated,
                D::ParityStateUnstated,
                D::ReviewPackMatrixStale,
            ],
            vec![review_pack_invalidation_record_environment_capsule_drift_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::ProviderHandoff,
            "Provider-handoff owner",
            "The provider handoff feed carries the same resolved review-pack-invalidation-record (pack-digest drift) and review-pack-rerun-compare truth, so a dropped pack version / digest, an unnamed invalidation cause, a stale estimate masquerading as fresh provider-authoritative, or a stale-pack compare shown as current is visible in evidence — a base/head change, a pack-version-and-digest change, or an environment-capsule change — rather than hidden behind a green summary that keeps queue eligibility or approval validity green after a material drift",
            "review-pack:m5-provider-handoff:001",
            vec![
                D::ParityStateUnstated,
                D::UnevaluatedCheckHiddenBehindGreenSummary,
                D::ReviewPackMatrixStale,
            ],
            vec![review_pack_invalidation_record_pack_digest_drift_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5ReviewPackInvalidationAndRerunCompareRegistriesGovernanceReview {
    M5ReviewPackInvalidationAndRerunCompareRegistriesGovernanceReview {
        review_pack_invalidation_record_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_review_pack_invalidation_record_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        review_pack_rerun_compare_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        review_pack_invalidation_record_or_downgrade_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5ReviewPackInvalidationAndRerunCompareRegistriesConsumerProjection {
    M5ReviewPackInvalidationAndRerunCompareRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5ReviewPackInvalidationAndRerunCompareRegistriesProofFreshness {
    M5ReviewPackInvalidationAndRerunCompareRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ReviewPackInvalidationAndRerunCompareRegistriesReleasePosture {
    M5ReviewPackInvalidationAndRerunCompareRegistriesReleasePosture {
        proof_packet_ref: M5_REVIEW_PACK_INVALIDATION_AND_RERUN_COMPARE_REGISTRIES_ARTIFACT_REF
            .to_owned(),
        line_audit_ref: M5_REVIEW_PACK_INVALIDATION_AND_RERUN_COMPARE_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_REVIEW_PACK_INVALIDATION_AND_RERUN_COMPARE_REGISTRIES_SCHEMA_REF,
        M5_REVIEW_PACK_INVALIDATION_AND_RERUN_COMPARE_REGISTRIES_DOC_REF,
        M5_REVIEW_PACK_MATRIX_SCHEMA_REF,
        M5_REVIEW_PACK_MATRIX_DOC_REF,
        M5_REVIEW_PACK_DOMAIN_SCHEMA_REF,
        M5_REVIEW_PACK_RESULT_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 review-pack-invalidation-record and review-pack-rerun-compare registries packet.
pub fn seeded_m5_review_pack_invalidation_and_rerun_compare_registries(
) -> M5ReviewPackInvalidationAndRerunCompareRegistriesPacket {
    M5ReviewPackInvalidationAndRerunCompareRegistriesPacket::new(
        M5ReviewPackInvalidationAndRerunCompareRegistriesPacketInput {
            packet_id: M5_REVIEW_PACK_INVALIDATION_AND_RERUN_COMPARE_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 review-pack invalidation-record and rerun-compare registries emitting one machine-readable review-pack-invalidation-record per stale transition — one typed record naming the exact invalidation cause: base-revision drift, head-revision drift, worktree-scope drift, review-pack version drift, review-pack content-digest drift, or environment-capsule drift — each bound to one pack identity with its evaluator lineage, base / head, worktree scope, and pack version / digest, so a review result never drops its pack version / digest or template attribution and no stale local parity estimate reads as fresh provider-authoritative mergeability, with canonical / accessible / audit resolution-form coverage, and a machine-readable review-pack-rerun-compare (previous-packet-binding, current-packet-binding, or preserved-draft-evidence-binding) that lets a rerun-review and compare action inspect what changed between the previous evaluator packet and the current base/head or pack revision while preserving draft-only notes and local evidence marked stale — surfacing partial-scope, slice-omitted, stale-pack, ci-only, or provider-unavailable rather than a green summary — so no review surface keeps queue eligibility, approval validity, or AI policy compliance green after a material pack / base / environment drift across review, AI-review, provider-handoff, and support / export surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5ReviewPackInvalidationAndRerunCompareRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the review-detail row is held at Beta pending review-pack-invalidation-record parity on every pack
/// version / digest; every row stays visible and every example stays honest.
pub fn seeded_m5_review_pack_invalidation_and_rerun_compare_registries_review_pack_invalidation_record_beta_narrowed(
) -> M5ReviewPackInvalidationAndRerunCompareRegistriesPacket {
    let mut packet = seeded_m5_review_pack_invalidation_and_rerun_compare_registries();
    packet.packet_id =
        "m5-review-pack-invalidation-and-rerun-compare-registries:review-pack-invalidation-record-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ReviewPackConsumerSurface::ReviewDetail)
        .expect("review-detail row present");
    row.qualification = M5ReviewPackQualificationClass::Beta;
    packet
}

/// Narrowed variant: the AI-review row is narrowed to Preview pending review-pack-rerun-compare parity on every
/// evaluator binding; every row stays visible and every example stays honest.
pub fn seeded_m5_review_pack_invalidation_and_rerun_compare_registries_review_pack_rerun_compare_preview_narrowed(
) -> M5ReviewPackInvalidationAndRerunCompareRegistriesPacket {
    let mut packet = seeded_m5_review_pack_invalidation_and_rerun_compare_registries();
    packet.packet_id =
        "m5-review-pack-invalidation-and-rerun-compare-registries:review-pack-rerun-compare-preview:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ReviewPackConsumerSurface::AiReviewPanel)
        .expect("AI-review row present");
    row.qualification = M5ReviewPackQualificationClass::Preview;
    packet
}
