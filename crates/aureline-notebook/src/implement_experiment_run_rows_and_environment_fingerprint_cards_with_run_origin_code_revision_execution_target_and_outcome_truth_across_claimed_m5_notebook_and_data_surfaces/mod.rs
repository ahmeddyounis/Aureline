//! Two reusable M5 experiment components — the experiment run row and the environment
//! fingerprint card — so a user can tell where a run came from and whether its environment
//! is reproducible from the component alone, before trusting any downstream comparison or
//! share action: the run row names its run id, notebook / script / task origin, commit or
//! workspace revision, start / end window, execution origin, and outcome, and offers
//! first-class open / compare / export actions; the fingerprint card names its interpreter or
//! kernel, package / toolchain summary, execution target, hardware / profile class, and
//! capture freshness, and offers inspect / export paths.
//!
//! Aureline's frozen experiment-component matrix
//! ([`crate::freeze_the_m5_experiment_run_row_dataset_provenance_card_artifact_lineage_panel_run_comparison_table_environment_fingerprint_card_compare_guard_banner_sensitivity_sharing_banner_and_result_summary_card_component_matrix`])
//! names the experiment run row and the environment fingerprint card as two governed
//! component families and freezes their controlled vocabulary — the run origin kinds
//! (`notebook_cell`, `script_task`, `scheduled_task`, `manual_attach`, `imported_run`,
//! `unknown_origin`) and status states (`queued`, `running`, `succeeded`, `failed`,
//! `canceled`, `stale`) a run row binds; the fingerprint scope classes (`interpreter`,
//! `kernel_spec`, `packages`, `hardware_accelerator`, `os_platform`, `container_image`) and
//! capture states (`captured_complete`, `captured_partial`, `captured_missing`, `pinned`,
//! `drifted`, `unavailable`) a fingerprint card binds; the one controlled disposition
//! vocabulary; the surface families; the deployment lines; the consumer surfaces; the
//! accessibility routes; the required labels; and the downgrade triggers. This module
//! *implements* that contract as two co-equal component vectors so a claimed M5 notebook,
//! experiment-dashboard, comparison, data-catalog, share-review, or CLI surface can project a
//! run row and a fingerprint card that keep the same truth.
//!
//! The module has two derived resolvers:
//!
//! 1. [`resolve_run_origin`] — takes a run row's origin kind and derives its origin class
//!    (local, managed, imported, manually attached, or unknown), whether the run is a
//!    first-party run Aureline launched or manages, and which provenance note the row must
//!    carry — so an imported, manually attached, or unknown-origin run can never read as a
//!    first-party run before a compare or share.
//! 2. [`resolve_fingerprint_capture`] — takes a fingerprint card's capture state and derives
//!    its capture class (captured, partially captured, pinned, or uncaptured), whether the
//!    environment is reliably captured, and which capture note the card must carry — so a
//!    missing, drifted, or unavailable fingerprint can never read as a captured environment.
//!
//! A single controls packet — [`ExperimentRunRowEnvironmentFingerprintControlsPacket`] —
//! binds one vector of run rows and one vector of fingerprint cards to the same origin /
//! capture, code-revision, execution-target, deep-link, and non-visual accessibility
//! vocabulary, so run identity and reproducibility context stay explicit across desktop,
//! headless / export, and support consumers.
//!
//! The run origin kind ([`M5RunOriginKind`]), run status state ([`M5RunStatusState`]),
//! fingerprint scope class ([`M5FingerprintScopeClass`]), fingerprint state
//! ([`M5FingerprintState`]), disposition ([`M5ExperimentDisposition`]), surface family
//! ([`M5ExperimentSurfaceFamily`]), deployment line ([`M5ExperimentDeploymentLine`]),
//! consumer surface ([`M5ExperimentConsumerSurface`]), accessibility route
//! ([`M5ExperimentAccessibilityRoute`]), required label ([`M5ExperimentRequiredLabel`]), and
//! downgrade trigger ([`M5ExperimentDowngradeTrigger`]) are reused verbatim from the frozen
//! matrix. This module mints new vocabulary only for what that matrix left implicit about the
//! two components themselves: the derived origin and capture classes, the bounded run-row and
//! fingerprint-card actions, and the deep-link kinds. No M5 experiment surface invents a
//! second run-row or fingerprint grammar.
//!
//! Raw dataset payloads, pasted paths, credentials, and private endpoints stay outside the
//! export boundary; every context line, deep-link reference, and component identity is carried
//! only as an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_experiment_run_row_environment_fingerprint_controls,
    seeded_experiment_run_row_environment_fingerprint_controls_fingerprint_card_uncaptured,
    seeded_experiment_run_row_environment_fingerprint_controls_run_row_imported,
    EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_PACKET_ID,
};

// The run origin kinds and status states, the fingerprint scope classes and capture states,
// the disposition vocabulary, and the surface / deployment / consumer / accessibility / label
// / downgrade vocabularies are frozen once, in the experiment-component matrix. This lane
// reuses them verbatim so it never invents a parallel run-row or fingerprint vocabulary.
pub use crate::freeze_the_m5_experiment_run_row_dataset_provenance_card_artifact_lineage_panel_run_comparison_table_environment_fingerprint_card_compare_guard_banner_sensitivity_sharing_banner_and_result_summary_card_component_matrix::{
    M5ExperimentAccessibilityRoute, M5ExperimentComponentFamily, M5ExperimentConsumerSurface,
    M5ExperimentDeploymentLine, M5ExperimentDisposition, M5ExperimentDowngradeTrigger,
    M5ExperimentRequiredLabel, M5ExperimentSurfaceFamily, M5FingerprintScopeClass,
    M5FingerprintState, M5RunOriginKind, M5RunStatusState,
    M5_ENVIRONMENT_FINGERPRINT_CARD_SCHEMA_REF, M5_EXPERIMENT_COMPONENT_DOC_REF,
    M5_EXPERIMENT_COMPONENT_SCHEMA_REF, M5_EXPERIMENT_RUN_ROW_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by
/// [`ExperimentRunRowEnvironmentFingerprintControlsPacket`].
pub const EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_RECORD_KIND: &str =
    "implement_m5_experiment_run_rows_and_environment_fingerprint_cards_with_run_origin_code_revision_execution_target_and_outcome_truth_across_claimed_m5_notebook_and_data_surfaces";

/// Schema version for M5 experiment-run-row / environment-fingerprint control records.
pub const EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_SCHEMA_REF: &str =
    "schemas/ui/m5-experiment-run-row-environment-fingerprint-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_DOC_REF: &str =
    "docs/notebooks/m5_experiment_run_row_environment_fingerprint_controls.md";

/// Repo-relative path of the protected fixture directory.
pub const EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_FIXTURE_DIR: &str =
    "fixtures/ui/m5-experiment-run-row-environment-fingerprint-controls";

/// Repo-relative path of the checked support-export artifact.
pub const EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_ARTIFACT_REF: &str =
    "artifacts/release/m5-experiment-run-row-environment-fingerprint-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_CSV_REF: &str =
    "artifacts/release/m5-experiment-run-row-environment-fingerprint-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_REPORT_REF: &str =
    "artifacts/design/m5-experiment-run-row-environment-fingerprint.md";

// ---- shared deep-link vocabulary ----------------------------------------

/// The kind of stable deep link an experiment component binds its next step against, so a run
/// row or fingerprint card never routes through an ephemeral overlay — every next step is a
/// stable run, notebook, dataset-catalog, or docs reference the user can reopen.
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

// ---- experiment-run-row vocabulary --------------------------------------

/// Derived origin class an experiment run row may present.
///
/// This is the run-row honesty axis: the class is derived from the frozen run origin kind,
/// never asserted, so an imported, manually attached, or unknown-origin run can never present
/// as a first-party run and a user can always tell whether they are looking at a local,
/// managed, imported, or manually attached run before trusting a compare or share.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunOriginClass {
    /// A local run Aureline launched (notebook cell or script task).
    LocalRun,
    /// A managed run Aureline schedules or manages.
    ManagedRun,
    /// A run imported from another tracker.
    ImportedRun,
    /// A run manually attached to an external execution.
    ManuallyAttached,
    /// A run whose origin could not be resolved.
    OriginUnknown,
}

impl RunOriginClass {
    /// Every origin class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalRun,
        Self::ManagedRun,
        Self::ImportedRun,
        Self::ManuallyAttached,
        Self::OriginUnknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalRun => "local_run",
            Self::ManagedRun => "managed_run",
            Self::ImportedRun => "imported_run",
            Self::ManuallyAttached => "manually_attached",
            Self::OriginUnknown => "origin_unknown",
        }
    }

    /// True when the run is a first-party run Aureline launched or manages (local or managed).
    pub const fn is_first_party_origin(self) -> bool {
        matches!(self, Self::LocalRun | Self::ManagedRun)
    }
}

/// One keyboard-complete default action an experiment run row offers, so a row never hides its
/// open / compare / export affordance behind a pointer-only gesture. `OpenRun`, `CompareRuns`,
/// and `ExportRun` are always offered so run identity is actionable before any trust decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunRowAction {
    /// Open the run object (always available).
    OpenRun,
    /// Compare this run against another (always available).
    CompareRuns,
    /// Export this run's summary / metadata (always available).
    ExportRun,
    /// Open the stable run / notebook / dataset / docs deep link.
    OpenDeepLink,
    /// Inspect the run's environment fingerprint.
    InspectFingerprint,
    /// Copy the stable run id.
    CopyRunId,
}

impl RunRowAction {
    /// Every run-row action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenRun,
        Self::CompareRuns,
        Self::ExportRun,
        Self::OpenDeepLink,
        Self::InspectFingerprint,
        Self::CopyRunId,
    ];

    /// The default actions every keyboard-complete run row must offer.
    pub const MANDATORY: [Self; 3] = [Self::OpenRun, Self::CompareRuns, Self::ExportRun];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenRun => "open_run",
            Self::CompareRuns => "compare_runs",
            Self::ExportRun => "export_run",
            Self::OpenDeepLink => "open_deep_link",
            Self::InspectFingerprint => "inspect_fingerprint",
            Self::CopyRunId => "copy_run_id",
        }
    }
}

/// Disclosures an experiment run row must carry, derived from the run origin kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunRowDisclosure {
    /// The derived origin class this run row may present.
    pub origin_class: RunOriginClass,
    /// Whether the run is a first-party run Aureline launched or manages.
    pub is_first_party_origin: bool,
    /// Whether the row must carry an explicit imported-run note.
    pub needs_imported_note: bool,
    /// Whether the row must carry an explicit manually-attached note.
    pub needs_manual_attach_note: bool,
    /// Whether the row must carry an explicit unknown-origin note.
    pub needs_unknown_origin_note: bool,
}

/// Resolves the origin truth an experiment run row may present.
///
/// A `notebook_cell` or `script_task` run is a local run. A `scheduled_task` run is a managed
/// run. An `imported_run` is imported. A `manual_attach` run is manually attached. An
/// `unknown_origin` run is unknown, so a run that Aureline did not launch can never read as a
/// first-party run.
pub fn resolve_run_origin(origin: M5RunOriginKind) -> RunRowDisclosure {
    use M5RunOriginKind as Origin;
    use RunOriginClass as Class;

    let origin_class = match origin {
        Origin::NotebookCell | Origin::ScriptTask => Class::LocalRun,
        Origin::ScheduledTask => Class::ManagedRun,
        Origin::ImportedRun => Class::ImportedRun,
        Origin::ManualAttach => Class::ManuallyAttached,
        Origin::UnknownOrigin => Class::OriginUnknown,
    };

    RunRowDisclosure {
        origin_class,
        is_first_party_origin: origin_class.is_first_party_origin(),
        needs_imported_note: matches!(origin_class, Class::ImportedRun),
        needs_manual_attach_note: matches!(origin_class, Class::ManuallyAttached),
        needs_unknown_origin_note: matches!(origin_class, Class::OriginUnknown),
    }
}

/// An experiment run row naming its run id, origin, status, code revision, execution origin,
/// run window, derived origin class, bounded open / compare / export actions, and a stable
/// run / notebook / dataset / docs deep link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentRunRow {
    /// Frozen component this control implements; must be `experiment_run_row`.
    pub component: M5ExperimentComponentFamily,
    /// Stable run id.
    pub run_id: String,
    /// Human-readable run label; required and non-empty.
    pub run_label: String,
    /// Run origin kind, reused from the frozen matrix.
    pub origin_kind: M5RunOriginKind,
    /// Run status state, reused from the frozen matrix.
    pub status_state: M5RunStatusState,
    /// Human-readable status / outcome label; required and non-empty.
    pub status_label: String,
    /// Derived origin class (must equal the resolved class).
    pub origin_class: RunOriginClass,
    /// Whether the row claims a first-party origin (must equal the derived truth).
    pub claims_first_party_origin: bool,
    /// Imported-run note; required when the run is imported.
    pub imported_note: String,
    /// Manually-attached note; required when the run is manually attached.
    pub manual_attach_note: String,
    /// Unknown-origin note; required when the run's origin is unknown.
    pub unknown_origin_note: String,
    /// Origin / status note; always required so origin and outcome stay explicit.
    pub origin_and_status_note: String,
    /// Commit or workspace revision behind the run; always required so revision stays explicit.
    pub code_revision: String,
    /// Execution origin (where the run actually executed); always required.
    pub execution_origin_label: String,
    /// Run start / end window label; always required.
    pub run_window_label: String,
    /// Context note; always required so the row names what to check before compare or share.
    pub context_note: String,
    /// Kind of stable deep link this row binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include open / compare / export).
    pub run_actions: Vec<RunRowAction>,
    /// Dispositions this row binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5ExperimentDisposition>,
    /// Downgrade triggers this row can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5ExperimentDowngradeTrigger>,
    /// Mandatory labels this row can show (must include the mandatory labels).
    pub required_labels: Vec<M5ExperimentRequiredLabel>,
    /// Claimed M5 surface families that render this row.
    pub surface_families: Vec<M5ExperimentSurfaceFamily>,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5ExperimentDeploymentLine>,
    /// Non-visual accessibility routes this row offers.
    pub accessibility_routes: Vec<M5ExperimentAccessibilityRoute>,
    /// Experiment subsystems that consume this row's projection.
    pub consumer_surfaces: Vec<M5ExperimentConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never masks provenance or sensitivity state. MUST be `false`.
    pub masks_provenance_or_sensitivity_state: bool,
    /// Hard invariant: never hides run origin or code revision. MUST be `false`.
    pub hides_run_origin_or_revision: bool,
    /// Hard invariant: never implies apples-to-apples without parity. MUST be `false`.
    pub implies_apples_to_apples_without_parity: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl ExperimentRunRow {
    /// Origin disclosures this row must carry, derived from the run origin kind.
    pub fn origin_disclosure(&self) -> RunRowDisclosure {
        resolve_run_origin(self.origin_kind)
    }

    /// Whether the row offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<RunRowAction> = self.run_actions.iter().copied().collect();
        RunRowAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5ExperimentRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5ExperimentRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the row offers a deep-link-opening action.
    fn offers_deep_link_action(&self) -> bool {
        self.run_actions.contains(&RunRowAction::OpenDeepLink)
    }
}

// ---- environment-fingerprint-card vocabulary ----------------------------

/// Derived capture class an environment fingerprint card may present.
///
/// This is the fingerprint honesty axis: the class is derived from the frozen capture state,
/// never asserted, so a missing, drifted, or unavailable fingerprint can never present as a
/// captured environment and a user can always tell how completely the environment was
/// captured before trusting reproducibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FingerprintCaptureClass {
    /// Captured completely.
    Captured,
    /// Captured partially.
    PartiallyCaptured,
    /// Pinned to an explicit version.
    Pinned,
    /// Not reliably captured (missing, drifted, or unavailable).
    Uncaptured,
}

impl FingerprintCaptureClass {
    /// Every capture class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Captured,
        Self::PartiallyCaptured,
        Self::Pinned,
        Self::Uncaptured,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Captured => "captured",
            Self::PartiallyCaptured => "partially_captured",
            Self::Pinned => "pinned",
            Self::Uncaptured => "uncaptured",
        }
    }

    /// True when the environment is reliably captured (fully captured or pinned).
    pub const fn is_reliably_captured(self) -> bool {
        matches!(self, Self::Captured | Self::Pinned)
    }
}

/// One keyboard-complete default action an environment fingerprint card offers, so a card
/// never hides its inspect / export path behind a pointer-only gesture. `InspectFingerprint`
/// and `ExportFingerprint` are always offered so the captured environment stays inspectable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FingerprintCardAction {
    /// Inspect the full fingerprint (always available).
    InspectFingerprint,
    /// Export the fingerprint (always available).
    ExportFingerprint,
    /// Open the stable run / notebook / dataset / docs deep link.
    OpenDeepLink,
    /// Compare this environment against another.
    CompareEnvironments,
    /// Copy the stable fingerprint id.
    CopyFingerprintId,
    /// Pin the environment for reproducibility.
    PinEnvironment,
}

impl FingerprintCardAction {
    /// Every fingerprint-card action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InspectFingerprint,
        Self::ExportFingerprint,
        Self::OpenDeepLink,
        Self::CompareEnvironments,
        Self::CopyFingerprintId,
        Self::PinEnvironment,
    ];

    /// The default actions every keyboard-complete fingerprint card must offer.
    pub const MANDATORY: [Self; 2] = [Self::InspectFingerprint, Self::ExportFingerprint];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectFingerprint => "inspect_fingerprint",
            Self::ExportFingerprint => "export_fingerprint",
            Self::OpenDeepLink => "open_deep_link",
            Self::CompareEnvironments => "compare_environments",
            Self::CopyFingerprintId => "copy_fingerprint_id",
            Self::PinEnvironment => "pin_environment",
        }
    }
}

/// Disclosures an environment fingerprint card must carry, derived from the capture state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FingerprintCardDisclosure {
    /// The derived capture class this card may present.
    pub capture_class: FingerprintCaptureClass,
    /// Whether the environment is reliably captured.
    pub is_reliably_captured: bool,
    /// Whether the card must carry an explicit partial-capture note.
    pub needs_partial_note: bool,
    /// Whether the card must carry an explicit uncaptured note.
    pub needs_uncaptured_note: bool,
}

/// Resolves the capture truth an environment fingerprint card may present.
///
/// A `captured_complete` fingerprint is captured. A `captured_partial` fingerprint is
/// partially captured. A `pinned` fingerprint is pinned. A `captured_missing`, `drifted`, or
/// `unavailable` fingerprint is uncaptured, so an environment that was not reliably captured
/// can never read as captured.
pub fn resolve_fingerprint_capture(state: M5FingerprintState) -> FingerprintCardDisclosure {
    use FingerprintCaptureClass as Class;
    use M5FingerprintState as State;

    let capture_class = match state {
        State::CapturedComplete => Class::Captured,
        State::CapturedPartial => Class::PartiallyCaptured,
        State::Pinned => Class::Pinned,
        State::CapturedMissing | State::Drifted | State::Unavailable => Class::Uncaptured,
    };

    FingerprintCardDisclosure {
        capture_class,
        is_reliably_captured: capture_class.is_reliably_captured(),
        needs_partial_note: matches!(capture_class, Class::PartiallyCaptured),
        needs_uncaptured_note: matches!(capture_class, Class::Uncaptured),
    }
}

/// An environment fingerprint card naming its scope, capture state, interpreter or kernel,
/// package / toolchain summary, execution target, hardware / profile class, freshness, derived
/// capture class, bounded inspect / export actions, and a stable deep link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentFingerprintCard {
    /// Frozen component this control implements; must be `environment_fingerprint_card`.
    pub component: M5ExperimentComponentFamily,
    /// Stable fingerprint-card id.
    pub card_id: String,
    /// Human-readable card label; required and non-empty.
    pub card_label: String,
    /// Fingerprint scope class, reused from the frozen matrix.
    pub scope_class: M5FingerprintScopeClass,
    /// Fingerprint capture state, reused from the frozen matrix.
    pub fingerprint_state: M5FingerprintState,
    /// Derived capture class (must equal the resolved class).
    pub capture_class: FingerprintCaptureClass,
    /// Whether the card claims the environment is reliably captured (must equal derived truth).
    pub claims_captured: bool,
    /// Partial-capture note; required when the fingerprint is partially captured.
    pub partial_note: String,
    /// Uncaptured note; required when the fingerprint is uncaptured.
    pub uncaptured_note: String,
    /// Interpreter or kernel label; always required so the runtime stays explicit.
    pub interpreter_or_kernel_label: String,
    /// Package / toolchain summary; always required.
    pub toolchain_summary: String,
    /// Execution target label; always required so where the run ran stays explicit.
    pub execution_target_label: String,
    /// Hardware / profile class label; always required (carries an explicit unavailable note
    /// when hardware detail is not available on this build).
    pub hardware_profile_label: String,
    /// Capture freshness label; always required so staleness stays explicit.
    pub freshness_label: String,
    /// Context note; always required so the card names what to check before trusting capture.
    pub context_note: String,
    /// Kind of stable deep link this card binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include inspect / export).
    pub card_actions: Vec<FingerprintCardAction>,
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
    /// Hard invariant: never hides run origin or code revision. MUST be `false`.
    pub hides_run_origin_or_revision: bool,
    /// Hard invariant: never implies apples-to-apples without parity. MUST be `false`.
    pub implies_apples_to_apples_without_parity: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl EnvironmentFingerprintCard {
    /// Capture disclosures this card must carry, derived from the capture state.
    pub fn capture_disclosure(&self) -> FingerprintCardDisclosure {
        resolve_fingerprint_capture(self.fingerprint_state)
    }

    /// Whether the card offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<FingerprintCardAction> = self.card_actions.iter().copied().collect();
        FingerprintCardAction::MANDATORY
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
        self.card_actions
            .contains(&FingerprintCardAction::OpenDeepLink)
    }
}

// ---- review blocks ------------------------------------------------------

/// First-glance experiment-component review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentRunEnvironmentReview {
    /// The run row names where the run came from.
    pub run_row_shows_origin: bool,
    /// The run row names its commit or workspace revision.
    pub run_row_shows_code_revision: bool,
    /// The run row offers open, compare, and export.
    pub run_row_offers_open_compare_export: bool,
    /// The fingerprint card names its captured environment.
    pub fingerprint_card_shows_environment: bool,
    /// The fingerprint card offers inspect and export.
    pub fingerprint_card_offers_inspect_export: bool,
    /// Origin and capture are derived from state, never asserted.
    pub origin_and_capture_derived_never_asserted: bool,
    /// An imported or unknown-origin run is never shown as first-party.
    pub unknown_origin_never_shown_as_first_party: bool,
    /// An uncaptured fingerprint is never shown as captured.
    pub uncaptured_never_shown_as_captured: bool,
    /// Comparison is never implied apples-to-apples without parity evidence.
    pub comparison_never_implied_apples_to_apples_without_parity: bool,
    /// Every next step names one stable run / notebook / dataset / docs deep link.
    pub every_next_step_names_stable_deep_link: bool,
    /// No component widens export scope or exposes raw payloads by default.
    pub no_component_widens_export_scope_or_exposes_raw_by_default: bool,
    /// Run identity and revision stay explicit.
    pub run_identity_and_revision_always_explicit: bool,
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

impl ExperimentRunEnvironmentReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.run_row_shows_origin
            && self.run_row_shows_code_revision
            && self.run_row_offers_open_compare_export
            && self.fingerprint_card_shows_environment
            && self.fingerprint_card_offers_inspect_export
            && self.origin_and_capture_derived_never_asserted
            && self.unknown_origin_never_shown_as_first_party
            && self.uncaptured_never_shown_as_captured
            && self.comparison_never_implied_apples_to_apples_without_parity
            && self.every_next_step_names_stable_deep_link
            && self.no_component_widens_export_scope_or_exposes_raw_by_default
            && self.run_identity_and_revision_always_explicit
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
pub struct ExperimentRunEnvironmentConsumerProjection {
    /// The experiment-run dashboard reads a single canonical source.
    pub experiment_run_dashboard_reads_single_source: bool,
    /// The environment-fingerprint surface reads a single canonical source.
    pub environment_fingerprint_surface_reads_single_source: bool,
    /// Origin and status are visible before compare or share.
    pub origin_and_status_visible_before_compare_or_share: bool,
    /// Environment capture is visible before trusting reproducibility.
    pub environment_capture_visible_before_trust: bool,
    /// Support export shows component truth.
    pub support_export_shows_component_truth: bool,
    /// Help / docs shows component truth.
    pub help_docs_shows_component_truth: bool,
}

impl ExperimentRunEnvironmentConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.experiment_run_dashboard_reads_single_source
            && self.environment_fingerprint_surface_reads_single_source
            && self.origin_and_status_visible_before_compare_or_share
            && self.environment_capture_visible_before_trust
            && self.support_export_shows_component_truth
            && self.help_docs_shows_component_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentRunEnvironmentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`ExperimentRunRowEnvironmentFingerprintControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExperimentRunRowEnvironmentFingerprintControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Experiment run rows.
    pub run_rows: Vec<ExperimentRunRow>,
    /// Environment fingerprint cards.
    pub fingerprint_cards: Vec<EnvironmentFingerprintCard>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5ExperimentDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5ExperimentConsumerSurface>,
    /// Experiment review block.
    pub experiment_review: ExperimentRunEnvironmentReview,
    /// Consumer projection block.
    pub consumer_projection: ExperimentRunEnvironmentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ExperimentRunEnvironmentProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe experiment-run-row / environment-fingerprint controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentRunRowEnvironmentFingerprintControlsPacket {
    /// Record kind; must equal [`EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Experiment run rows.
    pub run_rows: Vec<ExperimentRunRow>,
    /// Environment fingerprint cards.
    pub fingerprint_cards: Vec<EnvironmentFingerprintCard>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5ExperimentDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5ExperimentConsumerSurface>,
    /// Experiment review block.
    pub experiment_review: ExperimentRunEnvironmentReview,
    /// Consumer projection block.
    pub consumer_projection: ExperimentRunEnvironmentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ExperimentRunEnvironmentProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ExperimentRunRowEnvironmentFingerprintControlsPacket {
    /// Builds an experiment-run-row / environment-fingerprint controls packet from stable-lane
    /// input.
    pub fn new(input: ExperimentRunRowEnvironmentFingerprintControlsPacketInput) -> Self {
        Self {
            record_kind: EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_RECORD_KIND.to_owned(),
            schema_version: EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            run_rows: input.run_rows,
            fingerprint_cards: input.fingerprint_cards,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            experiment_review: input.experiment_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the experiment-run-row / environment-fingerprint control invariants.
    pub fn validate(&self) -> Vec<ExperimentRunRowEnvironmentFingerprintViolation> {
        let mut violations = Vec::new();

        if self.record_kind != EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_RECORD_KIND {
            violations.push(ExperimentRunRowEnvironmentFingerprintViolation::WrongRecordKind);
        }
        if self.schema_version != EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_SCHEMA_VERSION {
            violations.push(ExperimentRunRowEnvironmentFingerprintViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ExperimentRunRowEnvironmentFingerprintViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations
                .push(ExperimentRunRowEnvironmentFingerprintViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations
                .push(ExperimentRunRowEnvironmentFingerprintViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_run_rows(self, &mut violations);
        validate_fingerprint_cards(self, &mut violations);

        if !self.experiment_review.all_hold() {
            violations
                .push(ExperimentRunRowEnvironmentFingerprintViolation::ExperimentReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(
                ExperimentRunRowEnvironmentFingerprintViolation::ConsumerProjectionIncomplete,
            );
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations
                .push(ExperimentRunRowEnvironmentFingerprintViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("experiment run row fingerprint packet serializes"),
        ) {
            violations.push(ExperimentRunRowEnvironmentFingerprintViolation::RawMaterialInExport);
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
            .expect("experiment run row fingerprint packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component,id,state_or_origin,status_or_scope,derived,first_party_or_captured,deep_link_kind\n",
        );
        for row in &self.run_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "experiment_run_row",
                csv_field(&row.run_id),
                row.origin_kind.as_str(),
                row.status_state.as_str(),
                row.origin_disclosure().origin_class.as_str(),
                row.origin_disclosure().is_first_party_origin,
                row.deep_link_kind.as_str(),
            ));
        }
        for card in &self.fingerprint_cards {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "environment_fingerprint_card",
                csv_field(&card.card_id),
                card.fingerprint_state.as_str(),
                card.scope_class.as_str(),
                card.capture_disclosure().capture_class.as_str(),
                card.capture_disclosure().is_reliably_captured,
                card.deep_link_kind.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let non_first_party = self
            .run_rows
            .iter()
            .filter(|row| !row.origin_disclosure().is_first_party_origin)
            .count();
        let uncaptured = self
            .fingerprint_cards
            .iter()
            .filter(|card| !card.capture_disclosure().is_reliably_captured)
            .count();

        let mut out = String::new();
        out.push_str("# Experiment run rows and environment fingerprint cards\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Experiment run rows: {} ({} not first-party)\n",
            self.run_rows.len(),
            non_first_party
        ));
        out.push_str(&format!(
            "- Environment fingerprint cards: {} ({} not reliably captured)\n",
            self.fingerprint_cards.len(),
            uncaptured
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Experiment run rows\n\n");
        for row in &self.run_rows {
            out.push_str(&format!(
                "- **{}** — origin `{}`, status `{}` → `{}`, revision `{}`, deep link `{}`\n",
                row.run_label,
                row.origin_kind.as_str(),
                row.status_state.as_str(),
                row.origin_disclosure().origin_class.as_str(),
                row.code_revision,
                row.deep_link_kind.as_str(),
            ));
        }

        out.push_str("\n## Environment fingerprint cards\n\n");
        for card in &self.fingerprint_cards {
            out.push_str(&format!(
                "- **{}** — scope `{}`, state `{}` → `{}`, deep link `{}`\n",
                card.card_label,
                card.scope_class.as_str(),
                card.fingerprint_state.as_str(),
                card.capture_disclosure().capture_class.as_str(),
                card.deep_link_kind.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in experiment-run-row / environment-fingerprint
/// export.
#[derive(Debug)]
pub enum ExperimentRunRowEnvironmentFingerprintArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ExperimentRunRowEnvironmentFingerprintViolation>),
}

impl fmt::Display for ExperimentRunRowEnvironmentFingerprintArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "experiment run row fingerprint export parse failed: {error}"
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
                    "experiment run row fingerprint export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ExperimentRunRowEnvironmentFingerprintArtifactError {}

/// Validation failures emitted by
/// [`ExperimentRunRowEnvironmentFingerprintControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExperimentRunRowEnvironmentFingerprintViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No experiment run rows are present.
    RunRowsMissing,
    /// An experiment run row is incomplete.
    RunRowIncomplete,
    /// An experiment run row carries the wrong frozen component class.
    RunRowWrongComponentClass,
    /// A run row misrepresents its derived origin class.
    OriginMisrepresented,
    /// An imported run does not name its imported state.
    ImportedNoteMissing,
    /// A manually attached run does not name its manually attached state.
    ManualAttachNoteMissing,
    /// An unknown-origin run does not name its unknown origin.
    UnknownOriginNoteMissing,
    /// A run row does not name its origin / status.
    OriginAndStatusNoteMissing,
    /// A run row does not name its status label.
    StatusLabelMissing,
    /// A run row does not name its commit or workspace revision.
    CodeRevisionMissing,
    /// A run row does not name its execution origin.
    ExecutionOriginMissing,
    /// A run row does not name its start / end window.
    RunWindowMissing,
    /// A run row omits a mandatory open / compare / export action.
    RunRowActionsIncomplete,
    /// The run rows do not cover every derived origin class.
    OriginClassCoverageMissing,
    /// The run rows do not cover every run origin kind.
    RunOriginKindCoverageMissing,
    /// The run rows do not cover every run status state.
    RunStatusStateCoverageMissing,
    /// No environment fingerprint cards are present.
    FingerprintCardsMissing,
    /// An environment fingerprint card is incomplete.
    FingerprintCardIncomplete,
    /// An environment fingerprint card carries the wrong frozen component class.
    FingerprintCardWrongComponentClass,
    /// A fingerprint card misrepresents its derived capture class.
    CaptureMisrepresented,
    /// A partially captured fingerprint does not name its partial state.
    PartialNoteMissing,
    /// An uncaptured fingerprint does not name its uncaptured state.
    UncapturedNoteMissing,
    /// A fingerprint card does not name its interpreter or kernel.
    InterpreterOrKernelMissing,
    /// A fingerprint card does not name its package / toolchain summary.
    ToolchainSummaryMissing,
    /// A fingerprint card does not name its execution target.
    ExecutionTargetMissing,
    /// A fingerprint card does not name its hardware / profile class.
    HardwareProfileMissing,
    /// A fingerprint card does not name its capture freshness.
    FreshnessMissing,
    /// A fingerprint card omits a mandatory inspect / export action.
    FingerprintCardActionsIncomplete,
    /// The fingerprint cards do not cover every derived capture class.
    CaptureClassCoverageMissing,
    /// The fingerprint cards do not cover every fingerprint scope class.
    FingerprintScopeCoverageMissing,
    /// The fingerprint cards do not cover every fingerprint state.
    FingerprintStateCoverageMissing,
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
    /// A component hides run origin or code revision.
    RunOriginOrRevisionHidden,
    /// A component implies apples-to-apples without parity evidence.
    ApplesToApplesImpliedWithoutParity,
    /// A component invents an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Experiment review does not satisfy required invariants.
    ExperimentReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl ExperimentRunRowEnvironmentFingerprintViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RunRowsMissing => "run_rows_missing",
            Self::RunRowIncomplete => "run_row_incomplete",
            Self::RunRowWrongComponentClass => "run_row_wrong_component_class",
            Self::OriginMisrepresented => "origin_misrepresented",
            Self::ImportedNoteMissing => "imported_note_missing",
            Self::ManualAttachNoteMissing => "manual_attach_note_missing",
            Self::UnknownOriginNoteMissing => "unknown_origin_note_missing",
            Self::OriginAndStatusNoteMissing => "origin_and_status_note_missing",
            Self::StatusLabelMissing => "status_label_missing",
            Self::CodeRevisionMissing => "code_revision_missing",
            Self::ExecutionOriginMissing => "execution_origin_missing",
            Self::RunWindowMissing => "run_window_missing",
            Self::RunRowActionsIncomplete => "run_row_actions_incomplete",
            Self::OriginClassCoverageMissing => "origin_class_coverage_missing",
            Self::RunOriginKindCoverageMissing => "run_origin_kind_coverage_missing",
            Self::RunStatusStateCoverageMissing => "run_status_state_coverage_missing",
            Self::FingerprintCardsMissing => "fingerprint_cards_missing",
            Self::FingerprintCardIncomplete => "fingerprint_card_incomplete",
            Self::FingerprintCardWrongComponentClass => "fingerprint_card_wrong_component_class",
            Self::CaptureMisrepresented => "capture_misrepresented",
            Self::PartialNoteMissing => "partial_note_missing",
            Self::UncapturedNoteMissing => "uncaptured_note_missing",
            Self::InterpreterOrKernelMissing => "interpreter_or_kernel_missing",
            Self::ToolchainSummaryMissing => "toolchain_summary_missing",
            Self::ExecutionTargetMissing => "execution_target_missing",
            Self::HardwareProfileMissing => "hardware_profile_missing",
            Self::FreshnessMissing => "freshness_missing",
            Self::FingerprintCardActionsIncomplete => "fingerprint_card_actions_incomplete",
            Self::CaptureClassCoverageMissing => "capture_class_coverage_missing",
            Self::FingerprintScopeCoverageMissing => "fingerprint_scope_coverage_missing",
            Self::FingerprintStateCoverageMissing => "fingerprint_state_coverage_missing",
            Self::ContextNoteMissing => "context_note_missing",
            Self::DeepLinkUnresolved => "deep_link_unresolved",
            Self::DeepLinkRefMissing => "deep_link_ref_missing",
            Self::DispositionsMissing => "dispositions_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RequiredLabelsIncomplete => "required_labels_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ProvenanceOrSensitivityStateMasked => "provenance_or_sensitivity_state_masked",
            Self::RunOriginOrRevisionHidden => "run_origin_or_revision_hidden",
            Self::ApplesToApplesImpliedWithoutParity => "apples_to_apples_implied_without_parity",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ExperimentReviewIncomplete => "experiment_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable experiment-run-row / environment-fingerprint
/// export.
pub fn current_experiment_run_row_environment_fingerprint_export() -> Result<
    ExperimentRunRowEnvironmentFingerprintControlsPacket,
    ExperimentRunRowEnvironmentFingerprintArtifactError,
> {
    let packet: ExperimentRunRowEnvironmentFingerprintControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-experiment-run-row-environment-fingerprint-proof/support_export.json"
        )))
        .map_err(ExperimentRunRowEnvironmentFingerprintArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ExperimentRunRowEnvironmentFingerprintArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &ExperimentRunRowEnvironmentFingerprintControlsPacket,
    violations: &mut Vec<ExperimentRunRowEnvironmentFingerprintViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_SCHEMA_REF,
        EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_DOC_REF,
        M5_EXPERIMENT_COMPONENT_SCHEMA_REF,
        M5_EXPERIMENT_COMPONENT_DOC_REF,
        M5_EXPERIMENT_RUN_ROW_SCHEMA_REF,
        M5_ENVIRONMENT_FINGERPRINT_CARD_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations
                .push(ExperimentRunRowEnvironmentFingerprintViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_run_rows(
    packet: &ExperimentRunRowEnvironmentFingerprintControlsPacket,
    violations: &mut Vec<ExperimentRunRowEnvironmentFingerprintViolation>,
) {
    if packet.run_rows.is_empty() {
        violations.push(ExperimentRunRowEnvironmentFingerprintViolation::RunRowsMissing);
        return;
    }

    let mut origin_classes: BTreeSet<RunOriginClass> = BTreeSet::new();
    let mut origins: BTreeSet<M5RunOriginKind> = BTreeSet::new();
    let mut statuses: BTreeSet<M5RunStatusState> = BTreeSet::new();

    for row in &packet.run_rows {
        let disclosure = row.origin_disclosure();
        origin_classes.insert(disclosure.origin_class);
        origins.insert(row.origin_kind);
        statuses.insert(row.status_state);

        if row.run_id.trim().is_empty()
            || row.run_label.trim().is_empty()
            || row.fields_shown.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.consumer_surfaces.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(ExperimentRunRowEnvironmentFingerprintViolation::RunRowIncomplete);
        }
        if row.component != M5ExperimentComponentFamily::ExperimentRunRow {
            violations
                .push(ExperimentRunRowEnvironmentFingerprintViolation::RunRowWrongComponentClass);
        }
        if row.origin_class != disclosure.origin_class
            || row.claims_first_party_origin != disclosure.is_first_party_origin
        {
            violations.push(ExperimentRunRowEnvironmentFingerprintViolation::OriginMisrepresented);
        }
        if disclosure.needs_imported_note && row.imported_note.trim().is_empty() {
            violations.push(ExperimentRunRowEnvironmentFingerprintViolation::ImportedNoteMissing);
        }
        if disclosure.needs_manual_attach_note && row.manual_attach_note.trim().is_empty() {
            violations
                .push(ExperimentRunRowEnvironmentFingerprintViolation::ManualAttachNoteMissing);
        }
        if disclosure.needs_unknown_origin_note && row.unknown_origin_note.trim().is_empty() {
            violations
                .push(ExperimentRunRowEnvironmentFingerprintViolation::UnknownOriginNoteMissing);
        }
        if row.origin_and_status_note.trim().is_empty() {
            violations
                .push(ExperimentRunRowEnvironmentFingerprintViolation::OriginAndStatusNoteMissing);
        }
        if row.status_label.trim().is_empty() {
            violations.push(ExperimentRunRowEnvironmentFingerprintViolation::StatusLabelMissing);
        }
        if row.code_revision.trim().is_empty() {
            violations.push(ExperimentRunRowEnvironmentFingerprintViolation::CodeRevisionMissing);
        }
        if row.execution_origin_label.trim().is_empty() {
            violations
                .push(ExperimentRunRowEnvironmentFingerprintViolation::ExecutionOriginMissing);
        }
        if row.run_window_label.trim().is_empty() {
            violations.push(ExperimentRunRowEnvironmentFingerprintViolation::RunWindowMissing);
        }
        if !row.declares_mandatory_actions() {
            violations
                .push(ExperimentRunRowEnvironmentFingerprintViolation::RunRowActionsIncomplete);
        }
        validate_deep_link(
            row.offers_deep_link_action(),
            row.deep_link_kind,
            &row.deep_link_ref,
            &row.context_note,
            violations,
        );
        validate_common_control(
            &row.dispositions,
            &row.downgrade_triggers,
            row.declares_mandatory_labels(),
            &row.accessibility_routes,
            ControlInvariants {
                masks_provenance_or_sensitivity_state: row.masks_provenance_or_sensitivity_state,
                hides_run_origin_or_revision: row.hides_run_origin_or_revision,
                implies_apples_to_apples_without_parity: row
                    .implies_apples_to_apples_without_parity,
                invents_alternate_state_label: row.invents_alternate_state_label,
            },
            violations,
        );
    }

    for required in RunOriginClass::ALL {
        if !origin_classes.contains(&required) {
            violations
                .push(ExperimentRunRowEnvironmentFingerprintViolation::OriginClassCoverageMissing);
            break;
        }
    }
    for required in M5RunOriginKind::ALL {
        if !origins.contains(&required) {
            violations.push(
                ExperimentRunRowEnvironmentFingerprintViolation::RunOriginKindCoverageMissing,
            );
            break;
        }
    }
    for required in M5RunStatusState::ALL {
        if !statuses.contains(&required) {
            violations.push(
                ExperimentRunRowEnvironmentFingerprintViolation::RunStatusStateCoverageMissing,
            );
            break;
        }
    }
}

fn validate_fingerprint_cards(
    packet: &ExperimentRunRowEnvironmentFingerprintControlsPacket,
    violations: &mut Vec<ExperimentRunRowEnvironmentFingerprintViolation>,
) {
    if packet.fingerprint_cards.is_empty() {
        violations.push(ExperimentRunRowEnvironmentFingerprintViolation::FingerprintCardsMissing);
        return;
    }

    let mut capture_classes: BTreeSet<FingerprintCaptureClass> = BTreeSet::new();
    let mut scopes: BTreeSet<M5FingerprintScopeClass> = BTreeSet::new();
    let mut states: BTreeSet<M5FingerprintState> = BTreeSet::new();

    for card in &packet.fingerprint_cards {
        let disclosure = card.capture_disclosure();
        capture_classes.insert(disclosure.capture_class);
        scopes.insert(card.scope_class);
        states.insert(card.fingerprint_state);

        if card.card_id.trim().is_empty()
            || card.card_label.trim().is_empty()
            || card.fields_shown.is_empty()
            || card.surface_families.is_empty()
            || card.deployment_lines.is_empty()
            || card.consumer_surfaces.is_empty()
            || card.source_contract_refs.is_empty()
        {
            violations
                .push(ExperimentRunRowEnvironmentFingerprintViolation::FingerprintCardIncomplete);
        }
        if card.component != M5ExperimentComponentFamily::EnvironmentFingerprintCard {
            violations.push(
                ExperimentRunRowEnvironmentFingerprintViolation::FingerprintCardWrongComponentClass,
            );
        }
        if card.capture_class != disclosure.capture_class
            || card.claims_captured != disclosure.is_reliably_captured
        {
            violations.push(ExperimentRunRowEnvironmentFingerprintViolation::CaptureMisrepresented);
        }
        if disclosure.needs_partial_note && card.partial_note.trim().is_empty() {
            violations.push(ExperimentRunRowEnvironmentFingerprintViolation::PartialNoteMissing);
        }
        if disclosure.needs_uncaptured_note && card.uncaptured_note.trim().is_empty() {
            violations.push(ExperimentRunRowEnvironmentFingerprintViolation::UncapturedNoteMissing);
        }
        if card.interpreter_or_kernel_label.trim().is_empty() {
            violations
                .push(ExperimentRunRowEnvironmentFingerprintViolation::InterpreterOrKernelMissing);
        }
        if card.toolchain_summary.trim().is_empty() {
            violations
                .push(ExperimentRunRowEnvironmentFingerprintViolation::ToolchainSummaryMissing);
        }
        if card.execution_target_label.trim().is_empty() {
            violations
                .push(ExperimentRunRowEnvironmentFingerprintViolation::ExecutionTargetMissing);
        }
        if card.hardware_profile_label.trim().is_empty() {
            violations
                .push(ExperimentRunRowEnvironmentFingerprintViolation::HardwareProfileMissing);
        }
        if card.freshness_label.trim().is_empty() {
            violations.push(ExperimentRunRowEnvironmentFingerprintViolation::FreshnessMissing);
        }
        if !card.declares_mandatory_actions() {
            violations.push(
                ExperimentRunRowEnvironmentFingerprintViolation::FingerprintCardActionsIncomplete,
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
                hides_run_origin_or_revision: card.hides_run_origin_or_revision,
                implies_apples_to_apples_without_parity: card
                    .implies_apples_to_apples_without_parity,
                invents_alternate_state_label: card.invents_alternate_state_label,
            },
            violations,
        );
    }

    for required in FingerprintCaptureClass::ALL {
        if !capture_classes.contains(&required) {
            violations
                .push(ExperimentRunRowEnvironmentFingerprintViolation::CaptureClassCoverageMissing);
            break;
        }
    }
    for required in M5FingerprintScopeClass::ALL {
        if !scopes.contains(&required) {
            violations.push(
                ExperimentRunRowEnvironmentFingerprintViolation::FingerprintScopeCoverageMissing,
            );
            break;
        }
    }
    for required in M5FingerprintState::ALL {
        if !states.contains(&required) {
            violations.push(
                ExperimentRunRowEnvironmentFingerprintViolation::FingerprintStateCoverageMissing,
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
    violations: &mut Vec<ExperimentRunRowEnvironmentFingerprintViolation>,
) {
    if context_note.trim().is_empty() {
        violations.push(ExperimentRunRowEnvironmentFingerprintViolation::ContextNoteMissing);
    }
    if offers_deep_link_action && !deep_link_kind.is_resolvable() {
        violations.push(ExperimentRunRowEnvironmentFingerprintViolation::DeepLinkUnresolved);
    }
    if deep_link_kind.is_resolvable() && deep_link_ref.trim().is_empty() {
        violations.push(ExperimentRunRowEnvironmentFingerprintViolation::DeepLinkRefMissing);
    }
}

/// The four hard-invariant bools every component must keep `false`.
struct ControlInvariants {
    masks_provenance_or_sensitivity_state: bool,
    hides_run_origin_or_revision: bool,
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
    violations: &mut Vec<ExperimentRunRowEnvironmentFingerprintViolation>,
) {
    if dispositions.is_empty() {
        violations.push(ExperimentRunRowEnvironmentFingerprintViolation::DispositionsMissing);
    }
    if downgrade_triggers.is_empty() {
        violations.push(ExperimentRunRowEnvironmentFingerprintViolation::DowngradeTriggersMissing);
    }
    if !declares_mandatory_labels {
        violations.push(ExperimentRunRowEnvironmentFingerprintViolation::RequiredLabelsIncomplete);
    }
    if accessibility_routes.is_empty()
        || !accessibility_routes.contains(&M5ExperimentAccessibilityRoute::KeyboardFocusable)
    {
        violations.push(ExperimentRunRowEnvironmentFingerprintViolation::AccessibilityRouteMissing);
    }
    if invariants.masks_provenance_or_sensitivity_state {
        violations.push(
            ExperimentRunRowEnvironmentFingerprintViolation::ProvenanceOrSensitivityStateMasked,
        );
    }
    if invariants.hides_run_origin_or_revision {
        violations.push(ExperimentRunRowEnvironmentFingerprintViolation::RunOriginOrRevisionHidden);
    }
    if invariants.implies_apples_to_apples_without_parity {
        violations.push(
            ExperimentRunRowEnvironmentFingerprintViolation::ApplesToApplesImpliedWithoutParity,
        );
    }
    if invariants.invents_alternate_state_label {
        violations
            .push(ExperimentRunRowEnvironmentFingerprintViolation::AlternateStateLabelInvented);
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
