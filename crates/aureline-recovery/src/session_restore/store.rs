use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use super::records::{
    AvailabilityState, CheckpointSchemaVersion, DensityPreset, DirtyBufferJournalIdentity,
    DowngradeTriggerRecord, ExcludedLiveAuthorityClass, FocusChainEntry, FocusTargetKind,
    FollowMode, FollowPresentationState, HydrationBehavior, MonitorAffinityHint,
    MonitorAffinityStrength, PaneLeafNode, PaneNode, PaneSurfaceDescriptor, PaneTree,
    PaneTreeSchemaVersion, ProducerBuildStamp, RestoreClass, ScopeRefs, SnapshotReason,
    SplitOrientation, SurfaceClass, SurfaceRole, TabGroupInventoryEntry, TabRecord,
    TerminalPaneRestoreMetadata, TopologyPacketSchemaVersion, TrustedRootRecord, WindowChromeState,
    WindowRole, WindowState, WindowTopologySnapshotBodyRecord, WindowTopologySnapshotRecord,
    WorkspaceAuthorityCheckpointRecord,
};

/// Error returned when session-restore persistence fails.
#[derive(Debug)]
pub enum SessionRestoreError {
    Io(std::io::Error),
    Json(serde_json::Error),
    MissingRecord(String),
}

impl std::fmt::Display for SessionRestoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "session restore io error: {err}"),
            Self::Json(err) => write!(f, "session restore json error: {err}"),
            Self::MissingRecord(detail) => write!(f, "session restore missing record: {detail}"),
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

    fn mint(&mut self) -> String {
        let seq = self.next_seq;
        self.next_seq = self.next_seq.saturating_add(1);
        let stamp = unix_nanos();
        format!("{prefix}-{stamp:020}-{seq:06}", prefix = self.prefix)
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
}

impl SessionRestoreStore {
    /// Creates a store rooted at `root_dir/session_restore`.
    pub fn new(root_dir: impl AsRef<Path>) -> Self {
        let root = root_dir.as_ref().join("session_restore");
        Self {
            root,
            checkpoint_ids: IdSource::new("ckpt"),
            snapshot_ids: IdSource::new("snap"),
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
        let checkpoint_id = self.checkpoint_ids.mint();
        let snapshot_id = self.snapshot_ids.mint();
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
            materialize_topology_from_capture(&input.tab_groups, &snapshot_id);
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
        let path = self.root.join("latest.json");
        if let Ok(bytes) = std::fs::read(&path) {
            if let Ok(record) = serde_json::from_slice::<LatestIndexRecord>(&bytes) {
                return Ok(Some(SessionRestoreLatestRefs {
                    checkpoint_id: record.checkpoint_id,
                    snapshot_id: record.snapshot_id,
                }));
            }
        }
        let checkpoint_id = latest_id_in_dir(&self.root.join("workspace_authority_checkpoints"))?;
        let snapshot_id = latest_id_in_dir(&self.root.join("window_topology_snapshots"))?;
        match (checkpoint_id, snapshot_id) {
            (Some(checkpoint_id), Some(snapshot_id)) => Ok(Some(SessionRestoreLatestRefs {
                checkpoint_id,
                snapshot_id,
            })),
            _ => Ok(None),
        }
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
        let checkpoint_path = self
            .root
            .join("workspace_authority_checkpoints")
            .join(format!("{checkpoint_id}.json"));
        read_json(&checkpoint_path).map_err(|_| {
            SessionRestoreError::MissingRecord(format!(
                "checkpoint missing: {}",
                checkpoint_path.display()
            ))
        })
    }

    /// Loads a window-topology snapshot packet record by id.
    pub fn load_window_topology_snapshot(
        &self,
        snapshot_id: &str,
    ) -> Result<WindowTopologySnapshotRecord, SessionRestoreError> {
        let snapshot_path = self
            .root
            .join("window_topology_snapshots")
            .join(format!("{snapshot_id}.json"));
        read_json(&snapshot_path).map_err(|_| {
            SessionRestoreError::MissingRecord(format!(
                "snapshot missing: {}",
                snapshot_path.display()
            ))
        })
    }

    /// Loads a canonical pane-tree body for a window-topology snapshot id.
    pub fn load_pane_tree_body(
        &self,
        snapshot_id: &str,
    ) -> Result<WindowTopologySnapshotBodyRecord, SessionRestoreError> {
        let body_path = self
            .root
            .join("pane_tree_bodies")
            .join(format!("{snapshot_id}.json"));
        read_json(&body_path).map_err(|_| {
            SessionRestoreError::MissingRecord(format!(
                "pane tree body missing: {}",
                body_path.display()
            ))
        })
    }

    fn write_latest_index(
        &self,
        checkpoint_id: &str,
        snapshot_id: &str,
        emitted_at: &str,
    ) -> Result<(), SessionRestoreError> {
        if let Some(parent) = self.root.parent() {
            let _ = parent;
        }
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
        let json = serde_json::to_string_pretty(&record)?;
        let path = self.root.join("latest.json");
        std::fs::write(&path, json)?;
        Ok(())
    }
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, SessionRestoreError> {
    let bytes = std::fs::read(path)?;
    Ok(serde_json::from_slice(&bytes)?)
}

fn latest_id_in_dir(dir: &Path) -> Result<Option<String>, SessionRestoreError> {
    let Ok(iter) = std::fs::read_dir(dir) else {
        return Ok(None);
    };
    let mut best: Option<String> = None;
    for entry in iter {
        let Ok(entry) = entry else { continue };
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        let stem = stem.to_string();
        if best.as_ref().map_or(true, |current| stem > *current) {
            best = Some(stem);
        }
    }
    Ok(best)
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

fn materialize_topology_from_capture(
    groups: &[TabGroupCaptureInput],
    snapshot_id: &str,
) -> (
    Vec<TabGroupInventoryEntry>,
    Vec<super::records::StablePaneInventoryEntry>,
    PaneNode,
    Vec<FocusChainEntry>,
) {
    let mut tab_group_topology = Vec::new();
    let mut stable_panes = Vec::new();
    let mut all_tabs = Vec::new();

    for group in groups {
        let mut ordered_tab_ids = Vec::new();
        let mut tabs = Vec::new();
        for (idx, tab) in group.ordered_tabs.iter().enumerate() {
            let tab_id = tab.tab_id.clone();
            ordered_tab_ids.push(tab_id.clone());
            let pane_id = format!(
                "pane:{snapshot_id}:{group}:{tab}:{idx}",
                snapshot_id = snapshot_id,
                group = group.group_id,
                tab = tab_id,
                idx = idx
            );

            stable_panes.push(super::records::StablePaneInventoryEntry {
                pane_id: pane_id.clone(),
                surface_role: tab.surface_role,
                surface_class: tab.surface_class,
                hydration_behavior: HydrationBehavior::EagerLightweight,
                availability_state: AvailabilityState::Ready,
                presentation_spotlighted: None,
                follow_anchor_candidate: None,
                title_hint: tab.tab_label.clone(),
                restore_metadata: tab.restore_metadata.clone(),
            });

            let surface = PaneSurfaceDescriptor {
                surface_role: tab.surface_role,
                surface_class: tab.surface_class,
                live_surface_class: None,
                hydration_behavior: HydrationBehavior::EagerLightweight,
                availability_state: AvailabilityState::Ready,
                title_hint: tab.tab_label.clone(),
                surface_binding_ref: None,
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
            all_tabs.push((tab_id.clone(), pane_id));
        }

        let active_tab_id = group
            .active_tab_id
            .clone()
            .or_else(|| ordered_tab_ids.first().cloned())
            .unwrap_or_else(|| format!("tab:{snapshot_id}:missing"));

        tab_group_topology.push(TabGroupInventoryEntry {
            group_id: group.group_id.clone(),
            ordered_tab_ids,
            active_tab_id,
            pinned_tab_ids: None,
            close_empty_group: None,
        });

        if !tabs.is_empty() {
            all_tabs.extend(
                tabs.iter()
                    .map(|t| (t.tab_id.clone(), t.pane.pane_id.clone())),
            );
        }
    }

    let focus_chain = if let Some((tab_id, pane_id)) = all_tabs.first() {
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

    let root_node = if tab_group_topology.len() == 1 {
        let group_id = tab_group_topology[0].group_id.clone();
        let active_tab_id = tab_group_topology[0].active_tab_id.clone();
        let tabs = groups
            .first()
            .map(|g| {
                g.ordered_tabs
                    .iter()
                    .enumerate()
                    .map(|(idx, tab)| {
                        let tab_id = tab.tab_id.clone();
                        let pane_id = format!(
                            "pane:{snapshot_id}:{group}:{tab}:{idx}",
                            snapshot_id = snapshot_id,
                            group = g.group_id,
                            tab = tab_id,
                            idx = idx
                        );
                        let surface = PaneSurfaceDescriptor {
                            surface_role: tab.surface_role,
                            surface_class: tab.surface_class,
                            live_surface_class: None,
                            hydration_behavior: HydrationBehavior::EagerLightweight,
                            availability_state: AvailabilityState::Ready,
                            title_hint: tab.tab_label.clone(),
                            surface_binding_ref: None,
                            restore_metadata: tab.restore_metadata.clone(),
                            follow_anchor_candidate: None,
                            presentation_spotlighted: None,
                            placeholder_card: None,
                        };
                        TabRecord {
                            tab_id: tab_id.clone(),
                            tab_label: tab.tab_label.clone(),
                            pinned: Some(tab.pinned),
                            dirty_badge_visible: Some(tab.dirty_badge_visible),
                            pane: PaneLeafNode {
                                node_kind: "leaf".to_string(),
                                pane_id,
                                surface,
                            },
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        PaneNode::TabGroup {
            group_id,
            tabs,
            active_tab_id,
            close_empty_group: None,
        }
    } else {
        PaneNode::Split {
            split_id: format!("split:{snapshot_id}:root"),
            orientation: SplitOrientation::Vertical,
            children: tab_group_topology
                .iter()
                .map(|group| PaneNode::TabGroup {
                    group_id: group.group_id.clone(),
                    tabs: Vec::new(),
                    active_tab_id: group.active_tab_id.clone(),
                    close_empty_group: None,
                })
                .collect(),
            weights: None,
        }
    };

    (tab_group_topology, stable_panes, root_node, focus_chain)
}
