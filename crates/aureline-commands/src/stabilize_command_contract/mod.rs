//! Stabilized command-contract packet.
//!
//! This module finalizes the command lane into one export-safe packet that binds
//! — for a single canonical command family on a claimed stable row — the four
//! truths a command must keep identical no matter where it is reached:
//!
//! - the **stable descriptor fields** that make a command a public product object:
//!   command id, invocation schema, capability class, enablement rules,
//!   discoverability record, automation labels, result contract, lifecycle and
//!   origin metadata, alias/deprecation metadata, docs/help anchor, accessibility
//!   labels, and shortcut narration — every one typed, exported, and treated as a
//!   stable interface so docs/help and support traces never maintain a parallel
//!   hand-written command dictionary;
//! - the **invocation-session and result-packet contract** that pins the stable
//!   result codes, canonical command identity, alias resolution, artifact refs,
//!   notification/activity joins, rollback/checkpoint handles, and the strict
//!   no-bypass guards so support, automation, and UI surfaces never infer an
//!   outcome from rendered text alone;
//! - the **palette diagnostics contract** that requires the source badge,
//!   keybinding, dominant side-effect cue, disabled-with-reason state, and
//!   preview/approval posture, plus the Copy command ID, Copy CLI equivalent,
//!   Add to recipe, and Why not automatable? actions, and the structured
//!   disabled-reason cases (disabled-by-policy, wrong-focus, missing-runtime,
//!   degraded-provider, preview-required, approval-required, UI-only) every
//!   surface resolves to the same reason vocabulary; and
//! - the **cross-surface authority parity** that proves the same command requires
//!   the same preview, approval, rollback, and audit semantics from menu/button,
//!   keybinding, palette, CLI/headless, AI tool, voice, recipe, deep link, and
//!   browser companion — and that no surface widens authority or claims the Stable
//!   lane while it is narrowed below it.
//!
//! It does not re-derive the descriptor, registry, invocation, result, or
//! authority models. The [`crate::descriptor::CommandDescriptorRecord`],
//! [`crate::invocation::InvocationSessionPacketRecord`],
//! [`crate::invocation::CommandResultPacketRecord`],
//! [`crate::registry::CommandDescriptorPublicContractRecord`], and
//! [`crate::authority::CommandAuthorityScenarioRecord`] own those contracts. This
//! packet references them by stable schema ref and adds the finalized invariants
//! the stable line needs, projecting one canonical, attributable, exportable
//! object that UI, CLI, AI, support export, and documentation fixtures all ingest
//! instead of cloning status text.
//!
//! The frozen contracts this lane projects against are the command-descriptor
//! contract ([`docs/commands/command_descriptor_contract.md`](../../../docs/commands/command_descriptor_contract.md))
//! and the invocation-result and parity contract
//! ([`docs/commands/invocation_result_and_parity_contract.md`](../../../docs/commands/invocation_result_and_parity_contract.md)).
//!
//! The record is export-safe. It carries refs, state tokens, coarse classes,
//! counts, and review labels only. Raw command arguments, raw prompts, endpoint
//! URLs, credentials, and signing-key material stay outside the support boundary.

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::enablement::DisabledReasonCode;

/// Stable record-kind tag carried by [`CommandContractStabilizationPacket`].
pub const STABILIZE_COMMAND_CONTRACT_RECORD_KIND: &str = "command_contract_stabilization_packet";

/// Schema version for stabilized command-contract records.
pub const STABILIZE_COMMAND_CONTRACT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the stabilized command-contract boundary schema.
pub const STABILIZE_COMMAND_CONTRACT_SCHEMA_REF: &str =
    "schemas/commands/stabilize_command_contract.schema.json";

/// Repo-relative path of the stabilized command-contract doc.
pub const STABILIZE_COMMAND_CONTRACT_DOC_REF: &str =
    "docs/commands/m4/stabilize_command_contract.md";

/// Repo-relative path of the frozen command-descriptor contract.
pub const STABILIZE_COMMAND_CONTRACT_DESCRIPTOR_CONTRACT_REF: &str =
    "docs/commands/command_descriptor_contract.md";

/// Repo-relative path of the frozen invocation-result and parity contract.
pub const STABILIZE_COMMAND_CONTRACT_PARITY_CONTRACT_REF: &str =
    "docs/commands/invocation_result_and_parity_contract.md";

/// Repo-relative path of the protected stabilized command-contract fixture dir.
pub const STABILIZE_COMMAND_CONTRACT_FIXTURE_DIR: &str =
    "fixtures/commands/m4/stabilize_command_contract";

/// Repo-relative path of the checked stabilized command-contract export.
pub const STABILIZE_COMMAND_CONTRACT_ARTIFACT_REF: &str =
    "artifacts/commands/m4/stabilize_command_contract/support_export.json";

/// Repo-relative path of the checked stabilized command-contract Markdown summary.
pub const STABILIZE_COMMAND_CONTRACT_SUMMARY_REF: &str =
    "artifacts/commands/m4/stabilize_command_contract/summary.md";

// Canonical command-lane refs the stabilized packet binds together. These are the
// single descriptor registry and single result-packet schema the acceptance lane
// requires every surface to project from.
const CANONICAL_DESCRIPTOR_SCHEMA_REF: &str = "schemas/commands/command_descriptor.schema.json";
const CANONICAL_REGISTRY_ENTRY_SCHEMA_REF: &str =
    "schemas/commands/command_registry_entry.schema.json";
const CANONICAL_REGISTRY_SEED_REF: &str = "artifacts/commands/command_registry_seed.yaml";
const CANONICAL_INVOCATION_SESSION_SCHEMA_REF: &str =
    "schemas/commands/command_invocation_session.schema.json";
const CANONICAL_RESULT_PACKET_SCHEMA_REF: &str =
    "schemas/commands/command_result_packet.schema.json";
const CANONICAL_PUBLIC_CONTRACT_SCHEMA_REF: &str =
    "schemas/commands/command_projection.schema.json";
const CANONICAL_PARITY_EXPECTATION_SCHEMA_REF: &str =
    "schemas/commands/parity_expectation.schema.json";
const CANONICAL_DISABLED_REASON_VOCABULARY_REF: &str =
    "docs/commands/disabled_reason_vocabulary.md";

/// One finalized stable descriptor field class.
///
/// Promoting a command to the stable line freezes these fields as typed,
/// exportable interfaces. They are the spine of the public command object: the
/// fields docs/help, migration, CLI, AI, and support traces all read instead of
/// inferring command meaning from UI text or internal callbacks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableDescriptorFieldClass {
    /// Stable canonical command id; never repurposed once it leaves experimental.
    CommandId,
    /// Argument/provenance-bearing invocation-session schema ref.
    InvocationSchema,
    /// Capability class driving preview, approval, policy, and audit posture.
    CapabilityClass,
    /// Structured enablement rule and disabled-reason refs.
    EnablementRules,
    /// Discoverability-record references for palette, menu, help, and onboarding.
    DiscoverabilityRecord,
    /// Automation labels (macro-safe, recipe-safe, headless-safe, UI-only, approval-required).
    AutomationLabels,
    /// Structured result contract and result-packet schema ref.
    ResultContract,
    /// Lifecycle state, support class, release channel, and freshness class.
    LifecycleMetadata,
    /// Origin class, source ref, and publisher metadata.
    OriginMetadata,
    /// Alias and deprecation lifecycle metadata.
    AliasDeprecationMetadata,
    /// Canonical docs/help anchor ref.
    DocsHelpAnchor,
    /// Accessibility-facing labels for the command.
    AccessibilityLabels,
    /// Shortcut narration hints for bound/unbound states.
    ShortcutNarration,
}

impl StableDescriptorFieldClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandId => "command_id",
            Self::InvocationSchema => "invocation_schema",
            Self::CapabilityClass => "capability_class",
            Self::EnablementRules => "enablement_rules",
            Self::DiscoverabilityRecord => "discoverability_record",
            Self::AutomationLabels => "automation_labels",
            Self::ResultContract => "result_contract",
            Self::LifecycleMetadata => "lifecycle_metadata",
            Self::OriginMetadata => "origin_metadata",
            Self::AliasDeprecationMetadata => "alias_deprecation_metadata",
            Self::DocsHelpAnchor => "docs_help_anchor",
            Self::AccessibilityLabels => "accessibility_labels",
            Self::ShortcutNarration => "shortcut_narration",
        }
    }

    /// Stable descriptor fields the packet must finalize to claim the stable line.
    pub const fn required_coverage() -> [Self; 13] {
        [
            Self::CommandId,
            Self::InvocationSchema,
            Self::CapabilityClass,
            Self::EnablementRules,
            Self::DiscoverabilityRecord,
            Self::AutomationLabels,
            Self::ResultContract,
            Self::LifecycleMetadata,
            Self::OriginMetadata,
            Self::AliasDeprecationMetadata,
            Self::DocsHelpAnchor,
            Self::AccessibilityLabels,
            Self::ShortcutNarration,
        ]
    }
}

/// Stable result-code vocabulary every command result packet projects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandResultCodeClass {
    /// The command applied with no warnings.
    Succeeded,
    /// The command applied but raised warnings.
    SucceededWithWarnings,
    /// The command applied to part of its scope only.
    PartiallyApplied,
    /// The command was cancelled before applying anything.
    Cancelled,
    /// A policy blocked the command before it applied.
    BlockedByPolicy,
    /// The command was disabled with a structured reason.
    DisabledWithReason,
    /// The command requires a preview before it can apply.
    PreviewRequired,
    /// The command requires an approval before it can apply.
    ApprovalRequired,
    /// The command attempted to apply and failed.
    Failed,
}

impl CommandResultCodeClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Succeeded => "succeeded",
            Self::SucceededWithWarnings => "succeeded_with_warnings",
            Self::PartiallyApplied => "partially_applied",
            Self::Cancelled => "cancelled",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::DisabledWithReason => "disabled_with_reason",
            Self::PreviewRequired => "preview_required",
            Self::ApprovalRequired => "approval_required",
            Self::Failed => "failed",
        }
    }

    /// Result codes the stable result contract must enumerate.
    pub const fn required_coverage() -> [Self; 9] {
        [
            Self::Succeeded,
            Self::SucceededWithWarnings,
            Self::PartiallyApplied,
            Self::Cancelled,
            Self::BlockedByPolicy,
            Self::DisabledWithReason,
            Self::PreviewRequired,
            Self::ApprovalRequired,
            Self::Failed,
        ]
    }
}

/// Palette diagnostics action a stable command row must expose where valid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaletteActionClass {
    /// Copy the stable command id.
    CopyCommandId,
    /// Copy the headless/CLI equivalent invocation.
    CopyCliEquivalent,
    /// Add the command to a recipe.
    AddToRecipe,
    /// Explain why a command is not automatable.
    WhyNotAutomatable,
}

impl PaletteActionClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CopyCommandId => "copy_command_id",
            Self::CopyCliEquivalent => "copy_cli_equivalent",
            Self::AddToRecipe => "add_to_recipe",
            Self::WhyNotAutomatable => "why_not_automatable",
        }
    }

    /// Palette actions the diagnostics contract must expose.
    pub const fn required_coverage() -> [Self; 4] {
        [
            Self::CopyCommandId,
            Self::CopyCliEquivalent,
            Self::AddToRecipe,
            Self::WhyNotAutomatable,
        ]
    }
}

/// Structured disabled-reason case class the palette/diagnostics fixtures cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisabledReasonCaseClass {
    /// The command is disabled by policy.
    DisabledByPolicy,
    /// The command needs a different focus or selection.
    WrongFocus,
    /// A required runtime or execution context is missing.
    MissingRuntime,
    /// A required provider is degraded or unlinked.
    DegradedProvider,
    /// The command requires a preview that has not been shown.
    PreviewRequired,
    /// The command requires an approval that has not been granted.
    ApprovalRequired,
    /// The command is reachable only from an interactive UI surface.
    UiOnly,
}

impl DisabledReasonCaseClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DisabledByPolicy => "disabled_by_policy",
            Self::WrongFocus => "wrong_focus",
            Self::MissingRuntime => "missing_runtime",
            Self::DegradedProvider => "degraded_provider",
            Self::PreviewRequired => "preview_required",
            Self::ApprovalRequired => "approval_required",
            Self::UiOnly => "ui_only",
        }
    }

    /// Disabled-reason cases the palette/diagnostics fixtures must cover.
    pub const fn required_coverage() -> [Self; 7] {
        [
            Self::DisabledByPolicy,
            Self::WrongFocus,
            Self::MissingRuntime,
            Self::DegradedProvider,
            Self::PreviewRequired,
            Self::ApprovalRequired,
            Self::UiOnly,
        ]
    }
}

/// Invocation surface a single command graph must keep in parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandSurfaceClass {
    /// Application menu entry or toolbar button.
    MenuOrButton,
    /// Keybinding / chord.
    Keybinding,
    /// Command palette row.
    CommandPalette,
    /// CLI or headless invocation.
    CliHeadless,
    /// AI tool-call surface.
    AiTool,
    /// Voice / dictation command surface.
    Voice,
    /// Recipe / macro automation step.
    Recipe,
    /// Deep link into a command.
    DeepLink,
    /// Browser-companion surface.
    BrowserCompanion,
}

impl CommandSurfaceClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MenuOrButton => "menu_or_button",
            Self::Keybinding => "keybinding",
            Self::CommandPalette => "command_palette",
            Self::CliHeadless => "cli_headless",
            Self::AiTool => "ai_tool",
            Self::Voice => "voice",
            Self::Recipe => "recipe",
            Self::DeepLink => "deep_link",
            Self::BrowserCompanion => "browser_companion",
        }
    }

    /// True when the surface drives automation rather than a UI user.
    pub const fn is_non_ui(self) -> bool {
        matches!(self, Self::CliHeadless | Self::AiTool | Self::Recipe)
    }

    /// Invocation surfaces the packet must cover to claim cross-surface parity.
    pub const fn required_coverage() -> [Self; 9] {
        [
            Self::MenuOrButton,
            Self::Keybinding,
            Self::CommandPalette,
            Self::CliHeadless,
            Self::AiTool,
            Self::Voice,
            Self::Recipe,
            Self::DeepLink,
            Self::BrowserCompanion,
        ]
    }
}

/// Stable-qualification posture for an invocation surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceQualificationClass {
    /// Surface qualifies for the Stable lane.
    Stable,
    /// Surface is admitted to Beta only.
    Beta,
    /// Surface is experimental.
    Experimental,
    /// Command is not applicable on this surface.
    NotApplicable,
}

impl SurfaceQualificationClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Experimental => "experimental",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when this surface posture is the Stable lane.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// The single descriptor registry and single result schema every surface projects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableContractRefs {
    /// Canonical descriptor schema ref.
    pub descriptor_schema_ref: String,
    /// Canonical registry-entry schema ref.
    pub registry_entry_schema_ref: String,
    /// Canonical seeded-registry artifact ref.
    pub registry_seed_ref: String,
    /// Canonical invocation-session schema ref.
    pub invocation_session_schema_ref: String,
    /// Canonical result-packet schema ref.
    pub result_packet_schema_ref: String,
    /// Canonical public-contract projection schema ref.
    pub public_contract_schema_ref: String,
    /// Canonical parity-expectation schema ref.
    pub parity_expectation_schema_ref: String,
    /// Canonical structured disabled-reason vocabulary ref.
    pub disabled_reason_vocabulary_ref: String,
}

impl StableContractRefs {
    /// The canonical command-lane refs every stable command surface projects from.
    pub fn canonical() -> Self {
        Self {
            descriptor_schema_ref: CANONICAL_DESCRIPTOR_SCHEMA_REF.to_owned(),
            registry_entry_schema_ref: CANONICAL_REGISTRY_ENTRY_SCHEMA_REF.to_owned(),
            registry_seed_ref: CANONICAL_REGISTRY_SEED_REF.to_owned(),
            invocation_session_schema_ref: CANONICAL_INVOCATION_SESSION_SCHEMA_REF.to_owned(),
            result_packet_schema_ref: CANONICAL_RESULT_PACKET_SCHEMA_REF.to_owned(),
            public_contract_schema_ref: CANONICAL_PUBLIC_CONTRACT_SCHEMA_REF.to_owned(),
            parity_expectation_schema_ref: CANONICAL_PARITY_EXPECTATION_SCHEMA_REF.to_owned(),
            disabled_reason_vocabulary_ref: CANONICAL_DISABLED_REASON_VOCABULARY_REF.to_owned(),
        }
    }

    fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// One finalized stable descriptor field row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableDescriptorFieldRow {
    /// Field class being finalized.
    pub field_class: StableDescriptorFieldClass,
    /// Display label safe for docs, help, and support.
    pub field_label: String,
    /// JSON pointer into the descriptor record this field maps to.
    pub descriptor_pointer: String,
    /// True when the field is exported on the boundary.
    pub exported: bool,
    /// True when the field is treated as a stable interface.
    pub stable_interface: bool,
}

/// The stabilized invocation-session and result-packet contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultContractStabilization {
    /// Stable result-code vocabulary the result packet projects.
    pub result_codes: Vec<CommandResultCodeClass>,
    /// True when the result packet preserves the canonical command identity.
    pub preserves_canonical_command_identity: bool,
    /// True when the result packet records alias resolution.
    pub records_alias_resolution: bool,
    /// True when the result packet records the issuing surface and authority class.
    pub records_issuing_surface: bool,
    /// True when the result packet carries created-artifact refs.
    pub carries_artifact_refs: bool,
    /// True when the result packet joins a notification or activity row.
    pub joins_notification_or_activity: bool,
    /// True when durable commands carry a reversible rollback handle.
    pub requires_rollback_handle_for_durable: bool,
    /// True when the result packet supports checkpoint refs.
    pub supports_checkpoints: bool,
    /// True when the result packet carries evidence refs.
    pub carries_evidence_refs: bool,
    /// True when the strict no-bypass guards are enforced.
    pub no_bypass_guards_strict: bool,
}

/// The palette diagnostics contract a stable command row exposes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaletteDiagnosticsContract {
    /// True when the row shows the command source badge.
    pub shows_source_badge: bool,
    /// True when the row shows the resolved keybinding.
    pub shows_keybinding: bool,
    /// True when the row shows the dominant side-effect cue.
    pub shows_dominant_side_effect_cue: bool,
    /// True when the row shows the disabled-with-reason state.
    pub shows_disabled_with_reason: bool,
    /// True when the row shows the preview posture.
    pub shows_preview_posture: bool,
    /// True when the row shows the approval posture.
    pub shows_approval_posture: bool,
    /// Palette actions exposed where valid.
    pub actions: Vec<PaletteActionClass>,
}

impl PaletteDiagnosticsContract {
    fn shows_all_cues(&self) -> bool {
        self.shows_source_badge
            && self.shows_keybinding
            && self.shows_dominant_side_effect_cue
            && self.shows_disabled_with_reason
            && self.shows_preview_posture
            && self.shows_approval_posture
    }
}

/// One structured disabled-reason case every surface resolves identically.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisabledReasonCaseRow {
    /// Reviewer-facing disabled-reason case class.
    pub case_class: DisabledReasonCaseClass,
    /// Canonical machine disabled-reason code from the frozen vocabulary.
    pub disabled_reason_code: DisabledReasonCode,
    /// Structured explanation ref shared by support, docs, CLI, and UI.
    pub explanation_ref: String,
    /// Repair-hook ref surfaced with the disabled reason.
    pub repair_hook_ref: String,
    /// True when support, CLI, palette, and automation resolve the same reason.
    pub surfaces_resolve_identically: bool,
}

/// One cross-surface authority-parity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceParityRow {
    /// Invocation surface this row covers.
    pub surface_class: CommandSurfaceClass,
    /// Command descriptor ref projected on this surface.
    pub descriptor_ref: String,
    /// True when the command is reachable on this surface.
    pub reachable: bool,
    /// True when this surface shares the canonical command descriptor.
    pub shares_command_descriptor: bool,
    /// True when this surface shares the preview model.
    pub shares_preview_model: bool,
    /// True when this surface shares the approval model.
    pub shares_approval_model: bool,
    /// True when this surface shares the rollback model.
    pub shares_rollback_model: bool,
    /// True when this surface shares the audit model.
    pub shares_audit_model: bool,
    /// True when an alias on this surface resolves to the canonical command id.
    pub resolves_to_canonical: bool,
    /// True when this surface discloses provider/route truth.
    pub route_disclosed: bool,
    /// True when this surface runs the same policy checks.
    pub policy_checked: bool,
    /// True when this surface keeps its automation labels honest.
    pub automation_label_honest: bool,
    /// Stable-qualification posture for this surface.
    pub qualification: SurfaceQualificationClass,
    /// True when this surface claims the Stable lane.
    pub claimed_stable: bool,
}

impl SurfaceParityRow {
    fn preserves_full_parity(&self) -> bool {
        self.shares_command_descriptor
            && self.shares_preview_model
            && self.shares_approval_model
            && self.shares_rollback_model
            && self.shares_audit_model
            && self.resolves_to_canonical
            && self.route_disclosed
            && self.policy_checked
            && self.automation_label_honest
    }
}

/// Exportable evidence lineage binding the in-product evidence id to admin and
/// support reconstruction and the rollback lineage a revert replays from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandContractEvidenceExport {
    /// Evidence id shown in-product and reused by admin/support exports.
    pub evidence_id: String,
    /// JSON export ref.
    pub json_export_ref: String,
    /// Markdown summary ref.
    pub markdown_summary_ref: String,
    /// Admin inspector ref that resolves the same evidence id.
    pub admin_inspector_ref: String,
    /// Support export ref that resolves the same evidence id.
    pub support_export_ref: String,
    /// Rollback checkpoint lineage refs preserved for this command family.
    pub rollback_lineage_refs: Vec<String>,
    /// Export lineage refs (prior exports this one descends from).
    pub export_lineage_refs: Vec<String>,
}

/// Constructor input for [`CommandContractStabilizationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandContractStabilizationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Canonical command-family id this packet stabilizes.
    pub command_family_id: String,
    /// Display label safe for UI, docs, and support.
    pub display_label: String,
    /// Whether this row claims the Stable line.
    pub claimed_stable: bool,
    /// Policy epoch ref this row was evaluated under.
    pub policy_epoch_ref: String,
    /// The single descriptor registry and result schema every surface projects.
    pub contract_refs: StableContractRefs,
    /// Finalized stable descriptor field rows.
    pub stable_descriptor_fields: Vec<StableDescriptorFieldRow>,
    /// Stabilized invocation-session and result-packet contract.
    pub result_contract: ResultContractStabilization,
    /// Palette diagnostics contract.
    pub palette_diagnostics: PaletteDiagnosticsContract,
    /// Structured disabled-reason cases.
    pub disabled_reason_cases: Vec<DisabledReasonCaseRow>,
    /// Cross-surface authority-parity rows.
    pub surface_parity_rows: Vec<SurfaceParityRow>,
    /// Exportable evidence lineage.
    pub evidence_export: CommandContractEvidenceExport,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe stabilized command-contract record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandContractStabilizationPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Canonical command-family id this packet stabilizes.
    pub command_family_id: String,
    /// Display label safe for UI, docs, and support.
    pub display_label: String,
    /// Whether this row claims the Stable line.
    pub claimed_stable: bool,
    /// Policy epoch ref this row was evaluated under.
    pub policy_epoch_ref: String,
    /// The single descriptor registry and result schema every surface projects.
    pub contract_refs: StableContractRefs,
    /// Finalized stable descriptor field rows.
    pub stable_descriptor_fields: Vec<StableDescriptorFieldRow>,
    /// Stabilized invocation-session and result-packet contract.
    pub result_contract: ResultContractStabilization,
    /// Palette diagnostics contract.
    pub palette_diagnostics: PaletteDiagnosticsContract,
    /// Structured disabled-reason cases.
    pub disabled_reason_cases: Vec<DisabledReasonCaseRow>,
    /// Cross-surface authority-parity rows.
    pub surface_parity_rows: Vec<SurfaceParityRow>,
    /// Exportable evidence lineage.
    pub evidence_export: CommandContractEvidenceExport,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl CommandContractStabilizationPacket {
    /// Builds a stabilized command-contract packet from canonical rows.
    pub fn new(input: CommandContractStabilizationPacketInput) -> Self {
        Self {
            record_kind: STABILIZE_COMMAND_CONTRACT_RECORD_KIND.to_owned(),
            schema_version: STABILIZE_COMMAND_CONTRACT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            command_family_id: input.command_family_id,
            display_label: input.display_label,
            claimed_stable: input.claimed_stable,
            policy_epoch_ref: input.policy_epoch_ref,
            contract_refs: input.contract_refs,
            stable_descriptor_fields: input.stable_descriptor_fields,
            result_contract: input.result_contract,
            palette_diagnostics: input.palette_diagnostics,
            disabled_reason_cases: input.disabled_reason_cases,
            surface_parity_rows: input.surface_parity_rows,
            evidence_export: input.evidence_export,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the stabilized command-contract packet's stable-line invariants.
    pub fn validate(&self) -> Vec<CommandContractStabilizationViolation> {
        let mut violations = Vec::new();
        if self.record_kind != STABILIZE_COMMAND_CONTRACT_RECORD_KIND {
            violations.push(CommandContractStabilizationViolation::WrongRecordKind);
        }
        if self.schema_version != STABILIZE_COMMAND_CONTRACT_SCHEMA_VERSION {
            violations.push(CommandContractStabilizationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.command_family_id.trim().is_empty()
            || self.display_label.trim().is_empty()
            || self.policy_epoch_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(CommandContractStabilizationViolation::MissingIdentity);
        }
        validate_source_contracts(self, &mut violations);
        validate_contract_refs(self, &mut violations);
        validate_descriptor_fields(self, &mut violations);
        validate_result_contract(self, &mut violations);
        validate_palette_diagnostics(self, &mut violations);
        validate_disabled_reason_cases(self, &mut violations);
        validate_surface_parity(self, &mut violations);
        validate_evidence_export(self, &mut violations);
        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("stabilized command contract packet serializes"),
        ) {
            violations.push(CommandContractStabilizationViolation::RawMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("stabilized command contract packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed_surfaces = self
            .surface_parity_rows
            .iter()
            .filter(|row| !row.qualification.is_stable())
            .count();
        let identically_resolved = self
            .disabled_reason_cases
            .iter()
            .filter(|case| case.surfaces_resolve_identically)
            .count();
        let mut out = String::new();
        out.push_str("# Command Contract Stabilization\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Command family: `{}`\n", self.command_family_id));
        out.push_str(&format!(
            "- Evidence id: `{}`\n",
            self.evidence_export.evidence_id
        ));
        out.push_str(&format!("- Claimed stable: {}\n", self.claimed_stable));
        out.push_str(&format!(
            "- Stable descriptor fields: {}\n",
            self.stable_descriptor_fields.len()
        ));
        out.push_str(&format!(
            "- Result codes: {}\n",
            self.result_contract.result_codes.len()
        ));
        out.push_str(&format!(
            "- Palette actions: {}\n",
            self.palette_diagnostics.actions.len()
        ));
        out.push_str(&format!(
            "- Disabled-reason cases: {} ({} resolve identically across surfaces)\n",
            self.disabled_reason_cases.len(),
            identically_resolved
        ));
        out.push_str(&format!(
            "- Command surfaces: {} ({} narrowed below Stable)\n",
            self.surface_parity_rows.len(),
            narrowed_surfaces
        ));
        out.push_str(&format!(
            "- Rollback lineage refs: {}\n",
            self.evidence_export.rollback_lineage_refs.len()
        ));
        out
    }
}

/// Errors emitted when reading the checked-in stabilized command-contract export.
#[derive(Debug)]
pub enum CommandContractStabilizationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<CommandContractStabilizationViolation>),
}

impl fmt::Display for CommandContractStabilizationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "stabilized command contract export parse failed: {error}"
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
                    "stabilized command contract export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for CommandContractStabilizationArtifactError {}

/// Validation failures emitted by [`CommandContractStabilizationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandContractStabilizationViolation {
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
    /// Stable descriptor field coverage is incomplete.
    MissingDescriptorFieldCoverage,
    /// A stable descriptor field is not exported as a stable interface.
    DescriptorFieldNotStableInterface,
    /// Stable result-code coverage is incomplete.
    MissingResultCodeCoverage,
    /// The result contract does not preserve canonical command/result truth.
    ResultContractNotStabilized,
    /// The palette diagnostics row does not show every required cue.
    PaletteDiagnosticsMissingCue,
    /// The palette diagnostics row does not expose every required action.
    PaletteDiagnosticsMissingAction,
    /// Structured disabled-reason case coverage is incomplete.
    MissingDisabledReasonCoverage,
    /// A disabled-reason case is missing structured refs.
    DisabledReasonCaseRefsMissing,
    /// A disabled-reason case does not resolve identically across surfaces.
    DisabledReasonNotResolvedIdentically,
    /// Cross-surface coverage is incomplete.
    CommandSurfaceCoverageMissing,
    /// A claimed-stable surface broke preview/approval/rollback/audit parity.
    CommandParityBroken,
    /// A surface narrowed below Stable still claims the Stable lane.
    UnqualifiedSurfaceClaimsStable,
    /// Evidence export refs are missing.
    EvidenceExportRefsMissing,
    /// The rollback lineage is missing.
    RollbackLineageMissing,
    /// The packet carries raw material outside the export boundary.
    RawMaterialInExport,
}

impl CommandContractStabilizationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ContractRefsNotCanonical => "contract_refs_not_canonical",
            Self::MissingDescriptorFieldCoverage => "missing_descriptor_field_coverage",
            Self::DescriptorFieldNotStableInterface => "descriptor_field_not_stable_interface",
            Self::MissingResultCodeCoverage => "missing_result_code_coverage",
            Self::ResultContractNotStabilized => "result_contract_not_stabilized",
            Self::PaletteDiagnosticsMissingCue => "palette_diagnostics_missing_cue",
            Self::PaletteDiagnosticsMissingAction => "palette_diagnostics_missing_action",
            Self::MissingDisabledReasonCoverage => "missing_disabled_reason_coverage",
            Self::DisabledReasonCaseRefsMissing => "disabled_reason_case_refs_missing",
            Self::DisabledReasonNotResolvedIdentically => {
                "disabled_reason_not_resolved_identically"
            }
            Self::CommandSurfaceCoverageMissing => "command_surface_coverage_missing",
            Self::CommandParityBroken => "command_parity_broken",
            Self::UnqualifiedSurfaceClaimsStable => "unqualified_surface_claims_stable",
            Self::EvidenceExportRefsMissing => "evidence_export_refs_missing",
            Self::RollbackLineageMissing => "rollback_lineage_missing",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Returns the checked-in stabilized command-contract support export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or validate.
pub fn current_stable_command_contract_stabilization_export(
) -> Result<CommandContractStabilizationPacket, CommandContractStabilizationArtifactError> {
    let packet: CommandContractStabilizationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/commands/m4/stabilize_command_contract/support_export.json"
    )))
    .map_err(CommandContractStabilizationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(CommandContractStabilizationArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &CommandContractStabilizationPacket,
    violations: &mut Vec<CommandContractStabilizationViolation>,
) {
    for required in [
        STABILIZE_COMMAND_CONTRACT_DOC_REF,
        STABILIZE_COMMAND_CONTRACT_SCHEMA_REF,
        STABILIZE_COMMAND_CONTRACT_DESCRIPTOR_CONTRACT_REF,
        STABILIZE_COMMAND_CONTRACT_PARITY_CONTRACT_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(CommandContractStabilizationViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_contract_refs(
    packet: &CommandContractStabilizationPacket,
    violations: &mut Vec<CommandContractStabilizationViolation>,
) {
    if !packet.contract_refs.matches_canonical() {
        violations.push(CommandContractStabilizationViolation::ContractRefsNotCanonical);
    }
}

fn validate_descriptor_fields(
    packet: &CommandContractStabilizationPacket,
    violations: &mut Vec<CommandContractStabilizationViolation>,
) {
    for required in StableDescriptorFieldClass::required_coverage() {
        if !packet
            .stable_descriptor_fields
            .iter()
            .any(|row| row.field_class == required)
        {
            violations.push(CommandContractStabilizationViolation::MissingDescriptorFieldCoverage);
            break;
        }
    }

    for row in &packet.stable_descriptor_fields {
        if row.field_label.trim().is_empty() || row.descriptor_pointer.trim().is_empty() {
            violations.push(CommandContractStabilizationViolation::MissingDescriptorFieldCoverage);
            break;
        }
        // A stable command's descriptor fields must be exported, typed interfaces.
        if !row.exported || !row.stable_interface {
            violations
                .push(CommandContractStabilizationViolation::DescriptorFieldNotStableInterface);
            break;
        }
    }
}

fn validate_result_contract(
    packet: &CommandContractStabilizationPacket,
    violations: &mut Vec<CommandContractStabilizationViolation>,
) {
    let contract = &packet.result_contract;
    for required in CommandResultCodeClass::required_coverage() {
        if !contract.result_codes.iter().any(|code| *code == required) {
            violations.push(CommandContractStabilizationViolation::MissingResultCodeCoverage);
            break;
        }
    }

    // The result packet must preserve outcome truth without inferring it from
    // rendered text: canonical identity, alias resolution, issuing surface,
    // artifact refs, notification/activity join, rollback handle, checkpoints,
    // evidence refs, and the strict no-bypass guards.
    if !(contract.preserves_canonical_command_identity
        && contract.records_alias_resolution
        && contract.records_issuing_surface
        && contract.carries_artifact_refs
        && contract.joins_notification_or_activity
        && contract.requires_rollback_handle_for_durable
        && contract.supports_checkpoints
        && contract.carries_evidence_refs
        && contract.no_bypass_guards_strict)
    {
        violations.push(CommandContractStabilizationViolation::ResultContractNotStabilized);
    }
}

fn validate_palette_diagnostics(
    packet: &CommandContractStabilizationPacket,
    violations: &mut Vec<CommandContractStabilizationViolation>,
) {
    if !packet.palette_diagnostics.shows_all_cues() {
        violations.push(CommandContractStabilizationViolation::PaletteDiagnosticsMissingCue);
    }
    for required in PaletteActionClass::required_coverage() {
        if !packet
            .palette_diagnostics
            .actions
            .iter()
            .any(|action| *action == required)
        {
            violations.push(CommandContractStabilizationViolation::PaletteDiagnosticsMissingAction);
            break;
        }
    }
}

fn validate_disabled_reason_cases(
    packet: &CommandContractStabilizationPacket,
    violations: &mut Vec<CommandContractStabilizationViolation>,
) {
    for required in DisabledReasonCaseClass::required_coverage() {
        if !packet
            .disabled_reason_cases
            .iter()
            .any(|case| case.case_class == required)
        {
            violations.push(CommandContractStabilizationViolation::MissingDisabledReasonCoverage);
            break;
        }
    }

    for case in &packet.disabled_reason_cases {
        if case.explanation_ref.trim().is_empty() || case.repair_hook_ref.trim().is_empty() {
            violations.push(CommandContractStabilizationViolation::DisabledReasonCaseRefsMissing);
            break;
        }
        // Support, CLI, palette, and automation must resolve the same reason.
        if !case.surfaces_resolve_identically {
            violations
                .push(CommandContractStabilizationViolation::DisabledReasonNotResolvedIdentically);
            break;
        }
    }
}

fn validate_surface_parity(
    packet: &CommandContractStabilizationPacket,
    violations: &mut Vec<CommandContractStabilizationViolation>,
) {
    for required in CommandSurfaceClass::required_coverage() {
        if !packet
            .surface_parity_rows
            .iter()
            .any(|row| row.surface_class == required)
        {
            violations.push(CommandContractStabilizationViolation::CommandSurfaceCoverageMissing);
            break;
        }
    }

    for row in &packet.surface_parity_rows {
        if row.descriptor_ref.trim().is_empty() {
            violations.push(CommandContractStabilizationViolation::CommandSurfaceCoverageMissing);
            break;
        }
        // A surface narrowed below Stable may not claim the Stable lane.
        if row.claimed_stable && !row.qualification.is_stable() {
            violations.push(CommandContractStabilizationViolation::UnqualifiedSurfaceClaimsStable);
            break;
        }
        // A Stable, reachable surface must preserve full parity; it may not widen
        // authority or suppress preview/approval/rollback/audit.
        if row.qualification.is_stable() && row.reachable && !row.preserves_full_parity() {
            violations.push(CommandContractStabilizationViolation::CommandParityBroken);
            break;
        }
    }
}

fn validate_evidence_export(
    packet: &CommandContractStabilizationPacket,
    violations: &mut Vec<CommandContractStabilizationViolation>,
) {
    let export = &packet.evidence_export;
    if export.evidence_id.trim().is_empty()
        || export.json_export_ref.trim().is_empty()
        || export.markdown_summary_ref.trim().is_empty()
        || export.admin_inspector_ref.trim().is_empty()
        || export.support_export_ref.trim().is_empty()
    {
        violations.push(CommandContractStabilizationViolation::EvidenceExportRefsMissing);
    }
    if export.rollback_lineage_refs.is_empty()
        || export
            .rollback_lineage_refs
            .iter()
            .any(|reference| reference.trim().is_empty())
    {
        violations.push(CommandContractStabilizationViolation::RollbackLineageMissing);
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
        || lower.contains("raw_body")
        || lower.contains("billing-account")
}

#[cfg(test)]
mod tests;
