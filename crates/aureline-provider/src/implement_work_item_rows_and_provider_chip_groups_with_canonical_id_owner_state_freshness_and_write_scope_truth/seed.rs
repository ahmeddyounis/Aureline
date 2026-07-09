//! Canonical seed builders for the work-item-row / provider-chip-group controls.
//!
//! These builders are the single producer of the checked-in support export and
//! the scenario fixtures. The headless emitter and the inline tests both call
//! them so the in-code controls, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical work-item-row / provider-chip-group packet.
pub const WORK_ITEM_ROW_PROVIDER_CHIP_PACKET_ID: &str =
    "m5-work-item-row-provider-chip-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn row_source_refs() -> Vec<String> {
    strings(&[
        M5_WORK_ITEM_ROW_SCHEMA_REF,
        M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
    ])
}

fn chip_source_refs() -> Vec<String> {
    strings(&[
        M5_PROVIDER_CHIP_GROUP_SCHEMA_REF,
        M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
    ])
}

/// Builds a work-item row, deriving the state-authority class, the
/// provider-authoritative claim, and the required notes from the honest inputs so
/// the seed is always self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn work_item_row(
    row_id: &str,
    canonical_id: &str,
    title: &str,
    kind: M5WorkItemKind,
    authority: M5WorkItemProviderAuthority,
    local_state: M5WorkItemLocalState,
    owner_label: &str,
    priority_class: WorkItemPriorityClass,
    uses_severity_scale: bool,
    priority_label: &str,
    linked_change_count: u32,
) -> WorkItemRow {
    let disclosure = resolve_work_item_state_authority(authority, local_state);
    WorkItemRow {
        component: M5WorkItemComponentFamily::WorkItemRow,
        row_id: row_id.to_owned(),
        canonical_id: canonical_id.to_owned(),
        canonical_id_copyable: true,
        title: title.to_owned(),
        work_item_kind: kind,
        provider_authority: authority,
        local_state,
        owner_label: owner_label.to_owned(),
        priority_class,
        uses_severity_scale,
        priority_label: priority_label.to_owned(),
        linked_change_count,
        linked_change_label: if linked_change_count > 0 {
            format!("{linked_change_count} linked changes (branch, review, test)")
        } else {
            String::new()
        },
        state_authority_class: disclosure.authority_class,
        claims_provider_authoritative: disclosure.is_provider_authoritative,
        local_state_note: if disclosure.needs_local_state_note {
            format!("Local-versus-provider state: {}", local_state.as_str())
        } else {
            String::new()
        },
        publish_pending_note: if disclosure.needs_publish_pending_note {
            "Changes are held locally and not yet published to the provider".to_owned()
        } else {
            String::new()
        },
        blocked_capability_note: if disclosure.needs_blocked_note {
            "Capability is blocked by policy; this item cannot be written".to_owned()
        } else {
            String::new()
        },
        default_actions: WorkItemRowAction::ALL.to_vec(),
        surface_families: M5WorkItemSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5WorkItemDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5WorkItemAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5WorkItemConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "canonical_id",
            "title",
            "state",
            "owner",
            "priority",
            "linked_changes",
        ]),
        source_contract_refs: row_source_refs(),
        uses_generic_ticket_wording: false,
    }
}

/// Builds a provider chip group, deriving writability and the required posture
/// notes from the honest inputs so the seed is always self-consistent.
fn provider_chip_group(
    group_id: &str,
    provider_label: &str,
    project_or_space_label: &str,
    authority: M5WorkItemProviderAuthority,
    has_tenant_scope: bool,
    tenant_label: &str,
    write_posture: ProviderChipWritePosture,
) -> ProviderChipGroup {
    let disclosure = resolve_provider_chip_group_disclosure(authority, write_posture);
    ProviderChipGroup {
        component: M5WorkItemComponentFamily::ProviderChipGroup,
        group_id: group_id.to_owned(),
        provider_label: provider_label.to_owned(),
        project_or_space_label: project_or_space_label.to_owned(),
        provider_authority: authority,
        has_tenant_scope,
        tenant_scope_note: if has_tenant_scope {
            tenant_label.to_owned()
        } else {
            String::new()
        },
        write_posture,
        is_writable: disclosure.is_writable,
        read_only_note: if disclosure.needs_read_only_note {
            "Read-only mirror; no write back to the provider".to_owned()
        } else {
            String::new()
        },
        offline_capture_note: if disclosure.needs_offline_capture_note {
            "Captured locally as an offline draft; not yet published".to_owned()
        } else {
            String::new()
        },
        policy_block_note: if disclosure.needs_policy_block_note {
            "Binding is blocked by policy; no provider write is possible".to_owned()
        } else {
            String::new()
        },
        surface_families: M5WorkItemSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5WorkItemDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5WorkItemAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5WorkItemConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "provider",
            "project_or_space",
            "tenant_scope",
            "write_posture",
        ]),
        source_contract_refs: chip_source_refs(),
        uses_generic_ticket_wording: false,
    }
}

fn work_item_rows() -> Vec<WorkItemRow> {
    use M5WorkItemKind as Kind;
    use M5WorkItemLocalState as Local;
    use M5WorkItemProviderAuthority as Authority;
    use WorkItemPriorityClass as Priority;

    vec![
        // 1. Provider-authoritative: a provider-owned, provider-synced issue — the
        //    highest-trust list row a user can act on directly.
        work_item_row(
            "row-provider-authoritative",
            "PROJ-1421",
            "Checkout total rounds incorrectly for multi-currency carts",
            Kind::Issue,
            Authority::ProviderOwned,
            Local::SyncedWithProvider,
            "Priya Nair",
            Priority::High,
            false,
            "P1 (high)",
            3,
        ),
        // 2. Local-only draft: a local task not yet owned by any provider — never
        //    reads as provider-authoritative in the list.
        work_item_row(
            "row-local-only-draft",
            "LOCAL-0007",
            "Draft: split flaky checkout test into deterministic cases",
            Kind::Task,
            Authority::LocalDraft,
            Local::LocalOnlyDraft,
            "Sam Ortega",
            Priority::Medium,
            false,
            "P2 (medium)",
            0,
        ),
        // 3. Publish-pending: a provider-owned incident whose status change is queued
        //    locally and not yet published — severity scale, not priority.
        work_item_row(
            "row-publish-pending",
            "INC-3390",
            "Payment webhook backlog during regional failover",
            Kind::Incident,
            Authority::ProviderOwned,
            Local::QueuedForPublish,
            "On-call: Dana Wu",
            Priority::Critical,
            true,
            "SEV1 (critical)",
            2,
        ),
        // 4. Snapshot-only: a mirrored read-only change request detached from live
        //    provider truth.
        work_item_row(
            "row-snapshot-only",
            "CHG-2048",
            "Roll forward feature-flag defaults for onboarding",
            Kind::ChangeRequest,
            Authority::MirroredReadOnly,
            Local::SyncedWithProvider,
            "Release desk",
            Priority::Low,
            false,
            "P3 (low)",
            1,
        ),
        // 5. Blocked capability: a policy-pinned issue that cannot be written.
        work_item_row(
            "row-blocked-capability",
            "PROJ-1500",
            "Rotate signing material for the release pipeline",
            Kind::Issue,
            Authority::PolicyPinned,
            Local::SyncedWithProvider,
            "Security review",
            Priority::Medium,
            false,
            "P2 (medium)",
            0,
        ),
        // 6. Publish-pending via a held conflict — proves conflict-held also reads as
        //    publish-pending and never as reconciled.
        work_item_row(
            "row-conflict-held",
            "PROJ-1466",
            "Reconcile assignee changed both locally and upstream",
            Kind::Task,
            Authority::ProviderOwned,
            Local::ConflictHeld,
            "Priya Nair",
            Priority::High,
            false,
            "P1 (high)",
            1,
        ),
    ]
}

fn provider_chip_groups() -> Vec<ProviderChipGroup> {
    use M5WorkItemProviderAuthority as Authority;
    use ProviderChipWritePosture as Posture;

    vec![
        // 1. Read-only mirror in an org-scoped project.
        provider_chip_group(
            "chip-read-only",
            "GitHub Issues",
            "acme-eng / platform board",
            Authority::MirroredReadOnly,
            true,
            "Tenant: acme-eng organization",
            Posture::ReadOnly,
        ),
        // 2. Comment-link connection on a provider-owned project.
        provider_chip_group(
            "chip-comment-link",
            "Jira",
            "PLAT project",
            Authority::ProviderOwned,
            true,
            "Tenant: acme-eng cloud site",
            Posture::CommentLink,
        ),
        // 3. Full-edit connection on a provider-owned space.
        provider_chip_group(
            "chip-full-edit",
            "Linear",
            "Platform team space",
            Authority::ProviderOwned,
            false,
            "",
            Posture::FullEdit,
        ),
        // 4. Offline-capture on a local draft — captured locally, not published.
        provider_chip_group(
            "chip-offline-capture",
            "Local capture",
            "Unsynced drafts space",
            Authority::LocalDraft,
            false,
            "",
            Posture::OfflineCapture,
        ),
        // 5. Policy-blocked binding on a policy-pinned project.
        provider_chip_group(
            "chip-policy-blocked",
            "ServiceNow",
            "Security incidents queue",
            Authority::PolicyPinned,
            true,
            "Tenant: acme-eng restricted",
            Posture::PolicyBlocked,
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5WorkItemDowngradeTrigger> {
    vec![
        M5WorkItemDowngradeTrigger::ProviderAuthorityUnstated,
        M5WorkItemDowngradeTrigger::LocalVersusProviderStateHidden,
        M5WorkItemDowngradeTrigger::AlternateStateLabelInvented,
        M5WorkItemDowngradeTrigger::GenericTicketWordingUsed,
        M5WorkItemDowngradeTrigger::ProofStale,
    ]
}

fn trust_review() -> WorkItemRowProviderChipTrustReview {
    WorkItemRowProviderChipTrustReview {
        canonical_id_always_visible_and_copyable: true,
        work_item_state_shows_provider_authority: true,
        local_or_blocked_never_reads_as_provider_authoritative: true,
        blocked_capability_always_explicit: true,
        linked_change_count_always_shown: true,
        keyboard_complete_default_actions: true,
        provider_chip_shows_project_or_space_scope: true,
        provider_chip_shows_write_posture: true,
        tenant_org_cue_shown_when_relevant: true,
        offline_capture_and_policy_block_explicit: true,
        no_generic_ticket_wording_conceals_ownership: true,
        controls_stable_across_deployment_lines: true,
        copy_and_export_safe: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> WorkItemRowProviderChipConsumerProjection {
    WorkItemRowProviderChipConsumerProjection {
        list_rows_distinguish_authority_without_inspector: true,
        canonical_id_copyable_everywhere: true,
        chip_group_shows_scope_and_posture_inline: true,
        cli_headless_shows_control_truth: true,
        support_export_shows_control_truth: true,
        help_about_shows_control_truth: true,
    }
}

fn proof_freshness() -> WorkItemRowProviderChipProofFreshness {
    WorkItemRowProviderChipProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        WORK_ITEM_ROW_PROVIDER_CHIP_SCHEMA_REF,
        WORK_ITEM_ROW_PROVIDER_CHIP_DOC_REF,
        M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
        M5_WORK_ITEM_COMPONENT_DOC_REF,
        M5_WORK_ITEM_ROW_SCHEMA_REF,
        M5_PROVIDER_CHIP_GROUP_SCHEMA_REF,
    ])
}

/// Builds the canonical work-item-row / provider-chip-group controls packet.
pub fn seeded_work_item_row_provider_chip_controls() -> WorkItemRowProviderChipControlsPacket {
    WorkItemRowProviderChipControlsPacket::new(WorkItemRowProviderChipControlsPacketInput {
        packet_id: WORK_ITEM_ROW_PROVIDER_CHIP_PACKET_ID.to_owned(),
        surface_label:
            "M5 work-item rows and provider chip groups: canonical id, title, state, owner, priority/severity, linked-change count, keyboard-complete default actions, provider/project-or-space scope, tenant/org cue, and explicit read-only/comment-link/full-edit/offline-capture/policy-blocked write posture"
                .to_owned(),
        work_item_rows: work_item_rows(),
        provider_chip_groups: provider_chip_groups(),
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

/// Scenario fixture: spotlights a local-only work-item draft that must never read
/// as provider-authoritative in a list surface. Every state-authority class stays
/// covered so the fixture validates on its own.
pub fn seeded_work_item_row_provider_chip_controls_work_item_row_local_only(
) -> WorkItemRowProviderChipControlsPacket {
    let mut packet = seeded_work_item_row_provider_chip_controls();
    packet.packet_id =
        "m5-work-item-row-provider-chip-controls:fixture:work-item-row-local-only".to_owned();
    packet.surface_label =
        "M5 work-item rows: a local-only draft never reads as provider-authoritative".to_owned();
    packet
}

/// Scenario fixture: spotlights an offline-capture provider chip that must never
/// present as a live full-edit connection. Every write posture stays covered so
/// the fixture validates on its own.
pub fn seeded_work_item_row_provider_chip_controls_provider_chip_offline_capture(
) -> WorkItemRowProviderChipControlsPacket {
    let mut packet = seeded_work_item_row_provider_chip_controls();
    packet.packet_id =
        "m5-work-item-row-provider-chip-controls:fixture:provider-chip-offline-capture".to_owned();
    packet.surface_label =
        "M5 provider chip groups: an offline-capture chip never presents as live full-edit"
            .to_owned();
    packet
}
