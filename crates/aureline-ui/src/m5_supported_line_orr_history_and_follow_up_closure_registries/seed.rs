//! Canonical seed builders for the M5 orr-history-event and follow-up-closure registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean ORR-history-event and follow-up-closure entries
//! are built so the one typed ORR-history event resolving per line, lines never widening a go/no-go or cohort
//! claim without preserving their recorded decision history, a claim never running ahead of recorded ORR history,
//! the canonical / accessible / audit resolution forms, and the complete line-identity / affected-history-entry /
//! archived-versus-active-line / closure-scope / active-reason follow-up-closure object are proven across the
//! shiproom, release-center, executive-steering, program-governance, diagnostics, and support surfaces without any
//! hand-copied per-line assumption, widen-without-history, incomplete object, hidden closure, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_SUPPORTED_LINE_ORR_HISTORY_EVENT_FOLLOW_UP_CLOSURE_REGISTRIES_PACKET_ID: &str =
    "m5-supported-line-orr-history-and-follow-up-closure-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn orr_history_event(
    input: M5OrrHistoryEventEntryResolutionInput,
) -> M5ResolvedOrrHistoryEventEntry {
    resolve_orr_history_event_entry(input).expect("seed line-orr_history_event entry resolves")
}

fn downgrade(input: M5FollowUpClosureEntryResolutionInput) -> M5ResolvedFollowUpClosureEntry {
    resolve_follow_up_closure_entry(input).expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5SupportedLineOrrHistoryEventResolutionForm> {
    M5SupportedLineOrrHistoryEventResolutionForm::ALL.to_vec()
}

// -- Clean line-orr_history_event entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_orr_history_event_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5SupportedLineTransparencyRole,
    report_section: M5OrrHistoryEventKind,
    surface_context: M5SupportedLineOrrHistoryEventSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5OrrHistoryEventEntryResolutionInput {
    M5OrrHistoryEventEntryResolutionInput {
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

fn orr_history_event_orr_packet_archive_clean() -> M5ResolvedOrrHistoryEventEntry {
    orr_history_event(clean_orr_history_event_base(
        "orr_history_event:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.orr_history_event.core_team_canary",
        M5SupportedLineTransparencyRole::FreshnessWindow,
        M5OrrHistoryEventKind::OrrPacketArchive,
        M5SupportedLineOrrHistoryEventSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn orr_history_event_freeze_exception_clean() -> M5ResolvedOrrHistoryEventEntry {
    orr_history_event(clean_orr_history_event_base(
        "orr_history_event:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.orr_history_event.freeze_exception",
        M5SupportedLineTransparencyRole::TransparencyDisclosure,
        M5OrrHistoryEventKind::FreezeException,
        M5SupportedLineOrrHistoryEventSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn orr_history_event_rehearsal_outcome_clean() -> M5ResolvedOrrHistoryEventEntry {
    orr_history_event(clean_orr_history_event_base(
        "orr_history_event:program-governance:extension-author",
        "launch.line.extension-author",
        "line.orr_history_event.rehearsal_outcome",
        M5SupportedLineTransparencyRole::MigrationScoreboardCurrency,
        M5OrrHistoryEventKind::RehearsalOutcome,
        M5SupportedLineOrrHistoryEventSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn orr_history_event_cohort_transition_clean() -> M5ResolvedOrrHistoryEventEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_orr_history_event_base(
        "orr_history_event:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.orr_history_event.cohort_transition",
        M5SupportedLineTransparencyRole::FreshnessWindow,
        M5OrrHistoryEventKind::CohortTransition,
        M5SupportedLineOrrHistoryEventSurfaceContext::ExecutiveSteeringSurface,
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
    orr_history_event(base)
}

fn orr_history_event_go_no_go_decision_clean() -> M5ResolvedOrrHistoryEventEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_orr_history_event_base(
        "orr_history_event:support:public-preview",
        "launch.line.public-preview",
        "line.orr_history_event.go_no_go_decision",
        M5SupportedLineTransparencyRole::TransparencyDisclosure,
        M5OrrHistoryEventKind::GoNoGoDecision,
        M5SupportedLineOrrHistoryEventSurfaceContext::SupportOrExportForm,
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
    orr_history_event(base)
}

fn orr_history_event_action_item_closure_clean() -> M5ResolvedOrrHistoryEventEntry {
    orr_history_event(clean_orr_history_event_base(
        "orr_history_event:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.orr_history_event.action_item_closure",
        M5SupportedLineTransparencyRole::OrrHistoryRetention,
        M5OrrHistoryEventKind::ActionItemClosure,
        M5SupportedLineOrrHistoryEventSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded line-orr_history_event entries ---------------------------------------------------------

/// Degraded orr_history_event entry: the resolved orr_history_event object is incomplete — the bundle IDs are unstated.
fn orr_history_event_object_incomplete() -> M5ResolvedOrrHistoryEventEntry {
    let mut base = clean_orr_history_event_base(
        "orr_history_event:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.orr_history_event.core_team_canary",
        M5SupportedLineTransparencyRole::FreshnessWindow,
        M5OrrHistoryEventKind::OrrPacketArchive,
        M5SupportedLineOrrHistoryEventSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    orr_history_event(base)
}

/// Degraded orr_history_event entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn orr_history_event_widen_fold() -> M5ResolvedOrrHistoryEventEntry {
    let mut base = clean_orr_history_event_base(
        "orr_history_event:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.orr_history_event.freeze_exception",
        M5SupportedLineTransparencyRole::TransparencyDisclosure,
        M5OrrHistoryEventKind::FreezeException,
        M5SupportedLineOrrHistoryEventSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    orr_history_event(base)
}

/// Degraded orr_history_event entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn orr_history_event_unbound() -> M5ResolvedOrrHistoryEventEntry {
    let mut base = clean_orr_history_event_base(
        "orr_history_event:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.orr_history_event.action_item_closure",
        M5SupportedLineTransparencyRole::OrrHistoryRetention,
        M5OrrHistoryEventKind::ActionItemClosure,
        M5SupportedLineOrrHistoryEventSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    orr_history_event(base)
}

/// Degraded orr_history_event entry: the canonical registry token name is unstated.
fn orr_history_event_token_unstated() -> M5ResolvedOrrHistoryEventEntry {
    let mut base = clean_orr_history_event_base(
        "orr_history_event:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5SupportedLineTransparencyRole::MigrationScoreboardCurrency,
        M5OrrHistoryEventKind::RehearsalOutcome,
        M5SupportedLineOrrHistoryEventSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    orr_history_event(base)
}

// -- Clean line-downgrade-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_downgrade_base(
    entry_id: &str,
    comparison_ref: &str,
    token_name: &str,
    semantic_role: M5SupportedLineTransparencyRole,
    comparison_scope: M5FollowUpClosureScope,
    surface_context: M5SupportedLineOrrHistoryEventSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5FollowUpClosureEntryResolutionInput {
    M5FollowUpClosureEntryResolutionInput {
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
        keeps_follow_up_closure_visible: true,
        comparison_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn downgrade_dogfood_ring_clean() -> M5ResolvedFollowUpClosureEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5SupportedLineTransparencyRole::FreshnessWindow,
        M5FollowUpClosureScope::UnclosedActionItem,
        M5SupportedLineOrrHistoryEventSurfaceContext::ShiproomSurface,
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

fn downgrade_rehearsal_currency_clean() -> M5ResolvedFollowUpClosureEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.rehearsal_outcome",
        M5SupportedLineTransparencyRole::MigrationScoreboardCurrency,
        M5FollowUpClosureScope::StaleRehearsalEvidence,
        M5SupportedLineOrrHistoryEventSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedFollowUpClosureEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.action_item_closure",
        M5SupportedLineTransparencyRole::OrrHistoryRetention,
        M5FollowUpClosureScope::UnreconstructableLineHistory,
        M5SupportedLineOrrHistoryEventSurfaceContext::ReleaseCenterSurface,
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
fn downgrade_support_ahead() -> M5ResolvedFollowUpClosureEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.go_no_go_decision",
        M5SupportedLineTransparencyRole::TransparencyDisclosure,
        M5FollowUpClosureScope::UnclosedActionItem,
        M5SupportedLineOrrHistoryEventSurfaceContext::ShiproomSurface,
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
fn downgrade_form_incomplete() -> M5ResolvedFollowUpClosureEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.action_item_closure",
        M5SupportedLineTransparencyRole::OrrHistoryRetention,
        M5FollowUpClosureScope::UnreconstructableLineHistory,
        M5SupportedLineOrrHistoryEventSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage =
        vec![M5SupportedLineOrrHistoryEventResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn comparison_closure_scope_unclassified() -> M5ResolvedFollowUpClosureEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.cohort_transition",
        M5SupportedLineTransparencyRole::FreshnessWindow,
        M5FollowUpClosureScope::ClosureScopeUnclassified,
        M5SupportedLineOrrHistoryEventSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5SupportedLineTransparencyDowngradeTrigger>,
    orr_history_event_entries: Vec<M5ResolvedOrrHistoryEventEntry>,
    follow_up_closure_entries: Vec<M5ResolvedFollowUpClosureEntry>,
) -> M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesRow {
    M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesRow {
        consumer_surface,
        qualification: M5SupportedLineTransparencyQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        widening_stages: M5SupportedLineTransparencyWideningStage::ALL.to_vec(),
        required_labels: M5SupportedLineTransparencyRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5SupportedLineTransparencyAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5SupportedLineOrrHistoryEventAnatomyPart::ALL.to_vec(),
        export_fields: M5SupportedLineOrrHistoryEventExportField::ALL.to_vec(),
        downgrade_triggers,
        orr_history_event_entries,
        follow_up_closure_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SUPPORTED_LINE_ORR_HISTORY_EVENT_FOLLOW_UP_CLOSURE_REGISTRIES_SCHEMA_REF,
            M5_SUPPORTED_LINE_ORR_HISTORY_DOMAIN_SCHEMA_REF,
            M5_FOLLOW_UP_CLOSURE_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_follow_up_closure_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesRow> {
    use M5SupportedLineTransparencyConsumerSurface as C;
    use M5SupportedLineTransparencyDowngradeTrigger as D;

    vec![
        base_row(
            C::Shiproom,
            "Shiproom owner",
            "The shiproom resolves the line's archived ORR packet to one typed ORR-history event — the recorded readiness decision, its go/no-go outcome, the cohort and freeze context, and the owning roster — from the shared registry and proves the unclosed-action-item follow-up-closure event for that line; an ORR-history event missing its recorded decision evidence and a closure event that keeps a go/no-go claim ahead of recorded decision history degrade honestly instead of leaving an unclosed follow-up to read as still green",
            "downgrade:m5-launch-control-shiproom:001",
            vec![
                D::WidenedClaimOnStalePublicProof,
                D::RanSupportLanguageAheadOfPublicProof,
                D::ProofStale,
            ],
            vec![
                orr_history_event_orr_packet_archive_clean(),
                orr_history_event_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::ReleaseCenter,
            "Release-center owner",
            "The release center resolves the freeze-exception history event and the unreconstructable-line-history follow-up-closure event while keeping the active closure reason visible; a line widening its claim on stale rehearsal evidence and a resolution-form gap on a closure event are caught before a screenshot can reintroduce a still-green reading",
            "downgrade:m5-launch-control-release-center:001",
            vec![
                D::WidenedClaimOnStalePublicProof,
                D::ImpliedGreenWhileProofOrArchiveWasStale,
                D::ProofStale,
            ],
            vec![orr_history_event_freeze_exception_clean(), orr_history_event_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::ExecutiveSteering,
            "Executive-steering owner",
            "Executive steering resolves the cohort-transition history event (public-facing) while keeping its published go/no-go claim matched to recorded decision history and reports the follow-up-closure outcome; an ORR-history event that is a hand-copied per-entry assumption and a closure event on an unclassified closure scope degrade honestly",
            "downgrade:m5-launch-control-executive-steering:001",
            vec![
                D::RegistryReferenceUnstated,
                D::ExportClassUnstated,
                D::ProofStale,
            ],
            vec![
                orr_history_event_cohort_transition_clean(),
                orr_history_event_unbound(),
            ],
            vec![comparison_closure_scope_unclassified()],
        ),
        base_row(
            C::ProgramGovernance,
            "Program-governance owner",
            "Program governance resolves the rehearsal-outcome history event and the stale-rehearsal-evidence follow-up-closure event bound to the registry; an unstated registry token on an ORR-history event is caught before it can drift",
            "downgrade:m5-launch-control-program-governance:001",
            vec![
                D::RegistryReferenceUnstated,
                D::FreshnessWindowUnstated,
                D::ProofStale,
            ],
            vec![
                orr_history_event_rehearsal_outcome_clean(),
                orr_history_event_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics renders the same resolved ORR-history and follow-up-closure truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied line table; the action-item-closure history event and the unreconstructable-line-history closure event stay inspectable off-renderer",
            "downgrade:m5-launch-control-diagnostics:001",
            vec![
                D::FreshnessWindowUnstated,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![orr_history_event_action_item_closure_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved ORR-history and follow-up-closure truth, so a hand-copied constant, an unstated registry token, a widen-on-stale-rehearsal-evidence attempt, or a go/no-go claim running ahead of recorded history is visible in evidence — an unclosed action item, stale rehearsal evidence, or an unreconstructable line history — rather than hidden behind a shiproom note or oral memory",
            "downgrade:m5-launch-control-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::RanSupportLanguageAheadOfPublicProof,
                D::ProofStale,
            ],
            vec![orr_history_event_go_no_go_decision_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesGovernanceReview {
    M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesGovernanceReview {
        orr_history_event_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_orr_history_event_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        follow_up_closure_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        orr_history_event_or_downgrade_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesConsumerProjection
{
    M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesProofFreshness {
    M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesReleasePosture {
    M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesReleasePosture {
        proof_packet_ref:
            M5_SUPPORTED_LINE_ORR_HISTORY_EVENT_FOLLOW_UP_CLOSURE_REGISTRIES_ARTIFACT_REF.to_owned(),
        line_audit_ref: M5_SUPPORTED_LINE_ORR_HISTORY_EVENT_FOLLOW_UP_CLOSURE_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SUPPORTED_LINE_ORR_HISTORY_EVENT_FOLLOW_UP_CLOSURE_REGISTRIES_SCHEMA_REF,
        M5_SUPPORTED_LINE_ORR_HISTORY_EVENT_FOLLOW_UP_CLOSURE_REGISTRIES_DOC_REF,
        M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_SCHEMA_REF,
        M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_DOC_REF,
        M5_SUPPORTED_LINE_ORR_HISTORY_DOMAIN_SCHEMA_REF,
        M5_FOLLOW_UP_CLOSURE_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 supported-line ORR-history-event and follow-up-closure registries packet.
pub fn seeded_m5_supported_line_orr_history_and_follow_up_closure_registries(
) -> M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesPacket {
    M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesPacket::new(
        M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesPacketInput {
            packet_id: M5_SUPPORTED_LINE_ORR_HISTORY_EVENT_FOLLOW_UP_CLOSURE_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 ORR-history-event and follow-up-closure registries archiving one ORR-history event per recorded operational-readiness decision on each active stable or LTS-candidate line — one row per event class: an archived ORR packet, a freeze exception, a rehearsal outcome, a cohort transition, a go/no-go decision, and a post-review action-item closure, tracked against exact build / release-line identity — each bound to one supported-line identity with decision dates, cohort transitions, freeze exceptions, and follow-up closure state, public-safe cohort-transition and go/no-go decision history separated from internal-only freeze / rehearsal / action-item minutiae, recorded decision history preserved so a go/no-go or cohort claim never runs ahead of it, canonical / accessible / audit resolution-form coverage, and a machine-readable periodic follow-up-closure event (unclosed-action-item, stale-rehearsal-evidence, or unreconstructable-line-history) that turns unclosed follow-up work, stale rehearsal evidence, or a line that can no longer be reconstructed from ORR history into a typed event on the active line, naming the active closure reason across release / help, docs, support, and governance surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the shiproom row is held at Beta pending ORR-history-event parity on every event class;
/// every row stays visible and every example stays honest.
pub fn seeded_m5_supported_line_orr_history_and_follow_up_closure_registries_orr_history_event_beta_narrowed(
) -> M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesPacket {
    let mut packet = seeded_m5_supported_line_orr_history_and_follow_up_closure_registries();
    packet.packet_id =
        "m5-supported-line-orr-history-and-follow-up-closure-registries:orr-history-event-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5SupportedLineTransparencyConsumerSurface::Shiproom)
        .expect("shiproom row present");
    row.qualification = M5SupportedLineTransparencyQualificationClass::Beta;
    packet
}

/// Narrowed variant: the release-center row is narrowed to Preview pending follow-up-closure parity on every
/// closure scope; every row stays visible and every example stays honest.
pub fn seeded_m5_supported_line_orr_history_and_follow_up_closure_registries_follow_up_closure_preview_narrowed(
) -> M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesPacket {
    let mut packet = seeded_m5_supported_line_orr_history_and_follow_up_closure_registries();
    packet.packet_id =
        "m5-supported-line-orr-history-and-follow-up-closure-registries:follow-up-closure-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| {
            row.consumer_surface == M5SupportedLineTransparencyConsumerSurface::ReleaseCenter
        })
        .expect("release-center row present");
    row.qualification = M5SupportedLineTransparencyQualificationClass::Preview;
    packet
}
