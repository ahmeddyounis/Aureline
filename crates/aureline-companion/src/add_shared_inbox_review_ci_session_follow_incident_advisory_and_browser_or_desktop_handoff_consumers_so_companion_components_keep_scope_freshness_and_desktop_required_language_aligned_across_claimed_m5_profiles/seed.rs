//! Canonical seed builders for the M5 companion component-consumer lane.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix, the
//! artifact, the worked bindings, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical companion component-consumer packet.
pub const M5_COMPANION_COMPONENT_CONSUMER_PACKET_ID: &str =
    "m5-companion-component-consumer:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked binding case for one consumer/family adoption.
fn case(
    consumer: M5CompanionComponentConsumer,
    component_family: M5CompanionComponentFamily,
    parity_health: M5CompanionConsumerParityHealth,
    export_caveats: &[M5CompanionConsumerExportCaveat],
    note: &str,
) -> M5CompanionComponentBindingCase {
    M5CompanionComponentBindingCase::resolved(M5CompanionComponentBindingInput {
        consumer,
        component_family,
        descriptor_families: M5CompanionComponentDescriptor::ALL.to_vec(),
        parity_health,
        export_caveats: export_caveats.to_vec(),
        note_repr: Some(note.to_owned()),
    })
}

/// Builds a component binding that points at its canonical family refs.
fn binding(
    component_family: M5CompanionComponentFamily,
    example_bindings: Vec<M5CompanionComponentBindingCase>,
) -> M5CompanionComponentBinding {
    M5CompanionComponentBinding {
        component_family,
        canonical_schema_ref: family_canonical_schema_ref(component_family).to_owned(),
        canonical_artifact_ref: family_canonical_artifact_ref(component_family).to_owned(),
        references_canonical_not_local_prose: true,
        example_bindings,
    }
}

/// A base row with the shared parity vocabulary filled in.
fn base_row(
    consumer: M5CompanionComponentConsumer,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    component_bindings: Vec<M5CompanionComponentBinding>,
) -> M5CompanionComponentConsumerRow {
    M5CompanionComponentConsumerRow {
        consumer,
        qualification: M5CompanionQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5CompanionSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5CompanionDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5CompanionConsumerAnatomyPart::ALL.to_vec(),
        descriptor_families: M5CompanionComponentDescriptor::ALL.to_vec(),
        parity_health_modes: M5CompanionConsumerParityHealth::ALL.to_vec(),
        export_caveats: M5CompanionConsumerExportCaveat::ALL.to_vec(),
        claim_parity_states: M5CompanionClaimParityState::ALL.to_vec(),
        narrowing_reasons: M5CompanionConsumerNarrowingReason::ALL.to_vec(),
        recovery_actions: M5CompanionConsumerRecoveryAction::ALL.to_vec(),
        export_fields: M5CompanionConsumerExportField::ALL.to_vec(),
        accessibility_routes: M5CompanionAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5CompanionConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5CompanionDowngradeTrigger::ObjectIdentityUnstated,
            M5CompanionDowngradeTrigger::ClientScopeUnstated,
            M5CompanionDowngradeTrigger::FreshnessHidden,
            M5CompanionDowngradeTrigger::CapabilityBoundaryUnstated,
            M5CompanionDowngradeTrigger::SeverityUnstated,
            M5CompanionDowngradeTrigger::HandoffTargetUnresolved,
            M5CompanionDowngradeTrigger::StaleShownAsLive,
            M5CompanionDowngradeTrigger::DesktopRequiredActionOfferedInline,
            M5CompanionDowngradeTrigger::GenericCompanionWordingUsed,
            M5CompanionDowngradeTrigger::ProofStale,
        ],
        component_bindings,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_COMPANION_COMPONENT_CONSUMER_SCHEMA_REF,
            M5_COMPANION_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        ]),
        rewords_claims_per_surface: false,
        invents_new_companion_grammar: false,
        drops_scope_freshness_capability_severity_or_handoff_truth_when_narrowed: false,
        shows_stale_or_desktop_required_state_as_live_and_companion_safe: false,
        inherits_stronger_label_from_healthier_profile: false,
        uses_generic_companion_wording: false,
    }
}

#[allow(clippy::vec_init_then_push)]
fn consumer_rows() -> Vec<M5CompanionComponentConsumerRow> {
    use M5CompanionComponentConsumer as Consumer;
    use M5CompanionComponentFamily as Family;
    use M5CompanionConsumerExportCaveat as Caveat;
    use M5CompanionConsumerParityHealth as Health;

    let mut rows = Vec::new();

    // 1. Notification inbox — the notification row at full parity, plus the mobile review card
    //    auto-narrowed to a cached value because only the last-known review preview is available.
    rows.push(base_row(
        Consumer::Inbox,
        "Notification-inbox surface owner",
        "The notification inbox adopts the notification row at full parity and the mobile review card auto-narrowed to a cached value, pointing at the canonical component schemas so object identity, workspace/repo client scope, freshness, the companion-versus-desktop capability boundary, severity, and the exact desktop-handoff target match what the review queue, CI, session-follow, incident, advisory, Help / docs, the support / export desk, the desktop-handoff surface, and the export packet read",
        "evidence:m5-companion-consumer-inbox:001",
        vec![
            binding(
                Family::NotificationRow,
                vec![case(
                    Consumer::Inbox,
                    Family::NotificationRow,
                    Health::FullParity,
                    &[],
                    "inbox notification row at full parity",
                )],
            ),
            binding(
                Family::MobileReviewCard,
                vec![case(
                    Consumer::Inbox,
                    Family::MobileReviewCard,
                    Health::CachedNarrowed,
                    &[Caveat::CachedNotLive],
                    "inbox review card narrowed to a cached value",
                )],
            ),
        ],
    ));

    // 2. Review queue — the mobile review card at full parity, plus the desktop-handoff sheet
    //    auto-narrowed because completing the review is desktop-required.
    rows.push(base_row(
        Consumer::Review,
        "Review-queue surface owner",
        "The review queue adopts the mobile review card at full parity and the desktop-handoff sheet auto-narrowed because completing the review is desktop-required, keeping object identity, client scope, freshness, capability boundary, severity, and handoff target explicit so a desktop-required action is never implied companion-safe",
        "evidence:m5-companion-consumer-review:001",
        vec![
            binding(
                Family::MobileReviewCard,
                vec![case(
                    Consumer::Review,
                    Family::MobileReviewCard,
                    Health::FullParity,
                    &[],
                    "review mobile review card at full parity",
                )],
            ),
            binding(
                Family::DesktopHandoffSheet,
                vec![case(
                    Consumer::Review,
                    Family::DesktopHandoffSheet,
                    Health::DesktopRequiredNarrowed,
                    &[Caveat::DesktopRequiredNotCompanionSafe],
                    "review handoff sheet narrowed by a desktop-required action",
                )],
            ),
        ],
    ));

    // 3. CI status — the CI-status card at full parity, plus the notification row auto-narrowed
    //    because its CI notification is stale beyond its freshness window.
    rows.push(base_row(
        Consumer::Ci,
        "CI-status surface owner",
        "The CI-status surface adopts the CI-status card at full parity and the notification row auto-narrowed because a CI notification is stale beyond its freshness window, keeping object identity, client scope, freshness, capability boundary, severity, and handoff target explicit so a stale status never reads as a live pass or fail",
        "evidence:m5-companion-consumer-ci:001",
        vec![
            binding(
                Family::CiStatusCard,
                vec![case(
                    Consumer::Ci,
                    Family::CiStatusCard,
                    Health::FullParity,
                    &[],
                    "ci status card at full parity",
                )],
            ),
            binding(
                Family::NotificationRow,
                vec![case(
                    Consumer::Ci,
                    Family::NotificationRow,
                    Health::StaleNarrowed,
                    &[Caveat::StaleNotLive],
                    "ci notification row narrowed by a stale status",
                )],
            ),
        ],
    ));

    // 4. Session follow — the session-follow tile at full parity, plus the CI-status card
    //    auto-narrowed to a cached value because the followed session's CI is not live-refreshing.
    rows.push(base_row(
        Consumer::SessionFollow,
        "Session-follow surface owner",
        "The session-follow surface adopts the session-follow tile at full parity and the CI-status card auto-narrowed to a cached value because a followed session's CI is not live-refreshing, keeping object identity, client scope, freshness, capability boundary, severity, and handoff target explicit so a cached tile never reads as a live-following one",
        "evidence:m5-companion-consumer-session-follow:001",
        vec![
            binding(
                Family::SessionFollowTile,
                vec![case(
                    Consumer::SessionFollow,
                    Family::SessionFollowTile,
                    Health::FullParity,
                    &[],
                    "session-follow tile at full parity",
                )],
            ),
            binding(
                Family::CiStatusCard,
                vec![case(
                    Consumer::SessionFollow,
                    Family::CiStatusCard,
                    Health::CachedNarrowed,
                    &[Caveat::CachedNotLive],
                    "session-follow ci status card narrowed to a cached value",
                )],
            ),
        ],
    ));

    // 5. Incident awareness — the incident-snapshot card and desktop-handoff sheet at full
    //    parity: the incident's severity, latest status, and exact handoff target are exact.
    rows.push(base_row(
        Consumer::Incident,
        "Incident-awareness surface owner",
        "The incident-awareness surface adopts the incident-snapshot card and desktop-handoff sheet at full parity, referencing the canonical component schemas so an incident's severity, latest status, freshness, and the exact object that opens on desktop stay one truth and a stale incident is never shown as live",
        "evidence:m5-companion-consumer-incident:001",
        vec![
            binding(
                Family::IncidentSnapshotCard,
                vec![case(
                    Consumer::Incident,
                    Family::IncidentSnapshotCard,
                    Health::FullParity,
                    &[],
                    "incident snapshot card at full parity",
                )],
            ),
            binding(
                Family::DesktopHandoffSheet,
                vec![case(
                    Consumer::Incident,
                    Family::DesktopHandoffSheet,
                    Health::FullParity,
                    &[],
                    "incident desktop-handoff sheet at full parity",
                )],
            ),
        ],
    ));

    // 6. Advisory center — the notification row at full parity, plus the incident-snapshot card
    //    auto-narrowed because acting on the advisory is blocked by policy on the companion.
    rows.push(base_row(
        Consumer::Advisory,
        "Advisory-center surface owner",
        "The advisory center adopts the notification row at full parity and the incident-snapshot card auto-narrowed because acting on the advisory is blocked by policy on the companion, keeping object identity, client scope, freshness, capability boundary, severity, and handoff target explicit so a policy-blocked path never reads as companion-safe",
        "evidence:m5-companion-consumer-advisory:001",
        vec![
            binding(
                Family::NotificationRow,
                vec![case(
                    Consumer::Advisory,
                    Family::NotificationRow,
                    Health::FullParity,
                    &[],
                    "advisory notification row at full parity",
                )],
            ),
            binding(
                Family::IncidentSnapshotCard,
                vec![case(
                    Consumer::Advisory,
                    Family::IncidentSnapshotCard,
                    Health::PolicyBlockedNarrowed,
                    &[Caveat::PolicyBlockedNotCompanionSafe],
                    "advisory incident snapshot card narrowed by a policy-blocked path",
                )],
            ),
        ],
    ));

    // 7. Help / docs — the notification row, mobile review card, and CI-status card all at full
    //    parity: documentation describes the same object, scope, freshness, and capability truth
    //    the product renders.
    rows.push(base_row(
        Consumer::Help,
        "Help / docs surface owner",
        "Help / docs adopt the notification row, mobile review card, and CI-status card at full parity, referencing the canonical component schemas so object identity, client scope, freshness, capability boundary, severity, and handoff target stay one truth across every claimed companion surface rather than being re-worded in prose",
        "evidence:m5-companion-consumer-help:001",
        vec![
            binding(
                Family::NotificationRow,
                vec![case(
                    Consumer::Help,
                    Family::NotificationRow,
                    Health::FullParity,
                    &[],
                    "help notification row at full parity",
                )],
            ),
            binding(
                Family::MobileReviewCard,
                vec![case(
                    Consumer::Help,
                    Family::MobileReviewCard,
                    Health::FullParity,
                    &[],
                    "help mobile review card at full parity",
                )],
            ),
            binding(
                Family::CiStatusCard,
                vec![case(
                    Consumer::Help,
                    Family::CiStatusCard,
                    Health::FullParity,
                    &[],
                    "help ci status card at full parity",
                )],
            ),
        ],
    ));

    // 8. Support / export desk — all six families, referencing the canonical schemas so its prose
    //    can never drift from the product truth. This is the authoritative rendering every other
    //    surface keeps parity with.
    rows.push(base_row(
        Consumer::Support,
        "Support / export desk surface owner",
        "The support / export desk adopts the notification row, mobile review card, CI-status card, session-follow tile, incident-snapshot card, and desktop-handoff sheet, referencing the canonical component schemas so its prose can never drift from the product truth and keeping object identity, client scope, freshness, capability boundary, severity, and handoff target exact in every exported case",
        "evidence:m5-companion-consumer-support:001",
        vec![
            binding(
                Family::NotificationRow,
                vec![case(
                    Consumer::Support,
                    Family::NotificationRow,
                    Health::FullParity,
                    &[],
                    "support notification row at full parity",
                )],
            ),
            binding(
                Family::MobileReviewCard,
                vec![case(
                    Consumer::Support,
                    Family::MobileReviewCard,
                    Health::FullParity,
                    &[],
                    "support mobile review card at full parity",
                )],
            ),
            binding(
                Family::CiStatusCard,
                vec![case(
                    Consumer::Support,
                    Family::CiStatusCard,
                    Health::FullParity,
                    &[],
                    "support ci status card at full parity",
                )],
            ),
            binding(
                Family::SessionFollowTile,
                vec![case(
                    Consumer::Support,
                    Family::SessionFollowTile,
                    Health::FullParity,
                    &[],
                    "support session-follow tile at full parity",
                )],
            ),
            binding(
                Family::IncidentSnapshotCard,
                vec![case(
                    Consumer::Support,
                    Family::IncidentSnapshotCard,
                    Health::FullParity,
                    &[],
                    "support incident snapshot card at full parity",
                )],
            ),
            binding(
                Family::DesktopHandoffSheet,
                vec![case(
                    Consumer::Support,
                    Family::DesktopHandoffSheet,
                    Health::FullParity,
                    &[],
                    "support desktop-handoff sheet at full parity",
                )],
            ),
        ],
    ));

    // 9. Desktop handoff — the desktop-handoff sheet and session-follow tile at full parity: the
    //    exact target that opens on desktop and the followed session it belongs to stay explicit.
    rows.push(base_row(
        Consumer::Handoff,
        "Desktop-handoff surface owner",
        "The desktop-handoff surface adopts the desktop-handoff sheet and session-follow tile at full parity, keeping object identity, client scope, freshness, capability boundary, severity, and handoff target explicit so a handoff always names the exact object that opens on desktop rather than a generic activity page",
        "evidence:m5-companion-consumer-handoff:001",
        vec![
            binding(
                Family::DesktopHandoffSheet,
                vec![case(
                    Consumer::Handoff,
                    Family::DesktopHandoffSheet,
                    Health::FullParity,
                    &[],
                    "handoff desktop-handoff sheet at full parity",
                )],
            ),
            binding(
                Family::SessionFollowTile,
                vec![case(
                    Consumer::Handoff,
                    Family::SessionFollowTile,
                    Health::FullParity,
                    &[],
                    "handoff session-follow tile at full parity",
                )],
            ),
        ],
    ));

    // 10. Export packet — the desktop-handoff sheet, incident-snapshot card, and CI-status card at
    //     full parity, so an exported companion packet carries the exact handoff target, incident
    //     context, and CI status without leaking any raw payload.
    rows.push(base_row(
        Consumer::Export,
        "Export packet surface owner",
        "The export packet adopts the desktop-handoff sheet, incident-snapshot card, and CI-status card at full parity, keeping object identity, client scope, freshness, capability boundary, severity, and handoff target explicit so an exported packet always states the exact desktop target and never implies a stale card is live",
        "evidence:m5-companion-consumer-export:001",
        vec![
            binding(
                Family::DesktopHandoffSheet,
                vec![case(
                    Consumer::Export,
                    Family::DesktopHandoffSheet,
                    Health::FullParity,
                    &[],
                    "export desktop-handoff sheet at full parity",
                )],
            ),
            binding(
                Family::IncidentSnapshotCard,
                vec![case(
                    Consumer::Export,
                    Family::IncidentSnapshotCard,
                    Health::FullParity,
                    &[],
                    "export incident snapshot card at full parity",
                )],
            ),
            binding(
                Family::CiStatusCard,
                vec![case(
                    Consumer::Export,
                    Family::CiStatusCard,
                    Health::FullParity,
                    &[],
                    "export ci status card at full parity",
                )],
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5CompanionComponentConsumerGovernanceReview {
    M5CompanionComponentConsumerGovernanceReview {
        consumers_adopt_shared_primitives: true,
        consumers_reference_canonical_schema: true,
        descriptor_vocabulary_shared_not_reworded: true,
        no_consumer_invents_new_grammar: true,
        object_scope_freshness_capability_severity_handoff_explicit_on_every_surface: true,
        degraded_state_auto_narrows_claim: true,
        narrowed_rendering_always_shows_self_contained_banner: true,
        banner_names_exact_reason_and_recovery_action: true,
        stale_or_desktop_required_state_never_shown_as_live_and_companion_safe: true,
        no_generic_companion_wording_conceals_object_scope_or_capability: true,
        help_support_export_present_same_companion_truth: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5CompanionComponentConsumerProjection {
    M5CompanionComponentConsumerProjection {
        all_consumers_adopt_shared_components: true,
        object_identity_reads_single_source: true,
        client_scope_reads_single_source: true,
        freshness_reads_single_source: true,
        capability_boundary_reads_single_source: true,
        handoff_target_reads_single_source: true,
    }
}

fn proof_freshness() -> M5CompanionComponentConsumerProofFreshness {
    M5CompanionComponentConsumerProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5CompanionComponentConsumerReleasePosture {
    M5CompanionComponentConsumerReleasePosture {
        release_packet_ref: M5_COMPANION_COMPONENT_CONSUMER_ARTIFACT_REF.to_owned(),
        companion_consumer_audit_ref: M5_COMPANION_COMPONENT_CONSUMER_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_COMPANION_COMPONENT_CONSUMER_SCHEMA_REF,
        M5_COMPANION_COMPONENT_CONSUMER_DOC_REF,
        M5_COMPANION_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF,
        M5_COMPANION_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        family_canonical_schema_ref(M5CompanionComponentFamily::NotificationRow),
        family_canonical_schema_ref(M5CompanionComponentFamily::CiStatusCard),
        family_canonical_schema_ref(M5CompanionComponentFamily::IncidentSnapshotCard),
    ])
}

/// Builds the canonical M5 companion component-consumer packet.
pub fn seeded_m5_companion_component_consumer_packet() -> M5CompanionComponentConsumerPacket {
    M5CompanionComponentConsumerPacket::new(M5CompanionComponentConsumerPacketInput {
        packet_id: M5_COMPANION_COMPONENT_CONSUMER_PACKET_ID.to_owned(),
        matrix_label:
            "M5 companion component consumers: the notification inbox, the review queue, CI status, session follow, incident awareness, the advisory center, Help / docs, the support / export desk, the desktop-handoff surface, and the export packet keep scope, freshness, and desktop-required parity"
                .to_owned(),
        consumer_rows: consumer_rows(),
        vocabulary_set: M5CompanionComponentConsumerVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the advisory center is held at Beta because a slice of advisory renderings
/// still resolve a policy-blocked companion path; every consumer stays visible.
pub fn seeded_m5_companion_component_consumer_advisory_beta_narrowed(
) -> M5CompanionComponentConsumerPacket {
    let mut packet = seeded_m5_companion_component_consumer_packet();
    packet.packet_id = "m5-companion-component-consumer:advisory-beta:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5CompanionComponentConsumer::Advisory)
        .expect("advisory row present");
    row.qualification = M5CompanionQualificationClass::Beta;
    packet
}

/// Narrowed variant: the desktop-handoff surface is held at Preview because a slice of handoff
/// renderings still resolve a desktop-required action; every consumer stays visible.
pub fn seeded_m5_companion_component_consumer_handoff_preview_narrowed(
) -> M5CompanionComponentConsumerPacket {
    let mut packet = seeded_m5_companion_component_consumer_packet();
    packet.packet_id = "m5-companion-component-consumer:handoff-preview:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5CompanionComponentConsumer::Handoff)
        .expect("handoff row present");
    row.qualification = M5CompanionQualificationClass::Preview;
    packet
}
