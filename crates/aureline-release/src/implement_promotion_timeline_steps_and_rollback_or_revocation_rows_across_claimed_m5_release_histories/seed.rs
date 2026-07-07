//! Canonical seed builders for the M5 promotion-timeline-step / rollback-or-revocation
//! row primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical release-history-primitive packet.
pub const M5_RELEASE_HISTORY_PRIMITIVE_PACKET_ID: &str =
    "m5-promotion-timeline-and-rollback-revocation-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-06T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked resolution case for a promotion timeline step.
#[allow(clippy::too_many_arguments)]
fn promo_case(
    event_identity: &str,
    source_stage: &str,
    destination_stage: &str,
    stage_state: M5PromotionStageState,
    reversible_window: M5ReversibleWindowState,
    rollout_ring: M5RolloutRing,
    break_glass_posture: M5BreakGlassPosture,
    digests: &[&str],
    evidence: &[&str],
    actors: &[&str],
    effective_time: &str,
) -> M5ReleaseHistoryResolutionCase {
    M5ReleaseHistoryResolutionCase::resolved(M5ReleaseHistoryEventInput {
        event_identity_repr: event_identity.to_owned(),
        event_kind: M5ReleaseHistoryEventKind::PromotionStep,
        source_stage_repr: source_stage.to_owned(),
        destination_stage_repr: destination_stage.to_owned(),
        stage_state,
        rollout_ring,
        reversible_window,
        digest_refs: strings(digests),
        evidence_refs: strings(evidence),
        approving_actors: strings(actors),
        effective_time_repr: effective_time.to_owned(),
        break_glass_posture,
        affected_node_set: Vec::new(),
        blast_radius: M5RollbackBlastRadius::SingleArtifact,
        node_targeting: M5NodeTargeting::NotApplicableTargeting,
        revocation_scope: M5RevocationScope::NoRevocation,
        last_known_good_target_repr: String::new(),
        continuity_note_repr: String::new(),
    })
}

/// Builds a worked resolution case for a rollback / revocation row.
#[allow(clippy::too_many_arguments)]
fn rollback_case(
    event_identity: &str,
    revocation_scope: M5RevocationScope,
    blast_radius: M5RollbackBlastRadius,
    node_targeting: M5NodeTargeting,
    break_glass_posture: M5BreakGlassPosture,
    digests: &[&str],
    evidence: &[&str],
    actors: &[&str],
    affected_nodes: &[&str],
    last_known_good_target: &str,
    continuity_note: &str,
    effective_time: &str,
) -> M5ReleaseHistoryResolutionCase {
    M5ReleaseHistoryResolutionCase::resolved(M5ReleaseHistoryEventInput {
        event_identity_repr: event_identity.to_owned(),
        event_kind: M5ReleaseHistoryEventKind::RollbackRevocationRow,
        source_stage_repr: String::new(),
        destination_stage_repr: String::new(),
        stage_state: M5PromotionStageState::StagePromoted,
        rollout_ring: M5RolloutRing::HeldNotPromoted,
        reversible_window: M5ReversibleWindowState::NotApplicableWindow,
        digest_refs: strings(digests),
        evidence_refs: strings(evidence),
        approving_actors: strings(actors),
        effective_time_repr: effective_time.to_owned(),
        break_glass_posture,
        affected_node_set: strings(affected_nodes),
        blast_radius,
        node_targeting,
        revocation_scope,
        last_known_good_target_repr: last_known_good_target.to_owned(),
        continuity_note_repr: continuity_note.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full anatomy, event-kind,
/// promotion-stage, rollout-ring, reversible-window, blast-radius, node-targeting,
/// revocation-scope, break-glass-posture, history-posture, block-reason, next-action,
/// export-field, and accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5ReleaseHistoryConsumerSurface,
    qualification: M5ReleaseCenterQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    example_resolutions: Vec<M5ReleaseHistoryResolutionCase>,
) -> M5ReleaseHistoryRow {
    M5ReleaseHistoryRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5PublicationSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5DeploymentLine::ALL.to_vec(),
        anatomy_parts: M5ReleaseHistoryAnatomyPart::ALL.to_vec(),
        event_kinds: M5ReleaseHistoryEventKind::ALL.to_vec(),
        promotion_stage_states: M5PromotionStageState::ALL.to_vec(),
        rollout_rings: M5RolloutRing::ALL.to_vec(),
        reversible_window_states: M5ReversibleWindowState::ALL.to_vec(),
        blast_radii: M5RollbackBlastRadius::ALL.to_vec(),
        node_targetings: M5NodeTargeting::ALL.to_vec(),
        revocation_scopes: M5RevocationScope::ALL.to_vec(),
        break_glass_postures: M5BreakGlassPosture::ALL.to_vec(),
        history_postures: M5ReleaseHistoryPosture::ALL.to_vec(),
        block_reasons: M5ReleaseHistoryBlockReason::ALL.to_vec(),
        next_actions: M5ReleaseHistoryNextAction::ALL.to_vec(),
        export_fields: M5ReleaseHistoryExportField::ALL.to_vec(),
        accessibility_routes: M5ReleaseCenterAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ReleaseCenterConsumerSurface::ReleaseCenterUi,
            M5ReleaseCenterConsumerSurface::HelpAbout,
            M5ReleaseCenterConsumerSurface::AdminConsole,
            M5ReleaseCenterConsumerSurface::MirrorConsole,
            M5ReleaseCenterConsumerSurface::SupportExport,
            M5ReleaseCenterConsumerSurface::CliInspect,
            M5ReleaseCenterConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5ReleaseCenterDowngradeTrigger::RolloutRingUnstated,
            M5ReleaseCenterDowngradeTrigger::RollbackBlastRadiusUnderstated,
            M5ReleaseCenterDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_RELEASE_HISTORY_STEP_SCHEMA_REF,
            M5_RELEASE_HISTORY_ROW_SCHEMA_REF,
            M5_RELEASE_HISTORY_OBJECT_MODEL_REF,
        ]),
        example_resolutions,
        reads_rollback_as_generic_status: false,
        drops_break_glass_attribution: false,
        hides_blast_radius_or_unaffected_nodes: false,
        lets_emergency_disappear_into_ci_only_metadata: false,
    }
}

fn history_rows() -> Vec<M5ReleaseHistoryRow> {
    use M5BreakGlassPosture as Bg;
    use M5NodeTargeting as Target;
    use M5PromotionStageState as Stage;
    use M5ReversibleWindowState as Window;
    use M5RevocationScope as Revoke;
    use M5RollbackBlastRadius as Blast;
    use M5RolloutRing as Ring;

    let mut rows = Vec::new();

    // 1. Release-center promotion timeline — a promotion step that was rolled back
    //    inside its reversible window (reversible, reconstructable from actors, digest
    //    joins, and evidence), and a promotion step whose stage is blocked (blocked with
    //    a self-contained banner naming the reason and next action).
    rows.push(base_row(
        M5ReleaseHistoryConsumerSurface::ReleaseCenterTimeline,
        M5ReleaseCenterQualificationClass::Stable,
        "Release-center promotion-timeline owner",
        "The release-center promotion timeline renders the shared history primitive so a promotion step from the canary ring to the pilot ring that was reversed inside its reversible window reads as promotion-recorded-reversible — reconstructable from its approving actors, immutable-digest joins, and evidence refs — while a promotion step whose stage is blocked reads as promotion-blocked with a self-contained banner naming the reason, the bound digest, its actors, and the resolve-stage-blocker next action",
        "evidence:m5-history-center:001",
        vec![
            promo_case(
                "event:promote-core-runtime 5.2.0 canary->pilot",
                "canary_ring",
                "pilot_ring",
                Stage::StageRolledBack,
                Window::ReversibleWithinWindow,
                Ring::CanaryRing,
                Bg::StandardChangeControl,
                &["sha256:aa11core", "sha512:bb22core"],
                &["evidence:qual-report:core-5.2.0", "evidence:canary-slo:core"],
                &["actor:release-captain:rowan", "actor:qa-lead:mira"],
                "2026-07-05T09:00:00Z",
            ),
            promo_case(
                "event:promote-registry 5.2.0 pilot->broad",
                "pilot_ring",
                "broad_ring",
                Stage::StageBlocked,
                Window::IrreversibleByDesign,
                Ring::BroadRing,
                Bg::StandardChangeControl,
                &["sha256:ee55registry"],
                &["evidence:blocker:registry-perf-regression"],
                &["actor:release-captain:rowan"],
                "2026-07-05T11:30:00Z",
            ),
        ],
    ));

    // 2. Update-center release history — a promotion step promoted past its reversible
    //    window (irreversible, honest about it), and a bounded rollback with an explicit
    //    family-scoped blast radius, an explicit partial node set, and a last-known-good
    //    target (never a generic status change).
    rows.push(base_row(
        M5ReleaseHistoryConsumerSurface::UpdateCenterHistory,
        M5ReleaseCenterQualificationClass::Stable,
        "Update-center release-history owner",
        "The update-center release history renders the shared primitive so a promotion step whose reversible window has expired reads as promotion-recorded-irreversible — honest that it can no longer be reversed — while a bounded rollback that repoints a mutable tag over an explicitly enumerated partial node set within a family-scoped blast radius, restoring a named last-known-good, reads as rollback-recorded-bounded rather than a generic status change",
        "evidence:m5-history-update:001",
        vec![
            promo_case(
                "event:promote-shell 5.2.0 pilot->broad",
                "pilot_ring",
                "broad_ring",
                Stage::StagePromoted,
                Window::ReversibleWindowExpired,
                Ring::PilotRing,
                Bg::StandardChangeControl,
                &["sha256:cc33shell"],
                &["evidence:qual-report:shell-5.2.0"],
                &["actor:release-captain:rowan"],
                "2026-07-04T14:00:00Z",
            ),
            rollback_case(
                "event:rollback-update 5.2.0->5.1.9 family",
                Revoke::TagRepointOnly,
                Blast::FamilyScoped,
                Target::PartialNodeSetExplicit,
                Bg::StandardChangeControl,
                &["sha256:dd44update"],
                &["evidence:incident:update-crash-loop"],
                &["actor:incident-commander:nadia"],
                &["node:update-worker-a", "node:update-worker-b"],
                "release:aureline-update 5.1.9",
                "Rolls the update family back to 5.1.9; unaffected nodes stay on 5.2.0",
                "2026-07-05T16:45:00Z",
            ),
        ],
    ));

    // 3. CLI history inspect — a promotion step still in progress, and a trust-root
    //    rotation attempted with no immutable-digest join (blocked for artifact-graph
    //    consistency).
    rows.push(base_row(
        M5ReleaseHistoryConsumerSurface::CliHistoryInspect,
        M5ReleaseCenterQualificationClass::Stable,
        "CLI history-inspect owner",
        "The CLI history-inspect surface renders the shared primitive so a promotion step still moving through the early-access ring reads as promotion-in-progress, while a trust-root rotation attempted with no immutable-digest join reads as history-blocked-missing-digest-join with a record-immutable-digest-join next action — artifact-graph consistency requires a digest join before the event can be recorded",
        "evidence:m5-history-cli:001",
        vec![
            promo_case(
                "event:promote-graph 5.3.0 early-access",
                "canary_ring",
                "early_access_ring",
                Stage::StageInProgress,
                Window::ReversibleWindowClosing,
                Ring::EarlyAccessRing,
                Bg::StandardChangeControl,
                &["sha256:ff66graph"],
                &["evidence:canary-slo:graph"],
                &["actor:release-captain:rowan"],
                "2026-07-06T08:15:00Z",
            ),
            rollback_case(
                "event:rotate-trust-root fleet",
                Revoke::TrustRootRotated,
                Blast::FleetWide,
                Target::AllNodes,
                Bg::StandardChangeControl,
                &[],
                &["evidence:security-advisory:trust-root"],
                &["actor:security-officer:idris"],
                &["node:fleet"],
                "release:aureline-trust-root prior",
                "Trust-root rotation pending a recorded digest join",
                "2026-07-06T10:00:00Z",
            ),
        ],
    ));

    // 4. Admin history report — a signing-key revocation with an explicit train-scoped
    //    blast radius (revocation recorded), and an emergency artifact revocation with no
    //    attributed actor (blocked-unattributed — break-glass must stay attributable).
    rows.push(base_row(
        M5ReleaseHistoryConsumerSurface::AdminHistoryReport,
        M5ReleaseCenterQualificationClass::Stable,
        "Admin history-report owner",
        "The admin history report renders the shared primitive so a signing-key revocation with an explicit train-scoped blast radius over the whole affected node set, restoring a named last-known-good, reads as revocation-recorded, while an emergency artifact revocation carrying no attributed actor reads as history-blocked-unattributed with an attribute-emergency-actor next action — break-glass must stay attributable and never disappear into CI-only metadata",
        "evidence:m5-history-admin:001",
        vec![
            rollback_case(
                "event:revoke-signing-key train",
                Revoke::SigningKeyRevoked,
                Blast::TrainScoped,
                Target::AllNodes,
                Bg::StandardChangeControl,
                &["sha256:aa77key"],
                &["evidence:security-advisory:key-compromise"],
                &["actor:security-officer:idris", "actor:release-captain:rowan"],
                &["node:train-5.2"],
                "release:aureline-signing-key prior",
                "Revokes the compromised signing key across the 5.2 train",
                "2026-07-05T20:00:00Z",
            ),
            rollback_case(
                "event:emergency-revoke-artifact cross-train",
                Revoke::ArtifactRevoked,
                Blast::CrossTrainScoped,
                Target::SingleNodeTargeted,
                Bg::BreakGlassUnattributed,
                &["sha256:bb88artifact"],
                &["evidence:incident:artifact-poisoned"],
                &[],
                &["node:cross-train-edge"],
                "release:aureline-core-runtime 5.1.9",
                "Emergency artifact revocation awaiting an attributed actor",
                "2026-07-06T02:30:00Z",
            ),
        ],
    ));

    // 5. Support history export — an emergency break-glass promotion attributed to a
    //    named actor (recorded and visible in the same history model), a rollback that
    //    names no last-known-good target (blocked), and an emergency break-glass artifact
    //    revocation with review pending (attributed and visible).
    rows.push(base_row(
        M5ReleaseHistoryConsumerSurface::SupportHistoryExport,
        M5ReleaseCenterQualificationClass::Stable,
        "Support history-export owner",
        "The support history export renders the shared primitive so an emergency break-glass promotion attributed to a named actor reads as emergency-break-glass-recorded and stays visible in the same history model, a rollback that names no last-known-good target reads as history-blocked-missing-last-known-good with a record-last-known-good-target next action, and an emergency break-glass artifact revocation with review pending stays attributed and visible — the same history vocabulary a support or evaluation reviewer reads across every surface",
        "evidence:m5-history-support:001",
        vec![
            promo_case(
                "event:emergency-promote-hotfix ga",
                "broad_ring",
                "general_availability",
                Stage::StagePending,
                Window::NotApplicableWindow,
                Ring::GeneralAvailability,
                Bg::BreakGlassAttributed,
                &["sha256:cc99hotfix"],
                &["evidence:incident:critical-cve", "evidence:hotfix-verify:cc99"],
                &["actor:incident-commander:nadia"],
                "2026-07-06T03:10:00Z",
            ),
            rollback_case(
                "event:rollback-no-lkg single",
                Revoke::NoRevocation,
                Blast::SingleArtifact,
                Target::PartialNodeSetExplicit,
                Bg::StandardChangeControl,
                &["sha256:dd00rollback"],
                &["evidence:incident:single-node-fault"],
                &["actor:incident-commander:nadia"],
                &["node:single-a"],
                "",
                "Rollback awaiting a recorded last-known-good target",
                "2026-07-06T04:00:00Z",
            ),
            rollback_case(
                "event:emergency-revoke-artifact pending-review family",
                Revoke::ArtifactRevoked,
                Blast::FamilyScoped,
                Target::PartialNodeSetExplicit,
                Bg::BreakGlassPendingReview,
                &["sha256:ee11pending"],
                &["evidence:incident:artifact-regression"],
                &["actor:incident-commander:nadia"],
                &["node:family-edge-a", "node:family-edge-b"],
                "release:aureline-mirror 5.1.9",
                "Emergency revocation executed; post-hoc review pending",
                "2026-07-06T05:20:00Z",
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5ReleaseHistoryGovernanceReview {
    M5ReleaseHistoryGovernanceReview {
        one_primitive_carries_history_truth: true,
        reconstructable_from_timeline: true,
        rollback_never_reads_as_generic_status: true,
        blast_radius_and_unaffected_nodes_explicit: true,
        break_glass_attribution_and_partial_scope_preserved: true,
        emergency_stays_visible_in_history_model: true,
        artifact_graph_consistency_preserved: true,
        blocked_state_always_shows_self_contained_banner: true,
        banner_names_exact_reason_and_next_action: true,
        support_export_reconstructs_history_truth: true,
        no_surface_invents_second_history_grammar: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5ReleaseHistoryConsumerProjection {
    M5ReleaseHistoryConsumerProjection {
        history_surfaces_consume_shared_primitive: true,
        history_resolver_reads_single_source: true,
        promotion_view_reads_single_source: true,
        rollback_view_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5ReleaseHistoryProofFreshness {
    M5ReleaseHistoryProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ReleaseHistoryReleasePosture {
    M5ReleaseHistoryReleasePosture {
        release_packet_ref: M5_RELEASE_HISTORY_ARTIFACT_REF.to_owned(),
        history_audit_ref: M5_RELEASE_HISTORY_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_RELEASE_HISTORY_STEP_SCHEMA_REF,
        M5_RELEASE_HISTORY_ROW_SCHEMA_REF,
        M5_RELEASE_HISTORY_DOC_REF,
        M5_RELEASE_HISTORY_COMPONENT_MATRIX_REF,
        M5_RELEASE_HISTORY_OBJECT_MODEL_REF,
    ])
}

/// Builds the canonical M5 release-history-primitive packet.
pub fn seeded_m5_release_history_primitive_packet() -> M5ReleaseHistoryPrimitivePacket {
    M5ReleaseHistoryPrimitivePacket::new(M5ReleaseHistoryPrimitivePacketInput {
        packet_id: M5_RELEASE_HISTORY_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 promotion-timeline-step and rollback/revocation-row primitive: event identity, event kind, source and destination stage, immutable-digest joins, evidence refs, approving actors, effective time, reversible window, affected node set, blast radius, node targeting, last-known-good target, continuity note, revocation scope, and break-glass attribution"
                .to_owned(),
        history_rows: history_rows(),
        vocabulary_set: M5ReleaseHistoryVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the update-center release history is held at Beta because a slice
/// of update-center exports do not yet render the reversible-window state on every
/// profile; every consumer stays visible.
pub fn seeded_m5_release_history_primitive_update_center_history_beta_narrowed(
) -> M5ReleaseHistoryPrimitivePacket {
    let mut packet = seeded_m5_release_history_primitive_packet();
    packet.packet_id =
        "m5-promotion-timeline-and-rollback-revocation-primitive:update-center-beta:0001".to_owned();
    let row = packet
        .history_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ReleaseHistoryConsumerSurface::UpdateCenterHistory)
        .expect("update-center history row present");
    row.qualification = M5ReleaseCenterQualificationClass::Beta;
    packet
}

/// Narrowed variant: the CLI history-inspect surface is narrowed to Preview pending
/// self-contained-banner parity proof across every headless export path; every consumer
/// stays visible.
pub fn seeded_m5_release_history_primitive_cli_history_inspect_preview_narrowed(
) -> M5ReleaseHistoryPrimitivePacket {
    let mut packet = seeded_m5_release_history_primitive_packet();
    packet.packet_id =
        "m5-promotion-timeline-and-rollback-revocation-primitive:cli-history-inspect-preview:0001"
            .to_owned();
    let row = packet
        .history_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ReleaseHistoryConsumerSurface::CliHistoryInspect)
        .expect("cli history-inspect row present");
    row.qualification = M5ReleaseCenterQualificationClass::Preview;
    packet
}
