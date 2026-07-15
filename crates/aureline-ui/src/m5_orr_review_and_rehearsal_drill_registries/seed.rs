//! Canonical seed builders for the M5 orr-review and rehearsal-drill registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean orr-review and rehearsal-drill entries are built
//! so the one typed orr-review object resolving per regression asset type, rings never advancing without a
//! visible known-limits and rehearsal-drill posture, partner / public support language never running ahead of ring
//! proof, the canonical / accessible / audit resolution forms, and the complete transition-identity /
//! active-stop-condition-ledger / rehearsal-drill-target / protected-metric-regression / packet-freshness /
//! crash-data-loss-or-trust / last-ring-transition-revision rehearsal-drill record are proven across the shiproom,
//! release-center, executive-steering, program-governance, diagnostics, and support surfaces without any
//! hand-copied per-ring assumption, widen-without-stop, incomplete object, hidden rehearsal-drill, or
//! resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_ORR_REVIEW_REHEARSAL_DRILL_REGISTRIES_PACKET_ID: &str =
    "m5-orr-review-and-rehearsal-drill-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn descriptor(input: M5OrrReviewEntryResolutionInput) -> M5ResolvedOrrReviewEntry {
    resolve_orr_review_entry(input).expect("seed orr-review entry resolves")
}

fn evidence(input: M5RehearsalDrillEntryResolutionInput) -> M5ResolvedRehearsalDrillEntry {
    resolve_rehearsal_drill_entry(input).expect("seed rehearsal-drill entry resolves")
}

fn all_forms() -> Vec<M5OrrReviewResolutionForm> {
    M5OrrReviewResolutionForm::ALL.to_vec()
}

// -- Clean orr-review entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_descriptor_base(
    entry_id: &str,
    orr_packet_binding_id: &str,
    token_name: &str,
    semantic_role: M5LaunchControlRole,
    orr_review_packet_kind: M5OrrReviewPacketKind,
    surface_context: M5OrrReviewSurfaceContext,
    readiness_scope_reference: &str,
    release_owner_reference: &str,
    advisory_owner_reference: &str,
    support_room_owner_reference: &str,
    docs_comms_owner_reference: &str,
    backup_signer_reference: &str,
    rehearsal_freshness_reference: &str,
) -> M5OrrReviewEntryResolutionInput {
    M5OrrReviewEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        orr_packet_binding_id: orr_packet_binding_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        orr_review_packet_kind,
        surface_context,
        resolution_form_coverage: all_forms(),
        readiness_scope_reference: readiness_scope_reference.to_owned(),
        release_owner_reference: release_owner_reference.to_owned(),
        advisory_owner_reference: advisory_owner_reference.to_owned(),
        support_room_owner_reference: support_room_owner_reference.to_owned(),
        docs_comms_owner_reference: docs_comms_owner_reference.to_owned(),
        backup_signer_reference: backup_signer_reference.to_owned(),
        rehearsal_freshness_reference: rehearsal_freshness_reference.to_owned(),
        bound_to_registry: true,
        orr_review_documented_before_widening: true,
        requires_documented_exception: false,
        attributable_asset_or_approved_exception: true,
        proof_fresh: true,
    }
}

fn descriptor_monthly_orr_packet_clean() -> M5ResolvedOrrReviewEntry {
    descriptor(clean_descriptor_base(
        "descriptor:shiproom:dogfood-core-team-canary",
        "incident.lane.core-team-canary",
        "freeze.exception.core_team_canary",
        M5LaunchControlRole::CohortMembership,
        M5OrrReviewPacketKind::MonthlyOrrPacket,
        M5OrrReviewSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-archetypes",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn descriptor_release_candidate_orr_packet_clean() -> M5ResolvedOrrReviewEntry {
    descriptor(clean_descriptor_base(
        "descriptor:release-center:migration-alpha",
        "incident.lane.migration-alpha",
        "freeze.exception.release_candidate_orr_packet",
        M5LaunchControlRole::ReadinessEvent,
        M5OrrReviewPacketKind::ReleaseCandidateOrrPacket,
        M5OrrReviewSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-archetypes",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn descriptor_publish_rollback_drill_clean() -> M5ResolvedOrrReviewEntry {
    descriptor(clean_descriptor_base(
        "descriptor:program-governance:extension-author",
        "incident.lane.extension-author",
        "freeze.exception.publish_rollback_drill",
        M5LaunchControlRole::RehearsalCurrency,
        M5OrrReviewPacketKind::PublishRollbackDrill,
        M5OrrReviewSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-archetypes",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn descriptor_mixed_version_drill_clean() -> M5ResolvedOrrReviewEntry {
    // A design-partner preview cohort is public-facing and keeps its support language matched to cohort proof.
    let mut base = clean_descriptor_base(
        "descriptor:executive-steering:design-partner-preview",
        "incident.lane.design-partner-preview",
        "freeze.exception.mixed_version_drill",
        M5LaunchControlRole::CohortMembership,
        M5OrrReviewPacketKind::MixedVersionDrill,
        M5OrrReviewSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.design-partner-preview-archetypes",
        "bundle.ids.design-partner-0007",
        "install.topology.enrolled-design-partners",
        "toolchain.envelope.pinned-partner",
        "known-limits.published.design-partner",
        "rollback.target.partner-previous-preview",
        "diagnostics.posture.partner-telemetry",
    );
    base.requires_documented_exception = true;
    base.attributable_asset_or_approved_exception = true;
    descriptor(base)
}

fn descriptor_advisory_revocation_drill_clean() -> M5ResolvedOrrReviewEntry {
    // A public preview cohort is public-facing and keeps its support language matched to cohort proof.
    let mut base = clean_descriptor_base(
        "descriptor:support:public-preview",
        "incident.lane.public-preview",
        "freeze.exception.advisory_revocation_drill",
        M5LaunchControlRole::ReadinessEvent,
        M5OrrReviewPacketKind::AdvisoryRevocationDrill,
        M5OrrReviewSurfaceContext::SupportOrExportForm,
        "repo.rows.public-preview-archetypes",
        "bundle.ids.public-preview-0007",
        "install.topology.public-preview-ring",
        "toolchain.envelope.pinned-public",
        "known-limits.published.public-preview",
        "rollback.target.public-previous-stable",
        "diagnostics.posture.public-telemetry",
    );
    base.requires_documented_exception = true;
    base.attributable_asset_or_approved_exception = true;
    descriptor(base)
}

fn descriptor_support_incident_handoff_drill_clean() -> M5ResolvedOrrReviewEntry {
    descriptor(clean_descriptor_base(
        "descriptor:release-center:certified-archetype",
        "incident.lane.certified-archetype",
        "freeze.exception.support_incident_handoff_drill",
        M5LaunchControlRole::GoNoGoAuthority,
        M5OrrReviewPacketKind::SupportIncidentHandoffDrill,
        M5OrrReviewSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-archetype-archetypes",
        "bundle.ids.certified-0007",
        "install.topology.certified-archetype-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-archetype",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded orr-review entries ---------------------------------------------------------

/// Degraded descriptor entry: the resolved descriptor object is incomplete — the bundle IDs are unstated.
fn descriptor_object_incomplete() -> M5ResolvedOrrReviewEntry {
    let mut base = clean_descriptor_base(
        "descriptor:shiproom:incomplete",
        "incident.lane.core-team-canary",
        "freeze.exception.core_team_canary",
        M5LaunchControlRole::CohortMembership,
        M5OrrReviewPacketKind::MonthlyOrrPacket,
        M5OrrReviewSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-archetypes",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.release_owner_reference = "   ".to_owned();
    descriptor(base)
}

/// Degraded descriptor entry: the cohort's rollback and diagnostics posture is not preserved before widening —
/// a cohort widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn descriptor_widen_fold() -> M5ResolvedOrrReviewEntry {
    let mut base = clean_descriptor_base(
        "descriptor:release-center:widen-fold",
        "incident.lane.migration-alpha",
        "freeze.exception.release_candidate_orr_packet",
        M5LaunchControlRole::ReadinessEvent,
        M5OrrReviewPacketKind::ReleaseCandidateOrrPacket,
        M5OrrReviewSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-archetypes",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.orr_review_documented_before_widening = false;
    descriptor(base)
}

/// Degraded descriptor entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn descriptor_unbound() -> M5ResolvedOrrReviewEntry {
    let mut base = clean_descriptor_base(
        "descriptor:executive-steering:unbound",
        "incident.lane.certified-archetype",
        "freeze.exception.support_incident_handoff_drill",
        M5LaunchControlRole::GoNoGoAuthority,
        M5OrrReviewPacketKind::SupportIncidentHandoffDrill,
        M5OrrReviewSurfaceContext::ExecutiveSteeringSurface,
        "repo.rows.certified-archetype-archetypes",
        "bundle.ids.certified-0007",
        "install.topology.certified-archetype-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-archetype",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    );
    base.bound_to_registry = false;
    descriptor(base)
}

/// Degraded descriptor entry: the canonical registry token name is unstated.
fn orr_review_token_unstated() -> M5ResolvedOrrReviewEntry {
    let mut base = clean_descriptor_base(
        "descriptor:program-governance:token-unstated",
        "incident.lane.extension-author",
        "  ",
        M5LaunchControlRole::RehearsalCurrency,
        M5OrrReviewPacketKind::PublishRollbackDrill,
        M5OrrReviewSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-archetypes",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    );
    base.token_name = "  ".to_owned();
    descriptor(base)
}

// -- Clean rehearsal-drill entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_evidence_base(
    entry_id: &str,
    rehearsal_drill_ref: &str,
    token_name: &str,
    semantic_role: M5LaunchControlRole,
    rehearsal_drill_coverage: M5RehearsalDrillCoverageKind,
    surface_context: M5OrrReviewSurfaceContext,
    resolved_coverage_identity: &str,
    evidence_snapshot_ledger: &str,
    orr_signoff_reference: &str,
    on_call_roster_state: &str,
    rehearsal_drill_freshness_state: &str,
    widening_stage_reference: &str,
    last_rehearsal_drill_revision: &str,
) -> M5RehearsalDrillEntryResolutionInput {
    M5RehearsalDrillEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        rehearsal_drill_ref: rehearsal_drill_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        rehearsal_drill_coverage,
        surface_context,
        resolution_form_coverage: all_forms(),
        resolved_coverage_identity: resolved_coverage_identity.to_owned(),
        evidence_snapshot_ledger: evidence_snapshot_ledger.to_owned(),
        orr_signoff_reference: orr_signoff_reference.to_owned(),
        on_call_roster_state: on_call_roster_state.to_owned(),
        rehearsal_drill_freshness_state: rehearsal_drill_freshness_state.to_owned(),
        widening_stage_reference: widening_stage_reference.to_owned(),
        last_rehearsal_drill_revision: last_rehearsal_drill_revision.to_owned(),
        keeps_evidence_snapshot_visible: true,
        rehearsal_drill_lineage_is_truthful: true,
        override_without_evidence_requested: false,
        blocked_until_evidence_linked: false,
        lineage_gap_present: false,
        lineage_gap_flagged: false,
        proof_fresh: true,
    }
}

fn evidence_dogfood_ring_clean() -> M5ResolvedRehearsalDrillEntry {
    // A dogfood-ring evidence packet carries partner / public support language bound to cohort proof.
    let mut base = clean_evidence_base(
        "evidence:shiproom:dogfood-ring",
        "incident.lane.core-team-canary",
        "go.no.go.core_team_canary",
        M5LaunchControlRole::CohortMembership,
        M5RehearsalDrillCoverageKind::FullRosterCoverage,
        M5OrrReviewSurfaceContext::ShiproomSurface,
        "transition-id.core-team-canary-0007",
        "known-limits.ledger.canary",
        "rollback.target.ref.canary",
        "rehearsal.currency.dogfood-ring-current",
        "readiness.signoff.dogfood-reviewed",
        "support.language.canary-bound-to-proof",
        "widening.revision.0007",
    );
    base.override_without_evidence_requested = true;
    base.blocked_until_evidence_linked = true;
    evidence(base)
}

fn evidence_rehearsal_currency_clean() -> M5ResolvedRehearsalDrillEntry {
    evidence(clean_evidence_base(
        "evidence:program-governance:rehearsal-currency",
        "incident.lane.extension-author",
        "go.no.go.publish_rollback_drill",
        M5LaunchControlRole::RehearsalCurrency,
        M5RehearsalDrillCoverageKind::BackupRosterCoverage,
        M5OrrReviewSurfaceContext::ProgramGovernanceSurface,
        "transition-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn evidence_rehearsal_drill_signoff_clean() -> M5ResolvedRehearsalDrillEntry {
    evidence(clean_evidence_base(
        "evidence:release-center:rehearsal-drill-signoff",
        "incident.lane.certified-archetype",
        "go.no.go.support_incident_handoff_drill",
        M5LaunchControlRole::GoNoGoAuthority,
        M5RehearsalDrillCoverageKind::ConditionalRosterCoverage,
        M5OrrReviewSurfaceContext::ReleaseCenterSurface,
        "transition-id.certified-archetype-0007",
        "known-limits.ledger.certified-archetype",
        "rollback.target.ref.certified-archetype",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    ))
}

// -- Degraded rehearsal-drill entries ----------------------------------------------------

/// Degraded evidence entry: the evidence would run partner / public support language ahead of cohort proof — a
/// support-language reference present but not bound to cohort proof reads as trustworthy when the cohort proof
/// does not yet back it.
fn evidence_support_ahead() -> M5ResolvedRehearsalDrillEntry {
    let mut base = clean_evidence_base(
        "evidence:shiproom:support-ahead",
        "incident.lane.public-preview",
        "go.no.go.advisory_revocation_drill",
        M5LaunchControlRole::ReadinessEvent,
        M5RehearsalDrillCoverageKind::FullRosterCoverage,
        M5OrrReviewSurfaceContext::ShiproomSurface,
        "transition-id.public-preview-0007",
        "known-limits.ledger.public-preview",
        "rollback.target.ref.public-preview",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.public-preview-reviewed",
        "support.language.public-ahead-of-proof",
        "widening.revision.0007",
    );
    base.override_without_evidence_requested = true;
    base.blocked_until_evidence_linked = false;
    evidence(base)
}

/// Degraded evidence entry: the canonical / accessible / audit resolution-form coverage of the evidence is
/// incomplete.
fn evidence_form_incomplete() -> M5ResolvedRehearsalDrillEntry {
    let mut base = clean_evidence_base(
        "evidence:release-center:form-incomplete",
        "incident.lane.certified-archetype",
        "go.no.go.support_incident_handoff_drill",
        M5LaunchControlRole::GoNoGoAuthority,
        M5RehearsalDrillCoverageKind::ConditionalRosterCoverage,
        M5OrrReviewSurfaceContext::ReleaseCenterSurface,
        "transition-id.certified-archetype-0007",
        "known-limits.ledger.certified-archetype",
        "rollback.target.ref.certified-archetype",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5OrrReviewResolutionForm::CanonicalObject];
    evidence(base)
}

/// Degraded evidence entry: the evidence scope is unclassified.
fn rehearsal_drill_coverage_unclassified() -> M5ResolvedRehearsalDrillEntry {
    evidence(clean_evidence_base(
        "evidence:executive-steering:scope-unclassified",
        "incident.lane.design-partner-preview",
        "go.no.go.mixed_version_drill",
        M5LaunchControlRole::CohortMembership,
        M5RehearsalDrillCoverageKind::CoverageUnclassified,
        M5OrrReviewSurfaceContext::ExecutiveSteeringSurface,
        "transition-id.design-partner-preview-0007",
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
    consumer_surface: M5OrrReviewRehearsalDrillRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5LaunchControlDowngradeTrigger>,
    orr_review_entries: Vec<M5ResolvedOrrReviewEntry>,
    rehearsal_drill_entries: Vec<M5ResolvedRehearsalDrillEntry>,
) -> M5OrrReviewRehearsalDrillRegistriesRow {
    M5OrrReviewRehearsalDrillRegistriesRow {
        consumer_surface,
        qualification: M5LaunchControlQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        widening_stages: M5LaunchControlWideningStage::ALL.to_vec(),
        required_labels: M5LaunchControlRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5LaunchControlAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5OrrReviewAnatomyPart::ALL.to_vec(),
        export_fields: M5OrrReviewExportField::ALL.to_vec(),
        downgrade_triggers,
        orr_review_entries,
        rehearsal_drill_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_ORR_REVIEW_REHEARSAL_DRILL_REGISTRIES_SCHEMA_REF,
            M5_ORR_REVIEW_DOMAIN_SCHEMA_REF,
            M5_REHEARSAL_DRILL_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_stable_claim_without_current_orr_and_rehearsal_evidence: false,
        lets_a_rehearsal_packet_go_stale_or_skipped_before_widening: false,
        hides_the_required_role_roster_or_on_call_coverage: false,
        implies_green_when_orr_or_rehearsal_packets_are_stale: false,
    }
}

fn registry_rows() -> Vec<M5OrrReviewRehearsalDrillRegistriesRow> {
    use M5LaunchControlConsumerSurface as C;
    use M5LaunchControlDowngradeTrigger as D;

    vec![
        base_row(
            C::Shiproom,
            "Shiproom owner",
            "The shiproom resolves the monthly-ORR packet kind to one typed object — packet kind, readiness scope, release / advisory / support-room / docs-comms / backup-signer roster, and rehearsal-freshness expiry — from the shared registry and proves the full-roster rehearsal-drill record for the launch-bearing lane; an ORR packet missing its roster and a rehearsal-drill record that implies green while the rehearsal packet is stale degrade honestly instead of reading as a clean pass",
            "evidence:m5-launch-control-shiproom:001",
            vec![
                D::WidenedWithoutCurrentCohortEvidence,
                D::RanPartnerOrPublicLanguageAheadOfCohortProof,
                D::ProofStale,
            ],
            vec![
                descriptor_monthly_orr_packet_clean(),
                descriptor_object_incomplete(),
            ],
            vec![evidence_dogfood_ring_clean(), evidence_support_ahead()],
        ),
        base_row(
            C::ReleaseCenter,
            "Release-center owner",
            "The release center resolves the release-candidate-ORR packet kind and the conditional-roster rehearsal-drill record while keeping the rehearsal evidence visible; a lane widening on a stale or skipped rehearsal packet and a resolution-form gap on a rehearsal-drill record are caught before a screenshot can reintroduce a false-truth reading",
            "evidence:m5-launch-control-release-center:001",
            vec![
                D::WidenedWithoutCurrentCohortEvidence,
                D::ImpliedGreenWhileGoNoGoOrOrrWasStale,
                D::ProofStale,
            ],
            vec![descriptor_release_candidate_orr_packet_clean(), descriptor_widen_fold()],
            vec![evidence_rehearsal_drill_signoff_clean(), evidence_form_incomplete()],
        ),
        base_row(
            C::ExecutiveSteering,
            "Executive-steering owner",
            "Executive steering resolves the mixed-version-drill packet kind while keeping its partner support language matched to rehearsal proof and reports the roster-coverage rehearsal-drill record; an ORR packet that is a hand-copied per-entry assumption and a rehearsal-drill record on an unclassified roster coverage degrade honestly",
            "evidence:m5-launch-control-executive-steering:001",
            vec![
                D::RegistryReferenceUnstated,
                D::ReadinessStateUnstated,
                D::ProofStale,
            ],
            vec![
                descriptor_mixed_version_drill_clean(),
                descriptor_unbound(),
            ],
            vec![rehearsal_drill_coverage_unclassified()],
        ),
        base_row(
            C::ProgramGovernance,
            "Program-governance owner",
            "Program governance resolves the publish-rollback-drill packet kind and the backup-roster rehearsal-drill record bound to the registry; an unstated registry token on an ORR packet is caught before it can drift",
            "evidence:m5-launch-control-program-governance:001",
            vec![
                D::RegistryReferenceUnstated,
                D::CohortMembershipUnstated,
                D::ProofStale,
            ],
            vec![
                descriptor_publish_rollback_drill_clean(),
                orr_review_token_unstated(),
            ],
            vec![evidence_rehearsal_currency_clean()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics renders the same resolved ORR-packet and rehearsal-drill truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied readiness table; the support-incident-handoff-drill ORR packet and the full-roster rehearsal-drill record stay inspectable off-renderer",
            "evidence:m5-launch-control-diagnostics:001",
            vec![
                D::CohortMembershipUnstated,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![descriptor_support_incident_handoff_drill_clean()],
            vec![evidence_rehearsal_drill_signoff_clean()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved ORR-packet and rehearsal-drill truth, so a hand-copied constant, an unstated registry token, a widen-on-stale-rehearsal attempt, or support language running ahead of rehearsal proof is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-launch-control-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::RanPartnerOrPublicLanguageAheadOfCohortProof,
                D::ProofStale,
            ],
            vec![descriptor_advisory_revocation_drill_clean()],
            vec![evidence_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5OrrReviewRehearsalDrillRegistriesGovernanceReview {
    M5OrrReviewRehearsalDrillRegistriesGovernanceReview {
        orr_review_registry_names_token_role_and_type: true,
        type_resolves_to_typed_orr_review_from_shared_registry: true,
        build_row_and_cohort_lineage_published: true,
        scope_cannot_widen_without_documented_orr_review: true,
        rehearsal_drill_keeps_evidence_visible_and_blocks_stale_green: true,
        approved_exception_matched_to_scope_for_widening: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        exception_or_rehearsal_drill_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5OrrReviewRehearsalDrillRegistriesConsumerProjection {
    M5OrrReviewRehearsalDrillRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5OrrReviewRehearsalDrillRegistriesProofFreshness {
    M5OrrReviewRehearsalDrillRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5OrrReviewRehearsalDrillRegistriesReleasePosture {
    M5OrrReviewRehearsalDrillRegistriesReleasePosture {
        proof_packet_ref: M5_ORR_REVIEW_REHEARSAL_DRILL_REGISTRIES_ARTIFACT_REF.to_owned(),
        rehearsal_drill_control_audit_ref: M5_ORR_REVIEW_REHEARSAL_DRILL_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_ORR_REVIEW_REHEARSAL_DRILL_REGISTRIES_SCHEMA_REF,
        M5_ORR_REVIEW_REHEARSAL_DRILL_REGISTRIES_DOC_REF,
        M5_LAUNCH_CONTROL_MATRIX_SCHEMA_REF,
        M5_LAUNCH_CONTROL_MATRIX_DOC_REF,
        M5_ORR_REVIEW_DOMAIN_SCHEMA_REF,
        M5_REHEARSAL_DRILL_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 orr-review and rehearsal-drill registries packet.
pub fn seeded_m5_orr_review_and_rehearsal_drill_registries(
) -> M5OrrReviewRehearsalDrillRegistriesPacket {
    M5OrrReviewRehearsalDrillRegistriesPacket::new(
        M5OrrReviewRehearsalDrillRegistriesPacketInput {
            packet_id: M5_ORR_REVIEW_REHEARSAL_DRILL_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 operational-readiness-review and rehearsal-drill registries with one typed ORR-packet object resolving per packet kind (monthly ORR, release-candidate ORR, publish/rollback drill, mixed-version drill, advisory/revocation drill, support/incident handoff drill), each naming its readiness scope and its release / advisory / support-room / docs-comms / backup-signer role roster, a stable claim never widening on a stale or skipped rehearsal packet, rehearsal freshness and role coverage reading as first-class blockers, canonical / accessible / audit resolution-form coverage, and the complete resolved-coverage-identity / rehearsal-evidence-ledger / ORR-signoff / on-call-roster / rehearsal-freshness / widening-stage / last-rehearsal-drill-revision rehearsal-drill record across shiproom, release-center, executive-steering, program-governance, diagnostics, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5OrrReviewRehearsalDrillRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the shiproom row is held at Beta pending orr-review parity on every archetype; every
/// row stays visible and every example stays honest.
pub fn seeded_m5_orr_review_and_rehearsal_drill_registries_orr_review_beta_narrowed(
) -> M5OrrReviewRehearsalDrillRegistriesPacket {
    let mut packet = seeded_m5_orr_review_and_rehearsal_drill_registries();
    packet.packet_id =
        "m5-orr-review-and-rehearsal-drill-registries:orr-review-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5LaunchControlConsumerSurface::Shiproom)
        .expect("shiproom row present");
    row.qualification = M5LaunchControlQualificationClass::Beta;
    packet
}

/// Narrowed variant: the release-center row is narrowed to Preview pending rehearsal-drill parity on every
/// archetype; every row stays visible and every example stays honest.
pub fn seeded_m5_orr_review_and_rehearsal_drill_registries_rehearsal_drill_preview_narrowed(
) -> M5OrrReviewRehearsalDrillRegistriesPacket {
    let mut packet = seeded_m5_orr_review_and_rehearsal_drill_registries();
    packet.packet_id =
        "m5-orr-review-and-rehearsal-drill-registries:rehearsal-drill-preview:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5LaunchControlConsumerSurface::ReleaseCenter)
        .expect("release-center row present");
    row.qualification = M5LaunchControlQualificationClass::Preview;
    packet
}
