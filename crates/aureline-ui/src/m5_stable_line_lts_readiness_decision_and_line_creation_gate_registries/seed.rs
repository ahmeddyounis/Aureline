//! Canonical seed builders for the M5 LTS-readiness-decision and line-creation-gate registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean LTS-readiness-decision and line-creation-gate entries
//! are built so the one typed LTS-readiness-decision object resolving per candidate line, lines never widening LTS
//! language without preserving rollback and support evidence, public support-window / continuity language never
//! running ahead of current evidence, the canonical / accessible / audit resolution forms, and the complete
//! line-identity / linked-decision / required-packet / gate-scope / narrowed-posture / active-reason /
//! last-revision line-creation-gate object are proven across the shiproom, release-center, executive-steering,
//! program-governance, diagnostics, and support surfaces without any hand-copied per-line assumption,
//! LTS-widen-without-green-packet, incomplete object, hidden gate, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_STABLE_LINE_LTS_READINESS_DECISION_LINE_CREATION_GATE_REGISTRIES_PACKET_ID: &str =
    "m5-stable-line-lts-readiness-decision-and-line-creation-gate-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn lts_readiness_decision(
    input: M5LtsReadinessDecisionEntryResolutionInput,
) -> M5ResolvedLtsReadinessDecisionEntry {
    resolve_lts_readiness_decision_entry(input)
        .expect("seed line-lts_readiness_decision entry resolves")
}

fn downgrade(input: M5LineCreationGateEntryResolutionInput) -> M5ResolvedLineCreationGateEntry {
    resolve_line_creation_gate_entry(input).expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5StableLineLtsReadinessDecisionResolutionForm> {
    M5StableLineLtsReadinessDecisionResolutionForm::ALL.to_vec()
}

// -- Clean line-lts_readiness_decision entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_lts_readiness_decision_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5StableLineProtectionRole,
    decision_section: M5LtsReadinessDecisionKind,
    surface_context: M5StableLineLtsReadinessDecisionSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5LtsReadinessDecisionEntryResolutionInput {
    M5LtsReadinessDecisionEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        line_binding_id: line_binding_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        decision_section,
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

fn lts_readiness_decision_backport_branch_posture_section_clean(
) -> M5ResolvedLtsReadinessDecisionEntry {
    lts_readiness_decision(clean_lts_readiness_decision_base(
        "lts_readiness_decision:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.lts_readiness_decision.core_team_canary",
        M5StableLineProtectionRole::SupportWindow,
        M5LtsReadinessDecisionKind::BackportBranchPostureSection,
        M5StableLineLtsReadinessDecisionSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn lts_readiness_decision_correction_line_health_section_clean(
) -> M5ResolvedLtsReadinessDecisionEntry {
    lts_readiness_decision(clean_lts_readiness_decision_base(
        "lts_readiness_decision:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.lts_readiness_decision.correction_line_health_section",
        M5StableLineProtectionRole::CorrectionOwnership,
        M5LtsReadinessDecisionKind::CorrectionLineHealthSection,
        M5StableLineLtsReadinessDecisionSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn lts_readiness_decision_rollback_evidence_section_clean() -> M5ResolvedLtsReadinessDecisionEntry {
    lts_readiness_decision(clean_lts_readiness_decision_base(
        "lts_readiness_decision:program-governance:extension-author",
        "launch.line.extension-author",
        "line.lts_readiness_decision.rollback_evidence_section",
        M5StableLineProtectionRole::EvidenceRefresh,
        M5LtsReadinessDecisionKind::RollbackEvidenceSection,
        M5StableLineLtsReadinessDecisionSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn lts_readiness_decision_support_window_posture_section_clean(
) -> M5ResolvedLtsReadinessDecisionEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_lts_readiness_decision_base(
        "lts_readiness_decision:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.lts_readiness_decision.support_window_posture_section",
        M5StableLineProtectionRole::SupportWindow,
        M5LtsReadinessDecisionKind::SupportWindowPostureSection,
        M5StableLineLtsReadinessDecisionSurfaceContext::ExecutiveSteeringSurface,
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
    lts_readiness_decision(base)
}

fn lts_readiness_decision_mirror_air_gap_proof_section_clean() -> M5ResolvedLtsReadinessDecisionEntry
{
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_lts_readiness_decision_base(
        "lts_readiness_decision:support:public-preview",
        "launch.line.public-preview",
        "line.lts_readiness_decision.mirror_air_gap_proof_section",
        M5StableLineProtectionRole::CorrectionOwnership,
        M5LtsReadinessDecisionKind::MirrorAirGapProofSection,
        M5StableLineLtsReadinessDecisionSurfaceContext::SupportOrExportForm,
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
    lts_readiness_decision(base)
}

fn lts_readiness_decision_advisory_revocation_readiness_section_clean(
) -> M5ResolvedLtsReadinessDecisionEntry {
    lts_readiness_decision(clean_lts_readiness_decision_base(
        "lts_readiness_decision:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.lts_readiness_decision.advisory_revocation_readiness_section",
        M5StableLineProtectionRole::BackportDecision,
        M5LtsReadinessDecisionKind::AdvisoryRevocationReadinessSection,
        M5StableLineLtsReadinessDecisionSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded line-lts_readiness_decision entries ---------------------------------------------------------

/// Degraded lts_readiness_decision entry: the resolved lts_readiness_decision object is incomplete — the bundle IDs are unstated.
fn lts_readiness_decision_object_incomplete() -> M5ResolvedLtsReadinessDecisionEntry {
    let mut base = clean_lts_readiness_decision_base(
        "lts_readiness_decision:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.lts_readiness_decision.core_team_canary",
        M5StableLineProtectionRole::SupportWindow,
        M5LtsReadinessDecisionKind::BackportBranchPostureSection,
        M5StableLineLtsReadinessDecisionSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    lts_readiness_decision(base)
}

/// Degraded lts_readiness_decision entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn lts_readiness_decision_widen_fold() -> M5ResolvedLtsReadinessDecisionEntry {
    let mut base = clean_lts_readiness_decision_base(
        "lts_readiness_decision:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.lts_readiness_decision.correction_line_health_section",
        M5StableLineProtectionRole::CorrectionOwnership,
        M5LtsReadinessDecisionKind::CorrectionLineHealthSection,
        M5StableLineLtsReadinessDecisionSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    lts_readiness_decision(base)
}

/// Degraded lts_readiness_decision entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn lts_readiness_decision_unbound() -> M5ResolvedLtsReadinessDecisionEntry {
    let mut base = clean_lts_readiness_decision_base(
        "lts_readiness_decision:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.lts_readiness_decision.advisory_revocation_readiness_section",
        M5StableLineProtectionRole::BackportDecision,
        M5LtsReadinessDecisionKind::AdvisoryRevocationReadinessSection,
        M5StableLineLtsReadinessDecisionSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    lts_readiness_decision(base)
}

/// Degraded lts_readiness_decision entry: the canonical registry token name is unstated.
fn lts_readiness_decision_token_unstated() -> M5ResolvedLtsReadinessDecisionEntry {
    let mut base = clean_lts_readiness_decision_base(
        "lts_readiness_decision:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5StableLineProtectionRole::EvidenceRefresh,
        M5LtsReadinessDecisionKind::RollbackEvidenceSection,
        M5StableLineLtsReadinessDecisionSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    lts_readiness_decision(base)
}

// -- Clean line-downgrade-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_downgrade_base(
    entry_id: &str,
    gate_ref: &str,
    token_name: &str,
    semantic_role: M5StableLineProtectionRole,
    gate_scope: M5LineCreationGateScope,
    surface_context: M5StableLineLtsReadinessDecisionSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5LineCreationGateEntryResolutionInput {
    M5LineCreationGateEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        gate_ref: gate_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        gate_scope,
        surface_context,
        resolution_form_coverage: all_forms(),
        resolved_line_identity: resolved_line_identity.to_owned(),
        known_limits_ledger: known_limits_ledger.to_owned(),
        rollback_target_reference: rollback_target_reference.to_owned(),
        rehearsal_currency_state: rehearsal_currency_state.to_owned(),
        readiness_signoff_state: readiness_signoff_state.to_owned(),
        support_language_reference: support_language_reference.to_owned(),
        last_widening_revision: last_widening_revision.to_owned(),
        keeps_line_creation_gate_visible: true,
        gate_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn downgrade_dogfood_ring_clean() -> M5ResolvedLineCreationGateEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5StableLineProtectionRole::SupportWindow,
        M5LineCreationGateScope::LtsCommitted,
        M5StableLineLtsReadinessDecisionSurfaceContext::ShiproomSurface,
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

fn downgrade_rehearsal_currency_clean() -> M5ResolvedLineCreationGateEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.rollback_evidence_section",
        M5StableLineProtectionRole::EvidenceRefresh,
        M5LineCreationGateScope::LtsBlockedMissingPacket,
        M5StableLineLtsReadinessDecisionSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedLineCreationGateEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.advisory_revocation_readiness_section",
        M5StableLineProtectionRole::BackportDecision,
        M5LineCreationGateScope::LtsNarrowedToStable,
        M5StableLineLtsReadinessDecisionSurfaceContext::ReleaseCenterSurface,
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
fn downgrade_support_ahead() -> M5ResolvedLineCreationGateEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.mirror_air_gap_proof_section",
        M5StableLineProtectionRole::CorrectionOwnership,
        M5LineCreationGateScope::LtsCommitted,
        M5StableLineLtsReadinessDecisionSurfaceContext::ShiproomSurface,
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
fn downgrade_form_incomplete() -> M5ResolvedLineCreationGateEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.advisory_revocation_readiness_section",
        M5StableLineProtectionRole::BackportDecision,
        M5LineCreationGateScope::LtsNarrowedToStable,
        M5StableLineLtsReadinessDecisionSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage =
        vec![M5StableLineLtsReadinessDecisionResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn gate_scope_unclassified() -> M5ResolvedLineCreationGateEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.support_window_posture_section",
        M5StableLineProtectionRole::SupportWindow,
        M5LineCreationGateScope::ScopeUnclassified,
        M5StableLineLtsReadinessDecisionSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5StableLineLtsReadinessDecisionLineCreationGateRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5StableLineProtectionDowngradeTrigger>,
    lts_readiness_decision_entries: Vec<M5ResolvedLtsReadinessDecisionEntry>,
    line_creation_gate_entries: Vec<M5ResolvedLineCreationGateEntry>,
) -> M5StableLineLtsReadinessDecisionLineCreationGateRegistriesRow {
    M5StableLineLtsReadinessDecisionLineCreationGateRegistriesRow {
        consumer_surface,
        qualification: M5StableLineProtectionQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        widening_stages: M5StableLineProtectionWideningStage::ALL.to_vec(),
        required_labels: M5StableLineProtectionRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5StableLineProtectionAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5StableLineLtsReadinessDecisionAnatomyPart::ALL.to_vec(),
        export_fields: M5StableLineLtsReadinessDecisionExportField::ALL.to_vec(),
        downgrade_triggers,
        lts_readiness_decision_entries,
        line_creation_gate_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_STABLE_LINE_LTS_READINESS_DECISION_LINE_CREATION_GATE_REGISTRIES_SCHEMA_REF,
            M5_LTS_READINESS_DECISION_DOMAIN_SCHEMA_REF,
            M5_LINE_CREATION_GATE_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_line_creation_gate_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5StableLineLtsReadinessDecisionLineCreationGateRegistriesRow> {
    use M5StableLineProtectionConsumerSurface as C;
    use M5StableLineProtectionDowngradeTrigger as D;

    vec![
        base_row(
            C::Shiproom,
            "Shiproom owner",
            "The shiproom resolves the candidate line's backport-branch-posture section to one typed LTS-readiness-decision object — the candidate line rows, decision section, support-window / backport / rollback / mirror-air-gap / advisory proofs, rollback target, and named forum outcome — from the shared registry and proves the LTS-committed gate for that line; an LTS-readiness-decision object missing its rollback / support evidence and a gate that keeps LTS language ahead of the current packet degrade honestly instead of leaving an LTS promise to read as earned",
            "downgrade:m5-launch-control-shiproom:001",
            vec![
                D::WidenedSupportWithoutCurrentRefreshEvidence,
                D::RanSupportLanguageAheadOfRefreshProof,
                D::ProofStale,
            ],
            vec![
                lts_readiness_decision_backport_branch_posture_section_clean(),
                lts_readiness_decision_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::ReleaseCenter,
            "Release-center owner",
            "The release center resolves the correction-line-health section and the LTS-narrowed-to-stable gate while keeping the active gate reason visible; a candidate widening its LTS language while its decision packet is unresolved and a resolution-form gap on a gate are caught before a screenshot can reintroduce an LTS-promise-as-earned reading",
            "downgrade:m5-launch-control-release-center:001",
            vec![
                D::WidenedSupportWithoutCurrentRefreshEvidence,
                D::ImpliedGreenWhileRefreshOrLedgerWasStale,
                D::ProofStale,
            ],
            vec![lts_readiness_decision_correction_line_health_section_clean(), lts_readiness_decision_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::ExecutiveSteering,
            "Executive-steering owner",
            "Executive steering resolves the support-window-posture section while keeping its public LTS support-window claim matched to current rollback and support evidence and reports the line-creation-gate outcome; an LTS-readiness-decision entry that is a hand-copied per-entry assumption and a gate on an unclassified gate scope degrade honestly",
            "downgrade:m5-launch-control-executive-steering:001",
            vec![
                D::RegistryReferenceUnstated,
                D::RefreshStateUnstated,
                D::ProofStale,
            ],
            vec![
                lts_readiness_decision_support_window_posture_section_clean(),
                lts_readiness_decision_unbound(),
            ],
            vec![gate_scope_unclassified()],
        ),
        base_row(
            C::ProgramGovernance,
            "Program-governance owner",
            "Program governance resolves the rollback-evidence section and the LTS-blocked-missing-packet gate bound to the registry; an unstated registry token on an LTS-readiness-decision entry is caught before it can drift",
            "downgrade:m5-launch-control-program-governance:001",
            vec![
                D::RegistryReferenceUnstated,
                D::SupportWindowUnstated,
                D::ProofStale,
            ],
            vec![
                lts_readiness_decision_rollback_evidence_section_clean(),
                lts_readiness_decision_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics renders the same resolved LTS-readiness-decision and line-creation-gate truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied line table; the advisory-revocation-readiness section and the LTS-narrowed-to-stable gate stay inspectable off-renderer",
            "downgrade:m5-launch-control-diagnostics:001",
            vec![
                D::SupportWindowUnstated,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![lts_readiness_decision_advisory_revocation_readiness_section_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved LTS-readiness-decision and line-creation-gate truth, so a hand-copied constant, an unstated registry token, an LTS-widen-without-green-packet attempt, or LTS language running ahead of the current decision packet is visible in evidence — LTS committed, LTS blocked on a missing packet, or LTS narrowed to stable — rather than hidden behind a screenshot",
            "downgrade:m5-launch-control-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::RanSupportLanguageAheadOfRefreshProof,
                D::ProofStale,
            ],
            vec![lts_readiness_decision_mirror_air_gap_proof_section_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5StableLineLtsReadinessDecisionLineCreationGateRegistriesGovernanceReview
{
    M5StableLineLtsReadinessDecisionLineCreationGateRegistriesGovernanceReview {
        lts_readiness_decision_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_lts_readiness_decision_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        line_creation_gate_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        lts_readiness_decision_or_downgrade_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection(
) -> M5StableLineLtsReadinessDecisionLineCreationGateRegistriesConsumerProjection {
    M5StableLineLtsReadinessDecisionLineCreationGateRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5StableLineLtsReadinessDecisionLineCreationGateRegistriesProofFreshness {
    M5StableLineLtsReadinessDecisionLineCreationGateRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5StableLineLtsReadinessDecisionLineCreationGateRegistriesReleasePosture {
    M5StableLineLtsReadinessDecisionLineCreationGateRegistriesReleasePosture {
        proof_packet_ref:
            M5_STABLE_LINE_LTS_READINESS_DECISION_LINE_CREATION_GATE_REGISTRIES_ARTIFACT_REF
                .to_owned(),
        line_audit_ref:
            M5_STABLE_LINE_LTS_READINESS_DECISION_LINE_CREATION_GATE_REGISTRIES_REPORT_REF
                .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_STABLE_LINE_LTS_READINESS_DECISION_LINE_CREATION_GATE_REGISTRIES_SCHEMA_REF,
        M5_STABLE_LINE_LTS_READINESS_DECISION_LINE_CREATION_GATE_REGISTRIES_DOC_REF,
        M5_STABLE_LINE_PROTECTION_MATRIX_SCHEMA_REF,
        M5_STABLE_LINE_PROTECTION_MATRIX_DOC_REF,
        M5_LTS_READINESS_DECISION_DOMAIN_SCHEMA_REF,
        M5_LINE_CREATION_GATE_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 91–180 day LTS-readiness-decision and line-creation-gate registries packet.
pub fn seeded_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries(
) -> M5StableLineLtsReadinessDecisionLineCreationGateRegistriesPacket {
    M5StableLineLtsReadinessDecisionLineCreationGateRegistriesPacket::new(
        M5StableLineLtsReadinessDecisionLineCreationGateRegistriesPacketInput {
            packet_id: M5_STABLE_LINE_LTS_READINESS_DECISION_LINE_CREATION_GATE_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 91–180 day LTS-readiness-decision and line-creation-gate registries recording one typed decision packet per candidate line — one section per operating proof: backport branch posture, correction-line health, rollback evidence, support-window posture, mirror / air-gap continuity proof, and advisory / revocation readiness — each bound to the named decision-forum outcome and the preserved rollback / support evidence snapshot, with rollback / support posture preserved so LTS / support language never runs ahead of the current evidence, canonical / accessible / audit resolution-form coverage, and a machine-readable line-creation-gate (LTS-committed, LTS-blocked-missing-packet, or LTS-narrowed-to-stable) that gates LTS line creation and LTS-style language on a green decision packet and lets consumers explain why a line is or is not LTS-ready, naming the active gate reason across release / help, support, shiproom, executive-steering, program-governance, and public-proof surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5StableLineLtsReadinessDecisionLineCreationGateRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the shiproom row is held at Beta pending LTS-readiness-decision parity on every candidate line;
/// every row stays visible and every example stays honest.
pub fn seeded_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries_lts_readiness_decision_beta_narrowed(
) -> M5StableLineLtsReadinessDecisionLineCreationGateRegistriesPacket {
    let mut packet =
        seeded_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries();
    packet.packet_id =
        "m5-stable-line-lts-readiness-decision-and-line-creation-gate-registries:lts-readiness-decision-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5StableLineProtectionConsumerSurface::Shiproom)
        .expect("shiproom row present");
    row.qualification = M5StableLineProtectionQualificationClass::Beta;
    packet
}

/// Narrowed variant: the release-center row is narrowed to Preview pending line-creation-gate parity on every
/// candidate line; every row stays visible and every example stays honest.
pub fn seeded_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries_line_creation_gate_preview_narrowed(
) -> M5StableLineLtsReadinessDecisionLineCreationGateRegistriesPacket {
    let mut packet =
        seeded_m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries();
    packet.packet_id =
        "m5-stable-line-lts-readiness-decision-and-line-creation-gate-registries:line-creation-gate-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5StableLineProtectionConsumerSurface::ReleaseCenter)
        .expect("release-center row present");
    row.qualification = M5StableLineProtectionQualificationClass::Preview;
    packet
}
