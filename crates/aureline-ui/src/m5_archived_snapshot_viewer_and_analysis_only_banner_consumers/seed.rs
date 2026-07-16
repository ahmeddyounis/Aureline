//! Canonical seed for the archived-snapshot-viewer consumer packet.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export, matrix CSV,
//! Markdown summary, and narrowed fixtures. Every binding is derived from one per-profile
//! [`ArchiveViewerBannerGrammar`] so the same preserved-evidence profile always carries the same banner
//! grammar across surfaces, and every narrowed posture derives its disclosure and action set from
//! [`resolve_archive_view_render_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-16T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

/// The full accessibility route set every archived view offers so the non-live state, provenance, and
/// open-live-target action are discoverable without pointer-only chrome.
fn all_accessibility_routes() -> Vec<M5HistoricalReferenceAccessibilityRoute> {
    M5HistoricalReferenceAccessibilityRoute::ALL.to_vec()
}

#[allow(clippy::too_many_arguments)]
fn grammar(
    banner_role: &str,
    snapshot_label: &str,
    capture_time: &str,
    provenance: &str,
    analysis_only_posture: &str,
    allowed_action_set: &str,
) -> ArchiveViewerBannerGrammar {
    ArchiveViewerBannerGrammar {
        banner_role_word: banner_role.to_owned(),
        snapshot_label_word: snapshot_label.to_owned(),
        capture_time_word: capture_time.to_owned(),
        provenance_word: provenance.to_owned(),
        analysis_only_posture_word: analysis_only_posture.to_owned(),
        allowed_action_set_word: allowed_action_set.to_owned(),
    }
}

fn preserved_note_for(reason: ArchiveNarrowReason) -> String {
    match reason {
        ArchiveNarrowReason::LiveTargetRemovedMetadataOnly => {
            "banner-role, snapshot-label, capture-time, provenance, analysis-only-posture, and allowed-action-set words preserved; the live target was removed so only a metadata-only exit remains"
        }
        ArchiveNarrowReason::ImportedOfflineDisclosed => {
            "all banner grammar preserved; the evidence is imported / offline only and is not current live route, service, or workspace truth"
        }
        ArchiveNarrowReason::ExportRedactionNarrowed => {
            "all banner grammar preserved; only surrounding detail is redacted export-safe"
        }
    }
    .to_owned()
}

fn next_action_label_for(action: ArchiveNarrowNextAction) -> String {
    match action {
        ArchiveNarrowNextAction::OpenMetadataOnlyExit => "Open the metadata-only inspection exit",
        ArchiveNarrowNextAction::OpenImportSource => "Open the import / offline source",
        ArchiveNarrowNextAction::OpenFullDetail => "Open the full detail",
    }
    .to_owned()
}

fn allowed_actions_for(disclosure: ArchiveViewRenderDisclosure) -> Vec<ArchiveAction> {
    let mut actions = ArchiveAction::ANALYSIS_ONLY_BASE.to_vec();
    if disclosure.offers_open_live_target {
        actions.push(ArchiveAction::OpenCurrentLiveObject);
    }
    actions
}

fn binding_refs(object_class: M5HistoricalReferenceObject) -> Vec<String> {
    vec![
        M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF.to_owned(),
        object_class.canonical_domain_schema_ref().to_owned(),
    ]
}

#[allow(clippy::too_many_arguments)]
fn make_binding(
    binding_id: &str,
    evidence_profile_id: &str,
    evidence_profile_label: &str,
    object_class: M5HistoricalReferenceObject,
    consumer: M5HistoricalReferenceConsumerSurface,
    posture: ArchiveViewPosture,
    banner_grammar: ArchiveViewerBannerGrammar,
) -> ArchiveViewerConsumerBinding {
    let disclosure = resolve_archive_view_render_disclosure(posture);
    let narrow_note = disclosure.narrow_reason.map(|reason| {
        let next_action = disclosure
            .narrow_next_action
            .expect("a narrow reason always carries a next action");
        ArchiveNarrowNote {
            reason,
            preserved_grammar_note: preserved_note_for(reason),
            next_action,
            next_action_label: next_action_label_for(next_action),
        }
    });
    let import_offline_note = if disclosure.needs_import_offline_note {
        "imported / offline evidence only; not current live route, service, or workspace truth"
            .to_owned()
    } else {
        String::new()
    };
    let export_detail_note = if disclosure.needs_export_detail_note {
        "surrounding detail redacted export-safe in this packet; full detail available on request"
            .to_owned()
    } else {
        String::new()
    };

    ArchiveViewerConsumerBinding {
        binding_id: binding_id.to_owned(),
        evidence_profile_id: evidence_profile_id.to_owned(),
        evidence_profile_label: evidence_profile_label.to_owned(),
        object_class,
        consumer,
        posture,
        banner_grammar,
        parity_state: disclosure.parity_state,
        allowed_actions: allowed_actions_for(disclosure),
        accessibility_routes: all_accessibility_routes(),
        narrow_note,
        import_offline_note,
        export_detail_note,
        presents_write_capable_control_as_if_current_object_open_live: false,
        reopens_live_target_without_validating_identity_trust_route_and_authority: false,
        dead_links_expired_or_removed_artifact_instead_of_showing_metadata: false,
        leaves_non_live_evidence_unjoined_to_capture_context: false,
        lets_archived_or_imported_evidence_look_live_writable_or_current_by_omission: false,
        source_contract_refs: binding_refs(object_class),
    }
}

/// One consumer-surface adoption of a preserved-evidence profile, before any posture override.
struct BindingSpec {
    binding_id: &'static str,
    consumer: M5HistoricalReferenceConsumerSurface,
    posture: ArchiveViewPosture,
}

/// One preserved-evidence profile rendered across several consumer surfaces at one banner grammar.
struct ProfileSpec {
    profile_id: &'static str,
    profile_label: &'static str,
    object_class: M5HistoricalReferenceObject,
    grammar: ArchiveViewerBannerGrammar,
    bindings: Vec<BindingSpec>,
}

fn spec(
    profile_id: &'static str,
    profile_label: &'static str,
    object_class: M5HistoricalReferenceObject,
    grammar: ArchiveViewerBannerGrammar,
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
    posture: ArchiveViewPosture,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        posture,
    }
}

/// The five preserved-evidence profiles — one per B149 historical-reference object class — and the surfaces
/// that adopt each, drawn from the shell / archive-viewer, help / docs, support, review / incident,
/// runbook-archive, release-center, companion / export, program-governance, and CLI / export consumers.
fn profile_specs() -> Vec<ProfileSpec> {
    use ArchiveViewPosture::*;
    use M5HistoricalReferenceConsumerSurface::*;
    use M5HistoricalReferenceObject::*;

    let read_only_posture = "read_only_non_authoritative_for_mutation";
    let full_action_set = "inspect_compare_export_open_live";
    let analysis_only_action_set = "inspect_compare_export";

    vec![
        spec(
            "retirement-snapshot/last-supported-archive",
            "Retirement / last-supported snapshot (analysis-only archive)",
            RetirementSnapshot,
            grammar(
                "snapshot_labeling",
                "retirement_last_supported_snapshot",
                "retirement_capture_time",
                "last_supported_build_provenance",
                read_only_posture,
                full_action_set,
            ),
            vec![
                bs("asvc-retirement-release", ReleaseCenter, LiveTargetOpenable),
                bs("asvc-retirement-shell", Shell, LiveTargetOpenable),
                bs("asvc-retirement-cli", CliExport, ExportedRedacted),
            ],
        ),
        spec(
            "support-export-evidence/captured-bundle",
            "Captured support / export evidence (analysis-only bundle viewer)",
            SupportExportEvidence,
            grammar(
                "provenance_attribution",
                "captured_support_export_evidence",
                "evidence_capture_time",
                "support_bundle_capture_context",
                read_only_posture,
                full_action_set,
            ),
            vec![
                bs("asvc-support-evidence-support", Support, ExportedRedacted),
                bs("asvc-support-evidence-help", HelpDocs, LiveTargetOpenable),
                bs(
                    "asvc-support-evidence-companion",
                    CompanionExport,
                    ExportedRedacted,
                ),
            ],
        ),
        spec(
            "archived-runbook-packet/historical-run",
            "Archived runbook execution packet (historical run, validated reopen)",
            ArchivedRunbookPacket,
            grammar(
                "live_target_handoff",
                "archived_runbook_execution_packet",
                "run_capture_time",
                "runbook_run_provenance",
                read_only_posture,
                full_action_set,
            ),
            vec![
                bs("asvc-runbook-archive", RunbookArchive, LiveTargetOpenable),
                bs("asvc-runbook-review", ReviewIncident, MetadataOnlyExit),
                bs("asvc-runbook-program", ProgramGovernance, ExportedRedacted),
            ],
        ),
        spec(
            "imported-offline-route-evidence/offline-only",
            "Imported / offline route evidence (offline-only, not live truth)",
            ImportedOfflineRouteEvidence,
            grammar(
                "imported_offline_disclosure",
                "imported_offline_route_evidence",
                "import_capture_time",
                "import_offline_source_provenance",
                read_only_posture,
                analysis_only_action_set,
            ),
            vec![
                bs("asvc-imported-shell", Shell, ImportedOfflineOnly),
                bs("asvc-imported-runbook", RunbookArchive, ImportedOfflineOnly),
                bs("asvc-imported-cli", CliExport, ExportedRedacted),
            ],
        ),
        spec(
            "review-incident-snapshot/evidence-reopen",
            "Review / incident snapshot (analysis-only evidence reopen flow)",
            ReviewIncidentSnapshot,
            grammar(
                "mutation_blocked_posture",
                "review_incident_snapshot",
                "incident_capture_time",
                "incident_evidence_provenance",
                read_only_posture,
                full_action_set,
            ),
            vec![
                bs("asvc-review-review", ReviewIncident, LiveTargetOpenable),
                bs("asvc-review-shell", Shell, MetadataOnlyExit),
                bs("asvc-review-companion", CompanionExport, ExportedRedacted),
            ],
        ),
    ]
}

/// Builds all consumer bindings, applying `posture_override` to override a binding's posture.
fn build_bindings<F>(posture_override: F) -> Vec<ArchiveViewerConsumerBinding>
where
    F: Fn(&str, ArchiveViewPosture) -> ArchiveViewPosture,
{
    let mut bindings = Vec::new();
    for profile in profile_specs() {
        for spec in &profile.bindings {
            let posture = posture_override(spec.binding_id, spec.posture);
            bindings.push(make_binding(
                spec.binding_id,
                profile.profile_id,
                profile.profile_label,
                profile.object_class,
                spec.consumer,
                posture,
                profile.grammar.clone(),
            ));
        }
    }
    bindings
}

fn trust_review() -> ArchivedSnapshotViewerConsumersTrustReview {
    ArchivedSnapshotViewerConsumersTrustReview {
        object_class_reuse_proven_by_fixtures: true,
        same_profile_same_banner_across_surfaces: true,
        banner_role_words_stay_in_frozen_vocabulary: true,
        analysis_only_posture_never_masquerades_as_live: true,
        write_controls_never_shown_as_current_object_open_live: true,
        open_live_target_always_validates_identity_trust_route_authority: true,
        expired_or_removed_artifacts_show_metadata_not_dead_links: true,
        non_live_evidence_always_joined_to_capture_context: true,
        archived_evidence_never_looks_live_by_omission: true,
        accessibility_routes_present_for_state_provenance_and_open_live_target: true,
        narrowing_disclosed_across_postures: true,
        support_export_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> ArchivedSnapshotViewerConsumersProjection {
    ArchivedSnapshotViewerConsumersProjection {
        shell_consumes_archive_banner: true,
        help_docs_consumes_archive_banner: true,
        support_consumes_archive_banner: true,
        review_incident_consumes_archive_banner: true,
        runbook_archive_consumes_archive_banner: true,
        release_center_consumes_archive_banner: true,
        companion_export_consumes_archive_banner: true,
        program_governance_consumes_archive_banner: true,
        cli_export_consumes_archive_banner: true,
        every_object_class_adopted_by_two_or_more_consumers: true,
        banner_grammar_identical_for_same_profile: true,
        narrowing_disclosed_not_hidden: true,
        export_maps_back_to_one_historical_reference_object: true,
    }
}

fn proof_freshness() -> ArchivedSnapshotViewerConsumersProofFreshness {
    ArchivedSnapshotViewerConsumersProofFreshness {
        proof_freshness_slo_hours: M5_ARCHIVED_SNAPSHOT_VIEWER_CONSUMERS_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_ARCHIVED_SNAPSHOT_VIEWER_CONSUMERS_SCHEMA_REF.to_owned(),
        M5_ARCHIVED_SNAPSHOT_VIEWER_CONSUMERS_DOC_REF.to_owned(),
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
    consumer_bindings: Vec<ArchiveViewerConsumerBinding>,
) -> M5ArchivedSnapshotViewerConsumersPacket {
    M5ArchivedSnapshotViewerConsumersPacket::new(M5ArchivedSnapshotViewerConsumersPacketInput {
        packet_id: packet_id.to_owned(),
        surface_label: surface_label.to_owned(),
        consumer_bindings,
        downgrade_triggers: ArchivedSnapshotViewerConsumersDowngradeTrigger::ALL.to_vec(),
        consumer_surfaces: M5HistoricalReferenceConsumerSurface::ALL.to_vec(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// The canonical, checked-in archived-snapshot-viewer consumer packet.
pub fn seeded_m5_archived_snapshot_viewer_consumers() -> M5ArchivedSnapshotViewerConsumersPacket {
    packet_from_bindings(
        M5_ARCHIVED_SNAPSHOT_VIEWER_CONSUMERS_PACKET_ID,
        "M5 archived-snapshot viewers & analysis-only banners (one vocabulary across surfaces)",
        build_bindings(|_, default| default),
    )
}

/// Fixture: the same profiles with two more live-target-openable surfaces narrowed to a metadata-only exit,
/// proving the banner grammar survives when the live target is removed.
pub fn seeded_m5_archived_snapshot_viewer_consumers_metadata_only_narrowed(
) -> M5ArchivedSnapshotViewerConsumersPacket {
    packet_from_bindings(
        "m5-archived-snapshot-viewer-consumers:metadata-only:0001",
        "M5 archived-snapshot viewers (metadata-only exit narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "asvc-retirement-shell" => ArchiveViewPosture::MetadataOnlyExit,
            "asvc-support-evidence-help" => ArchiveViewPosture::MetadataOnlyExit,
            _ => default,
        }),
    )
}

/// Fixture: the same profiles with two more surfaces narrowed to imported / offline-only views, proving the
/// banner grammar survives into imported / offline forms.
pub fn seeded_m5_archived_snapshot_viewer_consumers_imported_offline_narrowed(
) -> M5ArchivedSnapshotViewerConsumersPacket {
    packet_from_bindings(
        "m5-archived-snapshot-viewer-consumers:imported-offline:0001",
        "M5 archived-snapshot viewers (imported / offline-only narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "asvc-runbook-archive" => ArchiveViewPosture::ImportedOfflineOnly,
            "asvc-review-review" => ArchiveViewPosture::ImportedOfflineOnly,
            _ => default,
        }),
    )
}
