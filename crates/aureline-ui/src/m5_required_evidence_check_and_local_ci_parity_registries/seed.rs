//! Canonical seed builders for the M5 required-evidence-check and local-ci-parity registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean required-evidence-check and local-CI-parity entries
//! are built so the one typed required-evidence-check row resolving per required check with its distinct
//! evidence-check state (required, optional, skipped, suppressed, timed out, ci-only, not evaluated here, or
//! provider unavailable), strips never widening a local parity estimate into provider-authoritative mergeability,
//! ci-only / not-evaluated-here / provider-unavailable states never hidden behind a green summary, the canonical /
//! accessible / audit resolution forms, and the complete capability-difference compare object naming the
//! environment / secrets / provider-only-merge-simulation deltas are proven across the review-detail, AI-review,
//! review-pack-summary, local-CI-parity, provider-handoff, and support surfaces without any hand-copied per-check
//! assumption, estimate-shown-as-authoritative, incomplete object, hidden unevaluated state, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_REQUIRED_EVIDENCE_CHECK_AND_LOCAL_CI_PARITY_REGISTRIES_PACKET_ID: &str =
    "m5-required-evidence-check-and-local-ci-parity-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn required_evidence_check(
    input: M5RequiredEvidenceCheckEntryResolutionInput,
) -> M5ResolvedRequiredEvidenceCheckEntry {
    resolve_required_evidence_check_entry(input)
        .expect("seed line-required_evidence_check entry resolves")
}

fn downgrade(input: M5LocalCiParityEntryResolutionInput) -> M5ResolvedLocalCiParityEntry {
    resolve_local_ci_parity_entry(input).expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5RequiredEvidenceCheckResolutionForm> {
    M5RequiredEvidenceCheckResolutionForm::ALL.to_vec()
}

// -- Clean line-required_evidence_check entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_required_evidence_check_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5ReviewPackRole,
    report_section: M5RequiredEvidenceCheckKind,
    surface_context: M5RequiredEvidenceCheckSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5RequiredEvidenceCheckEntryResolutionInput {
    M5RequiredEvidenceCheckEntryResolutionInput {
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

fn required_evidence_check_required_clean() -> M5ResolvedRequiredEvidenceCheckEntry {
    required_evidence_check(clean_required_evidence_check_base(
        "required_evidence_check:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.required_evidence_check.core_team_canary",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5RequiredEvidenceCheckKind::Required,
        M5RequiredEvidenceCheckSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn required_evidence_check_optional_clean() -> M5ResolvedRequiredEvidenceCheckEntry {
    required_evidence_check(clean_required_evidence_check_base(
        "required_evidence_check:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.required_evidence_check.optional",
        M5ReviewPackRole::PackVersionAndDigestDisclosure,
        M5RequiredEvidenceCheckKind::Optional,
        M5RequiredEvidenceCheckSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn required_evidence_check_skipped_clean() -> M5ResolvedRequiredEvidenceCheckEntry {
    required_evidence_check(clean_required_evidence_check_base(
        "required_evidence_check:program-governance:extension-author",
        "launch.line.extension-author",
        "line.required_evidence_check.skipped",
        M5ReviewPackRole::OwnerProvenanceDisclosure,
        M5RequiredEvidenceCheckKind::Skipped,
        M5RequiredEvidenceCheckSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn required_evidence_check_suppressed_clean() -> M5ResolvedRequiredEvidenceCheckEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_required_evidence_check_base(
        "required_evidence_check:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.required_evidence_check.suppressed",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5RequiredEvidenceCheckKind::Suppressed,
        M5RequiredEvidenceCheckSurfaceContext::ExecutiveSteeringSurface,
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
    required_evidence_check(base)
}

fn required_evidence_check_ci_only_clean() -> M5ResolvedRequiredEvidenceCheckEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_required_evidence_check_base(
        "required_evidence_check:support:public-preview",
        "launch.line.public-preview",
        "line.required_evidence_check.ci_only",
        M5ReviewPackRole::PackVersionAndDigestDisclosure,
        M5RequiredEvidenceCheckKind::CiOnly,
        M5RequiredEvidenceCheckSurfaceContext::SupportOrExportForm,
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
    required_evidence_check(base)
}

fn required_evidence_check_not_evaluated_here_clean() -> M5ResolvedRequiredEvidenceCheckEntry {
    required_evidence_check(clean_required_evidence_check_base(
        "required_evidence_check:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.required_evidence_check.not_evaluated_here",
        M5ReviewPackRole::LocalVersusProviderParityDisclosure,
        M5RequiredEvidenceCheckKind::NotEvaluatedHere,
        M5RequiredEvidenceCheckSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

fn required_evidence_check_timed_out_clean() -> M5ResolvedRequiredEvidenceCheckEntry {
    required_evidence_check(clean_required_evidence_check_base(
        "required_evidence_check:shiproom:timed-out-scanner",
        "launch.line.core-team-canary",
        "line.required_evidence_check.timed_out",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5RequiredEvidenceCheckKind::TimedOut,
        M5RequiredEvidenceCheckSurfaceContext::ShiproomSurface,
        "repo.rows.timed-out-scanner-journeys",
        "bundle.ids.timed-out-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.timed-out-scanner",
        "rollback.target.timed-out-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn required_evidence_check_provider_unavailable_clean() -> M5ResolvedRequiredEvidenceCheckEntry {
    required_evidence_check(clean_required_evidence_check_base(
        "required_evidence_check:provider-handoff:provider-unavailable",
        "launch.line.public-preview",
        "line.required_evidence_check.provider_unavailable",
        M5ReviewPackRole::LocalVersusProviderParityDisclosure,
        M5RequiredEvidenceCheckKind::ProviderUnavailable,
        M5RequiredEvidenceCheckSurfaceContext::SupportOrExportForm,
        "repo.rows.provider-unavailable-journeys",
        "bundle.ids.provider-unavailable-0007",
        "install.topology.public-preview-ring",
        "toolchain.envelope.pinned-public",
        "known-limits.published.provider-unavailable",
        "rollback.target.provider-unavailable-previous-stable",
        "diagnostics.posture.public-telemetry",
    ))
}

// -- Degraded line-required_evidence_check entries ---------------------------------------------------------

/// Degraded required_evidence_check entry: the resolved required_evidence_check object is incomplete — the bundle IDs are unstated.
fn required_evidence_check_object_incomplete() -> M5ResolvedRequiredEvidenceCheckEntry {
    let mut base = clean_required_evidence_check_base(
        "required_evidence_check:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.required_evidence_check.core_team_canary",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5RequiredEvidenceCheckKind::Required,
        M5RequiredEvidenceCheckSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    required_evidence_check(base)
}

/// Degraded required_evidence_check entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn required_evidence_check_widen_fold() -> M5ResolvedRequiredEvidenceCheckEntry {
    let mut base = clean_required_evidence_check_base(
        "required_evidence_check:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.required_evidence_check.optional",
        M5ReviewPackRole::PackVersionAndDigestDisclosure,
        M5RequiredEvidenceCheckKind::Optional,
        M5RequiredEvidenceCheckSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    required_evidence_check(base)
}

/// Degraded required_evidence_check entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn required_evidence_check_unbound() -> M5ResolvedRequiredEvidenceCheckEntry {
    let mut base = clean_required_evidence_check_base(
        "required_evidence_check:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.required_evidence_check.not_evaluated_here",
        M5ReviewPackRole::LocalVersusProviderParityDisclosure,
        M5RequiredEvidenceCheckKind::NotEvaluatedHere,
        M5RequiredEvidenceCheckSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    required_evidence_check(base)
}

/// Degraded required_evidence_check entry: the canonical registry token name is unstated.
fn required_evidence_check_token_unstated() -> M5ResolvedRequiredEvidenceCheckEntry {
    let mut base = clean_required_evidence_check_base(
        "required_evidence_check:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5ReviewPackRole::OwnerProvenanceDisclosure,
        M5RequiredEvidenceCheckKind::Skipped,
        M5RequiredEvidenceCheckSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    required_evidence_check(base)
}

// -- Clean line-downgrade-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_downgrade_base(
    entry_id: &str,
    comparison_ref: &str,
    token_name: &str,
    semantic_role: M5ReviewPackRole,
    comparison_scope: M5LocalCiParityScope,
    surface_context: M5RequiredEvidenceCheckSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5LocalCiParityEntryResolutionInput {
    M5LocalCiParityEntryResolutionInput {
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
        keeps_local_ci_parity_visible: true,
        comparison_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn downgrade_dogfood_ring_clean() -> M5ResolvedLocalCiParityEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5LocalCiParityScope::LocalParityEstimateBinding,
        M5RequiredEvidenceCheckSurfaceContext::ShiproomSurface,
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

fn downgrade_rehearsal_currency_clean() -> M5ResolvedLocalCiParityEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.skipped",
        M5ReviewPackRole::OwnerProvenanceDisclosure,
        M5LocalCiParityScope::ProviderAuthoritativeBinding,
        M5RequiredEvidenceCheckSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedLocalCiParityEntry {
    // A capability-difference compare packet names the environment, secrets, and provider-only merge-simulation
    // deltas between the local parity estimate and the provider-authoritative state (AC3), so a local estimate is
    // never widened into queue-eligible mergeability without explicit provider evidence.
    downgrade(clean_downgrade_base(
        "downgrade:release-center:capability-difference-compare",
        "launch.line.certified-journey",
        "line.local_ci_parity.capability_difference",
        M5ReviewPackRole::LocalVersusProviderParityDisclosure,
        M5LocalCiParityScope::CapabilityDifferenceBinding,
        M5RequiredEvidenceCheckSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "capability-difference.environment-and-secrets-and-provider-only-merge-simulation-deltas",
        "rollback.target.ref.certified-journey",
        "capability-delta.runner-class-and-service-dependencies-and-branch-protections",
        "readiness.signoff.local-estimate-distinct-from-provider-authoritative",
        "support.language.local-parity-estimate-not-provider-authoritative",
        "widening.revision.0007",
    ))
}

// -- Degraded line-downgrade-packet entries ----------------------------------------------------

/// Degraded downgrade entry: the downgrade would run partner / public support language ahead of line proof — a
/// support-language reference present but not bound to line proof reads as trustworthy when the line proof
/// does not yet back it.
fn downgrade_support_ahead() -> M5ResolvedLocalCiParityEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.ci_only",
        M5ReviewPackRole::PackVersionAndDigestDisclosure,
        M5LocalCiParityScope::LocalParityEstimateBinding,
        M5RequiredEvidenceCheckSurfaceContext::ShiproomSurface,
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
fn downgrade_form_incomplete() -> M5ResolvedLocalCiParityEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.not_evaluated_here",
        M5ReviewPackRole::LocalVersusProviderParityDisclosure,
        M5LocalCiParityScope::CapabilityDifferenceBinding,
        M5RequiredEvidenceCheckSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5RequiredEvidenceCheckResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn comparison_local_ci_parity_unclassified() -> M5ResolvedLocalCiParityEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.suppressed",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5LocalCiParityScope::LocalCiParityUnclassified,
        M5RequiredEvidenceCheckSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5RequiredEvidenceCheckAndLocalCiParityRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5ReviewPackDowngradeTrigger>,
    required_evidence_check_entries: Vec<M5ResolvedRequiredEvidenceCheckEntry>,
    local_ci_parity_entries: Vec<M5ResolvedLocalCiParityEntry>,
) -> M5RequiredEvidenceCheckAndLocalCiParityRegistriesRow {
    M5RequiredEvidenceCheckAndLocalCiParityRegistriesRow {
        consumer_surface,
        qualification: M5ReviewPackQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        capture_lifecycle_stages: M5ReviewPackClassificationStage::ALL.to_vec(),
        required_labels: M5ReviewPackRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5ReviewPackAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5RequiredEvidenceCheckAnatomyPart::ALL.to_vec(),
        export_fields: M5RequiredEvidenceCheckExportField::ALL.to_vec(),
        downgrade_triggers,
        required_evidence_check_entries,
        local_ci_parity_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_REQUIRED_EVIDENCE_CHECK_AND_LOCAL_CI_PARITY_REGISTRIES_SCHEMA_REF,
            M5_REVIEW_PACK_RESULT_DOMAIN_SCHEMA_REF,
            M5_LOCAL_CI_PARITY_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_local_ci_parity_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5RequiredEvidenceCheckAndLocalCiParityRegistriesRow> {
    use M5ReviewPackConsumerSurface as C;
    use M5ReviewPackDowngradeTrigger as D;

    vec![
        base_row(
            C::ReviewDetail,
            "Review-detail owner",
            "Review detail resolves each required check to one typed required-evidence-check row — the check identity, its evidence-check state (required, optional, skipped, suppressed, timed out, ci-only, not evaluated here, or provider unavailable), and whether Aureline evaluated it locally, imported it, or could not evaluate it here — from the shared registry and proves the local-parity-estimate compare for that check; a row that collapses an unevaluated check into a pass and a strip that would let a local parity estimate read as provider-authoritative mergeability degrade honestly instead of leaving a ci-only or not-evaluated-here check to read as a green success",
            "review-pack:m5-review-detail:001",
            vec![
                D::PackVersionDigestUnstated,
                D::UnevaluatedCheckHiddenBehindGreenSummary,
                D::ReviewPackMatrixStale,
            ],
            vec![
                required_evidence_check_required_clean(),
                required_evidence_check_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::AiReviewPanel,
            "AI-review owner",
            "The AI review panel resolves the provider-authoritative binding and the capability-difference compare while keeping each required check's evidence-check state and evaluation origin visible; a strip widening a local estimate into provider-authoritative mergeability and a resolution-form gap on a compare are caught before a green summary can reintroduce an authoritative reading, and an AI review can never present a ci-only or not-evaluated-here check as satisfied",
            "review-pack:m5-ai-review:001",
            vec![
                D::PackVersionDigestUnstated,
                D::PackVersionOrDigestDropped,
                D::ReviewPackMatrixStale,
            ],
            vec![required_evidence_check_optional_clean(), required_evidence_check_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::SupportExportPacket,
            "Support owner",
            "Support resolves each required check's evidence-check state and evaluation origin while keeping the capability-difference compare bound to the export, and reports the capability-difference binding; a row that is a hand-copied per-check assumption and a compare on an unclassified parity binding degrade honestly so the evidence-check state and the environment / secrets / provider-only-merge-simulation deltas are never dropped on export or reopen",
            "review-pack:m5-support:001",
            vec![
                D::ParityStateUnstated,
                D::PackFreshnessUnstated,
                D::ReviewPackMatrixStale,
            ],
            vec![
                required_evidence_check_suppressed_clean(),
                required_evidence_check_unbound(),
            ],
            vec![comparison_local_ci_parity_unclassified()],
        ),
        base_row(
            C::ReviewPackSummary,
            "Review-pack-summary owner",
            "The review-pack summary resolves the evidence-check state and the parity binding — local-parity-estimate, provider-authoritative, or capability-difference — bound to the registry so a skipped, suppressed, timed-out, ci-only, not-evaluated-here, or provider-unavailable check can no longer read as a fresh, full-coverage green result; an unstated evidence-check state on a row is caught before it can drift",
            "review-pack:m5-review-pack-summary:001",
            vec![
                D::ParityStateUnstated,
                D::EvaluatorResultClassUnstated,
                D::ReviewPackMatrixStale,
            ],
            vec![
                required_evidence_check_skipped_clean(),
                required_evidence_check_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::LocalCiParityStrip,
            "Local-CI-parity owner",
            "The local-CI parity strip renders the same resolved required-evidence-check and local-CI-parity truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied table; the ci-only / not-evaluated-here / provider-unavailable state and the capability-difference compare — environment, secrets, runner class, service dependencies, branch protections, or provider-only merge simulation — stay inspectable off-renderer so a local parity estimate never reads as provider-authoritative mergeability",
            "review-pack:m5-local-ci-parity:001",
            vec![
                D::EvaluatorResultClassUnstated,
                D::ParityStateUnstated,
                D::ReviewPackMatrixStale,
            ],
            vec![
                required_evidence_check_not_evaluated_here_clean(),
                required_evidence_check_timed_out_clean(),
            ],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::ProviderHandoff,
            "Provider-handoff owner",
            "The provider handoff feed carries the same resolved required-evidence-check and local-CI-parity truth, so a dropped evidence-check state, an unstated evaluation origin, a local estimate masquerading as provider-authoritative, or a provider-unavailable check shown as current is visible in evidence — a local-parity-estimate binding, a provider-authoritative binding, or a capability-difference binding — rather than hidden behind a green summary",
            "review-pack:m5-provider-handoff:001",
            vec![
                D::ParityStateUnstated,
                D::UnevaluatedCheckHiddenBehindGreenSummary,
                D::ReviewPackMatrixStale,
            ],
            vec![
                required_evidence_check_ci_only_clean(),
                required_evidence_check_provider_unavailable_clean(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5RequiredEvidenceCheckAndLocalCiParityRegistriesGovernanceReview {
    M5RequiredEvidenceCheckAndLocalCiParityRegistriesGovernanceReview {
        required_evidence_check_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_required_evidence_check_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        local_ci_parity_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        required_evidence_check_or_downgrade_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5RequiredEvidenceCheckAndLocalCiParityRegistriesConsumerProjection {
    M5RequiredEvidenceCheckAndLocalCiParityRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5RequiredEvidenceCheckAndLocalCiParityRegistriesProofFreshness {
    M5RequiredEvidenceCheckAndLocalCiParityRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5RequiredEvidenceCheckAndLocalCiParityRegistriesReleasePosture {
    M5RequiredEvidenceCheckAndLocalCiParityRegistriesReleasePosture {
        proof_packet_ref: M5_REQUIRED_EVIDENCE_CHECK_AND_LOCAL_CI_PARITY_REGISTRIES_ARTIFACT_REF
            .to_owned(),
        line_audit_ref: M5_REQUIRED_EVIDENCE_CHECK_AND_LOCAL_CI_PARITY_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_REQUIRED_EVIDENCE_CHECK_AND_LOCAL_CI_PARITY_REGISTRIES_SCHEMA_REF,
        M5_REQUIRED_EVIDENCE_CHECK_AND_LOCAL_CI_PARITY_REGISTRIES_DOC_REF,
        M5_REVIEW_PACK_MATRIX_SCHEMA_REF,
        M5_REVIEW_PACK_MATRIX_DOC_REF,
        M5_REVIEW_PACK_RESULT_DOMAIN_SCHEMA_REF,
        M5_LOCAL_CI_PARITY_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 required-evidence-check and local-ci-parity registries packet.
pub fn seeded_m5_required_evidence_check_and_local_ci_parity_registries(
) -> M5RequiredEvidenceCheckAndLocalCiParityRegistriesPacket {
    M5RequiredEvidenceCheckAndLocalCiParityRegistriesPacket::new(
        M5RequiredEvidenceCheckAndLocalCiParityRegistriesPacketInput {
            packet_id: M5_REQUIRED_EVIDENCE_CHECK_AND_LOCAL_CI_PARITY_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 required-evidence-check and local-CI-parity registries emitting one machine-readable required-evidence-check row per required check — a must-run test, scanner, docs / migration note, incident link, or rollout note — carrying its evidence-check state (required, optional, skipped, suppressed, timed out, ci-only, not evaluated here, or provider unavailable) and whether Aureline evaluated it locally, imported it, or could not evaluate it here, so the eight states never collapse into one success / failure bucket and no local parity estimate reads as provider-authoritative or queue-eligible mergeability, with canonical / accessible / audit resolution-form coverage, and a machine-readable local-CI-parity strip (local-parity-estimate-binding, provider-authoritative-binding, or capability-difference-binding) that compares the local parity estimate against the provider-authoritative state and names the capability difference — environment, secrets, runner class, service dependencies, branch protections, or provider-only merge simulation — rather than implying mergeability from one green summary across review, AI-review, provider-handoff, and support / export surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5RequiredEvidenceCheckAndLocalCiParityRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the review-detail row is held at Beta pending required-evidence-check parity on every pack
/// version / digest; every row stays visible and every example stays honest.
pub fn seeded_m5_required_evidence_check_and_local_ci_parity_registries_required_evidence_check_beta_narrowed(
) -> M5RequiredEvidenceCheckAndLocalCiParityRegistriesPacket {
    let mut packet = seeded_m5_required_evidence_check_and_local_ci_parity_registries();
    packet.packet_id =
        "m5-required-evidence-check-and-local-ci-parity-registries:required-evidence-check-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ReviewPackConsumerSurface::ReviewDetail)
        .expect("review-detail row present");
    row.qualification = M5ReviewPackQualificationClass::Beta;
    packet
}

/// Narrowed variant: the AI-review row is narrowed to Preview pending local-ci-parity parity on every
/// evaluator binding; every row stays visible and every example stays honest.
pub fn seeded_m5_required_evidence_check_and_local_ci_parity_registries_local_ci_parity_preview_narrowed(
) -> M5RequiredEvidenceCheckAndLocalCiParityRegistriesPacket {
    let mut packet = seeded_m5_required_evidence_check_and_local_ci_parity_registries();
    packet.packet_id =
        "m5-required-evidence-check-and-local-ci-parity-registries:local-ci-parity-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ReviewPackConsumerSurface::AiReviewPanel)
        .expect("AI-review row present");
    row.qualification = M5ReviewPackQualificationClass::Preview;
    packet
}
