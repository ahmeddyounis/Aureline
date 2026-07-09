//! Canonical seed builders for the detail-header / status-transition-sheet controls.
//!
//! These builders are the single producer of the checked-in support export and the
//! scenario fixtures. The headless emitter and the inline tests both call them so the
//! in-code controls, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical detail-header / status-transition-sheet packet.
pub const DETAIL_HEADER_TRANSITION_PACKET_ID: &str =
    "m5-work-item-detail-header-status-transition-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn header_source_refs() -> Vec<String> {
    strings(&[
        M5_WORK_ITEM_DETAIL_HEADER_SCHEMA_REF,
        M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
    ])
}

fn sheet_source_refs() -> Vec<String> {
    strings(&[
        M5_STATUS_TRANSITION_SHEET_SCHEMA_REF,
        M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
    ])
}

/// Builds a detail header, deriving the write scope, freshness class, the
/// provider-backed claim, and the required notes from the honest boundary inputs so the
/// seed is always self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn detail_header(
    header_id: &str,
    canonical_id: &str,
    provider_space_label: &str,
    title: &str,
    kind: M5WorkItemKind,
    state_label: &str,
    owner_label: &str,
    provider_authority: M5WorkItemProviderAuthority,
    local_state: M5WorkItemLocalState,
    is_reference_current: bool,
    is_freshness_known: bool,
) -> DetailHeader {
    let disclosure = resolve_detail_header(
        provider_authority,
        local_state,
        is_reference_current,
        is_freshness_known,
    );
    DetailHeader {
        component: M5WorkItemComponentFamily::WorkItemDetailHeader,
        header_id: header_id.to_owned(),
        canonical_id: canonical_id.to_owned(),
        provider_space_label: provider_space_label.to_owned(),
        title: title.to_owned(),
        work_item_kind: kind,
        state_label: state_label.to_owned(),
        owner_label: owner_label.to_owned(),
        provider_authority,
        local_state,
        is_reference_current,
        is_freshness_known,
        write_scope: disclosure.write_scope,
        freshness_class: disclosure.freshness_class,
        claims_provider_backed: disclosure.is_provider_backed,
        write_scope_note: if disclosure.needs_write_scope_note {
            format!(
                "Write scope is {}; changes stay local until they can flow to the provider",
                disclosure.write_scope.as_str()
            )
        } else {
            String::new()
        },
        freshness_note: if disclosure.needs_freshness_note {
            format!(
                "Header freshness is {}; re-sync before relying on this state",
                disclosure.freshness_class.as_str()
            )
        } else {
            String::new()
        },
        policy_block_note: if disclosure.needs_policy_note {
            "Writes are blocked by policy; open in the provider or export the header".to_owned()
        } else {
            String::new()
        },
        actions: DetailHeaderAction::ALL.to_vec(),
        surface_families: M5WorkItemSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5WorkItemDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5WorkItemAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5WorkItemConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "provider_space",
            "canonical_id",
            "title",
            "state",
            "owner",
            "freshness",
            "write_scope",
        ]),
        source_contract_refs: header_source_refs(),
        uses_generic_ticket_wording: false,
    }
}

/// Builds a status-transition sheet, deriving the publish class, the external-mutation
/// claim, and the required notes from the honest inputs so the seed is always
/// self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn transition_sheet(
    sheet_id: &str,
    canonical_id: &str,
    from_status: &str,
    to_status: &str,
    mutation_kind: TransitionMutationKind,
    primary_transition_effect: M5WorkItemTransitionEffect,
    local_state: M5WorkItemLocalState,
    is_policy_blocked: bool,
    permission_scope: PermissionScopeClass,
    side_effect_preview_label: &str,
    permission_scope_note: &str,
) -> StatusTransitionSheet {
    let disclosure = resolve_transition_publish(primary_transition_effect, is_policy_blocked);
    let actions = if disclosure.is_blocked {
        vec![
            TransitionSheetAction::Confirm,
            TransitionSheetAction::ExportPacket,
            TransitionSheetAction::Cancel,
            TransitionSheetAction::OpenInProvider,
        ]
    } else {
        vec![
            TransitionSheetAction::Confirm,
            TransitionSheetAction::ExportPacket,
            TransitionSheetAction::Cancel,
            TransitionSheetAction::SaveDraft,
        ]
    };
    StatusTransitionSheet {
        component: M5WorkItemComponentFamily::StatusTransitionSheet,
        sheet_id: sheet_id.to_owned(),
        canonical_id: canonical_id.to_owned(),
        from_status: from_status.to_owned(),
        to_status: to_status.to_owned(),
        mutation_kinds: vec![mutation_kind],
        transition_effects: vec![primary_transition_effect],
        primary_transition_effect,
        local_state,
        is_policy_blocked,
        publish_class: disclosure.publish_class,
        implies_external_mutation: disclosure.publishes_externally,
        side_effect_preview_label: side_effect_preview_label.to_owned(),
        linked_context_note: "Linked branch feature/checkout-rounding and review 482 stay attached"
            .to_owned(),
        notification_side_effect_note: if disclosure.needs_notification_note {
            "Publishing notifies watchers and the assignee in the provider".to_owned()
        } else {
            String::new()
        },
        permission_scope,
        permission_scope_note: permission_scope_note.to_owned(),
        actions,
        export_fallback_note:
            "If publish cannot proceed, export a metadata-safe packet or save a local draft"
                .to_owned(),
        policy_block_note: if disclosure.needs_policy_note {
            "This transition is blocked by policy; export the packet for review".to_owned()
        } else {
            String::new()
        },
        surface_families: M5WorkItemSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5WorkItemDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5WorkItemAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5WorkItemConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "from_status",
            "to_status",
            "mutations",
            "linked_context",
            "notifications",
            "permission_scope",
            "confirm_export_cancel",
        ]),
        source_contract_refs: sheet_source_refs(),
        uses_generic_ticket_wording: false,
    }
}

fn detail_headers() -> Vec<DetailHeader> {
    use M5WorkItemKind as Kind;
    use M5WorkItemLocalState as Local;
    use M5WorkItemProviderAuthority as Authority;

    vec![
        // 1. Provider-owned and reconciled: provider-writable, live-synced.
        detail_header(
            "header-checkout-rounding",
            "PROJ-1421",
            "acme / checkout board",
            "Rounding error at checkout total",
            Kind::Issue,
            "In Progress",
            "j.rivera (assignee)",
            Authority::ProviderOwned,
            Local::SyncedWithProvider,
            true,
            true,
        ),
        // 2. Local draft: local-draft-only write scope, local-only freshness, not
        //    provider-backed — never reads as a provider object.
        detail_header(
            "header-local-triage-note",
            "LOCAL-0007",
            "local drafts (unpublished)",
            "Triage note: intermittent 500 on retry",
            Kind::Task,
            "Draft",
            "unassigned (local draft)",
            Authority::LocalDraft,
            Local::LocalOnlyDraft,
            false,
            true,
        ),
        // 3. Imported snapshot: read-only mirror, stale snapshot.
        detail_header(
            "header-imported-change-request",
            "EXT-5521",
            "imported tracker (snapshot)",
            "Change request: rotate signing keys",
            Kind::ChangeRequest,
            "Approved (snapshot)",
            "external owner (imported)",
            Authority::ImportedSnapshot,
            Local::SyncedWithProvider,
            true,
            true,
        ),
        // 4. Policy-pinned incident: writes policy-blocked, live-synced.
        detail_header(
            "header-failover-incident",
            "INC-3390",
            "acme / incidents",
            "Failover did not promote replica",
            Kind::Incident,
            "Investigating",
            "on-call (incident commander)",
            Authority::PolicyPinned,
            Local::SyncedWithProvider,
            true,
            true,
        ),
        // 5. Mirror whose freshness cannot be determined: read-only mirror, unknown.
        detail_header(
            "header-mirror-unknown-freshness",
            "MIR-8830",
            "mirror / read-only",
            "Epic: unify provider sync surfaces",
            Kind::Epic,
            "Open",
            "team lead (mirror)",
            Authority::MirroredReadOnly,
            Local::SyncedWithProvider,
            true,
            false,
        ),
    ]
}

fn status_transition_sheets() -> Vec<StatusTransitionSheet> {
    use M5WorkItemLocalState as Local;
    use M5WorkItemTransitionEffect as Effect;
    use PermissionScopeClass as Scope;
    use TransitionMutationKind as Mutation;

    vec![
        // 1. Local-only state change: nothing publishes, current user authorized.
        transition_sheet(
            "sheet-local-triage-state",
            "LOCAL-0007",
            "Draft",
            "Triaging",
            Mutation::StateMutation,
            Effect::LocalOnlyTransition,
            Local::LocalOnlyDraft,
            false,
            Scope::CurrentUserAuthorized,
            "Sets local state to Triaging; no provider write and no notifications",
            "You can make this local change; nothing is published",
        ),
        // 2. Comment that publishes to the provider: needs a re-authenticated grant.
        transition_sheet(
            "sheet-publish-comment",
            "PROJ-1421",
            "In Progress",
            "In Review",
            Mutation::CommentMutation,
            Effect::CommentSideEffect,
            Local::SyncedWithProvider,
            false,
            Scope::NeedsProviderAuth,
            "Posts a comment and moves the item to In Review in the provider",
            "Publishing needs a current provider sign-in with comment scope",
        ),
        // 3. Link change that opens in the provider: needs an elevated role.
        transition_sheet(
            "sheet-open-in-provider-link",
            "PROJ-1421",
            "In Review",
            "Linked",
            Mutation::LinkMutation,
            Effect::OpenInProvider,
            Local::SyncedWithProvider,
            false,
            Scope::NeedsElevatedRole,
            "Opens the item in the provider to attach the linked pull request",
            "Linking needs a maintainer role on the target repository",
        ),
        // 4. Assignment change that is blocked and needs permission.
        transition_sheet(
            "sheet-blocked-assignment",
            "INC-3390",
            "Investigating",
            "Assigned",
            Mutation::AssignmentMutation,
            Effect::BlockedTransition,
            Local::ConflictHeld,
            false,
            Scope::NeedsElevatedRole,
            "Would reassign the incident; blocked until a conflict is resolved",
            "Reassignment needs an incident-commander role to proceed",
        ),
        // 5. Field change blocked by policy: policy-held, policy-restricted scope.
        transition_sheet(
            "sheet-policy-blocked-field",
            "INC-3390",
            "Assigned",
            "Escalated",
            Mutation::FieldMutation,
            Effect::PublishNowTransition,
            Local::QueuedForPublish,
            true,
            Scope::PolicyRestricted,
            "Would set severity to SEV-1; blocked by escalation policy",
            "Escalation is restricted by policy regardless of role",
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5WorkItemDowngradeTrigger> {
    vec![
        M5WorkItemDowngradeTrigger::IdentityUnstated,
        M5WorkItemDowngradeTrigger::ProviderAuthorityUnstated,
        M5WorkItemDowngradeTrigger::LocalVersusProviderStateHidden,
        M5WorkItemDowngradeTrigger::SideEffectPreviewHidden,
        M5WorkItemDowngradeTrigger::PublishLaterContinuityHidden,
        M5WorkItemDowngradeTrigger::GenericTicketWordingUsed,
        M5WorkItemDowngradeTrigger::ProofStale,
    ]
}

fn trust_review() -> DetailHeaderTransitionTrustReview {
    DetailHeaderTransitionTrustReview {
        header_states_identity_and_owner: true,
        header_write_scope_derived: true,
        header_freshness_derived: true,
        local_draft_never_reads_provider_backed: true,
        header_offers_open_external_escape_hatch: true,
        transition_previews_mutations_before_publish: true,
        local_transition_never_implies_external_mutation: true,
        transition_discloses_notification_side_effects: true,
        transition_names_permission_scope: true,
        transition_offers_confirm_export_cancel: true,
        export_fallback_always_available: true,
        no_generic_ticket_wording_conceals_truth: true,
        controls_stable_across_deployment_lines: true,
        copy_and_export_safe: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> DetailHeaderTransitionConsumerProjection {
    DetailHeaderTransitionConsumerProjection {
        detail_surface_renders_header_boundary: true,
        transition_surface_previews_before_publish: true,
        confirm_export_cancel_reachable_headless: true,
        cli_headless_shows_control_truth: true,
        support_export_shows_control_truth: true,
        help_about_shows_control_truth: true,
    }
}

fn proof_freshness() -> DetailHeaderTransitionProofFreshness {
    DetailHeaderTransitionProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        DETAIL_HEADER_TRANSITION_SCHEMA_REF,
        DETAIL_HEADER_TRANSITION_DOC_REF,
        M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
        M5_WORK_ITEM_COMPONENT_DOC_REF,
        M5_WORK_ITEM_DETAIL_HEADER_SCHEMA_REF,
        M5_STATUS_TRANSITION_SHEET_SCHEMA_REF,
    ])
}

/// Builds the canonical detail-header / status-transition-sheet controls packet.
pub fn seeded_detail_header_transition_controls() -> DetailHeaderTransitionControlsPacket {
    DetailHeaderTransitionControlsPacket::new(DetailHeaderTransitionControlsPacketInput {
        packet_id: DETAIL_HEADER_TRANSITION_PACKET_ID.to_owned(),
        surface_label:
            "M5 work-item detail headers and status-transition sheets: durable headers state provider space, canonical id, title, state, owner, derived write scope and freshness, and an open-external escape hatch, so a local draft never reads as a provider-backed object; transition sheets preview comment/state/assignment/link/field mutations, linked branch/review context, notification side effects, and the permission scope that can authorize the change, with confirm/export/cancel behavior and a metadata-safe export fallback before any publish"
                .to_owned(),
        detail_headers: detail_headers(),
        status_transition_sheets: status_transition_sheets(),
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

/// Scenario fixture: spotlights a local-draft detail header whose write scope is
/// local-only and whose freshness is local-only, so it never reads as a provider-backed
/// object. Every write scope and every freshness class stays covered so the fixture
/// validates on its own.
pub fn seeded_detail_header_transition_controls_detail_header_local_draft(
) -> DetailHeaderTransitionControlsPacket {
    let mut packet = seeded_detail_header_transition_controls();
    packet.packet_id =
        "m5-work-item-detail-header-status-transition-controls:fixture:detail-header-local-draft"
            .to_owned();
    packet.surface_label =
        "M5 detail headers: a local draft states local-only write scope and freshness, never reading as a provider-backed object"
            .to_owned();
    packet
}

/// Scenario fixture: spotlights a status-transition sheet that publishes to the
/// provider, previewing its side effects, notification, permission scope, and
/// confirm/export/cancel behavior before publish. Every publish class and every
/// mutation kind stays covered so the fixture validates on its own.
pub fn seeded_detail_header_transition_controls_status_transition_publish_now(
) -> DetailHeaderTransitionControlsPacket {
    let mut packet = seeded_detail_header_transition_controls();
    packet.packet_id =
        "m5-work-item-detail-header-status-transition-controls:fixture:status-transition-publish-now"
            .to_owned();
    packet.surface_label =
        "M5 status-transition sheets: a publishing transition previews side effects, permission scope, and confirm/export/cancel before any provider write"
            .to_owned();
    packet
}
