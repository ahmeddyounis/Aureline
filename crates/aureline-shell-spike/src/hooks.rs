//! Canonical hook-name vocabulary.
//!
//! Lifted verbatim from ADR 0002 §Protected-hot-path hook list. Every
//! benchmark, trace, and log line that references one of these events
//! MUST use the name on this page; no synonyms, no re-ordering, no
//! case changes.

/// The canonical hook set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Hook {
    WarmStartToFirstPaint,
    FirstPaint,
    ScrollFrame,
    CaretMove,
    SelectionChange,
    ImeCompositionUpdate,
    FallbackGlyphResolution,
    MultiMonitorScaleChange,
    AtlasShardRebind,
    AtlasEviction,
    FrameSubmit,
    ReflowLineRange,
    DegradedRendererBanner,
    AccessibilityTreeUpdate,
}

impl Hook {
    /// The stable string name emitted into traces, logs, and the
    /// capability manifest.
    pub const fn name(self) -> &'static str {
        match self {
            Self::WarmStartToFirstPaint => "warm_start_to_first_paint",
            Self::FirstPaint => "first_paint",
            Self::ScrollFrame => "scroll_frame",
            Self::CaretMove => "caret_move",
            Self::SelectionChange => "selection_change",
            Self::ImeCompositionUpdate => "ime_composition_update",
            Self::FallbackGlyphResolution => "fallback_glyph_resolution",
            Self::MultiMonitorScaleChange => "multi_monitor_scale_change",
            Self::AtlasShardRebind => "atlas_shard_rebind",
            Self::AtlasEviction => "atlas_eviction",
            Self::FrameSubmit => "frame_submit",
            Self::ReflowLineRange => "reflow_line_range",
            Self::DegradedRendererBanner => "degraded_renderer_banner",
            Self::AccessibilityTreeUpdate => "accessibility_tree_update",
        }
    }

    /// Whether this hook is a protected hot path per ADR 0002. Non-hot-path
    /// hooks are observability-only and do not gate release.
    pub const fn is_hot_path(self) -> bool {
        match self {
            Self::AtlasEviction | Self::DegradedRendererBanner => false,
            _ => true,
        }
    }

    /// The full enumeration in ADR 0002 order. Used by the capability
    /// manifest emitter so the printed order is deterministic.
    pub const ALL: &'static [Hook] = &[
        Self::WarmStartToFirstPaint,
        Self::FirstPaint,
        Self::ScrollFrame,
        Self::CaretMove,
        Self::SelectionChange,
        Self::ImeCompositionUpdate,
        Self::FallbackGlyphResolution,
        Self::MultiMonitorScaleChange,
        Self::AtlasShardRebind,
        Self::AtlasEviction,
        Self::FrameSubmit,
        Self::ReflowLineRange,
        Self::DegradedRendererBanner,
        Self::AccessibilityTreeUpdate,
    ];
}

#[cfg(test)]
mod tests {
    use super::Hook;

    #[test]
    fn every_hook_has_a_unique_stable_name() {
        let mut seen = Vec::new();
        for hook in Hook::ALL {
            let name = hook.name();
            assert!(
                !seen.contains(&name),
                "hook name {name} is not unique across Hook::ALL"
            );
            seen.push(name);
        }
        assert_eq!(seen.len(), Hook::ALL.len());
    }

    #[test]
    fn names_match_adr_vocabulary() {
        assert_eq!(Hook::FrameSubmit.name(), "frame_submit");
        assert_eq!(Hook::CaretMove.name(), "caret_move");
        assert_eq!(
            Hook::WarmStartToFirstPaint.name(),
            "warm_start_to_first_paint"
        );
    }

    #[test]
    fn non_hot_path_hooks_are_observability_only() {
        assert!(!Hook::AtlasEviction.is_hot_path());
        assert!(!Hook::DegradedRendererBanner.is_hot_path());
        assert!(Hook::FrameSubmit.is_hot_path());
    }
}
