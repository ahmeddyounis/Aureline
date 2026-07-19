//! Canonical seed builders for the M5 session-provenance-freshness and consumer-truth-label registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean provenance-freshness-label and
//! consumer-truth-disposition entries are built so the one typed provenance-freshness label resolving per consumed
//! context, records never letting collaboration-derived context read as canonical repo truth while its provenance
//! label or source link was dropped, dispositions never acting without actor / time
//! provenance behind a green summary, the canonical / accessible / audit resolution forms, and the complete
//! provenance-class / disposition / actor-time-provenance / binding / active-reason
//! consumer-truth-disposition object are proven across the shared-editor-replica-view, presence / cursor layer,
//! comment / annotation / review-pin layer, collaboration degradation banner, help / docs, and support / export
//! surfaces without any hand-copied per-consumer assumption, session-state-shown-as-canonical, incomplete
//! object, provenance-free disposition, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_SESSION_PROVENANCE_FRESHNESS_AND_CONSUMER_TRUTH_LABEL_REGISTRIES_PACKET_ID: &str =
    "m5-session-provenance-freshness-and-consumer-truth-label-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn review_pack_record(
    input: M5SessionProvenanceFreshnessRecordEntryResolutionInput,
) -> M5ResolvedSessionProvenanceFreshnessRecordEntry {
    resolve_review_pack_record_entry(input).expect("seed line-review_pack_record entry resolves")
}

fn downgrade(
    input: M5SessionProvenanceFreshnessResultEntryResolutionInput,
) -> M5ResolvedSessionProvenanceFreshnessResultEntry {
    resolve_review_pack_result_entry(input).expect("seed line-downgrade-packet entry resolves")
}

fn all_forms() -> Vec<M5SessionProvenanceFreshnessRecordResolutionForm> {
    M5SessionProvenanceFreshnessRecordResolutionForm::ALL.to_vec()
}

// -- Clean line-review_pack_record entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_review_pack_record_base(
    entry_id: &str,
    line_binding_id: &str,
    token_name: &str,
    semantic_role: M5CollaborationStateRole,
    report_section: M5SessionProvenanceFreshnessRecordKind,
    surface_context: M5SessionProvenanceFreshnessRecordSurfaceContext,
    exact_repo_journey_rows: &str,
    bundle_ids: &str,
    install_topology: &str,
    toolchain_envelope: &str,
    known_limits: &str,
    rollback_target: &str,
    diagnostics_posture: &str,
) -> M5SessionProvenanceFreshnessRecordEntryResolutionInput {
    M5SessionProvenanceFreshnessRecordEntryResolutionInput {
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

fn review_pack_record_changed_files_scope_clean() -> M5ResolvedSessionProvenanceFreshnessRecordEntry
{
    review_pack_record(clean_review_pack_record_base(
        "review_pack_record:shiproom:dogfood-core-team-canary",
        "launch.line.core-team-canary",
        "line.review_pack_record.core_team_canary",
        M5CollaborationStateRole::AuthorityModelDisclosure,
        M5SessionProvenanceFreshnessRecordKind::ChangedFilesScope,
        M5SessionProvenanceFreshnessRecordSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn review_pack_record_pull_request_scope_clean() -> M5ResolvedSessionProvenanceFreshnessRecordEntry
{
    review_pack_record(clean_review_pack_record_base(
        "review_pack_record:release-center:migration-alpha",
        "launch.line.migration-alpha",
        "line.review_pack_record.pull_request_scope",
        M5CollaborationStateRole::LocalTruthPreservationDisclosure,
        M5SessionProvenanceFreshnessRecordKind::PullRequestScope,
        M5SessionProvenanceFreshnessRecordSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-journeys",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn review_pack_record_base_head_range_scope_clean(
) -> M5ResolvedSessionProvenanceFreshnessRecordEntry {
    review_pack_record(clean_review_pack_record_base(
        "review_pack_record:program-governance:extension-author",
        "launch.line.extension-author",
        "line.review_pack_record.base_head_range_scope",
        M5CollaborationStateRole::MergeAndDriftSemanticsDisclosure,
        M5SessionProvenanceFreshnessRecordKind::BaseHeadRangeScope,
        M5SessionProvenanceFreshnessRecordSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-journeys",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn review_pack_record_worktree_uncommitted_scope_clean(
) -> M5ResolvedSessionProvenanceFreshnessRecordEntry {
    // A design-partner preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_review_pack_record_base(
        "review_pack_record:executive-steering:design-partner-preview",
        "launch.line.design-partner-preview",
        "line.review_pack_record.worktree_uncommitted_scope",
        M5CollaborationStateRole::AuthorityModelDisclosure,
        M5SessionProvenanceFreshnessRecordKind::WorktreeUncommittedScope,
        M5SessionProvenanceFreshnessRecordSurfaceContext::ExecutiveSteeringSurface,
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

fn review_pack_record_full_tree_scope_clean() -> M5ResolvedSessionProvenanceFreshnessRecordEntry {
    // A public preview line is public-facing and keeps its support language matched to line proof.
    let mut base = clean_review_pack_record_base(
        "review_pack_record:support:public-preview",
        "launch.line.public-preview",
        "line.review_pack_record.full_tree_scope",
        M5CollaborationStateRole::LocalTruthPreservationDisclosure,
        M5SessionProvenanceFreshnessRecordKind::FullTreeScope,
        M5SessionProvenanceFreshnessRecordSurfaceContext::SupportOrExportForm,
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

fn review_pack_record_saved_pack_snapshot_scope_clean(
) -> M5ResolvedSessionProvenanceFreshnessRecordEntry {
    review_pack_record(clean_review_pack_record_base(
        "review_pack_record:release-center:certified-journey",
        "launch.line.certified-journey",
        "line.review_pack_record.saved_pack_snapshot_scope",
        M5CollaborationStateRole::DowngradeBehaviorDisclosure,
        M5SessionProvenanceFreshnessRecordKind::SavedPackSnapshotScope,
        M5SessionProvenanceFreshnessRecordSurfaceContext::ReleaseCenterSurface,
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
fn review_pack_record_object_incomplete() -> M5ResolvedSessionProvenanceFreshnessRecordEntry {
    let mut base = clean_review_pack_record_base(
        "review_pack_record:shiproom:incomplete",
        "launch.line.core-team-canary",
        "line.review_pack_record.core_team_canary",
        M5CollaborationStateRole::AuthorityModelDisclosure,
        M5SessionProvenanceFreshnessRecordKind::ChangedFilesScope,
        M5SessionProvenanceFreshnessRecordSurfaceContext::ShiproomSurface,
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
fn review_pack_record_widen_fold() -> M5ResolvedSessionProvenanceFreshnessRecordEntry {
    let mut base = clean_review_pack_record_base(
        "review_pack_record:release-center:widen-fold",
        "launch.line.migration-alpha",
        "line.review_pack_record.pull_request_scope",
        M5CollaborationStateRole::LocalTruthPreservationDisclosure,
        M5SessionProvenanceFreshnessRecordKind::PullRequestScope,
        M5SessionProvenanceFreshnessRecordSurfaceContext::ReleaseCenterSurface,
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
fn review_pack_record_unbound() -> M5ResolvedSessionProvenanceFreshnessRecordEntry {
    let mut base = clean_review_pack_record_base(
        "review_pack_record:executive-steering:unbound",
        "launch.line.certified-journey",
        "line.review_pack_record.saved_pack_snapshot_scope",
        M5CollaborationStateRole::DowngradeBehaviorDisclosure,
        M5SessionProvenanceFreshnessRecordKind::SavedPackSnapshotScope,
        M5SessionProvenanceFreshnessRecordSurfaceContext::ExecutiveSteeringSurface,
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
fn review_pack_record_token_unstated() -> M5ResolvedSessionProvenanceFreshnessRecordEntry {
    let mut base = clean_review_pack_record_base(
        "review_pack_record:program-governance:token-unstated",
        "launch.line.extension-author",
        "  ",
        M5CollaborationStateRole::MergeAndDriftSemanticsDisclosure,
        M5SessionProvenanceFreshnessRecordKind::BaseHeadRangeScope,
        M5SessionProvenanceFreshnessRecordSurfaceContext::ProgramGovernanceSurface,
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
    comparison_scope: M5SessionProvenanceFreshnessResultScope,
    surface_context: M5SessionProvenanceFreshnessRecordSurfaceContext,
    resolved_line_identity: &str,
    known_limits_ledger: &str,
    rollback_target_reference: &str,
    rehearsal_currency_state: &str,
    readiness_signoff_state: &str,
    support_language_reference: &str,
    last_widening_revision: &str,
) -> M5SessionProvenanceFreshnessResultEntryResolutionInput {
    M5SessionProvenanceFreshnessResultEntryResolutionInput {
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

fn downgrade_dogfood_ring_clean() -> M5ResolvedSessionProvenanceFreshnessResultEntry {
    // A dogfood-ring downgrade packet carries partner / public support language bound to line proof.
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:dogfood-ring",
        "launch.line.core-team-canary",
        "line.downgrade.core_team_canary",
        M5CollaborationStateRole::AuthorityModelDisclosure,
        M5SessionProvenanceFreshnessResultScope::EvaluatedScopeBinding,
        M5SessionProvenanceFreshnessRecordSurfaceContext::ShiproomSurface,
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

fn downgrade_rehearsal_currency_clean() -> M5ResolvedSessionProvenanceFreshnessResultEntry {
    downgrade(clean_downgrade_base(
        "downgrade:program-governance:rehearsal-currency",
        "launch.line.extension-author",
        "line.downgrade.base_head_range_scope",
        M5CollaborationStateRole::MergeAndDriftSemanticsDisclosure,
        M5SessionProvenanceFreshnessResultScope::PackVersionDigestBinding,
        M5SessionProvenanceFreshnessRecordSurfaceContext::ProgramGovernanceSurface,
        "line-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn downgrade_go_no_go_signoff_clean() -> M5ResolvedSessionProvenanceFreshnessResultEntry {
    downgrade(clean_downgrade_base(
        "downgrade:release-center:go-no-go-signoff",
        "launch.line.certified-journey",
        "line.downgrade.saved_pack_snapshot_scope",
        M5CollaborationStateRole::DowngradeBehaviorDisclosure,
        M5SessionProvenanceFreshnessResultScope::DivergenceLabelBinding,
        M5SessionProvenanceFreshnessRecordSurfaceContext::ReleaseCenterSurface,
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
fn downgrade_support_ahead() -> M5ResolvedSessionProvenanceFreshnessResultEntry {
    let mut base = clean_downgrade_base(
        "downgrade:shiproom:support-ahead",
        "launch.line.public-preview",
        "line.downgrade.full_tree_scope",
        M5CollaborationStateRole::LocalTruthPreservationDisclosure,
        M5SessionProvenanceFreshnessResultScope::EvaluatedScopeBinding,
        M5SessionProvenanceFreshnessRecordSurfaceContext::ShiproomSurface,
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
fn downgrade_form_incomplete() -> M5ResolvedSessionProvenanceFreshnessResultEntry {
    let mut base = clean_downgrade_base(
        "downgrade:release-center:form-incomplete",
        "launch.line.certified-journey",
        "line.downgrade.saved_pack_snapshot_scope",
        M5CollaborationStateRole::DowngradeBehaviorDisclosure,
        M5SessionProvenanceFreshnessResultScope::DivergenceLabelBinding,
        M5SessionProvenanceFreshnessRecordSurfaceContext::ReleaseCenterSurface,
        "line-id.certified-journey-0007",
        "known-limits.ledger.certified-journey",
        "rollback.target.ref.certified-journey",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage =
        vec![M5SessionProvenanceFreshnessRecordResolutionForm::CanonicalObject];
    downgrade(base)
}

/// Degraded downgrade entry: the downgrade scope is unclassified.
fn comparison_review_pack_result_unclassified() -> M5ResolvedSessionProvenanceFreshnessResultEntry {
    downgrade(clean_downgrade_base(
        "downgrade:executive-steering:scope-unclassified",
        "launch.line.design-partner-preview",
        "line.downgrade.worktree_uncommitted_scope",
        M5CollaborationStateRole::AuthorityModelDisclosure,
        M5SessionProvenanceFreshnessResultScope::ChangeObjectResultUnclassified,
        M5SessionProvenanceFreshnessRecordSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5SessionProvenanceFreshnessRecordAndResultRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5CollaborationStateDowngradeTrigger>,
    review_pack_record_entries: Vec<M5ResolvedSessionProvenanceFreshnessRecordEntry>,
    review_pack_result_entries: Vec<M5ResolvedSessionProvenanceFreshnessResultEntry>,
) -> M5SessionProvenanceFreshnessRecordAndResultRegistriesRow {
    M5SessionProvenanceFreshnessRecordAndResultRegistriesRow {
        consumer_surface,
        qualification: M5CollaborationStateQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        capture_lifecycle_stages: M5CollaborationStateClassificationStage::ALL.to_vec(),
        required_labels: M5CollaborationStateRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5CollaborationStateAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5SessionProvenanceFreshnessRecordAnatomyPart::ALL.to_vec(),
        export_fields: M5SessionProvenanceFreshnessRecordExportField::ALL.to_vec(),
        downgrade_triggers,
        review_pack_record_entries,
        review_pack_result_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SESSION_PROVENANCE_FRESHNESS_AND_CONSUMER_TRUTH_LABEL_REGISTRIES_SCHEMA_REF,
            M5_CRDT_BACKED_SHARED_TEXT_DOMAIN_SCHEMA_REF,
            M5_SEALED_SESSION_ARCHIVE_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_line_without_current_rollback_and_diagnostics_downgrade: false,
        runs_partner_or_public_support_language_ahead_of_line_proof: false,
        hides_the_rollback_target_or_diagnostics_posture_before_widening: false,
        collapses_distinct_review_pack_result_classes_into_one_lane: false,
    }
}

fn registry_rows() -> Vec<M5SessionProvenanceFreshnessRecordAndResultRegistriesRow> {
    use M5CollaborationStateConsumerSurface as C;
    use M5CollaborationStateDowngradeTrigger as D;

    vec![
        base_row(
            C::SharedEditorReplicaView,
            "Shared-editor-replica-view owner",
            "The shared editor replica view resolves each piece of collaboration-derived context that search and AI context read from a shared session — live session state, a captured snapshot, an archived session artifact, or canonical Git / VFS truth — to one typed provenance-freshness-label record naming the provenance class, the freshness or archive class it carries, the source session and source link, the actor provenance of the capture, and whether it may enrich but never masquerade as canonical repo truth, and to the consumer-truth-disposition descriptor offering label-as-session-derived, link-to-source, enrich-repo-truth, block-as-canonical, or defer with actor / time provenance; the provenance label and its source link are recorded before any consumer presents the context, and a record that would let session-derived state read as canonical Git / VFS state without an explicit label and source link degrades honestly instead of promoting session truth to canonical repository truth",
            "session-provenance-freshness:m5-shared-editor-replica-view:001",
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
            "The presence / cursor layer resolves presence-derived context to the provenance-freshness-label record that keeps the provenance class and freshness or archive class inspectable — carrying the source session, source link, and actor provenance a consumer must see — rather than letting live-session presence read as canonical repo state; a record that would drop the provenance label or freshness class and let a stale capture read as current degrades honestly instead of letting session-derived presence masquerade as canonical truth",
            "session-provenance-freshness:m5-presence-cursor-layer:001",
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
            "Support resolves the recorded provenance-freshness-label entries and their consumer-truth-disposition descriptors while keeping the provenance class, freshness or archive class, source link, and disposition bound to the export route, so an export or support packet can show whether it is quoting live session state, a captured snapshot, an archived session artifact, or canonical Git / VFS truth and what disposition was taken, and preserve actor provenance; a descriptor whose actor / time provenance is unstated and a disposition that would promote session state to canonical without a label and source link degrade honestly so the provenance class, source link, and disposition are never dropped on export or companion handoff",
            "session-provenance-freshness:m5-support:001",
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
            "The comment / annotation / review-pin layer resolves review-pane context over server-ordered comments, annotations, and review pins to a provenance-freshness-label record that keeps the provenance class, freshness or archive class, and source link as an append-only, inspectable record with a label-or-link disposition; a descriptor whose disposition is unstated and a disposition that would let a session-scoped comment or review pin read as canonical repo truth without a label and source link degrade honestly instead of promoting session-derived review context to canonical state on the user's behalf",
            "session-provenance-freshness:m5-comment-annotation-review-pin-layer:001",
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
            "The collaboration degradation banner resolves a consumed-context condition to a provenance-freshness-label record that names the distinct live-session / captured-snapshot / archived-session-artifact / canonical-Git-VFS provenance class and surfaces the consumer-truth-disposition descriptor (label-as-session-derived, link-to-source, enrich-repo-truth, block-as-canonical, or defer) as a sticky label rather than a generic stale or canonical badge; a context whose provenance class is unstated and a distinct session-derived or archived class collapsed into a generic canonical badge degrade honestly so session-derived context never disappears behind an undifferentiated canonical banner",
            "session-provenance-freshness:m5-collaboration-degradation-banner:001",
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
            "The help / docs feed carries the same session-provenance and consumer-truth-disposition truth, so a docs link whose provenance class is unstated, a disposition that promotes session state to canonical without a label and source link, or a disposition that drops the source link is visible in evidence — each provenance class named as live session state, captured snapshot, archived session artifact, or canonical Git / VFS truth, and each disposition as label-as-session-derived, link-to-source, enrich-repo-truth, block-as-canonical, or defer — rather than hidden behind a green summary",
            "session-provenance-freshness:m5-help-docs:001",
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

fn governance_review() -> M5SessionProvenanceFreshnessRecordAndResultRegistriesGovernanceReview {
    M5SessionProvenanceFreshnessRecordAndResultRegistriesGovernanceReview {
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

fn consumer_projection() -> M5SessionProvenanceFreshnessRecordAndResultRegistriesConsumerProjection
{
    M5SessionProvenanceFreshnessRecordAndResultRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5SessionProvenanceFreshnessRecordAndResultRegistriesProofFreshness {
    M5SessionProvenanceFreshnessRecordAndResultRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SessionProvenanceFreshnessRecordAndResultRegistriesReleasePosture {
    M5SessionProvenanceFreshnessRecordAndResultRegistriesReleasePosture {
        proof_packet_ref:
            M5_SESSION_PROVENANCE_FRESHNESS_AND_CONSUMER_TRUTH_LABEL_REGISTRIES_ARTIFACT_REF
                .to_owned(),
        line_audit_ref:
            M5_SESSION_PROVENANCE_FRESHNESS_AND_CONSUMER_TRUTH_LABEL_REGISTRIES_REPORT_REF
                .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SESSION_PROVENANCE_FRESHNESS_AND_CONSUMER_TRUTH_LABEL_REGISTRIES_SCHEMA_REF,
        M5_SESSION_PROVENANCE_FRESHNESS_AND_CONSUMER_TRUTH_LABEL_REGISTRIES_DOC_REF,
        M5_COLLABORATION_STATE_MATRIX_SCHEMA_REF,
        M5_COLLABORATION_STATE_MATRIX_DOC_REF,
        M5_CRDT_BACKED_SHARED_TEXT_DOMAIN_SCHEMA_REF,
        M5_SEALED_SESSION_ARCHIVE_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 session-provenance-freshness and consumer-truth-label registries packet.
pub fn seeded_m5_session_provenance_freshness_and_consumer_truth_label_registries(
) -> M5SessionProvenanceFreshnessRecordAndResultRegistriesPacket {
    M5SessionProvenanceFreshnessRecordAndResultRegistriesPacket::new(
        M5SessionProvenanceFreshnessRecordAndResultRegistriesPacketInput {
            packet_id: M5_SESSION_PROVENANCE_FRESHNESS_AND_CONSUMER_TRUTH_LABEL_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 session-provenance-freshness and consumer-truth-label registries emitting one durable append-only machine-readable provenance-freshness-label record per piece of collaboration-derived context a consumer reads — live session state, a captured snapshot, an archived session artifact, or canonical Git / VFS truth — one typed field per record section: the provenance class of the source, the freshness or archive class it carries, the source session and source link, the actor provenance of the capture, and whether it may enrich but never masquerade as canonical repo truth — so a consumer keeps the provenance label and source link first rather than promoting session state to canonical repository truth, with canonical / accessible / audit resolution-form coverage, and a machine-readable consumer-truth-disposition-descriptor record per consumed context that declares the disposition a consumer can take (label-as-session-derived, link-to-source, enrich-repo-truth, block-as-canonical, or defer) with its actor / time provenance and its canonical-versus-session lineage — so search, AI context, review panes, docs links, and support packets never present session-derived state as canonical Git / VFS state without an explicit label and source link, users can tell when they are using collaboration-derived context and what its freshness or archive class is, a disposition never acts without actor / time provenance, and no disposition drops the source link — rather than a green summary across the shared editor replica view, presence / cursor layer, comment / annotation / review-pin layer, collaboration degradation banner, help / docs, and support / export surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5SessionProvenanceFreshnessRecordAndResultRegistriesVocabularySet::canonical(),
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
pub fn seeded_m5_session_provenance_freshness_and_consumer_truth_label_registries_review_pack_record_beta_narrowed(
) -> M5SessionProvenanceFreshnessRecordAndResultRegistriesPacket {
    let mut packet = seeded_m5_session_provenance_freshness_and_consumer_truth_label_registries();
    packet.packet_id =
        "m5-session-provenance-freshness-and-consumer-truth-label-registries:provenance-label-beta:0001"
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
pub fn seeded_m5_session_provenance_freshness_and_consumer_truth_label_registries_review_pack_result_preview_narrowed(
) -> M5SessionProvenanceFreshnessRecordAndResultRegistriesPacket {
    let mut packet = seeded_m5_session_provenance_freshness_and_consumer_truth_label_registries();
    packet.packet_id =
        "m5-session-provenance-freshness-and-consumer-truth-label-registries:consumer-disposition-preview:0001"
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
