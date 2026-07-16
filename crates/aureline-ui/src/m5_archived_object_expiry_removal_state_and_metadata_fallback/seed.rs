//! Canonical seed for the archived-evidence-state packet.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export, matrix CSV, Markdown
//! summary, and narrowed fixtures. Every binding is derived from one per-profile
//! [`ArchiveStateHistoricalGrammar`] so the same preserved-object profile always carries the same historical
//! grammar across surfaces, and every state derives its parity, removal / expiry note, content-presence flag,
//! and action set from [`resolve_archive_state_render_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-16T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

/// The full accessibility route set every archived-state surface offers so the archived state, provenance, and
/// removal / expiry reason are discoverable without pointer-only chrome.
fn all_accessibility_routes() -> Vec<M5HistoricalReferenceAccessibilityRoute> {
    M5HistoricalReferenceAccessibilityRoute::ALL.to_vec()
}

fn grammar(
    historical_role: &str,
    snapshot_label: &str,
    capture_time: &str,
    provenance: &str,
    mutation_blocked_posture: &str,
) -> ArchiveStateHistoricalGrammar {
    ArchiveStateHistoricalGrammar {
        historical_role_word: historical_role.to_owned(),
        snapshot_label_word: snapshot_label.to_owned(),
        capture_time_word: capture_time.to_owned(),
        provenance_word: provenance.to_owned(),
        mutation_blocked_posture_word: mutation_blocked_posture.to_owned(),
    }
}

/// Stable position of an object class in the frozen declaration order, used to spread removal-reason choices
/// deterministically across profiles without any per-binding hand-authoring.
fn object_class_index(object_class: M5HistoricalReferenceObject) -> usize {
    M5HistoricalReferenceObject::ALL
        .iter()
        .position(|candidate| *candidate == object_class)
        .expect("object class is a member of ALL")
}

/// The removal / expiry reason a disclosing binding names, chosen deterministically from the state's allowed
/// set. An available archive names none.
fn removal_reason_for(
    state: ArchivedEvidenceState,
    object_class: M5HistoricalReferenceObject,
) -> Option<RemovalExpiryReason> {
    let allowed = state.allowed_removal_reasons();
    if allowed.is_empty() {
        None
    } else {
        Some(allowed[object_class_index(object_class) % allowed.len()])
    }
}

fn removal_attribution_for(object_class: M5HistoricalReferenceObject) -> RemovalAttribution {
    RemovalAttribution {
        retention_or_deletion_receipt_ref: format!("retention-receipt-{}", object_class.as_str()),
        retirement_closure_ledger_ref: format!(
            "retirement-closure-ledger-{}",
            object_class.as_str()
        ),
        support_packet_manifest_ref: format!("support-packet-manifest-{}", object_class.as_str()),
    }
}

fn explanation_for(reason: RemovalExpiryReason) -> String {
    match reason {
        RemovalExpiryReason::RetentionWindowElapsed => {
            "the retention / validity window elapsed; the metadata, provenance, and capture time stay presented and the object may be safely cleaned up"
        }
        RemovalExpiryReason::ManualCleanupRequested => {
            "a reviewed manual cleanup removed the content bytes; the metadata and deletion receipt stay presented instead of a dead link"
        }
        RemovalExpiryReason::PolicyMandatedDeletion => {
            "a policy mandated deletion of the content; the metadata, provenance, and deletion receipt stay presented instead of a dead link"
        }
        RemovalExpiryReason::SourceLiveTargetRemoved => {
            "the source live object was removed, so no live counterpart remains; the archived metadata and provenance stay presented"
        }
        RemovalExpiryReason::StorageReclaimed => {
            "storage was reclaimed while the metadata and receipt were retained; the object presents metadata rather than a blank pane"
        }
        RemovalExpiryReason::LegalHoldReleased => {
            "a legal hold was released and the content was cleaned up; the metadata and receipt stay presented instead of a dead link"
        }
        RemovalExpiryReason::MetadataOnlyByDesign => {
            "only metadata was ever retained for this object by design; it presents metadata and provenance rather than a blank pane"
        }
    }
    .to_owned()
}

fn next_action_label_for(action: RemovalExpiryNextAction) -> String {
    match action {
        RemovalExpiryNextAction::RemoveThroughReviewedCleanup => {
            "Remove the archived object through its reviewed cleanup path"
        }
        RemovalExpiryNextAction::InspectMetadataOnly => {
            "Inspect the archived metadata (no removable content remains)"
        }
    }
    .to_owned()
}

fn preserved_metadata_note() -> String {
    "snapshot-label, capture-time, provenance, and mutation-blocked-posture words preserved; the object presents metadata and its removal / expiry reason instead of a dead link"
        .to_owned()
}

fn allowed_actions_for(disclosure: ArchiveStateRenderDisclosure) -> Vec<ArchiveStateAction> {
    let mut actions = ArchiveStateAction::BASE.to_vec();
    if disclosure.offers_remove_action {
        actions.push(ArchiveStateAction::RemoveArchivedObject);
    }
    if disclosure.offers_open_live_target {
        actions.push(ArchiveStateAction::OpenCurrentLiveObject);
    }
    actions
}

fn binding_refs(object_class: M5HistoricalReferenceObject) -> Vec<String> {
    vec![
        M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF.to_owned(),
        object_class.canonical_domain_schema_ref().to_owned(),
    ]
}

fn make_binding(
    binding_id: &str,
    snapshot_profile_id: &str,
    snapshot_profile_label: &str,
    object_class: M5HistoricalReferenceObject,
    consumer: M5HistoricalReferenceConsumerSurface,
    state: ArchivedEvidenceState,
    historical_grammar: ArchiveStateHistoricalGrammar,
) -> ArchivedEvidenceStateBinding {
    let disclosure = resolve_archive_state_render_disclosure(state);
    let reason = removal_reason_for(state, object_class);

    let removal_note = if disclosure.needs_removal_note {
        let reason =
            reason.expect("a removal / expiry state always has at least one allowed reason");
        let next_action = disclosure
            .removal_next_action
            .expect("a removal / expiry state always carries a next action");
        Some(RemovalExpiryNote {
            reason,
            explanation: explanation_for(reason),
            preserved_metadata_note: preserved_metadata_note(),
            removal_attribution: removal_attribution_for(object_class),
            next_action,
            next_action_label: next_action_label_for(next_action),
        })
    } else {
        None
    };

    ArchivedEvidenceStateBinding {
        binding_id: binding_id.to_owned(),
        snapshot_profile_id: snapshot_profile_id.to_owned(),
        snapshot_profile_label: snapshot_profile_label.to_owned(),
        object_class,
        consumer,
        state,
        state_label: state.stable_label().to_owned(),
        historical_grammar,
        content_bytes_present: disclosure.expects_content_bytes_present,
        parity_state: disclosure.parity_state,
        allowed_actions: allowed_actions_for(disclosure),
        accessibility_routes: all_accessibility_routes(),
        removal_note,
        historical_side_mutation_blocked: true,
        reopens_live_target_without_validating_identity_trust_route_and_authority: false,
        degrades_to_generic_dead_link: false,
        removes_content_without_attribution: false,
        presents_expired_or_removed_as_live_or_current: false,
        drops_removal_or_expiry_vocabulary_in_export: false,
        source_contract_refs: binding_refs(object_class),
    }
}

/// One consumer-surface adoption of a preserved-object profile, before any state override.
struct BindingSpec {
    binding_id: &'static str,
    consumer: M5HistoricalReferenceConsumerSurface,
    state: ArchivedEvidenceState,
}

/// One preserved-object profile stated across several consumer surfaces at one historical grammar.
struct ProfileSpec {
    profile_id: &'static str,
    profile_label: &'static str,
    object_class: M5HistoricalReferenceObject,
    grammar: ArchiveStateHistoricalGrammar,
    bindings: Vec<BindingSpec>,
}

fn spec(
    profile_id: &'static str,
    profile_label: &'static str,
    object_class: M5HistoricalReferenceObject,
    grammar: ArchiveStateHistoricalGrammar,
    bindings: Vec<BindingSpec>,
) -> ProfileSpec {
    ProfileSpec {
        profile_id,
        profile_label,
        object_class,
        grammar,
        bindings,
    }
}

fn bs(
    binding_id: &'static str,
    consumer: M5HistoricalReferenceConsumerSurface,
    state: ArchivedEvidenceState,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        state,
    }
}

/// The five preserved-object profiles — one per B149 historical-reference object class — and the surfaces that
/// state each across its lifecycle, drawn from the shell / archive-viewer, help / docs, support,
/// review / incident, runbook-archive, release-center, companion / export, program-governance, and CLI / export
/// consumers.
fn profile_specs() -> Vec<ProfileSpec> {
    use ArchivedEvidenceState::*;
    use M5HistoricalReferenceConsumerSurface::*;
    use M5HistoricalReferenceObject::*;

    let read_only_posture = "read_only_non_authoritative_for_mutation";

    vec![
        spec(
            "retirement-snapshot/last-supported-archive",
            "Retirement / last-supported snapshot (expiry / removal state)",
            RetirementSnapshot,
            grammar(
                "expiry_removal_handling",
                "retirement_last_supported_snapshot",
                "retirement_capture_time",
                "last_supported_build_provenance",
                read_only_posture,
            ),
            vec![
                bs("aes-retirement-release", ReleaseCenter, PreservedAvailable),
                bs("aes-retirement-shell", Shell, Expired),
                bs("aes-retirement-cli", CliExport, MissingLiveTarget),
            ],
        ),
        spec(
            "support-export-evidence/captured-bundle",
            "Captured support / export evidence (expiry / removal state)",
            SupportExportEvidence,
            grammar(
                "provenance_attribution",
                "captured_support_export_evidence",
                "evidence_capture_time",
                "support_bundle_capture_context",
                read_only_posture,
            ),
            vec![
                bs("aes-support-evidence-support", Support, PreservedAvailable),
                bs("aes-support-evidence-help", HelpDocs, Removed),
                bs(
                    "aes-support-evidence-companion",
                    CompanionExport,
                    MetadataOnly,
                ),
            ],
        ),
        spec(
            "archived-runbook-packet/historical-run",
            "Archived runbook execution packet (expiry / removal state)",
            ArchivedRunbookPacket,
            grammar(
                "snapshot_labeling",
                "archived_runbook_execution_packet",
                "run_capture_time",
                "runbook_run_provenance",
                read_only_posture,
            ),
            vec![
                bs("aes-runbook-archive", RunbookArchive, RetentionWindowEnded),
                bs("aes-runbook-review", ReviewIncident, Removed),
                bs("aes-runbook-program", ProgramGovernance, PreservedAvailable),
            ],
        ),
        spec(
            "imported-offline-route-evidence/offline-only",
            "Imported / offline route evidence (expiry / removal state)",
            ImportedOfflineRouteEvidence,
            grammar(
                "imported_offline_disclosure",
                "imported_offline_route_evidence",
                "import_capture_time",
                "import_offline_source_provenance",
                read_only_posture,
            ),
            vec![
                bs("aes-imported-shell", Shell, Expired),
                bs("aes-imported-runbook", RunbookArchive, MetadataOnly),
                bs("aes-imported-cli", CliExport, MissingLiveTarget),
            ],
        ),
        spec(
            "review-incident-snapshot/evidence-archive",
            "Review / incident snapshot (expiry / removal state)",
            ReviewIncidentSnapshot,
            grammar(
                "mutation_blocked_posture",
                "review_incident_snapshot",
                "incident_capture_time",
                "incident_evidence_provenance",
                read_only_posture,
            ),
            vec![
                bs("aes-review-review", ReviewIncident, PreservedAvailable),
                bs("aes-review-shell", Shell, RetentionWindowEnded),
                bs("aes-review-companion", CompanionExport, Removed),
            ],
        ),
    ]
}

/// Builds all archived-state bindings, applying `state_override` to override a binding's state.
fn build_bindings<F>(state_override: F) -> Vec<ArchivedEvidenceStateBinding>
where
    F: Fn(&str, ArchivedEvidenceState) -> ArchivedEvidenceState,
{
    let mut bindings = Vec::new();
    for profile in profile_specs() {
        for spec in &profile.bindings {
            let state = state_override(spec.binding_id, spec.state);
            bindings.push(make_binding(
                spec.binding_id,
                profile.profile_id,
                profile.profile_label,
                profile.object_class,
                spec.consumer,
                state,
                profile.grammar.clone(),
            ));
        }
    }
    bindings
}

fn trust_review() -> ArchivedEvidenceStateTrustReview {
    ArchivedEvidenceStateTrustReview {
        object_class_reuse_proven_by_fixtures: true,
        same_profile_same_historical_grammar_across_surfaces: true,
        historical_role_words_stay_in_frozen_vocabulary: true,
        mutation_blocked_posture_never_masquerades_as_live: true,
        every_non_available_state_carries_removal_or_expiry_explanation: true,
        metadata_provenance_and_reason_render_instead_of_dead_link: true,
        removal_outcomes_joined_to_receipts_ledgers_and_manifests: true,
        remove_action_offered_only_where_appropriate: true,
        expired_or_removed_never_presented_as_live_or_current: true,
        stable_state_labels_used_across_surfaces: true,
        accessibility_routes_present_for_state_provenance_and_reason: true,
        state_disclosed_across_all_lifecycle_states: true,
        support_export_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> ArchivedEvidenceStateProjection {
    ArchivedEvidenceStateProjection {
        shell_consumes_state: true,
        help_docs_consumes_state: true,
        support_consumes_state: true,
        review_incident_consumes_state: true,
        runbook_archive_consumes_state: true,
        release_center_consumes_state: true,
        companion_export_consumes_state: true,
        program_governance_consumes_state: true,
        cli_export_consumes_state: true,
        every_object_class_stated_by_two_or_more_consumers: true,
        historical_grammar_identical_for_same_profile: true,
        removal_or_expiry_disclosed_not_hidden: true,
        state_maps_back_to_one_historical_reference_object: true,
    }
}

fn proof_freshness() -> ArchivedEvidenceStateProofFreshness {
    ArchivedEvidenceStateProofFreshness {
        proof_freshness_slo_hours: M5_ARCHIVED_EVIDENCE_STATE_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_ARCHIVED_EVIDENCE_STATE_SCHEMA_REF.to_owned(),
        M5_ARCHIVED_EVIDENCE_STATE_DOC_REF.to_owned(),
        M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF.to_owned(),
        M5_HISTORICAL_REFERENCE_MATRIX_DOC_REF.to_owned(),
    ];
    // The five object classes map to three canonical domain schemas; include each distinct one once.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for object_class in M5HistoricalReferenceObject::ALL {
        domains.insert(object_class.canonical_domain_schema_ref());
    }
    for domain in domains {
        refs.push(domain.to_owned());
    }
    refs
}

fn packet_from_bindings(
    packet_id: &str,
    surface_label: &str,
    state_bindings: Vec<ArchivedEvidenceStateBinding>,
) -> M5ArchivedEvidenceStatePacket {
    M5ArchivedEvidenceStatePacket::new(M5ArchivedEvidenceStatePacketInput {
        packet_id: packet_id.to_owned(),
        surface_label: surface_label.to_owned(),
        state_bindings,
        downgrade_triggers: ArchivedEvidenceStateDowngradeTrigger::ALL.to_vec(),
        consumer_surfaces: M5HistoricalReferenceConsumerSurface::ALL.to_vec(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// The canonical, checked-in archived-evidence-state packet.
pub fn seeded_m5_archived_evidence_state() -> M5ArchivedEvidenceStatePacket {
    packet_from_bindings(
        M5_ARCHIVED_EVIDENCE_STATE_PACKET_ID,
        "M5 archived-object expiry / removal state (one vocabulary across surfaces)",
        build_bindings(|_, default| default),
    )
}

/// Fixture: two more preserved-available archives narrowed to an Expired state, proving the historical grammar
/// survives, the object stays cleanly labelled, and a reviewed remove action replaces the open-live affordance.
pub fn seeded_m5_archived_evidence_state_expired_narrowed() -> M5ArchivedEvidenceStatePacket {
    packet_from_bindings(
        "m5-archived-evidence-state:expired:0001",
        "M5 archived-object expiry / removal state (expired narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "aes-support-evidence-support" => ArchivedEvidenceState::Expired,
            "aes-runbook-program" => ArchivedEvidenceState::Expired,
            _ => default,
        }),
    )
}

/// Fixture: two more preserved-available archives narrowed to a Removed state, proving the metadata / provenance
/// and removal reason render instead of a dead link once the content bytes are gone.
pub fn seeded_m5_archived_evidence_state_removed_narrowed() -> M5ArchivedEvidenceStatePacket {
    packet_from_bindings(
        "m5-archived-evidence-state:removed:0001",
        "M5 archived-object expiry / removal state (removed narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "aes-review-review" => ArchivedEvidenceState::Removed,
            "aes-retirement-release" => ArchivedEvidenceState::Removed,
            _ => default,
        }),
    )
}
