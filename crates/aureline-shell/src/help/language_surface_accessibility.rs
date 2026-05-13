//! Accessibility review projection for launch-language assistance surfaces.
//!
//! This module is the first shell-side consumer for the language accessibility
//! packet. It validates that diagnostics, completion assistance, and refactor
//! preview surfaces keep keyboard routes, screen-reader semantics, reduced-
//! motion substitutions, and preview/content-integrity cues attached to the
//! upstream language contracts.

use std::collections::BTreeSet;

use aureline_editor::assist::{AssistSourceFamily, AssistSurfaceSnapshot};
use aureline_language::diagnostics::{DiagnosticBusSnapshot, DiagnosticSurfaceProjection};
use aureline_language::python::{PythonApplyPostureClass, PythonRenamePreviewRecord};
use aureline_language::tsjs::{TsJsApplyPostureClass, TsJsRenamePreviewRecord};
use serde::{Deserialize, Serialize};

/// Contract id for the launch-language surface accessibility packet.
pub const LANGUAGE_SURFACE_ACCESSIBILITY_CONTRACT_ID: &str =
    "aureline.accessibility.language_surface_alpha";

/// Schema version for language-surface accessibility review packets.
pub const LANGUAGE_SURFACE_ACCESSIBILITY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for language-surface accessibility packets.
pub const LANGUAGE_SURFACE_ACCESSIBILITY_RECORD_KIND: &str =
    "language_surface_accessibility_packet";

/// Stable record-kind tag for validation reports.
pub const LANGUAGE_SURFACE_ACCESSIBILITY_VALIDATION_RECORD_KIND: &str =
    "language_surface_accessibility_validation_report";

const CANONICAL_SURFACE_CLASSES: [LanguageAssistSurfaceClass; 3] = [
    LanguageAssistSurfaceClass::Diagnostics,
    LanguageAssistSurfaceClass::CompletionAssist,
    LanguageAssistSurfaceClass::RefactorPreview,
];

const CANONICAL_DIMENSION_CLASSES: [AccessibilityDimensionClass; 3] = [
    AccessibilityDimensionClass::Keyboard,
    AccessibilityDimensionClass::ScreenReader,
    AccessibilityDimensionClass::ReducedMotion,
];

/// Surface families protected by the launch-language accessibility packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LanguageAssistSurfaceClass {
    /// Diagnostic markers, Problems rows, and quick-fix entry points.
    Diagnostics,
    /// Completion list, signature-help card, and snippet-session strip.
    CompletionAssist,
    /// Rename or refactor preview with apply/revert posture.
    RefactorPreview,
}

impl LanguageAssistSurfaceClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Diagnostics => "diagnostics",
            Self::CompletionAssist => "completion_assist",
            Self::RefactorPreview => "refactor_preview",
        }
    }
}

/// Accessibility dimensions checked for every protected language surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessibilityDimensionClass {
    /// Keyboard route, focus, and escape/cancel behavior.
    Keyboard,
    /// Accessible names, source labels, announcements, and durable fallbacks.
    ScreenReader,
    /// Static meaning-preserving state when motion is reduced.
    ReducedMotion,
}

impl AccessibilityDimensionClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Keyboard => "keyboard",
            Self::ScreenReader => "screen_reader",
            Self::ReducedMotion => "reduced_motion",
        }
    }
}

/// Review state vocabulary for protected accessibility rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessibilityReviewState {
    /// Evidence satisfies the alpha acceptance row.
    Passed,
    /// A usable path remains but a dimension is narrowed.
    Degraded,
    /// Evidence is not current enough to claim a pass.
    PendingEvidence,
    /// A dependency or policy state blocks completion.
    Blocked,
    /// A claimed path fails the review.
    Failed,
    /// The behavior is deliberately narrowed by a known-limit row.
    KnownLimit,
}

impl AccessibilityReviewState {
    /// Returns true when this state is a pass.
    pub const fn is_passed(self) -> bool {
        matches!(self, Self::Passed)
    }
}

/// Top-level review packet consumed by help, support, and validation surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguageSurfaceAccessibilityPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Packet schema version.
    pub schema_version: u32,
    /// Contract id for this packet family.
    pub contract_id: String,
    /// Contract revision for this packet family.
    pub contract_revision: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Packet owner.
    pub owner: String,
    /// Reviewer-facing packet ref.
    pub review_packet_ref: String,
    /// Human-readable contract doc ref.
    pub docs_ref: String,
    /// First consumer that renders or validates the packet.
    pub consumer_ref: String,
    /// Current known-limit refs attached to this packet.
    pub known_limit_refs: Vec<String>,
    /// Launch-language refs covered by this packet.
    pub launch_language_refs: Vec<String>,
    /// Source artifacts consumed by the packet.
    pub source_refs: Vec<String>,
    /// Surface classes that must be represented.
    pub required_surface_classes: Vec<LanguageAssistSurfaceClass>,
    /// Accessibility dimensions that must pass on each surface.
    pub required_dimension_classes: Vec<AccessibilityDimensionClass>,
    /// Protected surface rows.
    pub rows: Vec<LanguageSurfaceAccessibilityRow>,
    /// Acceptance switches asserted by the packet.
    pub acceptance: LanguageSurfaceAccessibilityAcceptance,
}

/// Acceptance states for a language-surface accessibility packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguageSurfaceAccessibilityAcceptance {
    /// Required surface classes are all present.
    pub all_required_surfaces_present: bool,
    /// Keyboard routes and focus-return states pass.
    pub keyboard_routes_complete: bool,
    /// Screen-reader semantics pass.
    pub screen_reader_semantics_complete: bool,
    /// Reduced-motion substitutions preserve meaning.
    pub reduced_motion_meaning_preserved: bool,
    /// Content-integrity and preview cues remain visible.
    pub content_integrity_preview_cues_preserved: bool,
    /// Known-limit refs are current for remaining alpha narrowness.
    pub known_limits_current: bool,
}

impl LanguageSurfaceAccessibilityAcceptance {
    fn all_asserted(&self) -> bool {
        self.all_required_surfaces_present
            && self.keyboard_routes_complete
            && self.screen_reader_semantics_complete
            && self.reduced_motion_meaning_preserved
            && self.content_integrity_preview_cues_preserved
            && self.known_limits_current
    }
}

/// One protected language assistance surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguageSurfaceAccessibilityRow {
    /// Stable surface id.
    pub surface_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Surface family.
    pub surface_class: LanguageAssistSurfaceClass,
    /// Launch-language refs exercised by this row.
    pub language_refs: Vec<String>,
    /// Upstream runtime or schema refs this row consumes.
    pub upstream_contract_refs: Vec<String>,
    /// Protected fixture refs this row consumes.
    pub upstream_fixture_refs: Vec<String>,
    /// Support or export refs attached to this row.
    pub support_export_refs: Vec<String>,
    /// Known-limit refs attached to this row, when narrowed.
    pub known_limit_refs: Vec<String>,
    /// Keyboard review dimension.
    pub keyboard: KeyboardParityReview,
    /// Screen-reader review dimension.
    pub screen_reader: ScreenReaderParityReview,
    /// Reduced-motion review dimension.
    pub reduced_motion: ReducedMotionParityReview,
    /// Preview and content-integrity review dimension.
    pub content_integrity: ContentIntegrityParityReview,
}

impl LanguageSurfaceAccessibilityRow {
    /// Builds the diagnostics row from the diagnostic bus surface projection.
    pub fn from_diagnostic_projection(
        surface_id: impl Into<String>,
        surface_label: impl Into<String>,
        language_refs: Vec<String>,
        snapshot: &DiagnosticBusSnapshot,
        projection: &DiagnosticSurfaceProjection,
    ) -> Self {
        let mut state_label_refs = vec![
            "a11y.label.diagnostic.severity".to_string(),
            "a11y.label.diagnostic.source".to_string(),
            "a11y.label.diagnostic.freshness".to_string(),
        ];
        if projection.disclosure_required {
            state_label_refs.push("a11y.label.diagnostic.degraded_or_partial".to_string());
        }

        Self {
            surface_id: surface_id.into(),
            surface_label: surface_label.into(),
            surface_class: LanguageAssistSurfaceClass::Diagnostics,
            language_refs,
            upstream_contract_refs: vec![
                "crates/aureline-language/src/diagnostics/mod.rs".into(),
                "schemas/diagnostics/diagnostic_bus.schema.json".into(),
                "docs/language/diagnostic_freshness_and_delta_contract.md".into(),
            ],
            upstream_fixture_refs: vec![
                "fixtures/language/diagnostic_bus_alpha/bus_cases.json".into()
            ],
            support_export_refs: vec![
                snapshot.snapshot_id.clone(),
                projection.projection_id.clone(),
            ],
            known_limit_refs: Vec::new(),
            keyboard: KeyboardParityReview {
                review_state: AccessibilityReviewState::Passed,
                command_ids: vec![
                    "cmd:language.diagnostics.next".into(),
                    "cmd:language.diagnostics.previous".into(),
                    "cmd:language.codeAction.open".into(),
                ],
                route_summary:
                    "Use diagnostic navigation commands, then Enter to inspect or open actions."
                        .into(),
                focus_return_state: "returned_exact".into(),
                pointer_only_action_count: 0,
                route_same_as_pointer: true,
                focus_visible: true,
                escape_or_cancel_path: true,
            },
            screen_reader: ScreenReaderParityReview {
                review_state: AccessibilityReviewState::Passed,
                accessible_name_refs: state_label_refs,
                message_id_refs: vec![
                    "a11y.announcement.diagnostic_entered_active_line".into(),
                    "a11y.announcement.diagnostic_left_active_line".into(),
                ],
                live_region_policy: "polite".into(),
                durable_fallback_refs: vec![
                    "surface:problems.row".into(),
                    "surface:editor.status_summary".into(),
                ],
                announces_source_label: true,
                announces_scope_or_partiality: projection.disclosure_required,
                announces_preview_or_apply_posture: true,
                no_color_only_state: true,
            },
            reduced_motion: ReducedMotionParityReview {
                review_state: AccessibilityReviewState::Passed,
                motion_posture_class: "motion_reduced".into(),
                disabled_motion_refs: vec![
                    "motion:diagnostic.pulse".into(),
                    "motion:inline.marker_transition".into(),
                ],
                static_substitution_refs: vec![
                    "static:severity_icon_and_label".into(),
                    "static:source_freshness_badge".into(),
                    "static:problem_count_text".into(),
                ],
                meaning_preserved_without_motion: true,
                no_motion_only_cue: true,
                layout_shift_required: false,
            },
            content_integrity: ContentIntegrityParityReview {
                source_label_refs: projection.provider_availability_refs.clone(),
                preview_or_review_refs: vec!["surface:quick_fix.preview_or_inline_summary".into()],
                raw_rendered_state_refs: vec!["state:diagnostic.structured_metadata_only".into()],
                no_silent_multi_file_mutation: true,
                safe_preview_refs: Vec::new(),
            },
        }
    }

    /// Builds the completion-assist row from an editor assist surface snapshot.
    pub fn from_assist_surface_snapshot(
        surface_id: impl Into<String>,
        surface_label: impl Into<String>,
        snapshot: &AssistSurfaceSnapshot,
    ) -> Self {
        let mut static_refs = vec![
            "static:completion.active_row_outline".to_string(),
            "static:completion.source_label".to_string(),
            "static:snippet.placeholder_count".to_string(),
        ];
        if snapshot.source_counts.signature_help_count > 0 {
            static_refs.push("static:signature.active_parameter_label".to_string());
        }

        Self {
            surface_id: surface_id.into(),
            surface_label: surface_label.into(),
            surface_class: LanguageAssistSurfaceClass::CompletionAssist,
            language_refs: vec![snapshot.language_id.clone()],
            upstream_contract_refs: vec![
                "crates/aureline-editor/src/assist/mod.rs".into(),
                "schemas/assist/completion_item.schema.json".into(),
                "docs/language/completion_and_inline_hint_contract.md".into(),
            ],
            upstream_fixture_refs: vec![
                "fixtures/editor/assist_sessions_alpha/session_cases.json".into()
            ],
            support_export_refs: vec![snapshot.snapshot_id.clone()],
            known_limit_refs: Vec::new(),
            keyboard: KeyboardParityReview {
                review_state: AccessibilityReviewState::Passed,
                command_ids: vec![
                    "cmd:editor.completion.accept".into(),
                    "cmd:editor.completion.dismiss".into(),
                    "cmd:editor.snippet.nextPlaceholder".into(),
                    "cmd:editor.snippet.previousPlaceholder".into(),
                    "cmd:editor.snippet.cancel".into(),
                ],
                route_summary:
                    "Arrow through completions, Enter accepts, Escape dismisses, and Tab only traverses active snippet placeholders."
                        .into(),
                focus_return_state: "returned_exact".into(),
                pointer_only_action_count: 0,
                route_same_as_pointer: true,
                focus_visible: true,
                escape_or_cancel_path: true,
            },
            screen_reader: ScreenReaderParityReview {
                review_state: AccessibilityReviewState::Passed,
                accessible_name_refs: vec![
                    "a11y.label.completion.item_source".into(),
                    "a11y.label.completion.additional_edits".into(),
                    "a11y.label.signature.active_parameter".into(),
                    "a11y.label.snippet.placeholder_position".into(),
                ],
                message_id_refs: vec![
                    "a11y.assist.completion.available".into(),
                    "a11y.assist.signature.parameter".into(),
                    "a11y.assist.snippet.placeholder".into(),
                ],
                live_region_policy: "polite_or_focused_row".into(),
                durable_fallback_refs: vec![
                    snapshot.accessibility_summary.clone(),
                    "surface:editor.assist_status".into(),
                ],
                announces_source_label: snapshot.source_counts.source_label_count > 0,
                announces_scope_or_partiality: snapshot.disclosure_required,
                announces_preview_or_apply_posture: snapshot.source_counts.preview_required_count > 0,
                no_color_only_state: true,
            },
            reduced_motion: ReducedMotionParityReview {
                review_state: AccessibilityReviewState::Passed,
                motion_posture_class: "motion_reduced".into(),
                disabled_motion_refs: vec![
                    "motion:completion.popup_slide".into(),
                    "motion:signature.card_transition".into(),
                    "motion:snippet.placeholder_pulse".into(),
                ],
                static_substitution_refs: static_refs,
                meaning_preserved_without_motion: true,
                no_motion_only_cue: true,
                layout_shift_required: false,
            },
            content_integrity: ContentIntegrityParityReview {
                source_label_refs: snapshot
                    .source_counts
                    .source_families
                    .iter()
                    .map(|family| {
                        format!("assist_source_family:{}", assist_source_family_token(*family))
                    })
                    .collect(),
                preview_or_review_refs: if snapshot.source_counts.preview_required_count > 0 {
                    vec!["surface:completion.additional_edit_preview".into()]
                } else {
                    vec!["surface:completion.inline_acceptance_summary".into()]
                },
                raw_rendered_state_refs: vec!["state:completion.insert_text_ref_only".into()],
                no_silent_multi_file_mutation: true,
                safe_preview_refs: Vec::new(),
            },
        }
    }

    /// Builds a refactor-preview row from a TS/JS rename preview.
    pub fn from_tsjs_rename_preview(
        surface_id: impl Into<String>,
        surface_label: impl Into<String>,
        language_ref: impl Into<String>,
        preview: &TsJsRenamePreviewRecord,
    ) -> Self {
        let apply_is_previewed = matches!(
            preview.apply_posture_class,
            TsJsApplyPostureClass::ReadyForApplyAfterPreview
                | TsJsApplyPostureClass::BlockedPendingScopeReview
                | TsJsApplyPostureClass::BlockedPendingRefresh
                | TsJsApplyPostureClass::BlockedPendingPolicyOrProtectedReview
                | TsJsApplyPostureClass::InspectOnlyUnavailable
        );
        Self::refactor_preview_row(
            surface_id,
            surface_label,
            language_ref.into(),
            preview.rename_preview_id.clone(),
            preview.provider_snapshot.provider_display_label.clone(),
            preview.requires_degraded_disclosure(),
            preview.count_summary.changed_count,
            preview.count_summary.generated_count,
            preview.count_summary.protected_count,
            preview.count_summary.skipped_count,
            apply_is_previewed,
            preview.is_ready_for_apply_after_preview(),
            vec![
                "crates/aureline-language/src/tsjs/mod.rs".into(),
                "docs/language/tsjs_wedge_alpha.md".into(),
            ],
            vec!["fixtures/language/tsjs_nav_alpha/wedge_cases.json".into()],
        )
    }

    /// Builds a refactor-preview row from a Python rename preview.
    pub fn from_python_rename_preview(
        surface_id: impl Into<String>,
        surface_label: impl Into<String>,
        language_ref: impl Into<String>,
        preview: &PythonRenamePreviewRecord,
    ) -> Self {
        let apply_is_previewed = matches!(
            preview.apply_posture_class,
            PythonApplyPostureClass::ReadyForApplyAfterPreview
                | PythonApplyPostureClass::BlockedPendingScopeReview
                | PythonApplyPostureClass::BlockedPendingRefresh
                | PythonApplyPostureClass::BlockedPendingPolicyOrProtectedReview
                | PythonApplyPostureClass::InspectOnlyUnavailable
        );
        Self::refactor_preview_row(
            surface_id,
            surface_label,
            language_ref.into(),
            preview.rename_preview_id.clone(),
            preview.provider_snapshot.provider_display_label.clone(),
            preview.requires_degraded_disclosure(),
            preview.count_summary.changed_count,
            preview.count_summary.generated_count,
            preview.count_summary.protected_count,
            preview.count_summary.skipped_count,
            apply_is_previewed,
            preview.is_ready_for_apply_after_preview(),
            vec![
                "crates/aureline-language/src/python/mod.rs".into(),
                "docs/language/python_wedge_alpha.md".into(),
            ],
            vec!["fixtures/language/python_nav_alpha/wedge_cases.json".into()],
        )
    }

    fn refactor_preview_row(
        surface_id: impl Into<String>,
        surface_label: impl Into<String>,
        language_ref: String,
        preview_ref: String,
        provider_label: String,
        disclosure_required: bool,
        changed_count: usize,
        generated_count: usize,
        protected_count: usize,
        skipped_count: usize,
        apply_is_previewed: bool,
        ready_for_apply_after_preview: bool,
        upstream_contract_refs: Vec<String>,
        upstream_fixture_refs: Vec<String>,
    ) -> Self {
        let mut preview_labels = vec![
            "a11y.label.refactor.changed_count".to_string(),
            "a11y.label.refactor.scope".to_string(),
            "a11y.label.refactor.apply_posture".to_string(),
        ];
        if generated_count > 0 {
            preview_labels.push("a11y.label.refactor.generated_count".to_string());
        }
        if protected_count > 0 {
            preview_labels.push("a11y.label.refactor.protected_count".to_string());
        }

        Self {
            surface_id: surface_id.into(),
            surface_label: surface_label.into(),
            surface_class: LanguageAssistSurfaceClass::RefactorPreview,
            language_refs: vec![language_ref],
            upstream_contract_refs,
            upstream_fixture_refs,
            support_export_refs: vec![preview_ref.clone()],
            known_limit_refs: Vec::new(),
            keyboard: KeyboardParityReview {
                review_state: AccessibilityReviewState::Passed,
                command_ids: vec![
                    "cmd:editor.refactor.rename.preview".into(),
                    "cmd:editor.refactor.apply".into(),
                    "cmd:editor.refactor.narrowScope".into(),
                    "cmd:editor.refactor.exportPatch".into(),
                    "cmd:editor.refactor.cancel".into(),
                ],
                route_summary:
                    "Open rename preview, traverse affected files and warnings, then apply, narrow scope, export patch, or cancel by keyboard."
                        .into(),
                focus_return_state: if ready_for_apply_after_preview {
                    "returned_current_batch_or_detail_owner".into()
                } else {
                    "returned_placeholder_announced".into()
                },
                pointer_only_action_count: 0,
                route_same_as_pointer: true,
                focus_visible: true,
                escape_or_cancel_path: true,
            },
            screen_reader: ScreenReaderParityReview {
                review_state: AccessibilityReviewState::Passed,
                accessible_name_refs: preview_labels,
                message_id_refs: vec![
                    "a11y.refactor.preview.summary".into(),
                    "a11y.refactor.preview.warning".into(),
                    "a11y.refactor.preview.apply_posture".into(),
                ],
                live_region_policy: "polite_for_preview_ready_assertive_for_blocked_apply".into(),
                durable_fallback_refs: vec![
                    "surface:refactor_preview.tree".into(),
                    "surface:refactor_preview.warning_rows".into(),
                ],
                announces_source_label: !provider_label.trim().is_empty(),
                announces_scope_or_partiality: disclosure_required || skipped_count > 0,
                announces_preview_or_apply_posture: true,
                no_color_only_state: true,
            },
            reduced_motion: ReducedMotionParityReview {
                review_state: AccessibilityReviewState::Passed,
                motion_posture_class: "motion_reduced".into(),
                disabled_motion_refs: vec![
                    "motion:preview_tree.expand_collapse".into(),
                    "motion:diff_hunk_transition".into(),
                    "motion:blocked_warning_pulse".into(),
                ],
                static_substitution_refs: vec![
                    format!("static:refactor.changed_count:{changed_count}"),
                    format!("static:refactor.generated_count:{generated_count}"),
                    format!("static:refactor.protected_count:{protected_count}"),
                    "static:refactor.apply_posture_label".into(),
                ],
                meaning_preserved_without_motion: true,
                no_motion_only_cue: true,
                layout_shift_required: false,
            },
            content_integrity: ContentIntegrityParityReview {
                source_label_refs: vec![format!("provider:{provider_label}")],
                preview_or_review_refs: vec![preview_ref],
                raw_rendered_state_refs: vec![
                    "state:rename_preview.structured_diff".into(),
                    "state:rename_preview.blocked_generated_readonly_counts".into(),
                ],
                no_silent_multi_file_mutation: apply_is_previewed,
                safe_preview_refs: vec!["docs/security/trust_class_alpha.md".into()],
            },
        }
    }
}

/// Keyboard parity facts for one protected surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyboardParityReview {
    /// Review state for the keyboard dimension.
    pub review_state: AccessibilityReviewState,
    /// Stable command ids reachable from keyboard or palette paths.
    pub command_ids: Vec<String>,
    /// Human-readable keyboard route summary.
    pub route_summary: String,
    /// Focus-return contract state.
    pub focus_return_state: String,
    /// Pointer-only action count.
    pub pointer_only_action_count: u32,
    /// Whether keyboard reaches the same actions as pointer UI.
    pub route_same_as_pointer: bool,
    /// Whether focus remains visibly indicated.
    pub focus_visible: bool,
    /// Whether escape or cancel is available.
    pub escape_or_cancel_path: bool,
}

/// Screen-reader parity facts for one protected surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScreenReaderParityReview {
    /// Review state for the screen-reader dimension.
    pub review_state: AccessibilityReviewState,
    /// Stable accessible name or label refs.
    pub accessible_name_refs: Vec<String>,
    /// Stable announcement or assistive message ids.
    pub message_id_refs: Vec<String>,
    /// Live-region policy token.
    pub live_region_policy: String,
    /// Durable fallback rows that preserve the same meaning.
    pub durable_fallback_refs: Vec<String>,
    /// Whether source labels are announced where they affect trust.
    pub announces_source_label: bool,
    /// Whether scope, stale, fallback, or partiality state is announced.
    pub announces_scope_or_partiality: bool,
    /// Whether preview/apply posture is announced before mutation.
    pub announces_preview_or_apply_posture: bool,
    /// Whether no state depends on color alone.
    pub no_color_only_state: bool,
}

/// Reduced-motion parity facts for one protected surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReducedMotionParityReview {
    /// Review state for the reduced-motion dimension.
    pub review_state: AccessibilityReviewState,
    /// Motion posture under review.
    pub motion_posture_class: String,
    /// Motion effects disabled or simplified.
    pub disabled_motion_refs: Vec<String>,
    /// Static substitutions that keep the same meaning.
    pub static_substitution_refs: Vec<String>,
    /// Whether the surface remains meaningful without motion.
    pub meaning_preserved_without_motion: bool,
    /// Whether no cue is motion-only.
    pub no_motion_only_cue: bool,
    /// Whether a layout shift is required to understand state.
    pub layout_shift_required: bool,
}

/// Preview and content-integrity facts for one protected surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentIntegrityParityReview {
    /// Source, provider, freshness, or scope label refs.
    pub source_label_refs: Vec<String>,
    /// Preview or review refs shown before broad mutation.
    pub preview_or_review_refs: Vec<String>,
    /// Raw/rendered/structured state refs.
    pub raw_rendered_state_refs: Vec<String>,
    /// Whether broad changes refuse silent multi-file mutation.
    pub no_silent_multi_file_mutation: bool,
    /// Safe-preview refs when active or risky content is involved.
    pub safe_preview_refs: Vec<String>,
}

/// Validation report for a language-surface accessibility packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguageSurfaceAccessibilityValidationReport {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Validation schema version.
    pub schema_version: u32,
    /// Contract id checked by this report.
    pub contract_id: String,
    /// Whether the packet passed validation.
    pub passed: bool,
    /// Number of protected surface rows checked.
    pub checked_surface_count: usize,
    /// Findings emitted by validation.
    pub findings: Vec<LanguageSurfaceAccessibilityFinding>,
}

/// One validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguageSurfaceAccessibilityFinding {
    /// Finding severity.
    pub severity: ValidationFindingSeverity,
    /// Surface id, when scoped to one row.
    pub surface_id: Option<String>,
    /// Human-readable finding.
    pub message: String,
}

/// Finding severity for validation reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationFindingSeverity {
    /// The packet cannot satisfy acceptance.
    Error,
    /// The packet is valid but needs reviewer attention.
    Warning,
}

/// Validates a language-surface accessibility packet.
pub fn validate_language_surface_accessibility_packet(
    packet: &LanguageSurfaceAccessibilityPacket,
) -> LanguageSurfaceAccessibilityValidationReport {
    let mut findings = Vec::new();

    if packet.record_kind != LANGUAGE_SURFACE_ACCESSIBILITY_RECORD_KIND {
        push_error(
            &mut findings,
            None,
            format!(
                "record_kind must be {LANGUAGE_SURFACE_ACCESSIBILITY_RECORD_KIND}, got {}",
                packet.record_kind
            ),
        );
    }
    if packet.schema_version != LANGUAGE_SURFACE_ACCESSIBILITY_SCHEMA_VERSION {
        push_error(
            &mut findings,
            None,
            format!(
                "schema_version must be {LANGUAGE_SURFACE_ACCESSIBILITY_SCHEMA_VERSION}, got {}",
                packet.schema_version
            ),
        );
    }
    if packet.contract_id != LANGUAGE_SURFACE_ACCESSIBILITY_CONTRACT_ID {
        push_error(
            &mut findings,
            None,
            format!(
                "contract_id must be {LANGUAGE_SURFACE_ACCESSIBILITY_CONTRACT_ID}, got {}",
                packet.contract_id
            ),
        );
    }
    if packet.known_limit_refs.is_empty() {
        push_error(
            &mut findings,
            None,
            "packet must cite the active alpha known-limit refs".into(),
        );
    }
    if packet.launch_language_refs.is_empty() {
        push_error(
            &mut findings,
            None,
            "packet must name covered launch-language refs".into(),
        );
    }
    if packet.source_refs.is_empty() {
        push_error(
            &mut findings,
            None,
            "packet must name upstream source refs".into(),
        );
    }
    if !packet.acceptance.all_asserted() {
        push_error(
            &mut findings,
            None,
            "all acceptance switches must be true for a passing packet".into(),
        );
    }

    let present_surfaces = packet
        .rows
        .iter()
        .map(|row| row.surface_class)
        .collect::<BTreeSet<_>>();
    for canonical in CANONICAL_SURFACE_CLASSES {
        if !packet.required_surface_classes.contains(&canonical) {
            push_error(
                &mut findings,
                None,
                format!(
                    "required_surface_classes must include canonical class {}",
                    canonical.as_str()
                ),
            );
        }
    }
    for required in &packet.required_surface_classes {
        if !present_surfaces.contains(required) {
            push_error(
                &mut findings,
                None,
                format!("missing required surface class {}", required.as_str()),
            );
        }
    }

    let required_dimensions = packet
        .required_dimension_classes
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    for canonical in CANONICAL_DIMENSION_CLASSES {
        if !required_dimensions.contains(&canonical) {
            push_error(
                &mut findings,
                None,
                format!(
                    "required_dimension_classes must include canonical dimension {}",
                    canonical.as_str()
                ),
            );
        }
    }
    for row in &packet.rows {
        validate_surface_row(row, &required_dimensions, &mut findings);
    }

    LanguageSurfaceAccessibilityValidationReport {
        record_kind: LANGUAGE_SURFACE_ACCESSIBILITY_VALIDATION_RECORD_KIND.into(),
        schema_version: LANGUAGE_SURFACE_ACCESSIBILITY_SCHEMA_VERSION,
        contract_id: LANGUAGE_SURFACE_ACCESSIBILITY_CONTRACT_ID.into(),
        passed: findings
            .iter()
            .all(|finding| finding.severity != ValidationFindingSeverity::Error),
        checked_surface_count: packet.rows.len(),
        findings,
    }
}

/// Builds compact help or support lines from a language-surface packet.
pub fn build_language_surface_accessibility_summary_lines(
    packet: &LanguageSurfaceAccessibilityPacket,
) -> Vec<String> {
    let report = validate_language_surface_accessibility_packet(packet);
    let mut lines = Vec::new();
    let state = if report.passed {
        "passed"
    } else {
        "needs review"
    };
    lines.push(format!(
        "Language surface accessibility alpha - {state} ({} surfaces)",
        report.checked_surface_count
    ));
    for row in &packet.rows {
        lines.push(format!(
            "- {}: keyboard={:?}, screen_reader={:?}, reduced_motion={:?}",
            row.surface_label,
            row.keyboard.review_state,
            row.screen_reader.review_state,
            row.reduced_motion.review_state
        ));
    }
    if !report.passed {
        for finding in report.findings {
            lines.push(format!("- {:?}: {}", finding.severity, finding.message));
        }
    }
    lines
}

fn validate_surface_row(
    row: &LanguageSurfaceAccessibilityRow,
    required_dimensions: &BTreeSet<AccessibilityDimensionClass>,
    findings: &mut Vec<LanguageSurfaceAccessibilityFinding>,
) {
    if row.surface_id.trim().is_empty() {
        push_error(findings, None, "surface row must carry surface_id".into());
    }
    if row.language_refs.is_empty() {
        push_error(
            findings,
            Some(row.surface_id.clone()),
            "surface row must cite at least one language ref".into(),
        );
    }
    if row.upstream_contract_refs.is_empty() || row.upstream_fixture_refs.is_empty() {
        push_error(
            findings,
            Some(row.surface_id.clone()),
            "surface row must cite upstream contract and fixture refs".into(),
        );
    }

    if required_dimensions.contains(&AccessibilityDimensionClass::Keyboard) {
        validate_keyboard(row, findings);
    }
    if required_dimensions.contains(&AccessibilityDimensionClass::ScreenReader) {
        validate_screen_reader(row, findings);
    }
    if required_dimensions.contains(&AccessibilityDimensionClass::ReducedMotion) {
        validate_reduced_motion(row, findings);
    }
    validate_content_integrity(row, findings);
}

fn validate_keyboard(
    row: &LanguageSurfaceAccessibilityRow,
    findings: &mut Vec<LanguageSurfaceAccessibilityFinding>,
) {
    if !row.keyboard.review_state.is_passed() {
        push_error(
            findings,
            Some(row.surface_id.clone()),
            "keyboard review must pass".into(),
        );
    }
    if row.keyboard.command_ids.is_empty() {
        push_error(
            findings,
            Some(row.surface_id.clone()),
            "keyboard review must cite command ids".into(),
        );
    }
    if row.keyboard.route_summary.trim().is_empty() {
        push_error(
            findings,
            Some(row.surface_id.clone()),
            "keyboard route summary must be present".into(),
        );
    }
    if !is_allowed_focus_return_state(&row.keyboard.focus_return_state) {
        push_error(
            findings,
            Some(row.surface_id.clone()),
            format!(
                "focus_return_state {} is outside the shared focus contract",
                row.keyboard.focus_return_state
            ),
        );
    }
    if row.keyboard.pointer_only_action_count != 0
        || !row.keyboard.route_same_as_pointer
        || !row.keyboard.focus_visible
        || !row.keyboard.escape_or_cancel_path
    {
        push_error(
            findings,
            Some(row.surface_id.clone()),
            "keyboard path must be pointer-independent with visible focus and escape/cancel".into(),
        );
    }
}

fn validate_screen_reader(
    row: &LanguageSurfaceAccessibilityRow,
    findings: &mut Vec<LanguageSurfaceAccessibilityFinding>,
) {
    if !row.screen_reader.review_state.is_passed() {
        push_error(
            findings,
            Some(row.surface_id.clone()),
            "screen-reader review must pass".into(),
        );
    }
    if row.screen_reader.accessible_name_refs.is_empty()
        || row.screen_reader.message_id_refs.is_empty()
        || row.screen_reader.durable_fallback_refs.is_empty()
    {
        push_error(
            findings,
            Some(row.surface_id.clone()),
            "screen-reader review must cite names, messages, and durable fallbacks".into(),
        );
    }
    if row.screen_reader.live_region_policy.trim().is_empty()
        || !row.screen_reader.announces_source_label
        || !row.screen_reader.announces_preview_or_apply_posture
        || !row.screen_reader.no_color_only_state
    {
        push_error(
            findings,
            Some(row.surface_id.clone()),
            "screen-reader review must preserve source labels, preview posture, and non-color state"
                .into(),
        );
    }
}

fn validate_reduced_motion(
    row: &LanguageSurfaceAccessibilityRow,
    findings: &mut Vec<LanguageSurfaceAccessibilityFinding>,
) {
    if !row.reduced_motion.review_state.is_passed() {
        push_error(
            findings,
            Some(row.surface_id.clone()),
            "reduced-motion review must pass".into(),
        );
    }
    if row.reduced_motion.motion_posture_class.trim().is_empty()
        || row.reduced_motion.static_substitution_refs.is_empty()
    {
        push_error(
            findings,
            Some(row.surface_id.clone()),
            "reduced-motion review must cite posture and static substitutions".into(),
        );
    }
    if !row.reduced_motion.meaning_preserved_without_motion
        || !row.reduced_motion.no_motion_only_cue
        || row.reduced_motion.layout_shift_required
    {
        push_error(
            findings,
            Some(row.surface_id.clone()),
            "reduced-motion mode must preserve meaning without motion-only cues or required layout shifts"
                .into(),
        );
    }
}

fn validate_content_integrity(
    row: &LanguageSurfaceAccessibilityRow,
    findings: &mut Vec<LanguageSurfaceAccessibilityFinding>,
) {
    if row.content_integrity.source_label_refs.is_empty()
        || row.content_integrity.preview_or_review_refs.is_empty()
        || row.content_integrity.raw_rendered_state_refs.is_empty()
        || !row.content_integrity.no_silent_multi_file_mutation
    {
        push_error(
            findings,
            Some(row.surface_id.clone()),
            "content-integrity review must preserve source labels, preview refs, representation refs, and no silent broad mutation"
                .into(),
        );
    }
}

fn is_allowed_focus_return_state(value: &str) -> bool {
    matches!(
        value,
        "returned_exact"
            | "returned_nearest_safe_ancestor"
            | "returned_current_batch_or_detail_owner"
            | "returned_placeholder_announced"
            | "focus_loss_denied"
            | "focus_not_applicable_non_interactive"
    )
}

fn push_error(
    findings: &mut Vec<LanguageSurfaceAccessibilityFinding>,
    surface_id: Option<String>,
    message: String,
) {
    findings.push(LanguageSurfaceAccessibilityFinding {
        severity: ValidationFindingSeverity::Error,
        surface_id,
        message,
    });
}

fn assist_source_family_token(family: AssistSourceFamily) -> &'static str {
    match family {
        AssistSourceFamily::LanguageServer => "language_server",
        AssistSourceFamily::FallbackLexical => "fallback_lexical",
        AssistSourceFamily::Snippet => "snippet",
        AssistSourceFamily::ProjectGraph => "project_graph",
        AssistSourceFamily::FrameworkPack => "framework_pack",
        AssistSourceFamily::AiAssist => "ai_assist",
        AssistSourceFamily::ToolAdapter => "tool_adapter",
    }
}
