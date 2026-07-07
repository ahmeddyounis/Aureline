//! Canonical seed builders for the M5 provider mapping / sync-behavior row primitive.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix,
//! the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical mapping/sync-row primitive packet.
pub const M5_PROVIDER_MAPPING_SYNC_ROW_PACKET_ID: &str =
    "m5-provider-mapping-sync-behavior-row-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked mapping-row resolution case from a full mapping state.
fn mapping_case(
    target_kind: M5MappingTargetKind,
    mapping_origin: M5MappingOriginClass,
    provider_project_label: &str,
    repo_workspace_relation: &str,
    lock_note: Option<&str>,
    mapping_ref: &str,
) -> M5MappingRowResolutionCase {
    M5MappingRowResolutionCase::resolved(M5MappingRowResolutionInput {
        target_kind,
        mapping_origin,
        provider_project_label: provider_project_label.to_owned(),
        repo_workspace_relation: repo_workspace_relation.to_owned(),
        lock_note: lock_note.map(str::to_owned),
        mapping_ref: mapping_ref.to_owned(),
    })
}

/// Builds a worked sync-row resolution case from a full sync state.
fn sync_case(
    sync_mode: M5ProviderSyncMode,
    write_scope: M5ProviderWriteScope,
    queued_draft_state: M5QueuedDraftState,
    sync_label: &str,
    sync_ref: &str,
) -> M5SyncRowResolutionCase {
    M5SyncRowResolutionCase::resolved(M5SyncRowResolutionInput {
        sync_mode,
        write_scope,
        queued_draft_state,
        sync_label: sync_label.to_owned(),
        sync_ref: sync_ref.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full mapping-row and sync-row anatomy,
/// origin, target, scope, posture, action, mode, write-scope, behavior-class, queued-draft,
/// export-field, and accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5MappingSyncConsumerSurface,
    qualification: M5ProviderQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    mapping_examples: Vec<M5MappingRowResolutionCase>,
    sync_examples: Vec<M5SyncRowResolutionCase>,
) -> M5MappingSyncConsumerRow {
    M5MappingSyncConsumerRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5ProviderSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ProviderDeploymentLine::ALL.to_vec(),
        mapping_anatomy_parts: M5MappingRowAnatomyPart::ALL.to_vec(),
        sync_anatomy_parts: M5SyncRowAnatomyPart::ALL.to_vec(),
        mapping_target_kinds: M5MappingTargetKind::ALL.to_vec(),
        mapping_origins: M5MappingOriginClass::ALL.to_vec(),
        mapping_scopes: M5MappingScopeClass::ALL.to_vec(),
        mapping_row_postures: M5MappingRowPosture::ALL.to_vec(),
        mapping_row_actions: M5MappingRowAction::ALL.to_vec(),
        sync_modes: M5ProviderSyncMode::ALL.to_vec(),
        write_scopes: M5ProviderWriteScope::ALL.to_vec(),
        sync_behavior_classes: M5SyncBehaviorClass::ALL.to_vec(),
        queued_draft_states: M5QueuedDraftState::ALL.to_vec(),
        sync_row_actions: M5SyncRowAction::ALL.to_vec(),
        mapping_export_fields: M5MappingRowExportField::ALL.to_vec(),
        sync_export_fields: M5SyncRowExportField::ALL.to_vec(),
        accessibility_routes: M5ProviderAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5ProviderConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5ProviderDowngradeTrigger::MappingOriginUnstated,
            M5ProviderDowngradeTrigger::SyncModeUnstated,
            M5ProviderDowngradeTrigger::WriteScopeUnstated,
            M5ProviderDowngradeTrigger::QueuedDraftStateHidden,
            M5ProviderDowngradeTrigger::DefaultDestinationAssumed,
            M5ProviderDowngradeTrigger::AlternateStateLabelInvented,
            M5ProviderDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_PROVIDER_MAPPING_SYNC_ROW_SCHEMA_REF,
            M5_PROVIDER_MAPPING_SYNC_ROW_TARGET_MAPPING_REF,
            M5_PROVIDER_MAPPING_SYNC_ROW_SYNC_HEALTH_REF,
        ]),
        mapping_examples,
        sync_examples,
        assumes_default_destination_silently: false,
        masks_mapping_origin_or_lock: false,
        collapses_sync_into_generic_synced: false,
        hides_local_draft_queue_state: false,
    }
}

fn rows() -> Vec<M5MappingSyncConsumerRow> {
    use M5MappingOriginClass as Origin;
    use M5MappingTargetKind as Target;
    use M5ProviderSyncMode as Mode;
    use M5ProviderWriteScope as Scope;
    use M5QueuedDraftState as Draft;

    vec![
        // 1. Mapping-picker panel — an explicit user-chosen issue-tracker project (a
        //    local-scope row offering change and reset) and a policy-pinned kanban board (a
        //    policy-locked row whose change is blocked and lock note is shown); a live
        //    full-write sync with a queued publish (full bidirectional, queue visible) and a
        //    read-only mirror (read-only metadata, no queue).
        base_row(
            M5MappingSyncConsumerSurface::MappingPickerPanel,
            M5ProviderQualificationClass::Stable,
            "Mapping picker panel owner",
            "The mapping-picker panel renders the shared mapping row so a user's explicit issue-tracker choice reads as a local-scope row offering change and reset, and an admin-pinned kanban board reads as a policy-locked row that blocks change and shows its lock note — never a silent default destination; the same panel's sync row separates a live full-write sync from a read-only mirror",
            "evidence:m5-mapping-sync-row-mapping-picker:001",
            vec![
                mapping_case(
                    Target::IssueTrackerProject,
                    Origin::ExplicitUserChoice,
                    "acme-eng issues (chosen)",
                    "repo acme/eng ↔ issues project",
                    None,
                    "mapping:acme-eng:issues:explicit",
                ),
                mapping_case(
                    Target::KanbanBoard,
                    Origin::PolicyPinned,
                    "acme-eng delivery board (pinned)",
                    "workspace acme ↔ delivery board",
                    Some("Pinned by org admin policy; change requires an admin"),
                    "mapping:acme-eng:board:policy",
                ),
            ],
            vec![
                sync_case(
                    Mode::LiveBidirectional,
                    Scope::FullWrite,
                    Draft::QueuedPublish,
                    "acme-eng live sync (queued)",
                    "sync:acme-eng:live-full",
                ),
                sync_case(
                    Mode::ReadOnlyMirror,
                    Scope::ReadOnly,
                    Draft::NoLocalDraft,
                    "acme-eng read-only mirror",
                    "sync:acme-eng:read-mirror",
                ),
            ],
        ),
        // 2. Sync-behavior panel — an inherited-default repository (inherited scope, change
        //    without reset) and an auto-matched milestone (inherited scope, change and reset);
        //    a manual comment-only push with a pending draft (comment/link sync, queue
        //    visible) and a scheduled status-only sync whose publish failed (status-transition
        //    sync, retry offered).
        base_row(
            M5MappingSyncConsumerSurface::SyncBehaviorPanel,
            M5ProviderQualificationClass::Stable,
            "Sync-behavior panel owner",
            "The sync-behavior panel renders the shared sync row so a manual comment-only push with a pending draft reads as a comment/link sync with a visible queue, and a scheduled status-only sync whose publish failed reads as a status-transition sync offering retry — never one ambiguous synced label; the same panel's mapping row separates an inherited default repository from an auto-matched milestone",
            "evidence:m5-mapping-sync-row-sync-behavior:001",
            vec![
                mapping_case(
                    Target::Repository,
                    Origin::InheritedDefault,
                    "acme-eng default repo",
                    "workspace acme ↔ default repository",
                    None,
                    "mapping:acme-eng:repo:inherited",
                ),
                mapping_case(
                    Target::Milestone,
                    Origin::AutoMatched,
                    "acme-eng Q3 milestone (auto)",
                    "repo acme/eng ↔ Q3 milestone",
                    None,
                    "mapping:acme-eng:milestone:auto",
                ),
            ],
            vec![
                sync_case(
                    Mode::ManualPush,
                    Scope::CommentOnly,
                    Draft::DraftPending,
                    "acme-eng comment push (pending)",
                    "sync:acme-eng:comment-push",
                ),
                sync_case(
                    Mode::ScheduledSync,
                    Scope::StatusOnly,
                    Draft::PublishFailed,
                    "acme-eng status sync (failed)",
                    "sync:acme-eng:status-sched",
                ),
            ],
        ),
        // 3. Provider status bar — an imported-config label set (inherited scope, change and
        //    reset) and a genuinely unmapped target (unmapped scope, flagged unmapped rather
        //    than defaulted); an offline-only queue (offline-capture-only) and a paused sync
        //    with a blocked publish (paused, queue visible).
        base_row(
            M5MappingSyncConsumerSurface::ProviderStatusBar,
            M5ProviderQualificationClass::Stable,
            "Provider status bar owner",
            "The provider status bar renders both rows so an imported-config label set reads as an inherited-scope row that can be changed or reset, an unmapped target reads as an unmapped row that never resolves to a silent default, an offline-only queue reads as offline-capture-only, and a paused sync with a blocked publish reads as paused with a visible queue — so a user can tell destination and sync behavior from the bar alone",
            "evidence:m5-mapping-sync-row-status-bar:001",
            vec![
                mapping_case(
                    Target::LabelSet,
                    Origin::ImportedConfig,
                    "acme-eng imported labels",
                    "repo acme/eng ↔ imported label set",
                    None,
                    "mapping:acme-eng:labels:imported",
                ),
                mapping_case(
                    Target::UnmappedTarget,
                    Origin::UnmappedOrigin,
                    "no destination chosen",
                    "repo acme/eng ↔ (unmapped)",
                    None,
                    "mapping:acme-eng:unmapped:slot-1",
                ),
            ],
            vec![
                sync_case(
                    Mode::OfflineOnly,
                    Scope::NoWrite,
                    Draft::QueuedPublish,
                    "acme-eng offline capture (queued)",
                    "sync:acme-eng:offline-only",
                ),
                sync_case(
                    Mode::PausedSync,
                    Scope::ScopeUnknown,
                    Draft::PublishBlocked,
                    "acme-eng sync paused (blocked)",
                    "sync:acme-eng:paused",
                ),
            ],
        ),
        // 4. Headless / CLI mappings — a policy-pinned repository (policy-locked, change
        //    blocked) and an explicit user-chosen kanban board (local scope, change and reset);
        //    a live status-only sync reconciled (status-transition, no queue) and a scheduled
        //    full-write sync clean (full bidirectional, no queue) — proving the same grammar
        //    works headless.
        base_row(
            M5MappingSyncConsumerSurface::HeadlessCliMappings,
            M5ProviderQualificationClass::Stable,
            "Headless CLI mappings owner",
            "The headless / CLI mappings surface renders both rows so a policy-pinned repository reads as a policy-locked row that blocks change, an explicit user-chosen board reads as a local-scope row offering change and reset, a reconciled live status sync reads as a status-transition sync, and a clean scheduled full-write sync reads as full bidirectional — proving the same mapping/sync grammar works headless",
            "evidence:m5-mapping-sync-row-headless-cli:001",
            vec![
                mapping_case(
                    Target::Repository,
                    Origin::PolicyPinned,
                    "acme-infra repo (pinned)",
                    "workspace acme ↔ infra repository",
                    Some("Pinned by platform policy; managed centrally"),
                    "mapping:acme-infra:repo:policy",
                ),
                mapping_case(
                    Target::KanbanBoard,
                    Origin::ExplicitUserChoice,
                    "acme-infra ops board (chosen)",
                    "workspace acme ↔ ops board",
                    None,
                    "mapping:acme-infra:board:explicit",
                ),
            ],
            vec![
                sync_case(
                    Mode::LiveBidirectional,
                    Scope::StatusOnly,
                    Draft::PublishedReconciled,
                    "acme-infra status sync (reconciled)",
                    "sync:acme-infra:status-live",
                ),
                sync_case(
                    Mode::ScheduledSync,
                    Scope::FullWrite,
                    Draft::NoLocalDraft,
                    "acme-infra full sync (clean)",
                    "sync:acme-infra:full-sched",
                ),
            ],
        ),
        // 5. Support mapping export — an inherited-default issue-tracker project and an
        //    imported-config repository (both inherited scope, export-safe); a read-only mirror
        //    reconciled (read-only metadata) and a manual comment push queued (comment/link
        //    sync, queue visible) — the same rows a support agent reads elsewhere.
        base_row(
            M5MappingSyncConsumerSurface::SupportMappingExport,
            M5ProviderQualificationClass::Stable,
            "Support mapping export owner",
            "The support mapping export renders both rows so an inherited-default issue-tracker project and an imported-config repository export as inherited-scope rows without leaking endpoints, a reconciled read-only mirror exports as read-only metadata, and a queued manual comment push exports as a comment/link sync with a visible queue — the same rows a support agent reads elsewhere",
            "evidence:m5-mapping-sync-row-support-export:001",
            vec![
                mapping_case(
                    Target::IssueTrackerProject,
                    Origin::InheritedDefault,
                    "acme-eng default issues",
                    "workspace acme ↔ default issues project",
                    None,
                    "mapping:acme-eng:issues:inherited",
                ),
                mapping_case(
                    Target::Repository,
                    Origin::ImportedConfig,
                    "acme-eng imported repo",
                    "repo acme/eng ↔ imported repository",
                    None,
                    "mapping:acme-eng:repo:imported",
                ),
            ],
            vec![
                sync_case(
                    Mode::ReadOnlyMirror,
                    Scope::ReadOnly,
                    Draft::PublishedReconciled,
                    "acme-eng read mirror (reconciled)",
                    "sync:acme-eng:read-mirror-2",
                ),
                sync_case(
                    Mode::ManualPush,
                    Scope::CommentOnly,
                    Draft::QueuedPublish,
                    "acme-eng comment push (queued)",
                    "sync:acme-eng:comment-push-2",
                ),
            ],
        ),
    ]
}

fn governance_review() -> M5MappingSyncRowGovernanceReview {
    M5MappingSyncRowGovernanceReview {
        mapping_row_shows_provider_project: true,
        mapping_row_shows_repo_workspace_relation: true,
        mapping_row_shows_mapping_origin_and_scope: true,
        mapping_row_shows_lock_note: true,
        mapping_row_offers_change_and_reset: true,
        mapping_never_assumes_default_destination: true,
        sync_row_separates_read_comment_status_offline: true,
        sync_row_shows_local_draft_queue_state: true,
        sync_never_uses_one_generic_synced_label: true,
        rows_stable_across_deployment_lines: true,
        rows_stable_across_consumer_surfaces: true,
        every_row_declares_accessibility_route: true,
        support_export_reconstructs_mapping_and_sync_truth: true,
        later_rows_cannot_invent_parallel_mapping_or_sync_vocabulary: true,
    }
}

fn consumer_projection() -> M5MappingSyncRowConsumerProjection {
    M5MappingSyncRowConsumerProjection {
        provider_surfaces_consume_mapping_sync_vocabulary: true,
        mapping_posture_reads_single_source: true,
        sync_behavior_reads_single_source: true,
        support_export_reads_single_source: true,
        headless_and_desktop_read_single_source: true,
    }
}

fn proof_freshness() -> M5MappingSyncRowProofFreshness {
    M5MappingSyncRowProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5MappingSyncRowReleasePosture {
    M5MappingSyncRowReleasePosture {
        release_packet_ref: M5_PROVIDER_MAPPING_SYNC_ROW_ARTIFACT_REF.to_owned(),
        provider_mapping_sync_audit_ref: M5_PROVIDER_MAPPING_SYNC_ROW_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_PROVIDER_MAPPING_SYNC_ROW_SCHEMA_REF,
        M5_PROVIDER_MAPPING_SYNC_ROW_DOC_REF,
        M5_PROVIDER_MAPPING_SYNC_ROW_COMPONENT_MATRIX_REF,
        M5_PROVIDER_MAPPING_SYNC_ROW_TARGET_MAPPING_REF,
        M5_PROVIDER_MAPPING_SYNC_ROW_SYNC_HEALTH_REF,
    ])
}

/// Builds the canonical M5 provider mapping/sync-row packet.
pub fn seeded_m5_provider_mapping_sync_row_packet() -> M5ProviderMappingSyncRowPacket {
    M5ProviderMappingSyncRowPacket::new(M5ProviderMappingSyncRowPacketInput {
        packet_id: M5_PROVIDER_MAPPING_SYNC_ROW_PACKET_ID.to_owned(),
        matrix_label:
            "M5 provider mapping / sync-behavior row primitive: project/board mapping origin (explicit/inherited/auto/imported/policy/unmapped), inherited/local/policy/unmapped scope, target kind, lock note, and change/reset actions, plus sync mode, effective write scope, derived read-only-metadata/comment-link/status-transition/full-bidirectional/offline-capture-only/paused behavior class, visible queued-draft state, and bounded reveal/change/view-queue/retry/export actions"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5MappingSyncRowVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the sync-behavior panel consumer is narrowed to Preview pending
/// read-only-versus-write sync-behavior-separation parity proof across every deployment; every
/// consumer stays visible.
pub fn seeded_m5_provider_mapping_sync_row_sync_behavior_preview_narrowed(
) -> M5ProviderMappingSyncRowPacket {
    let mut packet = seeded_m5_provider_mapping_sync_row_packet();
    packet.packet_id =
        "m5-provider-mapping-sync-behavior-row-primitive:sync-behavior-preview:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5MappingSyncConsumerSurface::SyncBehaviorPanel)
        .expect("sync-behavior panel row present");
    row.qualification = M5ProviderQualificationClass::Preview;
    packet
}

/// Narrowed variant: the headless / CLI mappings consumer is held at Beta because a slice of
/// headless rows do not yet render the keyboard route cue on every profile; every consumer
/// stays visible.
pub fn seeded_m5_provider_mapping_sync_row_headless_cli_mappings_beta_narrowed(
) -> M5ProviderMappingSyncRowPacket {
    let mut packet = seeded_m5_provider_mapping_sync_row_packet();
    packet.packet_id =
        "m5-provider-mapping-sync-behavior-row-primitive:headless-cli-mappings-beta:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5MappingSyncConsumerSurface::HeadlessCliMappings)
        .expect("headless-cli-mappings row present");
    row.qualification = M5ProviderQualificationClass::Beta;
    packet
}
