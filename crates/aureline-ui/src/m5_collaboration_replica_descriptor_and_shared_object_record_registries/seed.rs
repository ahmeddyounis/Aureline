//! Canonical seed builders for the M5 review-pack-record and review-pack-result registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean review-pack-record and review-pack-result entries
//! are built so the one typed review-pack-record object resolving per pack, records never widening a local
//! parity estimate into provider-authoritative mergeability, divergence labels never hidden behind a green
//! summary, the canonical / accessible / audit resolution forms, and the complete evaluated-scope / pack
//! version-and-digest / target-diff / worktree-base / evaluator-outcome review-pack-result object are proven
//! across the review-detail, AI-review, review-pack-summary, local-CI-parity, provider-handoff, and support
//! surfaces without any hand-copied per-pack assumption, estimate-shown-as-authoritative, incomplete object,
//! hidden divergence label, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_COLLABORATION_REPLICA_DESCRIPTOR_AND_SHARED_OBJECT_RECORD_REGISTRIES_PACKET_ID: &str =
    "m5-collaboration-replica-descriptor-and-shared-object-record-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn review_pack_record(
    input: M5CollaborationReplicaRecordEntryResolutionInput,
) -> M5ResolvedCollaborationReplicaRecordEntry {
    resolve_review_pack_record_entry(input).expect("seed line-review_pack_record entry resolves")
}

fn downgrade(
    input: M5CollaborationReplicaResultEntryResolutionInput,
) -> M5ResolvedCollaborationReplicaResultEntry {
    resolve_review_pack_result_entry(input).expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5CollaborationReplicaRecordResolutionForm> {
    M5CollaborationReplicaRecordResolutionForm::ALL.to_vec()
}

// -- Clean line-review_pack_record entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_review_pack_record_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5CollaborationStateRole,
    report_section: M5CollaborationReplicaRecordKind,
    surface_context: M5CollaborationReplicaRecordSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5CollaborationReplicaRecordEntryResolutionInput {
    M5CollaborationReplicaRecordEntryResolutionInput {
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

fn review_pack_record_changed_files_scope_clean() -> M5ResolvedCollaborationReplicaRecordEntry {
    review_pack_record(clean_review_pack_record_base(
        "review_pack_record:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.review_pack_record.core_team_canary",
        M5CollaborationStateRole::AuthorityModelDisclosure,
        M5CollaborationReplicaRecordKind::ChangedFilesScope,
        M5CollaborationReplicaRecordSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn review_pack_record_pull_request_scope_clean() -> M5ResolvedCollaborationReplicaRecordEntry {
    review_pack_record(clean_review_pack_record_base(
        "review_pack_record:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.review_pack_record.pull_request_scope",
        M5CollaborationStateRole::LocalTruthPreservationDisclosure,
        M5CollaborationReplicaRecordKind::PullRequestScope,
        M5CollaborationReplicaRecordSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn review_pack_record_base_head_range_scope_clean() -> M5ResolvedCollaborationReplicaRecordEntry {
    review_pack_record(clean_review_pack_record_base(
        "review_pack_record:program-governance:extension-author",
        "launch.line.extension-author",
        "line.review_pack_record.base_head_range_scope",
        M5CollaborationStateRole::MergeAndDriftSemanticsDisclosure,
        M5CollaborationReplicaRecordKind::BaseHeadRangeScope,
        M5CollaborationReplicaRecordSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn review_pack_record_worktree_uncommitted_scope_clean() -> M5ResolvedCollaborationReplicaRecordEntry
{
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_review_pack_record_base(
        "review_pack_record:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.review_pack_record.worktree_uncommitted_scope",
        M5CollaborationStateRole::AuthorityModelDisclosure,
        M5CollaborationReplicaRecordKind::WorktreeUncommittedScope,
        M5CollaborationReplicaRecordSurfaceContext::ExecutiveSteeringSurface,
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
    review_pack_record(base)
}

fn review_pack_record_full_tree_scope_clean() -> M5ResolvedCollaborationReplicaRecordEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_review_pack_record_base(
        "review_pack_record:support:public-preview",
        "launch.line.public-preview",
        "line.review_pack_record.full_tree_scope",
        M5CollaborationStateRole::LocalTruthPreservationDisclosure,
        M5CollaborationReplicaRecordKind::FullTreeScope,
        M5CollaborationReplicaRecordSurfaceContext::SupportOrExportForm,
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
    review_pack_record(base)
}

fn review_pack_record_saved_pack_snapshot_scope_clean() -> M5ResolvedCollaborationReplicaRecordEntry
{
    review_pack_record(clean_review_pack_record_base(
        "review_pack_record:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.review_pack_record.saved_pack_snapshot_scope",
        M5CollaborationStateRole::DowngradeBehaviorDisclosure,
        M5CollaborationReplicaRecordKind::SavedPackSnapshotScope,
        M5CollaborationReplicaRecordSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded line-review_pack_record entries ---------------------------------------------------------

/// Degraded review_pack_record entry: the resolved review_pack_record object is incomplete — the bundle IDs are unstated.
fn review_pack_record_object_incomplete() -> M5ResolvedCollaborationReplicaRecordEntry {
    let mut base = clean_review_pack_record_base(
        "review_pack_record:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.review_pack_record.core_team_canary",
        M5CollaborationStateRole::AuthorityModelDisclosure,
        M5CollaborationReplicaRecordKind::ChangedFilesScope,
        M5CollaborationReplicaRecordSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.bundle_ids = "   ".to_owned();
    review_pack_record(base)
}

/// Degraded review_pack_record entry: the line's rollback and diagnostics posture is not preserved before widening —
/// a line widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn review_pack_record_widen_fold() -> M5ResolvedCollaborationReplicaRecordEntry {
    let mut base = clean_review_pack_record_base(
        "review_pack_record:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.review_pack_record.pull_request_scope",
        M5CollaborationStateRole::LocalTruthPreservationDisclosure,
        M5CollaborationReplicaRecordKind::PullRequestScope,
        M5CollaborationReplicaRecordSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.rollback_and_diagnostics_bounded = false;
    review_pack_record(base)
}

/// Degraded review_pack_record entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn review_pack_record_unbound() -> M5ResolvedCollaborationReplicaRecordEntry {
    let mut base = clean_review_pack_record_base(
        "review_pack_record:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.review_pack_record.saved_pack_snapshot_scope",
        M5CollaborationStateRole::DowngradeBehaviorDisclosure,
        M5CollaborationReplicaRecordKind::SavedPackSnapshotScope,
        M5CollaborationReplicaRecordSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-journey-journeys",
        "bundle.ids.certified-0007",
        "install.topology.certified-journey-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-journey",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    review_pack_record(base)
}

/// Degraded review_pack_record entry: the canonical registry token name is unstated.
fn review_pack_record_token_unstated() -> M5ResolvedCollaborationReplicaRecordEntry {
    let mut base = clean_review_pack_record_base(
        "review_pack_record:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5CollaborationStateRole::MergeAndDriftSemanticsDisclosure,
        M5CollaborationReplicaRecordKind::BaseHeadRangeScope,
        M5CollaborationReplicaRecordSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    review_pack_record(base)
}

// -- Clean line-downgrade-packet entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_downgrade_base(
    entry_id: &str,
    comparison_ref: &str,
    token_name: &str,
    semantic_role: M5CollaborationStateRole,
    comparison_scope: M5CollaborationReplicaResultScope,
    surface_context: M5CollaborationReplicaRecordSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5CollaborationReplicaResultEntryResolutionInput {
    M5CollaborationReplicaResultEntryResolutionInput {
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
        keeps_review_pack_result_visible: true,
        comparison_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

fn downgrade_dogfood_ring_clean() -> M5ResolvedCollaborationReplicaResultEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5CollaborationStateRole::AuthorityModelDisclosure,
        M5CollaborationReplicaResultScope::EvaluatedScopeBinding,
        M5CollaborationReplicaRecordSurfaceContext::ShiproomSurface,
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

fn downgrade_rehearsal_currency_clean() -> M5ResolvedCollaborationReplicaResultEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.base_head_range_scope",
        M5CollaborationStateRole::MergeAndDriftSemanticsDisclosure,
        M5CollaborationReplicaResultScope::PackVersionDigestBinding,
        M5CollaborationReplicaRecordSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedCollaborationReplicaResultEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.saved_pack_snapshot_scope",
        M5CollaborationStateRole::DowngradeBehaviorDisclosure,
        M5CollaborationReplicaResultScope::DivergenceLabelBinding,
        M5CollaborationReplicaRecordSurfaceContext::ReleaseCenterSurface,
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
fn downgrade_support_ahead() -> M5ResolvedCollaborationReplicaResultEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.full_tree_scope",
        M5CollaborationStateRole::LocalTruthPreservationDisclosure,
        M5CollaborationReplicaResultScope::EvaluatedScopeBinding,
        M5CollaborationReplicaRecordSurfaceContext::ShiproomSurface,
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
fn downgrade_form_incomplete() -> M5ResolvedCollaborationReplicaResultEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.saved_pack_snapshot_scope",
        M5CollaborationStateRole::DowngradeBehaviorDisclosure,
        M5CollaborationReplicaResultScope::DivergenceLabelBinding,
        M5CollaborationReplicaRecordSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage =
        vec![M5CollaborationReplicaRecordResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn comparison_review_pack_result_unclassified() -> M5ResolvedCollaborationReplicaResultEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.worktree_uncommitted_scope",
        M5CollaborationStateRole::AuthorityModelDisclosure,
        M5CollaborationReplicaResultScope::ChangeObjectResultUnclassified,
        M5CollaborationReplicaRecordSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5CollaborationReplicaRecordAndResultRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5CollaborationStateDowngradeTrigger>,
    review_pack_record_entries: Vec<M5ResolvedCollaborationReplicaRecordEntry>,
    review_pack_result_entries: Vec<M5ResolvedCollaborationReplicaResultEntry>,
) -> M5CollaborationReplicaRecordAndResultRegistriesRow {
    M5CollaborationReplicaRecordAndResultRegistriesRow {
        consumer_surface,
        qualification: M5CollaborationStateQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        capture_lifecycle_stages: M5CollaborationStateClassificationStage::ALL.to_vec(),
        required_labels: M5CollaborationStateRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5CollaborationStateAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5CollaborationReplicaRecordAnatomyPart::ALL.to_vec(),
        export_fields: M5CollaborationReplicaRecordExportField::ALL.to_vec(),
        downgrade_triggers,
        review_pack_record_entries,
        review_pack_result_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_COLLABORATION_REPLICA_DESCRIPTOR_AND_SHARED_OBJECT_RECORD_REGISTRIES_SCHEMA_REF,
            M5_CRDT_BACKED_SHARED_TEXT_DOMAIN_SCHEMA_REF,
            M5_SHARED_OBJECT_DESCRIPTOR_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_review_pack_result_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5CollaborationReplicaRecordAndResultRegistriesRow> {
    use M5CollaborationStateConsumerSurface as C;
    use M5CollaborationStateDowngradeTrigger as D;

    vec![
        base_row(
            C::SharedEditorReplicaView,
            "Shared-editor-replica-view owner",
            "The shared editor replica view resolves each shared object it renders to one typed collaboration replica descriptor — its workspace-root and buffer / object identity, revision and session epoch, trust and policy epoch, and export posture — and to the shared-object record naming the CRDT-convergent shared editable text as a convergent replica of the local buffer rather than the canonical buffer itself; entering share mode creates the replica record without replacing local buffer authority, and a descriptor that cannot bind its buffer identity or that would discard unsent local edits on a permission or relay downgrade degrades honestly instead of letting the replica overwrite local canonical truth",
            "collaboration-replica:m5-shared-editor-replica-view:001",
            vec![
                D::LocalTruthPreservationUnstated,
                D::UnsentLocalEditsDiscardedOnDowngrade,
                D::CollaborationStateMatrixStale,
            ],
            vec![
                review_pack_record_changed_files_scope_clean(),
                review_pack_record_object_incomplete(),
            ],
            vec![downgrade_dogfood_ring_clean(), downgrade_support_ahead()],
        ),
        base_row(
            C::PresenceCursorLayer,
            "Presence-cursor-layer owner",
            "The presence / cursor layer resolves the sampled presence, cursor, and selection shared objects to one shared-object record that declares them sampled and non-authoritative — presence-only, never convergence truth — and carries the session provenance and freshness a consumer must see; a presence sample presented as authoritative shared state and a stale sample shown as current degrade honestly instead of letting sampled presence read as the converged buffer",
            "collaboration-replica:m5-presence-cursor-layer:001",
            vec![
                D::LocalTruthPreservationUnstated,
                D::ProvenanceOrFreshnessUnstated,
                D::CollaborationStateMatrixStale,
            ],
            vec![review_pack_record_pull_request_scope_clean(), review_pack_record_widen_fold()],
            vec![downgrade_go_no_go_signoff_clean(), downgrade_form_incomplete()],
        ),
        base_row(
            C::SupportExportPacket,
            "Support owner",
            "Support resolves the collaboration replica descriptor and its shared-object records while keeping each object's declared authority model, convergence state, and export posture bound to the export; a shared object whose authority model is unstated and an op-log, snapshot, or archive that would export without policy-labeled redaction and actor lineage degrade honestly so the replica identity, authority model, and export posture are never dropped on export or companion handoff",
            "collaboration-replica:m5-support:001",
            vec![
                D::AuthorityModelUnstated,
                D::ExportPostureUnstated,
                D::CollaborationStateMatrixStale,
            ],
            vec![
                review_pack_record_worktree_uncommitted_scope_clean(),
                review_pack_record_unbound(),
            ],
            vec![comparison_review_pack_result_unclassified()],
        ),
        base_row(
            C::CommentAnnotationReviewPinLayer,
            "Comment-annotation-review-pin-layer owner",
            "The comment / annotation / review-pin layer resolves the server-ordered comments, annotations, and review pins to one shared-object record that declares server-ordered authority and keeps an append-only, reviewable anchor-drift history; a pin whose authority model is unstated and an anchor rebound without drift history degrade honestly instead of silently rebinding a comment or pin",
            "collaboration-replica:m5-comment-annotation-review-pin-layer:001",
            vec![
                D::AuthorityModelUnstated,
                D::ConvergenceStateUnstated,
                D::CollaborationStateMatrixStale,
            ],
            vec![
                review_pack_record_base_head_range_scope_clean(),
                review_pack_record_token_unstated(),
            ],
            vec![downgrade_rehearsal_currency_clean()],
        ),
        base_row(
            C::CollaborationDegradationBanner,
            "Collaboration-degradation-banner owner",
            "The collaboration degradation banner resolves the higher-risk control-plane and the convergence- or awareness-degraded shared object to one shared-object record that names the distinct degraded state — convergence-degraded, awareness-degraded, anchor-unresolved, or relay-partitioned — rather than a generic stale or broken badge; a shared object whose convergence state is unstated and a degraded state collapsed into a generic stale badge degrade honestly so a host-authoritative control object never reads as convergent",
            "collaboration-replica:m5-collaboration-degradation-banner:001",
            vec![
                D::ConvergenceStateUnstated,
                D::AuthorityModelUnstated,
                D::CollaborationStateMatrixStale,
            ],
            vec![review_pack_record_saved_pack_snapshot_scope_clean()],
            vec![downgrade_go_no_go_signoff_clean()],
        ),
        base_row(
            C::HelpDocs,
            "Help-docs owner",
            "The help / docs feed carries the same collaboration replica descriptor and shared-object-record truth, so a shared object whose authority model is unstated, unsent local edits discarded on a downgrade, or an immutable sealed-archive object mislabeled as convergent is visible in evidence — each object class named as CRDT-convergent, server-ordered, host-authoritative, or immutable evidence — rather than hidden behind a green summary",
            "collaboration-replica:m5-help-docs:001",
            vec![
                D::AuthorityModelUnstated,
                D::UnsentLocalEditsDiscardedOnDowngrade,
                D::CollaborationStateMatrixStale,
            ],
            vec![review_pack_record_full_tree_scope_clean()],
            vec![downgrade_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5CollaborationReplicaRecordAndResultRegistriesGovernanceReview {
    M5CollaborationReplicaRecordAndResultRegistriesGovernanceReview {
        review_pack_record_registry_names_token_role_and_journey: true,
        line_resolves_to_typed_review_pack_record_from_shared_registry: true,
        repo_bundle_toolchain_and_deployment_rows_published: true,
        lines_cannot_widen_without_rollback_and_diagnostics: true,
        review_pack_result_keeps_proof_visible_and_binds_support_language: true,
        support_language_matched_to_line_proof_for_public_lines: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        review_pack_record_or_downgrade_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5CollaborationReplicaRecordAndResultRegistriesConsumerProjection {
    M5CollaborationReplicaRecordAndResultRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5CollaborationReplicaRecordAndResultRegistriesProofFreshness {
    M5CollaborationReplicaRecordAndResultRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5CollaborationReplicaRecordAndResultRegistriesReleasePosture {
    M5CollaborationReplicaRecordAndResultRegistriesReleasePosture {
        proof_packet_ref:
            M5_COLLABORATION_REPLICA_DESCRIPTOR_AND_SHARED_OBJECT_RECORD_REGISTRIES_ARTIFACT_REF
                .to_owned(),
        line_audit_ref:
            M5_COLLABORATION_REPLICA_DESCRIPTOR_AND_SHARED_OBJECT_RECORD_REGISTRIES_REPORT_REF
                .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_COLLABORATION_REPLICA_DESCRIPTOR_AND_SHARED_OBJECT_RECORD_REGISTRIES_SCHEMA_REF,
        M5_COLLABORATION_REPLICA_DESCRIPTOR_AND_SHARED_OBJECT_RECORD_REGISTRIES_DOC_REF,
        M5_COLLABORATION_STATE_MATRIX_SCHEMA_REF,
        M5_COLLABORATION_STATE_MATRIX_DOC_REF,
        M5_CRDT_BACKED_SHARED_TEXT_DOMAIN_SCHEMA_REF,
        M5_SHARED_OBJECT_DESCRIPTOR_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 collaboration-replica-descriptor and shared-object-record registries packet.
pub fn seeded_m5_collaboration_replica_descriptor_and_shared_object_record_registries(
) -> M5CollaborationReplicaRecordAndResultRegistriesPacket {
    M5CollaborationReplicaRecordAndResultRegistriesPacket::new(
        M5CollaborationReplicaRecordAndResultRegistriesPacketInput {
            packet_id: M5_COLLABORATION_REPLICA_DESCRIPTOR_AND_SHARED_OBJECT_RECORD_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 collaboration-replica-descriptor and shared-object-record registries emitting one durable machine-readable collaboration replica descriptor per shared object entering share mode — one typed field per record section: the workspace-root identity, the buffer / object identity, the revision and session epoch, the trust and policy epoch, and the export posture — so entering share mode creates an explicit replica record rather than replacing local buffer, VFS, or Git canonical truth outright, with canonical / accessible / audit resolution-form coverage, and a machine-readable shared-object record per object class (CRDT-convergent shared editable text, sampled presence / cursor / selection samples, server-ordered comments / annotations / review pins, presenter / follow state, linked higher-risk control objects, and immutable sealed-archive evidence) that declares which authority model each object holds — CRDT-convergent, server-ordered, host-authoritative, or immutable evidence — its convergence and merge-drift posture, its append-only anchor-drift history, and its policy-labeled export lineage — so a permission or relay downgrade preserves local unsent work first, anchor drift stays append-only and reviewable, convergence- or awareness-degraded state is never collapsed into a generic stale badge, and op-logs, snapshots, or archives never export without policy-labeled redaction and actor lineage — rather than a green summary across the shared editor replica view, presence / cursor layer, comment / annotation / review-pin layer, collaboration degradation banner, help / docs, and support / export surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5CollaborationReplicaRecordAndResultRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the review-detail row is held at Beta pending review-pack-record parity on every pack
/// version / digest; every row stays visible and every example stays honest.
pub fn seeded_m5_collaboration_replica_descriptor_and_shared_object_record_registries_review_pack_record_beta_narrowed(
) -> M5CollaborationReplicaRecordAndResultRegistriesPacket {
    let mut packet =
        seeded_m5_collaboration_replica_descriptor_and_shared_object_record_registries();
    packet.packet_id =
        "m5-collaboration-replica-descriptor-and-shared-object-record-registries:collaboration-replica-descriptor-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| {
            row.consumer_surface == M5CollaborationStateConsumerSurface::SharedEditorReplicaView
        })
        .expect("review-detail row present");
    row.qualification = M5CollaborationStateQualificationClass::Beta;
    packet
}

/// Narrowed variant: the AI-review row is narrowed to Preview pending review-pack-result parity on every
/// evaluator binding; every row stays visible and every example stays honest.
pub fn seeded_m5_collaboration_replica_descriptor_and_shared_object_record_registries_review_pack_result_preview_narrowed(
) -> M5CollaborationReplicaRecordAndResultRegistriesPacket {
    let mut packet =
        seeded_m5_collaboration_replica_descriptor_and_shared_object_record_registries();
    packet.packet_id =
        "m5-collaboration-replica-descriptor-and-shared-object-record-registries:shared-object-record-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| {
            row.consumer_surface == M5CollaborationStateConsumerSurface::PresenceCursorLayer
        })
        .expect("AI-review row present");
    row.qualification = M5CollaborationStateQualificationClass::Preview;
    packet
}
