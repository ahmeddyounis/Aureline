//! Density contracts and derived shell metrics.
//!
//! Density is a presentation choice that alters row/control heights and
//! spacing budgets while preserving command semantics, focus order, and
//! information architecture. First-party surfaces derive their density-aware
//! measurements from semantic geometry tokens (`size.*`, `space.*`) instead of
//! hard-coded pixel values.

use serde::{Deserialize, Serialize};

use crate::tokens::{TokenRegistry, TokenRegistryError};

/// Density class for the in-effect appearance session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DensityClass {
    Compact,
    Standard,
    Comfortable,
}

impl DensityClass {
    /// Returns the canonical density token identifier.
    pub const fn token(self) -> &'static str {
        match self {
            Self::Compact => "compact",
            Self::Standard => "standard",
            Self::Comfortable => "comfortable",
        }
    }

    /// Returns the next density class in canonical cycle order.
    pub const fn next(self) -> Self {
        match self {
            Self::Compact => Self::Standard,
            Self::Standard => Self::Comfortable,
            Self::Comfortable => Self::Compact,
        }
    }

    /// Returns the semantic geometry token name for the typical row height.
    pub const fn row_height_token(self) -> &'static str {
        match self {
            Self::Compact => "size.row.compact",
            Self::Standard => "size.row.standard",
            Self::Comfortable => "size.row.comfortable",
        }
    }

    /// Returns the semantic geometry token name for the typical control height.
    pub const fn control_height_token(self) -> &'static str {
        match self {
            Self::Compact => "size.control.compact",
            Self::Standard => "size.control.standard",
            Self::Comfortable => "size.control.comfortable",
        }
    }

    /// Returns the preferred `space.*` token name for panel padding.
    pub const fn panel_padding_token(self) -> &'static str {
        match self {
            Self::Compact => "space.3",
            Self::Standard => "space.4",
            Self::Comfortable => "space.5",
        }
    }

    /// Returns the preferred `space.*` token name for shell-zone insets.
    pub const fn zone_inset_token(self) -> &'static str {
        match self {
            Self::Compact => "space.2",
            Self::Standard => "space.3",
            Self::Comfortable => "space.4",
        }
    }

    /// Returns the preferred `space.*` token name for gutters between regions.
    pub const fn gutter_token(self) -> &'static str {
        match self {
            Self::Compact => "space.2",
            Self::Standard => "space.3",
            Self::Comfortable => "space.4",
        }
    }
}

/// Density-derived sizing and spacing measurements loaded from the token registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DensityProfile {
    density_class: DensityClass,
    row_height_px: u32,
    control_height_px: u32,
    tab_height_px: u32,
    panel_padding_px: u32,
    zone_inset_px: u32,
    gutter_px: u32,
}

impl DensityProfile {
    /// Loads the profile for the requested density class from the provided [`TokenRegistry`].
    pub fn load(
        registry: &TokenRegistry,
        density_class: DensityClass,
    ) -> Result<Self, TokenRegistryError> {
        let row_height_px = registry.require_size_px(density_class.row_height_token())?;
        let control_height_px = registry.require_size_px(density_class.control_height_token())?;
        let tab_height_px = registry.require_size_px("size.tab")?;
        let panel_padding_px = registry.require_space_px(density_class.panel_padding_token())?;
        let zone_inset_px = registry.require_space_px(density_class.zone_inset_token())?;
        let gutter_px = registry.require_space_px(density_class.gutter_token())?;

        Ok(Self {
            density_class,
            row_height_px,
            control_height_px,
            tab_height_px,
            panel_padding_px,
            zone_inset_px,
            gutter_px,
        })
    }

    /// Returns the profile density class.
    pub const fn density_class(&self) -> DensityClass {
        self.density_class
    }

    /// Returns the typical list/tree/table row height in logical pixels.
    pub const fn row_height_px(&self) -> u32 {
        self.row_height_px
    }

    /// Returns the typical control height in logical pixels.
    pub const fn control_height_px(&self) -> u32 {
        self.control_height_px
    }

    /// Returns the baseline tab height in logical pixels.
    pub const fn tab_height_px(&self) -> u32 {
        self.tab_height_px
    }

    /// Returns the preferred panel padding in logical pixels.
    pub const fn panel_padding_px(&self) -> u32 {
        self.panel_padding_px
    }

    /// Returns the preferred shell-zone inset in logical pixels.
    pub const fn zone_inset_px(&self) -> u32 {
        self.zone_inset_px
    }

    /// Returns the preferred gutter spacing in logical pixels.
    pub const fn gutter_px(&self) -> u32 {
        self.gutter_px
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tokens::ThemeClass;

    #[test]
    fn loads_profiles_for_all_density_classes() {
        let registry = TokenRegistry::load(ThemeClass::DarkReference).expect("load token registry");
        for density in [
            DensityClass::Compact,
            DensityClass::Standard,
            DensityClass::Comfortable,
        ] {
            let profile = DensityProfile::load(&registry, density).expect("load density profile");
            assert!(profile.row_height_px() > 0);
            assert!(profile.control_height_px() > 0);
            assert!(profile.panel_padding_px() > 0);
            assert_eq!(profile.density_class(), density);
        }
    }
}
