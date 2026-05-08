//! Dirty-region computation and retained-frame planning.
//!
//! The renderer and shell exchange invalidation intent via [`crate::DamageEvent`] records. This
//! module turns those intent records into a conservative set of pixel rectangles that must be
//! repainted for the next frame.
//!
//! The engine is intentionally conservative: when an event does not include a concrete rectangle,
//! the planner escalates to a full-window repaint. This keeps correctness attached to the live
//! render path while the editor/view models grow the higher-fidelity damage vocabulary.

use crate::draw_queue::{DamageEvent, PixelRect};

/// The repaint strategy selected for the next frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirtyRegionStrategy {
    /// Redraw the entire window.
    FullWindow,
    /// Redraw only the returned dirty rectangles.
    Partial { rects: Vec<PixelRect> },
}

impl DirtyRegionStrategy {
    /// Returns the dirty rectangles implied by this strategy.
    pub fn rects(&self, window_bounds: PixelRect) -> Vec<PixelRect> {
        match self {
            DirtyRegionStrategy::FullWindow => vec![window_bounds],
            DirtyRegionStrategy::Partial { rects } => rects.clone(),
        }
    }
}

/// Output of dirty-region planning for one composited frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirtyRegionPlan {
    pub window_bounds: PixelRect,
    pub strategy: DirtyRegionStrategy,
}

impl DirtyRegionPlan {
    /// Returns true when the planner requires a full-window repaint.
    pub fn is_full_window(&self) -> bool {
        matches!(self.strategy, DirtyRegionStrategy::FullWindow)
    }

    /// Returns the dirty rectangles for this plan.
    pub fn rects(&self) -> Vec<PixelRect> {
        self.strategy.rects(self.window_bounds)
    }
}

/// Computes a repaint plan from queued damage events.
#[derive(Debug, Default)]
pub struct DirtyRegionEngine;

impl DirtyRegionEngine {
    /// Plans which pixel rectangles must be repainted for `events`.
    pub fn plan(window_bounds: PixelRect, events: &[DamageEvent]) -> DirtyRegionPlan {
        if window_bounds.is_empty() {
            return DirtyRegionPlan {
                window_bounds,
                strategy: DirtyRegionStrategy::FullWindow,
            };
        }

        let mut rects = Vec::new();
        for event in events {
            if event.region.is_unspecified() {
                return DirtyRegionPlan {
                    window_bounds,
                    strategy: DirtyRegionStrategy::FullWindow,
                };
            }
            if let Some(rect) = event.region.rect() {
                rects.push(rect);
            }
        }

        let mut clipped = Vec::new();
        for rect in rects {
            if let Some(intersection) = rect.intersection(window_bounds) {
                if !intersection.is_empty() {
                    clipped.push(intersection);
                }
            }
        }

        if clipped.is_empty() {
            return DirtyRegionPlan {
                window_bounds,
                strategy: DirtyRegionStrategy::FullWindow,
            };
        }

        if should_promote_to_full_window(window_bounds, &clipped) {
            return DirtyRegionPlan {
                window_bounds,
                strategy: DirtyRegionStrategy::FullWindow,
            };
        }

        DirtyRegionPlan {
            window_bounds,
            strategy: DirtyRegionStrategy::Partial { rects: clipped },
        }
    }
}

fn should_promote_to_full_window(window_bounds: PixelRect, rects: &[PixelRect]) -> bool {
    const MAX_RECTS: usize = 64;
    const AREA_THRESHOLD_NUM: u64 = 7;
    const AREA_THRESHOLD_DEN: u64 = 10;

    if rects.len() > MAX_RECTS {
        return true;
    }

    let mut area = 0u64;
    for rect in rects {
        area = area.saturating_add(rect.area());
    }
    area.saturating_mul(AREA_THRESHOLD_DEN)
        >= window_bounds.area().saturating_mul(AREA_THRESHOLD_NUM)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::draw_queue::{CompositionLayerId, DamageClassId, DamageEvent, DamageRegion};

    #[test]
    fn promotes_unspecified_region_to_full_window() {
        let window_bounds = PixelRect::new(0, 0, 100, 100);
        let event = DamageEvent::new(
            CompositionLayerId::WindowChromeBase,
            DamageClassId::WindowExposedRegionRefresh,
        );
        let plan = DirtyRegionEngine::plan(window_bounds, &[event]);
        assert!(plan.is_full_window());
    }

    #[test]
    fn keeps_rectangular_damage_inside_window_bounds() {
        let window_bounds = PixelRect::new(0, 0, 10, 10);
        let event = DamageEvent::with_region(
            CompositionLayerId::OverlayEphemera,
            DamageClassId::CaretOverlayOnly,
            DamageRegion::Rect(PixelRect::new(8, 8, 10, 10)),
        );
        let plan = DirtyRegionEngine::plan(window_bounds, &[event]);
        assert!(!plan.is_full_window());
        assert_eq!(plan.rects(), vec![PixelRect::new(8, 8, 2, 2)]);
    }

    #[test]
    fn promotes_large_dirty_coverage_to_full_window() {
        let window_bounds = PixelRect::new(0, 0, 100, 100);
        let event = DamageEvent::with_region(
            CompositionLayerId::TextAndDecoration,
            DamageClassId::TextReflowLocal,
            DamageRegion::Rect(PixelRect::new(0, 0, 100, 90)),
        );
        let plan = DirtyRegionEngine::plan(window_bounds, &[event]);
        assert!(plan.is_full_window());
    }
}
