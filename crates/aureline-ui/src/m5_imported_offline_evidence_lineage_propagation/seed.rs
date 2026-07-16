//! Canonical seed for the imported / offline lineage packet.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export, matrix CSV, Markdown
//! summary, and narrowed fixtures. Every binding is derived from one per-profile [`NonLiveEvidenceGrammar`] so the
//! same imported / offline evidence profile always carries the same non-live grammar across surfaces, and every
//! disposition derives its parity, lineage descriptor, content-available flag, and action set from
//! [`resolve_lineage_disposition_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-16T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

/// The full accessibility route set every lineage consumer offers so the non-live boundary, provenance, and
/// lineage join are discoverable without pointer-only chrome.
fn all_accessibility_routes() -> Vec<M5HistoricalReferenceAccessibilityRoute> {
    M5HistoricalReferenceAccessibilityRoute::ALL.to_vec()
}

fn grammar(
    historical_role: &str,
    snapshot_label: &str,
    capture_time: &str,
    provenance: &str,
    mutation_blocked_posture: &str,
) -> NonLiveEvidenceGrammar {
    NonLiveEvidenceGrammar {
        historical_role_word: historical_role.to_owned(),
        snapshot_label_word: snapshot_label.to_owned(),
        capture_time_word: capture_time.to_owned(),
        provenance_word: provenance.to_owned(),
        mutation_blocked_posture_word: mutation_blocked_posture.to_owned(),
        imported_offline_label_word: M5_IMPORTED_OFFLINE_LABEL.to_owned(),
    }
}

fn lineage_join_for(object_class: M5HistoricalReferenceObject) -> LineageJoin {
    LineageJoin {
        source_capture_context_ref: format!("capture-context-{}", object_class.as_str()),
        producer_build_ref: format!("producer-build-{}", object_class.as_str()),
        provenance_lineage_ref: format!("provenance-lineage-{}", object_class.as_str()),
    }
}

fn source_descriptor_ref_for(object_class: M5HistoricalReferenceObject) -> String {
    format!("snapshot-descriptor-{}", object_class.as_str())
}

fn next_action_label_for(action: LineageNextAction) -> String {
    match action {
        LineageNextAction::OpenCurrentLiveObjectThroughValidatedHandoff => {
            "Open the current live object through its joined, validated live-target handoff"
        }
        LineageNextAction::InspectLineageMetadataOnly => {
            "Inspect the imported / offline lineage metadata (no live counterpart remains)"
        }
    }
    .to_owned()
}

fn non_live_boundary_note_for(disposition: EvidenceLineageDisposition) -> String {
    match disposition {
        EvidenceLineageDisposition::LiveTargetJoinable => {
            "Showing imported or offline evidence; not current live route, provider, or service truth. A validated live-target handoff is joined for an explicit open-current-live-object exit."
        }
        EvidenceLineageDisposition::ImportedOfflineOnly => {
            "Showing imported or offline evidence; not current live route, provider, or service truth. No live counterpart remains, so only a metadata-only exit is offered."
        }
        EvidenceLineageDisposition::MetadataOnlyExit => {
            "Showing imported or offline evidence; not current live route, provider, or service truth. The content is unavailable, so metadata, provenance, and this boundary render instead of a dead link."
        }
        EvidenceLineageDisposition::ExportedRedactedLineage => {
            "Showing imported or offline evidence; not current live route, provider, or service truth. This lineage was carried into a redacted export and stays non-live with a metadata-only exit."
        }
    }
    .to_owned()
}

fn make_descriptor(
    object_class: M5HistoricalReferenceObject,
    disposition: EvidenceLineageDisposition,
    disclosure: LineageRenderDisclosure,
) -> LineageDescriptor {
    let live_target_handoff_ref = if disclosure.requires_live_target_handoff_ref {
        Some(format!("live-target-handoff-{}", object_class.as_str()))
    } else {
        None
    };
    let metadata_only_exit_ref = if disclosure.requires_metadata_only_exit_ref {
        Some(format!("metadata-only-exit-{}", object_class.as_str()))
    } else {
        None
    };
    LineageDescriptor {
        source_snapshot_descriptor_ref: source_descriptor_ref_for(object_class),
        lineage_join: lineage_join_for(object_class),
        live_target_handoff_ref,
        metadata_only_exit_ref,
        non_live_boundary_note: non_live_boundary_note_for(disposition),
        next_action: disclosure.next_action,
        next_action_label: next_action_label_for(disclosure.next_action),
    }
}

fn allowed_actions_for(disclosure: LineageRenderDisclosure) -> Vec<LineageConsumerAction> {
    let mut actions = LineageConsumerAction::BASE.to_vec();
    if disclosure.offers_open_live_target {
        actions.push(LineageConsumerAction::OpenCurrentLiveObject);
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
    lineage_profile_id: &str,
    lineage_profile_label: &str,
    object_class: M5HistoricalReferenceObject,
    consumer: M5HistoricalReferenceConsumerSurface,
    disposition: EvidenceLineageDisposition,
    non_live_grammar: NonLiveEvidenceGrammar,
) -> ImportedOfflineLineageBinding {
    let disclosure = resolve_lineage_disposition_disclosure(disposition);
    ImportedOfflineLineageBinding {
        binding_id: binding_id.to_owned(),
        lineage_profile_id: lineage_profile_id.to_owned(),
        lineage_profile_label: lineage_profile_label.to_owned(),
        object_class,
        consumer,
        disposition,
        disposition_label: disposition.stable_label().to_owned(),
        non_live_grammar,
        content_available: disclosure.expects_content_available,
        parity_state: disclosure.parity_state,
        allowed_actions: allowed_actions_for(disclosure),
        accessibility_routes: all_accessibility_routes(),
        lineage_descriptor: make_descriptor(object_class, disposition, disclosure),
        non_live_boundary_explicitly_called_out: true,
        ranked_or_narrated_as_current_live_service_truth: false,
        presents_imported_offline_as_current_route_or_provider_state: false,
        reopens_live_target_without_validating_identity_trust_route_and_authority: false,
        leaks_live_secret_or_stale_authority_through_lineage: false,
        drops_non_live_vocabulary_in_export: false,
        source_contract_refs: binding_refs(object_class),
    }
}

/// One consumer-surface adoption of an imported / offline evidence profile, before any disposition override.
struct BindingSpec {
    binding_id: &'static str,
    consumer: M5HistoricalReferenceConsumerSurface,
    disposition: EvidenceLineageDisposition,
}

/// One imported / offline evidence profile propagated across several consumer surfaces at one non-live grammar.
struct ProfileSpec {
    profile_id: &'static str,
    profile_label: &'static str,
    object_class: M5HistoricalReferenceObject,
    grammar: NonLiveEvidenceGrammar,
    bindings: Vec<BindingSpec>,
}

fn spec(
    profile_id: &'static str,
    profile_label: &'static str,
    object_class: M5HistoricalReferenceObject,
    grammar: NonLiveEvidenceGrammar,
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
    disposition: EvidenceLineageDisposition,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        disposition,
    }
}

/// The five imported / offline evidence profiles — one per B149 historical-reference object class — and the
/// downstream consumers that propagate each across its dispositions, drawn from the shell / archive-viewer,
/// help / docs, support, review / incident, runbook-archive, release-center, companion / export,
/// program-governance, and CLI / export consumers.
fn profile_specs() -> Vec<ProfileSpec> {
    use EvidenceLineageDisposition::*;
    use M5HistoricalReferenceConsumerSurface::*;
    use M5HistoricalReferenceObject::*;

    let read_only_posture = "read_only_non_authoritative_for_mutation";

    vec![
        spec(
            "retirement-snapshot/last-supported-archive",
            "Retirement / last-supported snapshot (imported / offline lineage)",
            RetirementSnapshot,
            grammar(
                "live_target_handoff",
                "retirement_last_supported_snapshot",
                "retirement_capture_time",
                "last_supported_build_provenance",
                read_only_posture,
            ),
            vec![
                bs("iol-retirement-release", ReleaseCenter, LiveTargetJoinable),
                bs("iol-retirement-shell", Shell, ImportedOfflineOnly),
                bs("iol-retirement-cli", CliExport, MetadataOnlyExit),
            ],
        ),
        spec(
            "support-export-evidence/captured-bundle",
            "Captured support / export evidence (imported / offline lineage)",
            SupportExportEvidence,
            grammar(
                "provenance_attribution",
                "captured_support_export_evidence",
                "evidence_capture_time",
                "support_bundle_capture_context",
                read_only_posture,
            ),
            vec![
                bs("iol-support-evidence-support", Support, LiveTargetJoinable),
                bs(
                    "iol-support-evidence-help",
                    HelpDocs,
                    ExportedRedactedLineage,
                ),
                bs(
                    "iol-support-evidence-companion",
                    CompanionExport,
                    ImportedOfflineOnly,
                ),
            ],
        ),
        spec(
            "archived-runbook-packet/historical-run",
            "Archived runbook execution packet (imported / offline lineage)",
            ArchivedRunbookPacket,
            grammar(
                "snapshot_labeling",
                "archived_runbook_execution_packet",
                "run_capture_time",
                "runbook_run_provenance",
                read_only_posture,
            ),
            vec![
                bs("iol-runbook-archive", RunbookArchive, ImportedOfflineOnly),
                bs("iol-runbook-review", ReviewIncident, MetadataOnlyExit),
                bs("iol-runbook-program", ProgramGovernance, LiveTargetJoinable),
            ],
        ),
        spec(
            "imported-offline-route-evidence/offline-only",
            "Imported / offline route evidence (imported / offline lineage)",
            ImportedOfflineRouteEvidence,
            grammar(
                "imported_offline_disclosure",
                "imported_offline_route_evidence",
                "import_capture_time",
                "import_offline_source_provenance",
                read_only_posture,
            ),
            vec![
                bs("iol-imported-shell", Shell, ImportedOfflineOnly),
                bs("iol-imported-runbook", RunbookArchive, MetadataOnlyExit),
                bs("iol-imported-cli", CliExport, ExportedRedactedLineage),
            ],
        ),
        spec(
            "review-incident-snapshot/evidence-archive",
            "Review / incident snapshot (imported / offline lineage)",
            ReviewIncidentSnapshot,
            grammar(
                "mutation_blocked_posture",
                "review_incident_snapshot",
                "incident_capture_time",
                "incident_evidence_provenance",
                read_only_posture,
            ),
            vec![
                bs("iol-review-review", ReviewIncident, LiveTargetJoinable),
                bs("iol-review-shell", Shell, ExportedRedactedLineage),
                bs("iol-review-companion", CompanionExport, ImportedOfflineOnly),
            ],
        ),
    ]
}

/// Builds all lineage bindings, applying `disposition_override` to override a binding's disposition.
fn build_bindings<F>(disposition_override: F) -> Vec<ImportedOfflineLineageBinding>
where
    F: Fn(&str, EvidenceLineageDisposition) -> EvidenceLineageDisposition,
{
    let mut bindings = Vec::new();
    for profile in profile_specs() {
        for spec in &profile.bindings {
            let disposition = disposition_override(spec.binding_id, spec.disposition);
            bindings.push(make_binding(
                spec.binding_id,
                profile.profile_id,
                profile.profile_label,
                profile.object_class,
                spec.consumer,
                disposition,
                profile.grammar.clone(),
            ));
        }
    }
    bindings
}

fn trust_review() -> ImportedOfflineLineageTrustReview {
    ImportedOfflineLineageTrustReview {
        object_class_reuse_proven_by_fixtures: true,
        same_profile_same_non_live_grammar_across_surfaces: true,
        historical_role_words_stay_in_frozen_vocabulary: true,
        imported_offline_label_matches_primary_archive_viewer: true,
        mutation_blocked_posture_never_masquerades_as_live: true,
        every_binding_joins_lineage_back_to_source_descriptor: true,
        companion_and_support_consumers_share_non_live_vocabulary: true,
        metadata_provenance_and_boundary_render_instead_of_dead_link: true,
        historical_packet_never_narrated_as_current_live_truth: true,
        imported_offline_never_presented_as_current_route_or_provider: true,
        lineage_metadata_never_leaks_secret_or_stale_authority: true,
        open_live_offered_only_through_validated_handoff: true,
        stable_disposition_labels_used_across_surfaces: true,
        accessibility_routes_present_for_boundary_provenance_and_join: true,
        disposition_disclosed_across_all_lineage_dispositions: true,
        support_export_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> ImportedOfflineLineageProjection {
    ImportedOfflineLineageProjection {
        shell_consumes_lineage: true,
        help_docs_consumes_lineage: true,
        support_consumes_lineage: true,
        review_incident_consumes_lineage: true,
        runbook_archive_consumes_lineage: true,
        release_center_consumes_lineage: true,
        companion_export_consumes_lineage: true,
        program_governance_consumes_lineage: true,
        cli_export_consumes_lineage: true,
        every_object_class_stated_by_two_or_more_consumers: true,
        non_live_grammar_identical_for_same_profile: true,
        non_live_boundary_disclosed_not_hidden: true,
        lineage_maps_back_to_one_historical_reference_object: true,
    }
}

fn proof_freshness() -> ImportedOfflineLineageProofFreshness {
    ImportedOfflineLineageProofFreshness {
        proof_freshness_slo_hours: M5_IMPORTED_OFFLINE_LINEAGE_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_IMPORTED_OFFLINE_LINEAGE_SCHEMA_REF.to_owned(),
        M5_IMPORTED_OFFLINE_LINEAGE_DOC_REF.to_owned(),
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
    lineage_bindings: Vec<ImportedOfflineLineageBinding>,
) -> M5ImportedOfflineLineagePacket {
    M5ImportedOfflineLineagePacket::new(M5ImportedOfflineLineagePacketInput {
        packet_id: packet_id.to_owned(),
        surface_label: surface_label.to_owned(),
        lineage_bindings,
        downgrade_triggers: ImportedOfflineLineageDowngradeTrigger::ALL.to_vec(),
        consumer_surfaces: M5HistoricalReferenceConsumerSurface::ALL.to_vec(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// The canonical, checked-in imported / offline lineage packet.
pub fn seeded_m5_imported_offline_lineage() -> M5ImportedOfflineLineagePacket {
    packet_from_bindings(
        M5_IMPORTED_OFFLINE_LINEAGE_PACKET_ID,
        "M5 imported / offline evidence lineage propagation (one vocabulary across consumers)",
        build_bindings(|_, default| default),
    )
}

/// Fixture: two live-target-joinable lineages narrowed to imported-offline-only, proving the non-live grammar and
/// lineage join survive and the open-live affordance is dropped when no live counterpart remains.
pub fn seeded_m5_imported_offline_lineage_imported_offline_narrowed(
) -> M5ImportedOfflineLineagePacket {
    packet_from_bindings(
        "m5-imported-offline-lineage:imported-offline:0001",
        "M5 imported / offline evidence lineage propagation (imported-offline narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "iol-support-evidence-support" => EvidenceLineageDisposition::ImportedOfflineOnly,
            "iol-runbook-program" => EvidenceLineageDisposition::ImportedOfflineOnly,
            _ => default,
        }),
    )
}

/// Fixture: two live-target-joinable lineages narrowed to metadata-only-exit, proving the metadata / provenance
/// and non-live boundary render instead of a dead link once the content is unavailable.
pub fn seeded_m5_imported_offline_lineage_metadata_only_narrowed() -> M5ImportedOfflineLineagePacket
{
    packet_from_bindings(
        "m5-imported-offline-lineage:metadata-only:0001",
        "M5 imported / offline evidence lineage propagation (metadata-only narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "iol-review-review" => EvidenceLineageDisposition::MetadataOnlyExit,
            "iol-retirement-release" => EvidenceLineageDisposition::MetadataOnlyExit,
            _ => default,
        }),
    )
}
