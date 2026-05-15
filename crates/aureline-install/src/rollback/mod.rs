//! Synthetic rollback drill driver for install-topology state roots.
//!
//! The driver walks only caller-provided synthetic roots. It captures a
//! pre-rollback snapshot, applies a bounded fake update to the target install
//! roots, restores those roots from the snapshot, and compares every walked
//! root against the captured state while ignoring declared post-rollback
//! evidence deltas.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs::{self, create_dir_all};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::topology::{
    ChannelClass, InstallModeClass, InstallTopologyAlphaPacket, InstallTopologyRow,
    InstallTopologyValidationFinding,
};

/// Schema version for rollback-drill records.
pub const ROLLBACK_DRILL_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`RollbackDrillPreStateSnapshot`].
pub const ROLLBACK_DRILL_PRE_STATE_RECORD_KIND: &str =
    "install_topology_rollback_pre_state_snapshot";

/// Stable record-kind tag for [`RollbackDrillReport`].
pub const ROLLBACK_DRILL_REPORT_RECORD_KIND: &str = "install_topology_rollback_drill_report";

/// Role a state root plays in the rollback drill.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackDrillRootRole {
    /// Root restored from the captured pre-state snapshot.
    TargetRollback,
    /// Installed side-by-side peer root that must remain untouched.
    SideBySidePeer,
    /// Portable colocated root that must remain isolated and untouched.
    PortableStateRoot,
}

/// Expected post-rollback delta class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackDrillDeltaClass {
    /// Evidence emitted after rollback validation completes.
    PostRollbackEvidence,
    /// Local health probe output that is intentionally not restored.
    RuntimeHealthProbe,
}

/// Filesystem entry kind captured in a rollback snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackDrillEntryKind {
    /// Directory entry.
    Directory,
    /// Regular file entry.
    File,
}

/// Difference class emitted when post-state does not match pre-state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackDrillDiffKind {
    /// Entry existed before rollback but is absent afterwards.
    MissingAfterRollback,
    /// Entry did not exist before rollback but exists afterwards.
    UnexpectedAfterRollback,
    /// Entry kind changed between pre-state and post-state.
    EntryKindChanged,
    /// File contents changed between pre-state and post-state.
    ContentsChanged,
}

/// One durable state root included in a rollback drill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackDrillRoot {
    /// Durable state-root ref from the install-topology packet.
    pub root_ref: String,
    /// Role this root plays in the drill.
    pub role: RollbackDrillRootRole,
    /// Install-topology row that owns this root.
    pub topology_row_id: String,
    /// Channel class that owns this root.
    pub channel_class: ChannelClass,
}

/// Expected delta ignored during post-rollback comparison.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackDrillExpectedDelta {
    /// Durable state-root ref containing the delta.
    pub root_ref: String,
    /// Slash-separated path relative to the durable state root.
    pub relative_path: String,
    /// Reason the delta is expected.
    pub delta_class: RollbackDrillDeltaClass,
}

/// Rollback drill plan derived from install-topology truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackDrillPlan {
    /// Stable drill id.
    pub drill_id: String,
    /// Install-topology row restored by the rollback drill.
    pub target_topology_row_id: String,
    /// Durable state roots walked by the drill.
    pub roots: Vec<RollbackDrillRoot>,
    /// Post-rollback evidence paths ignored during state comparison.
    pub expected_deltas: Vec<RollbackDrillExpectedDelta>,
}

impl RollbackDrillPlan {
    /// Builds a portable plus side-by-side rollback drill plan from topology truth.
    ///
    /// # Errors
    ///
    /// Returns [`RollbackDrillError`] when the topology packet does not
    /// validate, the requested rows are missing, or the selected rows do not
    /// model a rollback-capable side-by-side target with an isolated portable
    /// state root.
    pub fn portable_side_by_side(
        topology: &InstallTopologyAlphaPacket,
        target_topology_row_id: &str,
        portable_topology_row_id: &str,
    ) -> Result<Self, RollbackDrillError> {
        let validation = topology.validate();
        if !validation.passed {
            return Err(RollbackDrillError::TopologyPacketInvalid {
                findings: validation.findings,
            });
        }

        let target = topology.row_by_id(target_topology_row_id).ok_or_else(|| {
            RollbackDrillError::MissingTopologyRow {
                topology_row_id: target_topology_row_id.to_string(),
            }
        })?;
        if !target.is_side_by_side() {
            return Err(RollbackDrillError::InvalidDrillPlan {
                detail: format!(
                    "target row {} does not claim side-by-side behavior",
                    target.topology_row_id
                ),
            });
        }
        if !target.rollback_posture.rollback_available {
            return Err(RollbackDrillError::InvalidDrillPlan {
                detail: format!(
                    "target row {} does not claim rollback availability",
                    target.topology_row_id
                ),
            });
        }
        let paired_channel =
            target
                .paired_channel_class
                .ok_or_else(|| RollbackDrillError::InvalidDrillPlan {
                    detail: format!(
                        "target row {} is missing paired channel truth",
                        target.topology_row_id
                    ),
                })?;
        let peer = find_side_by_side_peer(topology, target, paired_channel)?;

        let portable = topology
            .row_by_id(portable_topology_row_id)
            .ok_or_else(|| RollbackDrillError::MissingTopologyRow {
                topology_row_id: portable_topology_row_id.to_string(),
            })?;
        if portable.install_mode_class != InstallModeClass::Portable {
            return Err(RollbackDrillError::InvalidDrillPlan {
                detail: format!(
                    "portable row {} is not an install-mode portable row",
                    portable.topology_row_id
                ),
            });
        }
        if !portable
            .durable_state_root_refs
            .iter()
            .any(|root| root.contains("portable_colocated_root"))
        {
            return Err(RollbackDrillError::InvalidDrillPlan {
                detail: format!(
                    "portable row {} does not expose a portable colocated root",
                    portable.topology_row_id
                ),
            });
        }

        let mut roots = Vec::new();
        extend_roots(&mut roots, target, RollbackDrillRootRole::TargetRollback);
        extend_roots(&mut roots, peer, RollbackDrillRootRole::SideBySidePeer);
        extend_roots(
            &mut roots,
            portable,
            RollbackDrillRootRole::PortableStateRoot,
        );
        reject_duplicate_root_roles(&roots)?;

        let evidence_root = roots
            .iter()
            .filter(|root| root.role == RollbackDrillRootRole::TargetRollback)
            .find(|root| root.root_ref.contains("recovery_root"))
            .or_else(|| {
                roots
                    .iter()
                    .find(|root| root.role == RollbackDrillRootRole::TargetRollback)
            })
            .map(|root| root.root_ref.clone())
            .ok_or_else(|| RollbackDrillError::InvalidDrillPlan {
                detail: "drill plan has no target rollback roots".to_string(),
            })?;

        Ok(Self {
            drill_id: format!(
                "install.rollback.drill.{}",
                sanitize_id(target_topology_row_id)
            ),
            target_topology_row_id: target.topology_row_id.clone(),
            roots,
            expected_deltas: vec![RollbackDrillExpectedDelta {
                root_ref: evidence_root,
                relative_path: "rollback-evidence/post-rollback.json".to_string(),
                delta_class: RollbackDrillDeltaClass::PostRollbackEvidence,
            }],
        })
    }

    /// Returns target root refs restored by the drill.
    pub fn target_root_refs(&self) -> Vec<&str> {
        self.roots
            .iter()
            .filter(|root| root.role == RollbackDrillRootRole::TargetRollback)
            .map(|root| root.root_ref.as_str())
            .collect()
    }
}

/// One captured filesystem entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackDrillEntry {
    /// Durable state-root ref containing the entry.
    pub root_ref: String,
    /// Slash-separated path relative to the durable state root.
    pub relative_path: String,
    /// Captured entry kind.
    pub entry_kind: RollbackDrillEntryKind,
    /// File bytes for regular files.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contents: Vec<u8>,
}

/// Pre-rollback state snapshot used to restore target roots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackDrillPreStateSnapshot {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Snapshot schema version.
    pub schema_version: u32,
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Drill plan id that produced the snapshot.
    pub drill_id: String,
    /// Install-topology row restored by the snapshot.
    pub target_topology_row_id: String,
    /// Durable state roots included in the snapshot.
    pub roots: Vec<RollbackDrillRoot>,
    /// Captured entries under all walked roots.
    pub entries: Vec<RollbackDrillEntry>,
    /// Integrity digest over plan identity, roots, and entries.
    pub entry_digest: String,
    /// Redaction-safe capture timestamp.
    pub captured_at: String,
}

/// One post-rollback state difference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackDrillDiff {
    /// Durable state-root ref containing the difference.
    pub root_ref: String,
    /// Slash-separated path relative to the durable state root.
    pub relative_path: String,
    /// Difference class.
    pub diff_kind: RollbackDrillDiffKind,
}

/// Filesystem path for one synthetic state root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackDrillRootPath {
    /// Durable state-root ref.
    pub root_ref: String,
    /// Synthetic path for the root.
    pub path: PathBuf,
}

/// Synthetic filesystem layout materialized for a drill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackDrillLayout {
    /// Root paths created for the drill.
    pub roots: Vec<RollbackDrillRootPath>,
}

/// Rollback drill result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackDrillReport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Report schema version.
    pub schema_version: u32,
    /// Drill plan id.
    pub drill_id: String,
    /// Snapshot id used for rollback.
    pub pre_state_snapshot_id: String,
    /// True when pre-state contained at least one captured entry.
    pub pre_state_captured: bool,
    /// True when target roots matched their pre-state after rollback.
    pub target_rolled_back: bool,
    /// Number of entries in the pre-state snapshot.
    pub pre_state_entry_count: usize,
    /// Number of entries in the post-rollback snapshot.
    pub post_state_entry_count: usize,
    /// Number of declared expected deltas ignored during comparison.
    pub expected_delta_count: usize,
    /// Durable state-root refs compared by the drill.
    pub compared_root_refs: Vec<String>,
    /// Differences found after rollback.
    pub diffs: Vec<RollbackDrillDiff>,
}

/// Errors returned while running the rollback drill.
#[derive(Debug, PartialEq, Eq)]
pub enum RollbackDrillError {
    /// The install-topology packet failed validation.
    TopologyPacketInvalid {
        /// Validation findings from the topology packet.
        findings: Vec<InstallTopologyValidationFinding>,
    },
    /// A requested install-topology row was not present.
    MissingTopologyRow {
        /// Missing topology row id.
        topology_row_id: String,
    },
    /// The selected rows cannot form a rollback drill.
    InvalidDrillPlan {
        /// Redaction-safe failure detail.
        detail: String,
    },
    /// A state-root ref cannot be mapped into the synthetic tree.
    UnsafeStateRoot {
        /// Unsafe state-root ref.
        root_ref: String,
        /// Redaction-safe failure detail.
        detail: String,
    },
    /// A planned state root was missing from the synthetic tree.
    MissingStateRoot {
        /// Durable state-root ref.
        root_ref: String,
        /// Expected synthetic path.
        path: PathBuf,
    },
    /// Filesystem I/O failed while reading or writing the synthetic tree.
    Io {
        /// Path involved in the I/O operation.
        path: PathBuf,
        /// Redaction-safe I/O error detail.
        detail: String,
    },
    /// Snapshot serialization failed.
    Serialization {
        /// Redaction-safe serialization error detail.
        detail: String,
    },
    /// The captured pre-state snapshot is unreadable or fails integrity checks.
    CorruptedPreStateSnapshot {
        /// Snapshot path that failed.
        path: PathBuf,
        /// Redaction-safe failure detail.
        detail: String,
    },
    /// Pre-state did not capture any entries.
    PreStateNotCaptured {
        /// Drill plan id.
        drill_id: String,
    },
    /// The synthetic update did not alter the target roots before rollback.
    SyntheticUpdateDidNotTouchTarget {
        /// Drill plan id.
        drill_id: String,
    },
    /// Post-state did not match the captured pre-state after rollback.
    TargetNotRolledBack {
        /// Differences found after rollback.
        diffs: Vec<RollbackDrillDiff>,
    },
}

impl fmt::Display for RollbackDrillError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TopologyPacketInvalid { findings } => {
                write!(f, "install topology packet is invalid: {}", findings.len())
            }
            Self::MissingTopologyRow { topology_row_id } => {
                write!(f, "missing install topology row: {topology_row_id}")
            }
            Self::InvalidDrillPlan { detail } => write!(f, "invalid rollback drill plan: {detail}"),
            Self::UnsafeStateRoot { root_ref, detail } => {
                write!(f, "unsafe rollback drill state root {root_ref}: {detail}")
            }
            Self::MissingStateRoot { root_ref, path } => {
                write!(
                    f,
                    "rollback drill state root {root_ref} is missing at {}",
                    path.display()
                )
            }
            Self::Io { path, detail } => write!(
                f,
                "rollback drill I/O failed at {}: {detail}",
                path.display()
            ),
            Self::Serialization { detail } => {
                write!(f, "rollback drill serialization failed: {detail}")
            }
            Self::CorruptedPreStateSnapshot { path, detail } => write!(
                f,
                "rollback drill pre-state snapshot is corrupted at {}: {detail}",
                path.display()
            ),
            Self::PreStateNotCaptured { drill_id } => {
                write!(f, "rollback drill {drill_id} captured no pre-state")
            }
            Self::SyntheticUpdateDidNotTouchTarget { drill_id } => write!(
                f,
                "rollback drill {drill_id} synthetic update did not touch target roots"
            ),
            Self::TargetNotRolledBack { diffs } => {
                write!(
                    f,
                    "rollback drill target did not roll back: {}",
                    diffs.len()
                )
            }
        }
    }
}

impl std::error::Error for RollbackDrillError {}

/// Filesystem-backed driver for synthetic rollback drills.
#[derive(Debug, Clone)]
pub struct RollbackDrillDriver {
    synthetic_tree_root: PathBuf,
}

impl RollbackDrillDriver {
    /// Creates a driver rooted at a synthetic filesystem tree.
    pub fn new(synthetic_tree_root: impl AsRef<Path>) -> Self {
        Self {
            synthetic_tree_root: synthetic_tree_root.as_ref().to_path_buf(),
        }
    }

    /// Returns the synthetic root directory used by this driver.
    pub fn synthetic_tree_root(&self) -> &Path {
        &self.synthetic_tree_root
    }

    /// Returns the path for a durable state-root ref under the synthetic tree.
    ///
    /// # Errors
    ///
    /// Returns [`RollbackDrillError::UnsafeStateRoot`] when the ref cannot be
    /// represented as a single synthetic path segment.
    pub fn state_root_path(&self, root_ref: &str) -> Result<PathBuf, RollbackDrillError> {
        Ok(self
            .synthetic_tree_root
            .join("state-roots")
            .join(safe_root_segment(root_ref)?))
    }

    /// Returns the pre-state snapshot path for `drill_id`.
    pub fn pre_state_snapshot_path(&self, drill_id: &str) -> PathBuf {
        self.synthetic_tree_root
            .join(".rollback_drill")
            .join(format!("{}.pre_state.json", sanitize_id(drill_id)))
    }

    /// Creates a deterministic synthetic state tree for the plan.
    ///
    /// # Errors
    ///
    /// Returns [`RollbackDrillError`] if root refs are unsafe or the synthetic
    /// tree cannot be written.
    pub fn seed_synthetic_state_tree(
        &self,
        plan: &RollbackDrillPlan,
    ) -> Result<RollbackDrillLayout, RollbackDrillError> {
        let mut roots = Vec::new();
        for root in &plan.roots {
            let path = self.state_root_path(&root.root_ref)?;
            create_dir_all(&path).map_err(|err| io_error(&path, err))?;
            write_bytes(
                &path.join("state-root.json"),
                synthetic_state_root_body(root).as_bytes(),
            )?;
            write_bytes(
                &path.join("settings").join("profile.json"),
                format!(
                    "{{\"root_ref\":\"{}\",\"channel\":\"{:?}\",\"role\":\"{:?}\"}}\n",
                    root.root_ref, root.channel_class, root.role
                )
                .as_bytes(),
            )?;
            write_bytes(
                &path.join("build").join("current.txt"),
                format!("previous-build:{}\n", root.topology_row_id).as_bytes(),
            )?;
            write_bytes(
                &path.join("support").join("export-index.json"),
                format!(
                    "{{\"support_ref\":\"support.install.rollback.{}\"}}\n",
                    root.root_ref
                )
                .as_bytes(),
            )?;
            roots.push(RollbackDrillRootPath {
                root_ref: root.root_ref.clone(),
                path,
            });
        }
        Ok(RollbackDrillLayout { roots })
    }

    /// Captures and writes the pre-state snapshot for a plan.
    ///
    /// # Errors
    ///
    /// Returns [`RollbackDrillError`] when a planned root is missing,
    /// unreadable, unsafe, or captures no entries.
    pub fn capture_pre_state(
        &self,
        plan: &RollbackDrillPlan,
    ) -> Result<RollbackDrillPreStateSnapshot, RollbackDrillError> {
        let mut snapshot = self.capture_snapshot(plan)?;
        if snapshot.entries.is_empty() {
            return Err(RollbackDrillError::PreStateNotCaptured {
                drill_id: plan.drill_id.clone(),
            });
        }
        snapshot.entry_digest = digest_snapshot(&snapshot);
        let path = self.pre_state_snapshot_path(&plan.drill_id);
        let json = serde_json::to_vec_pretty(&snapshot).map_err(|err| {
            RollbackDrillError::Serialization {
                detail: err.to_string(),
            }
        })?;
        write_bytes(&path, &json)?;
        Ok(snapshot)
    }

    /// Runs the full rollback drill after capturing pre-state.
    ///
    /// # Errors
    ///
    /// Returns [`RollbackDrillError`] when snapshot capture, synthetic update,
    /// rollback, or post-state comparison fails.
    pub fn run(&self, plan: &RollbackDrillPlan) -> Result<RollbackDrillReport, RollbackDrillError> {
        self.capture_pre_state(plan)?;
        self.run_from_captured_pre_state(plan)
    }

    /// Runs the rollback drill using an already captured pre-state snapshot.
    ///
    /// # Errors
    ///
    /// Returns [`RollbackDrillError::CorruptedPreStateSnapshot`] when the
    /// snapshot cannot be parsed or fails its integrity digest.
    pub fn run_from_captured_pre_state(
        &self,
        plan: &RollbackDrillPlan,
    ) -> Result<RollbackDrillReport, RollbackDrillError> {
        let snapshot = self.load_pre_state_snapshot(plan)?;
        self.apply_synthetic_update(plan)?;
        let mutated_snapshot = self.capture_snapshot(plan)?;
        if !target_changed(&snapshot, &mutated_snapshot, plan) {
            return Err(RollbackDrillError::SyntheticUpdateDidNotTouchTarget {
                drill_id: plan.drill_id.clone(),
            });
        }

        self.restore_target_roots(plan, &snapshot)?;
        self.write_expected_delta_evidence(plan)?;

        let mut post_snapshot = self.capture_snapshot(plan)?;
        post_snapshot.entry_digest = digest_snapshot(&post_snapshot);
        let diffs = compare_snapshots(&snapshot, &post_snapshot, &plan.expected_deltas);
        if !diffs.is_empty() {
            return Err(RollbackDrillError::TargetNotRolledBack { diffs });
        }

        Ok(RollbackDrillReport {
            record_kind: ROLLBACK_DRILL_REPORT_RECORD_KIND.to_string(),
            schema_version: ROLLBACK_DRILL_SCHEMA_VERSION,
            drill_id: plan.drill_id.clone(),
            pre_state_snapshot_id: snapshot.snapshot_id,
            pre_state_captured: true,
            target_rolled_back: true,
            pre_state_entry_count: snapshot.entries.len(),
            post_state_entry_count: post_snapshot.entries.len(),
            expected_delta_count: plan.expected_deltas.len(),
            compared_root_refs: plan
                .roots
                .iter()
                .map(|root| root.root_ref.clone())
                .collect(),
            diffs: Vec::new(),
        })
    }

    fn capture_snapshot(
        &self,
        plan: &RollbackDrillPlan,
    ) -> Result<RollbackDrillPreStateSnapshot, RollbackDrillError> {
        let mut entries = Vec::new();
        for root in &plan.roots {
            let root_path = self.state_root_path(&root.root_ref)?;
            if !root_path.exists() {
                return Err(RollbackDrillError::MissingStateRoot {
                    root_ref: root.root_ref.clone(),
                    path: root_path,
                });
            }
            walk_root(&root.root_ref, &root_path, &root_path, &mut entries)?;
        }
        entries.sort_by(|left, right| {
            left.root_ref
                .cmp(&right.root_ref)
                .then_with(|| left.relative_path.cmp(&right.relative_path))
        });

        Ok(RollbackDrillPreStateSnapshot {
            record_kind: ROLLBACK_DRILL_PRE_STATE_RECORD_KIND.to_string(),
            schema_version: ROLLBACK_DRILL_SCHEMA_VERSION,
            snapshot_id: format!("snapshot:rollback-drill:{}", now_nanos()),
            drill_id: plan.drill_id.clone(),
            target_topology_row_id: plan.target_topology_row_id.clone(),
            roots: plan.roots.clone(),
            entries,
            entry_digest: String::new(),
            captured_at: format!("unix-nanos:{}", now_nanos()),
        })
    }

    fn load_pre_state_snapshot(
        &self,
        plan: &RollbackDrillPlan,
    ) -> Result<RollbackDrillPreStateSnapshot, RollbackDrillError> {
        let path = self.pre_state_snapshot_path(&plan.drill_id);
        let bytes = fs::read(&path).map_err(|err| io_error(&path, err))?;
        let snapshot: RollbackDrillPreStateSnapshot =
            serde_json::from_slice(&bytes).map_err(|err| {
                RollbackDrillError::CorruptedPreStateSnapshot {
                    path: path.clone(),
                    detail: err.to_string(),
                }
            })?;
        validate_snapshot(&path, plan, &snapshot)?;
        Ok(snapshot)
    }

    fn apply_synthetic_update(&self, plan: &RollbackDrillPlan) -> Result<(), RollbackDrillError> {
        for root in plan
            .roots
            .iter()
            .filter(|root| root.role == RollbackDrillRootRole::TargetRollback)
        {
            let root_path = self.state_root_path(&root.root_ref)?;
            write_bytes(
                &root_path.join("build").join("current.txt"),
                format!("candidate-build:{}\n", root.topology_row_id).as_bytes(),
            )?;
            write_bytes(
                &root_path
                    .join("update-staging")
                    .join("candidate-marker.json"),
                format!(
                    "{{\"target\":\"{}\",\"root_ref\":\"{}\",\"synthetic\":true}}\n",
                    root.topology_row_id, root.root_ref
                )
                .as_bytes(),
            )?;
        }
        Ok(())
    }

    fn restore_target_roots(
        &self,
        plan: &RollbackDrillPlan,
        snapshot: &RollbackDrillPreStateSnapshot,
    ) -> Result<(), RollbackDrillError> {
        let target_roots: BTreeSet<&str> = plan.target_root_refs().into_iter().collect();
        for root_ref in &target_roots {
            let root_path = self.state_root_path(root_ref)?;
            clear_directory_contents(&root_path)?;
        }

        for entry in snapshot
            .entries
            .iter()
            .filter(|entry| target_roots.contains(entry.root_ref.as_str()))
        {
            let root_path = self.state_root_path(&entry.root_ref)?;
            let path = join_relative(&root_path, &entry.relative_path)?;
            match entry.entry_kind {
                RollbackDrillEntryKind::Directory => {
                    create_dir_all(&path).map_err(|err| io_error(&path, err))?;
                }
                RollbackDrillEntryKind::File => {
                    write_bytes(&path, &entry.contents)?;
                }
            }
        }
        Ok(())
    }

    fn write_expected_delta_evidence(
        &self,
        plan: &RollbackDrillPlan,
    ) -> Result<(), RollbackDrillError> {
        for delta in &plan.expected_deltas {
            if delta.delta_class != RollbackDrillDeltaClass::PostRollbackEvidence {
                continue;
            }
            let root_path = self.state_root_path(&delta.root_ref)?;
            let path = join_relative(&root_path, &delta.relative_path)?;
            write_bytes(
                &path,
                format!(
                    "{{\"drill_id\":\"{}\",\"delta_class\":\"post_rollback_evidence\"}}\n",
                    plan.drill_id
                )
                .as_bytes(),
            )?;
        }
        Ok(())
    }
}

fn find_side_by_side_peer<'a>(
    topology: &'a InstallTopologyAlphaPacket,
    target: &InstallTopologyRow,
    paired_channel: ChannelClass,
) -> Result<&'a InstallTopologyRow, RollbackDrillError> {
    topology
        .rows
        .iter()
        .find(|row| {
            row.channel_class == paired_channel
                && row.platform_class == target.platform_class
                && row.is_side_by_side()
                && row.paired_channel_class == Some(target.channel_class)
        })
        .ok_or_else(|| RollbackDrillError::InvalidDrillPlan {
            detail: format!(
                "no side-by-side peer found for target {} and channel {:?}",
                target.topology_row_id, paired_channel
            ),
        })
}

fn extend_roots(
    roots: &mut Vec<RollbackDrillRoot>,
    row: &InstallTopologyRow,
    role: RollbackDrillRootRole,
) {
    roots.extend(
        row.durable_state_root_refs
            .iter()
            .map(|root_ref| RollbackDrillRoot {
                root_ref: root_ref.clone(),
                role,
                topology_row_id: row.topology_row_id.clone(),
                channel_class: row.channel_class,
            }),
    );
}

fn reject_duplicate_root_roles(roots: &[RollbackDrillRoot]) -> Result<(), RollbackDrillError> {
    let mut seen = BTreeMap::<&str, RollbackDrillRootRole>::new();
    for root in roots {
        if let Some(existing) = seen.insert(&root.root_ref, root.role) {
            return Err(RollbackDrillError::InvalidDrillPlan {
                detail: format!(
                    "root {} appears in both {:?} and {:?} roles",
                    root.root_ref, existing, root.role
                ),
            });
        }
    }
    Ok(())
}

fn synthetic_state_root_body(root: &RollbackDrillRoot) -> String {
    format!(
        "{{\"root_ref\":\"{}\",\"topology_row_id\":\"{}\",\"channel_class\":\"{:?}\",\"role\":\"{:?}\"}}\n",
        root.root_ref, root.topology_row_id, root.channel_class, root.role
    )
}

fn walk_root(
    root_ref: &str,
    root_path: &Path,
    current_path: &Path,
    entries: &mut Vec<RollbackDrillEntry>,
) -> Result<(), RollbackDrillError> {
    let metadata = fs::symlink_metadata(current_path).map_err(|err| io_error(current_path, err))?;
    if metadata.file_type().is_symlink() {
        return Err(RollbackDrillError::UnsafeStateRoot {
            root_ref: root_ref.to_string(),
            detail: format!("symlink entry is not followed: {}", current_path.display()),
        });
    }

    if current_path != root_path {
        let relative_path = relative_path(root_path, current_path)?;
        if metadata.is_dir() {
            entries.push(RollbackDrillEntry {
                root_ref: root_ref.to_string(),
                relative_path,
                entry_kind: RollbackDrillEntryKind::Directory,
                contents: Vec::new(),
            });
        } else if metadata.is_file() {
            let contents = fs::read(current_path).map_err(|err| io_error(current_path, err))?;
            entries.push(RollbackDrillEntry {
                root_ref: root_ref.to_string(),
                relative_path,
                entry_kind: RollbackDrillEntryKind::File,
                contents,
            });
        }
    }

    if metadata.is_dir() {
        let mut children = fs::read_dir(current_path)
            .map_err(|err| io_error(current_path, err))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| io_error(current_path, err))?;
        children.sort_by_key(|entry| entry.path());
        for child in children {
            walk_root(root_ref, root_path, &child.path(), entries)?;
        }
    }
    Ok(())
}

fn clear_directory_contents(path: &Path) -> Result<(), RollbackDrillError> {
    if !path.exists() {
        create_dir_all(path).map_err(|err| io_error(path, err))?;
        return Ok(());
    }
    let mut children = fs::read_dir(path)
        .map_err(|err| io_error(path, err))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| io_error(path, err))?;
    children.sort_by_key(|entry| entry.path());
    children.reverse();
    for child in children {
        let child_path = child.path();
        let metadata =
            fs::symlink_metadata(&child_path).map_err(|err| io_error(&child_path, err))?;
        if metadata.file_type().is_symlink() || metadata.is_file() {
            fs::remove_file(&child_path).map_err(|err| io_error(&child_path, err))?;
        } else if metadata.is_dir() {
            clear_directory_contents(&child_path)?;
            fs::remove_dir(&child_path).map_err(|err| io_error(&child_path, err))?;
        }
    }
    Ok(())
}

fn write_bytes(path: &Path, contents: &[u8]) -> Result<(), RollbackDrillError> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent).map_err(|err| io_error(parent, err))?;
    }
    fs::write(path, contents).map_err(|err| io_error(path, err))
}

fn compare_snapshots(
    pre: &RollbackDrillPreStateSnapshot,
    post: &RollbackDrillPreStateSnapshot,
    expected_deltas: &[RollbackDrillExpectedDelta],
) -> Vec<RollbackDrillDiff> {
    let pre_map = normalized_entries(pre, expected_deltas);
    let post_map = normalized_entries(post, expected_deltas);
    let keys: BTreeSet<_> = pre_map.keys().chain(post_map.keys()).cloned().collect();
    let mut diffs = Vec::new();
    for key in keys {
        match (pre_map.get(&key), post_map.get(&key)) {
            (Some(_), None) => diffs.push(diff(key, RollbackDrillDiffKind::MissingAfterRollback)),
            (None, Some(_)) => {
                diffs.push(diff(key, RollbackDrillDiffKind::UnexpectedAfterRollback))
            }
            (Some(left), Some(right)) if left.entry_kind != right.entry_kind => {
                diffs.push(diff(key, RollbackDrillDiffKind::EntryKindChanged));
            }
            (Some(left), Some(right))
                if left.entry_kind == RollbackDrillEntryKind::File
                    && left.contents != right.contents =>
            {
                diffs.push(diff(key, RollbackDrillDiffKind::ContentsChanged));
            }
            _ => {}
        }
    }
    diffs
}

fn normalized_entries<'a>(
    snapshot: &'a RollbackDrillPreStateSnapshot,
    expected_deltas: &[RollbackDrillExpectedDelta],
) -> BTreeMap<(String, String), &'a RollbackDrillEntry> {
    snapshot
        .entries
        .iter()
        .filter(|entry| !is_expected_delta(&entry.root_ref, &entry.relative_path, expected_deltas))
        .map(|entry| ((entry.root_ref.clone(), entry.relative_path.clone()), entry))
        .collect()
}

fn diff(key: (String, String), diff_kind: RollbackDrillDiffKind) -> RollbackDrillDiff {
    RollbackDrillDiff {
        root_ref: key.0,
        relative_path: key.1,
        diff_kind,
    }
}

fn target_changed(
    pre: &RollbackDrillPreStateSnapshot,
    post: &RollbackDrillPreStateSnapshot,
    plan: &RollbackDrillPlan,
) -> bool {
    let target_roots: BTreeSet<String> = plan
        .roots
        .iter()
        .filter(|root| root.role == RollbackDrillRootRole::TargetRollback)
        .map(|root| root.root_ref.clone())
        .collect();
    let pre_target = target_entries(pre, &target_roots);
    let post_target = target_entries(post, &target_roots);
    pre_target != post_target
}

fn target_entries(
    snapshot: &RollbackDrillPreStateSnapshot,
    roots: &BTreeSet<String>,
) -> BTreeMap<(String, String), RollbackDrillEntry> {
    snapshot
        .entries
        .iter()
        .filter(|entry| roots.contains(&entry.root_ref))
        .map(|entry| {
            (
                (entry.root_ref.clone(), entry.relative_path.clone()),
                entry.clone(),
            )
        })
        .collect()
}

fn is_expected_delta(
    root_ref: &str,
    relative_path: &str,
    expected_deltas: &[RollbackDrillExpectedDelta],
) -> bool {
    expected_deltas
        .iter()
        .filter(|delta| delta.root_ref == root_ref)
        .any(|delta| {
            relative_path == delta.relative_path
                || relative_path
                    .strip_prefix(delta.relative_path.as_str())
                    .is_some_and(|suffix| suffix.starts_with('/'))
                || delta
                    .relative_path
                    .strip_prefix(relative_path)
                    .is_some_and(|suffix| suffix.starts_with('/'))
        })
}

fn validate_snapshot(
    path: &Path,
    plan: &RollbackDrillPlan,
    snapshot: &RollbackDrillPreStateSnapshot,
) -> Result<(), RollbackDrillError> {
    if snapshot.record_kind != ROLLBACK_DRILL_PRE_STATE_RECORD_KIND {
        return corrupted(path, "snapshot record_kind is unsupported");
    }
    if snapshot.schema_version != ROLLBACK_DRILL_SCHEMA_VERSION {
        return corrupted(path, "snapshot schema_version is unsupported");
    }
    if snapshot.drill_id != plan.drill_id {
        return corrupted(path, "snapshot drill_id does not match the active plan");
    }
    if snapshot.target_topology_row_id != plan.target_topology_row_id {
        return corrupted(
            path,
            "snapshot target_topology_row_id does not match the active plan",
        );
    }
    if snapshot.roots != plan.roots {
        return corrupted(path, "snapshot root set does not match the active plan");
    }
    let expected_digest = digest_snapshot(snapshot);
    if snapshot.entry_digest != expected_digest {
        return corrupted(
            path,
            "snapshot entry digest does not match captured contents",
        );
    }
    if snapshot.entries.is_empty() {
        return Err(RollbackDrillError::PreStateNotCaptured {
            drill_id: plan.drill_id.clone(),
        });
    }
    Ok(())
}

fn corrupted<T>(path: &Path, detail: impl Into<String>) -> Result<T, RollbackDrillError> {
    Err(RollbackDrillError::CorruptedPreStateSnapshot {
        path: path.to_path_buf(),
        detail: detail.into(),
    })
}

fn digest_snapshot(snapshot: &RollbackDrillPreStateSnapshot) -> String {
    let mut hasher = Fnv1a64::default();
    hasher.update(snapshot.drill_id.as_bytes());
    hasher.update(snapshot.target_topology_row_id.as_bytes());
    for root in &snapshot.roots {
        hasher.update(root.root_ref.as_bytes());
        hasher.update(format!("{:?}", root.role).as_bytes());
        hasher.update(root.topology_row_id.as_bytes());
        hasher.update(format!("{:?}", root.channel_class).as_bytes());
    }
    for entry in &snapshot.entries {
        hasher.update(entry.root_ref.as_bytes());
        hasher.update(entry.relative_path.as_bytes());
        hasher.update(format!("{:?}", entry.entry_kind).as_bytes());
        hasher.update(&entry.contents);
    }
    format!("fnv1a64:{:016x}", hasher.finish())
}

#[derive(Debug, Clone, Copy)]
struct Fnv1a64(u64);

impl Default for Fnv1a64 {
    fn default() -> Self {
        Self(0xcbf29ce484222325)
    }
}

impl Fnv1a64 {
    fn update(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 ^= u64::from(*byte);
            self.0 = self.0.wrapping_mul(0x100000001b3);
        }
        self.0 ^= 0xff;
        self.0 = self.0.wrapping_mul(0x100000001b3);
    }

    const fn finish(self) -> u64 {
        self.0
    }
}

fn safe_root_segment(root_ref: &str) -> Result<String, RollbackDrillError> {
    if root_ref.is_empty() {
        return Err(RollbackDrillError::UnsafeStateRoot {
            root_ref: root_ref.to_string(),
            detail: "root ref must not be empty".to_string(),
        });
    }
    if root_ref == "." || root_ref == ".." || root_ref.contains('/') || root_ref.contains('\\') {
        return Err(RollbackDrillError::UnsafeStateRoot {
            root_ref: root_ref.to_string(),
            detail: "root ref must be a single path segment".to_string(),
        });
    }
    if !root_ref
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '.' || ch == '_' || ch == '-')
    {
        return Err(RollbackDrillError::UnsafeStateRoot {
            root_ref: root_ref.to_string(),
            detail: "root ref contains unsupported characters".to_string(),
        });
    }
    Ok(root_ref.to_string())
}

fn join_relative(root_path: &Path, relative_path: &str) -> Result<PathBuf, RollbackDrillError> {
    if relative_path.is_empty()
        || relative_path.starts_with('/')
        || relative_path.contains('\\')
        || relative_path
            .split('/')
            .any(|component| component.is_empty() || component == "." || component == "..")
    {
        return Err(RollbackDrillError::UnsafeStateRoot {
            root_ref: root_path.display().to_string(),
            detail: format!("relative path is unsafe: {relative_path}"),
        });
    }
    Ok(root_path.join(relative_path))
}

fn relative_path(root_path: &Path, current_path: &Path) -> Result<String, RollbackDrillError> {
    let relative = current_path.strip_prefix(root_path).map_err(|err| {
        RollbackDrillError::UnsafeStateRoot {
            root_ref: root_path.display().to_string(),
            detail: err.to_string(),
        }
    })?;
    Ok(relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("/"))
}

fn sanitize_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '.' || ch == '_' || ch == '-' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

fn io_error(path: &Path, err: std::io::Error) -> RollbackDrillError {
    RollbackDrillError::Io {
        path: path.to_path_buf(),
        detail: err.to_string(),
    }
}

fn now_nanos() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
}
