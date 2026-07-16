//! Canonical seed builders for the M5 line-resolution_memory_row and line-downgrade-packet registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean line-resolution_memory_row and line-downgrade-packet entries
//! are built so the one typed line-resolution_memory_row object resolving per line, lines never widening without
//! preserving rollback and diagnostics, partner / public support language never running ahead of line proof,
//! the canonical / accessible / audit resolution forms, and the complete line-identity / known-limits-ledger /
//! rollback-target / rehearsal-currency / readiness-signoff / support-language / last-widening-revision
//! line-downgrade object are proven across the shiproom, release-center, executive-steering,
//! program-governance, diagnostics, and support surfaces without any hand-copied per-line assumption,
//! widen-without-rollback, incomplete object, hidden line downgrade, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_AI_REVIEW_RESOLUTION_MEMORY_AND_FINDING_LIFECYCLE_REGISTRIES_PACKET_ID: &str =
    "m5-ai-review-resolution-memory-and-finding-lifecycle-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn resolution_memory_row(
    input: M5ResolutionMemoryRowEntryResolutionInput,
) -> M5ResolvedResolutionMemoryRowEntry {
    resolve_resolution_memory_row_entry(input)
        .expect("seed line-resolution_memory_row entry resolves")
}

fn downgrade(
    input: M5FindingLifecycleTransitionEntryResolutionInput,
) -> M5ResolvedFindingLifecycleTransitionEntry {
    resolve_finding_lifecycle_transition_entry(input)
        .expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5ResolutionMemoryRowResolutionForm> {
    M5ResolutionMemoryRowResolutionForm::ALL.to_vec()
}

// -- Clean line-resolution_memory_row entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_resolution_memory_row_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5AiReviewAssistRole,
    report_section: M5ResolutionMemoryRowKind,
    surface_context: M5ResolutionMemoryRowSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5ResolutionMemoryRowEntryResolutionInput {
    M5ResolutionMemoryRowEntryResolutionInput {
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

fn resolution_memory_row_open_clean() -> M5ResolvedResolutionMemoryRowEntry {
    resolution_memory_row(clean_resolution_memory_row_base(
        "resolution_memory_row:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.resolution_memory_row.core_team_canary",
        M5AiReviewAssistRole::FindingClassification,
        M5ResolutionMemoryRowKind::Open,
        M5ResolutionMemoryRowSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn resolution_memory_row_dismissed_clean() -> M5ResolvedResolutionMemoryRowEntry {
    resolution_memory_row(clean_resolution_memory_row_base(
        "resolution_memory_row:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.resolution_memory_row.dismissed",
        M5AiReviewAssistRole::AnalyzedScopeDisclosure,
        M5ResolutionMemoryRowKind::Dismissed,
        M5ResolutionMemoryRowSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn resolution_memory_row_suppressed_clean() -> M5ResolvedResolutionMemoryRowEntry {
    resolution_memory_row(clean_resolution_memory_row_base(
        "resolution_memory_row:program-governance:extension-author",
        "launch.line.extension-author",
        "line.resolution_memory_row.suppressed",
        M5AiReviewAssistRole::PublishDestinationDisclosure,
        M5ResolutionMemoryRowKind::Suppressed,
        M5ResolutionMemoryRowSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn resolution_memory_row_published_clean() -> M5ResolvedResolutionMemoryRowEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_resolution_memory_row_base(
        "resolution_memory_row:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.resolution_memory_row.published",
        M5AiReviewAssistRole::FindingClassification,
        M5ResolutionMemoryRowKind::Published,
        M5ResolutionMemoryRowSurfaceContext::ExecutiveSteeringSurface,
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
    resolution_memory_row(base)
}

fn resolution_memory_row_outdated_clean() -> M5ResolvedResolutionMemoryRowEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_resolution_memory_row_base(
        "resolution_memory_row:support:public-preview",
        "launch.line.public-preview",
        "line.resolution_memory_row.outdated",
        M5AiReviewAssistRole::AnalyzedScopeDisclosure,
        M5ResolutionMemoryRowKind::Outdated,
        M5ResolutionMemoryRowSurfaceContext::SupportOrExportForm,
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
    resolution_memory_row(base)
}

fn resolution_memory_row_reopened_clean() -> M5ResolvedResolutionMemoryRowEntry {
    resolution_memory_row(clean_resolution_memory_row_base(
        "resolution_memory_row:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.resolution_memory_row.reopened",
        M5AiReviewAssistRole::LocalVersusProviderState,
        M5ResolutionMemoryRowKind::Reopened,
        M5ResolutionMemoryRowSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded line-resolution_memory_row entries ---------------------------------------------------------

/// Degraded resolution_memory_row entry: the resolved resolution_memory_row object is incomplete — the bundle IDs are unstated.
fn resolution_memory_row_object_incomplete() -> M5ResolvedResolutionMemoryRowEntry {
    let mut base = clean_resolution_memory_row_base(
        "resolution_memory_row:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.resolution_memory_row.core_team_canary",
        M5AiReviewAssistRole::FindingClassification,
        M5ResolutionMemoryRowKind::Open,
        M5ResolutionMemoryRowSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    resolution_memory_row(base)
}

/// Degraded resolution_memory_row entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn resolution_memory_row_widen_fold() -> M5ResolvedResolutionMemoryRowEntry {
    let mut base = clean_resolution_memory_row_base(
        "resolution_memory_row:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.resolution_memory_row.dismissed",
        M5AiReviewAssistRole::AnalyzedScopeDisclosure,
        M5ResolutionMemoryRowKind::Dismissed,
        M5ResolutionMemoryRowSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    resolution_memory_row(base)
}

/// Degraded resolution_memory_row entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn resolution_memory_row_unbound() -> M5ResolvedResolutionMemoryRowEntry {
    let mut base = clean_resolution_memory_row_base(
        "resolution_memory_row:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.resolution_memory_row.reopened",
        M5AiReviewAssistRole::LocalVersusProviderState,
        M5ResolutionMemoryRowKind::Reopened,
        M5ResolutionMemoryRowSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    resolution_memory_row(base)
}

/// Degraded resolution_memory_row entry: the canonical registry token name is unstated.
fn resolution_memory_row_token_unstated() -> M5ResolvedResolutionMemoryRowEntry {
    let mut base = clean_resolution_memory_row_base(
        "resolution_memory_row:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5AiReviewAssistRole::PublishDestinationDisclosure,
        M5ResolutionMemoryRowKind::Suppressed,
        M5ResolutionMemoryRowSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    resolution_memory_row(base)
}

// -- Clean line-downgrade-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_downgrade_base(
    entry_id: &str,
    comparison_ref: &str,
    token_name: &str,
    semantic_role: M5AiReviewAssistRole,
    comparison_scope: M5FindingLifecycleTransitionScope,
    surface_context: M5ResolutionMemoryRowSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5FindingLifecycleTransitionEntryResolutionInput {
    M5FindingLifecycleTransitionEntryResolutionInput {
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
        keeps_finding_lifecycle_transition_visible: true,
        comparison_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn downgrade_dogfood_ring_clean() -> M5ResolvedFindingLifecycleTransitionEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5AiReviewAssistRole::FindingClassification,
        M5FindingLifecycleTransitionScope::PublishedTransitionJoined,
        M5ResolutionMemoryRowSurfaceContext::ShiproomSurface,
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

fn downgrade_rehearsal_currency_clean() -> M5ResolvedFindingLifecycleTransitionEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.suppressed",
        M5AiReviewAssistRole::PublishDestinationDisclosure,
        M5FindingLifecycleTransitionScope::OutdatedTransitionJoined,
        M5ResolutionMemoryRowSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedFindingLifecycleTransitionEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.reopened",
        M5AiReviewAssistRole::LocalVersusProviderState,
        M5FindingLifecycleTransitionScope::ReopenedTransitionJoined,
        M5ResolutionMemoryRowSurfaceContext::ReleaseCenterSurface,
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
fn downgrade_support_ahead() -> M5ResolvedFindingLifecycleTransitionEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.outdated",
        M5AiReviewAssistRole::AnalyzedScopeDisclosure,
        M5FindingLifecycleTransitionScope::PublishedTransitionJoined,
        M5ResolutionMemoryRowSurfaceContext::ShiproomSurface,
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
fn downgrade_form_incomplete() -> M5ResolvedFindingLifecycleTransitionEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.reopened",
        M5AiReviewAssistRole::LocalVersusProviderState,
        M5FindingLifecycleTransitionScope::ReopenedTransitionJoined,
        M5ResolutionMemoryRowSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5ResolutionMemoryRowResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn comparison_lifecycle_transition_unclassified() -> M5ResolvedFindingLifecycleTransitionEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.published",
        M5AiReviewAssistRole::FindingClassification,
        M5FindingLifecycleTransitionScope::LifecycleTransitionUnclassified,
        M5ResolutionMemoryRowSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5AiReviewResolutionMemoryAndFindingLifecycleRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5AiReviewAssistDowngradeTrigger>,
    resolution_memory_row_entries: Vec<M5ResolvedResolutionMemoryRowEntry>,
    finding_lifecycle_transition_entries: Vec<M5ResolvedFindingLifecycleTransitionEntry>,
) -> M5AiReviewResolutionMemoryAndFindingLifecycleRegistriesRow {
    M5AiReviewResolutionMemoryAndFindingLifecycleRegistriesRow {
        consumer_surface,
        qualification: M5AiReviewAssistQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        capture_lifecycle_stages: M5AiReviewAssistClassificationStage::ALL.to_vec(),
        required_labels: M5AiReviewAssistRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5AiReviewAssistAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5ResolutionMemoryRowAnatomyPart::ALL.to_vec(),
        export_fields: M5ResolutionMemoryRowExportField::ALL.to_vec(),
        downgrade_triggers,
        resolution_memory_row_entries,
        finding_lifecycle_transition_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_AI_REVIEW_RESOLUTION_MEMORY_AND_FINDING_LIFECYCLE_REGISTRIES_SCHEMA_REF,
            M5_AI_REVIEW_RESOLUTION_MEMORY_DOMAIN_SCHEMA_REF,
            M5_AI_REVIEW_FINDING_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_finding_lifecycle_transition_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5AiReviewResolutionMemoryAndFindingLifecycleRegistriesRow> {
    use M5AiReviewAssistConsumerSurface as C;
    use M5AiReviewAssistDowngradeTrigger as D;

    vec![
        base_row(
            C::ReviewDetail,
            "Review-detail owner",
            "Review detail resolves an AI review finding to one durable resolution-memory row — its lifecycle state (open, dismissed, suppressed, published, outdated, reopened), the actor / source and rationale class captured on the transition, the timestamp, the reopen action, and any provider destination or local-draft relation — from the shared registry and proves the finding-lifecycle transition that joins each published, outdated, or reopened state back to the original finding; a row that collapses a transition into an unclassified state and a transition that hides the join back to the original finding degrade honestly instead of letting a dismissal, suppression, or staleness decision disappear into UI state",
            "retirement:m5-release-center:001",
            vec![
                D::PublishModeUnstated,
                D::PublishDestinationHidden,
                D::AiReviewAssistMatrixStale,
            ],
            vec![
                resolution_memory_row_open_clean(),
                resolution_memory_row_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::AiReviewPanel,
            "AI-review-panel owner",
            "The AI review panel resolves the dismiss, suppress, publish, and reopen transitions and the finding-lifecycle transition (a reopened state joined back to prior lineage) while keeping the actor / source and rationale class visible; a resolution-memory row that would collapse a dismissed / suppressed transition into a generic hidden state and a resolution-form gap on a transition are caught before the finding's history becomes unreconstructable",
            "retirement:m5-help-docs:001",
            vec![
                D::PublishModeUnstated,
                D::PublishExportFallbackMissing,
                D::AiReviewAssistMatrixStale,
            ],
            vec![resolution_memory_row_dismissed_clean(), resolution_memory_row_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::SupportExportPacket,
            "Support owner",
            "Support / export resolves the same finding packet — stable ID, actor / source, rationale class, and destination or local-draft relation — keeping the lifecycle available outside the live provider UI without implying provider commitment, and reports the finding-lifecycle transition outcome; a row that is a hand-copied per-entry assumption and a transition on an unclassified lifecycle scope degrade honestly",
            "retirement:m5-support:001",
            vec![
                D::AnalyzedScopeUnstated,
                D::LifecycleStateMissing,
                D::AiReviewAssistMatrixStale,
            ],
            vec![
                resolution_memory_row_published_clean(),
                resolution_memory_row_unbound(),
            ],
            vec![comparison_lifecycle_transition_unclassified()],
        ),
        base_row(
            C::FindingRow,
            "Finding-row owner",
            "The finding row resolves the rationale-class field and the outdated-transition-joined state bound to the registry so a stale finding surfaces as an explicit outdated state joined back to its original finding and diff scope rather than keeping it looking current after drift; an unstated registry token on a resolution-memory row is caught before its history can drift",
            "retirement:m5-marketplace-registry:001",
            vec![
                D::AnalyzedScopeUnstated,
                D::FindingClassBadgeMissing,
                D::AiReviewAssistMatrixStale,
            ],
            vec![
                resolution_memory_row_suppressed_clean(),
                resolution_memory_row_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::ResolutionMemoryLedger,
            "Resolution-memory-ledger owner",
            "The resolution memory ledger renders the same resolved resolution-memory-row and finding-lifecycle-transition truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied history table; the finding's stable ID, actor / source, and rationale class stay inspectable off-renderer so a user can always reconstruct exactly what happened to a finding through dismiss, suppress, publish, outdated, and reopen transitions",
            "retirement:m5-install-update:001",
            vec![
                D::FindingClassBadgeMissing,
                D::AnalyzedScopeUnstated,
                D::AiReviewAssistMatrixStale,
            ],
            vec![resolution_memory_row_reopened_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::ProviderPublishReview,
            "Provider-publish-review owner",
            "The provider publish-review feed carries the same resolved resolution-memory-row and finding-lifecycle-transition truth, so a hand-copied constant, an unstated registry token, a transition collapsing into a generic hidden state, or a published / outdated / reopened state severed from its original finding is visible in evidence — the state joined back to its finding and diff scope — rather than hidden behind a screenshot",
            "retirement:m5-partner-procurement:001",
            vec![
                D::AnalyzedScopeUnstated,
                D::PublishDestinationHidden,
                D::AiReviewAssistMatrixStale,
            ],
            vec![resolution_memory_row_outdated_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5AiReviewResolutionMemoryAndFindingLifecycleRegistriesGovernanceReview {
    M5AiReviewResolutionMemoryAndFindingLifecycleRegistriesGovernanceReview {
        resolution_memory_row_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_resolution_memory_row_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        finding_lifecycle_transition_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        resolution_memory_row_or_downgrade_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5AiReviewResolutionMemoryAndFindingLifecycleRegistriesConsumerProjection
{
    M5AiReviewResolutionMemoryAndFindingLifecycleRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5AiReviewResolutionMemoryAndFindingLifecycleRegistriesProofFreshness {
    M5AiReviewResolutionMemoryAndFindingLifecycleRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5AiReviewResolutionMemoryAndFindingLifecycleRegistriesReleasePosture {
    M5AiReviewResolutionMemoryAndFindingLifecycleRegistriesReleasePosture {
        proof_packet_ref:
            M5_AI_REVIEW_RESOLUTION_MEMORY_AND_FINDING_LIFECYCLE_REGISTRIES_ARTIFACT_REF.to_owned(),
        line_audit_ref: M5_AI_REVIEW_RESOLUTION_MEMORY_AND_FINDING_LIFECYCLE_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_AI_REVIEW_RESOLUTION_MEMORY_AND_FINDING_LIFECYCLE_REGISTRIES_SCHEMA_REF,
        M5_AI_REVIEW_RESOLUTION_MEMORY_AND_FINDING_LIFECYCLE_REGISTRIES_DOC_REF,
        M5_AI_REVIEW_ASSIST_MATRIX_SCHEMA_REF,
        M5_AI_REVIEW_ASSIST_MATRIX_DOC_REF,
        M5_AI_REVIEW_RESOLUTION_MEMORY_DOMAIN_SCHEMA_REF,
        M5_AI_REVIEW_FINDING_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 post-launch public-proof-ledger and claim-history-diff registries packet.
pub fn seeded_m5_ai_review_resolution_memory_and_finding_lifecycle_registries(
) -> M5AiReviewResolutionMemoryAndFindingLifecycleRegistriesPacket {
    M5AiReviewResolutionMemoryAndFindingLifecycleRegistriesPacket::new(
        M5AiReviewResolutionMemoryAndFindingLifecycleRegistriesPacketInput {
            packet_id: M5_AI_REVIEW_RESOLUTION_MEMORY_AND_FINDING_LIFECYCLE_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 resolution-memory-row and finding-lifecycle-transition registries recording one durable resolution-memory row per AI review finding-state transition — one typed field per section: the finding's lifecycle state (open, dismissed, suppressed, published, outdated, reopened), the actor / source and rationale class captured on the transition without shaming language or anthropomorphic copy, the timestamp, the reopen action, and any provider destination or local-draft relation — each bound to one object-class identity, so a dismissed finding never collapses into the same generic hidden state as a suppressed one and a stale finding never keeps looking current after diff or instruction drift, with canonical / accessible / audit resolution-form coverage, and a machine-readable finding-lifecycle transition (published-transition-joined, outdated-transition-joined, or reopened-transition-joined) that joins each published, outdated, or reopened state back to the original finding and diff scope so later review and support exports can reconstruct the full lifecycle, and preserves the same finding packet — stable ID, actor / source, rationale class, and destination or local-draft relation — in local history and support / export so the lifecycle stays available after restart, export, and support capture without implying provider commitment where none exists, across review, AI, provider, and support / export surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5AiReviewResolutionMemoryAndFindingLifecycleRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the shiproom row is held at Beta pending public-proof-ledger parity on every proof source;
/// every row stays visible and every example stays honest.
pub fn seeded_m5_ai_review_resolution_memory_and_finding_lifecycle_registries_resolution_memory_row_beta_narrowed(
) -> M5AiReviewResolutionMemoryAndFindingLifecycleRegistriesPacket {
    let mut packet = seeded_m5_ai_review_resolution_memory_and_finding_lifecycle_registries();
    packet.packet_id =
        "m5-ai-review-resolution-memory-and-finding-lifecycle-registries:snapshot-descriptor-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5AiReviewAssistConsumerSurface::ReviewDetail)
        .expect("shiproom row present");
    row.qualification = M5AiReviewAssistQualificationClass::Beta;
    packet
}

/// Narrowed variant: the release-center row is narrowed to Preview pending claim-history-diff parity on every
/// diff scope; every row stays visible and every example stays honest.
pub fn seeded_m5_ai_review_resolution_memory_and_finding_lifecycle_registries_finding_lifecycle_transition_preview_narrowed(
) -> M5AiReviewResolutionMemoryAndFindingLifecycleRegistriesPacket {
    let mut packet = seeded_m5_ai_review_resolution_memory_and_finding_lifecycle_registries();
    packet.packet_id =
        "m5-ai-review-resolution-memory-and-finding-lifecycle-registries:finding-lifecycle-transition-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5AiReviewAssistConsumerSurface::AiReviewPanel)
        .expect("release-center row present");
    row.qualification = M5AiReviewAssistQualificationClass::Preview;
    packet
}
