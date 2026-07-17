//! Canonical seed builders for the M5 ownership-signal-row and owner-conflict registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean ownership-signal-row and owner-conflict entries
//! are built so the one typed ownership-signal row resolving per owned slice, an advisory owner never
//! promoted into an enforced merge gate, the owner source class (repo rule / graph overlay / provider
//! metadata) never dropped on export, the canonical / accessible / audit resolution forms, and the complete
//! owner-authority / owner-source-provenance / owner-conflict-rationale reconciliation object are proven
//! across the review-detail, AI-review, review-pack-summary, local-CI-parity, provider-handoff, and support
//! surfaces without any hand-copied per-slice assumption, advisory-flattened-into-enforced owner, incomplete
//! object, hidden owner conflict, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_OWNERSHIP_SIGNAL_AND_CONFLICT_REGISTRIES_PACKET_ID: &str =
    "m5-ownership-signal-and-conflict-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn ownership_signal_row(
    input: M5OwnershipSignalRowEntryResolutionInput,
) -> M5ResolvedOwnershipSignalRowEntry {
    resolve_ownership_signal_row_entry(input)
        .expect("seed line-ownership_signal_row entry resolves")
}

fn downgrade(input: M5OwnerConflictEntryResolutionInput) -> M5ResolvedOwnerConflictEntry {
    resolve_owner_conflict_entry(input).expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5OwnershipSignalRowResolutionForm> {
    M5OwnershipSignalRowResolutionForm::ALL.to_vec()
}

// -- Clean line-ownership_signal_row entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_ownership_signal_row_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5ReviewPackRole,
    report_section: M5OwnershipSignalRowKind,
    surface_context: M5OwnershipSignalRowSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5OwnershipSignalRowEntryResolutionInput {
    M5OwnershipSignalRowEntryResolutionInput {
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

fn ownership_signal_row_codeowners_rule_owner_clean() -> M5ResolvedOwnershipSignalRowEntry {
    ownership_signal_row(clean_ownership_signal_row_base(
        "ownership_signal_row:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.ownership_signal_row.core_team_canary",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5OwnershipSignalRowKind::CodeownersRuleOwner,
        M5OwnershipSignalRowSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn ownership_signal_row_graph_overlay_maintainer_clean() -> M5ResolvedOwnershipSignalRowEntry {
    ownership_signal_row(clean_ownership_signal_row_base(
        "ownership_signal_row:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.ownership_signal_row.graph_overlay_maintainer",
        M5ReviewPackRole::PackVersionAndDigestDisclosure,
        M5OwnershipSignalRowKind::GraphOverlayMaintainer,
        M5OwnershipSignalRowSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn ownership_signal_row_provider_suggested_reviewer_clean() -> M5ResolvedOwnershipSignalRowEntry {
    ownership_signal_row(clean_ownership_signal_row_base(
        "ownership_signal_row:program-governance:extension-author",
        "launch.line.extension-author",
        "line.ownership_signal_row.provider_suggested_reviewer",
        M5ReviewPackRole::OwnerProvenanceDisclosure,
        M5OwnershipSignalRowKind::ProviderSuggestedReviewer,
        M5OwnershipSignalRowSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn ownership_signal_row_enforced_review_gate_owner_clean() -> M5ResolvedOwnershipSignalRowEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_ownership_signal_row_base(
        "ownership_signal_row:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.ownership_signal_row.enforced_review_gate_owner",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5OwnershipSignalRowKind::EnforcedReviewGateOwner,
        M5OwnershipSignalRowSurfaceContext::ExecutiveSteeringSurface,
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
    ownership_signal_row(base)
}

fn ownership_signal_row_advisory_area_owner_clean() -> M5ResolvedOwnershipSignalRowEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_ownership_signal_row_base(
        "ownership_signal_row:support:public-preview",
        "launch.line.public-preview",
        "line.ownership_signal_row.advisory_area_owner",
        M5ReviewPackRole::PackVersionAndDigestDisclosure,
        M5OwnershipSignalRowKind::AdvisoryAreaOwner,
        M5OwnershipSignalRowSurfaceContext::SupportOrExportForm,
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
    ownership_signal_row(base)
}

fn ownership_signal_row_fallback_default_owner_clean() -> M5ResolvedOwnershipSignalRowEntry {
    ownership_signal_row(clean_ownership_signal_row_base(
        "ownership_signal_row:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.ownership_signal_row.fallback_default_owner",
        M5ReviewPackRole::LocalVersusProviderParityDisclosure,
        M5OwnershipSignalRowKind::FallbackDefaultOwner,
        M5OwnershipSignalRowSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded line-ownership_signal_row entries ---------------------------------------------------------

/// Degraded ownership_signal_row entry: the resolved ownership_signal_row object is incomplete — the bundle IDs are unstated.
fn ownership_signal_row_object_incomplete() -> M5ResolvedOwnershipSignalRowEntry {
    let mut base = clean_ownership_signal_row_base(
        "ownership_signal_row:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.ownership_signal_row.core_team_canary",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5OwnershipSignalRowKind::CodeownersRuleOwner,
        M5OwnershipSignalRowSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    ownership_signal_row(base)
}

/// Degraded ownership_signal_row entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn ownership_signal_row_widen_fold() -> M5ResolvedOwnershipSignalRowEntry {
    let mut base = clean_ownership_signal_row_base(
        "ownership_signal_row:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.ownership_signal_row.graph_overlay_maintainer",
        M5ReviewPackRole::PackVersionAndDigestDisclosure,
        M5OwnershipSignalRowKind::GraphOverlayMaintainer,
        M5OwnershipSignalRowSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    ownership_signal_row(base)
}

/// Degraded ownership_signal_row entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn ownership_signal_row_unbound() -> M5ResolvedOwnershipSignalRowEntry {
    let mut base = clean_ownership_signal_row_base(
        "ownership_signal_row:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.ownership_signal_row.fallback_default_owner",
        M5ReviewPackRole::LocalVersusProviderParityDisclosure,
        M5OwnershipSignalRowKind::FallbackDefaultOwner,
        M5OwnershipSignalRowSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    ownership_signal_row(base)
}

/// Degraded ownership_signal_row entry: the canonical registry token name is unstated.
fn ownership_signal_row_token_unstated() -> M5ResolvedOwnershipSignalRowEntry {
    let mut base = clean_ownership_signal_row_base(
        "ownership_signal_row:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5ReviewPackRole::OwnerProvenanceDisclosure,
        M5OwnershipSignalRowKind::ProviderSuggestedReviewer,
        M5OwnershipSignalRowSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    ownership_signal_row(base)
}

// -- Clean line-downgrade-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_downgrade_base(
    entry_id: &str,
    comparison_ref: &str,
    token_name: &str,
    semantic_role: M5ReviewPackRole,
    comparison_scope: M5OwnerConflictScope,
    surface_context: M5OwnershipSignalRowSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5OwnerConflictEntryResolutionInput {
    M5OwnerConflictEntryResolutionInput {
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
        keeps_owner_conflict_visible: true,
        comparison_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn downgrade_dogfood_ring_clean() -> M5ResolvedOwnerConflictEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5OwnerConflictScope::OwnerAuthorityBinding,
        M5OwnershipSignalRowSurfaceContext::ShiproomSurface,
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

fn downgrade_rehearsal_currency_clean() -> M5ResolvedOwnerConflictEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.provider_suggested_reviewer",
        M5ReviewPackRole::OwnerProvenanceDisclosure,
        M5OwnerConflictScope::OwnerSourceProvenanceBinding,
        M5OwnershipSignalRowSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedOwnerConflictEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.fallback_default_owner",
        M5ReviewPackRole::LocalVersusProviderParityDisclosure,
        M5OwnerConflictScope::OwnerConflictRationaleBinding,
        M5OwnershipSignalRowSurfaceContext::ReleaseCenterSurface,
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
fn downgrade_support_ahead() -> M5ResolvedOwnerConflictEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.advisory_area_owner",
        M5ReviewPackRole::PackVersionAndDigestDisclosure,
        M5OwnerConflictScope::OwnerAuthorityBinding,
        M5OwnershipSignalRowSurfaceContext::ShiproomSurface,
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
fn downgrade_form_incomplete() -> M5ResolvedOwnerConflictEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.fallback_default_owner",
        M5ReviewPackRole::LocalVersusProviderParityDisclosure,
        M5OwnerConflictScope::OwnerConflictRationaleBinding,
        M5OwnershipSignalRowSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5OwnershipSignalRowResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn comparison_owner_conflict_unclassified() -> M5ResolvedOwnerConflictEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.enforced_review_gate_owner",
        M5ReviewPackRole::EvaluatorResultClassDisclosure,
        M5OwnerConflictScope::OwnerConflictUnclassified,
        M5OwnershipSignalRowSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5OwnershipSignalAndConflictRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5ReviewPackDowngradeTrigger>,
    ownership_signal_row_entries: Vec<M5ResolvedOwnershipSignalRowEntry>,
    owner_conflict_entries: Vec<M5ResolvedOwnerConflictEntry>,
) -> M5OwnershipSignalAndConflictRegistriesRow {
    M5OwnershipSignalAndConflictRegistriesRow {
        consumer_surface,
        qualification: M5ReviewPackQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        capture_lifecycle_stages: M5ReviewPackClassificationStage::ALL.to_vec(),
        required_labels: M5ReviewPackRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5ReviewPackAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5OwnershipSignalRowAnatomyPart::ALL.to_vec(),
        export_fields: M5OwnershipSignalRowExportField::ALL.to_vec(),
        downgrade_triggers,
        ownership_signal_row_entries,
        owner_conflict_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_OWNERSHIP_SIGNAL_AND_CONFLICT_REGISTRIES_SCHEMA_REF,
            M5_OWNERSHIP_SIGNAL_DOMAIN_SCHEMA_REF,
            M5_OWNER_CONFLICT_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_owner_conflict_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5OwnershipSignalAndConflictRegistriesRow> {
    use M5ReviewPackConsumerSurface as C;
    use M5ReviewPackDowngradeTrigger as D;

    vec![
        base_row(
            C::ReviewDetail,
            "Review-detail owner",
            "Review detail resolves each owned slice to one typed ownership-signal row from the shared registry — the owner source class (a CODEOWNERS repo rule, a graph-overlay maintainer, or a provider-suggested reviewer) and the advisory-versus-enforced owner authority, never flattened into one ambiguous owner pill — and proves the owner-authority-binding reconciliation for that slice; an ownership row missing its owner provenance and a reconciliation that would promote an advisory owner into an enforced merge gate degrade honestly instead of reading as an authoritative owner signal",
            "ownership-signal:m5-review-detail:001",
            vec![
                D::PackVersionDigestUnstated,
                D::UnevaluatedCheckHiddenBehindGreenSummary,
                D::ReviewPackMatrixStale,
            ],
            vec![
                ownership_signal_row_codeowners_rule_owner_clean(),
                ownership_signal_row_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::AiReviewPanel,
            "AI-review owner",
            "The AI review panel resolves the owner-source-provenance binding and the owner-conflict rationale while keeping which owner came from a repo rule, a graph overlay, or provider metadata visible; an ownership row flattening advisory and enforced owners and a resolution-form gap on a reconciliation are caught before a green summary can hide the disagreement, and AI review never runs under an undisclosed owner set",
            "ownership-signal:m5-ai-review:001",
            vec![
                D::PackVersionDigestUnstated,
                D::PackVersionOrDigestDropped,
                D::ReviewPackMatrixStale,
            ],
            vec![ownership_signal_row_graph_overlay_maintainer_clean(), ownership_signal_row_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::SupportExportPacket,
            "Support owner",
            "Support resolves the enforced-review-gate owner while keeping the owner source class and reviewer rationale bound to the export, and reports the owner-conflict reconciliation; an ownership row that is a hand-copied per-entry assumption and a reconciliation on an unclassified binding degrade honestly so owner provenance and rationale are never dropped on export or reopen",
            "ownership-signal:m5-support:001",
            vec![
                D::ParityStateUnstated,
                D::PackFreshnessUnstated,
                D::ReviewPackMatrixStale,
            ],
            vec![
                ownership_signal_row_enforced_review_gate_owner_clean(),
                ownership_signal_row_unbound(),
            ],
            vec![comparison_owner_conflict_unclassified()],
        ),
        base_row(
            C::ReviewPackSummary,
            "Review-pack-summary owner",
            "The review-pack summary resolves the provider-suggested reviewer and the owner-source-provenance reconciliation — repo rule versus graph overlay versus provider metadata — bound to the registry so a provider suggestion can no longer silently overwrite a CODEOWNERS repo rule; an unstated owner provenance on a row is caught before it can drift",
            "ownership-signal:m5-review-pack-summary:001",
            vec![
                D::ParityStateUnstated,
                D::EvaluatorResultClassUnstated,
                D::ReviewPackMatrixStale,
            ],
            vec![
                ownership_signal_row_provider_suggested_reviewer_clean(),
                ownership_signal_row_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::LocalCiParityStrip,
            "Local-CI-parity owner",
            "The local-CI parity strip renders the same resolved ownership-signal-row and owner-conflict truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied table; the advisory-versus-enforced owner authority and the owner-authority-binding reconciliation stay inspectable off-renderer so an advisory owner never reads as an enforced merge gate",
            "ownership-signal:m5-local-ci-parity:001",
            vec![
                D::EvaluatorResultClassUnstated,
                D::ParityStateUnstated,
                D::ReviewPackMatrixStale,
            ],
            vec![ownership_signal_row_fallback_default_owner_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::ProviderHandoff,
            "Provider-handoff owner",
            "The provider handoff feed carries the same resolved ownership-signal-row and owner-conflict truth, so the conflicting-owner set — a CODEOWNERS repo rule, a graph-overlay maintainer, and a provider-suggested reviewer disagreeing at once — stays visible with an explicit winning-versus-advisory relationship carried by the owner-conflict-rationale reconciliation rather than collapsed into one owner pill or hidden behind a green summary",
            "ownership-signal:m5-provider-handoff:001",
            vec![
                D::ParityStateUnstated,
                D::UnevaluatedCheckHiddenBehindGreenSummary,
                D::ReviewPackMatrixStale,
            ],
            vec![ownership_signal_row_advisory_area_owner_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5OwnershipSignalAndConflictRegistriesGovernanceReview {
    M5OwnershipSignalAndConflictRegistriesGovernanceReview {
        ownership_signal_row_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_ownership_signal_row_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        owner_conflict_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        ownership_signal_row_or_downgrade_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5OwnershipSignalAndConflictRegistriesConsumerProjection {
    M5OwnershipSignalAndConflictRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5OwnershipSignalAndConflictRegistriesProofFreshness {
    M5OwnershipSignalAndConflictRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5OwnershipSignalAndConflictRegistriesReleasePosture {
    M5OwnershipSignalAndConflictRegistriesReleasePosture {
        proof_packet_ref: M5_OWNERSHIP_SIGNAL_AND_CONFLICT_REGISTRIES_ARTIFACT_REF.to_owned(),
        line_audit_ref: M5_OWNERSHIP_SIGNAL_AND_CONFLICT_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_OWNERSHIP_SIGNAL_AND_CONFLICT_REGISTRIES_SCHEMA_REF,
        M5_OWNERSHIP_SIGNAL_AND_CONFLICT_REGISTRIES_DOC_REF,
        M5_REVIEW_PACK_MATRIX_SCHEMA_REF,
        M5_REVIEW_PACK_MATRIX_DOC_REF,
        M5_OWNERSHIP_SIGNAL_DOMAIN_SCHEMA_REF,
        M5_OWNER_CONFLICT_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 ownership-signal-row and owner-conflict registries packet.
pub fn seeded_m5_ownership_signal_and_conflict_registries(
) -> M5OwnershipSignalAndConflictRegistriesPacket {
    M5OwnershipSignalAndConflictRegistriesPacket::new(
        M5OwnershipSignalAndConflictRegistriesPacketInput {
            packet_id: M5_OWNERSHIP_SIGNAL_AND_CONFLICT_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 ownership-signal-row and owner-conflict registries emitting one machine-readable ownership-signal row per owned slice — the owner source class (a CODEOWNERS repo rule, a graph-overlay maintainer, or a provider-suggested reviewer) and the advisory-versus-enforced owner authority, never flattened into one ambiguous owner pill — each bound to its pack association with its reviewer rationale, so an exported review / support packet never drops the owner source class or rationale and no advisory owner is silently promoted into an enforced merge gate, with canonical / accessible / audit resolution-form coverage, and a machine-readable owner-conflict reconciliation (owner-authority-binding, owner-source-provenance-binding, or owner-conflict-rationale-binding) that turns a disagreement between a repo rule, a graph-derived maintainer, and a provider suggestion into a visible, explained event with an explicit winning-versus-advisory relationship rather than a silent last-writer-wins collapse across review lists, review detail, merge-readiness, AI-review, browser handoff, and support / export surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5OwnershipSignalAndConflictRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the review-detail row is held at Beta pending ownership-signal-row parity on every pack
/// version / digest; every row stays visible and every example stays honest.
pub fn seeded_m5_ownership_signal_and_conflict_registries_ownership_signal_row_beta_narrowed(
) -> M5OwnershipSignalAndConflictRegistriesPacket {
    let mut packet = seeded_m5_ownership_signal_and_conflict_registries();
    packet.packet_id =
        "m5-ownership-signal-and-conflict-registries:ownership-signal-row-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ReviewPackConsumerSurface::ReviewDetail)
        .expect("review-detail row present");
    row.qualification = M5ReviewPackQualificationClass::Beta;
    packet
}

/// Narrowed variant: the AI-review row is narrowed to Preview pending owner-conflict parity on every
/// evaluator binding; every row stays visible and every example stays honest.
pub fn seeded_m5_ownership_signal_and_conflict_registries_owner_conflict_preview_narrowed(
) -> M5OwnershipSignalAndConflictRegistriesPacket {
    let mut packet = seeded_m5_ownership_signal_and_conflict_registries();
    packet.packet_id =
        "m5-ownership-signal-and-conflict-registries:owner-conflict-preview:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ReviewPackConsumerSurface::AiReviewPanel)
        .expect("AI-review row present");
    row.qualification = M5ReviewPackQualificationClass::Preview;
    packet
}
