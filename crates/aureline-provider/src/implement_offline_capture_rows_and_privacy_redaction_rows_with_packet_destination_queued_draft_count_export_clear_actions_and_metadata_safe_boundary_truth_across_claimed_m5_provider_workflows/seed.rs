//! Canonical seed builders for the M5 provider offline-capture / privacy-redaction row
//! primitive.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix,
//! the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical offline/privacy-row primitive packet.
pub const M5_PROVIDER_OFFLINE_PRIVACY_ROW_PACKET_ID: &str =
    "m5-provider-offline-capture-privacy-redaction-row-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked offline-capture-row resolution case from a full capture state.
#[allow(clippy::too_many_arguments)]
fn capture_case(
    capture_state: M5OfflineCaptureState,
    capture_kind: M5OfflineCaptureKind,
    destination_class: M5OfflinePacketDestinationClass,
    queued_draft_state: M5QueuedDraftState,
    redaction_default: M5ProviderRedactionClass,
    queued_draft_count: u32,
    packet_destination_label: &str,
    capture_label: &str,
    capture_ref: &str,
) -> M5OfflineCaptureRowResolutionCase {
    M5OfflineCaptureRowResolutionCase::resolved(M5OfflineCaptureRowResolutionInput {
        capture_state,
        capture_kind,
        destination_class,
        queued_draft_state,
        redaction_default,
        queued_draft_count,
        packet_destination_label: packet_destination_label.to_owned(),
        capture_label: capture_label.to_owned(),
        capture_ref: capture_ref.to_owned(),
    })
}

/// Builds a worked privacy/redaction-row resolution case from a full redaction state.
fn privacy_case(
    redaction_class: M5ProviderRedactionClass,
    export_boundary: M5ExportBoundaryClass,
    policy_source: M5RedactionPolicySource,
    telemetry_limit: M5TelemetryEventLimit,
    policy_label: &str,
    redaction_ref: &str,
) -> M5PrivacyRedactionRowResolutionCase {
    M5PrivacyRedactionRowResolutionCase::resolved(M5PrivacyRedactionRowResolutionInput {
        redaction_class,
        export_boundary,
        policy_source,
        telemetry_limit,
        policy_label: policy_label.to_owned(),
        redaction_ref: redaction_ref.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full offline-capture-row and privacy-row
/// anatomy, state, destination, posture, action, redaction, boundary, policy, telemetry,
/// export-field, and accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5OfflinePrivacyConsumerSurface,
    qualification: M5ProviderQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    offline_examples: Vec<M5OfflineCaptureRowResolutionCase>,
    privacy_examples: Vec<M5PrivacyRedactionRowResolutionCase>,
) -> M5OfflinePrivacyConsumerRow {
    M5OfflinePrivacyConsumerRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5ProviderSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ProviderDeploymentLine::ALL.to_vec(),
        offline_anatomy_parts: M5OfflineCaptureRowAnatomyPart::ALL.to_vec(),
        privacy_anatomy_parts: M5PrivacyRedactionRowAnatomyPart::ALL.to_vec(),
        capture_states: M5OfflineCaptureState::ALL.to_vec(),
        capture_kinds: M5OfflineCaptureKind::ALL.to_vec(),
        destination_classes: M5OfflinePacketDestinationClass::ALL.to_vec(),
        capture_row_postures: M5OfflineCaptureRowPosture::ALL.to_vec(),
        publish_later_behaviors: M5PublishLaterBehavior::ALL.to_vec(),
        offline_row_actions: M5OfflineCaptureRowAction::ALL.to_vec(),
        redaction_classes: M5ProviderRedactionClass::ALL.to_vec(),
        export_boundaries: M5ExportBoundaryClass::ALL.to_vec(),
        policy_sources: M5RedactionPolicySource::ALL.to_vec(),
        telemetry_limits: M5TelemetryEventLimit::ALL.to_vec(),
        support_bundle_treatments: M5SupportBundleTreatment::ALL.to_vec(),
        privacy_field_classes: M5PrivacyFieldClass::ALL.to_vec(),
        privacy_row_postures: M5PrivacyRedactionRowPosture::ALL.to_vec(),
        privacy_row_actions: M5PrivacyRedactionRowAction::ALL.to_vec(),
        queued_draft_states: M5QueuedDraftState::ALL.to_vec(),
        offline_export_fields: M5OfflineCaptureRowExportField::ALL.to_vec(),
        privacy_export_fields: M5PrivacyRedactionRowExportField::ALL.to_vec(),
        accessibility_routes: M5ProviderAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5ProviderConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5ProviderDowngradeTrigger::OfflineCaptureStateUnstated,
            M5ProviderDowngradeTrigger::QueuedDraftStateHidden,
            M5ProviderDowngradeTrigger::RedactionClassUnstated,
            M5ProviderDowngradeTrigger::ExportBoundaryHidden,
            M5ProviderDowngradeTrigger::DefaultDestinationAssumed,
            M5ProviderDowngradeTrigger::AlternateStateLabelInvented,
            M5ProviderDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_PROVIDER_OFFLINE_PRIVACY_ROW_SCHEMA_REF,
            M5_PROVIDER_OFFLINE_PRIVACY_ROW_OFFLINE_HANDOFF_REF,
            M5_PROVIDER_OFFLINE_PRIVACY_ROW_EXPORT_REDACTION_REF,
        ]),
        offline_examples,
        privacy_examples,
        assumes_default_destination_silently: false,
        hides_queued_local_work: false,
        drops_prepared_handoff_state: false,
        hides_export_or_redaction_boundary: false,
        leaks_credentials_or_endpoints: false,
    }
}

fn rows() -> Vec<M5OfflinePrivacyConsumerRow> {
    use M5ExportBoundaryClass as Boundary;
    use M5OfflineCaptureKind as Kind;
    use M5OfflineCaptureState as State;
    use M5OfflinePacketDestinationClass as Dest;
    use M5ProviderRedactionClass as Redaction;
    use M5QueuedDraftState as Draft;
    use M5RedactionPolicySource as Policy;
    use M5TelemetryEventLimit as Telemetry;

    vec![
        // 1. Offline-capture panel — a bug report queued for publish to a routed provider target
        //    (publishes-when-reachable, three drafts queued, defer offered) and an
        //    already-synced task update (already-published, zero queued, no clear); a
        //    metadata-only redaction and a full-body-visible reveal, both exporting only
        //    metadata-safe fields.
        base_row(
            M5OfflinePrivacyConsumerSurface::OfflineCapturePanel,
            M5ProviderQualificationClass::Stable,
            "Offline-capture panel owner",
            "The offline-capture panel renders the shared offline row so a queued bug report reads as publishes-when-reachable with its destination and three-draft queue in view and a defer action, and an already-synced task update reads as already-published with a cleared queue and no clear action — never a silent default destination; the same panel's privacy row keeps a metadata-only and a full-body reveal both metadata-safe",
            "evidence:m5-offline-privacy-row-offline-capture:001",
            vec![
                capture_case(
                    State::QueuedForPublish,
                    Kind::BugReport,
                    Dest::RoutedToProvider,
                    Draft::QueuedPublish,
                    Redaction::MetadataOnly,
                    3,
                    "acme-eng issues (queued)",
                    "crash on export (bug)",
                    "capture:acme-eng:bug:queued-1",
                ),
                capture_case(
                    State::SyncedCleared,
                    Kind::TaskUpdate,
                    Dest::RoutedToProvider,
                    Draft::PublishedReconciled,
                    Redaction::MetadataOnly,
                    0,
                    "acme-eng issues (synced)",
                    "task update published",
                    "capture:acme-eng:task:synced-1",
                ),
            ],
            vec![
                privacy_case(
                    Redaction::MetadataOnly,
                    Boundary::MetadataSafe,
                    Policy::UserDefault,
                    Telemetry::MetadataCountersOnly,
                    "acme-eng metadata-only default",
                    "redaction:acme-eng:metadata:1",
                ),
                privacy_case(
                    Redaction::FullBodyVisible,
                    Boundary::MetadataSafe,
                    Policy::WorkspacePolicy,
                    Telemetry::RedactedEventShare,
                    "acme-eng full reveal (local)",
                    "redaction:acme-eng:fullbody:1",
                ),
            ],
        ),
        // 2. Privacy/redaction panel — a locally captured blocked-work note held as a local
        //    bundle (held-locally, one draft) and a user-deferred task update (held-by-user, two
        //    drafts); a redacted share and a policy-restricted row whose org policy blocks a
        //    local adjust.
        base_row(
            M5OfflinePrivacyConsumerSurface::PrivacyRedactionPanel,
            M5ProviderQualificationClass::Stable,
            "Privacy/redaction panel owner",
            "The privacy/redaction panel renders both rows so a captured blocked-work note reads as held-locally with a local-bundle destination and a redacted share names exactly which fields it copies out, while a policy-restricted row states its org-policy source and offers no local adjust — every row withholds credentials and endpoints and offers a reviewed escalation before anything leaves the device",
            "evidence:m5-offline-privacy-row-privacy-redaction:001",
            vec![
                capture_case(
                    State::CapturedLocal,
                    Kind::BlockedWorkNote,
                    Dest::LocalBundleOnly,
                    Draft::DraftPending,
                    Redaction::RedactedShare,
                    1,
                    "acme-eng local bundle",
                    "blocked on infra access (note)",
                    "capture:acme-eng:note:local-1",
                ),
                capture_case(
                    State::PublishDeferred,
                    Kind::TaskUpdate,
                    Dest::RoutedToProvider,
                    Draft::DraftPending,
                    Redaction::MetadataOnly,
                    2,
                    "acme-eng issues (deferred)",
                    "task update deferred",
                    "capture:acme-eng:task:deferred-1",
                ),
            ],
            vec![
                privacy_case(
                    Redaction::RedactedShare,
                    Boundary::BodyExcluded,
                    Policy::WorkspacePolicy,
                    Telemetry::RedactedEventShare,
                    "acme-eng redacted share",
                    "redaction:acme-eng:redacted:1",
                ),
                privacy_case(
                    Redaction::PolicyRestricted,
                    Boundary::FullDisclosureBlocked,
                    Policy::OrgPolicy,
                    Telemetry::EventsSuppressed,
                    "acme-eng org-restricted",
                    "redaction:acme-eng:policy:1",
                ),
            ],
        ),
        // 3. Provider status bar — a conflict-held bug report whose blocked publish offers retry
        //    (held-pending-conflict) and a discard-pending blocked-work note whose failed
        //    publish offers retry and is still unrouted (will-discard, unrouted flagged); a
        //    raw-withheld row with regulatory policy and a no-export row that offers no export.
        base_row(
            M5OfflinePrivacyConsumerSurface::ProviderStatusBar,
            M5ProviderQualificationClass::Stable,
            "Provider status bar owner",
            "The provider status bar renders both rows so a conflict-held bug report reads as held-pending-conflict with a retry action and an unrouted discard-pending note flags itself unrouted rather than defaulting, while a raw-withheld row names its regulatory policy and a no-export row offers no export at all — a user can tell destination, queue, and boundary from the bar alone",
            "evidence:m5-offline-privacy-row-status-bar:001",
            vec![
                capture_case(
                    State::ConflictHeld,
                    Kind::BugReport,
                    Dest::RoutedToProvider,
                    Draft::PublishBlocked,
                    Redaction::PolicyRestricted,
                    1,
                    "acme-eng issues (conflict)",
                    "duplicate detected (bug)",
                    "capture:acme-eng:bug:conflict-1",
                ),
                capture_case(
                    State::DiscardPending,
                    Kind::BlockedWorkNote,
                    Dest::UnroutedPending,
                    Draft::PublishFailed,
                    Redaction::RawWithheld,
                    1,
                    "(no destination chosen)",
                    "obsolete blocked note",
                    "capture:acme-eng:note:discard-1",
                ),
            ],
            vec![
                privacy_case(
                    Redaction::RawWithheld,
                    Boundary::CredentialsScrubbed,
                    Policy::RegulatoryPolicy,
                    Telemetry::EventsSuppressed,
                    "acme-eng raw withheld",
                    "redaction:acme-eng:raw:1",
                ),
                privacy_case(
                    Redaction::NoExport,
                    Boundary::LocalOnly,
                    Policy::ProviderPolicy,
                    Telemetry::NoEventExport,
                    "acme-eng no export",
                    "redaction:acme-eng:noexport:1",
                ),
            ],
        ),
        // 4. Headless / CLI capture — a queued task update (publishes-when-reachable, five
        //    drafts) and a locally captured bug report on a local bundle (held-locally) — proving
        //    the same grammar works headless; a metadata-only row and a redacted share, both
        //    metadata-safe and adjustable.
        base_row(
            M5OfflinePrivacyConsumerSurface::HeadlessCliCapture,
            M5ProviderQualificationClass::Stable,
            "Headless CLI capture owner",
            "The headless / CLI capture surface renders both rows so a queued task update reads as publishes-when-reachable with its five-draft queue in view and a locally captured bug report reads as held-locally on a local bundle — proving the same offline/privacy grammar works headless with every field copied/exported stated and credentials/endpoints withheld",
            "evidence:m5-offline-privacy-row-headless-cli:001",
            vec![
                capture_case(
                    State::QueuedForPublish,
                    Kind::TaskUpdate,
                    Dest::RoutedToProvider,
                    Draft::QueuedPublish,
                    Redaction::MetadataOnly,
                    5,
                    "acme-infra tasks (queued)",
                    "deploy status update",
                    "capture:acme-infra:task:queued-1",
                ),
                capture_case(
                    State::CapturedLocal,
                    Kind::BugReport,
                    Dest::LocalBundleOnly,
                    Draft::DraftPending,
                    Redaction::MetadataOnly,
                    1,
                    "acme-infra local bundle",
                    "flaky pipeline (bug)",
                    "capture:acme-infra:bug:local-1",
                ),
            ],
            vec![
                privacy_case(
                    Redaction::MetadataOnly,
                    Boundary::EndpointsMasked,
                    Policy::UserDefault,
                    Telemetry::MetadataCountersOnly,
                    "acme-infra metadata-only",
                    "redaction:acme-infra:metadata:1",
                ),
                privacy_case(
                    Redaction::RedactedShare,
                    Boundary::BodyExcluded,
                    Policy::WorkspacePolicy,
                    Telemetry::RedactedEventShare,
                    "acme-infra redacted share",
                    "redaction:acme-infra:redacted:1",
                ),
            ],
        ),
        // 5. Support privacy export — an already-synced task update (already-published, cleared)
        //    and a user-deferred blocked-work note on a local bundle (held-by-user) — the same
        //    rows a support agent reads elsewhere; a metadata-only export and a policy-restricted
        //    row, both withholding raw bodies.
        base_row(
            M5OfflinePrivacyConsumerSurface::SupportPrivacyExport,
            M5ProviderQualificationClass::Stable,
            "Support privacy export owner",
            "The support privacy export renders both rows so an already-synced task update exports as already-published with a cleared queue and a deferred blocked-work note exports as held-by-user on a local bundle, while a metadata-only export stays metadata-safe and a policy-restricted row states its org policy — the same rows a support agent reads elsewhere, with raw bodies, credentials, and endpoints withheld",
            "evidence:m5-offline-privacy-row-support-export:001",
            vec![
                capture_case(
                    State::SyncedCleared,
                    Kind::TaskUpdate,
                    Dest::RoutedToProvider,
                    Draft::PublishedReconciled,
                    Redaction::MetadataOnly,
                    0,
                    "acme-eng issues (synced)",
                    "task update reconciled",
                    "capture:acme-eng:task:synced-2",
                ),
                capture_case(
                    State::PublishDeferred,
                    Kind::BlockedWorkNote,
                    Dest::LocalBundleOnly,
                    Draft::DraftPending,
                    Redaction::RedactedShare,
                    2,
                    "acme-eng local bundle",
                    "blocked note deferred",
                    "capture:acme-eng:note:deferred-1",
                ),
            ],
            vec![
                privacy_case(
                    Redaction::MetadataOnly,
                    Boundary::MetadataSafe,
                    Policy::WorkspacePolicy,
                    Telemetry::MetadataCountersOnly,
                    "acme-eng support metadata",
                    "redaction:acme-eng:metadata:2",
                ),
                privacy_case(
                    Redaction::PolicyRestricted,
                    Boundary::FullDisclosureBlocked,
                    Policy::OrgPolicy,
                    Telemetry::EventsSuppressed,
                    "acme-eng support restricted",
                    "redaction:acme-eng:policy:2",
                ),
            ],
        ),
    ]
}

fn governance_review() -> M5OfflinePrivacyRowGovernanceReview {
    M5OfflinePrivacyRowGovernanceReview {
        offline_row_shows_packet_destination: true,
        offline_row_shows_queued_draft_count: true,
        offline_row_shows_redaction_default: true,
        offline_row_shows_publish_later_behavior: true,
        offline_row_offers_export_and_clear: true,
        offline_never_erases_prepared_handoff: true,
        privacy_row_states_copied_exported_fields: true,
        privacy_row_states_support_bundle_treatment: true,
        privacy_row_states_telemetry_event_limits: true,
        privacy_row_states_policy_source: true,
        privacy_row_offers_reviewed_escalation: true,
        metadata_safe_default_explicit_before_leaving_device: true,
        rows_stable_across_deployment_lines: true,
        rows_stable_across_consumer_surfaces: true,
        every_row_declares_accessibility_route: true,
        support_export_reconstructs_offline_and_privacy_truth: true,
        later_rows_cannot_invent_parallel_offline_or_privacy_vocabulary: true,
    }
}

fn consumer_projection() -> M5OfflinePrivacyRowConsumerProjection {
    M5OfflinePrivacyRowConsumerProjection {
        provider_surfaces_consume_offline_privacy_vocabulary: true,
        offline_posture_reads_single_source: true,
        privacy_redaction_reads_single_source: true,
        support_export_reads_single_source: true,
        headless_and_desktop_read_single_source: true,
    }
}

fn proof_freshness() -> M5OfflinePrivacyRowProofFreshness {
    M5OfflinePrivacyRowProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5OfflinePrivacyRowReleasePosture {
    M5OfflinePrivacyRowReleasePosture {
        release_packet_ref: M5_PROVIDER_OFFLINE_PRIVACY_ROW_ARTIFACT_REF.to_owned(),
        provider_offline_privacy_audit_ref: M5_PROVIDER_OFFLINE_PRIVACY_ROW_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_PROVIDER_OFFLINE_PRIVACY_ROW_SCHEMA_REF,
        M5_PROVIDER_OFFLINE_PRIVACY_ROW_DOC_REF,
        M5_PROVIDER_OFFLINE_PRIVACY_ROW_COMPONENT_MATRIX_REF,
        M5_PROVIDER_OFFLINE_PRIVACY_ROW_OFFLINE_HANDOFF_REF,
        M5_PROVIDER_OFFLINE_PRIVACY_ROW_EXPORT_REDACTION_REF,
    ])
}

/// Builds the canonical M5 provider offline/privacy-row packet.
pub fn seeded_m5_provider_offline_privacy_row_packet() -> M5ProviderOfflinePrivacyRowPacket {
    M5ProviderOfflinePrivacyRowPacket::new(M5ProviderOfflinePrivacyRowPacketInput {
        packet_id: M5_PROVIDER_OFFLINE_PRIVACY_ROW_PACKET_ID.to_owned(),
        matrix_label:
            "M5 provider offline-capture / privacy-redaction row primitive: offline-capture state (captured/queued/deferred/conflict/discard/synced), capture kind (bug report/task update/blocked-work note), packet destination (routed/local-bundle/unrouted), queued-draft count, redaction default, publish-later behavior, and export/clear actions, plus redaction class, export boundary, policy source, telemetry limit, support-bundle treatment, copied/exported and withheld field classes, and bounded reveal/adjust/export/reviewed-escalation actions with a metadata-safe boundary"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5OfflinePrivacyRowVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the offline-capture panel consumer is held at Beta because a slice of
/// offline rows do not yet render the keyboard route cue on every profile; every consumer stays
/// visible.
pub fn seeded_m5_provider_offline_privacy_row_offline_capture_beta_narrowed(
) -> M5ProviderOfflinePrivacyRowPacket {
    let mut packet = seeded_m5_provider_offline_privacy_row_packet();
    packet.packet_id =
        "m5-provider-offline-capture-privacy-redaction-row-primitive:offline-capture-beta:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5OfflinePrivacyConsumerSurface::OfflineCapturePanel)
        .expect("offline-capture panel row present");
    row.qualification = M5ProviderQualificationClass::Beta;
    packet
}

/// Narrowed variant: the privacy/redaction panel consumer is narrowed to Preview pending
/// metadata-safe-boundary parity proof across every deployment; every consumer stays visible.
pub fn seeded_m5_provider_offline_privacy_row_privacy_redaction_preview_narrowed(
) -> M5ProviderOfflinePrivacyRowPacket {
    let mut packet = seeded_m5_provider_offline_privacy_row_packet();
    packet.packet_id =
        "m5-provider-offline-capture-privacy-redaction-row-primitive:privacy-redaction-preview:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5OfflinePrivacyConsumerSurface::PrivacyRedactionPanel)
        .expect("privacy-redaction panel row present");
    row.qualification = M5ProviderQualificationClass::Preview;
    packet
}
