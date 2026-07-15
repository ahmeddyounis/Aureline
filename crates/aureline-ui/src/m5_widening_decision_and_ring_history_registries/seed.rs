//! Canonical seed builders for the M5 widening-decision and ring-history registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean widening-decision and ring-history entries are built
//! so the one typed widening-decision object resolving per regression asset type, rings never advancing without a
//! visible known-limits and ring-history posture, partner / public support language never running ahead of ring
//! proof, the canonical / accessible / audit resolution forms, and the complete transition-identity /
//! active-stop-condition-ledger / ring-history-target / protected-metric-regression / packet-freshness /
//! crash-data-loss-or-trust / last-ring-transition-revision ring-history record are proven across the shiproom,
//! release-center, executive-steering, program-governance, diagnostics, and support surfaces without any
//! hand-copied per-ring assumption, widen-without-stop, incomplete object, hidden ring-history, or
//! resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_WIDENING_DECISION_RING_HISTORY_REGISTRIES_PACKET_ID: &str =
    "m5-widening-decision-and-ring-history-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn descriptor(input: M5WideningDecisionEntryResolutionInput) -> M5ResolvedWideningDecisionEntry {
    resolve_widening_decision_entry(input).expect("seed widening-decision entry resolves")
}

fn evidence(input: M5RingHistoryEntryResolutionInput) -> M5ResolvedRingHistoryEntry {
    resolve_ring_history_entry(input).expect("seed ring-history entry resolves")
}

fn all_forms() -> Vec<M5WideningDecisionResolutionForm> {
    M5WideningDecisionResolutionForm::ALL.to_vec()
}

// -- Clean widening-decision entries (typed object, rollback / diagnostics preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_descriptor_base(
    entry_id: &str,
    widening_event_binding_id: &str,
    token_name: &str,
    semantic_role: M5LaunchControlRole,
    widening_decision_packet_kind: M5WideningDecisionPacketKind,
    surface_context: M5WideningDecisionSurfaceContext,
    final_decision_reference: &str,
    open_risks_reference: &str,
    narrowed_claims_reference: &str,
    on_call_roster_reference: &str,
    signoff_roster_reference: &str,
    evidence_snapshot_reference: &str,
    decision_freshness_reference: &str,
) -> M5WideningDecisionEntryResolutionInput {
    M5WideningDecisionEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        widening_event_binding_id: widening_event_binding_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        widening_decision_packet_kind,
        surface_context,
        resolution_form_coverage: all_forms(),
        final_decision_reference: final_decision_reference.to_owned(),
        open_risks_reference: open_risks_reference.to_owned(),
        narrowed_claims_reference: narrowed_claims_reference.to_owned(),
        on_call_roster_reference: on_call_roster_reference.to_owned(),
        signoff_roster_reference: signoff_roster_reference.to_owned(),
        evidence_snapshot_reference: evidence_snapshot_reference.to_owned(),
        decision_freshness_reference: decision_freshness_reference.to_owned(),
        bound_to_registry: true,
        widening_decision_documented_before_widening: true,
        requires_documented_exception: false,
        attributable_asset_or_approved_exception: true,
        proof_fresh: true,
    }
}

fn descriptor_alpha_widening_decision_clean() -> M5ResolvedWideningDecisionEntry {
    descriptor(clean_descriptor_base(
        "descriptor:shiproom:dogfood-core-team-canary",
        "incident.lane.core-team-canary",
        "freeze.exception.core_team_canary",
        M5LaunchControlRole::CohortMembership,
        M5WideningDecisionPacketKind::AlphaWideningDecision,
        M5WideningDecisionSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-archetypes",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ))
}

fn descriptor_beta_widening_decision_clean() -> M5ResolvedWideningDecisionEntry {
    descriptor(clean_descriptor_base(
        "descriptor:release-center:migration-alpha",
        "incident.lane.migration-alpha",
        "freeze.exception.beta_widening_decision",
        M5LaunchControlRole::ReadinessEvent,
        M5WideningDecisionPacketKind::BetaWideningDecision,
        M5WideningDecisionSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-archetypes",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    ))
}

fn descriptor_release_candidate_widening_decision_clean() -> M5ResolvedWideningDecisionEntry {
    descriptor(clean_descriptor_base(
        "descriptor:program-governance:extension-author",
        "incident.lane.extension-author",
        "freeze.exception.release_candidate_widening_decision",
        M5LaunchControlRole::RehearsalCurrency,
        M5WideningDecisionPacketKind::ReleaseCandidateWideningDecision,
        M5WideningDecisionSurfaceContext::ProgramGovernanceSurface,
        "repo.rows.extension-author-archetypes",
        "bundle.ids.extension-author-0007",
        "install.topology.extension-author-sandbox",
        "toolchain.envelope.pinned-extension",
        "known-limits.published.extension-author",
        "rollback.target.extension-previous-abi",
        "diagnostics.posture.compatibility-telemetry",
    ))
}

fn descriptor_stable_widening_decision_clean() -> M5ResolvedWideningDecisionEntry {
    // A design-partner preview cohort is public-facing and keeps its support language matched to cohort proof.
    let mut base = clean_descriptor_base(
        "descriptor:executive-steering:design-partner-preview",
        "incident.lane.design-partner-preview",
        "freeze.exception.stable_widening_decision",
        M5LaunchControlRole::CohortMembership,
        M5WideningDecisionPacketKind::StableWideningDecision,
        M5WideningDecisionSurfaceContext::ExecutiveSteeringSurface,
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

fn descriptor_long_term_support_widening_decision_clean() -> M5ResolvedWideningDecisionEntry {
    // A public preview cohort is public-facing and keeps its support language matched to cohort proof.
    let mut base = clean_descriptor_base(
        "descriptor:support:public-preview",
        "incident.lane.public-preview",
        "freeze.exception.long_term_support_widening_decision",
        M5LaunchControlRole::ReadinessEvent,
        M5WideningDecisionPacketKind::LongTermSupportWideningDecision,
        M5WideningDecisionSurfaceContext::SupportOrExportForm,
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

fn descriptor_correction_reissue_decision_clean() -> M5ResolvedWideningDecisionEntry {
    descriptor(clean_descriptor_base(
        "descriptor:release-center:certified-archetype",
        "incident.lane.certified-archetype",
        "freeze.exception.correction_reissue_decision",
        M5LaunchControlRole::GoNoGoAuthority,
        M5WideningDecisionPacketKind::CorrectionReissueDecision,
        M5WideningDecisionSurfaceContext::ReleaseCenterSurface,
        "repo.rows.certified-archetype-archetypes",
        "bundle.ids.certified-0007",
        "install.topology.certified-archetype-fleet",
        "toolchain.envelope.pinned-certified",
        "known-limits.published.certified-archetype",
        "rollback.target.certified-previous-stable",
        "diagnostics.posture.certified-telemetry",
    ))
}

// -- Degraded widening-decision entries ---------------------------------------------------------

/// Degraded descriptor entry: the resolved descriptor object is incomplete — the bundle IDs are unstated.
fn descriptor_object_incomplete() -> M5ResolvedWideningDecisionEntry {
    let mut base = clean_descriptor_base(
        "descriptor:shiproom:incomplete",
        "incident.lane.core-team-canary",
        "freeze.exception.core_team_canary",
        M5LaunchControlRole::CohortMembership,
        M5WideningDecisionPacketKind::AlphaWideningDecision,
        M5WideningDecisionSurfaceContext::ShiproomSurface,
        "repo.rows.core-team-canary-archetypes",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    );
    base.open_risks_reference = "   ".to_owned();
    descriptor(base)
}

/// Degraded descriptor entry: the cohort's rollback and diagnostics posture is not preserved before widening —
/// a cohort widening without a rollback target and diagnostics posture. The structured blocker reason for a
/// widen-without-rollback attempt.
fn descriptor_widen_fold() -> M5ResolvedWideningDecisionEntry {
    let mut base = clean_descriptor_base(
        "descriptor:release-center:widen-fold",
        "incident.lane.migration-alpha",
        "freeze.exception.beta_widening_decision",
        M5LaunchControlRole::ReadinessEvent,
        M5WideningDecisionPacketKind::BetaWideningDecision,
        M5WideningDecisionSurfaceContext::ReleaseCenterSurface,
        "repo.rows.migration-alpha-archetypes",
        "bundle.ids.migration-alpha-0007",
        "install.topology.external-migration-alpha",
        "toolchain.envelope.pinned-migration",
        "known-limits.published.migration-alpha",
        "rollback.target.migration-previous-toolchain",
        "diagnostics.posture.migration-telemetry",
    );
    base.widening_decision_documented_before_widening = false;
    descriptor(base)
}

/// Degraded descriptor entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn descriptor_unbound() -> M5ResolvedWideningDecisionEntry {
    let mut base = clean_descriptor_base(
        "descriptor:executive-steering:unbound",
        "incident.lane.certified-archetype",
        "freeze.exception.correction_reissue_decision",
        M5LaunchControlRole::GoNoGoAuthority,
        M5WideningDecisionPacketKind::CorrectionReissueDecision,
        M5WideningDecisionSurfaceContext::ExecutiveSteeringSurface,
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
fn widening_decision_token_unstated() -> M5ResolvedWideningDecisionEntry {
    let mut base = clean_descriptor_base(
        "descriptor:program-governance:token-unstated",
        "incident.lane.extension-author",
        "  ",
        M5LaunchControlRole::RehearsalCurrency,
        M5WideningDecisionPacketKind::ReleaseCandidateWideningDecision,
        M5WideningDecisionSurfaceContext::ProgramGovernanceSurface,
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

// -- Clean ring-history entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_evidence_base(
    entry_id: &str,
    ring_history_ref: &str,
    token_name: &str,
    semantic_role: M5LaunchControlRole,
    ring_history_coverage: M5RingHistoryCoverageKind,
    surface_context: M5WideningDecisionSurfaceContext,
    resolved_coverage_identity: &str,
    evidence_snapshot_ledger: &str,
    orr_signoff_reference: &str,
    on_call_roster_state: &str,
    ring_history_freshness_state: &str,
    widening_stage_reference: &str,
    last_ring_history_revision: &str,
) -> M5RingHistoryEntryResolutionInput {
    M5RingHistoryEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        ring_history_ref: ring_history_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        ring_history_coverage,
        surface_context,
        resolution_form_coverage: all_forms(),
        resolved_coverage_identity: resolved_coverage_identity.to_owned(),
        evidence_snapshot_ledger: evidence_snapshot_ledger.to_owned(),
        orr_signoff_reference: orr_signoff_reference.to_owned(),
        on_call_roster_state: on_call_roster_state.to_owned(),
        ring_history_freshness_state: ring_history_freshness_state.to_owned(),
        widening_stage_reference: widening_stage_reference.to_owned(),
        last_ring_history_revision: last_ring_history_revision.to_owned(),
        keeps_evidence_snapshot_visible: true,
        ring_history_lineage_is_truthful: true,
        override_without_evidence_requested: false,
        blocked_until_evidence_linked: false,
        lineage_gap_present: false,
        lineage_gap_flagged: false,
        proof_fresh: true,
    }
}

fn evidence_dogfood_ring_clean() -> M5ResolvedRingHistoryEntry {
    // A dogfood-ring evidence packet carries partner / public support language bound to cohort proof.
    let mut base = clean_evidence_base(
        "evidence:shiproom:dogfood-ring",
        "incident.lane.core-team-canary",
        "go.no.go.core_team_canary",
        M5LaunchControlRole::CohortMembership,
        M5RingHistoryCoverageKind::RingHistoryScope,
        M5WideningDecisionSurfaceContext::ShiproomSurface,
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

fn evidence_rehearsal_currency_clean() -> M5ResolvedRingHistoryEntry {
    evidence(clean_evidence_base(
        "evidence:program-governance:rehearsal-currency",
        "incident.lane.extension-author",
        "go.no.go.release_candidate_widening_decision",
        M5LaunchControlRole::RehearsalCurrency,
        M5RingHistoryCoverageKind::PriorBlockerScope,
        M5WideningDecisionSurfaceContext::ProgramGovernanceSurface,
        "transition-id.extension-author-0007",
        "known-limits.ledger.extension-author",
        "rollback.target.ref.extension-author",
        "rehearsal.currency.mixed-version-current",
        "readiness.signoff.compatibility-reviewed",
        "support.language.extension-bound-to-proof",
        "widening.revision.0007",
    ))
}

fn evidence_ring_history_signoff_clean() -> M5ResolvedRingHistoryEntry {
    evidence(clean_evidence_base(
        "evidence:release-center:ring-history-signoff",
        "incident.lane.certified-archetype",
        "go.no.go.correction_reissue_decision",
        M5LaunchControlRole::GoNoGoAuthority,
        M5RingHistoryCoverageKind::PacketFreshnessScope,
        M5WideningDecisionSurfaceContext::ReleaseCenterSurface,
        "transition-id.certified-archetype-0007",
        "known-limits.ledger.certified-archetype",
        "rollback.target.ref.certified-archetype",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    ))
}

// -- Degraded ring-history entries ----------------------------------------------------

/// Degraded evidence entry: the evidence would run partner / public support language ahead of cohort proof — a
/// support-language reference present but not bound to cohort proof reads as trustworthy when the cohort proof
/// does not yet back it.
fn evidence_support_ahead() -> M5ResolvedRingHistoryEntry {
    let mut base = clean_evidence_base(
        "evidence:shiproom:support-ahead",
        "incident.lane.public-preview",
        "go.no.go.long_term_support_widening_decision",
        M5LaunchControlRole::ReadinessEvent,
        M5RingHistoryCoverageKind::RingHistoryScope,
        M5WideningDecisionSurfaceContext::ShiproomSurface,
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
fn evidence_form_incomplete() -> M5ResolvedRingHistoryEntry {
    let mut base = clean_evidence_base(
        "evidence:release-center:form-incomplete",
        "incident.lane.certified-archetype",
        "go.no.go.correction_reissue_decision",
        M5LaunchControlRole::GoNoGoAuthority,
        M5RingHistoryCoverageKind::PacketFreshnessScope,
        M5WideningDecisionSurfaceContext::ReleaseCenterSurface,
        "transition-id.certified-archetype-0007",
        "known-limits.ledger.certified-archetype",
        "rollback.target.ref.certified-archetype",
        "rehearsal.currency.publish-rollback-current",
        "readiness.signoff.orr-signed-and-recorded",
        "support.language.certified-bound-to-proof",
        "widening.revision.0007",
    );
    base.resolution_form_coverage = vec![M5WideningDecisionResolutionForm::CanonicalObject];
    evidence(base)
}

/// Degraded evidence entry: the evidence scope is unclassified.
fn ring_history_coverage_unclassified() -> M5ResolvedRingHistoryEntry {
    evidence(clean_evidence_base(
        "evidence:executive-steering:scope-unclassified",
        "incident.lane.design-partner-preview",
        "go.no.go.stable_widening_decision",
        M5LaunchControlRole::CohortMembership,
        M5RingHistoryCoverageKind::CoverageUnclassified,
        M5WideningDecisionSurfaceContext::ExecutiveSteeringSurface,
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
    consumer_surface: M5WideningDecisionRingHistoryRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5LaunchControlDowngradeTrigger>,
    widening_decision_entries: Vec<M5ResolvedWideningDecisionEntry>,
    ring_history_entries: Vec<M5ResolvedRingHistoryEntry>,
) -> M5WideningDecisionRingHistoryRegistriesRow {
    M5WideningDecisionRingHistoryRegistriesRow {
        consumer_surface,
        qualification: M5LaunchControlQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        widening_stages: M5LaunchControlWideningStage::ALL.to_vec(),
        required_labels: M5LaunchControlRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5LaunchControlAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5WideningDecisionAnatomyPart::ALL.to_vec(),
        export_fields: M5WideningDecisionExportField::ALL.to_vec(),
        downgrade_triggers,
        widening_decision_entries,
        ring_history_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_WIDENING_DECISION_RING_HISTORY_REGISTRIES_SCHEMA_REF,
            M5_WIDENING_DECISION_DOMAIN_SCHEMA_REF,
            M5_RING_HISTORY_DOMAIN_SCHEMA_REF,
        ]),
        widens_a_stable_claim_without_a_durable_go_no_go_record: false,
        drops_the_evidence_snapshot_or_roster_from_a_widening_record: false,
        hides_the_ring_history_or_prior_blockers_before_widening: false,
        implies_green_when_go_no_go_records_or_evidence_snapshots_are_stale: false,
    }
}

fn registry_rows() -> Vec<M5WideningDecisionRingHistoryRegistriesRow> {
    use M5LaunchControlConsumerSurface as C;
    use M5LaunchControlDowngradeTrigger as D;

    vec![
        base_row(
            C::Shiproom,
            "Shiproom owner",
            "The shiproom resolves the monthly-ORR packet kind to one typed object — packet kind, readiness scope, release / advisory / support-room / docs-comms / backup-signer roster, and rehearsal-freshness expiry — from the shared registry and proves the full-roster ring-history record for the launch-bearing lane; an ORR packet missing its roster and a ring-history record that implies green while the rehearsal packet is stale degrade honestly instead of reading as a clean pass",
            "evidence:m5-launch-control-shiproom:001",
            vec![
                D::WidenedWithoutCurrentCohortEvidence,
                D::RanPartnerOrPublicLanguageAheadOfCohortProof,
                D::ProofStale,
            ],
            vec![
                descriptor_alpha_widening_decision_clean(),
                descriptor_object_incomplete(),
            ],
            vec![evidence_dogfood_ring_clean(), evidence_support_ahead()],
        ),
        base_row(
            C::ReleaseCenter,
            "Release-center owner",
            "The release center resolves the release-candidate-ORR packet kind and the conditional-roster ring-history record while keeping the rehearsal evidence visible; a lane widening on a stale or skipped rehearsal packet and a resolution-form gap on a ring-history record are caught before a screenshot can reintroduce a false-truth reading",
            "evidence:m5-launch-control-release-center:001",
            vec![
                D::WidenedWithoutCurrentCohortEvidence,
                D::ImpliedGreenWhileGoNoGoOrOrrWasStale,
                D::ProofStale,
            ],
            vec![descriptor_beta_widening_decision_clean(), descriptor_widen_fold()],
            vec![evidence_ring_history_signoff_clean(), evidence_form_incomplete()],
        ),
        base_row(
            C::ExecutiveSteering,
            "Executive-steering owner",
            "Executive steering resolves the mixed-version-drill packet kind while keeping its partner support language matched to rehearsal proof and reports the roster-coverage ring-history record; an ORR packet that is a hand-copied per-entry assumption and a ring-history record on an unclassified roster coverage degrade honestly",
            "evidence:m5-launch-control-executive-steering:001",
            vec![
                D::RegistryReferenceUnstated,
                D::ReadinessStateUnstated,
                D::ProofStale,
            ],
            vec![
                descriptor_stable_widening_decision_clean(),
                descriptor_unbound(),
            ],
            vec![ring_history_coverage_unclassified()],
        ),
        base_row(
            C::ProgramGovernance,
            "Program-governance owner",
            "Program governance resolves the publish-rollback-drill packet kind and the backup-roster ring-history record bound to the registry; an unstated registry token on an ORR packet is caught before it can drift",
            "evidence:m5-launch-control-program-governance:001",
            vec![
                D::RegistryReferenceUnstated,
                D::CohortMembershipUnstated,
                D::ProofStale,
            ],
            vec![
                descriptor_release_candidate_widening_decision_clean(),
                widening_decision_token_unstated(),
            ],
            vec![evidence_rehearsal_currency_clean()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics renders the same resolved ORR-packet and ring-history truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied readiness table; the support-incident-handoff-drill ORR packet and the full-roster ring-history record stay inspectable off-renderer",
            "evidence:m5-launch-control-diagnostics:001",
            vec![
                D::CohortMembershipUnstated,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![descriptor_correction_reissue_decision_clean()],
            vec![evidence_ring_history_signoff_clean()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved ORR-packet and ring-history truth, so a hand-copied constant, an unstated registry token, a widen-on-stale-rehearsal attempt, or support language running ahead of rehearsal proof is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-launch-control-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::RanPartnerOrPublicLanguageAheadOfCohortProof,
                D::ProofStale,
            ],
            vec![descriptor_long_term_support_widening_decision_clean()],
            vec![evidence_rehearsal_currency_clean()],
        ),
    ]
}

fn governance_review() -> M5WideningDecisionRingHistoryRegistriesGovernanceReview {
    M5WideningDecisionRingHistoryRegistriesGovernanceReview {
        widening_decision_registry_names_token_role_and_type: true,
        type_resolves_to_typed_widening_decision_from_shared_registry: true,
        build_row_and_cohort_lineage_published: true,
        scope_cannot_widen_without_documented_widening_decision: true,
        ring_history_keeps_evidence_visible_and_blocks_stale_green: true,
        approved_exception_matched_to_scope_for_widening: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shiproom_release_center_executive_steering_and_program_governance_read_single_source: true,
        exception_or_ring_history_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5WideningDecisionRingHistoryRegistriesConsumerProjection {
    M5WideningDecisionRingHistoryRegistriesConsumerProjection {
        shiproom_and_release_center_consume_shared_registries: true,
        executive_steering_and_program_governance_consume_shared_registries: true,
        diagnostics_and_public_proof_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5WideningDecisionRingHistoryRegistriesProofFreshness {
    M5WideningDecisionRingHistoryRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5WideningDecisionRingHistoryRegistriesReleasePosture {
    M5WideningDecisionRingHistoryRegistriesReleasePosture {
        proof_packet_ref: M5_WIDENING_DECISION_RING_HISTORY_REGISTRIES_ARTIFACT_REF.to_owned(),
        ring_history_control_audit_ref: M5_WIDENING_DECISION_RING_HISTORY_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_WIDENING_DECISION_RING_HISTORY_REGISTRIES_SCHEMA_REF,
        M5_WIDENING_DECISION_RING_HISTORY_REGISTRIES_DOC_REF,
        M5_LAUNCH_CONTROL_MATRIX_SCHEMA_REF,
        M5_LAUNCH_CONTROL_MATRIX_DOC_REF,
        M5_WIDENING_DECISION_DOMAIN_SCHEMA_REF,
        M5_RING_HISTORY_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 widening-decision and ring-history registries packet.
pub fn seeded_m5_widening_decision_and_ring_history_registries(
) -> M5WideningDecisionRingHistoryRegistriesPacket {
    M5WideningDecisionRingHistoryRegistriesPacket::new(
        M5WideningDecisionRingHistoryRegistriesPacketInput {
            packet_id: M5_WIDENING_DECISION_RING_HISTORY_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 stable go/no-go widening-decision and ring-history registries with one durable go/no-go record resolving per widening event (alpha, beta, release-candidate, stable, long-term-support, correction-reissue widening), each preserving its final go/no-go decision, open risks, narrowed claims, named on-call and signoff roster, and exact evidence snapshot, a stable claim never widening on a stale or dropped record, ring history and prior blockers reading as first-class blockers, canonical / accessible / audit resolution-form coverage, and the complete resolved-coverage-identity / ring-history-ledger / signoff / on-call-roster / packet-freshness / widening-stage / last-ring-history-revision ring-history snapshot across shiproom, release-center, executive-steering, program-governance, diagnostics, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5WideningDecisionRingHistoryRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the shiproom row is held at Beta pending widening-decision parity on every archetype; every
/// row stays visible and every example stays honest.
pub fn seeded_m5_widening_decision_and_ring_history_registries_widening_decision_beta_narrowed(
) -> M5WideningDecisionRingHistoryRegistriesPacket {
    let mut packet = seeded_m5_widening_decision_and_ring_history_registries();
    packet.packet_id =
        "m5-widening-decision-and-ring-history-registries:widening-decision-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5LaunchControlConsumerSurface::Shiproom)
        .expect("shiproom row present");
    row.qualification = M5LaunchControlQualificationClass::Beta;
    packet
}

/// Narrowed variant: the release-center row is narrowed to Preview pending ring-history parity on every
/// archetype; every row stays visible and every example stays honest.
pub fn seeded_m5_widening_decision_and_ring_history_registries_ring_history_preview_narrowed(
) -> M5WideningDecisionRingHistoryRegistriesPacket {
    let mut packet = seeded_m5_widening_decision_and_ring_history_registries();
    packet.packet_id =
        "m5-widening-decision-and-ring-history-registries:ring-history-preview:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5LaunchControlConsumerSurface::ReleaseCenter)
        .expect("release-center row present");
    row.qualification = M5LaunchControlQualificationClass::Preview;
    packet
}
