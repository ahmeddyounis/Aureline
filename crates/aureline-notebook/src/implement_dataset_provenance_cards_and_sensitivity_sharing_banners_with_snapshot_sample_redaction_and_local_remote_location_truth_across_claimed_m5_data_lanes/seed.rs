//! Canonical seed builders for the dataset-provenance-card / sensitivity-sharing-banner
//! controls.
//!
//! These builders are the single producer of the checked-in support export and the scenario
//! fixtures. The headless emitter and the inline tests both call them so the in-code
//! components, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical dataset-provenance-card / sensitivity-sharing-banner
/// packet.
pub const DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_PACKET_ID: &str =
    "m5-dataset-provenance-card-sensitivity-sharing-banner-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn dataset_card_source_refs() -> Vec<String> {
    strings(&[
        M5_DATASET_PROVENANCE_CARD_SCHEMA_REF,
        M5_EXPERIMENT_COMPONENT_SCHEMA_REF,
    ])
}

fn sharing_banner_source_refs() -> Vec<String> {
    strings(&[
        M5_SENSITIVITY_SHARING_BANNER_SCHEMA_REF,
        M5_EXPERIMENT_COMPONENT_SCHEMA_REF,
    ])
}

fn dataset_card_downgrade_triggers() -> Vec<M5ExperimentDowngradeTrigger> {
    vec![
        M5ExperimentDowngradeTrigger::DatasetProvenanceSevered,
        M5ExperimentDowngradeTrigger::RawPayloadExposedByDefault,
        M5ExperimentDowngradeTrigger::CachedStateHidden,
        M5ExperimentDowngradeTrigger::AlternateStateLabelInvented,
        M5ExperimentDowngradeTrigger::ProofStale,
    ]
}

fn sharing_banner_downgrade_triggers() -> Vec<M5ExperimentDowngradeTrigger> {
    vec![
        M5ExperimentDowngradeTrigger::SensitivityClassUnstated,
        M5ExperimentDowngradeTrigger::RawPayloadExposedByDefault,
        M5ExperimentDowngradeTrigger::CachedStateHidden,
        M5ExperimentDowngradeTrigger::AlternateStateLabelInvented,
        M5ExperimentDowngradeTrigger::ProofStale,
    ]
}

/// Builds a dataset provenance card, deriving the location class, the provenance class, the
/// first-party-local / fully-provenanced claims, and the required notes from the honest inputs
/// so the seed is always self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn dataset_card(
    card_id: &str,
    dataset_label: &str,
    source_class: M5DatasetSourceClass,
    provenance_state: M5DatasetProvenanceState,
    version_snapshot_partition_note: &str,
    row_or_file_count_label: &str,
    sample_or_truncation_note: &str,
    is_sampled_or_truncated: bool,
    sensitivity_class: M5SensitivityClass,
    redaction_note: &str,
    context_note: &str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    card_actions: Vec<DatasetCardAction>,
    dispositions: Vec<M5ExperimentDisposition>,
) -> DatasetProvenanceCard {
    let disclosure = resolve_dataset_provenance(source_class, provenance_state);
    DatasetProvenanceCard {
        component: M5ExperimentComponentFamily::DatasetProvenanceCard,
        card_id: card_id.to_owned(),
        dataset_label: dataset_label.to_owned(),
        source_class,
        provenance_state,
        location_class: disclosure.location_class,
        claims_local_data: disclosure.is_local_data,
        provenance_class: disclosure.provenance_class,
        claims_fully_provenanced: disclosure.is_fully_provenanced,
        remote_location_note: if disclosure.needs_remote_note {
            "Data is pulled from a remote store; it is not a local dataset".to_owned()
        } else {
            String::new()
        },
        unknown_location_note: if disclosure.needs_unknown_location_note {
            "Data location could not be resolved; do not treat it as a local dataset".to_owned()
        } else {
            String::new()
        },
        partial_provenance_note: if disclosure.needs_partial_note {
            "Only part of this dataset's provenance was captured; treat lineage as incomplete"
                .to_owned()
        } else {
            String::new()
        },
        unprovenanced_note: if disclosure.needs_unprovenanced_note {
            format!(
                "Provenance is {} and not reliably captured; do not assume the data is verified",
                provenance_state.as_str()
            )
        } else {
            String::new()
        },
        source_and_provenance_note: format!(
            "Source {}; provenance {}",
            source_class.as_str(),
            provenance_state.as_str()
        ),
        version_snapshot_partition_note: version_snapshot_partition_note.to_owned(),
        row_or_file_count_label: row_or_file_count_label.to_owned(),
        sample_or_truncation_note: sample_or_truncation_note.to_owned(),
        is_sampled_or_truncated,
        sensitivity_class,
        redaction_note: redaction_note.to_owned(),
        context_note: context_note.to_owned(),
        deep_link_kind,
        deep_link_ref: deep_link_ref.to_owned(),
        card_actions,
        dispositions,
        downgrade_triggers: dataset_card_downgrade_triggers(),
        required_labels: M5ExperimentRequiredLabel::ALL.to_vec(),
        surface_families: M5ExperimentSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ExperimentDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5ExperimentAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5ExperimentConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "dataset_label",
            "source_class",
            "provenance_state",
            "location_class",
            "version_snapshot_partition_note",
            "row_or_file_count_label",
            "sample_or_truncation_note",
            "sensitivity_class",
            "deep_link_kind",
        ]),
        source_contract_refs: dataset_card_source_refs(),
        masks_provenance_or_sensitivity_state: false,
        hides_dataset_location_or_provenance: false,
        exposes_raw_payload_by_default: false,
        implies_apples_to_apples_without_parity: false,
        invents_alternate_state_label: false,
    }
}

/// Builds a sensitivity / sharing banner, deriving the share disposition, the raw-payload /
/// metadata-only claims, and the required warnings from the honest inputs so the seed is always
/// self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn sharing_banner(
    banner_id: &str,
    banner_label: &str,
    sensitivity_class: M5SensitivityClass,
    share_scope_state: M5ShareScopeState,
    sensitivity_and_share_class_note: &str,
    blocked_destinations_note: &str,
    copy_export_policy_note: &str,
    local_safe_alternative_note: &str,
    context_note: &str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    banner_actions: Vec<ShareBannerAction>,
    dispositions: Vec<M5ExperimentDisposition>,
) -> SensitivitySharingBanner {
    let disclosure = resolve_share_scope(sensitivity_class, share_scope_state);
    SensitivitySharingBanner {
        component: M5ExperimentComponentFamily::SensitivitySharingBanner,
        banner_id: banner_id.to_owned(),
        banner_label: banner_label.to_owned(),
        sensitivity_class,
        share_scope_state,
        share_disposition: disclosure.share_disposition,
        claims_includes_raw_payload: disclosure.includes_raw_payload,
        claims_metadata_only: disclosure.is_metadata_only,
        raw_payload_warning: if disclosure.needs_raw_payload_warning {
            "This share includes a raw payload by explicit choice; it is not the metadata-only default"
                .to_owned()
        } else {
            String::new()
        },
        blocked_note: if disclosure.needs_blocked_note {
            "This share is blocked; use the metadata-only or local-safe alternative instead"
                .to_owned()
        } else {
            String::new()
        },
        redaction_note: if disclosure.needs_redaction_note {
            "This share is redacted; redacted fields are removed before the payload leaves"
                .to_owned()
        } else {
            String::new()
        },
        sensitivity_warning: if disclosure.needs_sensitivity_warning {
            format!(
                "Data is {}; review the share scope before sharing",
                sensitivity_class.as_str()
            )
        } else {
            String::new()
        },
        sensitivity_and_share_class_note: sensitivity_and_share_class_note.to_owned(),
        blocked_destinations_note: blocked_destinations_note.to_owned(),
        copy_export_policy_note: copy_export_policy_note.to_owned(),
        local_safe_alternative_note: local_safe_alternative_note.to_owned(),
        context_note: context_note.to_owned(),
        deep_link_kind,
        deep_link_ref: deep_link_ref.to_owned(),
        banner_actions,
        dispositions,
        downgrade_triggers: sharing_banner_downgrade_triggers(),
        required_labels: M5ExperimentRequiredLabel::ALL.to_vec(),
        surface_families: M5ExperimentSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ExperimentDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5ExperimentAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5ExperimentConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "banner_label",
            "sensitivity_class",
            "share_scope_state",
            "share_disposition",
            "blocked_destinations_note",
            "copy_export_policy_note",
            "local_safe_alternative_note",
            "deep_link_kind",
        ]),
        source_contract_refs: sharing_banner_source_refs(),
        masks_provenance_or_sensitivity_state: false,
        hides_dataset_location_or_provenance: false,
        exposes_raw_payload_by_default: false,
        implies_apples_to_apples_without_parity: false,
        invents_alternate_state_label: false,
    }
}

fn dataset_cards() -> Vec<DatasetProvenanceCard> {
    use DatasetCardAction as Action;
    use DeepLinkKind as Link;
    use M5DatasetProvenanceState as State;
    use M5DatasetSourceClass as Source;
    use M5ExperimentDisposition as Disp;
    use M5SensitivityClass as Sensitivity;

    vec![
        // 1. Tracked dataset, provenance complete → remote data, provenanced.
        dataset_card(
            "ds-tracked-001",
            "Customer events (tracked)",
            Source::TrackedDataset,
            State::ProvenanceComplete,
            "Snapshot v2026.07.01, partition dt=2026-07-01",
            "~12.4M rows (full)",
            "Full dataset; no sampling or truncation",
            false,
            Sensitivity::Internal,
            "Internal data; no additional redaction applied",
            "Tracked remote dataset with complete provenance; compare or export metadata directly",
            Link::DatasetCatalogAnchor,
            "dataset:catalog/customer-events",
            vec![
                Action::OpenDataset,
                Action::InspectProvenance,
                Action::ExportMetadata,
                Action::OpenDeepLink,
                Action::CompareDatasets,
            ],
            vec![Disp::Reproducible],
        ),
        // 2. Local file, version pinned → local data, pinned.
        dataset_card(
            "ds-localfile-002",
            "Feature matrix (local file)",
            Source::LocalFile,
            State::VersionPinned,
            "Pinned to content hash sha-pinned-9f",
            "~48k rows (full)",
            "Full local file; no sampling or truncation",
            false,
            Sensitivity::PublicSafe,
            "Public-safe data; no redaction required",
            "Local file pinned to a content hash; safe to reproduce and export metadata",
            Link::NotebookLocation,
            "notebook:features.ipynb#cell-2",
            vec![
                Action::OpenDataset,
                Action::InspectProvenance,
                Action::ExportMetadata,
                Action::CopyDatasetId,
            ],
            vec![Disp::Reproducible],
        ),
        // 3. Remote snapshot, provenance partial → remote data, partially provenanced.
        dataset_card(
            "ds-remote-003",
            "Pricing snapshot (remote)",
            Source::RemoteSnapshot,
            State::ProvenancePartial,
            "Remote snapshot 2026-06-30 (partition list partial)",
            "~2.1M rows (estimate)",
            "Full snapshot; row count is an estimate, not sampled",
            false,
            Sensitivity::Confidential,
            "Confidential data; column-level redaction applied before preview",
            "Remote snapshot with partial provenance; check lineage before trusting a compare",
            Link::DatasetCatalogAnchor,
            "dataset:catalog/pricing-snapshots",
            vec![
                Action::OpenDataset,
                Action::InspectProvenance,
                Action::ExportMetadata,
                Action::OpenDeepLink,
            ],
            vec![Disp::LikelyReproducible],
        ),
        // 4. Synthetic data, provenance missing → local data, unprovenanced.
        dataset_card(
            "ds-synthetic-004",
            "Synthetic eval set",
            Source::SyntheticData,
            State::ProvenanceMissing,
            "Generated locally; no upstream snapshot",
            "~10k rows (generated)",
            "Generated set; not a sample of production data",
            false,
            Sensitivity::PublicSafe,
            "Synthetic public-safe data; no redaction required",
            "Synthetic data with no upstream provenance; do not treat as verified production data",
            Link::DocsAnchor,
            "docs:data/synthetic-eval-sets",
            vec![
                Action::OpenDataset,
                Action::InspectProvenance,
                Action::ExportMetadata,
                Action::OpenDeepLink,
            ],
            vec![Disp::ContextIncomplete],
        ),
        // 5. Redacted sample, access restricted → local data, unprovenanced (sampled/redacted).
        dataset_card(
            "ds-redacted-005",
            "Redacted PII sample",
            Source::RedactedSample,
            State::AccessRestricted,
            "Redacted sample extract; source partition access-restricted",
            "~5k rows (sampled)",
            "Redacted sample; truncated to 5k rows from a restricted source",
            true,
            Sensitivity::Regulated,
            "Regulated data; PII redacted and access-restricted before extract",
            "Access-restricted regulated source; only a redacted sample is available locally",
            Link::DocsAnchor,
            "docs:data/redacted-samples",
            vec![
                Action::OpenDataset,
                Action::InspectProvenance,
                Action::ExportMetadata,
                Action::OpenDeepLink,
            ],
            vec![Disp::ContextIncomplete],
        ),
        // 6. Unknown source, version drifted → unknown location, unprovenanced.
        dataset_card(
            "ds-unknown-006",
            "Unlabeled input",
            Source::UnknownSource,
            State::VersionDrifted,
            "Version drifted from last known snapshot; source unresolved",
            "Unknown row/file count",
            "Sampling state unknown; treat completeness as unverified",
            false,
            Sensitivity::UnknownSensitivity,
            "Sensitivity unknown; withhold from share until classified",
            "Source and version could not be resolved; do not trust it in a compare or share",
            Link::NoDeepLink,
            "",
            vec![
                Action::OpenDataset,
                Action::InspectProvenance,
                Action::ExportMetadata,
            ],
            vec![Disp::ContextIncomplete],
        ),
    ]
}

fn sharing_banners() -> Vec<SensitivitySharingBanner> {
    use DeepLinkKind as Link;
    use M5ExperimentDisposition as Disp;
    use M5SensitivityClass as Sensitivity;
    use M5ShareScopeState as Scope;
    use ShareBannerAction as Action;

    vec![
        // 1. Public-safe, summary plus metadata → metadata-safe.
        sharing_banner(
            "sb-public-001",
            "Public-safe metadata share",
            Sensitivity::PublicSafe,
            Scope::SummaryPlusMetadata,
            "Public-safe; summary plus metadata only",
            "No destinations blocked for public-safe metadata",
            "Copy and export permitted for summary and metadata",
            "Already metadata-only; nothing to downgrade",
            "Public-safe summary and metadata; no raw payload leaves this share",
            Link::DocsAnchor,
            "docs:data/share-public-safe",
            vec![
                Action::ReviewShareScope,
                Action::ShareMetadataOnly,
                Action::ExportMetadataOnly,
                Action::OpenDeepLink,
            ],
            vec![Disp::Reproducible],
        ),
        // 2. Internal, raw payload included → raw-exposed (explicit opt-in, warned).
        sharing_banner(
            "sb-internal-raw-002",
            "Internal raw-payload share",
            Sensitivity::Internal,
            Scope::RawPayloadIncluded,
            "Internal; raw payload included by explicit choice",
            "Blocked to external and public destinations",
            "Copy and export gated behind explicit raw-payload confirmation",
            "Switch to share-metadata-only to keep the raw payload local",
            "Raw payload is included only because it was explicitly chosen, never by default",
            Link::DocsAnchor,
            "docs:data/share-internal-raw",
            vec![
                Action::ReviewShareScope,
                Action::ShareMetadataOnly,
                Action::CopyLocalSafeReference,
                Action::OpenDeepLink,
            ],
            vec![Disp::ContextIncomplete],
        ),
        // 3. Confidential, evidence included → evidence-scoped (high-sensitivity, warned).
        sharing_banner(
            "sb-confidential-003",
            "Confidential evidence share",
            Sensitivity::Confidential,
            Scope::EvidenceIncluded,
            "Confidential; evidence links included, no raw payload",
            "Blocked to external destinations",
            "Copy permitted for evidence links; raw export withheld",
            "Share metadata only to omit evidence links entirely",
            "Confidential evidence links included; raw payload stays out of this share",
            Link::DatasetCatalogAnchor,
            "dataset:catalog/confidential-evidence",
            vec![
                Action::ReviewShareScope,
                Action::ShareMetadataOnly,
                Action::ExportMetadataOnly,
                Action::OpenDeepLink,
            ],
            vec![Disp::LikelyReproducible],
        ),
        // 4. Regulated, redacted share → redacted (high-sensitivity, warned).
        sharing_banner(
            "sb-regulated-004",
            "Regulated redacted share",
            Sensitivity::Regulated,
            Scope::RedactedShare,
            "Regulated; redacted share only",
            "Blocked to external and unmanaged destinations",
            "Copy and export permitted only for redacted fields",
            "Share metadata only to omit even the redacted payload",
            "Regulated data is redacted before it leaves; raw fields never cross this boundary",
            Link::DocsAnchor,
            "docs:data/share-regulated-redacted",
            vec![
                Action::ReviewShareScope,
                Action::ShareMetadataOnly,
                Action::CopyLocalSafeReference,
                Action::OpenDeepLink,
            ],
            vec![Disp::ContextIncomplete],
        ),
        // 5. Production-like, share blocked → blocked (high-sensitivity, warned).
        sharing_banner(
            "sb-prodlike-005",
            "Production-like blocked share",
            Sensitivity::ProductionLike,
            Scope::ShareBlocked,
            "Production-like; share blocked by policy",
            "All external and internal share destinations blocked",
            "Copy and export blocked; only local inspection remains",
            "Use the local-safe metadata reference; the payload stays on this machine",
            "Production-like data cannot be shared; only a local-safe metadata reference is offered",
            Link::DocsAnchor,
            "docs:data/production-like-blocked",
            vec![
                Action::ReviewShareScope,
                Action::ShareMetadataOnly,
                Action::CopyLocalSafeReference,
                Action::BlockShare,
                Action::OpenDeepLink,
            ],
            vec![Disp::ContextIncomplete],
        ),
        // 6. Unknown sensitivity, summary only → metadata-safe.
        sharing_banner(
            "sb-unknown-006",
            "Unknown-sensitivity summary share",
            Sensitivity::UnknownSensitivity,
            Scope::SummaryOnly,
            "Sensitivity unknown; summary only until classified",
            "Blocked to all external destinations until classified",
            "Copy and export limited to the summary line",
            "Already summary-only; keep it local until sensitivity is classified",
            "Sensitivity is unknown, so only the summary is shared and raw data is withheld",
            Link::NoDeepLink,
            "",
            vec![Action::ReviewShareScope, Action::ShareMetadataOnly],
            vec![Disp::ContextIncomplete],
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5ExperimentDowngradeTrigger> {
    vec![
        M5ExperimentDowngradeTrigger::DatasetProvenanceSevered,
        M5ExperimentDowngradeTrigger::SensitivityClassUnstated,
        M5ExperimentDowngradeTrigger::RawPayloadExposedByDefault,
        M5ExperimentDowngradeTrigger::CachedStateHidden,
        M5ExperimentDowngradeTrigger::AlternateStateLabelInvented,
        M5ExperimentDowngradeTrigger::ProofStale,
    ]
}

fn dataset_review() -> DatasetSensitivityReview {
    DatasetSensitivityReview {
        dataset_card_shows_source_and_provenance: true,
        dataset_card_shows_location_and_sample_state: true,
        dataset_card_offers_open_inspect_export: true,
        share_banner_shows_sensitivity_and_scope: true,
        share_banner_offers_review_and_metadata_only: true,
        location_and_provenance_derived_never_asserted: true,
        remote_or_unknown_never_shown_as_local: true,
        unprovenanced_never_shown_as_provenanced: true,
        raw_payload_never_implied_by_default: true,
        metadata_only_and_sampled_state_visible_before_share: true,
        every_next_step_names_stable_deep_link: true,
        reuses_existing_privacy_redaction_share_vocabulary: true,
        provenance_and_sensitivity_state_visible: true,
        cached_offline_local_only_state_visible: true,
        no_surface_invents_alternate_state_label: true,
        components_stable_across_deployment_lines: true,
        copy_and_export_safe: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> DatasetSensitivityConsumerProjection {
    DatasetSensitivityConsumerProjection {
        dataset_catalog_reads_single_source: true,
        share_review_sheet_reads_single_source: true,
        provenance_and_location_visible_before_preview_or_compare: true,
        sensitivity_and_scope_visible_before_share: true,
        support_export_shows_component_truth: true,
        help_docs_shows_component_truth: true,
    }
}

fn proof_freshness() -> DatasetSensitivityProofFreshness {
    DatasetSensitivityProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_SCHEMA_REF,
        DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_DOC_REF,
        M5_EXPERIMENT_COMPONENT_SCHEMA_REF,
        M5_EXPERIMENT_COMPONENT_DOC_REF,
        M5_DATASET_PROVENANCE_CARD_SCHEMA_REF,
        M5_SENSITIVITY_SHARING_BANNER_SCHEMA_REF,
    ])
}

/// Builds the canonical dataset-provenance-card / sensitivity-sharing-banner controls packet.
pub fn seeded_dataset_provenance_card_sensitivity_sharing_banner_controls(
) -> DatasetProvenanceCardSensitivitySharingBannerControlsPacket {
    DatasetProvenanceCardSensitivitySharingBannerControlsPacket::new(
        DatasetProvenanceCardSensitivitySharingBannerControlsPacketInput {
            packet_id: DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_PACKET_ID.to_owned(),
            surface_label:
                "M5 dataset provenance cards and sensitivity/sharing banners: source, snapshot, sample/redaction, sensitivity, and local-versus-remote location truth across claimed data lanes"
                    .to_owned(),
            dataset_cards: dataset_cards(),
            sharing_banners: sharing_banners(),
            downgrade_triggers: downgrade_triggers(),
            consumer_surfaces: M5ExperimentConsumerSurface::ALL.to_vec(),
            dataset_review: dataset_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Scenario fixture: spotlights a remote dataset provenance card that must never read as a
/// local dataset. Every location class, provenance class, dataset source class, and provenance
/// state stays covered so the fixture validates on its own.
pub fn seeded_dataset_provenance_card_sensitivity_sharing_banner_controls_dataset_card_remote(
) -> DatasetProvenanceCardSensitivitySharingBannerControlsPacket {
    let mut packet = seeded_dataset_provenance_card_sensitivity_sharing_banner_controls();
    packet.packet_id =
        "m5-dataset-provenance-card-sensitivity-sharing-banner-controls:fixture:dataset-card-remote"
            .to_owned();
    packet.surface_label =
        "M5 dataset provenance cards: a remote dataset never reads as a local dataset".to_owned();
    packet
}

/// Scenario fixture: spotlights a raw-payload sensitivity/sharing banner that must flag its raw
/// scope and never read as a metadata-only default. Every share disposition, sensitivity class,
/// and share scope state stays covered so the fixture validates on its own.
pub fn seeded_dataset_provenance_card_sensitivity_sharing_banner_controls_sharing_banner_raw_payload(
) -> DatasetProvenanceCardSensitivitySharingBannerControlsPacket {
    let mut packet = seeded_dataset_provenance_card_sensitivity_sharing_banner_controls();
    packet.packet_id =
        "m5-dataset-provenance-card-sensitivity-sharing-banner-controls:fixture:sharing-banner-raw-payload"
            .to_owned();
    packet.surface_label =
        "M5 sensitivity/sharing banners: a raw-payload share is never the metadata-only default"
            .to_owned();
    packet
}
