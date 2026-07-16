//! Canonical seed builders for the M5 line-canonical_source_relation and line-downgrade-packet registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean line-canonical_source_relation and line-downgrade-packet entries
//! are built so the one typed line-canonical_source_relation object resolving per line, lines never widening without
//! preserving rollback and diagnostics, partner / public support language never running ahead of line proof,
//! the canonical / accessible / audit resolution forms, and the complete line-identity / known-limits-ledger /
//! rollback-target / rehearsal-currency / readiness-signoff / support-language / last-widening-revision
//! line-downgrade object are proven across the shiproom, release-center, executive-steering,
//! program-governance, diagnostics, and support surfaces without any hand-copied per-line assumption,
//! widen-without-rollback, incomplete object, hidden line downgrade, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_CANONICAL_SOURCE_RELATION_WRITE_TARGET_REVIEW_REGISTRIES_PACKET_ID: &str =
    "m5-canonical-source-relation-and-write-target-review-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn canonical_source_relation(
    input: M5CanonicalSourceRelationEntryResolutionInput,
) -> M5ResolvedCanonicalSourceRelationEntry {
    resolve_canonical_source_relation_entry(input)
        .expect("seed line-canonical_source_relation entry resolves")
}

fn downgrade(input: M5WriteTargetReviewEntryResolutionInput) -> M5ResolvedWriteTargetReviewEntry {
    resolve_write_target_review_entry(input).expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5CanonicalSourceRelationResolutionForm> {
    M5CanonicalSourceRelationResolutionForm::ALL.to_vec()
}

// -- Clean line-canonical_source_relation entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_canonical_source_relation_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5ConstrainedFileStateRole,
    report_section: M5CanonicalSourceRelationKind,
    surface_context: M5CanonicalSourceRelationSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5CanonicalSourceRelationEntryResolutionInput {
    M5CanonicalSourceRelationEntryResolutionInput {
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

fn canonical_source_relation_read_only_path_object_clean() -> M5ResolvedCanonicalSourceRelationEntry
{
    canonical_source_relation(clean_canonical_source_relation_base(
        "canonical_source_relation:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.canonical_source_relation.core_team_canary",
        M5ConstrainedFileStateRole::StateBadgeClassification,
        M5CanonicalSourceRelationKind::ReadOnlyPathObject,
        M5CanonicalSourceRelationSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn canonical_source_relation_generated_artifact_object_clean(
) -> M5ResolvedCanonicalSourceRelationEntry {
    canonical_source_relation(clean_canonical_source_relation_base(
        "canonical_source_relation:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.canonical_source_relation.generated_artifact_object",
        M5ConstrainedFileStateRole::BlockedWriteReason,
        M5CanonicalSourceRelationKind::GeneratedArtifactObject,
        M5CanonicalSourceRelationSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn canonical_source_relation_policy_locked_object_clean() -> M5ResolvedCanonicalSourceRelationEntry
{
    canonical_source_relation(clean_canonical_source_relation_base(
        "canonical_source_relation:program-governance:extension-author",
        "launch.line.extension-author",
        "line.canonical_source_relation.policy_locked_object",
        M5ConstrainedFileStateRole::CanonicalSourceRelation,
        M5CanonicalSourceRelationKind::PolicyLockedObject,
        M5CanonicalSourceRelationSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn canonical_source_relation_managed_mirrored_object_clean(
) -> M5ResolvedCanonicalSourceRelationEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_canonical_source_relation_base(
        "canonical_source_relation:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.canonical_source_relation.managed_mirrored_object",
        M5ConstrainedFileStateRole::StateBadgeClassification,
        M5CanonicalSourceRelationKind::ManagedMirroredObject,
        M5CanonicalSourceRelationSurfaceContext::ExecutiveSteeringSurface,
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
    canonical_source_relation(base)
}

fn canonical_source_relation_projection_object_clean() -> M5ResolvedCanonicalSourceRelationEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_canonical_source_relation_base(
        "canonical_source_relation:support:public-preview",
        "launch.line.public-preview",
        "line.canonical_source_relation.projection_object",
        M5ConstrainedFileStateRole::BlockedWriteReason,
        M5CanonicalSourceRelationKind::ProjectionObject,
        M5CanonicalSourceRelationSurfaceContext::SupportOrExportForm,
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
    canonical_source_relation(base)
}

fn canonical_source_relation_captured_snapshot_object_clean(
) -> M5ResolvedCanonicalSourceRelationEntry {
    canonical_source_relation(clean_canonical_source_relation_base(
        "canonical_source_relation:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.canonical_source_relation.captured_snapshot_object",
        M5ConstrainedFileStateRole::ExactWriteTarget,
        M5CanonicalSourceRelationKind::CapturedSnapshotObject,
        M5CanonicalSourceRelationSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded line-canonical_source_relation entries ---------------------------------------------------------

/// Degraded canonical_source_relation entry: the resolved canonical_source_relation object is incomplete — the bundle IDs are unstated.
fn canonical_source_relation_object_incomplete() -> M5ResolvedCanonicalSourceRelationEntry {
    let mut base = clean_canonical_source_relation_base(
        "canonical_source_relation:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.canonical_source_relation.core_team_canary",
        M5ConstrainedFileStateRole::StateBadgeClassification,
        M5CanonicalSourceRelationKind::ReadOnlyPathObject,
        M5CanonicalSourceRelationSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    canonical_source_relation(base)
}

/// Degraded canonical_source_relation entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn canonical_source_relation_widen_fold() -> M5ResolvedCanonicalSourceRelationEntry {
    let mut base = clean_canonical_source_relation_base(
        "canonical_source_relation:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.canonical_source_relation.generated_artifact_object",
        M5ConstrainedFileStateRole::BlockedWriteReason,
        M5CanonicalSourceRelationKind::GeneratedArtifactObject,
        M5CanonicalSourceRelationSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    canonical_source_relation(base)
}

/// Degraded canonical_source_relation entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn canonical_source_relation_unbound() -> M5ResolvedCanonicalSourceRelationEntry {
    let mut base = clean_canonical_source_relation_base(
        "canonical_source_relation:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.canonical_source_relation.captured_snapshot_object",
        M5ConstrainedFileStateRole::ExactWriteTarget,
        M5CanonicalSourceRelationKind::CapturedSnapshotObject,
        M5CanonicalSourceRelationSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    canonical_source_relation(base)
}

/// Degraded canonical_source_relation entry: the canonical registry token name is unstated.
fn canonical_source_relation_token_unstated() -> M5ResolvedCanonicalSourceRelationEntry {
    let mut base = clean_canonical_source_relation_base(
        "canonical_source_relation:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5ConstrainedFileStateRole::CanonicalSourceRelation,
        M5CanonicalSourceRelationKind::PolicyLockedObject,
        M5CanonicalSourceRelationSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    canonical_source_relation(base)
}

// -- Clean line-downgrade-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_downgrade_base(
    entry_id: &str,
    comparison_ref: &str,
    token_name: &str,
    semantic_role: M5ConstrainedFileStateRole,
    comparison_scope: M5WriteTargetReviewScope,
    surface_context: M5CanonicalSourceRelationSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5WriteTargetReviewEntryResolutionInput {
    M5WriteTargetReviewEntryResolutionInput {
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
        keeps_write_target_review_visible: true,
        comparison_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn downgrade_dogfood_ring_clean() -> M5ResolvedWriteTargetReviewEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5ConstrainedFileStateRole::StateBadgeClassification,
        M5WriteTargetReviewScope::StateClassChange,
        M5CanonicalSourceRelationSurfaceContext::ShiproomSurface,
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

fn downgrade_rehearsal_currency_clean() -> M5ResolvedWriteTargetReviewEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.policy_locked_object",
        M5ConstrainedFileStateRole::CanonicalSourceRelation,
        M5WriteTargetReviewScope::CanonicalSourceChange,
        M5CanonicalSourceRelationSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedWriteTargetReviewEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.captured_snapshot_object",
        M5ConstrainedFileStateRole::ExactWriteTarget,
        M5WriteTargetReviewScope::WriteTargetChange,
        M5CanonicalSourceRelationSurfaceContext::ReleaseCenterSurface,
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
fn downgrade_support_ahead() -> M5ResolvedWriteTargetReviewEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.projection_object",
        M5ConstrainedFileStateRole::BlockedWriteReason,
        M5WriteTargetReviewScope::StateClassChange,
        M5CanonicalSourceRelationSurfaceContext::ShiproomSurface,
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
fn downgrade_form_incomplete() -> M5ResolvedWriteTargetReviewEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.captured_snapshot_object",
        M5ConstrainedFileStateRole::ExactWriteTarget,
        M5WriteTargetReviewScope::WriteTargetChange,
        M5CanonicalSourceRelationSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5CanonicalSourceRelationResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn comparison_diff_scope_unclassified() -> M5ResolvedWriteTargetReviewEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.managed_mirrored_object",
        M5ConstrainedFileStateRole::StateBadgeClassification,
        M5WriteTargetReviewScope::DiffScopeUnclassified,
        M5CanonicalSourceRelationSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5CanonicalSourceRelationWriteTargetReviewRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5ConstrainedFileStateDowngradeTrigger>,
    canonical_source_relation_entries: Vec<M5ResolvedCanonicalSourceRelationEntry>,
    write_target_review_entries: Vec<M5ResolvedWriteTargetReviewEntry>,
) -> M5CanonicalSourceRelationWriteTargetReviewRegistriesRow {
    M5CanonicalSourceRelationWriteTargetReviewRegistriesRow {
        consumer_surface,
        qualification: M5ConstrainedFileStateQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        capture_lifecycle_stages: M5ConstrainedFileStateClassificationStage::ALL.to_vec(),
        required_labels: M5ConstrainedFileStateRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5ConstrainedFileStateAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5CanonicalSourceRelationAnatomyPart::ALL.to_vec(),
        export_fields: M5CanonicalSourceRelationExportField::ALL.to_vec(),
        downgrade_triggers,
        canonical_source_relation_entries,
        write_target_review_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_CANONICAL_SOURCE_RELATION_WRITE_TARGET_REVIEW_REGISTRIES_SCHEMA_REF,
            M5_CANONICAL_SOURCE_RELATION_DOMAIN_SCHEMA_REF,
            M5_WRITE_TARGET_REVIEW_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_write_target_review_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5CanonicalSourceRelationWriteTargetReviewRegistriesRow> {
    use M5ConstrainedFileStateConsumerSurface as C;
    use M5ConstrainedFileStateDowngradeTrigger as D;

    vec![
        base_row(
            C::TabChrome,
            "Release-center owner",
            "The release center resolves a retiring class to one typed canonical-source-relation object — the object identity, last-supported version / channel pinned to an exact build, retirement trigger, cutoff date, successor reference, disable path, and export / rollback route — from the shared registry and proves the cutoff-date-change diff for that class; a manifest object missing its exact-build joins and a diff that keeps support language ahead of the closed support note degrade honestly instead of leaving a retired class to read as still supported",
            "retirement:m5-release-center:001",
            vec![
                D::ExactWriteTargetUnstated,
                D::BlockedWriteReasonMissing,
                D::ConstrainedFileStateDescriptorStale,
            ],
            vec![
                canonical_source_relation_read_only_path_object_clean(),
                canonical_source_relation_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::StatusBar,
            "Help/docs owner",
            "Help / docs resolves the retirement-trigger field and the disable-or-export-route-change diff while keeping the active diff reason visible; a retiring class widening its claim without a preserved rollback / export route and a resolution-form gap on a diff are caught before a screenshot can reintroduce a still-supported reading",
            "retirement:m5-help-docs:001",
            vec![
                D::ExactWriteTargetUnstated,
                D::NearestSafeActionMissing,
                D::ConstrainedFileStateDescriptorStale,
            ],
            vec![canonical_source_relation_generated_artifact_object_clean(), canonical_source_relation_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::SupportExportPacket,
            "Support owner",
            "Support resolves the successor-reference field while keeping its public-facing successor / exit claim matched to the closed support note and reports the write-target-review outcome; a manifest entry that is a hand-copied per-entry assumption and a diff on an unclassified diff scope degrade honestly",
            "retirement:m5-support:001",
            vec![
                D::CanonicalSourceUnstated,
                D::PreservedVersusLostSyncUnstated,
                D::ConstrainedFileStateDescriptorStale,
            ],
            vec![
                canonical_source_relation_managed_mirrored_object_clean(),
                canonical_source_relation_unbound(),
            ],
            vec![comparison_diff_scope_unclassified()],
        ),
        base_row(
            C::DiffReviewHeader,
            "Marketplace/registry owner",
            "The marketplace / registry resolves the cutoff-date field and the replacement-path-change diff bound to the registry so a retired class can no longer be selected in a new install or by a new tenant; an unstated registry token on a manifest entry is caught before it can drift",
            "retirement:m5-marketplace-registry:001",
            vec![
                D::CanonicalSourceUnstated,
                D::StateBadgeMissing,
                D::ConstrainedFileStateDescriptorStale,
            ],
            vec![
                canonical_source_relation_policy_locked_object_clean(),
                canonical_source_relation_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::CommandPalette,
            "Install/update owner",
            "Install / update renders the same resolved canonical-source-relation and write-target-review truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied class table; the export / rollback-route field and the disable-or-export-route-change diff stay inspectable off-renderer so no new install can still select a retired class",
            "retirement:m5-install-update:001",
            vec![
                D::StateBadgeMissing,
                D::CanonicalSourceUnstated,
                D::ConstrainedFileStateDescriptorStale,
            ],
            vec![canonical_source_relation_captured_snapshot_object_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::AiAutomationPath,
            "Partner/procurement owner",
            "The partner / procurement feed carries the same resolved canonical-source-relation and write-target-review truth, so a hand-copied constant, an unstated registry token, a manifest widening its claim without a preserved rollback / export route, or support language running ahead of the closed support note is visible in evidence — a cutoff-date change, a replacement-path change, or a disable / export-route change — rather than hidden behind a screenshot",
            "retirement:m5-partner-procurement:001",
            vec![
                D::CanonicalSourceUnstated,
                D::BlockedWriteReasonMissing,
                D::ConstrainedFileStateDescriptorStale,
            ],
            vec![canonical_source_relation_projection_object_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5CanonicalSourceRelationWriteTargetReviewRegistriesGovernanceReview {
    M5CanonicalSourceRelationWriteTargetReviewRegistriesGovernanceReview {
        canonical_source_relation_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_canonical_source_relation_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        write_target_review_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        canonical_source_relation_or_downgrade_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5CanonicalSourceRelationWriteTargetReviewRegistriesConsumerProjection {
    M5CanonicalSourceRelationWriteTargetReviewRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5CanonicalSourceRelationWriteTargetReviewRegistriesProofFreshness {
    M5CanonicalSourceRelationWriteTargetReviewRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5CanonicalSourceRelationWriteTargetReviewRegistriesReleasePosture {
    M5CanonicalSourceRelationWriteTargetReviewRegistriesReleasePosture {
        proof_packet_ref: M5_CANONICAL_SOURCE_RELATION_WRITE_TARGET_REVIEW_REGISTRIES_ARTIFACT_REF
            .to_owned(),
        line_audit_ref: M5_CANONICAL_SOURCE_RELATION_WRITE_TARGET_REVIEW_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_CANONICAL_SOURCE_RELATION_WRITE_TARGET_REVIEW_REGISTRIES_SCHEMA_REF,
        M5_CANONICAL_SOURCE_RELATION_WRITE_TARGET_REVIEW_REGISTRIES_DOC_REF,
        M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF,
        M5_CONSTRAINED_FILE_STATE_MATRIX_DOC_REF,
        M5_CANONICAL_SOURCE_RELATION_DOMAIN_SCHEMA_REF,
        M5_WRITE_TARGET_REVIEW_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 post-launch public-proof-ledger and claim-history-diff registries packet.
pub fn seeded_m5_canonical_source_relation_and_write_target_review_registries(
) -> M5CanonicalSourceRelationWriteTargetReviewRegistriesPacket {
    M5CanonicalSourceRelationWriteTargetReviewRegistriesPacket::new(
        M5CanonicalSourceRelationWriteTargetReviewRegistriesPacketInput {
            packet_id: M5_CANONICAL_SOURCE_RELATION_WRITE_TARGET_REVIEW_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 canonical-source-relation and write-target-review registries emitting one machine-readable retirement manifest per retiring supported line or stable-facing capability — one typed field per manifest section: the last-supported version / channel pinned to an exact build, the retirement trigger, the cutoff date, the successor reference, the disable path, and the export / rollback route — each bound to one object-class identity with its exact-build joins, so a retired class never disappears silently and no new install or new tenant can still select it, with canonical / accessible / audit resolution-form coverage, and a machine-readable manifest-change diff (cutoff-date-change, replacement-path-change, or disable-or-export-route-change) that turns a changed cutoff date or replacement path into a visible, typed diff event rather than a silent mutation across CLI, docs / help, partner-packet, and support-bundle surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5CanonicalSourceRelationWriteTargetReviewRegistriesVocabularySet::canonical(),
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
pub fn seeded_m5_canonical_source_relation_and_write_target_review_registries_canonical_source_relation_beta_narrowed(
) -> M5CanonicalSourceRelationWriteTargetReviewRegistriesPacket {
    let mut packet = seeded_m5_canonical_source_relation_and_write_target_review_registries();
    packet.packet_id =
        "m5-canonical-source-relation-and-write-target-review-registries:canonical-source-relation-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ConstrainedFileStateConsumerSurface::TabChrome)
        .expect("shiproom row present");
    row.qualification = M5ConstrainedFileStateQualificationClass::Beta;
    packet
}

/// Narrowed variant: the release-center row is narrowed to Preview pending claim-history-diff parity on every
/// diff scope; every row stays visible and every example stays honest.
pub fn seeded_m5_canonical_source_relation_and_write_target_review_registries_write_target_review_preview_narrowed(
) -> M5CanonicalSourceRelationWriteTargetReviewRegistriesPacket {
    let mut packet = seeded_m5_canonical_source_relation_and_write_target_review_registries();
    packet.packet_id =
        "m5-canonical-source-relation-and-write-target-review-registries:write-target-review-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ConstrainedFileStateConsumerSurface::StatusBar)
        .expect("release-center row present");
    row.qualification = M5ConstrainedFileStateQualificationClass::Preview;
    packet
}
