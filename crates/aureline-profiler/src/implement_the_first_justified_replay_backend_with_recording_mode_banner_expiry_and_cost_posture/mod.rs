//! Justified replay backend with recording-mode banner, expiry, and cost posture.
//!
//! This module materializes the typed records that keep replay surfaces honest about
//! what recording mode is active, when captures expire, and what overhead or storage
//! cost a replay capture imposes. The records and closed vocabularies here mirror the
//! boundary schema at
//! `/schemas/perf/implement-the-first-justified-replay-backend-with-recording-mode-banner-expiry-and-cost-posture.schema.json`
//! and reuse the capture-class, provenance, mapping-quality, and environment-identity
//! axes already frozen in `/docs/performance/profiling_trace_replay_contract.md`.
//!
//! The module exposes:
//!
//! - the [`RecordingModeBannerRow`] record that binds recording state, backend ref,
//!   allowed verbs, chronology support, reverse-step availability, and the honest reason
//!   when reverse step is unavailable so users never mistake a live session for a
//!   recorded one;
//! - the [`ReplayExpiryRow`] record that carries retention class, expiry timestamp,
//!   freshness state, policy posture, and degraded-state warnings so expired or stale
//!   captures never look current;
//! - the [`ReplayCostPostureRow`] record that carries overhead class, storage band,
//!   cost label, and honest cost warnings so users know the impact before starting
//!   a recording;
//! - the [`ReplayQualificationPacket`] checked-in artifact that downstream docs, help,
//!   support, and CI surfaces ingest instead of cloning status text.
//!
//! Raw payload bytes, raw command lines, secrets, and ambient credentials MUST NOT
//! appear on any record carried here.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version stamped on every replay qualification packet carried by this module.
/// Bumped only on breaking payload changes; additive-optional fields do not bump this value.
pub const REPLAY_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ReplayQualificationPacket`].
pub const REPLAY_QUALIFICATION_RECORD_KIND: &str =
    "implement_the_first_justified_replay_backend_with_recording_mode_banner_expiry_and_cost_posture";

/// Repo-relative path to the checked-in replay qualification packet JSON.
pub const REPLAY_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/perf/m5/implement-the-first-justified-replay-backend-with-recording-mode-banner-expiry-and-cost-posture.json";

/// Embedded checked-in qualification packet JSON.
pub const REPLAY_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/perf/m5/implement-the-first-justified-replay-backend-with-recording-mode-banner-expiry-and-cost-posture.json"
));

/// Qualification label shown on promoted replay surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayQualificationLabel {
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

impl ReplayQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Replay surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplaySurfaceKind {
    /// Recording-mode banner surface.
    RecordingModeBanner,
    /// Replay expiry inspector surface.
    ReplayExpiryInspector,
    /// Cost posture inspector surface.
    CostPostureInspector,
    /// Replay backend control surface.
    ReplayBackend,
    /// Export review surface for replay evidence.
    ExportReview,
    /// Support export surface for replay evidence.
    SupportExport,
}

/// Recording mode state shown on the replay banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordingModeState {
    /// A valid recording is currently being captured.
    Recording,
    /// No recording is active; session is live.
    NotRecording,
    /// A valid recording exists and replay is possible.
    Recorded,
    /// The recording has expired and is no longer replayable.
    Expired,
    /// The current backend or runtime cannot produce a recording.
    Unsupported,
    /// Recording is disabled by organizational or user policy.
    PolicyBlocked,
}

impl RecordingModeState {
    /// Returns true when the state allows replay controls to be shown.
    pub const fn allows_replay(self) -> bool {
        matches!(self, Self::Recorded)
    }

    /// Returns true when the banner should show a degraded-state label.
    pub const fn shows_degraded_label(self) -> bool {
        matches!(
            self,
            Self::Expired | Self::Unsupported | Self::PolicyBlocked
        )
    }
}

/// Expiry status for a replay capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpiryStatus {
    /// Capture is current and within retention policy.
    Current,
    /// Capture is older than freshness policy but still present.
    Stale,
    /// Capture has passed retention expiry.
    Expired,
    /// Capture is missing or was deleted.
    Missing,
    /// Capture is pinned and does not expire.
    Pinned,
    /// Capture retention is blocked by policy.
    PolicyBlocked,
}

impl ExpiryStatus {
    /// Returns true when the capture is considered replayable.
    pub const fn is_replayable(self) -> bool {
        matches!(self, Self::Current | Self::Stale | Self::Pinned)
    }

    /// Returns true when the status should show a degraded-state label.
    pub const fn shows_degraded_label(self) -> bool {
        matches!(
            self,
            Self::Stale | Self::Expired | Self::Missing | Self::PolicyBlocked
        )
    }
}

/// Cost posture class describing overhead and storage impact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CostPostureClass {
    /// Low overhead and small storage footprint.
    Low,
    /// Moderate overhead and medium storage footprint.
    Moderate,
    /// High overhead and large storage footprint.
    High,
    /// Extreme overhead or very large storage footprint.
    Extreme,
    /// Cost is unknown or unmeasured.
    Unknown,
}

impl CostPostureClass {
    /// Returns true when the posture requires an explicit warning before recording.
    pub const fn requires_warning(self) -> bool {
        matches!(self, Self::High | Self::Extreme)
    }

    /// Returns true when the posture blocks automatic recording without consent.
    pub const fn blocks_auto_record(self) -> bool {
        matches!(self, Self::Extreme)
    }
}

/// One recording-mode banner row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordingModeBannerRow {
    /// Stable banner row id.
    pub banner_id: String,
    /// Human-readable title.
    pub title: String,
    /// Recording mode state.
    pub recording_mode_state: RecordingModeState,
    /// Backend ref.
    pub backend_ref: String,
    /// Allowed verbs.
    #[serde(default)]
    pub allowed_verbs: Vec<String>,
    /// Chronology support label.
    pub chronology_support: String,
    /// Reverse-step available flag.
    pub reverse_step_available: bool,
    /// Reason when reverse step is unavailable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reverse_step_unavailable_reason: Option<String>,
    /// True when the banner is present in the promoted build.
    pub promoted_build_surface: bool,
    /// True when the banner shows its recording mode state.
    pub shows_recording_mode_state: bool,
    /// True when the banner shows a degraded-state label when applicable.
    pub shows_degraded_label: bool,
    /// True when the banner shows allowed verbs.
    pub shows_allowed_verbs: bool,
}

/// One replay expiry row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayExpiryRow {
    /// Stable expiry row id.
    pub expiry_id: String,
    /// Human-readable title.
    pub title: String,
    /// Retention class label.
    pub retention_class: String,
    /// Expiry timestamp or null when pinned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Expiry status.
    pub expiry_status: ExpiryStatus,
    /// Policy posture label.
    pub policy_posture: String,
    /// True when the expiry row is present in the promoted build.
    pub promoted_build_surface: bool,
    /// True when the row shows its retention class.
    pub shows_retention_class: bool,
    /// True when the row shows its expiry status.
    pub shows_expiry_status: bool,
    /// True when the row warns on degraded state.
    pub warns_on_degraded_state: bool,
}

/// One replay cost-posture row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayCostPostureRow {
    /// Stable cost-posture row id.
    pub cost_id: String,
    /// Human-readable title.
    pub title: String,
    /// Cost posture class.
    pub cost_posture_class: CostPostureClass,
    /// Overhead class label.
    pub overhead_class: String,
    /// Storage band label.
    pub storage_band: String,
    /// True when the cost-posture row is present in the promoted build.
    pub promoted_build_surface: bool,
    /// True when the row shows its cost posture class.
    pub shows_cost_posture_class: bool,
    /// True when the row shows an overhead warning when applicable.
    pub shows_overhead_warning: bool,
    /// True when the row shows a storage cost note.
    pub shows_storage_cost_note: bool,
    /// True when the row blocks automatic recording for extreme cost.
    pub blocks_auto_record_when_extreme: bool,
}

/// Checked-in proof bundle for one surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayQualificationProof {
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
pub struct ReplayQualificationSummary {
    /// Total number of recording-mode banner rows.
    pub recording_mode_banner_count: usize,
    /// Total number of replay expiry rows.
    pub replay_expiry_count: usize,
    /// Total number of replay cost-posture rows.
    pub replay_cost_posture_count: usize,
    /// Number of rows claiming stable.
    pub stable_count: usize,
    /// Number of rows below stable.
    pub below_stable_count: usize,
    /// True when every row has a non-empty disclosure ref if below stable.
    pub all_below_stable_have_disclosure: bool,
    /// Number of banners that show degraded labels when applicable.
    pub banner_showing_degraded_count: usize,
    /// Number of expiry rows that warn on degraded state.
    pub expiry_warning_count: usize,
    /// Number of cost-posture rows that show overhead warnings.
    pub cost_warning_count: usize,
}

/// Guard set for a replay surface qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplaySurfaceGuardSet {
    /// Recording-mode banner is visible.
    pub recording_mode_banner_visible: bool,
    /// Replay expiry is visible.
    pub replay_expiry_visible: bool,
    /// Cost posture is visible.
    pub cost_posture_visible: bool,
    /// Allowed verbs are visible.
    pub allowed_verbs_visible: bool,
    /// Degraded-state label is visible when applicable.
    pub degraded_state_label_visible: bool,
    /// Export posture is visible.
    pub export_posture_visible: bool,
    /// Retention and policy posture is visible.
    pub retention_policy_visible: bool,
    /// Cost warning is visible when applicable.
    pub cost_warning_visible: bool,
}

/// One surface qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplaySurfaceQualificationRow {
    /// Surface id.
    pub surface_id: String,
    /// Surface title.
    pub title: String,
    /// Surface kind.
    pub surface_kind: ReplaySurfaceKind,
    /// True when the surface is present in the promoted build.
    pub promoted_build_surface: bool,
    /// Claim label.
    pub claim_label: ReplayQualificationLabel,
    /// Displayed label (may differ from claim when narrowed).
    pub displayed_label: String,
    /// Qualification proof bundle.
    pub qualification_packet: ReplayQualificationProof,
    /// Guard set.
    pub guards: ReplaySurfaceGuardSet,
    /// True when the surface downgrades if required guards are missing.
    pub downgrade_if_missing: bool,
    /// Rationale string.
    pub rationale: String,
}

/// The checked-in replay qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayQualificationPacket {
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
    pub surfaces: Vec<ReplaySurfaceQualificationRow>,
    /// Recording-mode banner rows.
    pub recording_mode_banners: Vec<RecordingModeBannerRow>,
    /// Replay expiry rows.
    pub replay_expiries: Vec<ReplayExpiryRow>,
    /// Replay cost-posture rows.
    pub replay_cost_postures: Vec<ReplayCostPostureRow>,
    /// Summary.
    pub summary: ReplayQualificationSummary,
}

impl ReplayQualificationPacket {
    /// Computes the summary from current rows.
    pub fn computed_summary(&self) -> ReplayQualificationSummary {
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
        let banner_showing_degraded_count = self
            .recording_mode_banners
            .iter()
            .filter(|b| b.shows_degraded_label)
            .count();
        let expiry_warning_count = self
            .replay_expiries
            .iter()
            .filter(|e| e.warns_on_degraded_state)
            .count();
        let cost_warning_count = self
            .replay_cost_postures
            .iter()
            .filter(|c| c.shows_overhead_warning)
            .count();

        ReplayQualificationSummary {
            recording_mode_banner_count: self.recording_mode_banners.len(),
            replay_expiry_count: self.replay_expiries.len(),
            replay_cost_posture_count: self.replay_cost_postures.len(),
            stable_count,
            below_stable_count,
            all_below_stable_have_disclosure,
            banner_showing_degraded_count,
            expiry_warning_count,
            cost_warning_count,
        }
    }

    /// Validates the packet and returns any violations.
    pub fn validate(&self) -> Vec<ReplayQualificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != REPLAY_QUALIFICATION_SCHEMA_VERSION {
            violations.push(ReplayQualificationViolation::SchemaVersion {
                expected: REPLAY_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }

        if self.record_kind != REPLAY_QUALIFICATION_RECORD_KIND {
            violations.push(ReplayQualificationViolation::RecordKind {
                expected: REPLAY_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        let mut surface_ids = BTreeSet::new();
        for surface in &self.surfaces {
            if !surface_ids.insert(surface.surface_id.clone()) {
                violations.push(ReplayQualificationViolation::DuplicateId {
                    kind: ReplayQualificationViolationKind::Surface,
                    id: surface.surface_id.clone(),
                });
            }
            if surface.promoted_build_surface
                && surface.claim_label.is_stable()
                && (!surface.guards.recording_mode_banner_visible
                    || !surface.guards.replay_expiry_visible
                    || !surface.guards.cost_posture_visible
                    || !surface.guards.degraded_state_label_visible
                    || !surface.guards.retention_policy_visible
                    || !surface.guards.cost_warning_visible)
            {
                violations.push(ReplayQualificationViolation::IncompleteGuardSet {
                    surface_id: surface.surface_id.clone(),
                });
            }
        }

        let mut banner_ids = BTreeSet::new();
        for banner in &self.recording_mode_banners {
            if !banner_ids.insert(banner.banner_id.clone()) {
                violations.push(ReplayQualificationViolation::DuplicateId {
                    kind: ReplayQualificationViolationKind::RecordingModeBanner,
                    id: banner.banner_id.clone(),
                });
            }
            if banner.banner_id.trim().is_empty()
                || banner.title.trim().is_empty()
                || banner.backend_ref.trim().is_empty()
                || banner.chronology_support.trim().is_empty()
            {
                violations.push(
                    ReplayQualificationViolation::IncompleteRecordingModeBanner {
                        banner_id: banner.banner_id.clone(),
                    },
                );
            }
            if !banner.shows_recording_mode_state {
                violations.push(
                    ReplayQualificationViolation::RecordingModeBannerMissingState {
                        banner_id: banner.banner_id.clone(),
                    },
                );
            }
            if !banner.shows_degraded_label {
                violations.push(
                    ReplayQualificationViolation::RecordingModeBannerMissingDegradedLabel {
                        banner_id: banner.banner_id.clone(),
                    },
                );
            }
            if banner.recording_mode_state.shows_degraded_label() && !banner.shows_degraded_label {
                violations.push(
                    ReplayQualificationViolation::RecordingModeBannerMissingDegradedLabel {
                        banner_id: banner.banner_id.clone(),
                    },
                );
            }
        }

        let mut expiry_ids = BTreeSet::new();
        for expiry in &self.replay_expiries {
            if !expiry_ids.insert(expiry.expiry_id.clone()) {
                violations.push(ReplayQualificationViolation::DuplicateId {
                    kind: ReplayQualificationViolationKind::ReplayExpiry,
                    id: expiry.expiry_id.clone(),
                });
            }
            if expiry.expiry_id.trim().is_empty()
                || expiry.title.trim().is_empty()
                || expiry.retention_class.trim().is_empty()
                || expiry.policy_posture.trim().is_empty()
            {
                violations.push(ReplayQualificationViolation::IncompleteReplayExpiry {
                    expiry_id: expiry.expiry_id.clone(),
                });
            }
            if !expiry.shows_retention_class
                || !expiry.shows_expiry_status
                || !expiry.warns_on_degraded_state
            {
                violations.push(
                    ReplayQualificationViolation::ReplayExpiryMissingTruthLabels {
                        expiry_id: expiry.expiry_id.clone(),
                    },
                );
            }
            if expiry.expiry_status.shows_degraded_label() && !expiry.warns_on_degraded_state {
                violations.push(
                    ReplayQualificationViolation::ReplayExpiryMissingTruthLabels {
                        expiry_id: expiry.expiry_id.clone(),
                    },
                );
            }
        }

        let mut cost_ids = BTreeSet::new();
        for cost in &self.replay_cost_postures {
            if !cost_ids.insert(cost.cost_id.clone()) {
                violations.push(ReplayQualificationViolation::DuplicateId {
                    kind: ReplayQualificationViolationKind::ReplayCostPosture,
                    id: cost.cost_id.clone(),
                });
            }
            if cost.cost_id.trim().is_empty()
                || cost.title.trim().is_empty()
                || cost.overhead_class.trim().is_empty()
                || cost.storage_band.trim().is_empty()
            {
                violations.push(ReplayQualificationViolation::IncompleteReplayCostPosture {
                    cost_id: cost.cost_id.clone(),
                });
            }
            if !cost.shows_cost_posture_class {
                violations.push(
                    ReplayQualificationViolation::ReplayCostPostureMissingClass {
                        cost_id: cost.cost_id.clone(),
                    },
                );
            }
            if cost.cost_posture_class.requires_warning() && !cost.shows_overhead_warning {
                violations.push(
                    ReplayQualificationViolation::ReplayCostPostureMissingWarning {
                        cost_id: cost.cost_id.clone(),
                    },
                );
            }
            if cost.cost_posture_class.blocks_auto_record() && !cost.blocks_auto_record_when_extreme
            {
                violations.push(
                    ReplayQualificationViolation::ReplayCostPostureMissingAutoBlock {
                        cost_id: cost.cost_id.clone(),
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(ReplayQualificationViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in replay qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_replay_qualification() -> Result<ReplayQualificationPacket, serde_json::Error> {
    serde_json::from_str(REPLAY_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Recording-mode banner rows.
    RecordingModeBanner,
    /// Replay expiry rows.
    ReplayExpiry,
    /// Replay cost-posture rows.
    ReplayCostPosture,
}

impl fmt::Display for ReplayQualificationViolationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Surface => write!(f, "surface"),
            Self::RecordingModeBanner => write!(f, "recording_mode_banner"),
            Self::ReplayExpiry => write!(f, "replay_expiry"),
            Self::ReplayCostPosture => write!(f, "replay_cost_posture"),
        }
    }
}

/// Validation failure for replay qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayQualificationViolation {
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
        kind: ReplayQualificationViolationKind,
        /// Duplicate id.
        id: String,
    },
    /// A surface with a stable claim has an incomplete guard set.
    IncompleteGuardSet {
        /// Surface id.
        surface_id: String,
    },
    /// A recording-mode banner row is incomplete.
    IncompleteRecordingModeBanner {
        /// Banner id.
        banner_id: String,
    },
    /// A recording-mode banner row must show its recording mode state.
    RecordingModeBannerMissingState {
        /// Banner id.
        banner_id: String,
    },
    /// A recording-mode banner row must show a degraded-state label.
    RecordingModeBannerMissingDegradedLabel {
        /// Banner id.
        banner_id: String,
    },
    /// A replay expiry row is incomplete.
    IncompleteReplayExpiry {
        /// Expiry id.
        expiry_id: String,
    },
    /// A replay expiry row must show retention class, expiry status, and degraded warnings.
    ReplayExpiryMissingTruthLabels {
        /// Expiry id.
        expiry_id: String,
    },
    /// A replay cost-posture row is incomplete.
    IncompleteReplayCostPosture {
        /// Cost id.
        cost_id: String,
    },
    /// A replay cost-posture row must show its cost posture class.
    ReplayCostPostureMissingClass {
        /// Cost id.
        cost_id: String,
    },
    /// A replay cost-posture row must show an overhead warning when applicable.
    ReplayCostPostureMissingWarning {
        /// Cost id.
        cost_id: String,
    },
    /// A replay cost-posture row must block automatic recording when cost is extreme.
    ReplayCostPostureMissingAutoBlock {
        /// Cost id.
        cost_id: String,
    },
    /// Computed summary does not match the stored summary.
    SummaryMismatch,
}

impl fmt::Display for ReplayQualificationViolation {
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
            Self::IncompleteRecordingModeBanner { banner_id } => {
                write!(f, "incomplete recording-mode banner row: {banner_id}")
            }
            Self::RecordingModeBannerMissingState { banner_id } => {
                write!(
                    f,
                    "recording-mode banner {banner_id} must show its recording mode state"
                )
            }
            Self::RecordingModeBannerMissingDegradedLabel { banner_id } => {
                write!(
                    f,
                    "recording-mode banner {banner_id} must show a degraded-state label"
                )
            }
            Self::IncompleteReplayExpiry { expiry_id } => {
                write!(f, "incomplete replay expiry row: {expiry_id}")
            }
            Self::ReplayExpiryMissingTruthLabels { expiry_id } => {
                write!(
                    f,
                    "replay expiry row {expiry_id} must show retention class, expiry status, and degraded warnings"
                )
            }
            Self::IncompleteReplayCostPosture { cost_id } => {
                write!(f, "incomplete replay cost-posture row: {cost_id}")
            }
            Self::ReplayCostPostureMissingClass { cost_id } => {
                write!(
                    f,
                    "replay cost-posture row {cost_id} must show its cost posture class"
                )
            }
            Self::ReplayCostPostureMissingWarning { cost_id } => {
                write!(
                    f,
                    "replay cost-posture row {cost_id} must show an overhead warning when applicable"
                )
            }
            Self::ReplayCostPostureMissingAutoBlock { cost_id } => {
                write!(
                    f,
                    "replay cost-posture row {cost_id} must block automatic recording when cost is extreme"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "computed summary does not match stored summary")
            }
        }
    }
}

impl Error for ReplayQualificationViolation {}
