//! Stable safety-critical string catalog and glossary-linked controlled terms.
//!
//! This module materializes the canonical, export-safe inventory of the
//! safety-critical wording the shell renders for trust, policy, recovery, AI, and
//! runtime surfaces. It is the concrete catalog that consumes the frozen content
//! governance matrix: where the matrix locks *which* wording objects are governed,
//! this catalog locks the *actual* objects — each [`SafetyCriticalMessage`] carries
//! a stable, locale-neutral message id, an [`MessageAudience`], a
//! [`MessageSeverity`], named [`MessageVariable`]s with declared semantics,
//! [`TruncationGuidance`], and the [`ControlledTerm`] ids it embeds.
//!
//! Controlled terms are first-class [`ControlledTerm`] objects with a reserved
//! meaning, a locale-neutral machine token, and the surfaces they may appear on.
//! Messages never inline a protected term: a reference template uses a
//! `{term:<term_id>}` placeholder that resolves against the glossary, and a
//! `{var:<name>}` placeholder that resolves against the declared variables. So the
//! catalog — not a scattered literal string — is the source of truth for protected
//! terminology, and the same controlled term resolves identically across trust
//! prompts, degraded-state banners, Project Doctor findings, AI review flows,
//! execution-context sheets, and support/export headings.
//!
//! Machine-facing identity stays locale-neutral while human prose localizes safely
//! around it. Message ids, term ids, machine tokens, and variable names are
//! lowercase ascii (`[a-z0-9_.]`); only the canonical labels, reserved meanings,
//! and reference templates carry human prose. A localized overlay rewrites the
//! prose but never the ids or the `{term:...}` / `{var:...}` placeholders, so a
//! translation can never fork the meaning of a lifecycle/trust/policy/runtime
//! state.
//!
//! The boundary schema is
//! [`schemas/content/m5-safety-critical-strings.schema.json`](../../../../schemas/content/m5-safety-critical-strings.schema.json).
//! The contract doc is
//! [`docs/content/m5/m5_safety_critical_string_catalog.md`](../../../../docs/content/m5/m5_safety_critical_string_catalog.md).
//! The protected fixture directory is
//! [`fixtures/content/m5-safety-critical-strings/`](../../../../fixtures/content/m5-safety-critical-strings/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_safety_critical_string_catalog, seeded_safety_critical_string_catalog_localized,
    seeded_safety_critical_string_catalog_offline_mirror, SAFETY_CRITICAL_STRING_CATALOG_ID,
};

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`SafetyCriticalStringCatalog`].
pub const SAFETY_CRITICAL_STRING_CATALOG_RECORD_KIND: &str = "m5_safety_critical_string_catalog";

/// Schema version for safety-critical string catalog records.
pub const SAFETY_CRITICAL_STRING_CATALOG_SCHEMA_VERSION: u32 = 1;

/// Minimum number of distinct surface families a shared reuse term must appear on.
pub const SHARED_TERM_MIN_REUSE_SURFACES: usize = 3;

/// Repo-relative path of the boundary schema.
pub const SAFETY_CRITICAL_STRING_CATALOG_SCHEMA_REF: &str =
    "schemas/content/m5-safety-critical-strings.schema.json";

/// Repo-relative path of the catalog contract doc.
pub const SAFETY_CRITICAL_STRING_CATALOG_DOC_REF: &str =
    "docs/content/m5/m5_safety_critical_string_catalog.md";

/// Repo-relative path of the frozen UI copy contract (action labels, error copy).
pub const CATALOG_UI_COPY_CONTRACT_REF: &str = "docs/copy/ui_copy_contract.md";

/// Repo-relative path of the frozen naming and state-label contract.
pub const CATALOG_NAMING_LABEL_CONTRACT_REF: &str = "docs/copy/naming_and_state_label_contract.md";

/// Repo-relative path of the frozen count/scope/freshness grammar contract.
pub const CATALOG_COUNT_SCOPE_GRAMMAR_REF: &str = "docs/copy/count_scope_freshness_grammar.md";

/// Repo-relative path of the frozen translation-safe content-ops contract.
pub const CATALOG_CONTENT_OPS_CONTRACT_REF: &str =
    "docs/copy/translation_safe_content_ops_contract.md";

/// Repo-relative path of the frozen AI copy guardrails contract.
pub const CATALOG_AI_COPY_GUARDRAILS_CONTRACT_REF: &str = "docs/ai/ai_copy_guardrails_contract.md";

/// Repo-relative path of the controlled glossary register.
pub const CATALOG_CONTROLLED_GLOSSARY_REF: &str = "artifacts/copy/controlled_glossary.yaml";

/// Repo-relative path of the product truth vocabulary register.
pub const CATALOG_PRODUCT_TRUTH_VOCABULARY_REF: &str =
    "artifacts/governance/product_truth_vocabulary.yaml";

/// Repo-relative path of the upstream content-wording governance matrix schema.
pub const CATALOG_WORDING_MATRIX_SCHEMA_REF: &str =
    "schemas/content/freeze-the-m5-content-design-controlled-vocabulary-content-ops-and-commercial-boundary-wording-matrix.schema.json";

/// Repo-relative path of the upstream content-wording governance matrix doc.
pub const CATALOG_WORDING_MATRIX_DOC_REF: &str =
    "docs/content/m5/freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix.md";

/// Repo-relative path of the protected fixture directory.
pub const SAFETY_CRITICAL_STRING_CATALOG_FIXTURE_DIR: &str =
    "fixtures/content/m5-safety-critical-strings";

/// Repo-relative path of the checked support-export artifact.
pub const SAFETY_CRITICAL_STRING_CATALOG_ARTIFACT_REF: &str =
    "artifacts/content/m5-terminology-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const SAFETY_CRITICAL_STRING_CATALOG_SUMMARY_REF: &str =
    "artifacts/content/m5-terminology-proof/m5_safety_critical_string_catalog.md";

/// Audience a safety-critical message addresses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageAudience {
    /// The person using the product.
    EndUser,
    /// The operator/administrator managing a deployment.
    Operator,
    /// The developer working in the project.
    Developer,
    /// The support or success engineer reading an exported packet.
    Support,
    /// A screen-reader / narrated or durable-attention surface.
    ScreenReader,
}

impl MessageAudience {
    /// Every audience, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::EndUser,
        Self::Operator,
        Self::Developer,
        Self::Support,
        Self::ScreenReader,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EndUser => "end_user",
            Self::Operator => "operator",
            Self::Developer => "developer",
            Self::Support => "support",
            Self::ScreenReader => "screen_reader",
        }
    }
}

/// Severity of a safety-critical message, ordered low to high.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageSeverity {
    /// Neutral, informational status.
    Info,
    /// A notice worth reading but not a hazard.
    Notice,
    /// A caution: proceed carefully.
    Caution,
    /// A warning about a degraded or risky state.
    Warning,
    /// A critical condition demanding attention.
    Critical,
    /// A blocking condition: the action cannot proceed.
    Blocking,
}

impl MessageSeverity {
    /// Every severity, low to high.
    pub const ALL: [Self; 6] = [
        Self::Info,
        Self::Notice,
        Self::Caution,
        Self::Warning,
        Self::Critical,
        Self::Blocking,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Notice => "notice",
            Self::Caution => "caution",
            Self::Warning => "warning",
            Self::Critical => "critical",
            Self::Blocking => "blocking",
        }
    }

    /// Whether this severity describes a failure/degradation rather than status.
    pub const fn is_hazardous(self) -> bool {
        matches!(self, Self::Warning | Self::Critical | Self::Blocking)
    }
}

/// Surface family a safety-critical message renders on.
///
/// These are the concrete trust/policy/recovery/AI/runtime surfaces named by the
/// reuse contract; the same controlled term must resolve identically across them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageSurfaceFamily {
    /// A trust prompt (verify-this-source gate).
    TrustPrompt,
    /// A degraded-state banner.
    DegradedStateBanner,
    /// A Project Doctor finding.
    ProjectDoctorFinding,
    /// An AI review / answer flow line.
    AiReviewFlow,
    /// An execution-context sheet.
    ExecutionContextSheet,
    /// A support / export heading.
    SupportExportHeading,
    /// A recovery action block.
    RecoveryActionBlock,
    /// A runtime status line.
    RuntimeStatus,
}

impl MessageSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::TrustPrompt,
        Self::DegradedStateBanner,
        Self::ProjectDoctorFinding,
        Self::AiReviewFlow,
        Self::ExecutionContextSheet,
        Self::SupportExportHeading,
        Self::RecoveryActionBlock,
        Self::RuntimeStatus,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustPrompt => "trust_prompt",
            Self::DegradedStateBanner => "degraded_state_banner",
            Self::ProjectDoctorFinding => "project_doctor_finding",
            Self::AiReviewFlow => "ai_review_flow",
            Self::ExecutionContextSheet => "execution_context_sheet",
            Self::SupportExportHeading => "support_export_heading",
            Self::RecoveryActionBlock => "recovery_action_block",
            Self::RuntimeStatus => "runtime_status",
        }
    }
}

/// The wording-object class a message belongs to.
///
/// Mirrors the governed object kinds in the upstream wording matrix that actually
/// produce rendered copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageClass {
    /// A safety-critical UI string asserting a trust/policy/runtime state.
    SafetyCriticalString,
    /// A four-part error/recovery block.
    ErrorRecoveryBlock,
    /// A verb-first action label.
    ActionLabel,
    /// An AI copy line governed by the certainty/autonomy guardrails.
    AiCopyLine,
    /// A count/scope/freshness phrase.
    CountScopePhrase,
}

impl MessageClass {
    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafetyCriticalString => "safety_critical_string",
            Self::ErrorRecoveryBlock => "error_recovery_block",
            Self::ActionLabel => "action_label",
            Self::AiCopyLine => "ai_copy_line",
            Self::CountScopePhrase => "count_scope_phrase",
        }
    }
}

/// The controlled vocabulary a [`ControlledTerm`] belongs to.
///
/// Mirrors the canonical state vocabularies owned by the product truth vocabulary
/// and the controlled glossary so the catalog never mints a parallel synonym.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlledTermClass {
    /// Capability lifecycle / release-stage term.
    Lifecycle,
    /// Source / destination authority (trust) term.
    Trust,
    /// Policy / entitlement gating term.
    Policy,
    /// Compatibility term between a claim and the active target.
    Compatibility,
    /// Freshness term for the data behind a claim.
    Freshness,
    /// Client-scope term.
    ClientScope,
}

impl ControlledTermClass {
    /// Every term class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Lifecycle,
        Self::Trust,
        Self::Policy,
        Self::Compatibility,
        Self::Freshness,
        Self::ClientScope,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lifecycle => "lifecycle",
            Self::Trust => "trust",
            Self::Policy => "policy",
            Self::Compatibility => "compatibility",
            Self::Freshness => "freshness",
            Self::ClientScope => "client_scope",
        }
    }
}

/// Alias posture for a controlled term.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AliasPosture {
    /// No alias is permitted; the canonical label is the only label.
    NoAlias,
    /// A controlled alias is permitted and tracked.
    ControlledAlias,
    /// A prior label is deprecated and kept only for back-reference.
    DeprecatedAlias,
}

impl AliasPosture {
    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAlias => "no_alias",
            Self::ControlledAlias => "controlled_alias",
            Self::DeprecatedAlias => "deprecated_alias",
        }
    }
}

/// Role of a declared message variable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VariableRole {
    /// Resolves to a controlled term from the glossary.
    ControlledTerm,
    /// A numeric count.
    Count,
    /// A scope / omission-reason label.
    ScopeLabel,
    /// A human-entity name (project, source, capability).
    EntityName,
    /// A path or location.
    Location,
    /// A machine-facing code or identifier.
    Code,
    /// A duration or elapsed-time value.
    Duration,
}

impl VariableRole {
    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlledTerm => "controlled_term",
            Self::Count => "count",
            Self::ScopeLabel => "scope_label",
            Self::EntityName => "entity_name",
            Self::Location => "location",
            Self::Code => "code",
            Self::Duration => "duration",
        }
    }

    /// Whether values for this role are locale-neutral (codes, counts, durations).
    pub const fn is_locale_neutral_value(self) -> bool {
        matches!(self, Self::Count | Self::Code | Self::Duration)
    }
}

/// Strategy for safely truncating a message when space is constrained.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TruncationStrategy {
    /// Never truncate this message.
    NeverTruncate,
    /// Drop characters from the tail of free-text variables only.
    TruncateVariableTail,
    /// Elide the middle of free-text variables only.
    TruncateVariableMiddle,
    /// Drop lower-priority trailing clauses, keeping the state and next action.
    PriorityDropTrailingClause,
}

impl TruncationStrategy {
    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NeverTruncate => "never_truncate",
            Self::TruncateVariableTail => "truncate_variable_tail",
            Self::TruncateVariableMiddle => "truncate_variable_middle",
            Self::PriorityDropTrailingClause => "priority_drop_trailing_clause",
        }
    }
}

/// A first-class controlled term: one reserved meaning, one machine token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlledTerm {
    /// Stable, locale-neutral term id (e.g. `term.trust_required`).
    pub term_id: String,
    /// Controlled vocabulary this term belongs to.
    pub term_class: ControlledTermClass,
    /// Locale-neutral machine token (flag/code/JSON key) this term carries.
    pub machine_token: String,
    /// Canonical (default-locale) display label.
    pub canonical_label: String,
    /// The single reserved meaning this term holds everywhere.
    pub reserved_meaning: String,
    /// Alias posture for this term.
    pub alias_posture: AliasPosture,
    /// True when the term is never softened for tone.
    pub never_softened: bool,
    /// Surface families this term may appear on.
    pub allowed_surfaces: Vec<MessageSurfaceFamily>,
}

/// A declared variable slot in a safety-critical message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageVariable {
    /// Locale-neutral variable name used in the `{var:<name>}` placeholder.
    pub name: String,
    /// Semantic role of the variable.
    pub role: VariableRole,
    /// True when the variable's *value* is locale-neutral (codes, counts).
    pub locale_neutral_value: bool,
    /// True when the variable's rendering may be truncated.
    pub truncatable: bool,
    /// The controlled term this variable resolves to, when its role is
    /// [`VariableRole::ControlledTerm`].
    pub term_ref: Option<String>,
}

/// Guidance for truncating a message without losing safety-critical meaning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TruncationGuidance {
    /// The truncation strategy.
    pub strategy: TruncationStrategy,
    /// True when controlled terms are never dropped by truncation.
    pub controlled_terms_never_dropped: bool,
    /// Human note on what stays and what may be dropped.
    pub note: String,
}

/// A consumer surface that must project a catalog message identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CatalogConsumerSurface {
    /// Product UI.
    ProductUi,
    /// CLI / help text.
    CliHelp,
    /// Documentation.
    Docs,
    /// Support / export packet.
    SupportExport,
    /// Screen-reader / narrated / durable-attention surface.
    ScreenReader,
    /// AI explain / answer surface.
    AiSurface,
    /// Onboarding / tour surface.
    Onboarding,
    /// Help / About surface.
    HelpAbout,
}

impl CatalogConsumerSurface {
    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProductUi => "product_ui",
            Self::CliHelp => "cli_help",
            Self::Docs => "docs",
            Self::SupportExport => "support_export",
            Self::ScreenReader => "screen_reader",
            Self::AiSurface => "ai_surface",
            Self::Onboarding => "onboarding",
            Self::HelpAbout => "help_about",
        }
    }
}

/// One safety-critical message object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafetyCriticalMessage {
    /// Stable, locale-neutral message id (e.g. `msg.trust.unverified_source_prompt`).
    pub message_id: String,
    /// Wording-object class.
    pub message_class: MessageClass,
    /// Audience the message addresses.
    pub audience: MessageAudience,
    /// Severity of the message.
    pub severity: MessageSeverity,
    /// Surface family the message renders on.
    pub surface_family: MessageSurfaceFamily,
    /// Controlled-term ids this message embeds.
    pub glossary_term_refs: Vec<String>,
    /// Declared variable slots.
    pub variables: Vec<MessageVariable>,
    /// Non-authoritative default-locale reference rendering, built from
    /// `{term:<id>}` and `{var:<name>}` placeholders.
    pub reference_template: String,
    /// Truncation guidance.
    pub truncation: TruncationGuidance,
    /// Consumer surfaces that must project this message identity.
    pub consumer_surfaces: Vec<CatalogConsumerSurface>,
    /// Optional cross-reference to the action label that resolves this message.
    pub next_action_label_ref: Option<String>,
}

impl SafetyCriticalMessage {
    /// Returns the four reserved variable names an error/recovery block must carry.
    pub const fn recovery_variable_names() -> [&'static str; 4] {
        [
            "what_failed",
            "likely_cause",
            "what_still_works",
            "next_safe_action",
        ]
    }
}

/// Catalog-level trust and wording-honesty review block.
///
/// Every flag is a hard invariant; all must hold for the catalog to validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogTrustReview {
    /// Safety-critical messages carry stable, locale-neutral message ids.
    pub messages_have_stable_locale_neutral_ids: bool,
    /// Controlled terms resolve from the glossary, never inlined as literals.
    pub controlled_terms_resolved_not_inlined: bool,
    /// Every message carries audience and severity metadata.
    pub audience_and_severity_metadata_present: bool,
    /// Machine tokens, ids, and placeholders stay locale-neutral.
    pub machine_tokens_stay_locale_neutral: bool,
    /// Human prose localizes around the locale-neutral tokens.
    pub human_prose_localizes_around_tokens: bool,
    /// Error copy explains failure, remaining capability, and the next action.
    pub error_copy_explains_failure_remaining_and_next_action: bool,
    /// AI copy never overstates confidence or autonomy.
    pub ai_copy_never_overstates_confidence_or_autonomy: bool,
    /// Counts disclose freshness and scope honestly.
    pub counts_disclose_freshness_and_scope: bool,
    /// One catalog is the source of truth, not parallel string islands.
    pub one_catalog_not_parallel_string_islands: bool,
    /// Truncation never drops a controlled term.
    pub truncation_never_drops_controlled_terms: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogConsumerProjection {
    /// Product UI resolves copy through the catalog.
    pub product_ui_resolves_through_catalog: bool,
    /// CLI / help text shows controlled terms.
    pub cli_help_shows_controlled_terms: bool,
    /// Docs render the same controlled terms.
    pub docs_render_controlled_terms: bool,
    /// Support export uses catalog headings.
    pub support_export_uses_catalog_headings: bool,
    /// Screen-reader announcements reuse the same message identities.
    pub screen_reader_reuses_message_identities: bool,
    /// AI surfaces honor the copy guardrails.
    pub ai_surfaces_honor_copy_guardrails: bool,
    /// Onboarding uses controlled terms.
    pub onboarding_uses_controlled_terms: bool,
    /// Help / About shows the same state terms.
    pub help_about_shows_state_terms: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the catalog claim.
    pub auto_narrow_on_stale: bool,
}

/// Release and mirror/offline parity posture for the catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting mirror/offline packet.
    pub mirror_offline_packet_ref: String,
    /// True when support/export parity is required for every message.
    pub support_export_parity_required: bool,
    /// True when mirror/offline parity is required for every message.
    pub mirror_offline_parity_required: bool,
}

/// Constructor input for [`SafetyCriticalStringCatalog::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SafetyCriticalStringCatalogInput {
    /// Stable catalog id.
    pub catalog_id: String,
    /// Human-readable catalog label.
    pub catalog_label: String,
    /// Reference locale of the default templates (e.g. `en`).
    pub reference_locale: String,
    /// Controlled terms.
    pub terms: Vec<ControlledTerm>,
    /// Safety-critical messages.
    pub messages: Vec<SafetyCriticalMessage>,
    /// Shared reuse term ids that must span multiple surfaces.
    pub shared_reuse_term_ids: Vec<String>,
    /// Trust review block.
    pub trust_review: CatalogTrustReview,
    /// Consumer projection block.
    pub consumer_projection: CatalogConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: CatalogProofFreshness,
    /// Release posture.
    pub release_posture: CatalogReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe stable safety-critical string catalog packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafetyCriticalStringCatalog {
    /// Record kind; must equal [`SAFETY_CRITICAL_STRING_CATALOG_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`SAFETY_CRITICAL_STRING_CATALOG_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable catalog id.
    pub catalog_id: String,
    /// Human-readable catalog label.
    pub catalog_label: String,
    /// Reference locale of the default templates.
    pub reference_locale: String,
    /// Closed audience inventory (locale-neutral tokens).
    pub audience_inventory: Vec<String>,
    /// Closed severity inventory (locale-neutral tokens, low to high).
    pub severity_inventory: Vec<String>,
    /// Closed surface-family inventory (locale-neutral tokens).
    pub surface_inventory: Vec<String>,
    /// Closed term-class inventory (locale-neutral tokens).
    pub term_class_inventory: Vec<String>,
    /// Controlled terms.
    pub terms: Vec<ControlledTerm>,
    /// Safety-critical messages.
    pub messages: Vec<SafetyCriticalMessage>,
    /// Shared reuse term ids that must span multiple surfaces.
    pub shared_reuse_term_ids: Vec<String>,
    /// Trust review block.
    pub trust_review: CatalogTrustReview,
    /// Consumer projection block.
    pub consumer_projection: CatalogConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: CatalogProofFreshness,
    /// Release posture.
    pub release_posture: CatalogReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl SafetyCriticalStringCatalog {
    /// Builds a catalog packet from stable-lane input, filling the closed
    /// inventories from the canonical enum token lists.
    pub fn new(input: SafetyCriticalStringCatalogInput) -> Self {
        Self {
            record_kind: SAFETY_CRITICAL_STRING_CATALOG_RECORD_KIND.to_owned(),
            schema_version: SAFETY_CRITICAL_STRING_CATALOG_SCHEMA_VERSION,
            catalog_id: input.catalog_id,
            catalog_label: input.catalog_label,
            reference_locale: input.reference_locale,
            audience_inventory: token_list(&MessageAudience::ALL, MessageAudience::as_str),
            severity_inventory: token_list(&MessageSeverity::ALL, MessageSeverity::as_str),
            surface_inventory: token_list(&MessageSurfaceFamily::ALL, MessageSurfaceFamily::as_str),
            term_class_inventory: token_list(
                &ControlledTermClass::ALL,
                ControlledTermClass::as_str,
            ),
            terms: input.terms,
            messages: input.messages,
            shared_reuse_term_ids: input.shared_reuse_term_ids,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Resolves a message by id.
    pub fn message(&self, message_id: &str) -> Option<&SafetyCriticalMessage> {
        self.messages.iter().find(|m| m.message_id == message_id)
    }

    /// Resolves a controlled term by id.
    pub fn term(&self, term_id: &str) -> Option<&ControlledTerm> {
        self.terms.iter().find(|t| t.term_id == term_id)
    }

    /// Renders the default-locale reference text for a message, resolving each
    /// `{term:<id>}` to its canonical label and keeping each `{var:<name>}` as a
    /// named slot. Returns `None` if the message id is unknown.
    ///
    /// This is the catalog's consumer entry point: a surface never inlines a
    /// protected term, it asks the catalog to resolve the controlled terms.
    pub fn render_reference(&self, message_id: &str) -> Option<String> {
        let message = self.message(message_id)?;
        let mut out = String::new();
        for segment in parse_template(&message.reference_template) {
            match segment {
                TemplateSegment::Text(text) => out.push_str(&text),
                TemplateSegment::Term(term_id) => match self.term(&term_id) {
                    Some(term) => out.push_str(&term.canonical_label),
                    None => out.push_str(&format!("{{term:{term_id}}}")),
                },
                TemplateSegment::Var(name) => out.push_str(&format!("{{{name}}}")),
                TemplateSegment::Unknown(raw) => out.push_str(&raw),
            }
        }
        Some(out)
    }

    /// Maps each controlled-term id to the distinct surface families that embed it.
    ///
    /// This is the reuse proof: a shared term that appears on a trust prompt, a
    /// degraded banner, and a support heading shows the catalog is the one source
    /// the surfaces share.
    pub fn cross_surface_reuse(&self) -> BTreeMap<String, BTreeSet<&'static str>> {
        let mut reuse: BTreeMap<String, BTreeSet<&'static str>> = BTreeMap::new();
        for message in &self.messages {
            for term_id in &message.glossary_term_refs {
                reuse
                    .entry(term_id.clone())
                    .or_default()
                    .insert(message.surface_family.as_str());
            }
        }
        reuse
    }

    /// Validates every catalog invariant.
    pub fn validate(&self) -> Vec<SafetyCriticalStringCatalogViolation> {
        let mut violations = Vec::new();

        if self.record_kind != SAFETY_CRITICAL_STRING_CATALOG_RECORD_KIND {
            violations.push(SafetyCriticalStringCatalogViolation::WrongRecordKind);
        }
        if self.schema_version != SAFETY_CRITICAL_STRING_CATALOG_SCHEMA_VERSION {
            violations.push(SafetyCriticalStringCatalogViolation::WrongSchemaVersion);
        }
        if self.catalog_id.trim().is_empty()
            || self.catalog_label.trim().is_empty()
            || self.reference_locale.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(SafetyCriticalStringCatalogViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_inventories(self, &mut violations);
        validate_terms(self, &mut violations);
        validate_messages(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_shared_reuse(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("safety-critical string catalog serializes"),
        ) {
            violations.push(SafetyCriticalStringCatalogViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("safety-critical string catalog serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Safety-Critical String Catalog and Controlled Terms\n\n");
        out.push_str(&format!("- Catalog: `{}`\n", self.catalog_id));
        out.push_str(&format!("- Label: `{}`\n", self.catalog_label));
        out.push_str(&format!(
            "- Reference locale: `{}`\n",
            self.reference_locale
        ));
        out.push_str(&format!(
            "- Controlled terms: {} | Messages: {}\n",
            self.terms.len(),
            self.messages.len()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Controlled terms\n\n");
        for term in &self.terms {
            out.push_str(&format!(
                "- `{}` ({}, token `{}`): {}\n",
                term.term_id,
                term.term_class.as_str(),
                term.machine_token,
                term.reserved_meaning
            ));
        }

        out.push_str("\n## Messages\n\n");
        for message in &self.messages {
            out.push_str(&format!(
                "- `{}` [{} / {} / {}] on `{}`\n",
                message.message_id,
                message.message_class.as_str(),
                message.audience.as_str(),
                message.severity.as_str(),
                message.surface_family.as_str()
            ));
            if let Some(rendered) = self.render_reference(&message.message_id) {
                out.push_str(&format!("  - Reference: {rendered}\n"));
            }
            if !message.glossary_term_refs.is_empty() {
                out.push_str(&format!(
                    "  - Terms: {}\n",
                    message.glossary_term_refs.join(", ")
                ));
            }
        }

        out.push_str("\n## Cross-surface term reuse\n\n");
        for (term_id, surfaces) in self.cross_surface_reuse() {
            out.push_str(&format!(
                "- `{}`: {}\n",
                term_id,
                surfaces.into_iter().collect::<Vec<_>>().join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in catalog export.
#[derive(Debug)]
pub enum SafetyCriticalStringCatalogArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<SafetyCriticalStringCatalogViolation>),
}

impl fmt::Display for SafetyCriticalStringCatalogArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "safety-critical string catalog export parse failed: {error}"
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
                    "safety-critical string catalog export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for SafetyCriticalStringCatalogArtifactError {}

/// Validation failures emitted by [`SafetyCriticalStringCatalog::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SafetyCriticalStringCatalogViolation {
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
    /// A controlled term is incomplete.
    TermIncomplete,
    /// A term id or machine token is not locale-neutral.
    TermTokenNotLocaleNeutral,
    /// A controlled term is duplicated.
    DuplicateTerm,
    /// A controlled term is not marked never-softened.
    TermSoftenable,
    /// A message is incomplete.
    MessageIncomplete,
    /// A message id is not locale-neutral.
    MessageIdNotLocaleNeutral,
    /// A message id is duplicated.
    DuplicateMessage,
    /// A message variable name is not locale-neutral.
    VariableNameNotLocaleNeutral,
    /// A message variable is malformed (term ref present/absent for its role).
    VariableRoleMismatch,
    /// A message references a glossary term that does not resolve.
    GlossaryTermRefUnresolved,
    /// A message embeds a term on a surface the term does not allow.
    TermUsedOnDisallowedSurface,
    /// A template placeholder does not resolve to a declared term or variable.
    TemplatePlaceholderUnresolved,
    /// A declared term ref or variable is not used by the template.
    DeclaredTokenUnused,
    /// An error/recovery block omits a reserved recovery variable.
    RecoveryBlockMissingPart,
    /// An AI copy line overstates confidence or autonomy.
    AiCopyOverclaim,
    /// A count/scope phrase does not disclose count and freshness.
    CountScopeNotFreshnessHonest,
    /// A controlled-term variable is marked truncatable.
    ControlledTermVariableTruncatable,
    /// A message has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A next-action reference does not resolve to an action label.
    NextActionRefUnresolved,
    /// An audience, severity, or surface family is never used.
    CoverageGap,
    /// A shared reuse term does not span enough surfaces.
    SharedTermReuseInsufficient,
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

impl SafetyCriticalStringCatalogViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::InventoryDrift => "inventory_drift",
            Self::TermIncomplete => "term_incomplete",
            Self::TermTokenNotLocaleNeutral => "term_token_not_locale_neutral",
            Self::DuplicateTerm => "duplicate_term",
            Self::TermSoftenable => "term_softenable",
            Self::MessageIncomplete => "message_incomplete",
            Self::MessageIdNotLocaleNeutral => "message_id_not_locale_neutral",
            Self::DuplicateMessage => "duplicate_message",
            Self::VariableNameNotLocaleNeutral => "variable_name_not_locale_neutral",
            Self::VariableRoleMismatch => "variable_role_mismatch",
            Self::GlossaryTermRefUnresolved => "glossary_term_ref_unresolved",
            Self::TermUsedOnDisallowedSurface => "term_used_on_disallowed_surface",
            Self::TemplatePlaceholderUnresolved => "template_placeholder_unresolved",
            Self::DeclaredTokenUnused => "declared_token_unused",
            Self::RecoveryBlockMissingPart => "recovery_block_missing_part",
            Self::AiCopyOverclaim => "ai_copy_overclaim",
            Self::CountScopeNotFreshnessHonest => "count_scope_not_freshness_honest",
            Self::ControlledTermVariableTruncatable => "controlled_term_variable_truncatable",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::NextActionRefUnresolved => "next_action_ref_unresolved",
            Self::CoverageGap => "coverage_gap",
            Self::SharedTermReuseInsufficient => "shared_term_reuse_insufficient",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable catalog export.
pub fn current_stable_safety_critical_string_catalog_export(
) -> Result<SafetyCriticalStringCatalog, SafetyCriticalStringCatalogArtifactError> {
    let packet: SafetyCriticalStringCatalog = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/content/m5-terminology-proof/support_export.json"
    )))
    .map_err(SafetyCriticalStringCatalogArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(SafetyCriticalStringCatalogArtifactError::Validation(
            violations,
        ))
    }
}

/// A parsed segment of a reference template.
#[derive(Debug, Clone, PartialEq, Eq)]
enum TemplateSegment {
    /// Literal text run.
    Text(String),
    /// A `{term:<id>}` placeholder; carries the term id.
    Term(String),
    /// A `{var:<name>}` placeholder; carries the variable name.
    Var(String),
    /// An unrecognized `{...}` placeholder; carries the raw `{...}` text.
    Unknown(String),
}

/// Parses a reference template into ordered text/placeholder segments.
///
/// Placeholders are `{term:<id>}` or `{var:<name>}`. A `{...}` that is not one of
/// those, or an unbalanced brace, becomes an [`TemplateSegment::Unknown`] segment so
/// validation can reject it.
fn parse_template(template: &str) -> Vec<TemplateSegment> {
    let mut segments = Vec::new();
    let mut text = String::new();
    let mut chars = template.char_indices().peekable();
    while let Some((_, ch)) = chars.next() {
        if ch == '{' {
            // Flush any pending literal run before the placeholder.
            if !text.is_empty() {
                segments.push(TemplateSegment::Text(std::mem::take(&mut text)));
            }
            let mut inner = String::new();
            let mut closed = false;
            for (_, inner_ch) in chars.by_ref() {
                if inner_ch == '}' {
                    closed = true;
                    break;
                }
                inner.push(inner_ch);
            }
            if !closed {
                segments.push(TemplateSegment::Unknown(format!("{{{inner}")));
            } else if let Some(id) = inner.strip_prefix("term:") {
                segments.push(TemplateSegment::Term(id.to_owned()));
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

fn validate_source_contracts(
    packet: &SafetyCriticalStringCatalog,
    violations: &mut Vec<SafetyCriticalStringCatalogViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        SAFETY_CRITICAL_STRING_CATALOG_SCHEMA_REF,
        SAFETY_CRITICAL_STRING_CATALOG_DOC_REF,
        CATALOG_UI_COPY_CONTRACT_REF,
        CATALOG_NAMING_LABEL_CONTRACT_REF,
        CATALOG_COUNT_SCOPE_GRAMMAR_REF,
        CATALOG_CONTENT_OPS_CONTRACT_REF,
        CATALOG_AI_COPY_GUARDRAILS_CONTRACT_REF,
        CATALOG_CONTROLLED_GLOSSARY_REF,
        CATALOG_PRODUCT_TRUTH_VOCABULARY_REF,
        CATALOG_WORDING_MATRIX_SCHEMA_REF,
        CATALOG_WORDING_MATRIX_DOC_REF,
    ] {
        if !refs.contains(required) {
            violations.push(SafetyCriticalStringCatalogViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_inventories(
    packet: &SafetyCriticalStringCatalog,
    violations: &mut Vec<SafetyCriticalStringCatalogViolation>,
) {
    if packet.audience_inventory != token_list(&MessageAudience::ALL, MessageAudience::as_str)
        || packet.severity_inventory != token_list(&MessageSeverity::ALL, MessageSeverity::as_str)
        || packet.surface_inventory
            != token_list(&MessageSurfaceFamily::ALL, MessageSurfaceFamily::as_str)
        || packet.term_class_inventory
            != token_list(&ControlledTermClass::ALL, ControlledTermClass::as_str)
    {
        violations.push(SafetyCriticalStringCatalogViolation::InventoryDrift);
    }
}

fn validate_terms(
    packet: &SafetyCriticalStringCatalog,
    violations: &mut Vec<SafetyCriticalStringCatalogViolation>,
) {
    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    let mut seen_tokens: BTreeSet<&str> = BTreeSet::new();
    for term in &packet.terms {
        if term.canonical_label.trim().is_empty()
            || term.reserved_meaning.trim().is_empty()
            || term.allowed_surfaces.is_empty()
        {
            violations.push(SafetyCriticalStringCatalogViolation::TermIncomplete);
        }
        if !is_locale_neutral(&term.term_id) || !is_locale_neutral(&term.machine_token) {
            violations.push(SafetyCriticalStringCatalogViolation::TermTokenNotLocaleNeutral);
        }
        if !seen_ids.insert(term.term_id.as_str())
            || !seen_tokens.insert(term.machine_token.as_str())
        {
            violations.push(SafetyCriticalStringCatalogViolation::DuplicateTerm);
        }
        if !term.never_softened {
            violations.push(SafetyCriticalStringCatalogViolation::TermSoftenable);
        }
    }
}

fn validate_messages(
    packet: &SafetyCriticalStringCatalog,
    violations: &mut Vec<SafetyCriticalStringCatalogViolation>,
) {
    let term_ids: BTreeSet<&str> = packet.terms.iter().map(|t| t.term_id.as_str()).collect();
    let action_label_ids: BTreeSet<&str> = packet
        .messages
        .iter()
        .filter(|m| m.message_class == MessageClass::ActionLabel)
        .map(|m| m.message_id.as_str())
        .collect();
    let mut seen_messages: BTreeSet<&str> = BTreeSet::new();

    for message in &packet.messages {
        if message.reference_template.trim().is_empty()
            || message.consumer_surfaces.is_empty()
            || message.truncation.note.trim().is_empty()
        {
            violations.push(SafetyCriticalStringCatalogViolation::MessageIncomplete);
        }
        if message.consumer_surfaces.is_empty() {
            violations.push(SafetyCriticalStringCatalogViolation::ConsumerSurfacesMissing);
        }
        if !is_locale_neutral(&message.message_id) {
            violations.push(SafetyCriticalStringCatalogViolation::MessageIdNotLocaleNeutral);
        }
        if !seen_messages.insert(message.message_id.as_str()) {
            violations.push(SafetyCriticalStringCatalogViolation::DuplicateMessage);
        }
        if !message.truncation.controlled_terms_never_dropped {
            violations
                .push(SafetyCriticalStringCatalogViolation::ControlledTermVariableTruncatable);
        }

        validate_message_variables(message, &term_ids, violations);
        validate_message_glossary_refs(message, packet, violations);
        validate_message_template(message, violations);
        validate_message_class_rules(message, packet, violations);

        if let Some(next) = &message.next_action_label_ref {
            if !action_label_ids.contains(next.as_str()) {
                violations.push(SafetyCriticalStringCatalogViolation::NextActionRefUnresolved);
            }
        }
    }
}

fn validate_message_variables(
    message: &SafetyCriticalMessage,
    term_ids: &BTreeSet<&str>,
    violations: &mut Vec<SafetyCriticalStringCatalogViolation>,
) {
    for variable in &message.variables {
        if !is_locale_neutral(&variable.name) {
            violations.push(SafetyCriticalStringCatalogViolation::VariableNameNotLocaleNeutral);
        }
        match variable.role {
            VariableRole::ControlledTerm => {
                let resolves = variable
                    .term_ref
                    .as_deref()
                    .is_some_and(|term_id| term_ids.contains(term_id));
                if !resolves {
                    violations.push(SafetyCriticalStringCatalogViolation::VariableRoleMismatch);
                }
                if variable.truncatable {
                    violations.push(
                        SafetyCriticalStringCatalogViolation::ControlledTermVariableTruncatable,
                    );
                }
            }
            _ => {
                if variable.term_ref.is_some() {
                    violations.push(SafetyCriticalStringCatalogViolation::VariableRoleMismatch);
                }
            }
        }
    }
}

fn validate_message_glossary_refs(
    message: &SafetyCriticalMessage,
    packet: &SafetyCriticalStringCatalog,
    violations: &mut Vec<SafetyCriticalStringCatalogViolation>,
) {
    for term_id in &message.glossary_term_refs {
        match packet.term(term_id) {
            None => {
                violations.push(SafetyCriticalStringCatalogViolation::GlossaryTermRefUnresolved);
            }
            Some(term) => {
                if !term.allowed_surfaces.contains(&message.surface_family) {
                    violations
                        .push(SafetyCriticalStringCatalogViolation::TermUsedOnDisallowedSurface);
                }
            }
        }
    }
}

fn validate_message_template(
    message: &SafetyCriticalMessage,
    violations: &mut Vec<SafetyCriticalStringCatalogViolation>,
) {
    let declared_terms: BTreeSet<&str> = message
        .glossary_term_refs
        .iter()
        .map(String::as_str)
        .collect();
    let declared_vars: BTreeSet<&str> = message.variables.iter().map(|v| v.name.as_str()).collect();
    let mut used_terms: BTreeSet<String> = BTreeSet::new();
    let mut used_vars: BTreeSet<String> = BTreeSet::new();

    for segment in parse_template(&message.reference_template) {
        match segment {
            TemplateSegment::Term(term_id) => {
                if !declared_terms.contains(term_id.as_str()) {
                    violations
                        .push(SafetyCriticalStringCatalogViolation::TemplatePlaceholderUnresolved);
                }
                used_terms.insert(term_id);
            }
            TemplateSegment::Var(name) => {
                if !declared_vars.contains(name.as_str()) {
                    violations
                        .push(SafetyCriticalStringCatalogViolation::TemplatePlaceholderUnresolved);
                }
                used_vars.insert(name);
            }
            TemplateSegment::Unknown(_) => {
                violations
                    .push(SafetyCriticalStringCatalogViolation::TemplatePlaceholderUnresolved);
            }
            TemplateSegment::Text(_) => {}
        }
    }

    // Every declared controlled term and variable must actually be used by the
    // template — a declared-but-unused token would let copy drift from its data.
    let unused_terms = declared_terms.iter().any(|t| !used_terms.contains(*t));
    let unused_vars = declared_vars.iter().any(|v| !used_vars.contains(*v));
    if unused_terms || unused_vars {
        violations.push(SafetyCriticalStringCatalogViolation::DeclaredTokenUnused);
    }
}

fn validate_message_class_rules(
    message: &SafetyCriticalMessage,
    packet: &SafetyCriticalStringCatalog,
    violations: &mut Vec<SafetyCriticalStringCatalogViolation>,
) {
    match message.message_class {
        MessageClass::ErrorRecoveryBlock => {
            let var_names: BTreeSet<&str> =
                message.variables.iter().map(|v| v.name.as_str()).collect();
            for required in SafetyCriticalMessage::recovery_variable_names() {
                if !var_names.contains(required) {
                    violations.push(SafetyCriticalStringCatalogViolation::RecoveryBlockMissingPart);
                }
            }
        }
        MessageClass::AiCopyLine => {
            if template_overclaims(&message.reference_template) {
                violations.push(SafetyCriticalStringCatalogViolation::AiCopyOverclaim);
            }
        }
        MessageClass::CountScopePhrase => {
            let has_count = message
                .variables
                .iter()
                .any(|v| v.role == VariableRole::Count);
            let discloses_freshness = message.glossary_term_refs.iter().any(|term_id| {
                packet
                    .term(term_id)
                    .is_some_and(|t| t.term_class == ControlledTermClass::Freshness)
            });
            if !has_count || !discloses_freshness {
                violations.push(SafetyCriticalStringCatalogViolation::CountScopeNotFreshnessHonest);
            }
        }
        MessageClass::SafetyCriticalString | MessageClass::ActionLabel => {}
    }
}

/// Returns true when an AI copy template uses overclaiming certainty/autonomy
/// language that the AI copy guardrails forbid.
fn template_overclaims(template: &str) -> bool {
    let lower = template.to_lowercase();
    const FORBIDDEN: [&str; 8] = [
        "guarantee",
        "100%",
        "fully autonomous",
        "always correct",
        "never wrong",
        "cannot fail",
        "absolutely certain",
        "no review needed",
    ];
    FORBIDDEN.iter().any(|phrase| lower.contains(phrase))
}

fn validate_coverage(
    packet: &SafetyCriticalStringCatalog,
    violations: &mut Vec<SafetyCriticalStringCatalogViolation>,
) {
    let audiences: BTreeSet<MessageAudience> = packet.messages.iter().map(|m| m.audience).collect();
    let severities: BTreeSet<MessageSeverity> =
        packet.messages.iter().map(|m| m.severity).collect();
    let surfaces: BTreeSet<MessageSurfaceFamily> =
        packet.messages.iter().map(|m| m.surface_family).collect();

    let audiences_covered = MessageAudience::ALL.iter().all(|a| audiences.contains(a));
    let severities_covered = MessageSeverity::ALL.iter().all(|s| severities.contains(s));
    let surfaces_covered = MessageSurfaceFamily::ALL
        .iter()
        .all(|s| surfaces.contains(s));
    if !audiences_covered || !severities_covered || !surfaces_covered {
        violations.push(SafetyCriticalStringCatalogViolation::CoverageGap);
    }
}

fn validate_shared_reuse(
    packet: &SafetyCriticalStringCatalog,
    violations: &mut Vec<SafetyCriticalStringCatalogViolation>,
) {
    if packet.shared_reuse_term_ids.is_empty() {
        violations.push(SafetyCriticalStringCatalogViolation::SharedTermReuseInsufficient);
        return;
    }
    let reuse = packet.cross_surface_reuse();
    for term_id in &packet.shared_reuse_term_ids {
        let spans = reuse.get(term_id).map(BTreeSet::len).unwrap_or(0);
        if spans < SHARED_TERM_MIN_REUSE_SURFACES {
            violations.push(SafetyCriticalStringCatalogViolation::SharedTermReuseInsufficient);
        }
    }
}

fn validate_trust_review(
    packet: &SafetyCriticalStringCatalog,
    violations: &mut Vec<SafetyCriticalStringCatalogViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.messages_have_stable_locale_neutral_ids,
        review.controlled_terms_resolved_not_inlined,
        review.audience_and_severity_metadata_present,
        review.machine_tokens_stay_locale_neutral,
        review.human_prose_localizes_around_tokens,
        review.error_copy_explains_failure_remaining_and_next_action,
        review.ai_copy_never_overstates_confidence_or_autonomy,
        review.counts_disclose_freshness_and_scope,
        review.one_catalog_not_parallel_string_islands,
        review.truncation_never_drops_controlled_terms,
    ] {
        if !ok {
            violations.push(SafetyCriticalStringCatalogViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &SafetyCriticalStringCatalog,
    violations: &mut Vec<SafetyCriticalStringCatalogViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.product_ui_resolves_through_catalog,
        projection.cli_help_shows_controlled_terms,
        projection.docs_render_controlled_terms,
        projection.support_export_uses_catalog_headings,
        projection.screen_reader_reuses_message_identities,
        projection.ai_surfaces_honor_copy_guardrails,
        projection.onboarding_uses_controlled_terms,
        projection.help_about_shows_state_terms,
    ] {
        if !ok {
            violations.push(SafetyCriticalStringCatalogViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &SafetyCriticalStringCatalog,
    violations: &mut Vec<SafetyCriticalStringCatalogViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(SafetyCriticalStringCatalogViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &SafetyCriticalStringCatalog,
    violations: &mut Vec<SafetyCriticalStringCatalogViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.mirror_offline_packet_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.mirror_offline_parity_required
    {
        violations.push(SafetyCriticalStringCatalogViolation::ReleasePostureIncomplete);
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
/// human text run in locale markers while leaving every `{term:...}` and
/// `{var:...}` placeholder — the locale-neutral machine identity — untouched.
///
/// This is the engine behind the localized overlay: it proves that human prose can
/// localize freely without ever moving a message id, term id, or placeholder.
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
            TemplateSegment::Term(id) => out.push_str(&format!("{{term:{id}}}")),
            TemplateSegment::Var(name) => out.push_str(&format!("{{var:{name}}}")),
            TemplateSegment::Unknown(raw) => out.push_str(&raw),
        }
    }
    out
}
