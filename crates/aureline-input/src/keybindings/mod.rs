use std::collections::HashMap;
use std::sync::OnceLock;

use aureline_commands::{CommandId, CommandRevisionRef, PolicyContext};
use serde::{Deserialize, Serialize};

/// Canonical platform vocabulary used by the keybinding resolver boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformClass {
    Macos,
    Windows,
    Linux,
    Web,
    CrossPlatform,
}

/// Canonical surface-support vocabulary used by sequence-help and inspection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceSupportClass {
    FullySupported,
    MultiStrokeLimited,
    LeaderLimited,
    Unsupported,
}

/// Canonical layer precedence model for shortcut resolution.
///
/// Earlier variants in the contract outrank later variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolverLayerClass {
    PlatformReserved,
    EmergencySecurityHardBlock,
    AdminPolicyLock,
    TemporaryModeOverlay,
    UserProfileBinding,
    WorkspaceRecommendation,
    ExtensionBinding,
    CoreDefault,
}

impl ResolverLayerClass {
    pub const fn precedence_rank(self) -> u8 {
        match self {
            Self::PlatformReserved => 0,
            Self::EmergencySecurityHardBlock => 1,
            Self::AdminPolicyLock => 2,
            Self::TemporaryModeOverlay => 3,
            Self::UserProfileBinding => 4,
            Self::WorkspaceRecommendation => 5,
            Self::ExtensionBinding => 6,
            Self::CoreDefault => 7,
        }
    }

    pub const fn in_precedence_order() -> [Self; 8] {
        [
            Self::PlatformReserved,
            Self::EmergencySecurityHardBlock,
            Self::AdminPolicyLock,
            Self::TemporaryModeOverlay,
            Self::UserProfileBinding,
            Self::WorkspaceRecommendation,
            Self::ExtensionBinding,
            Self::CoreDefault,
        ]
    }
}

/// Canonical sequence-shape vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SequenceShapeClass {
    SingleStroke,
    MultiStrokeChord,
    LeaderSequence,
    OperatorPendingSequence,
}

/// Canonical resolution-state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SequenceResolutionState {
    Resolved,
    WaitingForNextStroke,
    TimedOut,
    Unbound,
    BlockedByHost,
    BlockedBySecurity,
    BlockedByPolicy,
    DisabledCommand,
    UnsupportedOnSurface,
}

/// Canonical reason-code vocabulary for resolution and loss explainability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolutionReasonCode {
    PlatformReservedBeforeDispatch,
    EmergencySecurityBlockedDispatch,
    AdminPolicyLockedBinding,
    TemporaryModeOverlayPreempted,
    HigherPrecedenceBindingWon,
    MoreSpecificBindingWon,
    PartialSequenceWaitingForNextStroke,
    SequenceTimeoutExpired,
    DisabledReasonInheritedFromDescriptor,
    HostCapturePreventsDispatch,
    SurfaceCannotHonorSequence,
    NoCandidateBound,
    SameLayerCollisionRequiresReview,
}

/// Canonical winner-kind vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WinningResolutionKind {
    CommandCandidate,
    PlatformReserved,
    EmergencySecurityHardBlock,
    AdminPolicyLock,
    WaitingState,
    Unbound,
}

/// Minimal sequence descriptor carried in resolver packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SequenceDescriptor {
    pub literal_sequence: String,
    pub shape_class: SequenceShapeClass,
    pub stroke_count: usize,
    pub leader_key_ref: Option<String>,
}

/// Active inspection scope carried in resolver packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectionScope {
    pub platform_class: PlatformClass,
    pub surface_ref: String,
    pub focus_context_ref: String,
    pub active_mode_ref: Option<String>,
    pub workspace_scope_ref: String,
    pub surface_support_class: SurfaceSupportClass,
}

/// Snapshot of command semantics used by the resolver boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandSemanticsSnapshot {
    pub command_id: CommandId,
    pub command_revision_ref: CommandRevisionRef,
    pub preview_class: String,
    pub approval_posture_class: String,
    pub capability_scope_class: String,
    pub required_evidence_ref_classes: Vec<String>,
}

/// One binding candidate considered by the resolver.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BindingCandidateRecord {
    pub candidate_ref: String,
    pub command: CommandSemanticsSnapshot,
    pub bound_sequence: SequenceDescriptor,
    pub resolver_layer: ResolverLayerClass,
    pub source_provenance_ref: Option<String>,
    pub imported_from_ref: Option<String>,
    pub publisher_or_provider_ref: Option<String>,
    pub scope_note: Option<String>,
}

/// Canonical precedence-trace row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrecedenceTraceRow {
    pub resolver_layer: ResolverLayerClass,
    pub precedence_rank: u8,
    pub disposition: PrecedenceDisposition,
    pub candidate_ref: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrecedenceDisposition {
    NoMatch,
    CandidateVisible,
    CandidateShadowed,
    Blocked,
    Reserved,
    WaitingPrefix,
    UnsupportedOnSurface,
}

/// Canonical outcome-change condition vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeChangeConditionClass {
    RemoveHigherPrecedenceBinding,
    ExitCurrentMode,
    MoveFocusToSupportedSurface,
    GrantRequiredPolicyOrTrust,
    DisableExtensionBinding,
    RebindSequence,
    ChooseDifferentImportMapping,
    WaitForNextStroke,
    AdjustSequenceTimeout,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutcomeChangeCondition {
    pub condition_class: OutcomeChangeConditionClass,
    pub explanation: String,
    pub resulting_layer: Option<ResolverLayerClass>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PivotActionClass {
    OpenCommandPalette,
    OpenKeybindingSettings,
    OpenCommandDocs,
    OpenConflictReview,
    OpenMigrationReport,
    OpenProfileImportReview,
    SwitchModeOrFocus,
    RetryOnSupportedSurface,
    DismissSequenceHelp,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NextSafeActionRecord {
    pub action_class: PivotActionClass,
    pub label: String,
    pub target_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LosingCandidateRecord {
    pub candidate: BindingCandidateRecord,
    pub loss_reason_code: ResolutionReasonCode,
    pub what_changes_outcome: Vec<OutcomeChangeCondition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WinningResolution {
    pub winner_kind: WinningResolutionKind,
    pub resolver_layer: Option<ResolverLayerClass>,
    pub command_candidate: Option<BindingCandidateRecord>,
    pub reason_code: ResolutionReasonCode,
    pub note: Option<String>,
}

/// Canonical keybinding resolution packet record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeybindingResolutionPacketRecord {
    pub record_kind: String,
    pub keybinding_resolver_schema_version: u32,
    pub resolution_id: String,
    pub inspected_sequence: SequenceDescriptor,
    pub inspection_scope: InspectionScope,
    pub sequence_state: SequenceResolutionState,
    pub winning_resolution: WinningResolution,
    pub precedence_trace: Vec<PrecedenceTraceRow>,
    pub losing_candidates: Vec<LosingCandidateRecord>,
    pub outcome_change_conditions: Vec<OutcomeChangeCondition>,
    pub next_safe_actions: Vec<NextSafeActionRecord>,
    pub docs_help_refs: Vec<String>,
    pub policy_context: PolicyContext,
    pub redaction_class: String,
    pub emitted_at: String,
    pub conflict_review_ref: Option<String>,
    pub disabled_explanation_ref: Option<String>,
}

/// Modifier state for one keystroke.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Modifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub cmd: bool,
}

/// One keystroke expressed as a key + modifiers.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyStroke {
    pub modifiers: Modifiers,
    pub key: String,
}

impl KeyStroke {
    pub fn parse(value: &str) -> Result<Self, ParseKeySequenceError> {
        let mut modifiers = Modifiers::default();
        let mut key: Option<String> = None;

        for raw in value.split('+').map(str::trim).filter(|v| !v.is_empty()) {
            if is_modifier_token(raw, "ctrl", &["control", "ctl"]) {
                modifiers.ctrl = true;
                continue;
            }
            if is_modifier_token(raw, "shift", &[]) {
                modifiers.shift = true;
                continue;
            }
            if is_modifier_token(raw, "alt", &["option", "opt"]) {
                modifiers.alt = true;
                continue;
            }
            if is_modifier_token(raw, "cmd", &["meta", "super", "logo", "command"]) {
                modifiers.cmd = true;
                continue;
            }

            key = Some(raw.to_string());
        }

        let key = key.ok_or_else(|| ParseKeySequenceError::MissingKey {
            stroke: value.to_string(),
        })?;

        Ok(Self { modifiers, key })
    }

    pub fn to_literal(&self) -> String {
        let mut parts: Vec<&str> = Vec::with_capacity(5);
        if self.modifiers.cmd {
            parts.push("Cmd");
        }
        if self.modifiers.ctrl {
            parts.push("Ctrl");
        }
        if self.modifiers.alt {
            parts.push("Alt");
        }
        if self.modifiers.shift {
            parts.push("Shift");
        }
        parts.push(self.key.as_str());
        parts.join("+")
    }
}

/// A sequence of one or more keystrokes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeySequence {
    strokes: Vec<KeyStroke>,
}

impl KeySequence {
    pub fn new(strokes: Vec<KeyStroke>) -> Self {
        Self { strokes }
    }

    pub fn strokes(&self) -> &[KeyStroke] {
        &self.strokes
    }

    pub fn stroke_count(&self) -> usize {
        self.strokes.len()
    }

    pub fn starts_with(&self, prefix: &KeySequence) -> bool {
        self.strokes.starts_with(prefix.strokes())
    }

    pub fn to_literal_sequence(&self) -> String {
        self.strokes
            .iter()
            .map(KeyStroke::to_literal)
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn parse_literal_sequence(value: &str) -> Result<Self, ParseKeySequenceError> {
        let strokes: Vec<KeyStroke> = value
            .split_whitespace()
            .filter(|v| !v.trim().is_empty())
            .map(KeyStroke::parse)
            .collect::<Result<Vec<_>, _>>()?;
        if strokes.is_empty() {
            return Err(ParseKeySequenceError::EmptySequence);
        }
        Ok(Self::new(strokes))
    }

    pub fn to_descriptor(&self) -> SequenceDescriptor {
        let literal_sequence = self.to_literal_sequence();
        let stroke_count = self.stroke_count();
        let (shape_class, leader_key_ref) = if stroke_count == 1 {
            (SequenceShapeClass::SingleStroke, None)
        } else if self
            .strokes
            .first()
            .map(|stroke| stroke.key.eq_ignore_ascii_case("<leader>"))
            .unwrap_or(false)
        {
            (
                SequenceShapeClass::LeaderSequence,
                Some("leader-key:default".to_string()),
            )
        } else {
            (SequenceShapeClass::MultiStrokeChord, None)
        };

        SequenceDescriptor {
            literal_sequence,
            shape_class,
            stroke_count,
            leader_key_ref,
        }
    }
}

/// Optional applicability context used for within-layer specificity.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CandidateContext {
    pub surface_ref: Option<String>,
    pub focus_context_ref: Option<String>,
    pub active_mode_ref: Option<String>,
    pub workspace_scope_ref: Option<String>,
    pub scope_specificity_rank: u8,
}

#[derive(Debug, Clone)]
struct BindingCandidate {
    record: BindingCandidateRecord,
    sequence: KeySequence,
    context: CandidateContext,
}

#[derive(Debug, Clone)]
pub struct KeybindingResolver {
    reserved_sequences: Vec<KeySequence>,
    emergency_block_active: bool,
    admin_locked_sequences: Vec<KeySequence>,
    layers: HashMap<ResolverLayerClass, Vec<BindingCandidate>>,
    docs_help_refs: Vec<String>,
    policy_context: PolicyContext,
    redaction_class: String,
}

impl KeybindingResolver {
    pub fn new(policy_context: PolicyContext) -> Self {
        Self {
            reserved_sequences: Vec::new(),
            emergency_block_active: false,
            admin_locked_sequences: Vec::new(),
            layers: HashMap::new(),
            docs_help_refs: vec![
                "docs/ux/keybinding_resolver_contract.md#canonical-precedence-model".to_string(),
            ],
            policy_context,
            redaction_class: "metadata_safe_default".to_string(),
        }
    }

    pub fn set_emergency_block_active(&mut self, active: bool) {
        self.emergency_block_active = active;
    }

    pub fn set_reserved_sequences(&mut self, sequences: Vec<KeySequence>) {
        self.reserved_sequences = sequences;
    }

    pub fn set_admin_locked_sequences(&mut self, sequences: Vec<KeySequence>) {
        self.admin_locked_sequences = sequences;
    }

    pub fn push_candidate(
        &mut self,
        layer: ResolverLayerClass,
        candidate_ref: impl Into<String>,
        command: CommandSemanticsSnapshot,
        sequence: KeySequence,
        source_provenance_ref: Option<String>,
        scope_note: Option<String>,
        context: CandidateContext,
    ) {
        let record = BindingCandidateRecord {
            candidate_ref: candidate_ref.into(),
            command,
            bound_sequence: sequence.to_descriptor(),
            resolver_layer: layer,
            source_provenance_ref,
            imported_from_ref: None,
            publisher_or_provider_ref: None,
            scope_note,
        };
        self.layers.entry(layer).or_default().push(BindingCandidate {
            record,
            sequence,
            context,
        });
    }

    pub fn resolve(&self, inspected: &KeySequence, scope: &InspectionScope) -> KeybindingResolutionPacketRecord {
        let inspected_descriptor = inspected.to_descriptor();
        let mut precedence_trace: Vec<PrecedenceTraceRow> = Vec::with_capacity(8);
        let mut losing_candidates: Vec<LosingCandidateRecord> = Vec::new();

        let mut first_exact_layer: Option<ResolverLayerClass> = None;
        let mut exact_candidates_by_layer: HashMap<ResolverLayerClass, Vec<&BindingCandidate>> =
            HashMap::new();
        let mut prefix_layers: Vec<ResolverLayerClass> = Vec::new();

        for layer in ResolverLayerClass::in_precedence_order() {
            if let Some(candidates) = self.layers.get(&layer) {
                let mut exact: Vec<&BindingCandidate> = Vec::new();
                let mut has_prefix = false;
                for candidate in candidates {
                    if !candidate_applies(candidate, scope) {
                        continue;
                    }
                    if candidate.sequence == *inspected {
                        exact.push(candidate);
                    } else if candidate.sequence.starts_with(inspected) {
                        has_prefix = true;
                    }
                }

                if !exact.is_empty() {
                    if first_exact_layer.is_none() {
                        first_exact_layer = Some(layer);
                    }
                    exact_candidates_by_layer.insert(layer, exact);
                }
                if has_prefix {
                    prefix_layers.push(layer);
                }
            }
        }

        let reserved_match = self
            .reserved_sequences
            .iter()
            .any(|seq| seq == inspected);
        let admin_locked_match = self
            .admin_locked_sequences
            .iter()
            .any(|seq| seq == inspected);

        let winner: WinningResolution;
        let sequence_state: SequenceResolutionState;
        let mut conflict_review_ref: Option<String> = None;

        if reserved_match {
            sequence_state = SequenceResolutionState::BlockedByHost;
            winner = WinningResolution {
                winner_kind: WinningResolutionKind::PlatformReserved,
                resolver_layer: Some(ResolverLayerClass::PlatformReserved),
                command_candidate: None,
                reason_code: ResolutionReasonCode::PlatformReservedBeforeDispatch,
                note: Some("Host reserved the sequence before dispatch.".to_string()),
            };
        } else if self.emergency_block_active {
            sequence_state = SequenceResolutionState::BlockedBySecurity;
            winner = WinningResolution {
                winner_kind: WinningResolutionKind::EmergencySecurityHardBlock,
                resolver_layer: Some(ResolverLayerClass::EmergencySecurityHardBlock),
                command_candidate: None,
                reason_code: ResolutionReasonCode::EmergencySecurityBlockedDispatch,
                note: Some("Emergency/security hard block denied shortcut dispatch.".to_string()),
            };
        } else if admin_locked_match {
            sequence_state = SequenceResolutionState::BlockedByPolicy;
            winner = WinningResolution {
                winner_kind: WinningResolutionKind::AdminPolicyLock,
                resolver_layer: Some(ResolverLayerClass::AdminPolicyLock),
                command_candidate: None,
                reason_code: ResolutionReasonCode::AdminPolicyLockedBinding,
                note: Some("Admin/policy lock denied shortcut dispatch.".to_string()),
            };
        } else if first_exact_layer.is_none() && !prefix_layers.is_empty() {
            sequence_state = SequenceResolutionState::WaitingForNextStroke;
            let highest_prefix_layer = prefix_layers
                .iter()
                .copied()
                .min_by_key(|layer| layer.precedence_rank())
                .unwrap();
            winner = WinningResolution {
                winner_kind: WinningResolutionKind::WaitingState,
                resolver_layer: Some(highest_prefix_layer),
                command_candidate: None,
                reason_code: ResolutionReasonCode::PartialSequenceWaitingForNextStroke,
                note: Some("Sequence is a prefix of a longer binding.".to_string()),
            };
        } else if let Some(layer) = first_exact_layer {
            let candidates = exact_candidates_by_layer.get(&layer).cloned().unwrap_or_default();
            let selection = select_winning_candidate(candidates, scope);
            match selection {
                SelectedCandidate::Winner(winning, losers_same_layer) => {
                    sequence_state = SequenceResolutionState::Resolved;
                    let reason_code = match layer {
                        ResolverLayerClass::AdminPolicyLock => {
                            ResolutionReasonCode::AdminPolicyLockedBinding
                        }
                        ResolverLayerClass::TemporaryModeOverlay => {
                            ResolutionReasonCode::TemporaryModeOverlayPreempted
                        }
                        _ => ResolutionReasonCode::HigherPrecedenceBindingWon,
                    };
                    winner = WinningResolution {
                        winner_kind: WinningResolutionKind::CommandCandidate,
                        resolver_layer: Some(layer),
                        command_candidate: Some(winning.record.clone()),
                        reason_code,
                        note: Some("Resolved to the highest-precedence matching binding.".to_string()),
                    };
                    for losing in losers_same_layer {
                        losing_candidates.push(LosingCandidateRecord {
                            candidate: losing.record.clone(),
                            loss_reason_code: ResolutionReasonCode::MoreSpecificBindingWon,
                            what_changes_outcome: vec![OutcomeChangeCondition {
                                condition_class: OutcomeChangeConditionClass::RebindSequence,
                                explanation: "Rebind or remove the winning binding.".to_string(),
                                resulting_layer: Some(layer),
                            }],
                        });
                    }
                }
                SelectedCandidate::Collision(layer) => {
                    sequence_state = SequenceResolutionState::Unbound;
                    conflict_review_ref = Some(format!(
                        "keybinding-conflict-review:{}:{}",
                        inspected_descriptor.literal_sequence.replace(' ', "_").replace('+', "-"),
                        layer.precedence_rank()
                    ));
                    winner = WinningResolution {
                        winner_kind: WinningResolutionKind::Unbound,
                        resolver_layer: Some(layer),
                        command_candidate: None,
                        reason_code: ResolutionReasonCode::SameLayerCollisionRequiresReview,
                        note: Some("Multiple equally-specific candidates exist; review required.".to_string()),
                    };
                }
            }
        } else {
            sequence_state = SequenceResolutionState::Unbound;
            winner = WinningResolution {
                winner_kind: WinningResolutionKind::Unbound,
                resolver_layer: None,
                command_candidate: None,
                reason_code: ResolutionReasonCode::NoCandidateBound,
                note: None,
            };
        }

        for layer in ResolverLayerClass::in_precedence_order() {
            let rank = layer.precedence_rank();
            if layer == ResolverLayerClass::PlatformReserved && reserved_match {
                precedence_trace.push(PrecedenceTraceRow {
                    resolver_layer: layer,
                    precedence_rank: rank,
                    disposition: PrecedenceDisposition::Reserved,
                    candidate_ref: None,
                    note: Some("Host shell intercepted the sequence.".to_string()),
                });
                continue;
            }
            if layer == ResolverLayerClass::EmergencySecurityHardBlock && self.emergency_block_active {
                precedence_trace.push(PrecedenceTraceRow {
                    resolver_layer: layer,
                    precedence_rank: rank,
                    disposition: PrecedenceDisposition::Blocked,
                    candidate_ref: None,
                    note: Some("Emergency/security hard block active.".to_string()),
                });
                continue;
            }
            if layer == ResolverLayerClass::AdminPolicyLock && admin_locked_match {
                precedence_trace.push(PrecedenceTraceRow {
                    resolver_layer: layer,
                    precedence_rank: rank,
                    disposition: PrecedenceDisposition::Blocked,
                    candidate_ref: None,
                    note: Some("Admin/policy lock denied dispatch.".to_string()),
                });
                continue;
            }

            let exact = exact_candidates_by_layer.get(&layer).cloned().unwrap_or_default();
            let has_prefix = prefix_layers.contains(&layer);
            let layer_won = winner
                .resolver_layer
                .is_some_and(|w| w == layer && matches!(winner.winner_kind, WinningResolutionKind::CommandCandidate));
            let disposition = if layer_won {
                PrecedenceDisposition::CandidateVisible
            } else if has_prefix && first_exact_layer.is_none() {
                PrecedenceDisposition::WaitingPrefix
            } else if !exact.is_empty() {
                PrecedenceDisposition::CandidateShadowed
            } else {
                PrecedenceDisposition::NoMatch
            };

            let candidate_ref = if layer_won {
                winner
                    .command_candidate
                    .as_ref()
                    .map(|c| c.candidate_ref.clone())
            } else {
                exact.first().map(|c| c.record.candidate_ref.clone())
            };

            precedence_trace.push(PrecedenceTraceRow {
                resolver_layer: layer,
                precedence_rank: rank,
                disposition,
                candidate_ref,
                note: None,
            });
        }

        // Add losing candidates for shadowed lower-precedence layers when a blocking
        // or higher-precedence winner exists.
        if matches!(
            winner.winner_kind,
            WinningResolutionKind::PlatformReserved
                | WinningResolutionKind::EmergencySecurityHardBlock
                | WinningResolutionKind::AdminPolicyLock
        )
            || matches!(winner.winner_kind, WinningResolutionKind::CommandCandidate)
        {
            for layer in ResolverLayerClass::in_precedence_order() {
                let exact = exact_candidates_by_layer.get(&layer).cloned().unwrap_or_default();
                if exact.is_empty() {
                    continue;
                }
                let layer_is_winner = winner.resolver_layer == Some(layer)
                    && matches!(winner.winner_kind, WinningResolutionKind::CommandCandidate);
                if layer_is_winner {
                    continue;
                }
                for candidate in exact {
                    let loss_reason = match winner.winner_kind {
                        WinningResolutionKind::PlatformReserved => {
                            ResolutionReasonCode::PlatformReservedBeforeDispatch
                        }
                        WinningResolutionKind::EmergencySecurityHardBlock => {
                            ResolutionReasonCode::EmergencySecurityBlockedDispatch
                        }
                        WinningResolutionKind::AdminPolicyLock => {
                            ResolutionReasonCode::AdminPolicyLockedBinding
                        }
                        WinningResolutionKind::CommandCandidate => {
                            ResolutionReasonCode::HigherPrecedenceBindingWon
                        }
                        _ => ResolutionReasonCode::HigherPrecedenceBindingWon,
                    };
                    losing_candidates.push(LosingCandidateRecord {
                        candidate: candidate.record.clone(),
                        loss_reason_code: loss_reason,
                        what_changes_outcome: vec![OutcomeChangeCondition {
                            condition_class: OutcomeChangeConditionClass::RemoveHigherPrecedenceBinding,
                            explanation: "Remove or rebind the higher-precedence binding.".to_string(),
                            resulting_layer: Some(layer),
                        }],
                    });
                }
            }
        }

        let outcome_change_conditions = match winner.reason_code {
            ResolutionReasonCode::NoCandidateBound => vec![OutcomeChangeCondition {
                condition_class: OutcomeChangeConditionClass::RebindSequence,
                explanation: "Bind this sequence to a command.".to_string(),
                resulting_layer: Some(ResolverLayerClass::UserProfileBinding),
            }],
            ResolutionReasonCode::SameLayerCollisionRequiresReview => vec![OutcomeChangeCondition {
                condition_class: OutcomeChangeConditionClass::ChooseDifferentImportMapping,
                explanation: "Resolve the collision by rebinding or scoping one candidate.".to_string(),
                resulting_layer: winner.resolver_layer,
            }],
            _ => Vec::new(),
        };

        let mut next_safe_actions = Vec::new();
        next_safe_actions.push(NextSafeActionRecord {
            action_class: PivotActionClass::OpenKeybindingSettings,
            label: "Open keybinding settings".to_string(),
            target_ref: Some("settings:keybindings".to_string()),
        });
        next_safe_actions.push(NextSafeActionRecord {
            action_class: PivotActionClass::OpenCommandPalette,
            label: "Open command palette".to_string(),
            target_ref: Some("surface:command_palette".to_string()),
        });

        KeybindingResolutionPacketRecord {
            record_kind: "keybinding_resolution_packet_record".to_string(),
            keybinding_resolver_schema_version: 1,
            resolution_id: format!(
                "keybinding-resolution:{}",
                inspected_descriptor
                    .literal_sequence
                    .replace(' ', "_")
                    .replace('+', "-")
                    .to_lowercase()
            ),
            inspected_sequence: inspected_descriptor,
            inspection_scope: scope.clone(),
            sequence_state,
            winning_resolution: winner,
            precedence_trace,
            losing_candidates,
            outcome_change_conditions,
            next_safe_actions,
            docs_help_refs: self.docs_help_refs.clone(),
            policy_context: self.policy_context.clone(),
            redaction_class: self.redaction_class.clone(),
            emitted_at: "now".to_string(),
            conflict_review_ref,
            disabled_explanation_ref: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseKeySequenceError {
    EmptySequence,
    MissingKey { stroke: String },
}

impl std::fmt::Display for ParseKeySequenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptySequence => write!(f, "key sequence must not be empty"),
            Self::MissingKey { stroke } => write!(f, "missing key in stroke: {stroke}"),
        }
    }
}

impl std::error::Error for ParseKeySequenceError {}

fn is_modifier_token(token: &str, canonical: &str, aliases: &[&str]) -> bool {
    if token.eq_ignore_ascii_case(canonical) {
        return true;
    }
    aliases.iter().any(|alias| token.eq_ignore_ascii_case(alias))
}

fn candidate_applies(candidate: &BindingCandidate, scope: &InspectionScope) -> bool {
    if let Some(surface_ref) = candidate.context.surface_ref.as_deref() {
        if surface_ref != scope.surface_ref {
            return false;
        }
    }
    if let Some(focus_context_ref) = candidate.context.focus_context_ref.as_deref() {
        if focus_context_ref != scope.focus_context_ref {
            return false;
        }
    }
    if let Some(active_mode_ref) = candidate.context.active_mode_ref.as_deref() {
        if scope.active_mode_ref.as_deref() != Some(active_mode_ref) {
            return false;
        }
    }
    if let Some(workspace_scope_ref) = candidate.context.workspace_scope_ref.as_deref() {
        if workspace_scope_ref != scope.workspace_scope_ref {
            return false;
        }
    }
    true
}

enum SelectedCandidate<'a> {
    Winner(&'a BindingCandidate, Vec<&'a BindingCandidate>),
    Collision(ResolverLayerClass),
}

fn select_winning_candidate<'a>(
    mut candidates: Vec<&'a BindingCandidate>,
    scope: &InspectionScope,
) -> SelectedCandidate<'a> {
    if candidates.is_empty() {
        return SelectedCandidate::Collision(ResolverLayerClass::CoreDefault);
    }
    candidates.sort_by_key(|candidate| candidate_specificity(candidate, scope));
    let best = candidates[0];
    if candidates.len() == 1 {
        return SelectedCandidate::Winner(best, Vec::new());
    }
    let best_key = candidate_specificity(best, scope);
    let ties: Vec<&BindingCandidate> = candidates
        .iter()
        .copied()
        .filter(|candidate| candidate_specificity(candidate, scope) == best_key)
        .collect();
    if ties.len() > 1 {
        return SelectedCandidate::Collision(best.record.resolver_layer);
    }
    let losers = candidates.into_iter().skip(1).collect();
    SelectedCandidate::Winner(best, losers)
}

fn candidate_specificity(candidate: &BindingCandidate, scope: &InspectionScope) -> (u8, u8, u8, u8, u8) {
    let surface = candidate
        .context
        .surface_ref
        .as_deref()
        .map(|v| (v == scope.surface_ref) as u8)
        .unwrap_or(0);
    let mode = candidate
        .context
        .active_mode_ref
        .as_deref()
        .map(|v| (scope.active_mode_ref.as_deref() == Some(v)) as u8)
        .unwrap_or(0);
    let focus = candidate
        .context
        .focus_context_ref
        .as_deref()
        .map(|v| (v == scope.focus_context_ref) as u8)
        .unwrap_or(0);
    let workspace = candidate
        .context
        .workspace_scope_ref
        .as_deref()
        .map(|v| (v == scope.workspace_scope_ref) as u8)
        .unwrap_or(0);
    let scope_rank = candidate.context.scope_specificity_rank;

    // Sort ascending; higher specificity should come first.
    (
        1u8.saturating_sub(surface),
        1u8.saturating_sub(mode),
        1u8.saturating_sub(focus),
        1u8.saturating_sub(workspace),
        255u8.saturating_sub(scope_rank),
    )
}

static SEEDED_RESOLVER: OnceLock<KeybindingResolver> = OnceLock::new();

/// Returns a small seeded resolver suitable for the live shell.
///
/// The seeded map is intentionally minimal: consumers are expected to layer in
/// presets, user overrides, workspace recommendations, and policy locks from
/// the owning state stores.
pub fn seeded_keybinding_resolver() -> &'static KeybindingResolver {
    SEEDED_RESOLVER.get_or_init(|| {
        let mut resolver = KeybindingResolver::new(PolicyContext {
            policy_epoch: "pe:seed:01".to_string(),
            trust_state: "trusted".to_string(),
            execution_context_id: Some("exec:keybindings:seeded".to_string()),
        });

        let command = CommandSemanticsSnapshot {
            command_id: "cmd:command_palette.open".to_string(),
            command_revision_ref: "cmd-rev:command_palette.open:seeded".to_string(),
            preview_class: "no_preview_required".to_string(),
            approval_posture_class: "no_approval_required".to_string(),
            capability_scope_class: "inert_metadata_only".to_string(),
            required_evidence_ref_classes: Vec::new(),
        };

        let ctrl_shift_p = KeySequence::new(vec![KeyStroke {
            modifiers: Modifiers {
                ctrl: true,
                shift: true,
                alt: false,
                cmd: false,
            },
            key: "P".to_string(),
        }]);
        resolver.push_candidate(
            ResolverLayerClass::CoreDefault,
            "candidate:core:command-palette-open:ctrl_shift_p",
            command.clone(),
            ctrl_shift_p,
            Some("core:keybindings".to_string()),
            Some("Default binding suitable for non-mac platforms.".to_string()),
            CandidateContext {
                surface_ref: None,
                focus_context_ref: None,
                active_mode_ref: None,
                workspace_scope_ref: None,
                scope_specificity_rank: 1,
            },
        );

        let cmd_shift_p = KeySequence::new(vec![KeyStroke {
            modifiers: Modifiers {
                ctrl: false,
                shift: true,
                alt: false,
                cmd: true,
            },
            key: "P".to_string(),
        }]);
        resolver.push_candidate(
            ResolverLayerClass::CoreDefault,
            "candidate:core:command-palette-open:cmd_shift_p",
            command,
            cmd_shift_p,
            Some("core:keybindings".to_string()),
            Some("Default binding suitable for macOS.".to_string()),
            CandidateContext {
                surface_ref: None,
                focus_context_ref: None,
                active_mode_ref: None,
                workspace_scope_ref: None,
                scope_specificity_rank: 1,
            },
        );

        resolver
    })
}
