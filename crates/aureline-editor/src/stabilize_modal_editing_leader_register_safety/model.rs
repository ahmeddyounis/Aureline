//! Canonical model for modal-editing safety, leader-key discovery, register
//! routing, macro-safe replay, and keyboard-mode downgrade truth.
//!
//! This module mints the governed [`ModalEditingSafetyPacket`] that composes
//! the editor mode-state record with surface downgrade truth and keymap import
//! regression records. It enforces the M04-191 honesty invariants:
//!
//! - Every blocked or unsupported register route fails closed with a visible reason.
//! - Unsafe macro replays are reviewed or rejected, never silently executed.
//! - Partial and unsupported sequence states are visible with an explanation.
//! - Recovery paths to keymap diagnostics, command palette, and safe-mode reset exist.
//! - Surface downgrades (IME, accessibility, browser, restricted-mode, large-file)
//!   are labeled, reversible, and announced to assistive technology.
//! - Imported keymap regressions use the closed vocabulary
//!   exact / translated / partial / shimmed / unsupported.

use serde::{Deserialize, Serialize};

use crate::modes::EditorModeStateRecord;

/// Schema version for the modal-editing safety packet.
pub const MODAL_EDITING_SAFETY_SCHEMA_VERSION: u32 = 1;

/// Schema reference consumed by every surface that ingests this record.
pub const MODAL_EDITING_SAFETY_SCHEMA_REF: &str =
    "schemas/editor/mode_state_record.schema.json";

/// Stable record-kind tag for [`ModalEditingSafetyPacket`].
pub const MODAL_EDITING_SAFETY_PACKET_RECORD_KIND: &str = "modal_editing_safety_packet";

/// Latency budget in microseconds for modal cue rendering on claimed stable rows.
pub const MODAL_CUE_LATENCY_BUDGET_MICROS: u64 = 1_000;

// ---------------------------------------------------------------------------
// Surface downgrade vocabulary.
// ---------------------------------------------------------------------------

/// Canonical surface kinds that may narrow modal fidelity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceDowngradeKind {
    /// IME composition is active and may swallow or reinterpret strokes.
    Ime,
    /// Accessibility constraints require simplified or narrated modal state.
    Accessibility,
    /// Browser or companion host limits key dispatch or focus behavior.
    BrowserCompanion,
    /// Restricted mode narrows available commands and register routes.
    RestrictedMode,
    /// Large-file posture disables certain modal features for performance.
    LargeFile,
}

impl SurfaceDowngradeKind {
    /// Returns the stable schema token for this downgrade kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ime => "ime",
            Self::Accessibility => "accessibility",
            Self::BrowserCompanion => "browser_companion",
            Self::RestrictedMode => "restricted_mode",
            Self::LargeFile => "large_file",
        }
    }
}

/// One labeled, reversible surface downgrade that affects modal behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceDowngradeRecord {
    /// Stable downgrade ref.
    pub downgrade_ref: String,
    /// Kind of surface narrowing.
    pub downgrade_kind: SurfaceDowngradeKind,
    /// Surface whose modal semantics are narrowed.
    pub surface_ref: String,
    /// Human-facing label shown before key meaning changes.
    pub visible_label: String,
    /// Export-safe visible reason.
    pub visible_reason: String,
    /// True when the downgrade can be reversed without restart.
    pub reversible: bool,
    /// Keyboard route to restore full modal fidelity, when available.
    pub keyboard_route_to_restore: String,
    /// Screen-reader announcement for this downgrade.
    pub accessibility_announcement: String,
}

// ---------------------------------------------------------------------------
// Keymap import regression vocabulary.
// ---------------------------------------------------------------------------

/// Closed outcome vocabulary for imported keymap regressions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeymapImportOutcomeClass {
    /// Mapping is a one-to-one exact match.
    Exact,
    /// Mapping was translated to an equivalent Aureline command.
    Translated,
    /// Mapping covers only a subset of the original behavior.
    Partial,
    /// Mapping uses a shim that approximates behavior with known limits.
    Shimmed,
    /// Mapping is not supported and fails closed.
    Unsupported,
}

impl KeymapImportOutcomeClass {
    /// Returns the stable schema token for this outcome class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Translated => "translated",
            Self::Partial => "partial",
            Self::Shimmed => "shimmed",
            Self::Unsupported => "unsupported",
        }
    }
}

/// One imported keymap sequence or command whose fidelity was narrowed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeymapImportRegressionRecord {
    /// Stable regression ref.
    pub regression_ref: String,
    /// Source preset that contributed the mapping.
    pub source_preset_ref: String,
    /// Sequence or command under review.
    pub sequence_or_command_ref: String,
    /// Outcome class for this mapping.
    pub outcome_class: KeymapImportOutcomeClass,
    /// Visible explanation of the outcome.
    pub visible_reason: String,
    /// Fallback command id when one exists.
    pub fallback_command_id: Option<String>,
    /// Route to open keymap diagnostics for this regression.
    pub diagnostics_route_ref: String,
}

// ---------------------------------------------------------------------------
// Packet and builder.
// ---------------------------------------------------------------------------

/// Input used to build a governed modal-editing safety packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModalEditingSafetyInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Underlying mode-state record produced by the live editor.
    pub mode_state: EditorModeStateRecord,
    /// Surface downgrades active for the current surface.
    pub surface_downgrades: Vec<SurfaceDowngradeRecord>,
    /// Keymap import regressions to surface.
    pub import_regressions: Vec<KeymapImportRegressionRecord>,
    /// Support-safe packet refs.
    pub support_export_refs: Vec<String>,
}

/// Top-level governed record for modal-editing safety.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModalEditingSafetyPacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Underlying editor mode-state record.
    pub mode_state: EditorModeStateRecord,
    /// Active surface downgrades.
    pub surface_downgrades: Vec<SurfaceDowngradeRecord>,
    /// Keymap import regressions.
    pub import_regressions: Vec<KeymapImportRegressionRecord>,
    /// True when modal discovery is unified with the command graph.
    pub command_graph_unified: bool,
    /// Claimed latency budget for modal cue rendering in microseconds.
    pub latency_budget_micros: u64,
    /// Support-safe packet refs.
    pub support_export_refs: Vec<String>,
}

/// Reasons a [`ModalEditingSafetyPacket`] cannot honestly be minted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// The underlying mode-state record does not cover all required register routes.
    MissingRegisterRoutes,
    /// A blocked or unsupported route does not fail closed with a reason.
    RouteFailOpen,
    /// An unsafe macro replay is not reviewed or rejected.
    UnsafeMacroUnbounded,
    /// Partial or unsupported sequence states are not visible.
    SequencesHidden,
    /// Required recovery paths are missing.
    MissingRecoveryPaths,
    /// A surface downgrade is missing a visible reason.
    DowngradeMissingReason,
    /// A surface downgrade is missing an accessibility announcement.
    DowngradeMissingAccessibility,
    /// An import regression outcome is not one of the closed vocabulary values.
    InvalidImportOutcome,
}

impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MissingRegisterRoutes => {
                write!(f, "mode-state record must cover all required register routes")
            }
            Self::RouteFailOpen => write!(
                f,
                "blocked or unsupported register routes must fail closed with a visible reason"
            ),
            Self::UnsafeMacroUnbounded => write!(
                f,
                "unsafe macro replays must require review or be rejected"
            ),
            Self::SequencesHidden => write!(
                f,
                "partial and unsupported sequence states must be visible with a reason"
            ),
            Self::MissingRecoveryPaths => write!(
                f,
                "mode-state record must expose keymap diagnostics, command palette, and safe-mode reset paths"
            ),
            Self::DowngradeMissingReason => {
                write!(f, "every surface downgrade must carry a visible reason")
            }
            Self::DowngradeMissingAccessibility => {
                write!(f, "every surface downgrade must carry an accessibility announcement")
            }
            Self::InvalidImportOutcome => write!(
                f,
                "import regression outcomes must be one of exact, translated, partial, shimmed, unsupported"
            ),
        }
    }
}

impl std::error::Error for BuildError {}

impl ModalEditingSafetyPacket {
    /// Builds a governed modal-editing safety packet from validated input.
    ///
    /// Returns a [`BuildError`] when the input would mint a record that violates
    /// the M04-191 honesty invariants.
    pub fn build(input: ModalEditingSafetyInput) -> Result<Self, BuildError> {
        // --- mode-state contract validation ----------------------------------
        if !input.mode_state.covers_required_register_routes() {
            return Err(BuildError::MissingRegisterRoutes);
        }
        if !input.mode_state.blocked_or_unsupported_routes_fail_closed() {
            return Err(BuildError::RouteFailOpen);
        }
        if !input.mode_state.unsafe_macro_replays_are_bounded() {
            return Err(BuildError::UnsafeMacroUnbounded);
        }
        if !input.mode_state.exposes_partial_and_unsupported_sequences() {
            return Err(BuildError::SequencesHidden);
        }
        if !input.mode_state.has_required_recovery_paths() {
            return Err(BuildError::MissingRecoveryPaths);
        }

        // --- surface downgrade validation ------------------------------------
        for downgrade in &input.surface_downgrades {
            if downgrade.visible_reason.trim().is_empty() {
                return Err(BuildError::DowngradeMissingReason);
            }
            if downgrade.accessibility_announcement.trim().is_empty() {
                return Err(BuildError::DowngradeMissingAccessibility);
            }
        }

        // --- import regression validation ------------------------------------
        for regression in &input.import_regressions {
            let _ = regression.outcome_class.as_str(); // vocabulary is closed via enum
            if regression.visible_reason.trim().is_empty() {
                return Err(BuildError::InvalidImportOutcome);
            }
        }

        Ok(Self {
            record_kind: MODAL_EDITING_SAFETY_PACKET_RECORD_KIND.to_string(),
            schema_version: MODAL_EDITING_SAFETY_SCHEMA_VERSION,
            schema_ref: MODAL_EDITING_SAFETY_SCHEMA_REF.to_string(),
            packet_id: input.packet_id,
            mode_state: input.mode_state,
            surface_downgrades: input.surface_downgrades,
            import_regressions: input.import_regressions,
            command_graph_unified: true,
            latency_budget_micros: MODAL_CUE_LATENCY_BUDGET_MICROS,
            support_export_refs: input.support_export_refs,
        })
    }

    /// Returns contract findings; an empty list means the packet obeys the lane invariants.
    pub fn contract_findings(&self) -> Vec<String> {
        let mut findings = Vec::new();
        if !self.mode_state.covers_required_register_routes() {
            findings.push("mode-state record must cover all required register routes".to_string());
        }
        if !self.mode_state.blocked_or_unsupported_routes_fail_closed() {
            findings.push(
                "blocked or unsupported register routes must fail closed with a visible reason"
                    .to_string(),
            );
        }
        if !self.mode_state.unsafe_macro_replays_are_bounded() {
            findings.push(
                "unsafe macro replays must require review or be rejected".to_string(),
            );
        }
        if !self.mode_state.exposes_partial_and_unsupported_sequences() {
            findings.push(
                "partial and unsupported sequence states must be visible with a reason".to_string(),
            );
        }
        if !self.mode_state.has_required_recovery_paths() {
            findings.push(
                "mode-state record must expose keymap diagnostics, command palette, and safe-mode reset paths"
                    .to_string(),
            );
        }
        for downgrade in &self.surface_downgrades {
            if downgrade.visible_reason.trim().is_empty() {
                findings.push(format!(
                    "surface downgrade {} must carry a visible reason",
                    downgrade.downgrade_ref
                ));
            }
            if downgrade.accessibility_announcement.trim().is_empty() {
                findings.push(format!(
                    "surface downgrade {} must carry an accessibility announcement",
                    downgrade.downgrade_ref
                ));
            }
        }
        for regression in &self.import_regressions {
            if regression.visible_reason.trim().is_empty() {
                findings.push(format!(
                    "import regression {} must carry a visible reason",
                    regression.regression_ref
                ));
            }
        }
        if !self.command_graph_unified {
            findings.push("modal discovery must be unified with the command graph".to_string());
        }
        findings
    }

    /// Returns true when the packet obeys the lane invariants.
    pub fn is_contract_valid(&self) -> bool {
        self.contract_findings().is_empty()
    }

    /// Returns a deterministic plaintext truth block for support exports.
    pub fn support_export_lines(&self) -> Vec<String> {
        let mode = &self.mode_state;
        let mut lines = vec![
            format!("modal_editing_safety_packet: {}", self.packet_id),
            format!("schema_version: {}", self.schema_version),
            format!("schema_ref: {}", self.schema_ref),
            format!("current_mode: {}", mode.current_mode.as_str()),
            format!("source_preset: {} ({})", mode.source_preset_ref, mode.source_preset_label),
            format!("surface: {}", mode.surface_ref),
            format!("platform: {}", mode.platform_class),
            format!("command_graph_unified: {}", self.command_graph_unified),
            format!("latency_budget_micros: {}", self.latency_budget_micros),
        ];

        lines.push(format!(
            "sequence_guides: {}",
            mode.sequence_guides.len()
        ));
        for guide in &mode.sequence_guides {
            lines.push(format!(
                "  guide {}: state={} prefix={:?}",
                guide.guide_id,
                guide.sequence_state.as_str(),
                guide.typed_prefix
            ));
        }

        lines.push(format!("register_routes: {}", mode.register_routes.len()));
        for route in &mode.register_routes {
            lines.push(format!(
                "  route {}: kind={} availability={} fail_closed={}",
                route.route_ref,
                route.route_kind.as_str(),
                route.availability.as_str(),
                route.fail_closed
            ));
        }

        lines.push(format!(
            "macro_replay_reviews: {}",
            mode.macro_replay_reviews.len()
        ));
        for review in &mode.macro_replay_reviews {
            lines.push(format!(
                "  review {}: outcome={} crosses_files={} run_capable={} mutates_settings={} timing={}",
                review.review_ref,
                review.outcome_class.as_str(),
                review.crosses_files,
                review.invokes_run_capable_commands,
                review.mutates_settings,
                review.relies_on_unstable_timing
            ));
        }

        lines.push(format!(
            "recovery_actions: {}",
            mode.recovery_actions.len()
        ));
        for action in &mode.recovery_actions {
            lines.push(format!(
                "  action {}: label={:?} command_id={:?} route={:?}",
                action.action_ref,
                action.label,
                action.command_id,
                action.keyboard_route
            ));
        }

        if !self.surface_downgrades.is_empty() {
            lines.push(format!("surface_downgrades: {}", self.surface_downgrades.len()));
            for downgrade in &self.surface_downgrades {
                lines.push(format!(
                    "  downgrade {}: kind={} reversible={} surface={} reason={:?} restore_route={:?}",
                    downgrade.downgrade_ref,
                    downgrade.downgrade_kind.as_str(),
                    downgrade.reversible,
                    downgrade.surface_ref,
                    downgrade.visible_reason,
                    downgrade.keyboard_route_to_restore
                ));
            }
        }

        if !self.import_regressions.is_empty() {
            lines.push(format!("import_regressions: {}", self.import_regressions.len()));
            for regression in &self.import_regressions {
                lines.push(format!(
                    "  regression {}: outcome={} preset={} seq={} reason={:?} fallback={:?}",
                    regression.regression_ref,
                    regression.outcome_class.as_str(),
                    regression.source_preset_ref,
                    regression.sequence_or_command_ref,
                    regression.visible_reason,
                    regression.fallback_command_id
                ));
            }
        }

        for export_ref in &self.support_export_refs {
            lines.push(format!("support_export_ref: {export_ref}"));
        }

        lines
    }
}
