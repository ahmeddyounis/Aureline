//! Keyboard / screen-reader / high-zoom / high-contrast / localization / CLI / export parity, and honest
//! automatic claim narrowing for the M5 platform-convention / shortcut-notation / file-path-reveal /
//! theme-contrast-live-change / credential-store-wording / input-method platform-fit families.
//!
//! This module is the M05-1170 accessibility-localization-support-export parity and auto-narrowing capstone
//! over the frozen M5 platform-fit matrix ([`crate::m5_platform_fit_matrix`]). Where the freeze matrix
//! defines the six governed platform-fit families, and the 1165-1168 implementation lanes resolve their
//! per-surface shortcut, path, appearance, credential-wording, and input-method truth, this lane certifies —
//! per platform-fit family — that platform-convention / shortcut / path / appearance / credential-wording /
//! input-method claims stay **keyboard-reachable, screen-reader-announced, high-zoom-legible,
//! high-contrast-safe, localization-safe, CLI/export-safe, and self-narrowing** rather than presenting a
//! shortcut that only lives in a screenshot, a path verb that mislabels the host platform, an appearance
//! response shown as live when it did not apply, a credential-store wording that hides a plaintext downgrade,
//! or an input method that corrupts text as still a stable, trusted platform-fit surface:
//!
//! - **Keyboard / screen-reader / high-zoom / high-contrast / localization / CLI reach.** Every family
//!   exposes a keyboard-reachable, screen-reader-announced, high-zoom-reflowing, high-contrast-legible,
//!   localization-safe, and CLI/headless-reachable path into the same platform-fit identity, semantic role,
//!   registry reference, host platform, shortcut notation, and path verb the rendered surface shows — never a
//!   pointer-only affordance hidden in OS chrome, an unlabeled control, or a shortcut / path verb that only
//!   lives in a screenshot and strands assistive-tech, localized, or headless-CLI users. Structure-heavy
//!   families (the shortcut-notation help table, the file-path-reveal table, the input-method composition
//!   table) additionally bind their structured layout to a flat list / textual / CLI path.
//! - **Export parity.** The support / release / CLI export reconstructs each family's meaning from typed
//!   tokens and opaque refs **without a raw payload**, preserving the same platform-fit identity, semantic
//!   role, registry reference, host platform, shortcut notation, and path verb shown in-product so support,
//!   help, and release proof can reconstruct which platform-fit truth class was active without leaking a raw
//!   credential blob, a secret handle, or a renderer-only screenshot.
//! - **Honest auto-narrowing.** When a file-path-reveal registry's localization evidence can only be
//!   partially disclosed, a theme / contrast response's live-apply cannot be confirmed, a credential-store
//!   wording's truthfulness cannot be confirmed, or an input method's text fidelity is unconfirmed, the
//!   family's claim auto-narrows from `trusted_platform_fit_surface` / `reviewable_platform_fit_surface` to a
//!   path-terminology-disclosed / appearance-response-unverified / credential-wording-unverified /
//!   input-fidelity-unverified projection, discloses the narrowing with a precise trigger and binding
//!   dimension, and preserves the canonical platform-fit identity / last-known registry reference. The
//!   underlying shortcut / path / appearance / credential-wording / input-method truth is never dropped
//!   opaquely. A family with every dimension intact must NOT carry a spurious narrowing, and a
//!   command-renaming / plaintext-downgrading / text-corrupting / OS-chrome-hiding state can never keep a
//!   trusted, stable platform-fit claim — platform-fit meaning is never conveyed by an OS-chrome-only
//!   affordance, a mislabeled screenshot, or an unlabeled control alone.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the shell UI, the settings UI, the
//!   auth UI, the input UI, the docs / help surface, the onboarding surface, the CLI export, the support
//!   export, and the product UI so product, help, and release publication stay aligned on downgrade behavior
//!   rather than drifting in copy — a trusted-looking platform-fit surface can never outrun the shortcut /
//!   path / appearance / credential-wording / input-method evidence it is being viewed away from.
//!
//! Each [`PlatformFitAccessibilityRow`] keys on one
//! [`crate::m5_platform_fit_matrix::M5PlatformFitFamily`] and reuses that frozen family vocabulary plus the
//! frozen [`M5PlatformFitRequiredLabel`], [`M5PlatformFitDowngradeTrigger`], and shared
//! [`M5PlatformFitConsumerSurface`] consumer surfaces rather than minting parallel synonyms, so the certified
//! labels stay byte-identical to the matrix and the sibling platform-fit packets.
//!
//! The packet is metadata-only: raw credential blobs, secret handles, plaintext payloads, and endpoint refs
//! never cross this boundary; the packet carries only typed class tokens, opaque platform-fit refs, booleans,
//! and controlled labels so support, release, and diagnostics exports can reconstruct exactly which
//! platform-fit truth class was active without leaking sensitive material or a raw payload.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen platform-fit vocabulary — the capstone certifies the freeze matrix's families, required
// labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::m5_platform_fit_matrix::{
    M5PlatformFitConsumerSurface, M5PlatformFitDowngradeTrigger, M5PlatformFitFamily,
    M5PlatformFitRequiredLabel, M5_PLATFORM_FIT_MATRIX_SCHEMA_REF,
};

/// Schema version stamped on the M05-1170 platform-fit accessibility parity packet.
pub const PLATFORM_FIT_A11Y_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`PlatformFitAccessibilityPacket`].
pub const PLATFORM_FIT_A11Y_RECORD_KIND: &str = "m5_platform_fit_accessibility_parity_packet";

/// Stable record-kind tag carried by each [`PlatformFitAccessibilityRow`].
pub const PLATFORM_FIT_A11Y_ROW_RECORD_KIND: &str = "m5_platform_fit_accessibility_parity_row";

/// Repo-relative path of the boundary schema.
pub const PLATFORM_FIT_A11Y_SCHEMA_REF: &str =
    "schemas/platform/m5-platform-fit-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const PLATFORM_FIT_A11Y_DOC_REF: &str = "docs/platform/m5_platform_fit_accessibility_parity.md";

/// Repo-relative path of the frozen platform-fit matrix this lane certifies.
pub const PLATFORM_FIT_A11Y_MATRIX_REF: &str = M5_PLATFORM_FIT_MATRIX_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const PLATFORM_FIT_A11Y_FIXTURE_DIR: &str =
    "fixtures/platform/m5-platform-fit-accessibility-parity";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const PLATFORM_FIT_A11Y_ARTIFACT_REF: &str =
    "artifacts/release/m5-platform-fit-accessibility-parity/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const PLATFORM_FIT_A11Y_CSV_REF: &str =
    "artifacts/release/m5-platform-fit-accessibility-parity/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const PLATFORM_FIT_A11Y_REPORT_REF: &str =
    "artifacts/release/m5-platform-fit-accessibility-parity.md";

/// The reusable platform-fit families that render a dense, structured surface (the shortcut-notation help
/// table, the file-path-reveal table, the input-method composition table) and therefore MUST bind their
/// structured layout to an equivalent flat list / textual / CLI path so the structure is navigable
/// non-visually.
const fn family_is_structure_heavy(family: M5PlatformFitFamily) -> bool {
    matches!(
        family,
        M5PlatformFitFamily::ShortcutNotation
            | M5PlatformFitFamily::FilePathReveal
            | M5PlatformFitFamily::InputMethod
    )
}

/// The platform-fit-truth dimension whose weakening a family primarily discloses. Every row must model at
/// least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(family: M5PlatformFitFamily) -> M5PlatformFitClaimDimension {
    match family {
        M5PlatformFitFamily::PlatformConvention => {
            M5PlatformFitClaimDimension::PlatformConventionClarity
        }
        M5PlatformFitFamily::ShortcutNotation => {
            M5PlatformFitClaimDimension::ShortcutNotationClarity
        }
        M5PlatformFitFamily::FilePathReveal => M5PlatformFitClaimDimension::PathTerminologyClarity,
        M5PlatformFitFamily::ThemeContrastLiveChange => {
            M5PlatformFitClaimDimension::AppearanceResponseClarity
        }
        M5PlatformFitFamily::CredentialStoreWording => {
            M5PlatformFitClaimDimension::CredentialWordingClarity
        }
        M5PlatformFitFamily::InputMethod => M5PlatformFitClaimDimension::InputFidelityClarity,
    }
}

/// A rendered fallback modality for a platform-fit family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PlatformFitFallbackModality {
    /// A rich, structured (shortcut / path / input-composition table) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / label-first projection.
    Textual,
    /// A CLI / headless text projection.
    Cli,
}

impl M5PlatformFitFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich, structured surface
    /// (i.e. a keyboard / screen-reader / CLI path).
    pub const fn is_non_visual(self) -> bool {
        matches!(self, Self::List | Self::Textual | Self::Cli)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Structured => "structured",
            Self::List => "list",
            Self::Textual => "textual",
            Self::Cli => "cli",
        }
    }
}

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the same platform-fit
/// family may render at desktop-full capability or narrow to a companion, read-only browser, headless CLI,
/// docs export, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PlatformFitRenderingSurface {
    /// The full-capability desktop surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A docs / help export projection.
    DocsExport,
    /// A support / release / evaluation export.
    SupportExport,
}

impl M5PlatformFitRenderingSurface {
    /// Returns true when the surface narrows the platform-fit family below the desktop full-capability
    /// baseline and therefore must disclose its reduction.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::CliHeadless => "cli_headless",
            Self::DocsExport => "docs_export",
            Self::SupportExport => "support_export",
        }
    }
}

/// Keyboard / screen-reader / high-zoom / high-contrast / localization / CLI reach for a platform-fit
/// family's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformFitNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// An OS-chrome-only / pointer-only / view-only surface that traps keyboard / assistive-tech /
    /// localized / headless-CLI users (red).
    ViewOnlyTrap,
}

impl PlatformFitNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech / localized / CLI users.
    pub const fn never_traps(self) -> bool {
        !matches!(self, Self::ViewOnlyTrap)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedReducedButReachable)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReachableAndLabeled => "reachable_and_labeled",
            Self::DisclosedReducedButReachable => "disclosed_reduced_but_reachable",
            Self::ViewOnlyTrap => "view_only_trap",
        }
    }
}

/// Whether an export-safe summary preserves the platform-fit meaning without leaking a raw payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformFitExportSummaryState {
    /// The platform-fit meaning reconstructs from the metadata summary without a raw payload.
    ReconstructableWithoutRawPayload,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export can only carry meaning by dumping a raw payload (red).
    RequiresRawPayload,
}

impl PlatformFitExportSummaryState {
    /// Returns true when the export never falls back to leaking a raw payload.
    pub const fn never_requires_raw_payload(self) -> bool {
        !matches!(self, Self::RequiresRawPayload)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructableWithoutRawPayload => "reconstructable_without_raw_payload",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::RequiresRawPayload => "requires_raw_payload",
        }
    }
}

/// Whether a narrower rendering surface discloses its reduced platform-fit projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformFitNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced projection, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Platform-fit state or tokens dropped without disclosure (red).
    SilentlyDropped,
}

impl PlatformFitNarrowingDisclosureState {
    /// Returns true when the surface never silently drops state or tokens.
    pub const fn never_drops_silently(self) -> bool {
        !matches!(self, Self::SilentlyDropped)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedNarrowed)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParityPreserved => "parity_preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::SilentlyDropped => "silently_dropped",
        }
    }
}

/// The platform-fit claim ceiling a family asserts: how strong a trusted / stable posture it lets a surface
/// present. Auto-narrowing lowers this ceiling when a path-terminology / appearance-response /
/// credential-wording / input-fidelity dimension weakens so a partially-disclosed path localization, an
/// unconfirmed live theme response, an unconfirmed credential-store wording, or an unconfirmed input fidelity
/// can never keep an old `TrustedPlatformFitSurface` or `ReviewablePlatformFitSurface` label — platform-fit
/// meaning is never conveyed by an OS-chrome-only affordance, a mislabeled screenshot, or an unlabeled
/// control alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PlatformFitA11yClaim {
    /// Trusted platform-fit surface: a fully current, registry-bound, host-correct, live-appearance,
    /// truthful-credential-wording, text-faithful platform-fit family — the strongest claim, a platform-fit
    /// surface Aureline can present as exactly trusted and stable right now.
    TrustedPlatformFitSurface,
    /// Reviewable platform-fit surface: a self-sufficient, inspectable read-only platform-fit projection (a
    /// static shortcut-notation / registry reference a user can inspect) that is not itself an authoritative,
    /// live-rendering surface.
    ReviewablePlatformFitSurface,
    /// Path-terminology-disclosed projection: a file / path / reveal / save terminology can only be partially
    /// disclosed for a locale; the family stays a path-terminology-disclosed projection that discloses the
    /// partial localization alongside the last-known host-correct verb, never a mislabeled path / reveal verb
    /// shown as host-correct when its localization is incomplete.
    PathTerminologyDisclosedProjection,
    /// Appearance-response-unverified projection: a theme / contrast / accent / text-scale response's
    /// live-apply cannot be confirmed; the family stays an appearance-response-unverified projection that
    /// keeps the last-known appearance posture explicit, never a theme or contrast change shown as applied
    /// live when it may not have applied or explained its fallback.
    AppearanceResponseUnverifiedProjection,
    /// Credential-wording-unverified projection: a credential-store wording's truthful, non-leaky posture
    /// cannot be confirmed; the family stays a credential-wording-unverified projection that keeps the
    /// last-known credential-store wording explicit, never a credential-store message shown as truthful when
    /// it may hide a plaintext-storage fallback.
    CredentialWordingUnverifiedProjection,
    /// Input-fidelity-unverified projection: an input method's text and trust fidelity cannot be confirmed;
    /// the family stays an input-fidelity-unverified projection that keeps the last-known input-method state
    /// explicit, never an IME / dead-key / dictation flow shown as faithful when it may corrupt text or trust
    /// semantics.
    InputFidelityUnverifiedProjection,
}

impl M5PlatformFitA11yClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 6] = [
        Self::TrustedPlatformFitSurface,
        Self::ReviewablePlatformFitSurface,
        Self::PathTerminologyDisclosedProjection,
        Self::AppearanceResponseUnverifiedProjection,
        Self::CredentialWordingUnverifiedProjection,
        Self::InputFidelityUnverifiedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::TrustedPlatformFitSurface => 5,
            Self::ReviewablePlatformFitSurface => 4,
            Self::PathTerminologyDisclosedProjection => 3,
            Self::AppearanceResponseUnverifiedProjection => 2,
            Self::CredentialWordingUnverifiedProjection => 1,
            Self::InputFidelityUnverifiedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully trusted, stable platform-fit surface.
    pub const fn asserts_trusted_surface(self) -> bool {
        matches!(self, Self::TrustedPlatformFitSurface)
    }

    /// Returns true when this claim asserts a fully self-sufficient (trusted or reviewable) surface.
    pub const fn asserts_self_sufficient_surface(self) -> bool {
        matches!(
            self,
            Self::TrustedPlatformFitSurface | Self::ReviewablePlatformFitSurface
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedPlatformFitSurface => "trusted_platform_fit_surface",
            Self::ReviewablePlatformFitSurface => "reviewable_platform_fit_surface",
            Self::PathTerminologyDisclosedProjection => "path_terminology_disclosed_projection",
            Self::AppearanceResponseUnverifiedProjection => {
                "appearance_response_unverified_projection"
            }
            Self::CredentialWordingUnverifiedProjection => {
                "credential_wording_unverified_projection"
            }
            Self::InputFidelityUnverifiedProjection => "input_fidelity_unverified_projection",
        }
    }
}

/// The platform-convention / shortcut-notation / path-terminology / appearance-response / credential-wording /
/// input-fidelity dimension whose state governs how far a platform-fit family may claim to be a fully
/// trusted, stable platform-fit surface. The dimensions map 1:1 to the six frozen platform-fit families so
/// every family carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PlatformFitClaimDimension {
    /// Platform-convention clarity: do window controls, menu-bar behavior, and system chrome match the host
    /// platform without hiding a primary action only in OS chrome (platform-convention)?
    PlatformConventionClarity,
    /// Shortcut-notation clarity: does the shortcut notation adapt from one registry with stable command IDs
    /// rather than drifting by platform or mislabeling a screenshot (shortcut-notation)?
    ShortcutNotationClarity,
    /// Path-terminology clarity: do file / path / reveal / save terminology stay host-correct and fully
    /// localized rather than mislabeling the reveal verb (file-path-reveal)?
    PathTerminologyClarity,
    /// Appearance-response clarity: does a theme / contrast / accent / text-scale change apply live or explain
    /// its fallback rather than silently failing to apply (theme-contrast-live-change)?
    AppearanceResponseClarity,
    /// Credential-wording clarity: does the credential-store wording stay truthful and non-leaky rather than
    /// hiding a plaintext-storage fallback (credential-store-wording)?
    CredentialWordingClarity,
    /// Input-fidelity clarity: do IME, dead keys, AltGr, dictation, emoji, and layout switching preserve text
    /// and trust fidelity rather than corrupting text (input-method)?
    InputFidelityClarity,
}

impl M5PlatformFitClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PlatformConventionClarity,
        Self::ShortcutNotationClarity,
        Self::PathTerminologyClarity,
        Self::AppearanceResponseClarity,
        Self::CredentialWordingClarity,
        Self::InputFidelityClarity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlatformConventionClarity => "platform_convention_clarity",
            Self::ShortcutNotationClarity => "shortcut_notation_clarity",
            Self::PathTerminologyClarity => "path_terminology_clarity",
            Self::AppearanceResponseClarity => "appearance_response_clarity",
            Self::CredentialWordingClarity => "credential_wording_clarity",
            Self::InputFidelityClarity => "input_fidelity_clarity",
        }
    }
}

/// The observed condition of one platform-fit-truth dimension. Anything weaker than
/// [`Self::FullyQualified`] imposes a narrowing ceiling on the family's claim. The unconfirmed states the
/// lane must auto-narrow on as *weakened evidence* — an unconfirmed live-appearance response, an unconfirmed
/// truthful credential wording, and an unconfirmed input fidelity — are the states that
/// [`Self::cannot_be_shown_trusted`] flags. A partially-disclosed path localization is an honest
/// disclosed-absence operation (a partial localization shown honestly with the last-known host-correct verb),
/// not a truth overstatement, so it is deliberately excluded there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PlatformFitConditionState {
    /// Fully current, registry-bound, host-correct, live-appearance, truthful-credential-wording,
    /// text-faithful — imposes no ceiling.
    FullyQualified,
    /// The file / path / reveal / save terminology can only be partially disclosed for a locale — claim
    /// drops to a path-terminology-disclosed projection.
    PathTerminologyDisclosedPartial,
    /// The theme / contrast / accent / text-scale response's live-apply cannot be confirmed — claim drops to
    /// an appearance-response-unverified projection.
    AppearanceResponseUnconfirmed,
    /// The credential-store wording's truthful, non-leaky posture cannot be confirmed — claim drops to a
    /// credential-wording-unverified projection.
    CredentialWordingUnconfirmed,
    /// The input method's text and trust fidelity cannot be confirmed — claim drops to an
    /// input-fidelity-unverified projection.
    InputFidelityUnconfirmed,
}

impl M5PlatformFitConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullyQualified,
        Self::PathTerminologyDisclosedPartial,
        Self::AppearanceResponseUnconfirmed,
        Self::CredentialWordingUnconfirmed,
        Self::InputFidelityUnconfirmed,
    ];

    /// Returns true when the dimension is weaker than fully qualified and therefore imposes a narrowing
    /// ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::FullyQualified)
    }

    /// Returns true when the condition reflects weakened evidence that cannot be shown as a fully trusted,
    /// stable platform-fit surface and must never be shown as such. A partially-disclosed path localization
    /// is an honest disclosed-absence operation (a partial localization shown honestly with the last-known
    /// host-correct verb), not a truth overstatement, so it is deliberately excluded here.
    pub const fn cannot_be_shown_trusted(self) -> bool {
        matches!(
            self,
            Self::AppearanceResponseUnconfirmed
                | Self::CredentialWordingUnconfirmed
                | Self::InputFidelityUnconfirmed
        )
    }

    /// The strongest claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5PlatformFitA11yClaim {
        match self {
            Self::FullyQualified => M5PlatformFitA11yClaim::TrustedPlatformFitSurface,
            Self::PathTerminologyDisclosedPartial => {
                M5PlatformFitA11yClaim::PathTerminologyDisclosedProjection
            }
            Self::AppearanceResponseUnconfirmed => {
                M5PlatformFitA11yClaim::AppearanceResponseUnverifiedProjection
            }
            Self::CredentialWordingUnconfirmed => {
                M5PlatformFitA11yClaim::CredentialWordingUnverifiedProjection
            }
            Self::InputFidelityUnconfirmed => {
                M5PlatformFitA11yClaim::InputFidelityUnverifiedProjection
            }
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing. Each state maps
    /// to the on-topic frozen trigger the freeze matrix already governs, so the certified reason stays
    /// byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5PlatformFitDowngradeTrigger {
        match self {
            // The fully-qualified baseline never narrows; kept for exhaustiveness.
            Self::FullyQualified => M5PlatformFitDowngradeTrigger::ProofStale,
            Self::PathTerminologyDisclosedPartial => M5PlatformFitDowngradeTrigger::ProofStale,
            Self::AppearanceResponseUnconfirmed => {
                M5PlatformFitDowngradeTrigger::ThemeOrContrastChangeDidNotApplyLiveOrExplainFallback
            }
            Self::CredentialWordingUnconfirmed => {
                M5PlatformFitDowngradeTrigger::SecretStorageFellBackToPlaintextSilently
            }
            Self::InputFidelityUnconfirmed => {
                M5PlatformFitDowngradeTrigger::InputMethodCorruptedTextOrTrust
            }
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyQualified => "fully_qualified",
            Self::PathTerminologyDisclosedPartial => "path_terminology_disclosed_partial",
            Self::AppearanceResponseUnconfirmed => "appearance_response_unconfirmed",
            Self::CredentialWordingUnconfirmed => "credential_wording_unconfirmed",
            Self::InputFidelityUnconfirmed => "input_fidelity_unconfirmed",
        }
    }
}

/// One platform-fit-truth dimension's observed condition on a family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformFitClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5PlatformFitClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5PlatformFitConditionState,
}

/// An honest claim auto-narrow block. When a platform-fit-truth dimension weakens, the family's claim lowers
/// to the permitted ceiling, names the binding dimension and frozen trigger, and preserves the canonical
/// platform-fit identity / last-known registry reference rather than silently dropping it — the underlying
/// shortcut / path / appearance / credential-wording / input-method truth is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformFitClaimAutoNarrow {
    /// The claim the family is narrowed to.
    pub narrowed_to: M5PlatformFitA11yClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling constraint).
    pub binding_dimension: M5PlatformFitClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5PlatformFitDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical platform-fit identity and last-known registry reference are preserved rather than
    /// dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying shortcut / path / appearance / credential-wording / input-method truth is preserved
    /// (never dropped) across the narrowing; must hold so path-terminology-disclosed,
    /// appearance-response-unverified, credential-wording-unverified, and input-fidelity-unverified states
    /// never fail opaquely.
    pub preserves_truth_continuity: bool,
}

impl PlatformFitClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and shortcut / path /
    /// appearance / credential-wording / input-method truth and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_truth_continuity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a platform-fit family's accessible fallback: the same truth must be copyable as
/// text / JSON / Markdown, and a raw payload is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformFitCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A raw payload is never the only export; must always hold.
    pub raw_payload_only_prohibited: bool,
}

impl PlatformFitCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all offered, at least one
    /// export field is named, and a raw-payload-only export is prohibited.
    pub fn is_complete(&self) -> bool {
        self.raw_payload_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
            && !self.export_fields.is_empty()
    }
}

/// Per-rendering-surface narrowing disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformFitRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5PlatformFitRenderingSurface,
    /// How the surface discloses its reduced platform-fit projection.
    pub state: PlatformFitNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The platform-fit affordances reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a platform-fit accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformFitAccessibilityStatus {
    /// Full keyboard / screen-reader / high-zoom / high-contrast / localization / CLI / export parity with no
    /// narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a raw payload, over-claims trusted, or drops state silently (red).
    Stranded,
}

impl PlatformFitAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one platform-fit family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformFitAccessibilityRow {
    /// Record kind; must equal [`PLATFORM_FIT_A11Y_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`PLATFORM_FIT_A11Y_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen platform-fit family this row certifies.
    pub platform_fit_family: M5PlatformFitFamily,
    /// Ref to the frozen canonical per-domain schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the platform-fit family this row represents; stays visible on every surface, so this is
    /// never empty.
    pub platform_fit_context_ref: String,
    /// Rendered modalities offered; a structure-heavy family must also offer a non-visual (list / textual /
    /// CLI) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5PlatformFitFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical platform-fit identity, semantic role, registry
    /// reference, host platform, shortcut notation, and path verb as the rendered family; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: PlatformFitNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: PlatformFitNonVisualReachState,
    /// High-zoom (200–400% reflow / magnification) legibility of the non-visual path.
    pub high_zoom_reach: PlatformFitNonVisualReachState,
    /// High-contrast / larger-text legibility of the non-visual path.
    pub high_contrast_reach: PlatformFitNonVisualReachState,
    /// Localization (translated vocabulary / locale-specific verbs) fidelity of the non-visual path.
    pub localization_reach: PlatformFitNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: PlatformFitNonVisualReachState,
    /// Whether the export-safe summary preserves platform-fit meaning.
    pub export_summary: PlatformFitExportSummaryState,
    /// Ref to the export-safe summary object for this family.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: PlatformFitCopyExportParity,
    /// The full claim this family asserts when every dimension is intact.
    pub full_ready_claim: M5PlatformFitA11yClaim,
    /// The observed condition of each modeled platform-fit-truth dimension.
    #[serde(default)]
    pub claim_conditions: Vec<PlatformFitClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the family's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<PlatformFitClaimAutoNarrow>,
    /// Whether the underlying shortcut / path / appearance / credential-wording / input-method truth is
    /// preserved on this family regardless of narrowing; must hold so every unverified projection never fails
    /// opaquely.
    pub truth_preserved: bool,
    /// Rendering surfaces this family is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5PlatformFitRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<PlatformFitRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5PlatformFitRequiredLabel>,
    /// Semantic consumer surfaces this family is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5PlatformFitConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl PlatformFitAccessibilityRow {
    /// Returns true when this family renders a dense, structured surface and must bind to a flat non-visual
    /// path.
    pub const fn is_structure_heavy(&self) -> bool {
        family_is_structure_heavy(self.platform_fit_family)
    }

    /// Returns true when at least one non-visual (list / textual / CLI) fallback modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `FullyQualified` when the row does not model that
    /// dimension.
    pub fn condition_for(
        &self,
        dimension: M5PlatformFitClaimDimension,
    ) -> M5PlatformFitConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5PlatformFitConditionState::FullyQualified)
    }

    /// Whether any modeled dimension is weaker than fully qualified.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest claim permitted after applying every modeled dimension's ceiling, capped at the family's
    /// full claim.
    pub fn permitted_claim(&self) -> M5PlatformFitA11yClaim {
        let mut permitted = self.full_ready_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The condition entry imposing the strongest (lowest-rank) ceiling, if any weak dimension narrows below
    /// the family's full claim.
    pub fn binding_condition(&self) -> Option<&PlatformFitClaimConditionEntry> {
        let mut binding: Option<(&PlatformFitClaimConditionEntry, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_ready_claim.capability_rank() {
                // The dimension is weak but does not narrow below the full claim.
                continue;
            }
            let rank = ceiling.capability_rank();
            match binding {
                Some((_, best)) if best <= rank => {}
                _ => binding = Some((condition, rank)),
            }
        }
        binding.map(|(condition, _)| condition)
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any.
    pub fn binding_dimension(&self) -> Option<M5PlatformFitClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The claim this family effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5PlatformFitA11yClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_ready_claim,
        }
    }

    /// AC / auto-narrowing honesty: a partially-disclosed path localization, an unconfirmed live-appearance
    /// response, an unconfirmed credential wording, or an unconfirmed input fidelity can no longer keep an old
    /// `TrustedPlatformFitSurface` / `ReviewablePlatformFitSurface` label. The effective claim never exceeds
    /// the permitted ceiling; when a dimension narrows below the full claim, an honest narrow block is
    /// present, narrows to exactly the permitted ceiling, binds to the ceiling-imposing dimension with its
    /// frozen trigger, and preserves canonical identity and truth. When nothing narrows, no spurious narrow
    /// block is present.
    pub fn claim_is_honest(&self) -> bool {
        let permitted = self.permitted_claim();
        if self.effective_claim().capability_rank() > permitted.capability_rank() {
            return false;
        }
        match (&self.claim_narrow, self.binding_condition()) {
            (Some(narrow), Some(binding)) => {
                narrow.is_honest()
                    && narrow.narrowed_to == permitted
                    && narrow.binding_dimension == binding.dimension
                    && narrow.trigger == binding.state.default_trigger()
                    && binding.state.is_weak()
            }
            // A narrow block with no ceiling-imposing dimension is spurious.
            (Some(_), None) => false,
            // A ceiling-imposing dimension with no narrow block over-claims.
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }

    /// AC / trusted honesty: an unconfirmed-appearance / unconfirmed-credential / unconfirmed-input state
    /// never keeps a trusted claim — platform-fit meaning is never conveyed by an OS-chrome-only affordance, a
    /// mislabeled screenshot, or an unlabeled control alone. When such a state is modeled, the effective claim
    /// must not assert `TrustedPlatformFitSurface`.
    pub fn trusted_honesty_holds(&self) -> bool {
        let has_unprovable_state = self
            .claim_conditions
            .iter()
            .any(|c| c.state.cannot_be_shown_trusted());
        !(has_unprovable_state && self.effective_claim().asserts_trusted_surface())
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical truth — no
    /// keyboard / screen-reader / high-zoom / high-contrast / localization / CLI trap, a structure-heavy
    /// family offers a non-visual fallback, and the export reconstructs meaning without a raw payload.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.platform_fit_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.high_zoom_reach.never_traps()
            && self.high_contrast_reach.never_traps()
            && self.localization_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_structure_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the platform-fit meaning without leaking a raw payload.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_requires_raw_payload()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / no-loss: every unverified projection preserves the underlying shortcut / path / appearance /
    /// credential-wording / input-method truth. The row must assert `truth_preserved`, and any narrow block
    /// must preserve truth continuity too.
    pub fn preserves_truth_continuity(&self) -> bool {
        self.truth_preserved
            && self
                .claim_narrow
                .as_ref()
                .map(|n| n.preserves_truth_continuity)
                .unwrap_or(true)
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the family carries an honest claim
    /// narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
            || self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.high_zoom_reach.is_disclosed_reduction()
            || self.high_contrast_reach.is_disclosed_reduction()
            || self.localization_reach.is_disclosed_reduction()
            || self.cli_reach.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction())
    }

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its reduced platform-fit
    /// projection and keeps its labels, so product / help / release publication stay aligned on the same
    /// narrowed state.
    pub fn narrowing_disclosed(&self) -> bool {
        // Every declared narrowed rendering surface has a disclosure entry.
        for surface in &self.rendering_surfaces {
            if surface.is_narrowed()
                && !self
                    .narrowing_disclosures
                    .iter()
                    .any(|d| d.rendering_surface == *surface)
            {
                return false;
            }
        }
        // Every disclosure never silently drops and preserves labels on a narrowed surface.
        self.narrowing_disclosures.iter().all(|d| {
            d.state.never_drops_silently()
                && (!d.rendering_surface.is_narrowed() || !d.preserved_labels.is_empty())
        })
    }

    /// Whether the row models its family's primary weakening dimension.
    pub fn models_primary_dimension(&self) -> bool {
        let primary = family_primary_dimension(self.platform_fit_family);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5PlatformFitRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> PlatformFitAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.trusted_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_truth_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return PlatformFitAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            PlatformFitAccessibilityStatus::NarrowedDisclosed
        } else {
            PlatformFitAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == PLATFORM_FIT_A11Y_ROW_RECORD_KIND
            && self.schema_version == PLATFORM_FIT_A11Y_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.platform_fit_context_ref.trim().is_empty()
            && !self.fallback_modalities.is_empty()
            && !self.claim_conditions.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} keyboard={keyboard} screen_reader={screen_reader} \
high_zoom={high_zoom} high_contrast={high_contrast} localization={localization} cli={cli} \
export={export} full_claim={full} effective_claim={effective} status={status}",
            family = self.platform_fit_family.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            high_zoom = self.high_zoom_reach.as_str(),
            high_contrast = self.high_contrast_reach.as_str(),
            localization = self.localization_reach.as_str(),
            cli = self.cli_reach.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_ready_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1170 platform-fit accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformFitAccessibilitySummary {
    pub row_count: usize,
    pub family_count: usize,
    pub structure_heavy_family_count: usize,
    pub all_structure_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_trusted_honesty_holds: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_truth_preserved: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`PlatformFitAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformFitAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<PlatformFitAccessibilityRow>,
}

/// Checked-in M05-1170 platform-fit accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformFitAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<PlatformFitAccessibilityRow>,
    pub summary: PlatformFitAccessibilitySummary,
}

impl PlatformFitAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: PlatformFitAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: PLATFORM_FIT_A11Y_SCHEMA_VERSION,
            record_kind: PLATFORM_FIT_A11Y_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: PlatformFitAccessibilitySummary {
                row_count: 0,
                family_count: 0,
                structure_heavy_family_count: 0,
                all_structure_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_trusted_honesty_holds: false,
                all_export_summaries_preserve_meaning: false,
                all_truth_preserved: false,
                all_narrowing_disclosed: false,
                green_count: 0,
                yellow_count: 0,
                red_count: 0,
                rendering_surface_count: 0,
                consumer_surface_count: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5PlatformFitFamily> {
        self.rows.iter().map(|r| r.platform_fit_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5PlatformFitClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5PlatformFitConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5PlatformFitA11yClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5PlatformFitConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> PlatformFitAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5PlatformFitConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let structure_heavy: Vec<&PlatformFitAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_structure_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                PlatformFitAccessibilityStatus::Parity => green += 1,
                PlatformFitAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                PlatformFitAccessibilityStatus::Stranded => red += 1,
            }
        }

        PlatformFitAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            structure_heavy_family_count: structure_heavy.len(),
            all_structure_heavy_have_non_visual_fallback: structure_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(PlatformFitAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(PlatformFitAccessibilityRow::claim_is_honest),
            all_trusted_honesty_holds: self
                .rows
                .iter()
                .all(PlatformFitAccessibilityRow::trusted_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(PlatformFitAccessibilityRow::export_preserves_meaning),
            all_truth_preserved: self
                .rows
                .iter()
                .all(PlatformFitAccessibilityRow::preserves_truth_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(PlatformFitAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<PlatformFitAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != PLATFORM_FIT_A11Y_SCHEMA_VERSION {
            violations.push(PlatformFitAccessibilityViolation::SchemaVersion {
                expected: PLATFORM_FIT_A11Y_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != PLATFORM_FIT_A11Y_RECORD_KIND {
            violations.push(PlatformFitAccessibilityViolation::RecordKind {
                expected: PLATFORM_FIT_A11Y_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(PlatformFitAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        let mut has_unprovable_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(PlatformFitAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.platform_fit_family);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.cannot_be_shown_trusted())
            {
                has_unprovable_row = true;
            }

            if !row.is_complete() {
                violations.push(PlatformFitAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(PlatformFitAccessibilityViolation::MissingPrimaryDimension {
                    id: row.row_id.clone(),
                    dimension: family_primary_dimension(row.platform_fit_family),
                });
            }

            // Each row must preserve every mandatory platform-fit label.
            if !row.preserves_mandatory_labels() {
                violations.push(PlatformFitAccessibilityViolation::MissingMandatoryLabel {
                    id: row.row_id.clone(),
                });
            }

            // A structure-heavy family must render a structured projection *and* a non-visual path.
            if row.is_structure_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5PlatformFitFallbackModality::Structured)
            {
                violations.push(
                    PlatformFitAccessibilityViolation::StructureHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: claim never over-asserts a trusted / reviewable surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(PlatformFitAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // AC / trusted honesty: an unconfirmed-appearance / unconfirmed-credential / unconfirmed-input
            // state never keeps a trusted claim.
            if !row.trusted_honesty_holds() {
                violations.push(PlatformFitAccessibilityViolation::WeakStateShownAsTrusted {
                    id: row.row_id.clone(),
                });
            }

            // AC: assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(PlatformFitAccessibilityViolation::AssistiveTechStranded {
                    id: row.row_id.clone(),
                });
            }

            // AC: export preserves meaning without leaking a raw payload.
            if !row.export_preserves_meaning() {
                violations.push(
                    PlatformFitAccessibilityViolation::ExportRequiresRawPayload {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: weakened states preserve shortcut / path / appearance / credential / input truth.
            if !row.preserves_truth_continuity() {
                violations.push(PlatformFitAccessibilityViolation::TruthDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    PlatformFitAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(PlatformFitAccessibilityViolation::MissingConsumerParity {
                    id: row.row_id.clone(),
                });
            }

            // No red rows may ship.
            if row.status() == PlatformFitAccessibilityStatus::Stranded {
                violations.push(PlatformFitAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5PlatformFitFamily::ALL {
            if !seen_families.contains(&family) {
                violations
                    .push(PlatformFitAccessibilityViolation::MissingFamilyCoverage { family });
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5PlatformFitClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    PlatformFitAccessibilityViolation::MissingDimensionCoverage { dimension },
                );
            }
        }

        // Coverage: every condition state (the fully-qualified baseline plus each spec narrowing axis) is
        // exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5PlatformFitConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    PlatformFitAccessibilityViolation::MissingConditionStateCoverage { state },
                );
            }
        }

        // Coverage: every claim tier appears as an effective claim, so the full narrowing spectrum
        // (trusted → … → input-fidelity-unverified) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5PlatformFitA11yClaim::ALL {
            if !effective.contains(&claim) {
                violations
                    .push(PlatformFitAccessibilityViolation::MissingClaimTierCoverage { claim });
            }
        }

        // Trusted honesty must be proven with at least one unconfirmed-appearance / unconfirmed-credential /
        // unconfirmed-input row in the packet, so the "cannot-prove never shown as trusted" guarantee is
        // exercised end-to-end.
        if !has_unprovable_row {
            violations.push(PlatformFitAccessibilityViolation::TrustedHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the shell, settings, auth, input, docs/help,
        // onboarding, CLI-export, support-export, and product surfaces — so every consumer surface is
        // exercised at least once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5PlatformFitConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    PlatformFitAccessibilityViolation::MissingConsumerSurfaceCoverage { surface },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(PlatformFitAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("platform-fit accessibility parity packet serializes"),
        ) {
            violations.push(PlatformFitAccessibilityViolation::RawPlatformFitMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("platform-fit accessibility parity packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,platform_fit_family,keyboard_reach,screen_reader_reach,high_zoom_reach,high_contrast_reach,localization_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{high_zoom},{high_contrast},{localization},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                family = row.platform_fit_family.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                high_zoom = row.high_zoom_reach.as_str(),
                high_contrast = row.high_contrast_reach.as_str(),
                localization = row.localization_reach.as_str(),
                cli = row.cli_reach.as_str(),
                export = row.export_summary.as_str(),
                full = row.full_ready_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, help, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Platform-Fit Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5PlatformFitFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Status: {} green / {} yellow / {} red\n",
            self.summary.green_count, self.summary.yellow_count, self.summary.red_count,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.row_id,
                row.platform_fit_family.as_str(),
                row.chip_tokens(),
            ));
            if let Some(narrow) = &row.claim_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: {} → {} (dimension={}, trigger={}) — {}\n",
                    row.full_ready_claim.as_str(),
                    narrow.narrowed_to.as_str(),
                    narrow.binding_dimension.as_str(),
                    narrow.trigger.as_str(),
                    narrow.narrowed_label,
                ));
            }
        }
        out
    }
}

/// Reads and validates the checked-in platform-fit accessibility parity export.
pub fn current_m5_platform_fit_a11y_export(
) -> Result<PlatformFitAccessibilityPacket, PlatformFitAccessibilityArtifactError> {
    let packet: PlatformFitAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-platform-fit-accessibility-parity/support_export.json"
    )))
    .map_err(PlatformFitAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(PlatformFitAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in platform-fit accessibility parity export.
#[derive(Debug)]
pub enum PlatformFitAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<PlatformFitAccessibilityViolation>),
}

impl fmt::Display for PlatformFitAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "platform-fit accessibility parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "platform-fit accessibility parity export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for PlatformFitAccessibilityArtifactError {}

/// Validation failure for M05-1170 platform-fit accessibility parity packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformFitAccessibilityViolation {
    SchemaVersion {
        expected: u32,
        actual: u32,
    },
    RecordKind {
        expected: String,
        actual: String,
    },
    MissingIdentity,
    DuplicateId {
        id: String,
    },
    IncompleteRow {
        id: String,
    },
    MissingPrimaryDimension {
        id: String,
        dimension: M5PlatformFitClaimDimension,
    },
    MissingMandatoryLabel {
        id: String,
    },
    StructureHeavyMissingStructured {
        id: String,
    },
    ClaimOverAsserted {
        id: String,
    },
    WeakStateShownAsTrusted {
        id: String,
    },
    AssistiveTechStranded {
        id: String,
    },
    ExportRequiresRawPayload {
        id: String,
    },
    TruthDropped {
        id: String,
    },
    NarrowingDropsContextSilently {
        id: String,
    },
    MissingConsumerParity {
        id: String,
    },
    StrandedRow {
        id: String,
    },
    MissingFamilyCoverage {
        family: M5PlatformFitFamily,
    },
    MissingDimensionCoverage {
        dimension: M5PlatformFitClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5PlatformFitConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5PlatformFitA11yClaim,
    },
    TrustedHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5PlatformFitConsumerSurface,
    },
    SummaryMismatch,
    RawPlatformFitMaterialInExport,
}

impl PlatformFitAccessibilityViolation {
    /// Stable token for CLI / support handoff.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SchemaVersion { .. } => "schema_version",
            Self::RecordKind { .. } => "record_kind",
            Self::MissingIdentity => "missing_identity",
            Self::DuplicateId { .. } => "duplicate_id",
            Self::IncompleteRow { .. } => "incomplete_row",
            Self::MissingPrimaryDimension { .. } => "missing_primary_dimension",
            Self::MissingMandatoryLabel { .. } => "missing_mandatory_label",
            Self::StructureHeavyMissingStructured { .. } => "structure_heavy_missing_structured",
            Self::ClaimOverAsserted { .. } => "claim_over_asserted",
            Self::WeakStateShownAsTrusted { .. } => "weak_state_shown_as_trusted",
            Self::AssistiveTechStranded { .. } => "assistive_tech_stranded",
            Self::ExportRequiresRawPayload { .. } => "export_requires_raw_payload",
            Self::TruthDropped { .. } => "truth_dropped",
            Self::NarrowingDropsContextSilently { .. } => "narrowing_drops_context_silently",
            Self::MissingConsumerParity { .. } => "missing_consumer_parity",
            Self::StrandedRow { .. } => "stranded_row",
            Self::MissingFamilyCoverage { .. } => "missing_family_coverage",
            Self::MissingDimensionCoverage { .. } => "missing_dimension_coverage",
            Self::MissingConditionStateCoverage { .. } => "missing_condition_state_coverage",
            Self::MissingClaimTierCoverage { .. } => "missing_claim_tier_coverage",
            Self::TrustedHonestyUnproven => "trusted_honesty_unproven",
            Self::MissingConsumerSurfaceCoverage { .. } => "missing_consumer_surface_coverage",
            Self::SummaryMismatch => "summary_mismatch",
            Self::RawPlatformFitMaterialInExport => "raw_platform_fit_material_in_export",
        }
    }
}

impl fmt::Display for PlatformFitAccessibilityViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete accessibility row: {id}"),
            Self::MissingPrimaryDimension { id, dimension } => {
                write!(
                    f,
                    "row {id} does not model its family's primary dimension {}",
                    dimension.as_str()
                )
            }
            Self::MissingMandatoryLabel { id } => {
                write!(f, "row {id} drops a mandatory platform-fit label")
            }
            Self::StructureHeavyMissingStructured { id } => {
                write!(
                    f,
                    "structure-heavy row {id} does not render a structured modality"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts a trusted / reviewable surface for a weakened one, or narrows spuriously"
                )
            }
            Self::WeakStateShownAsTrusted { id } => {
                write!(
                    f,
                    "row {id} shows an unconfirmed-appearance / unconfirmed-credential / unconfirmed-input state as a trusted platform-fit surface"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / high-zoom / high-contrast / localization / CLI users from the canonical truth"
                )
            }
            Self::ExportRequiresRawPayload { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve meaning without leaking a raw payload"
                )
            }
            Self::TruthDropped { id } => {
                write!(
                    f,
                    "row {id} does not preserve shortcut / path / appearance / credential-wording / input-method truth across narrowing"
                )
            }
            Self::NarrowingDropsContextSilently { id } => {
                write!(
                    f,
                    "row {id} narrows a rendering surface without disclosing it"
                )
            }
            Self::MissingConsumerParity { id } => {
                write!(f, "row {id} is missing secondary consumer parity")
            }
            Self::StrandedRow { id } => write!(f, "row {id} is stranded (red) and may not ship"),
            Self::MissingFamilyCoverage { family } => {
                write!(
                    f,
                    "platform-fit family {family:?} is not certified in the packet"
                )
            }
            Self::MissingDimensionCoverage { dimension } => {
                write!(
                    f,
                    "claim dimension {} is not exercised in the packet",
                    dimension.as_str()
                )
            }
            Self::MissingConditionStateCoverage { state } => {
                write!(
                    f,
                    "condition state {} is not exercised in the packet",
                    state.as_str()
                )
            }
            Self::MissingClaimTierCoverage { claim } => {
                write!(
                    f,
                    "claim tier {} does not appear as an effective claim",
                    claim.as_str()
                )
            }
            Self::TrustedHonestyUnproven => {
                write!(
                    f,
                    "no unconfirmed-appearance / unconfirmed-credential / unconfirmed-input row is present to prove the trusted-honesty guarantee"
                )
            }
            Self::MissingConsumerSurfaceCoverage { surface } => {
                write!(
                    f,
                    "consumer surface {} does not ingest any row in the packet",
                    surface.as_str()
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawPlatformFitMaterialInExport => {
                write!(f, "export contains raw platform-fit material")
            }
        }
    }
}

impl Error for PlatformFitAccessibilityViolation {}

/// Whether a narrowed label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "degraded"
            | "narrowed"
            | "fallback"
            | "reduced"
            | "blocked"
            | "unresolved"
            | "partial"
            | "stale"
            | "incomplete"
            | "not comparable"
            | "restricted"
            | "collapsed"
            | "ellipsis"
            | "mixed"
            | "expired"
            | "inferred"
            | "unverified"
            | "trusted"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON. Mirrors the frozen platform-fit
/// matrix's own forbidden-material policy (see [`crate::m5_platform_fit_matrix`]) rather than 1162's stricter
/// list, because this lane reuses the matrix's `SecretStorageFellBackToPlaintextSilently` downgrade trigger
/// whose stable token legitimately contains the substring `secret`.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The canonical packet id for the checked-in stable export.
pub const PLATFORM_FIT_A11Y_PACKET_ID: &str = "m5-platform-fit-accessibility-parity:stable:0001";

/// Builds the canonical, checked-in platform-fit accessibility parity packet. This is the one source of truth
/// shared by the tests and the on-disk support export so both stay byte-aligned.
pub fn seeded_m5_platform_fit_a11y_packet() -> PlatformFitAccessibilityPacket {
    PlatformFitAccessibilityPacket::new(PlatformFitAccessibilityPacketInput {
        packet_id: PLATFORM_FIT_A11Y_PACKET_ID.to_owned(),
        as_of: "2026-07-13T00:00:00Z".to_owned(),
        matrix_ref: PLATFORM_FIT_A11Y_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:platform-fit-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5PlatformFitRequiredLabel> {
    M5PlatformFitRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> PlatformFitCopyExportParity {
    PlatformFitCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn condition(
    dimension: M5PlatformFitClaimDimension,
    state: M5PlatformFitConditionState,
) -> PlatformFitClaimConditionEntry {
    PlatformFitClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and the general product
/// UI — so the narrowed state always reaches headless field triage.
fn base_consumers(extra: &[M5PlatformFitConsumerSurface]) -> Vec<M5PlatformFitConsumerSurface> {
    let mut out = vec![
        M5PlatformFitConsumerSurface::SupportExport,
        M5PlatformFitConsumerSurface::ProductUi,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps full label
/// and summary parity on the narrower surfaces; a narrowed row discloses the reduced projection it drops
/// there.
fn surface_disclosures(
    labels: &[&str],
    state: PlatformFitNarrowingDisclosureState,
) -> Vec<PlatformFitRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        PlatformFitRenderingNarrowingDisclosure {
            rendering_surface: M5PlatformFitRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["native_chrome_pointer_affordance".to_owned()],
        },
        PlatformFitRenderingNarrowingDisclosure {
            rendering_surface: M5PlatformFitRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_appearance_transition".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<PlatformFitRenderingNarrowingDisclosure> {
    surface_disclosures(labels, PlatformFitNarrowingDisclosureState::ParityPreserved)
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced projection while
/// preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<PlatformFitRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        PlatformFitNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5PlatformFitRenderingSurface> {
    vec![
        M5PlatformFitRenderingSurface::DesktopFull,
        M5PlatformFitRenderingSurface::CliHeadless,
        M5PlatformFitRenderingSurface::SupportExport,
    ]
}

fn non_visual_modalities() -> Vec<M5PlatformFitFallbackModality> {
    vec![
        M5PlatformFitFallbackModality::List,
        M5PlatformFitFallbackModality::Textual,
        M5PlatformFitFallbackModality::Cli,
    ]
}

fn structured_modalities() -> Vec<M5PlatformFitFallbackModality> {
    vec![
        M5PlatformFitFallbackModality::Structured,
        M5PlatformFitFallbackModality::List,
        M5PlatformFitFallbackModality::Textual,
        M5PlatformFitFallbackModality::Cli,
    ]
}

const REACHABLE: PlatformFitNonVisualReachState =
    PlatformFitNonVisualReachState::ReachableAndLabeled;
const REDUCED: PlatformFitNonVisualReachState =
    PlatformFitNonVisualReachState::DisclosedReducedButReachable;

fn seeded_rows() -> Vec<PlatformFitAccessibilityRow> {
    vec![
        // Platform convention (window / menu / system chrome host-correct) — the platform-convention family
        // keeps window controls, menu-bar behavior, and system chrome host-correct without hiding a primary
        // action only in OS chrome, so it is a trusted platform-fit surface reachable on every surface with no
        // narrowing (green). Not structure-heavy: it exposes a flat list / textual / CLI path.
        PlatformFitAccessibilityRow {
            record_kind: PLATFORM_FIT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: PLATFORM_FIT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:platform-convention-window-menu-chrome-host-correct".to_owned(),
            platform_fit_family: M5PlatformFitFamily::PlatformConvention,
            source_family_schema_ref: M5PlatformFitFamily::PlatformConvention
                .canonical_domain_schema_ref()
                .to_owned(),
            platform_fit_context_ref: "shell:platform-convention:0001".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            localization_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: PlatformFitExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:platform-convention-window-menu-chrome-host-correct:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "platform_fit_identity",
                "semantic_role",
                "registry_reference",
                "window_menu_convention",
            ]),
            full_ready_claim: M5PlatformFitA11yClaim::TrustedPlatformFitSurface,
            claim_conditions: vec![condition(
                M5PlatformFitClaimDimension::PlatformConventionClarity,
                M5PlatformFitConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "platform_fit_identity",
                "semantic_role",
                "host_platform",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5PlatformFitConsumerSurface::ShellUi,
                M5PlatformFitConsumerSurface::DocsHelp,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.7 — Platform conventions / native desktop integration".to_owned(),
                PLATFORM_FIT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("platform-convention-window-menu-chrome-host-correct"),
        },
        // Shortcut notation (adapted from one registry with stable command IDs) — the shortcut-notation family
        // adapts modifier glyphs and accelerator labels from the one shortcut registry with stable command
        // IDs, so it is a self-sufficient reviewable platform-fit surface a user can inspect, but its narrower
        // non-visual traversal discloses a reduced high-zoom reflow walk of the dense glyph table (yellow).
        // Structure-heavy: its help-notation table binds to a flat list / textual path.
        PlatformFitAccessibilityRow {
            record_kind: PLATFORM_FIT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: PLATFORM_FIT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:shortcut-notation-adapts-from-one-registry".to_owned(),
            platform_fit_family: M5PlatformFitFamily::ShortcutNotation,
            source_family_schema_ref: M5PlatformFitFamily::ShortcutNotation
                .canonical_domain_schema_ref()
                .to_owned(),
            platform_fit_context_ref: "shell:shortcut-notation:0002".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REDUCED,
            high_contrast_reach: REACHABLE,
            localization_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: PlatformFitExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:shortcut-notation-adapts-from-one-registry:a11y".to_owned(),
            copy_export: copy_export(&[
                "platform_fit_identity",
                "semantic_role",
                "registry_reference",
                "shortcut_notation",
            ]),
            full_ready_claim: M5PlatformFitA11yClaim::ReviewablePlatformFitSurface,
            claim_conditions: vec![condition(
                M5PlatformFitClaimDimension::ShortcutNotationClarity,
                M5PlatformFitConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "platform_fit_identity",
                "semantic_role",
                "shortcut_notation",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5PlatformFitConsumerSurface::ShellUi,
                M5PlatformFitConsumerSurface::Onboarding,
            ]),
            source_refs: vec![
                "UX Style Guide §12.7 — Platform-native shortcut notation / command labels".to_owned(),
                PLATFORM_FIT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("shortcut-notation-adapts-from-one-registry"),
        },
        // File / path / reveal (localization partially disclosed) — the file-path-reveal family's reveal /
        // save terminology can only be partially disclosed for a locale, so it auto-narrows to a
        // path-terminology-disclosed projection that discloses the partial localization alongside the
        // last-known host-correct verb, never a mislabeled path / reveal verb shown as host-correct when its
        // localization is incomplete (yellow). Its localized traversal narrows the localization path to a
        // disclosed reduction. Structure-heavy: its path-reveal table binds to a flat list / textual path. A
        // partial localization disclosure is an honest disclosed-absence operation, not a trusted
        // overstatement.
        PlatformFitAccessibilityRow {
            record_kind: PLATFORM_FIT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: PLATFORM_FIT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:file-path-reveal-localization-disclosed-partial".to_owned(),
            platform_fit_family: M5PlatformFitFamily::FilePathReveal,
            source_family_schema_ref: M5PlatformFitFamily::FilePathReveal
                .canonical_domain_schema_ref()
                .to_owned(),
            platform_fit_context_ref: "settings:file-path-reveal:0003".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            localization_reach: REDUCED,
            cli_reach: REACHABLE,
            export_summary: PlatformFitExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:file-path-reveal-localization-disclosed-partial:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "platform_fit_identity",
                "semantic_role",
                "registry_reference",
                "path_reveal_verb",
            ]),
            full_ready_claim: M5PlatformFitA11yClaim::TrustedPlatformFitSurface,
            claim_conditions: vec![condition(
                M5PlatformFitClaimDimension::PathTerminologyClarity,
                M5PlatformFitConditionState::PathTerminologyDisclosedPartial,
            )],
            claim_narrow: Some(PlatformFitClaimAutoNarrow {
                narrowed_to: M5PlatformFitA11yClaim::PathTerminologyDisclosedProjection,
                binding_dimension: M5PlatformFitClaimDimension::PathTerminologyClarity,
                trigger: M5PlatformFitDowngradeTrigger::ProofStale,
                narrowed_label:
                    "This file / path / reveal / save terminology can only disclose a partial localization — shown as a path-terminology-disclosed projection that discloses the partial localization alongside the last-known host-correct reveal verb, never presenting a mislabeled path or reveal verb as host-correct when its localization is incomplete"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "platform_fit_identity",
                "semantic_role",
                "path_verb",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5PlatformFitConsumerSurface::CliExport,
                M5PlatformFitConsumerSurface::DocsHelp,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.7 — File / path / reveal / save terminology".to_owned(),
                PLATFORM_FIT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("file-path-reveal-localization-disclosed-partial"),
        },
        // Theme / contrast live change (live-apply unconfirmed) — the theme-contrast-live-change family's
        // live-apply response cannot be confirmed, so it auto-narrows to an appearance-response-unverified
        // projection that keeps the last-known appearance posture explicit, never a theme or contrast change
        // shown as applied live when it may not have applied or explained its fallback (yellow). Its
        // forced-colors response narrows the high-contrast path to a disclosed reduction.
        PlatformFitAccessibilityRow {
            record_kind: PLATFORM_FIT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: PLATFORM_FIT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:theme-contrast-live-apply-unconfirmed".to_owned(),
            platform_fit_family: M5PlatformFitFamily::ThemeContrastLiveChange,
            source_family_schema_ref: M5PlatformFitFamily::ThemeContrastLiveChange
                .canonical_domain_schema_ref()
                .to_owned(),
            platform_fit_context_ref: "settings:theme-contrast-live-change:0004".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REDUCED,
            localization_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: PlatformFitExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:theme-contrast-live-apply-unconfirmed:a11y".to_owned(),
            copy_export: copy_export(&[
                "platform_fit_identity",
                "semantic_role",
                "registry_reference",
                "appearance_posture",
            ]),
            full_ready_claim: M5PlatformFitA11yClaim::TrustedPlatformFitSurface,
            claim_conditions: vec![condition(
                M5PlatformFitClaimDimension::AppearanceResponseClarity,
                M5PlatformFitConditionState::AppearanceResponseUnconfirmed,
            )],
            claim_narrow: Some(PlatformFitClaimAutoNarrow {
                narrowed_to: M5PlatformFitA11yClaim::AppearanceResponseUnverifiedProjection,
                binding_dimension: M5PlatformFitClaimDimension::AppearanceResponseClarity,
                trigger:
                    M5PlatformFitDowngradeTrigger::ThemeOrContrastChangeDidNotApplyLiveOrExplainFallback,
                narrowed_label:
                    "This theme / contrast / accent / text-scale response cannot confirm that it applies live — shown as an appearance-response-unverified projection that keeps the last-known appearance posture explicit, never presenting a theme or contrast change as applied live when it may not have applied or explained its fallback"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "platform_fit_identity",
                "semantic_role",
                "appearance_posture",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5PlatformFitConsumerSurface::SettingsUi,
                M5PlatformFitConsumerSurface::ShellUi,
            ]),
            source_refs: vec![
                "UX Style Guide §12.7 — Live theme / contrast / accent / text-scale response".to_owned(),
                PLATFORM_FIT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("theme-contrast-live-apply-unconfirmed"),
        },
        // Credential-store wording (truthful, non-leaky posture unconfirmed) — the credential-store-wording
        // family's truthful, non-leaky posture cannot be confirmed, so it auto-narrows to a
        // credential-wording-unverified projection that keeps the last-known credential-store wording
        // explicit, never a credential-store message shown as truthful when it may hide a plaintext-storage
        // fallback (yellow). Not structure-heavy: it exposes a flat list / textual / CLI path.
        PlatformFitAccessibilityRow {
            record_kind: PLATFORM_FIT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: PLATFORM_FIT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:credential-store-wording-truthful-non-leaky-unconfirmed".to_owned(),
            platform_fit_family: M5PlatformFitFamily::CredentialStoreWording,
            source_family_schema_ref: M5PlatformFitFamily::CredentialStoreWording
                .canonical_domain_schema_ref()
                .to_owned(),
            platform_fit_context_ref: "auth:credential-store-wording:0005".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            localization_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: PlatformFitExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref:
                "summary:credential-store-wording-truthful-non-leaky-unconfirmed:a11y".to_owned(),
            copy_export: copy_export(&[
                "platform_fit_identity",
                "semantic_role",
                "registry_reference",
                "credential_store_wording",
            ]),
            full_ready_claim: M5PlatformFitA11yClaim::TrustedPlatformFitSurface,
            claim_conditions: vec![condition(
                M5PlatformFitClaimDimension::CredentialWordingClarity,
                M5PlatformFitConditionState::CredentialWordingUnconfirmed,
            )],
            claim_narrow: Some(PlatformFitClaimAutoNarrow {
                narrowed_to: M5PlatformFitA11yClaim::CredentialWordingUnverifiedProjection,
                binding_dimension: M5PlatformFitClaimDimension::CredentialWordingClarity,
                trigger: M5PlatformFitDowngradeTrigger::SecretStorageFellBackToPlaintextSilently,
                narrowed_label:
                    "This credential-store wording cannot confirm that it stays truthful and non-leaky — shown as a credential-wording-unverified projection that keeps the last-known credential-store wording explicit, never presenting stored credentials as more protected than they are or hiding a plaintext-storage fallback"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "platform_fit_identity",
                "semantic_role",
                "credential_store_wording",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5PlatformFitConsumerSurface::AuthUi,
                M5PlatformFitConsumerSurface::SettingsUi,
            ]),
            source_refs: vec![
                "UX Style Guide §12.7 — Credential-store wording (truthful, non-leaky)".to_owned(),
                PLATFORM_FIT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("credential-store-wording-truthful-non-leaky-unconfirmed"),
        },
        // Input method (text / trust fidelity unconfirmed) — the input-method family's IME / dead-key / AltGr
        // / dictation / emoji / layout-switch text and trust fidelity cannot be confirmed, so it auto-narrows
        // to an input-fidelity-unverified projection that keeps the last-known input-method state explicit,
        // never an input flow shown as faithful when it may corrupt text or trust semantics (yellow).
        // Structure-heavy: its input-composition table binds to a flat list / textual path.
        PlatformFitAccessibilityRow {
            record_kind: PLATFORM_FIT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: PLATFORM_FIT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:input-method-text-trust-fidelity-unconfirmed".to_owned(),
            platform_fit_family: M5PlatformFitFamily::InputMethod,
            source_family_schema_ref: M5PlatformFitFamily::InputMethod
                .canonical_domain_schema_ref()
                .to_owned(),
            platform_fit_context_ref: "input:input-method:0006".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            localization_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: PlatformFitExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:input-method-text-trust-fidelity-unconfirmed:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "platform_fit_identity",
                "semantic_role",
                "registry_reference",
                "input_method_state",
            ]),
            full_ready_claim: M5PlatformFitA11yClaim::TrustedPlatformFitSurface,
            claim_conditions: vec![condition(
                M5PlatformFitClaimDimension::InputFidelityClarity,
                M5PlatformFitConditionState::InputFidelityUnconfirmed,
            )],
            claim_narrow: Some(PlatformFitClaimAutoNarrow {
                narrowed_to: M5PlatformFitA11yClaim::InputFidelityUnverifiedProjection,
                binding_dimension: M5PlatformFitClaimDimension::InputFidelityClarity,
                trigger: M5PlatformFitDowngradeTrigger::InputMethodCorruptedTextOrTrust,
                narrowed_label:
                    "This input method cannot confirm that IME, dead keys, AltGr, dictation, emoji, and layout switching preserve text and trust fidelity — shown as an input-fidelity-unverified projection that keeps the last-known input-method state explicit, never presenting an input flow as faithful when it may corrupt text or trust semantics"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "platform_fit_identity",
                "semantic_role",
                "input_method_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5PlatformFitConsumerSurface::InputUi,
                M5PlatformFitConsumerSurface::Onboarding,
            ]),
            source_refs: vec![
                "UX Style Guide §12.7 — IME / dead-key / AltGr / dictation / emoji / layout input".to_owned(),
                PLATFORM_FIT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("input-method-text-trust-fidelity-unconfirmed"),
        },
    ]
}
