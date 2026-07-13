//! Frozen M5 platform-convention, shortcut-notation, file-path-reveal, theme/contrast live-change,
//! credential-store wording, and input-method platform-fit matrix.
//!
//! This module locks Aureline's concrete desktop-fit conventions into one export-safe packet. Every
//! claimed M5 desktop surface that still describes its own platform-native shortcut notation, window and
//! menu behavior, file / path / reveal / save terminology, live theme / contrast / accent / text-scale
//! response, credential-store wording, or IME / dead-key / AltGr / dictation / emoji / layout-switch
//! behavior — across macOS, Windows, and Linux — is named once here and constrained by the same shared
//! platform-fit-role taxonomy (shortcut, window_menu, path_terminology, appearance, credential_wording,
//! input_fidelity, command_stability), the same command-ids-stay-stable-while-labels-adapt rule, the same
//! high-frequency-actions-are-never-hidden-in-os-chrome-alone rule, the same
//! terminology-matches-the-host-platform rule, the same appearance-applies-live-or-explains-fallback rule,
//! the same credential-store-wording-stays-truthful-and-non-leaky rule, and the same
//! input-methods-never-corrupt-text-or-trust-fidelity rule regardless of the surface that renders it.
//!
//! The matrix does not revisit protocol-handler ownership, rollout-ring packaging, or residual-dependency
//! cards — it is the shared reusable platform-fit contract those desktop lanes consume, and it binds back
//! to the already-landed native-desktop-integration packet instead of leaving the platform-fit truth split
//! across scattered platform-specific notes and hand-copied screenshots. The controlled vocabularies are
//! frozen in one self-describing [`M5PlatformFitVocabularySet`] rather than minted per surface. The single
//! controlled platform-fit-role vocabulary consumers bind to — shortcut, window_menu, path_terminology,
//! appearance, credential_wording, input_fidelity, and command_stability — keeps command IDs stable while
//! platform labels and shortcut notation adapt, keeps primary actions out of OS-chrome-only hiding, keeps
//! file / path / reveal / save terminology matching the host, keeps theme / contrast / accent / text-scale
//! changes applying live or explaining their fallback, keeps credential-store wording truthful and
//! non-leaky, and keeps IME, dead keys, AltGr, dictation, emoji, and layout switching from corrupting text
//! fidelity or trust semantics. Raw secret values and private endpoints stay outside the export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_platform_fit_matrix, seeded_m5_platform_fit_matrix_input_method_preview_narrowed,
    seeded_m5_platform_fit_matrix_theme_contrast_live_change_beta_narrowed,
    M5_PLATFORM_FIT_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5PlatformFitMatrixPacket`].
pub const M5_PLATFORM_FIT_MATRIX_RECORD_KIND: &str =
    "freeze_m5_platform_convention_shortcut_notation_file_path_reveal_theme_contrast_live_change_credential_store_wording_and_input_method_matrix";

/// Schema version for M5 platform-fit matrix records.
pub const M5_PLATFORM_FIT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined platform-fit matrix schema.
pub const M5_PLATFORM_FIT_MATRIX_SCHEMA_REF: &str =
    "schemas/platform/m5-platform-fit-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_PLATFORM_FIT_MATRIX_DOC_REF: &str = "docs/platform/m5_platform_fit_contract.md";

/// Repo-relative path of the canonical platform-convention / shortcut-notation domain schema.
pub const M5_SHORTCUT_NOTATION_SCHEMA_REF: &str =
    "schemas/platform/m5-shortcut-notation.schema.json";

/// Repo-relative path of the canonical file-path-reveal / theme-contrast / credential-wording domain
/// schema (the terminology, appearance-response, and wording truth domain).
pub const M5_FILE_PATH_AND_REVEAL_SCHEMA_REF: &str =
    "schemas/platform/m5-file-path-and-reveal.schema.json";

/// Repo-relative path of the canonical input-method-behavior domain schema.
pub const M5_INPUT_METHOD_BEHAVIOR_SCHEMA_REF: &str =
    "schemas/platform/m5-input-method-behavior.schema.json";

/// Repo-relative path of the already-landed native-desktop matrix schema the platform-fit matrix binds
/// back to.
pub const M5_NATIVE_DESKTOP_MATRIX_SCHEMA_REF: &str =
    "schemas/platform/m5-native-desktop-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_PLATFORM_FIT_FIXTURE_DIR: &str = "fixtures/platform/m5-desktop-fit";

/// Repo-relative path of the checked support-export artifact.
pub const M5_PLATFORM_FIT_ARTIFACT_REF: &str =
    "artifacts/release/m5-platform-fit-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_PLATFORM_FIT_CSV_REF: &str = "artifacts/release/m5-platform-fit-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_PLATFORM_FIT_REPORT_REF: &str = "artifacts/platform/m5-platform-fit-matrix.md";

/// One of the six governed platform-fit families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PlatformFitFamily {
    /// Platform conventions: window controls, menu-bar behavior, title-bar convention, system chrome.
    PlatformConvention,
    /// Shortcut notation: platform-native modifier glyphs, accelerator labels, and chord sequences with
    /// stable command IDs.
    ShortcutNotation,
    /// File / path / reveal / save terminology matched to the host platform.
    FilePathReveal,
    /// Live theme / contrast / accent / text-scale response, or an explained fallback.
    ThemeContrastLiveChange,
    /// Credential-store wording that stays truthful and non-leaky.
    CredentialStoreWording,
    /// Input-method behavior: IME, dead keys, AltGr, dictation, emoji, and layout switching preserving
    /// text and trust fidelity.
    InputMethod,
}

impl M5PlatformFitFamily {
    /// Every governed platform-fit family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PlatformConvention,
        Self::ShortcutNotation,
        Self::FilePathReveal,
        Self::ThemeContrastLiveChange,
        Self::CredentialStoreWording,
        Self::InputMethod,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlatformConvention => "platform_convention",
            Self::ShortcutNotation => "shortcut_notation",
            Self::FilePathReveal => "file_path_reveal",
            Self::ThemeContrastLiveChange => "theme_contrast_live_change",
            Self::CredentialStoreWording => "credential_store_wording",
            Self::InputMethod => "input_method",
        }
    }

    /// The canonical per-domain schema ref a downstream surface points at instead of restating this
    /// family's platform-convention, shortcut, terminology, appearance, wording, or input meaning by hand.
    pub const fn canonical_domain_schema_ref(self) -> &'static str {
        match self {
            Self::PlatformConvention | Self::ShortcutNotation => M5_SHORTCUT_NOTATION_SCHEMA_REF,
            Self::FilePathReveal | Self::ThemeContrastLiveChange | Self::CredentialStoreWording => {
                M5_FILE_PATH_AND_REVEAL_SCHEMA_REF
            }
            Self::InputMethod => M5_INPUT_METHOD_BEHAVIOR_SCHEMA_REF,
        }
    }

    /// `true` when this family must name a controlled platform-convention role.
    pub const fn declares_platform_convention_roles(self) -> bool {
        matches!(self, Self::PlatformConvention)
    }

    /// `true` when this family must name a controlled shortcut-notation role.
    pub const fn declares_shortcut_notation_roles(self) -> bool {
        matches!(self, Self::ShortcutNotation)
    }

    /// `true` when this family must name a controlled file-path-reveal role.
    pub const fn declares_file_path_reveal_roles(self) -> bool {
        matches!(self, Self::FilePathReveal)
    }

    /// `true` when this family must name a controlled theme-contrast-live-change role.
    pub const fn declares_theme_contrast_live_change_roles(self) -> bool {
        matches!(self, Self::ThemeContrastLiveChange)
    }

    /// `true` when this family must name a controlled credential-store-wording role.
    pub const fn declares_credential_store_wording_roles(self) -> bool {
        matches!(self, Self::CredentialStoreWording)
    }

    /// `true` when this family must name a controlled input-method role.
    pub const fn declares_input_method_roles(self) -> bool {
        matches!(self, Self::InputMethod)
    }
}

/// The single controlled platform-fit-role vocabulary every macOS, Windows, Linux, docs, or support
/// consumer binds to. These are the exact acceptance-criteria tokens that keep `shortcut`, `window_menu`,
/// `path_terminology`, `appearance`, `credential_wording`, `input_fidelity`, and `command_stability`
/// meaning the same thing everywhere the platform-fit grammar ships. No surface invents a parallel word
/// for any of these roles, and the adaptation-sensitive roles may never let a platform-specific label
/// change command meaning, permission meaning, text fidelity, or trust semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PlatformFitRole {
    /// Platform-native shortcut-notation role (modifier glyphs, accelerator labels, chords).
    Shortcut,
    /// Window / menu behavior role (title bar, menu bar, system chrome).
    WindowMenu,
    /// File / path / reveal / save terminology role.
    PathTerminology,
    /// Live theme / contrast / accent / text-scale response role.
    Appearance,
    /// Credential-store wording role.
    CredentialWording,
    /// IME / dead-key / AltGr / dictation / emoji / layout-switch fidelity role.
    InputFidelity,
    /// Command-identity-stays-stable-while-labels-adapt role.
    CommandStability,
}

impl M5PlatformFitRole {
    /// Every platform-fit role token, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Shortcut,
        Self::WindowMenu,
        Self::PathTerminology,
        Self::Appearance,
        Self::CredentialWording,
        Self::InputFidelity,
        Self::CommandStability,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shortcut => "shortcut",
            Self::WindowMenu => "window_menu",
            Self::PathTerminology => "path_terminology",
            Self::Appearance => "appearance",
            Self::CredentialWording => "credential_wording",
            Self::InputFidelity => "input_fidelity",
            Self::CommandStability => "command_stability",
        }
    }

    /// Whether this role carries adaptation behavior whose platform-specific presentation must never change
    /// command meaning, permission meaning, focus order, text fidelity, or trust semantics as it adapts
    /// (`shortcut`, `window_menu`, `input_fidelity`, `command_stability`). The terminology-matching,
    /// appearance-response, and wording-truth roles (`path_terminology`, `appearance`, `credential_wording`)
    /// are truthful mappings rather than command-carrying adaptation and so do not carry this requirement.
    pub const fn must_preserve_command_identity_under_platform_adaptation(self) -> bool {
        matches!(
            self,
            Self::Shortcut | Self::WindowMenu | Self::InputFidelity | Self::CommandStability
        )
    }
}

/// Controlled platform-convention role — how window and menu behavior is named, so window controls, the
/// menu bar, the title bar, and system chrome integration follow one platform registry rather than an
/// invented private convention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PlatformConventionRole {
    /// Window-control placement (traffic lights vs caption buttons).
    WindowControlPlacement,
    /// Menu-bar behavior (global menu bar vs in-window menu).
    MenuBarBehavior,
    /// Title-bar convention.
    TitleBarConvention,
    /// System-chrome integration.
    SystemChromeIntegration,
    /// A convention bound to the single platform registry.
    BoundToPlatformRegistry,
    /// An invented private convention, which is disallowed.
    InventedPrivateConventionDisallowed,
}

impl M5PlatformConventionRole {
    /// Every platform-convention role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WindowControlPlacement,
        Self::MenuBarBehavior,
        Self::TitleBarConvention,
        Self::SystemChromeIntegration,
        Self::BoundToPlatformRegistry,
        Self::InventedPrivateConventionDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WindowControlPlacement => "window_control_placement",
            Self::MenuBarBehavior => "menu_bar_behavior",
            Self::TitleBarConvention => "title_bar_convention",
            Self::SystemChromeIntegration => "system_chrome_integration",
            Self::BoundToPlatformRegistry => "bound_to_platform_registry",
            Self::InventedPrivateConventionDisallowed => "invented_private_convention_disallowed",
        }
    }
}

/// Controlled shortcut-notation role — how platform-native shortcuts are named, so modifier glyphs,
/// accelerator labels, and chord sequences adapt per platform while the command ID stays stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShortcutNotationRole {
    /// Modifier-glyph notation (⌘/⌥/⌃/⇧ vs Ctrl/Alt/Shift).
    ModifierGlyphNotation,
    /// Accelerator label shown in menus and tooltips.
    AcceleratorLabel,
    /// Chord / sequence notation.
    ChordSequence,
    /// Notation that adapts per platform.
    PlatformAdaptiveNotation,
    /// Notation bound to a stable command ID.
    StableCommandIdBinding,
    /// Notation hard-coded for one platform, which is disallowed.
    HardcodedPlatformNotationDisallowed,
}

impl M5ShortcutNotationRole {
    /// Every shortcut-notation role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ModifierGlyphNotation,
        Self::AcceleratorLabel,
        Self::ChordSequence,
        Self::PlatformAdaptiveNotation,
        Self::StableCommandIdBinding,
        Self::HardcodedPlatformNotationDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ModifierGlyphNotation => "modifier_glyph_notation",
            Self::AcceleratorLabel => "accelerator_label",
            Self::ChordSequence => "chord_sequence",
            Self::PlatformAdaptiveNotation => "platform_adaptive_notation",
            Self::StableCommandIdBinding => "stable_command_id_binding",
            Self::HardcodedPlatformNotationDisallowed => "hardcoded_platform_notation_disallowed",
        }
    }
}

/// Controlled file-path-reveal role — how file, path, reveal, and save terminology is named, so the reveal
/// verb, save-dialog wording, and path presentation match the host platform rather than a mislabeled verb.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FilePathRevealRole {
    /// File / path terminology.
    FilePathTerminology,
    /// Reveal verb (Reveal in Finder / Show in Explorer / Open Containing Folder).
    RevealVerb,
    /// Save-dialog terminology.
    SaveDialogTerminology,
    /// Host-matched separator and case presentation.
    HostMatchedSeparatorAndCase,
    /// Terminology bound to the path registry.
    BoundToPathRegistry,
    /// A mislabeled path or reveal verb, which is disallowed.
    MislabeledPathVerbDisallowed,
}

impl M5FilePathRevealRole {
    /// Every file-path-reveal role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FilePathTerminology,
        Self::RevealVerb,
        Self::SaveDialogTerminology,
        Self::HostMatchedSeparatorAndCase,
        Self::BoundToPathRegistry,
        Self::MislabeledPathVerbDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FilePathTerminology => "file_path_terminology",
            Self::RevealVerb => "reveal_verb",
            Self::SaveDialogTerminology => "save_dialog_terminology",
            Self::HostMatchedSeparatorAndCase => "host_matched_separator_and_case",
            Self::BoundToPathRegistry => "bound_to_path_registry",
            Self::MislabeledPathVerbDisallowed => "mislabeled_path_verb_disallowed",
        }
    }
}

/// Controlled theme-contrast-live-change role — how live appearance response is named, so theme, contrast,
/// accent, and text-scale changes apply live or explain their fallback rather than silently drifting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ThemeContrastLiveChangeRole {
    /// Live theme response.
    LiveThemeResponse,
    /// Live contrast response.
    LiveContrastResponse,
    /// Live accent and text-scale response.
    AccentAndTextScaleResponse,
    /// An explained fallback when a change cannot apply live.
    ExplainedFallbackWhenNotLive,
    /// Response bound to the appearance registry.
    BoundToAppearanceRegistry,
    /// A silent theme or contrast drift, which is disallowed.
    SilentThemeDriftDisallowed,
}

impl M5ThemeContrastLiveChangeRole {
    /// Every theme-contrast-live-change role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LiveThemeResponse,
        Self::LiveContrastResponse,
        Self::AccentAndTextScaleResponse,
        Self::ExplainedFallbackWhenNotLive,
        Self::BoundToAppearanceRegistry,
        Self::SilentThemeDriftDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveThemeResponse => "live_theme_response",
            Self::LiveContrastResponse => "live_contrast_response",
            Self::AccentAndTextScaleResponse => "accent_and_text_scale_response",
            Self::ExplainedFallbackWhenNotLive => "explained_fallback_when_not_live",
            Self::BoundToAppearanceRegistry => "bound_to_appearance_registry",
            Self::SilentThemeDriftDisallowed => "silent_theme_drift_disallowed",
        }
    }
}

/// Controlled credential-store-wording role — how credential-store messaging is named, so the host store
/// name, the storage claim, and any fallback disclosure stay truthful and non-leaky rather than hiding a
/// plaintext fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialStoreWordingRole {
    /// Host credential-store name (Keychain / Credential Manager / Secret Service).
    HostCredentialStoreName,
    /// A truthful storage claim.
    TruthfulStorageClaim,
    /// Non-leaky wording that never surfaces a secret value.
    NonLeakyWording,
    /// A disclosed fallback when the host store is unavailable.
    FallbackDisclosure,
    /// Wording bound to the credential registry.
    BoundToCredentialRegistry,
    /// A hidden plaintext fallback, which is disallowed.
    PlaintextFallbackHiddenDisallowed,
}

impl M5CredentialStoreWordingRole {
    /// Every credential-store-wording role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HostCredentialStoreName,
        Self::TruthfulStorageClaim,
        Self::NonLeakyWording,
        Self::FallbackDisclosure,
        Self::BoundToCredentialRegistry,
        Self::PlaintextFallbackHiddenDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostCredentialStoreName => "host_credential_store_name",
            Self::TruthfulStorageClaim => "truthful_storage_claim",
            Self::NonLeakyWording => "non_leaky_wording",
            Self::FallbackDisclosure => "fallback_disclosure",
            Self::BoundToCredentialRegistry => "bound_to_credential_registry",
            Self::PlaintextFallbackHiddenDisallowed => "plaintext_fallback_hidden_disallowed",
        }
    }
}

/// Controlled input-method role — how input-method behavior is named, so IME composition, dead keys and
/// AltGr, dictation and emoji, and layout switching preserve text and trust fidelity rather than corrupting
/// them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InputMethodRole {
    /// IME composition fidelity.
    ImeCompositionFidelity,
    /// Dead-key and AltGr fidelity.
    DeadKeyAndAltGrFidelity,
    /// Dictation and emoji fidelity.
    DictationAndEmojiFidelity,
    /// Layout-switch fidelity.
    LayoutSwitchFidelity,
    /// Behavior bound to the input registry.
    BoundToInputRegistry,
    /// A corruption of text or trust fidelity, which is disallowed.
    TextOrTrustCorruptionDisallowed,
}

impl M5InputMethodRole {
    /// Every input-method role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ImeCompositionFidelity,
        Self::DeadKeyAndAltGrFidelity,
        Self::DictationAndEmojiFidelity,
        Self::LayoutSwitchFidelity,
        Self::BoundToInputRegistry,
        Self::TextOrTrustCorruptionDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ImeCompositionFidelity => "ime_composition_fidelity",
            Self::DeadKeyAndAltGrFidelity => "dead_key_and_alt_gr_fidelity",
            Self::DictationAndEmojiFidelity => "dictation_and_emoji_fidelity",
            Self::LayoutSwitchFidelity => "layout_switch_fidelity",
            Self::BoundToInputRegistry => "bound_to_input_registry",
            Self::TextOrTrustCorruptionDisallowed => "text_or_trust_corruption_disallowed",
        }
    }
}

/// Claimed M5 surface family that renders / consumes a platform-fit family. No family may invent a parallel
/// surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PlatformFitSurfaceFamily {
    /// The macOS desktop surface.
    Macos,
    /// The Windows desktop surface.
    Windows,
    /// The Linux desktop surface.
    Linux,
    /// The desktop shell surface shared across platforms.
    DesktopShell,
    /// The docs / help surface.
    DocsHelp,
    /// The support export.
    SupportExport,
}

impl M5PlatformFitSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Macos,
        Self::Windows,
        Self::Linux,
        Self::DesktopShell,
        Self::DocsHelp,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Macos => "macos",
            Self::Windows => "windows",
            Self::Linux => "linux",
            Self::DesktopShell => "desktop_shell",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
        }
    }
}

/// Deployment line a family must survive with the same truth, so a family's platform-convention, shortcut,
/// terminology, appearance, wording, or input meaning never silently narrows or widens between deployment
/// shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PlatformFitDeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5PlatformFitDeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Subsystem that consumes a family's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PlatformFitConsumerSurface {
    /// The shell UI.
    ShellUi,
    /// The settings UI.
    SettingsUi,
    /// The auth UI.
    AuthUi,
    /// The input UI / handling.
    InputUi,
    /// The docs / help surface.
    DocsHelp,
    /// The onboarding surface.
    Onboarding,
    /// The CLI / export path.
    CliExport,
    /// The support export.
    SupportExport,
    /// The general product UI.
    ProductUi,
}

impl M5PlatformFitConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ShellUi,
        Self::SettingsUi,
        Self::AuthUi,
        Self::InputUi,
        Self::DocsHelp,
        Self::Onboarding,
        Self::CliExport,
        Self::SupportExport,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellUi => "shell_ui",
            Self::SettingsUi => "settings_ui",
            Self::AuthUi => "auth_ui",
            Self::InputUi => "input_ui",
            Self::DocsHelp => "docs_help",
            Self::Onboarding => "onboarding",
            Self::CliExport => "cli_export",
            Self::SupportExport => "support_export",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every family must offer so no platform-fit meaning disappears under
/// zoom, high contrast, keyboard-only use, or export. Records the keyboard, screen-reader, high-zoom,
/// high-contrast, CLI/export, and support-packet requirements up front.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PlatformFitAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader (via a non-visual cue / label).
    ScreenReaderAnnounced,
    /// Reflows legibly at high zoom.
    HighZoomReflow,
    /// Preserves truth under high-contrast and forced-colors modes.
    HighContrastSafe,
    /// Reachable and inspectable through the CLI / export path.
    CliExportable,
    /// Present in the support / export packet, never renderer-only.
    SupportPacketPresent,
}

impl M5PlatformFitAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::HighZoomReflow,
        Self::HighContrastSafe,
        Self::CliExportable,
        Self::SupportPacketPresent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::CliExportable => "cli_exportable",
            Self::SupportPacketPresent => "support_packet_present",
        }
    }
}

/// Reason a platform-fit family has degraded below its qualified state. Required on every row so a stale,
/// unresolved, or narrowed fallback is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PlatformFitDegradedReason {
    /// Proof has gone stale.
    ProofStale,
    /// The shortcut-notation registry source is unavailable.
    ShortcutRegistrySourceUnavailable,
    /// The file / path terminology source is unavailable.
    PathTerminologySourceUnavailable,
    /// Live theme / contrast response is unverified.
    ThemeResponseUnverified,
    /// The credential-store wording is unverified.
    CredentialWordingUnverified,
    /// Input-method coverage is unavailable.
    InputMethodCoverageUnavailable,
}

impl M5PlatformFitDegradedReason {
    /// Every degraded reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProofStale,
        Self::ShortcutRegistrySourceUnavailable,
        Self::PathTerminologySourceUnavailable,
        Self::ThemeResponseUnverified,
        Self::CredentialWordingUnverified,
        Self::InputMethodCoverageUnavailable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::ShortcutRegistrySourceUnavailable => "shortcut_registry_source_unavailable",
            Self::PathTerminologySourceUnavailable => "path_terminology_source_unavailable",
            Self::ThemeResponseUnverified => "theme_response_unverified",
            Self::CredentialWordingUnverified => "credential_wording_unverified",
            Self::InputMethodCoverageUnavailable => "input_method_coverage_unavailable",
        }
    }
}

/// Mandatory label a claimed platform-fit family must be able to show. The first three are hard
/// requirements on every family; the remaining three close the acceptance-criteria ambiguity about the
/// host platform, the shortcut notation, and the path verb.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PlatformFitRequiredLabel {
    /// The family's stable identity.
    Identity,
    /// The family's platform-fit role.
    SemanticRole,
    /// The canonical registry reference the family points at.
    RegistryReference,
    /// The host platform (macOS / Windows / Linux) the family adapts to.
    HostPlatform,
    /// The shortcut notation the family presents.
    ShortcutNotation,
    /// The path / reveal / save verb the family presents.
    PathVerb,
}

impl M5PlatformFitRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::HostPlatform,
        Self::ShortcutNotation,
        Self::PathVerb,
    ];

    /// The three labels every claimed family must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::SemanticRole, Self::RegistryReference];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::SemanticRole => "semantic_role",
            Self::RegistryReference => "registry_reference",
            Self::HostPlatform => "host_platform",
            Self::ShortcutNotation => "shortcut_notation",
            Self::PathVerb => "path_verb",
        }
    }
}

/// Qualification class for an M5 platform-fit row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PlatformFitQualificationClass {
    /// Family qualifies for the Stable claim.
    Stable,
    /// Family is narrowed to Beta.
    Beta,
    /// Family is narrowed to Preview.
    Preview,
    /// Family is experimental and not claimed.
    Experimental,
    /// Family is unavailable on this build.
    Unavailable,
    /// Family is held pending upstream resolution.
    Held,
}

impl M5PlatformFitQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the family may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a platform-fit family below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PlatformFitDowngradeTrigger {
    /// Platform-specific wording changed command or permission meaning.
    PlatformWordingChangedCommandOrPermissionMeaning,
    /// A primary action was hidden only in OS chrome (menus / title bars).
    PrimaryActionHiddenOnlyInOsChrome,
    /// Secret storage silently fell back to plaintext.
    SecretStorageFellBackToPlaintextSilently,
    /// An input method corrupted text or trust fidelity.
    InputMethodCorruptedTextOrTrust,
    /// A screenshot or docs page mislabeled a shortcut or path / reveal verb.
    ScreenshotOrDocsMislabeledShortcutOrPathVerb,
    /// A theme or contrast change did not apply live and did not explain its fallback.
    ThemeOrContrastChangeDidNotApplyLiveOrExplainFallback,
    /// Shortcut notation drifted by platform instead of adapting from one registry.
    ShortcutNotationDriftedByPlatform,
    /// A family left its host platform unstated.
    HostPlatformUnstated,
    /// A family left its shortcut notation unstated.
    ShortcutNotationUnstated,
    /// A family left its path verb unstated.
    PathVerbUnstated,
    /// A family left its canonical registry reference unstated.
    RegistryReferenceUnstated,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5PlatformFitDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::PlatformWordingChangedCommandOrPermissionMeaning,
        Self::PrimaryActionHiddenOnlyInOsChrome,
        Self::SecretStorageFellBackToPlaintextSilently,
        Self::InputMethodCorruptedTextOrTrust,
        Self::ScreenshotOrDocsMislabeledShortcutOrPathVerb,
        Self::ThemeOrContrastChangeDidNotApplyLiveOrExplainFallback,
        Self::ShortcutNotationDriftedByPlatform,
        Self::HostPlatformUnstated,
        Self::ShortcutNotationUnstated,
        Self::PathVerbUnstated,
        Self::RegistryReferenceUnstated,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlatformWordingChangedCommandOrPermissionMeaning => {
                "platform_wording_changed_command_or_permission_meaning"
            }
            Self::PrimaryActionHiddenOnlyInOsChrome => "primary_action_hidden_only_in_os_chrome",
            Self::SecretStorageFellBackToPlaintextSilently => {
                "secret_storage_fell_back_to_plaintext_silently"
            }
            Self::InputMethodCorruptedTextOrTrust => "input_method_corrupted_text_or_trust",
            Self::ScreenshotOrDocsMislabeledShortcutOrPathVerb => {
                "screenshot_or_docs_mislabeled_shortcut_or_path_verb"
            }
            Self::ThemeOrContrastChangeDidNotApplyLiveOrExplainFallback => {
                "theme_or_contrast_change_did_not_apply_live_or_explain_fallback"
            }
            Self::ShortcutNotationDriftedByPlatform => "shortcut_notation_drifted_by_platform",
            Self::HostPlatformUnstated => "host_platform_unstated",
            Self::ShortcutNotationUnstated => "shortcut_notation_unstated",
            Self::PathVerbUnstated => "path_verb_unstated",
            Self::RegistryReferenceUnstated => "registry_reference_unstated",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed platform-fit family bound to the surface-specific truth it must
/// project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PlatformFitRow {
    /// Governed platform-fit family.
    pub platform_fit_family: M5PlatformFitFamily,
    /// Qualification class earned by this family.
    pub qualification: M5PlatformFitQualificationClass,
    /// Owner role accountable for keeping this family governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this family.
    pub surface_families: Vec<M5PlatformFitSurfaceFamily>,
    /// Deployment lines this family keeps the same truth across.
    pub deployment_lines: Vec<M5PlatformFitDeploymentLine>,
    /// Mandatory labels this family must be able to show (must include the three
    /// [`M5PlatformFitRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5PlatformFitRequiredLabel>,
    /// Platform-fit roles this family can carry (the frozen AC vocabulary; required on every family).
    pub semantic_roles: Vec<M5PlatformFitRole>,
    /// Platform-convention roles this family names (platform-convention family only).
    pub platform_convention_roles: Vec<M5PlatformConventionRole>,
    /// Shortcut-notation roles this family names (shortcut-notation family only).
    pub shortcut_notation_roles: Vec<M5ShortcutNotationRole>,
    /// File-path-reveal roles this family names (file-path-reveal family only).
    pub file_path_reveal_roles: Vec<M5FilePathRevealRole>,
    /// Theme-contrast-live-change roles this family names (theme-contrast family only).
    pub theme_contrast_live_change_roles: Vec<M5ThemeContrastLiveChangeRole>,
    /// Credential-store-wording roles this family names (credential-store-wording family only).
    pub credential_store_wording_roles: Vec<M5CredentialStoreWordingRole>,
    /// Input-method roles this family names (input-method family only).
    pub input_method_roles: Vec<M5InputMethodRole>,
    /// Degraded reasons this family can name (required on every family).
    pub degraded_reasons: Vec<M5PlatformFitDegradedReason>,
    /// Non-visual accessibility routes this family offers.
    pub accessibility_routes: Vec<M5PlatformFitAccessibilityRoute>,
    /// Subsystems that consume this family's projection.
    pub consumer_surfaces: Vec<M5PlatformFitConsumerSurface>,
    /// Downgrade triggers that apply to this family.
    pub downgrade_triggers: Vec<M5PlatformFitDowngradeTrigger>,
    /// Proof packet refs that keep this family current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this family (must include its own canonical domain schema so
    /// downstream surfaces have one target to point at).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this family never lets platform-specific wording change command or permission
    /// meaning. MUST be `false`.
    pub platform_wording_changes_command_or_permission_meaning: bool,
    /// Hard invariant: this family never hides a primary action only in OS menus / title bars. MUST be
    /// `false`.
    pub hides_primary_action_only_in_os_chrome: bool,
    /// Hard invariant: this family never silently falls back to plaintext secret storage. MUST be `false`.
    pub falls_back_to_plaintext_secret_storage_silently: bool,
    /// Hard invariant: this family never lets an input method corrupt text or trust fidelity. MUST be
    /// `false`.
    pub input_method_corrupts_text_or_trust_fidelity: bool,
    /// Hard invariant: this family never produces a screenshot or docs page that mislabels a shortcut or
    /// path / reveal verb. MUST be `false`.
    pub screenshot_or_docs_mislabels_shortcut_or_path_verb: bool,
}

impl M5PlatformFitRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5PlatformFitRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5PlatformFitRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.platform_wording_changes_command_or_permission_meaning
            && !self.hides_primary_action_only_in_os_chrome
            && !self.falls_back_to_plaintext_secret_storage_silently
            && !self.input_method_corrupts_text_or_trust_fidelity
            && !self.screenshot_or_docs_mislabels_shortcut_or_path_verb
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PlatformFitVocabularySet {
    /// Platform-fit-family tokens.
    pub platform_fit_families: Vec<String>,
    /// Platform-fit-role tokens.
    pub semantic_roles: Vec<String>,
    /// Platform-convention-role tokens.
    pub platform_convention_roles: Vec<String>,
    /// Shortcut-notation-role tokens.
    pub shortcut_notation_roles: Vec<String>,
    /// File-path-reveal-role tokens.
    pub file_path_reveal_roles: Vec<String>,
    /// Theme-contrast-live-change-role tokens.
    pub theme_contrast_live_change_roles: Vec<String>,
    /// Credential-store-wording-role tokens.
    pub credential_store_wording_roles: Vec<String>,
    /// Input-method-role tokens.
    pub input_method_roles: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Degraded-reason tokens.
    pub degraded_reasons: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
    /// Downgrade-trigger tokens.
    pub downgrade_triggers: Vec<String>,
}

impl M5PlatformFitVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            platform_fit_families: tokens(&M5PlatformFitFamily::ALL, |v| v.as_str()),
            semantic_roles: tokens(&M5PlatformFitRole::ALL, |v| v.as_str()),
            platform_convention_roles: tokens(&M5PlatformConventionRole::ALL, |v| v.as_str()),
            shortcut_notation_roles: tokens(&M5ShortcutNotationRole::ALL, |v| v.as_str()),
            file_path_reveal_roles: tokens(&M5FilePathRevealRole::ALL, |v| v.as_str()),
            theme_contrast_live_change_roles: tokens(&M5ThemeContrastLiveChangeRole::ALL, |v| {
                v.as_str()
            }),
            credential_store_wording_roles: tokens(&M5CredentialStoreWordingRole::ALL, |v| {
                v.as_str()
            }),
            input_method_roles: tokens(&M5InputMethodRole::ALL, |v| v.as_str()),
            surface_families: tokens(&M5PlatformFitSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5PlatformFitDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5PlatformFitConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5PlatformFitAccessibilityRoute::ALL, |v| v.as_str()),
            degraded_reasons: tokens(&M5PlatformFitDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5PlatformFitRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5PlatformFitDowngradeTrigger::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PlatformFitGovernanceReview {
    /// Command IDs stay stable while platform labels and shortcut notation adapt.
    pub command_ids_stable_while_labels_adapt: bool,
    /// High-frequency actions are never hidden in OS chrome alone.
    pub high_frequency_actions_never_hidden_in_os_chrome_alone: bool,
    /// File / path / reveal / save terminology matches the host platform.
    pub file_path_reveal_save_terminology_matches_host: bool,
    /// Theme, contrast, accent, and text-scale changes apply live or explain their fallback.
    pub theme_contrast_accent_text_scale_apply_live_or_explain_fallback: bool,
    /// Credential-store wording stays truthful and non-leaky.
    pub credential_store_wording_stays_truthful_and_non_leaky: bool,
    /// IME, dead keys, AltGr, dictation, emoji, and layout switching never corrupt text or trust fidelity.
    pub input_method_never_corrupts_text_or_trust_fidelity: bool,
    /// Shortcut notation adapts per platform from one registry.
    pub shortcut_notation_adapts_per_platform: bool,
    /// No primary action is hidden only in OS chrome.
    pub no_primary_action_hidden_only_in_os_chrome: bool,
    /// Secrets never fall back to plaintext storage silently.
    pub secrets_never_fall_back_to_plaintext_silently: bool,
    /// Every family keeps the same truth across every deployment line.
    pub every_family_declares_deployment_lines: bool,
    /// Every family declares a non-visual accessibility route.
    pub every_family_declares_accessibility_route: bool,
    /// Support / export reads a single canonical platform-fit source.
    pub support_export_reads_single_platform_fit_source: bool,
    /// Screenshots and docs bind to a single canonical platform-fit source.
    pub screenshots_and_docs_bind_to_single_platform_fit_source: bool,
    /// Later M5 rows cannot invent parallel platform vocabulary.
    pub later_rows_cannot_invent_parallel_platform_vocabulary: bool,
    /// Platform-fit survives zoom and high contrast.
    pub platform_fit_survives_zoom_and_high_contrast: bool,
    /// Claims narrow automatically when the registry is missing, stale, or not yet qualified.
    pub claims_narrow_automatically_when_registry_missing_or_stale: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PlatformFitConsumerProjection {
    /// Shell and settings consume the shared shortcut and menu grammar.
    pub shell_and_settings_consume_shared_shortcut_and_menu_grammar: bool,
    /// Auth and settings consume the shared credential wording.
    pub auth_and_settings_consume_shared_credential_wording: bool,
    /// Input surfaces consume the shared input-method behavior.
    pub input_surfaces_consume_shared_input_method_behavior: bool,
    /// Docs, help, and screenshots read a single platform-fit source.
    pub docs_help_and_screenshots_read_single_platform_fit_source: bool,
    /// Appearance consumers bind to the shared theme / contrast response.
    pub appearance_consumers_bind_to_shared_theme_response: bool,
    /// Support / export reads a single canonical platform-fit source.
    pub support_export_reads_single_platform_fit_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PlatformFitProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the family.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the platform-fit lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PlatformFitReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting platform-fit audit for the lane.
    pub platform_fit_audit_ref: String,
    /// True when support/export parity is required for every family.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every family.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5PlatformFitMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PlatformFitMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Platform-fit rows.
    pub platform_fit_rows: Vec<M5PlatformFitRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5PlatformFitVocabularySet,
    /// Governance-review block.
    pub governance_review: M5PlatformFitGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5PlatformFitConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5PlatformFitProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5PlatformFitReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 platform-fit matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PlatformFitMatrixPacket {
    /// Record kind; must equal [`M5_PLATFORM_FIT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_PLATFORM_FIT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Platform-fit rows.
    pub platform_fit_rows: Vec<M5PlatformFitRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5PlatformFitVocabularySet,
    /// Governance-review block.
    pub governance_review: M5PlatformFitGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5PlatformFitConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5PlatformFitProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5PlatformFitReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5PlatformFitMatrixPacket {
    /// Builds an M5 platform-fit matrix packet from stable-lane input.
    pub fn new(input: M5PlatformFitMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_PLATFORM_FIT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_PLATFORM_FIT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            platform_fit_rows: input.platform_fit_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 platform-fit matrix invariants.
    pub fn validate(&self) -> Vec<M5PlatformFitMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_PLATFORM_FIT_MATRIX_RECORD_KIND {
            violations.push(M5PlatformFitMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_PLATFORM_FIT_MATRIX_SCHEMA_VERSION {
            violations.push(M5PlatformFitMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5PlatformFitMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_platform_fit_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 platform-fit matrix serializes"),
        ) {
            violations.push(M5PlatformFitMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 platform-fit matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "platform_fit_family,qualification,owner,canonical_schema,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.platform_fit_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.platform_fit_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.platform_fit_family.canonical_domain_schema_ref(),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_families = self
            .platform_fit_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Platform-Convention, Shortcut-Notation, File-Path-Reveal, Theme/Contrast Live-Change, Credential-Store Wording, and Input-Method Platform-Fit Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Platform-fit families: {} ({} stable)\n",
            self.platform_fit_rows.len(),
            stable_families
        ));
        out.push_str(&format!(
            "- Platform-fit roles: {}\n",
            self.vocabulary_set.semantic_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Shortcut-notation roles: {}\n",
            self.vocabulary_set.shortcut_notation_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Platform-fit families\n\n");
        for row in &self.platform_fit_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.platform_fit_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.platform_fit_family.canonical_domain_schema_ref()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 platform-fit matrix export.
#[derive(Debug)]
pub enum M5PlatformFitMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5PlatformFitMatrixViolation>),
}

impl fmt::Display for M5PlatformFitMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 platform-fit matrix export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 platform-fit matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5PlatformFitMatrixArtifactError {}

/// Validation failures emitted by [`M5PlatformFitMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5PlatformFitMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required governed platform-fit family is missing from the matrix.
    RequiredFamilyMissing,
    /// A platform-fit row is incomplete.
    PlatformFitRowIncomplete,
    /// A platform-fit row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A platform-fit row does not point at its own canonical domain schema.
    DomainSchemaRefMissing,
    /// A family declares no platform-fit roles.
    SemanticRoleMissing,
    /// The platform-convention family declares no platform-convention roles.
    PlatformConventionRoleMissing,
    /// The shortcut-notation family declares no shortcut-notation roles.
    ShortcutNotationRoleMissing,
    /// The file-path-reveal family declares no file-path-reveal roles.
    FilePathRevealRoleMissing,
    /// The theme-contrast-live-change family declares no theme-contrast-live-change roles.
    ThemeContrastLiveChangeRoleMissing,
    /// The credential-store-wording family declares no credential-store-wording roles.
    CredentialStoreWordingRoleMissing,
    /// The input-method family declares no input-method roles.
    InputMethodRoleMissing,
    /// A family declares no degraded reasons.
    DegradedReasonMissing,
    /// A family declares no surface families.
    SurfaceFamilyMissing,
    /// A family declares no deployment lines.
    DeploymentLineMissing,
    /// A family declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A family declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A family declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A family claiming Stable is missing required proof packet refs.
    StableFamilyMissingProof,
    /// A family violates a hard invariant (platform wording changing command / permission meaning, a
    /// primary action hidden only in OS chrome, a silent plaintext secret fallback, an input method
    /// corrupting text or trust fidelity, or a screenshot / docs page mislabeling a shortcut or path verb).
    PlatformFitInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5PlatformFitMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredFamilyMissing => "required_family_missing",
            Self::PlatformFitRowIncomplete => "platform_fit_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::SemanticRoleMissing => "semantic_role_missing",
            Self::PlatformConventionRoleMissing => "platform_convention_role_missing",
            Self::ShortcutNotationRoleMissing => "shortcut_notation_role_missing",
            Self::FilePathRevealRoleMissing => "file_path_reveal_role_missing",
            Self::ThemeContrastLiveChangeRoleMissing => "theme_contrast_live_change_role_missing",
            Self::CredentialStoreWordingRoleMissing => "credential_store_wording_role_missing",
            Self::InputMethodRoleMissing => "input_method_role_missing",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableFamilyMissingProof => "stable_family_missing_proof",
            Self::PlatformFitInvariantViolated => "platform_fit_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 platform-fit matrix export.
pub fn current_stable_m5_platform_fit_matrix_export(
) -> Result<M5PlatformFitMatrixPacket, M5PlatformFitMatrixArtifactError> {
    let packet: M5PlatformFitMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-platform-fit-proof/support_export.json"
    )))
    .map_err(M5PlatformFitMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5PlatformFitMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5PlatformFitMatrixPacket,
    violations: &mut Vec<M5PlatformFitMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_PLATFORM_FIT_MATRIX_SCHEMA_REF,
        M5_PLATFORM_FIT_MATRIX_DOC_REF,
        M5_SHORTCUT_NOTATION_SCHEMA_REF,
        M5_FILE_PATH_AND_REVEAL_SCHEMA_REF,
        M5_INPUT_METHOD_BEHAVIOR_SCHEMA_REF,
        M5_NATIVE_DESKTOP_MATRIX_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5PlatformFitMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5PlatformFitMatrixPacket,
    violations: &mut Vec<M5PlatformFitMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5PlatformFitMatrixViolation::VocabularySetDrift);
    }
}

fn validate_platform_fit_rows(
    packet: &M5PlatformFitMatrixPacket,
    violations: &mut Vec<M5PlatformFitMatrixViolation>,
) {
    let present: BTreeSet<M5PlatformFitFamily> = packet
        .platform_fit_rows
        .iter()
        .map(|row| row.platform_fit_family)
        .collect();
    for required in M5PlatformFitFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5PlatformFitMatrixViolation::RequiredFamilyMissing);
            return;
        }
    }

    for row in &packet.platform_fit_rows {
        let family = row.platform_fit_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5PlatformFitMatrixViolation::PlatformFitRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5PlatformFitMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == family.canonical_domain_schema_ref())
        {
            violations.push(M5PlatformFitMatrixViolation::DomainSchemaRefMissing);
        }
        if row.semantic_roles.is_empty() {
            violations.push(M5PlatformFitMatrixViolation::SemanticRoleMissing);
        }
        if family.declares_platform_convention_roles() && row.platform_convention_roles.is_empty() {
            violations.push(M5PlatformFitMatrixViolation::PlatformConventionRoleMissing);
        }
        if family.declares_shortcut_notation_roles() && row.shortcut_notation_roles.is_empty() {
            violations.push(M5PlatformFitMatrixViolation::ShortcutNotationRoleMissing);
        }
        if family.declares_file_path_reveal_roles() && row.file_path_reveal_roles.is_empty() {
            violations.push(M5PlatformFitMatrixViolation::FilePathRevealRoleMissing);
        }
        if family.declares_theme_contrast_live_change_roles()
            && row.theme_contrast_live_change_roles.is_empty()
        {
            violations.push(M5PlatformFitMatrixViolation::ThemeContrastLiveChangeRoleMissing);
        }
        if family.declares_credential_store_wording_roles()
            && row.credential_store_wording_roles.is_empty()
        {
            violations.push(M5PlatformFitMatrixViolation::CredentialStoreWordingRoleMissing);
        }
        if family.declares_input_method_roles() && row.input_method_roles.is_empty() {
            violations.push(M5PlatformFitMatrixViolation::InputMethodRoleMissing);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5PlatformFitMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5PlatformFitMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5PlatformFitMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5PlatformFitMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5PlatformFitMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5PlatformFitMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5PlatformFitMatrixViolation::StableFamilyMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5PlatformFitMatrixViolation::PlatformFitInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5PlatformFitMatrixPacket,
    violations: &mut Vec<M5PlatformFitMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.command_ids_stable_while_labels_adapt,
        review.high_frequency_actions_never_hidden_in_os_chrome_alone,
        review.file_path_reveal_save_terminology_matches_host,
        review.theme_contrast_accent_text_scale_apply_live_or_explain_fallback,
        review.credential_store_wording_stays_truthful_and_non_leaky,
        review.input_method_never_corrupts_text_or_trust_fidelity,
        review.shortcut_notation_adapts_per_platform,
        review.no_primary_action_hidden_only_in_os_chrome,
        review.secrets_never_fall_back_to_plaintext_silently,
        review.every_family_declares_deployment_lines,
        review.every_family_declares_accessibility_route,
        review.support_export_reads_single_platform_fit_source,
        review.screenshots_and_docs_bind_to_single_platform_fit_source,
        review.later_rows_cannot_invent_parallel_platform_vocabulary,
        review.platform_fit_survives_zoom_and_high_contrast,
        review.claims_narrow_automatically_when_registry_missing_or_stale,
    ] {
        if !ok {
            violations.push(M5PlatformFitMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5PlatformFitMatrixPacket,
    violations: &mut Vec<M5PlatformFitMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_and_settings_consume_shared_shortcut_and_menu_grammar,
        projection.auth_and_settings_consume_shared_credential_wording,
        projection.input_surfaces_consume_shared_input_method_behavior,
        projection.docs_help_and_screenshots_read_single_platform_fit_source,
        projection.appearance_consumers_bind_to_shared_theme_response,
        projection.support_export_reads_single_platform_fit_source,
    ] {
        if !ok {
            violations.push(M5PlatformFitMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5PlatformFitMatrixPacket,
    violations: &mut Vec<M5PlatformFitMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5PlatformFitMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5PlatformFitMatrixPacket,
    violations: &mut Vec<M5PlatformFitMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.platform_fit_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5PlatformFitMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON. The controlled vocabulary
/// deliberately uses platform / shortcut / path / credential words; what is rejected is a raw secret
/// *value* shape — a pasted passphrase, a bearer token, a raw endpoint URL, or a PEM key block.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
