//! Component-state registry and shared visual treatment.
//!
//! The registry is the canonical place to define the shared component-state
//! vocabulary and its baseline token-backed treatments. Surfaces should map
//! their local state machines back to this vocabulary instead of minting
//! one-off state labels or private styling rules.

use std::ops::{BitOr, BitOrAssign};

use serde::{Deserialize, Serialize};

use crate::tokens::{alpha_state_semantics_registry, ColorRgba, TokenRegistry, TokenRegistryError};

/// Closed component-state vocabulary used across reusable component contracts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentStateClass {
    /// Default surface state.
    Idle,
    /// Pointer hover.
    Hover,
    /// Focused element.
    Focus,
    /// Focus rendered because the operator is likely using keyboard or AT.
    FocusVisible,
    /// Pointer press or keyboard activation in flight.
    Pressed,
    /// Durable selection across focus changes.
    Selected,
    /// Current route/location or live context owner.
    Current,
    /// Unavailable and non-actionable in the current context.
    Disabled,
    /// Inspectable but not editable or writable.
    ReadOnly,
    /// Background work in progress for this surface.
    Loading,
    /// User action submitted but not yet committed.
    Pending,
    /// Reduced capability remains; certainty/freshness is lowered.
    Degraded,
    /// Last-known-good shown while refresh lags.
    Stale,
    /// Trust narrowing posture in effect.
    Restricted,
    /// Admin policy narrowing posture in effect.
    PolicyBlocked,
    /// Policy/trust/permission/ownership/source lock posture.
    Locked,
    /// Warning posture worth surfacing.
    Warning,
    /// Destructive action posture.
    Destructive,
    /// Live reconnecting posture (remote attach / collaboration / provider).
    Reconnecting,
    /// Durable success posture.
    Completed,
    /// Post-recovery posture.
    Restored,
    /// Durable-attention row held by quiet hours.
    QuietHoursHeld,
}

impl ComponentStateClass {
    /// Returns the canonical snake-case state name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Hover => "hover",
            Self::Focus => "focus",
            Self::FocusVisible => "focus_visible",
            Self::Pressed => "pressed",
            Self::Selected => "selected",
            Self::Current => "current",
            Self::Disabled => "disabled",
            Self::ReadOnly => "read_only",
            Self::Loading => "loading",
            Self::Pending => "pending",
            Self::Degraded => "degraded",
            Self::Stale => "stale",
            Self::Restricted => "restricted",
            Self::PolicyBlocked => "policy_blocked",
            Self::Locked => "locked",
            Self::Warning => "warning",
            Self::Destructive => "destructive",
            Self::Reconnecting => "reconnecting",
            Self::Completed => "completed",
            Self::Restored => "restored",
            Self::QuietHoursHeld => "quiet_hours_held",
        }
    }
}

/// Bitmask of component state flags applied to a rendered control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ComponentStates(u32);

impl ComponentStates {
    /// Empty component state set.
    pub const NONE: Self = Self(0);

    /// Pointer hover.
    pub const HOVER: Self = Self(1 << 0);
    /// Focus-visible posture (keyboard or assistive tech).
    pub const FOCUS_VISIBLE: Self = Self(1 << 1);
    /// Activation-in-flight posture.
    pub const PRESSED: Self = Self(1 << 2);
    /// Durable selection posture.
    pub const SELECTED: Self = Self(1 << 3);
    /// Disabled posture.
    pub const DISABLED: Self = Self(1 << 4);
    /// Loading posture.
    pub const LOADING: Self = Self(1 << 5);
    /// Stale posture.
    pub const STALE: Self = Self(1 << 6);
    /// Warning posture.
    pub const WARNING: Self = Self(1 << 7);
    /// Destructive posture.
    pub const DESTRUCTIVE: Self = Self(1 << 8);

    /// Returns `true` when this set contains all flags in `other`.
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl BitOr for ComponentStates {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for ComponentStates {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Surface tone used when selecting baseline fill and border treatments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentSurfaceTone {
    /// A normal surface inside the app chrome (cards, rows, panels).
    Surface,
    /// A raised surface that sits above surrounding chrome (overlays, sheets).
    Raised,
}

/// Token-backed focus ring treatment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FocusRingStyle {
    /// Stroke width in logical pixels.
    pub stroke_px: u32,
    /// Ring color.
    pub color: ColorRgba,
}

/// Token-backed chrome treatment for a rendered control surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComponentChromeStyle {
    /// Fill color for the control surface.
    pub fill: ColorRgba,
    /// Border color for the control surface.
    pub border: ColorRgba,
    /// Border stroke width in logical pixels.
    pub border_stroke_px: u32,
    /// Optional focus ring applied on top of the border.
    pub focus_ring: Option<FocusRingStyle>,
}

#[derive(Debug, Clone)]
struct ComponentStateTokens {
    bg_surface: ColorRgba,
    bg_raised: ColorRgba,
    bg_hover: ColorRgba,
    bg_active: ColorRgba,
    border_default: ColorRgba,
    focus_ring: ColorRgba,
    accent_interactive: ColorRgba,
    status_warning_border: ColorRgba,
    status_danger_border: ColorRgba,
}

/// Token-backed component-state registry used by protected shell surfaces.
#[derive(Debug, Clone)]
pub struct ComponentStateRegistry {
    tokens: ComponentStateTokens,
    stroke_default_px: u32,
    stroke_focus_px: u32,
}

impl ComponentStateRegistry {
    /// Loads the registry from the provided semantic token registry.
    pub fn load(registry: &TokenRegistry) -> Result<Self, TokenRegistryError> {
        alpha_state_semantics_registry().map_err(|err| {
            TokenRegistryError::LoadFailed("state semantics registry", err.to_string())
        })?;

        Ok(Self {
            tokens: ComponentStateTokens {
                bg_surface: registry.require_color("al.color.bg.surface")?,
                bg_raised: registry.require_color("al.color.bg.raised")?,
                bg_hover: registry.require_color("al.color.bg.hover")?,
                bg_active: registry.require_color("al.color.bg.active")?,
                border_default: registry.require_color("al.color.border.default")?,
                focus_ring: registry.require_color("al.color.focus.ring")?,
                accent_interactive: registry.require_color("al.color.accent.interactive")?,
                status_warning_border: registry.require_color("status.warning.border")?,
                status_danger_border: registry.require_color("status.danger.border")?,
            },
            stroke_default_px: registry.require_stroke_px("stroke.border.default")?,
            stroke_focus_px: registry.require_stroke_px("stroke.focus.ring")?,
        })
    }

    /// Returns the token-backed focus ring treatment.
    pub const fn focus_ring_style(&self) -> FocusRingStyle {
        FocusRingStyle {
            stroke_px: self.stroke_focus_px,
            color: self.tokens.focus_ring,
        }
    }

    /// Returns the token-backed chrome treatment for a component surface.
    pub fn chrome_style(
        &self,
        tone: ComponentSurfaceTone,
        states: ComponentStates,
    ) -> ComponentChromeStyle {
        let fill = match tone {
            ComponentSurfaceTone::Surface => self.tokens.bg_surface,
            ComponentSurfaceTone::Raised => self.tokens.bg_raised,
        };

        let mut style = ComponentChromeStyle {
            fill,
            border: self.tokens.border_default,
            border_stroke_px: self.stroke_default_px,
            focus_ring: None,
        };

        if states.contains(ComponentStates::HOVER) {
            style.fill = self.tokens.bg_hover;
        }

        if states.contains(ComponentStates::PRESSED) {
            style.fill = self.tokens.bg_active;
        }

        if states.contains(ComponentStates::SELECTED) {
            style.fill = self.tokens.bg_hover;
            style.border = self.tokens.accent_interactive;
            style.border_stroke_px = self.stroke_focus_px.max(style.border_stroke_px);
        }

        if states.contains(ComponentStates::WARNING) {
            style.border = self.tokens.status_warning_border;
            style.border_stroke_px = self.stroke_focus_px.max(style.border_stroke_px);
        }

        if states.contains(ComponentStates::DESTRUCTIVE) {
            style.border = self.tokens.status_danger_border;
            style.border_stroke_px = self.stroke_focus_px.max(style.border_stroke_px);
        }

        if states.contains(ComponentStates::FOCUS_VISIBLE) {
            style.focus_ring = Some(self.focus_ring_style());
        }

        style
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tokens::{seeded_token_registry, ThemeClass};

    #[test]
    fn loads_registry_from_seeded_tokens() {
        let tokens = seeded_token_registry(ThemeClass::DarkReference).expect("seeded tokens");
        let registry = ComponentStateRegistry::load(tokens).expect("load component-state registry");
        assert!(registry.focus_ring_style().stroke_px > 0);
    }

    #[test]
    fn focus_visible_adds_focus_ring() {
        let tokens = seeded_token_registry(ThemeClass::DarkReference).expect("seeded tokens");
        let registry = ComponentStateRegistry::load(tokens).expect("load component-state registry");
        let style = registry.chrome_style(
            ComponentSurfaceTone::Surface,
            ComponentStates::FOCUS_VISIBLE,
        );
        assert!(style.focus_ring.is_some());
    }
}
