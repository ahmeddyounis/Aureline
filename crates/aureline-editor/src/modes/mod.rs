//! Editor keyboard-mode safety records for preset keymap surfaces.
//!
//! This module freezes the editor-facing state that status strips, sequence
//! guides, register pickers, macro review, settings, help, and support exports
//! consume. It intentionally models the bounded alpha contract rather than a
//! full modal-editor implementation.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Boundary schema version for [`EditorModeStateRecord`].
pub const MODE_STATE_SCHEMA_VERSION: u32 = 1;

/// Canonical user-visible editor mode vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditorModeClass {
    /// Ordinary text insertion semantics.
    Insert,
    /// Modal command/navigation semantics.
    Normal,
    /// Range-oriented modal semantics.
    Visual,
    /// Command-line or command-prefix semantics.
    Command,
    /// Modeless keymap surface that still exposes source and sequence truth.
    Modeless,
}

impl EditorModeClass {
    /// Returns the stable schema token for this editor mode.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Insert => "insert",
            Self::Normal => "normal",
            Self::Visual => "visual",
            Self::Command => "command",
            Self::Modeless => "modeless",
        }
    }
}

/// Canonical state vocabulary for visible sequence guidance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SequenceGuideState {
    /// No prefix is pending and the guide can stay compact.
    Ready,
    /// A prefix is waiting for the next stroke.
    Partial,
    /// More than one command or group can still win.
    Ambiguous,
    /// The sequence is blocked by policy, host, or command posture.
    Blocked,
    /// The current surface cannot honor the sequence faithfully.
    UnsupportedSurface,
}

impl SequenceGuideState {
    /// Returns the stable schema token for this sequence-guide state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Partial => "partial",
            Self::Ambiguous => "ambiguous",
            Self::Blocked => "blocked",
            Self::UnsupportedSurface => "unsupported_surface",
        }
    }
}

/// Canonical register and clipboard route vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegisterRouteKind {
    /// Editor-local unnamed or numbered register storage.
    EditorLocal,
    /// Host system clipboard route.
    SystemClipboard,
    /// Bridge between a remote workspace and local desktop clipboard.
    RemoteClipboardBridge,
    /// User-selected named register.
    NamedRegister,
    /// Last-search or search-history register.
    SearchRegister,
    /// Macro recording or replay register.
    MacroRegister,
    /// Route that exists only to explain a policy denial.
    PolicyBlocked,
}

impl RegisterRouteKind {
    /// Returns the stable schema token for this route kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorLocal => "editor_local",
            Self::SystemClipboard => "system_clipboard",
            Self::RemoteClipboardBridge => "remote_clipboard_bridge",
            Self::NamedRegister => "named_register",
            Self::SearchRegister => "search_register",
            Self::MacroRegister => "macro_register",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// Availability and safety posture for one register route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegisterRouteAvailability {
    /// Route can execute with the visible label.
    Available,
    /// Route can execute only after a visible review.
    RequiresReview,
    /// Policy denied the route and no nearby route may be substituted.
    BlockedByPolicy,
    /// Current surface or host does not support the route.
    Unsupported,
}

impl RegisterRouteAvailability {
    /// Returns the stable schema token for this route availability.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::RequiresReview => "requires_review",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Outcome class for a macro replay review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroReplayOutcomeClass {
    /// Replay stays within one editor buffer and can proceed.
    LocalEditorOnly,
    /// Replay needs review before execution.
    RequiresReview,
    /// Replay is denied closed with a visible reason.
    Rejected,
}

impl MacroReplayOutcomeClass {
    /// Returns the stable schema token for this macro outcome.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalEditorOnly => "local_editor_only",
            Self::RequiresReview => "requires_review",
            Self::Rejected => "rejected",
        }
    }
}

/// Keyboard-reachable action exposed by mode safety surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModeRecoveryAction {
    /// Stable action reference.
    pub action_ref: String,
    /// Human-facing label.
    pub label: String,
    /// Canonical command id when the action is command-backed.
    pub command_id: Option<String>,
    /// Keyboard route exposed to users and assistive technology.
    pub keyboard_route: String,
}

/// One option row in a leader or sequence guide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SequenceGuideOption {
    /// Next key or group token shown in the guide.
    pub next_key_label: String,
    /// Stable command id resolved by this option, when known.
    pub command_id: Option<String>,
    /// Source label such as core, imported bridge, or extension.
    pub source_label: String,
    /// State token for this option.
    pub state: SequenceGuideState,
}

/// Visible sequence-guide state for a partial, ambiguous, blocked, or unsupported sequence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SequenceGuideRecord {
    /// Stable sequence-guide id.
    pub guide_id: String,
    /// Literal prefix safe to show locally.
    pub typed_prefix: String,
    /// Current state for this prefix.
    pub sequence_state: SequenceGuideState,
    /// Source preset whose key meaning is being explained.
    pub source_preset_ref: String,
    /// Timeout or wait posture shown to the user.
    pub timeout_posture: String,
    /// Candidate next keys or groups.
    pub available_next_keys: Vec<SequenceGuideOption>,
    /// Visible note for unsupported, blocked, or narrowed behavior.
    pub unsupported_or_blocked_note: Option<String>,
    /// Conflict review record when ambiguity requires review.
    pub conflict_review_ref: Option<String>,
    /// Keyboard route for opening the full keymap diagnostics surface.
    pub diagnostics_route_ref: String,
    /// Short screen-reader phrase for this guide.
    pub accessibility_announcement: String,
}

/// Register, clipboard, or macro route shown before paste or replay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisterRouteRecord {
    /// Stable route id.
    pub route_ref: String,
    /// Canonical route kind.
    pub route_kind: RegisterRouteKind,
    /// Human-facing route label.
    pub route_label: String,
    /// Target reference, redacted when necessary.
    pub target_ref: String,
    /// Availability and fail-closed posture.
    pub availability: RegisterRouteAvailability,
    /// True when the route can mutate editor or workspace state.
    pub destructive_action_requires_review: bool,
    /// True when blocked or unsupported routes must not approximate a nearby route.
    pub fail_closed: bool,
    /// Visible reason for review, block, or unsupported posture.
    pub visible_reason: String,
    /// Review or detail action for the route.
    pub review_action_ref: String,
    /// Screen-reader label for this route.
    pub accessibility_label: String,
}

/// Scope preview for an operator-pending modal command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingOperatorRecord {
    /// Stable pending-operator id.
    pub pending_operator_ref: String,
    /// Operator key or label.
    pub operator_label: String,
    /// Repetition count, if present.
    pub count: Option<u32>,
    /// Expected object or motion class.
    pub object_class: String,
    /// Scope preview route shown before execution.
    pub scope_preview_ref: String,
    /// Canonical command id when resolution is known.
    pub command_id: Option<String>,
    /// True when the pending operator can destroy or replace content.
    pub destructive: bool,
    /// Cancel route shown next to the pending state.
    pub cancel_route_ref: String,
}

/// Review record for macro replay scope and side effects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroReplayReviewRecord {
    /// Stable macro review id.
    pub review_ref: String,
    /// Macro register used as the replay source.
    pub source_register_ref: String,
    /// Target scope class for the replay.
    pub target_scope_class: String,
    /// Write or side-effect classes touched by the replay.
    pub write_classes_touched: Vec<String>,
    /// True when replay would leave the active file.
    pub crosses_files: bool,
    /// True when replay would invoke run-capable commands.
    pub invokes_run_capable_commands: bool,
    /// True when replay would mutate settings or profiles.
    pub mutates_settings: bool,
    /// True when replay depends on unstable timing.
    pub relies_on_unstable_timing: bool,
    /// Review outcome class.
    pub outcome_class: MacroReplayOutcomeClass,
    /// Visible explanation for the outcome.
    pub visible_reason: String,
    /// Next safe actions shown by the review.
    pub next_safe_actions: Vec<ModeRecoveryAction>,
}

/// Input used to build a bounded alpha mode-state record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlphaModeStateInput {
    /// Stable record id.
    pub mode_state_id: String,
    /// Source preset ref such as `preset:keymap:vim`.
    pub source_preset_ref: String,
    /// Human-facing preset label.
    pub source_preset_label: String,
    /// Current editor mode for the visible surface.
    pub current_mode: EditorModeClass,
    /// Surface whose key semantics are being described.
    pub surface_ref: String,
    /// Platform token used by shortcut labels.
    pub platform_class: String,
}

/// Canonical keyboard-mode state consumed by editor chrome and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorModeStateRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for the record.
    pub schema_version: u32,
    /// Stable mode-state id.
    pub mode_state_id: String,
    /// Current user-visible editor mode.
    pub current_mode: EditorModeClass,
    /// Active preset source ref.
    pub source_preset_ref: String,
    /// Active preset source label.
    pub source_preset_label: String,
    /// Coarse source-kind token used by telemetry and settings.
    pub keymap_source_kind: String,
    /// Read/write posture for the current surface.
    pub read_write_posture: String,
    /// Surface whose key semantics are described.
    pub surface_ref: String,
    /// Platform token used by shortcuts in this record.
    pub platform_class: String,
    /// Partial, ambiguous, blocked, or unsupported sequence guides.
    pub sequence_guides: Vec<SequenceGuideRecord>,
    /// Register, clipboard, and macro routes visible before paste or replay.
    pub register_routes: Vec<RegisterRouteRecord>,
    /// Pending operator scope preview.
    pub pending_operator: Option<PendingOperatorRecord>,
    /// Macro replay review or rejection records.
    pub macro_replay_reviews: Vec<MacroReplayReviewRecord>,
    /// Keyboard-reachable recovery actions.
    pub recovery_actions: Vec<ModeRecoveryAction>,
    /// Screen-reader phrases emitted for current state.
    pub accessibility_announcements: Vec<String>,
    /// Support-safe packet refs that explain the state.
    pub support_export_refs: Vec<String>,
}

impl EditorModeStateRecord {
    /// Stable record-kind tag carried in serialized mode-state records.
    pub const RECORD_KIND: &'static str = "editor_mode_state_record";

    /// Returns true when every required register route kind is represented.
    pub fn covers_required_register_routes(&self) -> bool {
        let observed = self
            .register_routes
            .iter()
            .map(|route| route.route_kind)
            .collect::<BTreeSet<_>>();
        [
            RegisterRouteKind::EditorLocal,
            RegisterRouteKind::SystemClipboard,
            RegisterRouteKind::RemoteClipboardBridge,
            RegisterRouteKind::NamedRegister,
            RegisterRouteKind::SearchRegister,
            RegisterRouteKind::MacroRegister,
            RegisterRouteKind::PolicyBlocked,
        ]
        .iter()
        .all(|required| observed.contains(required))
    }

    /// Returns true when blocked or unsupported routes fail closed with a reason.
    pub fn blocked_or_unsupported_routes_fail_closed(&self) -> bool {
        self.register_routes.iter().all(|route| {
            let narrowed = matches!(
                route.availability,
                RegisterRouteAvailability::BlockedByPolicy | RegisterRouteAvailability::Unsupported
            );
            !narrowed || (route.fail_closed && !route.visible_reason.trim().is_empty())
        })
    }

    /// Returns true when unsafe macro replays are reviewed or rejected.
    pub fn unsafe_macro_replays_are_bounded(&self) -> bool {
        self.macro_replay_reviews.iter().all(|review| {
            let unsafe_replay = review.crosses_files
                || review.invokes_run_capable_commands
                || review.mutates_settings
                || review.relies_on_unstable_timing;
            !unsafe_replay
                || matches!(
                    review.outcome_class,
                    MacroReplayOutcomeClass::RequiresReview | MacroReplayOutcomeClass::Rejected
                )
        })
    }

    /// Returns true when partial and unsupported sequence states are visible.
    pub fn exposes_partial_and_unsupported_sequences(&self) -> bool {
        let observed = self
            .sequence_guides
            .iter()
            .map(|guide| guide.sequence_state)
            .collect::<BTreeSet<_>>();
        observed.contains(&SequenceGuideState::Partial)
            && observed.contains(&SequenceGuideState::UnsupportedSurface)
            && self
                .sequence_guides
                .iter()
                .filter(|guide| guide.sequence_state == SequenceGuideState::UnsupportedSurface)
                .all(|guide| guide.unsupported_or_blocked_note.is_some())
    }

    /// Returns true when the record exposes diagnostics, command search, and reset paths.
    pub fn has_required_recovery_paths(&self) -> bool {
        let action_refs = self
            .recovery_actions
            .iter()
            .map(|action| action.action_ref.as_str())
            .collect::<BTreeSet<_>>();
        action_refs.contains("surface:help.keybinding_inspector")
            && action_refs.contains("surface:command_palette.search")
            && action_refs.contains("mode-reset:default_safe_entry")
    }
}

/// Builds the bounded alpha mode-state record for one preset lane.
pub fn build_alpha_mode_state_record(input: AlphaModeStateInput) -> EditorModeStateRecord {
    let recovery_actions = vec![
        ModeRecoveryAction {
            action_ref: "surface:help.keybinding_inspector".to_string(),
            label: "Open keymap diagnostics".to_string(),
            command_id: None,
            keyboard_route: "Ctrl+I from the shell keybinding inspector".to_string(),
        },
        ModeRecoveryAction {
            action_ref: "surface:command_palette.search".to_string(),
            label: "Search commands".to_string(),
            command_id: Some("cmd:command_palette.open".to_string()),
            keyboard_route: "Use the active preset shortcut for the command palette".to_string(),
        },
        ModeRecoveryAction {
            action_ref: "mode-reset:default_safe_entry".to_string(),
            label: "Reset to safe entry mode".to_string(),
            command_id: None,
            keyboard_route: "Esc twice, then choose Reset mode from the mode strip".to_string(),
        },
    ];

    EditorModeStateRecord {
        record_kind: EditorModeStateRecord::RECORD_KIND.to_string(),
        schema_version: MODE_STATE_SCHEMA_VERSION,
        mode_state_id: input.mode_state_id,
        current_mode: input.current_mode,
        source_preset_ref: input.source_preset_ref.clone(),
        source_preset_label: input.source_preset_label,
        keymap_source_kind: "profile_override".to_string(),
        read_write_posture: "source_editor_writable".to_string(),
        surface_ref: input.surface_ref,
        platform_class: input.platform_class,
        sequence_guides: alpha_sequence_guides(&input.source_preset_ref),
        register_routes: alpha_register_routes(),
        pending_operator: Some(PendingOperatorRecord {
            pending_operator_ref: "pending-operator:delete-line:alpha".to_string(),
            operator_label: "delete".to_string(),
            count: Some(3),
            object_class: "line_range".to_string(),
            scope_preview_ref: "scope-preview:current-buffer.lines-3".to_string(),
            command_id: Some("cmd:editor.cut".to_string()),
            destructive: true,
            cancel_route_ref: "route:mode.pending.cancel_escape".to_string(),
        }),
        macro_replay_reviews: alpha_macro_reviews(&recovery_actions),
        recovery_actions,
        accessibility_announcements: vec![
            format!("Mode changed to {}", input.current_mode.as_str()),
            "Delete pending for three lines; scope preview available".to_string(),
            "Remote clipboard route unavailable; paste blocked closed".to_string(),
            "Macro replay crosses the current file and requires review".to_string(),
        ],
        support_export_refs: vec![
            "artifacts/commands/alpha_mode_state_parity_report.json".to_string(),
            "fixtures/editor/mode_and_orientation/alpha_mode_and_orientation_cases.json"
                .to_string(),
            "docs/ux/keyboard_mode_orientation_alpha.md".to_string(),
        ],
    }
}

fn alpha_sequence_guides(source_preset_ref: &str) -> Vec<SequenceGuideRecord> {
    vec![
        SequenceGuideRecord {
            guide_id: "sequence-guide:leader-run-alpha".to_string(),
            typed_prefix: "<leader> r".to_string(),
            sequence_state: SequenceGuideState::Partial,
            source_preset_ref: source_preset_ref.to_string(),
            timeout_posture: "waiting_for_next_stroke".to_string(),
            available_next_keys: vec![
                SequenceGuideOption {
                    next_key_label: "t".to_string(),
                    command_id: Some("cmd:test.rerun_last".to_string()),
                    source_label: "core".to_string(),
                    state: SequenceGuideState::Ready,
                },
                SequenceGuideOption {
                    next_key_label: "r".to_string(),
                    command_id: Some("cmd:task.rerun_last".to_string()),
                    source_label: "core".to_string(),
                    state: SequenceGuideState::Ready,
                },
            ],
            unsupported_or_blocked_note: None,
            conflict_review_ref: None,
            diagnostics_route_ref: "surface:help.keybinding_inspector".to_string(),
            accessibility_announcement: "Leader r is waiting for the next key".to_string(),
        },
        SequenceGuideRecord {
            guide_id: "sequence-guide:terminal-operator-unsupported".to_string(),
            typed_prefix: "3d".to_string(),
            sequence_state: SequenceGuideState::UnsupportedSurface,
            source_preset_ref: source_preset_ref.to_string(),
            timeout_posture: "stopped_before_dispatch".to_string(),
            available_next_keys: Vec::new(),
            unsupported_or_blocked_note: Some(
                "Terminal passthrough cannot honor editor delete motions; sequence stopped without approximation."
                    .to_string(),
            ),
            conflict_review_ref: None,
            diagnostics_route_ref: "surface:help.keybinding_inspector".to_string(),
            accessibility_announcement:
                "Delete motion unsupported in terminal passthrough; no action taken.".to_string(),
        },
    ]
}

fn alpha_register_routes() -> Vec<RegisterRouteRecord> {
    vec![
        RegisterRouteRecord {
            route_ref: "register-route:editor-local:unnamed".to_string(),
            route_kind: RegisterRouteKind::EditorLocal,
            route_label: "Local editor register".to_string(),
            target_ref: "register:editor.unnamed".to_string(),
            availability: RegisterRouteAvailability::Available,
            destructive_action_requires_review: false,
            fail_closed: false,
            visible_reason: "Yank and paste remain inside the active editor register.".to_string(),
            review_action_ref: "surface:mode_strip.register_picker".to_string(),
            accessibility_label: "Local editor register available".to_string(),
        },
        RegisterRouteRecord {
            route_ref: "register-route:system-clipboard".to_string(),
            route_kind: RegisterRouteKind::SystemClipboard,
            route_label: "System clipboard".to_string(),
            target_ref: "clipboard:system".to_string(),
            availability: RegisterRouteAvailability::Available,
            destructive_action_requires_review: false,
            fail_closed: false,
            visible_reason: "Paste targets the host system clipboard route.".to_string(),
            review_action_ref: "surface:mode_strip.register_picker".to_string(),
            accessibility_label: "System clipboard route available".to_string(),
        },
        RegisterRouteRecord {
            route_ref: "register-route:remote-clipboard-bridge".to_string(),
            route_kind: RegisterRouteKind::RemoteClipboardBridge,
            route_label: "Remote clipboard bridge".to_string(),
            target_ref: "clipboard:remote.bridge".to_string(),
            availability: RegisterRouteAvailability::Unsupported,
            destructive_action_requires_review: true,
            fail_closed: true,
            visible_reason:
                "No remote clipboard bridge is attached for this alpha surface; paste is blocked instead of using another route."
                    .to_string(),
            review_action_ref: "surface:mode_strip.register_picker.remote_bridge".to_string(),
            accessibility_label: "Remote clipboard bridge unavailable; paste blocked".to_string(),
        },
        RegisterRouteRecord {
            route_ref: "register-route:named:a".to_string(),
            route_kind: RegisterRouteKind::NamedRegister,
            route_label: "Named register a".to_string(),
            target_ref: "register:named:a".to_string(),
            availability: RegisterRouteAvailability::RequiresReview,
            destructive_action_requires_review: true,
            fail_closed: false,
            visible_reason: "Non-default named-register paste previews the target before replacing text."
                .to_string(),
            review_action_ref: "surface:mode_strip.register_picker.named".to_string(),
            accessibility_label: "Named register a requires paste review".to_string(),
        },
        RegisterRouteRecord {
            route_ref: "register-route:search:last".to_string(),
            route_kind: RegisterRouteKind::SearchRegister,
            route_label: "Search register".to_string(),
            target_ref: "register:search.last_query".to_string(),
            availability: RegisterRouteAvailability::RequiresReview,
            destructive_action_requires_review: false,
            fail_closed: false,
            visible_reason: "Search-register paste is labeled because it can replace the query text."
                .to_string(),
            review_action_ref: "surface:find.search_register_detail".to_string(),
            accessibility_label: "Search register route labeled before paste".to_string(),
        },
        RegisterRouteRecord {
            route_ref: "register-route:macro:q".to_string(),
            route_kind: RegisterRouteKind::MacroRegister,
            route_label: "Macro register q".to_string(),
            target_ref: "register:macro:q".to_string(),
            availability: RegisterRouteAvailability::RequiresReview,
            destructive_action_requires_review: true,
            fail_closed: false,
            visible_reason: "Macro replay opens review when it may leave editor-local scope.".to_string(),
            review_action_ref: "surface:macro_replay.review".to_string(),
            accessibility_label: "Macro register q requires replay review".to_string(),
        },
        RegisterRouteRecord {
            route_ref: "register-route:policy-blocked:secret".to_string(),
            route_kind: RegisterRouteKind::PolicyBlocked,
            route_label: "Policy-blocked clipboard route".to_string(),
            target_ref: "policy:clipboard.secret_material_denied".to_string(),
            availability: RegisterRouteAvailability::BlockedByPolicy,
            destructive_action_requires_review: true,
            fail_closed: true,
            visible_reason:
                "Managed policy blocks this clipboard route; Aureline will not fall back to the system clipboard."
                    .to_string(),
            review_action_ref: "surface:policy.effective_rule_detail".to_string(),
            accessibility_label: "Clipboard route blocked by policy; no fallback used".to_string(),
        },
    ]
}

fn alpha_macro_reviews(recovery_actions: &[ModeRecoveryAction]) -> Vec<MacroReplayReviewRecord> {
    vec![
        MacroReplayReviewRecord {
            review_ref: "macro-review:q-cross-file".to_string(),
            source_register_ref: "register:macro:q".to_string(),
            target_scope_class: "workspace_two_files".to_string(),
            write_classes_touched: vec!["text_edit".to_string(), "cross_file_edit".to_string()],
            crosses_files: true,
            invokes_run_capable_commands: false,
            mutates_settings: false,
            relies_on_unstable_timing: false,
            outcome_class: MacroReplayOutcomeClass::RequiresReview,
            visible_reason: "Replay touches another file, so it opens scope review before execution."
                .to_string(),
            next_safe_actions: vec![
                recovery_actions[1].clone(),
                ModeRecoveryAction {
                    action_ref: "surface:macro_replay.scope_review".to_string(),
                    label: "Review target".to_string(),
                    command_id: None,
                    keyboard_route: "Enter on the macro review row".to_string(),
                },
            ],
        },
        MacroReplayReviewRecord {
            review_ref: "macro-review:q-run-settings-timing".to_string(),
            source_register_ref: "register:macro:q".to_string(),
            target_scope_class: "project_command_sequence".to_string(),
            write_classes_touched: vec![
                "run_capable_command".to_string(),
                "settings_mutation".to_string(),
                "timing_sensitive_replay".to_string(),
            ],
            crosses_files: false,
            invokes_run_capable_commands: true,
            mutates_settings: true,
            relies_on_unstable_timing: true,
            outcome_class: MacroReplayOutcomeClass::Rejected,
            visible_reason:
                "Replay would run commands, mutate settings, and rely on timing; save it as a reviewed recipe instead."
                    .to_string(),
            next_safe_actions: vec![ModeRecoveryAction {
                action_ref: "surface:recipe.review_from_macro".to_string(),
                label: "Save as reviewed recipe".to_string(),
                command_id: None,
                keyboard_route: "Enter on Save as recipe from the replay warning".to_string(),
            }],
        },
    ]
}
