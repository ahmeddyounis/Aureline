//! Canonical seed builders for the M5 line-retirement_countdown and line-downgrade-packet registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean line-retirement_countdown and line-downgrade-packet entries
//! are built so the one typed line-retirement_countdown object resolving per line, lines never widening without
//! preserving rollback and diagnostics, partner / public support language never running ahead of line proof,
//! the canonical / accessible / audit resolution forms, and the complete line-identity / known-limits-ledger /
//! rollback-target / rehearsal-currency / readiness-signoff / support-language / last-widening-revision
//! line-downgrade object are proven across the shiproom, release-center, executive-steering,
//! program-governance, diagnostics, and support surfaces without any hand-copied per-line assumption,
//! widen-without-rollback, incomplete object, hidden line downgrade, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_RETIREMENT_COUNTDOWN_SAFETY_GATE_REGISTRIES_PACKET_ID: &str =
    "m5-retirement-countdown-and-safety-gate-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn retirement_countdown(
    input: M5RetirementCountdownEntryResolutionInput,
) -> M5ResolvedRetirementCountdownEntry {
    resolve_retirement_countdown_entry(input)
        .expect("seed line-retirement_countdown entry resolves")
}

fn downgrade(input: M5SafetyGateEntryResolutionInput) -> M5ResolvedSafetyGateEntry {
    resolve_safety_gate_entry(input).expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5RetirementCountdownResolutionForm> {
    M5RetirementCountdownResolutionForm::ALL.to_vec()
}

// -- Clean line-retirement_countdown entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_retirement_countdown_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5RetiredStateRole,
    report_section: M5RetirementCountdownKind,
    surface_context: M5RetirementCountdownSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5RetirementCountdownEntryResolutionInput {
    M5RetirementCountdownEntryResolutionInput {
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

fn retirement_countdown_first_deprecated_version_clean() -> M5ResolvedRetirementCountdownEntry {
    retirement_countdown(clean_retirement_countdown_base(
        "retirement_countdown:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.retirement_countdown.core_team_canary",
        M5RetiredStateRole::LastSupportedPin,
        M5RetirementCountdownKind::FirstDeprecatedVersion,
        M5RetirementCountdownSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn retirement_countdown_cutoff_version_or_date_clean() -> M5ResolvedRetirementCountdownEntry {
    retirement_countdown(clean_retirement_countdown_base(
        "retirement_countdown:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.retirement_countdown.cutoff_version_or_date",
        M5RetiredStateRole::SuccessorRouting,
        M5RetirementCountdownKind::CutoffVersionOrDate,
        M5RetirementCountdownSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn retirement_countdown_remaining_overlap_window_clean() -> M5ResolvedRetirementCountdownEntry {
    retirement_countdown(clean_retirement_countdown_base(
        "retirement_countdown:program-governance:extension-author",
        "launch.line.extension-author",
        "line.retirement_countdown.remaining_overlap_window",
        M5RetiredStateRole::DisablePath,
        M5RetirementCountdownKind::RemainingOverlapWindow,
        M5RetirementCountdownSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn retirement_countdown_successor_route_clean() -> M5ResolvedRetirementCountdownEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_retirement_countdown_base(
        "retirement_countdown:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.retirement_countdown.successor_route",
        M5RetiredStateRole::LastSupportedPin,
        M5RetirementCountdownKind::SuccessorRoute,
        M5RetirementCountdownSurfaceContext::ExecutiveSteeringSurface,
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
    retirement_countdown(base)
}

fn retirement_countdown_fallback_action_clean() -> M5ResolvedRetirementCountdownEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_retirement_countdown_base(
        "retirement_countdown:support:public-preview",
        "launch.line.public-preview",
        "line.retirement_countdown.fallback_action",
        M5RetiredStateRole::SuccessorRouting,
        M5RetirementCountdownKind::FallbackAction,
        M5RetirementCountdownSurfaceContext::SupportOrExportForm,
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
    retirement_countdown(base)
}

fn retirement_countdown_no_surprises_explanation_clean() -> M5ResolvedRetirementCountdownEntry {
    retirement_countdown(clean_retirement_countdown_base(
        "retirement_countdown:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.retirement_countdown.no_surprises_explanation",
        M5RetiredStateRole::SupportNoteClosure,
        M5RetirementCountdownKind::NoSurprisesExplanation,
        M5RetirementCountdownSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded line-retirement_countdown entries ---------------------------------------------------------

/// Degraded retirement_countdown entry: the resolved retirement_countdown object is incomplete — the bundle IDs are unstated.
fn retirement_countdown_object_incomplete() -> M5ResolvedRetirementCountdownEntry {
    let mut base = clean_retirement_countdown_base(
        "retirement_countdown:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.retirement_countdown.core_team_canary",
        M5RetiredStateRole::LastSupportedPin,
        M5RetirementCountdownKind::FirstDeprecatedVersion,
        M5RetirementCountdownSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    retirement_countdown(base)
}

/// Degraded retirement_countdown entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn retirement_countdown_widen_fold() -> M5ResolvedRetirementCountdownEntry {
    let mut base = clean_retirement_countdown_base(
        "retirement_countdown:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.retirement_countdown.cutoff_version_or_date",
        M5RetiredStateRole::SuccessorRouting,
        M5RetirementCountdownKind::CutoffVersionOrDate,
        M5RetirementCountdownSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    retirement_countdown(base)
}

/// Degraded retirement_countdown entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn retirement_countdown_unbound() -> M5ResolvedRetirementCountdownEntry {
    let mut base = clean_retirement_countdown_base(
        "retirement_countdown:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.retirement_countdown.no_surprises_explanation",
        M5RetiredStateRole::SupportNoteClosure,
        M5RetirementCountdownKind::NoSurprisesExplanation,
        M5RetirementCountdownSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    retirement_countdown(base)
}

/// Degraded retirement_countdown entry: the canonical registry token name is unstated.
fn retirement_countdown_token_unstated() -> M5ResolvedRetirementCountdownEntry {
    let mut base = clean_retirement_countdown_base(
        "retirement_countdown:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5RetiredStateRole::DisablePath,
        M5RetirementCountdownKind::RemainingOverlapWindow,
        M5RetirementCountdownSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    retirement_countdown(base)
}

// -- Clean line-downgrade-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_downgrade_base(
    entry_id: &str,
    comparison_ref: &str,
    token_name: &str,
    semantic_role: M5RetiredStateRole,
    comparison_scope: M5SafetyGateScope,
    surface_context: M5RetirementCountdownSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5SafetyGateEntryResolutionInput {
    M5SafetyGateEntryResolutionInput {
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
        keeps_safety_gate_visible: true,
        comparison_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn downgrade_dogfood_ring_clean() -> M5ResolvedSafetyGateEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5RetiredStateRole::LastSupportedPin,
        M5SafetyGateScope::MissingRollbackOrExportPath,
        M5RetirementCountdownSurfaceContext::ShiproomSurface,
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

fn downgrade_rehearsal_currency_clean() -> M5ResolvedSafetyGateEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.remaining_overlap_window",
        M5RetiredStateRole::DisablePath,
        M5SafetyGateScope::MissingArchiveBundle,
        M5RetirementCountdownSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedSafetyGateEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.no_surprises_explanation",
        M5RetiredStateRole::SupportNoteClosure,
        M5SafetyGateScope::MissingSuccessorOrFallbackRoute,
        M5RetirementCountdownSurfaceContext::ReleaseCenterSurface,
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
fn downgrade_support_ahead() -> M5ResolvedSafetyGateEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.fallback_action",
        M5RetiredStateRole::SuccessorRouting,
        M5SafetyGateScope::MissingRollbackOrExportPath,
        M5RetirementCountdownSurfaceContext::ShiproomSurface,
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
fn downgrade_form_incomplete() -> M5ResolvedSafetyGateEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.no_surprises_explanation",
        M5RetiredStateRole::SupportNoteClosure,
        M5SafetyGateScope::MissingSuccessorOrFallbackRoute,
        M5RetirementCountdownSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5RetirementCountdownResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn comparison_safety_gate_scope_unclassified() -> M5ResolvedSafetyGateEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.successor_route",
        M5RetiredStateRole::LastSupportedPin,
        M5SafetyGateScope::SafetyGateScopeUnclassified,
        M5RetirementCountdownSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5RetirementCountdownSafetyGateRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5RetiredStateDowngradeTrigger>,
    retirement_countdown_entries: Vec<M5ResolvedRetirementCountdownEntry>,
    safety_gate_entries: Vec<M5ResolvedSafetyGateEntry>,
) -> M5RetirementCountdownSafetyGateRegistriesRow {
    M5RetirementCountdownSafetyGateRegistriesRow {
        consumer_surface,
        qualification: M5RetiredStateQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        removal_horizon_stages: M5RetiredStateRemovalHorizonStage::ALL.to_vec(),
        required_labels: M5RetiredStateRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5RetiredStateAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5RetirementCountdownAnatomyPart::ALL.to_vec(),
        export_fields: M5RetirementCountdownExportField::ALL.to_vec(),
        downgrade_triggers,
        retirement_countdown_entries,
        safety_gate_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_RETIREMENT_COUNTDOWN_SAFETY_GATE_REGISTRIES_SCHEMA_REF,
            M5_RETIREMENT_COUNTDOWN_DOMAIN_SCHEMA_REF,
            M5_SAFETY_GATE_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_safety_gate_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5RetirementCountdownSafetyGateRegistriesRow> {
    use M5RetiredStateConsumerSurface as C;
    use M5RetiredStateDowngradeTrigger as D;

    vec![
        base_row(
            C::ReleaseCenter,
            "Release-center owner",
            "The release center resolves an affected surface to one typed retirement-countdown object — the classified countdown field (here the first-deprecated version), its owning team, the exact-build joins, and the successor route or manual fallback — from the shared registry and proves the missing-rollback-or-export-path safety gate for that candidate; a countdown missing its exact-build joins and a gate that keeps support language ahead of the closed support note degrade honestly instead of leaving a retiring surface to read as safe to close",
            "retirement:m5-release-center:001",
            vec![
                D::SuccessorPathUnnamed,
                D::SupportNoteClosureIncomplete,
                D::RetirementManifestStale,
            ],
            vec![
                retirement_countdown_first_deprecated_version_clean(),
                retirement_countdown_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::HelpDocs,
            "Help/docs owner",
            "Help / docs resolves the cutoff-version-or-date countdown field and the missing-successor-or-fallback-route safety gate while keeping the active gate reason visible; a retiring surface widening its claim without a preserved rollback / export route and a resolution-form gap on a gate are caught before a screenshot can reintroduce a safe-to-close reading",
            "retirement:m5-help-docs:001",
            vec![
                D::SuccessorPathUnnamed,
                D::ArchivalNoteMissing,
                D::RetirementManifestStale,
            ],
            vec![retirement_countdown_cutoff_version_or_date_clean(), retirement_countdown_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::Support,
            "Support owner",
            "Support resolves the successor-route countdown field while keeping its public-facing successor / fallback claim matched to the closed support note and reports the pre-retirement-safety-gate outcome; a countdown entry that is a hand-copied per-entry assumption and a gate on an unclassified gate scope degrade honestly",
            "retirement:m5-support:001",
            vec![
                D::RegistryReferenceUnstated,
                D::DisablePathUnnamed,
                D::RetirementManifestStale,
            ],
            vec![
                retirement_countdown_successor_route_clean(),
                retirement_countdown_unbound(),
            ],
            vec![comparison_safety_gate_scope_unclassified()],
        ),
        base_row(
            C::MarketplaceRegistry,
            "Marketplace/registry owner",
            "The marketplace / registry resolves the remaining-overlap-window countdown field and the missing-archive-bundle safety gate bound to the registry so a retiring surface can no longer be selected in a new install or by a new tenant while it is still missing a declared safe-exit route; an unstated registry token on a countdown entry is caught before it can drift",
            "retirement:m5-marketplace-registry:001",
            vec![
                D::RegistryReferenceUnstated,
                D::CutoffDateUnstated,
                D::RetirementManifestStale,
            ],
            vec![
                retirement_countdown_remaining_overlap_window_clean(),
                retirement_countdown_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::InstallUpdate,
            "Install/update owner",
            "Install / update renders the same resolved retirement-countdown and pre-retirement-safety-gate truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied class table; the no-surprises-explanation countdown field and the missing-successor-or-fallback-route safety gate stay inspectable off-renderer so no new install can still select a retiring surface with an open pre-cutoff blocker",
            "retirement:m5-install-update:001",
            vec![
                D::CutoffDateUnstated,
                D::RegistryReferenceUnstated,
                D::RetirementManifestStale,
            ],
            vec![retirement_countdown_no_surprises_explanation_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::PartnerProcurement,
            "Partner/procurement owner",
            "The partner / procurement feed carries the same resolved retirement-countdown and pre-retirement-safety-gate truth, so a hand-copied constant, an unstated registry token, a countdown widening its claim without a preserved rollback / export route, or support language running ahead of the closed support note is visible in evidence — a candidate missing its declared rollback / export path, archive bundle, or successor / fallback route — rather than hidden behind a screenshot",
            "retirement:m5-partner-procurement:001",
            vec![
                D::RegistryReferenceUnstated,
                D::SupportNoteClosureIncomplete,
                D::RetirementManifestStale,
            ],
            vec![retirement_countdown_fallback_action_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5RetirementCountdownSafetyGateRegistriesGovernanceReview {
    M5RetirementCountdownSafetyGateRegistriesGovernanceReview {
        retirement_countdown_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_retirement_countdown_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        safety_gate_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        retirement_countdown_or_downgrade_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5RetirementCountdownSafetyGateRegistriesConsumerProjection {
    M5RetirementCountdownSafetyGateRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5RetirementCountdownSafetyGateRegistriesProofFreshness {
    M5RetirementCountdownSafetyGateRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5RetirementCountdownSafetyGateRegistriesReleasePosture {
    M5RetirementCountdownSafetyGateRegistriesReleasePosture {
        proof_packet_ref: M5_RETIREMENT_COUNTDOWN_SAFETY_GATE_REGISTRIES_ARTIFACT_REF.to_owned(),
        line_audit_ref: M5_RETIREMENT_COUNTDOWN_SAFETY_GATE_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_RETIREMENT_COUNTDOWN_SAFETY_GATE_REGISTRIES_SCHEMA_REF,
        M5_RETIREMENT_COUNTDOWN_SAFETY_GATE_REGISTRIES_DOC_REF,
        M5_RETIRED_STATE_MATRIX_SCHEMA_REF,
        M5_RETIRED_STATE_MATRIX_DOC_REF,
        M5_RETIREMENT_COUNTDOWN_DOMAIN_SCHEMA_REF,
        M5_SAFETY_GATE_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 retirement-countdown and safety-gate registries packet.
pub fn seeded_m5_retirement_countdown_and_safety_gate_registries(
) -> M5RetirementCountdownSafetyGateRegistriesPacket {
    M5RetirementCountdownSafetyGateRegistriesPacket::new(
        M5RetirementCountdownSafetyGateRegistriesPacketInput {
            packet_id: M5_RETIREMENT_COUNTDOWN_SAFETY_GATE_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 retirement-countdown and pre-retirement-safety-gate registries emitting one export-safe grace-window countdown per affected install / update, settings / help, docs, marketplace, and support surface — one classified countdown field per published fact (the first-deprecated version, cutoff version / date, remaining overlap window, successor route, fallback action, or no-surprises explanation) with an owning team and joined to the current compatibility / public-proof state and the successor path or manual fallback, so a retirement stops being a hidden date in release notes and becomes a visible, inspectable countdown, with canonical / accessible / audit resolution-form coverage, and a machine-readable pre-retirement safety gate (missing-rollback-or-export-path, missing-archive-bundle, or missing-successor-or-fallback-route) that blocks a candidate from passing to final closure while it is still missing a declared rollback / export / archive path or successor / fallback route, so a product surface and an operator / support surface open the same cutoff and successor data without contradiction and no surface transitions to Retired through a surprise shutdown"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5RetirementCountdownSafetyGateRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the release-center row is held at Beta pending retirement-countdown parity on every
/// countdown field; every row stays visible and every example stays honest.
pub fn seeded_m5_retirement_countdown_and_safety_gate_registries_retirement_countdown_beta_narrowed(
) -> M5RetirementCountdownSafetyGateRegistriesPacket {
    let mut packet = seeded_m5_retirement_countdown_and_safety_gate_registries();
    packet.packet_id =
        "m5-retirement-countdown-and-safety-gate-registries:retirement-countdown-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5RetiredStateConsumerSurface::ReleaseCenter)
        .expect("release-center row present");
    row.qualification = M5RetiredStateQualificationClass::Beta;
    packet
}

/// Narrowed variant: the help/docs row is narrowed to Preview pending pre-retirement-safety-gate parity on every
/// gate scope; every row stays visible and every example stays honest.
pub fn seeded_m5_retirement_countdown_and_safety_gate_registries_safety_gate_preview_narrowed(
) -> M5RetirementCountdownSafetyGateRegistriesPacket {
    let mut packet = seeded_m5_retirement_countdown_and_safety_gate_registries();
    packet.packet_id =
        "m5-retirement-countdown-and-safety-gate-registries:pre-retirement-safety-gate-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5RetiredStateConsumerSurface::HelpDocs)
        .expect("help/docs row present");
    row.qualification = M5RetiredStateQualificationClass::Preview;
    packet
}
