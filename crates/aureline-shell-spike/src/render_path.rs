//! Render seam.
//!
//! The spike does not own the GPU; it declares the layer boundary, the
//! damage / invalidation entry points, and the placeholder-surface
//! ownership the renderer crate will implement. The render path is
//! intentionally a pure data-flow: events in, composited frame records
//! out. The binary binds this to a window; tests bind it to a recorder.

use crate::hooks::Hook;
use crate::input_path::{InputAction, SelectionDelta};
use crate::zones::{Rect, ShellFrame, ZoneId};

/// Which of the ADR 0002 two-layer scene the paint touches.
///
/// The spike's invalidation rule is identical to the ADR's: caret,
/// selection, and IME composition ride the overlay layer and never
/// re-raster glyphs. Text reflow and scroll ride the text-and-decoration
/// layer and may re-shape clusters. This mapping lives in one function
/// ([`classify_layer`]) so the renderer crate only needs one seam to
/// replace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    TextAndDecoration,
    Overlay,
}

impl Layer {
    pub const fn name(self) -> &'static str {
        match self {
            Self::TextAndDecoration => "text_and_decoration",
            Self::Overlay => "overlay",
        }
    }
}

/// A single damage record. The compositor coalesces per zone per frame;
/// the fixture trace emits them in arrival order so the sequence is
/// inspectable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DamageRecord {
    pub zone: ZoneId,
    pub layer: Layer,
    pub rect: Rect,
    pub hook: Hook,
}

/// Placeholder surface ownership. Each zone owns one surface; the title
/// bar and status bar are static and use ownership mode `Stable`. The
/// editor viewport hosts the text-and-decoration layer and uses
/// `TextPipeline`. The sidebar is a placeholder list surface and uses
/// `Stable` until the panel system lands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceOwnership {
    Stable,
    TextPipeline,
}

impl SurfaceOwnership {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::TextPipeline => "text_pipeline",
        }
    }

    pub const fn for_zone(zone: ZoneId) -> Self {
        match zone {
            ZoneId::EditorViewport => Self::TextPipeline,
            ZoneId::TitleBar | ZoneId::Sidebar | ZoneId::StatusBar => Self::Stable,
        }
    }
}

/// Classify an action into the layer it damages and the zone it targets.
/// This is the single function that encodes the ADR's overlay-separation
/// rule; the renderer crate may replace it but may not contradict it.
pub fn classify(frame: &ShellFrame, action: &InputAction) -> Option<DamageRecord> {
    let (zone, layer, hook) = match action {
        InputAction::InsertText(_) => (
            ZoneId::EditorViewport,
            Layer::TextAndDecoration,
            Hook::ReflowLineRange,
        ),
        InputAction::MoveCaret(_) => (ZoneId::EditorViewport, Layer::Overlay, Hook::CaretMove),
        InputAction::ChangeSelection(delta) => match delta {
            SelectionDelta::Cleared
            | SelectionDelta::ExtendedLeft
            | SelectionDelta::ExtendedRight => (
                ZoneId::EditorViewport,
                Layer::Overlay,
                Hook::SelectionChange,
            ),
        },
        InputAction::UpdateComposition(_) => (
            ZoneId::EditorViewport,
            Layer::Overlay,
            Hook::ImeCompositionUpdate,
        ),
        InputAction::Scroll { .. } => (
            ZoneId::EditorViewport,
            Layer::TextAndDecoration,
            Hook::ScrollFrame,
        ),
        InputAction::ScaleChange { .. } => (
            ZoneId::EditorViewport,
            Layer::TextAndDecoration,
            Hook::MultiMonitorScaleChange,
        ),
        InputAction::None => return None,
    };
    Some(DamageRecord {
        zone,
        layer,
        rect: frame.zone(zone),
        hook,
    })
}

pub const fn classify_layer(action: &InputAction) -> Option<Layer> {
    match action {
        InputAction::InsertText(_)
        | InputAction::Scroll { .. }
        | InputAction::ScaleChange { .. } => Some(Layer::TextAndDecoration),
        InputAction::MoveCaret(_)
        | InputAction::ChangeSelection(_)
        | InputAction::UpdateComposition(_) => Some(Layer::Overlay),
        InputAction::None => None,
    }
}

/// The frame the compositor submits to the GPU. In the spike this is a
/// record, not a real draw call; the binary feeds it to whichever
/// backend is linked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompositedFrame {
    pub frame_index: u64,
    pub damage: Vec<DamageRecord>,
}

impl CompositedFrame {
    pub fn new(frame_index: u64) -> Self {
        Self {
            frame_index,
            damage: Vec::new(),
        }
    }

    pub fn push(&mut self, record: DamageRecord) {
        self.damage.push(record);
    }

    pub fn is_empty(&self) -> bool {
        self.damage.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_path::CaretMove;

    fn frame() -> ShellFrame {
        ShellFrame::fixture()
    }

    #[test]
    fn caret_move_rides_overlay_never_text_layer() {
        let record = classify(&frame(), &InputAction::MoveCaret(CaretMove::Left)).unwrap();
        assert_eq!(record.layer, Layer::Overlay);
        assert_eq!(record.hook, Hook::CaretMove);
        assert_eq!(record.zone, ZoneId::EditorViewport);
    }

    #[test]
    fn text_insertion_rides_text_layer() {
        let record = classify(&frame(), &InputAction::InsertText("x".to_owned())).unwrap();
        assert_eq!(record.layer, Layer::TextAndDecoration);
        assert_eq!(record.hook, Hook::ReflowLineRange);
    }

    #[test]
    fn none_action_produces_no_damage() {
        assert!(classify(&frame(), &InputAction::None).is_none());
    }

    #[test]
    fn editor_uses_text_pipeline_other_zones_stable() {
        assert_eq!(
            SurfaceOwnership::for_zone(ZoneId::EditorViewport),
            SurfaceOwnership::TextPipeline
        );
        for zone in [ZoneId::TitleBar, ZoneId::Sidebar, ZoneId::StatusBar] {
            assert_eq!(SurfaceOwnership::for_zone(zone), SurfaceOwnership::Stable);
        }
    }
}
