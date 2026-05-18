//! Keyboard-reachable command reference / help projection.
//!
//! This module turns the canonical command descriptor into the
//! first-class reference/help surface that the shell exposes through
//! the command-detail panel, palette deep-detail row, in-product
//! docs/help, CLI/headless help renderings, onboarding tips, and
//! support exports. Every consumer reads one structured
//! [`CommandReferenceEntry`] for a given stable command id rather
//! than maintaining a separate handwritten help record.
//!
//! The reference catalog is intentionally deterministic and seeded
//! so the JSON fixtures checked in under
//! `fixtures/ux/m3/command_reference_and_discoverability/` and the
//! markdown report at
//! `artifacts/ux/m3/command_reference_parity_report.md` stay
//! bit-for-bit equal to the structured projection. The same record
//! drives:
//!
//! - the live shell detail surface (palette deep row, docs/help
//!   browser, onboarding tip, command-parity inspector);
//! - the markdown parity report under
//!   `artifacts/ux/m3/command_reference_parity_report.md`; and
//! - the beta contract under
//!   `docs/ux/m3/command_reference_beta_contract.md`.
//!
//! The catalog covers the same beta-claimed commands the parity
//! report covers (see [`crate::command_parity`]) so a reviewer can
//! pivot from a parity blocker to the command's reference entry
//! using the same stable command id.

use serde::{Deserialize, Serialize};

pub mod render;
pub mod search;
pub mod seed;
pub mod validation;

/// Schema version exported with every command-reference record.
pub const COMMAND_REFERENCE_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every command reference
/// projection.
pub const COMMAND_REFERENCE_SHARED_CONTRACT_REF: &str = "shell:command_reference_beta:v1";

/// Stable record kind for [`CommandReferenceEntry`] payloads.
pub const COMMAND_REFERENCE_ENTRY_RECORD_KIND: &str = "command_reference_entry_record";

/// Stable record kind for [`CommandReferenceCatalog`] payloads.
pub const COMMAND_REFERENCE_CATALOG_RECORD_KIND: &str = "command_reference_catalog_record";

/// Stable catalog id consumed by the parity report and the beta
/// contract doc.
pub const COMMAND_REFERENCE_CATALOG_ID: &str = "shell:command_reference_beta:catalog:v1";

/// Source descriptor schema the reference projection is minted from.
pub const COMMAND_REFERENCE_DESCRIPTOR_SCHEMA_REF: &str =
    "schemas/commands/command_descriptor.schema.json";

/// Path of the published markdown parity report.
pub const COMMAND_REFERENCE_PUBLISHED_REPORT_REF: &str =
    "artifacts/ux/m3/command_reference_parity_report.md";

/// Path of the published companion beta contract doc.
pub const COMMAND_REFERENCE_PUBLISHED_DOC_REF: &str =
    "docs/ux/m3/command_reference_beta_contract.md";

/// Generation timestamp captured in every seeded record. Pinned so
/// the fixtures and rendered artifacts stay deterministic.
pub const COMMAND_REFERENCE_GENERATED_AT: &str = "2026-05-18T00:00:00Z";

/// Risk class pinned for the reference entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceRiskClass {
    InertMetadataOnly,
    ReversibleLocalRead,
    ReversibleLocalMutation,
    RecoverableDurableMutation,
    DestructiveBulkMutation,
    IrreversiblePublish,
}

impl ReferenceRiskClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InertMetadataOnly => "inert_metadata_only",
            Self::ReversibleLocalRead => "reversible_local_read",
            Self::ReversibleLocalMutation => "reversible_local_mutation",
            Self::RecoverableDurableMutation => "recoverable_durable_mutation",
            Self::DestructiveBulkMutation => "destructive_bulk_mutation",
            Self::IrreversiblePublish => "irreversible_publish",
        }
    }

    /// `true` when the risk class warrants preview / approval review.
    pub const fn is_high_risk(self) -> bool {
        matches!(
            self,
            Self::RecoverableDurableMutation
                | Self::DestructiveBulkMutation
                | Self::IrreversiblePublish
        )
    }
}

/// Preview class pinned for the reference entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferencePreviewClass {
    NoPreviewRequired,
    StructuredDiffPreview,
    DestructiveBulkMutationPreview,
    PolicyAuthoringOrWaiverPreview,
    IrreversiblePublishPreview,
}

impl ReferencePreviewClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoPreviewRequired => "no_preview_required",
            Self::StructuredDiffPreview => "structured_diff_preview",
            Self::DestructiveBulkMutationPreview => "destructive_bulk_mutation_preview",
            Self::PolicyAuthoringOrWaiverPreview => "policy_authoring_or_waiver_preview",
            Self::IrreversiblePublishPreview => "irreversible_publish_preview",
        }
    }

    /// `true` when the surface must show a preview before apply.
    pub const fn requires_preview(self) -> bool {
        !matches!(self, Self::NoPreviewRequired)
    }
}

/// Idempotency posture pinned for the reference entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceIdempotencyClass {
    Idempotent,
    IdempotentWithVisibleRedirect,
    NonIdempotentObservableOnly,
    NonIdempotentDestructive,
}

impl ReferenceIdempotencyClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Idempotent => "idempotent",
            Self::IdempotentWithVisibleRedirect => "idempotent_with_visible_redirect",
            Self::NonIdempotentObservableOnly => "non_idempotent_observable_only",
            Self::NonIdempotentDestructive => "non_idempotent_destructive",
        }
    }
}

/// Lifecycle state quoted by the reference entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceLifecycleState {
    Alpha,
    Beta,
    Stable,
    LtsFacing,
    Deprecated,
    Labs,
}

impl ReferenceLifecycleState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Alpha => "alpha",
            Self::Beta => "beta",
            Self::Stable => "stable",
            Self::LtsFacing => "lts_facing",
            Self::Deprecated => "deprecated",
            Self::Labs => "labs",
        }
    }
}

/// Surface family the reference entry lights up.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceSurfaceFamily {
    CommandPalette,
    MenuOrButton,
    KeybindingHelp,
    CliHeadless,
    AiToolSurface,
    DocsHelp,
    Onboarding,
}

impl ReferenceSurfaceFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandPalette => "command_palette",
            Self::MenuOrButton => "menu_or_button",
            Self::KeybindingHelp => "keybinding_help",
            Self::CliHeadless => "cli_headless",
            Self::AiToolSurface => "ai_tool_surface",
            Self::DocsHelp => "docs_help",
            Self::Onboarding => "onboarding",
        }
    }
}

/// Lifecycle state quoted on an alias reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AliasLifecycleState {
    Active,
    Deprecated,
    Retired,
}

impl AliasLifecycleState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Deprecated => "deprecated",
            Self::Retired => "retired",
        }
    }
}

/// Alias kind quoted on an alias reference. Matches the registry alias
/// taxonomy in `schemas/commands/command_registry_entry.schema.json`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AliasKind {
    LegacyCommandId,
    AlternatePalettePhrasing,
    AlternateCliVerb,
    AiToolHandle,
    KeybindingTarget,
}

impl AliasKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LegacyCommandId => "legacy_command_id",
            Self::AlternatePalettePhrasing => "alternate_palette_phrasing",
            Self::AlternateCliVerb => "alternate_cli_verb",
            Self::AiToolHandle => "ai_tool_handle",
            Self::KeybindingTarget => "keybinding_target",
        }
    }
}

/// Likely import impact a migrator faces when a command id, alias, or
/// chord is removed or renamed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportImpactClass {
    NoActionRequired,
    RebindKeymap,
    RewriteRecipe,
    AiToolHandleRenames,
    RemoveInvocation,
}

impl ImportImpactClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoActionRequired => "no_action_required",
            Self::RebindKeymap => "rebind_keymap",
            Self::RewriteRecipe => "rewrite_recipe",
            Self::AiToolHandleRenames => "ai_tool_handle_renames",
            Self::RemoveInvocation => "remove_invocation",
        }
    }
}

/// Binding state quoted by a [`KeybindingFact`]. Surfaces use these
/// labels verbatim so the in-product detail panel, the CLI help
/// dump, and the support export agree on whether a chord is active,
/// shadowed, or in conflict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeybindingState {
    Default,
    OverridingUserBinding,
    ShadowedByUserBinding,
    Conflict,
    Unassigned,
}

impl KeybindingState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::OverridingUserBinding => "overriding_user_binding",
            Self::ShadowedByUserBinding => "shadowed_by_user_binding",
            Self::Conflict => "conflict",
            Self::Unassigned => "unassigned",
        }
    }
}

/// Platform variant a chord applies to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformVariant {
    Macos,
    Windows,
    Linux,
    All,
}

impl PlatformVariant {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Macos => "macos",
            Self::Windows => "windows",
            Self::Linux => "linux",
            Self::All => "all",
        }
    }
}

/// Token class quoted by a [`SearchIndexToken`]. The classes drive
/// the same search index used by the command palette, the in-product
/// docs/help search box, the CLI help renderer, and onboarding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchTokenClass {
    HumanLabel,
    CommandId,
    CanonicalVerb,
    AliasId,
    KeySequence,
}

impl SearchTokenClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HumanLabel => "human_label",
            Self::CommandId => "command_id",
            Self::CanonicalVerb => "canonical_verb",
            Self::AliasId => "alias_id",
            Self::KeySequence => "key_sequence",
        }
    }
}

/// Automation label quoted on an [`AutomationEligibility`] section.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationLabel {
    UiOnly,
    HeadlessSafe,
    RecipeSafe,
    MacroSafe,
    AiCallableWithApproval,
    AiNotCallable,
}

impl AutomationLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UiOnly => "ui_only",
            Self::HeadlessSafe => "headless_safe",
            Self::RecipeSafe => "recipe_safe",
            Self::MacroSafe => "macro_safe",
            Self::AiCallableWithApproval => "ai_callable_with_approval",
            Self::AiNotCallable => "ai_not_callable",
        }
    }
}

/// One alias reference projected from the canonical descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AliasReference {
    pub alias_id: String,
    pub alias_kind: AliasKind,
    pub lifecycle_state: AliasLifecycleState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub introduced_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retirement_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement_command_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement_note_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub import_impact_class: Option<ImportImpactClass>,
}

/// Deprecation record on a command's reference entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeprecationRecord {
    pub state: AliasLifecycleState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated_in_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retires_in_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement_command_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub import_impact_class: Option<ImportImpactClass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migration_note_ref: Option<String>,
}

impl DeprecationRecord {
    /// Returns the active default for a stable command.
    pub fn active() -> Self {
        Self {
            state: AliasLifecycleState::Active,
            deprecated_in_version: None,
            retires_in_version: None,
            replacement_command_id: None,
            import_impact_class: None,
            migration_note_ref: None,
        }
    }
}

/// One typed argument slot quoted on the reference entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArgumentSchemaSlot {
    pub argument_name: String,
    pub argument_kind: String,
    pub is_required: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_provenance_when_omitted: Option<String>,
    pub narration_label_ref: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enum_value_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_pinned_when_trust_state_is: Vec<String>,
}

/// Availability section explaining trust/policy gates, dependency
/// presence, and the current disabled-reason codes the surface should
/// quote when the command is unavailable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AvailabilitySection {
    pub supported_surfaces: Vec<ReferenceSurfaceFamily>,
    pub trust_gate_class: String,
    pub policy_gate_class: String,
    pub dependency_presence_class: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub current_disabled_reason_codes: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub current_disabled_reason_explanation_refs: Vec<String>,
}

/// One keybinding fact quoted on the reference entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeybindingFact {
    pub chord_ref: String,
    pub platform_variant: PlatformVariant,
    pub binding_state: KeybindingState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shadowed_by_chord_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shadowed_by_command_id: Option<String>,
}

/// Automation eligibility section on the reference entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationEligibility {
    pub headless_eligible: bool,
    pub recipe_eligible: bool,
    pub macro_eligible: bool,
    pub ai_eligible: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub automation_labels: Vec<AutomationLabel>,
}

/// One search index token on the reference entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchIndexToken {
    pub token_class: SearchTokenClass,
    pub value: String,
}

/// Stable discoverability link the reference entry resolves back to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscoverabilityLink {
    pub surface_family: ReferenceSurfaceFamily,
    pub anchor_ref: String,
}

/// One command reference entry. This is the structured projection
/// surfaces read when they need to show the canonical detail panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandReferenceEntry {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub command_id: String,
    pub command_revision_ref: String,
    pub canonical_verb: String,
    pub primary_label_ref: String,
    pub title: String,
    pub summary: String,
    pub lifecycle_state: ReferenceLifecycleState,
    pub origin_class: String,
    pub risk_class: ReferenceRiskClass,
    pub preview_class: ReferencePreviewClass,
    pub idempotency_class: ReferenceIdempotencyClass,
    pub supports_dry_run: bool,
    pub aliases: Vec<AliasReference>,
    pub deprecation: DeprecationRecord,
    pub argument_schema: Vec<ArgumentSchemaSlot>,
    pub availability: AvailabilitySection,
    pub keybindings: Vec<KeybindingFact>,
    pub automation: AutomationEligibility,
    pub search_index: Vec<SearchIndexToken>,
    pub discoverability_links: Vec<DiscoverabilityLink>,
    pub docs_help_anchor_ref: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub migration_notes_refs: Vec<String>,
    pub generated_at: String,
}

impl CommandReferenceEntry {
    /// Returns the canonical search tokens for this entry. The tokens
    /// include the human label, command id, canonical verb, every
    /// alias id, and every chord ref so the palette, docs/help, and
    /// CLI surfaces resolve by label, id, alias, or literal key
    /// sequence.
    pub fn iter_search_tokens(&self) -> impl Iterator<Item = &SearchIndexToken> {
        self.search_index.iter()
    }
}

/// A catalog of command reference entries. The catalog is the
/// single mint-from-truth source consumed by the shell detail
/// surface, the parity report, and the docs/help index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandReferenceCatalog {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub catalog_id: String,
    pub source_descriptor_schema_ref: String,
    pub entries: Vec<CommandReferenceEntry>,
    pub generated_at: String,
}

impl CommandReferenceCatalog {
    /// Finds the entry for a stable command id.
    pub fn entry(&self, command_id: &str) -> Option<&CommandReferenceEntry> {
        self.entries
            .iter()
            .find(|entry| entry.command_id == command_id)
    }

    /// Returns the count of entries marked deprecated.
    pub fn deprecated_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.deprecation.state != AliasLifecycleState::Active)
            .count()
    }

    /// Returns the count of high-risk entries.
    pub fn high_risk_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.risk_class.is_high_risk())
            .count()
    }
}

pub use render::render_catalog_markdown;
pub use search::{search_entries, SearchHit, SearchMatchKind};
pub use seed::seeded_command_reference_catalog;
pub use validation::{
    validate_command_reference_catalog, validate_command_reference_entry,
    CommandReferenceValidationError,
};
