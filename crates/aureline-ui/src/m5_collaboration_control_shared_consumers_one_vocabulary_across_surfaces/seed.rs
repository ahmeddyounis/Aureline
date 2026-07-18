//! Canonical seed for the collaboration-control shared-consumer parity packet.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export, matrix CSV,
//! Markdown summary, and narrowed fixtures. Every binding is derived from one per-subject
//! [`CollaborationControlSharedStateFacetValues`] so the same seeded collaboration session always carries the
//! same vocabulary across surfaces, and every narrowed representation derives its disclosure from
//! [`resolve_collaboration_control_shared_render_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-18T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

fn facets(
    collaboration_control_role: &str,
    object: &str,
    registry_reference: &str,
    session_state: &str,
    surface_context: &str,
    authority_source: &str,
) -> CollaborationControlSharedStateFacetValues {
    CollaborationControlSharedStateFacetValues {
        collaboration_control_role_word: collaboration_control_role.to_owned(),
        object_word: object.to_owned(),
        registry_reference_word: registry_reference.to_owned(),
        session_state_word: session_state.to_owned(),
        surface_context_word: surface_context.to_owned(),
        authority_source_word: authority_source.to_owned(),
    }
}

fn preserved_note_for(reason: CollaborationControlSharedNarrowReason) -> String {
    match reason {
        CollaborationControlSharedNarrowReason::CompactionNarrowed => {
            "collaboration-control-role, object, registry-reference, session-state, surface-context, and authority-source words preserved; only disclosure depth compacted"
        }
        CollaborationControlSharedNarrowReason::RemoteProjectionNarrowed => {
            "all collaboration-control vocabulary preserved; the object is projected from the remote source of truth"
        }
        CollaborationControlSharedNarrowReason::ExportRedactionNarrowed => {
            "all collaboration-control vocabulary preserved; only surrounding detail is redacted export-safe"
        }
    }
    .to_owned()
}

fn next_action_label_for(action: CollaborationControlSharedNarrowNextAction) -> String {
    match action {
        CollaborationControlSharedNarrowNextAction::ExpandInDesktop => {
            "Expand in the desktop surface"
        }
        CollaborationControlSharedNarrowNextAction::OpenRemoteSource => "Open the remote source",
        CollaborationControlSharedNarrowNextAction::OpenFullDetail => "Open the full detail",
    }
    .to_owned()
}

fn binding_refs(object: M5CollaborationControlObject) -> Vec<String> {
    vec![
        M5_COLLABORATION_CONTROL_MATRIX_SCHEMA_REF.to_owned(),
        object.canonical_domain_schema_ref().to_owned(),
    ]
}

fn make_binding(
    binding_id: &str,
    subject_id: &str,
    subject_label: &str,
    object: M5CollaborationControlObject,
    consumer: M5CollaborationControlConsumerSurface,
    representation: CollaborationControlSharedRepresentation,
    state_facets: CollaborationControlSharedStateFacetValues,
) -> CollaborationControlSharedConsumerBinding {
    let disclosure = resolve_collaboration_control_shared_render_disclosure(representation);
    let narrow_note = disclosure.narrow_reason.map(|reason| {
        let next_action = disclosure
            .narrow_next_action
            .expect("a narrow reason always carries a next action");
        CollaborationControlSharedNarrowNote {
            reason,
            preserved_vocabulary_note: preserved_note_for(reason),
            next_action,
            next_action_label: next_action_label_for(next_action),
        }
    });
    let remote_source_note = if disclosure.needs_remote_source_note {
        "projected from the remote source of truth; the source stays remote".to_owned()
    } else {
        String::new()
    };
    let export_detail_note = if disclosure.needs_export_detail_note {
        "surrounding detail redacted export-safe in this packet; full detail available on request"
            .to_owned()
    } else {
        String::new()
    };

    CollaborationControlSharedConsumerBinding {
        binding_id: binding_id.to_owned(),
        subject_id: subject_id.to_owned(),
        subject_label: subject_label.to_owned(),
        object,
        consumer,
        representation,
        state_facets,
        vocabulary_state: disclosure.vocabulary_state,
        narrow_note,
        remote_source_note,
        export_detail_note,
        acquires_terminal_or_debug_control_from_presence_without_an_explicit_grant: false,
        allows_more_than_one_active_driver_on_a_sensitive_surface: false,
        starts_recording_retention_or_guest_scope_widening_silently: false,
        replays_prior_terminal_or_debug_input_on_join_or_restore: false,
        reveals_raw_secrets_command_text_variable_bodies_or_clipboard_contents_without_a_guard:
            false,
        source_contract_refs: binding_refs(object),
    }
}

/// One consumer-surface adoption of a seeded subject, before any representation override.
struct BindingSpec {
    binding_id: &'static str,
    consumer: M5CollaborationControlConsumerSurface,
    representation: CollaborationControlSharedRepresentation,
}

/// One seeded collaboration session rendered across several consumer surfaces at one vocabulary.
struct SubjectSpec {
    subject_id: &'static str,
    subject_label: &'static str,
    object: M5CollaborationControlObject,
    facets: CollaborationControlSharedStateFacetValues,
    bindings: Vec<BindingSpec>,
}

fn spec(
    subject_id: &'static str,
    subject_label: &'static str,
    object: M5CollaborationControlObject,
    facets: CollaborationControlSharedStateFacetValues,
    bindings: Vec<BindingSpec>,
) -> SubjectSpec {
    SubjectSpec {
        subject_id,
        subject_label,
        object,
        facets,
        bindings,
    }
}

fn bs(
    binding_id: &'static str,
    consumer: M5CollaborationControlConsumerSurface,
    representation: CollaborationControlSharedRepresentation,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        representation,
    }
}

/// The six seeded collaboration-control subjects — one per B155 collaboration-control object (the shared
/// terminal / debug view, the control grant, the presenter token, the consent envelope, the retention review,
/// and the session-restore view) — and the surfaces that adopt each, drawn from the shared-terminal-debug-view,
/// collaboration-join-review-sheet, control-grant-prompt, presenter-handoff-sheet, paste / secret guard,
/// collaboration-retention-sheet, session-restore-view, support-export, and help / docs consumers that back the
/// desktop-collaboration, browser / mobile companion, incident / support, audit-export, and help / docs
/// surfaces.
fn subject_specs() -> Vec<SubjectSpec> {
    use CollaborationControlSharedRepresentation::*;
    use M5CollaborationControlConsumerSurface::*;
    use M5CollaborationControlObject as Object;

    // Gate-role subjects (control-authority / active-driver / view-first-default / consent-scope) keep a real
    // authority-source / control-grant continuity; non-gate subjects carry a scoped descriptor.
    let authority_bound = "authority_source_disclosed_and_control_grant_bound";
    let collaboration_control_scoped_descriptor = "collaboration_control_scoped_descriptor";

    vec![
        spec(
            "shared-terminal-debug-view/one-active-driver",
            "Shared terminal / debug view (a live shared terminal or debugger stream that begins view-first, names its single active driver, and shows the provenance of every input rather than letting presence imply control)",
            Object::SharedTerminalDebugView,
            facets(
                "control_authority_disclosure",
                "shared_terminal_debug_view",
                "shared_terminal_debug_view_registry",
                "driver",
                "shared_terminal_debug_view_and_control_grant_prompt",
                authority_bound,
            ),
            vec![
                bs("ccsc-sterm-view", SharedTerminalDebugView, DesktopFull),
                bs("ccsc-sterm-grant", ControlGrantPrompt, DesktopFull),
                bs("ccsc-sterm-support", SupportExportPacket, ExportedRedacted),
            ],
        ),
        spec(
            "control-grant/explicit-time-boxed",
            "Control grant (an explicit, time-boxed grant of terminal / debug write control that names its authority, enforces a single active driver, and shows its scope, expiry, and revoke / reclaim path)",
            Object::ControlGrant,
            facets(
                "active_driver_disclosure",
                "control_grant",
                "control_grant_registry",
                "control_granted",
                "control_grant_prompt_and_shared_terminal_debug_view",
                authority_bound,
            ),
            vec![
                bs("ccsc-grant-prompt", ControlGrantPrompt, DesktopFull),
                bs("ccsc-grant-view", SharedTerminalDebugView, CompactNarrowed),
                bs("ccsc-grant-support", SupportExportPacket, ExportedRedacted),
            ],
        ),
        spec(
            "presenter-token/single-holder-handoff",
            "Presenter token (the presenter / moderator token that names its holder, its handoff target, and its moderation scope, never letting two presenters drive one sensitive surface)",
            Object::PresenterToken,
            facets(
                "view_first_default_disclosure",
                "presenter_token",
                "presenter_token_registry",
                "presenter_moderator",
                "presenter_handoff_sheet_and_control_grant_prompt",
                authority_bound,
            ),
            vec![
                bs("ccsc-presenter-handoff", PresenterHandoffSheet, DesktopFull),
                bs("ccsc-presenter-grant", ControlGrantPrompt, DesktopFull),
                bs("ccsc-presenter-support", SupportExportPacket, ExportedRedacted),
            ],
        ),
        spec(
            "consent-envelope/join-time-scope",
            "Consent envelope (the join-time consent scope that discloses recording, retention, guest scope, and route visibility consequences before a participant joins, never widening scope silently)",
            Object::ConsentEnvelope,
            facets(
                "consent_scope_disclosure",
                "consent_envelope",
                "consent_envelope_registry",
                "consent_renewal_required",
                "collaboration_join_review_sheet_and_collaboration_retention_sheet",
                authority_bound,
            ),
            vec![
                bs("ccsc-consent-join", CollaborationJoinReviewSheet, DesktopFull),
                bs("ccsc-consent-retention", CollaborationRetentionSheet, RemoteProjected),
                bs("ccsc-consent-help", HelpDocs, DesktopFull),
            ],
        ),
        spec(
            "retention-review/disclosed-retention",
            "Retention review (the recording / retention / sealed-archive review that names the recording state, retention mode and duration, and replayable-archive scope, never broadening retention silently)",
            Object::RetentionReview,
            facets(
                "recording_retention_state_disclosure",
                "retention_review",
                "retention_review_registry",
                "recording_active",
                "collaboration_retention_sheet_and_support_export_packet",
                collaboration_control_scoped_descriptor,
            ),
            vec![
                bs("ccsc-retention-sheet", CollaborationRetentionSheet, DesktopFull),
                bs("ccsc-retention-support", SupportExportPacket, ExportedRedacted),
                bs("ccsc-retention-help", HelpDocs, CompactNarrowed),
            ],
        ),
        spec(
            "session-restore-view/replay-free-rejoin",
            "Session-restore view (the replay-free session-restore view that reattaches read-only, replays no prior input, preserves retention scope, and requires a fresh control grant before write control resumes)",
            Object::SessionRestoreView,
            facets(
                "replay_free_restore_disclosure",
                "session_restore_view",
                "session_restore_view_registry",
                "restore_view_only",
                "session_restore_view_and_help_docs",
                collaboration_control_scoped_descriptor,
            ),
            vec![
                bs("ccsc-restore-view", SessionRestoreView, DesktopFull),
                bs("ccsc-restore-guard", PasteSecretGuard, DesktopFull),
                bs("ccsc-restore-support", SupportExportPacket, ExportedRedacted),
            ],
        ),
    ]
}

/// Builds all consumer bindings, applying `rep` to override a binding's representation.
fn build_bindings<F>(rep: F) -> Vec<CollaborationControlSharedConsumerBinding>
where
    F: Fn(
        &str,
        CollaborationControlSharedRepresentation,
    ) -> CollaborationControlSharedRepresentation,
{
    let mut bindings = Vec::new();
    for subject in subject_specs() {
        for spec in &subject.bindings {
            let representation = rep(spec.binding_id, spec.representation);
            bindings.push(make_binding(
                spec.binding_id,
                subject.subject_id,
                subject.subject_label,
                subject.object,
                spec.consumer,
                representation,
                subject.facets.clone(),
            ));
        }
    }
    bindings
}

fn trust_review() -> M5CollaborationControlSharedConsumersTrustReview {
    M5CollaborationControlSharedConsumersTrustReview {
        object_reuse_proven_by_fixtures: true,
        same_subject_same_collaboration_control_vocabulary_across_surfaces: true,
        collaboration_control_role_words_stay_in_frozen_vocabulary: true,
        gate_roles_never_let_presence_read_as_control_authority: true,
        never_more_than_one_active_driver_on_a_sensitive_surface: true,
        recording_retention_and_guest_scope_never_widened_silently: true,
        prior_terminal_debug_input_never_replayed_on_join_or_restore: true,
        raw_secrets_command_text_and_clipboard_never_revealed_without_a_guard: true,
        deferred_intent_never_queues_control_grants_presenter_handoffs_or_terminal_input: true,
        refused_control_actions_explain_instead_of_replaying_as_idempotent_background_writes: true,
        narrowing_disclosed_across_representations: true,
        support_export_point_canonical_contracts: true,
        copy_export_open_provider_preserve_one_payload: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> M5CollaborationControlSharedConsumersProjection {
    M5CollaborationControlSharedConsumersProjection {
        shared_terminal_debug_view_consumes_shared_collaboration_control_vocabulary: true,
        collaboration_join_review_sheet_consumes_shared_collaboration_control_vocabulary: true,
        control_grant_prompt_consumes_shared_collaboration_control_vocabulary: true,
        presenter_handoff_sheet_consumes_shared_collaboration_control_vocabulary: true,
        paste_secret_guard_consumes_shared_collaboration_control_vocabulary: true,
        collaboration_retention_sheet_consumes_shared_collaboration_control_vocabulary: true,
        session_restore_view_consumes_shared_collaboration_control_vocabulary: true,
        support_export_packet_consumes_shared_collaboration_control_vocabulary: true,
        help_docs_consumes_shared_collaboration_control_vocabulary: true,
        every_object_adopted_by_two_or_more_consumers: true,
        collaboration_control_vocabulary_identical_for_same_subject: true,
        narrowing_disclosed_not_hidden: true,
        export_maps_back_to_one_collaboration_control_object: true,
        deferred_intent_and_outbox_systems_blocked_from_queueing_sensitive_control_actions: true,
    }
}

fn proof_freshness() -> M5CollaborationControlSharedConsumersProofFreshness {
    M5CollaborationControlSharedConsumersProofFreshness {
        proof_freshness_slo_hours: M5_COLLABORATION_CONTROL_SHARED_CONSUMERS_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_COLLABORATION_CONTROL_SHARED_CONSUMERS_SCHEMA_REF.to_owned(),
        M5_COLLABORATION_CONTROL_SHARED_CONSUMERS_DOC_REF.to_owned(),
        M5_COLLABORATION_CONTROL_MATRIX_SCHEMA_REF.to_owned(),
        M5_COLLABORATION_CONTROL_MATRIX_DOC_REF.to_owned(),
    ];
    // The six objects each map to their own canonical domain schema; include each distinct one once.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for object in M5CollaborationControlObject::ALL {
        domains.insert(object.canonical_domain_schema_ref());
    }
    for domain in domains {
        refs.push(domain.to_owned());
    }
    refs
}

fn packet_from_bindings(
    packet_id: &str,
    surface_label: &str,
    consumer_bindings: Vec<CollaborationControlSharedConsumerBinding>,
) -> M5CollaborationControlSharedConsumersPacket {
    M5CollaborationControlSharedConsumersPacket::new(
        M5CollaborationControlSharedConsumersPacketInput {
            packet_id: packet_id.to_owned(),
            surface_label: surface_label.to_owned(),
            consumer_bindings,
            downgrade_triggers: M5CollaborationControlSharedConsumersDowngradeTrigger::ALL.to_vec(),
            consumer_surfaces: M5CollaborationControlConsumerSurface::ALL.to_vec(),
            trust_review: trust_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// The canonical, checked-in collaboration-control shared-consumer parity packet.
pub fn seeded_m5_collaboration_control_shared_consumers(
) -> M5CollaborationControlSharedConsumersPacket {
    packet_from_bindings(
        M5_COLLABORATION_CONTROL_SHARED_CONSUMERS_PACKET_ID,
        "M5 collaboration-control shared consumers (one vocabulary across surfaces)",
        build_bindings(|_, default| default),
    )
}

/// Fixture: the same subjects with two more desktop surfaces narrowed to compact and remote
/// representations, proving vocabulary survives compact and remote forms.
pub fn seeded_m5_collaboration_control_shared_consumers_compact_remote_narrowed(
) -> M5CollaborationControlSharedConsumersPacket {
    packet_from_bindings(
        "m5-collaboration-control-shared-consumers:compact-remote:0001",
        "M5 collaboration-control shared consumers (compact / remote narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "ccsc-presenter-handoff" => CollaborationControlSharedRepresentation::CompactNarrowed,
            "ccsc-restore-view" => CollaborationControlSharedRepresentation::RemoteProjected,
            _ => default,
        }),
    )
}

/// Fixture: the same subjects with two more surfaces narrowed to exported, export-safe
/// representations, proving vocabulary survives into exported forms.
pub fn seeded_m5_collaboration_control_shared_consumers_exported_redaction_narrowed(
) -> M5CollaborationControlSharedConsumersPacket {
    packet_from_bindings(
        "m5-collaboration-control-shared-consumers:exported-redaction:0001",
        "M5 collaboration-control shared consumers (exported redaction narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "ccsc-sterm-view" => CollaborationControlSharedRepresentation::ExportedRedacted,
            "ccsc-consent-join" => CollaborationControlSharedRepresentation::ExportedRedacted,
            _ => default,
        }),
    )
}
