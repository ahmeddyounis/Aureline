//! Portable, round-trip-safe token-overlay descriptors for M5 appearance
//! customization.
//!
//! The M5 depth lanes let users, profiles, workspaces, managed policies,
//! extensions, and imported themes override design tokens on top of the active
//! theme package. Before this lane those overrides lived as a flat settings
//! fragment: which scope won was inferred from the rendered pixels, an
//! unsupported token quietly disappeared on export or sync, and a support
//! reviewer had nothing concrete to name when a customer asked "why is my
//! accent the wrong colour after I moved machines?". That makes appearance
//! customization both support-hostile and migration-hostile.
//!
//! This module promotes the per-token overlay state frozen in
//! `schemas/design/token_overlay.schema.json` into a **portable, inspectable,
//! downgrade-safe object**:
//!
//! - [`TokenOverrideEntry`] — one override: its token, family, declared scope,
//!   value state (inherited / overridden / deprecated / unmapped), validation
//!   result, **provenance**, **portability flags**, disclosed-downgrade class,
//!   and an explicit fallback chain. The entry mints no parallel per-token
//!   vocabulary; it re-exports the canonical design `token_overlay_record`
//!   vocabulary by token and links the canonical record by id.
//! - [`ScopeOverlay`] — the token-overlay descriptor for one override scope
//!   (`user_global`, `profile`, `workspace`, `policy_managed`,
//!   `extension_contributed`, `imported_theme`, or the `theme_package_default`
//!   base). Overlays stay structured per scope; they are never flattened into
//!   an opaque profile blob.
//! - [`ResolvedToken`] — the winning-versus-shadowed resolution for one token:
//!   which scope's value won, which scopes were shadowed, and a reviewable
//!   sentence explaining why. Precedence is explicit, not pixel-inferred.
//! - [`RoundTripStage`] / [`RoundTripEntryTrace`] / [`RoundTripProof`] — the
//!   export / import / sync round trip. Every portable entry is traced across
//!   the channels; an unsupported token survives as an inert or downgraded
//!   entry with a disclosed downgrade note instead of being silently dropped,
//!   rewritten, or treated as fully supported.
//! - [`TokenOverlayPortabilityReport`] — the canonical truth object binding the
//!   per-scope overlays, the resolution table, and the round-trip proof, with a
//!   blocking-finding summary that the live shell appearance inspector, the
//!   docs/help and support-export surfaces, the sync/import flows, and the CI
//!   gate all reuse.
//!
//! Acceptance invariants enforced by the validator:
//!
//! 1. Every override entry is scope-explicit: an overridden / deprecated /
//!    unmapped entry declares a real override scope (never the theme-package
//!    default), carries provenance and portability flags, and rides a non-empty
//!    fallback chain. An inherited entry resolves to the theme-package default.
//! 2. An unsupported entry survives as inert or downgraded with a disclosed
//!    downgrade note: an `unmapped` entry cites its source slot and resolves to
//!    `inert_unresolved`, a `deprecated` entry cites its replacement, and an
//!    entry that claims full portability carries no downgrade. An unsupported
//!    entry treated as fully supported is a blocker.
//! 3. Resolution is inspectable: every resolved token names exactly one winning
//!    scope — the highest-precedence scope that contributed an entry — and lists
//!    every shadowed entry, so users and support can explain which value won.
//! 4. The round trip is lossless for portable entries: no export / import /
//!    sync stage drops or rewrites an entry, scope is preserved across the
//!    round trip, and any downgrade that survives the round trip is disclosed.
//! 5. Overlays stay structured: every overlay declares its scope and structured
//!    entries; an overlay flattened into an opaque blob, or one whose entries
//!    disagree with its scope, is a blocker.
//! 6. `report_clean` holds exactly when the audit carries no blocking finding.
//!
//! All identifiers, refs, and label strings are deterministic so the
//! checked-in fixtures under `fixtures/ux/m5/token-overlay-sync-import/` are
//! bit-for-bit equal to the seeded report returned by
//! [`seeded_token_overlay_portability`].

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Schema version exported with every token-overlay portability record.
pub const TOKEN_OVERLAY_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every token-overlay record.
pub const TOKEN_OVERLAY_SHARED_CONTRACT_REF: &str = "shell:m5_token_overlays:v1";

/// Stable record kind for [`TokenOverlayPortabilityReport`] payloads.
pub const TOKEN_OVERLAY_REPORT_RECORD_KIND: &str =
    "shell_m5_token_overlay_portability_report_record";

/// Stable record kind for [`ScopeOverlay`] payloads.
pub const TOKEN_OVERLAY_SCOPE_OVERLAY_RECORD_KIND: &str =
    "shell_m5_token_overlay_scope_overlay_record";

/// Stable record kind for [`TokenOverrideEntry`] payloads.
pub const TOKEN_OVERLAY_OVERRIDE_ENTRY_RECORD_KIND: &str =
    "shell_m5_token_overlay_override_entry_record";

/// Stable record kind for [`ResolvedToken`] payloads.
pub const TOKEN_OVERLAY_RESOLVED_TOKEN_RECORD_KIND: &str =
    "shell_m5_token_overlay_resolved_token_record";

/// Stable record kind for [`RoundTripStage`] payloads.
pub const TOKEN_OVERLAY_ROUND_TRIP_STAGE_RECORD_KIND: &str =
    "shell_m5_token_overlay_round_trip_stage_record";

/// Stable record kind for [`RoundTripEntryTrace`] payloads.
pub const TOKEN_OVERLAY_ROUND_TRIP_TRACE_RECORD_KIND: &str =
    "shell_m5_token_overlay_round_trip_entry_trace_record";

/// Stable record kind for [`RoundTripProof`] payloads.
pub const TOKEN_OVERLAY_ROUND_TRIP_PROOF_RECORD_KIND: &str =
    "shell_m5_token_overlay_round_trip_proof_record";

/// Stable record kind for [`TokenOverlaySupportExport`] payloads.
pub const TOKEN_OVERLAY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_token_overlay_portability_support_export_record";

/// Stable report id quoted across surfaces.
pub const TOKEN_OVERLAY_REPORT_ID: &str = "shell:m5_token_overlays:portability:v1";

/// Stable support-export id quoted in the published wrapper.
pub const TOKEN_OVERLAY_SUPPORT_EXPORT_ID: &str = "support-export:m5-token-overlays:001";

/// Source schema ref for the canonical token-overlay portability contract.
pub const TOKEN_OVERLAY_SOURCE_SCHEMA_REF: &str = "schemas/ux/token-overlay.schema.json";

/// Schema ref for the canonical per-token overlay-state record this lane
/// re-exports its overlay vocabulary from instead of re-declaring.
pub const TOKEN_OVERLAY_CANONICAL_RECORD_SCHEMA_REF: &str =
    "schemas/design/token_overlay.schema.json";

/// The live appearance session these overlays apply to.
pub const TOKEN_OVERLAY_APPEARANCE_SESSION_REF: &str = "appearance-session:primary";

/// Path of the published markdown audit artifact.
pub const TOKEN_OVERLAY_PUBLISHED_REPORT_REF: &str =
    "artifacts/ux/m5/token-overlay-roundtrip/m5_token_overlay_roundtrip_audit.md";

/// Path of the published companion doc.
pub const TOKEN_OVERLAY_PUBLISHED_DOC_REF: &str = "docs/m5/token-overlays-and-scope.md";

/// Generation timestamp captured in every seeded record.
const GENERATED_AT: &str = "2026-06-17T00:00:00Z";

/// The override scope a token-overlay entry was authored in. Re-exported from
/// the canonical `overlay_scope_class` vocabulary without modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverrideScope {
    /// The active theme package's value — the inherited base.
    ThemePackageDefault,
    /// An imported third-party theme.
    ImportedTheme,
    /// An extension-contributed appearance override.
    ExtensionContributed,
    /// A user-global override.
    UserGlobal,
    /// A profile-scoped override.
    Profile,
    /// A workspace-scoped override.
    Workspace,
    /// A managed-policy override (the hard cap).
    PolicyManaged,
}

impl OverrideScope {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ThemePackageDefault => "theme_package_default",
            Self::ImportedTheme => "imported_theme",
            Self::ExtensionContributed => "extension_contributed",
            Self::UserGlobal => "user_global",
            Self::Profile => "profile",
            Self::Workspace => "workspace",
            Self::PolicyManaged => "policy_managed",
        }
    }

    /// Reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::ThemePackageDefault => "Theme package default",
            Self::ImportedTheme => "Imported theme",
            Self::ExtensionContributed => "Extension contributed",
            Self::UserGlobal => "User global",
            Self::Profile => "Profile",
            Self::Workspace => "Workspace",
            Self::PolicyManaged => "Policy managed",
        }
    }

    /// Precedence rank: a higher rank wins when more than one scope contributes
    /// an entry for the same token. A managed policy is the hard cap; the
    /// theme-package default is the lowest base.
    pub const fn precedence_rank(self) -> u32 {
        match self {
            Self::ThemePackageDefault => 0,
            Self::ImportedTheme => 10,
            Self::ExtensionContributed => 20,
            Self::UserGlobal => 30,
            Self::Profile => 40,
            Self::Workspace => 50,
            Self::PolicyManaged => 100,
        }
    }

    /// `true` for the real override scopes — everything except the inherited
    /// theme-package default.
    pub const fn is_override(self) -> bool {
        !matches!(self, Self::ThemePackageDefault)
    }
}

/// The per-token value state. Re-exported from the canonical `value_state_class`
/// vocabulary without modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValueState {
    /// The effective value is the active theme package's value.
    Inherited,
    /// A higher-priority overlay supplied a different value.
    Overridden,
    /// The overlay points at a deprecated token; a replacement is mandatory.
    Deprecated,
    /// The overlay references a slot with no current target token; an inert
    /// placeholder is required.
    Unmapped,
}

impl ValueState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inherited => "inherited",
            Self::Overridden => "overridden",
            Self::Deprecated => "deprecated",
            Self::Unmapped => "unmapped",
        }
    }
}

/// The validation result for an overlay or entry. Re-exported from the canonical
/// `validation_state_class` vocabulary without modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverlayValidationState {
    /// Fully valid.
    Valid,
    /// Valid, with non-blocking warnings (e.g. a deprecated alias).
    ValidWithWarnings,
    /// References an unresolved slot; survives as an inert placeholder.
    InertUnresolved,
    /// Blocked by a managed policy.
    BlockedPolicy,
    /// Rolled back to a prior state.
    RolledBack,
}

impl OverlayValidationState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Valid => "valid",
            Self::ValidWithWarnings => "valid_with_warnings",
            Self::InertUnresolved => "inert_unresolved",
            Self::BlockedPolicy => "blocked_policy",
            Self::RolledBack => "rolled_back",
        }
    }
}

/// Where an override came from. The provenance an entry carries so support and
/// migration flows can name how a value got here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceClass {
    /// Authored by a user in the product.
    AuthoredInProduct,
    /// Supplied by an imported theme package.
    ImportedFromThemePackage,
    /// Contributed by an installed extension.
    ContributedByExtension,
    /// Applied by a managed policy.
    AppliedByPolicy,
    /// Migrated forward from a legacy settings fragment.
    MigratedFromLegacySettings,
    /// Synced from another device.
    SyncedFromDevice,
}

impl ProvenanceClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoredInProduct => "authored_in_product",
            Self::ImportedFromThemePackage => "imported_from_theme_package",
            Self::ContributedByExtension => "contributed_by_extension",
            Self::AppliedByPolicy => "applied_by_policy",
            Self::MigratedFromLegacySettings => "migrated_from_legacy_settings",
            Self::SyncedFromDevice => "synced_from_device",
        }
    }
}

/// How an entry behaves under export / import / sync.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortabilityClass {
    /// Survives the round trip unchanged.
    FullyPortable,
    /// Survives the round trip, but downgraded on targets that do not support
    /// it fully.
    PortableWithDowngrade,
    /// Deliberately stays local (e.g. a managed-policy cap or a workspace cap);
    /// it is disclosed as non-portable rather than silently dropped.
    ScopeLocalNonPortable,
    /// Inherited from the theme package and re-resolved from the package on
    /// import rather than serialized as an override.
    RidesThemePackage,
}

impl PortabilityClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyPortable => "fully_portable",
            Self::PortableWithDowngrade => "portable_with_downgrade",
            Self::ScopeLocalNonPortable => "scope_local_non_portable",
            Self::RidesThemePackage => "rides_theme_package",
        }
    }
}

/// The single disclosed-downgrade vocabulary. `None` means the entry is fully
/// supported and preserved; every other value is a disclosed downgrade that
/// keeps an unsupported token alive instead of dropping it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeClass {
    /// No downgrade — fully supported and preserved.
    None,
    /// The target does not support the token; kept as an inert placeholder.
    InertUnsupportedToken,
    /// Points at a deprecated token awaiting replacement.
    DeprecatedAliasPendingReplacement,
    /// A managed policy capped the value on import.
    PolicyCapped,
    /// The scope could not be preserved on the target; demoted with disclosure.
    ScopeDemoted,
    /// The value form is unsupported on the target; kept as a placeholder.
    ValueUnsupportedKeptPlaceholder,
}

impl DowngradeClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::InertUnsupportedToken => "inert_unsupported_token",
            Self::DeprecatedAliasPendingReplacement => "deprecated_alias_pending_replacement",
            Self::PolicyCapped => "policy_capped",
            Self::ScopeDemoted => "scope_demoted",
            Self::ValueUnsupportedKeptPlaceholder => "value_unsupported_kept_placeholder",
        }
    }

    /// `true` when this is a disclosed downgrade (anything but [`Self::None`]).
    pub const fn is_downgrade(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// The kind of step in a token's fallback chain. Re-exported from the canonical
/// `fallback_step_kind` vocabulary without modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackStepKind {
    /// The active theme package's default value.
    ThemePackageDefault,
    /// A scope override.
    ScopeOverride,
    /// A deprecated alias awaiting replacement.
    DeprecatedAlias,
    /// An inert placeholder for an unmapped slot.
    InertPlaceholder,
}

impl FallbackStepKind {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ThemePackageDefault => "theme_package_default",
            Self::ScopeOverride => "scope_override",
            Self::DeprecatedAlias => "deprecated_alias",
            Self::InertPlaceholder => "inert_placeholder",
        }
    }
}

/// The design-token family an entry targets. Re-exported from the canonical
/// `token_family_class` vocabulary without modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenFamily {
    /// Brand colour.
    ColorBrand,
    /// Functional accent colour.
    ColorFunctionalAccent,
    /// Neutral colour.
    ColorNeutral,
    /// State colour.
    ColorState,
    /// Semantic theme colour.
    ColorSemanticTheme,
    /// Syntax colour.
    ColorSyntax,
    /// Diff colour.
    ColorDiff,
    /// Chart colour.
    ColorChart,
    /// Typography role.
    TypographyRole,
    /// Typography scale.
    TypographyScale,
    /// Text rule.
    TextRule,
    /// Spacing.
    Spacing,
    /// Sizing.
    Sizing,
    /// Radius.
    Radius,
    /// Border stroke.
    BorderStroke,
    /// Elevation.
    Elevation,
    /// Opacity / scrim.
    OpacityScrim,
    /// Layer / portal order.
    LayerPortalOrder,
    /// Motion duration.
    MotionDuration,
    /// Motion easing.
    MotionEasing,
    /// Motion restriction.
    MotionRestriction,
    /// Density.
    Density,
    /// Icon treatment.
    IconTreatment,
    /// Semantic status.
    SemanticStatus,
    /// Trust visual state.
    TrustVisualState,
}

impl TokenFamily {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ColorBrand => "color_brand",
            Self::ColorFunctionalAccent => "color_functional_accent",
            Self::ColorNeutral => "color_neutral",
            Self::ColorState => "color_state",
            Self::ColorSemanticTheme => "color_semantic_theme",
            Self::ColorSyntax => "color_syntax",
            Self::ColorDiff => "color_diff",
            Self::ColorChart => "color_chart",
            Self::TypographyRole => "typography_role",
            Self::TypographyScale => "typography_scale",
            Self::TextRule => "text_rule",
            Self::Spacing => "spacing",
            Self::Sizing => "sizing",
            Self::Radius => "radius",
            Self::BorderStroke => "border_stroke",
            Self::Elevation => "elevation",
            Self::OpacityScrim => "opacity_scrim",
            Self::LayerPortalOrder => "layer_portal_order",
            Self::MotionDuration => "motion_duration",
            Self::MotionEasing => "motion_easing",
            Self::MotionRestriction => "motion_restriction",
            Self::Density => "density",
            Self::IconTreatment => "icon_treatment",
            Self::SemanticStatus => "semantic_status",
            Self::TrustVisualState => "trust_visual_state",
        }
    }
}

/// One export / import / sync channel the round-trip proof exercises.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoundTripChannel {
    /// Serialize the portable overlay set into an export bundle.
    ExportBundle,
    /// Import the bundle into a (possibly reduced) target.
    ImportBundle,
    /// Push the imported overlay set into cross-device sync.
    SyncPush,
    /// Pull the overlay set back from cross-device sync.
    SyncPull,
}

impl RoundTripChannel {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExportBundle => "export_bundle",
            Self::ImportBundle => "import_bundle",
            Self::SyncPush => "sync_push",
            Self::SyncPull => "sync_pull",
        }
    }
}

/// What happened to one entry across the round trip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryDisposition {
    /// Survived the round trip unchanged.
    Preserved,
    /// Survived the round trip, downgraded with a disclosed note.
    Downgraded,
    /// Silently lost — always a blocker.
    Dropped,
    /// Silently rewritten to a different value or scope — always a blocker.
    Rewritten,
}

impl EntryDisposition {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::Downgraded => "downgraded",
            Self::Dropped => "dropped",
            Self::Rewritten => "rewritten",
        }
    }

    /// `true` when the entry survived (preserved or downgraded).
    pub const fn survived(self) -> bool {
        matches!(self, Self::Preserved | Self::Downgraded)
    }

    /// `true` when this disposition is a lossy round-trip violation.
    pub const fn is_lossy(self) -> bool {
        matches!(self, Self::Dropped | Self::Rewritten)
    }
}

/// The portability flags an entry carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortabilityFlags {
    /// Summary portability class.
    pub portability_class: PortabilityClass,
    /// `true` when the entry is written into export bundles.
    pub exportable: bool,
    /// `true` when the entry is carried across cross-device sync.
    pub syncable: bool,
    /// `true` when the entry survives a target that does not support the token
    /// (as an inert or downgraded entry rather than being dropped).
    pub survives_unsupported_target: bool,
}

/// One step in a token's fallback chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FallbackStep {
    /// Stable step id.
    pub step_id: String,
    /// Zero-based index in the chain.
    pub step_index: u32,
    /// The scope this step resolves from.
    pub step_scope: OverrideScope,
    /// The kind of step.
    pub step_kind: FallbackStepKind,
    /// The token this step resolves to, when any (null for inert placeholders).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_token_ref: Option<String>,
    /// `true` when this step supplied the effective value.
    pub applied: bool,
}

/// One token-overlay override entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenOverrideEntry {
    /// Record discriminator.
    pub record_kind: String,
    /// Stable entry id.
    pub entry_id: String,
    /// The token this entry targets.
    pub token_ref: String,
    /// The design-token family.
    pub token_family: TokenFamily,
    /// The scope this entry was authored in.
    pub declared_scope: OverrideScope,
    /// The per-token value state.
    pub value_state: ValueState,
    /// The validation result.
    pub validation_state: OverlayValidationState,
    /// Where this override came from.
    pub provenance: ProvenanceClass,
    /// The portability flags.
    pub portability: PortabilityFlags,
    /// The disclosed-downgrade class (`none` when fully supported).
    pub downgrade_class: DowngradeClass,
    /// The explicit fallback chain.
    pub fallback_chain: Vec<FallbackStep>,
    /// The replacement token a deprecated entry must cite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_replacement_ref: Option<String>,
    /// The source slot an unmapped entry must cite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmapped_source_slot_ref: Option<String>,
    /// The canonical per-token overlay-state record this entry projects, by id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_overlay_record_ref: Option<String>,
    /// A reviewable sentence explaining the entry's state and downgrade.
    pub explanation: String,
    /// Blocking findings detected for this entry.
    pub blocking_findings: Vec<TokenOverlayBlockingFinding>,
}

/// One scope's token-overlay descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeOverlay {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable overlay id.
    pub overlay_id: String,
    /// The scope this overlay carries.
    pub scope: OverrideScope,
    /// The live appearance session these overrides apply to.
    pub appearance_session_ref: String,
    /// `true` when the overlay keeps structured per-token entries (never an
    /// opaque blob).
    pub structured: bool,
    /// The aggregate validation result for the overlay.
    pub validation_state: OverlayValidationState,
    /// The override entries, sorted by `entry_id`.
    pub entries: Vec<TokenOverrideEntry>,
    /// Number of entries in the overlay.
    pub entry_count: usize,
    /// Number of entries carrying a disclosed downgrade.
    pub downgraded_count: usize,
    /// Blocking findings detected for this overlay.
    pub blocking_findings: Vec<TokenOverlayBlockingFinding>,
}

/// One shadowed entry in a token's resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShadowedEntry {
    /// The scope whose entry was shadowed.
    pub scope: OverrideScope,
    /// The shadowed entry id.
    pub entry_ref: String,
    /// The shadowed value state.
    pub value_state: ValueState,
    /// A reviewable sentence explaining why this entry lost.
    pub reason: String,
}

/// The winning-versus-shadowed resolution for one token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedToken {
    /// Record discriminator.
    pub record_kind: String,
    /// The token this resolution covers.
    pub token_ref: String,
    /// The design-token family.
    pub token_family: TokenFamily,
    /// The scope whose value won.
    pub winning_scope: OverrideScope,
    /// The winning entry id.
    pub winning_entry_ref: String,
    /// The winning value state.
    pub winning_value_state: ValueState,
    /// The shadowed entries, sorted by descending precedence then `entry_ref`.
    pub shadowed: Vec<ShadowedEntry>,
    /// A reviewable sentence explaining which value won and why.
    pub precedence_explained: String,
    /// Blocking findings detected for this resolution.
    pub blocking_findings: Vec<TokenOverlayBlockingFinding>,
}

/// One stage of the round trip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoundTripStage {
    /// Record discriminator.
    pub record_kind: String,
    /// Stable stage id.
    pub stage_id: String,
    /// Zero-based stage index.
    pub sequence_index: u32,
    /// The channel exercised by this stage.
    pub channel: RoundTripChannel,
    /// The token-support profile of the stage's target.
    pub target_support_profile: String,
    /// Number of entries entering the stage.
    pub input_entry_count: usize,
    /// Number of entries leaving the stage.
    pub output_entry_count: usize,
    /// Number preserved unchanged.
    pub preserved_count: usize,
    /// Number downgraded with disclosure.
    pub downgraded_count: usize,
    /// Number silently dropped (must be zero).
    pub dropped_count: usize,
    /// Number silently rewritten (must be zero).
    pub rewritten_count: usize,
    /// `true` when scope is preserved across the stage.
    pub scope_preserved: bool,
    /// Blocking findings detected for this stage.
    pub blocking_findings: Vec<TokenOverlayBlockingFinding>,
}

/// One entry traced across the round trip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoundTripEntryTrace {
    /// Record discriminator.
    pub record_kind: String,
    /// The traced entry id.
    pub entry_ref: String,
    /// The traced token.
    pub token_ref: String,
    /// The scope the entry started in.
    pub origin_scope: OverrideScope,
    /// The scope the entry ended in (must equal `origin_scope`).
    pub final_scope: OverrideScope,
    /// What happened to the entry across the round trip.
    pub disposition: EntryDisposition,
    /// The disclosed-downgrade class, when the entry was downgraded.
    pub downgrade_class: DowngradeClass,
    /// The channels the entry traversed, in order.
    pub channels_traversed: Vec<RoundTripChannel>,
    /// `true` when the entry survived the round trip.
    pub survived: bool,
    /// A reviewable sentence explaining the trace.
    pub explanation: String,
    /// Blocking findings detected for this trace.
    pub blocking_findings: Vec<TokenOverlayBlockingFinding>,
}

/// The export / import / sync round-trip proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoundTripProof {
    /// Record discriminator.
    pub record_kind: String,
    /// Stable proof id.
    pub proof_id: String,
    /// The round-trip stages, sorted by `sequence_index`.
    pub stages: Vec<RoundTripStage>,
    /// The per-entry traces, sorted by `entry_ref`.
    pub entry_traces: Vec<RoundTripEntryTrace>,
    /// Number of portable entries traced.
    pub portable_entry_count: usize,
    /// Number of unsupported entries preserved as inert or downgraded.
    pub unsupported_preserved_count: usize,
    /// `true` when no stage or trace dropped, rewrote, or lost the scope of an
    /// entry.
    pub lossless: bool,
    /// Blocking findings detected for the proof.
    pub blocking_findings: Vec<TokenOverlayBlockingFinding>,
}

/// A blocking finding detected by the token-overlay portability audit.
///
/// Every variant is always blocking: a clean audit carries none. The owning
/// object ref (`entry_ref`, `overlay_ref`, `token_ref`, or `stage_ref`) is
/// quoted so support, diagnostics, and golden-evidence flows can pivot straight
/// to the object that flagged the problem.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum TokenOverlayBlockingFinding {
    /// An entry carries no explanation.
    EntryMissingExplanation {
        /// Entry id.
        entry_ref: String,
    },
    /// An entry rides an empty fallback chain.
    EntryFallbackChainEmpty {
        /// Entry id.
        entry_ref: String,
    },
    /// An override entry claims the inherited theme-package default scope, or an
    /// inherited entry claims an override scope.
    EntryScopeNotExplicit {
        /// Entry id.
        entry_ref: String,
    },
    /// A deprecated entry cites no replacement.
    EntryDeprecatedWithoutReplacement {
        /// Entry id.
        entry_ref: String,
    },
    /// An unmapped entry cites no source slot or does not resolve to an inert
    /// placeholder.
    EntryUnmappedWithoutPlaceholder {
        /// Entry id.
        entry_ref: String,
    },
    /// An unsupported entry is treated as fully supported (no disclosed
    /// downgrade).
    EntryUnsupportedTreatedAsSupported {
        /// Entry id.
        entry_ref: String,
    },
    /// An entry's portability flags disagree with its downgrade class.
    EntryPortabilityInconsistent {
        /// Entry id.
        entry_ref: String,
    },
    /// An overlay is flattened into an opaque blob.
    OverlayFlattenedToOpaqueBlob {
        /// Overlay id.
        overlay_ref: String,
    },
    /// An overlay carries no entries.
    OverlayHasNoEntries {
        /// Overlay id.
        overlay_ref: String,
    },
    /// An entry's declared scope disagrees with its overlay's scope.
    OverlayScopeMismatch {
        /// Overlay id.
        overlay_ref: String,
        /// The mismatched entry id.
        entry_ref: String,
    },
    /// A resolved token names no winner.
    ResolvedTokenNoWinner {
        /// Token ref.
        token_ref: String,
    },
    /// A resolved token's winner is not the highest-precedence scope.
    ResolvedTokenWrongWinner {
        /// Token ref.
        token_ref: String,
    },
    /// A resolved token does not list every shadowed entry.
    ResolvedTokenShadowedNotInspectable {
        /// Token ref.
        token_ref: String,
    },
    /// A round-trip stage dropped one or more entries.
    RoundTripStageDroppedEntries {
        /// Stage id.
        stage_ref: String,
    },
    /// A round-trip stage rewrote one or more entries.
    RoundTripStageRewroteEntries {
        /// Stage id.
        stage_ref: String,
    },
    /// A traced entry was dropped across the round trip.
    RoundTripEntryDropped {
        /// Entry id.
        entry_ref: String,
    },
    /// A traced entry was rewritten across the round trip.
    RoundTripEntryRewritten {
        /// Entry id.
        entry_ref: String,
    },
    /// A traced entry lost its scope across the round trip.
    RoundTripScopeLost {
        /// Entry id.
        entry_ref: String,
    },
    /// A traced entry was downgraded without a disclosed downgrade note.
    RoundTripDowngradeNotDisclosed {
        /// Entry id.
        entry_ref: String,
    },
}

impl TokenOverlayBlockingFinding {
    /// Stable class token quoted in summaries and the CI gate.
    pub fn class_token(&self) -> &'static str {
        match self {
            Self::EntryMissingExplanation { .. } => "entry_missing_explanation",
            Self::EntryFallbackChainEmpty { .. } => "entry_fallback_chain_empty",
            Self::EntryScopeNotExplicit { .. } => "entry_scope_not_explicit",
            Self::EntryDeprecatedWithoutReplacement { .. } => {
                "entry_deprecated_without_replacement"
            }
            Self::EntryUnmappedWithoutPlaceholder { .. } => "entry_unmapped_without_placeholder",
            Self::EntryUnsupportedTreatedAsSupported { .. } => {
                "entry_unsupported_treated_as_supported"
            }
            Self::EntryPortabilityInconsistent { .. } => "entry_portability_inconsistent",
            Self::OverlayFlattenedToOpaqueBlob { .. } => "overlay_flattened_to_opaque_blob",
            Self::OverlayHasNoEntries { .. } => "overlay_has_no_entries",
            Self::OverlayScopeMismatch { .. } => "overlay_scope_mismatch",
            Self::ResolvedTokenNoWinner { .. } => "resolved_token_no_winner",
            Self::ResolvedTokenWrongWinner { .. } => "resolved_token_wrong_winner",
            Self::ResolvedTokenShadowedNotInspectable { .. } => {
                "resolved_token_shadowed_not_inspectable"
            }
            Self::RoundTripStageDroppedEntries { .. } => "round_trip_stage_dropped_entries",
            Self::RoundTripStageRewroteEntries { .. } => "round_trip_stage_rewrote_entries",
            Self::RoundTripEntryDropped { .. } => "round_trip_entry_dropped",
            Self::RoundTripEntryRewritten { .. } => "round_trip_entry_rewritten",
            Self::RoundTripScopeLost { .. } => "round_trip_scope_lost",
            Self::RoundTripDowngradeNotDisclosed { .. } => "round_trip_downgrade_not_disclosed",
        }
    }

    /// The owning object ref (entry, overlay, token, or stage).
    pub fn subject_ref(&self) -> &str {
        match self {
            Self::EntryMissingExplanation { entry_ref }
            | Self::EntryFallbackChainEmpty { entry_ref }
            | Self::EntryScopeNotExplicit { entry_ref }
            | Self::EntryDeprecatedWithoutReplacement { entry_ref }
            | Self::EntryUnmappedWithoutPlaceholder { entry_ref }
            | Self::EntryUnsupportedTreatedAsSupported { entry_ref }
            | Self::EntryPortabilityInconsistent { entry_ref }
            | Self::OverlayScopeMismatch { entry_ref, .. }
            | Self::RoundTripEntryDropped { entry_ref }
            | Self::RoundTripEntryRewritten { entry_ref }
            | Self::RoundTripScopeLost { entry_ref }
            | Self::RoundTripDowngradeNotDisclosed { entry_ref } => entry_ref,
            Self::OverlayFlattenedToOpaqueBlob { overlay_ref }
            | Self::OverlayHasNoEntries { overlay_ref } => overlay_ref,
            Self::ResolvedTokenNoWinner { token_ref }
            | Self::ResolvedTokenWrongWinner { token_ref }
            | Self::ResolvedTokenShadowedNotInspectable { token_ref } => token_ref,
            Self::RoundTripStageDroppedEntries { stage_ref }
            | Self::RoundTripStageRewroteEntries { stage_ref } => stage_ref,
        }
    }
}

/// Per-scope blocking-finding summary.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenOverlayFindingSummary {
    /// Total blocking findings across every object.
    pub total_blocking_findings: usize,
    /// Entry-scoped blocking findings.
    pub entry_findings: usize,
    /// Overlay-scoped blocking findings.
    pub overlay_findings: usize,
    /// Resolution-scoped blocking findings.
    pub resolution_findings: usize,
    /// Round-trip-scoped blocking findings.
    pub round_trip_findings: usize,
}

impl TokenOverlayFindingSummary {
    /// Records one finding into the summary.
    fn record(&mut self, finding: &TokenOverlayBlockingFinding) {
        self.total_blocking_findings += 1;
        match finding {
            TokenOverlayBlockingFinding::EntryMissingExplanation { .. }
            | TokenOverlayBlockingFinding::EntryFallbackChainEmpty { .. }
            | TokenOverlayBlockingFinding::EntryScopeNotExplicit { .. }
            | TokenOverlayBlockingFinding::EntryDeprecatedWithoutReplacement { .. }
            | TokenOverlayBlockingFinding::EntryUnmappedWithoutPlaceholder { .. }
            | TokenOverlayBlockingFinding::EntryUnsupportedTreatedAsSupported { .. }
            | TokenOverlayBlockingFinding::EntryPortabilityInconsistent { .. } => {
                self.entry_findings += 1;
            }
            TokenOverlayBlockingFinding::OverlayFlattenedToOpaqueBlob { .. }
            | TokenOverlayBlockingFinding::OverlayHasNoEntries { .. }
            | TokenOverlayBlockingFinding::OverlayScopeMismatch { .. } => {
                self.overlay_findings += 1;
            }
            TokenOverlayBlockingFinding::ResolvedTokenNoWinner { .. }
            | TokenOverlayBlockingFinding::ResolvedTokenWrongWinner { .. }
            | TokenOverlayBlockingFinding::ResolvedTokenShadowedNotInspectable { .. } => {
                self.resolution_findings += 1;
            }
            TokenOverlayBlockingFinding::RoundTripStageDroppedEntries { .. }
            | TokenOverlayBlockingFinding::RoundTripStageRewroteEntries { .. }
            | TokenOverlayBlockingFinding::RoundTripEntryDropped { .. }
            | TokenOverlayBlockingFinding::RoundTripEntryRewritten { .. }
            | TokenOverlayBlockingFinding::RoundTripScopeLost { .. }
            | TokenOverlayBlockingFinding::RoundTripDowngradeNotDisclosed { .. } => {
                self.round_trip_findings += 1;
            }
        }
    }
}

/// Computes the blocking findings for one override entry.
fn compute_entry_findings(entry: &TokenOverrideEntry) -> Vec<TokenOverlayBlockingFinding> {
    let mut findings = Vec::new();
    let entry_ref = entry.entry_id.clone();

    if entry.explanation.trim().is_empty() {
        findings.push(TokenOverlayBlockingFinding::EntryMissingExplanation {
            entry_ref: entry_ref.clone(),
        });
    }
    if entry.fallback_chain.is_empty() {
        findings.push(TokenOverlayBlockingFinding::EntryFallbackChainEmpty {
            entry_ref: entry_ref.clone(),
        });
    }

    // Scope must be explicit: an override scope for overridden / deprecated /
    // unmapped entries, the theme-package default for inherited entries.
    let inherited = entry.value_state == ValueState::Inherited;
    let scope_is_default = entry.declared_scope == OverrideScope::ThemePackageDefault;
    if inherited != scope_is_default {
        findings.push(TokenOverlayBlockingFinding::EntryScopeNotExplicit {
            entry_ref: entry_ref.clone(),
        });
    }

    if entry.value_state == ValueState::Deprecated && entry.deprecated_replacement_ref.is_none() {
        findings.push(
            TokenOverlayBlockingFinding::EntryDeprecatedWithoutReplacement {
                entry_ref: entry_ref.clone(),
            },
        );
    }

    if entry.value_state == ValueState::Unmapped {
        if entry.unmapped_source_slot_ref.is_none()
            || entry.validation_state != OverlayValidationState::InertUnresolved
        {
            findings.push(
                TokenOverlayBlockingFinding::EntryUnmappedWithoutPlaceholder {
                    entry_ref: entry_ref.clone(),
                },
            );
        }
        if !entry.downgrade_class.is_downgrade() {
            findings.push(
                TokenOverlayBlockingFinding::EntryUnsupportedTreatedAsSupported {
                    entry_ref: entry_ref.clone(),
                },
            );
        }
    }

    // Portability flags must agree with the downgrade class.
    if !portability_is_consistent(&entry.portability, entry.downgrade_class, entry.value_state) {
        findings.push(TokenOverlayBlockingFinding::EntryPortabilityInconsistent {
            entry_ref: entry_ref.clone(),
        });
    }

    findings
}

/// `true` when an entry's portability flags are consistent with its downgrade
/// class and value state.
fn portability_is_consistent(
    flags: &PortabilityFlags,
    downgrade: DowngradeClass,
    value_state: ValueState,
) -> bool {
    match flags.portability_class {
        // A fully portable entry must carry no downgrade and must not be an
        // unsupported (unmapped) slot.
        PortabilityClass::FullyPortable => {
            !downgrade.is_downgrade() && flags.exportable && value_state != ValueState::Unmapped
        }
        // A downgraded-but-portable entry must disclose its downgrade and must
        // survive an unsupported target.
        PortabilityClass::PortableWithDowngrade => {
            downgrade.is_downgrade() && flags.exportable && flags.survives_unsupported_target
        }
        // A scope-local entry is deliberately not exported or synced.
        PortabilityClass::ScopeLocalNonPortable => !flags.exportable && !flags.syncable,
        // An inherited entry rides the package; it is not serialized as an
        // override and carries no downgrade.
        PortabilityClass::RidesThemePackage => {
            value_state == ValueState::Inherited && !flags.exportable && !downgrade.is_downgrade()
        }
    }
}

/// Computes the blocking findings for one overlay.
fn compute_overlay_findings(overlay: &ScopeOverlay) -> Vec<TokenOverlayBlockingFinding> {
    let mut findings = Vec::new();
    let overlay_ref = overlay.overlay_id.clone();

    if !overlay.structured {
        findings.push(TokenOverlayBlockingFinding::OverlayFlattenedToOpaqueBlob {
            overlay_ref: overlay_ref.clone(),
        });
    }
    if overlay.entries.is_empty() {
        findings.push(TokenOverlayBlockingFinding::OverlayHasNoEntries {
            overlay_ref: overlay_ref.clone(),
        });
    }
    for entry in &overlay.entries {
        if entry.declared_scope != overlay.scope {
            findings.push(TokenOverlayBlockingFinding::OverlayScopeMismatch {
                overlay_ref: overlay_ref.clone(),
                entry_ref: entry.entry_id.clone(),
            });
        }
    }

    findings
}

/// Computes the blocking findings for one resolved token against the entries
/// that contributed to it, grouped by token.
fn compute_resolution_findings(
    resolved: &ResolvedToken,
    contributing: &[(OverrideScope, String)],
) -> Vec<TokenOverlayBlockingFinding> {
    let mut findings = Vec::new();
    let token_ref = resolved.token_ref.clone();

    if contributing.is_empty() {
        findings.push(TokenOverlayBlockingFinding::ResolvedTokenNoWinner {
            token_ref: token_ref.clone(),
        });
        return findings;
    }

    // The winner must be the highest-precedence contributing scope.
    let expected_winner = contributing
        .iter()
        .max_by_key(|(scope, _)| scope.precedence_rank())
        .map(|(scope, _)| *scope);
    if Some(resolved.winning_scope) != expected_winner {
        findings.push(TokenOverlayBlockingFinding::ResolvedTokenWrongWinner {
            token_ref: token_ref.clone(),
        });
    }

    // Every non-winning contributing entry must be listed as shadowed.
    if resolved.shadowed.len() + 1 != contributing.len() {
        findings.push(
            TokenOverlayBlockingFinding::ResolvedTokenShadowedNotInspectable {
                token_ref: token_ref.clone(),
            },
        );
    }

    findings
}

/// Computes the blocking findings for one round-trip stage.
fn compute_stage_findings(stage: &RoundTripStage) -> Vec<TokenOverlayBlockingFinding> {
    let mut findings = Vec::new();
    let stage_ref = stage.stage_id.clone();

    if stage.dropped_count > 0 {
        findings.push(TokenOverlayBlockingFinding::RoundTripStageDroppedEntries {
            stage_ref: stage_ref.clone(),
        });
    }
    if stage.rewritten_count > 0 {
        findings.push(TokenOverlayBlockingFinding::RoundTripStageRewroteEntries {
            stage_ref: stage_ref.clone(),
        });
    }

    findings
}

/// Computes the blocking findings for one round-trip entry trace.
fn compute_trace_findings(trace: &RoundTripEntryTrace) -> Vec<TokenOverlayBlockingFinding> {
    let mut findings = Vec::new();
    let entry_ref = trace.entry_ref.clone();

    match trace.disposition {
        EntryDisposition::Dropped => {
            findings.push(TokenOverlayBlockingFinding::RoundTripEntryDropped {
                entry_ref: entry_ref.clone(),
            });
        }
        EntryDisposition::Rewritten => {
            findings.push(TokenOverlayBlockingFinding::RoundTripEntryRewritten {
                entry_ref: entry_ref.clone(),
            });
        }
        EntryDisposition::Downgraded => {
            if !trace.downgrade_class.is_downgrade() {
                findings.push(
                    TokenOverlayBlockingFinding::RoundTripDowngradeNotDisclosed {
                        entry_ref: entry_ref.clone(),
                    },
                );
            }
        }
        EntryDisposition::Preserved => {}
    }

    if trace.origin_scope != trace.final_scope {
        findings.push(TokenOverlayBlockingFinding::RoundTripScopeLost {
            entry_ref: entry_ref.clone(),
        });
    }

    if trace.disposition.survived() != trace.survived {
        findings.push(TokenOverlayBlockingFinding::RoundTripEntryDropped {
            entry_ref: entry_ref.clone(),
        });
    }

    findings
}

/// The canonical token-overlay portability audit: the per-scope overlays, the
/// winning-versus-shadowed resolution table, and the export / import / sync
/// round-trip proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenOverlayPortabilityReport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable report id quoted across surfaces.
    pub report_id: String,
    /// Source schema ref for the canonical contract.
    pub source_schema_ref: String,
    /// Schema ref for the canonical per-token overlay-state records.
    pub canonical_overlay_schema_ref: String,
    /// The live appearance session these overlays apply to.
    pub appearance_session_ref: String,
    /// The per-scope overlays, sorted by precedence then `overlay_id`.
    pub overlays: Vec<ScopeOverlay>,
    /// The resolution table, sorted by `token_ref`.
    pub resolved_tokens: Vec<ResolvedToken>,
    /// The round-trip proof.
    pub round_trip: RoundTripProof,
    /// Per-scope blocking-finding summary.
    pub findings_summary: TokenOverlayFindingSummary,
    /// Every blocking finding across the audit, sorted by class then subject.
    pub blocking_findings: Vec<TokenOverlayBlockingFinding>,
    /// Total override entries across every overlay.
    pub total_override_count: usize,
    /// Override entries carrying a disclosed downgrade.
    pub downgraded_override_count: usize,
    /// Override entries that survive as inert placeholders.
    pub inert_override_count: usize,
    /// Number of distinct override scopes covered.
    pub scope_covered_count: usize,
    /// `true` when the round trip dropped or rewrote nothing.
    pub round_trip_lossless: bool,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Markdown publication ref this audit is rendered to.
    pub published_report_ref: String,
    /// Companion doc publication ref.
    pub published_doc_ref: String,
    /// Docs/help refs the audit can be reopened from.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs the audit can be reopened from.
    pub support_export_refs: Vec<String>,
    /// Timestamp captured when the audit was generated.
    pub generated_at: String,
}

impl TokenOverlayPortabilityReport {
    /// Returns the overlay registered under `overlay_id`, if any.
    pub fn overlay(&self, overlay_id: &str) -> Option<&ScopeOverlay> {
        self.overlays
            .iter()
            .find(|overlay| overlay.overlay_id == overlay_id)
    }

    /// Returns every override entry across every overlay.
    pub fn all_entries(&self) -> impl Iterator<Item = &TokenOverrideEntry> {
        self.overlays
            .iter()
            .flat_map(|overlay| overlay.entries.iter())
    }

    /// Returns the resolved token registered under `token_ref`, if any.
    pub fn resolved(&self, token_ref: &str) -> Option<&ResolvedToken> {
        self.resolved_tokens
            .iter()
            .find(|resolved| resolved.token_ref == token_ref)
    }

    /// Builds compact text rows for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "audit: overlays={}, entries={}, resolved={}, stages={}, traces={}, downgraded={}, inert={}, lossless={}, blocking={}, clean={}",
            self.overlays.len(),
            self.total_override_count,
            self.resolved_tokens.len(),
            self.round_trip.stages.len(),
            self.round_trip.entry_traces.len(),
            self.downgraded_override_count,
            self.inert_override_count,
            self.round_trip_lossless,
            self.findings_summary.total_blocking_findings,
            self.report_clean,
        ));
        for overlay in &self.overlays {
            lines.push(format!(
                "overlay: {} -- scope={}, entries={}, downgraded={}, validation={}, structured={}",
                overlay.overlay_id,
                overlay.scope.as_str(),
                overlay.entry_count,
                overlay.downgraded_count,
                overlay.validation_state.as_str(),
                overlay.structured,
            ));
        }
        for resolved in &self.resolved_tokens {
            lines.push(format!(
                "resolved: {} -- winner={}, state={}, shadowed={}",
                resolved.token_ref,
                resolved.winning_scope.as_str(),
                resolved.winning_value_state.as_str(),
                resolved.shadowed.len(),
            ));
        }
        for stage in &self.round_trip.stages {
            lines.push(format!(
                "stage: {} -- channel={}, target={}, preserved={}, downgraded={}, dropped={}, rewritten={}",
                stage.stage_id,
                stage.channel.as_str(),
                stage.target_support_profile,
                stage.preserved_count,
                stage.downgraded_count,
                stage.dropped_count,
                stage.rewritten_count,
            ));
        }
        for trace in &self.round_trip.entry_traces {
            lines.push(format!(
                "trace: {} -- token={}, disposition={}, downgrade={}, scope={}->{}, survived={}",
                trace.entry_ref,
                trace.token_ref,
                trace.disposition.as_str(),
                trace.downgrade_class.as_str(),
                trace.origin_scope.as_str(),
                trace.final_scope.as_str(),
                trace.survived,
            ));
        }
        for finding in &self.blocking_findings {
            lines.push(format!(
                "blocker: {} -- {}",
                finding.class_token(),
                finding.subject_ref(),
            ));
        }
        lines
    }

    /// Renders the markdown audit artifact.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 token-overlay round-trip audit\n\n");
        out.push_str(
            "Generated from the seeded audit in\n\
             [`crate::token_overlays`](../../../../crates/aureline-shell/src/token_overlays/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_token_overlays -- report-md > \\\n  artifacts/ux/m5/token-overlay-roundtrip/m5_token_overlay_roundtrip_audit.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Report id: `{}`\n", self.report_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Canonical overlay schema: `{}`\n",
            self.canonical_overlay_schema_ref
        ));
        out.push_str(&format!(
            "- Appearance session: `{}`\n",
            self.appearance_session_ref
        ));
        out.push_str(&format!("- Overlays: `{}`\n", self.overlays.len()));
        out.push_str(&format!(
            "- Override entries: `{}` (downgraded `{}`, inert `{}`)\n",
            self.total_override_count, self.downgraded_override_count, self.inert_override_count
        ));
        out.push_str(&format!(
            "- Resolved tokens: `{}`\n",
            self.resolved_tokens.len()
        ));
        out.push_str(&format!(
            "- Round trip lossless: `{}`\n",
            self.round_trip_lossless
        ));
        out.push_str(&format!(
            "- Unsupported entries preserved: `{}`\n",
            self.round_trip.unsupported_preserved_count
        ));
        out.push_str(&format!(
            "- Blocking findings: `{}`\n",
            self.findings_summary.total_blocking_findings
        ));
        out.push_str(&format!(
            "- Status: **{}**\n",
            if self.report_clean {
                "clean"
            } else {
                "blocked"
            }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Scope overlays\n\n");
        out.push_str(
            "| Overlay | Scope | Entries | Downgraded | Validation | Structured |\n\
             | ------- | ----- | ------: | ---------: | ---------- | ---------- |\n",
        );
        for overlay in &self.overlays {
            out.push_str(&format!(
                "| `{}` | `{}` | {} | {} | `{}` | `{}` |\n",
                overlay.overlay_id,
                overlay.scope.as_str(),
                overlay.entry_count,
                overlay.downgraded_count,
                overlay.validation_state.as_str(),
                overlay.structured,
            ));
        }
        out.push('\n');

        out.push_str("## Override entries\n\n");
        out.push_str(
            "| Entry | Token | Family | Scope | State | Provenance | Portability | Downgrade |\n\
             | ----- | ----- | ------ | ----- | ----- | ---------- | ----------- | --------- |\n",
        );
        for entry in self.all_entries() {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                entry.entry_id,
                entry.token_ref,
                entry.token_family.as_str(),
                entry.declared_scope.as_str(),
                entry.value_state.as_str(),
                entry.provenance.as_str(),
                entry.portability.portability_class.as_str(),
                entry.downgrade_class.as_str(),
            ));
        }
        out.push('\n');

        out.push_str("## Resolved tokens (winning versus shadowed)\n\n");
        out.push_str(
            "| Token | Winner | State | Shadowed | Why |\n\
             | ----- | ------ | ----- | -------- | --- |\n",
        );
        for resolved in &self.resolved_tokens {
            let shadowed = if resolved.shadowed.is_empty() {
                "—".to_owned()
            } else {
                resolved
                    .shadowed
                    .iter()
                    .map(|s| s.scope.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | {} | {} |\n",
                resolved.token_ref,
                resolved.winning_scope.as_str(),
                resolved.winning_value_state.as_str(),
                shadowed,
                resolved.precedence_explained,
            ));
        }
        out.push('\n');

        out.push_str("## Round-trip stages\n\n");
        out.push_str(
            "| Seq | Channel | Target | In | Out | Preserved | Downgraded | Dropped | Rewritten |\n\
             | --: | ------- | ------ | -: | --: | --------: | ---------: | ------: | --------: |\n",
        );
        for stage in &self.round_trip.stages {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | {} | {} | {} | {} | {} | {} |\n",
                stage.sequence_index,
                stage.channel.as_str(),
                stage.target_support_profile,
                stage.input_entry_count,
                stage.output_entry_count,
                stage.preserved_count,
                stage.downgraded_count,
                stage.dropped_count,
                stage.rewritten_count,
            ));
        }
        out.push('\n');

        out.push_str("## Round-trip entry traces\n\n");
        out.push_str(
            "| Entry | Token | Disposition | Downgrade | Scope | Survived |\n\
             | ----- | ----- | ----------- | --------- | ----- | -------- |\n",
        );
        for trace in &self.round_trip.entry_traces {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                trace.entry_ref,
                trace.token_ref,
                trace.disposition.as_str(),
                trace.downgrade_class.as_str(),
                trace.origin_scope.as_str(),
                trace.survived,
            ));
        }
        out.push('\n');

        out.push_str("## Findings summary\n\n");
        out.push_str("| Scope | Count |\n| ----- | ----: |\n");
        out.push_str(&format!(
            "| `entry` | {} |\n",
            self.findings_summary.entry_findings
        ));
        out.push_str(&format!(
            "| `overlay` | {} |\n",
            self.findings_summary.overlay_findings
        ));
        out.push_str(&format!(
            "| `resolution` | {} |\n",
            self.findings_summary.resolution_findings
        ));
        out.push_str(&format!(
            "| `round_trip` | {} |\n",
            self.findings_summary.round_trip_findings
        ));
        out.push_str(&format!(
            "| `total` | {} |\n\n",
            self.findings_summary.total_blocking_findings
        ));

        if self.blocking_findings.is_empty() {
            out.push_str("Findings: none.\n\n");
        } else {
            out.push_str("Findings:\n\n");
            for finding in &self.blocking_findings {
                out.push_str(&format!(
                    "- `{}` — `{}`\n",
                    finding.class_token(),
                    finding.subject_ref()
                ));
            }
            out.push('\n');
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_token_overlays -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_token_overlays_fixtures\n");
        out.push_str("python3 tools/ci/m5/token_overlay_check.py\n");
        out.push_str("```\n");
        out
    }
}

/// Support-export wrapper for the token-overlay portability audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenOverlaySupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Audit report quoted in full.
    pub report: TokenOverlayPortabilityReport,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl TokenOverlaySupportExport {
    /// Builds the support-export wrapper for an audit report.
    ///
    /// Every report id, the appearance-session ref, each overlay id, each entry
    /// id, each resolved token ref, the proof id, each stage id, and each traced
    /// entry ref is quoted as a case id so a support reviewer — or a
    /// golden-evidence pack — can name the same overlay object the runtime used.
    pub fn from_report(
        support_export_id: impl Into<String>,
        report: TokenOverlayPortabilityReport,
    ) -> Self {
        let mut case_ids = vec![
            report.report_id.clone(),
            report.appearance_session_ref.clone(),
        ];
        for overlay in &report.overlays {
            case_ids.push(overlay.overlay_id.clone());
            for entry in &overlay.entries {
                case_ids.push(entry.entry_id.clone());
            }
        }
        for resolved in &report.resolved_tokens {
            case_ids.push(resolved.token_ref.clone());
        }
        case_ids.push(report.round_trip.proof_id.clone());
        for stage in &report.round_trip.stages {
            case_ids.push(stage.stage_id.clone());
        }
        for trace in &report.round_trip.entry_traces {
            case_ids.push(trace.entry_ref.clone());
        }
        Self {
            record_kind: TOKEN_OVERLAY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: TOKEN_OVERLAY_SCHEMA_VERSION,
            shared_contract_ref: TOKEN_OVERLAY_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            report,
            case_ids,
        }
    }
}

/// Assembles a token-overlay portability report from its parts, recomputing
/// every per-object finding so the report is the single source of truth.
pub fn build_token_overlay_portability(
    mut overlays: Vec<ScopeOverlay>,
    mut resolved_tokens: Vec<ResolvedToken>,
    mut round_trip: RoundTripProof,
) -> TokenOverlayPortabilityReport {
    // Sort overlays by precedence then id, and entries within each by id.
    overlays.sort_by(|left, right| {
        left.scope
            .precedence_rank()
            .cmp(&right.scope.precedence_rank())
            .then_with(|| left.overlay_id.cmp(&right.overlay_id))
    });
    for overlay in &mut overlays {
        overlay
            .entries
            .sort_by(|left, right| left.entry_id.cmp(&right.entry_id));
        for entry in &mut overlay.entries {
            entry.blocking_findings = compute_entry_findings(entry);
        }
        overlay.entry_count = overlay.entries.len();
        overlay.downgraded_count = overlay
            .entries
            .iter()
            .filter(|entry| entry.downgrade_class.is_downgrade())
            .count();
        overlay.blocking_findings = compute_overlay_findings(overlay);
    }

    // Build the contributing-scope index per token from the overlays.
    resolved_tokens.sort_by(|left, right| left.token_ref.cmp(&right.token_ref));
    for resolved in &mut resolved_tokens {
        resolved.shadowed.sort_by(|left, right| {
            right
                .scope
                .precedence_rank()
                .cmp(&left.scope.precedence_rank())
                .then_with(|| left.entry_ref.cmp(&right.entry_ref))
        });
        let contributing: Vec<(OverrideScope, String)> = overlays
            .iter()
            .flat_map(|overlay| overlay.entries.iter())
            .filter(|entry| entry.token_ref == resolved.token_ref)
            .map(|entry| (entry.declared_scope, entry.entry_id.clone()))
            .collect();
        resolved.blocking_findings = compute_resolution_findings(resolved, &contributing);
    }

    // Recompute round-trip findings.
    round_trip
        .stages
        .sort_by(|left, right| left.sequence_index.cmp(&right.sequence_index));
    for stage in &mut round_trip.stages {
        stage.blocking_findings = compute_stage_findings(stage);
    }
    round_trip
        .entry_traces
        .sort_by(|left, right| left.entry_ref.cmp(&right.entry_ref));
    for trace in &mut round_trip.entry_traces {
        trace.blocking_findings = compute_trace_findings(trace);
    }
    round_trip.portable_entry_count = round_trip.entry_traces.len();
    round_trip.unsupported_preserved_count = round_trip
        .entry_traces
        .iter()
        .filter(|trace| trace.downgrade_class.is_downgrade() && trace.survived)
        .count();
    let round_trip_lossless = round_trip
        .stages
        .iter()
        .all(|stage| stage.dropped_count == 0 && stage.rewritten_count == 0)
        && round_trip
            .entry_traces
            .iter()
            .all(|trace| !trace.disposition.is_lossy() && trace.origin_scope == trace.final_scope);
    round_trip.lossless = round_trip_lossless;
    round_trip.blocking_findings = round_trip
        .stages
        .iter()
        .flat_map(|stage| stage.blocking_findings.iter().cloned())
        .chain(
            round_trip
                .entry_traces
                .iter()
                .flat_map(|trace| trace.blocking_findings.iter().cloned()),
        )
        .collect();

    // Aggregate every finding.
    let mut findings_summary = TokenOverlayFindingSummary::default();
    let mut blocking_findings: Vec<TokenOverlayBlockingFinding> = Vec::new();
    for overlay in &overlays {
        for finding in &overlay.blocking_findings {
            findings_summary.record(finding);
            blocking_findings.push(finding.clone());
        }
        for entry in &overlay.entries {
            for finding in &entry.blocking_findings {
                findings_summary.record(finding);
                blocking_findings.push(finding.clone());
            }
        }
    }
    for resolved in &resolved_tokens {
        for finding in &resolved.blocking_findings {
            findings_summary.record(finding);
            blocking_findings.push(finding.clone());
        }
    }
    for finding in &round_trip.blocking_findings {
        findings_summary.record(finding);
        blocking_findings.push(finding.clone());
    }

    blocking_findings.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });

    let total_override_count = overlays.iter().map(|overlay| overlay.entries.len()).sum();
    let downgraded_override_count = overlays
        .iter()
        .flat_map(|overlay| overlay.entries.iter())
        .filter(|entry| entry.downgrade_class.is_downgrade())
        .count();
    let inert_override_count = overlays
        .iter()
        .flat_map(|overlay| overlay.entries.iter())
        .filter(|entry| entry.value_state == ValueState::Unmapped)
        .count();
    let scope_covered_count = {
        let mut scopes: Vec<OverrideScope> = overlays.iter().map(|overlay| overlay.scope).collect();
        scopes.sort();
        scopes.dedup();
        scopes.len()
    };
    let report_clean = findings_summary.total_blocking_findings == 0;

    TokenOverlayPortabilityReport {
        record_kind: TOKEN_OVERLAY_REPORT_RECORD_KIND.to_owned(),
        schema_version: TOKEN_OVERLAY_SCHEMA_VERSION,
        shared_contract_ref: TOKEN_OVERLAY_SHARED_CONTRACT_REF.to_owned(),
        report_id: TOKEN_OVERLAY_REPORT_ID.to_owned(),
        source_schema_ref: TOKEN_OVERLAY_SOURCE_SCHEMA_REF.to_owned(),
        canonical_overlay_schema_ref: TOKEN_OVERLAY_CANONICAL_RECORD_SCHEMA_REF.to_owned(),
        appearance_session_ref: TOKEN_OVERLAY_APPEARANCE_SESSION_REF.to_owned(),
        overlays,
        resolved_tokens,
        round_trip,
        findings_summary,
        blocking_findings,
        total_override_count,
        downgraded_override_count,
        inert_override_count,
        scope_covered_count,
        round_trip_lossless,
        report_clean,
        published_report_ref: TOKEN_OVERLAY_PUBLISHED_REPORT_REF.to_owned(),
        published_doc_ref: TOKEN_OVERLAY_PUBLISHED_DOC_REF.to_owned(),
        docs_help_refs: vec![
            TOKEN_OVERLAY_PUBLISHED_DOC_REF.to_owned(),
            "docs/m5/theme-package-and-appearance-objects.md".to_owned(),
        ],
        support_export_refs: vec!["support:m5-token-overlays".to_owned()],
        generated_at: GENERATED_AT.to_owned(),
    }
}

/// Validation error produced by [`validate_token_overlay_portability`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum TokenOverlayValidationError {
    /// The audit has no registered overlays.
    NoRegisteredOverlays,
    /// The audit has no resolved tokens.
    NoResolvedTokens,
    /// The audit has no round-trip stages.
    NoRoundTripStages,
    /// The audit has no round-trip entry traces.
    NoRoundTripTraces,
    /// A resolved token's winning entry does not resolve to a real entry.
    ResolvedWinnerUnresolved {
        /// Token ref.
        token_ref: String,
        /// The unresolved winning entry ref.
        winning_entry_ref: String,
    },
    /// A blocking finding remains in the audit.
    BlockingFindingPresent {
        /// Finding class.
        class: String,
        /// Owning object ref.
        subject_ref: String,
    },
    /// The round trip is not lossless.
    RoundTripNotLossless,
    /// The published markdown report ref is empty.
    PublishedReportRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
}

/// Validates an audit report against the token-overlay portability acceptance
/// invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_token_overlay_portability(
    report: &TokenOverlayPortabilityReport,
) -> Result<(), Vec<TokenOverlayValidationError>> {
    let mut errors = Vec::new();

    if report.overlays.is_empty() {
        errors.push(TokenOverlayValidationError::NoRegisteredOverlays);
    }
    if report.resolved_tokens.is_empty() {
        errors.push(TokenOverlayValidationError::NoResolvedTokens);
    }
    if report.round_trip.stages.is_empty() {
        errors.push(TokenOverlayValidationError::NoRoundTripStages);
    }
    if report.round_trip.entry_traces.is_empty() {
        errors.push(TokenOverlayValidationError::NoRoundTripTraces);
    }

    let entry_ids: std::collections::BTreeSet<&str> = report
        .all_entries()
        .map(|entry| entry.entry_id.as_str())
        .collect();
    for resolved in &report.resolved_tokens {
        if !entry_ids.contains(resolved.winning_entry_ref.as_str()) {
            errors.push(TokenOverlayValidationError::ResolvedWinnerUnresolved {
                token_ref: resolved.token_ref.clone(),
                winning_entry_ref: resolved.winning_entry_ref.clone(),
            });
        }
    }

    for finding in &report.blocking_findings {
        errors.push(TokenOverlayValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if !report.round_trip_lossless {
        errors.push(TokenOverlayValidationError::RoundTripNotLossless);
    }
    if report.published_report_ref.trim().is_empty() {
        errors.push(TokenOverlayValidationError::PublishedReportRefMissing);
    }
    if report.published_doc_ref.trim().is_empty() {
        errors.push(TokenOverlayValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Builds a deterministic fallback chain for an entry by value state.
fn seed_fallback_chain(
    entry_id: &str,
    scope: OverrideScope,
    value_state: ValueState,
    token_ref: &str,
    deprecated_replacement_ref: Option<&str>,
) -> Vec<FallbackStep> {
    let step = |index: u32, step_scope, step_kind, target: Option<&str>, applied| FallbackStep {
        step_id: format!("{entry_id}:fallback:{index}"),
        step_index: index,
        step_scope,
        step_kind,
        target_token_ref: target.map(str::to_owned),
        applied,
    };
    match value_state {
        ValueState::Inherited => vec![step(
            0,
            OverrideScope::ThemePackageDefault,
            FallbackStepKind::ThemePackageDefault,
            Some(token_ref),
            true,
        )],
        ValueState::Overridden => vec![
            step(
                0,
                OverrideScope::ThemePackageDefault,
                FallbackStepKind::ThemePackageDefault,
                Some(token_ref),
                false,
            ),
            step(
                1,
                scope,
                FallbackStepKind::ScopeOverride,
                Some(token_ref),
                true,
            ),
        ],
        ValueState::Deprecated => vec![
            step(
                0,
                OverrideScope::ThemePackageDefault,
                FallbackStepKind::ThemePackageDefault,
                Some(token_ref),
                false,
            ),
            step(
                1,
                scope,
                FallbackStepKind::DeprecatedAlias,
                deprecated_replacement_ref.or(Some(token_ref)),
                true,
            ),
        ],
        ValueState::Unmapped => vec![step(
            0,
            scope,
            FallbackStepKind::InertPlaceholder,
            None,
            false,
        )],
    }
}

/// Builds one seeded override entry.
#[allow(clippy::too_many_arguments)]
fn seed_entry(
    token_ref: &str,
    token_family: TokenFamily,
    scope: OverrideScope,
    value_state: ValueState,
    validation_state: OverlayValidationState,
    provenance: ProvenanceClass,
    portability: PortabilityFlags,
    downgrade_class: DowngradeClass,
    deprecated_replacement_ref: Option<&str>,
    unmapped_source_slot_ref: Option<&str>,
    explanation: &str,
) -> TokenOverrideEntry {
    let entry_id = format!("entry:{}:{}", scope.as_str(), token_ref);
    let fallback_chain = seed_fallback_chain(
        &entry_id,
        scope,
        value_state,
        token_ref,
        deprecated_replacement_ref,
    );
    let canonical_overlay_record_ref = Some(format!("token-overlay-record:{}", scope.as_str()));
    TokenOverrideEntry {
        record_kind: TOKEN_OVERLAY_OVERRIDE_ENTRY_RECORD_KIND.to_owned(),
        entry_id,
        token_ref: token_ref.to_owned(),
        token_family,
        declared_scope: scope,
        value_state,
        validation_state,
        provenance,
        portability,
        downgrade_class,
        fallback_chain,
        deprecated_replacement_ref: deprecated_replacement_ref.map(str::to_owned),
        unmapped_source_slot_ref: unmapped_source_slot_ref.map(str::to_owned),
        canonical_overlay_record_ref,
        explanation: explanation.to_owned(),
        blocking_findings: Vec::new(),
    }
}

/// Builds one seeded overlay from its scope and entries.
fn seed_overlay(
    scope: OverrideScope,
    validation_state: OverlayValidationState,
    entries: Vec<TokenOverrideEntry>,
) -> ScopeOverlay {
    ScopeOverlay {
        record_kind: TOKEN_OVERLAY_SCOPE_OVERLAY_RECORD_KIND.to_owned(),
        schema_version: TOKEN_OVERLAY_SCHEMA_VERSION,
        shared_contract_ref: TOKEN_OVERLAY_SHARED_CONTRACT_REF.to_owned(),
        overlay_id: format!("overlay:{}", scope.as_str()),
        scope,
        appearance_session_ref: TOKEN_OVERLAY_APPEARANCE_SESSION_REF.to_owned(),
        structured: true,
        validation_state,
        entries,
        entry_count: 0,
        downgraded_count: 0,
        blocking_findings: Vec::new(),
    }
}

/// Builds the clean, deterministic token-overlay portability audit that the
/// checked-in fixtures under `fixtures/ux/m5/token-overlay-sync-import/` and the
/// markdown artifact under `artifacts/ux/m5/token-overlay-roundtrip/` mirror.
pub fn seeded_token_overlay_portability() -> TokenOverlayPortabilityReport {
    let fully_portable = PortabilityFlags {
        portability_class: PortabilityClass::FullyPortable,
        exportable: true,
        syncable: true,
        survives_unsupported_target: true,
    };
    let portable_with_downgrade = PortabilityFlags {
        portability_class: PortabilityClass::PortableWithDowngrade,
        exportable: true,
        syncable: true,
        survives_unsupported_target: true,
    };
    let rides_package = PortabilityFlags {
        portability_class: PortabilityClass::RidesThemePackage,
        exportable: false,
        syncable: false,
        survives_unsupported_target: true,
    };
    let scope_local = PortabilityFlags {
        portability_class: PortabilityClass::ScopeLocalNonPortable,
        exportable: false,
        syncable: false,
        survives_unsupported_target: false,
    };
    // A workspace cap is exported in the workspace bundle but not personally
    // synced; model it as portable-with-no-downgrade by keeping it fully
    // portable for the export bundle path.
    let workspace_portable = PortabilityFlags {
        portability_class: PortabilityClass::FullyPortable,
        exportable: true,
        syncable: false,
        survives_unsupported_target: true,
    };

    // Tokens exercised by the audit.
    let accent = "color.accent.primary";
    let danger = "color.semantic.danger";
    let chart = "color.chart.series_9";
    let code = "typography.role.code";
    let spacing = "spacing.density.row";

    let overlays = vec![
        // The inherited theme-package base.
        seed_overlay(
            OverrideScope::ThemePackageDefault,
            OverlayValidationState::Valid,
            vec![
                seed_entry(
                    accent,
                    TokenFamily::ColorFunctionalAccent,
                    OverrideScope::ThemePackageDefault,
                    ValueState::Inherited,
                    OverlayValidationState::Valid,
                    ProvenanceClass::ImportedFromThemePackage,
                    rides_package,
                    DowngradeClass::None,
                    None,
                    None,
                    "Base accent inherited from the active theme package; re-resolved on import.",
                ),
                seed_entry(
                    danger,
                    TokenFamily::ColorState,
                    OverrideScope::ThemePackageDefault,
                    ValueState::Inherited,
                    OverlayValidationState::Valid,
                    ProvenanceClass::ImportedFromThemePackage,
                    rides_package,
                    DowngradeClass::None,
                    None,
                    None,
                    "Base danger colour inherited from the active theme package.",
                ),
                seed_entry(
                    code,
                    TokenFamily::TypographyRole,
                    OverrideScope::ThemePackageDefault,
                    ValueState::Inherited,
                    OverlayValidationState::Valid,
                    ProvenanceClass::ImportedFromThemePackage,
                    rides_package,
                    DowngradeClass::None,
                    None,
                    None,
                    "Base code typography role inherited from the active theme package.",
                ),
            ],
        ),
        // An imported theme contributing an unmapped (inert) token.
        seed_overlay(
            OverrideScope::ImportedTheme,
            OverlayValidationState::InertUnresolved,
            vec![seed_entry(
                chart,
                TokenFamily::ColorChart,
                OverrideScope::ImportedTheme,
                ValueState::Unmapped,
                OverlayValidationState::InertUnresolved,
                ProvenanceClass::ImportedFromThemePackage,
                portable_with_downgrade,
                DowngradeClass::InertUnsupportedToken,
                None,
                Some("imported-slot:chart.series_9"),
                "Imported theme references a chart slot the active package does not map; kept as an inert placeholder, never applied.",
            )],
        ),
        // An extension contributing a deprecated alias.
        seed_overlay(
            OverrideScope::ExtensionContributed,
            OverlayValidationState::ValidWithWarnings,
            vec![seed_entry(
                code,
                TokenFamily::TypographyRole,
                OverrideScope::ExtensionContributed,
                ValueState::Deprecated,
                OverlayValidationState::ValidWithWarnings,
                ProvenanceClass::ContributedByExtension,
                portable_with_downgrade,
                DowngradeClass::DeprecatedAliasPendingReplacement,
                Some("typography.role.monospace"),
                None,
                "Extension override points at a deprecated code-role alias; survives with a disclosed replacement pending.",
            )],
        ),
        // The user-global overrides.
        seed_overlay(
            OverrideScope::UserGlobal,
            OverlayValidationState::Valid,
            vec![
                seed_entry(
                    accent,
                    TokenFamily::ColorFunctionalAccent,
                    OverrideScope::UserGlobal,
                    ValueState::Overridden,
                    OverlayValidationState::Valid,
                    ProvenanceClass::AuthoredInProduct,
                    fully_portable,
                    DowngradeClass::None,
                    None,
                    None,
                    "User-global accent override; shadowed by the workspace override.",
                ),
                seed_entry(
                    spacing,
                    TokenFamily::Spacing,
                    OverrideScope::UserGlobal,
                    ValueState::Overridden,
                    OverlayValidationState::Valid,
                    ProvenanceClass::AuthoredInProduct,
                    fully_portable,
                    DowngradeClass::None,
                    None,
                    None,
                    "User-global row spacing override; shadowed by the profile override.",
                ),
            ],
        ),
        // The profile overrides.
        seed_overlay(
            OverrideScope::Profile,
            OverlayValidationState::Valid,
            vec![
                seed_entry(
                    danger,
                    TokenFamily::ColorState,
                    OverrideScope::Profile,
                    ValueState::Overridden,
                    OverlayValidationState::Valid,
                    ProvenanceClass::AuthoredInProduct,
                    fully_portable,
                    DowngradeClass::None,
                    None,
                    None,
                    "Profile danger-colour override; shadowed by the managed-policy cap.",
                ),
                seed_entry(
                    spacing,
                    TokenFamily::Spacing,
                    OverrideScope::Profile,
                    ValueState::Overridden,
                    OverlayValidationState::Valid,
                    ProvenanceClass::AuthoredInProduct,
                    fully_portable,
                    DowngradeClass::None,
                    None,
                    None,
                    "Profile row-spacing override; wins over the user-global override.",
                ),
            ],
        ),
        // The workspace override.
        seed_overlay(
            OverrideScope::Workspace,
            OverlayValidationState::Valid,
            vec![seed_entry(
                accent,
                TokenFamily::ColorFunctionalAccent,
                OverrideScope::Workspace,
                ValueState::Overridden,
                OverlayValidationState::Valid,
                ProvenanceClass::AuthoredInProduct,
                workspace_portable,
                DowngradeClass::None,
                None,
                None,
                "Workspace accent override; wins over the user-global override and the theme default.",
            )],
        ),
        // The managed-policy cap.
        seed_overlay(
            OverrideScope::PolicyManaged,
            OverlayValidationState::Valid,
            vec![seed_entry(
                danger,
                TokenFamily::ColorState,
                OverrideScope::PolicyManaged,
                ValueState::Overridden,
                OverlayValidationState::Valid,
                ProvenanceClass::AppliedByPolicy,
                scope_local,
                DowngradeClass::None,
                None,
                None,
                "Managed-policy danger-colour cap; wins over every personal scope and stays local by design.",
            )],
        ),
    ];

    let resolved_tokens = vec![
        ResolvedToken {
            record_kind: TOKEN_OVERLAY_RESOLVED_TOKEN_RECORD_KIND.to_owned(),
            token_ref: accent.to_owned(),
            token_family: TokenFamily::ColorFunctionalAccent,
            winning_scope: OverrideScope::Workspace,
            winning_entry_ref: format!("entry:{}:{}", OverrideScope::Workspace.as_str(), accent),
            winning_value_state: ValueState::Overridden,
            shadowed: vec![
                ShadowedEntry {
                    scope: OverrideScope::UserGlobal,
                    entry_ref: format!("entry:{}:{}", OverrideScope::UserGlobal.as_str(), accent),
                    value_state: ValueState::Overridden,
                    reason: "Lower precedence than the workspace override.".to_owned(),
                },
                ShadowedEntry {
                    scope: OverrideScope::ThemePackageDefault,
                    entry_ref: format!(
                        "entry:{}:{}",
                        OverrideScope::ThemePackageDefault.as_str(),
                        accent
                    ),
                    value_state: ValueState::Inherited,
                    reason: "Theme-package default shadowed by an override.".to_owned(),
                },
            ],
            precedence_explained:
                "Workspace accent wins over the user-global override and the theme default."
                    .to_owned(),
            blocking_findings: Vec::new(),
        },
        ResolvedToken {
            record_kind: TOKEN_OVERLAY_RESOLVED_TOKEN_RECORD_KIND.to_owned(),
            token_ref: danger.to_owned(),
            token_family: TokenFamily::ColorState,
            winning_scope: OverrideScope::PolicyManaged,
            winning_entry_ref: format!("entry:{}:{}", OverrideScope::PolicyManaged.as_str(), danger),
            winning_value_state: ValueState::Overridden,
            shadowed: vec![
                ShadowedEntry {
                    scope: OverrideScope::Profile,
                    entry_ref: format!("entry:{}:{}", OverrideScope::Profile.as_str(), danger),
                    value_state: ValueState::Overridden,
                    reason: "Capped by the managed policy.".to_owned(),
                },
                ShadowedEntry {
                    scope: OverrideScope::ThemePackageDefault,
                    entry_ref: format!(
                        "entry:{}:{}",
                        OverrideScope::ThemePackageDefault.as_str(),
                        danger
                    ),
                    value_state: ValueState::Inherited,
                    reason: "Theme-package default shadowed by an override.".to_owned(),
                },
            ],
            precedence_explained:
                "Managed-policy danger colour caps the profile override and the theme default."
                    .to_owned(),
            blocking_findings: Vec::new(),
        },
        ResolvedToken {
            record_kind: TOKEN_OVERLAY_RESOLVED_TOKEN_RECORD_KIND.to_owned(),
            token_ref: chart.to_owned(),
            token_family: TokenFamily::ColorChart,
            winning_scope: OverrideScope::ImportedTheme,
            winning_entry_ref: format!("entry:{}:{}", OverrideScope::ImportedTheme.as_str(), chart),
            winning_value_state: ValueState::Unmapped,
            shadowed: Vec::new(),
            precedence_explained:
                "Imported chart slot is unmapped; it stays an inert placeholder and is never applied."
                    .to_owned(),
            blocking_findings: Vec::new(),
        },
        ResolvedToken {
            record_kind: TOKEN_OVERLAY_RESOLVED_TOKEN_RECORD_KIND.to_owned(),
            token_ref: code.to_owned(),
            token_family: TokenFamily::TypographyRole,
            winning_scope: OverrideScope::ExtensionContributed,
            winning_entry_ref: format!(
                "entry:{}:{}",
                OverrideScope::ExtensionContributed.as_str(),
                code
            ),
            winning_value_state: ValueState::Deprecated,
            shadowed: vec![ShadowedEntry {
                scope: OverrideScope::ThemePackageDefault,
                entry_ref: format!(
                    "entry:{}:{}",
                    OverrideScope::ThemePackageDefault.as_str(),
                    code
                ),
                value_state: ValueState::Inherited,
                reason: "Theme-package default shadowed by an extension override.".to_owned(),
            }],
            precedence_explained:
                "Extension code-role override wins over the theme default but carries a deprecated-alias downgrade."
                    .to_owned(),
            blocking_findings: Vec::new(),
        },
        ResolvedToken {
            record_kind: TOKEN_OVERLAY_RESOLVED_TOKEN_RECORD_KIND.to_owned(),
            token_ref: spacing.to_owned(),
            token_family: TokenFamily::Spacing,
            winning_scope: OverrideScope::Profile,
            winning_entry_ref: format!("entry:{}:{}", OverrideScope::Profile.as_str(), spacing),
            winning_value_state: ValueState::Overridden,
            shadowed: vec![ShadowedEntry {
                scope: OverrideScope::UserGlobal,
                entry_ref: format!("entry:{}:{}", OverrideScope::UserGlobal.as_str(), spacing),
                value_state: ValueState::Overridden,
                reason: "Lower precedence than the profile override.".to_owned(),
            }],
            precedence_explained: "Profile row spacing wins over the user-global override."
                .to_owned(),
            blocking_findings: Vec::new(),
        },
    ];

    // The portable set traced across the round trip: entries that are both
    // exportable and syncable. The workspace cap (export-only) and the
    // policy cap (scope-local) are disclosed as non-portable at the entry level
    // and are deliberately excluded from the personal portable set.
    let trace = |entry_ref: String,
                 token_ref: &str,
                 scope: OverrideScope,
                 disposition: EntryDisposition,
                 downgrade_class: DowngradeClass,
                 explanation: &str| RoundTripEntryTrace {
        record_kind: TOKEN_OVERLAY_ROUND_TRIP_TRACE_RECORD_KIND.to_owned(),
        entry_ref,
        token_ref: token_ref.to_owned(),
        origin_scope: scope,
        final_scope: scope,
        disposition,
        downgrade_class,
        channels_traversed: vec![
            RoundTripChannel::ExportBundle,
            RoundTripChannel::ImportBundle,
            RoundTripChannel::SyncPush,
            RoundTripChannel::SyncPull,
        ],
        survived: disposition.survived(),
        explanation: explanation.to_owned(),
        blocking_findings: Vec::new(),
    };

    let entry_traces = vec![
        trace(
            format!("entry:{}:{}", OverrideScope::UserGlobal.as_str(), accent),
            accent,
            OverrideScope::UserGlobal,
            EntryDisposition::Preserved,
            DowngradeClass::None,
            "User-global accent survives export, import, and sync unchanged.",
        ),
        trace(
            format!("entry:{}:{}", OverrideScope::UserGlobal.as_str(), spacing),
            spacing,
            OverrideScope::UserGlobal,
            EntryDisposition::Preserved,
            DowngradeClass::None,
            "User-global row spacing survives the round trip unchanged.",
        ),
        trace(
            format!("entry:{}:{}", OverrideScope::Profile.as_str(), danger),
            danger,
            OverrideScope::Profile,
            EntryDisposition::Preserved,
            DowngradeClass::None,
            "Profile danger colour survives the round trip unchanged.",
        ),
        trace(
            format!("entry:{}:{}", OverrideScope::Profile.as_str(), spacing),
            spacing,
            OverrideScope::Profile,
            EntryDisposition::Preserved,
            DowngradeClass::None,
            "Profile row spacing survives the round trip unchanged.",
        ),
        trace(
            format!(
                "entry:{}:{}",
                OverrideScope::ExtensionContributed.as_str(),
                code
            ),
            code,
            OverrideScope::ExtensionContributed,
            EntryDisposition::Downgraded,
            DowngradeClass::DeprecatedAliasPendingReplacement,
            "Extension code-role override survives the round trip with a disclosed deprecated-alias downgrade.",
        ),
        trace(
            format!("entry:{}:{}", OverrideScope::ImportedTheme.as_str(), chart),
            chart,
            OverrideScope::ImportedTheme,
            EntryDisposition::Downgraded,
            DowngradeClass::InertUnsupportedToken,
            "Imported chart slot survives the round trip as an inert placeholder instead of being dropped.",
        ),
    ];

    let portable = entry_traces.len();
    let preserved = entry_traces
        .iter()
        .filter(|trace| trace.disposition == EntryDisposition::Preserved)
        .count();
    let downgraded = portable - preserved;

    let stage = |stage_id: &str,
                 sequence_index: u32,
                 channel: RoundTripChannel,
                 target_support_profile: &str,
                 preserved_count: usize,
                 downgraded_count: usize| RoundTripStage {
        record_kind: TOKEN_OVERLAY_ROUND_TRIP_STAGE_RECORD_KIND.to_owned(),
        stage_id: stage_id.to_owned(),
        sequence_index,
        channel,
        target_support_profile: target_support_profile.to_owned(),
        input_entry_count: portable,
        output_entry_count: portable,
        preserved_count,
        downgraded_count,
        dropped_count: 0,
        rewritten_count: 0,
        scope_preserved: true,
        blocking_findings: Vec::new(),
    };

    let stages = vec![
        stage(
            "round-trip-stage:export",
            0,
            RoundTripChannel::ExportBundle,
            "full_support",
            portable,
            0,
        ),
        stage(
            "round-trip-stage:import",
            1,
            RoundTripChannel::ImportBundle,
            "reduced_target",
            preserved,
            downgraded,
        ),
        stage(
            "round-trip-stage:sync-push",
            2,
            RoundTripChannel::SyncPush,
            "full_support",
            preserved,
            downgraded,
        ),
        stage(
            "round-trip-stage:sync-pull",
            3,
            RoundTripChannel::SyncPull,
            "full_support",
            preserved,
            downgraded,
        ),
    ];

    let round_trip = RoundTripProof {
        record_kind: TOKEN_OVERLAY_ROUND_TRIP_PROOF_RECORD_KIND.to_owned(),
        proof_id: "round-trip-proof:export-import-sync".to_owned(),
        stages,
        entry_traces,
        portable_entry_count: 0,
        unsupported_preserved_count: 0,
        lossless: true,
        blocking_findings: Vec::new(),
    };

    build_token_overlay_portability(overlays, resolved_tokens, round_trip)
}
