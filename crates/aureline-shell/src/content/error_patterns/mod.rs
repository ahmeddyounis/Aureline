//! Reusable error/recovery copy objects and degraded-state reason chips.
//!
//! This module materializes the canonical, export-safe inventory of the
//! error/recovery wording the shell renders when an M5 workflow fails or runs in a
//! degraded state. Where the safety-critical string catalog locks *which* state
//! terms are governed, this catalog locks the *shape* of a recovery explanation:
//! every [`RecoveryBlock`] carries four explicit parts — a [`CopyLine`] for
//! **what failed**, one for **why it likely happened**, one for **what still
//! works**, and a [`NextAction`] with a verb-first label and a [`RecoveryLink`].
//! A surface can never stop at generic failure text: the block's structure forces
//! it to say what remains safe to do and how to proceed.
//!
//! Degraded states are first-class [`ReasonChip`] objects — one per
//! [`DegradedState`] (`Restricted`, `PartialIndex`, `RemoteHost`, `PolicyBlocked`,
//! `Cached`, `Stale`, `Reconnecting`, `RollbackAvailable`) — with a reserved
//! meaning, a locale-neutral machine token, a severity, and the surfaces they may
//! appear on. A copy line never inlines a chip label: its template uses a
//! `{chip:<chip_id>}` placeholder that resolves against the chip register and a
//! `{var:<name>}` placeholder that resolves against the declared variables. So the
//! catalog — not a scattered literal string — is the source of truth, and the same
//! chip resolves identically across dynamic banners, inline blockers, Project
//! Doctor findings, CLI/help summaries, support exports, screenshot/demo captions,
//! and narrated surfaces.
//!
//! Machine-facing identity stays locale-neutral while human prose localizes safely
//! around it. Block ids, chip ids, machine tokens, link ids, and variable names are
//! lowercase ascii (`[a-z0-9_.]`); only the canonical labels, reserved meanings,
//! and reference templates carry human prose. A localized overlay rewrites the
//! prose but never the ids or the `{chip:...}` / `{var:...}` placeholders, so a
//! translation can never fork the meaning of a failure or a degraded state, and a
//! support export can reconstruct the exact explanation the user saw.
//!
//! The boundary schema is
//! [`schemas/content/m5-error-recovery-copy.schema.json`](../../../../../schemas/content/m5-error-recovery-copy.schema.json).
//! The contract doc is
//! [`docs/content/m5/m5_error_recovery_copy.md`](../../../../../docs/content/m5/m5_error_recovery_copy.md).
//! The protected fixture directory is
//! [`fixtures/content/m5-error-recovery-copy/`](../../../../../fixtures/content/m5-error-recovery-copy/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_error_recovery_copy_catalog, seeded_error_recovery_copy_catalog_localized,
    seeded_error_recovery_copy_catalog_offline_mirror, ERROR_RECOVERY_COPY_CATALOG_ID,
};

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`ErrorRecoveryCopyCatalog`].
pub const ERROR_RECOVERY_COPY_CATALOG_RECORD_KIND: &str = "m5_error_recovery_copy_catalog";

/// Schema version for error/recovery copy catalog records.
pub const ERROR_RECOVERY_COPY_CATALOG_SCHEMA_VERSION: u32 = 1;

/// Minimum number of distinct consumer surfaces a shared reuse chip must span.
pub const SHARED_CHIP_MIN_REUSE_SURFACES: usize = 3;

/// Repo-relative path of the boundary schema.
pub const ERROR_RECOVERY_COPY_CATALOG_SCHEMA_REF: &str =
    "schemas/content/m5-error-recovery-copy.schema.json";

/// Repo-relative path of the catalog contract doc.
pub const ERROR_RECOVERY_COPY_CATALOG_DOC_REF: &str = "docs/content/m5/m5_error_recovery_copy.md";

/// Repo-relative path of the frozen UI copy contract (action labels, error copy).
pub const RECOVERY_UI_COPY_CONTRACT_REF: &str = "docs/copy/ui_copy_contract.md";

/// Repo-relative path of the frozen naming and state-label contract.
pub const RECOVERY_NAMING_LABEL_CONTRACT_REF: &str = "docs/copy/naming_and_state_label_contract.md";

/// Repo-relative path of the controlled glossary register; chip machine tokens
/// must align with the controlled state vocabulary owned there.
pub const RECOVERY_CONTROLLED_GLOSSARY_REF: &str = "artifacts/copy/controlled_glossary.yaml";

/// Repo-relative path of the upstream safety-critical string catalog schema, the
/// sibling lane that owns the controlled state terms these chips project.
pub const RECOVERY_SAFETY_CRITICAL_SCHEMA_REF: &str =
    "schemas/content/m5-safety-critical-strings.schema.json";

/// Repo-relative path of the upstream safety-critical string catalog doc.
pub const RECOVERY_SAFETY_CRITICAL_DOC_REF: &str =
    "docs/content/m5/m5_safety_critical_string_catalog.md";

/// Repo-relative path of the protected fixture directory.
pub const ERROR_RECOVERY_COPY_CATALOG_FIXTURE_DIR: &str = "fixtures/content/m5-error-recovery-copy";

/// Repo-relative path of the checked support-export artifact.
pub const ERROR_RECOVERY_COPY_CATALOG_ARTIFACT_REF: &str =
    "artifacts/content/m5-recovery-copy-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const ERROR_RECOVERY_COPY_CATALOG_SUMMARY_REF: &str =
    "artifacts/content/m5-recovery-copy-proof/m5_error_recovery_copy.md";

/// The subsystem whose failure or degradation a recovery block explains.
///
/// These are the concrete recovery surfaces the goal names: runtime, network,
/// repair, install, review, and docs/help.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureDomain {
    /// Local runtime / execution surface.
    Runtime,
    /// Network / remote connectivity surface.
    Network,
    /// Repair / Project Doctor surface.
    Repair,
    /// Install / update surface.
    Install,
    /// Review / change-approval surface.
    Review,
    /// Documentation / help surface.
    DocsHelp,
}

impl FailureDomain {
    /// Every failure domain, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Runtime,
        Self::Network,
        Self::Repair,
        Self::Install,
        Self::Review,
        Self::DocsHelp,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Runtime => "runtime",
            Self::Network => "network",
            Self::Repair => "repair",
            Self::Install => "install",
            Self::Review => "review",
            Self::DocsHelp => "docs_help",
        }
    }
}

/// Severity of a failure / degraded-state explanation, ordered low to high.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoverySeverity {
    /// A degraded notice worth reading but not a hazard.
    Notice,
    /// A caution: proceed carefully within a narrower capability.
    Caution,
    /// A warning about a degraded or risky state.
    Warning,
    /// A critical condition demanding attention.
    Critical,
    /// A blocking condition: the action cannot proceed as requested.
    Blocking,
}

impl RecoverySeverity {
    /// Every severity, low to high.
    pub const ALL: [Self; 5] = [
        Self::Notice,
        Self::Caution,
        Self::Warning,
        Self::Critical,
        Self::Blocking,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notice => "notice",
            Self::Caution => "caution",
            Self::Warning => "warning",
            Self::Critical => "critical",
            Self::Blocking => "blocking",
        }
    }
}

/// A reusable degraded-state a reason chip declares.
///
/// These are exactly the degraded states the goal enumerates; the catalog must
/// carry one chip for each so a surface never invents a synonym.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradedState {
    /// The capability is permitted only within a narrower, disclosed scope.
    Restricted,
    /// The index is incomplete; results are partial until it finishes.
    PartialIndex,
    /// Work depends on a remote host whose reachability is not guaranteed.
    RemoteHost,
    /// The action is blocked by an active policy.
    PolicyBlocked,
    /// Data is shown from a cache, not proven current.
    Cached,
    /// Prior data is shown after its freshness floor was lost.
    Stale,
    /// A connection is being re-established; the state self-heals when it returns.
    Reconnecting,
    /// A prior, known-good state can be restored.
    RollbackAvailable,
}

impl DegradedState {
    /// Every degraded state, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Restricted,
        Self::PartialIndex,
        Self::RemoteHost,
        Self::PolicyBlocked,
        Self::Cached,
        Self::Stale,
        Self::Reconnecting,
        Self::RollbackAvailable,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Restricted => "restricted",
            Self::PartialIndex => "partial_index",
            Self::RemoteHost => "remote_host",
            Self::PolicyBlocked => "policy_blocked",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::Reconnecting => "reconnecting",
            Self::RollbackAvailable => "rollback_available",
        }
    }
}

/// A consumer surface that must resolve the same recovery copy object.
///
/// These are the surfaces the goal names: dynamic banners, inline blockers,
/// Project Doctor, CLI/help summaries, support exports, screenshot/demo captions,
/// and narrated surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryConsumerSurface {
    /// A dynamic, dismissible banner.
    DynamicBanner,
    /// An inline blocker that gates an action in place.
    InlineBlocker,
    /// A Project Doctor finding.
    ProjectDoctor,
    /// A CLI / help summary line.
    CliHelpSummary,
    /// A support / export packet.
    SupportExport,
    /// A screenshot / demo caption.
    ScreenshotCaption,
    /// A screen-reader / narrated surface.
    ScreenReader,
}

impl RecoveryConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::DynamicBanner,
        Self::InlineBlocker,
        Self::ProjectDoctor,
        Self::CliHelpSummary,
        Self::SupportExport,
        Self::ScreenshotCaption,
        Self::ScreenReader,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DynamicBanner => "dynamic_banner",
            Self::InlineBlocker => "inline_blocker",
            Self::ProjectDoctor => "project_doctor",
            Self::CliHelpSummary => "cli_help_summary",
            Self::SupportExport => "support_export",
            Self::ScreenshotCaption => "screenshot_caption",
            Self::ScreenReader => "screen_reader",
        }
    }
}

/// Which of the three prose parts a [`CopyLine`] fills.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CopyRole {
    /// The bounded statement of what failed.
    WhatFailed,
    /// The likely cause, hedged honestly.
    WhyLikely,
    /// What still works locally / what remains safe to do.
    WhatStillWorks,
}

impl CopyRole {
    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WhatFailed => "what_failed",
            Self::WhyLikely => "why_likely",
            Self::WhatStillWorks => "what_still_works",
        }
    }
}

/// Semantic role of a declared copy variable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CopyVariableRole {
    /// A human-entity name (host, project, capability).
    EntityName,
    /// A path or location.
    Location,
    /// A machine-facing code or identifier.
    Code,
    /// A numeric count.
    Count,
    /// A duration or elapsed-time value.
    Duration,
    /// A scope / omission-reason label.
    ScopeLabel,
}

impl CopyVariableRole {
    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EntityName => "entity_name",
            Self::Location => "location",
            Self::Code => "code",
            Self::Count => "count",
            Self::Duration => "duration",
            Self::ScopeLabel => "scope_label",
        }
    }

    /// Whether values for this role are locale-neutral (codes, counts, durations).
    pub const fn is_locale_neutral_value(self) -> bool {
        matches!(self, Self::Code | Self::Count | Self::Duration)
    }
}

/// The kind of target a [`RecoveryLink`] points at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryLinkKind {
    /// A documentation topic.
    DocsTopic,
    /// A help topic.
    HelpTopic,
    /// A repair / Project Doctor flow.
    RepairFlow,
    /// A settings pane.
    SettingsPane,
    /// A reconnect flow.
    ReconnectFlow,
    /// A rollback / restore flow.
    RollbackFlow,
    /// A support / export action.
    SupportExport,
}

impl RecoveryLinkKind {
    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsTopic => "docs_topic",
            Self::HelpTopic => "help_topic",
            Self::RepairFlow => "repair_flow",
            Self::SettingsPane => "settings_pane",
            Self::ReconnectFlow => "reconnect_flow",
            Self::RollbackFlow => "rollback_flow",
            Self::SupportExport => "support_export",
        }
    }
}

/// A reusable degraded-state reason chip: one reserved meaning, one machine token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasonChip {
    /// Stable, locale-neutral chip id (e.g. `chip.policy_blocked`).
    pub chip_id: String,
    /// The degraded state this chip names.
    pub state: DegradedState,
    /// Locale-neutral machine token (flag/code/JSON key), aligned with the
    /// controlled glossary's state vocabulary.
    pub machine_token: String,
    /// Canonical (default-locale) display label.
    pub canonical_label: String,
    /// The single reserved meaning this chip holds everywhere.
    pub reserved_meaning: String,
    /// Severity carried by the chip.
    pub severity: RecoverySeverity,
    /// True when the state self-heals once the underlying condition clears.
    pub self_heals: bool,
    /// True when the state offers an explicit recovery affordance.
    pub offers_recovery: bool,
    /// True when the chip is stated in grounded cause/boundary language and is
    /// never softened into a euphemism. Must be `true`.
    pub grounded: bool,
    /// Consumer surfaces this chip may appear on.
    pub allowed_surfaces: Vec<RecoveryConsumerSurface>,
}

/// A declared variable slot in a copy line or next-action label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyVariable {
    /// Locale-neutral variable name used in the `{var:<name>}` placeholder.
    pub name: String,
    /// Semantic role of the variable.
    pub role: CopyVariableRole,
    /// True when the variable's *value* is locale-neutral (codes, counts).
    pub locale_neutral_value: bool,
    /// True when the variable's rendering may be truncated.
    pub truncatable: bool,
}

/// One prose part of a recovery block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyLine {
    /// Which part this line fills.
    pub role: CopyRole,
    /// Non-authoritative default-locale reference rendering, built from
    /// `{chip:<id>}` and `{var:<name>}` placeholders.
    pub reference_template: String,
    /// Reason-chip ids this line embeds.
    pub chip_refs: Vec<String>,
    /// Declared variable slots.
    pub variables: Vec<CopyVariable>,
}

/// A recovery link attached to a next action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryLink {
    /// Stable, locale-neutral link id (e.g. `link.flow.reconnect_host`).
    pub link_id: String,
    /// The kind of target.
    pub kind: RecoveryLinkKind,
    /// Locale-neutral target ref (topic/flow/pane id).
    pub target_ref: String,
    /// Canonical (default-locale) link label.
    pub label: String,
    /// True when the link resolves offline / air-gapped, so the recovery entry
    /// point is reachable even while degraded. Must be `true`.
    pub offline_available: bool,
}

/// The verb-first next action a recovery block offers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NextAction {
    /// Stable, locale-neutral action id (e.g. `action.reconnect_remote_host`).
    pub action_id: String,
    /// Verb-first imperative label (e.g. `Reconnect to {var:host_name}`).
    pub label: String,
    /// Declared variable slots for the label.
    pub variables: Vec<CopyVariable>,
    /// The recovery link the action opens.
    pub recovery_link: RecoveryLink,
}

/// One error/recovery copy object.
///
/// The four explicit parts force a complete explanation: what failed, why it
/// likely happened, what still works, and the next action with its recovery link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryBlock {
    /// Stable, locale-neutral block id (e.g. `recovery.network.remote_host_unreachable`).
    pub block_id: String,
    /// The subsystem whose failure or degradation this block explains.
    pub failure_domain: FailureDomain,
    /// Severity of the failure / degraded state.
    pub severity: RecoverySeverity,
    /// The bounded statement of what failed.
    pub what_failed: CopyLine,
    /// The likely cause, hedged honestly.
    pub why_likely: CopyLine,
    /// What still works / remains safe to do.
    pub what_still_works: CopyLine,
    /// The verb-first next action with its recovery link.
    pub next_action: NextAction,
    /// The distinct reason chips this block carries (union of its lines' refs).
    pub reason_chips: Vec<String>,
    /// Consumer surfaces that must resolve this block.
    pub consumer_surfaces: Vec<RecoveryConsumerSurface>,
}

impl RecoveryBlock {
    /// The three prose lines, in render order.
    pub fn lines(&self) -> [&CopyLine; 3] {
        [&self.what_failed, &self.why_likely, &self.what_still_works]
    }
}

/// Catalog-level trust and recovery-honesty review block.
///
/// Every flag is a hard invariant; all must hold for the catalog to validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryTrustReview {
    /// Blocks explain failure, likely cause, remaining capability, and next action.
    pub blocks_explain_failure_cause_remaining_and_next_action: bool,
    /// Recovery messaging always states what still works and how to proceed.
    pub recovery_messaging_states_what_still_works_and_how_to_proceed: bool,
    /// Degraded-state chips are reused, never reinvented per surface.
    pub degraded_state_chips_reused_not_reinvented_per_surface: bool,
    /// Chips use grounded cause/boundary language, never euphemism.
    pub chips_use_grounded_cause_language_not_euphemism: bool,
    /// Next-action labels are verb-first and carry a recovery link.
    pub next_action_labels_are_verb_first_with_recovery_link: bool,
    /// Machine tokens, ids, and placeholders stay locale-neutral.
    pub machine_tokens_and_ids_stay_locale_neutral: bool,
    /// Human prose localizes around the locale-neutral tokens.
    pub human_prose_localizes_around_tokens: bool,
    /// Support export reconstructs the same explanation the user saw in-product.
    pub support_export_reconstructs_in_product_explanation: bool,
    /// One catalog is the source of truth, not parallel recovery islands.
    pub one_catalog_not_parallel_recovery_islands: bool,
    /// Recovery links resolve offline so a degraded user can still proceed.
    pub recovery_links_resolve_offline: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryConsumerProjection {
    /// Dynamic banners resolve copy through the catalog.
    pub dynamic_banners_resolve_through_catalog: bool,
    /// Inline blockers resolve copy through the catalog.
    pub inline_blockers_resolve_through_catalog: bool,
    /// Project Doctor reuses the block identities.
    pub project_doctor_reuses_block_identities: bool,
    /// CLI / help summaries show the same copy.
    pub cli_help_summaries_show_same_copy: bool,
    /// Support export uses the catalog blocks.
    pub support_export_uses_catalog_blocks: bool,
    /// Screenshot / demo captions reuse the block copy.
    pub screenshot_captions_reuse_block_copy: bool,
    /// Screen-reader announcements reuse the block identities.
    pub screen_reader_reuses_block_identities: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the catalog claim.
    pub auto_narrow_on_stale: bool,
}

/// Release and mirror/offline parity posture for the catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting mirror/offline packet.
    pub mirror_offline_packet_ref: String,
    /// True when support/export parity is required for every block.
    pub support_export_parity_required: bool,
    /// True when mirror/offline parity is required for every block.
    pub mirror_offline_parity_required: bool,
}

/// Constructor input for [`ErrorRecoveryCopyCatalog::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorRecoveryCopyCatalogInput {
    /// Stable catalog id.
    pub catalog_id: String,
    /// Human-readable catalog label.
    pub catalog_label: String,
    /// Reference locale of the default templates (e.g. `en`).
    pub reference_locale: String,
    /// Degraded-state reason chips.
    pub chips: Vec<ReasonChip>,
    /// Error/recovery blocks.
    pub blocks: Vec<RecoveryBlock>,
    /// Shared reuse chip ids that must span multiple surfaces.
    pub shared_reuse_chip_ids: Vec<String>,
    /// Trust review block.
    pub trust_review: RecoveryTrustReview,
    /// Consumer projection block.
    pub consumer_projection: RecoveryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: RecoveryProofFreshness,
    /// Release posture.
    pub release_posture: RecoveryReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe error/recovery copy catalog packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorRecoveryCopyCatalog {
    /// Record kind; must equal [`ERROR_RECOVERY_COPY_CATALOG_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`ERROR_RECOVERY_COPY_CATALOG_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable catalog id.
    pub catalog_id: String,
    /// Human-readable catalog label.
    pub catalog_label: String,
    /// Reference locale of the default templates.
    pub reference_locale: String,
    /// Closed failure-domain inventory (locale-neutral tokens).
    pub domain_inventory: Vec<String>,
    /// Closed severity inventory (locale-neutral tokens, low to high).
    pub severity_inventory: Vec<String>,
    /// Closed degraded-state inventory (locale-neutral tokens).
    pub degraded_state_inventory: Vec<String>,
    /// Closed consumer-surface inventory (locale-neutral tokens).
    pub surface_inventory: Vec<String>,
    /// Degraded-state reason chips.
    pub chips: Vec<ReasonChip>,
    /// Error/recovery blocks.
    pub blocks: Vec<RecoveryBlock>,
    /// Shared reuse chip ids that must span multiple surfaces.
    pub shared_reuse_chip_ids: Vec<String>,
    /// Trust review block.
    pub trust_review: RecoveryTrustReview,
    /// Consumer projection block.
    pub consumer_projection: RecoveryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: RecoveryProofFreshness,
    /// Release posture.
    pub release_posture: RecoveryReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ErrorRecoveryCopyCatalog {
    /// Builds a catalog packet from lane input, filling the closed inventories
    /// from the canonical enum token lists.
    pub fn new(input: ErrorRecoveryCopyCatalogInput) -> Self {
        Self {
            record_kind: ERROR_RECOVERY_COPY_CATALOG_RECORD_KIND.to_owned(),
            schema_version: ERROR_RECOVERY_COPY_CATALOG_SCHEMA_VERSION,
            catalog_id: input.catalog_id,
            catalog_label: input.catalog_label,
            reference_locale: input.reference_locale,
            domain_inventory: token_list(&FailureDomain::ALL, FailureDomain::as_str),
            severity_inventory: token_list(&RecoverySeverity::ALL, RecoverySeverity::as_str),
            degraded_state_inventory: token_list(&DegradedState::ALL, DegradedState::as_str),
            surface_inventory: token_list(
                &RecoveryConsumerSurface::ALL,
                RecoveryConsumerSurface::as_str,
            ),
            chips: input.chips,
            blocks: input.blocks,
            shared_reuse_chip_ids: input.shared_reuse_chip_ids,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Resolves a block by id.
    pub fn block(&self, block_id: &str) -> Option<&RecoveryBlock> {
        self.blocks.iter().find(|b| b.block_id == block_id)
    }

    /// Resolves a reason chip by id.
    pub fn chip(&self, chip_id: &str) -> Option<&ReasonChip> {
        self.chips.iter().find(|c| c.chip_id == chip_id)
    }

    /// Renders the default-locale reference text for a single template, resolving
    /// each `{chip:<id>}` to its canonical label and keeping each `{var:<name>}`
    /// as a named slot.
    fn render_template(&self, template: &str) -> String {
        let mut out = String::new();
        for segment in parse_template(template) {
            match segment {
                TemplateSegment::Text(text) => out.push_str(&text),
                TemplateSegment::Chip(chip_id) => match self.chip(&chip_id) {
                    Some(chip) => out.push_str(&chip.canonical_label),
                    None => out.push_str(&format!("{{chip:{chip_id}}}")),
                },
                TemplateSegment::Var(name) => out.push_str(&format!("{{{name}}}")),
                TemplateSegment::Unknown(raw) => out.push_str(&raw),
            }
        }
        out
    }

    /// Renders the full default-locale explanation for a block: every part, the
    /// next-action label, and the recovery-link label, with chips resolved and
    /// variables kept as named slots. Returns `None` if the block id is unknown.
    ///
    /// This is the catalog's consumer entry point — a surface never inlines a
    /// degraded-state label, it asks the catalog to resolve the chip.
    pub fn render_block_reference(&self, block_id: &str) -> Option<String> {
        let block = self.block(block_id)?;
        let mut out = String::new();
        out.push_str("Failed: ");
        out.push_str(&self.render_template(&block.what_failed.reference_template));
        out.push_str(" Why: ");
        out.push_str(&self.render_template(&block.why_likely.reference_template));
        out.push_str(" Still works: ");
        out.push_str(&self.render_template(&block.what_still_works.reference_template));
        out.push_str(" Next: ");
        out.push_str(&self.render_template(&block.next_action.label));
        out.push_str(&format!(" ({})", block.next_action.recovery_link.label));
        Some(out)
    }

    /// Maps each reason-chip id to the distinct consumer surfaces that embed it.
    ///
    /// This is the reuse proof: a shared chip that appears on a banner, a CLI
    /// summary, and a support export shows the catalog is the one source the
    /// surfaces share.
    pub fn cross_surface_reuse(&self) -> BTreeMap<String, BTreeSet<&'static str>> {
        let mut reuse: BTreeMap<String, BTreeSet<&'static str>> = BTreeMap::new();
        for block in &self.blocks {
            for chip_id in &block.reason_chips {
                let entry = reuse.entry(chip_id.clone()).or_default();
                for surface in &block.consumer_surfaces {
                    entry.insert(surface.as_str());
                }
            }
        }
        reuse
    }

    /// Validates every catalog invariant.
    pub fn validate(&self) -> Vec<ErrorRecoveryCopyViolation> {
        let mut violations = Vec::new();

        if self.record_kind != ERROR_RECOVERY_COPY_CATALOG_RECORD_KIND {
            violations.push(ErrorRecoveryCopyViolation::WrongRecordKind);
        }
        if self.schema_version != ERROR_RECOVERY_COPY_CATALOG_SCHEMA_VERSION {
            violations.push(ErrorRecoveryCopyViolation::WrongSchemaVersion);
        }
        if self.catalog_id.trim().is_empty()
            || self.catalog_label.trim().is_empty()
            || self.reference_locale.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ErrorRecoveryCopyViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_inventories(self, &mut violations);
        validate_chips(self, &mut violations);
        validate_blocks(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_shared_reuse(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("error/recovery copy catalog serializes"),
        ) {
            violations.push(ErrorRecoveryCopyViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("error/recovery copy catalog serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Error/Recovery Copy Objects and Degraded-State Reason Chips\n\n");
        out.push_str(&format!("- Catalog: `{}`\n", self.catalog_id));
        out.push_str(&format!("- Label: `{}`\n", self.catalog_label));
        out.push_str(&format!(
            "- Reference locale: `{}`\n",
            self.reference_locale
        ));
        out.push_str(&format!(
            "- Reason chips: {} | Recovery blocks: {}\n",
            self.chips.len(),
            self.blocks.len()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Degraded-state reason chips\n\n");
        for chip in &self.chips {
            out.push_str(&format!(
                "- `{}` ({}, token `{}`, {}): {}\n",
                chip.chip_id,
                chip.state.as_str(),
                chip.machine_token,
                chip.severity.as_str(),
                chip.reserved_meaning
            ));
        }

        out.push_str("\n## Recovery blocks\n\n");
        for block in &self.blocks {
            out.push_str(&format!(
                "- `{}` [{} / {}]\n",
                block.block_id,
                block.failure_domain.as_str(),
                block.severity.as_str()
            ));
            if let Some(rendered) = self.render_block_reference(&block.block_id) {
                out.push_str(&format!("  - {rendered}\n"));
            }
            if !block.reason_chips.is_empty() {
                out.push_str(&format!("  - Chips: {}\n", block.reason_chips.join(", ")));
            }
        }

        out.push_str("\n## Cross-surface chip reuse\n\n");
        for (chip_id, surfaces) in self.cross_surface_reuse() {
            out.push_str(&format!(
                "- `{}`: {}\n",
                chip_id,
                surfaces.into_iter().collect::<Vec<_>>().join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in catalog export.
#[derive(Debug)]
pub enum ErrorRecoveryCopyArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ErrorRecoveryCopyViolation>),
}

impl fmt::Display for ErrorRecoveryCopyArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "error/recovery copy catalog export parse failed: {error}"
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
                    "error/recovery copy catalog export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ErrorRecoveryCopyArtifactError {}

/// Validation failures emitted by [`ErrorRecoveryCopyCatalog::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorRecoveryCopyViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A closed inventory drifted from the canonical token list.
    InventoryDrift,
    /// A reason chip is incomplete.
    ChipIncomplete,
    /// A chip id or machine token is not locale-neutral.
    ChipTokenNotLocaleNeutral,
    /// A reason chip id or machine token is duplicated.
    DuplicateChip,
    /// A chip is not grounded (softened to euphemism or not flagged grounded).
    ChipNotGrounded,
    /// A degraded state has no chip.
    DegradedStateNotCovered,
    /// A recovery block is incomplete.
    BlockIncomplete,
    /// A block id is not locale-neutral.
    BlockIdNotLocaleNeutral,
    /// A block id is duplicated.
    DuplicateBlock,
    /// A recovery block omits one of its four required parts.
    RecoveryBlockMissingPart,
    /// A copy line has the wrong role for its slot.
    CopyLineRoleMismatch,
    /// The what-still-works line is empty or says nothing remains.
    WhatStillWorksMissing,
    /// A next-action label is not verb-first.
    NextActionNotVerbFirst,
    /// A next-action recovery link is incomplete.
    NextActionMissingRecoveryLink,
    /// A recovery link does not resolve offline.
    RecoveryLinkNotOfflineResolvable,
    /// A link id or target ref is not locale-neutral.
    LinkTokenNotLocaleNeutral,
    /// A variable name is not locale-neutral.
    VariableNameNotLocaleNeutral,
    /// A template placeholder does not resolve to a declared chip or variable.
    TemplatePlaceholderUnresolved,
    /// A declared chip ref or variable is not used by its template.
    DeclaredTokenUnused,
    /// A line references a chip that does not resolve.
    ChipRefUnresolved,
    /// A block embeds a chip on a surface the chip does not allow.
    ChipUsedOnDisallowedSurface,
    /// A block's copy uses playful, anthropomorphic, or generic failure language.
    BlockUsesGenericOrPlayfulTone,
    /// A failure domain, severity, or consumer surface is never used by a block.
    CoverageGap,
    /// A shared reuse chip does not span enough surfaces.
    SharedChipReuseInsufficient,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/mirror-offline parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl ErrorRecoveryCopyViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::InventoryDrift => "inventory_drift",
            Self::ChipIncomplete => "chip_incomplete",
            Self::ChipTokenNotLocaleNeutral => "chip_token_not_locale_neutral",
            Self::DuplicateChip => "duplicate_chip",
            Self::ChipNotGrounded => "chip_not_grounded",
            Self::DegradedStateNotCovered => "degraded_state_not_covered",
            Self::BlockIncomplete => "block_incomplete",
            Self::BlockIdNotLocaleNeutral => "block_id_not_locale_neutral",
            Self::DuplicateBlock => "duplicate_block",
            Self::RecoveryBlockMissingPart => "recovery_block_missing_part",
            Self::CopyLineRoleMismatch => "copy_line_role_mismatch",
            Self::WhatStillWorksMissing => "what_still_works_missing",
            Self::NextActionNotVerbFirst => "next_action_not_verb_first",
            Self::NextActionMissingRecoveryLink => "next_action_missing_recovery_link",
            Self::RecoveryLinkNotOfflineResolvable => "recovery_link_not_offline_resolvable",
            Self::LinkTokenNotLocaleNeutral => "link_token_not_locale_neutral",
            Self::VariableNameNotLocaleNeutral => "variable_name_not_locale_neutral",
            Self::TemplatePlaceholderUnresolved => "template_placeholder_unresolved",
            Self::DeclaredTokenUnused => "declared_token_unused",
            Self::ChipRefUnresolved => "chip_ref_unresolved",
            Self::ChipUsedOnDisallowedSurface => "chip_used_on_disallowed_surface",
            Self::BlockUsesGenericOrPlayfulTone => "block_uses_generic_or_playful_tone",
            Self::CoverageGap => "coverage_gap",
            Self::SharedChipReuseInsufficient => "shared_chip_reuse_insufficient",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in catalog export.
pub fn current_error_recovery_copy_catalog_export(
) -> Result<ErrorRecoveryCopyCatalog, ErrorRecoveryCopyArtifactError> {
    let packet: ErrorRecoveryCopyCatalog = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/content/m5-recovery-copy-proof/support_export.json"
    )))
    .map_err(ErrorRecoveryCopyArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ErrorRecoveryCopyArtifactError::Validation(violations))
    }
}

/// A parsed segment of a reference template.
#[derive(Debug, Clone, PartialEq, Eq)]
enum TemplateSegment {
    /// Literal text run.
    Text(String),
    /// A `{chip:<id>}` placeholder; carries the chip id.
    Chip(String),
    /// A `{var:<name>}` placeholder; carries the variable name.
    Var(String),
    /// An unrecognized `{...}` placeholder; carries the raw `{...}` text.
    Unknown(String),
}

/// Parses a reference template into ordered text/placeholder segments.
///
/// Placeholders are `{chip:<id>}` or `{var:<name>}`. A `{...}` that is not one of
/// those, or an unbalanced brace, becomes an [`TemplateSegment::Unknown`] segment so
/// validation can reject it.
fn parse_template(template: &str) -> Vec<TemplateSegment> {
    let mut segments = Vec::new();
    let mut text = String::new();
    let mut chars = template.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '{' {
            if !text.is_empty() {
                segments.push(TemplateSegment::Text(std::mem::take(&mut text)));
            }
            let mut inner = String::new();
            let mut closed = false;
            for inner_ch in chars.by_ref() {
                if inner_ch == '}' {
                    closed = true;
                    break;
                }
                inner.push(inner_ch);
            }
            if !closed {
                segments.push(TemplateSegment::Unknown(format!("{{{inner}")));
            } else if let Some(id) = inner.strip_prefix("chip:") {
                segments.push(TemplateSegment::Chip(id.to_owned()));
            } else if let Some(name) = inner.strip_prefix("var:") {
                segments.push(TemplateSegment::Var(name.to_owned()));
            } else {
                segments.push(TemplateSegment::Unknown(format!("{{{inner}}}")));
            }
        } else {
            text.push(ch);
        }
    }
    if !text.is_empty() {
        segments.push(TemplateSegment::Text(text));
    }
    segments
}

/// True when `token` is a locale-neutral machine identifier: non-empty and only
/// lowercase ascii letters, digits, `_`, and `.`.
fn is_locale_neutral(token: &str) -> bool {
    !token.is_empty()
        && token
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '.')
}

fn token_list<T: Copy>(all: &[T], as_str: fn(T) -> &'static str) -> Vec<String> {
    all.iter().map(|t| as_str(*t).to_owned()).collect()
}

/// The controlled verbs a next-action label may open with. Verb-first, concrete
/// recovery verbs only — never a vague `Continue` / `Accept` / `Submit`.
const RECOVERY_VERBS: [&str; 24] = [
    "Reconnect",
    "Retry",
    "Rebuild",
    "Reindex",
    "Request",
    "Open",
    "Review",
    "Restore",
    "Roll",
    "Switch",
    "Grant",
    "Install",
    "Update",
    "Refresh",
    "Reload",
    "Repair",
    "Resume",
    "Inspect",
    "Replace",
    "Cancel",
    "Verify",
    "Enable",
    "Export",
    "Download",
];

/// True when a next-action label opens with a controlled recovery verb.
fn is_verb_first(label: &str) -> bool {
    match label.split_whitespace().next() {
        Some(first) => RECOVERY_VERBS.contains(&first),
        None => false,
    }
}

/// Phrases that are playful, anthropomorphic, or generic failure filler. Grounded
/// recovery copy on a protected path must never use them.
const FORBIDDEN_TONE: [&str; 14] = [
    "oops",
    "uh oh",
    "uh-oh",
    "whoops",
    "yikes",
    "oh no",
    "don't worry",
    "dont worry",
    "no worries",
    "sorry",
    "our bad",
    "my bad",
    "something went wrong",
    "hang tight",
];

/// True when copy uses playful, anthropomorphic, or generic failure language.
fn uses_forbidden_tone(text: &str) -> bool {
    let lower = text.to_lowercase();
    FORBIDDEN_TONE.iter().any(|phrase| lower.contains(phrase))
}

fn validate_source_contracts(
    packet: &ErrorRecoveryCopyCatalog,
    violations: &mut Vec<ErrorRecoveryCopyViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        ERROR_RECOVERY_COPY_CATALOG_SCHEMA_REF,
        ERROR_RECOVERY_COPY_CATALOG_DOC_REF,
        RECOVERY_UI_COPY_CONTRACT_REF,
        RECOVERY_NAMING_LABEL_CONTRACT_REF,
        RECOVERY_CONTROLLED_GLOSSARY_REF,
        RECOVERY_SAFETY_CRITICAL_SCHEMA_REF,
        RECOVERY_SAFETY_CRITICAL_DOC_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ErrorRecoveryCopyViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_inventories(
    packet: &ErrorRecoveryCopyCatalog,
    violations: &mut Vec<ErrorRecoveryCopyViolation>,
) {
    if packet.domain_inventory != token_list(&FailureDomain::ALL, FailureDomain::as_str)
        || packet.severity_inventory != token_list(&RecoverySeverity::ALL, RecoverySeverity::as_str)
        || packet.degraded_state_inventory != token_list(&DegradedState::ALL, DegradedState::as_str)
        || packet.surface_inventory
            != token_list(
                &RecoveryConsumerSurface::ALL,
                RecoveryConsumerSurface::as_str,
            )
    {
        violations.push(ErrorRecoveryCopyViolation::InventoryDrift);
    }
}

fn validate_chips(
    packet: &ErrorRecoveryCopyCatalog,
    violations: &mut Vec<ErrorRecoveryCopyViolation>,
) {
    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    let mut seen_tokens: BTreeSet<&str> = BTreeSet::new();
    for chip in &packet.chips {
        if chip.canonical_label.trim().is_empty()
            || chip.reserved_meaning.trim().is_empty()
            || chip.allowed_surfaces.is_empty()
        {
            violations.push(ErrorRecoveryCopyViolation::ChipIncomplete);
        }
        if !is_locale_neutral(&chip.chip_id) || !is_locale_neutral(&chip.machine_token) {
            violations.push(ErrorRecoveryCopyViolation::ChipTokenNotLocaleNeutral);
        }
        if !seen_ids.insert(chip.chip_id.as_str())
            || !seen_tokens.insert(chip.machine_token.as_str())
        {
            violations.push(ErrorRecoveryCopyViolation::DuplicateChip);
        }
        if !chip.grounded
            || uses_forbidden_tone(&chip.canonical_label)
            || uses_forbidden_tone(&chip.reserved_meaning)
        {
            violations.push(ErrorRecoveryCopyViolation::ChipNotGrounded);
        }
    }
}

fn validate_blocks(
    packet: &ErrorRecoveryCopyCatalog,
    violations: &mut Vec<ErrorRecoveryCopyViolation>,
) {
    let chip_ids: BTreeSet<&str> = packet.chips.iter().map(|c| c.chip_id.as_str()).collect();
    let mut seen_blocks: BTreeSet<&str> = BTreeSet::new();

    for block in &packet.blocks {
        if block.consumer_surfaces.is_empty() {
            violations.push(ErrorRecoveryCopyViolation::BlockIncomplete);
        }
        if !is_locale_neutral(&block.block_id) {
            violations.push(ErrorRecoveryCopyViolation::BlockIdNotLocaleNeutral);
        }
        if !seen_blocks.insert(block.block_id.as_str()) {
            violations.push(ErrorRecoveryCopyViolation::DuplicateBlock);
        }

        validate_block_parts(block, violations);
        validate_block_lines(block, &chip_ids, violations);
        validate_next_action(block, violations);
        validate_block_chip_surfaces(block, packet, violations);

        for line in block.lines() {
            if uses_forbidden_tone(&line.reference_template) {
                violations.push(ErrorRecoveryCopyViolation::BlockUsesGenericOrPlayfulTone);
            }
        }
        if uses_forbidden_tone(&block.next_action.label) {
            violations.push(ErrorRecoveryCopyViolation::BlockUsesGenericOrPlayfulTone);
        }
    }
}

fn validate_block_parts(block: &RecoveryBlock, violations: &mut Vec<ErrorRecoveryCopyViolation>) {
    // Each of the four explicit parts must be present and non-empty: a surface can
    // never stop at generic failure text.
    if block.what_failed.reference_template.trim().is_empty()
        || block.why_likely.reference_template.trim().is_empty()
        || block.what_still_works.reference_template.trim().is_empty()
        || block.next_action.label.trim().is_empty()
    {
        violations.push(ErrorRecoveryCopyViolation::RecoveryBlockMissingPart);
    }

    if block.what_failed.role != CopyRole::WhatFailed
        || block.why_likely.role != CopyRole::WhyLikely
        || block.what_still_works.role != CopyRole::WhatStillWorks
    {
        violations.push(ErrorRecoveryCopyViolation::CopyLineRoleMismatch);
    }

    // Recovery messaging always says what still works — never "nothing".
    let still = block.what_still_works.reference_template.to_lowercase();
    if block.what_still_works.reference_template.trim().is_empty()
        || still.contains("nothing")
        || still.contains("no features")
    {
        violations.push(ErrorRecoveryCopyViolation::WhatStillWorksMissing);
    }
}

fn validate_block_lines(
    block: &RecoveryBlock,
    chip_ids: &BTreeSet<&str>,
    violations: &mut Vec<ErrorRecoveryCopyViolation>,
) {
    for line in block.lines() {
        for chip_id in &line.chip_refs {
            if !chip_ids.contains(chip_id.as_str()) {
                violations.push(ErrorRecoveryCopyViolation::ChipRefUnresolved);
            }
        }
        validate_variables(&line.variables, violations);
        validate_template(
            &line.reference_template,
            &line.chip_refs,
            &line.variables,
            violations,
        );
    }

    // The block's declared reason_chips must equal the union of its lines' refs —
    // no phantom chip and no chip the lines actually use left undeclared.
    let declared: BTreeSet<&str> = block.reason_chips.iter().map(String::as_str).collect();
    let used: BTreeSet<&str> = block
        .lines()
        .iter()
        .flat_map(|line| line.chip_refs.iter().map(String::as_str))
        .collect();
    if declared != used {
        violations.push(ErrorRecoveryCopyViolation::ChipRefUnresolved);
    }
}

fn validate_next_action(block: &RecoveryBlock, violations: &mut Vec<ErrorRecoveryCopyViolation>) {
    let action = &block.next_action;
    if !is_locale_neutral(&action.action_id) {
        violations.push(ErrorRecoveryCopyViolation::BlockIdNotLocaleNeutral);
    }
    if !is_verb_first(&action.label) {
        violations.push(ErrorRecoveryCopyViolation::NextActionNotVerbFirst);
    }
    validate_variables(&action.variables, violations);
    // The label may carry vars but never chips.
    validate_template(&action.label, &[], &action.variables, violations);

    let link = &action.recovery_link;
    if link.label.trim().is_empty()
        || link.target_ref.trim().is_empty()
        || link.link_id.trim().is_empty()
    {
        violations.push(ErrorRecoveryCopyViolation::NextActionMissingRecoveryLink);
    }
    if !is_locale_neutral(&link.link_id) || !is_locale_neutral(&link.target_ref) {
        violations.push(ErrorRecoveryCopyViolation::LinkTokenNotLocaleNeutral);
    }
    if !link.offline_available {
        violations.push(ErrorRecoveryCopyViolation::RecoveryLinkNotOfflineResolvable);
    }
}

fn validate_block_chip_surfaces(
    block: &RecoveryBlock,
    packet: &ErrorRecoveryCopyCatalog,
    violations: &mut Vec<ErrorRecoveryCopyViolation>,
) {
    for chip_id in &block.reason_chips {
        if let Some(chip) = packet.chip(chip_id) {
            // The chip must be allowed on every surface the block renders on.
            let allowed: BTreeSet<RecoveryConsumerSurface> =
                chip.allowed_surfaces.iter().copied().collect();
            if !block
                .consumer_surfaces
                .iter()
                .all(|surface| allowed.contains(surface))
            {
                violations.push(ErrorRecoveryCopyViolation::ChipUsedOnDisallowedSurface);
            }
        }
    }
}

fn validate_variables(
    variables: &[CopyVariable],
    violations: &mut Vec<ErrorRecoveryCopyViolation>,
) {
    for variable in variables {
        if !is_locale_neutral(&variable.name) {
            violations.push(ErrorRecoveryCopyViolation::VariableNameNotLocaleNeutral);
        }
    }
}

fn validate_template(
    template: &str,
    chip_refs: &[String],
    variables: &[CopyVariable],
    violations: &mut Vec<ErrorRecoveryCopyViolation>,
) {
    let declared_chips: BTreeSet<&str> = chip_refs.iter().map(String::as_str).collect();
    let declared_vars: BTreeSet<&str> = variables.iter().map(|v| v.name.as_str()).collect();
    let mut used_chips: BTreeSet<String> = BTreeSet::new();
    let mut used_vars: BTreeSet<String> = BTreeSet::new();

    for segment in parse_template(template) {
        match segment {
            TemplateSegment::Chip(chip_id) => {
                if !declared_chips.contains(chip_id.as_str()) {
                    violations.push(ErrorRecoveryCopyViolation::TemplatePlaceholderUnresolved);
                }
                used_chips.insert(chip_id);
            }
            TemplateSegment::Var(name) => {
                if !declared_vars.contains(name.as_str()) {
                    violations.push(ErrorRecoveryCopyViolation::TemplatePlaceholderUnresolved);
                }
                used_vars.insert(name);
            }
            TemplateSegment::Unknown(_) => {
                violations.push(ErrorRecoveryCopyViolation::TemplatePlaceholderUnresolved);
            }
            TemplateSegment::Text(_) => {}
        }
    }

    let unused_chips = declared_chips.iter().any(|c| !used_chips.contains(*c));
    let unused_vars = declared_vars.iter().any(|v| !used_vars.contains(*v));
    if unused_chips || unused_vars {
        violations.push(ErrorRecoveryCopyViolation::DeclaredTokenUnused);
    }
}

fn validate_coverage(
    packet: &ErrorRecoveryCopyCatalog,
    violations: &mut Vec<ErrorRecoveryCopyViolation>,
) {
    let domains: BTreeSet<FailureDomain> = packet.blocks.iter().map(|b| b.failure_domain).collect();
    let severities: BTreeSet<RecoverySeverity> = packet.blocks.iter().map(|b| b.severity).collect();
    let surfaces: BTreeSet<RecoveryConsumerSurface> = packet
        .blocks
        .iter()
        .flat_map(|b| b.consumer_surfaces.iter().copied())
        .collect();

    let domains_covered = FailureDomain::ALL.iter().all(|d| domains.contains(d));
    let severities_covered = RecoverySeverity::ALL.iter().all(|s| severities.contains(s));
    let surfaces_covered = RecoveryConsumerSurface::ALL
        .iter()
        .all(|s| surfaces.contains(s));
    if !domains_covered || !severities_covered || !surfaces_covered {
        violations.push(ErrorRecoveryCopyViolation::CoverageGap);
    }

    // Every degraded state must have a chip.
    let states: BTreeSet<DegradedState> = packet.chips.iter().map(|c| c.state).collect();
    if !DegradedState::ALL.iter().all(|s| states.contains(s)) {
        violations.push(ErrorRecoveryCopyViolation::DegradedStateNotCovered);
    }
}

fn validate_shared_reuse(
    packet: &ErrorRecoveryCopyCatalog,
    violations: &mut Vec<ErrorRecoveryCopyViolation>,
) {
    if packet.shared_reuse_chip_ids.is_empty() {
        violations.push(ErrorRecoveryCopyViolation::SharedChipReuseInsufficient);
        return;
    }
    let reuse = packet.cross_surface_reuse();
    for chip_id in &packet.shared_reuse_chip_ids {
        let spans = reuse.get(chip_id).map(BTreeSet::len).unwrap_or(0);
        if spans < SHARED_CHIP_MIN_REUSE_SURFACES {
            violations.push(ErrorRecoveryCopyViolation::SharedChipReuseInsufficient);
        }
    }
}

fn validate_trust_review(
    packet: &ErrorRecoveryCopyCatalog,
    violations: &mut Vec<ErrorRecoveryCopyViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.blocks_explain_failure_cause_remaining_and_next_action,
        review.recovery_messaging_states_what_still_works_and_how_to_proceed,
        review.degraded_state_chips_reused_not_reinvented_per_surface,
        review.chips_use_grounded_cause_language_not_euphemism,
        review.next_action_labels_are_verb_first_with_recovery_link,
        review.machine_tokens_and_ids_stay_locale_neutral,
        review.human_prose_localizes_around_tokens,
        review.support_export_reconstructs_in_product_explanation,
        review.one_catalog_not_parallel_recovery_islands,
        review.recovery_links_resolve_offline,
    ] {
        if !ok {
            violations.push(ErrorRecoveryCopyViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &ErrorRecoveryCopyCatalog,
    violations: &mut Vec<ErrorRecoveryCopyViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.dynamic_banners_resolve_through_catalog,
        projection.inline_blockers_resolve_through_catalog,
        projection.project_doctor_reuses_block_identities,
        projection.cli_help_summaries_show_same_copy,
        projection.support_export_uses_catalog_blocks,
        projection.screenshot_captions_reuse_block_copy,
        projection.screen_reader_reuses_block_identities,
    ] {
        if !ok {
            violations.push(ErrorRecoveryCopyViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &ErrorRecoveryCopyCatalog,
    violations: &mut Vec<ErrorRecoveryCopyViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(ErrorRecoveryCopyViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &ErrorRecoveryCopyCatalog,
    violations: &mut Vec<ErrorRecoveryCopyViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.mirror_offline_packet_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.mirror_offline_parity_required
    {
        violations.push(ErrorRecoveryCopyViolation::ReleasePostureIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Rewrites a reference template into a pseudo-localized form by wrapping each
/// human text run in locale markers while leaving every `{chip:...}` and
/// `{var:...}` placeholder — the locale-neutral machine identity — untouched.
///
/// This is the engine behind the localized overlay: it proves that human prose can
/// localize freely without ever moving a block id, chip id, or placeholder.
pub fn pseudo_localize_template(template: &str) -> String {
    let mut out = String::new();
    for segment in parse_template(template) {
        match segment {
            TemplateSegment::Text(text) => {
                let trimmed = text.trim();
                if trimmed.is_empty() {
                    out.push_str(&text);
                } else {
                    let leading = &text[..text.len() - text.trim_start().len()];
                    let trailing = &text[text.trim_end().len()..];
                    out.push_str(leading);
                    out.push('\u{27e6}');
                    out.push_str(trimmed);
                    out.push('\u{27e7}');
                    out.push_str(trailing);
                }
            }
            TemplateSegment::Chip(id) => out.push_str(&format!("{{chip:{id}}}")),
            TemplateSegment::Var(name) => out.push_str(&format!("{{var:{name}}}")),
            TemplateSegment::Unknown(raw) => out.push_str(&raw),
        }
    }
    out
}
