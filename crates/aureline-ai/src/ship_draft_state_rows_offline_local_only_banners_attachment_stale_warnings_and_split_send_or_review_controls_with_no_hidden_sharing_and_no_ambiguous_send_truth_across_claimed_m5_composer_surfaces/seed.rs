//! Canonical seed builders for the M5 draft-state-row / stale-banner / send-review-control
//! primitive.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix, the
//! artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical draft/stale/send-primitive packet.
pub const M5_DRAFT_SEND_PACKET_ID: &str =
    "m5-draft-state-row-stale-banner-send-review-control-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked draft-state-row resolution case from a full draft state.
#[allow(clippy::too_many_arguments)]
fn draft_case(
    draft_id: &str,
    draft_label: &str,
    locality: M5DraftLocality,
    saved: bool,
    shared_or_retained: bool,
    sharing_exception_note: Option<&str>,
    sync_or_policy_note: Option<&str>,
    clearable: bool,
    deletable: bool,
) -> M5DraftStateRowResolutionCase {
    M5DraftStateRowResolutionCase::resolved(M5DraftStateRowResolutionInput {
        draft_id: draft_id.to_owned(),
        draft_label: draft_label.to_owned(),
        locality,
        saved,
        shared_or_retained,
        sharing_exception_note: sharing_exception_note.map(str::to_owned),
        sync_or_policy_note: sync_or_policy_note.map(str::to_owned),
        clearable,
        deletable,
    })
}

/// Builds a worked attachment-stale-banner resolution case from a full banner state.
#[allow(clippy::too_many_arguments)]
fn stale_case(
    banner_id: &str,
    attachment_label: &str,
    offline_local_only: bool,
    staleness_reason: Option<M5StalenessReason>,
    refresh_available: bool,
    local_safe_alternative_available: bool,
    recovery_note: Option<&str>,
) -> M5AttachmentStaleBannerResolutionCase {
    M5AttachmentStaleBannerResolutionCase::resolved(M5AttachmentStaleBannerResolutionInput {
        banner_id: banner_id.to_owned(),
        attachment_label: attachment_label.to_owned(),
        offline_local_only,
        staleness_reason,
        refresh_available,
        local_safe_alternative_available,
        recovery_note: recovery_note.map(str::to_owned),
    })
}

/// Builds a worked send-review-control resolution case from a full send state.
#[allow(clippy::too_many_arguments)]
fn send_case(
    control_id: &str,
    control_label: &str,
    route_before: Option<M5ComposerRouteClass>,
    route_after: M5ComposerRouteClass,
    widens_authority: bool,
    is_mutating_route: bool,
    pending_reviews: Vec<M5ReviewRequirement>,
    policy_blocked: bool,
    over_budget: bool,
    taint_blocked: bool,
) -> M5SendReviewControlResolutionCase {
    M5SendReviewControlResolutionCase::resolved(M5SendReviewControlResolutionInput {
        control_id: control_id.to_owned(),
        control_label: control_label.to_owned(),
        route_before,
        route_after,
        widens_authority,
        is_mutating_route,
        pending_reviews,
        policy_blocked,
        over_budget,
        taint_blocked,
    })
}

/// A base row with the shared fields filled in and the full draft / stale / send anatomy, posture,
/// reason, class, path, action, export-field, and accessibility parity every consumer carries.
#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5DraftSendConsumerSurface,
    qualification: M5ComposerQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    draft_examples: Vec<M5DraftStateRowResolutionCase>,
    stale_examples: Vec<M5AttachmentStaleBannerResolutionCase>,
    send_examples: Vec<M5SendReviewControlResolutionCase>,
) -> M5DraftSendRow {
    M5DraftSendRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5ComposerSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ComposerDeploymentLine::ALL.to_vec(),
        draft_anatomy_parts: M5DraftStateRowAnatomyPart::ALL.to_vec(),
        stale_anatomy_parts: M5StaleBannerAnatomyPart::ALL.to_vec(),
        send_anatomy_parts: M5SendControlAnatomyPart::ALL.to_vec(),
        draft_localities: M5DraftLocality::ALL.to_vec(),
        retention_postures: M5DraftRetentionPosture::ALL.to_vec(),
        draft_actions: M5DraftStateAction::ALL.to_vec(),
        staleness_reasons: M5StalenessReason::ALL.to_vec(),
        banner_postures: M5StaleBannerPosture::ALL.to_vec(),
        stale_actions: M5StaleBannerAction::ALL.to_vec(),
        send_postures: M5SendPosture::ALL.to_vec(),
        send_paths: M5SendPath::ALL.to_vec(),
        review_requirements: M5ReviewRequirement::ALL.to_vec(),
        route_classes: M5ComposerRouteClass::ALL.to_vec(),
        send_actions: M5SendControlAction::ALL.to_vec(),
        draft_export_fields: M5DraftStateRowExportField::ALL.to_vec(),
        stale_export_fields: M5StaleBannerExportField::ALL.to_vec(),
        send_export_fields: M5SendControlExportField::ALL.to_vec(),
        accessibility_routes: M5ComposerAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ComposerConsumerSurface::InlineComposerUi,
            M5ComposerConsumerSurface::ComposerPanelUi,
            M5ComposerConsumerSurface::PatchReviewUi,
            M5ComposerConsumerSurface::SupportExport,
            M5ComposerConsumerSurface::CliInspect,
            M5ComposerConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5ComposerDowngradeTrigger::DraftLocalityMasked,
            M5ComposerDowngradeTrigger::AttachmentStalenessUndisclosed,
            M5ComposerDowngradeTrigger::SendReviewGateBypassed,
            M5ComposerDowngradeTrigger::RouteOrProviderMasked,
            M5ComposerDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_DRAFT_SEND_SCHEMA_REF,
            M5_DRAFT_SEND_PROMPT_COMPOSER_DRAFT_REF,
            M5_DRAFT_SEND_CONTEXT_ATTACHMENT_REF,
        ]),
        draft_examples,
        stale_examples,
        send_examples,
        masks_draft_locality_or_retention: false,
        assumes_hidden_sharing: false,
        invents_private_send_grammar: false,
        collapses_high_authority_send: false,
    }
}

// Keep the numbered contract cases beside their explanatory comments.
#[allow(clippy::vec_init_then_push)]
fn rows() -> Vec<M5DraftSendRow> {
    use M5ComposerRouteClass as Route;
    use M5DraftLocality as Loc;
    use M5ReviewRequirement as Req;
    use M5StalenessReason as Why;

    let mut rows = Vec::new();

    // 1. Inline composer — a persisted local-only draft and an ephemeral unsaved draft; a fresh
    //    attachment and an offline-local-only route that preserves the draft with refresh and a
    //    local-safe alternative; a plain ready send and a review-before-send that requires an
    //    attachment review.
    rows.push(base_row(
        M5DraftSendConsumerSurface::InlineComposer,
        M5ComposerQualificationClass::Stable,
        "Inline composer owner",
        "The inline composer renders the shared draft-state row, attachment-stale banner, and send-review control so a persisted local-only draft and an ephemeral unsaved draft each name their retention posture and clear / save behavior, a fresh attachment and an offline-local-only route each keep the current draft and offer refresh or a local-safe alternative, a plain non-widening send stays a single qualified send, and a route that requires an attachment review opens review before send",
        "evidence:m5-draft-send-inline:001",
        vec![
            draft_case(
                "draft.inline.local",
                "Inline composition draft",
                Loc::LocalOnly,
                true,
                false,
                None,
                None,
                true,
                false,
            ),
            draft_case(
                "draft.inline.ephemeral",
                "Inline composition draft",
                Loc::EphemeralUnsaved,
                false,
                false,
                None,
                None,
                false,
                false,
            ),
        ],
        vec![
            stale_case(
                "stale.inline.fresh",
                "attached repo file",
                false,
                None,
                false,
                false,
                None,
            ),
            stale_case(
                "stale.inline.offline",
                "attached mirrored doc",
                true,
                None,
                true,
                true,
                None,
            ),
        ],
        vec![
            send_case(
                "send.inline.ready",
                "Inline explain send",
                Some(Route::LocalModel),
                Route::LocalModel,
                false,
                false,
                vec![],
                false,
                false,
                false,
            ),
            send_case(
                "send.inline.review",
                "Inline review send",
                Some(Route::ManagedRoute),
                Route::ManagedRoute,
                false,
                false,
                vec![Req::AttachmentReview],
                false,
                false,
                false,
            ),
        ],
    ));

    // 2. Side panel — a workspace-synced draft and an account-synced draft, each disclosing its
    //    sync exception; two refreshable stale attachments (edited, moved); a split-send-review
    //    that widens on-device to managed with a route-change ack and an over-budget-blocked send.
    rows.push(base_row(
        M5DraftSendConsumerSurface::SidePanel,
        M5ComposerQualificationClass::Stable,
        "Side panel owner",
        "The side panel renders the same draft-state row, attachment-stale banner, and send-review control so a workspace-synced draft and an account-synced draft each disclose their sync exception instead of reading as local-only, an edited and a moved attachment each stay refreshable while the draft is preserved, an on-device-to-managed route that widens authority splits into explain-only / review / mutating paths, and an over-budget send is blocked until the budget is resolved",
        "evidence:m5-draft-send-side-panel:001",
        vec![
            draft_case(
                "draft.side.workspace",
                "Side panel draft",
                Loc::WorkspaceSynced,
                true,
                true,
                Some("synced to this workspace for teammates with access"),
                Some("workspace retention policy applies"),
                true,
                true,
            ),
            draft_case(
                "draft.side.account",
                "Side panel draft",
                Loc::AccountSynced,
                true,
                true,
                Some("synced to your account across your devices"),
                None,
                false,
                false,
            ),
        ],
        vec![
            stale_case(
                "stale.side.edited",
                "attached source file",
                false,
                Some(Why::SourceEdited),
                true,
                false,
                None,
            ),
            stale_case(
                "stale.side.moved",
                "attached module",
                false,
                Some(Why::SourceMoved),
                true,
                false,
                None,
            ),
        ],
        vec![
            send_case(
                "send.side.split",
                "Side panel widened send",
                Some(Route::LocalModel),
                Route::ManagedRoute,
                true,
                true,
                vec![Req::RouteChangeAck],
                false,
                false,
                false,
            ),
            send_case(
                "send.side.overbudget",
                "Side panel over-budget send",
                Some(Route::ManagedRoute),
                Route::ManagedRoute,
                false,
                false,
                vec![],
                false,
                true,
                false,
            ),
        ],
    ));

    // 3. Patch draft — a shared-thread draft that can stop sharing and a purge-pending draft with
    //    its retention note; a superseded-review attachment and a deleted source that is gone; a
    //    policy-blocked send and a taint-blocked send.
    rows.push(base_row(
        M5DraftSendConsumerSurface::PatchDraft,
        M5ComposerQualificationClass::Stable,
        "Patch draft owner",
        "The patch draft renders the same draft-state row, attachment-stale banner, and send-review control so a shared-thread draft discloses its sharing and offers stop-sharing, a purge-pending draft names its retention note, a superseded attachment offers a review of the newer revision, a deleted source reads as gone with a detach and a local-safe alternative while the draft is preserved, and a policy-blocked or taint-blocked send refuses to leave the shell until the blocker is resolved",
        "evidence:m5-draft-send-patch-draft:001",
        vec![
            draft_case(
                "draft.patch.shared",
                "Patch draft",
                Loc::SharedThread,
                true,
                true,
                Some("shared into this review thread for reviewers"),
                None,
                false,
                true,
            ),
            draft_case(
                "draft.patch.purge",
                "Patch draft",
                Loc::RetentionPendingPurge,
                true,
                true,
                Some("retained copy still exists on the sync line"),
                Some("scheduled for purge at the end of the retention window"),
                true,
                false,
            ),
        ],
        vec![
            stale_case(
                "stale.patch.superseded",
                "attached diff revision",
                false,
                Some(Why::RevisionSuperseded),
                true,
                false,
                None,
            ),
            stale_case(
                "stale.patch.deleted",
                "attached deleted file",
                false,
                Some(Why::SourceDeleted),
                false,
                true,
                Some("the source file was deleted; use the checked-in snapshot instead"),
            ),
        ],
        vec![
            send_case(
                "send.patch.policy",
                "Patch draft policy send",
                Some(Route::ManagedRoute),
                Route::ManagedRoute,
                false,
                true,
                vec![],
                true,
                false,
                false,
            ),
            send_case(
                "send.patch.taint",
                "Patch draft taint send",
                Some(Route::ManagedRoute),
                Route::ManagedRoute,
                false,
                true,
                vec![],
                false,
                false,
                true,
            ),
        ],
    ));

    // 4. CLI / headless — a clearable-and-deletable local draft and a workspace-synced draft; an
    //    access-revoked attachment and a reindexed attachment; a split-send-review that widens
    //    with a budget ack and a review-before-send that requires a taint ack.
    rows.push(base_row(
        M5DraftSendConsumerSurface::CliHeadless,
        M5ComposerQualificationClass::Stable,
        "CLI / headless owner",
        "The CLI / headless surface renders the same draft-state row, attachment-stale banner, and send-review control so a clearable-and-deletable local draft and a workspace-synced draft each name their retention and clear / delete behavior, a permission-revoked attachment reads as access-revoked with a detach and a local-safe alternative and a reindexed attachment stays refreshable, a widened route splits into explain-only / review / mutating paths behind a budget ack, and a taint-ack review runs before send — the same truth a headless reviewer reads elsewhere",
        "evidence:m5-draft-send-cli:001",
        vec![
            draft_case(
                "draft.cli.local",
                "Headless draft",
                Loc::LocalOnly,
                false,
                false,
                None,
                None,
                true,
                true,
            ),
            draft_case(
                "draft.cli.workspace",
                "Headless draft",
                Loc::WorkspaceSynced,
                true,
                true,
                Some("synced to this workspace for headless runs"),
                None,
                false,
                false,
            ),
        ],
        vec![
            stale_case(
                "stale.cli.revoked",
                "attached restricted file",
                false,
                Some(Why::PermissionRevoked),
                false,
                true,
                Some("access was revoked; use the local cached copy instead"),
            ),
            stale_case(
                "stale.cli.reindexed",
                "attached indexed snippet",
                false,
                Some(Why::IndexReindexed),
                true,
                false,
                None,
            ),
        ],
        vec![
            send_case(
                "send.cli.split",
                "Headless widened send",
                Some(Route::LocalModel),
                Route::ManagedRoute,
                true,
                true,
                vec![Req::BudgetAck],
                false,
                false,
                false,
            ),
            send_case(
                "send.cli.review",
                "Headless taint-ack send",
                Some(Route::ManagedRoute),
                Route::ManagedRoute,
                false,
                true,
                vec![Req::TaintAck],
                false,
                false,
                false,
            ),
        ],
    ));

    // 5. Support export — an ephemeral unsaved draft and a shared-thread draft; an offline
    //    local-only route with only a local-safe alternative and a fresh attachment; a mutating
    //    ready send that still offers an explain-only path and a widened split-send-review.
    rows.push(base_row(
        M5DraftSendConsumerSurface::SupportExport,
        M5ComposerQualificationClass::Stable,
        "Support export owner",
        "The support export renders the same draft-state row, attachment-stale banner, and send-review control so an ephemeral unsaved draft's local-only posture and a shared-thread draft's disclosed sharing are reconstructable from the export alone, an offline-local-only route with only a local-safe alternative preserves the draft, a fresh attachment reads as fresh, a mutating ready send still offers an explain-only path beside its direct send, and a widened route stays split into explain-only / review / mutating paths",
        "evidence:m5-draft-send-support:001",
        vec![
            draft_case(
                "draft.support.ephemeral",
                "Support export draft",
                Loc::EphemeralUnsaved,
                false,
                false,
                None,
                None,
                false,
                false,
            ),
            draft_case(
                "draft.support.shared",
                "Support export draft",
                Loc::SharedThread,
                true,
                true,
                Some("shared into a support thread for the assigned agent"),
                None,
                false,
                true,
            ),
        ],
        vec![
            stale_case(
                "stale.support.offline",
                "attached offline bundle",
                true,
                None,
                false,
                true,
                None,
            ),
            stale_case(
                "stale.support.fresh",
                "attached current file",
                false,
                None,
                false,
                false,
                None,
            ),
        ],
        vec![
            send_case(
                "send.support.ready",
                "Support export mutating send",
                Some(Route::ManagedRoute),
                Route::ManagedRoute,
                false,
                true,
                vec![],
                false,
                false,
                false,
            ),
            send_case(
                "send.support.split",
                "Support export widened send",
                Some(Route::LocalModel),
                Route::ManagedRoute,
                true,
                true,
                vec![Req::TaintAck],
                false,
                false,
                false,
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5DraftSendGovernanceReview {
    M5DraftSendGovernanceReview {
        one_primitive_carries_draft_stale_send_truth: true,
        draft_row_names_locality_and_retention: true,
        shared_or_retained_exceptions_always_disclosed: true,
        sync_or_policy_notes_exportable: true,
        clear_delete_behavior_always_available: true,
        stale_banner_preserves_draft: true,
        offline_local_only_offers_refresh_or_local_alternative: true,
        no_silent_retry_loops: true,
        send_control_splits_high_authority_paths: true,
        no_single_unqualified_send_on_widened_authority: true,
        every_row_declares_accessibility_route: true,
        descriptors_stable_across_ui_export_support: true,
    }
}

fn consumer_projection() -> M5DraftSendConsumerProjection {
    M5DraftSendConsumerProjection {
        send_capable_surfaces_consume_shared_primitive: true,
        draft_state_reads_single_source: true,
        stale_state_reads_single_source: true,
        send_posture_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5DraftSendProofFreshness {
    M5DraftSendProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5DraftSendReleasePosture {
    M5DraftSendReleasePosture {
        release_packet_ref: M5_DRAFT_SEND_ARTIFACT_REF.to_owned(),
        ai_audit_ref: M5_DRAFT_SEND_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_DRAFT_SEND_SCHEMA_REF,
        M5_DRAFT_SEND_DOC_REF,
        M5_DRAFT_SEND_COMPONENT_MATRIX_REF,
        M5_DRAFT_SEND_PROMPT_COMPOSER_DRAFT_REF,
        M5_DRAFT_SEND_CONTEXT_ATTACHMENT_REF,
    ])
}

/// Builds the canonical M5 draft-state-row / stale-banner / send-review-control packet.
pub fn seeded_m5_draft_send_packet() -> M5DraftSendPacket {
    M5DraftSendPacket::new(M5DraftSendPacketInput {
        packet_id: M5_DRAFT_SEND_PACKET_ID.to_owned(),
        matrix_label:
            "M5 draft-state row, attachment-stale banner, and send-review control primitive: draft locality and retention posture, shared-or-retained disclosure, sync / policy notes, clear / delete behavior, offline-local-only and attachment-stale banner postures with preserved-draft and refresh / local-safe alternatives, and split explain-only / review / mutating send paths with no single unqualified send on widened authority"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5DraftSendVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the patch draft is narrowed to Preview pending clear / delete parity proof
/// across every patch-apply path; every consumer stays visible.
pub fn seeded_m5_draft_send_patch_draft_preview_narrowed() -> M5DraftSendPacket {
    let mut packet = seeded_m5_draft_send_packet();
    packet.packet_id =
        "m5-draft-state-row-stale-banner-send-review-control-primitive:patch-draft-preview:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5DraftSendConsumerSurface::PatchDraft)
        .expect("patch-draft row present");
    row.qualification = M5ComposerQualificationClass::Preview;
    packet
}

/// Narrowed variant: the CLI / headless surface is held at Beta because a slice of headless paths
/// do not yet render the split send-path cue on every profile; every consumer stays visible.
pub fn seeded_m5_draft_send_cli_headless_beta_narrowed() -> M5DraftSendPacket {
    let mut packet = seeded_m5_draft_send_packet();
    packet.packet_id =
        "m5-draft-state-row-stale-banner-send-review-control-primitive:cli-headless-beta:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5DraftSendConsumerSurface::CliHeadless)
        .expect("cli-headless row present");
    row.qualification = M5ComposerQualificationClass::Beta;
    packet
}
