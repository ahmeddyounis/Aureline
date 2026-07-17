//! Canonical seed builders for the M5 ai-policy-hook and ai-policy-result registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean AI-policy-hook and AI-policy-result entries
//! are built so the one typed AI review policy hook resolving per run — its allowed analyzers, severity
//! thresholds, suppression classes, and mandatory citation requirements bound to the active review-pack
//! version / digest — an experimental or policy-downgraded run never read as a full pack-compliant review,
//! a prior finding marked rerun-required or stale after a pack change rather than kept as current evidence,
//! the canonical / accessible / audit resolution forms, and the complete analyzer-result-class /
//! pack-version-and-digest / rerun-staleness AI-policy-result object are proven across the review-detail,
//! AI-review, review-pack-summary, local-CI-parity, provider-handoff, and support surfaces without any
//! hand-copied per-run assumption, divergent-pack-version applied silently, incomplete object,
//! undisclosed experimental / policy-downgraded result, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_AI_POLICY_HOOK_AND_RESULT_REGISTRIES_PACKET_ID: &str =
    "m5-ai-policy-hook-and-result-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn ai_policy_hook(input: M5AiPolicyHookEntryResolutionInput) -> M5ResolvedAiPolicyHookEntry {
    resolve_ai_policy_hook_entry(input).expect("seed line-ai_policy_hook entry resolves")
}

fn downgrade(input: M5AiPolicyResultEntryResolutionInput) -> M5ResolvedAiPolicyResultEntry {
    resolve_ai_policy_result_entry(input).expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5AiPolicyHookResolutionForm> {
    M5AiPolicyHookResolutionForm::ALL.to_vec()
}

// -- Clean line-ai_policy_hook entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_ai_policy_hook_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5ReviewPackRole,
    report_section: M5AiPolicyHookKind,
    surface_context: M5AiPolicyHookSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5AiPolicyHookEntryResolutionInput {
    M5AiPolicyHookEntryResolutionInput {
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

fn ai_policy_hook_allowed_analyzer_clean() -> M5ResolvedAiPolicyHookEntry {
    ai_policy_hook(clean_ai_policy_hook_base(
        "ai_policy_hook:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.ai_policy_hook.core_team_canary",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5AiPolicyHookKind::AllowedAnalyzer,
        M5AiPolicyHookSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn ai_policy_hook_severity_threshold_clean() -> M5ResolvedAiPolicyHookEntry {
    ai_policy_hook(clean_ai_policy_hook_base(
        "ai_policy_hook:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.ai_policy_hook.severity_threshold",
        M5ReviewPackRole::PackVersionAndDigestDisclosure,
        M5AiPolicyHookKind::SeverityThreshold,
        M5AiPolicyHookSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn ai_policy_hook_suppression_class_clean() -> M5ResolvedAiPolicyHookEntry {
    ai_policy_hook(clean_ai_policy_hook_base(
        "ai_policy_hook:program-governance:extension-author",
        "launch.line.extension-author",
        "line.ai_policy_hook.suppression_class",
        M5ReviewPackRole::OwnerProvenanceDisclosure,
        M5AiPolicyHookKind::SuppressionClass,
        M5AiPolicyHookSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn ai_policy_hook_policy_downgraded_clean() -> M5ResolvedAiPolicyHookEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_ai_policy_hook_base(
        "ai_policy_hook:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.ai_policy_hook.policy_downgraded",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5AiPolicyHookKind::PolicyDowngraded,
        M5AiPolicyHookSurfaceContext::ExecutiveSteeringSurface,
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
    ai_policy_hook(base)
}

fn ai_policy_hook_experimental_analyzer_clean() -> M5ResolvedAiPolicyHookEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_ai_policy_hook_base(
        "ai_policy_hook:support:public-preview",
        "launch.line.public-preview",
        "line.ai_policy_hook.experimental_analyzer",
        M5ReviewPackRole::PackVersionAndDigestDisclosure,
        M5AiPolicyHookKind::ExperimentalAnalyzer,
        M5AiPolicyHookSurfaceContext::SupportOrExportForm,
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
    ai_policy_hook(base)
}

fn ai_policy_hook_mandatory_citation_clean() -> M5ResolvedAiPolicyHookEntry {
    ai_policy_hook(clean_ai_policy_hook_base(
        "ai_policy_hook:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.ai_policy_hook.mandatory_citation",
        M5ReviewPackRole::LocalVersusProviderParityDisclosure,
        M5AiPolicyHookKind::MandatoryCitation,
        M5AiPolicyHookSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded line-ai_policy_hook entries ---------------------------------------------------------

/// Degraded ai_policy_hook entry: the resolved ai_policy_hook object is incomplete — the bundle IDs are unstated.
fn ai_policy_hook_object_incomplete() -> M5ResolvedAiPolicyHookEntry {
    let mut base = clean_ai_policy_hook_base(
        "ai_policy_hook:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.ai_policy_hook.core_team_canary",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5AiPolicyHookKind::AllowedAnalyzer,
        M5AiPolicyHookSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    ai_policy_hook(base)
}

/// Degraded ai_policy_hook entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn ai_policy_hook_widen_fold() -> M5ResolvedAiPolicyHookEntry {
    let mut base = clean_ai_policy_hook_base(
        "ai_policy_hook:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.ai_policy_hook.severity_threshold",
        M5ReviewPackRole::PackVersionAndDigestDisclosure,
        M5AiPolicyHookKind::SeverityThreshold,
        M5AiPolicyHookSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    ai_policy_hook(base)
}

/// Degraded ai_policy_hook entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn ai_policy_hook_unbound() -> M5ResolvedAiPolicyHookEntry {
    let mut base = clean_ai_policy_hook_base(
        "ai_policy_hook:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.ai_policy_hook.mandatory_citation",
        M5ReviewPackRole::LocalVersusProviderParityDisclosure,
        M5AiPolicyHookKind::MandatoryCitation,
        M5AiPolicyHookSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    ai_policy_hook(base)
}

/// Degraded ai_policy_hook entry: the canonical registry token name is unstated.
fn ai_policy_hook_token_unstated() -> M5ResolvedAiPolicyHookEntry {
    let mut base = clean_ai_policy_hook_base(
        "ai_policy_hook:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5ReviewPackRole::OwnerProvenanceDisclosure,
        M5AiPolicyHookKind::SuppressionClass,
        M5AiPolicyHookSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    ai_policy_hook(base)
}

// -- Clean line-downgrade-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_downgrade_base(
    entry_id: &str,
    comparison_ref: &str,
    token_name: &str,
    semantic_role: M5ReviewPackRole,
    comparison_scope: M5AiPolicyResultScope,
    surface_context: M5AiPolicyHookSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5AiPolicyResultEntryResolutionInput {
    M5AiPolicyResultEntryResolutionInput {
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
        keeps_ai_policy_result_visible: true,
        comparison_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn downgrade_dogfood_ring_clean() -> M5ResolvedAiPolicyResultEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5AiPolicyResultScope::AnalyzerResultClassBinding,
        M5AiPolicyHookSurfaceContext::ShiproomSurface,
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

fn downgrade_rehearsal_currency_clean() -> M5ResolvedAiPolicyResultEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.suppression_class",
        M5ReviewPackRole::OwnerProvenanceDisclosure,
        M5AiPolicyResultScope::PackVersionDigestBinding,
        M5AiPolicyHookSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedAiPolicyResultEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.mandatory_citation",
        M5ReviewPackRole::LocalVersusProviderParityDisclosure,
        M5AiPolicyResultScope::RerunStalenessBinding,
        M5AiPolicyHookSurfaceContext::ReleaseCenterSurface,
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
fn downgrade_support_ahead() -> M5ResolvedAiPolicyResultEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.experimental_analyzer",
        M5ReviewPackRole::PackVersionAndDigestDisclosure,
        M5AiPolicyResultScope::AnalyzerResultClassBinding,
        M5AiPolicyHookSurfaceContext::ShiproomSurface,
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
fn downgrade_form_incomplete() -> M5ResolvedAiPolicyResultEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.mandatory_citation",
        M5ReviewPackRole::LocalVersusProviderParityDisclosure,
        M5AiPolicyResultScope::RerunStalenessBinding,
        M5AiPolicyHookSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5AiPolicyHookResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn comparison_ai_policy_result_unclassified() -> M5ResolvedAiPolicyResultEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.policy_downgraded",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5AiPolicyResultScope::AiPolicyResultUnclassified,
        M5AiPolicyHookSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5AiPolicyHookAndResultRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5ReviewPackDowngradeTrigger>,
    ai_policy_hook_entries: Vec<M5ResolvedAiPolicyHookEntry>,
    ai_policy_result_entries: Vec<M5ResolvedAiPolicyResultEntry>,
) -> M5AiPolicyHookAndResultRegistriesRow {
    M5AiPolicyHookAndResultRegistriesRow {
        consumer_surface,
        qualification: M5ReviewPackQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        capture_lifecycle_stages: M5ReviewPackClassificationStage::ALL.to_vec(),
        required_labels: M5ReviewPackRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5ReviewPackAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5AiPolicyHookAnatomyPart::ALL.to_vec(),
        export_fields: M5AiPolicyHookExportField::ALL.to_vec(),
        downgrade_triggers,
        ai_policy_hook_entries,
        ai_policy_result_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_AI_POLICY_HOOK_AND_RESULT_REGISTRIES_SCHEMA_REF,
            M5_AI_POLICY_HOOK_DOMAIN_SCHEMA_REF,
            M5_AI_POLICY_RESULT_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_ai_policy_result_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5AiPolicyHookAndResultRegistriesRow> {
    use M5ReviewPackConsumerSurface as C;
    use M5ReviewPackDowngradeTrigger as D;

    vec![
        base_row(
            C::ReviewDetail,
            "Review-detail owner",
            "Review detail resolves the active review pack to one typed AI review policy hook — the allowed analyzers, the severity thresholds, the suppression classes, and the mandatory citation requirements — bound to the same review-pack version and content digest and evaluator lineage as human, local, and CI review, and proves the analyzer-result-class binding for that run (full, experimental, or policy-downgraded); a hook that cannot name the pack version it resolved through and a result that would let an experimental or policy-downgraded run read as a full, pack-compliant review degrade honestly instead of applying a suppression class or severity threshold from a different or stale pack revision",
            "review-pack:m5-review-detail:001",
            vec![
                D::PackVersionDigestUnstated,
                D::UnevaluatedCheckHiddenBehindGreenSummary,
                D::ReviewPackMatrixStale,
            ],
            vec![
                ai_policy_hook_allowed_analyzer_clean(),
                ai_policy_hook_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::AiReviewPanel,
            "AI-review owner",
            "The AI review panel resolves the pack-version-and-digest binding and the analyzer-result-class result while keeping the active review-pack version / digest, the analyzer class, and whether the result is full, experimental, or policy-downgraded visible; a hook operating with a narrower or different capability set than the declared pack and a resolution-form gap on a result are caught before a green summary can present the run as full pack-compliant evidence, and an AI review can never run under an undisclosed or divergent pack version",
            "review-pack:m5-ai-review:001",
            vec![
                D::PackVersionDigestUnstated,
                D::PackVersionOrDigestDropped,
                D::ReviewPackMatrixStale,
            ],
            vec![ai_policy_hook_severity_threshold_clean(), ai_policy_hook_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::SupportExportPacket,
            "Support owner",
            "Support resolves the analyzer result class while keeping the review-pack version / digest and the mandatory-citation attribution bound to the export, and reports the rerun / staleness result; a hook that is a hand-copied per-entry assumption and a result on an unclassified binding degrade honestly so the governing pack version and the analyzer lineage are never dropped on export or reopen",
            "review-pack:m5-support:001",
            vec![
                D::ParityStateUnstated,
                D::PackFreshnessUnstated,
                D::ReviewPackMatrixStale,
            ],
            vec![
                ai_policy_hook_policy_downgraded_clean(),
                ai_policy_hook_unbound(),
            ],
            vec![comparison_ai_policy_result_unclassified()],
        ),
        base_row(
            C::ReviewPackSummary,
            "Review-pack-summary owner",
            "The review-pack summary resolves the suppression classes and severity thresholds and the rerun-staleness result — current, rerun-required-after-pack-change, or stale-after-pack-change — bound to the registry so a prior AI finding can no longer read as current pack-compliant evidence after the pack changed; an unstated pack version / digest on a hook is caught before it can drift",
            "review-pack:m5-review-pack-summary:001",
            vec![
                D::ParityStateUnstated,
                D::EvaluatorResultClassUnstated,
                D::ReviewPackMatrixStale,
            ],
            vec![
                ai_policy_hook_suppression_class_clean(),
                ai_policy_hook_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::LocalCiParityStrip,
            "Local-CI-parity owner",
            "The local-CI parity strip renders the same resolved AI-policy-hook and AI-policy-result truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied table; the experimental-analyzer / policy-downgraded label and the analyzer-result-class binding stay inspectable off-renderer so an AI run under a narrower capability set never reads as a full, pack-authoritative review",
            "review-pack:m5-local-ci-parity:001",
            vec![
                D::EvaluatorResultClassUnstated,
                D::ParityStateUnstated,
                D::ReviewPackMatrixStale,
            ],
            vec![ai_policy_hook_mandatory_citation_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::ProviderHandoff,
            "Provider-handoff owner",
            "The provider handoff feed carries the same resolved AI-policy-hook and AI-policy-result truth, so a dropped review-pack version / digest, an undisclosed divergent pack version, an experimental or policy-downgraded run shown as full, or a stale-after-pack-change finding shown as current is visible in evidence — an analyzer-result-class change, a pack-version-and-digest change, or a rerun-staleness change — rather than hidden behind a green summary",
            "review-pack:m5-provider-handoff:001",
            vec![
                D::ParityStateUnstated,
                D::UnevaluatedCheckHiddenBehindGreenSummary,
                D::ReviewPackMatrixStale,
            ],
            vec![ai_policy_hook_experimental_analyzer_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5AiPolicyHookAndResultRegistriesGovernanceReview {
    M5AiPolicyHookAndResultRegistriesGovernanceReview {
        ai_policy_hook_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_ai_policy_hook_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        ai_policy_result_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        ai_policy_hook_or_downgrade_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5AiPolicyHookAndResultRegistriesConsumerProjection {
    M5AiPolicyHookAndResultRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5AiPolicyHookAndResultRegistriesProofFreshness {
    M5AiPolicyHookAndResultRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5AiPolicyHookAndResultRegistriesReleasePosture {
    M5AiPolicyHookAndResultRegistriesReleasePosture {
        proof_packet_ref: M5_AI_POLICY_HOOK_AND_RESULT_REGISTRIES_ARTIFACT_REF.to_owned(),
        line_audit_ref: M5_AI_POLICY_HOOK_AND_RESULT_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_AI_POLICY_HOOK_AND_RESULT_REGISTRIES_SCHEMA_REF,
        M5_AI_POLICY_HOOK_AND_RESULT_REGISTRIES_DOC_REF,
        M5_REVIEW_PACK_MATRIX_SCHEMA_REF,
        M5_REVIEW_PACK_MATRIX_DOC_REF,
        M5_AI_POLICY_HOOK_DOMAIN_SCHEMA_REF,
        M5_AI_POLICY_RESULT_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 ai-policy-hook and ai-policy-result registries packet.
pub fn seeded_m5_ai_policy_hook_and_result_registries() -> M5AiPolicyHookAndResultRegistriesPacket {
    M5AiPolicyHookAndResultRegistriesPacket::new(
        M5AiPolicyHookAndResultRegistriesPacketInput {
            packet_id: M5_AI_POLICY_HOOK_AND_RESULT_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 AI-review-policy-hook and AI-policy-result registries binding one machine-readable policy hook per AI review run to the active review pack — the allowed analyzers, the severity thresholds, the suppression classes, and the mandatory citation requirements, each resolving through the same review-pack version / content digest and evaluator lineage as human, local, and CI review — so an AI review never applies a suppression class, severity threshold, or citation expectation from a different or stale pack revision, with canonical / accessible / audit resolution-form coverage, and a machine-readable AI-policy-result (analyzer-result-class-binding, pack-version-and-digest-binding, or rerun-staleness-binding) that surfaces whether the run is full, experimental, or policy-downgraded and marks a prior finding rerun-required or stale after a pack change rather than preserving it as current pack-compliant evidence across review, AI-review, provider-handoff, and support / export surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5AiPolicyHookAndResultRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the review-detail row is held at Beta pending ai-policy-hook parity on every pack
/// version / digest; every row stays visible and every example stays honest.
pub fn seeded_m5_ai_policy_hook_and_result_registries_ai_policy_hook_beta_narrowed(
) -> M5AiPolicyHookAndResultRegistriesPacket {
    let mut packet = seeded_m5_ai_policy_hook_and_result_registries();
    packet.packet_id =
        "m5-ai-policy-hook-and-result-registries:ai-policy-hook-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ReviewPackConsumerSurface::ReviewDetail)
        .expect("review-detail row present");
    row.qualification = M5ReviewPackQualificationClass::Beta;
    packet
}

/// Narrowed variant: the AI-review row is narrowed to Preview pending ai-policy-result parity on every
/// evaluator binding; every row stays visible and every example stays honest.
pub fn seeded_m5_ai_policy_hook_and_result_registries_ai_policy_result_preview_narrowed(
) -> M5AiPolicyHookAndResultRegistriesPacket {
    let mut packet = seeded_m5_ai_policy_hook_and_result_registries();
    packet.packet_id =
        "m5-ai-policy-hook-and-result-registries:ai-policy-result-preview:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ReviewPackConsumerSurface::AiReviewPanel)
        .expect("AI-review row present");
    row.qualification = M5ReviewPackQualificationClass::Preview;
    packet
}
