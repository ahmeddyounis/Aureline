//! Profile launcher, attach sheets, capture-mode descriptors, and storage-location truth.
//!
//! This module materializes the typed records that keep profiler launch and attach
//! surfaces honest about how a capture was started, what mode it runs in, and where
//! the resulting evidence is stored. The records and closed vocabularies here mirror
//! the boundary schema at
//! `/schemas/perf/materialize-profile-launcher-and-attach-sheets-capture-mode-descriptors-and-storage-location-truth.schema.json`
//! and reuse the capture-class, provenance, and storage axes already frozen in
//! `/docs/performance/profiling_trace_replay_contract.md`.
//!
//! The module exposes:
//!
//! - the [`ProfileLauncherRow`] record that binds launcher identity, launch mode,
//!   target ref, build/runtime identity, execution context, and capture-mode ref;
//! - the [`AttachSheetRow`] record that carries attach-sheet kind, target process
//!   identity, attach protocol state, and capture-mode ref so attach surfaces never
//!   silently downgrade to launch semantics;
//! - the [`CaptureModeDescriptorRow`] record that carries capture mode class,
//!   sampling or instrumentation parameters, overhead class, and mapping quality
//!   state so users always know how evidence was produced;
//! - the [`StorageLocationTruthRow`] record that carries storage location class,
//!   path or URI ref, retention class, freshness state, provenance chain, and
//!   policy posture so evidence is never shown without knowing where it lives;
//! - the [`ProfileLauncherQualificationPacket`] checked-in artifact that downstream
//!   docs, help, support, and CI surfaces ingest instead of cloning status text.
//!
//! Raw command lines, raw process environment bytes, raw payload bytes, secrets,
//! and ambient credentials MUST NOT appear on any record carried here.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version stamped on every profile-launcher qualification packet carried by
/// this module. Bumped only on breaking payload changes; additive-optional fields do
/// not bump this value.
pub const PROFILE_LAUNCHER_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ProfileLauncherQualificationPacket`].
pub const PROFILE_LAUNCHER_QUALIFICATION_RECORD_KIND: &str =
    "materialize_profile_launcher_and_attach_sheets_capture_mode_descriptors_and_storage_location_truth";

/// Repo-relative path to the checked-in profile-launcher qualification packet JSON.
pub const PROFILE_LAUNCHER_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/perf/m5/materialize-profile-launcher-and-attach-sheets-capture-mode-descriptors-and-storage-location-truth.json";

/// Embedded checked-in qualification packet JSON.
pub const PROFILE_LAUNCHER_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/perf/m5/materialize-profile-launcher-and-attach-sheets-capture-mode-descriptors-and-storage-location-truth.json"
));

/// Qualification label shown on promoted profile-launcher surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileLauncherQualificationLabel {
    /// Surface has current proof and may be called stable for its declared scope.
    Stable,
    /// Surface is visible but below stable.
    Preview,
    /// Surface is an experiment or internal lab.
    Labs,
    /// Surface may inspect metadata but must not execute or export live data.
    InspectOnly,
    /// Surface may import or view captured files only.
    ImportOnly,
}

impl ProfileLauncherQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Profile-launcher surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileLauncherSurfaceKind {
    /// Profile launcher surface (start a new capture).
    ProfileLauncher,
    /// Attach sheet surface (attach to a running target).
    AttachSheet,
    /// Capture-mode inspector surface.
    CaptureModeInspector,
    /// Storage-location browser or inspector surface.
    StorageLocationBrowser,
    /// Export review surface for profile evidence.
    ExportReview,
    /// Support export surface for profile evidence.
    SupportExport,
}

/// Launch versus attach posture for a profiling session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileLaunchMode {
    /// Profiler launches the target and captures from startup.
    Launch,
    /// Profiler attaches to an already-running target.
    Attach,
    /// Profiler imports a previously captured bundle.
    Import,
    /// Profiler opens a cached or stale local bundle for inspection.
    Cached,
}

impl ProfileLaunchMode {
    /// Stable string token for export records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Launch => "launch",
            Self::Attach => "attach",
            Self::Import => "import",
            Self::Cached => "cached",
        }
    }
}

/// Kind of attach sheet used to reach a running target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttachSheetKind {
    /// Local process picker.
    ProcessPicker,
    /// Direct PID entry.
    PidEntry,
    /// Remote or container attach via helper.
    RemoteAttach,
    /// Notebook kernel attach.
    NotebookKernelAttach,
    /// Browser runtime attach.
    BrowserRuntimeAttach,
    /// Import from file system.
    FileSystemImport,
    /// Import from support bundle.
    SupportBundleImport,
}

/// Capture mode class describing how samples or spans are collected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureModeClass {
    /// Time-based sampling with a fixed interval.
    TimeSampling,
    /// Event-based sampling triggered by hardware counters.
    EventSampling,
    /// Full instrumentation with probe insertion.
    Instrumentation,
    /// Hybrid sampling plus selective instrumentation.
    Hybrid,
    /// Allocation or heap sampling.
    AllocationSampling,
    /// Render or frame timeline capture.
    RenderTimeline,
    /// Trace span collection from a backend.
    TraceSpanCollection,
    /// Replay sidecar recording.
    ReplayRecording,
}

/// Storage location class for captured evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageLocationClass {
    /// Local temporary directory.
    LocalTemp,
    /// Local cache directory with retention policy.
    LocalCache,
    /// Workspace-relative storage under version control exclusion.
    WorkspaceRelative,
    /// Remote object store or managed service.
    RemoteStore,
    /// Support-retained artifact bundle.
    SupportBundle,
    /// Imported from an external file or bundle.
    Imported,
    /// Evidence that has been moved or archived.
    Archived,
    /// Location blocked by policy.
    PolicyBlocked,
}

/// Truth label for a storage location entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageLocationTruthLabel {
    /// Evidence is current and reachable.
    Current,
    /// Evidence is stale but still present.
    Stale,
    /// Evidence has passed retention expiry.
    Expired,
    /// Evidence is missing or was deleted.
    Missing,
    /// Evidence was imported and may have different provenance.
    Imported,
    /// Evidence location is blocked by policy.
    PolicyBlocked,
    /// Evidence integrity is unverified.
    Unverified,
}

/// One profile-launcher row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileLauncherRow {
    /// Stable launcher row id.
    pub launcher_id: String,
    /// Human-readable title.
    pub title: String,
    /// Launch mode.
    pub launch_mode: ProfileLaunchMode,
    /// Target ref (process, task, or run configuration).
    pub target_ref: String,
    /// Target kind label.
    pub target_kind: String,
    /// Exact build identity ref.
    pub exact_build_identity_ref: String,
    /// Execution context id shared with runtime.
    pub execution_context_id: String,
    /// Capture-mode descriptor ref.
    pub capture_mode_ref: String,
    /// Storage-location truth ref.
    pub storage_location_ref: String,
    /// True when the launcher is present in the promoted build.
    pub promoted_build_surface: bool,
}

/// One attach-sheet row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttachSheetRow {
    /// Stable attach-sheet row id.
    pub sheet_id: String,
    /// Human-readable title.
    pub title: String,
    /// Attach-sheet kind.
    pub sheet_kind: AttachSheetKind,
    /// Target process ref or handle.
    pub target_process_ref: String,
    /// Target process kind label.
    pub target_process_kind: String,
    /// Exact build identity ref when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact_build_identity_ref: Option<String>,
    /// Execution context id shared with runtime when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_id: Option<String>,
    /// Capture-mode descriptor ref.
    pub capture_mode_ref: String,
    /// Storage-location truth ref.
    pub storage_location_ref: String,
    /// True when the attach sheet is present in the promoted build.
    pub promoted_build_surface: bool,
    /// True when the sheet shows a degraded-state label instead of optimistic text.
    pub shows_degraded_label: bool,
}

/// One capture-mode descriptor row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaptureModeDescriptorRow {
    /// Stable descriptor id.
    pub descriptor_id: String,
    /// Human-readable title.
    pub title: String,
    /// Capture mode class.
    pub capture_mode_class: CaptureModeClass,
    /// Sampling interval in microseconds when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sampling_interval_us: Option<u64>,
    /// Event counter name when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_counter: Option<String>,
    /// Overhead class label.
    pub overhead_class: String,
    /// Mapping quality state label.
    pub mapping_quality: String,
    /// True when the descriptor is selectable in the UI.
    pub selectable: bool,
    /// True when the descriptor shows an honest overhead warning.
    pub shows_overhead_warning: bool,
    /// True when the descriptor explains why a mode is unavailable.
    pub shows_unavailable_reason: bool,
}

/// One storage-location truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageLocationTruthRow {
    /// Stable location row id.
    pub location_id: String,
    /// Human-readable title.
    pub title: String,
    /// Storage location class.
    pub location_class: StorageLocationClass,
    /// Path or URI ref.
    pub path_or_uri_ref: String,
    /// Retention class label.
    pub retention_class: String,
    /// Freshness state.
    pub freshness: StorageLocationTruthLabel,
    /// Provenance chain refs.
    #[serde(default)]
    pub provenance_refs: Vec<String>,
    /// Policy posture label.
    pub policy_posture: String,
    /// True when the location is shown with its class and freshness.
    pub shows_location_class: bool,
    /// True when the location shows retention and policy posture.
    pub shows_retention_policy: bool,
    /// True when the location warns on stale, expired, or missing state.
    pub warns_on_degraded_state: bool,
}

/// Checked-in proof bundle for one surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileLauncherQualificationProof {
    /// Packet id.
    pub packet_id: String,
    /// Packet ref path.
    pub packet_ref: String,
    /// Proof index ref path.
    pub proof_index_ref: String,
    /// Captured-at timestamp.
    pub captured_at: String,
    /// Evidence refs.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

/// Summary projected onto help, release, and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileLauncherQualificationSummary {
    /// Total number of launcher rows.
    pub launcher_count: usize,
    /// Total number of attach-sheet rows.
    pub attach_sheet_count: usize,
    /// Total number of capture-mode descriptors.
    pub capture_mode_count: usize,
    /// Total number of storage-location truth rows.
    pub storage_location_count: usize,
    /// Number of rows claiming stable.
    pub stable_count: usize,
    /// Number of rows below stable.
    pub below_stable_count: usize,
    /// True when every row has a non-empty disclosure ref if below stable.
    pub all_below_stable_have_disclosure: bool,
}

/// Guard set for a profile-launcher surface qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileLauncherSurfaceGuardSet {
    /// Launch mode is visible.
    pub launch_mode_visible: bool,
    /// Target identity is visible.
    pub target_identity_visible: bool,
    /// Capture-mode descriptor is visible.
    pub capture_mode_visible: bool,
    /// Storage location is visible.
    pub storage_location_visible: bool,
    /// Build/runtime identity is visible.
    pub build_runtime_visible: bool,
    /// Degraded-state label is visible when applicable.
    pub degraded_state_label_visible: bool,
    /// Export posture is visible.
    pub export_posture_visible: bool,
    /// Retention and policy posture is visible.
    pub retention_policy_visible: bool,
}

/// One surface qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileLauncherSurfaceQualificationRow {
    /// Surface id.
    pub surface_id: String,
    /// Surface title.
    pub title: String,
    /// Surface kind.
    pub surface_kind: ProfileLauncherSurfaceKind,
    /// True when the surface is present in the promoted build.
    pub promoted_build_surface: bool,
    /// Claim label.
    pub claim_label: ProfileLauncherQualificationLabel,
    /// Displayed label (may differ from claim when narrowed).
    pub displayed_label: String,
    /// Qualification proof bundle.
    pub qualification_packet: ProfileLauncherQualificationProof,
    /// Guard set.
    pub guards: ProfileLauncherSurfaceGuardSet,
    /// True when the surface downgrades if required guards are missing.
    pub downgrade_if_missing: bool,
    /// Rationale string.
    pub rationale: String,
}

/// The checked-in profile-launcher qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileLauncherQualificationPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record kind.
    pub record_kind: String,
    /// Packet id.
    pub packet_id: String,
    /// As-of timestamp.
    pub as_of: String,
    /// Release doc ref.
    pub release_doc_ref: String,
    /// Help doc ref.
    pub help_doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Surface qualification rows.
    pub surfaces: Vec<ProfileLauncherSurfaceQualificationRow>,
    /// Profile launcher rows.
    pub launchers: Vec<ProfileLauncherRow>,
    /// Attach sheet rows.
    pub attach_sheets: Vec<AttachSheetRow>,
    /// Capture-mode descriptor rows.
    pub capture_modes: Vec<CaptureModeDescriptorRow>,
    /// Storage-location truth rows.
    pub storage_locations: Vec<StorageLocationTruthRow>,
    /// Summary.
    pub summary: ProfileLauncherQualificationSummary,
}

impl ProfileLauncherQualificationPacket {
    /// Computes the summary from current rows.
    pub fn computed_summary(&self) -> ProfileLauncherQualificationSummary {
        let stable_count = self
            .surfaces
            .iter()
            .filter(|s| s.claim_label.is_stable())
            .count();
        let below_stable_count = self.surfaces.len().saturating_sub(stable_count);
        let all_below_stable_have_disclosure = self
            .surfaces
            .iter()
            .filter(|s| !s.claim_label.is_stable())
            .all(|s| !s.rationale.is_empty());

        ProfileLauncherQualificationSummary {
            launcher_count: self.launchers.len(),
            attach_sheet_count: self.attach_sheets.len(),
            capture_mode_count: self.capture_modes.len(),
            storage_location_count: self.storage_locations.len(),
            stable_count,
            below_stable_count,
            all_below_stable_have_disclosure,
        }
    }

    /// Validates the packet and returns any violations.
    pub fn validate(&self) -> Vec<ProfileLauncherQualificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != PROFILE_LAUNCHER_QUALIFICATION_SCHEMA_VERSION {
            violations.push(ProfileLauncherQualificationViolation::SchemaVersion {
                expected: PROFILE_LAUNCHER_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }

        if self.record_kind != PROFILE_LAUNCHER_QUALIFICATION_RECORD_KIND {
            violations.push(ProfileLauncherQualificationViolation::RecordKind {
                expected: PROFILE_LAUNCHER_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        let mut surface_ids = BTreeSet::new();
        for surface in &self.surfaces {
            if !surface_ids.insert(surface.surface_id.clone()) {
                violations.push(ProfileLauncherQualificationViolation::DuplicateId {
                    kind: ProfileLauncherQualificationViolationKind::Surface,
                    id: surface.surface_id.clone(),
                });
            }
            if surface.promoted_build_surface
                && surface.claim_label.is_stable()
                && (!surface.guards.launch_mode_visible
                    || !surface.guards.target_identity_visible
                    || !surface.guards.capture_mode_visible
                    || !surface.guards.storage_location_visible)
            {
                violations.push(ProfileLauncherQualificationViolation::IncompleteGuardSet {
                    surface_id: surface.surface_id.clone(),
                });
            }
        }

        let mut launcher_ids = BTreeSet::new();
        for launcher in &self.launchers {
            if !launcher_ids.insert(launcher.launcher_id.clone()) {
                violations.push(ProfileLauncherQualificationViolation::DuplicateId {
                    kind: ProfileLauncherQualificationViolationKind::Launcher,
                    id: launcher.launcher_id.clone(),
                });
            }
            if launcher.launcher_id.trim().is_empty()
                || launcher.title.trim().is_empty()
                || launcher.target_ref.trim().is_empty()
                || launcher.exact_build_identity_ref.trim().is_empty()
                || launcher.execution_context_id.trim().is_empty()
                || launcher.capture_mode_ref.trim().is_empty()
                || launcher.storage_location_ref.trim().is_empty()
            {
                violations.push(ProfileLauncherQualificationViolation::IncompleteLauncher {
                    launcher_id: launcher.launcher_id.clone(),
                });
            }
        }

        let mut sheet_ids = BTreeSet::new();
        for sheet in &self.attach_sheets {
            if !sheet_ids.insert(sheet.sheet_id.clone()) {
                violations.push(ProfileLauncherQualificationViolation::DuplicateId {
                    kind: ProfileLauncherQualificationViolationKind::AttachSheet,
                    id: sheet.sheet_id.clone(),
                });
            }
            if sheet.sheet_id.trim().is_empty()
                || sheet.title.trim().is_empty()
                || sheet.target_process_ref.trim().is_empty()
                || sheet.capture_mode_ref.trim().is_empty()
                || sheet.storage_location_ref.trim().is_empty()
            {
                violations.push(
                    ProfileLauncherQualificationViolation::IncompleteAttachSheet {
                        sheet_id: sheet.sheet_id.clone(),
                    },
                );
            }
            if !sheet.shows_degraded_label {
                violations.push(
                    ProfileLauncherQualificationViolation::AttachSheetMissingDegradedLabel {
                        sheet_id: sheet.sheet_id.clone(),
                    },
                );
            }
        }

        let mut descriptor_ids = BTreeSet::new();
        for descriptor in &self.capture_modes {
            if !descriptor_ids.insert(descriptor.descriptor_id.clone()) {
                violations.push(ProfileLauncherQualificationViolation::DuplicateId {
                    kind: ProfileLauncherQualificationViolationKind::CaptureMode,
                    id: descriptor.descriptor_id.clone(),
                });
            }
            if descriptor.descriptor_id.trim().is_empty()
                || descriptor.title.trim().is_empty()
                || descriptor.overhead_class.trim().is_empty()
                || descriptor.mapping_quality.trim().is_empty()
            {
                violations.push(
                    ProfileLauncherQualificationViolation::IncompleteCaptureMode {
                        descriptor_id: descriptor.descriptor_id.clone(),
                    },
                );
            }
            if !descriptor.shows_overhead_warning {
                violations.push(
                    ProfileLauncherQualificationViolation::CaptureModeMissingOverheadWarning {
                        descriptor_id: descriptor.descriptor_id.clone(),
                    },
                );
            }
        }

        let mut location_ids = BTreeSet::new();
        for location in &self.storage_locations {
            if !location_ids.insert(location.location_id.clone()) {
                violations.push(ProfileLauncherQualificationViolation::DuplicateId {
                    kind: ProfileLauncherQualificationViolationKind::StorageLocation,
                    id: location.location_id.clone(),
                });
            }
            if location.location_id.trim().is_empty()
                || location.title.trim().is_empty()
                || location.path_or_uri_ref.trim().is_empty()
                || location.retention_class.trim().is_empty()
                || location.policy_posture.trim().is_empty()
            {
                violations.push(
                    ProfileLauncherQualificationViolation::IncompleteStorageLocation {
                        location_id: location.location_id.clone(),
                    },
                );
            }
            if !location.shows_location_class
                || !location.shows_retention_policy
                || !location.warns_on_degraded_state
            {
                violations.push(
                    ProfileLauncherQualificationViolation::StorageLocationMissingTruthLabels {
                        location_id: location.location_id.clone(),
                    },
                );
            }
        }

        // Cross-reference: every launcher and attach sheet must point to a known capture mode.
        let descriptor_id_set: BTreeSet<String> = self
            .capture_modes
            .iter()
            .map(|d| d.descriptor_id.clone())
            .collect();
        for launcher in &self.launchers {
            if !descriptor_id_set.contains(&launcher.capture_mode_ref) {
                violations.push(
                    ProfileLauncherQualificationViolation::LauncherCaptureModeRefUnknown {
                        launcher_id: launcher.launcher_id.clone(),
                        capture_mode_ref: launcher.capture_mode_ref.clone(),
                    },
                );
            }
        }
        for sheet in &self.attach_sheets {
            if !descriptor_id_set.contains(&sheet.capture_mode_ref) {
                violations.push(
                    ProfileLauncherQualificationViolation::AttachSheetCaptureModeRefUnknown {
                        sheet_id: sheet.sheet_id.clone(),
                        capture_mode_ref: sheet.capture_mode_ref.clone(),
                    },
                );
            }
        }

        // Cross-reference: every launcher and attach sheet must point to a known storage location.
        let location_id_set: BTreeSet<String> = self
            .storage_locations
            .iter()
            .map(|l| l.location_id.clone())
            .collect();
        for launcher in &self.launchers {
            if !location_id_set.contains(&launcher.storage_location_ref) {
                violations.push(
                    ProfileLauncherQualificationViolation::LauncherStorageLocationRefUnknown {
                        launcher_id: launcher.launcher_id.clone(),
                        storage_location_ref: launcher.storage_location_ref.clone(),
                    },
                );
            }
        }
        for sheet in &self.attach_sheets {
            if !location_id_set.contains(&sheet.storage_location_ref) {
                violations.push(
                    ProfileLauncherQualificationViolation::AttachSheetStorageLocationRefUnknown {
                        sheet_id: sheet.sheet_id.clone(),
                        storage_location_ref: sheet.storage_location_ref.clone(),
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(ProfileLauncherQualificationViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in profile-launcher qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_profile_launcher_qualification(
) -> Result<ProfileLauncherQualificationPacket, serde_json::Error> {
    serde_json::from_str(PROFILE_LAUNCHER_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileLauncherQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Profile launcher rows.
    Launcher,
    /// Attach-sheet rows.
    AttachSheet,
    /// Capture-mode descriptor rows.
    CaptureMode,
    /// Storage-location truth rows.
    StorageLocation,
}

impl fmt::Display for ProfileLauncherQualificationViolationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Surface => write!(f, "surface"),
            Self::Launcher => write!(f, "launcher"),
            Self::AttachSheet => write!(f, "attach_sheet"),
            Self::CaptureMode => write!(f, "capture_mode"),
            Self::StorageLocation => write!(f, "storage_location"),
        }
    }
}

/// Validation failure for profile-launcher qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileLauncherQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion {
        /// Expected schema version.
        expected: u32,
        /// Actual schema version.
        actual: u32,
    },
    /// Record kind does not match the model.
    RecordKind {
        /// Expected record kind.
        expected: String,
        /// Actual record kind.
        actual: String,
    },
    /// IDs must be unique inside an object family.
    DuplicateId {
        /// Kind of object family.
        kind: ProfileLauncherQualificationViolationKind,
        /// Duplicate id.
        id: String,
    },
    /// A surface with a stable claim has an incomplete guard set.
    IncompleteGuardSet {
        /// Surface id.
        surface_id: String,
    },
    /// A profile-launcher row is incomplete.
    IncompleteLauncher {
        /// Launcher id.
        launcher_id: String,
    },
    /// An attach-sheet row is incomplete.
    IncompleteAttachSheet {
        /// Sheet id.
        sheet_id: String,
    },
    /// An attach-sheet row must show a degraded-state label.
    AttachSheetMissingDegradedLabel {
        /// Sheet id.
        sheet_id: String,
    },
    /// A capture-mode descriptor row is incomplete.
    IncompleteCaptureMode {
        /// Descriptor id.
        descriptor_id: String,
    },
    /// A capture-mode descriptor row must show an overhead warning.
    CaptureModeMissingOverheadWarning {
        /// Descriptor id.
        descriptor_id: String,
    },
    /// A storage-location truth row is incomplete.
    IncompleteStorageLocation {
        /// Location id.
        location_id: String,
    },
    /// A storage-location truth row must show class, retention, and degraded warnings.
    StorageLocationMissingTruthLabels {
        /// Location id.
        location_id: String,
    },
    /// A launcher references an unknown capture-mode descriptor.
    LauncherCaptureModeRefUnknown {
        /// Launcher id.
        launcher_id: String,
        /// Unknown capture-mode ref.
        capture_mode_ref: String,
    },
    /// An attach sheet references an unknown capture-mode descriptor.
    AttachSheetCaptureModeRefUnknown {
        /// Sheet id.
        sheet_id: String,
        /// Unknown capture-mode ref.
        capture_mode_ref: String,
    },
    /// A launcher references an unknown storage location.
    LauncherStorageLocationRefUnknown {
        /// Launcher id.
        launcher_id: String,
        /// Unknown storage-location ref.
        storage_location_ref: String,
    },
    /// An attach sheet references an unknown storage location.
    AttachSheetStorageLocationRefUnknown {
        /// Sheet id.
        sheet_id: String,
        /// Unknown storage-location ref.
        storage_location_ref: String,
    },
    /// Computed summary does not match the stored summary.
    SummaryMismatch,
}

impl fmt::Display for ProfileLauncherQualificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::DuplicateId { kind, id } => {
                write!(f, "duplicate {kind} id: {id}")
            }
            Self::IncompleteGuardSet { surface_id } => {
                write!(
                    f,
                    "surface {surface_id} claims stable but guard set is incomplete"
                )
            }
            Self::IncompleteLauncher { launcher_id } => {
                write!(f, "incomplete launcher row: {launcher_id}")
            }
            Self::IncompleteAttachSheet { sheet_id } => {
                write!(f, "incomplete attach-sheet row: {sheet_id}")
            }
            Self::AttachSheetMissingDegradedLabel { sheet_id } => {
                write!(
                    f,
                    "attach sheet {sheet_id} must show a degraded-state label"
                )
            }
            Self::IncompleteCaptureMode { descriptor_id } => {
                write!(f, "incomplete capture-mode descriptor: {descriptor_id}")
            }
            Self::CaptureModeMissingOverheadWarning { descriptor_id } => {
                write!(
                    f,
                    "capture-mode descriptor {descriptor_id} must show an overhead warning"
                )
            }
            Self::IncompleteStorageLocation { location_id } => {
                write!(f, "incomplete storage-location row: {location_id}")
            }
            Self::StorageLocationMissingTruthLabels { location_id } => {
                write!(
                    f,
                    "storage-location row {location_id} must show class, retention, and degraded warnings"
                )
            }
            Self::LauncherCaptureModeRefUnknown {
                launcher_id,
                capture_mode_ref,
            } => {
                write!(
                    f,
                    "launcher {launcher_id} references unknown capture mode {capture_mode_ref}"
                )
            }
            Self::AttachSheetCaptureModeRefUnknown {
                sheet_id,
                capture_mode_ref,
            } => {
                write!(
                    f,
                    "attach sheet {sheet_id} references unknown capture mode {capture_mode_ref}"
                )
            }
            Self::LauncherStorageLocationRefUnknown {
                launcher_id,
                storage_location_ref,
            } => {
                write!(
                    f,
                    "launcher {launcher_id} references unknown storage location {storage_location_ref}"
                )
            }
            Self::AttachSheetStorageLocationRefUnknown {
                sheet_id,
                storage_location_ref,
            } => {
                write!(
                    f,
                    "attach sheet {sheet_id} references unknown storage location {storage_location_ref}"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "computed summary does not match stored summary")
            }
        }
    }
}

impl Error for ProfileLauncherQualificationViolation {}
