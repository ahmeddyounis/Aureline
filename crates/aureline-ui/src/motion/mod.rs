//! Motion timing guards and reduced-motion policy.
//!
//! Aureline treats reduced motion as a first-class runtime posture. Shell and
//! other protected UI surfaces must consult the shared policy before running
//! transitions so:
//!
//! - motion never becomes the only carrier of state,
//! - protected-path interactions remain responsive, and
//! - postures such as `motion_reduced` and `motion_critical_hot_path` suppress
//!   or simplify non-essential motion.
//!
//! The canonical duration tokens live in `artifacts/design/motion_tokens.yaml`.

use std::time::Duration;

use crate::themes::AccessibilityPostureClass;
use crate::tokens::{TokenRegistry, TokenRegistryError};

/// Closed vocabulary describing how a motion preset adapts under constrained postures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReducedMotionSubstitutionClass {
    /// Render an opacity-only transition (no translation or scale).
    CrossfadeOnly,
    /// Keep a simplified version of the motion where state conveyance requires it.
    MaintainEssentialKeepSimplified,
    /// Remove the motion entirely but preserve static state markers.
    SuppressEntirely,
    /// Collapse the transition to an instantaneous cut.
    CollapseToInstant,
    /// Promote a non-motion state marker (chip/label/icon) in place of motion.
    NonMotionStateMarker,
}

/// A motion-preset fallback row pinned to one accessibility posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MotionFallback {
    pub posture: AccessibilityPostureClass,
    pub substitution_class: ReducedMotionSubstitutionClass,
    pub duration_token: Option<&'static str>,
    pub easing_token: Option<&'static str>,
}

/// Describes how a transition should behave under the active posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MotionPlan {
    pub posture: AccessibilityPostureClass,
    pub substitution_class: Option<ReducedMotionSubstitutionClass>,
    pub duration_token: Option<&'static str>,
    pub easing_token: Option<&'static str>,
}

impl MotionPlan {
    /// Resolves the planned duration to milliseconds.
    pub fn duration_ms(self, registry: &TokenRegistry) -> Result<u32, TokenRegistryError> {
        let Some(token) = self.duration_token else {
            return Ok(0);
        };
        registry.require_motion_ms(token)
    }

    /// Resolves the planned duration to a [`Duration`].
    pub fn duration(self, registry: &TokenRegistry) -> Result<Duration, TokenRegistryError> {
        let ms = self.duration_ms(registry)?;
        Ok(Duration::from_millis(u64::from(ms)))
    }
}

/// Canonical definition for a named motion preset used by protected surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MotionPresetDefinition {
    pub preset_id: &'static str,
    pub default_duration_token: &'static str,
    pub default_easing_token: &'static str,
    pub reduced_motion: MotionFallback,
    pub low_motion: MotionFallback,
    pub power_saver: MotionFallback,
    pub critical_hot_path: MotionFallback,
}

impl MotionPresetDefinition {
    /// Resolves the motion plan for a given posture.
    pub const fn plan_for(self, posture: AccessibilityPostureClass) -> MotionPlan {
        match posture {
            AccessibilityPostureClass::MotionStandard => MotionPlan {
                posture,
                substitution_class: None,
                duration_token: Some(self.default_duration_token),
                easing_token: Some(self.default_easing_token),
            },
            AccessibilityPostureClass::MotionReduced => MotionPlan {
                posture,
                substitution_class: Some(self.reduced_motion.substitution_class),
                duration_token: self.reduced_motion.duration_token,
                easing_token: self.reduced_motion.easing_token,
            },
            AccessibilityPostureClass::MotionLowMotion => MotionPlan {
                posture,
                substitution_class: Some(self.low_motion.substitution_class),
                duration_token: self.low_motion.duration_token,
                easing_token: self.low_motion.easing_token,
            },
            AccessibilityPostureClass::MotionPowerSaver => MotionPlan {
                posture,
                substitution_class: Some(self.power_saver.substitution_class),
                duration_token: self.power_saver.duration_token,
                easing_token: self.power_saver.easing_token,
            },
            AccessibilityPostureClass::MotionCriticalHotPath => MotionPlan {
                posture,
                substitution_class: Some(self.critical_hot_path.substitution_class),
                duration_token: self.critical_hot_path.duration_token,
                easing_token: self.critical_hot_path.easing_token,
            },
        }
    }
}

/// Returns an ordering key that increases with restrictiveness.
pub const fn posture_precedence(posture: AccessibilityPostureClass) -> u8 {
    match posture {
        AccessibilityPostureClass::MotionStandard => 0,
        AccessibilityPostureClass::MotionReduced => 1,
        AccessibilityPostureClass::MotionPowerSaver => 2,
        AccessibilityPostureClass::MotionLowMotion => 3,
        AccessibilityPostureClass::MotionCriticalHotPath => 4,
    }
}

/// Returns the more restrictive of two postures.
pub const fn more_restrictive(
    left: AccessibilityPostureClass,
    right: AccessibilityPostureClass,
) -> AccessibilityPostureClass {
    if posture_precedence(left) >= posture_precedence(right) {
        left
    } else {
        right
    }
}

/// Canonical overlay-dialog enter preset, aligned with the motion case fixtures.
pub const OVERLAY_DIALOG_ENTER: MotionPresetDefinition = MotionPresetDefinition {
    preset_id: "motion_preset:overlay.dialog.enter",
    default_duration_token: "motion.dialog",
    default_easing_token: "ease.enter",
    reduced_motion: MotionFallback {
        posture: AccessibilityPostureClass::MotionReduced,
        substitution_class: ReducedMotionSubstitutionClass::CrossfadeOnly,
        duration_token: Some("motion.fast"),
        easing_token: Some("ease.standard"),
    },
    low_motion: MotionFallback {
        posture: AccessibilityPostureClass::MotionLowMotion,
        substitution_class: ReducedMotionSubstitutionClass::CollapseToInstant,
        duration_token: None,
        easing_token: None,
    },
    power_saver: MotionFallback {
        posture: AccessibilityPostureClass::MotionPowerSaver,
        substitution_class: ReducedMotionSubstitutionClass::CollapseToInstant,
        duration_token: None,
        easing_token: None,
    },
    critical_hot_path: MotionFallback {
        posture: AccessibilityPostureClass::MotionCriticalHotPath,
        substitution_class: ReducedMotionSubstitutionClass::SuppressEntirely,
        duration_token: None,
        easing_token: None,
    },
};

/// Canonical overlay-dialog exit preset, aligned with the motion case fixtures.
pub const OVERLAY_DIALOG_EXIT: MotionPresetDefinition = MotionPresetDefinition {
    preset_id: "motion_preset:overlay.dialog.exit",
    default_duration_token: "motion.dialog",
    default_easing_token: "ease.exit",
    reduced_motion: MotionFallback {
        posture: AccessibilityPostureClass::MotionReduced,
        substitution_class: ReducedMotionSubstitutionClass::CrossfadeOnly,
        duration_token: Some("motion.fast"),
        easing_token: Some("ease.standard"),
    },
    low_motion: MotionFallback {
        posture: AccessibilityPostureClass::MotionLowMotion,
        substitution_class: ReducedMotionSubstitutionClass::CollapseToInstant,
        duration_token: None,
        easing_token: None,
    },
    power_saver: MotionFallback {
        posture: AccessibilityPostureClass::MotionPowerSaver,
        substitution_class: ReducedMotionSubstitutionClass::CollapseToInstant,
        duration_token: None,
        easing_token: None,
    },
    critical_hot_path: MotionFallback {
        posture: AccessibilityPostureClass::MotionCriticalHotPath,
        substitution_class: ReducedMotionSubstitutionClass::SuppressEntirely,
        duration_token: None,
        easing_token: None,
    },
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::{seeded_token_registry, ThemeClass};

    #[test]
    fn overlay_dialog_enter_preset_matches_fixtures() {
        let plan = OVERLAY_DIALOG_ENTER.plan_for(AccessibilityPostureClass::MotionStandard);
        assert_eq!(plan.duration_token, Some("motion.dialog"));
        assert_eq!(plan.easing_token, Some("ease.enter"));
        assert!(plan.substitution_class.is_none());

        let plan = OVERLAY_DIALOG_ENTER.plan_for(AccessibilityPostureClass::MotionReduced);
        assert_eq!(plan.duration_token, Some("motion.fast"));
        assert_eq!(plan.easing_token, Some("ease.standard"));
        assert_eq!(
            plan.substitution_class,
            Some(ReducedMotionSubstitutionClass::CrossfadeOnly)
        );

        let plan = OVERLAY_DIALOG_ENTER.plan_for(AccessibilityPostureClass::MotionLowMotion);
        assert!(plan.duration_token.is_none());
        assert!(plan.easing_token.is_none());
        assert_eq!(
            plan.substitution_class,
            Some(ReducedMotionSubstitutionClass::CollapseToInstant)
        );

        let plan = OVERLAY_DIALOG_ENTER.plan_for(AccessibilityPostureClass::MotionPowerSaver);
        assert!(plan.duration_token.is_none());
        assert!(plan.easing_token.is_none());
        assert_eq!(
            plan.substitution_class,
            Some(ReducedMotionSubstitutionClass::CollapseToInstant)
        );

        let plan = OVERLAY_DIALOG_ENTER.plan_for(AccessibilityPostureClass::MotionCriticalHotPath);
        assert!(plan.duration_token.is_none());
        assert!(plan.easing_token.is_none());
        assert_eq!(
            plan.substitution_class,
            Some(ReducedMotionSubstitutionClass::SuppressEntirely)
        );
    }

    #[test]
    fn overlay_dialog_exit_preset_matches_fixtures() {
        let plan = OVERLAY_DIALOG_EXIT.plan_for(AccessibilityPostureClass::MotionStandard);
        assert_eq!(plan.duration_token, Some("motion.dialog"));
        assert_eq!(plan.easing_token, Some("ease.exit"));
        assert!(plan.substitution_class.is_none());

        let plan = OVERLAY_DIALOG_EXIT.plan_for(AccessibilityPostureClass::MotionReduced);
        assert_eq!(plan.duration_token, Some("motion.fast"));
        assert_eq!(plan.easing_token, Some("ease.standard"));
        assert_eq!(
            plan.substitution_class,
            Some(ReducedMotionSubstitutionClass::CrossfadeOnly)
        );
    }

    #[test]
    fn overlay_dialog_plans_resolve_durations_via_registry() {
        let registry =
            seeded_token_registry(ThemeClass::DarkReference).expect("seed token registry");

        let enter_plan = OVERLAY_DIALOG_ENTER.plan_for(AccessibilityPostureClass::MotionStandard);
        let enter_ms = enter_plan.duration_ms(registry).expect("duration ms");
        assert!(
            enter_ms >= 100,
            "expected dialog enter duration to be non-trivial"
        );

        let reduced_plan = OVERLAY_DIALOG_ENTER.plan_for(AccessibilityPostureClass::MotionReduced);
        let reduced_ms = reduced_plan.duration_ms(registry).expect("duration ms");
        assert!(
            reduced_ms <= enter_ms,
            "reduced motion should not lengthen duration"
        );
    }
}
