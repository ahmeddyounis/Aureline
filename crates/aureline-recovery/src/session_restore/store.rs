// SPDX-FileCopyrightText: 2026 Aureline contributors
// SPDX-License-Identifier: Apache-2.0

use std::collections::{HashMap, HashSet};
use std::fs::{create_dir_all, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use super::records::{
    AvailabilityState, CheckpointSchemaVersion, DensityPreset, DirtyBufferJournalIdentity,
    DowngradeTriggerRecord, ExcludedLiveAuthorityClass, FocusChainEntry, FocusTargetKind,
    FollowMode, FollowPresentationState, HydrationBehavior, MonitorAffinityHint,
    MonitorAffinityStrength, PaneLeafNode, PaneNode, PaneSurfaceDescriptor, PaneTree,
    PaneTreeSchemaVersion, ProducerBuildStamp, RestoreClass, ScopeRefs, SnapshotReason,
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct LatestIndexRecord {
    record_kind: String,
    latest_index_schema_version: u32,
    checkpoint_id: String,
    snapshot_id: String,
    emitted_at: String,
}

/// File-backed store for session-restore skeleton artifacts.
#[derive(Debug, Clone)]
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
        let workspace_authority_ref = format!("workspace-authority:{}", input.workspace_ref);

        let restore_class = if input.dirty_buffer_journal_identities.is_empty() {
            RestoreClass::LayoutOnly
        } else {
            RestoreClass::RecoveredDrafts
        };

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
            downgrade_triggers: input.downgrade_triggers.clone(),
            rollback_checkpoint_ref: None,
            preserved_prior_artifact_refs: Vec::new(),
            emitted_at: input.emitted_at.clone(),
            notes: input.notes.clone(),
        };

        let (tab_group_topology, stable_pane_inventory, pane_tree_root, focus_chain) =
            materialize_topology_from_capture(
                &input.tab_groups,
                input.pane_tree_layout.as_ref(),
                input.focused_group_id.as_deref(),
                &snapshot_id,
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
            placeholder_behaviors: Vec::new(),
            topology_adjustments: Vec::new(),
            restore_class,
            downgrade_triggers: input.downgrade_triggers.clone(),
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

        write_new_json(
            &self
                .root
                .join("workspace_authority_checkpoints")
                .join(format!("{checkpoint_id}.json")),
            &checkpoint,
        )?;

        write_new_json(
            &self
                .root
                .join("window_topology_snapshots")
                .join(format!("{snapshot_id}.json")),
            &topology_packet,
        )?;

        write_new_json(
            &self
                .root
                .join("pane_tree_bodies")
                .join(format!("{snapshot_id}.json")),
            &pane_tree_body,
        )?;

        self.write_latest_index(&checkpoint_id, &snapshot_id, &input.emitted_at)?;

        Ok(SessionRestoreLatestRefs {
            checkpoint_id,
            snapshot_id,
        })
    }

    /// Loads the latest captured refs, if any.
    pub fn latest_refs(&self) -> Result<Option<SessionRestoreLatestRefs>, SessionRestoreError> {
        let (has_versioned_index, versioned_refs) = self.newest_versioned_index_refs()?;
        if let Some(refs) = versioned_refs {
            return Ok(Some(refs));
        }
        if has_versioned_index {
            return self.latest_valid_joined_refs();
        }

        let path = self.root.join("latest.json");
        match read_json::<LatestIndexRecord>(&path) {
            Ok(record) => {
                let refs = SessionRestoreLatestRefs {
                    checkpoint_id: record.checkpoint_id.clone(),
                    snapshot_id: record.snapshot_id.clone(),
                };
                if valid_latest_index_record(&record, &refs) && self.joined_refs_are_valid(&refs) {
                    return Ok(Some(refs));
                }
            }
            Err(SessionRestoreError::Io(err)) if err.kind() == std::io::ErrorKind::NotFound => {}
            Err(SessionRestoreError::Json(_) | SessionRestoreError::CorruptStore(_)) => {}
            Err(err) => return Err(err),
        }
        self.latest_valid_joined_refs()
    }

    /// Loads a summary for the latest captured snapshot.
    pub fn latest_summary(&self) -> Result<Option<SessionRestoreSummary>, SessionRestoreError> {
        let Some(latest) = self.latest_refs()? else {
            return Ok(None);
        };

        let checkpoint = self.load_checkpoint(&latest.checkpoint_id)?;
        let snapshot = self.load_window_topology_snapshot(&latest.snapshot_id)?;

        let tab_group_count = snapshot.tab_group_topology.len();
        let tab_count = snapshot
            .tab_group_topology
            .iter()
            .map(|group| group.ordered_tab_ids.len())
            .sum();

        Ok(Some(SessionRestoreSummary {
            restore_class: checkpoint.restore_class,
            checkpoint_id: latest.checkpoint_id,
            snapshot_id: latest.snapshot_id,
            window_id: snapshot.window_id,
            tab_group_count,
            tab_count,
            dirty_buffer_journal_count: checkpoint.dirty_buffer_journal_identities.len(),
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
        read_json(&checkpoint_path)
            .map_err(|_| SessionRestoreError::MissingRecord("checkpoint unavailable".to_string()))
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
        read_json(&snapshot_path)
            .map_err(|_| SessionRestoreError::MissingRecord("snapshot unavailable".to_string()))
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
        read_json(&body_path).map_err(|_| {
            SessionRestoreError::MissingRecord("pane tree body unavailable".to_string())
        })
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
    ) -> Result<(), SessionRestoreError> {
        if !self.root.exists() {
            create_dir_all(&self.root)?;
        }
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
        write_new_json_atomically(&versioned_path, &record)?;

        // `latest.json` is a legacy advisory pointer. Publish it once with
        // create-new semantics; immutable versioned indices remain canonical
        // so later captures never rely on platform-specific overwrite behavior.
        let advisory_path = self.root.join("latest.json");
        if !advisory_path.exists() {
            match write_new_json_atomically(&advisory_path, &record) {
                Ok(()) => {}
                Err(SessionRestoreError::Io(err))
                    if err.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(err) => return Err(err),
            }
        }
        Ok(())
    }

    fn newest_versioned_index_refs(
        &self,
    ) -> Result<(bool, Option<SessionRestoreLatestRefs>), SessionRestoreError> {
        let index_dir = self.root.join("latest_indices");
        let entries = match std::fs::read_dir(&index_dir) {
            Ok(entries) => entries,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok((false, None)),
            Err(err) => return Err(err.into()),
        };
        let mut candidates = Vec::new();
        for entry in entries {
            let entry = entry?;
            if !entry.file_type()?.is_file()
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
        let Some((_, indexed_snapshot_id, path)) = candidates.first() else {
            return Ok((false, None));
        };
        let Ok(record) = read_json::<LatestIndexRecord>(path) else {
            return Ok((true, None));
        };
        let refs = SessionRestoreLatestRefs {
            checkpoint_id: record.checkpoint_id.clone(),
            snapshot_id: record.snapshot_id.clone(),
        };
        if refs.snapshot_id == *indexed_snapshot_id
            && valid_latest_index_record(&record, &refs)
            && self.joined_refs_are_valid(&refs)
        {
            Ok((true, Some(refs)))
        } else {
            Ok((true, None))
        }
    }

    fn joined_refs_are_valid(&self, refs: &SessionRestoreLatestRefs) -> bool {
        if parse_durable_record_id(&refs.checkpoint_id, "ckpt").is_err()
            || parse_durable_record_id(&refs.snapshot_id, "snap").is_err()
        {
            return false;
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
        let Ok(checkpoint) = read_json::<WorkspaceAuthorityCheckpointRecord>(&checkpoint_path)
        else {
            return false;
        };
        let Ok(snapshot) = read_json::<WindowTopologySnapshotRecord>(&snapshot_path) else {
            return false;
        };
        let Ok(body) = read_json::<WindowTopologySnapshotBodyRecord>(&body_path) else {
            return false;
        };
        checkpoint.record_kind == "workspace_authority_checkpoint_record"
            && checkpoint.checkpoint_id == refs.checkpoint_id
            && snapshot.record_kind == "window_topology_snapshot_record"
            && snapshot.snapshot_id == refs.snapshot_id
            && snapshot.workspace_authority_checkpoint_ref == refs.checkpoint_id
            && snapshot.pane_tree_record_ref == refs.snapshot_id
            && snapshot.restore_class == checkpoint.restore_class
            && snapshot.source_schema_version == checkpoint.source_schema_version
            && snapshot.producer_build == checkpoint.producer_build
            && snapshot.emitted_at == checkpoint.emitted_at
            && body.record_kind == "window_topology_snapshot_record"
            && body.snapshot_id == refs.snapshot_id
            && body.window_id == snapshot.window_id
            && body.window_role == snapshot.window_role
            && body.topology_family_ref == snapshot.topology_family_ref
            && body.sibling_window_refs == snapshot.sibling_window_refs
            && body.pane_tree_schema_version == snapshot.pane_tree_schema_version
            && body.emitted_at == snapshot.emitted_at
            && body.scope_refs.workspace_authority_ref == checkpoint.workspace_authority_ref
    }

    fn latest_valid_joined_refs(
        &self,
    ) -> Result<Option<SessionRestoreLatestRefs>, SessionRestoreError> {
        let snapshot_dir = self.root.join("window_topology_snapshots");
        let entries = match std::fs::read_dir(&snapshot_dir) {
            Ok(entries) => entries,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(err) => return Err(err.into()),
        };
        let mut candidates = Vec::new();
        for entry in entries {
            let entry = entry?;
            if !entry.file_type()?.is_file()
                || entry.path().extension().and_then(|ext| ext.to_str()) != Some("json")
            {
                continue;
            }
            let file_name = entry.file_name();
            let file_name = file_name.to_str().ok_or(SessionRestoreError::CorruptStore(
                "snapshot filename is not UTF-8",
            ))?;
            let snapshot_id =
                file_name
                    .strip_suffix(".json")
                    .ok_or(SessionRestoreError::CorruptStore(
                        "snapshot filename has no JSON suffix",
                    ))?;
            let order = parse_durable_record_id(snapshot_id, "snap")?;
            candidates.push((order, snapshot_id.to_string()));
        }
        candidates.sort_by(|left, right| right.0.cmp(&left.0));

        for (_, snapshot_id) in candidates {
            let snapshot_path = snapshot_dir.join(format!("{snapshot_id}.json"));
            let Ok(snapshot) = read_json::<WindowTopologySnapshotRecord>(&snapshot_path) else {
                continue;
            };
            let refs = SessionRestoreLatestRefs {
                checkpoint_id: snapshot.workspace_authority_checkpoint_ref,
                snapshot_id,
            };
            if self.joined_refs_are_valid(&refs) {
                return Ok(Some(refs));
            }
        }
        Ok(None)
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
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(1),
        Err(err) => return Err(err.into()),
    };
    let mut max_sequence = 0_u64;
    for entry in entries {
        let entry = entry?;
        if !entry.file_type()?.is_file()
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

const MAX_CAPTURE_GROUPS: usize = 64;
const MAX_CAPTURE_TABS_PER_GROUP: usize = 256;
const MAX_CAPTURE_TOTAL_TABS: usize = 1_024;
const MAX_CAPTURE_REF_LIST_ITEMS: usize = 256;
const MAX_CAPTURE_DOWNGRADE_TRIGGERS: usize = 256;
const MAX_CAPTURE_LAYOUT_DEPTH: usize = 32;
const MAX_CAPTURE_LAYOUT_NODES: usize = 256;
const MAX_CAPTURE_LABEL_BYTES: usize = 1_024;
const MAX_CAPTURE_NOTE_BYTES: usize = 4_096;

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
    if !is_bounded_capture_text(&input.producer_build.producer_name, 256, false)
        || !is_bounded_capture_text(&input.producer_build.producer_version, 256, false)
        || input
            .producer_build
            .producer_channel
            .as_deref()
            .is_some_and(|channel| !matches!(channel, "experimental" | "beta" | "stable" | "lts"))
        || input
            .producer_build
            .producer_platform_class
            .as_deref()
            .is_some_and(|platform| {
                !matches!(
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
        || input
            .producer_build
            .producer_instance_handle
            .as_ref()
            .is_some_and(|handle| !is_bounded_opaque_ref(handle))
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
    if !is_bounded_opaque_ref(&input.workspace_ref) || !is_bounded_opaque_ref(&input.window_id) {
        return Err(SessionRestoreError::InvalidCapture(
            "workspace and window refs must be bounded opaque ids",
        ));
    }
    if input
        .topology_family_ref
        .as_ref()
        .is_some_and(|value| !is_bounded_opaque_ref(value))
        || input
            .sibling_window_refs
            .iter()
            .any(|value| !is_bounded_opaque_ref(value))
    {
        return Err(SessionRestoreError::InvalidCapture(
            "window topology refs must be bounded opaque ids",
        ));
    }
    if input.trusted_root_refs.iter().any(|root| {
        !is_bounded_opaque_ref(&root.root_id)
            || !is_bounded_opaque_ref(&root.scope_ref)
            || root
                .policy_epoch_ref
                .as_ref()
                .is_some_and(|value| !is_bounded_opaque_ref(value))
            || !is_bounded_capture_text(&root.trust_state, 64, false)
            || root
                .note
                .as_ref()
                .is_some_and(|note| !is_bounded_capture_text(note, MAX_CAPTURE_NOTE_BYTES, true))
    }) {
        return Err(SessionRestoreError::InvalidCapture(
            "trusted-root refs must be bounded opaque ids",
        ));
    }
    if [
        &input.active_workset_ids,
        &input.recovery_journal_refs,
        &input.local_history_snapshot_refs,
        &input.evidence_bundle_refs,
    ]
    .into_iter()
    .flatten()
    .any(|value| !is_bounded_opaque_ref(value))
    {
        return Err(SessionRestoreError::InvalidCapture(
            "checkpoint refs must be bounded opaque ids",
        ));
    }
    if input.dirty_buffer_journal_identities.iter().any(|journal| {
        !is_bounded_opaque_ref(&journal.journal_id)
            || !is_bounded_opaque_ref(&journal.last_known_revision_ref)
            || !is_bounded_capture_text(&journal.journal_kind, 128, false)
            || journal
                .note
                .as_ref()
                .is_some_and(|note| !is_bounded_capture_text(note, MAX_CAPTURE_NOTE_BYTES, true))
    }) {
        return Err(SessionRestoreError::InvalidCapture(
            "dirty-journal refs must be bounded opaque ids",
        ));
    }
    if input.downgrade_triggers.iter().any(|trigger| {
        let refs_out_of_bounds = [
            trigger.affected_root_refs.as_ref(),
            trigger.affected_workset_ids.as_ref(),
            trigger.affected_pane_ids.as_ref(),
        ]
        .into_iter()
        .flatten()
        .any(|refs| refs.len() > MAX_CAPTURE_REF_LIST_ITEMS);
        refs_out_of_bounds
            || trigger
                .note
                .as_ref()
                .is_some_and(|note| !is_bounded_capture_text(note, MAX_CAPTURE_NOTE_BYTES, true))
            || [
                trigger.affected_root_refs.as_ref(),
                trigger.affected_workset_ids.as_ref(),
                trigger.affected_pane_ids.as_ref(),
            ]
            .into_iter()
            .flatten()
            .flatten()
            .any(|value| !is_bounded_opaque_ref(value))
    }) {
        return Err(SessionRestoreError::InvalidCapture(
            "downgrade refs must be bounded opaque ids",
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
            if !is_bounded_opaque_ref(&tab.tab_id) || !tab_ids.insert(tab.tab_id.as_str()) {
                return Err(SessionRestoreError::InvalidCapture(
                    "tab ids must be present and unique",
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
                if !is_bounded_opaque_ref(&metadata.restore_metadata_ref)
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

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, SessionRestoreError> {
    let metadata = std::fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
        return Err(SessionRestoreError::CorruptStore(
            "recovery record is not a regular file",
        ));
    }
    if metadata.len() > MAX_RECOVERY_RECORD_BYTES {
        return Err(SessionRestoreError::CorruptStore(
            "recovery record exceeds the byte limit",
        ));
    }
    let file = std::fs::File::open(path)?;
    let mut bytes = Vec::with_capacity(metadata.len() as usize);
    file.take(MAX_RECOVERY_RECORD_BYTES + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > MAX_RECOVERY_RECORD_BYTES {
        return Err(SessionRestoreError::CorruptStore(
            "recovery record grew beyond the byte limit",
        ));
    }
    Ok(serde_json::from_slice(&bytes)?)
}

fn write_new_json<T: Serialize>(path: &Path, value: &T) -> Result<(), SessionRestoreError> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(value)?;
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(json.as_bytes())?;
    file.sync_all()?;
    Ok(())
}

fn write_new_json_atomically<T: Serialize>(
    path: &Path,
    value: &T,
) -> Result<(), SessionRestoreError> {
    let parent = path.parent().ok_or(SessionRestoreError::CorruptStore(
        "atomic index path has no parent",
    ))?;
    create_dir_all(parent)?;
    let file_name = path.file_name().and_then(|name| name.to_str()).ok_or(
        SessionRestoreError::CorruptStore("atomic index filename is invalid"),
    )?;
    let temporary_path = parent.join(format!(".{file_name}.{:020}.tmp", unix_nanos()));
    let bytes = serde_json::to_vec_pretty(value)?;
    let mut temporary = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary_path)?;
    if let Err(err) = (|| -> Result<(), std::io::Error> {
        temporary.write_all(&bytes)?;
        temporary.flush()?;
        temporary.sync_all()?;
        drop(temporary);
        // Hard-link publication gives create-new semantics on Unix and
        // Windows: an existing immutable index/advisory is never replaced.
        std::fs::hard_link(&temporary_path, path)?;
        let _ = std::fs::remove_file(&temporary_path);
        #[cfg(unix)]
        std::fs::File::open(parent)?.sync_all()?;
        Ok(())
    })() {
        let _ = std::fs::remove_file(&temporary_path);
        return Err(err.into());
    }
    Ok(())
}

type MaterializedTopology = (
    Vec<TabGroupInventoryEntry>,
    Vec<StablePaneInventoryEntry>,
    PaneNode,
    Vec<FocusChainEntry>,
);

fn materialize_topology_from_capture(
    groups: &[TabGroupCaptureInput],
    pane_tree_layout: Option<&TabGroupLayoutCapture>,
    focused_group_id: Option<&str>,
    snapshot_id: &str,
) -> Result<MaterializedTopology, SessionRestoreError> {
    let mut tab_group_topology = Vec::new();
    let mut stable_panes = Vec::new();
    let mut group_nodes = HashMap::new();
    let mut group_focus_targets = Vec::new();

    for group in groups {
        let mut ordered_tab_ids = Vec::new();
        let mut pinned_tab_ids = Vec::new();
        let mut tabs = Vec::new();
        for (idx, tab) in group.ordered_tabs.iter().enumerate() {
            let tab_id = tab.tab_id.clone();
            ordered_tab_ids.push(tab_id.clone());
            if tab.pinned {
                pinned_tab_ids.push(tab_id.clone());
            }
            let pane_id = format!(
                "pane:{snapshot_id}:{group}:{tab}:{idx}",
                snapshot_id = snapshot_id,
                group = group.group_id,
                tab = tab_id,
                idx = idx
            );
            let (hydration_behavior, availability_state) =
                restore_posture_for_surface(tab.surface_role, tab.surface_class);

            stable_panes.push(StablePaneInventoryEntry {
                pane_id: pane_id.clone(),
                surface_role: tab.surface_role,
                surface_class: tab.surface_class,
                hydration_behavior,
                availability_state,
                presentation_spotlighted: None,
                follow_anchor_candidate: None,
                title_hint: tab.tab_label.clone(),
                restore_metadata: tab.restore_metadata.clone(),
            });

            let surface = PaneSurfaceDescriptor {
                surface_role: tab.surface_role,
                surface_class: tab.surface_class,
                live_surface_class: None,
                hydration_behavior,
                availability_state,
                title_hint: tab.tab_label.clone(),
                surface_binding_ref: tab.surface_binding_ref.clone(),
                restore_metadata: tab.restore_metadata.clone(),
                follow_anchor_candidate: None,
                presentation_spotlighted: None,
                placeholder_card: None,
            };

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
            .unwrap_or_else(|| format!("tab:{snapshot_id}:missing"));

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
            target_ref: format!("window:{snapshot_id}"),
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

    Ok((tab_group_topology, stable_panes, root_node, focus_chain))
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
        let (inventory, panes, root, focus_chain) = materialize_topology_from_capture(
            &groups,
            Some(&layout),
            Some("group:second"),
            "snapshot:test",
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

        let (inventory, _, root, focus_chain) =
            materialize_topology_from_capture(&groups, None, Some("group:first"), "snapshot:test")
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
    fn corrupt_versioned_index_and_torn_newest_snapshot_fall_back_to_joined_pair() {
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

        assert_eq!(
            store.latest_refs().expect("joined fallback"),
            Some(second),
            "the orphan snapshot must not pair with an independent checkpoint maximum"
        );
        assert!(store.load_checkpoint(&first.checkpoint_id).is_ok());
    }

    #[test]
    fn torn_legacy_latest_pointer_falls_back_to_newest_joined_pair() {
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
        std::fs::remove_dir_all(store.root_path().join("latest_indices"))
            .expect("remove versioned indices to exercise legacy fallback");
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

        assert_eq!(store.latest_refs().expect("joined fallback"), Some(second));
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
            Err(SessionRestoreError::CorruptStore(_))
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
    }
}
