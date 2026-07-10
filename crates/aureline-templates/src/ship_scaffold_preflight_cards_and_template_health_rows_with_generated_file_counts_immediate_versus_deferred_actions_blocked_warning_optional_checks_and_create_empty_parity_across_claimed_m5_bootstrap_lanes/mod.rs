//! Two reusable M5 scaffold / project-entry components — the scaffold preflight card and the
//! template health row — so a user can tell exactly what a starter will write, which checks are
//! current, which actions run immediately versus later, and how to recover before creation
//! begins: the preflight card names its target path and name, its generated file and folder
//! counts, its dependency / task / extension impact, the concrete side effect it discloses
//! (package install, dependency restore, remote provisioning, trust prompt, script execution, or
//! extension install), whether that action runs immediately or is deferred, and a named
//! checkpoint or delete-generated recovery path; the health row names its check name, its status,
//! its freshness / source, its `Blocker` / `Warning` / `Info` severity, an auto-fix or manual-fix
//! note, rerun / open-detail actions, and an explicit same-weight path to `Create empty` or
//! `Continue without starter`.
//!
//! Aureline's frozen scaffold-component matrix
//! ([`crate::freeze_the_m5_scaffold_template_card_starter_parameter_row_scaffold_preflight_card_template_health_row_generated_project_diff_card_and_scaffold_handoff_banner_component_matrix`])
//! names the scaffold preflight card and the template health row as two governed component
//! families and freezes their controlled vocabulary — the preflight check classes
//! (`tooling_present`, `dependency_availability`, `network_access`, `workspace_writable`,
//! `host_boundary`, `credential_scope`) and result states (`passed`, `warning`, `blocked`,
//! `skipped_optional`, `not_run`, `unknown`) a preflight card binds; the health signal classes
//! (`build_health`, `dependency_freshness`, `security_advisories`, `test_status`,
//! `maintenance_cadence`, `compatibility`) and freshness states (`fresh`, `aging`, `stale`,
//! `expired`, `never_checked`, `unavailable`) a health row binds; the one controlled disposition
//! vocabulary; the surface families; the deployment lines; the consumer surfaces; the
//! accessibility routes; the required labels; and the downgrade triggers. This module
//! *implements* that contract as two co-equal component vectors so a claimed M5 start-center,
//! scaffold-preflight, template-health, or CLI surface can project a preflight card and a health
//! row that keep the same truth.
//!
//! The module has two derived resolvers:
//!
//! 1. [`resolve_preflight_disclosure`] — takes a preflight card's frozen result state and derives
//!    its severity (clear, advisory, blocked prerequisite, optional skipped, or needs attention),
//!    whether it is a blocking prerequisite, and which notes the card must carry — so a blocked
//!    prerequisite can never read as an optional optimization and a not-run or unknown check can
//!    never read as passed.
//! 2. [`resolve_health_disclosure`] — takes a template health row's frozen freshness state and
//!    derives its freshness posture (current, aging, stale-or-expired, never checked, or
//!    unavailable), whether it is current, and which notes the row must carry — so a stale,
//!    expired, never-checked, or unavailable signal can never read as fresh.
//!
//! A single controls packet — [`ScaffoldPreflightCardTemplateHealthRowControlsPacket`] — binds one
//! vector of preflight cards and one vector of health rows to the same side-effect,
//! generated-impact, immediate-versus-deferred, severity, freshness, recovery, deep-link, and
//! non-visual accessibility vocabulary, so what a starter writes and how to recover stay explicit
//! across desktop, headless / export, and support consumers.
//!
//! The preflight check class ([`M5PreflightCheckClass`]), preflight result state
//! ([`M5PreflightResultState`]), health signal class ([`M5HealthSignalClass`]), health freshness
//! state ([`M5HealthFreshnessState`]), disposition ([`M5ScaffoldDisposition`]), surface family
//! ([`M5ScaffoldSurfaceFamily`]), deployment line ([`M5ScaffoldDeploymentLine`]), consumer surface
//! ([`M5ScaffoldConsumerSurface`]), accessibility route ([`M5ScaffoldAccessibilityRoute`]),
//! required label ([`M5ScaffoldRequiredLabel`]), and downgrade trigger
//! ([`M5ScaffoldDowngradeTrigger`]) are reused verbatim from the frozen matrix. This module mints
//! new vocabulary only for what that matrix left implicit about the two components themselves: the
//! acceptance-criteria side-effect kinds a preflight card discloses, the immediate-versus-deferred
//! action timing, the derived preflight severity, the `Blocker` / `Warning` / `Info` health
//! severity and fix kind the acceptance criteria pin, the derived health freshness posture, the
//! bounded preflight-card and health-row actions, and the deep-link kinds. No M5 bootstrap surface
//! invents a second preflight-card or health-row grammar.
//!
//! Raw file bodies, raw secret values, pasted local paths, repository URLs, credentials, and
//! secrets stay outside the export boundary; every note, deep-link reference, and component
//! identity is carried only as an opaque, export-safe representation.

#[cfg(test)]
mod tests;

// The preflight check classes and result states, the health signal classes and freshness states,
// the disposition vocabulary, and the surface / deployment / consumer / accessibility / label /
// downgrade vocabularies are frozen once, in the scaffold-component matrix. This lane reuses them
// verbatim so it never invents a parallel preflight-card or health-row vocabulary.
pub use crate::freeze_the_m5_scaffold_template_card_starter_parameter_row_scaffold_preflight_card_template_health_row_generated_project_diff_card_and_scaffold_handoff_banner_component_matrix::{
    M5HealthFreshnessState, M5HealthSignalClass, M5PreflightCheckClass, M5PreflightResultState,
    M5ScaffoldAccessibilityRoute, M5ScaffoldComponentFamily, M5ScaffoldConsumerSurface,
    M5ScaffoldDeploymentLine, M5ScaffoldDisposition, M5ScaffoldDowngradeTrigger,
    M5ScaffoldRequiredLabel, M5ScaffoldSurfaceFamily, M5_SCAFFOLD_COMPONENT_DOC_REF,
    M5_SCAFFOLD_COMPONENT_SCHEMA_REF, M5_SCAFFOLD_PREFLIGHT_CARD_SCHEMA_REF,
    M5_TEMPLATE_HEALTH_ROW_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by
/// [`ScaffoldPreflightCardTemplateHealthRowControlsPacket`].
pub const SCAFFOLD_READINESS_CONTROLS_RECORD_KIND: &str =
    "ship_scaffold_preflight_cards_and_template_health_rows_with_generated_file_counts_immediate_versus_deferred_actions_blocked_warning_optional_checks_and_create_empty_parity_across_claimed_m5_bootstrap_lanes";

/// Schema version for M5 scaffold-preflight-card / template-health-row control records.
pub const SCAFFOLD_READINESS_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const SCAFFOLD_READINESS_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-scaffold-preflight-card-template-health-row-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const SCAFFOLD_READINESS_CONTROLS_DOC_REF: &str =
    "docs/templates/m5_scaffold_preflight_card_template_health_row_controls.md";

/// Repo-relative path of the protected fixture directory.
pub const SCAFFOLD_READINESS_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-scaffold-preflight-card-template-health-row-controls";

/// Repo-relative path of the checked support-export artifact.
pub const SCAFFOLD_READINESS_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-scaffold-preflight-card-template-health-row-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const SCAFFOLD_READINESS_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-scaffold-preflight-card-template-health-row-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const SCAFFOLD_READINESS_CONTROLS_REPORT_REF: &str =
    "artifacts/design/m5-scaffold-preflight-card-template-health-row.md";

// ---- shared deep-link vocabulary ----------------------------------------

/// The kind of stable deep link a scaffold-readiness component binds its next step against, so a
/// preflight card or health row never routes through an ephemeral overlay — every next step is a
/// stable template manifest, starter-registry entry, docs, or policy reference the user can
/// reopen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkKind {
    /// A stable template-manifest reference.
    TemplateManifest,
    /// A stable starter-registry entry reference.
    StarterRegistryEntry,
    /// A stable docs anchor.
    DocsAnchor,
    /// A stable policy reference.
    PolicyReference,
    /// No deep link is bound (the component names that it routes nowhere).
    NoDeepLink,
}

impl DeepLinkKind {
    /// Every deep-link kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::TemplateManifest,
        Self::StarterRegistryEntry,
        Self::DocsAnchor,
        Self::PolicyReference,
        Self::NoDeepLink,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TemplateManifest => "template_manifest",
            Self::StarterRegistryEntry => "starter_registry_entry",
            Self::DocsAnchor => "docs_anchor",
            Self::PolicyReference => "policy_reference",
            Self::NoDeepLink => "no_deep_link",
        }
    }

    /// True when this kind names a resolvable deep-link target.
    pub const fn is_resolvable(self) -> bool {
        !matches!(self, Self::NoDeepLink)
    }
}

// ---- scaffold-preflight-card vocabulary ---------------------------------

/// The concrete side effect a scaffold preflight card discloses. These are the exact
/// acceptance-criteria labels so a generic Create never hides a package install, dependency
/// restore, remote provisioning, trust prompt, script execution, or extension install; a check
/// with no write side effect names `no_side_effect` rather than leaving it implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreflightSideEffectKind {
    /// Installs packages.
    PackageInstall,
    /// Restores / downloads dependencies.
    DependencyRestore,
    /// Provisions a remote or managed resource.
    RemoteProvisioning,
    /// Prompts for trust before running.
    TrustPrompt,
    /// Runs a script or setup task.
    ScriptExecution,
    /// Installs an editor / workspace extension.
    ExtensionInstall,
    /// No write side effect (a pure check).
    NoSideEffect,
}

impl PreflightSideEffectKind {
    /// Every side-effect kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::PackageInstall,
        Self::DependencyRestore,
        Self::RemoteProvisioning,
        Self::TrustPrompt,
        Self::ScriptExecution,
        Self::ExtensionInstall,
        Self::NoSideEffect,
    ];

    /// The six real side-effect kinds the acceptance criteria pin — a generic Create must never
    /// hide any of these.
    pub const REAL: [Self; 6] = [
        Self::PackageInstall,
        Self::DependencyRestore,
        Self::RemoteProvisioning,
        Self::TrustPrompt,
        Self::ScriptExecution,
        Self::ExtensionInstall,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackageInstall => "package_install",
            Self::DependencyRestore => "dependency_restore",
            Self::RemoteProvisioning => "remote_provisioning",
            Self::TrustPrompt => "trust_prompt",
            Self::ScriptExecution => "script_execution",
            Self::ExtensionInstall => "extension_install",
            Self::NoSideEffect => "no_side_effect",
        }
    }

    /// True when the check carries a real write side effect that must be disclosed.
    pub const fn is_side_effecting(self) -> bool {
        !matches!(self, Self::NoSideEffect)
    }
}

/// Whether a scaffold preflight action runs immediately or is deferred for later, so a preflight
/// card never leaves the immediate-versus-deferred boundary implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreflightActionTiming {
    /// Aureline runs this action immediately on create.
    RunsImmediately,
    /// Aureline defers this action for later.
    DeferredForLater,
    /// Aureline requires explicit confirmation before running this action.
    RequiresConfirmation,
    /// The action is blocked until a prerequisite is resolved.
    BlockedUntilResolved,
    /// The action does not apply to this check.
    NotApplicable,
}

impl PreflightActionTiming {
    /// Every action timing, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RunsImmediately,
        Self::DeferredForLater,
        Self::RequiresConfirmation,
        Self::BlockedUntilResolved,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunsImmediately => "runs_immediately",
            Self::DeferredForLater => "deferred_for_later",
            Self::RequiresConfirmation => "requires_confirmation",
            Self::BlockedUntilResolved => "blocked_until_resolved",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True only when Aureline runs this action immediately on create.
    pub const fn is_immediate(self) -> bool {
        matches!(self, Self::RunsImmediately)
    }
}

/// Derived severity a scaffold preflight card may present.
///
/// This is the preflight honesty axis: the severity is derived from the frozen result state,
/// never asserted, so a blocked prerequisite can never present as an optional optimization and a
/// not-run or unknown check can never present as passed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreflightSeverity {
    /// Passed with nothing to resolve.
    Clear,
    /// Passed with an advisory warning.
    Advisory,
    /// A blocked prerequisite that must be resolved before create.
    BlockedPrerequisite,
    /// An optional check that was skipped.
    OptionalSkipped,
    /// A check that has not run or whose result is unknown and needs attention.
    NeedsAttention,
}

impl PreflightSeverity {
    /// Every severity, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Clear,
        Self::Advisory,
        Self::BlockedPrerequisite,
        Self::OptionalSkipped,
        Self::NeedsAttention,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Clear => "clear",
            Self::Advisory => "advisory",
            Self::BlockedPrerequisite => "blocked_prerequisite",
            Self::OptionalSkipped => "optional_skipped",
            Self::NeedsAttention => "needs_attention",
        }
    }

    /// True only when this severity is a blocked prerequisite.
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::BlockedPrerequisite)
    }
}

/// One keyboard-complete default action a scaffold preflight card offers, so a card never hides
/// its side-effect, impact, or recovery affordance behind a pointer-only gesture and never routes
/// creation through a generic Create. `ReviewSideEffects`, `ReviewGeneratedImpact`, and
/// `ReviewRecoveryPath` are always offered so the side effects, generated impact, and recovery
/// path are inspectable before any commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreflightCardAction {
    /// Review the concrete side effects the starter will run (always available).
    ReviewSideEffects,
    /// Review the generated file / folder counts and dependency / task / extension impact
    /// (always available).
    ReviewGeneratedImpact,
    /// Review the named checkpoint or delete-generated recovery path (always available).
    ReviewRecoveryPath,
    /// Run the immediate actions (never a generic Create that hides side effects).
    RunImmediateActions,
    /// Create empty instead, with no starter writes.
    CreateEmpty,
    /// Open the stable manifest / registry / docs / policy deep link.
    OpenDeepLink,
}

impl PreflightCardAction {
    /// Every preflight-card action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReviewSideEffects,
        Self::ReviewGeneratedImpact,
        Self::ReviewRecoveryPath,
        Self::RunImmediateActions,
        Self::CreateEmpty,
        Self::OpenDeepLink,
    ];

    /// The default actions every keyboard-complete preflight card must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::ReviewSideEffects,
        Self::ReviewGeneratedImpact,
        Self::ReviewRecoveryPath,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewSideEffects => "review_side_effects",
            Self::ReviewGeneratedImpact => "review_generated_impact",
            Self::ReviewRecoveryPath => "review_recovery_path",
            Self::RunImmediateActions => "run_immediate_actions",
            Self::CreateEmpty => "create_empty",
            Self::OpenDeepLink => "open_deep_link",
        }
    }
}

/// Disclosures a scaffold preflight card must carry, derived from the frozen result state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreflightCardDisclosure {
    /// The derived severity this card may present.
    pub severity: PreflightSeverity,
    /// Whether this card is a blocking prerequisite.
    pub is_blocking: bool,
    /// Whether the card must carry an explicit blocked-prerequisite note.
    pub needs_blocked_note: bool,
    /// Whether the card must carry an explicit advisory-warning note.
    pub needs_warning_note: bool,
    /// Whether the card must carry an explicit optional-skipped note.
    pub needs_skipped_note: bool,
    /// Whether the card must carry an explicit needs-attention note.
    pub needs_attention_note: bool,
}

/// Resolves the severity truth a scaffold preflight card may present.
///
/// A `passed` check is clear, a `warning` check is advisory, a `blocked` check is a blocked
/// prerequisite, a `skipped_optional` check is optional skipped, and a `not_run` or `unknown`
/// check needs attention — so a blocked prerequisite can never read as an optional optimization
/// and a not-run or unknown check can never read as passed.
pub fn resolve_preflight_disclosure(
    result_state: M5PreflightResultState,
) -> PreflightCardDisclosure {
    use M5PreflightResultState as State;
    use PreflightSeverity as Sev;

    let severity = match result_state {
        State::Passed => Sev::Clear,
        State::Warning => Sev::Advisory,
        State::Blocked => Sev::BlockedPrerequisite,
        State::SkippedOptional => Sev::OptionalSkipped,
        State::NotRun | State::Unknown => Sev::NeedsAttention,
    };

    PreflightCardDisclosure {
        severity,
        is_blocking: severity.is_blocking(),
        needs_blocked_note: matches!(severity, Sev::BlockedPrerequisite),
        needs_warning_note: matches!(severity, Sev::Advisory),
        needs_skipped_note: matches!(severity, Sev::OptionalSkipped),
        needs_attention_note: matches!(severity, Sev::NeedsAttention),
    }
}

/// A scaffold preflight card naming its target path and name, generated file and folder counts,
/// dependency / task / extension impact, the concrete side effect it discloses, whether that
/// action runs immediately or is deferred, a named checkpoint or delete-generated recovery path,
/// its derived severity, bounded review / create-empty actions, and a stable manifest / registry /
/// docs / policy deep link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldPreflightCard {
    /// Frozen component this control implements; must be `scaffold_preflight_card`.
    pub component: M5ScaffoldComponentFamily,
    /// Stable card id.
    pub card_id: String,
    /// Human-readable card name; required and non-empty.
    pub card_name: String,
    /// Preflight check class, reused from the frozen matrix.
    pub check_class: M5PreflightCheckClass,
    /// Preflight result state, reused from the frozen matrix.
    pub result_state: M5PreflightResultState,
    /// The concrete side effect this check discloses (the acceptance-criteria label).
    pub side_effect_kind: PreflightSideEffectKind,
    /// Whether the action runs immediately or is deferred.
    pub action_timing: PreflightActionTiming,
    /// Derived severity (must equal the resolved severity).
    pub derived_severity: PreflightSeverity,
    /// Whether the card claims a blocking prerequisite (must equal derived truth).
    pub claims_blocking_prerequisite: bool,
    /// Whether the card claims a real write side effect (must equal the side-effect truth).
    pub claims_side_effecting: bool,
    /// Whether the card claims the action runs immediately (must equal the timing truth).
    pub claims_immediate_action: bool,
    /// Side-effect note; required when the check carries a real write side effect.
    pub side_effect_note: String,
    /// Blocked-prerequisite note; required when the severity is a blocked prerequisite.
    pub blocked_note: String,
    /// Advisory-warning note; required when the severity is advisory.
    pub warning_note: String,
    /// Optional-skipped note; required when the check was skipped as optional.
    pub skipped_note: String,
    /// Needs-attention note; required when the check has not run or is unknown.
    pub attention_note: String,
    /// Target path label; always required so where the starter writes stays explicit.
    pub target_path_label: String,
    /// Target name label; always required.
    pub target_name_label: String,
    /// Count of files the starter will generate.
    pub generated_file_count: u32,
    /// Count of folders the starter will generate.
    pub generated_folder_count: u32,
    /// Generated-impact note; always required so the file / folder counts stay explicit.
    pub generated_impact_note: String,
    /// Dependency impact label; always required.
    pub dependency_impact_label: String,
    /// Task impact label; always required.
    pub task_impact_label: String,
    /// Extension impact label; always required.
    pub extension_impact_label: String,
    /// Immediate-action label; always required so what Aureline runs now stays explicit.
    pub immediate_action_label: String,
    /// Deferred-action label; always required so what Aureline defers stays explicit.
    pub deferred_action_label: String,
    /// Recovery-path label; always required (a named checkpoint or delete-generated path).
    pub recovery_path_label: String,
    /// Context note; always required so the card names what to check before committing.
    pub context_note: String,
    /// Kind of stable deep link this card binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include review-side-effects / review-impact /
    /// review-recovery).
    pub card_actions: Vec<PreflightCardAction>,
    /// Dispositions this card binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5ScaffoldDisposition>,
    /// Downgrade triggers this card can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5ScaffoldDowngradeTrigger>,
    /// Mandatory labels this card can show (must include the mandatory labels).
    pub required_labels: Vec<M5ScaffoldRequiredLabel>,
    /// Claimed M5 surface families that render this card.
    pub surface_families: Vec<M5ScaffoldSurfaceFamily>,
    /// Deployment lines this card keeps the same truth across.
    pub deployment_lines: Vec<M5ScaffoldDeploymentLine>,
    /// Non-visual accessibility routes this card offers.
    pub accessibility_routes: Vec<M5ScaffoldAccessibilityRoute>,
    /// Scaffold subsystems that consume this card's projection.
    pub consumer_surfaces: Vec<M5ScaffoldConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this card.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never hides a side effect behind a generic Create. MUST be `false`.
    pub hides_side_effect_behind_generic_create: bool,
    /// Hard invariant: never hides its generated file / folder impact or its recovery path. MUST
    /// be `false`.
    pub hides_generated_impact_or_recovery_path: bool,
    /// Hard invariant: never monopolizes the plain Create-empty / Continue-without-starter path.
    /// MUST be `false`.
    pub monopolizes_plain_create_without_starter_path: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl ScaffoldPreflightCard {
    /// Severity disclosures this card must carry, derived from the frozen result state.
    pub fn severity_disclosure(&self) -> PreflightCardDisclosure {
        resolve_preflight_disclosure(self.result_state)
    }

    /// Whether the card offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<PreflightCardAction> = self.card_actions.iter().copied().collect();
        PreflightCardAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the card offers a same-weight Create-empty path.
    fn offers_create_empty(&self) -> bool {
        self.card_actions
            .contains(&PreflightCardAction::CreateEmpty)
    }

    /// Whether the card declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        declares_mandatory_labels(&self.required_labels)
    }

    /// Whether the card offers a deep-link-opening action.
    fn offers_deep_link_action(&self) -> bool {
        self.card_actions
            .contains(&PreflightCardAction::OpenDeepLink)
    }
}

// ---- template-health-row vocabulary -------------------------------------

/// The severity a template health row reports. These are the exact acceptance-criteria labels so
/// a health row distinguishes a blocked prerequisite from a warning and from an optional
/// optimization rather than collapsing them into one status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthSeverity {
    /// A blocked prerequisite.
    Blocker,
    /// A warning.
    Warning,
    /// An informational / optional optimization.
    Info,
}

impl HealthSeverity {
    /// Every severity, in declaration order.
    pub const ALL: [Self; 3] = [Self::Blocker, Self::Warning, Self::Info];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocker => "blocker",
            Self::Warning => "warning",
            Self::Info => "info",
        }
    }

    /// True only when this severity is a blocked prerequisite.
    pub const fn is_blocking_prerequisite(self) -> bool {
        matches!(self, Self::Blocker)
    }
}

/// Whether a template health row offers an auto-fix, a manual fix, or no fix, so a row never
/// leaves whether the user can act implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthFixKind {
    /// An automatic fix is available.
    AutoFixAvailable,
    /// A manual fix is required.
    ManualFixRequired,
    /// No fix is needed.
    NoFixNeeded,
}

impl HealthFixKind {
    /// Every fix kind, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::AutoFixAvailable,
        Self::ManualFixRequired,
        Self::NoFixNeeded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AutoFixAvailable => "auto_fix_available",
            Self::ManualFixRequired => "manual_fix_required",
            Self::NoFixNeeded => "no_fix_needed",
        }
    }

    /// True when the row must carry an explicit fix note.
    pub const fn needs_fix_note(self) -> bool {
        !matches!(self, Self::NoFixNeeded)
    }
}

/// Derived freshness posture a template health row may present.
///
/// This is the health honesty axis: the posture is derived from the frozen freshness state, never
/// asserted, so a stale, expired, never-checked, or unavailable signal can never present as fresh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthFreshnessPosture {
    /// The signal is current.
    Current,
    /// The signal is aging.
    Aging,
    /// The signal is stale or expired.
    StaleOrExpired,
    /// The signal has never been checked.
    NeverChecked,
    /// The signal is unavailable.
    Unavailable,
}

impl HealthFreshnessPosture {
    /// Every freshness posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Current,
        Self::Aging,
        Self::StaleOrExpired,
        Self::NeverChecked,
        Self::Unavailable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Aging => "aging",
            Self::StaleOrExpired => "stale_or_expired",
            Self::NeverChecked => "never_checked",
            Self::Unavailable => "unavailable",
        }
    }

    /// True only when the signal is current.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }
}

/// One keyboard-complete default action a template health row offers, so a row never hides its
/// rerun or open-detail affordance behind a pointer-only gesture and always keeps a same-weight
/// path to Create empty or Continue without starter. `RerunCheck` and `OpenDetail` are always
/// offered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthRowAction {
    /// Rerun the health check (always available).
    RerunCheck,
    /// Open the detail for this health signal (always available).
    OpenDetail,
    /// Create empty instead, with no starter writes (a same-weight path).
    CreateEmpty,
    /// Continue without a starter (a same-weight path).
    ContinueWithoutStarter,
    /// Apply the available auto-fix.
    ApplyFix,
    /// Open the stable manifest / registry / docs / policy deep link.
    OpenDeepLink,
}

impl HealthRowAction {
    /// Every health-row action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RerunCheck,
        Self::OpenDetail,
        Self::CreateEmpty,
        Self::ContinueWithoutStarter,
        Self::ApplyFix,
        Self::OpenDeepLink,
    ];

    /// The default actions every keyboard-complete health row must offer.
    pub const MANDATORY: [Self; 2] = [Self::RerunCheck, Self::OpenDetail];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RerunCheck => "rerun_check",
            Self::OpenDetail => "open_detail",
            Self::CreateEmpty => "create_empty",
            Self::ContinueWithoutStarter => "continue_without_starter",
            Self::ApplyFix => "apply_fix",
            Self::OpenDeepLink => "open_deep_link",
        }
    }
}

/// Disclosures a template health row must carry, derived from the frozen freshness state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HealthRowDisclosure {
    /// The derived freshness posture this row may present.
    pub freshness_posture: HealthFreshnessPosture,
    /// Whether the signal is current.
    pub is_current: bool,
    /// Whether the row must carry an explicit stale / expired note.
    pub needs_stale_note: bool,
    /// Whether the row must carry an explicit never-checked note.
    pub needs_never_checked_note: bool,
    /// Whether the row must carry an explicit unavailable note.
    pub needs_unavailable_note: bool,
}

/// Resolves the freshness truth a template health row may present.
///
/// A `fresh` signal is current, an `aging` signal is aging, a `stale` or `expired` signal is
/// stale-or-expired, a `never_checked` signal is never checked, and an `unavailable` signal is
/// unavailable — so a stale, expired, never-checked, or unavailable signal can never read as
/// fresh.
pub fn resolve_health_disclosure(freshness_state: M5HealthFreshnessState) -> HealthRowDisclosure {
    use HealthFreshnessPosture as Posture;
    use M5HealthFreshnessState as State;

    let freshness_posture = match freshness_state {
        State::Fresh => Posture::Current,
        State::Aging => Posture::Aging,
        State::Stale | State::Expired => Posture::StaleOrExpired,
        State::NeverChecked => Posture::NeverChecked,
        State::Unavailable => Posture::Unavailable,
    };

    HealthRowDisclosure {
        freshness_posture,
        is_current: freshness_posture.is_current(),
        needs_stale_note: matches!(freshness_posture, Posture::StaleOrExpired),
        needs_never_checked_note: matches!(freshness_posture, Posture::NeverChecked),
        needs_unavailable_note: matches!(freshness_posture, Posture::Unavailable),
    }
}

/// A template health row naming its check name, status, freshness / source, `Blocker` / `Warning`
/// / `Info` severity, auto-fix or manual-fix note, derived freshness posture, bounded rerun /
/// open-detail actions, an explicit same-weight Create-empty / Continue-without-starter path, and
/// a stable deep link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateHealthRow {
    /// Frozen component this control implements; must be `template_health_row`.
    pub component: M5ScaffoldComponentFamily,
    /// Stable row id.
    pub row_id: String,
    /// Human-readable check name; required and non-empty.
    pub check_name: String,
    /// Health signal class, reused from the frozen matrix.
    pub signal_class: M5HealthSignalClass,
    /// Health freshness state, reused from the frozen matrix.
    pub freshness_state: M5HealthFreshnessState,
    /// Severity (the acceptance-criteria `Blocker` / `Warning` / `Info` label).
    pub severity: HealthSeverity,
    /// Whether the row offers an auto-fix, a manual fix, or no fix.
    pub fix_kind: HealthFixKind,
    /// Derived freshness posture (must equal the resolved posture).
    pub derived_freshness_posture: HealthFreshnessPosture,
    /// Whether the row claims the signal is current (must equal derived truth).
    pub claims_current: bool,
    /// Whether the row claims a blocked prerequisite (must equal the severity truth).
    pub claims_blocking_prerequisite: bool,
    /// Status label; always required so the check status stays explicit.
    pub status_label: String,
    /// Freshness / source label; always required so how current the signal is stays explicit.
    pub freshness_or_source_label: String,
    /// Stale / expired note; required when the signal is stale or expired.
    pub stale_note: String,
    /// Never-checked note; required when the signal has never been checked.
    pub never_checked_note: String,
    /// Unavailable note; required when the signal is unavailable.
    pub unavailable_note: String,
    /// Auto-fix / manual-fix note; required when a fix is available or required.
    pub fix_note: String,
    /// Create-empty / continue-without-starter note; always required so the same-weight recovery
    /// path stays explicit.
    pub create_empty_or_continue_note: String,
    /// Context note; always required so the row names what to check before committing.
    pub context_note: String,
    /// Kind of stable deep link this row binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include rerun-check / open-detail and a
    /// same-weight create-empty / continue-without-starter path).
    pub row_actions: Vec<HealthRowAction>,
    /// Dispositions this row binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5ScaffoldDisposition>,
    /// Downgrade triggers this row can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5ScaffoldDowngradeTrigger>,
    /// Mandatory labels this row can show (must include the mandatory labels).
    pub required_labels: Vec<M5ScaffoldRequiredLabel>,
    /// Claimed M5 surface families that render this row.
    pub surface_families: Vec<M5ScaffoldSurfaceFamily>,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5ScaffoldDeploymentLine>,
    /// Non-visual accessibility routes this row offers.
    pub accessibility_routes: Vec<M5ScaffoldAccessibilityRoute>,
    /// Scaffold subsystems that consume this row's projection.
    pub consumer_surfaces: Vec<M5ScaffoldConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never hides a side effect behind a generic Create. MUST be `false`.
    pub hides_side_effect_behind_generic_create: bool,
    /// Hard invariant: never hides its generated file / folder impact or its recovery path. MUST
    /// be `false`.
    pub hides_generated_impact_or_recovery_path: bool,
    /// Hard invariant: never monopolizes the plain Create-empty / Continue-without-starter path.
    /// MUST be `false`.
    pub monopolizes_plain_create_without_starter_path: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl TemplateHealthRow {
    /// Freshness disclosures this row must carry, derived from the frozen freshness state.
    pub fn freshness_disclosure(&self) -> HealthRowDisclosure {
        resolve_health_disclosure(self.freshness_state)
    }

    /// Whether the row offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<HealthRowAction> = self.row_actions.iter().copied().collect();
        HealthRowAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the row offers a same-weight Create-empty or Continue-without-starter path.
    fn offers_create_without_starter(&self) -> bool {
        self.row_actions.contains(&HealthRowAction::CreateEmpty)
            || self
                .row_actions
                .contains(&HealthRowAction::ContinueWithoutStarter)
    }

    /// Whether the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        declares_mandatory_labels(&self.required_labels)
    }

    /// Whether the row offers a deep-link-opening action.
    fn offers_deep_link_action(&self) -> bool {
        self.row_actions.contains(&HealthRowAction::OpenDeepLink)
    }
}

/// Whether a required-label list declares all three mandatory labels.
fn declares_mandatory_labels(labels: &[M5ScaffoldRequiredLabel]) -> bool {
    let present: BTreeSet<M5ScaffoldRequiredLabel> = labels.iter().copied().collect();
    M5ScaffoldRequiredLabel::MANDATORY
        .iter()
        .all(|label| present.contains(label))
}

// ---- review blocks ------------------------------------------------------

/// First-glance scaffold-readiness review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldReadinessReview {
    /// The preflight card names its concrete side effects.
    pub preflight_card_shows_side_effects: bool,
    /// The preflight card names its generated file / folder counts and impact.
    pub preflight_card_shows_generated_impact: bool,
    /// The preflight card names its immediate-versus-deferred actions.
    pub preflight_card_shows_immediate_versus_deferred: bool,
    /// The preflight card names a checkpoint or delete-generated recovery path.
    pub preflight_card_names_recovery_path: bool,
    /// The health row names its severity and freshness.
    pub health_row_shows_severity_and_freshness: bool,
    /// The health row names its auto-fix or manual-fix note.
    pub health_row_shows_fix_note: bool,
    /// The health row offers rerun and open-detail.
    pub health_row_offers_rerun_and_open_detail: bool,
    /// The health row keeps a same-weight Create-empty or Continue-without-starter path.
    pub health_row_offers_create_empty_or_continue_without_starter: bool,
    /// Severity and freshness are derived from state, never asserted.
    pub severity_and_freshness_derived_never_asserted: bool,
    /// A blocked prerequisite is never shown as an optional optimization.
    pub blocked_prerequisite_never_shown_as_optional: bool,
    /// A stale, expired, never-checked, or unavailable signal is never shown as fresh.
    pub stale_signal_never_shown_as_fresh: bool,
    /// Creation never routes through a generic Create that hides side effects.
    pub create_never_generic_hides_side_effects: bool,
    /// Generated file / folder counts stay explicit.
    pub generated_file_counts_always_explicit: bool,
    /// The recovery path stays reachable.
    pub recovery_path_always_reachable: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// The components keep the same truth across every deployment line.
    pub components_stable_across_deployment_lines: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl ScaffoldReadinessReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.preflight_card_shows_side_effects
            && self.preflight_card_shows_generated_impact
            && self.preflight_card_shows_immediate_versus_deferred
            && self.preflight_card_names_recovery_path
            && self.health_row_shows_severity_and_freshness
            && self.health_row_shows_fix_note
            && self.health_row_offers_rerun_and_open_detail
            && self.health_row_offers_create_empty_or_continue_without_starter
            && self.severity_and_freshness_derived_never_asserted
            && self.blocked_prerequisite_never_shown_as_optional
            && self.stale_signal_never_shown_as_fresh
            && self.create_never_generic_hides_side_effects
            && self.generated_file_counts_always_explicit
            && self.recovery_path_always_reachable
            && self.no_surface_invents_alternate_state_label
            && self.components_stable_across_deployment_lines
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldReadinessConsumerProjection {
    /// The start-center reads a single canonical source.
    pub start_center_reads_single_source: bool,
    /// The preflight surface reads a single canonical source.
    pub preflight_surface_reads_single_source: bool,
    /// The health dashboard reads a single canonical source.
    pub health_dashboard_reads_single_source: bool,
    /// Side effects are visible before commit.
    pub side_effects_visible_before_commit: bool,
    /// The recovery path is visible before commit.
    pub recovery_path_visible_before_commit: bool,
    /// Support export shows component truth.
    pub support_export_shows_component_truth: bool,
}

impl ScaffoldReadinessConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.start_center_reads_single_source
            && self.preflight_surface_reads_single_source
            && self.health_dashboard_reads_single_source
            && self.side_effects_visible_before_commit
            && self.recovery_path_visible_before_commit
            && self.support_export_shows_component_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldReadinessProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for
/// [`ScaffoldPreflightCardTemplateHealthRowControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaffoldPreflightCardTemplateHealthRowControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Scaffold preflight cards.
    pub preflight_cards: Vec<ScaffoldPreflightCard>,
    /// Template health rows.
    pub health_rows: Vec<TemplateHealthRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5ScaffoldDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5ScaffoldConsumerSurface>,
    /// Scaffold-readiness review block.
    pub readiness_review: ScaffoldReadinessReview,
    /// Consumer projection block.
    pub consumer_projection: ScaffoldReadinessConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ScaffoldReadinessProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe scaffold-preflight-card / template-health-row controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldPreflightCardTemplateHealthRowControlsPacket {
    /// Record kind; must equal [`SCAFFOLD_READINESS_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`SCAFFOLD_READINESS_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Scaffold preflight cards.
    pub preflight_cards: Vec<ScaffoldPreflightCard>,
    /// Template health rows.
    pub health_rows: Vec<TemplateHealthRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5ScaffoldDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5ScaffoldConsumerSurface>,
    /// Scaffold-readiness review block.
    pub readiness_review: ScaffoldReadinessReview,
    /// Consumer projection block.
    pub consumer_projection: ScaffoldReadinessConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ScaffoldReadinessProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ScaffoldPreflightCardTemplateHealthRowControlsPacket {
    /// Builds a scaffold-preflight-card / template-health-row controls packet from stable-lane
    /// input.
    pub fn new(input: ScaffoldPreflightCardTemplateHealthRowControlsPacketInput) -> Self {
        Self {
            record_kind: SCAFFOLD_READINESS_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: SCAFFOLD_READINESS_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            preflight_cards: input.preflight_cards,
            health_rows: input.health_rows,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            readiness_review: input.readiness_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the scaffold-preflight-card / template-health-row control invariants.
    pub fn validate(&self) -> Vec<ScaffoldReadinessControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != SCAFFOLD_READINESS_CONTROLS_RECORD_KIND {
            violations.push(ScaffoldReadinessControlsViolation::WrongRecordKind);
        }
        if self.schema_version != SCAFFOLD_READINESS_CONTROLS_SCHEMA_VERSION {
            violations.push(ScaffoldReadinessControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ScaffoldReadinessControlsViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(ScaffoldReadinessControlsViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(ScaffoldReadinessControlsViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_preflight_cards(self, &mut violations);
        validate_health_rows(self, &mut violations);

        if !self.readiness_review.all_hold() {
            violations.push(ScaffoldReadinessControlsViolation::ReadinessReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(ScaffoldReadinessControlsViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(ScaffoldReadinessControlsViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("scaffold readiness controls packet serializes"),
        ) {
            violations.push(ScaffoldReadinessControlsViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("scaffold readiness controls packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component,id,frozen_state,secondary_state,derived,blocking_or_current,deep_link_kind\n",
        );
        for card in &self.preflight_cards {
            let disclosure = card.severity_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "scaffold_preflight_card",
                csv_field(&card.card_id),
                card.check_class.as_str(),
                card.result_state.as_str(),
                disclosure.severity.as_str(),
                disclosure.is_blocking,
                card.deep_link_kind.as_str(),
            ));
        }
        for row in &self.health_rows {
            let disclosure = row.freshness_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "template_health_row",
                csv_field(&row.row_id),
                row.signal_class.as_str(),
                row.freshness_state.as_str(),
                disclosure.freshness_posture.as_str(),
                disclosure.is_current,
                row.deep_link_kind.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let blocking_cards = self
            .preflight_cards
            .iter()
            .filter(|card| card.severity_disclosure().is_blocking)
            .count();
        let stale_rows = self
            .health_rows
            .iter()
            .filter(|row| !row.freshness_disclosure().is_current)
            .count();

        let mut out = String::new();
        out.push_str("# Scaffold preflight cards and template health rows\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Scaffold preflight cards: {} ({} blocked prerequisites)\n",
            self.preflight_cards.len(),
            blocking_cards
        ));
        out.push_str(&format!(
            "- Template health rows: {} ({} not current)\n",
            self.health_rows.len(),
            stale_rows
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Scaffold preflight cards\n\n");
        for card in &self.preflight_cards {
            out.push_str(&format!(
                "- **{}** — check `{}`, result `{}` → `{}`, side effect `{}`, timing `{}`, {} files / {} folders, deep link `{}`\n",
                card.card_name,
                card.check_class.as_str(),
                card.result_state.as_str(),
                card.severity_disclosure().severity.as_str(),
                card.side_effect_kind.as_str(),
                card.action_timing.as_str(),
                card.generated_file_count,
                card.generated_folder_count,
                card.deep_link_kind.as_str(),
            ));
        }

        out.push_str("\n## Template health rows\n\n");
        for row in &self.health_rows {
            out.push_str(&format!(
                "- **{}** — signal `{}`, freshness `{}` → `{}`, severity `{}`, fix `{}`, deep link `{}`\n",
                row.check_name,
                row.signal_class.as_str(),
                row.freshness_state.as_str(),
                row.freshness_disclosure().freshness_posture.as_str(),
                row.severity.as_str(),
                row.fix_kind.as_str(),
                row.deep_link_kind.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in scaffold-readiness controls export.
#[derive(Debug)]
pub enum ScaffoldReadinessControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ScaffoldReadinessControlsViolation>),
}

impl fmt::Display for ScaffoldReadinessControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "scaffold readiness controls export parse failed: {error}"
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
                    "scaffold readiness controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ScaffoldReadinessControlsArtifactError {}

/// Validation failures emitted by
/// [`ScaffoldPreflightCardTemplateHealthRowControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScaffoldReadinessControlsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No scaffold preflight cards are present.
    PreflightCardsMissing,
    /// A scaffold preflight card is incomplete.
    PreflightCardIncomplete,
    /// A scaffold preflight card carries the wrong frozen component class.
    PreflightCardWrongComponentClass,
    /// A preflight card misrepresents its derived severity or claims.
    PreflightSeverityMisrepresented,
    /// A side-effecting card does not name its side effect.
    SideEffectNoteMissing,
    /// A blocked-prerequisite card does not name its blocked state.
    PreflightBlockedNoteMissing,
    /// An advisory card does not name its warning.
    PreflightWarningNoteMissing,
    /// An optional-skipped card does not name its skip.
    PreflightSkippedNoteMissing,
    /// A needs-attention card does not name its attention state.
    PreflightAttentionNoteMissing,
    /// A preflight card does not name its target path.
    TargetPathMissing,
    /// A preflight card does not name its target name.
    TargetNameMissing,
    /// A preflight card does not name its generated impact.
    GeneratedImpactNoteMissing,
    /// A preflight card does not name its dependency impact.
    DependencyImpactMissing,
    /// A preflight card does not name its task impact.
    TaskImpactMissing,
    /// A preflight card does not name its extension impact.
    ExtensionImpactMissing,
    /// A preflight card does not name its immediate action.
    ImmediateActionMissing,
    /// A preflight card does not name its deferred action.
    DeferredActionMissing,
    /// A preflight card does not name a recovery path.
    RecoveryPathMissing,
    /// A preflight card omits a mandatory review action.
    PreflightCardActionsIncomplete,
    /// The preflight cards do not cover every preflight check class.
    PreflightCheckClassCoverageMissing,
    /// The preflight cards do not cover every preflight result state.
    PreflightResultStateCoverageMissing,
    /// The preflight cards do not cover every real side-effect kind.
    PreflightSideEffectCoverageMissing,
    /// The preflight cards do not cover every derived severity.
    PreflightSeverityCoverageMissing,
    /// No template health rows are present.
    HealthRowsMissing,
    /// A template health row is incomplete.
    HealthRowIncomplete,
    /// A template health row carries the wrong frozen component class.
    HealthRowWrongComponentClass,
    /// A health row misrepresents its derived freshness posture or claims.
    HealthPostureMisrepresented,
    /// A stale / expired row does not name its stale state.
    HealthStaleNoteMissing,
    /// A never-checked row does not name its never-checked state.
    HealthNeverCheckedNoteMissing,
    /// An unavailable row does not name its unavailable state.
    HealthUnavailableNoteMissing,
    /// A fixable row does not name its auto-fix / manual-fix note.
    HealthFixNoteMissing,
    /// A health row does not name its status.
    HealthStatusMissing,
    /// A health row does not name its freshness / source.
    HealthFreshnessLabelMissing,
    /// A health row does not name its Create-empty / Continue-without-starter path.
    HealthRecoveryNoteMissing,
    /// A health row does not offer a same-weight Create-empty / Continue-without-starter action.
    HealthRecoveryPathMissing,
    /// A health row omits a mandatory rerun / open-detail action.
    HealthRowActionsIncomplete,
    /// The health rows do not cover every health signal class.
    HealthSignalClassCoverageMissing,
    /// The health rows do not cover every health freshness state.
    HealthFreshnessStateCoverageMissing,
    /// The health rows do not cover every severity.
    HealthSeverityCoverageMissing,
    /// The health rows do not cover every derived freshness posture.
    HealthPostureCoverageMissing,
    /// A component does not name its context.
    ContextNoteMissing,
    /// A component offers a deep-link action but its deep link does not resolve exactly.
    DeepLinkUnresolved,
    /// A component names a deep-link kind but not its stable reference.
    DeepLinkRefMissing,
    /// A component does not bind any disposition.
    DispositionsMissing,
    /// A component does not declare its downgrade triggers.
    DowngradeTriggersMissing,
    /// A component does not declare its mandatory labels.
    RequiredLabelsIncomplete,
    /// A component does not declare an accessibility route (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A component hides a side effect behind a generic Create.
    SideEffectBehindGenericCreate,
    /// A component hides its generated impact or recovery path.
    GeneratedImpactOrRecoveryHidden,
    /// A component monopolizes the plain Create-empty / Continue-without-starter path.
    PlainCreateWithoutStarterMonopolized,
    /// A component invents an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Readiness review does not satisfy required invariants.
    ReadinessReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl ScaffoldReadinessControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::PreflightCardsMissing => "preflight_cards_missing",
            Self::PreflightCardIncomplete => "preflight_card_incomplete",
            Self::PreflightCardWrongComponentClass => "preflight_card_wrong_component_class",
            Self::PreflightSeverityMisrepresented => "preflight_severity_misrepresented",
            Self::SideEffectNoteMissing => "side_effect_note_missing",
            Self::PreflightBlockedNoteMissing => "preflight_blocked_note_missing",
            Self::PreflightWarningNoteMissing => "preflight_warning_note_missing",
            Self::PreflightSkippedNoteMissing => "preflight_skipped_note_missing",
            Self::PreflightAttentionNoteMissing => "preflight_attention_note_missing",
            Self::TargetPathMissing => "target_path_missing",
            Self::TargetNameMissing => "target_name_missing",
            Self::GeneratedImpactNoteMissing => "generated_impact_note_missing",
            Self::DependencyImpactMissing => "dependency_impact_missing",
            Self::TaskImpactMissing => "task_impact_missing",
            Self::ExtensionImpactMissing => "extension_impact_missing",
            Self::ImmediateActionMissing => "immediate_action_missing",
            Self::DeferredActionMissing => "deferred_action_missing",
            Self::RecoveryPathMissing => "recovery_path_missing",
            Self::PreflightCardActionsIncomplete => "preflight_card_actions_incomplete",
            Self::PreflightCheckClassCoverageMissing => "preflight_check_class_coverage_missing",
            Self::PreflightResultStateCoverageMissing => "preflight_result_state_coverage_missing",
            Self::PreflightSideEffectCoverageMissing => "preflight_side_effect_coverage_missing",
            Self::PreflightSeverityCoverageMissing => "preflight_severity_coverage_missing",
            Self::HealthRowsMissing => "health_rows_missing",
            Self::HealthRowIncomplete => "health_row_incomplete",
            Self::HealthRowWrongComponentClass => "health_row_wrong_component_class",
            Self::HealthPostureMisrepresented => "health_posture_misrepresented",
            Self::HealthStaleNoteMissing => "health_stale_note_missing",
            Self::HealthNeverCheckedNoteMissing => "health_never_checked_note_missing",
            Self::HealthUnavailableNoteMissing => "health_unavailable_note_missing",
            Self::HealthFixNoteMissing => "health_fix_note_missing",
            Self::HealthStatusMissing => "health_status_missing",
            Self::HealthFreshnessLabelMissing => "health_freshness_label_missing",
            Self::HealthRecoveryNoteMissing => "health_recovery_note_missing",
            Self::HealthRecoveryPathMissing => "health_recovery_path_missing",
            Self::HealthRowActionsIncomplete => "health_row_actions_incomplete",
            Self::HealthSignalClassCoverageMissing => "health_signal_class_coverage_missing",
            Self::HealthFreshnessStateCoverageMissing => "health_freshness_state_coverage_missing",
            Self::HealthSeverityCoverageMissing => "health_severity_coverage_missing",
            Self::HealthPostureCoverageMissing => "health_posture_coverage_missing",
            Self::ContextNoteMissing => "context_note_missing",
            Self::DeepLinkUnresolved => "deep_link_unresolved",
            Self::DeepLinkRefMissing => "deep_link_ref_missing",
            Self::DispositionsMissing => "dispositions_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RequiredLabelsIncomplete => "required_labels_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::SideEffectBehindGenericCreate => "side_effect_behind_generic_create",
            Self::GeneratedImpactOrRecoveryHidden => "generated_impact_or_recovery_hidden",
            Self::PlainCreateWithoutStarterMonopolized => {
                "plain_create_without_starter_monopolized"
            }
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ReadinessReviewIncomplete => "readiness_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable scaffold-readiness controls export.
///
/// This is the first real consumer of the scaffold-readiness component lane: a start-center,
/// preflight, template-health, or support-export surface calls it to ingest the canonical
/// components rather than cloning status text.
///
/// # Errors
///
/// Returns [`ScaffoldReadinessControlsArtifactError`] when the checked-in support export fails to
/// parse or fails validation.
pub fn current_scaffold_readiness_controls_export() -> Result<
    ScaffoldPreflightCardTemplateHealthRowControlsPacket,
    ScaffoldReadinessControlsArtifactError,
> {
    let packet: ScaffoldPreflightCardTemplateHealthRowControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-scaffold-preflight-card-template-health-row-proof/support_export.json"
        )))
        .map_err(ScaffoldReadinessControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ScaffoldReadinessControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &ScaffoldPreflightCardTemplateHealthRowControlsPacket,
    violations: &mut Vec<ScaffoldReadinessControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        SCAFFOLD_READINESS_CONTROLS_SCHEMA_REF,
        SCAFFOLD_READINESS_CONTROLS_DOC_REF,
        M5_SCAFFOLD_COMPONENT_SCHEMA_REF,
        M5_SCAFFOLD_COMPONENT_DOC_REF,
        M5_SCAFFOLD_PREFLIGHT_CARD_SCHEMA_REF,
        M5_TEMPLATE_HEALTH_ROW_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ScaffoldReadinessControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_preflight_cards(
    packet: &ScaffoldPreflightCardTemplateHealthRowControlsPacket,
    violations: &mut Vec<ScaffoldReadinessControlsViolation>,
) {
    if packet.preflight_cards.is_empty() {
        violations.push(ScaffoldReadinessControlsViolation::PreflightCardsMissing);
        return;
    }

    let mut checks: BTreeSet<M5PreflightCheckClass> = BTreeSet::new();
    let mut results: BTreeSet<M5PreflightResultState> = BTreeSet::new();
    let mut side_effects: BTreeSet<PreflightSideEffectKind> = BTreeSet::new();
    let mut severities: BTreeSet<PreflightSeverity> = BTreeSet::new();

    for card in &packet.preflight_cards {
        let disclosure = card.severity_disclosure();
        checks.insert(card.check_class);
        results.insert(card.result_state);
        side_effects.insert(card.side_effect_kind);
        severities.insert(disclosure.severity);

        if card.card_id.trim().is_empty()
            || card.card_name.trim().is_empty()
            || card.fields_shown.is_empty()
            || card.surface_families.is_empty()
            || card.deployment_lines.is_empty()
            || card.consumer_surfaces.is_empty()
            || card.source_contract_refs.is_empty()
        {
            violations.push(ScaffoldReadinessControlsViolation::PreflightCardIncomplete);
        }
        if card.component != M5ScaffoldComponentFamily::ScaffoldPreflightCard {
            violations.push(ScaffoldReadinessControlsViolation::PreflightCardWrongComponentClass);
        }
        if card.derived_severity != disclosure.severity
            || card.claims_blocking_prerequisite != disclosure.is_blocking
            || card.claims_side_effecting != card.side_effect_kind.is_side_effecting()
            || card.claims_immediate_action != card.action_timing.is_immediate()
        {
            violations.push(ScaffoldReadinessControlsViolation::PreflightSeverityMisrepresented);
        }
        if card.side_effect_kind.is_side_effecting() && card.side_effect_note.trim().is_empty() {
            violations.push(ScaffoldReadinessControlsViolation::SideEffectNoteMissing);
        }
        if disclosure.needs_blocked_note && card.blocked_note.trim().is_empty() {
            violations.push(ScaffoldReadinessControlsViolation::PreflightBlockedNoteMissing);
        }
        if disclosure.needs_warning_note && card.warning_note.trim().is_empty() {
            violations.push(ScaffoldReadinessControlsViolation::PreflightWarningNoteMissing);
        }
        if disclosure.needs_skipped_note && card.skipped_note.trim().is_empty() {
            violations.push(ScaffoldReadinessControlsViolation::PreflightSkippedNoteMissing);
        }
        if disclosure.needs_attention_note && card.attention_note.trim().is_empty() {
            violations.push(ScaffoldReadinessControlsViolation::PreflightAttentionNoteMissing);
        }
        if card.target_path_label.trim().is_empty() {
            violations.push(ScaffoldReadinessControlsViolation::TargetPathMissing);
        }
        if card.target_name_label.trim().is_empty() {
            violations.push(ScaffoldReadinessControlsViolation::TargetNameMissing);
        }
        if card.generated_impact_note.trim().is_empty() {
            violations.push(ScaffoldReadinessControlsViolation::GeneratedImpactNoteMissing);
        }
        if card.dependency_impact_label.trim().is_empty() {
            violations.push(ScaffoldReadinessControlsViolation::DependencyImpactMissing);
        }
        if card.task_impact_label.trim().is_empty() {
            violations.push(ScaffoldReadinessControlsViolation::TaskImpactMissing);
        }
        if card.extension_impact_label.trim().is_empty() {
            violations.push(ScaffoldReadinessControlsViolation::ExtensionImpactMissing);
        }
        if card.immediate_action_label.trim().is_empty() {
            violations.push(ScaffoldReadinessControlsViolation::ImmediateActionMissing);
        }
        if card.deferred_action_label.trim().is_empty() {
            violations.push(ScaffoldReadinessControlsViolation::DeferredActionMissing);
        }
        if card.recovery_path_label.trim().is_empty() {
            violations.push(ScaffoldReadinessControlsViolation::RecoveryPathMissing);
        }
        if !card.declares_mandatory_actions() {
            violations.push(ScaffoldReadinessControlsViolation::PreflightCardActionsIncomplete);
        }
        if !card.offers_create_empty() {
            violations
                .push(ScaffoldReadinessControlsViolation::PlainCreateWithoutStarterMonopolized);
        }
        validate_deep_link(
            card.offers_deep_link_action(),
            card.deep_link_kind,
            &card.deep_link_ref,
            &card.context_note,
            violations,
        );
        validate_common_control(
            &card.dispositions,
            &card.downgrade_triggers,
            card.declares_mandatory_labels(),
            &card.accessibility_routes,
            ControlInvariants {
                hides_side_effect_behind_generic_create: card
                    .hides_side_effect_behind_generic_create,
                hides_generated_impact_or_recovery_path: card
                    .hides_generated_impact_or_recovery_path,
                monopolizes_plain_create_without_starter_path: card
                    .monopolizes_plain_create_without_starter_path,
                invents_alternate_state_label: card.invents_alternate_state_label,
            },
            violations,
        );
    }

    for required in M5PreflightCheckClass::ALL {
        if !checks.contains(&required) {
            violations.push(ScaffoldReadinessControlsViolation::PreflightCheckClassCoverageMissing);
            break;
        }
    }
    for required in M5PreflightResultState::ALL {
        if !results.contains(&required) {
            violations
                .push(ScaffoldReadinessControlsViolation::PreflightResultStateCoverageMissing);
            break;
        }
    }
    for required in PreflightSideEffectKind::REAL {
        if !side_effects.contains(&required) {
            violations.push(ScaffoldReadinessControlsViolation::PreflightSideEffectCoverageMissing);
            break;
        }
    }
    for required in PreflightSeverity::ALL {
        if !severities.contains(&required) {
            violations.push(ScaffoldReadinessControlsViolation::PreflightSeverityCoverageMissing);
            break;
        }
    }
}

fn validate_health_rows(
    packet: &ScaffoldPreflightCardTemplateHealthRowControlsPacket,
    violations: &mut Vec<ScaffoldReadinessControlsViolation>,
) {
    if packet.health_rows.is_empty() {
        violations.push(ScaffoldReadinessControlsViolation::HealthRowsMissing);
        return;
    }

    let mut signals: BTreeSet<M5HealthSignalClass> = BTreeSet::new();
    let mut freshnesses: BTreeSet<M5HealthFreshnessState> = BTreeSet::new();
    let mut severities: BTreeSet<HealthSeverity> = BTreeSet::new();
    let mut postures: BTreeSet<HealthFreshnessPosture> = BTreeSet::new();

    for row in &packet.health_rows {
        let disclosure = row.freshness_disclosure();
        signals.insert(row.signal_class);
        freshnesses.insert(row.freshness_state);
        severities.insert(row.severity);
        postures.insert(disclosure.freshness_posture);

        if row.row_id.trim().is_empty()
            || row.check_name.trim().is_empty()
            || row.fields_shown.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.consumer_surfaces.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(ScaffoldReadinessControlsViolation::HealthRowIncomplete);
        }
        if row.component != M5ScaffoldComponentFamily::TemplateHealthRow {
            violations.push(ScaffoldReadinessControlsViolation::HealthRowWrongComponentClass);
        }
        if row.derived_freshness_posture != disclosure.freshness_posture
            || row.claims_current != disclosure.is_current
            || row.claims_blocking_prerequisite != row.severity.is_blocking_prerequisite()
        {
            violations.push(ScaffoldReadinessControlsViolation::HealthPostureMisrepresented);
        }
        if disclosure.needs_stale_note && row.stale_note.trim().is_empty() {
            violations.push(ScaffoldReadinessControlsViolation::HealthStaleNoteMissing);
        }
        if disclosure.needs_never_checked_note && row.never_checked_note.trim().is_empty() {
            violations.push(ScaffoldReadinessControlsViolation::HealthNeverCheckedNoteMissing);
        }
        if disclosure.needs_unavailable_note && row.unavailable_note.trim().is_empty() {
            violations.push(ScaffoldReadinessControlsViolation::HealthUnavailableNoteMissing);
        }
        if row.fix_kind.needs_fix_note() && row.fix_note.trim().is_empty() {
            violations.push(ScaffoldReadinessControlsViolation::HealthFixNoteMissing);
        }
        if row.status_label.trim().is_empty() {
            violations.push(ScaffoldReadinessControlsViolation::HealthStatusMissing);
        }
        if row.freshness_or_source_label.trim().is_empty() {
            violations.push(ScaffoldReadinessControlsViolation::HealthFreshnessLabelMissing);
        }
        if row.create_empty_or_continue_note.trim().is_empty() {
            violations.push(ScaffoldReadinessControlsViolation::HealthRecoveryNoteMissing);
        }
        if !row.offers_create_without_starter() {
            violations.push(ScaffoldReadinessControlsViolation::HealthRecoveryPathMissing);
        }
        if !row.declares_mandatory_actions() {
            violations.push(ScaffoldReadinessControlsViolation::HealthRowActionsIncomplete);
        }
        validate_deep_link(
            row.offers_deep_link_action(),
            row.deep_link_kind,
            &row.deep_link_ref,
            &row.context_note,
            violations,
        );
        validate_common_control(
            &row.dispositions,
            &row.downgrade_triggers,
            row.declares_mandatory_labels(),
            &row.accessibility_routes,
            ControlInvariants {
                hides_side_effect_behind_generic_create: row
                    .hides_side_effect_behind_generic_create,
                hides_generated_impact_or_recovery_path: row
                    .hides_generated_impact_or_recovery_path,
                monopolizes_plain_create_without_starter_path: row
                    .monopolizes_plain_create_without_starter_path,
                invents_alternate_state_label: row.invents_alternate_state_label,
            },
            violations,
        );
    }

    for required in M5HealthSignalClass::ALL {
        if !signals.contains(&required) {
            violations.push(ScaffoldReadinessControlsViolation::HealthSignalClassCoverageMissing);
            break;
        }
    }
    for required in M5HealthFreshnessState::ALL {
        if !freshnesses.contains(&required) {
            violations
                .push(ScaffoldReadinessControlsViolation::HealthFreshnessStateCoverageMissing);
            break;
        }
    }
    for required in HealthSeverity::ALL {
        if !severities.contains(&required) {
            violations.push(ScaffoldReadinessControlsViolation::HealthSeverityCoverageMissing);
            break;
        }
    }
    for required in HealthFreshnessPosture::ALL {
        if !postures.contains(&required) {
            violations.push(ScaffoldReadinessControlsViolation::HealthPostureCoverageMissing);
            break;
        }
    }
}

/// Validates the context and stable deep-link truth shared by both component vectors.
///
/// A component that offers a deep-link action must name a resolvable deep-link kind, a component
/// that names a resolvable kind must carry its stable reference, and every component must name its
/// context — so a next step is never an ephemeral overlay or hidden route.
fn validate_deep_link(
    offers_deep_link_action: bool,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    context_note: &str,
    violations: &mut Vec<ScaffoldReadinessControlsViolation>,
) {
    if context_note.trim().is_empty() {
        violations.push(ScaffoldReadinessControlsViolation::ContextNoteMissing);
    }
    if offers_deep_link_action && !deep_link_kind.is_resolvable() {
        violations.push(ScaffoldReadinessControlsViolation::DeepLinkUnresolved);
    }
    if deep_link_kind.is_resolvable() && deep_link_ref.trim().is_empty() {
        violations.push(ScaffoldReadinessControlsViolation::DeepLinkRefMissing);
    }
}

/// The four hard-invariant bools every component must keep `false`.
struct ControlInvariants {
    hides_side_effect_behind_generic_create: bool,
    hides_generated_impact_or_recovery_path: bool,
    monopolizes_plain_create_without_starter_path: bool,
    invents_alternate_state_label: bool,
}

/// Validates the axes shared by both component vectors.
fn validate_common_control(
    dispositions: &[M5ScaffoldDisposition],
    downgrade_triggers: &[M5ScaffoldDowngradeTrigger],
    declares_mandatory_labels: bool,
    accessibility_routes: &[M5ScaffoldAccessibilityRoute],
    invariants: ControlInvariants,
    violations: &mut Vec<ScaffoldReadinessControlsViolation>,
) {
    if dispositions.is_empty() {
        violations.push(ScaffoldReadinessControlsViolation::DispositionsMissing);
    }
    if downgrade_triggers.is_empty() {
        violations.push(ScaffoldReadinessControlsViolation::DowngradeTriggersMissing);
    }
    if !declares_mandatory_labels {
        violations.push(ScaffoldReadinessControlsViolation::RequiredLabelsIncomplete);
    }
    if accessibility_routes.is_empty()
        || !accessibility_routes.contains(&M5ScaffoldAccessibilityRoute::KeyboardFocusable)
    {
        violations.push(ScaffoldReadinessControlsViolation::AccessibilityRouteMissing);
    }
    if invariants.hides_side_effect_behind_generic_create {
        violations.push(ScaffoldReadinessControlsViolation::SideEffectBehindGenericCreate);
    }
    if invariants.hides_generated_impact_or_recovery_path {
        violations.push(ScaffoldReadinessControlsViolation::GeneratedImpactOrRecoveryHidden);
    }
    if invariants.monopolizes_plain_create_without_starter_path {
        violations.push(ScaffoldReadinessControlsViolation::PlainCreateWithoutStarterMonopolized);
    }
    if invariants.invents_alternate_state_label {
        violations.push(ScaffoldReadinessControlsViolation::AlternateStateLabelInvented);
    }
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Canonical seed builders
//
// These builders are the single producer of the checked-in support export and the scenario
// fixtures. The headless emitter example and the inline tests both call them so the in-code
// components, the artifact, and the fixtures never drift.
// ---------------------------------------------------------------------------

/// Stable packet id for the canonical scaffold-readiness controls packet.
pub const SCAFFOLD_READINESS_CONTROLS_PACKET_ID: &str =
    "m5-scaffold-preflight-card-template-health-row-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn preflight_card_source_refs() -> Vec<String> {
    strings(&[
        M5_SCAFFOLD_PREFLIGHT_CARD_SCHEMA_REF,
        M5_SCAFFOLD_COMPONENT_SCHEMA_REF,
    ])
}

fn health_row_source_refs() -> Vec<String> {
    strings(&[
        M5_TEMPLATE_HEALTH_ROW_SCHEMA_REF,
        M5_SCAFFOLD_COMPONENT_SCHEMA_REF,
    ])
}

fn preflight_card_downgrade_triggers() -> Vec<M5ScaffoldDowngradeTrigger> {
    vec![
        M5ScaffoldDowngradeTrigger::SideEffectUndisclosed,
        M5ScaffoldDowngradeTrigger::HostBoundaryUnstated,
        M5ScaffoldDowngradeTrigger::ImpactUndisclosed,
        M5ScaffoldDowngradeTrigger::AlternateStateLabelInvented,
        M5ScaffoldDowngradeTrigger::ProofStale,
    ]
}

fn health_row_downgrade_triggers() -> Vec<M5ScaffoldDowngradeTrigger> {
    vec![
        M5ScaffoldDowngradeTrigger::HealthFreshnessStale,
        M5ScaffoldDowngradeTrigger::RecoveryPathOmitted,
        M5ScaffoldDowngradeTrigger::AlternateStateLabelInvented,
        M5ScaffoldDowngradeTrigger::ProofStale,
    ]
}

/// Input for [`preflight_card`], grouped so the seed builder stays under the argument limit and
/// reads as one preflight scenario.
struct PreflightCardSeed<'a> {
    card_id: &'a str,
    card_name: &'a str,
    check_class: M5PreflightCheckClass,
    result_state: M5PreflightResultState,
    side_effect_kind: PreflightSideEffectKind,
    action_timing: PreflightActionTiming,
    side_effect_note: &'a str,
    target_path_label: &'a str,
    target_name_label: &'a str,
    generated_file_count: u32,
    generated_folder_count: u32,
    dependency_impact_label: &'a str,
    task_impact_label: &'a str,
    extension_impact_label: &'a str,
    immediate_action_label: &'a str,
    deferred_action_label: &'a str,
    recovery_path_label: &'a str,
    context_note: &'a str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &'a str,
    card_actions: Vec<PreflightCardAction>,
    dispositions: Vec<M5ScaffoldDisposition>,
}

/// Builds a scaffold preflight card, deriving the severity, blocking and side-effect claims, and
/// the required notes from the honest inputs so the seed is always self-consistent with the
/// resolver.
fn preflight_card(seed: PreflightCardSeed<'_>) -> ScaffoldPreflightCard {
    let disclosure = resolve_preflight_disclosure(seed.result_state);
    ScaffoldPreflightCard {
        component: M5ScaffoldComponentFamily::ScaffoldPreflightCard,
        card_id: seed.card_id.to_owned(),
        card_name: seed.card_name.to_owned(),
        check_class: seed.check_class,
        result_state: seed.result_state,
        side_effect_kind: seed.side_effect_kind,
        action_timing: seed.action_timing,
        derived_severity: disclosure.severity,
        claims_blocking_prerequisite: disclosure.is_blocking,
        claims_side_effecting: seed.side_effect_kind.is_side_effecting(),
        claims_immediate_action: seed.action_timing.is_immediate(),
        side_effect_note: if seed.side_effect_kind.is_side_effecting() {
            seed.side_effect_note.to_owned()
        } else {
            String::new()
        },
        blocked_note: if disclosure.needs_blocked_note {
            "Blocked prerequisite; resolve it before create can proceed".to_owned()
        } else {
            String::new()
        },
        warning_note: if disclosure.needs_warning_note {
            "Advisory warning; create can proceed but review this first".to_owned()
        } else {
            String::new()
        },
        skipped_note: if disclosure.needs_skipped_note {
            "Optional check skipped; nothing is blocked by it".to_owned()
        } else {
            String::new()
        },
        attention_note: if disclosure.needs_attention_note {
            "Check has not run or its result is unknown; rerun it before relying on this preflight"
                .to_owned()
        } else {
            String::new()
        },
        target_path_label: seed.target_path_label.to_owned(),
        target_name_label: seed.target_name_label.to_owned(),
        generated_file_count: seed.generated_file_count,
        generated_folder_count: seed.generated_folder_count,
        generated_impact_note: format!(
            "Generates {} files and {} folders",
            seed.generated_file_count, seed.generated_folder_count
        ),
        dependency_impact_label: seed.dependency_impact_label.to_owned(),
        task_impact_label: seed.task_impact_label.to_owned(),
        extension_impact_label: seed.extension_impact_label.to_owned(),
        immediate_action_label: seed.immediate_action_label.to_owned(),
        deferred_action_label: seed.deferred_action_label.to_owned(),
        recovery_path_label: seed.recovery_path_label.to_owned(),
        context_note: seed.context_note.to_owned(),
        deep_link_kind: seed.deep_link_kind,
        deep_link_ref: seed.deep_link_ref.to_owned(),
        card_actions: seed.card_actions,
        dispositions: seed.dispositions,
        downgrade_triggers: preflight_card_downgrade_triggers(),
        required_labels: label_set(M5ScaffoldRequiredLabel::SideEffectDisclosure),
        surface_families: M5ScaffoldSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ScaffoldDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5ScaffoldAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5ScaffoldConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "target_path_label",
            "target_name_label",
            "generated_file_count",
            "generated_folder_count",
            "side_effect_kind",
            "action_timing",
            "result_state",
            "recovery_path_label",
        ]),
        source_contract_refs: preflight_card_source_refs(),
        hides_side_effect_behind_generic_create: false,
        hides_generated_impact_or_recovery_path: false,
        monopolizes_plain_create_without_starter_path: false,
        invents_alternate_state_label: false,
    }
}

/// Input for [`health_row`], grouped so the seed builder stays under the argument limit and reads
/// as one health scenario.
struct HealthRowSeed<'a> {
    row_id: &'a str,
    check_name: &'a str,
    signal_class: M5HealthSignalClass,
    freshness_state: M5HealthFreshnessState,
    severity: HealthSeverity,
    fix_kind: HealthFixKind,
    status_label: &'a str,
    freshness_or_source_label: &'a str,
    fix_note: &'a str,
    context_note: &'a str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &'a str,
    row_actions: Vec<HealthRowAction>,
    dispositions: Vec<M5ScaffoldDisposition>,
}

/// Builds a template health row, deriving the freshness posture, current and blocking claims, and
/// the required notes from the honest inputs so the seed is always self-consistent with the
/// resolver.
fn health_row(seed: HealthRowSeed<'_>) -> TemplateHealthRow {
    let disclosure = resolve_health_disclosure(seed.freshness_state);
    TemplateHealthRow {
        component: M5ScaffoldComponentFamily::TemplateHealthRow,
        row_id: seed.row_id.to_owned(),
        check_name: seed.check_name.to_owned(),
        signal_class: seed.signal_class,
        freshness_state: seed.freshness_state,
        severity: seed.severity,
        fix_kind: seed.fix_kind,
        derived_freshness_posture: disclosure.freshness_posture,
        claims_current: disclosure.is_current,
        claims_blocking_prerequisite: seed.severity.is_blocking_prerequisite(),
        status_label: seed.status_label.to_owned(),
        freshness_or_source_label: seed.freshness_or_source_label.to_owned(),
        stale_note: if disclosure.needs_stale_note {
            "Signal is stale or expired; rerun the check before trusting it".to_owned()
        } else {
            String::new()
        },
        never_checked_note: if disclosure.needs_never_checked_note {
            "Signal has never been checked; run it to establish a baseline".to_owned()
        } else {
            String::new()
        },
        unavailable_note: if disclosure.needs_unavailable_note {
            "Signal is unavailable on this build; treat its health as unknown".to_owned()
        } else {
            String::new()
        },
        fix_note: if seed.fix_kind.needs_fix_note() {
            seed.fix_note.to_owned()
        } else {
            String::new()
        },
        create_empty_or_continue_note:
            "Create empty or continue without a starter carries the same weight as fixing this"
                .to_owned(),
        context_note: seed.context_note.to_owned(),
        deep_link_kind: seed.deep_link_kind,
        deep_link_ref: seed.deep_link_ref.to_owned(),
        row_actions: seed.row_actions,
        dispositions: seed.dispositions,
        downgrade_triggers: health_row_downgrade_triggers(),
        required_labels: label_set(M5ScaffoldRequiredLabel::RecoveryAndOwnershipBoundary),
        surface_families: M5ScaffoldSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ScaffoldDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5ScaffoldAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5ScaffoldConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "check_name",
            "status_label",
            "freshness_or_source_label",
            "severity",
            "fix_kind",
            "signal_class",
            "freshness_state",
            "create_empty_or_continue_note",
        ]),
        source_contract_refs: health_row_source_refs(),
        hides_side_effect_behind_generic_create: false,
        hides_generated_impact_or_recovery_path: false,
        monopolizes_plain_create_without_starter_path: false,
        invents_alternate_state_label: false,
    }
}

/// The three mandatory labels plus one extra truth label.
fn label_set(extra: M5ScaffoldRequiredLabel) -> Vec<M5ScaffoldRequiredLabel> {
    let mut labels = M5ScaffoldRequiredLabel::MANDATORY.to_vec();
    labels.push(extra);
    labels
}

fn preflight_cards() -> Vec<ScaffoldPreflightCard> {
    use DeepLinkKind as Link;
    use M5PreflightCheckClass as Check;
    use M5PreflightResultState as State;
    use M5ScaffoldDisposition as Disp;
    use PreflightActionTiming as Timing;
    use PreflightCardAction as Action;
    use PreflightSideEffectKind as Side;

    vec![
        // 1. Tooling present / passed → clear; extension install deferred for later.
        preflight_card(PreflightCardSeed {
            card_id: "preflight-tooling",
            card_name: "Tooling present",
            check_class: Check::ToolingPresent,
            result_state: State::Passed,
            side_effect_kind: Side::ExtensionInstall,
            action_timing: Timing::DeferredForLater,
            side_effect_note: "Installs the recommended editor extension after create",
            target_path_label: "workspace/apps/web",
            target_name_label: "web",
            generated_file_count: 24,
            generated_folder_count: 6,
            dependency_impact_label: "18 dependencies to install",
            task_impact_label: "1 setup task",
            extension_impact_label: "1 editor extension",
            immediate_action_label: "None run immediately",
            deferred_action_label: "Extension install deferred until after create",
            recovery_path_label: "Named checkpoint before extension install",
            context_note:
                "Tooling is present; the only side effect is a deferred extension install",
            deep_link_kind: Link::TemplateManifest,
            deep_link_ref: "manifest:starters/react-spa#preflight",
            card_actions: vec![
                Action::ReviewSideEffects,
                Action::ReviewGeneratedImpact,
                Action::ReviewRecoveryPath,
                Action::RunImmediateActions,
                Action::CreateEmpty,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::FirstParty],
        }),
        // 2. Dependency availability / warning → advisory; package install requires confirmation.
        preflight_card(PreflightCardSeed {
            card_id: "preflight-dependencies",
            card_name: "Dependency availability",
            check_class: Check::DependencyAvailability,
            result_state: State::Warning,
            side_effect_kind: Side::PackageInstall,
            action_timing: Timing::RequiresConfirmation,
            side_effect_note:
                "Installs packages from the configured registry; confirm before it runs",
            target_path_label: "workspace/apps/web",
            target_name_label: "web",
            generated_file_count: 24,
            generated_folder_count: 6,
            dependency_impact_label: "18 dependencies, 2 with newer majors available",
            task_impact_label: "1 setup task",
            extension_impact_label: "No extensions",
            immediate_action_label: "None run immediately",
            deferred_action_label: "Package install after confirmation",
            recovery_path_label: "Delete generated files and retry with pinned versions",
            context_note: "Some dependencies have newer majors; review before installing",
            deep_link_kind: Link::StarterRegistryEntry,
            deep_link_ref: "registry:team/web-starter#dependencies",
            card_actions: vec![
                Action::ReviewSideEffects,
                Action::ReviewGeneratedImpact,
                Action::ReviewRecoveryPath,
                Action::CreateEmpty,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::Warning],
        }),
        // 3. Network access / blocked → blocked prerequisite; remote provisioning blocked.
        preflight_card(PreflightCardSeed {
            card_id: "preflight-network",
            card_name: "Network access",
            check_class: Check::NetworkAccess,
            result_state: State::Blocked,
            side_effect_kind: Side::RemoteProvisioning,
            action_timing: Timing::BlockedUntilResolved,
            side_effect_note:
                "Provisions a managed namespace remotely; blocked until network is allowed",
            target_path_label: "workspace/services/api",
            target_name_label: "api",
            generated_file_count: 15,
            generated_folder_count: 4,
            dependency_impact_label: "9 dependencies to install",
            task_impact_label: "2 setup tasks",
            extension_impact_label: "No extensions",
            immediate_action_label: "None can run; blocked",
            deferred_action_label: "Remote provisioning deferred until network is allowed",
            recovery_path_label: "Create empty or continue offline without remote provisioning",
            context_note: "Network is blocked; remote provisioning cannot run until it is allowed",
            deep_link_kind: Link::PolicyReference,
            deep_link_ref: "policy:workspace/network",
            card_actions: vec![
                Action::ReviewSideEffects,
                Action::ReviewGeneratedImpact,
                Action::ReviewRecoveryPath,
                Action::CreateEmpty,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::Blocked],
        }),
        // 4. Workspace writable / skipped optional → optional skipped; script execution skipped.
        preflight_card(PreflightCardSeed {
            card_id: "preflight-workspace",
            card_name: "Workspace writable",
            check_class: Check::WorkspaceWritable,
            result_state: State::SkippedOptional,
            side_effect_kind: Side::ScriptExecution,
            action_timing: Timing::NotApplicable,
            side_effect_note: "Runs an optional post-generate script; skipped this run",
            target_path_label: "workspace/apps/web",
            target_name_label: "web",
            generated_file_count: 24,
            generated_folder_count: 6,
            dependency_impact_label: "No new dependencies",
            task_impact_label: "1 optional setup task (skipped)",
            extension_impact_label: "No extensions",
            immediate_action_label: "None run immediately",
            deferred_action_label: "Optional script skipped this run",
            recovery_path_label: "Named checkpoint; rerun the script later if wanted",
            context_note: "Workspace is writable; the optional post-generate script was skipped",
            deep_link_kind: Link::DocsAnchor,
            deep_link_ref: "docs:templates/preflight-checks",
            card_actions: vec![
                Action::ReviewSideEffects,
                Action::ReviewGeneratedImpact,
                Action::ReviewRecoveryPath,
                Action::CreateEmpty,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::Optional],
        }),
        // 5. Host boundary / not run → needs attention; dependency restore runs immediately.
        preflight_card(PreflightCardSeed {
            card_id: "preflight-host-boundary",
            card_name: "Host boundary",
            check_class: Check::HostBoundary,
            result_state: State::NotRun,
            side_effect_kind: Side::DependencyRestore,
            action_timing: Timing::RunsImmediately,
            side_effect_note: "Restores dependencies into the workspace immediately on create",
            target_path_label: "workspace/apps/web",
            target_name_label: "web",
            generated_file_count: 24,
            generated_folder_count: 6,
            dependency_impact_label: "18 dependencies to restore",
            task_impact_label: "1 setup task",
            extension_impact_label: "No extensions",
            immediate_action_label: "Dependency restore runs immediately",
            deferred_action_label: "None deferred",
            recovery_path_label: "Delete generated files to undo the restore",
            context_note:
                "Host-boundary check has not run; rerun it before relying on this preflight",
            deep_link_kind: Link::DocsAnchor,
            deep_link_ref: "docs:templates/host-boundary",
            card_actions: vec![
                Action::ReviewSideEffects,
                Action::ReviewGeneratedImpact,
                Action::ReviewRecoveryPath,
                Action::RunImmediateActions,
                Action::CreateEmpty,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::Warning],
        }),
        // 6. Credential scope / unknown → needs attention; trust prompt requires confirmation.
        preflight_card(PreflightCardSeed {
            card_id: "preflight-credentials",
            card_name: "Credential scope",
            check_class: Check::CredentialScope,
            result_state: State::Unknown,
            side_effect_kind: Side::TrustPrompt,
            action_timing: Timing::RequiresConfirmation,
            side_effect_note: "Prompts for trust before the starter can use the credential scope",
            target_path_label: "workspace/services/api",
            target_name_label: "api",
            generated_file_count: 15,
            generated_folder_count: 4,
            dependency_impact_label: "9 dependencies to install",
            task_impact_label: "2 setup tasks",
            extension_impact_label: "No extensions",
            immediate_action_label: "None run immediately",
            deferred_action_label: "Trust prompt required before any credential use",
            recovery_path_label: "Create empty or continue without the credentialed starter",
            context_note: "Credential scope is unknown; a trust prompt is required before use",
            deep_link_kind: Link::PolicyReference,
            deep_link_ref: "policy:workspace/credentials",
            card_actions: vec![
                Action::ReviewSideEffects,
                Action::ReviewGeneratedImpact,
                Action::ReviewRecoveryPath,
                Action::CreateEmpty,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::Warning],
        }),
    ]
}

fn health_rows() -> Vec<TemplateHealthRow> {
    use DeepLinkKind as Link;
    use HealthFixKind as Fix;
    use HealthRowAction as Action;
    use HealthSeverity as Sev;
    use M5HealthFreshnessState as Fresh;
    use M5HealthSignalClass as Signal;
    use M5ScaffoldDisposition as Disp;

    vec![
        // 1. Build health / fresh / info / no fix → current.
        health_row(HealthRowSeed {
            row_id: "health-build",
            check_name: "Build health",
            signal_class: Signal::BuildHealth,
            freshness_state: Fresh::Fresh,
            severity: Sev::Info,
            fix_kind: Fix::NoFixNeeded,
            status_label: "Build passing",
            freshness_or_source_label: "Checked 1 hour ago from CI",
            fix_note: "",
            context_note: "Build is healthy and current; nothing to fix",
            deep_link_kind: Link::DocsAnchor,
            deep_link_ref: "docs:health/build",
            row_actions: vec![
                Action::RerunCheck,
                Action::OpenDetail,
                Action::CreateEmpty,
                Action::ContinueWithoutStarter,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::Optional],
        }),
        // 2. Dependency freshness / aging / warning / auto-fix → aging.
        health_row(HealthRowSeed {
            row_id: "health-dependencies",
            check_name: "Dependency freshness",
            signal_class: Signal::DependencyFreshness,
            freshness_state: Fresh::Aging,
            severity: Sev::Warning,
            fix_kind: Fix::AutoFixAvailable,
            status_label: "3 dependencies behind",
            freshness_or_source_label: "Checked 5 days ago from the registry",
            fix_note: "Auto-fix available: bump the 3 aging dependencies",
            context_note: "Dependencies are aging; an auto-fix can bump them",
            deep_link_kind: Link::StarterRegistryEntry,
            deep_link_ref: "registry:team/web-starter#health",
            row_actions: vec![
                Action::RerunCheck,
                Action::OpenDetail,
                Action::ApplyFix,
                Action::CreateEmpty,
                Action::ContinueWithoutStarter,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::Warning],
        }),
        // 3. Security advisories / stale / blocker / manual fix → stale-or-expired.
        health_row(HealthRowSeed {
            row_id: "health-security",
            check_name: "Security advisories",
            signal_class: Signal::SecurityAdvisories,
            freshness_state: Fresh::Stale,
            severity: Sev::Blocker,
            fix_kind: Fix::ManualFixRequired,
            status_label: "1 blocking advisory",
            freshness_or_source_label: "Last scan 40 days ago; stale",
            fix_note: "Manual fix required: patch the affected package before use",
            context_note: "A blocking advisory is present and the scan is stale; rerun and patch",
            deep_link_kind: Link::DocsAnchor,
            deep_link_ref: "docs:health/security-advisories",
            row_actions: vec![
                Action::RerunCheck,
                Action::OpenDetail,
                Action::CreateEmpty,
                Action::ContinueWithoutStarter,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::Blocked],
        }),
        // 4. Test status / expired / warning / manual fix → stale-or-expired.
        health_row(HealthRowSeed {
            row_id: "health-tests",
            check_name: "Test status",
            signal_class: Signal::TestStatus,
            freshness_state: Fresh::Expired,
            severity: Sev::Warning,
            fix_kind: Fix::ManualFixRequired,
            status_label: "Last known: 2 failing",
            freshness_or_source_label: "Result expired; rerun to refresh",
            fix_note: "Manual fix required: address the 2 failing tests",
            context_note: "Test result has expired; rerun before trusting the last status",
            deep_link_kind: Link::DocsAnchor,
            deep_link_ref: "docs:health/test-status",
            row_actions: vec![
                Action::RerunCheck,
                Action::OpenDetail,
                Action::CreateEmpty,
                Action::ContinueWithoutStarter,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::Warning],
        }),
        // 5. Maintenance cadence / never checked / info / no fix → never checked.
        health_row(HealthRowSeed {
            row_id: "health-maintenance",
            check_name: "Maintenance cadence",
            signal_class: Signal::MaintenanceCadence,
            freshness_state: Fresh::NeverChecked,
            severity: Sev::Info,
            fix_kind: Fix::NoFixNeeded,
            status_label: "Cadence unknown",
            freshness_or_source_label: "Never checked on this machine",
            fix_note: "",
            context_note: "Maintenance cadence has never been checked; run it for a baseline",
            deep_link_kind: Link::DocsAnchor,
            deep_link_ref: "docs:health/maintenance-cadence",
            row_actions: vec![
                Action::RerunCheck,
                Action::OpenDetail,
                Action::CreateEmpty,
                Action::ContinueWithoutStarter,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::Optional],
        }),
        // 6. Compatibility / unavailable / blocker / auto-fix → unavailable.
        health_row(HealthRowSeed {
            row_id: "health-compatibility",
            check_name: "Compatibility",
            signal_class: Signal::Compatibility,
            freshness_state: Fresh::Unavailable,
            severity: Sev::Blocker,
            fix_kind: Fix::AutoFixAvailable,
            status_label: "Compatibility unknown",
            freshness_or_source_label: "Signal unavailable on this build",
            fix_note: "Auto-fix available once the compatibility signal is reachable",
            context_note: "Compatibility signal is unavailable; treat its health as unknown",
            deep_link_kind: Link::DocsAnchor,
            deep_link_ref: "docs:health/compatibility",
            row_actions: vec![
                Action::RerunCheck,
                Action::OpenDetail,
                Action::ApplyFix,
                Action::CreateEmpty,
                Action::ContinueWithoutStarter,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::Blocked],
        }),
    ]
}

fn downgrade_triggers() -> Vec<M5ScaffoldDowngradeTrigger> {
    vec![
        M5ScaffoldDowngradeTrigger::SideEffectUndisclosed,
        M5ScaffoldDowngradeTrigger::HostBoundaryUnstated,
        M5ScaffoldDowngradeTrigger::ImpactUndisclosed,
        M5ScaffoldDowngradeTrigger::HealthFreshnessStale,
        M5ScaffoldDowngradeTrigger::RecoveryPathOmitted,
        M5ScaffoldDowngradeTrigger::AlternateStateLabelInvented,
        M5ScaffoldDowngradeTrigger::ProofStale,
    ]
}

fn readiness_review() -> ScaffoldReadinessReview {
    ScaffoldReadinessReview {
        preflight_card_shows_side_effects: true,
        preflight_card_shows_generated_impact: true,
        preflight_card_shows_immediate_versus_deferred: true,
        preflight_card_names_recovery_path: true,
        health_row_shows_severity_and_freshness: true,
        health_row_shows_fix_note: true,
        health_row_offers_rerun_and_open_detail: true,
        health_row_offers_create_empty_or_continue_without_starter: true,
        severity_and_freshness_derived_never_asserted: true,
        blocked_prerequisite_never_shown_as_optional: true,
        stale_signal_never_shown_as_fresh: true,
        create_never_generic_hides_side_effects: true,
        generated_file_counts_always_explicit: true,
        recovery_path_always_reachable: true,
        no_surface_invents_alternate_state_label: true,
        components_stable_across_deployment_lines: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> ScaffoldReadinessConsumerProjection {
    ScaffoldReadinessConsumerProjection {
        start_center_reads_single_source: true,
        preflight_surface_reads_single_source: true,
        health_dashboard_reads_single_source: true,
        side_effects_visible_before_commit: true,
        recovery_path_visible_before_commit: true,
        support_export_shows_component_truth: true,
    }
}

fn proof_freshness() -> ScaffoldReadinessProofFreshness {
    ScaffoldReadinessProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        SCAFFOLD_READINESS_CONTROLS_SCHEMA_REF,
        SCAFFOLD_READINESS_CONTROLS_DOC_REF,
        M5_SCAFFOLD_COMPONENT_SCHEMA_REF,
        M5_SCAFFOLD_COMPONENT_DOC_REF,
        M5_SCAFFOLD_PREFLIGHT_CARD_SCHEMA_REF,
        M5_TEMPLATE_HEALTH_ROW_SCHEMA_REF,
    ])
}

/// Builds the canonical scaffold-preflight-card / template-health-row controls packet.
pub fn seeded_scaffold_readiness_controls() -> ScaffoldPreflightCardTemplateHealthRowControlsPacket
{
    ScaffoldPreflightCardTemplateHealthRowControlsPacket::new(
        ScaffoldPreflightCardTemplateHealthRowControlsPacketInput {
            packet_id: SCAFFOLD_READINESS_CONTROLS_PACKET_ID.to_owned(),
            surface_label:
                "M5 scaffold preflight cards and template health rows: generated file counts, immediate-versus-deferred actions, blocked/warning/optional checks, and create-empty parity across claimed bootstrap surfaces"
                    .to_owned(),
            preflight_cards: preflight_cards(),
            health_rows: health_rows(),
            downgrade_triggers: downgrade_triggers(),
            consumer_surfaces: M5ScaffoldConsumerSurface::ALL.to_vec(),
            readiness_review: readiness_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Scenario fixture: spotlights a blocked network / remote-provisioning preflight card that must
/// never hide its side effect under a generic Create. Every check class, result state, real
/// side-effect kind, and severity stays covered so the fixture validates on its own.
pub fn seeded_scaffold_readiness_controls_preflight_card_blocked(
) -> ScaffoldPreflightCardTemplateHealthRowControlsPacket {
    let mut packet = seeded_scaffold_readiness_controls();
    packet.packet_id =
        "m5-scaffold-preflight-card-template-health-row-controls:fixture:preflight-card-blocked"
            .to_owned();
    packet.surface_label =
        "M5 scaffold preflight cards: a blocked remote-provisioning check never hides under a generic Create"
            .to_owned();
    packet
}

/// Scenario fixture: spotlights a stale security-advisory template health row that must keep a
/// same-weight Create-empty / Continue-without-starter path. Every signal class, freshness state,
/// severity, and freshness posture stays covered so the fixture validates on its own.
pub fn seeded_scaffold_readiness_controls_health_row_stale(
) -> ScaffoldPreflightCardTemplateHealthRowControlsPacket {
    let mut packet = seeded_scaffold_readiness_controls();
    packet.packet_id =
        "m5-scaffold-preflight-card-template-health-row-controls:fixture:health-row-stale"
            .to_owned();
    packet.surface_label =
        "M5 template health rows: a stale blocking advisory never monopolizes the plain create-without-starter path"
            .to_owned();
    packet
}
