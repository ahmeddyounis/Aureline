//! Render-side helpers for host-owned placeholder cards in the desktop shell.
//!
//! These helpers are intentionally lightweight: they provide stable labels and
//! text blocks for shell-zone placeholder occupants without requiring each
//! surface to mint its own copy.

use super::DegradedStateToken;

/// Minimal text model for a host-owned placeholder card rendered inside a shell slot.
#[derive(Debug, Clone, Copy)]
pub struct ShellPlaceholderCard {
    /// Card title (usually the slot label).
    pub title: &'static str,
    /// Short one-line summary rendered under the title.
    pub summary: &'static str,
    degraded_tokens: [DegradedStateToken; 2],
    degraded_token_count: usize,
}

impl ShellPlaceholderCard {
    /// Creates a placeholder card model for the given shell slot id.
    pub fn for_slot(slot_id: &str, degraded_tokens: &[DegradedStateToken]) -> Self {
        let title = shell_slot_label(slot_id);
        let (summary, tokens, count) = if degraded_tokens.is_empty() {
            (
                DegradedStateToken::Unsupported.default_description(),
                [
                    DegradedStateToken::Unsupported,
                    DegradedStateToken::Unsupported,
                ],
                1,
            )
        } else {
            let summary = degraded_tokens
                .first()
                .map(|token| token.default_description())
                .unwrap_or_else(|| DegradedStateToken::Limited.default_description());
            let first = degraded_tokens[0];
            let second = degraded_tokens.get(1).copied().unwrap_or(first);
            (summary, [first, second], degraded_tokens.len().min(2))
        };
        Self {
            title,
            summary,
            degraded_tokens: tokens,
            degraded_token_count: count,
        }
    }

    /// Returns the degraded tokens to render as badges.
    pub fn degraded_tokens(&self) -> &[DegradedStateToken] {
        &self.degraded_tokens[..self.degraded_token_count]
    }
}

/// Returns the stable human-readable label for a shell slot id.
pub fn shell_slot_label(slot_id: &str) -> &'static str {
    match slot_id {
        "slot.title_context_bar.identity" => "Title/context bar identity chrome",
        "slot.activity_rail.primary_routes" => "Activity rail primary route entries",
        "slot.sidebar.section_surface" => "Sidebar section surface body",
        "slot.main_workspace.working_set" => "Main workspace working-set container",
        "slot.main_workspace.review_surface" => "Main workspace review/approval surface container",
        "slot.right_inspector.contextual_detail" => "Right inspector contextual detail container",
        "slot.bottom_panel.tool_panels" => "Bottom panel tool panel container",
        "status.slot.recovery.primary" => "Status bar recovery-primary slot",
        "status.slot.extension.scoped" => "Status bar extension-scoped slot",
        "slot.overlay.command_palette" => "Command palette overlay slot",
        "slot.overlay.dialog_or_sheet" => "Dialog/sheet overlay slot",
        _ => "Shell surface",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_slot_labels_match_seeded_slot_definitions() {
        assert_eq!(
            shell_slot_label("slot.title_context_bar.identity"),
            "Title/context bar identity chrome"
        );
        assert_eq!(
            shell_slot_label("status.slot.recovery.primary"),
            "Status bar recovery-primary slot"
        );
        assert_eq!(
            shell_slot_label("slot.overlay.command_palette"),
            "Command palette overlay slot"
        );
    }

    #[test]
    fn placeholder_card_caps_degraded_tokens_at_two() {
        let card = ShellPlaceholderCard::for_slot(
            "slot.main_workspace.working_set",
            &[
                DegradedStateToken::Warming,
                DegradedStateToken::Partial,
                DegradedStateToken::Offline,
            ],
        );
        assert_eq!(card.degraded_tokens().len(), 2);
        assert_eq!(card.degraded_tokens()[0], DegradedStateToken::Warming);
        assert_eq!(card.degraded_tokens()[1], DegradedStateToken::Partial);
    }

    #[test]
    fn placeholder_card_defaults_to_unsupported_when_tokens_missing() {
        let card = ShellPlaceholderCard::for_slot("slot.main_workspace.working_set", &[]);
        assert_eq!(card.degraded_tokens(), &[DegradedStateToken::Unsupported]);
    }
}
