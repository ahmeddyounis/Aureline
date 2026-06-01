//! Finalized command-parity, discoverability, and palette-session packet.
//!
//! Where [`crate::stabilize_command_contract`] froze the descriptor fields,
//! invocation/result contract, and cross-surface authority parity for a stable
//! command, this module finalizes the *discoverability* half of the same lane
//! into one export-safe packet. For one canonical command family on a claimed
//! stable row it binds the four discoverability truths a command must keep
//! identical no matter where it is *found* or *footer-acted on*:
//!
//! - the **one canonical discoverability record** — primary label, alias set,
//!   category refs, docs/help anchor, and keyword refs — that every projection
//!   surface reads from, so menus, tooltips, leader/help overlays, onboarding
//!   tips, voice-command hints, deep links, keybinding help, and docs/help pages
//!   project from the same record and alias set instead of inventing a local
//!   command name, alias, or example;
//! - the **modifier-action footer contract** — split/open-alt, open-alternate-
//!   target, copy command ID, copy CLI form, add to recipe, and inspect "why not
//!   automatable?" — exposed without a debug-only mode, with held-modifier intent
//!   surfaced and the no-bypass guards that keep copy/inspect non-dispatching and
//!   keep placement/target deltas from widening authority;
//! - the **palette query-session privacy posture** — local-first text and
//!   history, a typed history policy with bounded retention, clear-or-disable
//!   controls, held-modifier intent, and the invariant that query history is
//!   never silently widened into cross-device or cross-tenant memory without an
//!   explicit governing feature; and
//! - the **disabled-with-reason chip parity** — each structured disabled-reason
//!   case mapped to a canonical machine reason code with a shared explanation ref
//!   and a "why not automatable?" ref, resolved identically across every surface.
//!
//! It does not re-derive the descriptor, registry, palette-row, query-session, or
//! discoverability models. The frozen contracts it projects against are the
//! combined palette row/modifier contract
//! ([`docs/commands/palette_row_and_modifier_contract.md`](../../../docs/commands/palette_row_and_modifier_contract.md)),
//! the palette query-session contract
//! ([`docs/commands/palette_query_session_contract.md`](../../../docs/commands/palette_query_session_contract.md)),
//! the sequence/modal discoverability contract
//! ([`docs/commands/sequence_and_modal_discoverability_contract.md`](../../../docs/commands/sequence_and_modal_discoverability_contract.md)),
//! and the command-descriptor contract
//! ([`docs/commands/command_descriptor_contract.md`](../../../docs/commands/command_descriptor_contract.md)).
//! It reuses the canonical contract refs, disabled-reason case vocabulary,
//! surface-qualification posture, and evidence-export shape from
//! [`crate::stabilize_command_contract`] so the two halves of the lane stay one
//! command truth rather than two parallel dictionaries.
//!
//! The record is export-safe. It carries refs, state tokens, coarse classes,
//! counts, and review labels only. Raw query text, raw command arguments, raw
//! prompts, endpoint URLs, credentials, and signing-key material stay outside the
//! support boundary.

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::enablement::DisabledReasonCode;
use crate::stabilize_command_contract::{
    CommandContractEvidenceExport, DisabledReasonCaseClass, StableContractRefs,
    SurfaceQualificationClass,
};

/// Stable record-kind tag carried by [`CommandParityFinalizationPacket`].
pub const FINALIZE_COMMAND_PARITY_RECORD_KIND: &str = "command_parity_finalization_packet";

/// Schema version for finalized command-parity records.
pub const FINALIZE_COMMAND_PARITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the finalized command-parity boundary schema.
pub const FINALIZE_COMMAND_PARITY_SCHEMA_REF: &str =
    "schemas/commands/finalize_command_parity.schema.json";

/// Repo-relative path of the finalized command-parity doc.
pub const FINALIZE_COMMAND_PARITY_DOC_REF: &str = "docs/commands/m4/finalize_command_parity.md";

/// Repo-relative path of the frozen palette row/modifier contract.
pub const FINALIZE_COMMAND_PARITY_PALETTE_ROW_CONTRACT_REF: &str =
    "docs/commands/palette_row_and_modifier_contract.md";

/// Repo-relative path of the frozen palette query-session contract.
pub const FINALIZE_COMMAND_PARITY_QUERY_SESSION_CONTRACT_REF: &str =
    "docs/commands/palette_query_session_contract.md";

/// Repo-relative path of the frozen sequence/modal discoverability contract.
pub const FINALIZE_COMMAND_PARITY_DISCOVERABILITY_CONTRACT_REF: &str =
    "docs/commands/sequence_and_modal_discoverability_contract.md";

/// Repo-relative path of the frozen command-descriptor contract.
pub const FINALIZE_COMMAND_PARITY_DESCRIPTOR_CONTRACT_REF: &str =
    "docs/commands/command_descriptor_contract.md";

/// Repo-relative path of the protected finalized command-parity fixture dir.
pub const FINALIZE_COMMAND_PARITY_FIXTURE_DIR: &str =
    "fixtures/commands/m4/finalize_command_parity";

/// Repo-relative path of the checked finalized command-parity export.
pub const FINALIZE_COMMAND_PARITY_ARTIFACT_REF: &str =
    "artifacts/commands/m4/finalize_command_parity/support_export.json";

/// Repo-relative path of the checked finalized command-parity Markdown summary.
pub const FINALIZE_COMMAND_PARITY_SUMMARY_REF: &str =
    "artifacts/commands/m4/finalize_command_parity/summary.md";

/// One discoverability projection surface a command's record must reach identically.
///
/// Menus, tooltips, leader/help overlays, onboarding tips, voice-command hints,
/// deep links, keybinding help, and docs/help pages are all views over one
/// command graph. Each projects from the same discoverability record and alias
/// set; none mints a local label, alias, example, or disabled-reason prose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscoverabilitySurfaceClass {
    /// Command palette row.
    CommandPalette,
    /// Application menu entry.
    Menu,
    /// Tooltip / hover affordance.
    Tooltip,
    /// Leader / which-key style overlay row.
    LeaderOverlay,
    /// Keybinding help / shortcut-teaching surface.
    KeybindingHelp,
    /// Onboarding tip / coach mark.
    OnboardingTip,
    /// Voice-command hint surface.
    VoiceHint,
    /// Deep link into a command.
    DeepLink,
    /// Docs / help page.
    DocsHelpPage,
}

impl DiscoverabilitySurfaceClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandPalette => "command_palette",
            Self::Menu => "menu",
            Self::Tooltip => "tooltip",
            Self::LeaderOverlay => "leader_overlay",
            Self::KeybindingHelp => "keybinding_help",
            Self::OnboardingTip => "onboarding_tip",
            Self::VoiceHint => "voice_hint",
            Self::DeepLink => "deep_link",
            Self::DocsHelpPage => "docs_help_page",
        }
    }

    /// Discoverability surfaces the packet must cover to claim projection parity.
    pub const fn required_coverage() -> [Self; 9] {
        [
            Self::CommandPalette,
            Self::Menu,
            Self::Tooltip,
            Self::LeaderOverlay,
            Self::KeybindingHelp,
            Self::OnboardingTip,
            Self::VoiceHint,
            Self::DeepLink,
            Self::DocsHelpPage,
        ]
    }
}

/// Modifier / footer action class a stable command's selected row exposes.
///
/// This vocabulary is the one footer/action set the frozen palette row and
/// query-session contracts require keyboard help, menus, docs/help, CLI help,
/// automation recipes, and support export to share.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModifierActionClass {
    /// Default run/open path for the selected row.
    DefaultRun,
    /// Placement delta such as split pane or open alternate.
    SplitOrOpenAlt,
    /// Declared target change where the descriptor and enablement decision allow it.
    OpenAlternateTarget,
    /// Copy the canonical command ID. No dispatch.
    CopyCommandId,
    /// Copy a documented CLI/headless form. No pre-approved token.
    CopyCliForm,
    /// Insert a typed recipe step preserving command identity. No dispatch.
    AddToRecipe,
    /// Open the structured "why not automatable?" explanation. No dispatch.
    InspectWhyNotAutomatable,
}

impl ModifierActionClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DefaultRun => "default_run",
            Self::SplitOrOpenAlt => "split_or_open_alt",
            Self::OpenAlternateTarget => "open_alternate_target",
            Self::CopyCommandId => "copy_command_id",
            Self::CopyCliForm => "copy_cli_form",
            Self::AddToRecipe => "add_to_recipe",
            Self::InspectWhyNotAutomatable => "inspect_why_not_automatable",
        }
    }

    /// True when the action only reads/copies/inspects and never dispatches.
    pub const fn is_non_dispatching(self) -> bool {
        matches!(
            self,
            Self::CopyCommandId
                | Self::CopyCliForm
                | Self::AddToRecipe
                | Self::InspectWhyNotAutomatable
        )
    }

    /// Footer/modifier actions the contract must expose.
    pub const fn required_coverage() -> [Self; 7] {
        [
            Self::DefaultRun,
            Self::SplitOrOpenAlt,
            Self::OpenAlternateTarget,
            Self::CopyCommandId,
            Self::CopyCliForm,
            Self::AddToRecipe,
            Self::InspectWhyNotAutomatable,
        ]
    }
}

/// Local-first history policy class for a palette query session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryPolicyClass {
    /// No history is retained.
    NoHistory,
    /// History lives only for the current session.
    SessionOnly,
    /// History is retained in the local profile.
    LocalProfile,
    /// History is retained on the local device.
    LocalDevice,
    /// History is governed by a managed/explicit policy feature.
    ManagedGoverned,
}

impl HistoryPolicyClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoHistory => "no_history",
            Self::SessionOnly => "session_only",
            Self::LocalProfile => "local_profile",
            Self::LocalDevice => "local_device",
            Self::ManagedGoverned => "managed_governed",
        }
    }

    /// True when this policy keeps history on the local device/profile only and
    /// cannot, on its own, widen into cross-device or cross-tenant memory.
    pub const fn is_local_first(self) -> bool {
        matches!(
            self,
            Self::NoHistory | Self::SessionOnly | Self::LocalProfile | Self::LocalDevice
        )
    }
}

/// Typed clear-history control a query session exposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClearHistoryRuleClass {
    /// Clears volatile current-session material.
    ClearCurrentSessionOnly,
    /// Clears palette recent query entries.
    ClearPaletteRecentQueries,
    /// Clears command-specific recent invocations used by the palette.
    ClearCommandRecents,
    /// Clears profile-local command history admitted by policy.
    ClearProfileCommandHistory,
    /// Clears managed/governed history according to the policy epoch.
    AdminPolicyPurge,
    /// Clears history when workspace trust is removed.
    EraseOnWorkspaceUntrust,
}

impl ClearHistoryRuleClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClearCurrentSessionOnly => "clear_current_session_only",
            Self::ClearPaletteRecentQueries => "clear_palette_recent_queries",
            Self::ClearCommandRecents => "clear_command_recents",
            Self::ClearProfileCommandHistory => "clear_profile_command_history",
            Self::AdminPolicyPurge => "admin_policy_purge",
            Self::EraseOnWorkspaceUntrust => "erase_on_workspace_untrust",
        }
    }

    /// Clear-history controls a stable query session must always expose.
    pub const fn required_coverage() -> [Self; 2] {
        [Self::ClearCurrentSessionOnly, Self::EraseOnWorkspaceUntrust]
    }
}

/// Redaction posture for the palette query session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionPostureClass {
    /// Local/private; raw text never leaves the device.
    LocalPrivate,
    /// Metadata-safe; only coarse classes/refs are exportable.
    MetadataSafe,
    /// Support-redacted; raw text replaced with redacted refs.
    SupportRedacted,
    /// Policy-review; export gated behind a policy review.
    PolicyReview,
    /// Export-denied; the session cannot prove redaction and denies export.
    ExportDenied,
}

impl RedactionPostureClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalPrivate => "local_private",
            Self::MetadataSafe => "metadata_safe",
            Self::SupportRedacted => "support_redacted",
            Self::PolicyReview => "policy_review",
            Self::ExportDenied => "export_denied",
        }
    }
}

/// One canonical discoverability record every projection surface reads from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscoverabilityRecord {
    /// Canonical stable command id the record describes.
    pub canonical_command_id: String,
    /// Primary user-facing label ref (not a surface-local alias).
    pub primary_label_ref: String,
    /// Alias ids that resolve to the canonical command id.
    pub alias_set: Vec<String>,
    /// Category refs consumed by search, onboarding, and help.
    pub category_refs: Vec<String>,
    /// Docs/help anchor ref that owns command help.
    pub docs_help_anchor_ref: String,
    /// Keyword refs that power discoverability search.
    pub keyword_refs: Vec<String>,
}

impl DiscoverabilityRecord {
    fn is_complete(&self) -> bool {
        !self.canonical_command_id.trim().is_empty()
            && !self.primary_label_ref.trim().is_empty()
            && !self.docs_help_anchor_ref.trim().is_empty()
            && !self.alias_set.is_empty()
            && self.alias_set.iter().all(|alias| !alias.trim().is_empty())
            && !self.category_refs.is_empty()
            && self
                .category_refs
                .iter()
                .all(|reference| !reference.trim().is_empty())
            && !self.keyword_refs.is_empty()
            && self
                .keyword_refs
                .iter()
                .all(|reference| !reference.trim().is_empty())
    }
}

/// One discoverability projection row for a single surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscoverabilityProjectionRow {
    /// Surface this row covers.
    pub surface_class: DiscoverabilitySurfaceClass,
    /// True when the surface projects from the canonical discoverability record.
    pub projects_from_canonical_record: bool,
    /// True when the surface shows the same alias set (no alias drift).
    pub alias_set_matches: bool,
    /// True when an alias/label on this surface resolves to the canonical command id.
    pub resolves_to_canonical_command: bool,
    /// True when Copy command ID behavior matches the canonical record.
    pub copy_id_consistent: bool,
    /// True when Copy CLI form behavior matches the canonical record.
    pub copy_cli_consistent: bool,
    /// True when Add to recipe behavior matches the canonical record.
    pub add_to_recipe_consistent: bool,
    /// True when modifier-action footer variants are consistent on this surface.
    pub modifier_footer_consistent: bool,
    /// True when the disabled-with-reason chip resolves identically here.
    pub disabled_reason_consistent: bool,
    /// True when the "why not automatable?" explanation is consistent here.
    pub why_not_automatable_consistent: bool,
    /// True when examples and disabled-with-reason copy do not drift from canon.
    pub example_drift_free: bool,
    /// Stable-qualification posture for this surface.
    pub qualification: SurfaceQualificationClass,
    /// True when this surface claims the Stable lane.
    pub claimed_stable: bool,
}

impl DiscoverabilityProjectionRow {
    fn preserves_full_parity(&self) -> bool {
        self.projects_from_canonical_record
            && self.alias_set_matches
            && self.resolves_to_canonical_command
            && self.copy_id_consistent
            && self.copy_cli_consistent
            && self.add_to_recipe_consistent
            && self.modifier_footer_consistent
            && self.disabled_reason_consistent
            && self.why_not_automatable_consistent
            && self.example_drift_free
    }
}

/// The modifier-action footer contract a stable command exposes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModifierActionFooterContract {
    /// Footer/modifier actions exposed where valid.
    pub actions: Vec<ModifierActionClass>,
    /// True when held modifier intent is surfaced before invocation.
    pub exposes_held_modifier_intent: bool,
    /// True when the "why not automatable?" explanation is reachable.
    pub exposes_why_not_automatable: bool,
    /// True when the footer requires a debug-only mode to appear (must be false).
    pub requires_debug_mode: bool,
    /// True when copy/inspect actions never dispatch the command.
    pub copy_and_inspect_never_dispatch: bool,
    /// True when placement/target deltas never widen command authority.
    pub placement_and_target_never_widen_authority: bool,
}

impl ModifierActionFooterContract {
    fn guards_hold(&self) -> bool {
        self.exposes_held_modifier_intent
            && self.exposes_why_not_automatable
            && !self.requires_debug_mode
            && self.copy_and_inspect_never_dispatch
            && self.placement_and_target_never_widen_authority
    }
}

/// The palette query-session privacy posture for a stable command's launcher.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuerySessionPrivacyContract {
    /// True when query text and history are local-first.
    pub local_first: bool,
    /// Typed history policy class.
    pub history_policy: HistoryPolicyClass,
    /// Bounded retention policy ref.
    pub retention_policy_ref: String,
    /// Maximum retained history entries.
    pub max_history_entries: u32,
    /// Clear-history controls the session exposes.
    pub clear_controls: Vec<ClearHistoryRuleClass>,
    /// True when the user can disable history entirely.
    pub disable_control_available: bool,
    /// True when held modifier intent is owned by the session.
    pub exposes_held_modifier_intent: bool,
    /// Redaction posture for exportable/support material.
    pub redaction_posture: RedactionPostureClass,
    /// True when raw query text is allowed in exports (must be false local-first).
    pub raw_query_export_allowed: bool,
    /// True when any cross-device/cross-tenant memory is gated by an explicit feature.
    pub cross_device_memory_governed_by_explicit_feature: bool,
    /// Governing feature ref required before history widens beyond the device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub governing_feature_ref: Option<String>,
}

impl QuerySessionPrivacyContract {
    fn widens_beyond_device(&self) -> bool {
        matches!(self.history_policy, HistoryPolicyClass::ManagedGoverned)
            || self.cross_device_memory_governed_by_explicit_feature
    }

    fn governance_present(&self) -> bool {
        self.governing_feature_ref
            .as_ref()
            .is_some_and(|reference| !reference.trim().is_empty())
    }

    fn controls_complete(&self) -> bool {
        ClearHistoryRuleClass::required_coverage()
            .into_iter()
            .all(|required| self.clear_controls.iter().any(|rule| *rule == required))
            && self.disable_control_available
            && self.exposes_held_modifier_intent
            && !self.retention_policy_ref.trim().is_empty()
    }
}

/// One disabled-with-reason chip row resolved identically across surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisabledReasonChipRow {
    /// Reviewer-facing disabled-reason case class.
    pub case_class: DisabledReasonCaseClass,
    /// Canonical machine disabled-reason code from the frozen vocabulary.
    pub disabled_reason_code: DisabledReasonCode,
    /// Structured explanation ref shared by every surface.
    pub explanation_ref: String,
    /// "Why not automatable?" explanation ref shared by every surface.
    pub why_not_automatable_ref: String,
    /// True when palette, menus, keybindings, voice, help, and deep links resolve
    /// the same reason rather than surface-local prose.
    pub surfaces_resolve_identically: bool,
}

/// Constructor input for [`CommandParityFinalizationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandParityFinalizationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Canonical command-family id this packet finalizes.
    pub command_family_id: String,
    /// Display label safe for UI, docs, and support.
    pub display_label: String,
    /// Whether this row claims the Stable line.
    pub claimed_stable: bool,
    /// Policy epoch ref this row was evaluated under.
    pub policy_epoch_ref: String,
    /// The single descriptor registry and result schema every surface projects.
    pub contract_refs: StableContractRefs,
    /// The canonical discoverability record.
    pub discoverability_record: DiscoverabilityRecord,
    /// Per-surface discoverability projection rows.
    pub projection_rows: Vec<DiscoverabilityProjectionRow>,
    /// Modifier-action footer contract.
    pub footer_contract: ModifierActionFooterContract,
    /// Palette query-session privacy posture.
    pub query_session_privacy: QuerySessionPrivacyContract,
    /// Disabled-with-reason chip rows.
    pub disabled_reason_chips: Vec<DisabledReasonChipRow>,
    /// Exportable evidence lineage.
    pub evidence_export: CommandContractEvidenceExport,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe finalized command-parity record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandParityFinalizationPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Canonical command-family id this packet finalizes.
    pub command_family_id: String,
    /// Display label safe for UI, docs, and support.
    pub display_label: String,
    /// Whether this row claims the Stable line.
    pub claimed_stable: bool,
    /// Policy epoch ref this row was evaluated under.
    pub policy_epoch_ref: String,
    /// The single descriptor registry and result schema every surface projects.
    pub contract_refs: StableContractRefs,
    /// The canonical discoverability record.
    pub discoverability_record: DiscoverabilityRecord,
    /// Per-surface discoverability projection rows.
    pub projection_rows: Vec<DiscoverabilityProjectionRow>,
    /// Modifier-action footer contract.
    pub footer_contract: ModifierActionFooterContract,
    /// Palette query-session privacy posture.
    pub query_session_privacy: QuerySessionPrivacyContract,
    /// Disabled-with-reason chip rows.
    pub disabled_reason_chips: Vec<DisabledReasonChipRow>,
    /// Exportable evidence lineage.
    pub evidence_export: CommandContractEvidenceExport,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl CommandParityFinalizationPacket {
    /// Builds a finalized command-parity packet from canonical rows.
    pub fn new(input: CommandParityFinalizationPacketInput) -> Self {
        Self {
            record_kind: FINALIZE_COMMAND_PARITY_RECORD_KIND.to_owned(),
            schema_version: FINALIZE_COMMAND_PARITY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            command_family_id: input.command_family_id,
            display_label: input.display_label,
            claimed_stable: input.claimed_stable,
            policy_epoch_ref: input.policy_epoch_ref,
            contract_refs: input.contract_refs,
            discoverability_record: input.discoverability_record,
            projection_rows: input.projection_rows,
            footer_contract: input.footer_contract,
            query_session_privacy: input.query_session_privacy,
            disabled_reason_chips: input.disabled_reason_chips,
            evidence_export: input.evidence_export,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the finalized command-parity packet's stable-line invariants.
    pub fn validate(&self) -> Vec<CommandParityFinalizationViolation> {
        let mut violations = Vec::new();
        if self.record_kind != FINALIZE_COMMAND_PARITY_RECORD_KIND {
            violations.push(CommandParityFinalizationViolation::WrongRecordKind);
        }
        if self.schema_version != FINALIZE_COMMAND_PARITY_SCHEMA_VERSION {
            violations.push(CommandParityFinalizationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.command_family_id.trim().is_empty()
            || self.display_label.trim().is_empty()
            || self.policy_epoch_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(CommandParityFinalizationViolation::MissingIdentity);
        }
        validate_source_contracts(self, &mut violations);
        validate_contract_refs(self, &mut violations);
        validate_discoverability(self, &mut violations);
        validate_footer_contract(self, &mut violations);
        validate_query_session(self, &mut violations);
        validate_disabled_reason_chips(self, &mut violations);
        validate_evidence_export(self, &mut violations);
        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("finalized command parity packet serializes"),
        ) {
            violations.push(CommandParityFinalizationViolation::RawMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("finalized command parity packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed_surfaces = self
            .projection_rows
            .iter()
            .filter(|row| !row.qualification.is_stable())
            .count();
        let identically_resolved = self
            .disabled_reason_chips
            .iter()
            .filter(|chip| chip.surfaces_resolve_identically)
            .count();
        let mut out = String::new();
        out.push_str("# Command Parity Finalization\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Command family: `{}`\n", self.command_family_id));
        out.push_str(&format!(
            "- Evidence id: `{}`\n",
            self.evidence_export.evidence_id
        ));
        out.push_str(&format!("- Claimed stable: {}\n", self.claimed_stable));
        out.push_str(&format!(
            "- Canonical command: `{}` ({} aliases)\n",
            self.discoverability_record.canonical_command_id,
            self.discoverability_record.alias_set.len()
        ));
        out.push_str(&format!(
            "- Discoverability surfaces: {} ({} narrowed below Stable)\n",
            self.projection_rows.len(),
            narrowed_surfaces
        ));
        out.push_str(&format!(
            "- Modifier-action footer variants: {}\n",
            self.footer_contract.actions.len()
        ));
        out.push_str(&format!(
            "- Query-session history policy: `{}` (local-first: {})\n",
            self.query_session_privacy.history_policy.as_str(),
            self.query_session_privacy.local_first
        ));
        out.push_str(&format!(
            "- Disabled-reason chips: {} ({} resolve identically across surfaces)\n",
            self.disabled_reason_chips.len(),
            identically_resolved
        ));
        out.push_str(&format!(
            "- Rollback lineage refs: {}\n",
            self.evidence_export.rollback_lineage_refs.len()
        ));
        out
    }
}

/// Errors emitted when reading the checked-in finalized command-parity export.
#[derive(Debug)]
pub enum CommandParityFinalizationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<CommandParityFinalizationViolation>),
}

impl fmt::Display for CommandParityFinalizationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "finalized command parity export parse failed: {error}"
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
                    "finalized command parity export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for CommandParityFinalizationArtifactError {}

/// Validation failures emitted by [`CommandParityFinalizationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandParityFinalizationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The bound canonical contract refs drifted from the single registry/schema.
    ContractRefsNotCanonical,
    /// The canonical discoverability record is incomplete.
    DiscoverabilityRecordIncomplete,
    /// Discoverability surface coverage is incomplete.
    DiscoverabilitySurfaceCoverageMissing,
    /// A claimed-stable surface drifted from the canonical discoverability record.
    DiscoverabilityProjectionDrift,
    /// A surface narrowed below Stable still claims the Stable lane.
    UnqualifiedSurfaceClaimsStable,
    /// Footer/modifier action coverage is incomplete.
    FooterActionCoverageMissing,
    /// A footer/modifier guard was broken (debug-only, dispatching copy, widening).
    FooterGuardBroken,
    /// The query session is not local-first.
    QuerySessionNotLocalFirst,
    /// Query history widened beyond the device without an explicit governing feature.
    QueryHistoryWidenedWithoutGovernance,
    /// Required query-session controls (clear/disable/retention/modifier) are missing.
    QuerySessionControlsMissing,
    /// Structured disabled-reason chip coverage is incomplete.
    DisabledReasonChipCoverageMissing,
    /// A disabled-reason chip is missing structured refs.
    DisabledReasonChipRefsMissing,
    /// A disabled-reason chip does not resolve identically across surfaces.
    DisabledReasonNotResolvedIdentically,
    /// Evidence export refs are missing.
    EvidenceExportRefsMissing,
    /// The rollback lineage is missing.
    RollbackLineageMissing,
    /// The packet carries raw material outside the export boundary.
    RawMaterialInExport,
}

impl CommandParityFinalizationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ContractRefsNotCanonical => "contract_refs_not_canonical",
            Self::DiscoverabilityRecordIncomplete => "discoverability_record_incomplete",
            Self::DiscoverabilitySurfaceCoverageMissing => {
                "discoverability_surface_coverage_missing"
            }
            Self::DiscoverabilityProjectionDrift => "discoverability_projection_drift",
            Self::UnqualifiedSurfaceClaimsStable => "unqualified_surface_claims_stable",
            Self::FooterActionCoverageMissing => "footer_action_coverage_missing",
            Self::FooterGuardBroken => "footer_guard_broken",
            Self::QuerySessionNotLocalFirst => "query_session_not_local_first",
            Self::QueryHistoryWidenedWithoutGovernance => {
                "query_history_widened_without_governance"
            }
            Self::QuerySessionControlsMissing => "query_session_controls_missing",
            Self::DisabledReasonChipCoverageMissing => "disabled_reason_chip_coverage_missing",
            Self::DisabledReasonChipRefsMissing => "disabled_reason_chip_refs_missing",
            Self::DisabledReasonNotResolvedIdentically => {
                "disabled_reason_not_resolved_identically"
            }
            Self::EvidenceExportRefsMissing => "evidence_export_refs_missing",
            Self::RollbackLineageMissing => "rollback_lineage_missing",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Returns the checked-in finalized command-parity support export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or validate.
pub fn current_finalize_command_parity_export(
) -> Result<CommandParityFinalizationPacket, CommandParityFinalizationArtifactError> {
    let packet: CommandParityFinalizationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/commands/m4/finalize_command_parity/support_export.json"
    )))
    .map_err(CommandParityFinalizationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(CommandParityFinalizationArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &CommandParityFinalizationPacket,
    violations: &mut Vec<CommandParityFinalizationViolation>,
) {
    for required in [
        FINALIZE_COMMAND_PARITY_DOC_REF,
        FINALIZE_COMMAND_PARITY_SCHEMA_REF,
        FINALIZE_COMMAND_PARITY_PALETTE_ROW_CONTRACT_REF,
        FINALIZE_COMMAND_PARITY_QUERY_SESSION_CONTRACT_REF,
        FINALIZE_COMMAND_PARITY_DISCOVERABILITY_CONTRACT_REF,
        FINALIZE_COMMAND_PARITY_DESCRIPTOR_CONTRACT_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(CommandParityFinalizationViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_contract_refs(
    packet: &CommandParityFinalizationPacket,
    violations: &mut Vec<CommandParityFinalizationViolation>,
) {
    if packet.contract_refs != StableContractRefs::canonical() {
        violations.push(CommandParityFinalizationViolation::ContractRefsNotCanonical);
    }
}

fn validate_discoverability(
    packet: &CommandParityFinalizationPacket,
    violations: &mut Vec<CommandParityFinalizationViolation>,
) {
    if !packet.discoverability_record.is_complete() {
        violations.push(CommandParityFinalizationViolation::DiscoverabilityRecordIncomplete);
    }

    for required in DiscoverabilitySurfaceClass::required_coverage() {
        if !packet
            .projection_rows
            .iter()
            .any(|row| row.surface_class == required)
        {
            violations
                .push(CommandParityFinalizationViolation::DiscoverabilitySurfaceCoverageMissing);
            break;
        }
    }

    for row in &packet.projection_rows {
        // A surface narrowed below Stable may not claim the Stable lane.
        if row.claimed_stable && !row.qualification.is_stable() {
            violations.push(CommandParityFinalizationViolation::UnqualifiedSurfaceClaimsStable);
            break;
        }
        // A Stable surface must project from the canonical record with no drift in
        // alias set, copy-ID/CLI, add-to-recipe, footer, disabled-reason, or examples.
        if row.qualification.is_stable() && !row.preserves_full_parity() {
            violations.push(CommandParityFinalizationViolation::DiscoverabilityProjectionDrift);
            break;
        }
    }
}

fn validate_footer_contract(
    packet: &CommandParityFinalizationPacket,
    violations: &mut Vec<CommandParityFinalizationViolation>,
) {
    for required in ModifierActionClass::required_coverage() {
        if !packet
            .footer_contract
            .actions
            .iter()
            .any(|action| *action == required)
        {
            violations.push(CommandParityFinalizationViolation::FooterActionCoverageMissing);
            break;
        }
    }
    if !packet.footer_contract.guards_hold() {
        violations.push(CommandParityFinalizationViolation::FooterGuardBroken);
    }
}

fn validate_query_session(
    packet: &CommandParityFinalizationPacket,
    violations: &mut Vec<CommandParityFinalizationViolation>,
) {
    let session = &packet.query_session_privacy;
    // Local-first text and history; raw query text is never silently exported.
    if !session.local_first || session.raw_query_export_allowed {
        violations.push(CommandParityFinalizationViolation::QuerySessionNotLocalFirst);
    }
    // Any widening beyond the device requires an explicit governing feature ref.
    if session.widens_beyond_device() && !session.governance_present() {
        violations.push(CommandParityFinalizationViolation::QueryHistoryWidenedWithoutGovernance);
    }
    // Clear-or-disable controls, retention policy, and held-modifier intent.
    if !session.controls_complete() {
        violations.push(CommandParityFinalizationViolation::QuerySessionControlsMissing);
    }
}

fn validate_disabled_reason_chips(
    packet: &CommandParityFinalizationPacket,
    violations: &mut Vec<CommandParityFinalizationViolation>,
) {
    for required in DisabledReasonCaseClass::required_coverage() {
        if !packet
            .disabled_reason_chips
            .iter()
            .any(|chip| chip.case_class == required)
        {
            violations.push(CommandParityFinalizationViolation::DisabledReasonChipCoverageMissing);
            break;
        }
    }

    for chip in &packet.disabled_reason_chips {
        if chip.explanation_ref.trim().is_empty() || chip.why_not_automatable_ref.trim().is_empty()
        {
            violations.push(CommandParityFinalizationViolation::DisabledReasonChipRefsMissing);
            break;
        }
        if !chip.surfaces_resolve_identically {
            violations
                .push(CommandParityFinalizationViolation::DisabledReasonNotResolvedIdentically);
            break;
        }
    }
}

fn validate_evidence_export(
    packet: &CommandParityFinalizationPacket,
    violations: &mut Vec<CommandParityFinalizationViolation>,
) {
    let export = &packet.evidence_export;
    if export.evidence_id.trim().is_empty()
        || export.json_export_ref.trim().is_empty()
        || export.markdown_summary_ref.trim().is_empty()
        || export.admin_inspector_ref.trim().is_empty()
        || export.support_export_ref.trim().is_empty()
    {
        violations.push(CommandParityFinalizationViolation::EvidenceExportRefsMissing);
    }
    if export.rollback_lineage_refs.is_empty()
        || export
            .rollback_lineage_refs
            .iter()
            .any(|reference| reference.trim().is_empty())
    {
        violations.push(CommandParityFinalizationViolation::RollbackLineageMissing);
    }
}

fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => contains_forbidden_material(text),
        serde_json::Value::Array(values) => values.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

fn contains_forbidden_material(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.contains("://")
        || lower.contains("bearer ")
        || lower.contains("api_key")
        || lower.contains("api-key")
        || lower.contains("oauth_token")
        || lower.contains("private_key")
        || lower.contains("signing_key")
        || lower.contains("raw_prompt")
        || lower.contains("raw_query")
        || lower.contains("raw_body")
        || lower.contains("billing-account")
}

#[cfg(test)]
mod tests;
