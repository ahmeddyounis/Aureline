//! Two reusable M5 experiment components — the dataset provenance card and the
//! sensitivity / sharing banner — so a user can tell what data a result was built on and
//! whether a share stays metadata-first, privacy-safe, and location-aware from the component
//! alone, before any preview, compare, or share action: the provenance card names its
//! dataset / table, source class, version / snapshot / partition, row / file count or estimate,
//! sample / truncation state, sensitivity / redaction state, and local-versus-remote location,
//! and offers first-class open / inspect-provenance / export-metadata actions; the sharing
//! banner names its share class, blocked destinations, metadata-only-versus-raw-payload choice,
//! copy / export policy, and local-safe alternatives, and offers review / share-metadata-only
//! paths.
//!
//! Aureline's frozen experiment-component matrix
//! ([`crate::freeze_the_m5_experiment_run_row_dataset_provenance_card_artifact_lineage_panel_run_comparison_table_environment_fingerprint_card_compare_guard_banner_sensitivity_sharing_banner_and_result_summary_card_component_matrix`])
//! names the dataset provenance card and the sensitivity / sharing banner as two governed
//! component families and freezes their controlled vocabulary — the dataset source classes
//! (`tracked_dataset`, `local_file`, `remote_snapshot`, `synthetic_data`, `redacted_sample`,
//! `unknown_source`) and provenance states (`provenance_complete`, `provenance_partial`,
//! `provenance_missing`, `version_pinned`, `version_drifted`, `access_restricted`) a card
//! binds; the sensitivity classes (`public_safe`, `internal`, `confidential`, `regulated`,
//! `production_like`, `unknown_sensitivity`) and share scope states (`summary_only`,
//! `summary_plus_metadata`, `evidence_included`, `raw_payload_included`, `redacted_share`,
//! `share_blocked`) a banner binds; the one controlled disposition vocabulary; the surface
//! families; the deployment lines; the consumer surfaces; the accessibility routes; the
//! required labels; and the downgrade triggers. This module *implements* that contract as two
//! co-equal component vectors so a claimed M5 notebook, experiment-dashboard, comparison,
//! data-catalog, share-review, or CLI surface can project a dataset card and a sharing banner
//! that keep the same truth.
//!
//! The module has two derived resolvers:
//!
//! 1. [`resolve_dataset_provenance`] — takes a dataset card's source class and provenance state
//!    and derives its location class (local, remote, or unknown), whether the data is local,
//!    its provenance class (provenanced, pinned, partially provenanced, or unprovenanced),
//!    whether the dataset is fully provenanced, and which notes the card must carry — so a
//!    remote, unknown-location, partially provenanced, or unprovenanced dataset can never read
//!    as a fully-provenanced local dataset before a preview, compare, or share.
//! 2. [`resolve_share_scope`] — takes a banner's sensitivity class and share scope state and
//!    derives its share disposition (metadata-safe, evidence-scoped, raw-exposed, redacted, or
//!    blocked), whether the share includes a raw payload, whether it is metadata-only, whether
//!    it is blocked, whether the data is high-sensitivity, and which warnings the banner must
//!    carry — so a raw-payload share is never implied by default and metadata-only, sampled, or
//!    redacted states stay visible before a share.
//!
//! A single controls packet — [`DatasetProvenanceCardSensitivitySharingBannerControlsPacket`] —
//! binds one vector of dataset cards and one vector of sharing banners to the same source /
//! provenance / location, sensitivity / share-scope, deep-link, and non-visual accessibility
//! vocabulary, so dataset provenance and share safety stay explicit across desktop, headless /
//! export, and support consumers.
//!
//! The dataset source class ([`M5DatasetSourceClass`]), dataset provenance state
//! ([`M5DatasetProvenanceState`]), sensitivity class ([`M5SensitivityClass`]), share scope
//! state ([`M5ShareScopeState`]), disposition ([`M5ExperimentDisposition`]), surface family
//! ([`M5ExperimentSurfaceFamily`]), deployment line ([`M5ExperimentDeploymentLine`]), consumer
//! surface ([`M5ExperimentConsumerSurface`]), accessibility route
//! ([`M5ExperimentAccessibilityRoute`]), required label ([`M5ExperimentRequiredLabel`]), and
//! downgrade trigger ([`M5ExperimentDowngradeTrigger`]) are reused verbatim from the frozen
//! matrix. This module mints new vocabulary only for what that matrix left implicit about the
//! two components themselves: the derived location, provenance, and share-disposition classes,
//! the bounded dataset-card and sharing-banner actions, and the deep-link kinds. No M5
//! experiment surface invents a second dataset-card or sharing-banner grammar, and no card or
//! banner invents a data-specific privacy, redaction, retention, or share-class exception.
//!
//! Raw dataset payloads, pasted paths, credentials, and private endpoints stay outside the
//! export boundary; every context line, deep-link reference, and component identity is carried
//! only as an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_dataset_provenance_card_sensitivity_sharing_banner_controls,
    seeded_dataset_provenance_card_sensitivity_sharing_banner_controls_dataset_card_remote,
    seeded_dataset_provenance_card_sensitivity_sharing_banner_controls_sharing_banner_raw_payload,
    DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_PACKET_ID,
};

// The dataset source classes and provenance states, the sensitivity classes and share scope
// states, the disposition vocabulary, and the surface / deployment / consumer / accessibility /
// label / downgrade vocabularies are frozen once, in the experiment-component matrix. This lane
// reuses them verbatim so it never invents a parallel dataset-card or sharing-banner vocabulary.
pub use crate::freeze_the_m5_experiment_run_row_dataset_provenance_card_artifact_lineage_panel_run_comparison_table_environment_fingerprint_card_compare_guard_banner_sensitivity_sharing_banner_and_result_summary_card_component_matrix::{
    M5DatasetProvenanceState, M5DatasetSourceClass, M5ExperimentAccessibilityRoute,
    M5ExperimentComponentFamily, M5ExperimentConsumerSurface, M5ExperimentDeploymentLine,
    M5ExperimentDisposition, M5ExperimentDowngradeTrigger, M5ExperimentRequiredLabel,
    M5ExperimentSurfaceFamily, M5SensitivityClass, M5ShareScopeState,
    M5_DATASET_PROVENANCE_CARD_SCHEMA_REF, M5_EXPERIMENT_COMPONENT_DOC_REF,
    M5_EXPERIMENT_COMPONENT_SCHEMA_REF, M5_SENSITIVITY_SHARING_BANNER_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by
/// [`DatasetProvenanceCardSensitivitySharingBannerControlsPacket`].
pub const DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_RECORD_KIND: &str =
    "implement_m5_dataset_provenance_cards_and_sensitivity_sharing_banners_with_snapshot_sample_redaction_and_local_remote_location_truth_across_claimed_m5_data_lanes";

/// Schema version for M5 dataset-provenance-card / sensitivity-sharing-banner control records.
pub const DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_SCHEMA_REF: &str =
    "schemas/ui/m5-dataset-provenance-card-sensitivity-sharing-banner-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_DOC_REF: &str =
    "docs/notebooks/m5_dataset_provenance_card_sensitivity_sharing_banner_controls.md";

/// Repo-relative path of the protected fixture directory.
pub const DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-dataset-provenance-card-sensitivity-sharing-banner-controls";

/// Repo-relative path of the checked support-export artifact.
pub const DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_ARTIFACT_REF: &str =
    "artifacts/release/m5-dataset-provenance-card-sensitivity-sharing-banner-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_CSV_REF: &str =
    "artifacts/release/m5-dataset-provenance-card-sensitivity-sharing-banner-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_REPORT_REF: &str =
    "artifacts/design/m5-dataset-provenance-card-sensitivity-sharing-banner.md";

// ---- shared deep-link vocabulary ----------------------------------------

/// The kind of stable deep link a data component binds its next step against, so a dataset card
/// or sharing banner never routes through an ephemeral overlay — every next step is a stable
/// run, notebook, dataset-catalog, or docs reference the user can reopen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkKind {
    /// A stable experiment-run object reference.
    RunObject,
    /// A stable notebook / cell location.
    NotebookLocation,
    /// A stable dataset-catalog anchor.
    DatasetCatalogAnchor,
    /// A stable docs anchor.
    DocsAnchor,
    /// No deep link is bound (the component names that it routes nowhere).
    NoDeepLink,
}

impl DeepLinkKind {
    /// Every deep-link kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RunObject,
        Self::NotebookLocation,
        Self::DatasetCatalogAnchor,
        Self::DocsAnchor,
        Self::NoDeepLink,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunObject => "run_object",
            Self::NotebookLocation => "notebook_location",
            Self::DatasetCatalogAnchor => "dataset_catalog_anchor",
            Self::DocsAnchor => "docs_anchor",
            Self::NoDeepLink => "no_deep_link",
        }
    }

    /// True when this kind names a resolvable deep-link target.
    pub const fn is_resolvable(self) -> bool {
        !matches!(self, Self::NoDeepLink)
    }
}

// ---- dataset-provenance-card vocabulary ---------------------------------

/// Derived location class a dataset provenance card may present.
///
/// This is the dataset location honesty axis: the class is derived from the frozen dataset
/// source class, never asserted, so a remote or unknown-location dataset can never present as a
/// local dataset and a user can always tell whether the data is local, remote, or of unknown
/// location before trusting a preview, compare, or share.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatasetLocationClass {
    /// Data that lives locally or is a local-safe extract.
    LocalData,
    /// Data pulled from a remote store or snapshot.
    RemoteData,
    /// Data whose location could not be resolved.
    LocationUnknown,
}

impl DatasetLocationClass {
    /// Every location class, in declaration order.
    pub const ALL: [Self; 3] = [Self::LocalData, Self::RemoteData, Self::LocationUnknown];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalData => "local_data",
            Self::RemoteData => "remote_data",
            Self::LocationUnknown => "location_unknown",
        }
    }

    /// True when the dataset is reliably local.
    pub const fn is_local_data(self) -> bool {
        matches!(self, Self::LocalData)
    }
}

/// Derived provenance class a dataset provenance card may present.
///
/// This is the dataset provenance honesty axis: the class is derived from the frozen provenance
/// state, never asserted, so a missing, drifted, or access-restricted dataset can never present
/// as a fully-provenanced dataset and a user can always tell how completely the data is
/// provenanced before trusting a result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatasetProvenanceClass {
    /// Provenance is complete.
    Provenanced,
    /// Provenance is pinned to an explicit version.
    Pinned,
    /// Provenance is only partial.
    PartiallyProvenanced,
    /// Provenance is missing, drifted, or restricted (not reliably provenanced).
    Unprovenanced,
}

impl DatasetProvenanceClass {
    /// Every provenance class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Provenanced,
        Self::Pinned,
        Self::PartiallyProvenanced,
        Self::Unprovenanced,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Provenanced => "provenanced",
            Self::Pinned => "pinned",
            Self::PartiallyProvenanced => "partially_provenanced",
            Self::Unprovenanced => "unprovenanced",
        }
    }

    /// True when the dataset is reliably provenanced (fully provenanced or pinned).
    pub const fn is_fully_provenanced(self) -> bool {
        matches!(self, Self::Provenanced | Self::Pinned)
    }
}

/// One keyboard-complete default action a dataset provenance card offers, so a card never hides
/// its open / inspect / export path behind a pointer-only gesture. `OpenDataset`,
/// `InspectProvenance`, and `ExportMetadata` are always offered so dataset provenance is
/// actionable — and metadata-first — before any trust decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatasetCardAction {
    /// Open the dataset (always available).
    OpenDataset,
    /// Inspect the dataset's provenance (always available).
    InspectProvenance,
    /// Export the dataset's metadata only (always available).
    ExportMetadata,
    /// Open the stable run / notebook / dataset / docs deep link.
    OpenDeepLink,
    /// Compare this dataset against another.
    CompareDatasets,
    /// Copy the stable dataset id.
    CopyDatasetId,
}

impl DatasetCardAction {
    /// Every dataset-card action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenDataset,
        Self::InspectProvenance,
        Self::ExportMetadata,
        Self::OpenDeepLink,
        Self::CompareDatasets,
        Self::CopyDatasetId,
    ];

    /// The default actions every keyboard-complete dataset card must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::OpenDataset,
        Self::InspectProvenance,
        Self::ExportMetadata,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenDataset => "open_dataset",
            Self::InspectProvenance => "inspect_provenance",
            Self::ExportMetadata => "export_metadata",
            Self::OpenDeepLink => "open_deep_link",
            Self::CompareDatasets => "compare_datasets",
            Self::CopyDatasetId => "copy_dataset_id",
        }
    }
}

/// Disclosures a dataset provenance card must carry, derived from the source class and
/// provenance state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DatasetCardDisclosure {
    /// The derived location class this card may present.
    pub location_class: DatasetLocationClass,
    /// Whether the data is reliably local.
    pub is_local_data: bool,
    /// Whether the card must carry an explicit remote-location note.
    pub needs_remote_note: bool,
    /// Whether the card must carry an explicit unknown-location note.
    pub needs_unknown_location_note: bool,
    /// The derived provenance class this card may present.
    pub provenance_class: DatasetProvenanceClass,
    /// Whether the dataset is reliably provenanced.
    pub is_fully_provenanced: bool,
    /// Whether the card must carry an explicit partial-provenance note.
    pub needs_partial_note: bool,
    /// Whether the card must carry an explicit unprovenanced note.
    pub needs_unprovenanced_note: bool,
}

/// Resolves the location and provenance truth a dataset provenance card may present.
///
/// A `local_file`, `synthetic_data`, or `redacted_sample` dataset is local data. A
/// `tracked_dataset` or `remote_snapshot` dataset is remote data. An `unknown_source` dataset
/// has an unknown location, so a dataset that Aureline cannot place locally can never read as a
/// local dataset. A `provenance_complete` dataset is provenanced, a `version_pinned` dataset is
/// pinned, a `provenance_partial` dataset is partially provenanced, and a `provenance_missing`,
/// `version_drifted`, or `access_restricted` dataset is unprovenanced, so a dataset that was
/// not reliably provenanced can never read as fully provenanced.
pub fn resolve_dataset_provenance(
    source: M5DatasetSourceClass,
    state: M5DatasetProvenanceState,
) -> DatasetCardDisclosure {
    use DatasetLocationClass as Location;
    use DatasetProvenanceClass as Provenance;
    use M5DatasetProvenanceState as State;
    use M5DatasetSourceClass as Source;

    let location_class = match source {
        Source::LocalFile | Source::SyntheticData | Source::RedactedSample => Location::LocalData,
        Source::TrackedDataset | Source::RemoteSnapshot => Location::RemoteData,
        Source::UnknownSource => Location::LocationUnknown,
    };

    let provenance_class = match state {
        State::ProvenanceComplete => Provenance::Provenanced,
        State::VersionPinned => Provenance::Pinned,
        State::ProvenancePartial => Provenance::PartiallyProvenanced,
        State::ProvenanceMissing | State::VersionDrifted | State::AccessRestricted => {
            Provenance::Unprovenanced
        }
    };

    DatasetCardDisclosure {
        location_class,
        is_local_data: location_class.is_local_data(),
        needs_remote_note: matches!(location_class, Location::RemoteData),
        needs_unknown_location_note: matches!(location_class, Location::LocationUnknown),
        provenance_class,
        is_fully_provenanced: provenance_class.is_fully_provenanced(),
        needs_partial_note: matches!(provenance_class, Provenance::PartiallyProvenanced),
        needs_unprovenanced_note: matches!(provenance_class, Provenance::Unprovenanced),
    }
}

/// A dataset provenance card naming its dataset / table, source class, provenance state,
/// version / snapshot / partition, row / file count or estimate, sample / truncation state,
/// sensitivity / redaction state, derived location and provenance classes, bounded open /
/// inspect / export actions, and a stable deep link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatasetProvenanceCard {
    /// Frozen component this control implements; must be `dataset_provenance_card`.
    pub component: M5ExperimentComponentFamily,
    /// Stable dataset-card id.
    pub card_id: String,
    /// Human-readable dataset / table label; required and non-empty.
    pub dataset_label: String,
    /// Dataset source class, reused from the frozen matrix.
    pub source_class: M5DatasetSourceClass,
    /// Dataset provenance state, reused from the frozen matrix.
    pub provenance_state: M5DatasetProvenanceState,
    /// Derived location class (must equal the resolved class).
    pub location_class: DatasetLocationClass,
    /// Whether the card claims the data is local (must equal the derived truth).
    pub claims_local_data: bool,
    /// Derived provenance class (must equal the resolved class).
    pub provenance_class: DatasetProvenanceClass,
    /// Whether the card claims the dataset is fully provenanced (must equal the derived truth).
    pub claims_fully_provenanced: bool,
    /// Remote-location note; required when the data is remote.
    pub remote_location_note: String,
    /// Unknown-location note; required when the data's location is unknown.
    pub unknown_location_note: String,
    /// Partial-provenance note; required when the dataset is partially provenanced.
    pub partial_provenance_note: String,
    /// Unprovenanced note; required when the dataset is unprovenanced.
    pub unprovenanced_note: String,
    /// Source / provenance note; always required so source and provenance stay explicit.
    pub source_and_provenance_note: String,
    /// Version / snapshot / partition note; always required.
    pub version_snapshot_partition_note: String,
    /// Row / file count or estimate label; always required.
    pub row_or_file_count_label: String,
    /// Sample / truncation note; always required so sampled or truncated data stays explicit.
    pub sample_or_truncation_note: String,
    /// Whether this card represents sampled or truncated data (explicit, never inferred away).
    pub is_sampled_or_truncated: bool,
    /// Sensitivity class, reused from the frozen matrix so no data-specific exception is minted.
    pub sensitivity_class: M5SensitivityClass,
    /// Sensitivity / redaction note; always required so the redaction posture stays explicit.
    pub redaction_note: String,
    /// Context note; always required so the card names what to check before preview or share.
    pub context_note: String,
    /// Kind of stable deep link this card binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include open / inspect / export).
    pub card_actions: Vec<DatasetCardAction>,
    /// Dispositions this card binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5ExperimentDisposition>,
    /// Downgrade triggers this card can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5ExperimentDowngradeTrigger>,
    /// Mandatory labels this card can show (must include the mandatory labels).
    pub required_labels: Vec<M5ExperimentRequiredLabel>,
    /// Claimed M5 surface families that render this card.
    pub surface_families: Vec<M5ExperimentSurfaceFamily>,
    /// Deployment lines this card keeps the same truth across.
    pub deployment_lines: Vec<M5ExperimentDeploymentLine>,
    /// Non-visual accessibility routes this card offers.
    pub accessibility_routes: Vec<M5ExperimentAccessibilityRoute>,
    /// Experiment subsystems that consume this card's projection.
    pub consumer_surfaces: Vec<M5ExperimentConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this card.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never masks provenance or sensitivity state. MUST be `false`.
    pub masks_provenance_or_sensitivity_state: bool,
    /// Hard invariant: never hides dataset location or provenance. MUST be `false`.
    pub hides_dataset_location_or_provenance: bool,
    /// Hard invariant: never exposes a raw payload by default. MUST be `false`.
    pub exposes_raw_payload_by_default: bool,
    /// Hard invariant: never implies apples-to-apples without parity. MUST be `false`.
    pub implies_apples_to_apples_without_parity: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl DatasetProvenanceCard {
    /// Location and provenance disclosures this card must carry, derived from its state.
    pub fn provenance_disclosure(&self) -> DatasetCardDisclosure {
        resolve_dataset_provenance(self.source_class, self.provenance_state)
    }

    /// Whether the card offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<DatasetCardAction> = self.card_actions.iter().copied().collect();
        DatasetCardAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the card declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5ExperimentRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5ExperimentRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the card offers a deep-link-opening action.
    fn offers_deep_link_action(&self) -> bool {
        self.card_actions.contains(&DatasetCardAction::OpenDeepLink)
    }
}

// ---- sensitivity-sharing-banner vocabulary ------------------------------

/// Derived share disposition a sensitivity / sharing banner may present.
///
/// This is the share honesty axis: the disposition is derived from the frozen share scope
/// state, never asserted, so a raw-payload share can never present as a metadata-only share and
/// a user can always tell whether a share is metadata-safe, evidence-scoped, raw-exposed,
/// redacted, or blocked before sharing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShareDispositionClass {
    /// A metadata-only, metadata-safe share.
    MetadataSafe,
    /// A share that includes evidence links.
    EvidenceScoped,
    /// A share that includes a raw payload.
    RawExposed,
    /// A redacted share.
    Redacted,
    /// A blocked share.
    Blocked,
}

impl ShareDispositionClass {
    /// Every share disposition, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::MetadataSafe,
        Self::EvidenceScoped,
        Self::RawExposed,
        Self::Redacted,
        Self::Blocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafe => "metadata_safe",
            Self::EvidenceScoped => "evidence_scoped",
            Self::RawExposed => "raw_exposed",
            Self::Redacted => "redacted",
            Self::Blocked => "blocked",
        }
    }

    /// True when the share is metadata-only (metadata-safe).
    pub const fn is_metadata_only(self) -> bool {
        matches!(self, Self::MetadataSafe)
    }
}

/// One keyboard-complete default action a sensitivity / sharing banner offers, so a banner
/// never hides its safe alternative behind a pointer-only gesture. `ReviewShareScope` and
/// `ShareMetadataOnly` are always offered so the metadata-only, local-safe alternative stays
/// visible before any raw share.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShareBannerAction {
    /// Review the share scope (always available).
    ReviewShareScope,
    /// Share metadata only — the metadata-safe alternative (always available).
    ShareMetadataOnly,
    /// Export metadata only.
    ExportMetadataOnly,
    /// Open the stable run / notebook / dataset / docs deep link.
    OpenDeepLink,
    /// Copy a stable local-safe reference.
    CopyLocalSafeReference,
    /// Block the share.
    BlockShare,
}

impl ShareBannerAction {
    /// Every sharing-banner action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReviewShareScope,
        Self::ShareMetadataOnly,
        Self::ExportMetadataOnly,
        Self::OpenDeepLink,
        Self::CopyLocalSafeReference,
        Self::BlockShare,
    ];

    /// The default actions every keyboard-complete banner must offer.
    pub const MANDATORY: [Self; 2] = [Self::ReviewShareScope, Self::ShareMetadataOnly];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewShareScope => "review_share_scope",
            Self::ShareMetadataOnly => "share_metadata_only",
            Self::ExportMetadataOnly => "export_metadata_only",
            Self::OpenDeepLink => "open_deep_link",
            Self::CopyLocalSafeReference => "copy_local_safe_reference",
            Self::BlockShare => "block_share",
        }
    }
}

/// Disclosures a sensitivity / sharing banner must carry, derived from the sensitivity class and
/// share scope state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShareBannerDisclosure {
    /// The derived share disposition this banner may present.
    pub share_disposition: ShareDispositionClass,
    /// Whether the share includes a raw payload.
    pub includes_raw_payload: bool,
    /// Whether the share is metadata-only.
    pub is_metadata_only: bool,
    /// Whether the share is blocked.
    pub is_blocked: bool,
    /// Whether the banner must carry an explicit raw-payload warning.
    pub needs_raw_payload_warning: bool,
    /// Whether the banner must carry an explicit blocked note.
    pub needs_blocked_note: bool,
    /// Whether the banner must carry an explicit redaction note.
    pub needs_redaction_note: bool,
    /// Whether the data is high-sensitivity (confidential, regulated, or production-like).
    pub is_high_sensitivity: bool,
    /// Whether the banner must carry an explicit high-sensitivity warning.
    pub needs_sensitivity_warning: bool,
}

/// Resolves the share and sensitivity truth a sensitivity / sharing banner may present.
///
/// A `summary_only` or `summary_plus_metadata` share is metadata-safe. An `evidence_included`
/// share is evidence-scoped. A `raw_payload_included` share is raw-exposed (must carry an
/// explicit raw-payload warning). A `redacted_share` is redacted (must carry an explicit
/// redaction note). A `share_blocked` share is blocked (must carry an explicit blocked note).
/// A `confidential`, `regulated`, or `production_like` sensitivity is high-sensitivity, so raw
/// data is never implied by default and high-sensitivity data is always flagged.
pub fn resolve_share_scope(
    sensitivity: M5SensitivityClass,
    scope: M5ShareScopeState,
) -> ShareBannerDisclosure {
    use M5SensitivityClass as Sensitivity;
    use M5ShareScopeState as Scope;
    use ShareDispositionClass as Disposition;

    let share_disposition = match scope {
        Scope::SummaryOnly | Scope::SummaryPlusMetadata => Disposition::MetadataSafe,
        Scope::EvidenceIncluded => Disposition::EvidenceScoped,
        Scope::RawPayloadIncluded => Disposition::RawExposed,
        Scope::RedactedShare => Disposition::Redacted,
        Scope::ShareBlocked => Disposition::Blocked,
    };

    let is_high_sensitivity = matches!(
        sensitivity,
        Sensitivity::Confidential | Sensitivity::Regulated | Sensitivity::ProductionLike
    );

    ShareBannerDisclosure {
        share_disposition,
        includes_raw_payload: matches!(share_disposition, Disposition::RawExposed),
        is_metadata_only: share_disposition.is_metadata_only(),
        is_blocked: matches!(share_disposition, Disposition::Blocked),
        needs_raw_payload_warning: matches!(share_disposition, Disposition::RawExposed),
        needs_blocked_note: matches!(share_disposition, Disposition::Blocked),
        needs_redaction_note: matches!(share_disposition, Disposition::Redacted),
        is_high_sensitivity,
        needs_sensitivity_warning: is_high_sensitivity,
    }
}

/// A sensitivity / sharing banner naming its sensitivity class, share scope state, share class,
/// blocked destinations, metadata-only-versus-raw-payload choice, copy / export policy,
/// local-safe alternatives, derived share disposition, bounded review / metadata-only actions,
/// and a stable deep link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SensitivitySharingBanner {
    /// Frozen component this control implements; must be `sensitivity_sharing_banner`.
    pub component: M5ExperimentComponentFamily,
    /// Stable banner id.
    pub banner_id: String,
    /// Human-readable banner label; required and non-empty.
    pub banner_label: String,
    /// Sensitivity class, reused from the frozen matrix.
    pub sensitivity_class: M5SensitivityClass,
    /// Share scope state, reused from the frozen matrix.
    pub share_scope_state: M5ShareScopeState,
    /// Derived share disposition (must equal the resolved disposition).
    pub share_disposition: ShareDispositionClass,
    /// Whether the banner claims the share includes a raw payload (must equal derived truth).
    pub claims_includes_raw_payload: bool,
    /// Whether the banner claims the share is metadata-only (must equal derived truth).
    pub claims_metadata_only: bool,
    /// Raw-payload warning; required when the share includes a raw payload.
    pub raw_payload_warning: String,
    /// Blocked note; required when the share is blocked.
    pub blocked_note: String,
    /// Redaction note; required when the share is redacted.
    pub redaction_note: String,
    /// High-sensitivity warning; required when the data is high-sensitivity.
    pub sensitivity_warning: String,
    /// Sensitivity / share-class note; always required so sensitivity stays explicit.
    pub sensitivity_and_share_class_note: String,
    /// Blocked-destinations note; always required so blocked destinations stay explicit.
    pub blocked_destinations_note: String,
    /// Copy / export policy note; always required.
    pub copy_export_policy_note: String,
    /// Local-safe alternative note; always required so a safe alternative stays visible.
    pub local_safe_alternative_note: String,
    /// Context note; always required so the banner names what to check before a share.
    pub context_note: String,
    /// Kind of stable deep link this banner binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include review / share-metadata-only).
    pub banner_actions: Vec<ShareBannerAction>,
    /// Dispositions this banner binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5ExperimentDisposition>,
    /// Downgrade triggers this banner can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5ExperimentDowngradeTrigger>,
    /// Mandatory labels this banner can show (must include the mandatory labels).
    pub required_labels: Vec<M5ExperimentRequiredLabel>,
    /// Claimed M5 surface families that render this banner.
    pub surface_families: Vec<M5ExperimentSurfaceFamily>,
    /// Deployment lines this banner keeps the same truth across.
    pub deployment_lines: Vec<M5ExperimentDeploymentLine>,
    /// Non-visual accessibility routes this banner offers.
    pub accessibility_routes: Vec<M5ExperimentAccessibilityRoute>,
    /// Experiment subsystems that consume this banner's projection.
    pub consumer_surfaces: Vec<M5ExperimentConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this banner.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never masks provenance or sensitivity state. MUST be `false`.
    pub masks_provenance_or_sensitivity_state: bool,
    /// Hard invariant: never hides dataset location or provenance. MUST be `false`.
    pub hides_dataset_location_or_provenance: bool,
    /// Hard invariant: never exposes a raw payload by default. MUST be `false`.
    pub exposes_raw_payload_by_default: bool,
    /// Hard invariant: never implies apples-to-apples without parity. MUST be `false`.
    pub implies_apples_to_apples_without_parity: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl SensitivitySharingBanner {
    /// Share and sensitivity disclosures this banner must carry, derived from its state.
    pub fn share_disclosure(&self) -> ShareBannerDisclosure {
        resolve_share_scope(self.sensitivity_class, self.share_scope_state)
    }

    /// Whether the banner offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<ShareBannerAction> = self.banner_actions.iter().copied().collect();
        ShareBannerAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the banner declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5ExperimentRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5ExperimentRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the banner offers a deep-link-opening action.
    fn offers_deep_link_action(&self) -> bool {
        self.banner_actions
            .contains(&ShareBannerAction::OpenDeepLink)
    }
}

// ---- review blocks ------------------------------------------------------

/// First-glance dataset / sharing review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatasetSensitivityReview {
    /// The dataset card names its source and provenance.
    pub dataset_card_shows_source_and_provenance: bool,
    /// The dataset card names its location and sample / truncation state.
    pub dataset_card_shows_location_and_sample_state: bool,
    /// The dataset card offers open, inspect, and metadata-export.
    pub dataset_card_offers_open_inspect_export: bool,
    /// The sharing banner names its sensitivity class and share scope.
    pub share_banner_shows_sensitivity_and_scope: bool,
    /// The sharing banner offers review and share-metadata-only.
    pub share_banner_offers_review_and_metadata_only: bool,
    /// Location and provenance are derived from state, never asserted.
    pub location_and_provenance_derived_never_asserted: bool,
    /// A remote or unknown-location dataset is never shown as local.
    pub remote_or_unknown_never_shown_as_local: bool,
    /// An unprovenanced dataset is never shown as fully provenanced.
    pub unprovenanced_never_shown_as_provenanced: bool,
    /// Raw data is never implied by default.
    pub raw_payload_never_implied_by_default: bool,
    /// Metadata-only and sampled / redacted state stays visible before preview, compare, share.
    pub metadata_only_and_sampled_state_visible_before_share: bool,
    /// Every next step names one stable run / notebook / dataset / docs deep link.
    pub every_next_step_names_stable_deep_link: bool,
    /// Cards and banners reuse Aureline's existing privacy / redaction / share vocabulary.
    pub reuses_existing_privacy_redaction_share_vocabulary: bool,
    /// Provenance and sensitivity state stays visible.
    pub provenance_and_sensitivity_state_visible: bool,
    /// Cached, offline, and local-only state stays visible.
    pub cached_offline_local_only_state_visible: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// The components keep the same truth across every deployment line.
    pub components_stable_across_deployment_lines: bool,
    /// The components stay copy and export safe.
    pub copy_and_export_safe: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl DatasetSensitivityReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.dataset_card_shows_source_and_provenance
            && self.dataset_card_shows_location_and_sample_state
            && self.dataset_card_offers_open_inspect_export
            && self.share_banner_shows_sensitivity_and_scope
            && self.share_banner_offers_review_and_metadata_only
            && self.location_and_provenance_derived_never_asserted
            && self.remote_or_unknown_never_shown_as_local
            && self.unprovenanced_never_shown_as_provenanced
            && self.raw_payload_never_implied_by_default
            && self.metadata_only_and_sampled_state_visible_before_share
            && self.every_next_step_names_stable_deep_link
            && self.reuses_existing_privacy_redaction_share_vocabulary
            && self.provenance_and_sensitivity_state_visible
            && self.cached_offline_local_only_state_visible
            && self.no_surface_invents_alternate_state_label
            && self.components_stable_across_deployment_lines
            && self.copy_and_export_safe
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatasetSensitivityConsumerProjection {
    /// The dataset catalog reads a single canonical source.
    pub dataset_catalog_reads_single_source: bool,
    /// The share review sheet reads a single canonical source.
    pub share_review_sheet_reads_single_source: bool,
    /// Provenance and location are visible before preview or compare.
    pub provenance_and_location_visible_before_preview_or_compare: bool,
    /// Sensitivity and scope are visible before share.
    pub sensitivity_and_scope_visible_before_share: bool,
    /// Support export shows component truth.
    pub support_export_shows_component_truth: bool,
    /// Help / docs shows component truth.
    pub help_docs_shows_component_truth: bool,
}

impl DatasetSensitivityConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.dataset_catalog_reads_single_source
            && self.share_review_sheet_reads_single_source
            && self.provenance_and_location_visible_before_preview_or_compare
            && self.sensitivity_and_scope_visible_before_share
            && self.support_export_shows_component_truth
            && self.help_docs_shows_component_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatasetSensitivityProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for
/// [`DatasetProvenanceCardSensitivitySharingBannerControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetProvenanceCardSensitivitySharingBannerControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Dataset provenance cards.
    pub dataset_cards: Vec<DatasetProvenanceCard>,
    /// Sensitivity / sharing banners.
    pub sharing_banners: Vec<SensitivitySharingBanner>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5ExperimentDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5ExperimentConsumerSurface>,
    /// Dataset / sharing review block.
    pub dataset_review: DatasetSensitivityReview,
    /// Consumer projection block.
    pub consumer_projection: DatasetSensitivityConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: DatasetSensitivityProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe dataset-provenance-card / sensitivity-sharing-banner controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatasetProvenanceCardSensitivitySharingBannerControlsPacket {
    /// Record kind; must equal
    /// [`DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Dataset provenance cards.
    pub dataset_cards: Vec<DatasetProvenanceCard>,
    /// Sensitivity / sharing banners.
    pub sharing_banners: Vec<SensitivitySharingBanner>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5ExperimentDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5ExperimentConsumerSurface>,
    /// Dataset / sharing review block.
    pub dataset_review: DatasetSensitivityReview,
    /// Consumer projection block.
    pub consumer_projection: DatasetSensitivityConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: DatasetSensitivityProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl DatasetProvenanceCardSensitivitySharingBannerControlsPacket {
    /// Builds a dataset-provenance-card / sensitivity-sharing-banner controls packet from
    /// stable-lane input.
    pub fn new(input: DatasetProvenanceCardSensitivitySharingBannerControlsPacketInput) -> Self {
        Self {
            record_kind: DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_RECORD_KIND.to_owned(),
            schema_version: DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            dataset_cards: input.dataset_cards,
            sharing_banners: input.sharing_banners,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            dataset_review: input.dataset_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the dataset-provenance-card / sensitivity-sharing-banner control invariants.
    pub fn validate(&self) -> Vec<DatasetProvenanceCardSensitivitySharingBannerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_RECORD_KIND {
            violations
                .push(DatasetProvenanceCardSensitivitySharingBannerViolation::WrongRecordKind);
        }
        if self.schema_version != DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_SCHEMA_VERSION
        {
            violations
                .push(DatasetProvenanceCardSensitivitySharingBannerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations
                .push(DatasetProvenanceCardSensitivitySharingBannerViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::DowngradeTriggersMissing,
            );
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::ConsumerSurfacesMissing,
            );
        }

        validate_source_contracts(self, &mut violations);
        validate_dataset_cards(self, &mut violations);
        validate_sharing_banners(self, &mut violations);

        if !self.dataset_review.all_hold() {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::DatasetReviewIncomplete,
            );
        }
        if !self.consumer_projection.all_hold() {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::ConsumerProjectionIncomplete,
            );
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::ProofFreshnessIncomplete,
            );
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("dataset provenance sharing banner packet serializes"),
        ) {
            violations
                .push(DatasetProvenanceCardSensitivitySharingBannerViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("dataset provenance sharing banner packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component,id,state_or_scope,source_or_sensitivity,derived,safe_flag,deep_link_kind\n",
        );
        for card in &self.dataset_cards {
            let disclosure = card.provenance_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "dataset_provenance_card",
                csv_field(&card.card_id),
                card.provenance_state.as_str(),
                card.source_class.as_str(),
                disclosure.location_class.as_str(),
                disclosure.is_local_data,
                card.deep_link_kind.as_str(),
            ));
        }
        for banner in &self.sharing_banners {
            let disclosure = banner.share_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "sensitivity_sharing_banner",
                csv_field(&banner.banner_id),
                banner.share_scope_state.as_str(),
                banner.sensitivity_class.as_str(),
                disclosure.share_disposition.as_str(),
                disclosure.is_metadata_only,
                banner.deep_link_kind.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let non_local = self
            .dataset_cards
            .iter()
            .filter(|card| !card.provenance_disclosure().is_local_data)
            .count();
        let raw_shares = self
            .sharing_banners
            .iter()
            .filter(|banner| banner.share_disclosure().includes_raw_payload)
            .count();

        let mut out = String::new();
        out.push_str("# Dataset provenance cards and sensitivity / sharing banners\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Dataset provenance cards: {} ({} not local)\n",
            self.dataset_cards.len(),
            non_local
        ));
        out.push_str(&format!(
            "- Sensitivity / sharing banners: {} ({} include a raw payload)\n",
            self.sharing_banners.len(),
            raw_shares
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Dataset provenance cards\n\n");
        for card in &self.dataset_cards {
            let disclosure = card.provenance_disclosure();
            out.push_str(&format!(
                "- **{}** — source `{}`, provenance `{}` → `{}` / `{}`, sensitivity `{}`, deep link `{}`\n",
                card.dataset_label,
                card.source_class.as_str(),
                card.provenance_state.as_str(),
                disclosure.location_class.as_str(),
                disclosure.provenance_class.as_str(),
                card.sensitivity_class.as_str(),
                card.deep_link_kind.as_str(),
            ));
        }

        out.push_str("\n## Sensitivity / sharing banners\n\n");
        for banner in &self.sharing_banners {
            let disclosure = banner.share_disclosure();
            out.push_str(&format!(
                "- **{}** — sensitivity `{}`, scope `{}` → `{}`, deep link `{}`\n",
                banner.banner_label,
                banner.sensitivity_class.as_str(),
                banner.share_scope_state.as_str(),
                disclosure.share_disposition.as_str(),
                banner.deep_link_kind.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in dataset-provenance-card / sensitivity-sharing-banner
/// export.
#[derive(Debug)]
pub enum DatasetProvenanceCardSensitivitySharingBannerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<DatasetProvenanceCardSensitivitySharingBannerViolation>),
}

impl fmt::Display for DatasetProvenanceCardSensitivitySharingBannerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "dataset provenance sharing banner export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "dataset provenance sharing banner export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for DatasetProvenanceCardSensitivitySharingBannerArtifactError {}

/// Validation failures emitted by
/// [`DatasetProvenanceCardSensitivitySharingBannerControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DatasetProvenanceCardSensitivitySharingBannerViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No dataset provenance cards are present.
    DatasetCardsMissing,
    /// A dataset provenance card is incomplete.
    DatasetCardIncomplete,
    /// A dataset provenance card carries the wrong frozen component class.
    DatasetCardWrongComponentClass,
    /// A dataset card misrepresents its derived location class.
    LocationMisrepresented,
    /// A dataset card misrepresents its derived provenance class.
    ProvenanceMisrepresented,
    /// A remote dataset does not name its remote location.
    RemoteNoteMissing,
    /// An unknown-location dataset does not name its unknown location.
    UnknownLocationNoteMissing,
    /// A partially provenanced dataset does not name its partial state.
    PartialProvenanceNoteMissing,
    /// An unprovenanced dataset does not name its unprovenanced state.
    UnprovenancedNoteMissing,
    /// A dataset card does not name its source / provenance.
    SourceAndProvenanceNoteMissing,
    /// A dataset card does not name its version / snapshot / partition.
    VersionSnapshotPartitionMissing,
    /// A dataset card does not name its row / file count or estimate.
    RowOrFileCountMissing,
    /// A dataset card does not name its sample / truncation state.
    SampleOrTruncationNoteMissing,
    /// A dataset card does not name its sensitivity / redaction state.
    RedactionNoteMissing,
    /// A dataset card omits a mandatory open / inspect / export action.
    DatasetCardActionsIncomplete,
    /// The dataset cards do not cover every derived location class.
    LocationClassCoverageMissing,
    /// The dataset cards do not cover every derived provenance class.
    ProvenanceClassCoverageMissing,
    /// The dataset cards do not cover every dataset source class.
    DatasetSourceClassCoverageMissing,
    /// The dataset cards do not cover every dataset provenance state.
    DatasetProvenanceStateCoverageMissing,
    /// No sensitivity / sharing banners are present.
    SharingBannersMissing,
    /// A sensitivity / sharing banner is incomplete.
    SharingBannerIncomplete,
    /// A sensitivity / sharing banner carries the wrong frozen component class.
    SharingBannerWrongComponentClass,
    /// A sharing banner misrepresents its derived share disposition.
    ShareDispositionMisrepresented,
    /// A raw-payload share does not name its raw-payload warning.
    RawPayloadWarningMissing,
    /// A blocked share does not name its blocked state.
    BlockedNoteMissing,
    /// A redacted share does not name its redaction state.
    ShareRedactionNoteMissing,
    /// A high-sensitivity banner does not name its sensitivity warning.
    SensitivityWarningMissing,
    /// A sharing banner does not name its sensitivity / share class.
    SensitivityAndShareClassNoteMissing,
    /// A sharing banner does not name its blocked destinations.
    BlockedDestinationsNoteMissing,
    /// A sharing banner does not name its copy / export policy.
    CopyExportPolicyNoteMissing,
    /// A sharing banner does not name its local-safe alternative.
    LocalSafeAlternativeNoteMissing,
    /// A sharing banner omits a mandatory review / share-metadata-only action.
    SharingBannerActionsIncomplete,
    /// The sharing banners do not cover every derived share disposition.
    ShareDispositionCoverageMissing,
    /// The sharing banners do not cover every sensitivity class.
    SensitivityClassCoverageMissing,
    /// The sharing banners do not cover every share scope state.
    ShareScopeStateCoverageMissing,
    /// A component does not name its context.
    ContextNoteMissing,
    /// A component offers a deep-link action but its deep link does not resolve exactly.
    DeepLinkUnresolved,
    /// A component names a deep-link kind but not its stable reference.
    DeepLinkRefMissing,
    /// A component does not bind any disposition.
    DispositionsMissing,
    /// A component does not declare its downgrade triggers.
    DowngradeTriggersMissing,
    /// A component does not declare its mandatory labels.
    RequiredLabelsIncomplete,
    /// A component does not declare an accessibility route (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A component masks its provenance or sensitivity state.
    ProvenanceOrSensitivityStateMasked,
    /// A component hides dataset location or provenance.
    DatasetLocationOrProvenanceHidden,
    /// A component exposes a raw payload by default.
    RawPayloadExposedByDefault,
    /// A component implies apples-to-apples without parity evidence.
    ApplesToApplesImpliedWithoutParity,
    /// A component invents an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Dataset / sharing review does not satisfy required invariants.
    DatasetReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl DatasetProvenanceCardSensitivitySharingBannerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::DatasetCardsMissing => "dataset_cards_missing",
            Self::DatasetCardIncomplete => "dataset_card_incomplete",
            Self::DatasetCardWrongComponentClass => "dataset_card_wrong_component_class",
            Self::LocationMisrepresented => "location_misrepresented",
            Self::ProvenanceMisrepresented => "provenance_misrepresented",
            Self::RemoteNoteMissing => "remote_note_missing",
            Self::UnknownLocationNoteMissing => "unknown_location_note_missing",
            Self::PartialProvenanceNoteMissing => "partial_provenance_note_missing",
            Self::UnprovenancedNoteMissing => "unprovenanced_note_missing",
            Self::SourceAndProvenanceNoteMissing => "source_and_provenance_note_missing",
            Self::VersionSnapshotPartitionMissing => "version_snapshot_partition_missing",
            Self::RowOrFileCountMissing => "row_or_file_count_missing",
            Self::SampleOrTruncationNoteMissing => "sample_or_truncation_note_missing",
            Self::RedactionNoteMissing => "redaction_note_missing",
            Self::DatasetCardActionsIncomplete => "dataset_card_actions_incomplete",
            Self::LocationClassCoverageMissing => "location_class_coverage_missing",
            Self::ProvenanceClassCoverageMissing => "provenance_class_coverage_missing",
            Self::DatasetSourceClassCoverageMissing => "dataset_source_class_coverage_missing",
            Self::DatasetProvenanceStateCoverageMissing => {
                "dataset_provenance_state_coverage_missing"
            }
            Self::SharingBannersMissing => "sharing_banners_missing",
            Self::SharingBannerIncomplete => "sharing_banner_incomplete",
            Self::SharingBannerWrongComponentClass => "sharing_banner_wrong_component_class",
            Self::ShareDispositionMisrepresented => "share_disposition_misrepresented",
            Self::RawPayloadWarningMissing => "raw_payload_warning_missing",
            Self::BlockedNoteMissing => "blocked_note_missing",
            Self::ShareRedactionNoteMissing => "share_redaction_note_missing",
            Self::SensitivityWarningMissing => "sensitivity_warning_missing",
            Self::SensitivityAndShareClassNoteMissing => "sensitivity_and_share_class_note_missing",
            Self::BlockedDestinationsNoteMissing => "blocked_destinations_note_missing",
            Self::CopyExportPolicyNoteMissing => "copy_export_policy_note_missing",
            Self::LocalSafeAlternativeNoteMissing => "local_safe_alternative_note_missing",
            Self::SharingBannerActionsIncomplete => "sharing_banner_actions_incomplete",
            Self::ShareDispositionCoverageMissing => "share_disposition_coverage_missing",
            Self::SensitivityClassCoverageMissing => "sensitivity_class_coverage_missing",
            Self::ShareScopeStateCoverageMissing => "share_scope_state_coverage_missing",
            Self::ContextNoteMissing => "context_note_missing",
            Self::DeepLinkUnresolved => "deep_link_unresolved",
            Self::DeepLinkRefMissing => "deep_link_ref_missing",
            Self::DispositionsMissing => "dispositions_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RequiredLabelsIncomplete => "required_labels_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ProvenanceOrSensitivityStateMasked => "provenance_or_sensitivity_state_masked",
            Self::DatasetLocationOrProvenanceHidden => "dataset_location_or_provenance_hidden",
            Self::RawPayloadExposedByDefault => "raw_payload_exposed_by_default",
            Self::ApplesToApplesImpliedWithoutParity => "apples_to_apples_implied_without_parity",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DatasetReviewIncomplete => "dataset_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable dataset-provenance-card /
/// sensitivity-sharing-banner export.
pub fn current_dataset_provenance_card_sensitivity_sharing_banner_export() -> Result<
    DatasetProvenanceCardSensitivitySharingBannerControlsPacket,
    DatasetProvenanceCardSensitivitySharingBannerArtifactError,
> {
    let packet: DatasetProvenanceCardSensitivitySharingBannerControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-dataset-provenance-card-sensitivity-sharing-banner-proof/support_export.json"
        )))
        .map_err(DatasetProvenanceCardSensitivitySharingBannerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(DatasetProvenanceCardSensitivitySharingBannerArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &DatasetProvenanceCardSensitivitySharingBannerControlsPacket,
    violations: &mut Vec<DatasetProvenanceCardSensitivitySharingBannerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_SCHEMA_REF,
        DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_DOC_REF,
        M5_EXPERIMENT_COMPONENT_SCHEMA_REF,
        M5_EXPERIMENT_COMPONENT_DOC_REF,
        M5_DATASET_PROVENANCE_CARD_SCHEMA_REF,
        M5_SENSITIVITY_SHARING_BANNER_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::MissingSourceContracts,
            );
            return;
        }
    }
}

fn validate_dataset_cards(
    packet: &DatasetProvenanceCardSensitivitySharingBannerControlsPacket,
    violations: &mut Vec<DatasetProvenanceCardSensitivitySharingBannerViolation>,
) {
    if packet.dataset_cards.is_empty() {
        violations
            .push(DatasetProvenanceCardSensitivitySharingBannerViolation::DatasetCardsMissing);
        return;
    }

    let mut location_classes: BTreeSet<DatasetLocationClass> = BTreeSet::new();
    let mut provenance_classes: BTreeSet<DatasetProvenanceClass> = BTreeSet::new();
    let mut sources: BTreeSet<M5DatasetSourceClass> = BTreeSet::new();
    let mut states: BTreeSet<M5DatasetProvenanceState> = BTreeSet::new();

    for card in &packet.dataset_cards {
        let disclosure = card.provenance_disclosure();
        location_classes.insert(disclosure.location_class);
        provenance_classes.insert(disclosure.provenance_class);
        sources.insert(card.source_class);
        states.insert(card.provenance_state);

        if card.card_id.trim().is_empty()
            || card.dataset_label.trim().is_empty()
            || card.fields_shown.is_empty()
            || card.surface_families.is_empty()
            || card.deployment_lines.is_empty()
            || card.consumer_surfaces.is_empty()
            || card.source_contract_refs.is_empty()
        {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::DatasetCardIncomplete,
            );
        }
        if card.component != M5ExperimentComponentFamily::DatasetProvenanceCard {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::DatasetCardWrongComponentClass,
            );
        }
        if card.location_class != disclosure.location_class
            || card.claims_local_data != disclosure.is_local_data
        {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::LocationMisrepresented,
            );
        }
        if card.provenance_class != disclosure.provenance_class
            || card.claims_fully_provenanced != disclosure.is_fully_provenanced
        {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::ProvenanceMisrepresented,
            );
        }
        if disclosure.needs_remote_note && card.remote_location_note.trim().is_empty() {
            violations
                .push(DatasetProvenanceCardSensitivitySharingBannerViolation::RemoteNoteMissing);
        }
        if disclosure.needs_unknown_location_note && card.unknown_location_note.trim().is_empty() {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::UnknownLocationNoteMissing,
            );
        }
        if disclosure.needs_partial_note && card.partial_provenance_note.trim().is_empty() {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::PartialProvenanceNoteMissing,
            );
        }
        if disclosure.needs_unprovenanced_note && card.unprovenanced_note.trim().is_empty() {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::UnprovenancedNoteMissing,
            );
        }
        if card.source_and_provenance_note.trim().is_empty() {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::SourceAndProvenanceNoteMissing,
            );
        }
        if card.version_snapshot_partition_note.trim().is_empty() {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::VersionSnapshotPartitionMissing,
            );
        }
        if card.row_or_file_count_label.trim().is_empty() {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::RowOrFileCountMissing,
            );
        }
        if card.sample_or_truncation_note.trim().is_empty() {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::SampleOrTruncationNoteMissing,
            );
        }
        if card.redaction_note.trim().is_empty() {
            violations
                .push(DatasetProvenanceCardSensitivitySharingBannerViolation::RedactionNoteMissing);
        }
        if !card.declares_mandatory_actions() {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::DatasetCardActionsIncomplete,
            );
        }
        validate_deep_link(
            card.offers_deep_link_action(),
            card.deep_link_kind,
            &card.deep_link_ref,
            &card.context_note,
            violations,
        );
        validate_common_control(
            &card.dispositions,
            &card.downgrade_triggers,
            card.declares_mandatory_labels(),
            &card.accessibility_routes,
            ControlInvariants {
                masks_provenance_or_sensitivity_state: card.masks_provenance_or_sensitivity_state,
                hides_dataset_location_or_provenance: card.hides_dataset_location_or_provenance,
                exposes_raw_payload_by_default: card.exposes_raw_payload_by_default,
                implies_apples_to_apples_without_parity: card
                    .implies_apples_to_apples_without_parity,
                invents_alternate_state_label: card.invents_alternate_state_label,
            },
            violations,
        );
    }

    for required in DatasetLocationClass::ALL {
        if !location_classes.contains(&required) {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::LocationClassCoverageMissing,
            );
            break;
        }
    }
    for required in DatasetProvenanceClass::ALL {
        if !provenance_classes.contains(&required) {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::ProvenanceClassCoverageMissing,
            );
            break;
        }
    }
    for required in M5DatasetSourceClass::ALL {
        if !sources.contains(&required) {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::DatasetSourceClassCoverageMissing,
            );
            break;
        }
    }
    for required in M5DatasetProvenanceState::ALL {
        if !states.contains(&required) {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::DatasetProvenanceStateCoverageMissing,
            );
            break;
        }
    }
}

fn validate_sharing_banners(
    packet: &DatasetProvenanceCardSensitivitySharingBannerControlsPacket,
    violations: &mut Vec<DatasetProvenanceCardSensitivitySharingBannerViolation>,
) {
    if packet.sharing_banners.is_empty() {
        violations
            .push(DatasetProvenanceCardSensitivitySharingBannerViolation::SharingBannersMissing);
        return;
    }

    let mut dispositions: BTreeSet<ShareDispositionClass> = BTreeSet::new();
    let mut sensitivities: BTreeSet<M5SensitivityClass> = BTreeSet::new();
    let mut scopes: BTreeSet<M5ShareScopeState> = BTreeSet::new();

    for banner in &packet.sharing_banners {
        let disclosure = banner.share_disclosure();
        dispositions.insert(disclosure.share_disposition);
        sensitivities.insert(banner.sensitivity_class);
        scopes.insert(banner.share_scope_state);

        if banner.banner_id.trim().is_empty()
            || banner.banner_label.trim().is_empty()
            || banner.fields_shown.is_empty()
            || banner.surface_families.is_empty()
            || banner.deployment_lines.is_empty()
            || banner.consumer_surfaces.is_empty()
            || banner.source_contract_refs.is_empty()
        {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::SharingBannerIncomplete,
            );
        }
        if banner.component != M5ExperimentComponentFamily::SensitivitySharingBanner {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::SharingBannerWrongComponentClass,
            );
        }
        if banner.share_disposition != disclosure.share_disposition
            || banner.claims_includes_raw_payload != disclosure.includes_raw_payload
            || banner.claims_metadata_only != disclosure.is_metadata_only
        {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::ShareDispositionMisrepresented,
            );
        }
        if disclosure.needs_raw_payload_warning && banner.raw_payload_warning.trim().is_empty() {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::RawPayloadWarningMissing,
            );
        }
        if disclosure.needs_blocked_note && banner.blocked_note.trim().is_empty() {
            violations
                .push(DatasetProvenanceCardSensitivitySharingBannerViolation::BlockedNoteMissing);
        }
        if disclosure.needs_redaction_note && banner.redaction_note.trim().is_empty() {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::ShareRedactionNoteMissing,
            );
        }
        if disclosure.needs_sensitivity_warning && banner.sensitivity_warning.trim().is_empty() {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::SensitivityWarningMissing,
            );
        }
        if banner.sensitivity_and_share_class_note.trim().is_empty() {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::SensitivityAndShareClassNoteMissing,
            );
        }
        if banner.blocked_destinations_note.trim().is_empty() {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::BlockedDestinationsNoteMissing,
            );
        }
        if banner.copy_export_policy_note.trim().is_empty() {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::CopyExportPolicyNoteMissing,
            );
        }
        if banner.local_safe_alternative_note.trim().is_empty() {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::LocalSafeAlternativeNoteMissing,
            );
        }
        if !banner.declares_mandatory_actions() {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::SharingBannerActionsIncomplete,
            );
        }
        validate_deep_link(
            banner.offers_deep_link_action(),
            banner.deep_link_kind,
            &banner.deep_link_ref,
            &banner.context_note,
            violations,
        );
        validate_common_control(
            &banner.dispositions,
            &banner.downgrade_triggers,
            banner.declares_mandatory_labels(),
            &banner.accessibility_routes,
            ControlInvariants {
                masks_provenance_or_sensitivity_state: banner.masks_provenance_or_sensitivity_state,
                hides_dataset_location_or_provenance: banner.hides_dataset_location_or_provenance,
                exposes_raw_payload_by_default: banner.exposes_raw_payload_by_default,
                implies_apples_to_apples_without_parity: banner
                    .implies_apples_to_apples_without_parity,
                invents_alternate_state_label: banner.invents_alternate_state_label,
            },
            violations,
        );
    }

    for required in ShareDispositionClass::ALL {
        if !dispositions.contains(&required) {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::ShareDispositionCoverageMissing,
            );
            break;
        }
    }
    for required in M5SensitivityClass::ALL {
        if !sensitivities.contains(&required) {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::SensitivityClassCoverageMissing,
            );
            break;
        }
    }
    for required in M5ShareScopeState::ALL {
        if !scopes.contains(&required) {
            violations.push(
                DatasetProvenanceCardSensitivitySharingBannerViolation::ShareScopeStateCoverageMissing,
            );
            break;
        }
    }
}

/// Validates the context and stable deep-link truth shared by both component vectors.
///
/// A component that offers a deep-link action must name a resolvable deep-link kind, a
/// component that names a resolvable kind must carry its stable reference, and every component
/// must name its context — so a next step is never an ephemeral overlay or hidden route.
fn validate_deep_link(
    offers_deep_link_action: bool,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    context_note: &str,
    violations: &mut Vec<DatasetProvenanceCardSensitivitySharingBannerViolation>,
) {
    if context_note.trim().is_empty() {
        violations.push(DatasetProvenanceCardSensitivitySharingBannerViolation::ContextNoteMissing);
    }
    if offers_deep_link_action && !deep_link_kind.is_resolvable() {
        violations.push(DatasetProvenanceCardSensitivitySharingBannerViolation::DeepLinkUnresolved);
    }
    if deep_link_kind.is_resolvable() && deep_link_ref.trim().is_empty() {
        violations.push(DatasetProvenanceCardSensitivitySharingBannerViolation::DeepLinkRefMissing);
    }
}

/// The five hard-invariant bools every component must keep `false`.
struct ControlInvariants {
    masks_provenance_or_sensitivity_state: bool,
    hides_dataset_location_or_provenance: bool,
    exposes_raw_payload_by_default: bool,
    implies_apples_to_apples_without_parity: bool,
    invents_alternate_state_label: bool,
}

/// Validates the axes shared by both component vectors.
fn validate_common_control(
    dispositions: &[M5ExperimentDisposition],
    downgrade_triggers: &[M5ExperimentDowngradeTrigger],
    declares_mandatory_labels: bool,
    accessibility_routes: &[M5ExperimentAccessibilityRoute],
    invariants: ControlInvariants,
    violations: &mut Vec<DatasetProvenanceCardSensitivitySharingBannerViolation>,
) {
    if dispositions.is_empty() {
        violations
            .push(DatasetProvenanceCardSensitivitySharingBannerViolation::DispositionsMissing);
    }
    if downgrade_triggers.is_empty() {
        violations
            .push(DatasetProvenanceCardSensitivitySharingBannerViolation::DowngradeTriggersMissing);
    }
    if !declares_mandatory_labels {
        violations
            .push(DatasetProvenanceCardSensitivitySharingBannerViolation::RequiredLabelsIncomplete);
    }
    if accessibility_routes.is_empty()
        || !accessibility_routes.contains(&M5ExperimentAccessibilityRoute::KeyboardFocusable)
    {
        violations.push(
            DatasetProvenanceCardSensitivitySharingBannerViolation::AccessibilityRouteMissing,
        );
    }
    if invariants.masks_provenance_or_sensitivity_state {
        violations.push(
            DatasetProvenanceCardSensitivitySharingBannerViolation::ProvenanceOrSensitivityStateMasked,
        );
    }
    if invariants.hides_dataset_location_or_provenance {
        violations.push(
            DatasetProvenanceCardSensitivitySharingBannerViolation::DatasetLocationOrProvenanceHidden,
        );
    }
    if invariants.exposes_raw_payload_by_default {
        violations.push(
            DatasetProvenanceCardSensitivitySharingBannerViolation::RawPayloadExposedByDefault,
        );
    }
    if invariants.implies_apples_to_apples_without_parity {
        violations.push(
            DatasetProvenanceCardSensitivitySharingBannerViolation::ApplesToApplesImpliedWithoutParity,
        );
    }
    if invariants.invents_alternate_state_label {
        violations.push(
            DatasetProvenanceCardSensitivitySharingBannerViolation::AlternateStateLabelInvented,
        );
    }
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
