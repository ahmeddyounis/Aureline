//! Projection helpers for rendering grouped palette results.
//!
//! The palette view rows are a lightweight render-friendly representation for
//! the shell surface. They keep canonical command identity and provider state
//! visible while avoiding a UI-string-as-truth workflow.

use std::collections::HashMap;

use aureline_commands::{
    CommandRegistry, CommandRegistryEntryRecord, DisabledReasonCode, EnablementDecisionClass,
};

use super::query_session::{
    CommandPaletteState, PaletteItemKey, PaletteProviderClass, PaletteProviderStateClass,
    PaletteRankingSourceClass, PaletteResultGroup,
};

#[derive(Debug, Clone)]
/// One line rendered in the command palette overlay.
pub struct PaletteViewRow {
    /// Stable key used for selection/highlighting, if this row is selectable.
    pub key: Option<PaletteItemKey>,
    /// Pre-rendered text line suitable for fixed-width rendering.
    pub text: String,
    /// Whether this row is a non-selectable group/header line.
    pub is_group_header: bool,
    /// Provider class associated with this row, when applicable.
    pub provider: Option<PaletteProviderClass>,
    /// Provider state associated with this row, when applicable.
    pub provider_state: Option<PaletteProviderStateClass>,
}

fn provider_badge(provider: PaletteProviderClass) -> &'static str {
    match provider {
        PaletteProviderClass::RecentHistory => "recent",
        PaletteProviderClass::LexicalCommandIndex => "lexical",
        PaletteProviderClass::SemanticCommandIndex => "semantic",
        PaletteProviderClass::FileIndex => "files",
        PaletteProviderClass::KeybindingResolver => "keys",
    }
}

fn state_badge(state: PaletteProviderStateClass) -> &'static str {
    match state {
        PaletteProviderStateClass::NotRequested => "not_requested",
        PaletteProviderStateClass::Warming => "warming",
        PaletteProviderStateClass::Ready => "ready",
        PaletteProviderStateClass::Streaming => "streaming",
        PaletteProviderStateClass::Partial => "partial",
        PaletteProviderStateClass::Stale => "stale",
        PaletteProviderStateClass::PolicyBlocked => "blocked",
        PaletteProviderStateClass::Unavailable => "unavailable",
        PaletteProviderStateClass::Complete => "complete",
    }
}

fn ranking_badges(sources: &[PaletteRankingSourceClass]) -> String {
    if sources.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    out.push_str(" (");
    for (idx, src) in sources.iter().enumerate() {
        if idx > 0 {
            out.push(',');
        }
        out.push_str(src.badge());
    }
    out.push(')');
    out
}

fn shortcuts_label(
    shortcuts_by_command_id: &HashMap<String, Vec<String>>,
    command_id: &str,
) -> String {
    shortcuts_by_command_id
        .get(command_id)
        .map(|seqs| seqs.join(", "))
        .unwrap_or_else(|| "unbound".to_string())
}

fn format_command_row(
    entry: &CommandRegistryEntryRecord,
    shortcuts_by_command_id: &HashMap<String, Vec<String>>,
    enablement: (EnablementDecisionClass, Option<DisabledReasonCode>),
    provider: PaletteProviderClass,
    provider_state: PaletteProviderStateClass,
    sources: &[PaletteRankingSourceClass],
) -> String {
    let shortcuts = shortcuts_label(shortcuts_by_command_id, entry.command_id());
    let mut line = format!(
        "{}  —  {}  [{}]  [{}:{}]  [{}]{}",
        entry.title,
        entry.command_id(),
        shortcuts,
        provider_badge(provider),
        state_badge(provider_state),
        entry.dominant_side_effect_class,
        ranking_badges(sources),
    );
    if enablement.0 != EnablementDecisionClass::Enabled {
        if let Some(code) = enablement.1 {
            line.push_str("  [");
            line.push_str(code.as_str());
            line.push(']');
        }
    }
    line
}

fn format_file_row(relative_path: &str, provider_state: PaletteProviderStateClass) -> String {
    format!(
        "{}  [{}:{}]",
        relative_path,
        provider_badge(PaletteProviderClass::FileIndex),
        state_badge(provider_state)
    )
}

fn group_header(group: &PaletteResultGroup) -> String {
    format!(
        "-- {} ({})  [{}:{}]",
        group.label,
        group.items.len(),
        provider_badge(group.provider),
        state_badge(group.provider_state)
    )
}

/// Builds a flattened list of palette lines suitable for rendering in the shell.
pub fn palette_view_rows(
    palette: &CommandPaletteState,
    registry: &CommandRegistry,
    shortcuts_by_command_id: &HashMap<String, Vec<String>>,
    mut evaluate_enablement: impl FnMut(
        &CommandRegistryEntryRecord,
    ) -> (EnablementDecisionClass, Option<DisabledReasonCode>),
) -> Vec<PaletteViewRow> {
    let mut rows: Vec<PaletteViewRow> = Vec::new();

    rows.push(PaletteViewRow {
        key: None,
        text: format!("> {}", palette.query()),
        is_group_header: true,
        provider: None,
        provider_state: None,
    });

    for group in palette.groups() {
        rows.push(PaletteViewRow {
            key: None,
            text: group_header(group),
            is_group_header: true,
            provider: Some(group.provider),
            provider_state: Some(group.provider_state),
        });

        for item in &group.items {
            match &item.key {
                PaletteItemKey::Command { command_id } => {
                    let Some(entry) = registry.get(command_id) else {
                        continue;
                    };
                    let enablement = evaluate_enablement(entry);
                    rows.push(PaletteViewRow {
                        key: Some(item.key.clone()),
                        text: format_command_row(
                            entry,
                            shortcuts_by_command_id,
                            enablement,
                            item.provider,
                            item.provider_state,
                            &item.ranking_sources,
                        ),
                        is_group_header: false,
                        provider: Some(item.provider),
                        provider_state: Some(item.provider_state),
                    });
                }
                PaletteItemKey::File { relative_path } => {
                    rows.push(PaletteViewRow {
                        key: Some(item.key.clone()),
                        text: format_file_row(relative_path, item.provider_state),
                        is_group_header: false,
                        provider: Some(item.provider),
                        provider_state: Some(item.provider_state),
                    });
                }
            }
        }
    }

    rows
}
