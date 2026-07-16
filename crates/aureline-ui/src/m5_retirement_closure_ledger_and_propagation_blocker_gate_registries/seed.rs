//! Canonical seed builders for the M5 line-retirement_closure_ledger and line-downgrade-packet registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean line-retirement_closure_ledger and line-downgrade-packet entries
//! are built so the one typed line-retirement_closure_ledger object resolving per line, lines never widening without
//! preserving rollback and diagnostics, partner / public support language never running ahead of line proof,
//! the canonical / accessible / audit resolution forms, and the complete line-identity / known-limits-ledger /
//! rollback-target / rehearsal-currency / readiness-signoff / support-language / last-widening-revision
//! line-downgrade object are proven across the shiproom, release-center, executive-steering,
//! program-governance, diagnostics, and support surfaces without any hand-copied per-line assumption,
//! widen-without-rollback, incomplete object, hidden line downgrade, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_RETIREMENT_CLOSURE_LEDGER_PROPAGATION_BLOCKER_GATE_REGISTRIES_PACKET_ID: &str =
    "m5-retirement-closure-ledger-and-propagation-blocker-gate-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn retirement_closure_ledger(
    input: M5RetirementClosureLedgerEntryResolutionInput,
) -> M5ResolvedRetirementClosureLedgerEntry {
    resolve_retirement_closure_ledger_entry(input)
        .expect("seed line-retirement_closure_ledger entry resolves")
}

fn downgrade(
    input: M5PropagationBlockerGateEntryResolutionInput,
) -> M5ResolvedPropagationBlockerGateEntry {
    resolve_propagation_blocker_gate_entry(input)
        .expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5RetirementClosureLedgerResolutionForm> {
    M5RetirementClosureLedgerResolutionForm::ALL.to_vec()
}

// -- Clean line-retirement_closure_ledger entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_retirement_closure_ledger_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5RetiredStateRole,
    report_section: M5RetirementClosureLedgerKind,
    surface_context: M5RetirementClosureLedgerSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5RetirementClosureLedgerEntryResolutionInput {
    M5RetirementClosureLedgerEntryResolutionInput {
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

fn retirement_closure_ledger_migration_outcome_clean() -> M5ResolvedRetirementClosureLedgerEntry {
    retirement_closure_ledger(clean_retirement_closure_ledger_base(
        "retirement_closure_ledger:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.retirement_closure_ledger.core_team_canary",
        M5RetiredStateRole::LastSupportedPin,
        M5RetirementClosureLedgerKind::MigrationOutcome,
        M5RetirementClosureLedgerSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn retirement_closure_ledger_disable_evidence_clean() -> M5ResolvedRetirementClosureLedgerEntry {
    retirement_closure_ledger(clean_retirement_closure_ledger_base(
        "retirement_closure_ledger:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.retirement_closure_ledger.disable_evidence",
        M5RetiredStateRole::SuccessorRouting,
        M5RetirementClosureLedgerKind::DisableEvidence,
        M5RetirementClosureLedgerSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn retirement_closure_ledger_support_note_closure_clean() -> M5ResolvedRetirementClosureLedgerEntry
{
    retirement_closure_ledger(clean_retirement_closure_ledger_base(
        "retirement_closure_ledger:program-governance:extension-author",
        "launch.line.extension-author",
        "line.retirement_closure_ledger.support_note_closure",
        M5RetiredStateRole::DisablePath,
        M5RetirementClosureLedgerKind::SupportNoteClosure,
        M5RetirementClosureLedgerSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn retirement_closure_ledger_archival_note_clean() -> M5ResolvedRetirementClosureLedgerEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_retirement_closure_ledger_base(
        "retirement_closure_ledger:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.retirement_closure_ledger.archival_note",
        M5RetiredStateRole::LastSupportedPin,
        M5RetirementClosureLedgerKind::ArchivalNote,
        M5RetirementClosureLedgerSurfaceContext::ExecutiveSteeringSurface,
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
    retirement_closure_ledger(base)
}

fn retirement_closure_ledger_propagation_status_clean() -> M5ResolvedRetirementClosureLedgerEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_retirement_closure_ledger_base(
        "retirement_closure_ledger:support:public-preview",
        "launch.line.public-preview",
        "line.retirement_closure_ledger.propagation_status",
        M5RetiredStateRole::SuccessorRouting,
        M5RetirementClosureLedgerKind::PropagationStatus,
        M5RetirementClosureLedgerSurfaceContext::SupportOrExportForm,
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
    retirement_closure_ledger(base)
}

fn retirement_closure_ledger_remaining_carve_out_clean() -> M5ResolvedRetirementClosureLedgerEntry {
    retirement_closure_ledger(clean_retirement_closure_ledger_base(
        "retirement_closure_ledger:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.retirement_closure_ledger.remaining_carve_out",
        M5RetiredStateRole::SupportNoteClosure,
        M5RetirementClosureLedgerKind::RemainingCarveOut,
        M5RetirementClosureLedgerSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded line-retirement_closure_ledger entries ---------------------------------------------------------

/// Degraded retirement_closure_ledger entry: the resolved retirement_closure_ledger object is incomplete — the bundle IDs are unstated.
fn retirement_closure_ledger_object_incomplete() -> M5ResolvedRetirementClosureLedgerEntry {
    let mut base = clean_retirement_closure_ledger_base(
        "retirement_closure_ledger:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.retirement_closure_ledger.core_team_canary",
        M5RetiredStateRole::LastSupportedPin,
        M5RetirementClosureLedgerKind::MigrationOutcome,
        M5RetirementClosureLedgerSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    retirement_closure_ledger(base)
}

/// Degraded retirement_closure_ledger entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn retirement_closure_ledger_widen_fold() -> M5ResolvedRetirementClosureLedgerEntry {
    let mut base = clean_retirement_closure_ledger_base(
        "retirement_closure_ledger:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.retirement_closure_ledger.disable_evidence",
        M5RetiredStateRole::SuccessorRouting,
        M5RetirementClosureLedgerKind::DisableEvidence,
        M5RetirementClosureLedgerSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    retirement_closure_ledger(base)
}

/// Degraded retirement_closure_ledger entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn retirement_closure_ledger_unbound() -> M5ResolvedRetirementClosureLedgerEntry {
    let mut base = clean_retirement_closure_ledger_base(
        "retirement_closure_ledger:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.retirement_closure_ledger.remaining_carve_out",
        M5RetiredStateRole::SupportNoteClosure,
        M5RetirementClosureLedgerKind::RemainingCarveOut,
        M5RetirementClosureLedgerSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    retirement_closure_ledger(base)
}

/// Degraded retirement_closure_ledger entry: the canonical registry token name is unstated.
fn retirement_closure_ledger_token_unstated() -> M5ResolvedRetirementClosureLedgerEntry {
    let mut base = clean_retirement_closure_ledger_base(
        "retirement_closure_ledger:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5RetiredStateRole::DisablePath,
        M5RetirementClosureLedgerKind::SupportNoteClosure,
        M5RetirementClosureLedgerSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    retirement_closure_ledger(base)
}

// -- Clean line-downgrade-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_downgrade_base(
    entry_id: &str,
    comparison_ref: &str,
    token_name: &str,
    semantic_role: M5RetiredStateRole,
    comparison_scope: M5PropagationBlockerGateScope,
    surface_context: M5RetirementClosureLedgerSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5PropagationBlockerGateEntryResolutionInput {
    M5PropagationBlockerGateEntryResolutionInput {
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
        keeps_propagation_blocker_gate_visible: true,
        comparison_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn downgrade_dogfood_ring_clean() -> M5ResolvedPropagationBlockerGateEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5RetiredStateRole::LastSupportedPin,
        M5PropagationBlockerGateScope::ProfilePropagationLag,
        M5RetirementClosureLedgerSurfaceContext::ShiproomSurface,
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

fn downgrade_rehearsal_currency_clean() -> M5ResolvedPropagationBlockerGateEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.support_note_closure",
        M5RetiredStateRole::DisablePath,
        M5PropagationBlockerGateScope::ProfileRetiredStateMismatch,
        M5RetirementClosureLedgerSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedPropagationBlockerGateEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.remaining_carve_out",
        M5RetiredStateRole::SupportNoteClosure,
        M5PropagationBlockerGateScope::StillAdvertisingAfterClosure,
        M5RetirementClosureLedgerSurfaceContext::ReleaseCenterSurface,
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
fn downgrade_support_ahead() -> M5ResolvedPropagationBlockerGateEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.propagation_status",
        M5RetiredStateRole::SuccessorRouting,
        M5PropagationBlockerGateScope::ProfilePropagationLag,
        M5RetirementClosureLedgerSurfaceContext::ShiproomSurface,
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
fn downgrade_form_incomplete() -> M5ResolvedPropagationBlockerGateEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.remaining_carve_out",
        M5RetiredStateRole::SupportNoteClosure,
        M5PropagationBlockerGateScope::StillAdvertisingAfterClosure,
        M5RetirementClosureLedgerSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5RetirementClosureLedgerResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn comparison_propagation_blocker_gate_scope_unclassified() -> M5ResolvedPropagationBlockerGateEntry
{
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.archival_note",
        M5RetiredStateRole::LastSupportedPin,
        M5PropagationBlockerGateScope::PropagationBlockerGateScopeUnclassified,
        M5RetirementClosureLedgerSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5RetirementClosureLedgerPropagationBlockerGateRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5RetiredStateDowngradeTrigger>,
    retirement_closure_ledger_entries: Vec<M5ResolvedRetirementClosureLedgerEntry>,
    propagation_blocker_gate_entries: Vec<M5ResolvedPropagationBlockerGateEntry>,
) -> M5RetirementClosureLedgerPropagationBlockerGateRegistriesRow {
    M5RetirementClosureLedgerPropagationBlockerGateRegistriesRow {
        consumer_surface,
        qualification: M5RetiredStateQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        removal_horizon_stages: M5RetiredStateRemovalHorizonStage::ALL.to_vec(),
        required_labels: M5RetiredStateRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5RetiredStateAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5RetirementClosureLedgerAnatomyPart::ALL.to_vec(),
        export_fields: M5RetirementClosureLedgerExportField::ALL.to_vec(),
        downgrade_triggers,
        retirement_closure_ledger_entries,
        propagation_blocker_gate_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_RETIREMENT_CLOSURE_LEDGER_PROPAGATION_BLOCKER_GATE_REGISTRIES_SCHEMA_REF,
            M5_RETIREMENT_CLOSURE_LEDGER_DOMAIN_SCHEMA_REF,
            M5_PROPAGATION_BLOCKER_GATE_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_propagation_blocker_gate_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5RetirementClosureLedgerPropagationBlockerGateRegistriesRow> {
    use M5RetiredStateConsumerSurface as C;
    use M5RetiredStateDowngradeTrigger as D;

    vec![
        base_row(
            C::ReleaseCenter,
            "Release-center owner",
            "The release center resolves a retiring object to one typed retirement closure ledger — its migration outcome, disable evidence, and support-note closure recorded per deployment profile from the shared registry, joined to its exact build — and raises the profile-propagation-lag blocker when a mirror or offline bundle has not yet propagated the retirement manifest, tombstone, and last-supported archive ref; a ledger missing its migration outcome and a propagation blocker whose profile still lags after another profile closed the line degrade honestly instead of letting one profile keep advertising a retired line as still live",
            "retirement:m5-release-center:001",
            vec![
                D::SuccessorPathUnnamed,
                D::SupportNoteClosureIncomplete,
                D::RetirementManifestStale,
            ],
            vec![
                retirement_closure_ledger_migration_outcome_clean(),
                retirement_closure_ledger_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::HelpDocs,
            "Help/docs owner",
            "Help / docs resolves the disable-evidence closure field and the still-advertising-after-closure propagation blocker while keeping the migration outcome and archival note visible; a profile still offering a retired capability for new install after another profile closed it and a resolution-form gap on a propagation blocker are caught before a help / docs card can point at a line one profile has already retired",
            "retirement:m5-help-docs:001",
            vec![
                D::SuccessorPathUnnamed,
                D::ArchivalNoteMissing,
                D::RetirementManifestStale,
            ],
            vec![retirement_closure_ledger_disable_evidence_clean(), retirement_closure_ledger_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::Support,
            "Support owner",
            "Support resolves the archival-note closure field while keeping its public-facing archival / successor claim matched to the successor path each profile still needs and reports the propagation-blocker-gate outcome; a ledger that is a hand-copied per-entry assumption and a propagation blocker on an unclassified scope degrade honestly, and no internal-only detail leaks while the archival / successor path is named",
            "retirement:m5-support:001",
            vec![
                D::RegistryReferenceUnstated,
                D::DisablePathUnnamed,
                D::RetirementManifestStale,
            ],
            vec![
                retirement_closure_ledger_archival_note_clean(),
                retirement_closure_ledger_unbound(),
            ],
            vec![comparison_propagation_blocker_gate_scope_unclassified()],
        ),
        base_row(
            C::MarketplaceRegistry,
            "Marketplace/registry owner",
            "The marketplace / registry surface resolves the support-note-closure field and the profile-retired-state-mismatch propagation blocker bound to the registry so a self-hosted registry or policy bundle can never keep advertising a retired line whose retired-state truth disagrees with the profiles that already closed it, while staying inspectable by its migration outcome and disable evidence; an unstated registry token on a ledger is caught before it can drift",
            "retirement:m5-marketplace-registry:001",
            vec![
                D::RegistryReferenceUnstated,
                D::CutoffDateUnstated,
                D::RetirementManifestStale,
            ],
            vec![
                retirement_closure_ledger_support_note_closure_clean(),
                retirement_closure_ledger_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::InstallUpdate,
            "Install/update owner",
            "Install / update surfaces render the same resolved retirement-closure-ledger and propagation-blocker-gate truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied class table; the remaining-carve-out closure field and the still-advertising-after-closure blocker stay inspectable off-renderer so a managed new-tenant gate blocks a retired line from new provisioning and an offline consumer can read the closure state without live vendor dependencies",
            "retirement:m5-install-update:001",
            vec![
                D::CutoffDateUnstated,
                D::RegistryReferenceUnstated,
                D::RetirementManifestStale,
            ],
            vec![retirement_closure_ledger_remaining_carve_out_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::PartnerProcurement,
            "Partner/procurement owner",
            "The partner / procurement and CLI / headless inspect feed carries the same resolved retirement-closure-ledger and propagation-blocker-gate truth, so a hand-copied constant, an unstated registry token, a mirror still lagging its retirement manifest, or a managed tenant diverging from the profiles that already closed the line is visible in evidence — the closure ledger records propagation success / failure per profile and blocks final retirement certification while any claimed profile still diverges — rather than hidden behind a screenshot",
            "retirement:m5-partner-procurement:001",
            vec![
                D::RegistryReferenceUnstated,
                D::SupportNoteClosureIncomplete,
                D::RetirementManifestStale,
            ],
            vec![retirement_closure_ledger_propagation_status_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5RetirementClosureLedgerPropagationBlockerGateRegistriesGovernanceReview
{
    M5RetirementClosureLedgerPropagationBlockerGateRegistriesGovernanceReview {
        retirement_closure_ledger_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_retirement_closure_ledger_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        propagation_blocker_gate_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        retirement_closure_ledger_or_downgrade_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection(
) -> M5RetirementClosureLedgerPropagationBlockerGateRegistriesConsumerProjection {
    M5RetirementClosureLedgerPropagationBlockerGateRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5RetirementClosureLedgerPropagationBlockerGateRegistriesProofFreshness {
    M5RetirementClosureLedgerPropagationBlockerGateRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5RetirementClosureLedgerPropagationBlockerGateRegistriesReleasePosture {
    M5RetirementClosureLedgerPropagationBlockerGateRegistriesReleasePosture {
        proof_packet_ref:
            M5_RETIREMENT_CLOSURE_LEDGER_PROPAGATION_BLOCKER_GATE_REGISTRIES_ARTIFACT_REF.to_owned(),
        line_audit_ref: M5_RETIREMENT_CLOSURE_LEDGER_PROPAGATION_BLOCKER_GATE_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_RETIREMENT_CLOSURE_LEDGER_PROPAGATION_BLOCKER_GATE_REGISTRIES_SCHEMA_REF,
        M5_RETIREMENT_CLOSURE_LEDGER_PROPAGATION_BLOCKER_GATE_REGISTRIES_DOC_REF,
        M5_RETIRED_STATE_MATRIX_SCHEMA_REF,
        M5_RETIRED_STATE_MATRIX_DOC_REF,
        M5_RETIREMENT_CLOSURE_LEDGER_DOMAIN_SCHEMA_REF,
        M5_PROPAGATION_BLOCKER_GATE_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 retirement-closure-ledger and propagation-blocker-gate registries packet.
pub fn seeded_m5_retirement_closure_ledger_and_propagation_blocker_gate_registries(
) -> M5RetirementClosureLedgerPropagationBlockerGateRegistriesPacket {
    M5RetirementClosureLedgerPropagationBlockerGateRegistriesPacket::new(
        M5RetirementClosureLedgerPropagationBlockerGateRegistriesPacketInput {
            packet_id: M5_RETIREMENT_CLOSURE_LEDGER_PROPAGATION_BLOCKER_GATE_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 retirement-closure-ledger and propagation-blocker-gate registries propagating retirement manifests, tombstones, and last-supported archive refs into mirror metadata, offline bundle manifests, self-hosted registry / catalog paths, policy bundles, and managed new-tenant / new-workspace gates across the release-center, help / docs, support, marketplace / registry, install / update, and partner / procurement surfaces so mirrors, offline bundles, self-hosted registries, and managed tenant gates all converge on the same closed retired-state truth — one export-safe retirement closure ledger per retiring object per deployment profile (recording its migration outcome, disable evidence, support-note closure, archival note, propagation status, and any remaining carve-outs, joined to its exact build) with canonical / accessible / audit resolution-form coverage, and a machine-readable propagation-blocker gate (profile-propagation-lag, profile-retired-state-mismatch, or still-advertising-after-closure) that blocks final retirement certification while a claimed profile still lags, diverges from the closed profiles, or keeps advertising a retired line or capability after another profile closed it, so a managed consumer and a mirror / offline / self-hosted consumer agree on retired-state truth for the same object and the propagation naming the archival / successor path each profile needs never leaks internal-only detail"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5RetirementClosureLedgerPropagationBlockerGateRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the release-center row is held at Beta pending retirement-closure-ledger parity on every
/// snapshot field; every row stays visible and every example stays honest.
pub fn seeded_m5_retirement_closure_ledger_and_propagation_blocker_gate_registries_retirement_closure_ledger_beta_narrowed(
) -> M5RetirementClosureLedgerPropagationBlockerGateRegistriesPacket {
    let mut packet = seeded_m5_retirement_closure_ledger_and_propagation_blocker_gate_registries();
    packet.packet_id =
        "m5-retirement-closure-ledger-and-propagation-blocker-gate-registries:retirement-closure-ledger-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5RetiredStateConsumerSurface::ReleaseCenter)
        .expect("release-center row present");
    row.qualification = M5RetiredStateQualificationClass::Beta;
    packet
}

/// Narrowed variant: the help/docs row is narrowed to Preview pending propagation-blocker-gate parity on every
/// archive-export scope; every row stays visible and every example stays honest.
pub fn seeded_m5_retirement_closure_ledger_and_propagation_blocker_gate_registries_propagation_blocker_gate_preview_narrowed(
) -> M5RetirementClosureLedgerPropagationBlockerGateRegistriesPacket {
    let mut packet = seeded_m5_retirement_closure_ledger_and_propagation_blocker_gate_registries();
    packet.packet_id =
        "m5-retirement-closure-ledger-and-propagation-blocker-gate-registries:propagation-blocker-gate-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5RetiredStateConsumerSurface::HelpDocs)
        .expect("help/docs row present");
    row.qualification = M5RetiredStateQualificationClass::Preview;
    packet
}
