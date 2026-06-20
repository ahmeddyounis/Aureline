//! Shell-side localized-surface projection for the new M5 surfaces.
//!
//! This projects the canonical [`aureline_i18n`] localized render into the view
//! the shell renders on the command palette, settings, help, error, and
//! notification surfaces. It owns no localization truth: it reads stable message
//! ids, stable command/setting/diagnostic refs, localized prose, text
//! direction, severity, and text-expansion from the render so every audience
//! agrees on what localized and what stayed in the source language.
//!
//! The projection exists to make discoverability and operability under a locale
//! a checked property rather than a manual pass:
//!
//! - Stable command ids, setting ids, diagnostic ids, telemetry keys, and
//!   policy names survive verbatim from the render, so the palette routes and
//!   analytics fire identically whatever the active locale.
//! - Keyboard-path hints are locale-neutral and carried unchanged, so a
//!   translated command label never moves its shortcut.
//! - Disabled-state explanations keep their diagnostic id and severity while the
//!   prose localizes, and truncation shortens the label without hiding the fact
//!   the command is unavailable.
//!
//! The user view carries the localized label the shell paints; the metadata-only
//! support export omits the translated body and keeps ids, lengths, and states.

use serde::{Deserialize, Serialize};

use aureline_i18n::{
    seeded_m5_localized_render, LocalizationRenderState, LocalizedRenderRow, M5LocalizedRender,
    M5MessageSurface, RenderSeverityClass, TextDirection, DEFAULT_TRUNCATION_BUDGET_GRAPHEMES,
};

/// Record kind for [`LocalizedSurfaceView`].
pub const LOCALIZED_SURFACE_VIEW_RECORD_KIND: &str = "shell_localized_surface_view";

/// Who is reading the localized-surface view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalizedSurfaceAudience {
    /// User-facing shell surface that paints the localized label.
    User,
    /// Metadata-only support export.
    SupportExport,
}

impl LocalizedSurfaceAudience {
    /// Returns true when this audience may carry translated body text.
    const fn carries_translated_body(self) -> bool {
        matches!(self, Self::User)
    }
}

/// One projected localized row rendered by the shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalizedSurfaceRow {
    /// Stable, locale-neutral message id.
    pub message_id: String,
    /// Owning M5 surface.
    pub surface: M5MessageSurface,
    /// Stable snake_case surface key (scope).
    pub surface_key: String,
    /// Canonical command id, when command-bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_id_ref: Option<String>,
    /// Stable setting id, when settings-bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub setting_id_ref: Option<String>,
    /// Diagnostic id, when diagnostic-bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostic_id_ref: Option<String>,
    /// Docs-pack key, when help-bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_pack_key_ref: Option<String>,
    /// Locale-neutral telemetry key, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telemetry_key_ref: Option<String>,
    /// Locale-neutral policy name, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_name_ref: Option<String>,
    /// Locale-neutral keyboard-path hint, when one is bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyboard_path_hint: Option<String>,
    /// Requested locale.
    pub requested_locale: String,
    /// Locale that supplied the rendered prose.
    pub effective_locale: String,
    /// Whether the row localized or fell back to the source language.
    pub localization_state: LocalizationRenderState,
    /// Writing direction for the effective locale.
    pub text_direction: TextDirection,
    /// Severity carried by the row.
    pub severity: RenderSeverityClass,
    /// Localized label the shell paints (omitted for the support export).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_label: Option<String>,
    /// Label truncated to the view budget (omitted for the support export).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub truncated_label: Option<String>,
    /// Whether the label was shortened to fit the budget.
    pub was_truncated: bool,
    /// Visible-grapheme length of the localized label.
    pub display_grapheme_len: usize,
    /// Display length as a percentage of the source length (text expansion).
    pub expansion_ratio_pct: u32,
    /// Whether every source placeholder survived into the localized label.
    pub placeholders_preserved: bool,
    /// Stable, locale-neutral route to the source-language version.
    pub open_in_source_language_route_ref: String,
    /// Always true; severity and scope cannot be hidden by truncation.
    pub severity_preserved_under_truncation: bool,
}

impl LocalizedSurfaceRow {
    /// Returns true when this row exposes a routable command id.
    pub fn is_command_discoverable(&self) -> bool {
        self.command_id_ref.is_some()
    }
}

/// Inspectable localized-surface view shared by shell surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalizedSurfaceView {
    /// Boundary record kind.
    pub record_kind: String,
    /// Who is reading the view.
    pub audience: LocalizedSurfaceAudience,
    /// Source registry packet id.
    pub registry_packet_id: String,
    /// Source catalog id.
    pub catalog_id: String,
    /// Active build identity the render was minted for.
    pub target_build_identity_ref: String,
    /// Requested locale.
    pub requested_locale: String,
    /// Effective locale that produced rendered prose for localized rows.
    pub source_language_locale: String,
    /// Writing direction for the requested locale.
    pub text_direction: TextDirection,
    /// Visible-grapheme budget the truncated labels were cut to.
    pub truncation_budget_graphemes: usize,
    /// Total projected rows.
    pub total_rows: usize,
    /// Rows shown in the requested locale.
    pub localized_rows: usize,
    /// Rows that fell back to the source language.
    pub source_fallback_rows: usize,
    /// Largest display-to-source expansion ratio across rows.
    pub max_expansion_ratio_pct: u32,
    /// Stable command ids that remain discoverable under this locale.
    pub discoverable_command_ids: Vec<String>,
    /// Per-row projection.
    pub rows: Vec<LocalizedSurfaceRow>,
    /// True for the support export: no translated body crosses this boundary.
    pub raw_translated_body_omitted: bool,
}

impl LocalizedSurfaceView {
    /// Returns the row for a message id, when present.
    pub fn row(&self, message_id: &str) -> Option<&LocalizedSurfaceRow> {
        self.rows.iter().find(|row| row.message_id == message_id)
    }

    /// Returns true when severity and scope survive truncation on every row.
    pub fn all_severities_preserved(&self) -> bool {
        self.rows
            .iter()
            .all(|row| row.severity_preserved_under_truncation)
    }
}

/// Projects the localized-surface view for an audience and requested locale.
pub fn project_localized_surface(
    audience: LocalizedSurfaceAudience,
    requested_locale: &str,
) -> LocalizedSurfaceView {
    let render = seeded_m5_localized_render(requested_locale);
    build_view(&render, audience, DEFAULT_TRUNCATION_BUDGET_GRAPHEMES)
}

/// Projects the user-facing localized-surface view.
pub fn project_user_localized_surface(requested_locale: &str) -> LocalizedSurfaceView {
    project_localized_surface(LocalizedSurfaceAudience::User, requested_locale)
}

/// Projects the metadata-only support-export localized-surface view.
pub fn project_support_localized_surface(requested_locale: &str) -> LocalizedSurfaceView {
    project_localized_surface(LocalizedSurfaceAudience::SupportExport, requested_locale)
}

/// Builds the view from a render, audience, and truncation budget.
fn build_view(
    render: &M5LocalizedRender,
    audience: LocalizedSurfaceAudience,
    budget: usize,
) -> LocalizedSurfaceView {
    let rows: Vec<LocalizedSurfaceRow> = render
        .rows
        .iter()
        .map(|row| project_row(row, audience, budget))
        .collect();

    let discoverable_command_ids = rows
        .iter()
        .filter_map(|row| row.command_id_ref.clone())
        .collect();

    LocalizedSurfaceView {
        record_kind: LOCALIZED_SURFACE_VIEW_RECORD_KIND.to_owned(),
        audience,
        registry_packet_id: render.registry_packet_id_ref.clone(),
        catalog_id: render.catalog_id_ref.clone(),
        target_build_identity_ref: render.target_build_identity_ref.clone(),
        requested_locale: render.requested_locale.clone(),
        source_language_locale: render.source_language_locale.clone(),
        text_direction: render.text_direction,
        truncation_budget_graphemes: budget,
        total_rows: render.rows.len(),
        localized_rows: render.summary.localized_rows,
        source_fallback_rows: render.summary.source_fallback_rows,
        max_expansion_ratio_pct: render.summary.max_expansion_ratio_pct,
        discoverable_command_ids,
        rows,
        raw_translated_body_omitted: !audience.carries_translated_body(),
    }
}

/// Projects one render row, gating the translated body by audience.
fn project_row(
    row: &LocalizedRenderRow,
    audience: LocalizedSurfaceAudience,
    budget: usize,
) -> LocalizedSurfaceRow {
    let truncated = row.truncate(budget);
    let carries_body = audience.carries_translated_body();

    LocalizedSurfaceRow {
        message_id: row.message_id.clone(),
        surface: row.surface,
        surface_key: row.surface_key.clone(),
        command_id_ref: row.stable_refs.command_id_ref.clone(),
        setting_id_ref: row.stable_refs.setting_id_ref.clone(),
        diagnostic_id_ref: row.stable_refs.diagnostic_id_ref.clone(),
        docs_pack_key_ref: row.stable_refs.docs_pack_key_ref.clone(),
        telemetry_key_ref: row.stable_refs.telemetry_key_ref.clone(),
        policy_name_ref: row.stable_refs.policy_name_ref.clone(),
        keyboard_path_hint: row
            .stable_refs
            .command_id_ref
            .as_deref()
            .and_then(keyboard_path_hint),
        requested_locale: row.requested_locale.clone(),
        effective_locale: row.effective_locale.clone(),
        localization_state: row.localization_state,
        text_direction: row.text_direction,
        severity: row.severity,
        display_label: carries_body.then(|| row.display_text.clone()),
        truncated_label: carries_body.then(|| truncated.display_text.clone()),
        was_truncated: truncated.was_truncated,
        display_grapheme_len: row.display_grapheme_len,
        expansion_ratio_pct: row.expansion_ratio_pct,
        placeholders_preserved: row.placeholders_preserved,
        open_in_source_language_route_ref: source_language_route_ref(&row.message_id),
        severity_preserved_under_truncation: truncated.severity_preserved
            && truncated.severity == row.severity
            && truncated.scope_surface_key == row.surface_key,
    }
}

/// Returns the locale-neutral keyboard-path hint for a command id, when bound.
///
/// Shortcuts never localize; the hint is the same string under every locale so a
/// translated command label keeps its keyboard path.
fn keyboard_path_hint(command_id: &str) -> Option<String> {
    let hint = match command_id {
        "workbench.action.tasks.runBuild" => "Ctrl+Shift+B",
        "workbench.action.openSettings" => "Ctrl+,",
        "workbench.action.switchWindow" => "Ctrl+W",
        "notebook.action.open" => "Ctrl+K N",
        _ => return None,
    };
    Some(hint.to_owned())
}

/// Returns the stable, locale-neutral source-language route for a message id.
fn source_language_route_ref(message_id: &str) -> String {
    format!("i18n.openInSourceLanguage?messageId={message_id}")
}
