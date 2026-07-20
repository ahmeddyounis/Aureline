// SPDX-FileCopyrightText: 2026 Aureline contributors
// SPDX-License-Identifier: Apache-2.0

//! Production capture bridge from live shell state to session-restore records.
//!
//! This module deliberately sits between UI/runtime objects and the recovery
//! store. It makes the persistence boundary narrow and reviewable: editor
//! topology is captured in pane-tree order, file identity crosses the boundary
//! only as an opaque logical-document ref and a basename-like label, and
//! terminal state is reduced to class-level restore metadata. Live PTY ids,
//! command text, environment bodies, source bytes, and authority handles are
//! never forwarded.

use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

use aureline_build_info as build_info;
use aureline_recovery::session_restore::records::{
    DirtyBufferJournalIdentity, DowngradeTriggerClass, DowngradeTriggerRecord,
    ExcludedLiveAuthorityClass, ProducerBuildStamp, SplitOrientation, SurfaceClass, SurfaceRole,
    TerminalPaneRestoreMetadata, TrustedRootRecord, WindowRole,
};
use aureline_recovery::session_restore::{
    SessionRestoreCaptureInput, SessionRestoreError, SessionRestoreLatestRefs, SessionRestoreStore,
    TabGroupCaptureInput, TabGroupLayoutCapture, TabItemCaptureInput,
};
use aureline_workspace::TrustState;

use crate::app_frame::desktop_frame::{DesktopFrame, EditorTabId};
use crate::layout::split_tree::{SplitAxis, SplitTopologyNode};
use crate::layout::zone_registry::ShellZoneId;
use crate::terminal_pane::TerminalPaneSnapshot;

/// Source schema used by the live shell capture bridge.
pub const LIVE_SESSION_CAPTURE_SOURCE_SCHEMA_VERSION: &str = "1";

/// Availability of the file or virtual surface represented by an editor tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorRestoreTargetState {
    /// A file target was available when the capture was assembled.
    AvailableFile,
    /// The prior file identity remains known, but the target is unavailable.
    MissingFile,
    /// The tab is already a non-file placeholder or recovery surface.
    Placeholder,
}

/// Redaction-safe metadata for one live editor tab.
///
/// `logical_document_ref` must be an opaque id such as `ld:<digest>`, never a
/// path or URI. `display_label` may be a full presentation path from the live
/// UI; capture reduces it to a basename-like label before persistence.
#[derive(Debug, Clone)]
pub struct EditorTabCaptureMetadata {
    pub tab_id: EditorTabId,
    pub logical_document_ref: Option<String>,
    pub display_label: Option<String>,
    pub pinned: bool,
    pub dirty_badge_visible: bool,
    pub target_state: EditorRestoreTargetState,
    pub dirty_journal_identity: Option<DirtyBufferJournalIdentity>,
}

/// Workspace/window context needed to persist one live shell window.
#[derive(Debug, Clone)]
pub struct WorkspaceRestoreCaptureContext {
    pub workspace_ref: String,
    pub root_id: String,
    pub root_scope_ref: String,
    pub root_policy_epoch_ref: Option<String>,
    pub workspace_trust_state: TrustState,
    pub active_workset_ids: Vec<String>,
    pub recovery_journal_refs: Vec<String>,
    pub local_history_snapshot_refs: Vec<String>,
    pub evidence_bundle_refs: Vec<String>,
    pub window_id: String,
    pub window_role: WindowRole,
    pub topology_family_ref: Option<String>,
    pub sibling_window_refs: Vec<String>,
}

/// Borrowed live inputs consumed by one graceful session capture.
pub struct LiveSessionRestoreCapture<'a> {
    pub frame: &'a DesktopFrame,
    pub editor_tabs: &'a [EditorTabCaptureMetadata],
    pub terminal_snapshot: Option<&'a TerminalPaneSnapshot>,
    pub context: &'a WorkspaceRestoreCaptureContext,
}

/// Failure returned before or during live session capture.
#[derive(Debug)]
pub enum LiveSessionCaptureError {
    /// A field intended to be an opaque reference looked path- or payload-like.
    UnsafeOpaqueRef(&'static str),
    /// Two metadata rows described the same frame tab.
    DuplicateEditorTabMetadata,
    /// A frame tab did not have a corresponding metadata row.
    MissingEditorTabMetadata,
    /// Metadata described a tab no longer present in the frame.
    StaleEditorTabMetadata,
    /// No editor or terminal tab remained to form a valid pane tree.
    EmptyTopology,
    /// A file-backed tab lacked its opaque logical-document identity.
    MissingLogicalDocumentRef,
    /// A dirty badge had no durable journal identity to support recovery.
    DirtyTabMissingJournalIdentity,
    /// The same journal id was supplied with conflicting metadata.
    ConflictingDirtyJournalIdentity,
    /// Terminal metadata advertised raw command or environment content.
    ForbiddenTerminalPayloadFlag,
    /// Terminal metadata belonged to a different workspace.
    TerminalWorkspaceMismatch,
    /// The recovery store could not persist the validated capture.
    Persistence(SessionRestoreError),
}

impl std::fmt::Display for LiveSessionCaptureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::UnsafeOpaqueRef(field) => {
                return write!(f, "session capture rejected unsafe opaque ref in {field}")
            }
            Self::DuplicateEditorTabMetadata => "session capture has duplicate editor metadata",
            Self::MissingEditorTabMetadata => "session capture is missing live editor metadata",
            Self::StaleEditorTabMetadata => "session capture contains stale editor metadata",
            Self::EmptyTopology => "session capture has no restorable tab topology",
            Self::MissingLogicalDocumentRef => "session capture is missing an opaque file identity",
            Self::DirtyTabMissingJournalIdentity => {
                "session capture cannot prove dirty-buffer continuity"
            }
            Self::ConflictingDirtyJournalIdentity => {
                "session capture has conflicting dirty-journal identity"
            }
            Self::ForbiddenTerminalPayloadFlag => {
                "session capture rejected raw terminal payload posture"
            }
            Self::TerminalWorkspaceMismatch => {
                "session capture rejected cross-workspace terminal metadata"
            }
            Self::Persistence(_) => "session restore persistence failed",
        };
        f.write_str(message)
    }
}

impl std::error::Error for LiveSessionCaptureError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Persistence(err) => Some(err),
            _ => None,
        }
    }
}

impl From<SessionRestoreError> for LiveSessionCaptureError {
    fn from(value: SessionRestoreError) -> Self {
        Self::Persistence(value)
    }
}

/// Validates, redacts, and persists one live session-restore snapshot.
pub fn capture_live_session(
    store: &mut SessionRestoreStore,
    live: LiveSessionRestoreCapture<'_>,
) -> Result<SessionRestoreLatestRefs, LiveSessionCaptureError> {
    let input = materialize_live_session_capture(live)?;
    store.capture(input).map_err(Into::into)
}

/// Builds the recovery-owned capture input without persisting it.
///
/// This is public so the native shell can perform a preflight before marking a
/// graceful shutdown clean and so focused integration tests can inspect the
/// exact authority boundary.
pub fn materialize_live_session_capture(
    live: LiveSessionRestoreCapture<'_>,
) -> Result<SessionRestoreCaptureInput, LiveSessionCaptureError> {
    validate_context(live.context)?;

    let mut metadata_by_tab = HashMap::new();
    for metadata in live.editor_tabs {
        if metadata_by_tab.insert(metadata.tab_id, metadata).is_some() {
            return Err(LiveSessionCaptureError::DuplicateEditorTabMetadata);
        }
    }

    let mut seen_frame_tabs = HashSet::new();
    let mut tab_groups = Vec::new();
    let mut dirty_journals: Vec<DirtyBufferJournalIdentity> = Vec::new();
    let mut journal_index: HashMap<String, usize> = HashMap::new();
    let mut missing_target_present = false;

    for group_id in live.frame.editor_group_ids_in_order() {
        let frame_tabs = live.frame.tab_ids(group_id);
        let mut ordered_tabs = Vec::with_capacity(frame_tabs.len());
        let mut active_tab_id = None;
        for tab_id in frame_tabs {
            seen_frame_tabs.insert(tab_id);
            let metadata = metadata_by_tab
                .get(&tab_id)
                .copied()
                .ok_or(LiveSessionCaptureError::MissingEditorTabMetadata)?;
            let stable_tab_id = editor_restore_tab_id(metadata)?;
            if live.frame.active_tab_id(group_id) == Some(tab_id) {
                active_tab_id = Some(stable_tab_id.clone());
            }

            if metadata.dirty_badge_visible {
                let journal = metadata
                    .dirty_journal_identity
                    .as_ref()
                    .ok_or(LiveSessionCaptureError::DirtyTabMissingJournalIdentity)?;
                let sanitized = sanitize_dirty_journal_identity(journal)?;
                if let Some(existing_idx) = journal_index.get(&sanitized.journal_id).copied() {
                    if dirty_journals[existing_idx] != sanitized {
                        return Err(LiveSessionCaptureError::ConflictingDirtyJournalIdentity);
                    }
                } else {
                    journal_index.insert(sanitized.journal_id.clone(), dirty_journals.len());
                    dirty_journals.push(sanitized);
                }
            }

            let (surface_role, surface_class, fallback_label) = match metadata.target_state {
                EditorRestoreTargetState::AvailableFile => {
                    (SurfaceRole::Editor, SurfaceClass::TextEditor, "Editor")
                }
                EditorRestoreTargetState::MissingFile => {
                    missing_target_present = true;
                    (
                        SurfaceRole::Placeholder,
                        SurfaceClass::PlaceholderCard,
                        "Missing file",
                    )
                }
                EditorRestoreTargetState::Placeholder => (
                    SurfaceRole::Placeholder,
                    SurfaceClass::PlaceholderCard,
                    "Unavailable editor",
                ),
            };

            ordered_tabs.push(TabItemCaptureInput {
                tab_id: stable_tab_id,
                tab_label: Some(
                    safe_display_label(metadata.display_label.as_deref())
                        .unwrap_or_else(|| fallback_label.to_string()),
                ),
                surface_binding_ref: metadata.logical_document_ref.clone(),
                pinned: metadata.pinned,
                dirty_badge_visible: metadata.dirty_badge_visible,
                surface_role,
                surface_class,
                restore_metadata: None,
            });
        }

        if ordered_tabs.is_empty() {
            let placeholder_tab_id = format!("tab:placeholder:empty-group:{}", group_id.value());
            ordered_tabs.push(TabItemCaptureInput {
                tab_id: placeholder_tab_id.clone(),
                tab_label: Some("Empty editor".to_string()),
                surface_binding_ref: None,
                pinned: false,
                dirty_badge_visible: false,
                surface_role: SurfaceRole::Placeholder,
                surface_class: SurfaceClass::PlaceholderCard,
                restore_metadata: None,
            });
            active_tab_id = Some(placeholder_tab_id);
        }

        tab_groups.push(TabGroupCaptureInput {
            group_id: format!("group:editor:{}", group_id.value()),
            ordered_tabs,
            active_tab_id,
        });
    }

    if seen_frame_tabs.len() != metadata_by_tab.len() {
        return Err(LiveSessionCaptureError::StaleEditorTabMetadata);
    }

    let mut downgrade_triggers = Vec::new();
    if missing_target_present {
        downgrade_triggers.push(DowngradeTriggerRecord {
            trigger_class: DowngradeTriggerClass::ManualRepairRequired,
            affected_root_refs: Some(vec![live.context.root_id.clone()]),
            affected_workset_ids: None,
            affected_pane_ids: None,
            note: Some(
                "A file target was unavailable at capture; its slot was retained as a placeholder."
                    .to_string(),
            ),
        });
    }

    if live.context.workspace_trust_state != TrustState::Trusted {
        downgrade_triggers.push(DowngradeTriggerRecord {
            trigger_class: DowngradeTriggerClass::PolicyNarrowing,
            affected_root_refs: Some(vec![live.context.root_id.clone()]),
            affected_workset_ids: None,
            affected_pane_ids: None,
            note: Some(
                "Workspace authority must be re-evaluated before restore hydration.".to_string(),
            ),
        });
    }

    let mut terminal_group_added = false;
    if let Some(terminal_snapshot) = live.terminal_snapshot {
        if terminal_snapshot.workspace_id != live.context.workspace_ref {
            return Err(LiveSessionCaptureError::TerminalWorkspaceMismatch);
        }
        if !terminal_snapshot.tabs.is_empty() {
            let terminal_group =
                terminal_capture_group(terminal_snapshot, live.context.window_id.as_str())?;
            tab_groups.push(terminal_group);
            terminal_group_added = true;
            downgrade_triggers.push(DowngradeTriggerRecord {
                trigger_class: DowngradeTriggerClass::ExcludedLiveHandle,
                affected_root_refs: Some(vec![live.context.root_id.clone()]),
                affected_workset_ids: None,
                affected_pane_ids: None,
                note: Some(
                    "Terminal view metadata was retained without a live process or session handle."
                        .to_string(),
                ),
            });
        }
    }

    let editor_focus_group_id =
        format!("group:editor:{}", live.frame.focused_editor_group().value());
    let terminal_focus_group_id = format!("group:terminal:{}", live.context.window_id);
    let preferred_focus_group_id = if live.frame.focused_zone() == ShellZoneId::BottomPanel {
        terminal_focus_group_id
    } else {
        editor_focus_group_id
    };
    let focused_group_id = tab_groups
        .iter()
        .any(|group| group.group_id == preferred_focus_group_id)
        .then_some(preferred_focus_group_id)
        .or_else(|| tab_groups.first().map(|group| group.group_id.clone()));
    if tab_groups.is_empty() {
        return Err(LiveSessionCaptureError::EmptyTopology);
    }
    let editor_layout = editor_group_layout_capture(&live.frame.editor_split_topology());
    let pane_tree_layout = if terminal_group_added {
        TabGroupLayoutCapture::Split {
            split_id: format!("split:shell:{}", live.context.window_id),
            orientation: SplitOrientation::Horizontal,
            children: vec![
                editor_layout,
                TabGroupLayoutCapture::TabGroup {
                    group_id: format!("group:terminal:{}", live.context.window_id),
                },
            ],
            weights: None,
        }
    } else {
        editor_layout
    };

    Ok(SessionRestoreCaptureInput {
        workspace_ref: live.context.workspace_ref.clone(),
        producer_build: live_producer_build_stamp(),
        source_schema_version: LIVE_SESSION_CAPTURE_SOURCE_SCHEMA_VERSION.to_string(),
        trusted_root_refs: vec![TrustedRootRecord {
            root_id: live.context.root_id.clone(),
            trust_state: live.context.workspace_trust_state.as_str().to_string(),
            scope_ref: live.context.root_scope_ref.clone(),
            policy_epoch_ref: live.context.root_policy_epoch_ref.clone(),
            note: None,
        }],
        active_workset_ids: unique_refs(&live.context.active_workset_ids),
        dirty_buffer_journal_identities: dirty_journals,
        recovery_journal_refs: unique_refs(&live.context.recovery_journal_refs),
        local_history_snapshot_refs: unique_refs(&live.context.local_history_snapshot_refs),
        evidence_bundle_refs: unique_refs(&live.context.evidence_bundle_refs),
        excluded_live_authority_classes: excluded_live_authority_classes(),
        downgrade_triggers,
        window_id: live.context.window_id.clone(),
        window_role: live.context.window_role,
        topology_family_ref: live.context.topology_family_ref.clone(),
        sibling_window_refs: unique_refs(&live.context.sibling_window_refs),
        tab_groups,
        pane_tree_layout: Some(pane_tree_layout),
        focused_group_id,
        emitted_at: live_capture_timestamp(),
        notes: None,
    })
}

fn editor_group_layout_capture(node: &SplitTopologyNode) -> TabGroupLayoutCapture {
    match node {
        SplitTopologyNode::Leaf { pane_id } => TabGroupLayoutCapture::TabGroup {
            group_id: format!("group:editor:{}", pane_id.value()),
        },
        SplitTopologyNode::Split {
            split_id,
            axis,
            first_weight,
            second_weight,
            first,
            second,
        } => TabGroupLayoutCapture::Split {
            split_id: format!("split:editor:{}", split_id.value()),
            orientation: match axis {
                SplitAxis::Vertical => SplitOrientation::Vertical,
            },
            children: vec![
                editor_group_layout_capture(first),
                editor_group_layout_capture(second),
            ],
            weights: Some(vec![f64::from(*first_weight), f64::from(*second_weight)]),
        },
    }
}

fn validate_context(
    context: &WorkspaceRestoreCaptureContext,
) -> Result<(), LiveSessionCaptureError> {
    validate_opaque_ref("workspace_ref", &context.workspace_ref)?;
    validate_opaque_ref("root_id", &context.root_id)?;
    validate_opaque_ref("root_scope_ref", &context.root_scope_ref)?;
    validate_optional_ref(
        "root_policy_epoch_ref",
        context.root_policy_epoch_ref.as_deref(),
    )?;
    validate_refs("active_workset_ids", &context.active_workset_ids)?;
    validate_refs("recovery_journal_refs", &context.recovery_journal_refs)?;
    validate_refs(
        "local_history_snapshot_refs",
        &context.local_history_snapshot_refs,
    )?;
    validate_refs("evidence_bundle_refs", &context.evidence_bundle_refs)?;
    validate_opaque_ref("window_id", &context.window_id)?;
    validate_optional_ref(
        "topology_family_ref",
        context.topology_family_ref.as_deref(),
    )?;
    validate_refs("sibling_window_refs", &context.sibling_window_refs)?;
    Ok(())
}

fn editor_restore_tab_id(
    metadata: &EditorTabCaptureMetadata,
) -> Result<String, LiveSessionCaptureError> {
    match metadata.target_state {
        EditorRestoreTargetState::AvailableFile | EditorRestoreTargetState::MissingFile => {
            let identity = metadata
                .logical_document_ref
                .as_deref()
                .ok_or(LiveSessionCaptureError::MissingLogicalDocumentRef)?;
            validate_opaque_ref("logical_document_ref", identity)?;
            Ok(format!("tab:editor:{}:{identity}", metadata.tab_id.0))
        }
        EditorRestoreTargetState::Placeholder => {
            if let Some(identity) = metadata.logical_document_ref.as_deref() {
                validate_opaque_ref("logical_document_ref", identity)?;
                Ok(format!("tab:placeholder:{}:{identity}", metadata.tab_id.0))
            } else {
                Ok(format!("tab:placeholder:{}", metadata.tab_id.0))
            }
        }
    }
}

fn sanitize_dirty_journal_identity(
    journal: &DirtyBufferJournalIdentity,
) -> Result<DirtyBufferJournalIdentity, LiveSessionCaptureError> {
    validate_opaque_ref("dirty_journal_id", &journal.journal_id)?;
    validate_opaque_ref(
        "dirty_journal_revision_ref",
        &journal.last_known_revision_ref,
    )?;
    if !matches!(
        journal.journal_kind.as_str(),
        "dirty_buffer_recovery_journal"
            | "local_history_journal"
            | "deferred_intent_outbox"
            | "session_restore_journal"
            | "terminal_scrollback_restore"
            | "notebook_output_snapshot"
            | "checkpoint_lineage_journal"
    ) {
        return Err(LiveSessionCaptureError::UnsafeOpaqueRef(
            "dirty_journal_kind",
        ));
    }
    Ok(DirtyBufferJournalIdentity {
        journal_id: journal.journal_id.clone(),
        journal_kind: journal.journal_kind.clone(),
        last_known_revision_ref: journal.last_known_revision_ref.clone(),
        frame_count: journal.frame_count,
        // Journal notes can contain user-authored or path-bearing text. The
        // checkpoint needs identity and revision only.
        note: None,
    })
}

fn terminal_capture_group(
    snapshot: &TerminalPaneSnapshot,
    window_id: &str,
) -> Result<TabGroupCaptureInput, LiveSessionCaptureError> {
    let mut ordered_tabs = Vec::with_capacity(snapshot.tabs.len());
    let mut active_tab_id = None;
    for (idx, terminal) in snapshot.tabs.iter().enumerate() {
        if terminal.restore_metadata.raw_command_body_present
            || terminal.restore_metadata.raw_environment_body_present
        {
            return Err(LiveSessionCaptureError::ForbiddenTerminalPayloadFlag);
        }

        let tab_id = format!("tab:terminal-view:{window_id}:{idx}");
        if snapshot.active_tab_id.as_ref() == Some(&terminal.session_id) {
            active_tab_id = Some(tab_id.clone());
        }
        let source = &terminal.restore_metadata;
        ordered_tabs.push(TabItemCaptureInput {
            tab_id,
            tab_label: Some(
                safe_display_label(Some(terminal.display_title.as_str()))
                    .unwrap_or_else(|| "Terminal".to_string()),
            ),
            // A live terminal session id or execution-context ref is not a
            // restorable binding and must not cross this boundary.
            surface_binding_ref: None,
            pinned: false,
            dirty_badge_visible: false,
            surface_role: SurfaceRole::Terminal,
            surface_class: SurfaceClass::TerminalView,
            restore_metadata: Some(TerminalPaneRestoreMetadata {
                restore_metadata_ref: format!("terminal-metadata:{window_id}:{idx}"),
                working_directory: safe_display_label(source.working_directory.as_deref()),
                environment_scope_token: source.environment_scope.as_str().to_string(),
                shell_identity: safe_display_label(Some(source.shell_identity.as_str()))
                    .unwrap_or_else(|| "shell".to_string()),
                shell_family_token: source.shell_family.as_str().to_string(),
                last_command_class_token: source.last_command_class.as_str().to_string(),
                auto_rerun_forbidden: true,
                raw_command_body_present: false,
                raw_environment_body_present: false,
            }),
        });
    }

    Ok(TabGroupCaptureInput {
        group_id: format!("group:terminal:{window_id}"),
        ordered_tabs,
        active_tab_id,
    })
}

fn live_producer_build_stamp() -> ProducerBuildStamp {
    let identity = build_info::build_identity();
    ProducerBuildStamp {
        producer_name: "aureline-shell".to_string(),
        producer_version: identity.workspace_version,
        producer_channel: Some(
            match build_info::release_channel_class() {
                "beta" => "beta",
                "stable" => "stable",
                "lts" => "lts",
                _ => "experimental",
            }
            .to_string(),
        ),
        producer_platform_class: Some(
            match std::env::consts::OS {
                "macos" => "macos",
                "windows" => "windows",
                "linux" => "linux",
                _ => "other",
            }
            .to_string(),
        ),
        producer_instance_handle: Some(build_info::exact_build_identity_ref()),
    }
}

fn live_capture_timestamp() -> String {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!(
        "mono:unix:{}.{:09}",
        duration.as_secs(),
        duration.subsec_nanos()
    )
}

fn excluded_live_authority_classes() -> Vec<ExcludedLiveAuthorityClass> {
    vec![
        ExcludedLiveAuthorityClass::RawSecretMaterial,
        ExcludedLiveAuthorityClass::LiveTokenOrCookie,
        ExcludedLiveAuthorityClass::DelegatedApprovalOrUnspentTicket,
        ExcludedLiveAuthorityClass::MachineUniqueHandle,
        ExcludedLiveAuthorityClass::LiveProcessOrSessionHandle,
        ExcludedLiveAuthorityClass::RawProviderPayload,
        ExcludedLiveAuthorityClass::RawUrlPathCommandOrLog,
        ExcludedLiveAuthorityClass::RawSourceOrUserContent,
        ExcludedLiveAuthorityClass::LiveRemoteOrKernelBinding,
    ]
}

fn validate_refs(field: &'static str, refs: &[String]) -> Result<(), LiveSessionCaptureError> {
    for value in refs {
        validate_opaque_ref(field, value)?;
    }
    Ok(())
}

fn validate_optional_ref(
    field: &'static str,
    value: Option<&str>,
) -> Result<(), LiveSessionCaptureError> {
    if let Some(value) = value {
        validate_opaque_ref(field, value)?;
    }
    Ok(())
}

fn validate_opaque_ref(field: &'static str, value: &str) -> Result<(), LiveSessionCaptureError> {
    let valid = !value.is_empty()
        && value.len() <= 512
        && !value.contains('/')
        && !value.contains('\\')
        && !value.contains("://")
        && value.chars().all(|ch| {
            ch.is_ascii_alphanumeric() || matches!(ch, ':' | '.' | '_' | '-' | '#' | '@' | '|')
        });
    if valid {
        Ok(())
    } else {
        Err(LiveSessionCaptureError::UnsafeOpaqueRef(field))
    }
}

fn unique_refs(refs: &[String]) -> Vec<String> {
    let mut seen = HashSet::new();
    refs.iter()
        .filter(|value| seen.insert(value.as_str()))
        .cloned()
        .collect()
}

fn safe_display_label(value: Option<&str>) -> Option<String> {
    let value = value?.trim();
    let leaf = value
        .rsplit(|ch| ch == '/' || ch == '\\')
        .find(|part| !part.trim().is_empty())?
        .split(|ch| ch == '?' || ch == '#')
        .next()
        .unwrap_or_default()
        .trim();
    if leaf.is_empty() || matches!(leaf, "." | "..") {
        return None;
    }

    let mut out = String::new();
    for ch in leaf.chars() {
        if out.chars().count() >= 128 {
            break;
        }
        if ch.is_control() || matches!(ch, '/' | '\\' | ':') {
            out.push('_');
        } else {
            out.push(ch);
        }
    }
    let out = out.trim().to_string();
    (!out.is_empty()).then_some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    use aureline_recovery::crash_journal::CrashJournalStore;
    use aureline_recovery::session_restore::{
        RestorePaneExecutionKind, RestoreProposal, RestoreRuntime,
    };
    use aureline_terminal::{HostClass, OpenSessionRequest, PtyHost};

    fn context(trust: TrustState) -> WorkspaceRestoreCaptureContext {
        WorkspaceRestoreCaptureContext {
            workspace_ref: "workspace:test".to_string(),
            root_id: "root:test".to_string(),
            root_scope_ref: "scope:workspace:test".to_string(),
            root_policy_epoch_ref: Some("policy-epoch:7".to_string()),
            workspace_trust_state: trust,
            active_workset_ids: vec!["workset:primary".to_string()],
            recovery_journal_refs: vec!["journal:workspace:test".to_string()],
            local_history_snapshot_refs: vec!["history:snapshot:1".to_string()],
            evidence_bundle_refs: vec!["evidence:restore:1".to_string()],
            window_id: "window:primary".to_string(),
            window_role: WindowRole::Primary,
            topology_family_ref: Some("topology:family:1".to_string()),
            sibling_window_refs: Vec::new(),
        }
    }

    fn file_metadata(
        tab_id: EditorTabId,
        logical_ref: &str,
        label: &str,
        dirty: bool,
    ) -> EditorTabCaptureMetadata {
        EditorTabCaptureMetadata {
            tab_id,
            logical_document_ref: Some(logical_ref.to_string()),
            display_label: Some(label.to_string()),
            pinned: false,
            dirty_badge_visible: dirty,
            target_state: EditorRestoreTargetState::AvailableFile,
            dirty_journal_identity: dirty.then(|| DirtyBufferJournalIdentity {
                journal_id: format!("journal:dirty:{}", tab_id.0),
                journal_kind: "dirty_buffer_recovery_journal".to_string(),
                last_known_revision_ref: format!("revision:{}", tab_id.0),
                frame_count: Some(1),
                note: Some("must not cross capture boundary".to_string()),
            }),
        }
    }

    #[test]
    fn capture_preserves_all_group_tab_and_active_order_even_when_compact() {
        let mut frame = DesktopFrame::new(1920, 1080);
        let first_group = frame.focused_editor_group();
        let first = frame.open_tab().expect("first tab");
        let second = frame.open_tab().expect("second tab");
        assert!(frame.set_active_tab(first_group, first));
        let second_group = match frame.request_split_focused_editor_group() {
            crate::app_frame::desktop_frame::NewEditorGroupOutcome::Created { new_group } => {
                new_group
            }
            other => panic!("expected split, got {other:?}"),
        };
        let third = frame.open_tab().expect("third tab");
        let third_group = match frame.request_split_focused_editor_group() {
            crate::app_frame::desktop_frame::NewEditorGroupOutcome::Created { new_group } => {
                new_group
            }
            other => panic!("expected nested split, got {other:?}"),
        };
        let fourth = frame.open_tab().expect("fourth tab");

        // Responsive projection hides the second group, but structural
        // capture must keep it.
        frame.relayout(320, 720);
        assert_eq!(frame.editor_group_layouts().len(), 1);
        assert_eq!(frame.editor_group_ids_in_order().len(), 3);

        let metadata = vec![
            file_metadata(third, "ld:cccc", "/workspace/src/third.rs", false),
            file_metadata(first, "ld:aaaa", "/workspace/src/first.rs", false),
            file_metadata(second, "ld:bbbb", "/workspace/src/second.rs", false),
            file_metadata(fourth, "ld:dddd", "/workspace/src/fourth.rs", false),
        ];
        let capture = materialize_live_session_capture(LiveSessionRestoreCapture {
            frame: &frame,
            editor_tabs: &metadata,
            terminal_snapshot: None,
            context: &context(TrustState::Trusted),
        })
        .expect("capture input");

        assert_eq!(capture.tab_groups.len(), 3);
        assert_eq!(
            capture.tab_groups[0].group_id,
            format!("group:editor:{}", first_group.value())
        );
        assert_eq!(
            capture.tab_groups[0]
                .ordered_tabs
                .iter()
                .map(|tab| tab.tab_label.as_deref())
                .collect::<Vec<_>>(),
            vec![Some("first.rs"), Some("second.rs")]
        );
        let expected_active = format!("tab:editor:{}:ld:aaaa", first.0);
        assert_eq!(
            capture.tab_groups[0].active_tab_id.as_deref(),
            Some(expected_active.as_str())
        );
        assert_eq!(
            capture.tab_groups[1].group_id,
            format!("group:editor:{}", second_group.value())
        );
        assert_eq!(
            capture.tab_groups[1].ordered_tabs[0].tab_id,
            format!("tab:editor:{}:ld:cccc", third.0)
        );
        assert_eq!(
            capture.tab_groups[2].group_id,
            format!("group:editor:{}", third_group.value())
        );
        assert_eq!(
            capture.tab_groups[2].ordered_tabs[0].tab_id,
            format!("tab:editor:{}:ld:dddd", fourth.0)
        );

        let Some(TabGroupLayoutCapture::Split {
            split_id,
            orientation,
            children,
            weights,
        }) = capture.pane_tree_layout.as_ref()
        else {
            panic!("outer editor split")
        };
        assert_eq!(split_id, "split:editor:1");
        assert_eq!(*orientation, SplitOrientation::Vertical);
        assert_eq!(weights.as_deref(), Some(&[1.0, 1.0][..]));
        assert!(matches!(
            &children[0],
            TabGroupLayoutCapture::TabGroup { group_id }
                if group_id == &format!("group:editor:{}", first_group.value())
        ));
        let TabGroupLayoutCapture::Split {
            split_id,
            children,
            weights,
            ..
        } = &children[1]
        else {
            panic!("inner editor split")
        };
        assert_eq!(split_id, "split:editor:2");
        assert_eq!(weights.as_deref(), Some(&[1.0, 1.0][..]));
        assert!(matches!(
            &children[0],
            TabGroupLayoutCapture::TabGroup { group_id }
                if group_id == &format!("group:editor:{}", second_group.value())
        ));
        assert!(matches!(
            &children[1],
            TabGroupLayoutCapture::TabGroup { group_id }
                if group_id == &format!("group:editor:{}", third_group.value())
        ));
    }

    #[test]
    fn missing_target_is_a_named_placeholder_without_a_raw_path() {
        let mut frame = DesktopFrame::new(1280, 720);
        let tab = frame.open_tab().expect("tab");
        let metadata = vec![EditorTabCaptureMetadata {
            tab_id: tab,
            logical_document_ref: Some("ld:missing-target".to_string()),
            display_label: Some("/Users/alice/private/workspace/missing.rs".to_string()),
            pinned: false,
            dirty_badge_visible: false,
            target_state: EditorRestoreTargetState::MissingFile,
            dirty_journal_identity: None,
        }];

        let capture = materialize_live_session_capture(LiveSessionRestoreCapture {
            frame: &frame,
            editor_tabs: &metadata,
            terminal_snapshot: None,
            context: &context(TrustState::Trusted),
        })
        .expect("capture input");
        let persisted = &capture.tab_groups[0].ordered_tabs[0];
        assert_eq!(persisted.surface_role, SurfaceRole::Placeholder);
        assert_eq!(persisted.surface_class, SurfaceClass::PlaceholderCard);
        assert_eq!(persisted.tab_label.as_deref(), Some("missing.rs"));
        assert_eq!(
            persisted.surface_binding_ref.as_deref(),
            Some("ld:missing-target")
        );
        assert!(capture.downgrade_triggers.iter().any(|trigger| {
            trigger.trigger_class == DowngradeTriggerClass::ManualRepairRequired
        }));
        assert!(!persisted
            .tab_label
            .as_deref()
            .unwrap_or_default()
            .contains("/Users/alice"));
        assert!(persisted.tab_id.contains("ld:missing-target"));
    }

    #[test]
    fn dirty_identity_and_badge_persist_without_note_or_source_bytes() {
        let dir = tempfile::tempdir().expect("tempdir");
        let mut frame = DesktopFrame::new(1280, 720);
        let tab = frame.open_tab().expect("tab");
        let metadata = vec![file_metadata(
            tab,
            "ld:dirty-doc",
            "/workspace/secret/dirty.rs",
            true,
        )];
        let capture_context = context(TrustState::Trusted);
        let mut store = SessionRestoreStore::new(dir.path());
        let refs = capture_live_session(
            &mut store,
            LiveSessionRestoreCapture {
                frame: &frame,
                editor_tabs: &metadata,
                terminal_snapshot: None,
                context: &capture_context,
            },
        )
        .expect("persist capture");

        let checkpoint = store
            .load_checkpoint(&refs.checkpoint_id)
            .expect("checkpoint");
        assert_eq!(checkpoint.dirty_buffer_journal_identities.len(), 1);
        assert_eq!(
            checkpoint.dirty_buffer_journal_identities[0].journal_id,
            format!("journal:dirty:{}", tab.0)
        );
        assert_eq!(checkpoint.dirty_buffer_journal_identities[0].note, None);
        let exact_build_ref = build_info::exact_build_identity_ref();
        assert_eq!(
            checkpoint
                .producer_build
                .producer_instance_handle
                .as_deref(),
            Some(exact_build_ref.as_str())
        );
        assert!(checkpoint.emitted_at.starts_with("mono:unix:"));
        assert_eq!(checkpoint.trusted_root_refs[0].trust_state, "trusted");

        let body = store
            .load_pane_tree_body(&refs.snapshot_id)
            .expect("pane tree body");
        let body_json = serde_json::to_string(&body).expect("serialize body");
        assert!(body_json.contains("\"surface_binding_ref\":\"ld:dirty-doc\""));
        assert!(body_json.contains("\"dirty_badge_visible\":true"));
        assert!(!body_json.contains("/workspace/secret"));
        assert!(!body_json.contains("must not cross capture boundary"));
    }

    #[test]
    fn terminal_capture_excludes_live_authority_and_cannot_rerun() {
        let dir = tempfile::tempdir().expect("tempdir");
        let frame = DesktopFrame::new(1280, 720);
        let mut host = PtyHost::new();
        let session_id = host.open_session(OpenSessionRequest {
            workspace_id: "workspace:test",
            host_class: HostClass::RemoteAgentPrimary,
            display_title: "/usr/bin/zsh",
            cwd_hint: Some("/Users/alice/private/workspace/service"),
            execution_context_ref: "execution:secret-context",
            trust_state: TrustState::Trusted,
            observed_at: "mono:test:1",
        });
        let terminal_snapshot = TerminalPaneSnapshot::project("workspace:test", &host);
        let capture_context = context(TrustState::Trusted);
        let mut store = SessionRestoreStore::new(dir.path());
        let refs = capture_live_session(
            &mut store,
            LiveSessionRestoreCapture {
                frame: &frame,
                editor_tabs: &[],
                terminal_snapshot: Some(&terminal_snapshot),
                context: &capture_context,
            },
        )
        .expect("persist terminal capture");

        let snapshot = store
            .load_window_topology_snapshot(&refs.snapshot_id)
            .expect("snapshot");
        let terminal = snapshot
            .stable_pane_id_inventory
            .iter()
            .find(|pane| pane.surface_role == SurfaceRole::Terminal)
            .expect("terminal pane");
        let restore = terminal
            .restore_metadata
            .as_ref()
            .expect("restore metadata");
        assert_eq!(restore.working_directory.as_deref(), Some("service"));
        assert_eq!(restore.shell_identity, "zsh");
        assert!(restore.auto_rerun_forbidden);
        assert!(!restore.raw_command_body_present);
        assert!(!restore.raw_environment_body_present);

        let checkpoint = store
            .load_checkpoint(&refs.checkpoint_id)
            .expect("checkpoint");
        assert_eq!(
            checkpoint.excluded_live_authority_classes,
            excluded_live_authority_classes()
        );
        let persisted_json = std::fs::read_to_string(
            store
                .root_path()
                .join("window_topology_snapshots")
                .join(format!("{}.json", refs.snapshot_id)),
        )
        .expect("snapshot bytes");
        assert!(!persisted_json.contains(session_id.as_str()));
        assert!(!persisted_json.contains("execution:secret-context"));
        assert!(!persisted_json.contains("/Users/alice"));

        let crash_store = CrashJournalStore::new(dir.path());
        let proposal =
            RestoreProposal::build(&store, &crash_store, false).expect("restore proposal");
        assert!(proposal.auto_rerun_forbidden);
        let mut runtime = RestoreRuntime::new(&store, &crash_store);
        let outcome = proposal.execute(&mut runtime);
        assert_eq!(outcome.blocked_side_effectful_count(), 1);
        assert!(outcome.pane_outcomes.iter().any(|pane| {
            pane.surface_role == SurfaceRole::Terminal
                && pane.execution_kind == RestorePaneExecutionKind::BlockedSideEffectful
        }));
    }

    #[test]
    fn raw_terminal_payload_posture_fails_closed() {
        let frame = DesktopFrame::new(1280, 720);
        let mut host = PtyHost::new();
        host.open_session(OpenSessionRequest {
            workspace_id: "workspace:test",
            host_class: HostClass::RemoteAgentPrimary,
            display_title: "zsh",
            cwd_hint: None,
            execution_context_ref: "execution:test",
            trust_state: TrustState::Trusted,
            observed_at: "mono:test:1",
        });
        let mut terminal_snapshot = TerminalPaneSnapshot::project("workspace:test", &host);
        terminal_snapshot.tabs[0]
            .restore_metadata
            .raw_command_body_present = true;

        let err = materialize_live_session_capture(LiveSessionRestoreCapture {
            frame: &frame,
            editor_tabs: &[],
            terminal_snapshot: Some(&terminal_snapshot),
            context: &context(TrustState::Trusted),
        })
        .expect_err("raw command posture must fail closed");
        assert!(matches!(
            err,
            LiveSessionCaptureError::ForbiddenTerminalPayloadFlag
        ));
    }
}
