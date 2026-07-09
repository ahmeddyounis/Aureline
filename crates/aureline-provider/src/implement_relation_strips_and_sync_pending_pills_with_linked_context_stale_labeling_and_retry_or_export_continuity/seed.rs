//! Canonical seed builders for the relation-strip / sync-pending-pill controls.
//!
//! These builders are the single producer of the checked-in support export and
//! the scenario fixtures. The headless emitter and the inline tests both call
//! them so the in-code controls, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical relation-strip / sync-pending-pill packet.
pub const RELATION_STRIP_SYNC_PENDING_PACKET_ID: &str =
    "m5-relation-strip-sync-pending-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn strip_source_refs() -> Vec<String> {
    strings(&[
        M5_RELATION_STRIP_SCHEMA_REF,
        M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
    ])
}

fn pill_source_refs() -> Vec<String> {
    strings(&[
        M5_SYNC_PENDING_PILL_SCHEMA_REF,
        M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
    ])
}

/// Builds a relation entry, deriving the health class and the required note from the
/// honest reachability / freshness inputs so the seed is always self-consistent.
fn relation_entry(
    kind: M5WorkItemRelationKind,
    reference_label: &str,
    is_target_reachable: bool,
    is_reference_current: bool,
) -> RelationEntry {
    let disclosure = resolve_relation_health(kind, is_target_reachable, is_reference_current);
    RelationEntry {
        relation_kind: kind,
        reference_label: reference_label.to_owned(),
        is_target_reachable,
        is_reference_current,
        health_class: disclosure.health_class,
        relation_note: if disclosure.needs_relation_note {
            format!("Relation is {}; verify before relying on it", disclosure.health_class.as_str())
        } else {
            String::new()
        },
        actions: RelationStripAction::ALL.to_vec(),
    }
}

fn relation_strip(strip_id: &str, canonical_id: &str, relations: Vec<RelationEntry>) -> RelationStrip {
    RelationStrip {
        component: M5WorkItemComponentFamily::RelationStrip,
        strip_id: strip_id.to_owned(),
        canonical_id: canonical_id.to_owned(),
        relations,
        collapses_into_generic_linked_label: false,
        surface_families: M5WorkItemSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5WorkItemDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5WorkItemAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5WorkItemConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&["relation_kind", "reference", "health", "actions"]),
        source_contract_refs: strip_source_refs(),
        uses_generic_ticket_wording: false,
    }
}

/// Builds a sync-pending pill, deriving the recovery class, the provider-confirmed
/// claim, the required notes, and the recovery actions from the honest inputs so the
/// seed is always self-consistent with the resolver.
fn sync_pending_pill(
    pill_id: &str,
    canonical_id: &str,
    change_type: PendingChangeType,
    change_label: &str,
    local_state: M5WorkItemLocalState,
    is_policy_blocked: bool,
    is_provider_offline: bool,
) -> SyncPendingPill {
    let disclosure = resolve_sync_recovery(local_state, is_policy_blocked, is_provider_offline);
    let recovery_actions = if disclosure.needs_recovery_action {
        vec![
            SyncPillAction::RetryPublish,
            SyncPillAction::ExportPacket,
            SyncPillAction::OpenInProvider,
        ]
    } else if disclosure.needs_policy_block_note {
        vec![SyncPillAction::OpenInProvider, SyncPillAction::ExportPacket]
    } else {
        vec![SyncPillAction::OpenInProvider]
    };
    SyncPendingPill {
        component: M5WorkItemComponentFamily::SyncPendingPill,
        pill_id: pill_id.to_owned(),
        canonical_id: canonical_id.to_owned(),
        pending_change_type: change_type,
        pending_change_label: change_label.to_owned(),
        local_state,
        is_policy_blocked,
        is_provider_offline,
        recovery_class: disclosure.recovery_class,
        claims_provider_confirmed: disclosure.is_provider_confirmed,
        distinct_from_confirmed_style: disclosure.needs_distinct_style,
        last_sync_attempt_label: if disclosure.needs_last_sync_attempt {
            "Last sync attempt: 2026-07-08T22:14:00Z".to_owned()
        } else {
            String::new()
        },
        recovery_actions,
        policy_block_note: if disclosure.needs_policy_block_note {
            "Publish is blocked by policy; export the packet or open in the provider".to_owned()
        } else {
            String::new()
        },
        surface_families: M5WorkItemSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5WorkItemDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5WorkItemAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5WorkItemConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "pending_change",
            "local_state",
            "last_sync_attempt",
            "recovery_actions",
        ]),
        source_contract_refs: pill_source_refs(),
        uses_generic_ticket_wording: false,
    }
}

fn relation_strips() -> Vec<RelationStrip> {
    use M5WorkItemRelationKind as Kind;

    vec![
        // Strip 1: one work item whose linked contexts span every health class — each
        // context is named distinctly, never collapsed into a vague `Linked` label.
        relation_strip(
            "strip-checkout-rounding",
            "PROJ-1421",
            vec![
                // Current linked branch.
                relation_entry(
                    Kind::LinkedBranch,
                    "feature/checkout-rounding (3 commits ahead)",
                    true,
                    true,
                ),
                // Stale hosted review — reachable but out of date.
                relation_entry(
                    Kind::LinkedReview,
                    "review #482 (2 unresolved threads)",
                    true,
                    false,
                ),
                // Broken failing-test link — target no longer resolves.
                relation_entry(
                    Kind::LinkedTestRun,
                    "ci run 9921 checkout-suite",
                    false,
                    false,
                ),
                // Unmapped / dangling relation imported from an external tracker.
                relation_entry(
                    Kind::UnmappedRelation,
                    "imported link (unresolved target)",
                    false,
                    false,
                ),
            ],
        ),
        // Strip 2: a live incident with a linked incident bridge and a hotfix PR — two
        // distinct current contexts.
        relation_strip(
            "strip-failover-incident",
            "INC-3390",
            vec![
                relation_entry(Kind::LinkedIncident, "incident bridge #77", true, true),
                relation_entry(
                    Kind::LinkedPullRequest,
                    "PR #1290 failover hotfix",
                    true,
                    true,
                ),
            ],
        ),
    ]
}

fn sync_pending_pills() -> Vec<SyncPendingPill> {
    use M5WorkItemLocalState as Local;
    use PendingChangeType as Change;

    vec![
        // 1. Provider-confirmed: a comment that reconciled with the provider — the only
        //    state that may read as confirmed.
        sync_pending_pill(
            "pill-confirmed-comment",
            "PROJ-1421",
            Change::PendingComment,
            "Comment published and confirmed by the provider",
            Local::SyncedWithProvider,
            false,
            false,
        ),
        // 2. Pending-publish: a status transition queued locally, not yet published.
        sync_pending_pill(
            "pill-pending-transition",
            "PROJ-1421",
            Change::PendingTransition,
            "Transition to In Review queued for publish",
            Local::QueuedForPublish,
            false,
            false,
        ),
        // 3. Recoverable failure: a link change whose publish failed — retry or export.
        sync_pending_pill(
            "pill-failed-link",
            "PROJ-1466",
            Change::PendingLink,
            "Link to branch failed to publish",
            Local::PublishFailed,
            false,
            false,
        ),
        // 4. Offline-held: a field edit held locally while the provider is offline.
        sync_pending_pill(
            "pill-offline-field-edit",
            "LOCAL-0007",
            Change::PendingFieldEdit,
            "Assignee edit held while the provider is offline",
            Local::LocalOnlyDraft,
            false,
            true,
        ),
        // 5. Policy-blocked: a create blocked by policy — never presents as confirmed.
        sync_pending_pill(
            "pill-policy-blocked-create",
            "INC-3390",
            Change::PendingCreate,
            "Create of a linked incident record blocked by policy",
            Local::ConflictHeld,
            true,
            false,
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5WorkItemDowngradeTrigger> {
    vec![
        M5WorkItemDowngradeTrigger::LinkedContextUnstated,
        M5WorkItemDowngradeTrigger::LocalVersusProviderStateHidden,
        M5WorkItemDowngradeTrigger::SyncPendingStateHidden,
        M5WorkItemDowngradeTrigger::PublishLaterContinuityHidden,
        M5WorkItemDowngradeTrigger::GenericTicketWordingUsed,
        M5WorkItemDowngradeTrigger::ProofStale,
    ]
}

fn trust_review() -> RelationStripSyncPendingTrustReview {
    RelationStripSyncPendingTrustReview {
        relation_strip_names_each_linked_context: true,
        stale_and_broken_relations_labeled: true,
        relation_actions_metadata_safe_copy_open: true,
        sync_pending_visibly_distinct_from_confirmed: true,
        sync_pending_discloses_change_type: true,
        last_sync_attempt_shown_when_pending: true,
        retry_or_export_recovery_always_available: true,
        policy_blocked_state_always_explicit: true,
        no_generic_ticket_wording_conceals_context: true,
        controls_stable_across_deployment_lines: true,
        copy_and_export_safe: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> RelationStripSyncPendingConsumerProjection {
    RelationStripSyncPendingConsumerProjection {
        side_rail_relation_strips_name_each_context: true,
        list_and_rail_distinguish_pending_from_confirmed: true,
        retry_and_export_reachable_headless: true,
        cli_headless_shows_control_truth: true,
        support_export_shows_control_truth: true,
        help_about_shows_control_truth: true,
    }
}

fn proof_freshness() -> RelationStripSyncPendingProofFreshness {
    RelationStripSyncPendingProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        RELATION_STRIP_SYNC_PENDING_SCHEMA_REF,
        RELATION_STRIP_SYNC_PENDING_DOC_REF,
        M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
        M5_WORK_ITEM_COMPONENT_DOC_REF,
        M5_RELATION_STRIP_SCHEMA_REF,
        M5_SYNC_PENDING_PILL_SCHEMA_REF,
    ])
}

/// Builds the canonical relation-strip / sync-pending-pill controls packet.
pub fn seeded_relation_strip_sync_pending_controls() -> RelationStripSyncPendingControlsPacket {
    RelationStripSyncPendingControlsPacket::new(RelationStripSyncPendingControlsPacketInput {
        packet_id: RELATION_STRIP_SYNC_PENDING_PACKET_ID.to_owned(),
        surface_label:
            "M5 relation strips and sync-pending pills: linked branch/review/test/incident context with derived stale/broken relation labeling and metadata-safe copy/open actions, plus pending comment/transition/link/field-edit/create pills that read visibly differently from provider-confirmed state and stay recoverable via retry or export when publish fails or the provider is offline"
                .to_owned(),
        relation_strips: relation_strips(),
        sync_pending_pills: sync_pending_pills(),
        downgrade_triggers: downgrade_triggers(),
        consumer_surfaces: M5WorkItemConsumerSurface::ALL.to_vec(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Scenario fixture: spotlights a relation strip whose linked contexts span current,
/// stale, broken, and unmapped health, each labeled distinctly instead of collapsed
/// into a vague `Linked`. Every health class and every sync-recovery class stays
/// covered so the fixture validates on its own.
pub fn seeded_relation_strip_sync_pending_controls_relation_strip_stale_relation(
) -> RelationStripSyncPendingControlsPacket {
    let mut packet = seeded_relation_strip_sync_pending_controls();
    packet.packet_id =
        "m5-relation-strip-sync-pending-controls:fixture:relation-strip-stale-relation".to_owned();
    packet.surface_label =
        "M5 relation strips: stale and broken linked contexts are labeled, never collapsed into a vague 'Linked'"
            .to_owned();
    packet
}

/// Scenario fixture: spotlights a sync-pending pill whose publish failed and stays
/// recoverable via retry or export, and which never reads as provider-confirmed.
/// Every health class and every sync-recovery class stays covered so the fixture
/// validates on its own.
pub fn seeded_relation_strip_sync_pending_controls_sync_pending_recoverable_failure(
) -> RelationStripSyncPendingControlsPacket {
    let mut packet = seeded_relation_strip_sync_pending_controls();
    packet.packet_id =
        "m5-relation-strip-sync-pending-controls:fixture:sync-pending-recoverable-failure"
            .to_owned();
    packet.surface_label =
        "M5 sync-pending pills: a failed publish stays recoverable and never reads as provider-confirmed"
            .to_owned();
    packet
}
