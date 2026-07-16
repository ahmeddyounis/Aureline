//! Canonical seed builders for the M5 line-retirement_review_packet and line-downgrade-packet registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean line-retirement_review_packet and line-downgrade-packet entries
//! are built so the one typed line-retirement_review_packet object resolving per line, lines never widening without
//! preserving rollback and diagnostics, partner / public support language never running ahead of line proof,
//! the canonical / accessible / audit resolution forms, and the complete line-identity / known-limits-ledger /
//! rollback-target / rehearsal-currency / readiness-signoff / support-language / last-widening-revision
//! line-downgrade object are proven across the shiproom, release-center, executive-steering,
//! program-governance, diagnostics, and support surfaces without any hand-copied per-line assumption,
//! widen-without-rollback, incomplete object, hidden line downgrade, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_RETIREMENT_REVIEW_PACKET_CLOSURE_GATE_REGISTRIES_PACKET_ID: &str =
    "m5-retirement-review-packet-and-closure-gate-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn retirement_review_packet(
    input: M5RetirementReviewPacketEntryResolutionInput,
) -> M5ResolvedRetirementReviewPacketEntry {
    resolve_retirement_review_packet_entry(input)
        .expect("seed line-retirement_review_packet entry resolves")
}

fn downgrade(input: M5ClosureGateEntryResolutionInput) -> M5ResolvedClosureGateEntry {
    resolve_closure_gate_entry(input).expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5RetirementReviewPacketResolutionForm> {
    M5RetirementReviewPacketResolutionForm::ALL.to_vec()
}

// -- Clean line-retirement_review_packet entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_retirement_review_packet_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5RetiredStateRole,
    report_section: M5RetirementReviewPacketKind,
    surface_context: M5RetirementReviewPacketSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5RetirementReviewPacketEntryResolutionInput {
    M5RetirementReviewPacketEntryResolutionInput {
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

fn retirement_review_packet_exact_build_snapshot_ref_clean() -> M5ResolvedRetirementReviewPacketEntry
{
    retirement_review_packet(clean_retirement_review_packet_base(
        "retirement_review_packet:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.retirement_review_packet.core_team_canary",
        M5RetiredStateRole::LastSupportedPin,
        M5RetirementReviewPacketKind::ExactBuildSnapshotRef,
        M5RetirementReviewPacketSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn retirement_review_packet_final_compatibility_public_proof_join_clean(
) -> M5ResolvedRetirementReviewPacketEntry {
    retirement_review_packet(clean_retirement_review_packet_base(
        "retirement_review_packet:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.retirement_review_packet.final_compatibility_public_proof_join",
        M5RetiredStateRole::SuccessorRouting,
        M5RetirementReviewPacketKind::FinalCompatibilityPublicProofJoin,
        M5RetirementReviewPacketSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn retirement_review_packet_unresolved_dependent_blocker_clean(
) -> M5ResolvedRetirementReviewPacketEntry {
    retirement_review_packet(clean_retirement_review_packet_base(
        "retirement_review_packet:program-governance:extension-author",
        "launch.line.extension-author",
        "line.retirement_review_packet.unresolved_dependent_blocker",
        M5RetiredStateRole::DisablePath,
        M5RetirementReviewPacketKind::UnresolvedDependentBlocker,
        M5RetirementReviewPacketSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn retirement_review_packet_support_note_closure_status_clean(
) -> M5ResolvedRetirementReviewPacketEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_retirement_review_packet_base(
        "retirement_review_packet:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.retirement_review_packet.support_note_closure_status",
        M5RetiredStateRole::LastSupportedPin,
        M5RetirementReviewPacketKind::SupportNoteClosureStatus,
        M5RetirementReviewPacketSurfaceContext::ExecutiveSteeringSurface,
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
    retirement_review_packet(base)
}

fn retirement_review_packet_migration_outcome_summary_clean(
) -> M5ResolvedRetirementReviewPacketEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_retirement_review_packet_base(
        "retirement_review_packet:support:public-preview",
        "launch.line.public-preview",
        "line.retirement_review_packet.migration_outcome_summary",
        M5RetiredStateRole::SuccessorRouting,
        M5RetirementReviewPacketKind::MigrationOutcomeSummary,
        M5RetirementReviewPacketSurfaceContext::SupportOrExportForm,
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
    retirement_review_packet(base)
}

fn retirement_review_packet_archival_signoff_ref_clean() -> M5ResolvedRetirementReviewPacketEntry {
    retirement_review_packet(clean_retirement_review_packet_base(
        "retirement_review_packet:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.retirement_review_packet.archival_signoff_ref",
        M5RetiredStateRole::SupportNoteClosure,
        M5RetirementReviewPacketKind::ArchivalSignoffRef,
        M5RetirementReviewPacketSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded line-retirement_review_packet entries ---------------------------------------------------------

/// Degraded retirement_review_packet entry: the resolved retirement_review_packet object is incomplete — the bundle IDs are unstated.
fn retirement_review_packet_object_incomplete() -> M5ResolvedRetirementReviewPacketEntry {
    let mut base = clean_retirement_review_packet_base(
        "retirement_review_packet:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.retirement_review_packet.core_team_canary",
        M5RetiredStateRole::LastSupportedPin,
        M5RetirementReviewPacketKind::ExactBuildSnapshotRef,
        M5RetirementReviewPacketSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    retirement_review_packet(base)
}

/// Degraded retirement_review_packet entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn retirement_review_packet_widen_fold() -> M5ResolvedRetirementReviewPacketEntry {
    let mut base = clean_retirement_review_packet_base(
        "retirement_review_packet:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.retirement_review_packet.final_compatibility_public_proof_join",
        M5RetiredStateRole::SuccessorRouting,
        M5RetirementReviewPacketKind::FinalCompatibilityPublicProofJoin,
        M5RetirementReviewPacketSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    retirement_review_packet(base)
}

/// Degraded retirement_review_packet entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn retirement_review_packet_unbound() -> M5ResolvedRetirementReviewPacketEntry {
    let mut base = clean_retirement_review_packet_base(
        "retirement_review_packet:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.retirement_review_packet.archival_signoff_ref",
        M5RetiredStateRole::SupportNoteClosure,
        M5RetirementReviewPacketKind::ArchivalSignoffRef,
        M5RetirementReviewPacketSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    retirement_review_packet(base)
}

/// Degraded retirement_review_packet entry: the canonical registry token name is unstated.
fn retirement_review_packet_token_unstated() -> M5ResolvedRetirementReviewPacketEntry {
    let mut base = clean_retirement_review_packet_base(
        "retirement_review_packet:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5RetiredStateRole::DisablePath,
        M5RetirementReviewPacketKind::UnresolvedDependentBlocker,
        M5RetirementReviewPacketSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    retirement_review_packet(base)
}

// -- Clean line-downgrade-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_downgrade_base(
    entry_id: &str,
    comparison_ref: &str,
    token_name: &str,
    semantic_role: M5RetiredStateRole,
    comparison_scope: M5ClosureGateScope,
    surface_context: M5RetirementReviewPacketSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5ClosureGateEntryResolutionInput {
    M5ClosureGateEntryResolutionInput {
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
        keeps_closure_gate_visible: true,
        comparison_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn downgrade_dogfood_ring_clean() -> M5ResolvedClosureGateEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5RetiredStateRole::LastSupportedPin,
        M5ClosureGateScope::IncompleteRetirementReviewPacket,
        M5RetirementReviewPacketSurfaceContext::ShiproomSurface,
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

fn downgrade_rehearsal_currency_clean() -> M5ResolvedClosureGateEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.unresolved_dependent_blocker",
        M5RetiredStateRole::DisablePath,
        M5ClosureGateScope::UnclosedSupportNoteSurface,
        M5RetirementReviewPacketSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedClosureGateEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.archival_signoff_ref",
        M5RetiredStateRole::SupportNoteClosure,
        M5ClosureGateScope::SilentlyDroppedException,
        M5RetirementReviewPacketSurfaceContext::ReleaseCenterSurface,
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
fn downgrade_support_ahead() -> M5ResolvedClosureGateEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.migration_outcome_summary",
        M5RetiredStateRole::SuccessorRouting,
        M5ClosureGateScope::IncompleteRetirementReviewPacket,
        M5RetirementReviewPacketSurfaceContext::ShiproomSurface,
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
fn downgrade_form_incomplete() -> M5ResolvedClosureGateEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.archival_signoff_ref",
        M5RetiredStateRole::SupportNoteClosure,
        M5ClosureGateScope::SilentlyDroppedException,
        M5RetirementReviewPacketSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5RetirementReviewPacketResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn comparison_closure_gate_scope_unclassified() -> M5ResolvedClosureGateEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.support_note_closure_status",
        M5RetiredStateRole::LastSupportedPin,
        M5ClosureGateScope::ClosureGateScopeUnclassified,
        M5RetirementReviewPacketSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5RetirementReviewPacketClosureGateRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5RetiredStateDowngradeTrigger>,
    retirement_review_packet_entries: Vec<M5ResolvedRetirementReviewPacketEntry>,
    closure_gate_entries: Vec<M5ResolvedClosureGateEntry>,
) -> M5RetirementReviewPacketClosureGateRegistriesRow {
    M5RetirementReviewPacketClosureGateRegistriesRow {
        consumer_surface,
        qualification: M5RetiredStateQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        removal_horizon_stages: M5RetiredStateRemovalHorizonStage::ALL.to_vec(),
        required_labels: M5RetiredStateRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5RetiredStateAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5RetirementReviewPacketAnatomyPart::ALL.to_vec(),
        export_fields: M5RetirementReviewPacketExportField::ALL.to_vec(),
        downgrade_triggers,
        retirement_review_packet_entries,
        closure_gate_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_RETIREMENT_REVIEW_PACKET_CLOSURE_GATE_REGISTRIES_SCHEMA_REF,
            M5_RETIREMENT_REVIEW_PACKET_DOMAIN_SCHEMA_REF,
            M5_CLOSURE_GATE_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_closure_gate_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5RetirementReviewPacketClosureGateRegistriesRow> {
    use M5RetiredStateConsumerSurface as C;
    use M5RetiredStateDowngradeTrigger as D;

    vec![
        base_row(
            C::ReleaseCenter,
            "Release-center owner",
            "The release center resolves a retirement candidate to one typed retirement-review-packet object — the classified packet field (here the exact-build snapshot ref), its owning team, the exact-build joins, and the migration outcome or archival signoff — from the shared registry and proves the incomplete-retirement-review-packet closure gate for that candidate; a review packet missing its exact-build joins and a gate that keeps support language ahead of the closed support note degrade honestly instead of leaving a candidate to read as safe to close",
            "retirement:m5-release-center:001",
            vec![
                D::SuccessorPathUnnamed,
                D::SupportNoteClosureIncomplete,
                D::RetirementManifestStale,
            ],
            vec![
                retirement_review_packet_exact_build_snapshot_ref_clean(),
                retirement_review_packet_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::HelpDocs,
            "Help/docs owner",
            "Help / docs resolves the final-compatibility-public-proof-join packet field and the silently-dropped-exception closure gate while keeping the active gate reason visible; a candidate flipping to Retired without a completed packet and a resolution-form gap on a gate are caught before a screenshot can reintroduce a safe-to-close reading",
            "retirement:m5-help-docs:001",
            vec![
                D::SuccessorPathUnnamed,
                D::ArchivalNoteMissing,
                D::RetirementManifestStale,
            ],
            vec![retirement_review_packet_final_compatibility_public_proof_join_clean(), retirement_review_packet_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::Support,
            "Support owner",
            "Support resolves the support-note-closure-status packet field while keeping its public-facing support-note / migration claim matched to the closed support note and reports the support-note-closure-gate outcome; a review-packet entry that is a hand-copied per-entry assumption and a gate on an unclassified gate scope degrade honestly",
            "retirement:m5-support:001",
            vec![
                D::RegistryReferenceUnstated,
                D::DisablePathUnnamed,
                D::RetirementManifestStale,
            ],
            vec![
                retirement_review_packet_support_note_closure_status_clean(),
                retirement_review_packet_unbound(),
            ],
            vec![comparison_closure_gate_scope_unclassified()],
        ),
        base_row(
            C::MarketplaceRegistry,
            "Marketplace/registry owner",
            "The marketplace / registry resolves the unresolved-dependent-blocker packet field and the unclosed-support-note-surface closure gate bound to the registry so a retiring surface can no longer be selected in a new install or by a new tenant while its review packet is still incomplete; an unstated registry token on a review-packet entry is caught before it can drift",
            "retirement:m5-marketplace-registry:001",
            vec![
                D::RegistryReferenceUnstated,
                D::CutoffDateUnstated,
                D::RetirementManifestStale,
            ],
            vec![
                retirement_review_packet_unresolved_dependent_blocker_clean(),
                retirement_review_packet_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::InstallUpdate,
            "Install/update owner",
            "Install / update renders the same resolved retirement-review-packet and support-note-closure-gate truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied class table; the archival-signoff-ref packet field and the silently-dropped-exception closure gate stay inspectable off-renderer so no new install can still select a retiring surface with an open pre-closure blocker",
            "retirement:m5-install-update:001",
            vec![
                D::CutoffDateUnstated,
                D::RegistryReferenceUnstated,
                D::RetirementManifestStale,
            ],
            vec![retirement_review_packet_archival_signoff_ref_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::PartnerProcurement,
            "Partner/procurement owner",
            "The partner / procurement feed carries the same resolved retirement-review-packet and support-note-closure-gate truth, so a hand-copied constant, an unstated registry token, a candidate flipping to Retired without a completed packet, or support language running ahead of the closed support note is visible in evidence — a candidate with an incomplete review packet, an unclosed support-note surface, or a silently dropped exception — rather than hidden behind a screenshot",
            "retirement:m5-partner-procurement:001",
            vec![
                D::RegistryReferenceUnstated,
                D::SupportNoteClosureIncomplete,
                D::RetirementManifestStale,
            ],
            vec![retirement_review_packet_migration_outcome_summary_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5RetirementReviewPacketClosureGateRegistriesGovernanceReview {
    M5RetirementReviewPacketClosureGateRegistriesGovernanceReview {
        retirement_review_packet_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_retirement_review_packet_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        closure_gate_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        retirement_review_packet_or_downgrade_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5RetirementReviewPacketClosureGateRegistriesConsumerProjection {
    M5RetirementReviewPacketClosureGateRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5RetirementReviewPacketClosureGateRegistriesProofFreshness {
    M5RetirementReviewPacketClosureGateRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5RetirementReviewPacketClosureGateRegistriesReleasePosture {
    M5RetirementReviewPacketClosureGateRegistriesReleasePosture {
        proof_packet_ref: M5_RETIREMENT_REVIEW_PACKET_CLOSURE_GATE_REGISTRIES_ARTIFACT_REF
            .to_owned(),
        line_audit_ref: M5_RETIREMENT_REVIEW_PACKET_CLOSURE_GATE_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_RETIREMENT_REVIEW_PACKET_CLOSURE_GATE_REGISTRIES_SCHEMA_REF,
        M5_RETIREMENT_REVIEW_PACKET_CLOSURE_GATE_REGISTRIES_DOC_REF,
        M5_RETIRED_STATE_MATRIX_SCHEMA_REF,
        M5_RETIRED_STATE_MATRIX_DOC_REF,
        M5_RETIREMENT_REVIEW_PACKET_DOMAIN_SCHEMA_REF,
        M5_CLOSURE_GATE_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 retirement-review-packet and closure-gate registries packet.
pub fn seeded_m5_retirement_review_packet_and_closure_gate_registries(
) -> M5RetirementReviewPacketClosureGateRegistriesPacket {
    M5RetirementReviewPacketClosureGateRegistriesPacket::new(
        M5RetirementReviewPacketClosureGateRegistriesPacketInput {
            packet_id: M5_RETIREMENT_REVIEW_PACKET_CLOSURE_GATE_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 retirement-review-packet and support-note-closure-gate registries forcing one export-safe retirement review packet per retirement candidate before a line or stable-facing surface can move to Retired — one classified packet field per joined fact (the exact-build snapshot ref, the final compatibility / public-proof join, the unresolved dependent blocker, the support-note closure status, the migration outcome summary, or the archival signoff ref) with an owning team and joined to the retirement manifest and impact report, so retirement stops being an ad hoc decision buried in release notes and becomes a completed, inspectable proof of readiness, historical closure, and user-facing honesty, with canonical / accessible / audit resolution-form coverage, and a machine-readable support-note closure gate (incomplete-retirement-review-packet, unclosed-support-note-surface, or silently-dropped-exception) that blocks final retirement while the packet is missing its migration outcome or archival refs, still has an unclosed help / support / partner / procurement / incident surface, or would silently drop a recorded exception, so support, help, and public-proof consumers read the closure state directly from the packet and no object reaches Retired without a completed packet that records who approved it, what evidence was accepted, which surfaces were closed or redirected, and what exceptions remain"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5RetirementReviewPacketClosureGateRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the release-center row is held at Beta pending retirement-review-packet parity on every
/// packet field; every row stays visible and every example stays honest.
pub fn seeded_m5_retirement_review_packet_and_closure_gate_registries_retirement_review_packet_beta_narrowed(
) -> M5RetirementReviewPacketClosureGateRegistriesPacket {
    let mut packet = seeded_m5_retirement_review_packet_and_closure_gate_registries();
    packet.packet_id =
        "m5-retirement-review-packet-and-closure-gate-registries:retirement-review-packet-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5RetiredStateConsumerSurface::ReleaseCenter)
        .expect("release-center row present");
    row.qualification = M5RetiredStateQualificationClass::Beta;
    packet
}

/// Narrowed variant: the help/docs row is narrowed to Preview pending support-note-closure-gate parity on every
/// gate scope; every row stays visible and every example stays honest.
pub fn seeded_m5_retirement_review_packet_and_closure_gate_registries_closure_gate_preview_narrowed(
) -> M5RetirementReviewPacketClosureGateRegistriesPacket {
    let mut packet = seeded_m5_retirement_review_packet_and_closure_gate_registries();
    packet.packet_id =
        "m5-retirement-review-packet-and-closure-gate-registries:support-note-closure-gate-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5RetiredStateConsumerSurface::HelpDocs)
        .expect("help/docs row present");
    row.qualification = M5RetiredStateQualificationClass::Preview;
    packet
}
