// SPDX-FileCopyrightText: 2026 Aureline contributors
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
use super::records::DowngradeTriggerClass;
#[cfg(test)]
use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::fs::{self, File, Metadata, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use super::records::{
    AvailabilityState, CheckpointSchemaVersion, DensityPreset, DirtyBufferJournalIdentity,
    DowngradeTriggerRecord, ExcludedLiveAuthorityClass, FocusChainEntry, FocusTargetKind,
    FollowMode, FollowPresentationState, HydrationBehavior, MonitorAffinityHint,
    MonitorAffinityStrength, PaneLeafNode, PaneNode, PaneSurfaceDescriptor, PaneTree,
    PaneTreeSchemaVersion, PlaceholderAction, PlaceholderBehaviorRecord, PlaceholderCard,
    PlaceholderReasonClass, ProducerBuildStamp, RestoreClass, ScopeRefs, SnapshotReason,
    SplitOrientation, StablePaneInventoryEntry, SurfaceClass, SurfaceRole, TabGroupInventoryEntry,
    TabRecord, TerminalPaneRestoreMetadata, TopologyPacketSchemaVersion, TrustedRootRecord,
    WindowChromeState, WindowRole, WindowState, WindowTopologySnapshotBodyRecord,
    WindowTopologySnapshotRecord, WorkspaceAuthorityCheckpointRecord,
};

/// Error returned when session-restore persistence fails.
#[derive(Debug)]
pub enum SessionRestoreError {
    Io(std::io::Error),
    Json(serde_json::Error),
    MissingRecord(String),
    InvalidCapture(&'static str),
    CorruptStore(&'static str),
    /// At least one immutable member of this capture reached its create-new
    /// install point, but the complete joined publication could not be proven
    /// durable. Callers must reopen the store before deciding whether to retry.
    CommitStateUncertain(SessionRestoreLatestRefs),
}

impl std::fmt::Display for SessionRestoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "session restore io error: {err}"),
            Self::Json(err) => write!(f, "session restore json error: {err}"),
            Self::MissingRecord(detail) => write!(f, "session restore missing record: {detail}"),
            Self::InvalidCapture(detail) => {
                write!(f, "session restore capture rejected: {detail}")
            }
            Self::CorruptStore(detail) => write!(f, "session restore store is corrupt: {detail}"),
            Self::CommitStateUncertain(_) => write!(
                f,
                "session restore capture commit state is uncertain; reopen before retrying"
            ),
        }
    }
}

impl std::error::Error for SessionRestoreError {}

impl From<std::io::Error> for SessionRestoreError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for SessionRestoreError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

/// Identifier source used by session-restore stores.
#[derive(Debug, Clone)]
struct IdSource {
    prefix: &'static str,
    next_seq: u64,
}

impl IdSource {
    const fn new(prefix: &'static str) -> Self {
        Self {
            prefix,
            next_seq: 1,
        }
    }

    fn mint(&mut self) -> Result<String, SessionRestoreError> {
        let seq = self.next_seq;
        self.next_seq = self
            .next_seq
            .checked_add(1)
            .ok_or(SessionRestoreError::CorruptStore(
                "durable id sequence exhausted",
            ))?;
        let stamp = unix_nanos();
        Ok(format!(
            "{prefix}-{stamp:020}-{seq:020}",
            prefix = self.prefix
        ))
    }

    fn seed_next(&mut self, next_seq: u64) {
        self.next_seq = next_seq;
    }
}

fn unix_nanos() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
}

static RECOVERY_TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Capture input describing one tab in a restored topology.
#[derive(Debug, Clone)]
pub struct TabItemCaptureInput {
    pub tab_id: String,
    pub tab_label: Option<String>,
    /// Opaque restore binding for the surface (for example, a logical
    /// document id). Raw paths, URLs, and live capability handles are not
    /// valid bindings.
    pub surface_binding_ref: Option<String>,
    pub pinned: bool,
    pub dirty_badge_visible: bool,
    pub surface_role: SurfaceRole,
    pub surface_class: SurfaceClass,
    pub restore_metadata: Option<TerminalPaneRestoreMetadata>,
}

/// Capture input describing one tab group in a restored topology.
#[derive(Debug, Clone)]
pub struct TabGroupCaptureInput {
    pub group_id: String,
    pub ordered_tabs: Vec<TabItemCaptureInput>,
    pub active_tab_id: Option<String>,
}

/// Structural layout of the captured tab groups.
///
/// This mirrors only durable window topology. Leaves refer to group ids from
/// [`SessionRestoreCaptureInput::tab_groups`]; split ids, orientation, child
/// order, and optional weights are preserved verbatim.
#[derive(Debug, Clone, PartialEq)]
pub enum TabGroupLayoutCapture {
    TabGroup {
        group_id: String,
    },
    Split {
        split_id: String,
        orientation: SplitOrientation,
        children: Vec<TabGroupLayoutCapture>,
        weights: Option<Vec<f64>>,
    },
}

/// Capture input for one session-restore snapshot.
#[derive(Debug, Clone)]
pub struct SessionRestoreCaptureInput {
    pub workspace_ref: String,
    pub producer_build: ProducerBuildStamp,
    pub source_schema_version: String,
    pub trusted_root_refs: Vec<TrustedRootRecord>,
    pub active_workset_ids: Vec<String>,
    pub dirty_buffer_journal_identities: Vec<DirtyBufferJournalIdentity>,
    pub recovery_journal_refs: Vec<String>,
    pub local_history_snapshot_refs: Vec<String>,
    pub evidence_bundle_refs: Vec<String>,
    pub excluded_live_authority_classes: Vec<ExcludedLiveAuthorityClass>,
    pub downgrade_triggers: Vec<DowngradeTriggerRecord>,
    pub window_id: String,
    pub window_role: WindowRole,
    pub topology_family_ref: Option<String>,
    pub sibling_window_refs: Vec<String>,
    pub tab_groups: Vec<TabGroupCaptureInput>,
    pub pane_tree_layout: Option<TabGroupLayoutCapture>,
    /// Window-local tab group that owned focus at capture time.
    ///
    /// This is a structural ref only. It never carries live input authority.
    pub focused_group_id: Option<String>,
    pub emitted_at: String,
    pub notes: Option<String>,
}

/// Latest captured refs for session-restore artifacts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionRestoreLatestRefs {
    pub checkpoint_id: String,
    pub snapshot_id: String,
}

/// Typed reason a newer immutable restore candidate was not selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionRestoreSelectionWarningClass {
    CorruptIndex,
    InvalidIndexReference,
    InvalidJoinedCapture,
}

/// Redaction-safe evidence that a newer restore candidate was skipped.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionRestoreSelectionWarning {
    pub snapshot_id: String,
    pub warning_class: SessionRestoreSelectionWarningClass,
}

/// Latest valid restore selection plus any newer corrupt candidates skipped.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SessionRestoreSelection {
    pub latest_refs: Option<SessionRestoreLatestRefs>,
    pub skipped_newer_candidates: Vec<SessionRestoreSelectionWarning>,
}

/// Summary of the latest session-restore snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionRestoreSummary {
    pub restore_class: RestoreClass,
    pub checkpoint_id: String,
    pub snapshot_id: String,
    pub window_id: String,
    pub tab_group_count: usize,
    pub tab_count: usize,
    pub dirty_buffer_journal_count: usize,
    pub skipped_newer_candidate_count: usize,
}

#[derive(Debug)]
pub(crate) struct ReconciledSessionRestoreCapture {
    pub(crate) checkpoint: WorkspaceAuthorityCheckpointRecord,
    pub(crate) topology: WindowTopologySnapshotRecord,
    pub(crate) pane_tree_body: WindowTopologySnapshotBodyRecord,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct LatestIndexRecord {
    record_kind: String,
    latest_index_schema_version: u32,
    checkpoint_id: String,
    snapshot_id: String,
    emitted_at: String,
}

/// File-backed store for session-restore skeleton artifacts.
#[derive(Debug)]
pub struct SessionRestoreStore {
    root: PathBuf,
    checkpoint_ids: IdSource,
    snapshot_ids: IdSource,
    ids_initialized: bool,
}

impl SessionRestoreStore {
    /// Creates a store rooted at `root_dir/session_restore`.
    pub fn new(root_dir: impl AsRef<Path>) -> Self {
        let root = root_dir.as_ref().join("session_restore");
        Self {
            root,
            checkpoint_ids: IdSource::new("ckpt"),
            snapshot_ids: IdSource::new("snap"),
            ids_initialized: false,
        }
    }

    /// Returns the on-disk root path for this store.
    pub fn root_path(&self) -> &Path {
        &self.root
    }

    /// Captures and writes a workspace checkpoint + topology packet + pane-tree body.
    pub fn capture(
        &mut self,
        input: SessionRestoreCaptureInput,
    ) -> Result<SessionRestoreLatestRefs, SessionRestoreError> {
        validate_capture_input(&input)?;
        self.initialize_id_sources()?;
        let checkpoint_id = self.checkpoint_ids.mint()?;
        let snapshot_id = self.snapshot_ids.mint()?;
        let capture_refs = SessionRestoreLatestRefs {
            checkpoint_id: checkpoint_id.clone(),
            snapshot_id: snapshot_id.clone(),
        };
        let workspace_authority_ref = format!("workspace-authority:{}", input.workspace_ref);

        let restore_class = if input.dirty_buffer_journal_identities.is_empty() {
            RestoreClass::LayoutOnly
        } else {
            RestoreClass::RecoveredDrafts
        };
        let checkpoint_downgrade_triggers = input
            .downgrade_triggers
            .iter()
            .map(checkpoint_downgrade_trigger)
            .collect();
        let topology_downgrade_triggers = input
            .downgrade_triggers
            .iter()
            .map(topology_downgrade_trigger)
            .collect();

        let checkpoint = WorkspaceAuthorityCheckpointRecord {
            schema: None,
            fixture: None,
            record_kind: "workspace_authority_checkpoint_record".to_string(),
            checkpoint_schema_version: 1 as CheckpointSchemaVersion,
            checkpoint_id: checkpoint_id.clone(),
            workspace_authority_ref: workspace_authority_ref.clone(),
            producer_build: input.producer_build.clone(),
            source_schema_version: input.source_schema_version.clone(),
            restore_class,
            trusted_root_refs: input.trusted_root_refs.clone(),
            active_workset_ids: input.active_workset_ids.clone(),
            dirty_buffer_journal_identities: input.dirty_buffer_journal_identities.clone(),
            recovery_journal_refs: input.recovery_journal_refs.clone(),
            local_history_snapshot_refs: input.local_history_snapshot_refs.clone(),
            evidence_bundle_refs: input.evidence_bundle_refs.clone(),
            excluded_live_authority_classes: input.excluded_live_authority_classes.clone(),
            downgrade_triggers: checkpoint_downgrade_triggers,
            rollback_checkpoint_ref: None,
            preserved_prior_artifact_refs: Vec::new(),
            emitted_at: input.emitted_at.clone(),
            notes: input.notes.clone(),
        };

        let (
            tab_group_topology,
            stable_pane_inventory,
            pane_tree_root,
            focus_chain,
            placeholder_behaviors,
        ) = materialize_topology_from_capture(
            &input.tab_groups,
            input.pane_tree_layout.as_ref(),
            input.focused_group_id.as_deref(),
            &input.window_id,
        )?;
        let focus_chain_packet = focus_chain.clone();

        let follow_presentation_state = FollowPresentationState {
            follow_mode: FollowMode::Independent,
            presentation_mode: super::records::PresentationMode::Inactive,
            presenter_participant_ref: None,
            visible_role_badges: Vec::new(),
            shared_control_badge_visible: false,
            audience_breakaway_allowed: true,
        };

        let monitor_affinity_hint = MonitorAffinityHint {
            affinity_strength: MonitorAffinityStrength::None,
            display_class: None,
            last_known_display_ref: None,
            last_known_topology_hash: None,
            preferred_scale_bucket: None,
            preferred_bounds_hint: None,
            best_effort_only: true,
        };

        let topology_packet = WindowTopologySnapshotRecord {
            schema: None,
            fixture: None,
            record_kind: "window_topology_snapshot_record".to_string(),
            topology_packet_schema_version: 1 as TopologyPacketSchemaVersion,
            snapshot_id: snapshot_id.clone(),
            window_id: input.window_id.clone(),
            window_role: input.window_role,
            topology_family_ref: input.topology_family_ref.clone(),
            sibling_window_refs: input.sibling_window_refs.clone(),
            producer_build: input.producer_build.clone(),
            source_schema_version: input.source_schema_version.clone(),
            workspace_authority_checkpoint_ref: checkpoint_id.clone(),
            pane_tree_schema_version: 1 as PaneTreeSchemaVersion,
            pane_tree_record_ref: snapshot_id.clone(),
            stable_pane_id_inventory: stable_pane_inventory,
            tab_group_topology,
            visible_inspectors: Vec::new(),
            focus_chain: focus_chain_packet,
            follow_presentation_state: follow_presentation_state.clone(),
            monitor_affinity_hint: monitor_affinity_hint.clone(),
            placeholder_behaviors,
            topology_adjustments: Vec::new(),
            restore_class,
            downgrade_triggers: topology_downgrade_triggers,
            emitted_at: input.emitted_at.clone(),
            notes: input.notes.clone(),
        };

        let pane_tree_body = WindowTopologySnapshotBodyRecord {
            schema: None,
            fixture: None,
            record_kind: "window_topology_snapshot_record".to_string(),
            pane_tree_schema_version: 1 as PaneTreeSchemaVersion,
            snapshot_id: snapshot_id.clone(),
            snapshot_reason: SnapshotReason::GracefulShutdown,
            window_id: input.window_id.clone(),
            window_role: input.window_role,
            topology_family_ref: input.topology_family_ref.clone(),
            sibling_window_refs: input.sibling_window_refs.clone(),
            scope_refs: ScopeRefs {
                workspace_authority_ref,
                profile_defaults_ref: None,
                machine_display_hint_ref: None,
            },
            pane_tree: PaneTree {
                tree_revision: 1,
                root_node: pane_tree_root,
            },
            focus_chain,
            visible_inspectors: Vec::new(),
            follow_presentation_state,
            window_chrome_state: WindowChromeState {
                window_state: WindowState::Normal,
                zoom_percent: 100.0,
                density_preset: DensityPreset::Comfortable,
                activity_strip_visible: true,
                sidebar_visible: true,
                bottom_panel_visible: true,
            },
            monitor_affinity_hint,
            emitted_at: input.emitted_at.clone(),
            notes: input.notes.clone(),
        };

        // Reject deterministic serialization/size failures before installing
        // the first immutable member. Once publication begins, any later
        // failure must conservatively report commit-state uncertainty.
        preflight_publication_body(&checkpoint)?;
        preflight_publication_body(&topology_packet)?;
        preflight_publication_body(&pane_tree_body)?;

        match write_new_json(
            &self
                .root
                .join("workspace_authority_checkpoints")
                .join(format!("{checkpoint_id}.json")),
            &checkpoint,
        ) {
            Ok(PublicationOutcome::Durable) => {}
            Ok(PublicationOutcome::CommitStateUncertain) => {
                return Err(commit_state_uncertain(&capture_refs));
            }
            Err(error) => return Err(error),
        }

        require_capture_publication(
            write_new_json(
                &self
                    .root
                    .join("window_topology_snapshots")
                    .join(format!("{snapshot_id}.json")),
                &topology_packet,
            ),
            &capture_refs,
        )?;

        require_capture_publication(
            write_new_json(
                &self
                    .root
                    .join("pane_tree_bodies")
                    .join(format!("{snapshot_id}.json")),
                &pane_tree_body,
            ),
            &capture_refs,
        )?;

        require_capture_publication(
            self.write_latest_index(&checkpoint_id, &snapshot_id, &input.emitted_at),
            &capture_refs,
        )?;

        if recovery_io_failpoint(RecoveryIoFailpoint::BeforeCaptureValidation).is_err()
            || !matches!(self.reconcile_capture(&capture_refs), Ok(true))
        {
            return Err(commit_state_uncertain(&capture_refs));
        }

        Ok(capture_refs)
    }

    /// Loads the latest captured refs, if any.
    pub fn latest_refs(&self) -> Result<Option<SessionRestoreLatestRefs>, SessionRestoreError> {
        Ok(self.latest_selection()?.latest_refs)
    }

    /// Selects the newest valid joined capture and retains typed evidence for
    /// every newer immutable candidate that had to be skipped.
    pub fn latest_selection(&self) -> Result<SessionRestoreSelection, SessionRestoreError> {
        let (has_versioned_index, selection) = self.newest_versioned_index_selection()?;
        if has_versioned_index {
            return Ok(selection);
        }

        // Stores without an immutable versioned index have no committed
        // selection authority. `latest.json` is retained only as legacy
        // advisory evidence; selecting a body merely because it is fully
        // written would promote an interrupted pre-index capture.
        Ok(SessionRestoreSelection::default())
    }

    /// Loads a summary for the latest captured snapshot.
    pub fn latest_summary(&self) -> Result<Option<SessionRestoreSummary>, SessionRestoreError> {
        let selection = self.latest_selection()?;
        let skipped_newer_candidate_count = selection.skipped_newer_candidates.len();
        let Some(latest) = selection.latest_refs else {
            return Ok(None);
        };

        // Reopen the same durable join once for the summary. Loading the
        // checkpoint and snapshot independently would permit a same-id path
        // replacement between reads to splice unrelated, individually valid
        // records into one status surface.
        let joined =
            self.load_reconciled_capture(&latest)?
                .ok_or(SessionRestoreError::CorruptStore(
                    "selected restore capture changed before summary materialization",
                ))?;
        let checkpoint = joined.checkpoint;
        let snapshot = joined.topology;

        let tab_group_count = snapshot.tab_group_topology.len();
        let tab_count = snapshot
            .tab_group_topology
            .iter()
            .map(|group| group.ordered_tab_ids.len())
            .sum();

        Ok(Some(SessionRestoreSummary {
            // The window-local topology may lawfully be narrower than the
            // authority checkpoint; surface the effective window fidelity.
            restore_class: snapshot.restore_class,
            checkpoint_id: latest.checkpoint_id,
            snapshot_id: latest.snapshot_id,
            window_id: snapshot.window_id,
            tab_group_count,
            tab_count,
            dirty_buffer_journal_count: checkpoint.dirty_buffer_journal_identities.len(),
            skipped_newer_candidate_count,
        }))
    }

    /// Loads a workspace-authority checkpoint record by id.
    pub fn load_checkpoint(
        &self,
        checkpoint_id: &str,
    ) -> Result<WorkspaceAuthorityCheckpointRecord, SessionRestoreError> {
        validate_requested_durable_id(checkpoint_id, "ckpt", "checkpoint")?;
        let checkpoint_path = self
            .root
            .join("workspace_authority_checkpoints")
            .join(format!("{checkpoint_id}.json"));
        let record: WorkspaceAuthorityCheckpointRecord =
            read_json(&checkpoint_path).map_err(|_| {
                SessionRestoreError::MissingRecord("checkpoint unavailable".to_string())
            })?;
        if !checkpoint_record_is_valid(&record, checkpoint_id) {
            return Err(SessionRestoreError::MissingRecord(
                "checkpoint unavailable".to_string(),
            ));
        }
        Ok(record)
    }

    /// Loads a window-topology snapshot packet record by id.
    pub fn load_window_topology_snapshot(
        &self,
        snapshot_id: &str,
    ) -> Result<WindowTopologySnapshotRecord, SessionRestoreError> {
        validate_requested_durable_id(snapshot_id, "snap", "snapshot")?;
        let snapshot_path = self
            .root
            .join("window_topology_snapshots")
            .join(format!("{snapshot_id}.json"));
        let record: WindowTopologySnapshotRecord = read_json(&snapshot_path)
            .map_err(|_| SessionRestoreError::MissingRecord("snapshot unavailable".to_string()))?;
        if !snapshot_record_is_valid(&record, snapshot_id) {
            return Err(SessionRestoreError::MissingRecord(
                "snapshot unavailable".to_string(),
            ));
        }
        Ok(record)
    }

    /// Loads a canonical pane-tree body for a window-topology snapshot id.
    pub fn load_pane_tree_body(
        &self,
        snapshot_id: &str,
    ) -> Result<WindowTopologySnapshotBodyRecord, SessionRestoreError> {
        validate_requested_durable_id(snapshot_id, "snap", "pane tree")?;
        let body_path = self
            .root
            .join("pane_tree_bodies")
            .join(format!("{snapshot_id}.json"));
        let record: WindowTopologySnapshotBodyRecord = read_json(&body_path).map_err(|_| {
            SessionRestoreError::MissingRecord("pane tree body unavailable".to_string())
        })?;
        if !pane_tree_body_record_is_valid(&record, snapshot_id) {
            return Err(SessionRestoreError::MissingRecord(
                "pane tree body unavailable".to_string(),
            ));
        }
        Ok(record)
    }

    fn initialize_id_sources(&mut self) -> Result<(), SessionRestoreError> {
        if self.ids_initialized {
            return Ok(());
        }
        let checkpoint_next =
            next_durable_sequence(&self.root.join("workspace_authority_checkpoints"), "ckpt")?;
        let snapshot_next =
            next_durable_sequence(&self.root.join("window_topology_snapshots"), "snap")?;
        self.checkpoint_ids.seed_next(checkpoint_next);
        self.snapshot_ids.seed_next(snapshot_next);
        self.ids_initialized = true;
        Ok(())
    }

    fn write_latest_index(
        &self,
        checkpoint_id: &str,
        snapshot_id: &str,
        emitted_at: &str,
    ) -> Result<PublicationOutcome, SessionRestoreError> {
        let record = LatestIndexRecord {
            record_kind: "session_restore_latest_index".to_string(),
            latest_index_schema_version: 1,
            checkpoint_id: checkpoint_id.to_string(),
            snapshot_id: snapshot_id.to_string(),
            emitted_at: emitted_at.to_string(),
        };
        let versioned_path = self
            .root
            .join("latest_indices")
            .join(format!("{snapshot_id}.json"));
        if matches!(
            write_new_json(&versioned_path, &record)?,
            PublicationOutcome::CommitStateUncertain
        ) {
            return Ok(PublicationOutcome::CommitStateUncertain);
        }

        // `latest.json` is a legacy advisory pointer. Publish it once with
        // create-new semantics; immutable versioned indices remain canonical
        // so later captures never rely on platform-specific overwrite behavior.
        let advisory_path = self.root.join("latest.json");
        match write_new_json(&advisory_path, &record) {
            Ok(PublicationOutcome::Durable) => {}
            Ok(PublicationOutcome::CommitStateUncertain) => {
                return Ok(PublicationOutcome::CommitStateUncertain);
            }
            Err(SessionRestoreError::Io(err)) if err.kind() == io::ErrorKind::AlreadyExists => {}
            // The canonical immutable index is already durable. Any later
            // advisory failure is therefore post-commit from the capture's
            // point of view and must not masquerade as a pre-commit failure.
            Err(_) => return Ok(PublicationOutcome::CommitStateUncertain),
        }
        Ok(PublicationOutcome::Durable)
    }

    /// Reopens and validates one exact capture publication by its minted refs.
    ///
    /// This is the reconciliation path for [`SessionRestoreError::CommitStateUncertain`].
    /// It never substitutes a different capture and requires the immutable
    /// versioned index as well as the exact checkpoint/snapshot/body join.
    pub fn reconcile_capture(
        &self,
        refs: &SessionRestoreLatestRefs,
    ) -> Result<bool, SessionRestoreError> {
        Ok(self.load_reconciled_capture(refs)?.is_some())
    }

    fn newest_versioned_index_selection(
        &self,
    ) -> Result<(bool, SessionRestoreSelection), SessionRestoreError> {
        let index_dir = self.root.join("latest_indices");
        let entries = match bounded_directory_entries(&index_dir) {
            Ok(entries) => entries,
            Err(SessionRestoreError::Io(err)) if err.kind() == io::ErrorKind::NotFound => {
                return Ok((false, SessionRestoreSelection::default()));
            }
            Err(err) => return Err(err),
        };
        let mut candidates = Vec::new();
        for entry in entries {
            if !entry_is_direct_regular_file(&entry)?
                || entry.path().extension().and_then(|ext| ext.to_str()) != Some("json")
            {
                continue;
            }
            let file_name = entry.file_name();
            let file_name = file_name.to_str().ok_or(SessionRestoreError::CorruptStore(
                "latest-index filename is not UTF-8",
            ))?;
            let snapshot_id =
                file_name
                    .strip_suffix(".json")
                    .ok_or(SessionRestoreError::CorruptStore(
                        "latest-index filename has no JSON suffix",
                    ))?;
            let order = parse_durable_record_id(snapshot_id, "snap")?;
            candidates.push((order, snapshot_id.to_string(), entry.path()));
        }
        candidates.sort_by(|left, right| right.0.cmp(&left.0));
        if candidates.is_empty() {
            // The directory itself is format evidence. An interrupted first
            // capture may have created it before installing an index, so body
            // scanning must not become a fallback authority.
            return Ok((true, SessionRestoreSelection::default()));
        }

        let mut skipped_newer_candidates = Vec::new();
        for (_, indexed_snapshot_id, path) in candidates {
            let record = match read_json::<LatestIndexRecord>(&path) {
                Ok(record) => record,
                Err(SessionRestoreError::Io(error)) if error.kind() == io::ErrorKind::NotFound => {
                    skipped_newer_candidates.push(SessionRestoreSelectionWarning {
                        snapshot_id: indexed_snapshot_id,
                        warning_class: SessionRestoreSelectionWarningClass::CorruptIndex,
                    });
                    continue;
                }
                Err(SessionRestoreError::Json(_) | SessionRestoreError::CorruptStore(_)) => {
                    skipped_newer_candidates.push(SessionRestoreSelectionWarning {
                        snapshot_id: indexed_snapshot_id,
                        warning_class: SessionRestoreSelectionWarningClass::CorruptIndex,
                    });
                    continue;
                }
                Err(error) => return Err(error),
            };
            let refs = SessionRestoreLatestRefs {
                checkpoint_id: record.checkpoint_id.clone(),
                snapshot_id: record.snapshot_id.clone(),
            };
            if refs.snapshot_id != indexed_snapshot_id || !valid_latest_index_record(&record, &refs)
            {
                skipped_newer_candidates.push(SessionRestoreSelectionWarning {
                    snapshot_id: indexed_snapshot_id,
                    warning_class: SessionRestoreSelectionWarningClass::InvalidIndexReference,
                });
                continue;
            }
            if self.reconcile_capture(&refs)? {
                return Ok((
                    true,
                    SessionRestoreSelection {
                        latest_refs: Some(refs),
                        skipped_newer_candidates,
                    },
                ));
            }
            skipped_newer_candidates.push(SessionRestoreSelectionWarning {
                snapshot_id: indexed_snapshot_id,
                warning_class: SessionRestoreSelectionWarningClass::InvalidJoinedCapture,
            });
        }
        Ok((
            true,
            SessionRestoreSelection {
                latest_refs: None,
                skipped_newer_candidates,
            },
        ))
    }

    pub(crate) fn load_reconciled_capture(
        &self,
        refs: &SessionRestoreLatestRefs,
    ) -> Result<Option<ReconciledSessionRestoreCapture>, SessionRestoreError> {
        if parse_durable_record_id(&refs.checkpoint_id, "ckpt").is_err()
            || parse_durable_record_id(&refs.snapshot_id, "snap").is_err()
        {
            return Ok(None);
        }
        let checkpoint_path = self
            .root
            .join("workspace_authority_checkpoints")
            .join(format!("{}.json", refs.checkpoint_id));
        let snapshot_path = self
            .root
            .join("window_topology_snapshots")
            .join(format!("{}.json", refs.snapshot_id));
        let body_path = self
            .root
            .join("pane_tree_bodies")
            .join(format!("{}.json", refs.snapshot_id));
        let index_path = self
            .root
            .join("latest_indices")
            .join(format!("{}.json", refs.snapshot_id));
        let Some(checkpoint) =
            read_json_for_selection::<WorkspaceAuthorityCheckpointRecord>(&checkpoint_path)?
        else {
            return Ok(None);
        };
        let Some(snapshot) =
            read_json_for_selection::<WindowTopologySnapshotRecord>(&snapshot_path)?
        else {
            return Ok(None);
        };
        let Some(body) = read_json_for_selection::<WindowTopologySnapshotBodyRecord>(&body_path)?
        else {
            return Ok(None);
        };
        let Some(index) = read_json_for_selection::<LatestIndexRecord>(&index_path)? else {
            return Ok(None);
        };
        let valid = checkpoint_record_is_valid(&checkpoint, &refs.checkpoint_id)
            && snapshot_record_is_valid(&snapshot, &refs.snapshot_id)
            && pane_tree_body_record_is_valid(&body, &refs.snapshot_id)
            && index.checkpoint_id == refs.checkpoint_id
            && index.snapshot_id == refs.snapshot_id
            && index.emitted_at == checkpoint.emitted_at
            && valid_latest_index_record(&index, refs)
            && snapshot.workspace_authority_checkpoint_ref == refs.checkpoint_id
            && snapshot.pane_tree_record_ref == refs.snapshot_id
            && restore_class_is_no_broader_than(snapshot.restore_class, checkpoint.restore_class)
            && snapshot.source_schema_version == checkpoint.source_schema_version
            && snapshot.producer_build == checkpoint.producer_build
            && snapshot.emitted_at == checkpoint.emitted_at
            && downgrade_trigger_partitions_join(&checkpoint, &snapshot)
            && body.window_id == snapshot.window_id
            && body.window_role == snapshot.window_role
            && body.topology_family_ref == snapshot.topology_family_ref
            && body.sibling_window_refs == snapshot.sibling_window_refs
            && body.pane_tree_schema_version == snapshot.pane_tree_schema_version
            && body.emitted_at == snapshot.emitted_at
            && body.notes == snapshot.notes
            && body.focus_chain == snapshot.focus_chain
            && body.follow_presentation_state == snapshot.follow_presentation_state
            && body.monitor_affinity_hint == snapshot.monitor_affinity_hint
            && inspectors_join(&snapshot.visible_inspectors, &body.visible_inspectors)
            && body.scope_refs.workspace_authority_ref == checkpoint.workspace_authority_ref
            && joined_topology_semantics_are_valid(&snapshot, &body);
        Ok(valid.then_some(ReconciledSessionRestoreCapture {
            checkpoint,
            topology: snapshot,
            pane_tree_body: body,
        }))
    }
}

fn checkpoint_record_is_valid(
    checkpoint: &WorkspaceAuthorityCheckpointRecord,
    expected_checkpoint_id: &str,
) -> bool {
    checkpoint.record_kind == "workspace_authority_checkpoint_record"
        && checkpoint.checkpoint_schema_version == 1
        && checkpoint.checkpoint_id == expected_checkpoint_id
        && parse_durable_record_id(expected_checkpoint_id, "ckpt").is_ok()
        && checkpoint.fixture.is_none()
        && optional_schema_hint_is_valid(checkpoint.schema.as_deref())
        && is_bounded_opaque_ref(&checkpoint.workspace_authority_ref)
        && producer_build_is_valid(&checkpoint.producer_build)
        && is_bounded_capture_text(&checkpoint.source_schema_version, 128, false)
        && is_bounded_capture_text(&checkpoint.emitted_at, 128, false)
        && optional_capture_text_is_valid(checkpoint.notes.as_deref(), MAX_CAPTURE_NOTE_BYTES, true)
        && checkpoint.trusted_root_refs.len() <= MAX_CAPTURE_REF_LIST_ITEMS
        && checkpoint.active_workset_ids.len() <= MAX_CAPTURE_REF_LIST_ITEMS
        && checkpoint.dirty_buffer_journal_identities.len() <= MAX_CAPTURE_REF_LIST_ITEMS
        && checkpoint.recovery_journal_refs.len() <= MAX_CAPTURE_REF_LIST_ITEMS
        && checkpoint.local_history_snapshot_refs.len() <= MAX_CAPTURE_REF_LIST_ITEMS
        && checkpoint.evidence_bundle_refs.len() <= MAX_CAPTURE_REF_LIST_ITEMS
        && checkpoint.excluded_live_authority_classes.len() <= MAX_CAPTURE_REF_LIST_ITEMS
        && checkpoint.downgrade_triggers.len() <= MAX_CAPTURE_DOWNGRADE_TRIGGERS
        && checkpoint.preserved_prior_artifact_refs.len() <= MAX_CAPTURE_REF_LIST_ITEMS
        && trusted_roots_are_valid(&checkpoint.trusted_root_refs)
        && dirty_journals_are_valid(&checkpoint.dirty_buffer_journal_identities)
        && bounded_unique_refs(&checkpoint.active_workset_ids)
        && bounded_unique_refs(&checkpoint.recovery_journal_refs)
        && bounded_unique_refs(&checkpoint.local_history_snapshot_refs)
        && bounded_unique_refs(&checkpoint.evidence_bundle_refs)
        && bounded_unique_refs(&checkpoint.preserved_prior_artifact_refs)
        && no_duplicates(&checkpoint.excluded_live_authority_classes)
        && checkpoint
            .rollback_checkpoint_ref
            .as_deref()
            .map_or(true, is_bounded_opaque_ref)
        && checkpoint
            .downgrade_triggers
            .iter()
            .all(checkpoint_downgrade_trigger_is_valid)
        && checkpoint_trigger_scopes_join(checkpoint)
        && (!matches!(checkpoint.restore_class, RestoreClass::ExactRestore)
            || checkpoint.downgrade_triggers.is_empty())
        && (!matches!(checkpoint.restore_class, RestoreClass::RecoveredDrafts)
            || !checkpoint.dirty_buffer_journal_identities.is_empty())
}

fn snapshot_record_is_valid(
    snapshot: &WindowTopologySnapshotRecord,
    expected_snapshot_id: &str,
) -> bool {
    snapshot.record_kind == "window_topology_snapshot_record"
        && snapshot.topology_packet_schema_version == 1
        && snapshot.pane_tree_schema_version == 1
        && snapshot.snapshot_id == expected_snapshot_id
        && parse_durable_record_id(expected_snapshot_id, "snap").is_ok()
        && snapshot.fixture.is_none()
        && optional_schema_hint_is_valid(snapshot.schema.as_deref())
        && is_bounded_opaque_ref(&snapshot.window_id)
        && snapshot
            .topology_family_ref
            .as_deref()
            .map_or(true, is_bounded_opaque_ref)
        && snapshot.sibling_window_refs.len() <= MAX_CAPTURE_REF_LIST_ITEMS
        && bounded_unique_refs(&snapshot.sibling_window_refs)
        && producer_build_is_valid(&snapshot.producer_build)
        && is_bounded_capture_text(&snapshot.source_schema_version, 128, false)
        && is_bounded_opaque_ref(&snapshot.workspace_authority_checkpoint_ref)
        && snapshot.pane_tree_record_ref == expected_snapshot_id
        && !snapshot.stable_pane_id_inventory.is_empty()
        && snapshot.stable_pane_id_inventory.len() <= MAX_CAPTURE_TOTAL_TABS
        && snapshot.tab_group_topology.len() <= MAX_CAPTURE_GROUPS
        && snapshot.visible_inspectors.len() <= MAX_CAPTURE_REF_LIST_ITEMS
        && !snapshot.focus_chain.is_empty()
        && snapshot.focus_chain.len() <= MAX_CAPTURE_TOTAL_TABS + MAX_CAPTURE_REF_LIST_ITEMS
        && snapshot.placeholder_behaviors.len() <= MAX_CAPTURE_TOTAL_TABS
        && snapshot.topology_adjustments.len() <= MAX_CAPTURE_REF_LIST_ITEMS
        && snapshot.downgrade_triggers.len() <= MAX_CAPTURE_DOWNGRADE_TRIGGERS
        && is_bounded_capture_text(&snapshot.emitted_at, 128, false)
        && optional_capture_text_is_valid(snapshot.notes.as_deref(), MAX_CAPTURE_NOTE_BYTES, true)
        && stable_pane_inventory_is_valid(&snapshot.stable_pane_id_inventory)
        && tab_group_inventory_is_valid(&snapshot.tab_group_topology)
        && inspector_inventory_is_valid(&snapshot.visible_inspectors)
        && focus_chain_is_valid(&snapshot.focus_chain)
        && follow_state_is_valid(&snapshot.follow_presentation_state)
        && monitor_hint_is_valid(&snapshot.monitor_affinity_hint)
        && placeholder_inventory_is_valid(&snapshot.placeholder_behaviors)
        && topology_adjustments_are_valid(&snapshot.topology_adjustments)
        && snapshot
            .downgrade_triggers
            .iter()
            .all(topology_downgrade_trigger_is_valid)
        && (!matches!(snapshot.restore_class, RestoreClass::ExactRestore)
            || (snapshot.downgrade_triggers.is_empty()
                && snapshot.placeholder_behaviors.is_empty()
                && snapshot.topology_adjustments.is_empty()))
}

fn pane_tree_body_record_is_valid(
    body: &WindowTopologySnapshotBodyRecord,
    expected_snapshot_id: &str,
) -> bool {
    body.record_kind == "window_topology_snapshot_record"
        && body.pane_tree_schema_version == 1
        && body.snapshot_id == expected_snapshot_id
        && parse_durable_record_id(expected_snapshot_id, "snap").is_ok()
        && body.fixture.is_none()
        && optional_schema_hint_is_valid(body.schema.as_deref())
        && body.pane_tree.tree_revision >= 1
        && !body.focus_chain.is_empty()
        && body.focus_chain.len() <= MAX_CAPTURE_TOTAL_TABS + MAX_CAPTURE_REF_LIST_ITEMS
        && body.visible_inspectors.len() <= MAX_CAPTURE_REF_LIST_ITEMS
        && body.window_chrome_state.zoom_percent.is_finite()
        && body.window_chrome_state.zoom_percent > 0.0
        && is_bounded_opaque_ref(&body.window_id)
        && body
            .topology_family_ref
            .as_deref()
            .map_or(true, is_bounded_opaque_ref)
        && body.sibling_window_refs.len() <= MAX_CAPTURE_REF_LIST_ITEMS
        && bounded_unique_refs(&body.sibling_window_refs)
        && is_bounded_opaque_ref(&body.scope_refs.workspace_authority_ref)
        && body
            .scope_refs
            .profile_defaults_ref
            .as_deref()
            .map_or(true, is_bounded_opaque_ref)
        && body
            .scope_refs
            .machine_display_hint_ref
            .as_deref()
            .map_or(true, is_bounded_opaque_ref)
        && focus_chain_is_valid(&body.focus_chain)
        && body.visible_inspectors.iter().all(|inspector| {
            is_bounded_opaque_ref(&inspector.inspector_id)
                && inspector
                    .target_pane_ref
                    .as_deref()
                    .map_or(true, is_bounded_opaque_ref)
        })
        && follow_state_is_valid(&body.follow_presentation_state)
        && monitor_hint_is_valid(&body.monitor_affinity_hint)
        && is_bounded_capture_text(&body.emitted_at, 128, false)
        && optional_capture_text_is_valid(body.notes.as_deref(), MAX_CAPTURE_NOTE_BYTES, true)
}

fn optional_schema_hint_is_valid(schema: Option<&str>) -> bool {
    optional_capture_text_is_valid(schema, MAX_CAPTURE_LABEL_BYTES, false)
}

fn optional_capture_text_is_valid(value: Option<&str>, max_bytes: usize, multiline: bool) -> bool {
    value.map_or(true, |value| {
        is_bounded_capture_text(value, max_bytes, multiline)
    })
}

fn producer_build_is_valid(build: &ProducerBuildStamp) -> bool {
    is_bounded_capture_text(&build.producer_name, 256, false)
        && is_bounded_capture_text(&build.producer_version, 256, false)
        && build.producer_channel.as_deref().map_or(true, |channel| {
            matches!(channel, "experimental" | "beta" | "stable" | "lts")
        })
        && build
            .producer_platform_class
            .as_deref()
            .map_or(true, |platform| {
                matches!(
                    platform,
                    "macos"
                        | "windows"
                        | "linux"
                        | "container"
                        | "remote_agent"
                        | "managed_cloud"
                        | "other"
                )
            })
        && build
            .producer_instance_handle
            .as_deref()
            .map_or(true, is_bounded_opaque_ref)
}

fn no_duplicates<T: PartialEq>(values: &[T]) -> bool {
    !values
        .iter()
        .enumerate()
        .any(|(index, value)| values[..index].contains(value))
}

fn bounded_unique_refs(refs: &[String]) -> bool {
    no_duplicates(refs) && refs.iter().all(|value| is_bounded_opaque_ref(value))
}

fn trusted_roots_are_valid(roots: &[TrustedRootRecord]) -> bool {
    no_duplicates_by(roots, |root| root.root_id.as_str())
        && roots.iter().all(|root| {
            is_bounded_opaque_ref(&root.root_id)
                && matches!(
                    root.trust_state.as_str(),
                    "trusted" | "restricted" | "pending_evaluation"
                )
                && is_bounded_opaque_ref(&root.scope_ref)
                && root
                    .policy_epoch_ref
                    .as_deref()
                    .map_or(true, is_bounded_opaque_ref)
                && optional_capture_text_is_valid(
                    root.note.as_deref(),
                    MAX_CAPTURE_NOTE_BYTES,
                    true,
                )
        })
}

fn dirty_journals_are_valid(journals: &[DirtyBufferJournalIdentity]) -> bool {
    no_duplicates_by(journals, |journal| journal.journal_id.as_str())
        && journals.iter().all(|journal| {
            is_bounded_opaque_ref(&journal.journal_id)
                && matches!(
                    journal.journal_kind.as_str(),
                    "dirty_buffer_recovery_journal"
                        | "local_history_journal"
                        | "deferred_intent_outbox"
                        | "session_restore_journal"
                        | "terminal_scrollback_restore"
                        | "notebook_output_snapshot"
                        | "checkpoint_lineage_journal"
                )
                && is_bounded_opaque_ref(&journal.last_known_revision_ref)
                && optional_capture_text_is_valid(
                    journal.note.as_deref(),
                    MAX_CAPTURE_NOTE_BYTES,
                    true,
                )
        })
}

fn no_duplicates_by<T, K: PartialEq + ?Sized>(values: &[T], key: impl Fn(&T) -> &K) -> bool {
    !values
        .iter()
        .enumerate()
        .any(|(index, value)| values[..index].iter().any(|prior| key(prior) == key(value)))
}

fn ref_list_is_valid(refs: Option<&Vec<String>>) -> bool {
    refs.map_or(true, |refs| {
        refs.len() <= MAX_CAPTURE_REF_LIST_ITEMS && bounded_unique_refs(refs)
    })
}

fn checkpoint_downgrade_trigger_is_valid(trigger: &DowngradeTriggerRecord) -> bool {
    ref_list_is_valid(trigger.affected_journal_ids.as_ref())
        && ref_list_is_valid(trigger.affected_root_refs.as_ref())
        && ref_list_is_valid(trigger.affected_workset_ids.as_ref())
        && trigger.affected_pane_ids.is_none()
        && optional_capture_text_is_valid(trigger.note.as_deref(), MAX_CAPTURE_NOTE_BYTES, true)
}

fn checkpoint_trigger_scopes_join(checkpoint: &WorkspaceAuthorityCheckpointRecord) -> bool {
    let journal_ids = checkpoint
        .dirty_buffer_journal_identities
        .iter()
        .map(|journal| journal.journal_id.as_str())
        .chain(checkpoint.recovery_journal_refs.iter().map(String::as_str))
        .collect::<HashSet<_>>();
    let root_refs = checkpoint
        .trusted_root_refs
        .iter()
        .flat_map(|root| [root.root_id.as_str(), root.scope_ref.as_str()])
        .collect::<HashSet<_>>();
    let workset_ids = checkpoint
        .active_workset_ids
        .iter()
        .map(String::as_str)
        .collect::<HashSet<_>>();
    checkpoint.downgrade_triggers.iter().all(|trigger| {
        trigger.affected_journal_ids.as_ref().map_or(true, |refs| {
            refs.iter()
                .all(|value| journal_ids.contains(value.as_str()))
        }) && trigger.affected_root_refs.as_ref().map_or(true, |refs| {
            refs.iter().all(|value| root_refs.contains(value.as_str()))
        }) && trigger.affected_workset_ids.as_ref().map_or(true, |refs| {
            refs.iter()
                .all(|value| workset_ids.contains(value.as_str()))
        })
    })
}

fn downgrade_trigger_partitions_join(
    checkpoint: &WorkspaceAuthorityCheckpointRecord,
    snapshot: &WindowTopologySnapshotRecord,
) -> bool {
    checkpoint.downgrade_triggers.len() == snapshot.downgrade_triggers.len()
        && checkpoint
            .downgrade_triggers
            .iter()
            .zip(&snapshot.downgrade_triggers)
            .all(|(authority, topology)| {
                authority.trigger_class == topology.trigger_class && authority.note == topology.note
            })
}

fn topology_downgrade_trigger_is_valid(trigger: &DowngradeTriggerRecord) -> bool {
    trigger.affected_journal_ids.is_none()
        && trigger.affected_root_refs.is_none()
        && trigger.affected_workset_ids.is_none()
        && ref_list_is_valid(trigger.affected_pane_ids.as_ref())
        && optional_capture_text_is_valid(trigger.note.as_deref(), MAX_CAPTURE_NOTE_BYTES, true)
}

fn capture_downgrade_trigger_is_valid(trigger: &DowngradeTriggerRecord) -> bool {
    ref_list_is_valid(trigger.affected_journal_ids.as_ref())
        && ref_list_is_valid(trigger.affected_root_refs.as_ref())
        && ref_list_is_valid(trigger.affected_workset_ids.as_ref())
        && ref_list_is_valid(trigger.affected_pane_ids.as_ref())
        && optional_capture_text_is_valid(trigger.note.as_deref(), MAX_CAPTURE_NOTE_BYTES, true)
}

fn stable_pane_inventory_is_valid(panes: &[StablePaneInventoryEntry]) -> bool {
    no_duplicates_by(panes, |pane| pane.pane_id.as_str())
        && panes.iter().all(|pane| {
            is_bounded_opaque_ref(&pane.pane_id)
                && pane.restore_metadata.is_none()
                && optional_capture_text_is_valid(
                    pane.title_hint.as_deref(),
                    MAX_CAPTURE_LABEL_BYTES,
                    false,
                )
                && matches!(
                    (pane.hydration_behavior, pane.availability_state),
                    (
                        HydrationBehavior::EagerLightweight,
                        AvailabilityState::Ready
                    ) | (
                        HydrationBehavior::LazyHeavy,
                        AvailabilityState::NeedsHydration
                    ) | (
                        HydrationBehavior::PlaceholderOnly,
                        AvailabilityState::Placeholder
                    ) | (
                        HydrationBehavior::EvidenceOnly,
                        AvailabilityState::EvidenceOnly
                    )
                )
        })
}

fn tab_group_inventory_is_valid(groups: &[TabGroupInventoryEntry]) -> bool {
    !groups.is_empty()
        && no_duplicates_by(groups, |group| group.group_id.as_str())
        && groups.iter().all(|group| {
            is_bounded_opaque_ref(&group.group_id)
                && !group.ordered_tab_ids.is_empty()
                && group.ordered_tab_ids.len() <= MAX_CAPTURE_TABS_PER_GROUP
                && bounded_unique_refs(&group.ordered_tab_ids)
                && group.ordered_tab_ids.contains(&group.active_tab_id)
                && group.pinned_tab_ids.as_ref().map_or(true, |pinned| {
                    pinned.len() <= group.ordered_tab_ids.len()
                        && bounded_unique_refs(pinned)
                        && pinned
                            .iter()
                            .all(|tab_id| group.ordered_tab_ids.contains(tab_id))
                })
        })
}

fn inspector_inventory_is_valid(
    inspectors: &[super::records::VisibleInspectorInventoryEntry],
) -> bool {
    no_duplicates_by(inspectors, |inspector| inspector.inspector_id.as_str())
        && inspectors.iter().all(|inspector| {
            is_bounded_opaque_ref(&inspector.inspector_id)
                && inspector
                    .target_pane_ref
                    .as_deref()
                    .map_or(true, is_bounded_opaque_ref)
        })
}

fn focus_chain_is_valid(focus_chain: &[FocusChainEntry]) -> bool {
    focus_chain.iter().all(|entry| {
        is_bounded_opaque_ref(&entry.target_ref)
            && optional_capture_text_is_valid(entry.note.as_deref(), MAX_CAPTURE_NOTE_BYTES, true)
    })
}

fn follow_state_is_valid(state: &FollowPresentationState) -> bool {
    state
        .presenter_participant_ref
        .as_deref()
        .map_or(true, is_bounded_opaque_ref)
        && state.visible_role_badges.len() <= 16
        && no_duplicates(&state.visible_role_badges)
}

fn monitor_hint_is_valid(hint: &MonitorAffinityHint) -> bool {
    hint.best_effort_only
        && hint
            .last_known_display_ref
            .as_deref()
            .map_or(true, is_bounded_opaque_ref)
        && hint
            .last_known_topology_hash
            .as_deref()
            .map_or(true, is_bounded_opaque_ref)
        && hint
            .preferred_bounds_hint
            .map_or(true, |bounds| bounds.width > 0 && bounds.height > 0)
}

fn placeholder_inventory_is_valid(placeholders: &[PlaceholderBehaviorRecord]) -> bool {
    no_duplicates_by(placeholders, |placeholder| placeholder.pane_id.as_str())
        && placeholders.iter().all(|placeholder| {
            is_bounded_opaque_ref(&placeholder.pane_id)
                && placeholder.safe_actions.as_slice()
                    == required_placeholder_actions(placeholder.placeholder_reason)
                && optional_capture_text_is_valid(
                    placeholder.last_known_provenance_label.as_deref(),
                    MAX_CAPTURE_LABEL_BYTES,
                    false,
                )
                && optional_capture_text_is_valid(
                    placeholder.note.as_deref(),
                    MAX_CAPTURE_NOTE_BYTES,
                    true,
                )
        })
}

fn topology_adjustments_are_valid(
    adjustments: &[super::records::TopologyAdjustmentRecord],
) -> bool {
    adjustments.iter().all(|adjustment| {
        ref_list_is_valid(adjustment.affected_pane_ids.as_ref())
            && optional_capture_text_is_valid(
                adjustment.note.as_deref(),
                MAX_CAPTURE_NOTE_BYTES,
                true,
            )
    })
}

fn inspectors_join(
    inventory: &[super::records::VisibleInspectorInventoryEntry],
    body: &[super::records::VisibleInspectorRecord],
) -> bool {
    inventory.len() == body.len()
        && inventory.iter().zip(body).all(|(left, right)| {
            left.inspector_id == right.inspector_id
                && left.inspector_kind == right.inspector_kind
                && left.target_pane_ref == right.target_pane_ref
                && left.dock_position == right.dock_position
                && left.visible == right.visible
        })
}

fn restore_class_is_no_broader_than(topology: RestoreClass, authority: RestoreClass) -> bool {
    restore_class_fidelity(topology) <= restore_class_fidelity(authority)
}

fn restore_class_fidelity(class: RestoreClass) -> u8 {
    match class {
        RestoreClass::ExactRestore => 5,
        RestoreClass::CompatibleRestore => 4,
        RestoreClass::RecoveredDrafts => 3,
        RestoreClass::LayoutOnly => 2,
        RestoreClass::EvidenceOnly => 1,
        RestoreClass::NoRestore => 0,
    }
}

#[derive(Debug, Default)]
struct PaneTreeSemantics {
    surfaces: HashMap<String, PaneSurfaceDescriptor>,
    groups: HashMap<String, (Vec<String>, String, Vec<String>)>,
    structural_ids: HashSet<String>,
    visited_nodes: usize,
}

fn joined_topology_semantics_are_valid(
    snapshot: &WindowTopologySnapshotRecord,
    body: &WindowTopologySnapshotBodyRecord,
) -> bool {
    let mut semantics = PaneTreeSemantics::default();
    if collect_pane_tree_semantics(&body.pane_tree.root_node, 1, &mut semantics).is_err()
        || semantics.surfaces.len() != snapshot.stable_pane_id_inventory.len()
        || semantics.groups.len() != snapshot.tab_group_topology.len()
    {
        return false;
    }

    let mut inventory_ids = HashSet::new();
    let mut placeholder_pane_ids = HashSet::new();
    let mut unavailable_pane_ids = HashSet::new();
    for pane in &snapshot.stable_pane_id_inventory {
        if !is_bounded_opaque_ref(&pane.pane_id) || !inventory_ids.insert(pane.pane_id.as_str()) {
            return false;
        }
        let Some(surface) = semantics.surfaces.get(&pane.pane_id) else {
            return false;
        };
        if surface.surface_role != pane.surface_role
            || surface.surface_class != pane.surface_class
            || surface.hydration_behavior != pane.hydration_behavior
            || surface.availability_state != pane.availability_state
            || surface.title_hint != pane.title_hint
            || surface.follow_anchor_candidate != pane.follow_anchor_candidate
            || surface.presentation_spotlighted != pane.presentation_spotlighted
            || surface.restore_metadata.is_some()
            || pane.restore_metadata.is_some()
            || surface
                .surface_binding_ref
                .as_ref()
                .is_some_and(|binding| !is_bounded_opaque_ref(binding))
            || (is_side_effectful_capture_surface(pane.surface_role, pane.surface_class)
                && surface.surface_binding_ref.is_some())
        {
            return false;
        }

        let unavailable = matches!(
            pane.availability_state,
            AvailabilityState::Placeholder | AvailabilityState::EvidenceOnly
        ) || matches!(
            pane.hydration_behavior,
            HydrationBehavior::PlaceholderOnly | HydrationBehavior::EvidenceOnly
        );
        if unavailable {
            unavailable_pane_ids.insert(pane.pane_id.as_str());
            if surface.placeholder_card.is_none() {
                return false;
            }
        } else if surface.placeholder_card.is_some() {
            return false;
        }
    }

    for behavior in &snapshot.placeholder_behaviors {
        if !inventory_ids.contains(behavior.pane_id.as_str())
            || !placeholder_pane_ids.insert(behavior.pane_id.as_str())
            || behavior.safe_actions.as_slice()
                != required_placeholder_actions(behavior.placeholder_reason)
        {
            return false;
        }
        let Some(surface) = semantics.surfaces.get(&behavior.pane_id) else {
            return false;
        };
        let Some(card) = surface.placeholder_card.as_ref() else {
            return false;
        };
        if card.placeholder_reason != behavior.placeholder_reason
            || card.safe_actions != behavior.safe_actions
            || card.evidence_retained != behavior.evidence_retained
            || card.last_known_provenance_label != behavior.last_known_provenance_label
        {
            return false;
        }
    }
    if placeholder_pane_ids != unavailable_pane_ids {
        return false;
    }

    let mut topology_group_ids = HashSet::new();
    let mut topology_tab_ids = HashSet::new();
    for group in &snapshot.tab_group_topology {
        if !is_bounded_opaque_ref(&group.group_id)
            || !topology_group_ids.insert(group.group_id.as_str())
        {
            return false;
        }
        let Some((tab_ids, active_tab_id, pinned_tab_ids)) = semantics.groups.get(&group.group_id)
        else {
            return false;
        };
        if &group.ordered_tab_ids != tab_ids
            || &group.active_tab_id != active_tab_id
            || group.pinned_tab_ids.as_deref().unwrap_or_default() != pinned_tab_ids.as_slice()
        {
            return false;
        }
        topology_tab_ids.extend(group.ordered_tab_ids.iter().map(String::as_str));
    }
    let inspector_ids = snapshot
        .visible_inspectors
        .iter()
        .map(|inspector| inspector.inspector_id.as_str())
        .collect::<HashSet<_>>();
    if snapshot.visible_inspectors.iter().any(|inspector| {
        inspector
            .target_pane_ref
            .as_deref()
            .is_some_and(|pane_id| !inventory_ids.contains(pane_id))
    }) || snapshot.topology_adjustments.iter().any(|adjustment| {
        adjustment
            .affected_pane_ids
            .as_ref()
            .is_some_and(|pane_ids| {
                pane_ids
                    .iter()
                    .any(|pane_id| !inventory_ids.contains(pane_id.as_str()))
            })
    }) || snapshot.downgrade_triggers.iter().any(|trigger| {
        trigger.affected_pane_ids.as_ref().is_some_and(|pane_ids| {
            pane_ids
                .iter()
                .any(|pane_id| !inventory_ids.contains(pane_id.as_str()))
        })
    }) || snapshot
        .focus_chain
        .iter()
        .any(|entry| match entry.target_kind {
            FocusTargetKind::Pane => !inventory_ids.contains(entry.target_ref.as_str()),
            FocusTargetKind::Tab => !topology_tab_ids.contains(entry.target_ref.as_str()),
            FocusTargetKind::Inspector => !inspector_ids.contains(entry.target_ref.as_str()),
            FocusTargetKind::FollowBanner | FocusTargetKind::WindowChrome => false,
        })
    {
        return false;
    }
    true
}

fn collect_pane_tree_semantics(
    node: &PaneNode,
    depth: usize,
    semantics: &mut PaneTreeSemantics,
) -> Result<(), ()> {
    semantics.visited_nodes = semantics.visited_nodes.saturating_add(1);
    if depth > MAX_CAPTURE_LAYOUT_DEPTH
        || semantics.visited_nodes > MAX_CAPTURE_LAYOUT_NODES + MAX_CAPTURE_TOTAL_TABS
    {
        return Err(());
    }

    match node {
        PaneNode::Leaf { pane_id, surface } => {
            insert_semantic_surface(pane_id, surface, semantics)?;
        }
        PaneNode::Split {
            split_id,
            children,
            weights,
            ..
        } => {
            if !is_bounded_opaque_ref(split_id)
                || !semantics.structural_ids.insert(split_id.clone())
                || children.len() < 2
                || children.len() > MAX_CAPTURE_LAYOUT_NODES
                || weights.as_ref().is_some_and(|weights| {
                    weights.len() != children.len()
                        || weights
                            .iter()
                            .any(|weight| !weight.is_finite() || *weight <= 0.0)
                })
            {
                return Err(());
            }
            for child in children {
                collect_pane_tree_semantics(child, depth + 1, semantics)?;
            }
        }
        PaneNode::TabGroup {
            group_id,
            tabs,
            active_tab_id,
            ..
        } => {
            if !is_bounded_opaque_ref(group_id)
                || !semantics.structural_ids.insert(group_id.clone())
                || tabs.is_empty()
                || tabs.len() > MAX_CAPTURE_TABS_PER_GROUP
            {
                return Err(());
            }
            let mut tab_ids = Vec::with_capacity(tabs.len());
            let mut seen_tab_ids = HashSet::new();
            let mut pinned_tab_ids = Vec::new();
            for tab in tabs {
                if !is_bounded_opaque_ref(&tab.tab_id)
                    || !seen_tab_ids.insert(tab.tab_id.as_str())
                    || tab.pane.node_kind != "leaf"
                {
                    return Err(());
                }
                tab_ids.push(tab.tab_id.clone());
                if tab.pinned == Some(true) {
                    pinned_tab_ids.push(tab.tab_id.clone());
                }
                insert_semantic_surface(&tab.pane.pane_id, &tab.pane.surface, semantics)?;
            }
            if !seen_tab_ids.contains(active_tab_id.as_str())
                || semantics
                    .groups
                    .insert(
                        group_id.clone(),
                        (tab_ids, active_tab_id.clone(), pinned_tab_ids),
                    )
                    .is_some()
            {
                return Err(());
            }
        }
    }
    Ok(())
}

fn insert_semantic_surface(
    pane_id: &str,
    surface: &PaneSurfaceDescriptor,
    semantics: &mut PaneTreeSemantics,
) -> Result<(), ()> {
    if !is_bounded_opaque_ref(pane_id)
        || semantics
            .surfaces
            .insert(pane_id.to_string(), surface.clone())
            .is_some()
    {
        return Err(());
    }
    Ok(())
}

fn required_placeholder_actions(reason: PlaceholderReasonClass) -> &'static [PlaceholderAction] {
    use PlaceholderAction::*;
    match reason {
        PlaceholderReasonClass::MissingExtension => &[
            LocateExtension,
            InstallExtension,
            OpenWithout,
            ExportEvidence,
            RemovePane,
        ],
        PlaceholderReasonClass::MissingRemote => {
            &[ReconnectRemote, Reauthenticate, ExportEvidence, RemovePane]
        }
        PlaceholderReasonClass::MissingRemoteAuthority
        | PlaceholderReasonClass::RevokedPermission => {
            &[Reauthenticate, OpenRestricted, ExportEvidence]
        }
        PlaceholderReasonClass::UnsupportedDisplayTopology => &[ReflowToSafeBounds],
        PlaceholderReasonClass::NonReentrantLiveSurface => &[
            RerunExplicitly,
            RebindExistingSession,
            ExportEvidence,
            RemovePane,
        ],
        PlaceholderReasonClass::SchemaMigrationReviewRequired => &[
            CompareWithPreservedArtifact,
            OpenRepairInstructions,
            ExportEvidence,
        ],
        PlaceholderReasonClass::ManualRecoveryRequired => &[
            OpenRepairInstructions,
            EscalateToManualRepair,
            ExportEvidence,
        ],
    }
}

fn valid_latest_index_record(record: &LatestIndexRecord, refs: &SessionRestoreLatestRefs) -> bool {
    record.record_kind == "session_restore_latest_index"
        && record.latest_index_schema_version == 1
        && !record.emitted_at.is_empty()
        && parse_durable_record_id(&refs.checkpoint_id, "ckpt").is_ok()
        && parse_durable_record_id(&refs.snapshot_id, "snap").is_ok()
}

fn next_durable_sequence(dir: &Path, prefix: &str) -> Result<u64, SessionRestoreError> {
    let entries = match bounded_directory_entries(dir) {
        Ok(entries) => entries,
        Err(SessionRestoreError::Io(err)) if err.kind() == io::ErrorKind::NotFound => return Ok(1),
        Err(err) => return Err(err),
    };
    let mut max_sequence = 0_u64;
    for entry in entries {
        if !entry_is_direct_regular_file(&entry)?
            || entry.path().extension().and_then(|ext| ext.to_str()) != Some("json")
        {
            continue;
        }
        let file_name = entry.file_name();
        let file_name = file_name.to_str().ok_or(SessionRestoreError::CorruptStore(
            "durable record filename is not UTF-8",
        ))?;
        let record_id =
            file_name
                .strip_suffix(".json")
                .ok_or(SessionRestoreError::CorruptStore(
                    "durable record filename has no JSON suffix",
                ))?;
        let (sequence, _) = parse_durable_record_id(record_id, prefix)?;
        max_sequence = max_sequence.max(sequence);
    }
    max_sequence
        .checked_add(1)
        .ok_or(SessionRestoreError::CorruptStore(
            "durable id sequence exhausted",
        ))
}

fn parse_durable_record_id(
    record_id: &str,
    expected_prefix: &str,
) -> Result<(u64, u128), SessionRestoreError> {
    if record_id.len() > 128 {
        return Err(SessionRestoreError::CorruptStore(
            "durable record id is oversized",
        ));
    }
    let mut parts = record_id.split('-');
    let prefix = parts.next();
    let stamp = parts.next();
    let sequence = parts.next();
    if prefix != Some(expected_prefix)
        || parts.next().is_some()
        || stamp.map_or(true, |value| {
            value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit())
        })
        || sequence.map_or(true, |value| {
            value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit())
        })
    {
        return Err(SessionRestoreError::CorruptStore(
            "durable record id has an invalid shape",
        ));
    }
    let stamp = stamp.and_then(|value| value.parse::<u128>().ok()).ok_or(
        SessionRestoreError::CorruptStore("durable record timestamp is invalid"),
    )?;
    let sequence = sequence.and_then(|value| value.parse::<u64>().ok()).ok_or(
        SessionRestoreError::CorruptStore("durable record sequence is invalid"),
    )?;
    // Durable sequence is the primary order. Wall-clock timestamps are only
    // a collision-resistant tie breaker because system time may move back.
    Ok((sequence, stamp))
}

fn validate_requested_durable_id(
    record_id: &str,
    prefix: &str,
    record_class: &'static str,
) -> Result<(), SessionRestoreError> {
    parse_durable_record_id(record_id, prefix)
        .map(|_| ())
        .map_err(|_| SessionRestoreError::MissingRecord(format!("{record_class} id is invalid")))
}

fn checkpoint_downgrade_trigger(trigger: &DowngradeTriggerRecord) -> DowngradeTriggerRecord {
    DowngradeTriggerRecord {
        trigger_class: trigger.trigger_class,
        affected_journal_ids: trigger.affected_journal_ids.clone(),
        affected_root_refs: trigger.affected_root_refs.clone(),
        affected_workset_ids: trigger.affected_workset_ids.clone(),
        // Pane scope belongs to the window-local topology packet. The
        // authority checkpoint keeps the trigger class without duplicating
        // window topology.
        affected_pane_ids: None,
        note: trigger.note.clone(),
    }
}

fn topology_downgrade_trigger(trigger: &DowngradeTriggerRecord) -> DowngradeTriggerRecord {
    DowngradeTriggerRecord {
        trigger_class: trigger.trigger_class,
        // Authority-level journal/root/workset scopes remain in the joined
        // checkpoint and are not duplicated into the window-local packet.
        affected_journal_ids: None,
        affected_root_refs: None,
        affected_workset_ids: None,
        affected_pane_ids: trigger.affected_pane_ids.clone(),
        note: trigger.note.clone(),
    }
}

const MAX_CAPTURE_GROUPS: usize = 64;
const MAX_CAPTURE_TABS_PER_GROUP: usize = 256;
const MAX_CAPTURE_TOTAL_TABS: usize = 1_024;
const MAX_CAPTURE_REF_LIST_ITEMS: usize = 256;
const MAX_CAPTURE_DOWNGRADE_TRIGGERS: usize = 256;
const MAX_CAPTURE_LAYOUT_DEPTH: usize = 32;
const MAX_CAPTURE_LAYOUT_NODES: usize = 256;
const MAX_CAPTURE_LABEL_BYTES: usize = 1_024;
// The boundary schemas cap every redaction-aware text field at 1,024.
const MAX_CAPTURE_NOTE_BYTES: usize = 1_024;

fn validate_capture_input(input: &SessionRestoreCaptureInput) -> Result<(), SessionRestoreError> {
    if input.tab_groups.is_empty() || input.tab_groups.len() > MAX_CAPTURE_GROUPS {
        return Err(SessionRestoreError::InvalidCapture(
            "capture tab-group cardinality is out of bounds",
        ));
    }
    if input.trusted_root_refs.len() > MAX_CAPTURE_REF_LIST_ITEMS
        || input.dirty_buffer_journal_identities.len() > MAX_CAPTURE_REF_LIST_ITEMS
        || input.downgrade_triggers.len() > MAX_CAPTURE_DOWNGRADE_TRIGGERS
        || input.excluded_live_authority_classes.len() > MAX_CAPTURE_REF_LIST_ITEMS
        || [
            &input.active_workset_ids,
            &input.recovery_journal_refs,
            &input.local_history_snapshot_refs,
            &input.evidence_bundle_refs,
            &input.sibling_window_refs,
        ]
        .into_iter()
        .any(|refs| refs.len() > MAX_CAPTURE_REF_LIST_ITEMS)
    {
        return Err(SessionRestoreError::InvalidCapture(
            "capture reference cardinality is out of bounds",
        ));
    }
    if !producer_build_is_valid(&input.producer_build)
        || !is_bounded_capture_text(&input.source_schema_version, 128, false)
        || !is_bounded_capture_text(&input.emitted_at, 128, false)
        || input
            .notes
            .as_ref()
            .is_some_and(|note| !is_bounded_capture_text(note, MAX_CAPTURE_NOTE_BYTES, true))
    {
        return Err(SessionRestoreError::InvalidCapture(
            "capture text exceeds its bounded redaction-aware envelope",
        ));
    }
    if !is_bounded_opaque_ref(&input.workspace_ref)
        || !is_bounded_opaque_ref(&format!("workspace-authority:{}", input.workspace_ref))
        || !is_bounded_opaque_ref(&input.window_id)
    {
        return Err(SessionRestoreError::InvalidCapture(
            "workspace, derived authority, and window refs must be bounded opaque ids",
        ));
    }
    if input
        .topology_family_ref
        .as_deref()
        .map_or(false, |value| !is_bounded_opaque_ref(value))
        || !bounded_unique_refs(&input.sibling_window_refs)
    {
        return Err(SessionRestoreError::InvalidCapture(
            "window topology refs must be bounded opaque ids",
        ));
    }
    if !trusted_roots_are_valid(&input.trusted_root_refs) {
        return Err(SessionRestoreError::InvalidCapture(
            "trusted-root records must be unique and schema-valid",
        ));
    }
    if [
        &input.active_workset_ids,
        &input.recovery_journal_refs,
        &input.local_history_snapshot_refs,
        &input.evidence_bundle_refs,
    ]
    .into_iter()
    .any(|refs| !bounded_unique_refs(refs))
    {
        return Err(SessionRestoreError::InvalidCapture(
            "checkpoint refs must be unique bounded opaque ids",
        ));
    }
    if !dirty_journals_are_valid(&input.dirty_buffer_journal_identities) {
        return Err(SessionRestoreError::InvalidCapture(
            "dirty-journal records must be unique and schema-valid",
        ));
    }
    if !no_duplicates(&input.excluded_live_authority_classes) {
        return Err(SessionRestoreError::InvalidCapture(
            "excluded live-authority classes must be unique",
        ));
    }
    if input
        .downgrade_triggers
        .iter()
        .any(|trigger| !capture_downgrade_trigger_is_valid(trigger))
    {
        return Err(SessionRestoreError::InvalidCapture(
            "downgrade trigger scopes must be unique bounded opaque ids",
        ));
    }
    let mut group_ids = HashSet::new();
    let mut tab_ids = HashSet::new();
    let mut total_tabs = 0_usize;
    for group in &input.tab_groups {
        if !is_bounded_opaque_ref(&group.group_id) || !group_ids.insert(group.group_id.as_str()) {
            return Err(SessionRestoreError::InvalidCapture(
                "tab group ids must be present and unique",
            ));
        }
        if group.ordered_tabs.is_empty() || group.ordered_tabs.len() > MAX_CAPTURE_TABS_PER_GROUP {
            return Err(SessionRestoreError::InvalidCapture(
                "captured tab-group size is out of bounds",
            ));
        }
        total_tabs = total_tabs.saturating_add(group.ordered_tabs.len());
        if total_tabs > MAX_CAPTURE_TOTAL_TABS {
            return Err(SessionRestoreError::InvalidCapture(
                "captured tab count is out of bounds",
            ));
        }
        for tab in &group.ordered_tabs {
            if !is_bounded_opaque_ref(&tab.tab_id)
                || !is_bounded_opaque_ref(&format!("pane:{}", tab.tab_id))
                || !tab_ids.insert(tab.tab_id.as_str())
            {
                return Err(SessionRestoreError::InvalidCapture(
                    "tab and derived pane ids must be bounded and unique",
                ));
            }
            if tab.tab_label.as_ref().is_some_and(|label| {
                !is_bounded_capture_text(label, MAX_CAPTURE_LABEL_BYTES, false)
            }) {
                return Err(SessionRestoreError::InvalidCapture(
                    "tab labels must remain bounded redaction-aware text",
                ));
            }
            if tab
                .surface_binding_ref
                .as_ref()
                .is_some_and(|binding| !is_bounded_opaque_ref(binding))
            {
                return Err(SessionRestoreError::InvalidCapture(
                    "surface binding refs must be bounded opaque ids",
                ));
            }
            if tab.surface_binding_ref.is_some()
                && is_side_effectful_capture_surface(tab.surface_role, tab.surface_class)
            {
                return Err(SessionRestoreError::InvalidCapture(
                    "side-effectful surfaces must not carry restore bindings",
                ));
            }
            if let Some(metadata) = tab.restore_metadata.as_ref() {
                if !matches!(tab.surface_role, SurfaceRole::Terminal)
                    || !matches!(tab.surface_class, SurfaceClass::TerminalView)
                    || !is_bounded_opaque_ref(&metadata.restore_metadata_ref)
                    || metadata.working_directory.as_ref().is_some_and(|value| {
                        !is_bounded_capture_text(value, MAX_CAPTURE_LABEL_BYTES, false)
                    })
                    || !is_bounded_capture_text(&metadata.environment_scope_token, 128, false)
                    || !is_bounded_capture_text(&metadata.shell_identity, 256, false)
                    || !is_bounded_capture_text(&metadata.shell_family_token, 128, false)
                    || !is_bounded_capture_text(&metadata.last_command_class_token, 128, false)
                    || !metadata.auto_rerun_forbidden
                    || metadata.raw_command_body_present
                    || metadata.raw_environment_body_present
                {
                    return Err(SessionRestoreError::InvalidCapture(
                        "terminal metadata must exclude payloads and forbid automatic rerun",
                    ));
                }
            }
        }
        if group
            .active_tab_id
            .as_ref()
            .is_some_and(|active| !group.ordered_tabs.iter().any(|tab| tab.tab_id == *active))
        {
            return Err(SessionRestoreError::InvalidCapture(
                "active tab must belong to its captured group",
            ));
        }
    }

    let journal_ids = input
        .dirty_buffer_journal_identities
        .iter()
        .map(|journal| journal.journal_id.as_str())
        .chain(input.recovery_journal_refs.iter().map(String::as_str))
        .collect::<HashSet<_>>();
    let root_refs = input
        .trusted_root_refs
        .iter()
        .flat_map(|root| [root.root_id.as_str(), root.scope_ref.as_str()])
        .collect::<HashSet<_>>();
    let workset_ids = input
        .active_workset_ids
        .iter()
        .map(String::as_str)
        .collect::<HashSet<_>>();
    if input.downgrade_triggers.iter().any(|trigger| {
        trigger.affected_journal_ids.as_ref().is_some_and(|refs| {
            refs.iter()
                .any(|value| !journal_ids.contains(value.as_str()))
        }) || trigger
            .affected_root_refs
            .as_ref()
            .is_some_and(|refs| refs.iter().any(|value| !root_refs.contains(value.as_str())))
            || trigger.affected_workset_ids.as_ref().is_some_and(|refs| {
                refs.iter()
                    .any(|value| !workset_ids.contains(value.as_str()))
            })
            || trigger.affected_pane_ids.as_ref().is_some_and(|refs| {
                refs.iter().any(|value| {
                    value
                        .strip_prefix("pane:")
                        .map_or(true, |tab_id| !tab_ids.contains(tab_id))
                })
            })
    }) {
        return Err(SessionRestoreError::InvalidCapture(
            "downgrade trigger scopes must join the captured authority and topology",
        ));
    }

    if input
        .focused_group_id
        .as_ref()
        .is_some_and(|focused| !group_ids.contains(focused.as_str()))
    {
        return Err(SessionRestoreError::InvalidCapture(
            "focused group must belong to the captured topology",
        ));
    }

    match input.pane_tree_layout.as_ref() {
        Some(layout) => {
            let mut split_ids = HashSet::new();
            let mut layout_group_ids = Vec::new();
            let mut layout_node_count = 0;
            validate_group_layout(
                layout,
                1,
                &mut layout_node_count,
                &mut split_ids,
                &mut layout_group_ids,
            )?;
            let captured_group_ids: Vec<_> = input
                .tab_groups
                .iter()
                .map(|group| group.group_id.clone())
                .collect();
            if layout_group_ids != captured_group_ids {
                return Err(SessionRestoreError::InvalidCapture(
                    "pane-tree leaves must match captured groups exactly and in order",
                ));
            }
            if split_ids
                .iter()
                .any(|split_id| group_ids.contains(split_id.as_str()))
            {
                return Err(SessionRestoreError::InvalidCapture(
                    "split and tab-group ids must not collide",
                ));
            }
        }
        None if input.tab_groups.len() > 1 => {
            return Err(SessionRestoreError::InvalidCapture(
                "multi-group captures require structural pane-tree layout",
            ));
        }
        None => {}
    }

    Ok(())
}

const MAX_CAPTURE_OPAQUE_REF_LEN: usize = 512;

/// Applies the recovery schemas' opaque-id boundary before any record is
/// minted or written. The closed ASCII grammar intentionally excludes path
/// separators, URI syntax, whitespace/control bytes, and payload-like text.
pub(super) fn is_bounded_opaque_ref(value: &str) -> bool {
    !value.is_empty()
        && value != "."
        && value != ".."
        && value.len() <= MAX_CAPTURE_OPAQUE_REF_LEN
        && value.chars().all(|ch| {
            ch.is_ascii_alphanumeric() || matches!(ch, ':' | '.' | '_' | '-' | '#' | '@' | '|')
        })
}

fn is_side_effectful_capture_surface(role: SurfaceRole, class: SurfaceClass) -> bool {
    matches!(
        role,
        SurfaceRole::Terminal
            | SurfaceRole::Debugger
            | SurfaceRole::Notebook
            | SurfaceRole::AiPanel
            | SurfaceRole::Test
    ) || matches!(
        class,
        SurfaceClass::TerminalView
            | SurfaceClass::DebugView
            | SurfaceClass::NotebookView
            | SurfaceClass::AiPanel
            | SurfaceClass::TestResults
    )
}

fn is_bounded_capture_text(value: &str, max_bytes: usize, multiline: bool) -> bool {
    !value.is_empty()
        && value.len() <= max_bytes
        && value
            .chars()
            .all(|ch| !ch.is_control() || (multiline && matches!(ch, '\n' | '\r' | '\t')))
}

fn validate_group_layout(
    node: &TabGroupLayoutCapture,
    depth: usize,
    node_count: &mut usize,
    split_ids: &mut HashSet<String>,
    group_ids: &mut Vec<String>,
) -> Result<(), SessionRestoreError> {
    *node_count = node_count.saturating_add(1);
    if depth > MAX_CAPTURE_LAYOUT_DEPTH || *node_count > MAX_CAPTURE_LAYOUT_NODES {
        return Err(SessionRestoreError::InvalidCapture(
            "pane-tree depth or node count exceeds the capture bound",
        ));
    }
    match node {
        TabGroupLayoutCapture::TabGroup { group_id } => {
            if !is_bounded_opaque_ref(group_id) {
                return Err(SessionRestoreError::InvalidCapture(
                    "pane-tree group refs must be bounded opaque ids",
                ));
            }
            group_ids.push(group_id.clone());
        }
        TabGroupLayoutCapture::Split {
            split_id,
            children,
            weights,
            ..
        } => {
            if !is_bounded_opaque_ref(split_id) || !split_ids.insert(split_id.clone()) {
                return Err(SessionRestoreError::InvalidCapture(
                    "split ids must be bounded and unique",
                ));
            }
            if children.len() < 2 || children.len() > MAX_CAPTURE_GROUPS {
                return Err(SessionRestoreError::InvalidCapture(
                    "split child cardinality is out of bounds",
                ));
            }
            if weights.as_ref().is_some_and(|weights| {
                weights.len() != children.len()
                    || weights
                        .iter()
                        .any(|weight| !weight.is_finite() || *weight <= 0.0)
            }) {
                return Err(SessionRestoreError::InvalidCapture(
                    "split weights must be finite, positive, and match child count",
                ));
            }
            for child in children {
                validate_group_layout(child, depth + 1, node_count, split_ids, group_ids)?;
            }
        }
    }
    Ok(())
}

const MAX_RECOVERY_RECORD_BYTES: u64 = 4 * 1024 * 1024;
const MAX_RECOVERY_DIRECTORY_ENTRIES: usize = 4_096;
const MAX_RECOVERY_TEMP_ATTEMPTS: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PublicationOutcome {
    Durable,
    CommitStateUncertain,
}

fn commit_state_uncertain(refs: &SessionRestoreLatestRefs) -> SessionRestoreError {
    SessionRestoreError::CommitStateUncertain(refs.clone())
}

fn require_capture_publication(
    result: Result<PublicationOutcome, SessionRestoreError>,
    refs: &SessionRestoreLatestRefs,
) -> Result<(), SessionRestoreError> {
    match result {
        Ok(PublicationOutcome::Durable) => Ok(()),
        Ok(PublicationOutcome::CommitStateUncertain) | Err(_) => Err(commit_state_uncertain(refs)),
    }
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, SessionRestoreError> {
    read_json_with_post_read_hook(path, |_| {})
}

fn read_json_with_post_read_hook<T, F>(
    path: &Path,
    post_read_hook: F,
) -> Result<T, SessionRestoreError>
where
    T: for<'de> Deserialize<'de>,
    F: FnOnce(&Path),
{
    let resolved = resolve_recovery_file_path(path, false)?;
    let parent = resolved.parent().ok_or(SessionRestoreError::CorruptStore(
        "recovery record path has no parent",
    ))?;
    let parent_identity = observed_directory_identity(parent)?;
    require_directory_identity(parent, parent_identity)?;

    let before = fs::symlink_metadata(&resolved)?;
    require_direct_regular_file(&before)?;
    require_record_size(&before)?;
    let before_identity = FileIdentity::from_metadata(&before);
    require_directory_identity(parent, parent_identity)?;

    let mut file = File::open(&resolved)?;
    let opened = file.metadata()?;
    require_direct_regular_file(&opened)?;
    require_record_size(&opened)?;
    let opened_identity = FileIdentity::from_metadata(&opened);
    if opened_identity != before_identity {
        return Err(path_integrity_error("recovery record identity changed while opening").into());
    }
    require_directory_identity(parent, parent_identity)?;

    let initial_capacity = usize::try_from(opened.len())
        .unwrap_or(usize::MAX)
        .min(64 * 1024);
    let mut bytes = Vec::with_capacity(initial_capacity);
    Read::by_ref(&mut file)
        .take(MAX_RECOVERY_RECORD_BYTES.saturating_add(1))
        .read_to_end(&mut bytes)?;
    if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > MAX_RECOVERY_RECORD_BYTES {
        return Err(SessionRestoreError::CorruptStore(
            "recovery record grew beyond the byte limit",
        ));
    }

    post_read_hook(&resolved);

    let descriptor_after = file.metadata()?;
    require_direct_regular_file(&descriptor_after)?;
    require_record_size(&descriptor_after)?;
    if FileIdentity::from_metadata(&descriptor_after) != opened_identity {
        return Err(path_integrity_error("recovery record identity changed while reading").into());
    }
    let path_after = fs::symlink_metadata(&resolved)?;
    require_direct_regular_file(&path_after)?;
    require_record_size(&path_after)?;
    if FileIdentity::from_metadata(&path_after) != opened_identity {
        return Err(path_integrity_error("recovery record path changed while reading").into());
    }
    require_directory_identity(parent, parent_identity)?;

    Ok(serde_json::from_slice(&bytes)?)
}

fn read_json_for_selection<T: for<'de> Deserialize<'de>>(
    path: &Path,
) -> Result<Option<T>, SessionRestoreError> {
    match read_json(path) {
        Ok(value) => Ok(Some(value)),
        Err(SessionRestoreError::Io(error)) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(SessionRestoreError::Json(_) | SessionRestoreError::CorruptStore(_)) => Ok(None),
        Err(error) => Err(error),
    }
}

fn require_record_size(metadata: &Metadata) -> Result<(), SessionRestoreError> {
    if metadata.len() > MAX_RECOVERY_RECORD_BYTES {
        return Err(SessionRestoreError::CorruptStore(
            "recovery record exceeds the byte limit",
        ));
    }
    Ok(())
}

fn bounded_directory_entries(path: &Path) -> Result<Vec<fs::DirEntry>, SessionRestoreError> {
    bounded_directory_entries_with_limit(path, MAX_RECOVERY_DIRECTORY_ENTRIES)
}

fn bounded_directory_entries_with_limit(
    path: &Path,
    max_entries: usize,
) -> Result<Vec<fs::DirEntry>, SessionRestoreError> {
    let resolved = resolve_recovery_directory(path, false)?;
    let identity = observed_directory_identity(&resolved)?;
    let entries = fs::read_dir(&resolved)?;
    let mut bounded = Vec::new();
    for entry in entries {
        if bounded.len() >= max_entries {
            return Err(SessionRestoreError::CorruptStore(
                "recovery directory exceeds the entry limit",
            ));
        }
        bounded.push(entry?);
    }
    require_directory_identity(&resolved, identity)?;
    Ok(bounded)
}

fn entry_is_direct_regular_file(entry: &fs::DirEntry) -> Result<bool, SessionRestoreError> {
    let metadata = fs::symlink_metadata(entry.path())?;
    if metadata_is_redirect(&metadata) {
        return Err(
            path_integrity_error("recovery directory entry must not be a path redirect").into(),
        );
    }
    Ok(metadata.is_file())
}

fn write_new_json<T: Serialize + ?Sized>(
    path: &Path,
    value: &T,
) -> Result<PublicationOutcome, SessionRestoreError> {
    write_new_json_with_hooks(path, value, |_| {}, |_| Ok(()))
}

fn write_new_json_with_hooks<T, BeforeInstall, AfterInstall>(
    path: &Path,
    value: &T,
    before_install: BeforeInstall,
    after_install: AfterInstall,
) -> Result<PublicationOutcome, SessionRestoreError>
where
    T: Serialize + ?Sized,
    BeforeInstall: FnOnce(&Path),
    AfterInstall: FnOnce(&Path) -> io::Result<()>,
{
    let bytes = to_bounded_json_pretty(value)?;
    let target = resolve_recovery_file_path(path, true)?;
    let parent = target.parent().ok_or(SessionRestoreError::CorruptStore(
        "recovery publication path has no parent",
    ))?;
    let parent_identity = observed_directory_identity(parent)?;
    let directory_sync_handle = open_directory_sync_handle(parent, parent_identity)?;
    require_directory_identity(parent, parent_identity)?;
    require_destination_absent(&target)?;
    require_directory_identity(parent, parent_identity)?;

    let mut pending = create_pending_publication(parent, parent_identity)?;
    let preinstall = (|| -> Result<FileIdentity, SessionRestoreError> {
        pending.file_mut().write_all(&bytes)?;
        pending.file_mut().flush()?;
        pending.file_mut().sync_all()?;
        let temporary_identity = pending.file_identity()?;
        verify_direct_file_identity(pending.path(), temporary_identity)?;
        require_directory_identity(parent, parent_identity)?;

        before_install(parent);

        require_directory_identity(parent, parent_identity)?;
        verify_direct_file_identity(pending.path(), temporary_identity)?;
        require_destination_absent(&target)?;
        require_directory_identity(parent, parent_identity)?;
        Ok(temporary_identity)
    })();
    let temporary_identity = match preinstall {
        Ok(identity) => identity,
        Err(error) => {
            pending.scrub_and_abandon();
            return Err(error);
        }
    };

    // Stable Rust 1.75 has no cross-platform directory-handle-relative link
    // primitive. The pinned parent is rechecked immediately before this
    // create-new operation; a swap wholly inside that final name-operation
    // window remains a platform boundary rather than an overwrite risk.
    let link_result = fs::hard_link(pending.path(), &target).and_then(|()| {
        recovery_io_failpoint(RecoveryIoFailpoint::HardLinkReportedErrorAfterInstall)
    });
    if let Err(link_error) = link_result {
        let target_state = direct_file_matches_object(&target, temporary_identity);
        // Some filesystems can report a link error after the destination
        // became visible. Never scrub through the open staging handle after
        // the link syscall: an installed alias would reference that same
        // inode and be truncated too. Unlink only the still-owned private
        // staging name, then classify the observed destination state.
        pending.abandon_after_link_attempt();
        return match target_state {
            Ok(true) | Err(_) => Ok(PublicationOutcome::CommitStateUncertain),
            Ok(false) => Err(link_error.into()),
        };
    }

    // The destination now names the synchronized staged file. Disarm before
    // every fallible post-install step: scrubbing this handle would also
    // truncate the committed destination.
    pending.disarm();
    if recovery_io_failpoint(RecoveryIoFailpoint::AfterHardLink).is_err()
        || after_install(parent).is_err()
    {
        // The target is already installed. Best-effort cleanup is authorized
        // only while the temporary pathname, open handle, and destination all
        // still identify the same file object.
        let _ = pending.remove_installed_alias_if_still_owned(&target, temporary_identity);
        return Ok(PublicationOutcome::CommitStateUncertain);
    }

    if require_directory_identity(parent, parent_identity).is_err()
        || verify_direct_file_object(pending.path(), temporary_identity).is_err()
        || verify_direct_file_object(&target, temporary_identity).is_err()
    {
        return Ok(PublicationOutcome::CommitStateUncertain);
    }
    if !matches!(
        pending.remove_installed_alias_if_still_owned(&target, temporary_identity),
        Ok(true)
    ) {
        return Ok(PublicationOutcome::CommitStateUncertain);
    }
    if recovery_io_failpoint(RecoveryIoFailpoint::BeforeDirectorySync).is_err()
        || sync_directory(directory_sync_handle.as_ref()).is_err()
        || require_directory_identity(parent, parent_identity).is_err()
        || verify_direct_file_object(&target, temporary_identity).is_err()
    {
        return Ok(PublicationOutcome::CommitStateUncertain);
    }

    Ok(PublicationOutcome::Durable)
}

#[cfg(test)]
fn write_new_json_atomically<T: Serialize + ?Sized>(
    path: &Path,
    value: &T,
) -> Result<PublicationOutcome, SessionRestoreError> {
    write_new_json(path, value)
}

fn to_bounded_json_pretty<T: Serialize + ?Sized>(
    value: &T,
) -> Result<Vec<u8>, SessionRestoreError> {
    let mut writer = BoundedVecWriter::new(MAX_RECOVERY_RECORD_BYTES as usize);
    serde_json::to_writer_pretty(&mut writer, value)?;
    Ok(writer.into_inner())
}

fn preflight_publication_body<T: Serialize + ?Sized>(value: &T) -> Result<(), SessionRestoreError> {
    to_bounded_json_pretty(value).map(|_| ())
}

struct BoundedVecWriter {
    bytes: Vec<u8>,
    max_bytes: usize,
}

impl BoundedVecWriter {
    fn new(max_bytes: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(max_bytes.min(64 * 1024)),
            max_bytes,
        }
    }

    fn into_inner(self) -> Vec<u8> {
        self.bytes
    }
}

impl Write for BoundedVecWriter {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        if buffer.len() > self.max_bytes.saturating_sub(self.bytes.len()) {
            return Err(path_integrity_error(
                "serialized recovery record exceeds the byte limit",
            ));
        }
        self.bytes.extend_from_slice(buffer);
        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn resolve_recovery_file_path(path: &Path, create_parent: bool) -> io::Result<PathBuf> {
    let file_name = path
        .file_name()
        .filter(|name| !name.is_empty())
        .ok_or_else(|| path_input_error("recovery record path has no file name"))?;
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let canonical_parent = resolve_recovery_directory(parent, create_parent)?;
    Ok(canonical_parent.join(file_name))
}

fn resolve_recovery_directory(path: &Path, create_missing: bool) -> io::Result<PathBuf> {
    let absolute = if path.is_absolute() {
        path.to_owned()
    } else {
        std::env::current_dir()?.join(path)
    };
    if absolute
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(path_input_error(
            "recovery paths must not contain parent traversal",
        ));
    }

    let mut resolved = PathBuf::new();
    let mut normal_component_depth = 0_usize;
    for component in absolute.components() {
        match component {
            Component::Prefix(prefix) => resolved.push(prefix.as_os_str()),
            Component::RootDir => resolved.push(component.as_os_str()),
            Component::CurDir => {}
            Component::ParentDir => {
                return Err(path_input_error(
                    "recovery paths must not contain parent traversal",
                ));
            }
            Component::Normal(segment) => {
                resolved.push(segment);
                match fs::symlink_metadata(&resolved) {
                    Ok(metadata) => {
                        if metadata_is_redirect(&metadata) {
                            if !allow_trusted_platform_root_alias(
                                &resolved,
                                &metadata,
                                normal_component_depth,
                            ) {
                                return Err(path_integrity_error(
                                    "recovery path ancestors must not be redirects",
                                ));
                            }
                            if !fs::metadata(&resolved)?.is_dir() {
                                return Err(path_integrity_error(
                                    "recovery path ancestor must be a directory",
                                ));
                            }
                        } else if !metadata.is_dir() {
                            return Err(path_integrity_error(
                                "recovery path ancestor must be a directory",
                            ));
                        }
                    }
                    Err(error) if error.kind() == io::ErrorKind::NotFound && create_missing => {
                        let parent = resolved
                            .parent()
                            .ok_or_else(|| path_input_error("recovery directory has no parent"))?;
                        let canonical_parent = fs::canonicalize(parent)?;
                        let parent_identity = observed_directory_identity(&canonical_parent)?;
                        let parent_sync_handle =
                            open_directory_sync_handle(&canonical_parent, parent_identity)?;
                        match create_private_directory(&resolved) {
                            Ok(()) => {}
                            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
                            Err(error) => return Err(error),
                        }
                        require_directory_identity(&canonical_parent, parent_identity)?;
                        let metadata = fs::symlink_metadata(&resolved)?;
                        require_direct_directory(&metadata)?;
                        sync_directory(parent_sync_handle.as_ref())?;
                        require_directory_identity(&canonical_parent, parent_identity)?;
                    }
                    Err(error) => return Err(error),
                }
                normal_component_depth = normal_component_depth.saturating_add(1);
            }
        }
    }

    let canonical = fs::canonicalize(&resolved)?;
    let metadata = fs::symlink_metadata(&canonical)?;
    require_direct_directory(&metadata)?;
    Ok(canonical)
}

#[cfg(target_os = "macos")]
fn allow_trusted_platform_root_alias(
    path: &Path,
    metadata: &Metadata,
    normal_component_depth: usize,
) -> bool {
    use std::os::unix::fs::MetadataExt;

    if normal_component_depth != 0 || !metadata.file_type().is_symlink() || metadata.uid() != 0 {
        return false;
    }
    let approved_target = if path == Path::new("/var") {
        Path::new("/private/var")
    } else {
        return false;
    };
    let Some(parent) = path.parent() else {
        return false;
    };
    let Ok(parent_metadata) = fs::symlink_metadata(parent) else {
        return false;
    };
    parent_metadata.is_dir()
        && parent_metadata.uid() == 0
        && parent_metadata.mode() & 0o022 == 0
        && fs::canonicalize(path).is_ok_and(|canonical| canonical == approved_target)
}

#[cfg(not(target_os = "macos"))]
fn allow_trusted_platform_root_alias(
    _path: &Path,
    _metadata: &Metadata,
    _normal_component_depth: usize,
) -> bool {
    false
}

fn metadata_is_redirect(metadata: &Metadata) -> bool {
    metadata.file_type().is_symlink() || metadata_is_platform_redirect(metadata)
}

#[cfg(windows)]
fn metadata_is_platform_redirect(metadata: &Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;

    windows_attributes_include_reparse_point(metadata.file_attributes())
}

#[cfg(not(windows))]
fn metadata_is_platform_redirect(_metadata: &Metadata) -> bool {
    false
}

#[cfg(any(windows, test))]
fn windows_attributes_include_reparse_point(attributes: u32) -> bool {
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
    attributes & FILE_ATTRIBUTE_REPARSE_POINT != 0
}

fn require_direct_directory(metadata: &Metadata) -> io::Result<()> {
    if metadata_is_redirect(metadata) || !metadata.is_dir() {
        return Err(path_integrity_error(
            "recovery parent must be a direct directory",
        ));
    }
    Ok(())
}

fn require_direct_regular_file(metadata: &Metadata) -> Result<(), SessionRestoreError> {
    if metadata_is_redirect(metadata) {
        return Err(path_integrity_error("recovery record must not be a path redirect").into());
    }
    if !metadata.is_file() {
        return Err(SessionRestoreError::CorruptStore(
            "recovery record is not a regular file",
        ));
    }
    Ok(())
}

#[cfg(unix)]
fn create_private_directory(path: &Path) -> io::Result<()> {
    use std::os::unix::fs::DirBuilderExt;

    let mut builder = fs::DirBuilder::new();
    builder.mode(0o700);
    builder.create(path)
}

#[cfg(not(unix))]
fn create_private_directory(path: &Path) -> io::Result<()> {
    fs::DirBuilder::new().create(path)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DirectoryIdentity(DirectoryStamp);

fn observed_directory_identity(path: &Path) -> io::Result<DirectoryIdentity> {
    let metadata = fs::symlink_metadata(path)?;
    require_direct_directory(&metadata)?;
    Ok(DirectoryIdentity(DirectoryStamp::from_metadata(&metadata)))
}

fn require_directory_identity(path: &Path, expected: DirectoryIdentity) -> io::Result<()> {
    if observed_directory_identity(path)? != expected {
        return Err(path_integrity_error(
            "recovery parent identity changed during filesystem access",
        ));
    }
    Ok(())
}

#[cfg(unix)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DirectoryStamp {
    device: u64,
    inode: u64,
    mode: u32,
}

#[cfg(unix)]
impl DirectoryStamp {
    fn from_metadata(metadata: &Metadata) -> Self {
        use std::os::unix::fs::MetadataExt;

        Self {
            device: metadata.dev(),
            inode: metadata.ino(),
            mode: metadata.mode(),
        }
    }
}

#[cfg(windows)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DirectoryStamp {
    file_attributes: u32,
    creation_time: u64,
}

#[cfg(windows)]
impl DirectoryStamp {
    fn from_metadata(metadata: &Metadata) -> Self {
        use std::os::windows::fs::MetadataExt;

        Self {
            file_attributes: metadata.file_attributes(),
            creation_time: metadata.creation_time(),
        }
    }
}

#[cfg(not(any(unix, windows)))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DirectoryStamp {
    modified: Option<SystemTime>,
}

#[cfg(not(any(unix, windows)))]
impl DirectoryStamp {
    fn from_metadata(metadata: &Metadata) -> Self {
        Self {
            modified: metadata.modified().ok(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FileIdentity(FileStamp);

impl FileIdentity {
    fn from_metadata(metadata: &Metadata) -> Self {
        Self(FileStamp::from_metadata(metadata))
    }

    fn same_file_object(self, other: Self) -> bool {
        self.0.same_file_object(other.0)
    }
}

#[cfg(unix)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FileStamp {
    device: u64,
    inode: u64,
    size: u64,
    mode: u32,
    modified_seconds: i64,
    modified_nanoseconds: i64,
    changed_seconds: i64,
    changed_nanoseconds: i64,
}

#[cfg(unix)]
impl FileStamp {
    fn from_metadata(metadata: &Metadata) -> Self {
        use std::os::unix::fs::MetadataExt;

        Self {
            device: metadata.dev(),
            inode: metadata.ino(),
            size: metadata.size(),
            mode: metadata.mode(),
            modified_seconds: metadata.mtime(),
            modified_nanoseconds: metadata.mtime_nsec(),
            changed_seconds: metadata.ctime(),
            changed_nanoseconds: metadata.ctime_nsec(),
        }
    }

    fn same_file_object(self, other: Self) -> bool {
        self.device == other.device && self.inode == other.inode && self.size == other.size
    }
}

#[cfg(windows)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FileStamp {
    file_attributes: u32,
    creation_time: u64,
    last_write_time: u64,
    file_size: u64,
}

#[cfg(windows)]
impl FileStamp {
    fn from_metadata(metadata: &Metadata) -> Self {
        use std::os::windows::fs::MetadataExt;

        Self {
            file_attributes: metadata.file_attributes(),
            creation_time: metadata.creation_time(),
            last_write_time: metadata.last_write_time(),
            file_size: metadata.file_size(),
        }
    }

    fn same_file_object(self, other: Self) -> bool {
        self == other
    }
}

#[cfg(not(any(unix, windows)))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FileStamp {
    size: u64,
    modified: Option<SystemTime>,
}

#[cfg(not(any(unix, windows)))]
impl FileStamp {
    fn from_metadata(metadata: &Metadata) -> Self {
        Self {
            size: metadata.len(),
            modified: metadata.modified().ok(),
        }
    }

    fn same_file_object(self, other: Self) -> bool {
        self == other
    }
}

fn require_destination_absent(path: &Path) -> io::Result<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata_is_redirect(&metadata) => Err(path_integrity_error(
            "immutable recovery destination must not be a redirect",
        )),
        Ok(_) => Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "immutable recovery destination already exists",
        )),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

fn verify_direct_file_identity(path: &Path, expected: FileIdentity) -> io::Result<()> {
    let metadata = fs::symlink_metadata(path)?;
    if metadata_is_redirect(&metadata)
        || !metadata.is_file()
        || FileIdentity::from_metadata(&metadata) != expected
    {
        return Err(path_integrity_error(
            "recovery staged file identity changed",
        ));
    }
    Ok(())
}

fn verify_direct_file_object(path: &Path, expected: FileIdentity) -> io::Result<()> {
    let metadata = fs::symlink_metadata(path)?;
    let observed = FileIdentity::from_metadata(&metadata);
    if metadata_is_redirect(&metadata)
        || !metadata.is_file()
        || !observed.same_file_object(expected)
    {
        return Err(path_integrity_error(
            "installed recovery record identity is uncertain",
        ));
    }
    Ok(())
}

fn direct_file_matches_object(path: &Path, expected: FileIdentity) -> io::Result<bool> {
    match verify_direct_file_object(path, expected) {
        Ok(()) => Ok(true),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error),
    }
}

fn create_pending_publication(
    parent: &Path,
    parent_identity: DirectoryIdentity,
) -> Result<PendingPublication, SessionRestoreError> {
    for _ in 0..MAX_RECOVERY_TEMP_ATTEMPTS {
        let sequence = RECOVERY_TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let path = parent.join(format!(
            ".aureline-recovery-tmp-{}-{:020}-{sequence:020}",
            std::process::id(),
            unix_nanos()
        ));
        match open_private_new_file(&path) {
            Ok(file) => {
                let mut pending = PendingPublication {
                    file: Some(file),
                    path,
                    parent: parent.to_owned(),
                    parent_identity,
                    armed: true,
                };
                if let Err(error) = restrict_new_file_permissions(pending.file_mut()) {
                    pending.scrub_and_abandon();
                    return Err(error.into());
                }
                return Ok(pending);
            }
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error.into()),
        }
    }
    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        "unable to allocate recovery temporary file",
    )
    .into())
}

#[cfg(unix)]
fn open_private_new_file(path: &Path) -> io::Result<File> {
    use std::os::unix::fs::OpenOptionsExt;

    OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)
}

#[cfg(not(unix))]
fn open_private_new_file(path: &Path) -> io::Result<File> {
    OpenOptions::new().write(true).create_new(true).open(path)
}

#[cfg(unix)]
fn restrict_new_file_permissions(file: &File) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;

    file.set_permissions(fs::Permissions::from_mode(0o600))
}

#[cfg(not(unix))]
fn restrict_new_file_permissions(_file: &File) -> io::Result<()> {
    Ok(())
}

struct PendingPublication {
    file: Option<File>,
    path: PathBuf,
    parent: PathBuf,
    parent_identity: DirectoryIdentity,
    armed: bool,
}

impl PendingPublication {
    fn file_mut(&mut self) -> &mut File {
        self.file
            .as_mut()
            .expect("pending recovery file stays open until publication completes")
    }

    fn path(&self) -> &Path {
        &self.path
    }

    fn file_identity(&self) -> io::Result<FileIdentity> {
        let file = self
            .file
            .as_ref()
            .ok_or_else(|| path_integrity_error("recovery temporary handle is unavailable"))?;
        Ok(FileIdentity::from_metadata(&file.metadata()?))
    }

    fn disarm(&mut self) {
        self.armed = false;
    }

    fn scrub_and_abandon(&mut self) {
        self.scrub_open_handle();
        self.cleanup_path_if_still_owned();
        self.armed = false;
        self.file.take();
    }

    fn abandon_after_link_attempt(&mut self) {
        self.cleanup_path_if_still_owned();
        self.armed = false;
        self.file.take();
    }

    fn scrub_open_handle(&mut self) {
        if let Some(file) = self.file.as_mut() {
            let _ = file.set_len(0);
            let _ = file.sync_all();
        }
    }

    #[cfg(unix)]
    fn remove_installed_alias_if_still_owned(
        &self,
        target: &Path,
        expected: FileIdentity,
    ) -> io::Result<bool> {
        require_directory_identity(&self.parent, self.parent_identity)?;
        let Some(file) = self.file.as_ref() else {
            return Ok(false);
        };
        let handle_identity = FileIdentity::from_metadata(&file.metadata()?);
        if !handle_identity.same_file_object(expected)
            || verify_direct_file_object(&self.path, expected).is_err()
            || verify_direct_file_object(target, expected).is_err()
        {
            return Ok(false);
        }

        // Reobserve immediately before unlinking. Stable Rust 1.75 has no
        // directory-handle-relative conditional unlink, so a path swap inside
        // the final name-operation window remains a platform boundary. A
        // replacement observed on either side is never reported as clean.
        require_directory_identity(&self.parent, self.parent_identity)?;
        verify_direct_file_object(&self.path, expected)?;
        fs::remove_file(&self.path)?;
        require_directory_identity(&self.parent, self.parent_identity)?;
        match fs::symlink_metadata(&self.path) {
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Ok(_) => return Ok(false),
            Err(error) => return Err(error),
        }
        verify_direct_file_object(target, expected)?;
        Ok(true)
    }

    #[cfg(not(unix))]
    fn remove_installed_alias_if_still_owned(
        &self,
        _target: &Path,
        _expected: FileIdentity,
    ) -> io::Result<bool> {
        // Rust 1.75 does not expose a portable unique Windows file identity.
        // Retaining an owned private alias and reporting commit uncertainty is
        // safer than deleting a pathname that may have been replaced.
        Ok(false)
    }

    #[cfg(unix)]
    fn cleanup_path_if_still_owned(&self) {
        if require_directory_identity(&self.parent, self.parent_identity).is_err() {
            return;
        }
        let Some(file) = self.file.as_ref() else {
            return;
        };
        let Ok(handle_metadata) = file.metadata() else {
            return;
        };
        let Ok(path_metadata) = fs::symlink_metadata(&self.path) else {
            return;
        };
        if metadata_is_redirect(&path_metadata)
            || !path_metadata.is_file()
            || FileIdentity::from_metadata(&handle_metadata)
                != FileIdentity::from_metadata(&path_metadata)
        {
            return;
        }
        let _ = fs::remove_file(&self.path);
    }

    #[cfg(not(unix))]
    fn cleanup_path_if_still_owned(&self) {
        // Rust 1.75 does not expose a portable unique Windows file identity.
        // The open handle is scrubbed, but pathname deletion is not authorized
        // from creation-time and attribute metadata alone.
    }
}

impl Drop for PendingPublication {
    fn drop(&mut self) {
        if self.armed {
            self.scrub_open_handle();
            self.cleanup_path_if_still_owned();
        }
        self.file.take();
    }
}

#[cfg(unix)]
fn open_directory_sync_handle(
    path: &Path,
    expected: DirectoryIdentity,
) -> io::Result<Option<File>> {
    let directory = File::open(path)?;
    let metadata = directory.metadata()?;
    require_direct_directory(&metadata)?;
    if DirectoryIdentity(DirectoryStamp::from_metadata(&metadata)) != expected {
        return Err(path_integrity_error(
            "recovery parent changed while opening for directory sync",
        ));
    }
    Ok(Some(directory))
}

#[cfg(not(unix))]
fn open_directory_sync_handle(
    _path: &Path,
    _expected: DirectoryIdentity,
) -> io::Result<Option<File>> {
    Ok(None)
}

#[cfg(unix)]
fn sync_directory(directory: Option<&File>) -> io::Result<()> {
    directory
        .ok_or_else(|| path_integrity_error("recovery directory sync handle is unavailable"))?
        .sync_all()
}

#[cfg(not(unix))]
fn sync_directory(_directory: Option<&File>) -> io::Result<()> {
    // Rust 1.75 exposes no portable parent-directory fsync on Windows.
    Ok(())
}

fn path_integrity_error(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

fn path_input_error(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RecoveryIoFailpoint {
    HardLinkReportedErrorAfterInstall,
    AfterHardLink,
    BeforeDirectorySync,
    BeforeCaptureValidation,
}

#[cfg(test)]
thread_local! {
    static RECOVERY_IO_FAILPOINT: Cell<Option<RecoveryIoFailpoint>> = const { Cell::new(None) };
}

#[cfg(test)]
struct RecoveryIoFailpointGuard;

#[cfg(test)]
impl Drop for RecoveryIoFailpointGuard {
    fn drop(&mut self) {
        RECOVERY_IO_FAILPOINT.with(|configured| configured.set(None));
    }
}

#[cfg(test)]
fn inject_recovery_io_failure(failpoint: RecoveryIoFailpoint) -> RecoveryIoFailpointGuard {
    RECOVERY_IO_FAILPOINT.with(|configured| configured.set(Some(failpoint)));
    RecoveryIoFailpointGuard
}

#[cfg(test)]
fn recovery_io_failpoint(failpoint: RecoveryIoFailpoint) -> io::Result<()> {
    let fires = RECOVERY_IO_FAILPOINT.with(|configured| configured.get() == Some(failpoint));
    if fires {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "synthetic recovery publication failure",
        ));
    }
    Ok(())
}

#[cfg(not(test))]
fn recovery_io_failpoint(_failpoint: RecoveryIoFailpoint) -> io::Result<()> {
    Ok(())
}

type MaterializedTopology = (
    Vec<TabGroupInventoryEntry>,
    Vec<StablePaneInventoryEntry>,
    PaneNode,
    Vec<FocusChainEntry>,
    Vec<PlaceholderBehaviorRecord>,
);

fn materialize_topology_from_capture(
    groups: &[TabGroupCaptureInput],
    pane_tree_layout: Option<&TabGroupLayoutCapture>,
    focused_group_id: Option<&str>,
    window_id: &str,
) -> Result<MaterializedTopology, SessionRestoreError> {
    let mut tab_group_topology = Vec::new();
    let mut stable_panes = Vec::new();
    let mut group_nodes = HashMap::new();
    let mut group_focus_targets = Vec::new();
    let mut placeholder_behaviors = Vec::new();

    for group in groups {
        let mut ordered_tab_ids = Vec::new();
        let mut pinned_tab_ids = Vec::new();
        let mut tabs = Vec::new();
        for tab in &group.ordered_tabs {
            let tab_id = tab.tab_id.clone();
            ordered_tab_ids.push(tab_id.clone());
            if tab.pinned {
                pinned_tab_ids.push(tab_id.clone());
            }
            // Tab ids are the stable capture identity available at this
            // boundary. Do not include snapshot or group ids: doing so would
            // silently remint a pane whenever a new snapshot was captured or
            // the tab moved between groups.
            let pane_id = format!("pane:{tab_id}");
            let (hydration_behavior, availability_state) =
                restore_posture_for_surface(tab.surface_role, tab.surface_class);
            let placeholder_behavior = placeholder_behavior_for_surface(
                &pane_id,
                tab.surface_role,
                tab.surface_class,
                tab.tab_label.as_deref(),
                availability_state,
            );
            let placeholder_card = placeholder_behavior
                .as_ref()
                .map(|behavior| PlaceholderCard {
                    placeholder_reason: behavior.placeholder_reason,
                    safe_actions: behavior.safe_actions.clone(),
                    evidence_retained: behavior.evidence_retained,
                    last_known_provenance_label: behavior.last_known_provenance_label.clone(),
                });

            stable_panes.push(StablePaneInventoryEntry {
                pane_id: pane_id.clone(),
                surface_role: tab.surface_role,
                surface_class: tab.surface_class,
                hydration_behavior,
                availability_state,
                presentation_spotlighted: None,
                follow_anchor_candidate: None,
                title_hint: tab.tab_label.clone(),
                // Terminal-owned restore metadata must remain in its governed
                // record family. The recovery topology retains only the
                // redaction-safe placeholder posture and pane identity.
                restore_metadata: None,
            });

            let surface = PaneSurfaceDescriptor {
                surface_role: tab.surface_role,
                surface_class: tab.surface_class,
                live_surface_class: None,
                hydration_behavior,
                availability_state,
                title_hint: tab.tab_label.clone(),
                surface_binding_ref: tab.surface_binding_ref.clone(),
                restore_metadata: None,
                follow_anchor_candidate: None,
                presentation_spotlighted: None,
                placeholder_card,
            };

            if let Some(behavior) = placeholder_behavior {
                placeholder_behaviors.push(behavior);
            }

            tabs.push(TabRecord {
                tab_id: tab_id.clone(),
                tab_label: tab.tab_label.clone(),
                pinned: Some(tab.pinned),
                dirty_badge_visible: Some(tab.dirty_badge_visible),
                pane: PaneLeafNode {
                    node_kind: "leaf".to_string(),
                    pane_id: pane_id.clone(),
                    surface,
                },
            });
        }

        let active_tab_id = group
            .active_tab_id
            .clone()
            .filter(|candidate| ordered_tab_ids.iter().any(|tab_id| tab_id == candidate))
            .or_else(|| ordered_tab_ids.first().cloned())
            .unwrap_or_else(|| format!("tab:{window_id}:missing"));

        if let Some(active_tab) = tabs.iter().find(|tab| tab.tab_id == active_tab_id) {
            group_focus_targets.push((
                group.group_id.clone(),
                active_tab.tab_id.clone(),
                active_tab.pane.pane_id.clone(),
            ));
        }

        tab_group_topology.push(TabGroupInventoryEntry {
            group_id: group.group_id.clone(),
            ordered_tab_ids,
            active_tab_id: active_tab_id.clone(),
            pinned_tab_ids: (!pinned_tab_ids.is_empty()).then_some(pinned_tab_ids),
            close_empty_group: None,
        });

        group_nodes.insert(
            group.group_id.clone(),
            PaneNode::TabGroup {
                group_id: group.group_id.clone(),
                tabs,
                active_tab_id,
                close_empty_group: None,
            },
        );
    }

    let focus_target = focused_group_id
        .and_then(|focused| {
            group_focus_targets
                .iter()
                .find(|(group_id, _, _)| group_id == focused)
        })
        .or_else(|| group_focus_targets.first());
    let focus_chain = if let Some((_, tab_id, pane_id)) = focus_target {
        vec![
            FocusChainEntry {
                target_kind: FocusTargetKind::Tab,
                target_ref: tab_id.clone(),
                note: Some("active tab".to_string()),
            },
            FocusChainEntry {
                target_kind: FocusTargetKind::Pane,
                target_ref: pane_id.clone(),
                note: None,
            },
        ]
    } else {
        vec![FocusChainEntry {
            target_kind: FocusTargetKind::WindowChrome,
            target_ref: window_id.to_string(),
            note: Some("no tabs captured".to_string()),
        }]
    };

    let root_node = if let Some(layout) = pane_tree_layout {
        materialize_group_layout(layout, &mut group_nodes)?
    } else {
        let Some(group_id) = groups.first().map(|group| group.group_id.as_str()) else {
            return Err(SessionRestoreError::InvalidCapture(
                "pane-tree materialization requires a captured group",
            ));
        };
        group_nodes
            .remove(group_id)
            .ok_or(SessionRestoreError::InvalidCapture(
                "pane-tree layout referenced a missing captured group",
            ))?
    };

    Ok((
        tab_group_topology,
        stable_panes,
        root_node,
        focus_chain,
        placeholder_behaviors,
    ))
}

fn placeholder_behavior_for_surface(
    pane_id: &str,
    role: SurfaceRole,
    class: SurfaceClass,
    title_hint: Option<&str>,
    availability_state: AvailabilityState,
) -> Option<PlaceholderBehaviorRecord> {
    if matches!(availability_state, AvailabilityState::Ready) {
        return None;
    }

    let (placeholder_reason, safe_actions, note) = if is_side_effectful_capture_surface(role, class)
    {
        (
            PlaceholderReasonClass::NonReentrantLiveSurface,
            vec![
                PlaceholderAction::RerunExplicitly,
                PlaceholderAction::RebindExistingSession,
                PlaceholderAction::ExportEvidence,
                PlaceholderAction::RemovePane,
            ],
            "live surface retained as an inactive placeholder; automatic rerun is forbidden",
        )
    } else if matches!(role, SurfaceRole::CustomExtension)
        || matches!(class, SurfaceClass::ExtensionView)
    {
        (
            PlaceholderReasonClass::MissingExtension,
            vec![
                PlaceholderAction::LocateExtension,
                PlaceholderAction::InstallExtension,
                PlaceholderAction::OpenWithout,
                PlaceholderAction::ExportEvidence,
                PlaceholderAction::RemovePane,
            ],
            "extension surface retained as a placeholder until its dependency is available",
        )
    } else {
        (
            PlaceholderReasonClass::ManualRecoveryRequired,
            vec![
                PlaceholderAction::OpenRepairInstructions,
                PlaceholderAction::EscalateToManualRepair,
                PlaceholderAction::ExportEvidence,
            ],
            "surface retained in its original pane slot for explicit manual recovery",
        )
    };

    Some(PlaceholderBehaviorRecord {
        pane_id: pane_id.to_string(),
        placeholder_reason,
        safe_actions,
        // The stable pane inventory and its redaction-safe title/class are a
        // metadata-only summary even when no terminal-owned record is joined.
        evidence_retained: true,
        last_known_provenance_label: title_hint.map(str::to_string),
        note: Some(note.to_string()),
    })
}

fn materialize_group_layout(
    layout: &TabGroupLayoutCapture,
    group_nodes: &mut HashMap<String, PaneNode>,
) -> Result<PaneNode, SessionRestoreError> {
    match layout {
        TabGroupLayoutCapture::TabGroup { group_id } => {
            group_nodes
                .remove(group_id)
                .ok_or(SessionRestoreError::InvalidCapture(
                    "pane-tree layout referenced a missing or duplicate group",
                ))
        }
        TabGroupLayoutCapture::Split {
            split_id,
            orientation,
            children,
            weights,
        } => {
            let children = children
                .iter()
                .map(|child| materialize_group_layout(child, group_nodes))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(PaneNode::Split {
                split_id: split_id.clone(),
                orientation: *orientation,
                children,
                weights: weights.clone(),
            })
        }
    }
}

fn restore_posture_for_surface(
    role: SurfaceRole,
    class: SurfaceClass,
) -> (HydrationBehavior, AvailabilityState) {
    if matches!(
        role,
        SurfaceRole::Terminal
            | SurfaceRole::Debugger
            | SurfaceRole::Notebook
            | SurfaceRole::AiPanel
            | SurfaceRole::Test
            | SurfaceRole::Placeholder
            | SurfaceRole::CustomExtension
    ) || matches!(
        class,
        SurfaceClass::TerminalView
            | SurfaceClass::DebugView
            | SurfaceClass::NotebookView
            | SurfaceClass::AiPanel
            | SurfaceClass::TestResults
            | SurfaceClass::PlaceholderCard
            | SurfaceClass::ExtensionView
    ) {
        (
            HydrationBehavior::PlaceholderOnly,
            AvailabilityState::Placeholder,
        )
    } else {
        (
            HydrationBehavior::EagerLightweight,
            AvailabilityState::Ready,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn capture_input(
        groups: Vec<TabGroupCaptureInput>,
        focused_group_id: Option<&str>,
    ) -> SessionRestoreCaptureInput {
        let pane_tree_layout = flat_layout(&groups);
        SessionRestoreCaptureInput {
            workspace_ref: "workspace:test".to_string(),
            producer_build: ProducerBuildStamp {
                producer_name: "aureline-recovery-store-test".to_string(),
                producer_version: "0.0.0".to_string(),
                producer_channel: None,
                producer_platform_class: None,
                producer_instance_handle: None,
            },
            source_schema_version: "1".to_string(),
            trusted_root_refs: Vec::new(),
            active_workset_ids: Vec::new(),
            dirty_buffer_journal_identities: Vec::new(),
            recovery_journal_refs: Vec::new(),
            local_history_snapshot_refs: Vec::new(),
            evidence_bundle_refs: Vec::new(),
            excluded_live_authority_classes: Vec::new(),
            downgrade_triggers: Vec::new(),
            window_id: "window:test".to_string(),
            window_role: WindowRole::Primary,
            topology_family_ref: None,
            sibling_window_refs: Vec::new(),
            tab_groups: groups,
            pane_tree_layout,
            focused_group_id: focused_group_id.map(str::to_string),
            emitted_at: "mono:test:1".to_string(),
            notes: None,
        }
    }

    fn flat_layout(groups: &[TabGroupCaptureInput]) -> Option<TabGroupLayoutCapture> {
        (groups.len() > 1).then(|| TabGroupLayoutCapture::Split {
            split_id: "split:test:root".to_string(),
            orientation: SplitOrientation::Vertical,
            children: groups
                .iter()
                .map(|group| TabGroupLayoutCapture::TabGroup {
                    group_id: group.group_id.clone(),
                })
                .collect(),
            weights: None,
        })
    }

    fn tab(id: &str, pinned: bool, role: SurfaceRole, class: SurfaceClass) -> TabItemCaptureInput {
        TabItemCaptureInput {
            tab_id: id.to_string(),
            tab_label: Some(format!("{id}.label")),
            surface_binding_ref: None,
            pinned,
            dirty_badge_visible: id == "tab:b",
            surface_role: role,
            surface_class: class,
            restore_metadata: None,
        }
    }

    fn rewrite_json_value(path: &Path, mutate: impl FnOnce(&mut serde_json::Value)) {
        let bytes = fs::read(path).expect("read JSON record for mutation");
        let mut value: serde_json::Value =
            serde_json::from_slice(&bytes).expect("parse JSON record for mutation");
        mutate(&mut value);
        fs::write(
            path,
            serde_json::to_vec_pretty(&value).expect("serialize mutated JSON record"),
        )
        .expect("write mutated JSON record");
    }

    #[test]
    fn multi_group_materialization_preserves_tabs_active_focus_and_pins_once() {
        let groups = vec![
            TabGroupCaptureInput {
                group_id: "group:first".to_string(),
                ordered_tabs: vec![
                    tab("tab:a", true, SurfaceRole::Editor, SurfaceClass::TextEditor),
                    tab(
                        "tab:b",
                        false,
                        SurfaceRole::Editor,
                        SurfaceClass::TextEditor,
                    ),
                ],
                active_tab_id: Some("tab:b".to_string()),
            },
            TabGroupCaptureInput {
                group_id: "group:second".to_string(),
                ordered_tabs: vec![
                    tab(
                        "tab:c",
                        false,
                        SurfaceRole::Editor,
                        SurfaceClass::TextEditor,
                    ),
                    tab(
                        "tab:d",
                        false,
                        SurfaceRole::Editor,
                        SurfaceClass::TextEditor,
                    ),
                ],
                active_tab_id: Some("tab:d".to_string()),
            },
        ];

        let layout = flat_layout(&groups).expect("multi-group layout");
        let (inventory, panes, root, focus_chain, placeholder_behaviors) =
            materialize_topology_from_capture(
                &groups,
                Some(&layout),
                Some("group:second"),
                "window:test",
            )
            .expect("materialize topology");

        assert_eq!(inventory.len(), 2);
        assert_eq!(
            inventory[0].ordered_tab_ids,
            vec!["tab:a".to_string(), "tab:b".to_string()]
        );
        assert_eq!(inventory[0].active_tab_id, "tab:b");
        assert_eq!(inventory[0].pinned_tab_ids, Some(vec!["tab:a".to_string()]));
        assert_eq!(
            inventory[1].ordered_tab_ids,
            vec!["tab:c".to_string(), "tab:d".to_string()]
        );
        assert_eq!(inventory[1].active_tab_id, "tab:d");

        assert_eq!(panes.len(), 4);
        assert!(placeholder_behaviors.is_empty());
        let unique_panes: std::collections::HashSet<_> =
            panes.iter().map(|pane| pane.pane_id.as_str()).collect();
        assert_eq!(
            unique_panes.len(),
            4,
            "pane inventory must not duplicate tabs"
        );

        let PaneNode::Split { children, .. } = root else {
            panic!("multiple groups must materialize as a split")
        };
        assert_eq!(children.len(), 2);
        let PaneNode::TabGroup {
            tabs: first_tabs,
            active_tab_id: first_active,
            ..
        } = &children[0]
        else {
            panic!("first split child must remain a tab group")
        };
        assert_eq!(
            first_tabs
                .iter()
                .map(|tab| tab.tab_id.as_str())
                .collect::<Vec<_>>(),
            vec!["tab:a", "tab:b"]
        );
        assert_eq!(first_active, "tab:b");
        assert_eq!(first_tabs[1].dirty_badge_visible, Some(true));

        let PaneNode::TabGroup {
            tabs: second_tabs,
            active_tab_id: second_active,
            ..
        } = &children[1]
        else {
            panic!("second split child must remain a tab group")
        };
        assert_eq!(
            second_tabs
                .iter()
                .map(|tab| tab.tab_id.as_str())
                .collect::<Vec<_>>(),
            vec!["tab:c", "tab:d"]
        );
        assert_eq!(second_active, "tab:d");
        assert_eq!(focus_chain[0].target_kind, FocusTargetKind::Tab);
        assert_eq!(focus_chain[0].target_ref, "tab:d");
        assert_eq!(focus_chain[1].target_kind, FocusTargetKind::Pane);
        assert_eq!(focus_chain[1].target_ref, second_tabs[1].pane.pane_id);

        let dir = tempfile::tempdir().expect("tempdir");
        let mut store = SessionRestoreStore::new(dir.path());
        let refs = store
            .capture(capture_input(groups, Some("group:second")))
            .expect("persist multi-group capture");
        let persisted_snapshot = store
            .load_window_topology_snapshot(&refs.snapshot_id)
            .expect("persisted snapshot");
        assert_eq!(persisted_snapshot.stable_pane_id_inventory.len(), 4);
        assert_eq!(persisted_snapshot.focus_chain[0].target_ref, "tab:d");
        let persisted_body = store
            .load_pane_tree_body(&refs.snapshot_id)
            .expect("persisted pane tree");
        let PaneNode::Split { children, .. } = persisted_body.pane_tree.root_node else {
            panic!("persisted multi-group topology must remain split")
        };
        assert_eq!(children.len(), 2);
        assert!(children.iter().all(|child| matches!(
            child,
            PaneNode::TabGroup { tabs, .. } if tabs.len() == 2
        )));
    }

    #[test]
    fn nested_split_identity_order_orientation_and_weights_survive_persistence() {
        let groups = vec![
            TabGroupCaptureInput {
                group_id: "group:first".to_string(),
                ordered_tabs: vec![tab(
                    "tab:a",
                    false,
                    SurfaceRole::Editor,
                    SurfaceClass::TextEditor,
                )],
                active_tab_id: Some("tab:a".to_string()),
            },
            TabGroupCaptureInput {
                group_id: "group:second".to_string(),
                ordered_tabs: vec![tab(
                    "tab:b",
                    false,
                    SurfaceRole::Editor,
                    SurfaceClass::TextEditor,
                )],
                active_tab_id: Some("tab:b".to_string()),
            },
            TabGroupCaptureInput {
                group_id: "group:third".to_string(),
                ordered_tabs: vec![tab(
                    "tab:c",
                    false,
                    SurfaceRole::Editor,
                    SurfaceClass::TextEditor,
                )],
                active_tab_id: Some("tab:c".to_string()),
            },
        ];
        let layout = TabGroupLayoutCapture::Split {
            split_id: "split:outer".to_string(),
            orientation: SplitOrientation::Horizontal,
            children: vec![
                TabGroupLayoutCapture::TabGroup {
                    group_id: "group:first".to_string(),
                },
                TabGroupLayoutCapture::Split {
                    split_id: "split:inner".to_string(),
                    orientation: SplitOrientation::Vertical,
                    children: vec![
                        TabGroupLayoutCapture::TabGroup {
                            group_id: "group:second".to_string(),
                        },
                        TabGroupLayoutCapture::TabGroup {
                            group_id: "group:third".to_string(),
                        },
                    ],
                    weights: Some(vec![2.0, 1.0]),
                },
            ],
            weights: Some(vec![3.0, 2.0]),
        };
        let mut input = capture_input(groups, Some("group:third"));
        input.pane_tree_layout = Some(layout);
        let dir = tempfile::tempdir().expect("tempdir");
        let mut store = SessionRestoreStore::new(dir.path());
        let refs = store.capture(input).expect("nested capture");
        let body = store
            .load_pane_tree_body(&refs.snapshot_id)
            .expect("pane tree body");

        let PaneNode::Split {
            split_id,
            orientation,
            children,
            weights,
        } = body.pane_tree.root_node
        else {
            panic!("outer split must survive")
        };
        assert_eq!(split_id, "split:outer");
        assert_eq!(orientation, SplitOrientation::Horizontal);
        assert_eq!(weights, Some(vec![3.0, 2.0]));
        assert!(matches!(
            &children[0],
            PaneNode::TabGroup { group_id, .. } if group_id == "group:first"
        ));
        let PaneNode::Split {
            split_id,
            orientation,
            children,
            weights,
        } = &children[1]
        else {
            panic!("inner split must survive")
        };
        assert_eq!(split_id, "split:inner");
        assert_eq!(*orientation, SplitOrientation::Vertical);
        assert_eq!(weights.as_deref(), Some(&[2.0, 1.0][..]));
        assert!(matches!(
            &children[0],
            PaneNode::TabGroup { group_id, .. } if group_id == "group:second"
        ));
        assert!(matches!(
            &children[1],
            PaneNode::TabGroup { group_id, .. } if group_id == "group:third"
        ));
    }

    #[test]
    fn pane_identity_survives_new_snapshots_and_group_moves() {
        let dir = tempfile::tempdir().expect("tempdir");
        let mut store = SessionRestoreStore::new(dir.path());
        let groups = |group_id: &str| {
            vec![TabGroupCaptureInput {
                group_id: group_id.to_string(),
                ordered_tabs: vec![tab(
                    "tab:stable",
                    false,
                    SurfaceRole::Editor,
                    SurfaceClass::TextEditor,
                )],
                active_tab_id: Some("tab:stable".to_string()),
            }]
        };
        let first = store
            .capture(capture_input(groups("group:first"), Some("group:first")))
            .expect("first capture");
        let second = store
            .capture(capture_input(groups("group:moved"), Some("group:moved")))
            .expect("moved capture");
        let first_snapshot = store
            .load_window_topology_snapshot(&first.snapshot_id)
            .expect("first snapshot");
        let second_snapshot = store
            .load_window_topology_snapshot(&second.snapshot_id)
            .expect("second snapshot");

        assert_eq!(
            first_snapshot.stable_pane_id_inventory[0].pane_id,
            second_snapshot.stable_pane_id_inventory[0].pane_id
        );
        assert_eq!(
            second_snapshot.stable_pane_id_inventory[0].pane_id,
            "pane:tab:stable"
        );
    }

    #[test]
    fn materializer_falls_back_but_store_rejects_invalid_active_tab() {
        let groups = vec![TabGroupCaptureInput {
            group_id: "group:first".to_string(),
            ordered_tabs: vec![tab(
                "tab:a",
                false,
                SurfaceRole::Editor,
                SurfaceClass::TextEditor,
            )],
            active_tab_id: Some("tab:stale".to_string()),
        }];

        let (inventory, _, root, focus_chain, _) =
            materialize_topology_from_capture(&groups, None, Some("group:first"), "window:test")
                .expect("materialize topology");
        assert_eq!(inventory[0].active_tab_id, "tab:a");
        let PaneNode::TabGroup { active_tab_id, .. } = root else {
            panic!("one group must remain a tab group")
        };
        assert_eq!(active_tab_id, "tab:a");
        assert_eq!(focus_chain[0].target_ref, "tab:a");

        let dir = tempfile::tempdir().expect("tempdir");
        let mut store = SessionRestoreStore::new(dir.path());
        let err = store
            .capture(capture_input(groups, Some("group:first")))
            .expect_err("public store boundary must reject stale active refs");
        assert!(matches!(err, SessionRestoreError::InvalidCapture(_)));
    }

    #[test]
    fn absent_active_tab_uses_first_tab_without_losing_focus() {
        let groups = vec![TabGroupCaptureInput {
            group_id: "group:first".to_string(),
            ordered_tabs: vec![tab(
                "tab:a",
                false,
                SurfaceRole::Editor,
                SurfaceClass::TextEditor,
            )],
            active_tab_id: None,
        }];

        let dir = tempfile::tempdir().expect("tempdir");
        let mut store = SessionRestoreStore::new(dir.path());
        let refs = store
            .capture(capture_input(groups, Some("group:first")))
            .expect("capture without an explicit active tab");
        let snapshot = store
            .load_window_topology_snapshot(&refs.snapshot_id)
            .expect("snapshot");
        assert_eq!(snapshot.tab_group_topology[0].active_tab_id, "tab:a");
        assert_eq!(snapshot.focus_chain[0].target_ref, "tab:a");
    }

    #[test]
    fn terminal_metadata_requires_the_exact_terminal_role_and_class_pair() {
        for (role, class) in [
            (SurfaceRole::Editor, SurfaceClass::TerminalView),
            (SurfaceRole::Terminal, SurfaceClass::TextEditor),
        ] {
            let mut item = tab("tab:mismatch", false, role, class);
            item.restore_metadata = Some(TerminalPaneRestoreMetadata {
                restore_metadata_ref: "terminal-restore-metadata:test".to_string(),
                working_directory: Some("service".to_string()),
                environment_scope_token: "workspace".to_string(),
                shell_identity: "zsh".to_string(),
                shell_family_token: "zsh".to_string(),
                last_command_class_token: "build".to_string(),
                auto_rerun_forbidden: true,
                raw_command_body_present: false,
                raw_environment_body_present: false,
            });
            let groups = vec![TabGroupCaptureInput {
                group_id: "group:first".to_string(),
                ordered_tabs: vec![item],
                active_tab_id: Some("tab:mismatch".to_string()),
            }];
            let dir = tempfile::tempdir().expect("tempdir");
            let mut store = SessionRestoreStore::new(dir.path());
            assert!(matches!(
                store.capture(capture_input(groups, Some("group:first"))),
                Err(SessionRestoreError::InvalidCapture(_))
            ));
        }
    }

    #[test]
    fn rejects_path_url_control_and_oversized_surface_bindings() {
        let invalid_bindings = vec![
            ("path", "/Users/alice/private/file.rs".to_string()),
            ("url", "https://example.test/private".to_string()),
            ("control", "binding:\nsecret".to_string()),
            (
                "oversized",
                format!("binding:{}", "x".repeat(MAX_CAPTURE_OPAQUE_REF_LEN)),
            ),
        ];

        for (case, binding) in invalid_bindings {
            let mut editor_tab = tab(
                "tab:a",
                false,
                SurfaceRole::Editor,
                SurfaceClass::TextEditor,
            );
            editor_tab.surface_binding_ref = Some(binding);
            let groups = vec![TabGroupCaptureInput {
                group_id: "group:first".to_string(),
                ordered_tabs: vec![editor_tab],
                active_tab_id: Some("tab:a".to_string()),
            }];
            let dir = tempfile::tempdir().expect("tempdir");
            let mut store = SessionRestoreStore::new(dir.path());
            let err = store
                .capture(capture_input(groups, Some("group:first")))
                .expect_err("unsafe surface binding must fail closed");
            assert!(
                matches!(err, SessionRestoreError::InvalidCapture(_)),
                "unexpected error for {case}: {err}"
            );
        }
    }

    #[test]
    fn rejects_control_group_and_oversized_tab_ids() {
        let control_group = vec![TabGroupCaptureInput {
            group_id: "group:\u{7}".to_string(),
            ordered_tabs: vec![tab(
                "tab:a",
                false,
                SurfaceRole::Editor,
                SurfaceClass::TextEditor,
            )],
            active_tab_id: Some("tab:a".to_string()),
        }];
        let dir = tempfile::tempdir().expect("tempdir");
        let mut store = SessionRestoreStore::new(dir.path());
        assert!(matches!(
            store.capture(capture_input(control_group, None)),
            Err(SessionRestoreError::InvalidCapture(_))
        ));

        let oversized_tab_id = format!("tab:{}", "x".repeat(MAX_CAPTURE_OPAQUE_REF_LEN));
        let oversized_tab = TabItemCaptureInput {
            tab_id: oversized_tab_id.clone(),
            tab_label: Some("editor".to_string()),
            surface_binding_ref: None,
            pinned: false,
            dirty_badge_visible: false,
            surface_role: SurfaceRole::Editor,
            surface_class: SurfaceClass::TextEditor,
            restore_metadata: None,
        };
        let groups = vec![TabGroupCaptureInput {
            group_id: "group:first".to_string(),
            ordered_tabs: vec![oversized_tab],
            active_tab_id: Some(oversized_tab_id),
        }];
        assert!(matches!(
            store.capture(capture_input(groups, Some("group:first"))),
            Err(SessionRestoreError::InvalidCapture(_))
        ));
    }

    #[test]
    fn rejects_refs_that_overflow_derived_authority_ids_before_publication() {
        let groups = || {
            vec![TabGroupCaptureInput {
                group_id: "group:first".to_string(),
                ordered_tabs: vec![tab(
                    "tab:a",
                    false,
                    SurfaceRole::Editor,
                    SurfaceClass::TextEditor,
                )],
                active_tab_id: Some("tab:a".to_string()),
            }]
        };

        let workspace_dir = tempfile::tempdir().expect("workspace tempdir");
        let mut workspace_store = SessionRestoreStore::new(workspace_dir.path());
        let mut workspace_overflow = capture_input(groups(), Some("group:first"));
        workspace_overflow.workspace_ref = "w".repeat(MAX_CAPTURE_OPAQUE_REF_LEN);
        assert!(is_bounded_opaque_ref(&workspace_overflow.workspace_ref));
        assert!(matches!(
            workspace_store.capture(workspace_overflow),
            Err(SessionRestoreError::InvalidCapture(_))
        ));
        assert!(!workspace_store.root_path().exists());

        let tab_dir = tempfile::tempdir().expect("tab tempdir");
        let mut tab_store = SessionRestoreStore::new(tab_dir.path());
        let long_tab_id = "t".repeat(MAX_CAPTURE_OPAQUE_REF_LEN);
        assert!(is_bounded_opaque_ref(&long_tab_id));
        let long_tab_groups = vec![TabGroupCaptureInput {
            group_id: "group:first".to_string(),
            ordered_tabs: vec![tab(
                &long_tab_id,
                false,
                SurfaceRole::Editor,
                SurfaceClass::TextEditor,
            )],
            active_tab_id: Some(long_tab_id),
        }];
        assert!(matches!(
            tab_store.capture(capture_input(long_tab_groups, Some("group:first"))),
            Err(SessionRestoreError::InvalidCapture(_))
        ));
        assert!(!tab_store.root_path().exists());
    }

    #[test]
    fn reopening_store_seeds_ids_and_publishes_a_new_joined_capture() {
        let dir = tempfile::tempdir().expect("tempdir");
        let groups = || {
            vec![TabGroupCaptureInput {
                group_id: "group:first".to_string(),
                ordered_tabs: vec![tab(
                    "tab:a",
                    false,
                    SurfaceRole::Editor,
                    SurfaceClass::TextEditor,
                )],
                active_tab_id: Some("tab:a".to_string()),
            }]
        };

        let first = {
            let mut store = SessionRestoreStore::new(dir.path());
            store
                .capture(capture_input(groups(), Some("group:first")))
                .expect("first capture")
        };
        let mut reopened = SessionRestoreStore::new(dir.path());
        let second = reopened
            .capture(capture_input(groups(), Some("group:first")))
            .expect("second capture after reopen");

        assert_ne!(first, second);
        assert_eq!(
            parse_durable_record_id(&first.checkpoint_id, "ckpt")
                .unwrap()
                .0,
            1
        );
        assert_eq!(
            parse_durable_record_id(&second.checkpoint_id, "ckpt")
                .unwrap()
                .0,
            2
        );
        assert_eq!(
            parse_durable_record_id(&first.snapshot_id, "snap")
                .unwrap()
                .0,
            1
        );
        assert_eq!(
            parse_durable_record_id(&second.snapshot_id, "snap")
                .unwrap()
                .0,
            2
        );
        assert!(reopened.load_checkpoint(&first.checkpoint_id).is_ok());
        assert!(reopened.load_checkpoint(&second.checkpoint_id).is_ok());
        assert_eq!(reopened.latest_refs().expect("latest refs"), Some(second));
        let advisory: LatestIndexRecord =
            read_json(&reopened.root_path().join("latest.json")).expect("legacy advisory");
        assert_eq!(advisory.checkpoint_id, first.checkpoint_id);
        assert_eq!(advisory.snapshot_id, first.snapshot_id);
        assert_eq!(
            std::fs::read_dir(reopened.root_path().join("latest_indices"))
                .expect("latest index dir")
                .filter_map(Result::ok)
                .filter(
                    |entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("json")
                )
                .count(),
            2
        );
    }

    #[test]
    fn corrupt_newest_versioned_index_falls_back_to_previous_indexed_pair() {
        let dir = tempfile::tempdir().expect("tempdir");
        let groups = || {
            vec![TabGroupCaptureInput {
                group_id: "group:first".to_string(),
                ordered_tabs: vec![tab(
                    "tab:a",
                    false,
                    SurfaceRole::Editor,
                    SurfaceClass::TextEditor,
                )],
                active_tab_id: Some("tab:a".to_string()),
            }]
        };
        let mut store = SessionRestoreStore::new(dir.path());
        let first = store
            .capture(capture_input(groups(), Some("group:first")))
            .expect("first capture");
        let second = store
            .capture(capture_input(groups(), Some("group:first")))
            .expect("second capture");

        std::fs::write(
            store
                .root_path()
                .join("latest_indices")
                .join(format!("{}.json", second.snapshot_id)),
            b"{not-json",
        )
        .expect("corrupt newest immutable index");

        let orphan_snapshot_id = "snap-99999999999999999999-00000000000000000999".to_string();
        let mut orphan = store
            .load_window_topology_snapshot(&second.snapshot_id)
            .expect("second snapshot");
        orphan.snapshot_id.clone_from(&orphan_snapshot_id);
        orphan.pane_tree_record_ref.clone_from(&orphan_snapshot_id);
        orphan.workspace_authority_checkpoint_ref =
            "ckpt-99999999999999999999-00000000000000000999".to_string();
        write_new_json(
            &store
                .root_path()
                .join("window_topology_snapshots")
                .join(format!("{orphan_snapshot_id}.json")),
            &orphan,
        )
        .expect("write torn newest snapshot");

        let selection = store.latest_selection().expect("indexed fallback");
        assert_eq!(
            selection.latest_refs,
            Some(first.clone()),
            "neither an unindexed body nor a corrupt index may become selection authority"
        );
        assert_eq!(selection.skipped_newer_candidates.len(), 1);
        assert_eq!(
            selection.skipped_newer_candidates[0],
            SessionRestoreSelectionWarning {
                snapshot_id: second.snapshot_id,
                warning_class: SessionRestoreSelectionWarningClass::CorruptIndex,
            }
        );
        assert_eq!(
            store
                .latest_summary()
                .expect("summary with fallback evidence")
                .expect("fallback summary")
                .skipped_newer_candidate_count,
            1
        );
        assert!(store.load_checkpoint(&first.checkpoint_id).is_ok());
    }

    #[test]
    fn unsupported_versions_and_unknown_fields_are_skipped_with_evidence() {
        let dir = tempfile::tempdir().expect("tempdir");
        let groups = || {
            vec![TabGroupCaptureInput {
                group_id: "group:first".to_string(),
                ordered_tabs: vec![tab(
                    "tab:a",
                    false,
                    SurfaceRole::Editor,
                    SurfaceClass::TextEditor,
                )],
                active_tab_id: Some("tab:a".to_string()),
            }]
        };
        let mut store = SessionRestoreStore::new(dir.path());
        let first = store
            .capture(capture_input(groups(), Some("group:first")))
            .expect("first capture");
        let second = store
            .capture(capture_input(groups(), Some("group:first")))
            .expect("second capture");
        let third = store
            .capture(capture_input(groups(), Some("group:first")))
            .expect("third capture");

        rewrite_json_value(
            &store
                .root_path()
                .join("window_topology_snapshots")
                .join(format!("{}.json", second.snapshot_id)),
            |record| record["topology_packet_schema_version"] = serde_json::json!(999),
        );
        rewrite_json_value(
            &store
                .root_path()
                .join("pane_tree_bodies")
                .join(format!("{}.json", third.snapshot_id)),
            |record| record["unexpected_future_authority"] = serde_json::json!(true),
        );

        let selection = store.latest_selection().expect("safe fallback selection");
        assert_eq!(selection.latest_refs, Some(first));
        assert_eq!(selection.skipped_newer_candidates.len(), 2);
        assert!(selection
            .skipped_newer_candidates
            .iter()
            .all(|warning| warning.warning_class
                == SessionRestoreSelectionWarningClass::InvalidJoinedCapture));
        assert!(store
            .load_window_topology_snapshot(&second.snapshot_id)
            .is_err());
        assert!(store.load_pane_tree_body(&third.snapshot_id).is_err());
    }

    #[test]
    fn invalid_closed_vocabulary_and_cross_layer_trigger_scopes_fail_selection() {
        let dir = tempfile::tempdir().expect("tempdir");
        let groups = || {
            vec![TabGroupCaptureInput {
                group_id: "group:first".to_string(),
                ordered_tabs: vec![tab(
                    "tab:a",
                    false,
                    SurfaceRole::Editor,
                    SurfaceClass::TextEditor,
                )],
                active_tab_id: Some("tab:a".to_string()),
            }]
        };
        let mut store = SessionRestoreStore::new(dir.path());
        let first = store
            .capture(capture_input(groups(), Some("group:first")))
            .expect("first capture");
        let second = store
            .capture(capture_input(groups(), Some("group:first")))
            .expect("second capture");
        let third = store
            .capture(capture_input(groups(), Some("group:first")))
            .expect("third capture");

        rewrite_json_value(
            &store
                .root_path()
                .join("workspace_authority_checkpoints")
                .join(format!("{}.json", second.checkpoint_id)),
            |record| {
                record["trusted_root_refs"] = serde_json::json!([{
                    "root_id": "root:test",
                    "trust_state": "secretly_trusted",
                    "scope_ref": "scope:test"
                }]);
            },
        );
        rewrite_json_value(
            &store
                .root_path()
                .join("window_topology_snapshots")
                .join(format!("{}.json", third.snapshot_id)),
            |record| {
                record["downgrade_triggers"] = serde_json::json!([{
                    "trigger_class": "policy_narrowing",
                    "affected_root_refs": ["root:test"]
                }]);
            },
        );

        let selection = store.latest_selection().expect("safe fallback selection");
        assert_eq!(selection.latest_refs, Some(first));
        assert_eq!(selection.skipped_newer_candidates.len(), 2);
        assert!(selection
            .skipped_newer_candidates
            .iter()
            .all(|warning| warning.warning_class
                == SessionRestoreSelectionWarningClass::InvalidJoinedCapture));
    }

    #[test]
    fn mismatched_authority_and_topology_downgrade_triggers_fail_selection() {
        let dir = tempfile::tempdir().expect("tempdir");
        let groups = vec![TabGroupCaptureInput {
            group_id: "group:first".to_string(),
            ordered_tabs: vec![tab(
                "tab:a",
                false,
                SurfaceRole::Editor,
                SurfaceClass::TextEditor,
            )],
            active_tab_id: Some("tab:a".to_string()),
        }];
        let mut input = capture_input(groups, Some("group:first"));
        input.downgrade_triggers = vec![DowngradeTriggerRecord {
            trigger_class: DowngradeTriggerClass::PolicyNarrowing,
            affected_journal_ids: None,
            affected_root_refs: None,
            affected_workset_ids: None,
            affected_pane_ids: None,
            note: Some("restore authority requires policy review".to_string()),
        }];
        let mut store = SessionRestoreStore::new(dir.path());
        let refs = store
            .capture(input)
            .expect("capture with downgrade trigger");

        rewrite_json_value(
            &store
                .root_path()
                .join("window_topology_snapshots")
                .join(format!("{}.json", refs.snapshot_id)),
            |record| record["downgrade_triggers"] = serde_json::json!([]),
        );

        let selection = store.latest_selection().expect("safe selection");
        assert!(selection.latest_refs.is_none());
        assert_eq!(selection.skipped_newer_candidates.len(), 1);
        assert_eq!(
            selection.skipped_newer_candidates[0].warning_class,
            SessionRestoreSelectionWarningClass::InvalidJoinedCapture
        );
    }

    #[test]
    fn capture_partitions_authority_and_window_trigger_scopes() {
        let dir = tempfile::tempdir().expect("tempdir");
        let groups = vec![TabGroupCaptureInput {
            group_id: "group:first".to_string(),
            ordered_tabs: vec![tab(
                "tab:a",
                false,
                SurfaceRole::Editor,
                SurfaceClass::TextEditor,
            )],
            active_tab_id: Some("tab:a".to_string()),
        }];
        let mut input = capture_input(groups, Some("group:first"));
        input.trusted_root_refs = vec![TrustedRootRecord {
            root_id: "root:test".to_string(),
            trust_state: "trusted".to_string(),
            scope_ref: "scope:test".to_string(),
            policy_epoch_ref: None,
            note: None,
        }];
        input.active_workset_ids = vec!["workset:test".to_string()];
        input.dirty_buffer_journal_identities = vec![DirtyBufferJournalIdentity {
            journal_id: "journal:test".to_string(),
            journal_kind: "dirty_buffer_recovery_journal".to_string(),
            last_known_revision_ref: "entry:test".to_string(),
            frame_count: Some(1),
            note: None,
        }];
        input.downgrade_triggers = vec![DowngradeTriggerRecord {
            trigger_class: super::super::records::DowngradeTriggerClass::PolicyNarrowing,
            affected_journal_ids: Some(vec!["journal:test".to_string()]),
            affected_root_refs: Some(vec!["root:test".to_string()]),
            affected_workset_ids: Some(vec!["workset:test".to_string()]),
            affected_pane_ids: Some(vec!["pane:tab:a".to_string()]),
            note: Some("typed trigger scope partition".to_string()),
        }];
        let mut store = SessionRestoreStore::new(dir.path());
        let refs = store.capture(input).expect("partitioned capture");
        let checkpoint = store
            .load_checkpoint(&refs.checkpoint_id)
            .expect("checkpoint");
        let snapshot = store
            .load_window_topology_snapshot(&refs.snapshot_id)
            .expect("snapshot");

        let checkpoint_trigger = &checkpoint.downgrade_triggers[0];
        assert_eq!(
            checkpoint_trigger.affected_journal_ids.as_deref(),
            Some(&["journal:test".to_string()][..])
        );
        assert!(checkpoint_trigger.affected_root_refs.is_some());
        assert!(checkpoint_trigger.affected_workset_ids.is_some());
        assert!(checkpoint_trigger.affected_pane_ids.is_none());

        let topology_trigger = &snapshot.downgrade_triggers[0];
        assert!(topology_trigger.affected_journal_ids.is_none());
        assert!(topology_trigger.affected_root_refs.is_none());
        assert!(topology_trigger.affected_workset_ids.is_none());
        assert_eq!(
            topology_trigger.affected_pane_ids.as_deref(),
            Some(&["pane:tab:a".to_string()][..])
        );
    }

    #[test]
    fn topology_restore_class_may_narrow_but_never_broaden_authority() {
        let dir = tempfile::tempdir().expect("tempdir");
        let groups = vec![TabGroupCaptureInput {
            group_id: "group:first".to_string(),
            ordered_tabs: vec![tab(
                "tab:a",
                false,
                SurfaceRole::Editor,
                SurfaceClass::TextEditor,
            )],
            active_tab_id: Some("tab:a".to_string()),
        }];
        let mut store = SessionRestoreStore::new(dir.path());
        let refs = store
            .capture(capture_input(groups, Some("group:first")))
            .expect("capture");
        let snapshot_path = store
            .root_path()
            .join("window_topology_snapshots")
            .join(format!("{}.json", refs.snapshot_id));
        rewrite_json_value(&snapshot_path, |record| {
            record["restore_class"] = serde_json::json!("recovered_drafts")
        });
        assert_eq!(
            store
                .latest_selection()
                .expect("recovered-draft broadening refusal")
                .latest_refs,
            None,
            "topology cannot invent recovered-draft authority above layout-only authority"
        );
        rewrite_json_value(&snapshot_path, |record| {
            record["restore_class"] = serde_json::json!("layout_only")
        });
        rewrite_json_value(
            &store
                .root_path()
                .join("workspace_authority_checkpoints")
                .join(format!("{}.json", refs.checkpoint_id)),
            |record| record["restore_class"] = serde_json::json!("compatible_restore"),
        );
        assert_eq!(
            store.latest_refs().expect("narrow topology selection"),
            Some(refs.clone()),
            "layout-only topology is a valid narrowing of compatible authority"
        );
        assert_eq!(
            store
                .latest_summary()
                .expect("narrow topology summary")
                .expect("summary")
                .restore_class,
            RestoreClass::LayoutOnly
        );

        rewrite_json_value(&snapshot_path, |record| {
            record["restore_class"] = serde_json::json!("exact_restore")
        });
        let selection = store.latest_selection().expect("broader topology refusal");
        assert_eq!(selection.latest_refs, None);
        assert_eq!(
            selection.skipped_newer_candidates,
            vec![SessionRestoreSelectionWarning {
                snapshot_id: refs.snapshot_id,
                warning_class: SessionRestoreSelectionWarningClass::InvalidJoinedCapture,
            }]
        );
    }

    #[test]
    fn legacy_advisory_and_unindexed_bodies_are_not_selection_authority() {
        let dir = tempfile::tempdir().expect("tempdir");
        let groups = || {
            vec![TabGroupCaptureInput {
                group_id: "group:first".to_string(),
                ordered_tabs: vec![tab(
                    "tab:a",
                    false,
                    SurfaceRole::Editor,
                    SurfaceClass::TextEditor,
                )],
                active_tab_id: Some("tab:a".to_string()),
            }]
        };
        let mut store = SessionRestoreStore::new(dir.path());
        let first = store
            .capture(capture_input(groups(), Some("group:first")))
            .expect("first capture");
        let second = store
            .capture(capture_input(groups(), Some("group:first")))
            .expect("second capture");
        for entry in std::fs::read_dir(store.root_path().join("latest_indices"))
            .expect("read versioned indices")
        {
            std::fs::remove_file(entry.expect("versioned index entry").path())
                .expect("remove versioned index body");
        }
        let torn = LatestIndexRecord {
            record_kind: "session_restore_latest_index".to_string(),
            latest_index_schema_version: 1,
            checkpoint_id: first.checkpoint_id,
            snapshot_id: second.snapshot_id.clone(),
            emitted_at: "mono:test:torn".to_string(),
        };
        std::fs::write(
            store.root_path().join("latest.json"),
            serde_json::to_vec_pretty(&torn).expect("serialize torn pointer"),
        )
        .expect("write torn pointer");

        assert_eq!(
            store.latest_refs().expect("advisory ignored"),
            None,
            "a fully written body without an immutable versioned index may be interrupted"
        );
    }

    #[test]
    fn public_loaders_reject_traversal_control_and_oversized_ids_without_leaking_paths() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = SessionRestoreStore::new(dir.path());
        let invalid_ids = vec![
            "../../private/secret".to_string(),
            "snap-1-1\n../../secret".to_string(),
            format!("snap-{}-1", "9".repeat(256)),
        ];

        for invalid in invalid_ids {
            for err in [
                store
                    .load_checkpoint(&invalid)
                    .expect_err("checkpoint traversal must fail"),
                store
                    .load_window_topology_snapshot(&invalid)
                    .expect_err("snapshot traversal must fail"),
                store
                    .load_pane_tree_body(&invalid)
                    .expect_err("pane-tree traversal must fail"),
            ] {
                let SessionRestoreError::MissingRecord(detail) = err else {
                    panic!("invalid caller id must return a non-leaking lookup error")
                };
                assert!(!detail.contains(&invalid));
                assert!(!detail.contains("../"));
                assert!(!detail.contains(dir.path().to_string_lossy().as_ref()));
            }
        }
    }

    #[test]
    fn immutable_atomic_publication_never_replaces_an_existing_destination() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("immutable.json");
        let first = LatestIndexRecord {
            record_kind: "session_restore_latest_index".to_string(),
            latest_index_schema_version: 1,
            checkpoint_id: "ckpt-00000000000000000001-00000000000000000001".to_string(),
            snapshot_id: "snap-00000000000000000001-00000000000000000001".to_string(),
            emitted_at: "mono:test:first".to_string(),
        };
        let mut second = first.clone();
        second.checkpoint_id = "ckpt-00000000000000000002-00000000000000000002".to_string();
        second.snapshot_id = "snap-00000000000000000002-00000000000000000002".to_string();

        write_new_json_atomically(&path, &first).expect("publish first immutable record");
        let err = write_new_json_atomically(&path, &second)
            .expect_err("existing destination must never be replaced");
        assert!(matches!(
            err,
            SessionRestoreError::Io(ref io_err)
                if io_err.kind() == std::io::ErrorKind::AlreadyExists
        ));
        assert_eq!(
            read_json::<LatestIndexRecord>(&path).expect("read immutable record"),
            first
        );
    }

    #[test]
    fn post_install_failpoints_preserve_the_installed_record() {
        let dir = tempfile::tempdir().expect("tempdir");
        let record = LatestIndexRecord {
            record_kind: "session_restore_latest_index".to_string(),
            latest_index_schema_version: 1,
            checkpoint_id: "ckpt-00000000000000000001-00000000000000000001".to_string(),
            snapshot_id: "snap-00000000000000000001-00000000000000000001".to_string(),
            emitted_at: "mono:test:installed".to_string(),
        };

        for (case, failpoint) in [
            ("after-hard-link", RecoveryIoFailpoint::AfterHardLink),
            (
                "before-directory-sync",
                RecoveryIoFailpoint::BeforeDirectorySync,
            ),
        ] {
            let path = dir.path().join(case).join("record.json");
            let guard = inject_recovery_io_failure(failpoint);
            let outcome =
                write_new_json_atomically(&path, &record).expect("install reaches commit point");
            drop(guard);

            assert_eq!(outcome, PublicationOutcome::CommitStateUncertain);
            assert!(
                fs::metadata(&path)
                    .expect("installed record metadata")
                    .len()
                    > 0,
                "post-install failure must never scrub committed bytes"
            );
            assert_eq!(
                read_json::<LatestIndexRecord>(&path).expect("installed record remains readable"),
                record
            );
            #[cfg(unix)]
            assert!(
                fs::read_dir(path.parent().expect("record parent"))
                    .expect("inspect publication directory")
                    .filter_map(Result::ok)
                    .all(|entry| !entry
                        .file_name()
                        .to_string_lossy()
                        .starts_with(".aureline-recovery-tmp-")),
                "owned temporary aliases are cleaned after post-install uncertainty"
            );
        }
    }

    #[test]
    fn link_error_after_destination_install_is_uncertain_without_staging_leak() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("record.json");
        let record = LatestIndexRecord {
            record_kind: "session_restore_latest_index".to_string(),
            latest_index_schema_version: 1,
            checkpoint_id: "ckpt-00000000000000000001-00000000000000000001".to_string(),
            snapshot_id: "snap-00000000000000000001-00000000000000000001".to_string(),
            emitted_at: "mono:test:installed-before-error".to_string(),
        };

        let guard =
            inject_recovery_io_failure(RecoveryIoFailpoint::HardLinkReportedErrorAfterInstall);
        let outcome = write_new_json_atomically(&path, &record)
            .expect("visible installed inode is a commit-state uncertainty");
        drop(guard);

        assert_eq!(outcome, PublicationOutcome::CommitStateUncertain);
        assert_eq!(
            read_json::<LatestIndexRecord>(&path).expect("installed target remains readable"),
            record
        );
        #[cfg(unix)]
        assert!(
            fs::read_dir(path.parent().expect("record parent"))
                .expect("inspect publication directory")
                .filter_map(Result::ok)
                .all(|entry| !entry
                    .file_name()
                    .to_string_lossy()
                    .starts_with(".aureline-recovery-tmp-")),
            "owned staging alias must not accumulate after uncertain install"
        );
    }

    #[cfg(unix)]
    #[test]
    fn replaced_staged_alias_is_never_deleted_as_owned_cleanup() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("record.json");
        let record = LatestIndexRecord {
            record_kind: "session_restore_latest_index".to_string(),
            latest_index_schema_version: 1,
            checkpoint_id: "ckpt-00000000000000000001-00000000000000000001".to_string(),
            snapshot_id: "snap-00000000000000000001-00000000000000000001".to_string(),
            emitted_at: "mono:test:installed".to_string(),
        };
        let mut replaced_path = None;
        let outcome = write_new_json_with_hooks(
            &path,
            &record,
            |_| {},
            |parent| {
                let staged = fs::read_dir(parent)?
                    .filter_map(Result::ok)
                    .map(|entry| entry.path())
                    .find(|candidate| {
                        candidate.file_name().is_some_and(|name| {
                            name.to_string_lossy()
                                .starts_with(".aureline-recovery-tmp-")
                        })
                    })
                    .ok_or_else(|| path_integrity_error("staged alias is unavailable"))?;
                fs::rename(&staged, parent.join("held-original-stage"))?;
                fs::write(&staged, b"replacement must survive")?;
                replaced_path = Some(staged);
                Ok(())
            },
        )
        .expect("installed destination remains reconcilable");

        assert_eq!(outcome, PublicationOutcome::CommitStateUncertain);
        let replaced_path = replaced_path.expect("replacement path captured");
        assert_eq!(
            fs::read(&replaced_path).expect("replacement remains present"),
            b"replacement must survive"
        );
        assert_eq!(
            read_json::<LatestIndexRecord>(&path).expect("installed target remains original"),
            record
        );
    }

    #[test]
    fn capture_reports_installed_checkpoint_as_uncertain_not_precommit_failure() {
        let dir = tempfile::tempdir().expect("tempdir");
        let groups = vec![TabGroupCaptureInput {
            group_id: "group:first".to_string(),
            ordered_tabs: vec![tab(
                "tab:a",
                false,
                SurfaceRole::Editor,
                SurfaceClass::TextEditor,
            )],
            active_tab_id: Some("tab:a".to_string()),
        }];
        let mut store = SessionRestoreStore::new(dir.path());
        let guard = inject_recovery_io_failure(RecoveryIoFailpoint::AfterHardLink);
        let error = store
            .capture(capture_input(groups, Some("group:first")))
            .expect_err("post-install failure must be explicit");
        drop(guard);

        let SessionRestoreError::CommitStateUncertain(refs) = error else {
            panic!("installed bytes must not be reported as an ordinary error")
        };
        let checkpoint_path = store
            .root_path()
            .join("workspace_authority_checkpoints")
            .join(format!("{}.json", refs.checkpoint_id));
        let checkpoint: WorkspaceAuthorityCheckpointRecord =
            read_json(&checkpoint_path).expect("installed checkpoint survives uncertainty");
        assert_eq!(checkpoint.checkpoint_id, refs.checkpoint_id);
        assert!(
            fs::metadata(checkpoint_path)
                .expect("checkpoint metadata")
                .len()
                > 0
        );
    }

    #[test]
    fn capture_reopens_exact_join_before_reporting_success() {
        let dir = tempfile::tempdir().expect("tempdir");
        let groups = vec![TabGroupCaptureInput {
            group_id: "group:first".to_string(),
            ordered_tabs: vec![tab(
                "tab:a",
                false,
                SurfaceRole::Editor,
                SurfaceClass::TextEditor,
            )],
            active_tab_id: Some("tab:a".to_string()),
        }];
        let mut store = SessionRestoreStore::new(dir.path());
        let guard = inject_recovery_io_failure(RecoveryIoFailpoint::BeforeCaptureValidation);
        let error = store
            .capture(capture_input(groups, Some("group:first")))
            .expect_err("skipped final reopen must not report capture success");
        drop(guard);

        let SessionRestoreError::CommitStateUncertain(refs) = error else {
            panic!("final validation failure must retain the exact capture refs")
        };
        let reopened = SessionRestoreStore::new(dir.path());
        assert!(
            reopened
                .reconcile_capture(&refs)
                .expect("exact reconciliation"),
            "the exact indexed join is durable despite the uncertain return"
        );
        assert_eq!(
            reopened.latest_refs().expect("reopened joined refs"),
            Some(refs),
            "the failpoint fires only after every exact member and index is durable"
        );
    }

    #[test]
    fn recovery_directory_enumeration_is_bounded() {
        let dir = tempfile::tempdir().expect("tempdir");
        let records = dir.path().join("records");
        fs::create_dir(&records).expect("records directory");
        for index in 0..5 {
            fs::write(records.join(format!("{index}.json")), b"{}").expect("directory entry");
        }

        assert!(matches!(
            bounded_directory_entries_with_limit(&records, 4),
            Err(SessionRestoreError::CorruptStore(
                "recovery directory exceeds the entry limit"
            ))
        ));
    }

    #[cfg(unix)]
    #[test]
    fn ancestor_redirect_is_rejected_without_writing_through_it() {
        use std::os::unix::fs::symlink;

        let dir = tempfile::tempdir().expect("tempdir");
        let outside = dir.path().join("outside");
        let alias = dir.path().join("state-alias");
        fs::create_dir(&outside).expect("outside directory");
        symlink(&outside, &alias).expect("redirected state root");

        let groups = vec![TabGroupCaptureInput {
            group_id: "group:first".to_string(),
            ordered_tabs: vec![tab(
                "tab:a",
                false,
                SurfaceRole::Editor,
                SurfaceClass::TextEditor,
            )],
            active_tab_id: Some("tab:a".to_string()),
        }];
        let mut store = SessionRestoreStore::new(&alias);
        let error = store
            .capture(capture_input(groups, Some("group:first")))
            .expect_err("redirected ancestor must fail closed");

        assert!(matches!(
            error,
            SessionRestoreError::Io(ref io_error)
                if io_error.kind() == io::ErrorKind::InvalidData
        ));
        assert!(
            !outside.join("session_restore").exists(),
            "capture must not create through an untrusted ancestor"
        );
    }

    #[cfg(unix)]
    #[test]
    fn parent_swap_before_install_cannot_publish_outside_the_pinned_directory() {
        use std::os::unix::fs::symlink;

        let dir = tempfile::tempdir().expect("tempdir");
        let parent = dir.path().join("state");
        let moved_parent = dir.path().join("state-pinned");
        let outside = dir.path().join("outside");
        fs::create_dir(&outside).expect("outside directory");
        let path = parent.join("record.json");
        let record = LatestIndexRecord {
            record_kind: "session_restore_latest_index".to_string(),
            latest_index_schema_version: 1,
            checkpoint_id: "ckpt-00000000000000000001-00000000000000000001".to_string(),
            snapshot_id: "snap-00000000000000000001-00000000000000000001".to_string(),
            emitted_at: "mono:test:race".to_string(),
        };

        let error = write_new_json_with_hooks(
            &path,
            &record,
            |resolved_parent| {
                fs::rename(resolved_parent, &moved_parent).expect("move pinned parent");
                symlink(&outside, resolved_parent).expect("replace parent with redirect");
            },
            |_| Ok(()),
        )
        .expect_err("parent replacement must invalidate publication");

        assert!(matches!(
            error,
            SessionRestoreError::Io(ref io_error)
                if io_error.kind() == io::ErrorKind::InvalidData
        ));
        assert!(!outside.join("record.json").exists());
        for entry in fs::read_dir(&moved_parent).expect("moved parent remains inspectable") {
            let entry = entry.expect("staged entry");
            assert_eq!(
                entry.metadata().expect("staged metadata").len(),
                0,
                "abandoned staged content must be scrubbed through its open handle"
            );
        }
    }

    #[cfg(unix)]
    #[test]
    fn bounded_read_detects_same_path_replacement_after_read() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("record.json");
        let replacement = dir.path().join("replacement.json");
        fs::write(&path, br#"{"value":"first"}"#).expect("first record");
        fs::write(&replacement, br#"{"value":"other"}"#).expect("replacement record");

        let error = read_json_with_post_read_hook::<serde_json::Value, _>(&path, |resolved| {
            fs::rename(&replacement, resolved).expect("replace record after bounded read");
        })
        .expect_err("path replacement must invalidate the read");

        assert!(matches!(
            error,
            SessionRestoreError::Io(ref io_error)
                if io_error.kind() == io::ErrorKind::InvalidData
        ));
    }

    #[test]
    fn windows_reparse_attribute_is_classified_as_redirect() {
        assert!(windows_attributes_include_reparse_point(0x400));
        assert!(windows_attributes_include_reparse_point(0x400 | 0x10));
        assert!(!windows_attributes_include_reparse_point(0x10));
    }

    #[test]
    fn oversized_recovery_record_is_rejected_before_json_parsing() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = SessionRestoreStore::new(dir.path());
        let checkpoint_id = "ckpt-00000000000000000001-00000000000000000001";
        let path = store
            .root_path()
            .join("workspace_authority_checkpoints")
            .join(format!("{checkpoint_id}.json"));
        std::fs::create_dir_all(path.parent().expect("checkpoint parent"))
            .expect("create checkpoint directory");
        std::fs::write(&path, vec![b'x'; MAX_RECOVERY_RECORD_BYTES as usize + 1])
            .expect("write oversized hostile record");

        assert!(matches!(
            read_json::<serde_json::Value>(&path),
            Err(SessionRestoreError::CorruptStore(_))
        ));
        let err = store
            .load_checkpoint(checkpoint_id)
            .expect_err("public loader must fail closed");
        let SessionRestoreError::MissingRecord(detail) = err else {
            panic!("public loader must return a non-leaking missing-record error")
        };
        assert!(!detail.contains(dir.path().to_string_lossy().as_ref()));
    }

    #[cfg(unix)]
    #[test]
    fn symlinked_recovery_record_is_rejected() {
        use std::os::unix::fs::symlink;

        let dir = tempfile::tempdir().expect("tempdir");
        let store = SessionRestoreStore::new(dir.path());
        let checkpoint_id = "ckpt-00000000000000000001-00000000000000000001";
        let outside = dir.path().join("outside.json");
        std::fs::write(&outside, b"{}").expect("write symlink target");
        let path = store
            .root_path()
            .join("workspace_authority_checkpoints")
            .join(format!("{checkpoint_id}.json"));
        std::fs::create_dir_all(path.parent().expect("checkpoint parent"))
            .expect("create checkpoint directory");
        symlink(&outside, &path).expect("create hostile symlink");

        assert!(matches!(
            read_json::<serde_json::Value>(&path),
            Err(SessionRestoreError::Io(ref error))
                if error.kind() == io::ErrorKind::InvalidData
        ));
        assert!(matches!(
            store.load_checkpoint(checkpoint_id),
            Err(SessionRestoreError::MissingRecord(_))
        ));
    }

    #[test]
    fn capture_bounds_fail_closed_before_persistence() {
        let dir = tempfile::tempdir().expect("tempdir");
        let mut store = SessionRestoreStore::new(dir.path());

        let mut oversized_label = capture_input(
            vec![TabGroupCaptureInput {
                group_id: "group:first".to_string(),
                ordered_tabs: vec![tab(
                    "tab:a",
                    false,
                    SurfaceRole::Editor,
                    SurfaceClass::TextEditor,
                )],
                active_tab_id: Some("tab:a".to_string()),
            }],
            Some("group:first"),
        );
        oversized_label.tab_groups[0].ordered_tabs[0].tab_label =
            Some("x".repeat(MAX_CAPTURE_LABEL_BYTES + 1));
        assert!(matches!(
            store.capture(oversized_label),
            Err(SessionRestoreError::InvalidCapture(_))
        ));

        let groups = (0..=MAX_CAPTURE_GROUPS)
            .map(|idx| TabGroupCaptureInput {
                group_id: format!("group:{idx}"),
                ordered_tabs: vec![tab(
                    &format!("tab:{idx}"),
                    false,
                    SurfaceRole::Editor,
                    SurfaceClass::TextEditor,
                )],
                active_tab_id: Some(format!("tab:{idx}")),
            })
            .collect();
        assert!(matches!(
            store.capture(capture_input(groups, None)),
            Err(SessionRestoreError::InvalidCapture(_))
        ));

        let base_groups = vec![TabGroupCaptureInput {
            group_id: "group:first".to_string(),
            ordered_tabs: vec![tab(
                "tab:a",
                false,
                SurfaceRole::Editor,
                SurfaceClass::TextEditor,
            )],
            active_tab_id: Some("tab:a".to_string()),
        }];
        let mut deep = TabGroupLayoutCapture::TabGroup {
            group_id: "group:first".to_string(),
        };
        for idx in 0..MAX_CAPTURE_LAYOUT_DEPTH {
            deep = TabGroupLayoutCapture::Split {
                split_id: format!("split:{idx}"),
                orientation: SplitOrientation::Vertical,
                children: vec![
                    TabGroupLayoutCapture::TabGroup {
                        group_id: "group:first".to_string(),
                    },
                    deep,
                ],
                weights: None,
            };
        }
        let mut deep_input = capture_input(base_groups.clone(), Some("group:first"));
        deep_input.pane_tree_layout = Some(deep);
        assert!(matches!(
            store.capture(deep_input),
            Err(SessionRestoreError::InvalidCapture(_))
        ));

        let mut excessive_refs = capture_input(base_groups, Some("group:first"));
        excessive_refs.active_workset_ids = (0..=MAX_CAPTURE_REF_LIST_ITEMS)
            .map(|idx| format!("workset:{idx}"))
            .collect();
        assert!(matches!(
            store.capture(excessive_refs),
            Err(SessionRestoreError::InvalidCapture(_))
        ));
        assert!(!store.root_path().exists());
    }

    #[test]
    fn invalid_authority_vocabulary_and_duplicates_fail_before_publication() {
        let groups = || {
            vec![TabGroupCaptureInput {
                group_id: "group:first".to_string(),
                ordered_tabs: vec![tab(
                    "tab:a",
                    false,
                    SurfaceRole::Editor,
                    SurfaceClass::TextEditor,
                )],
                active_tab_id: Some("tab:a".to_string()),
            }]
        };

        let mut invalid_trust = capture_input(groups(), Some("group:first"));
        invalid_trust.trusted_root_refs = vec![TrustedRootRecord {
            root_id: "root:test".to_string(),
            trust_state: "unknown".to_string(),
            scope_ref: "scope:test".to_string(),
            policy_epoch_ref: None,
            note: None,
        }];

        let mut invalid_journal = capture_input(groups(), Some("group:first"));
        invalid_journal.dirty_buffer_journal_identities = vec![DirtyBufferJournalIdentity {
            journal_id: "journal:test".to_string(),
            journal_kind: "arbitrary_stream".to_string(),
            last_known_revision_ref: "entry:test".to_string(),
            frame_count: Some(1),
            note: None,
        }];

        let mut duplicate_refs = capture_input(groups(), Some("group:first"));
        duplicate_refs.active_workset_ids =
            vec!["workset:test".to_string(), "workset:test".to_string()];

        let mut duplicate_classes = capture_input(groups(), Some("group:first"));
        duplicate_classes.excluded_live_authority_classes = vec![
            ExcludedLiveAuthorityClass::RawSecretMaterial,
            ExcludedLiveAuthorityClass::RawSecretMaterial,
        ];

        let mut duplicate_trigger_scope = capture_input(groups(), Some("group:first"));
        duplicate_trigger_scope.downgrade_triggers = vec![DowngradeTriggerRecord {
            trigger_class: DowngradeTriggerClass::ManualRepairRequired,
            affected_journal_ids: None,
            affected_root_refs: Some(vec!["root:test".to_string(), "root:test".to_string()]),
            affected_workset_ids: None,
            affected_pane_ids: None,
            note: None,
        }];

        let dir = tempfile::tempdir().expect("tempdir");
        let mut store = SessionRestoreStore::new(dir.path());
        for input in [
            invalid_trust,
            invalid_journal,
            duplicate_refs,
            duplicate_classes,
            duplicate_trigger_scope,
        ] {
            assert!(matches!(
                store.capture(input),
                Err(SessionRestoreError::InvalidCapture(_))
            ));
            assert!(!store.root_path().exists());
        }
    }

    #[test]
    fn spliced_body_lineage_is_not_selected_as_latest() {
        let dir = tempfile::tempdir().expect("tempdir");
        let groups = || {
            vec![TabGroupCaptureInput {
                group_id: "group:first".to_string(),
                ordered_tabs: vec![tab(
                    "tab:a",
                    false,
                    SurfaceRole::Editor,
                    SurfaceClass::TextEditor,
                )],
                active_tab_id: Some("tab:a".to_string()),
            }]
        };
        let mut store = SessionRestoreStore::new(dir.path());
        let first = store
            .capture(capture_input(groups(), Some("group:first")))
            .expect("first capture");
        let mut second_input = capture_input(groups(), Some("group:first"));
        second_input.emitted_at = "mono:test:2".to_string();
        let second = store.capture(second_input).expect("second capture");
        let mut spliced_body = store
            .load_pane_tree_body(&second.snapshot_id)
            .expect("second body");
        spliced_body.emitted_at = "mono:test:1".to_string();
        std::fs::write(
            store
                .root_path()
                .join("pane_tree_bodies")
                .join(format!("{}.json", second.snapshot_id)),
            serde_json::to_vec_pretty(&spliced_body).expect("serialize spliced body"),
        )
        .expect("write spliced body");

        assert_eq!(
            store.latest_refs().expect("lineage-aware fallback"),
            Some(first)
        );
    }

    #[test]
    fn side_effectful_and_missing_surfaces_are_never_marked_ready() {
        for (role, class) in [
            (SurfaceRole::Terminal, SurfaceClass::TerminalView),
            (SurfaceRole::Debugger, SurfaceClass::DebugView),
            (SurfaceRole::Notebook, SurfaceClass::NotebookView),
            (SurfaceRole::Placeholder, SurfaceClass::PlaceholderCard),
        ] {
            assert_eq!(
                restore_posture_for_surface(role, class),
                (
                    HydrationBehavior::PlaceholderOnly,
                    AvailabilityState::Placeholder
                )
            );
        }
        assert_eq!(
            restore_posture_for_surface(SurfaceRole::Editor, SurfaceClass::TextEditor),
            (
                HydrationBehavior::EagerLightweight,
                AvailabilityState::Ready
            )
        );

        let dir = tempfile::tempdir().expect("tempdir");
        let mut terminal = tab(
            "tab:terminal",
            false,
            SurfaceRole::Terminal,
            SurfaceClass::TerminalView,
        );
        terminal.restore_metadata = Some(TerminalPaneRestoreMetadata {
            restore_metadata_ref: "terminal-restore-metadata:test".to_string(),
            working_directory: Some("service".to_string()),
            environment_scope_token: "workspace".to_string(),
            shell_identity: "zsh".to_string(),
            shell_family_token: "zsh".to_string(),
            last_command_class_token: "build".to_string(),
            auto_rerun_forbidden: true,
            raw_command_body_present: false,
            raw_environment_body_present: false,
        });
        let groups = vec![TabGroupCaptureInput {
            group_id: "group:first".to_string(),
            ordered_tabs: vec![
                tab(
                    "tab:editor",
                    false,
                    SurfaceRole::Editor,
                    SurfaceClass::TextEditor,
                ),
                terminal,
                tab(
                    "tab:missing",
                    false,
                    SurfaceRole::Placeholder,
                    SurfaceClass::PlaceholderCard,
                ),
            ],
            active_tab_id: Some("tab:editor".to_string()),
        }];
        let mut store = SessionRestoreStore::new(dir.path());
        let refs = store
            .capture(capture_input(groups, Some("group:first")))
            .expect("capture placeholder topology");
        let snapshot = store
            .load_window_topology_snapshot(&refs.snapshot_id)
            .expect("load placeholder inventory");
        assert_eq!(snapshot.placeholder_behaviors.len(), 2);
        let terminal_behavior = snapshot
            .placeholder_behaviors
            .iter()
            .find(|behavior| {
                snapshot
                    .stable_pane_id_inventory
                    .iter()
                    .find(|pane| pane.pane_id == behavior.pane_id)
                    .is_some_and(|pane| pane.surface_role == SurfaceRole::Terminal)
            })
            .expect("terminal placeholder behavior");
        assert_eq!(
            terminal_behavior.placeholder_reason,
            PlaceholderReasonClass::NonReentrantLiveSurface
        );
        assert_eq!(
            terminal_behavior.safe_actions.as_slice(),
            required_placeholder_actions(PlaceholderReasonClass::NonReentrantLiveSurface)
        );
        let terminal_pane_id = terminal_behavior.pane_id.clone();
        let missing_behavior = snapshot
            .placeholder_behaviors
            .iter()
            .find(|behavior| {
                snapshot
                    .stable_pane_id_inventory
                    .iter()
                    .find(|pane| pane.pane_id == behavior.pane_id)
                    .is_some_and(|pane| pane.surface_role == SurfaceRole::Placeholder)
            })
            .expect("missing-surface placeholder behavior");
        assert_eq!(
            missing_behavior.placeholder_reason,
            PlaceholderReasonClass::ManualRecoveryRequired
        );
        assert_eq!(
            missing_behavior.safe_actions.as_slice(),
            required_placeholder_actions(PlaceholderReasonClass::ManualRecoveryRequired)
        );

        let body = store
            .load_pane_tree_body(&refs.snapshot_id)
            .expect("load placeholder cards");
        let mut injected_snapshot = snapshot.clone();
        injected_snapshot
            .placeholder_behaviors
            .iter_mut()
            .find(|behavior| behavior.pane_id == terminal_pane_id)
            .expect("terminal behavior for injection probe")
            .safe_actions
            .push(PlaceholderAction::RetryHydrate);
        let mut injected_body = body.clone();
        let PaneNode::TabGroup { tabs, .. } = &mut injected_body.pane_tree.root_node else {
            panic!("single group remains a tab group")
        };
        tabs.iter_mut()
            .find(|tab| tab.pane.pane_id == terminal_pane_id)
            .and_then(|tab| tab.pane.surface.placeholder_card.as_mut())
            .expect("terminal placeholder card for injection probe")
            .safe_actions
            .push(PlaceholderAction::RetryHydrate);
        assert!(
            !joined_topology_semantics_are_valid(&injected_snapshot, &injected_body),
            "a matching but non-canonical extra action must fail closed"
        );
        let mut embedded_metadata =
            serde_json::to_value(&body).expect("serialize pane tree for rejection probe");
        embedded_metadata["pane_tree"]["root_node"]["tabs"][1]["pane"]["surface"]
            ["restore_metadata"] = serde_json::json!({
            "restore_metadata_ref": "terminal-restore-metadata:legacy",
            "working_directory": "service",
            "environment_scope_token": "workspace",
            "shell_identity": "zsh",
            "shell_family_token": "zsh",
            "last_command_class_token": "build",
            "auto_rerun_forbidden": true,
            "raw_command_body_present": false,
            "raw_environment_body_present": false
        });
        assert!(
            serde_json::from_value::<WindowTopologySnapshotBodyRecord>(embedded_metadata).is_err(),
            "legacy embedded terminal metadata must fail closed"
        );
        let PaneNode::TabGroup { tabs, .. } = body.pane_tree.root_node else {
            panic!("single group remains a tab group")
        };
        assert!(tabs[0].pane.surface.placeholder_card.is_none());
        assert!(tabs[1].pane.surface.placeholder_card.is_some());
        assert!(tabs[2].pane.surface.placeholder_card.is_some());
        let persisted_snapshot = fs::read_to_string(
            store
                .root_path()
                .join("window_topology_snapshots")
                .join(format!("{}.json", refs.snapshot_id)),
        )
        .expect("read persisted topology packet");
        let persisted_body = fs::read_to_string(
            store
                .root_path()
                .join("pane_tree_bodies")
                .join(format!("{}.json", refs.snapshot_id)),
        )
        .expect("read persisted pane tree");
        assert!(!persisted_snapshot.contains("restore_metadata"));
        assert!(!persisted_body.contains("restore_metadata"));
    }
}
