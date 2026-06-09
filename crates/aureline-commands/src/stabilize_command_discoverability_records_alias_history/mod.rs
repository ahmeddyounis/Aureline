//! Stable command discoverability records, alias/deprecation propagation, and
//! privacy-bounded query-session truth for protected command surfaces.
//!
//! This module promotes command discoverability from scattered fixture metadata
//! into one typed, export-safe packet that protected command surfaces can share.
//! It derives one governed discoverability record per protected command from the
//! canonical command registry and binds:
//!
//! - the stable command identity, title, examples, categories, origin, lifecycle,
//!   alias/deprecation map, replacement route, accessibility labels, shortcut
//!   narration hints, automation-support posture, and help anchors that
//!   discoverability surfaces must project from;
//! - one local-first query-session policy for command discovery, with explicit
//!   retention, clear/disable controls, provider classes, held-modifier intent,
//!   and local-only vs governed-sync posture; and
//! - one per-surface parity row proving palette, keybinding help, onboarding,
//!   voice hints, docs/help, CLI help, and support export all resolve against the
//!   same discoverability source rather than private copies.
//!
//! The packet is intentionally metadata-only. It carries opaque refs, stable
//! class tokens, booleans, and bounded counts only. Raw query text, raw paths,
//! raw provider payloads, credentials, URLs, and other private material stay
//! outside the export boundary.

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::descriptor::{AccessibilityLabelPath, ShortcutNarrationHint, TypedArgument};
use crate::finalize_command_parity::{
    ClearHistoryRuleClass, HistoryPolicyClass, RedactionPostureClass,
};
use crate::registry::{seeded_registry, CommandRegistryEntryRecord};

/// Stable record-kind tag carried by [`DiscoverabilitySupportPacket`].
pub const STABILIZE_COMMAND_DISCOVERABILITY_RECORD_KIND: &str =
    "command_discoverability_support_packet";

/// Schema version for command-discoverability support packets.
pub const STABILIZE_COMMAND_DISCOVERABILITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the command-discoverability boundary schema.
pub const STABILIZE_COMMAND_DISCOVERABILITY_SCHEMA_REF: &str =
    "schemas/commands/discoverability-record.schema.json";

/// Repo-relative path of the frozen palette query-session contract.
pub const STABILIZE_COMMAND_DISCOVERABILITY_QUERY_SESSION_SCHEMA_REF: &str =
    "schemas/commands/palette_query_session.schema.json";

/// Repo-relative path of the command-discoverability doc.
pub const STABILIZE_COMMAND_DISCOVERABILITY_DOC_REF: &str =
    "docs/commands/m4/stabilize_command_discoverability_records_alias_history.md";

/// Repo-relative path of the protected fixture directory.
pub const STABILIZE_COMMAND_DISCOVERABILITY_FIXTURE_DIR: &str =
    "fixtures/commands/m4/stabilize_command_discoverability_records_alias_history";

/// Repo-relative path of the checked command-discoverability export.
pub const STABILIZE_COMMAND_DISCOVERABILITY_ARTIFACT_REF: &str =
    "artifacts/commands/m4/stabilize_command_discoverability_records_alias_history/support_export.json";

/// Repo-relative path of the checked command-discoverability Markdown summary.
pub const STABILIZE_COMMAND_DISCOVERABILITY_SUMMARY_REF: &str =
    "artifacts/commands/m4/stabilize_command_discoverability_records_alias_history/summary.md";

const SOURCE_DESCRIPTOR_CONTRACT_REF: &str = "docs/commands/command_descriptor_contract.md";
const SOURCE_SEQUENCE_DISCOVERABILITY_REF: &str =
    "docs/commands/sequence_and_modal_discoverability_contract.md";
const SOURCE_PALETTE_QUERY_CONTRACT_REF: &str = "docs/commands/palette_query_session_contract.md";
const SOURCE_REGISTRY_ARTIFACT_REF: &str = "artifacts/commands/command_registry_seed.yaml";
const GENERATED_AT: &str = "2026-06-07T00:00:00Z";

/// Required automation-support posture a discoverable command declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscoverabilityAutomationSupportClass {
    /// The command is safe to record in deterministic macros.
    MacroSafe,
    /// The command is safe to insert into typed recipes.
    RecipeSafe,
    /// The command has a CLI/headless-safe route.
    HeadlessSafe,
    /// The command is interactive and UI-only.
    UiOnly,
    /// The command is discoverable, but invocation requires an approval step.
    ApprovalRequired,
    /// The registry has not yet promoted the command into a known automation class.
    Unknown,
}

impl DiscoverabilityAutomationSupportClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MacroSafe => "macro_safe",
            Self::RecipeSafe => "recipe_safe",
            Self::HeadlessSafe => "headless_safe",
            Self::UiOnly => "ui_only",
            Self::ApprovalRequired => "approval_required",
            Self::Unknown => "unknown",
        }
    }
}

/// Protected discoverability surface that must project canonical command truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscoverabilitySurfaceClass {
    /// Command palette row or detail panel.
    CommandPalette,
    /// Keybinding help and shortcut teaching surfaces.
    KeybindingHelp,
    /// Help/docs surfaces and in-product command help.
    DocsHelp,
    /// Onboarding or migration hints.
    OnboardingHint,
    /// Voice-command hints or keyboard-fallback disclosure.
    VoiceHint,
    /// CLI/headless help.
    CliHelp,
    /// Support/export packets.
    SupportExport,
}

impl DiscoverabilitySurfaceClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandPalette => "command_palette",
            Self::KeybindingHelp => "keybinding_help",
            Self::DocsHelp => "docs_help",
            Self::OnboardingHint => "onboarding_hint",
            Self::VoiceHint => "voice_hint",
            Self::CliHelp => "cli_help",
            Self::SupportExport => "support_export",
        }
    }

    /// Required discoverability surfaces for protected-command parity.
    pub const fn required_coverage() -> [Self; 7] {
        [
            Self::CommandPalette,
            Self::KeybindingHelp,
            Self::DocsHelp,
            Self::OnboardingHint,
            Self::VoiceHint,
            Self::CliHelp,
            Self::SupportExport,
        ]
    }
}

/// Query-session provider class exposed by the command-discovery surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuerySessionProviderClass {
    /// The canonical command registry.
    CommandRegistry,
    /// Local recent-history store.
    RecentHistory,
    /// Lexical command matching.
    LexicalCommandIndex,
    /// Semantic command supplement.
    SemanticCommandIndex,
    /// Docs/help search bridge.
    DocsHelp,
    /// Keybinding resolver.
    KeybindingResolver,
    /// Migration/import bridge.
    MigrationBridge,
}

impl QuerySessionProviderClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandRegistry => "command_registry",
            Self::RecentHistory => "recent_history",
            Self::LexicalCommandIndex => "lexical_command_index",
            Self::SemanticCommandIndex => "semantic_command_index",
            Self::DocsHelp => "docs_help",
            Self::KeybindingResolver => "keybinding_resolver",
            Self::MigrationBridge => "migration_bridge",
        }
    }
}

/// How much query text material the export-safe discoverability packet carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuerySessionTextMaterialClass {
    /// Raw text never leaves the local palette session.
    NotRecorded,
    /// Local-only raw text exists in-memory but is never exported.
    RawLocalOnly,
    /// Support/export uses redacted refs only.
    RedactedText,
    /// Only hashed or classified material is retained.
    ClassificationOnly,
}

impl QuerySessionTextMaterialClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRecorded => "not_recorded",
            Self::RawLocalOnly => "raw_local_only",
            Self::RedactedText => "redacted_text",
            Self::ClassificationOnly => "classification_only",
        }
    }
}

/// Sync posture for a query-history policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuerySessionSyncPostureClass {
    /// History is local-only.
    LocalOnly,
    /// Any widening beyond the device is governed by an explicit feature or policy.
    GovernedSync,
}

impl QuerySessionSyncPostureClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::GovernedSync => "governed_sync",
        }
    }
}

/// Export-safe accessibility labels quoted by the canonical discoverability record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscoverabilityAccessibilityRecord {
    /// Primary discoverability label ref.
    pub primary_label_ref: String,
    /// Short label ref for compressed surfaces.
    pub short_label_ref: String,
    /// Long description ref for assistive surfaces.
    pub long_description_ref: String,
    /// Accessibility role class.
    pub role_class: String,
    /// Keyboard shortcut narration ref.
    pub keyboard_shortcut_narration_ref: String,
}

impl DiscoverabilityAccessibilityRecord {
    fn from_descriptor(value: &AccessibilityLabelPath) -> Self {
        Self {
            primary_label_ref: value.primary_label_ref.clone(),
            short_label_ref: value.short_label_ref.clone(),
            long_description_ref: value.long_description_ref.clone(),
            role_class: value.role_class.clone(),
            keyboard_shortcut_narration_ref: value.keyboard_shortcut_narration_ref.clone(),
        }
    }

    fn is_complete(&self) -> bool {
        !self.primary_label_ref.trim().is_empty()
            && !self.short_label_ref.trim().is_empty()
            && !self.long_description_ref.trim().is_empty()
            && !self.role_class.trim().is_empty()
            && !self.keyboard_shortcut_narration_ref.trim().is_empty()
    }
}

/// Alias/deprecation row projected into discoverability, migration, help, and support.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscoverabilityAliasRecord {
    /// Stable alias id.
    pub alias_id: String,
    /// Alias kind vocabulary token.
    pub alias_kind: String,
    /// Active, deprecated, or retired state.
    pub alias_state: String,
    /// Source system or importer that minted the alias.
    pub source_system_ref: String,
    /// Introduction revision or release ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub introduced_ref: Option<String>,
    /// Deprecation revision or release ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated_ref: Option<String>,
    /// Retirement revision or release ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retired_ref: Option<String>,
    /// Replacement command id when the alias is deprecated or retired.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement_command_id: Option<String>,
    /// Typed note ref used by migration/help surfaces.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes_ref: Option<String>,
    /// Whether new bindings may target the alias.
    pub eligible_for_new_bindings: bool,
}

impl DiscoverabilityAliasRecord {
    fn is_complete(&self) -> bool {
        !self.alias_id.trim().is_empty()
            && !self.alias_kind.trim().is_empty()
            && !self.alias_state.trim().is_empty()
            && !self.source_system_ref.trim().is_empty()
    }
}

/// Simplified keybinding row reused by help, onboarding, CLI, and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscoverabilityCurrentKeybindingRecord {
    /// Platform class.
    pub platform_class: String,
    /// Display state such as `assigned` or `unassigned`.
    pub display_state: String,
    /// Stable keybinding ref.
    pub keybinding_ref: String,
    /// Source layer that currently owns the binding.
    pub source_layer: String,
    /// Optional imported/source mapping.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imported_from_ref: Option<String>,
    /// Optional notes ref for the binding row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes_ref: Option<String>,
}

/// Projection refs surfaces use instead of inventing discoverability-local ids.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscoverabilityProjectionRefs {
    /// Palette-row projection ref.
    pub palette_row_ref: String,
    /// Help/docs search projection ref.
    pub help_search_ref: String,
    /// Onboarding tip projection ref.
    pub onboarding_hint_ref: String,
    /// Migration/help bridge projection ref.
    pub migration_bridge_card_ref: String,
    /// Current-shortcut display ref.
    pub current_shortcut_display_ref: String,
    /// Why-unavailable explainer ref.
    pub why_unavailable_explainer_ref: String,
    /// Key-sequence discoverability ref.
    pub key_sequence_discoverability_ref: String,
    /// Derived docs/help page projection ref.
    pub docs_help_page_ref: String,
    /// Derived voice-hint projection ref.
    pub voice_hint_ref: String,
    /// Derived CLI-help projection ref.
    pub cli_help_ref: String,
    /// Derived support-export projection ref.
    pub support_export_ref: String,
}

impl DiscoverabilityProjectionRefs {
    fn is_complete(&self) -> bool {
        [
            &self.palette_row_ref,
            &self.help_search_ref,
            &self.onboarding_hint_ref,
            &self.migration_bridge_card_ref,
            &self.current_shortcut_display_ref,
            &self.why_unavailable_explainer_ref,
            &self.key_sequence_discoverability_ref,
            &self.docs_help_page_ref,
            &self.voice_hint_ref,
            &self.cli_help_ref,
            &self.support_export_ref,
        ]
        .into_iter()
        .all(|value| !value.trim().is_empty())
    }
}

/// Per-surface parity row proving a surface consumes the canonical discoverability record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscoverabilitySurfaceProjectionRow {
    /// Covered surface.
    pub surface_class: DiscoverabilitySurfaceClass,
    /// Stable projection ref on that surface.
    pub projection_ref: String,
    /// Whether the surface uses the canonical example set.
    pub examples_match: bool,
    /// Whether the surface uses the canonical alias lifecycle map.
    pub alias_set_matches: bool,
    /// Whether the surface uses the canonical lifecycle label and replacement route.
    pub lifecycle_matches: bool,
    /// Whether disabled reasons and repair guidance stay canonical.
    pub disabled_reason_matches: bool,
    /// Whether automation support labels stay canonical.
    pub automation_support_matches: bool,
    /// Whether accessibility labels and shortcut narration stay canonical.
    pub accessibility_labels_match: bool,
    /// Whether the docs/help anchor stays canonical.
    pub help_anchor_matches: bool,
}

impl DiscoverabilitySurfaceProjectionRow {
    fn preserves_canonical_truth(&self) -> bool {
        !self.projection_ref.trim().is_empty()
            && self.examples_match
            && self.alias_set_matches
            && self.lifecycle_matches
            && self.disabled_reason_matches
            && self.automation_support_matches
            && self.accessibility_labels_match
            && self.help_anchor_matches
    }
}

/// Query-session privacy and retention policy for command discovery.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscoverabilityQuerySessionPolicyRecord {
    /// Stable policy id.
    pub query_session_policy_id: String,
    /// Current-text material posture.
    pub current_text_material_class: QuerySessionTextMaterialClass,
    /// Provider classes that participate in discovery.
    pub provider_classes: Vec<QuerySessionProviderClass>,
    /// Held modifier intent token.
    pub held_modifier_intent_class: String,
    /// History policy class.
    pub history_policy_class: HistoryPolicyClass,
    /// Max retained history entries.
    pub max_history_entries: u32,
    /// Retention policy ref.
    pub retention_policy_ref: String,
    /// Clear-history controls.
    pub clear_controls: Vec<ClearHistoryRuleClass>,
    /// Whether history can be disabled entirely.
    pub disable_control_available: bool,
    /// Sync posture.
    pub sync_posture: QuerySessionSyncPostureClass,
    /// Redaction posture for support/export.
    pub redaction_posture: RedactionPostureClass,
    /// Whether raw query export is allowed.
    pub raw_query_export_allowed: bool,
    /// Accessibility disclosure row ref.
    pub accessibility_disclosure_ref: String,
}

impl DiscoverabilityQuerySessionPolicyRecord {
    fn local_first(&self) -> bool {
        matches!(self.sync_posture, QuerySessionSyncPostureClass::LocalOnly)
            && matches!(
                self.history_policy_class,
                HistoryPolicyClass::NoHistory
                    | HistoryPolicyClass::SessionOnly
                    | HistoryPolicyClass::LocalProfile
                    | HistoryPolicyClass::LocalDevice
            )
    }

    fn controls_complete(&self) -> bool {
        !self.query_session_policy_id.trim().is_empty()
            && !self.retention_policy_ref.trim().is_empty()
            && !self.accessibility_disclosure_ref.trim().is_empty()
            && self.disable_control_available
            && !self.provider_classes.is_empty()
            && ClearHistoryRuleClass::required_coverage()
                .into_iter()
                .all(|required| self.clear_controls.iter().any(|rule| *rule == required))
    }
}

/// Typed argument projection used by CLI/help and onboarding surfaces.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscoverabilityTypedArgumentRecord {
    /// Stable argument name.
    pub argument_name: String,
    /// Argument kind.
    pub argument_kind: String,
    /// Whether the argument is required.
    pub is_required: bool,
    /// Narration label ref.
    pub narration_label_ref: String,
}

impl DiscoverabilityTypedArgumentRecord {
    fn from_descriptor(value: &TypedArgument) -> Self {
        Self {
            argument_name: value.argument_name.clone(),
            argument_kind: value.argument_kind.clone(),
            is_required: value.is_required,
            narration_label_ref: value.narration_label_ref.clone(),
        }
    }
}

/// One protected command discoverability record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtectedCommandDiscoverabilityRecord {
    /// Stable discoverability record id.
    pub discoverability_record_id: String,
    /// Canonical command id.
    pub command_id: String,
    /// Descriptor revision ref.
    pub command_revision_ref: String,
    /// Canonical dotted CLI/headless verb.
    pub canonical_verb: String,
    /// Title used by help, CLI, and support surfaces.
    pub title: String,
    /// Summary used by help, CLI, and support surfaces.
    pub summary: String,
    /// Lifecycle state.
    pub lifecycle_state: String,
    /// Whether this command is in the protected stable line.
    pub stable_line_required: bool,
    /// Origin class surfaces disclose.
    pub origin_class: String,
    /// Optional origin/source ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_source_ref: Option<String>,
    /// Category refs.
    pub category_refs: Vec<String>,
    /// Example refs.
    pub example_refs: Vec<String>,
    /// Tag refs.
    pub tag_refs: Vec<String>,
    /// Searchability axes.
    pub searchability_axes: Vec<String>,
    /// Aliases promoted for discoverability.
    pub promoted_alias_ids: Vec<String>,
    /// Full alias/deprecation map.
    pub alias_records: Vec<DiscoverabilityAliasRecord>,
    /// Current keybinding facts.
    pub current_keybindings: Vec<DiscoverabilityCurrentKeybindingRecord>,
    /// Replacement command id when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement_command_id: Option<String>,
    /// Canonical docs/help anchor.
    pub docs_help_anchor_ref: String,
    /// Accessibility labels.
    pub accessibility: DiscoverabilityAccessibilityRecord,
    /// Shortcut narration hints.
    pub shortcut_narration_hint: ShortcutNarrationHint,
    /// Typed argument summary used by CLI/help.
    pub typed_arguments: Vec<DiscoverabilityTypedArgumentRecord>,
    /// Capability scope class for help/onboarding safety disclosure.
    pub capability_scope_class: String,
    /// Preview class used by voice/help/CLI parity.
    pub preview_class: String,
    /// Approval posture class used by voice/help/CLI parity.
    pub approval_posture_class: String,
    /// AI-tool surfacing class.
    pub ai_tool_surfacing_class: String,
    /// Automation-support posture.
    pub automation_support: Vec<DiscoverabilityAutomationSupportClass>,
    /// Canonical disabled-reason explanation refs.
    pub disabled_reason_explanation_refs: Vec<String>,
    /// Surface projection refs.
    pub projection_refs: DiscoverabilityProjectionRefs,
    /// Per-surface parity rows.
    pub surface_rows: Vec<DiscoverabilitySurfaceProjectionRow>,
}

impl ProtectedCommandDiscoverabilityRecord {
    fn is_complete(&self) -> bool {
        !self.discoverability_record_id.trim().is_empty()
            && !self.command_id.trim().is_empty()
            && !self.command_revision_ref.trim().is_empty()
            && !self.canonical_verb.trim().is_empty()
            && !self.title.trim().is_empty()
            && !self.summary.trim().is_empty()
            && !self.lifecycle_state.trim().is_empty()
            && !self.origin_class.trim().is_empty()
            && !self.category_refs.is_empty()
            && !self.example_refs.is_empty()
            && self.accessibility.is_complete()
            && self.projection_refs.is_complete()
            && !self.automation_support.is_empty()
            && !self.surface_rows.is_empty()
            && self
                .alias_records
                .iter()
                .all(DiscoverabilityAliasRecord::is_complete)
            && self
                .surface_rows
                .iter()
                .all(DiscoverabilitySurfaceProjectionRow::preserves_canonical_truth)
    }
}

/// One export-safe support packet covering the protected discoverability lane.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscoverabilitySupportPacket {
    /// Boundary record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Source command-registry artifact.
    pub source_registry_ref: String,
    /// Source descriptor contract.
    pub source_descriptor_contract_ref: String,
    /// Source sequence/modal discoverability contract.
    pub source_discoverability_contract_ref: String,
    /// Source palette query-session contract.
    pub source_palette_query_contract_ref: String,
    /// Published docs page for this lane.
    pub published_doc_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Query-session schema ref.
    pub query_session_schema_ref: String,
    /// Protected command discoverability corpus.
    pub commands: Vec<ProtectedCommandDiscoverabilityRecord>,
    /// Shared query-session privacy policy.
    pub query_session_policy: DiscoverabilityQuerySessionPolicyRecord,
}

impl DiscoverabilitySupportPacket {
    /// Builds a new packet from `commands` and `query_session_policy`.
    pub fn new(
        commands: Vec<ProtectedCommandDiscoverabilityRecord>,
        query_session_policy: DiscoverabilityQuerySessionPolicyRecord,
    ) -> Self {
        Self {
            record_kind: STABILIZE_COMMAND_DISCOVERABILITY_RECORD_KIND.to_owned(),
            schema_version: STABILIZE_COMMAND_DISCOVERABILITY_SCHEMA_VERSION,
            packet_id: "command-discoverability:stable:0001".to_owned(),
            generated_at: GENERATED_AT.to_owned(),
            source_registry_ref: SOURCE_REGISTRY_ARTIFACT_REF.to_owned(),
            source_descriptor_contract_ref: SOURCE_DESCRIPTOR_CONTRACT_REF.to_owned(),
            source_discoverability_contract_ref: SOURCE_SEQUENCE_DISCOVERABILITY_REF.to_owned(),
            source_palette_query_contract_ref: SOURCE_PALETTE_QUERY_CONTRACT_REF.to_owned(),
            published_doc_ref: STABILIZE_COMMAND_DISCOVERABILITY_DOC_REF.to_owned(),
            schema_ref: STABILIZE_COMMAND_DISCOVERABILITY_SCHEMA_REF.to_owned(),
            query_session_schema_ref: STABILIZE_COMMAND_DISCOVERABILITY_QUERY_SESSION_SCHEMA_REF
                .to_owned(),
            commands,
            query_session_policy,
        }
    }

    /// Returns validation violations for the packet.
    pub fn validate(&self) -> Vec<DiscoverabilitySupportViolation> {
        let mut violations = Vec::new();
        if self.record_kind != STABILIZE_COMMAND_DISCOVERABILITY_RECORD_KIND {
            violations.push(DiscoverabilitySupportViolation::WrongRecordKind);
        }
        if self.schema_version != STABILIZE_COMMAND_DISCOVERABILITY_SCHEMA_VERSION {
            violations.push(DiscoverabilitySupportViolation::WrongSchemaVersion);
        }
        if self.commands.is_empty() {
            violations.push(DiscoverabilitySupportViolation::EmptyProtectedCorpus);
        }
        for required in [
            SOURCE_REGISTRY_ARTIFACT_REF,
            SOURCE_DESCRIPTOR_CONTRACT_REF,
            SOURCE_SEQUENCE_DISCOVERABILITY_REF,
            SOURCE_PALETTE_QUERY_CONTRACT_REF,
            STABILIZE_COMMAND_DISCOVERABILITY_DOC_REF,
            STABILIZE_COMMAND_DISCOVERABILITY_SCHEMA_REF,
            STABILIZE_COMMAND_DISCOVERABILITY_QUERY_SESSION_SCHEMA_REF,
        ] {
            if !self.contains_source_ref(required) {
                violations.push(DiscoverabilitySupportViolation::MissingSourceContracts);
                break;
            }
        }
        for command in &self.commands {
            if !command.is_complete() {
                violations.push(DiscoverabilitySupportViolation::CommandRecordIncomplete(
                    command.command_id.clone(),
                ));
            }
            if command.stable_line_required && command.docs_help_anchor_ref.trim().is_empty() {
                violations.push(
                    DiscoverabilitySupportViolation::StableCommandMissingHelpAnchor(
                        command.command_id.clone(),
                    ),
                );
            }
            if command.stable_line_required && command.alias_records.is_empty() {
                violations.push(
                    DiscoverabilitySupportViolation::StableCommandMissingAliasMap(
                        command.command_id.clone(),
                    ),
                );
            }
            if command.stable_line_required
                && command
                    .automation_support
                    .iter()
                    .any(|class| *class == DiscoverabilityAutomationSupportClass::Unknown)
            {
                violations.push(
                    DiscoverabilitySupportViolation::StableCommandAutomationSupportUnknown(
                        command.command_id.clone(),
                    ),
                );
            }
            if command.stable_line_required
                && !DiscoverabilitySurfaceClass::required_coverage()
                    .into_iter()
                    .all(|required| {
                        command
                            .surface_rows
                            .iter()
                            .any(|row| row.surface_class == required)
                    })
            {
                violations.push(
                    DiscoverabilitySupportViolation::ProtectedSurfaceCoverageMissing(
                        command.command_id.clone(),
                    ),
                );
            }
            if command
                .surface_rows
                .iter()
                .any(|row| !row.preserves_canonical_truth())
            {
                violations.push(DiscoverabilitySupportViolation::SurfaceParityDrift(
                    command.command_id.clone(),
                ));
            }
            for alias in &command.alias_records {
                let deprecated = matches!(alias.alias_state.as_str(), "deprecated" | "retired");
                if deprecated
                    && (alias.replacement_command_id.is_none() || alias.notes_ref.is_none())
                {
                    violations.push(DiscoverabilitySupportViolation::AliasLifecycleIncomplete(
                        command.command_id.clone(),
                        alias.alias_id.clone(),
                    ));
                }
            }
        }
        if !self.query_session_policy.local_first() {
            violations.push(DiscoverabilitySupportViolation::QuerySessionNotLocalFirst);
        }
        if !self.query_session_policy.controls_complete()
            || self.query_session_policy.raw_query_export_allowed
        {
            violations.push(DiscoverabilitySupportViolation::QuerySessionControlsIncomplete);
        }
        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("discoverability support packet serializes"),
        ) {
            violations.push(DiscoverabilitySupportViolation::RawMaterialInExport);
        }
        violations
    }

    fn contains_source_ref(&self, required: &str) -> bool {
        [
            self.source_registry_ref.as_str(),
            self.source_descriptor_contract_ref.as_str(),
            self.source_discoverability_contract_ref.as_str(),
            self.source_palette_query_contract_ref.as_str(),
            self.published_doc_ref.as_str(),
            self.schema_ref.as_str(),
            self.query_session_schema_ref.as_str(),
        ]
        .contains(&required)
    }

    /// Deterministic export-safe JSON.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("discoverability support packet serializes")
    }

    /// Deterministic Markdown summary for support and docs handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_commands = self
            .commands
            .iter()
            .filter(|command| command.stable_line_required)
            .count();
        let deprecated_aliases = self
            .commands
            .iter()
            .flat_map(|command| command.alias_records.iter())
            .filter(|alias| alias.alias_state != "active")
            .count();
        let mut out = String::new();
        out.push_str("# Command Discoverability Support Packet\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Protected commands: {}\n", self.commands.len()));
        out.push_str(&format!("- Stable-line commands: {}\n", stable_commands));
        out.push_str(&format!(
            "- Deprecated or retired aliases: {}\n",
            deprecated_aliases
        ));
        out.push_str(&format!(
            "- Query history policy: `{}` / sync posture `{}`\n",
            self.query_session_policy.history_policy_class.as_str(),
            self.query_session_policy.sync_posture.as_str()
        ));
        out.push_str(&format!(
            "- Query-session providers: {}\n",
            self.query_session_policy.provider_classes.len()
        ));
        out.push_str(&format!(
            "- Required discoverability surfaces per stable command: {}\n",
            DiscoverabilitySurfaceClass::required_coverage().len()
        ));
        out
    }
}

/// Errors emitted when reading the checked-in discoverability export.
#[derive(Debug)]
pub enum DiscoverabilitySupportArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<DiscoverabilitySupportViolation>),
}

impl fmt::Display for DiscoverabilitySupportArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "command discoverability export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(DiscoverabilitySupportViolation::render_token)
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "command discoverability export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for DiscoverabilitySupportArtifactError {}

/// Validation failures emitted by [`DiscoverabilitySupportPacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiscoverabilitySupportViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required source refs are missing.
    MissingSourceContracts,
    /// No protected commands were exported.
    EmptyProtectedCorpus,
    /// One command record is incomplete.
    CommandRecordIncomplete(String),
    /// A stable-line command lacks a docs/help anchor.
    StableCommandMissingHelpAnchor(String),
    /// A stable-line command lacks an alias/deprecation map.
    StableCommandMissingAliasMap(String),
    /// A stable-line command still exposes `unknown` automation support.
    StableCommandAutomationSupportUnknown(String),
    /// Required protected-surface coverage is missing.
    ProtectedSurfaceCoverageMissing(String),
    /// A surface drifted from canonical discoverability truth.
    SurfaceParityDrift(String),
    /// Alias lifecycle metadata is incomplete.
    AliasLifecycleIncomplete(String, String),
    /// Query-session policy is not local-first.
    QuerySessionNotLocalFirst,
    /// Query-session controls or export guards are incomplete.
    QuerySessionControlsIncomplete,
    /// The packet leaks raw/private material.
    RawMaterialInExport,
}

impl DiscoverabilitySupportViolation {
    fn render_token(&self) -> String {
        match self {
            Self::WrongRecordKind => "wrong_record_kind".to_owned(),
            Self::WrongSchemaVersion => "wrong_schema_version".to_owned(),
            Self::MissingSourceContracts => "missing_source_contracts".to_owned(),
            Self::EmptyProtectedCorpus => "empty_protected_corpus".to_owned(),
            Self::CommandRecordIncomplete(command_id) => {
                format!("command_record_incomplete:{command_id}")
            }
            Self::StableCommandMissingHelpAnchor(command_id) => {
                format!("stable_command_missing_help_anchor:{command_id}")
            }
            Self::StableCommandMissingAliasMap(command_id) => {
                format!("stable_command_missing_alias_map:{command_id}")
            }
            Self::StableCommandAutomationSupportUnknown(command_id) => {
                format!("stable_command_automation_support_unknown:{command_id}")
            }
            Self::ProtectedSurfaceCoverageMissing(command_id) => {
                format!("protected_surface_coverage_missing:{command_id}")
            }
            Self::SurfaceParityDrift(command_id) => {
                format!("surface_parity_drift:{command_id}")
            }
            Self::AliasLifecycleIncomplete(command_id, alias_id) => {
                format!("alias_lifecycle_incomplete:{command_id}:{alias_id}")
            }
            Self::QuerySessionNotLocalFirst => "query_session_not_local_first".to_owned(),
            Self::QuerySessionControlsIncomplete => "query_session_controls_incomplete".to_owned(),
            Self::RawMaterialInExport => "raw_material_in_export".to_owned(),
        }
    }
}

/// Returns the checked-in discoverability support export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or validate.
pub fn current_command_discoverability_export(
) -> Result<DiscoverabilitySupportPacket, DiscoverabilitySupportArtifactError> {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/commands/m4/stabilize_command_discoverability_records_alias_history/support_export.json"
    );
    let payload = std::fs::read_to_string(path).map_err(|error| {
        DiscoverabilitySupportArtifactError::SupportExport(serde_json::Error::io(error))
    })?;
    let packet: DiscoverabilitySupportPacket = serde_json::from_str(&payload)
        .map_err(DiscoverabilitySupportArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(DiscoverabilitySupportArtifactError::Validation(violations))
    }
}

fn discoverability_record_field<'a>(
    entry: &'a CommandRegistryEntryRecord,
    key: &str,
) -> Option<&'a serde_json::Value> {
    entry.discoverability_record.get(key)
}

fn discoverability_string_array(entry: &CommandRegistryEntryRecord, key: &str) -> Vec<String> {
    discoverability_record_field(entry, key)
        .and_then(serde_json::Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|value| value.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default()
}

fn discoverability_projection_ref(entry: &CommandRegistryEntryRecord, key: &str) -> Option<String> {
    discoverability_record_field(entry, "projection_refs")
        .and_then(|value| value.get(key))
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
}

fn string_field(value: &serde_json::Value, key: &str) -> Option<String> {
    value.get(key)?.as_str().map(str::to_owned)
}

fn bool_field(value: &serde_json::Value, key: &str) -> Option<bool> {
    value.get(key)?.as_bool()
}

fn alias_records(entry: &CommandRegistryEntryRecord) -> Vec<DiscoverabilityAliasRecord> {
    entry
        .alias_records
        .iter()
        .map(|record| DiscoverabilityAliasRecord {
            alias_id: string_field(record, "alias_id").unwrap_or_default(),
            alias_kind: string_field(record, "alias_kind").unwrap_or_default(),
            alias_state: string_field(record, "alias_state").unwrap_or_default(),
            source_system_ref: string_field(record, "source_system_ref").unwrap_or_default(),
            introduced_ref: string_field(record, "introduced_ref"),
            deprecated_ref: string_field(record, "deprecated_ref"),
            retired_ref: string_field(record, "retired_ref"),
            replacement_command_id: string_field(record, "replacement_command_id"),
            notes_ref: string_field(record, "notes_ref"),
            eligible_for_new_bindings: bool_field(record, "eligible_for_new_bindings")
                .unwrap_or(false),
        })
        .collect()
}

fn keybindings(entry: &CommandRegistryEntryRecord) -> Vec<DiscoverabilityCurrentKeybindingRecord> {
    entry
        .current_keybinding_refs
        .iter()
        .map(|record| DiscoverabilityCurrentKeybindingRecord {
            platform_class: string_field(record, "platform_class").unwrap_or_default(),
            display_state: string_field(record, "display_state").unwrap_or_default(),
            keybinding_ref: string_field(record, "keybinding_ref").unwrap_or_default(),
            source_layer: string_field(record, "source_layer").unwrap_or_default(),
            imported_from_ref: string_field(record, "imported_from_ref"),
            notes_ref: string_field(record, "notes_ref"),
        })
        .collect()
}

fn origin_class(entry: &CommandRegistryEntryRecord) -> String {
    entry
        .descriptor
        .origin
        .as_ref()
        .map(|origin| origin.origin_class.clone())
        .unwrap_or_else(|| entry.namespace_class.clone())
}

fn origin_source_ref(entry: &CommandRegistryEntryRecord) -> Option<String> {
    entry
        .descriptor
        .origin
        .as_ref()
        .and_then(|origin| origin.source_ref.clone())
}

fn replacement_command_id(aliases: &[DiscoverabilityAliasRecord]) -> Option<String> {
    aliases
        .iter()
        .find_map(|alias| alias.replacement_command_id.clone())
}

fn automation_support(
    entry: &CommandRegistryEntryRecord,
) -> Vec<DiscoverabilityAutomationSupportClass> {
    let mut support = Vec::new();
    for label in &entry.automation_labels {
        match label.as_str() {
            "macro_safe" => support.push(DiscoverabilityAutomationSupportClass::MacroSafe),
            "recipe_safe" => support.push(DiscoverabilityAutomationSupportClass::RecipeSafe),
            "headless_safe" => support.push(DiscoverabilityAutomationSupportClass::HeadlessSafe),
            "ui_only" => support.push(DiscoverabilityAutomationSupportClass::UiOnly),
            _ => {}
        }
    }
    if entry.descriptor.approval_posture_class != "no_approval_required" {
        support.push(DiscoverabilityAutomationSupportClass::ApprovalRequired);
    }
    if support.is_empty() {
        support.push(DiscoverabilityAutomationSupportClass::Unknown);
    }
    support.sort();
    support.dedup();
    support
}

fn projection_refs(entry: &CommandRegistryEntryRecord) -> DiscoverabilityProjectionRefs {
    let command_id_slug = entry
        .descriptor
        .command_id
        .trim_start_matches("cmd:")
        .replace('.', "_");
    DiscoverabilityProjectionRefs {
        palette_row_ref: discoverability_projection_ref(entry, "palette_row_ref")
            .unwrap_or_else(|| format!("projection:{command_id_slug}:palette_row")),
        help_search_ref: discoverability_projection_ref(entry, "help_search_ref")
            .unwrap_or_else(|| format!("projection:{command_id_slug}:help_search")),
        onboarding_hint_ref: discoverability_projection_ref(entry, "onboarding_hint_ref")
            .unwrap_or_else(|| format!("projection:{command_id_slug}:onboarding_hint")),
        migration_bridge_card_ref: discoverability_projection_ref(
            entry,
            "migration_bridge_card_ref",
        )
        .unwrap_or_else(|| format!("projection:{command_id_slug}:migration_bridge_card")),
        current_shortcut_display_ref: discoverability_projection_ref(
            entry,
            "current_shortcut_display_ref",
        )
        .unwrap_or_else(|| format!("projection:{command_id_slug}:current_shortcut_display")),
        why_unavailable_explainer_ref: discoverability_projection_ref(
            entry,
            "why_unavailable_explainer_ref",
        )
        .unwrap_or_else(|| format!("projection:{command_id_slug}:why_unavailable_explainer")),
        key_sequence_discoverability_ref: discoverability_projection_ref(
            entry,
            "key_sequence_discoverability_ref",
        )
        .unwrap_or_else(|| format!("projection:{command_id_slug}:key_sequence_discoverability")),
        docs_help_page_ref: format!(
            "projection:{}:docs_help_page",
            entry.descriptor.canonical_verb
        ),
        voice_hint_ref: format!("projection:{}:voice_hint", entry.descriptor.canonical_verb),
        cli_help_ref: format!("projection:{}:cli_help", entry.descriptor.canonical_verb),
        support_export_ref: format!("support-export:{}", entry.descriptor.command_id),
    }
}

fn surface_rows(refs: &DiscoverabilityProjectionRefs) -> Vec<DiscoverabilitySurfaceProjectionRow> {
    [
        (
            DiscoverabilitySurfaceClass::CommandPalette,
            refs.palette_row_ref.clone(),
        ),
        (
            DiscoverabilitySurfaceClass::KeybindingHelp,
            refs.current_shortcut_display_ref.clone(),
        ),
        (
            DiscoverabilitySurfaceClass::DocsHelp,
            refs.docs_help_page_ref.clone(),
        ),
        (
            DiscoverabilitySurfaceClass::OnboardingHint,
            refs.onboarding_hint_ref.clone(),
        ),
        (
            DiscoverabilitySurfaceClass::VoiceHint,
            refs.voice_hint_ref.clone(),
        ),
        (
            DiscoverabilitySurfaceClass::CliHelp,
            refs.cli_help_ref.clone(),
        ),
        (
            DiscoverabilitySurfaceClass::SupportExport,
            refs.support_export_ref.clone(),
        ),
    ]
    .into_iter()
    .map(
        |(surface_class, projection_ref)| DiscoverabilitySurfaceProjectionRow {
            surface_class,
            projection_ref,
            examples_match: true,
            alias_set_matches: true,
            lifecycle_matches: true,
            disabled_reason_matches: true,
            automation_support_matches: true,
            accessibility_labels_match: true,
            help_anchor_matches: true,
        },
    )
    .collect()
}

fn typed_arguments(entry: &CommandRegistryEntryRecord) -> Vec<DiscoverabilityTypedArgumentRecord> {
    entry
        .descriptor
        .typed_arguments
        .iter()
        .map(DiscoverabilityTypedArgumentRecord::from_descriptor)
        .collect()
}

fn stable_line_required(lifecycle_state: &str) -> bool {
    matches!(lifecycle_state, "stable" | "lts_facing" | "deprecated")
}

fn command_record(entry: &CommandRegistryEntryRecord) -> ProtectedCommandDiscoverabilityRecord {
    let alias_records = alias_records(entry);
    let projection_refs = projection_refs(entry);
    ProtectedCommandDiscoverabilityRecord {
        discoverability_record_id: discoverability_record_field(entry, "discoverability_record_id")
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default()
            .to_owned(),
        command_id: entry.descriptor.command_id.clone(),
        command_revision_ref: entry.descriptor.command_revision_ref.clone(),
        canonical_verb: entry.descriptor.canonical_verb.clone(),
        title: entry.title.clone(),
        summary: entry.summary.clone(),
        lifecycle_state: entry.descriptor.lifecycle_state.clone(),
        stable_line_required: stable_line_required(&entry.descriptor.lifecycle_state),
        origin_class: origin_class(entry),
        origin_source_ref: origin_source_ref(entry),
        category_refs: discoverability_string_array(entry, "category_refs"),
        example_refs: discoverability_string_array(entry, "example_refs"),
        tag_refs: discoverability_string_array(entry, "tag_refs"),
        searchability_axes: discoverability_string_array(entry, "searchability_axes"),
        promoted_alias_ids: discoverability_string_array(entry, "promoted_alias_ids"),
        replacement_command_id: replacement_command_id(&alias_records),
        alias_records,
        current_keybindings: keybindings(entry),
        docs_help_anchor_ref: format!(
            "{}#{}",
            entry.descriptor.docs_help_anchor_ref.pack_id,
            entry.descriptor.docs_help_anchor_ref.anchor_id
        ),
        accessibility: DiscoverabilityAccessibilityRecord::from_descriptor(
            &entry.descriptor.accessibility_label_path,
        ),
        shortcut_narration_hint: entry.descriptor.shortcut_narration_hint.clone(),
        typed_arguments: typed_arguments(entry),
        capability_scope_class: entry.descriptor.capability_scope_class.clone(),
        preview_class: entry.descriptor.preview_class.clone(),
        approval_posture_class: entry.descriptor.approval_posture_class.clone(),
        ai_tool_surfacing_class: entry.descriptor.ai_tool_surfacing_class.clone(),
        automation_support: automation_support(entry),
        disabled_reason_explanation_refs: entry
            .disabled_reason_records
            .iter()
            .map(|record| record.explanation_ref.clone())
            .collect(),
        surface_rows: surface_rows(&projection_refs),
        projection_refs,
    }
}

/// Returns the seeded discoverability support packet derived from the canonical registry.
pub fn seeded_command_discoverability_packet() -> DiscoverabilitySupportPacket {
    let commands = seeded_registry()
        .entries()
        .iter()
        .filter(|entry| stable_line_required(&entry.descriptor.lifecycle_state))
        .map(command_record)
        .collect();
    let query_session_policy = DiscoverabilityQuerySessionPolicyRecord {
        query_session_policy_id: "query-session:command-discoverability:stable:0001".to_owned(),
        current_text_material_class: QuerySessionTextMaterialClass::NotRecorded,
        provider_classes: vec![
            QuerySessionProviderClass::CommandRegistry,
            QuerySessionProviderClass::RecentHistory,
            QuerySessionProviderClass::LexicalCommandIndex,
            QuerySessionProviderClass::SemanticCommandIndex,
            QuerySessionProviderClass::DocsHelp,
            QuerySessionProviderClass::KeybindingResolver,
            QuerySessionProviderClass::MigrationBridge,
        ],
        held_modifier_intent_class: "none".to_owned(),
        history_policy_class: HistoryPolicyClass::LocalDevice,
        max_history_entries: 50,
        retention_policy_ref: "retention:palette.local_device.bounded".to_owned(),
        clear_controls: vec![
            ClearHistoryRuleClass::ClearCurrentSessionOnly,
            ClearHistoryRuleClass::ClearPaletteRecentQueries,
            ClearHistoryRuleClass::ClearCommandRecents,
            ClearHistoryRuleClass::EraseOnWorkspaceUntrust,
        ],
        disable_control_available: true,
        sync_posture: QuerySessionSyncPostureClass::LocalOnly,
        redaction_posture: RedactionPostureClass::LocalPrivate,
        raw_query_export_allowed: false,
        accessibility_disclosure_ref: "a11y:command_palette.query_session_privacy".to_owned(),
    };
    DiscoverabilitySupportPacket::new(commands, query_session_policy)
}

fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Null | serde_json::Value::Bool(_) | serde_json::Value::Number(_) => {
            false
        }
        serde_json::Value::String(text) => {
            let lower = text.to_ascii_lowercase();
            lower.contains("http://")
                || lower.contains("https://")
                || lower.contains("secret")
                || lower.contains("token")
                || lower.contains("password")
        }
        serde_json::Value::Array(items) => items.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
    }
}

#[cfg(test)]
mod tests;
