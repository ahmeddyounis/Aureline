//! Shared shaping vocabulary.
//!
//! The types in this module are part of the cross-crate contract between text
//! shaping and rendering.

/// Stage in the fallback chain applied to a shaped cluster.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FallbackStage {
    /// Caller-declared explicit family (stage 1).
    ExplicitFamily,
    /// Script-aware preference group (stage 2).
    ScriptPreferenceGroup,
    /// OS system-UI family for the active locale (stage 3).
    SystemUi,
    /// Last-resort bundled subset (stage 4).
    BundledSubset,
    /// Terminal `.notdef` — must be zero on a supported host.
    Missing,
}

impl FallbackStage {
    /// Returns the stable string label for this stage.
    pub const fn name(self) -> &'static str {
        match self {
            Self::ExplicitFamily => "explicit_family",
            Self::ScriptPreferenceGroup => "script_preference_group",
            Self::SystemUi => "system_ui",
            Self::BundledSubset => "bundled_subset",
            Self::Missing => "missing",
        }
    }

    /// Returns the deterministic stage number in ADR order (1-indexed).
    pub const fn stage_number(self) -> u8 {
        match self {
            Self::ExplicitFamily => 1,
            Self::ScriptPreferenceGroup => 2,
            Self::SystemUi => 3,
            Self::BundledSubset => 4,
            Self::Missing => 5,
        }
    }

    /// Returns true when this stage should fire the fallback hook.
    pub const fn fires_fallback_hook(self) -> bool {
        self.stage_number() >= 2
    }
}

/// Shaper policy selection (ADR: default `rust_native`; `platform_native` opt-in).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaperPolicy {
    /// Pure Rust HarfBuzz-class shaper.
    RustNative,
    /// Platform-native shaper adapter (CoreText / DirectWrite / Pango).
    PlatformNative,
}

impl ShaperPolicy {
    /// Returns the stable string label for this policy.
    pub const fn name(self) -> &'static str {
        match self {
            Self::RustNative => "rust_native",
            Self::PlatformNative => "platform_native",
        }
    }
}

impl Default for ShaperPolicy {
    fn default() -> Self {
        Self::RustNative
    }
}

/// Typed feature flags on a shaping run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FeatureSet {
    /// Enables common ligatures (for example `liga` and `clig`).
    pub ligatures: bool,
    /// Enables an OpenType stylistic set (`ss01`..`ss20`) when non-zero.
    pub stylistic_set: u8,
}

impl FeatureSet {
    /// Default shaping features for code-like roles (ligatures off).
    pub const fn plain() -> Self {
        Self {
            ligatures: false,
            stylistic_set: 0,
        }
    }

    /// Default shaping features for UI copy (ligatures on).
    pub const fn ui_default() -> Self {
        Self {
            ligatures: true,
            stylistic_set: 0,
        }
    }
}

/// Logical text direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextDirection {
    /// Left-to-right.
    Ltr,
    /// Right-to-left.
    Rtl,
}

impl TextDirection {
    /// Returns the stable string label for this direction.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Ltr => "ltr",
            Self::Rtl => "rtl",
        }
    }

    /// Returns true when the direction is right-to-left.
    pub const fn is_rtl(self) -> bool {
        matches!(self, Self::Rtl)
    }
}

