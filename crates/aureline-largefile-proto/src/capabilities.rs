//! Frozen capability split between normal mode and limited mode.
//!
//! The ADR's reduced-or-denied list is reproduced here as one
//! reviewable constant table per mode. Later UX work consumes
//! these rows when it builds banners that explain why an action
//! is denied or downgraded; later refactor / save / AI-apply work
//! consumes them to gate which paths the large-file controller
//! routes through. Adding or moving a row is an ADR amendment,
//! not a silent code change.

/// Whether a capability is fully available, denied entirely, or
/// downgraded (offered in a narrower form).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CapabilityState {
    Allowed,
    Denied,
    Downgraded,
}

impl CapabilityState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::Denied => "denied",
            Self::Downgraded => "downgraded",
        }
    }
}

/// One reviewable row of the capability split.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CapabilityRow {
    /// Stable identifier; lanes that branch on the row cite this id.
    pub id: &'static str,
    pub state: CapabilityState,
    /// Short human-readable note the UX banner can quote. `None`
    /// for `Allowed` rows in normal mode.
    pub note: Option<&'static str>,
}

/// Normal mode: every capability is available. Listed explicitly
/// so the limited-mode rows can be diffed against the same id set.
pub const NORMAL_MODE_CAPABILITIES: &[CapabilityRow] = &[
    row("view", CapabilityState::Allowed, None),
    row("search_viewport", CapabilityState::Allowed, None),
    row("search_whole_file", CapabilityState::Allowed, None),
    row("copy", CapabilityState::Allowed, None),
    row("diagnostics_viewport", CapabilityState::Allowed, None),
    row("diagnostics_whole_file", CapabilityState::Allowed, None),
    row("multi_cursor_viewport", CapabilityState::Allowed, None),
    row("multi_cursor_whole_file", CapabilityState::Allowed, None),
    row("full_file_format_on_save", CapabilityState::Allowed, None),
    row("range_format_on_save", CapabilityState::Allowed, None),
    row("full_file_reflow", CapabilityState::Allowed, None),
    row("viewport_reflow", CapabilityState::Allowed, None),
    row("whole_file_syntax_parse", CapabilityState::Allowed, None),
    row("viewport_syntax_parse", CapabilityState::Allowed, None),
    row("indexing", CapabilityState::Allowed, None),
    row("background_analysis", CapabilityState::Allowed, None),
    row("cursor_local_lookup", CapabilityState::Allowed, None),
    row("whole_file_load_into_ram", CapabilityState::Allowed, None),
    row("rich_refactor_single_file", CapabilityState::Allowed, None),
    row("rich_refactor_multi_file", CapabilityState::Allowed, None),
    row("ai_apply_range", CapabilityState::Allowed, None),
    row("ai_apply_whole_file", CapabilityState::Allowed, None),
    row("save_participant_range_only", CapabilityState::Allowed, None),
    row("save_participant_whole_file", CapabilityState::Allowed, None),
    row("undo_redo_history_full", CapabilityState::Allowed, None),
    row("accessibility_tree_viewport", CapabilityState::Allowed, None),
];

/// Limited mode: the ADR-frozen reduced-or-denied list.
///
/// Identifiers MUST stay aligned with [`NORMAL_MODE_CAPABILITIES`]
/// so a diff between the two columns is the contract.
pub const LIMITED_MODE_CAPABILITIES: &[CapabilityRow] = &[
    row("view", CapabilityState::Allowed, Some("read-only viewport rendering")),
    row(
        "search_viewport",
        CapabilityState::Allowed,
        Some("viewport-bounded search remains"),
    ),
    row(
        "search_whole_file",
        CapabilityState::Downgraded,
        Some("offered as streaming search variant only"),
    ),
    row("copy", CapabilityState::Allowed, Some("selection copy remains")),
    row(
        "diagnostics_viewport",
        CapabilityState::Allowed,
        Some("viewport-bounded diagnostics remain"),
    ),
    row(
        "diagnostics_whole_file",
        CapabilityState::Denied,
        Some("full-file diagnostics are denied in large-file mode"),
    ),
    row(
        "multi_cursor_viewport",
        CapabilityState::Allowed,
        Some("viewport-bounded multi-cursor remains"),
    ),
    row(
        "multi_cursor_whole_file",
        CapabilityState::Denied,
        Some("multi-cursor edits across the whole file are denied"),
    ),
    row(
        "full_file_format_on_save",
        CapabilityState::Denied,
        Some("save participants that rewrite the whole file are denied"),
    ),
    row(
        "range_format_on_save",
        CapabilityState::Allowed,
        Some("save participants that touch only the edited range remain"),
    ),
    row(
        "full_file_reflow",
        CapabilityState::Denied,
        Some("full-file reflow is denied; viewport-bounded reflow remains"),
    ),
    row(
        "viewport_reflow",
        CapabilityState::Allowed,
        Some("viewport-bounded reflow remains"),
    ),
    row(
        "whole_file_syntax_parse",
        CapabilityState::Denied,
        Some("full-file syntax parsing is denied; viewport-bounded variants remain"),
    ),
    row(
        "viewport_syntax_parse",
        CapabilityState::Allowed,
        Some("viewport-bounded syntax parsing remains"),
    ),
    row(
        "indexing",
        CapabilityState::Denied,
        Some("background indexing is denied for large-file buffers"),
    ),
    row(
        "background_analysis",
        CapabilityState::Denied,
        Some("background analysis is denied for large-file buffers"),
    ),
    row(
        "cursor_local_lookup",
        CapabilityState::Allowed,
        Some("on-demand, cursor-local lookups remain"),
    ),
    row(
        "whole_file_load_into_ram",
        CapabilityState::Denied,
        Some("the buffer is paged through an mmap-backed reader; no whole-file load"),
    ),
    row(
        "rich_refactor_single_file",
        CapabilityState::Denied,
        Some("rich refactor is denied in large-file mode"),
    ),
    row(
        "rich_refactor_multi_file",
        CapabilityState::Denied,
        Some("rich multi-file refactor is denied in large-file mode"),
    ),
    row(
        "ai_apply_range",
        CapabilityState::Downgraded,
        Some("AI apply is offered for ranges only; whole-file apply is denied"),
    ),
    row(
        "ai_apply_whole_file",
        CapabilityState::Denied,
        Some("whole-file AI apply is denied in large-file mode"),
    ),
    row(
        "save_participant_range_only",
        CapabilityState::Allowed,
        Some("range-only save participants remain"),
    ),
    row(
        "save_participant_whole_file",
        CapabilityState::Denied,
        Some("whole-file save participants are denied"),
    ),
    row(
        "undo_redo_history_full",
        CapabilityState::Downgraded,
        Some(
            "coalescing windows shorten and per-edit storage bounds tighten; the journal MAY reject a transaction whose stored inverse exceeds the per-buffer cap",
        ),
    ),
    row(
        "accessibility_tree_viewport",
        CapabilityState::Allowed,
        Some("accessibility tree remains populated for the visible viewport"),
    ),
];

const fn row(id: &'static str, state: CapabilityState, note: Option<&'static str>) -> CapabilityRow {
    CapabilityRow { id, state, note }
}

/// Look up a capability id in a mode's table. Returns `None` if
/// the id is not present in either table — that means the lane
/// is asking about a capability the ADR table never named, which
/// is a contract bug.
pub fn lookup<'a>(
    table: &'a [CapabilityRow],
    id: &str,
) -> Option<&'a CapabilityRow> {
    table.iter().find(|r| r.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn ids_align_between_modes() {
        let normal: BTreeSet<&'static str> =
            NORMAL_MODE_CAPABILITIES.iter().map(|r| r.id).collect();
        let limited: BTreeSet<&'static str> =
            LIMITED_MODE_CAPABILITIES.iter().map(|r| r.id).collect();
        assert_eq!(
            normal, limited,
            "normal and limited tables MUST cover the same capability ids so the split is reviewable as a diff"
        );
    }

    #[test]
    fn limited_mode_denies_at_least_one_whole_file_capability() {
        let any_denied = LIMITED_MODE_CAPABILITIES
            .iter()
            .any(|r| r.state == CapabilityState::Denied);
        assert!(any_denied);
    }

    #[test]
    fn whole_file_load_is_denied_in_limited_mode() {
        let row = lookup(LIMITED_MODE_CAPABILITIES, "whole_file_load_into_ram").unwrap();
        assert_eq!(row.state, CapabilityState::Denied);
    }

    #[test]
    fn every_limited_row_has_a_note() {
        for row in LIMITED_MODE_CAPABILITIES {
            assert!(
                row.note.is_some(),
                "limited-mode row {} has no note",
                row.id
            );
        }
    }

    #[test]
    fn capability_ids_are_lowercase_snake() {
        for row in NORMAL_MODE_CAPABILITIES {
            assert!(
                row.id
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'),
                "id {} is not [a-z0-9_]+",
                row.id
            );
        }
    }
}
