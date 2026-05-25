//! Canonical stable truth model for first-run onboarding: no-account local
//! entry, setup-later posture, and repair-safe recovery cues.
//!
//! ## Why one first-run record, not three loose flows
//!
//! The very first launch on a fresh device has to answer three questions at
//! once, and a switching user judges the product on all three:
//!
//! 1. **Can I start working right now, without an account?** Useful local work
//!    — open a folder, open a file, clone a repository, start a scratch file —
//!    must be reachable with no sign-in, no managed service, and no blocking
//!    account-nag. The local-first promise is launch-bearing.
//! 2. **Can I do the setup later?** Sign-in, workspace trust, recommended
//!    extensions, appearance / keymap, an AI provider, a remote or managed
//!    connection, and an import from another editor are all *offered* but
//!    *deferrable*. Deferring one must never block first-useful-work, and it
//!    must never quietly widen trust, install packages, apply a workflow
//!    bundle, or skip a required checkpoint.
//! 3. **What happens if first-run state is damaged?** An unreadable settings
//!    store, a half-finished migration, a missing locale pack, or a profile
//!    written by a newer build must degrade *safely*: local work stays
//!    reachable, the user's own files are never destroyed, nothing is silently
//!    reset, and the surface routes to a real repair action — never a dead end.
//!
//! When each first-run surface answers these on its own, they drift: a tour
//! card implies an account is required, a deferred step silently grants trust, a
//! corrupt settings file dead-ends the launch, or a "reset to defaults" quietly
//! deletes the user's keymap. This module mints one governed
//! [`FirstRunOnboardingRecord`] that the desktop shell, command palette, menus,
//! diagnostics, support exports, Help/About, and docs all read verbatim instead
//! of cloning status text.
//!
//! The record is the canonical truth source for this lane (suggested-output stem
//! `finalize-first-run-onboarding-with-no-account-local`); its boundary schema
//! is
//! `schemas/ux/finalize-first-run-onboarding-with-no-account-local.schema.json`
//! and its contract narrative is
//! `docs/ux/m4/finalize-first-run-onboarding-with-no-account-local.md`. It is the
//! stable consumer of the upstream first-run no-account contract
//! `docs/ux/no_account_local_entry_contract.md`; it does not re-mint that
//! vocabulary, it projects it onto a governed stable record.
//!
//! ## The honesty invariants
//!
//! The builder refuses to mint a record that would lie. Each is a [`BuildError`],
//! not a warning, so a dishonest projection fails the row instead of shipping:
//!
//! - **No-account local entry.** Local work must be available at first run with
//!   no account and no managed service required, and at least one entry verb
//!   must be reachable without an account.
//! - **Setup-later posture.** No offered or deferred setup step may block
//!   first-useful-work, and deferring a step may never widen workspace trust,
//!   install packages, apply a workflow bundle, or suppress a required
//!   checkpoint. Every step keeps a resume route so it can be finished later.
//! - **Repair-safe recovery.** Every repair cue preserves the user's authored
//!   work, never dead-ends, never silently resets, routes through the
//!   `metadata_safe_default` redaction class, and carries an export-safe
//!   chain of custody (a `doctor.finding.*` finding code, a
//!   `repair_transaction:*` id, and a checkpoint ref) so support, docs, and
//!   shiproom packets reference the same truth. A `needs_repair` record must
//!   surface at least one repair cue; a `healthy` record must surface none.
//! - **Durable, accessible truth.** First-run truth is durable, not toast-only,
//!   and never theme-only; the entry surfaces (Start Center, command palette,
//!   menu) are keyboard-reachable and the flow stays reachable in normal,
//!   high-contrast, and zoomed layouts.
//! - **No-account landing.** First-useful-work routes to a keyboard-reachable,
//!   non-destructive landing that does not require an account.
//!
//! ## What never crosses this boundary
//!
//! Raw paths, raw command lines, raw URLs, raw tokens, and raw user content
//! never appear on these records. Surfaces carry opaque object refs
//! (`aureline://<class>/<id>`), the canonical `doctor.finding.*` and
//! `repair_transaction:*` ids, opaque checkpoint refs, and short reviewable
//! sentences only.

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried in serialized first-run records.
pub const FIRST_RUN_ONBOARDING_RECORD_KIND: &str = "first_run_onboarding_record";

/// Schema version for the [`FirstRunOnboardingRecord`] payload shape.
pub const FIRST_RUN_ONBOARDING_SCHEMA_VERSION: u32 = 1;

/// The single redaction class every repair cue must route through, matching the
/// support / project-doctor convention.
pub const REQUIRED_REDACTION_CLASS: &str = "metadata_safe_default";

/// Reviewer-facing notice rendered on every first-run surface.
pub const FIRST_RUN_ONBOARDING_NOTICE: &str =
    "First-run onboarding truth: the first launch makes useful local work reachable with no account \
     and no managed service; sign-in, trust, extensions, appearance/keymap, an AI provider, a \
     remote/managed connection, and editor imports are offered but deferrable and never block \
     first-useful-work or silently widen trust, install packages, apply a workflow bundle, or skip \
     a required checkpoint; damaged first-run state degrades safely with repair cues that preserve \
     your files, never dead-end, never silently reset, and carry an export-safe chain of custody; \
     and first-run truth is durable, keyboard-reachable, and never toast-only or theme-only. Shell, \
     command palette, menus, diagnostics, support exports, Help/About, and docs read this record \
     verbatim.";

/// Canonical durable-object URI scheme.
pub const CANONICAL_OBJECT_SCHEME: &str = "aureline://";

/// Prefix every repair-transaction ref carries (support / doctor convention).
pub const REPAIR_TRANSACTION_PREFIX: &str = "repair_transaction:";

/// Prefix every doctor finding code carries.
pub const DOCTOR_FINDING_PREFIX: &str = "doctor.finding.";

/// Upper bound on a reviewable explanation sentence.
const MAX_SENTENCE_CHARS: usize = 1024;
/// Upper bound on any stable id / ref.
const MAX_REF_CHARS: usize = 200;

/// Object-class segments that are generic landing destinations rather than a
/// specific durable object. A ref pointing at one of these is rejected so the
/// chrome cannot wire an affordance to a dashboard home.
const GENERIC_LANDING_CLASSES: &[&str] =
    &["home", "dashboard", "landing", "index", "overview", "root"];

/// Returns true when `reference` is a canonical durable-object ref of the form
/// `aureline://<class>/<id>` where `<class>` is not a generic landing page.
pub fn is_canonical_object_ref(reference: &str) -> bool {
    let reference = reference.trim();
    if reference.is_empty() || reference.len() > MAX_REF_CHARS {
        return false;
    }
    let Some(rest) = reference.strip_prefix(CANONICAL_OBJECT_SCHEME) else {
        return false;
    };
    let Some((class, ident)) = rest.split_once('/') else {
        return false;
    };
    if class.is_empty() || ident.is_empty() {
        return false;
    }
    !GENERIC_LANDING_CLASSES.contains(&class)
}

/// Returns true when `code` is a canonical project-doctor finding code of the
/// form `doctor.finding.<segment>(.<segment>)*` with lowercase tokens.
pub fn is_doctor_finding_code(code: &str) -> bool {
    let code = code.trim();
    if code.is_empty() || code.len() > MAX_REF_CHARS {
        return false;
    }
    let Some(rest) = code.strip_prefix(DOCTOR_FINDING_PREFIX) else {
        return false;
    };
    is_dotted_lower_segments(rest)
}

/// Returns true when `reference` is a canonical repair-transaction ref of the
/// form `repair_transaction:<family>.<reason>` with lowercase tokens.
pub fn is_repair_transaction_ref(reference: &str) -> bool {
    let reference = reference.trim();
    if reference.is_empty() || reference.len() > MAX_REF_CHARS {
        return false;
    }
    let Some(rest) = reference.strip_prefix(REPAIR_TRANSACTION_PREFIX) else {
        return false;
    };
    // Require at least a `<family>.<reason>` shape.
    rest.contains('.') && is_dotted_lower_segments(rest)
}

/// Returns true when `id` is an opaque, export-safe stable id: non-empty,
/// bounded, free of whitespace, and free of raw scheme / path leakage.
pub fn is_opaque_id(id: &str) -> bool {
    let id = id.trim();
    if id.is_empty() || id.len() > MAX_REF_CHARS {
        return false;
    }
    if id.contains("://") || id.contains("http") {
        return false;
    }
    !id.chars().any(|c| c.is_whitespace())
}

fn is_dotted_lower_segments(rest: &str) -> bool {
    if rest.is_empty() {
        return false;
    }
    rest.split('.').all(|segment| {
        !segment.is_empty()
            && segment
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-')
            && segment
                .chars()
                .any(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
    })
}

fn is_reviewable_sentence(sentence: &str) -> bool {
    let trimmed = sentence.trim();
    !trimmed.is_empty() && trimmed.len() <= MAX_SENTENCE_CHARS
}

// ---------------------------------------------------------------------------
// Scenario class
// ---------------------------------------------------------------------------

/// The first-run drill this record represents. Closed set; surfaces MUST NOT
/// invent classes outside it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FirstRunScenarioClass {
    /// Clean first run on a healthy fresh device.
    CleanFirstRun,
    /// First run where the user deferred every offered setup step.
    SetupDeferredLocalOnly,
    /// First run where an import and keymap step completed during onboarding.
    SetupCompletedWithImport,
    /// First run whose settings / keymap / appearance store was unreadable.
    DegradedSettingsStore,
    /// First run whose migration finished only partially.
    NeedsRepairPartialMigration,
    /// First run whose locale pack was missing and fell back to the base locale.
    MissingLocalePack,
    /// First run whose profile was written by a newer, incompatible build.
    NewerProfileIncompatible,
}

impl FirstRunScenarioClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CleanFirstRun => "clean_first_run",
            Self::SetupDeferredLocalOnly => "setup_deferred_local_only",
            Self::SetupCompletedWithImport => "setup_completed_with_import",
            Self::DegradedSettingsStore => "degraded_settings_store",
            Self::NeedsRepairPartialMigration => "needs_repair_partial_migration",
            Self::MissingLocalePack => "missing_locale_pack",
            Self::NewerProfileIncompatible => "newer_profile_incompatible",
        }
    }

    /// Human-readable label, quoted verbatim across surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CleanFirstRun => "Clean first run",
            Self::SetupDeferredLocalOnly => "Setup deferred, local only",
            Self::SetupCompletedWithImport => "Setup completed with import",
            Self::DegradedSettingsStore => "Degraded settings store",
            Self::NeedsRepairPartialMigration => "Needs repair: partial migration",
            Self::MissingLocalePack => "Missing locale pack",
            Self::NewerProfileIncompatible => "Newer, incompatible profile",
        }
    }
}

// ---------------------------------------------------------------------------
// No-account local entry
// ---------------------------------------------------------------------------

/// An entry verb that is reachable at first run without an account. Closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryVerbClass {
    OpenLocalFolder,
    OpenLocalFile,
    CloneRepository,
    NewScratchFile,
    OpenSampleProject,
    OpenRecent,
}

impl EntryVerbClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenLocalFolder => "open_local_folder",
            Self::OpenLocalFile => "open_local_file",
            Self::CloneRepository => "clone_repository",
            Self::NewScratchFile => "new_scratch_file",
            Self::OpenSampleProject => "open_sample_project",
            Self::OpenRecent => "open_recent",
        }
    }

    /// All entry verbs, for vocabulary coverage.
    pub const ALL: &'static [EntryVerbClass] = &[
        Self::OpenLocalFolder,
        Self::OpenLocalFile,
        Self::CloneRepository,
        Self::NewScratchFile,
        Self::OpenSampleProject,
        Self::OpenRecent,
    ];
}

/// The no-account local-entry posture at first run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryMode {
    /// Always false; the builder rejects a true value.
    pub account_required_for_local_work: bool,
    /// Always false; the builder rejects a true value.
    pub managed_services_required_for_local_work: bool,
    /// Always true; the builder rejects a false value.
    pub local_work_available: bool,
    /// Entry verbs reachable without an account; must be non-empty.
    pub entry_verbs: Vec<EntryVerbClass>,
}

// ---------------------------------------------------------------------------
// Setup-later posture
// ---------------------------------------------------------------------------

/// A setup step that is offered at first run but deferrable. Closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupStepClass {
    SignIn,
    GrantWorkspaceTrust,
    InstallRecommendedExtensions,
    ChooseAppearanceAndKeymap,
    ConfigureAiProvider,
    ConnectRemoteOrManaged,
    ImportFromOtherEditor,
}

impl SetupStepClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignIn => "sign_in",
            Self::GrantWorkspaceTrust => "grant_workspace_trust",
            Self::InstallRecommendedExtensions => "install_recommended_extensions",
            Self::ChooseAppearanceAndKeymap => "choose_appearance_and_keymap",
            Self::ConfigureAiProvider => "configure_ai_provider",
            Self::ConnectRemoteOrManaged => "connect_remote_or_managed",
            Self::ImportFromOtherEditor => "import_from_other_editor",
        }
    }

    /// All setup steps, for vocabulary coverage.
    pub const ALL: &'static [SetupStepClass] = &[
        Self::SignIn,
        Self::GrantWorkspaceTrust,
        Self::InstallRecommendedExtensions,
        Self::ChooseAppearanceAndKeymap,
        Self::ConfigureAiProvider,
        Self::ConnectRemoteOrManaged,
        Self::ImportFromOtherEditor,
    ];
}

/// Where a setup step stands at the end of first run. Closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupStepPosture {
    /// Offered, not started; the user can defer or do it.
    OfferedDeferrable,
    /// The user chose to set it up later.
    Deferred,
    /// The user completed it during first run.
    CompletedDuringFirstRun,
}

impl SetupStepPosture {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OfferedDeferrable => "offered_deferrable",
            Self::Deferred => "deferred",
            Self::CompletedDuringFirstRun => "completed_during_first_run",
        }
    }

    /// Whether this posture represents work the user has not yet done (so the
    /// bounded-deferral guarantees must hold).
    pub const fn is_outstanding(self) -> bool {
        matches!(self, Self::OfferedDeferrable | Self::Deferred)
    }
}

/// One offered-but-deferrable setup step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetupStep {
    pub step: SetupStepClass,
    pub posture: SetupStepPosture,
    /// Always false; the builder rejects a true value (setup never blocks work).
    pub blocks_first_useful_work: bool,
    /// Canonical ref of the route that finishes this step later.
    pub resume_route_ref: String,
    /// All four must be false; deferring may influence nothing but routing.
    pub widens_trust_on_defer: bool,
    pub installs_packages_on_defer: bool,
    pub applies_workflow_bundle_on_defer: bool,
    pub suppresses_required_checkpoint_on_defer: bool,
    /// Short reviewable detail sentence.
    pub detail: String,
}

// ---------------------------------------------------------------------------
// Repair-safe recovery cues
// ---------------------------------------------------------------------------

/// First-run health. Closed set, in worsening order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FirstRunHealthClass {
    /// Everything first-run needs is present and readable.
    Healthy,
    /// Something is narrowed but local work proceeds; a repair is offered.
    Degraded,
    /// A first-run resource needs an explicit repair before it is trusted.
    NeedsRepair,
}

impl FirstRunHealthClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Degraded => "degraded",
            Self::NeedsRepair => "needs_repair",
        }
    }

    /// Whether this health class must surface at least one repair cue.
    pub const fn requires_repair_cue(self) -> bool {
        matches!(self, Self::Degraded | Self::NeedsRepair)
    }
}

/// A first-run resource a repair cue can refer to. Closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FirstRunResourceClass {
    SettingsStore,
    KeymapProfile,
    AppearanceProfile,
    MigrationState,
    LocalePack,
    OnboardingState,
    WorkspaceProfile,
}

impl FirstRunResourceClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SettingsStore => "settings_store",
            Self::KeymapProfile => "keymap_profile",
            Self::AppearanceProfile => "appearance_profile",
            Self::MigrationState => "migration_state",
            Self::LocalePack => "locale_pack",
            Self::OnboardingState => "onboarding_state",
            Self::WorkspaceProfile => "workspace_profile",
        }
    }

    /// All resources, for vocabulary coverage.
    pub const ALL: &'static [FirstRunResourceClass] = &[
        Self::SettingsStore,
        Self::KeymapProfile,
        Self::AppearanceProfile,
        Self::MigrationState,
        Self::LocalePack,
        Self::OnboardingState,
        Self::WorkspaceProfile,
    ];
}

/// One repair-safe recovery cue with an export-safe chain of custody.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairCue {
    pub resource: FirstRunResourceClass,
    /// Stable `doctor.finding.*` code shared with support / docs / shiproom.
    pub finding_code: String,
    /// Stable `repair_transaction:<family>.<reason>` id.
    pub repair_transaction_ref: String,
    /// Redaction class; must be `metadata_safe_default`.
    pub redaction_class: String,
    /// Opaque, export-safe checkpoint id (chain of custody).
    pub checkpoint_ref: String,
    /// Canonical ref of the keyboard-reachable repair route.
    pub repair_route_ref: String,
    /// Always true; repair never destroys user-authored work.
    pub preserves_user_work: bool,
    /// Always false; a cue must offer a real route, never a dead end.
    pub dead_end: bool,
    /// Always false; a cue must never silently reset state.
    pub silent_reset: bool,
    /// Short reviewable detail sentence.
    pub detail: String,
}

// ---------------------------------------------------------------------------
// Accessibility / route parity
// ---------------------------------------------------------------------------

/// A surface that exposes the first-run entry routes. Closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntrySurfaceClass {
    StartCenter,
    CommandPalette,
    MenuCommand,
}

impl EntrySurfaceClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartCenter => "start_center",
            Self::CommandPalette => "command_palette",
            Self::MenuCommand => "menu_command",
        }
    }

    /// All entry surfaces, for vocabulary coverage.
    pub const ALL: &'static [EntrySurfaceClass] =
        &[Self::StartCenter, Self::CommandPalette, Self::MenuCommand];
}

/// One entry-surface route, keyboard-reachable and pointing at first-run entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntrySurface {
    pub surface: EntrySurfaceClass,
    /// Canonical ref of the route that opens first-run entry on this surface.
    pub route_ref: String,
    /// Always true; the builder rejects a false value.
    pub keyboard_reachable: bool,
}

/// The first-run accessibility disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityDisclosure {
    /// Every interactive affordance reachable by keyboard.
    pub keyboard_complete: bool,
    /// A defined, exact focus order.
    pub focus_order_defined: bool,
    /// Reachable under a high-contrast theme.
    pub high_contrast_reachable: bool,
    /// Reachable under OS / app zoom.
    pub zoom_reachable: bool,
    /// The entry surfaces that share the same routes.
    pub entry_surfaces: Vec<EntrySurface>,
}

// ---------------------------------------------------------------------------
// First-useful-work landing
// ---------------------------------------------------------------------------

/// Where first-useful-work routes after first run. Closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FirstUsefulWorkLandingClass {
    LocalWorkspace,
    Readme,
    SampleProject,
    EmptyEditor,
    StartCenter,
}

impl FirstUsefulWorkLandingClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalWorkspace => "local_workspace",
            Self::Readme => "readme",
            Self::SampleProject => "sample_project",
            Self::EmptyEditor => "empty_editor",
            Self::StartCenter => "start_center",
        }
    }

    /// All landing classes, for vocabulary coverage.
    pub const ALL: &'static [FirstUsefulWorkLandingClass] = &[
        Self::LocalWorkspace,
        Self::Readme,
        Self::SampleProject,
        Self::EmptyEditor,
        Self::StartCenter,
    ];
}

/// The first-useful-work landing after first run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirstUsefulWorkLanding {
    pub landing: FirstUsefulWorkLandingClass,
    /// Canonical ref of the landing target.
    pub target_ref: String,
    /// Always true; the builder rejects a false value.
    pub keyboard_reachable: bool,
    /// Always false; the builder rejects a true value.
    pub destructive: bool,
    /// Always false; the builder rejects a true value.
    pub requires_account: bool,
    /// Short reviewable detail sentence.
    pub detail: String,
}

// ---------------------------------------------------------------------------
// Input + derived record
// ---------------------------------------------------------------------------

/// Everything a caller supplies to build a [`FirstRunOnboardingRecord`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirstRunOnboardingInput {
    pub record_id: String,
    pub as_of: String,
    pub scenario: FirstRunScenarioClass,
    pub title: String,
    pub summary: String,
    pub entry: EntryMode,
    pub setup_steps: Vec<SetupStep>,
    pub health: FirstRunHealthClass,
    pub repair_cues: Vec<RepairCue>,
    pub accessibility: AccessibilityDisclosure,
    pub landing: FirstUsefulWorkLanding,
    pub diagnostics_export_ref: String,
    pub support_export_ref: String,
    pub evidence_refs: Vec<String>,
    pub narrative_refs: Vec<String>,
}

/// Derived rollup counts surfaced on the record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FirstRunSummaryCounts {
    pub entry_verb_count: u32,
    pub setup_step_count: u32,
    pub offered_step_count: u32,
    pub deferred_step_count: u32,
    pub completed_step_count: u32,
    pub repair_cue_count: u32,
    pub entry_surface_count: u32,
}

/// The "no lie" display-copy invariants. Every field must be false on a minted
/// record; the builder enforces it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FirstRunDisplayCopy {
    /// First-run implied an account was required to do local work.
    pub account_implied_for_local_work: bool,
    /// First-run implied a managed service was required to do local work.
    pub managed_services_implied_for_local_work: bool,
    /// A setup step blocked first-useful-work.
    pub setup_blocked_first_useful_work: bool,
    /// Deferring a setup step silently widened trust or ran setup.
    pub setup_deferral_widened_trust: bool,
    /// A repair destroyed user-authored work.
    pub repair_destroyed_user_work: bool,
    /// A repair cue dead-ended with no route.
    pub repair_dead_ended: bool,
    /// A repair silently reset state.
    pub repair_silently_reset: bool,
    /// First-run truth was carried only by a transient toast.
    pub toast_only_truth: bool,
    /// First-run state was encoded only by theme / color.
    pub theme_only_semantics: bool,
}

/// The canonical, governed first-run onboarding record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirstRunOnboardingRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub notice: String,
    pub record_id: String,
    pub as_of: String,
    pub scenario: FirstRunScenarioClass,
    pub scenario_label: String,
    pub title: String,
    pub summary: String,
    pub entry: EntryMode,
    pub setup_steps: Vec<SetupStep>,
    pub health: FirstRunHealthClass,
    pub repair_cues: Vec<RepairCue>,
    pub accessibility: AccessibilityDisclosure,
    pub landing: FirstUsefulWorkLanding,
    /// True when there is anything narrowed to disclose (non-healthy state, a
    /// deferred step, or a repair cue).
    pub honesty_marker_present: bool,
    pub summary_counts: FirstRunSummaryCounts,
    pub display_copy: FirstRunDisplayCopy,
    pub diagnostics_export_ref: String,
    pub support_export_ref: String,
    pub evidence_refs: Vec<String>,
    pub narrative_refs: Vec<String>,
}

/// Reasons a [`FirstRunOnboardingRecord`] cannot honestly be minted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// A field that must be a reviewable sentence was empty or too long.
    InvalidSentence { field: &'static str },
    /// A field that must be a canonical object ref was not.
    NonCanonicalRef { field: &'static str, value: String },
    /// A repair finding code was not a canonical `doctor.finding.*` code.
    InvalidFindingCode { value: String },
    /// A repair-transaction ref was not a canonical `repair_transaction:*` id.
    InvalidRepairTransactionRef { value: String },
    /// A checkpoint ref was not an opaque, export-safe id.
    InvalidCheckpointRef { value: String },
    /// A repair cue used a redaction class other than the required default.
    RepairRedactionNotSafe { value: String },
    /// Local work claimed to require an account.
    LocalWorkRequiresAccount,
    /// Local work claimed to require a managed service.
    LocalWorkRequiresManagedServices,
    /// Local work was not available at first run.
    LocalWorkUnavailable,
    /// No account-free entry verb was offered.
    NoAccountFreeEntryVerb,
    /// A setup step blocked first-useful-work.
    SetupStepBlocksFirstUsefulWork { step: SetupStepClass },
    /// Deferring a setup step widened trust or ran setup.
    SetupDeferralWidensTrust { step: SetupStepClass },
    /// A repair cue destroyed user-authored work.
    RepairDestroysUserWork { resource: FirstRunResourceClass },
    /// A repair cue dead-ended with no route.
    RepairCueDeadEnd { resource: FirstRunResourceClass },
    /// A repair cue silently reset state.
    RepairCueSilentReset { resource: FirstRunResourceClass },
    /// A non-healthy record surfaced no repair cue.
    MissingRepairCue { health: FirstRunHealthClass },
    /// A healthy record surfaced a repair cue.
    HealthyRecordWithRepairCue,
    /// An accessibility guarantee was not met.
    AccessibilityIncomplete { aspect: &'static str },
    /// An entry surface route was not keyboard reachable.
    EntrySurfaceNotKeyboardReachable { surface: EntrySurfaceClass },
    /// The required Start Center entry surface was missing.
    MissingStartCenterSurface,
    /// The first-useful-work landing was not keyboard reachable.
    LandingNotKeyboardReachable,
    /// The first-useful-work landing was destructive.
    LandingDestructive,
    /// The first-useful-work landing required an account.
    LandingRequiresAccount,
}

impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidSentence { field } => {
                write!(f, "field `{field}` must be a non-empty reviewable sentence")
            }
            Self::NonCanonicalRef { field, value } => {
                write!(
                    f,
                    "field `{field}` must be a canonical object ref, got {value:?}"
                )
            }
            Self::InvalidFindingCode { value } => {
                write!(
                    f,
                    "repair finding code {value:?} must match `doctor.finding.*`"
                )
            }
            Self::InvalidRepairTransactionRef { value } => write!(
                f,
                "repair-transaction ref {value:?} must match `repair_transaction:<family>.<reason>`"
            ),
            Self::InvalidCheckpointRef { value } => {
                write!(
                    f,
                    "checkpoint ref {value:?} must be an opaque, export-safe id"
                )
            }
            Self::RepairRedactionNotSafe { value } => write!(
                f,
                "repair redaction class {value:?} must be `{REQUIRED_REDACTION_CLASS}`"
            ),
            Self::LocalWorkRequiresAccount => {
                write!(f, "first-run local work must not require an account")
            }
            Self::LocalWorkRequiresManagedServices => {
                write!(f, "first-run local work must not require a managed service")
            }
            Self::LocalWorkUnavailable => {
                write!(f, "first-run local work must be available")
            }
            Self::NoAccountFreeEntryVerb => {
                write!(
                    f,
                    "first run must offer at least one account-free entry verb"
                )
            }
            Self::SetupStepBlocksFirstUsefulWork { step } => write!(
                f,
                "setup step `{}` must not block first-useful-work",
                step.as_str()
            ),
            Self::SetupDeferralWidensTrust { step } => write!(
                f,
                "deferring setup step `{}` must not widen trust, install packages, apply a bundle, \
                 or suppress a checkpoint",
                step.as_str()
            ),
            Self::RepairDestroysUserWork { resource } => write!(
                f,
                "repair cue for `{}` must preserve user-authored work",
                resource.as_str()
            ),
            Self::RepairCueDeadEnd { resource } => write!(
                f,
                "repair cue for `{}` must offer a route, not a dead end",
                resource.as_str()
            ),
            Self::RepairCueSilentReset { resource } => write!(
                f,
                "repair cue for `{}` must not silently reset state",
                resource.as_str()
            ),
            Self::MissingRepairCue { health } => write!(
                f,
                "`{}` first-run state must surface at least one repair cue",
                health.as_str()
            ),
            Self::HealthyRecordWithRepairCue => {
                write!(
                    f,
                    "a healthy first-run record must not surface a repair cue"
                )
            }
            Self::AccessibilityIncomplete { aspect } => {
                write!(
                    f,
                    "first-run accessibility guarantee `{aspect}` was not met"
                )
            }
            Self::EntrySurfaceNotKeyboardReachable { surface } => write!(
                f,
                "entry surface `{}` must be keyboard reachable",
                surface.as_str()
            ),
            Self::MissingStartCenterSurface => {
                write!(f, "first run must expose entry through the Start Center")
            }
            Self::LandingNotKeyboardReachable => {
                write!(f, "first-useful-work landing must be keyboard reachable")
            }
            Self::LandingDestructive => {
                write!(f, "first-useful-work landing must be non-destructive")
            }
            Self::LandingRequiresAccount => {
                write!(f, "first-useful-work landing must not require an account")
            }
        }
    }
}

impl std::error::Error for BuildError {}

impl FirstRunOnboardingRecord {
    /// Build a governed first-run onboarding record from validated input.
    ///
    /// Returns a [`BuildError`] when the input would mint a record that lies
    /// about no-account local entry, setup-later posture, repair safety,
    /// accessibility, or the first-useful-work landing.
    pub fn build(input: FirstRunOnboardingInput) -> Result<Self, BuildError> {
        // --- text / ref validation -------------------------------------------
        if !is_reviewable_sentence(&input.title) {
            return Err(BuildError::InvalidSentence { field: "title" });
        }
        if !is_reviewable_sentence(&input.summary) {
            return Err(BuildError::InvalidSentence { field: "summary" });
        }
        require_ref("diagnostics_export_ref", &input.diagnostics_export_ref)?;
        require_ref("support_export_ref", &input.support_export_ref)?;
        for r in &input.evidence_refs {
            require_ref("evidence_refs[]", r)?;
        }
        for r in &input.narrative_refs {
            require_ref("narrative_refs[]", r)?;
        }

        // --- no-account local entry ------------------------------------------
        if input.entry.account_required_for_local_work {
            return Err(BuildError::LocalWorkRequiresAccount);
        }
        if input.entry.managed_services_required_for_local_work {
            return Err(BuildError::LocalWorkRequiresManagedServices);
        }
        if !input.entry.local_work_available {
            return Err(BuildError::LocalWorkUnavailable);
        }
        if input.entry.entry_verbs.is_empty() {
            return Err(BuildError::NoAccountFreeEntryVerb);
        }

        // --- setup-later posture ---------------------------------------------
        let mut offered = 0u32;
        let mut deferred = 0u32;
        let mut completed = 0u32;
        for step in &input.setup_steps {
            if !is_reviewable_sentence(&step.detail) {
                return Err(BuildError::InvalidSentence {
                    field: "setup_step.detail",
                });
            }
            require_ref("setup_step.resume_route_ref", &step.resume_route_ref)?;
            if step.blocks_first_useful_work {
                return Err(BuildError::SetupStepBlocksFirstUsefulWork { step: step.step });
            }
            if step.posture.is_outstanding()
                && (step.widens_trust_on_defer
                    || step.installs_packages_on_defer
                    || step.applies_workflow_bundle_on_defer
                    || step.suppresses_required_checkpoint_on_defer)
            {
                return Err(BuildError::SetupDeferralWidensTrust { step: step.step });
            }
            match step.posture {
                SetupStepPosture::OfferedDeferrable => offered += 1,
                SetupStepPosture::Deferred => deferred += 1,
                SetupStepPosture::CompletedDuringFirstRun => completed += 1,
            }
        }

        // --- repair-safe recovery cues ---------------------------------------
        for cue in &input.repair_cues {
            if !is_reviewable_sentence(&cue.detail) {
                return Err(BuildError::InvalidSentence {
                    field: "repair_cue.detail",
                });
            }
            if !is_doctor_finding_code(&cue.finding_code) {
                return Err(BuildError::InvalidFindingCode {
                    value: cue.finding_code.clone(),
                });
            }
            if !is_repair_transaction_ref(&cue.repair_transaction_ref) {
                return Err(BuildError::InvalidRepairTransactionRef {
                    value: cue.repair_transaction_ref.clone(),
                });
            }
            if !is_opaque_id(&cue.checkpoint_ref) {
                return Err(BuildError::InvalidCheckpointRef {
                    value: cue.checkpoint_ref.clone(),
                });
            }
            if cue.redaction_class != REQUIRED_REDACTION_CLASS {
                return Err(BuildError::RepairRedactionNotSafe {
                    value: cue.redaction_class.clone(),
                });
            }
            require_ref("repair_cue.repair_route_ref", &cue.repair_route_ref)?;
            if !cue.preserves_user_work {
                return Err(BuildError::RepairDestroysUserWork {
                    resource: cue.resource,
                });
            }
            if cue.dead_end {
                return Err(BuildError::RepairCueDeadEnd {
                    resource: cue.resource,
                });
            }
            if cue.silent_reset {
                return Err(BuildError::RepairCueSilentReset {
                    resource: cue.resource,
                });
            }
        }
        if input.health == FirstRunHealthClass::Healthy && !input.repair_cues.is_empty() {
            return Err(BuildError::HealthyRecordWithRepairCue);
        }
        if input.health.requires_repair_cue() && input.repair_cues.is_empty() {
            return Err(BuildError::MissingRepairCue {
                health: input.health,
            });
        }

        // --- accessibility / route parity ------------------------------------
        if !input.accessibility.keyboard_complete {
            return Err(BuildError::AccessibilityIncomplete {
                aspect: "keyboard_complete",
            });
        }
        if !input.accessibility.focus_order_defined {
            return Err(BuildError::AccessibilityIncomplete {
                aspect: "focus_order_defined",
            });
        }
        if !input.accessibility.high_contrast_reachable {
            return Err(BuildError::AccessibilityIncomplete {
                aspect: "high_contrast_reachable",
            });
        }
        if !input.accessibility.zoom_reachable {
            return Err(BuildError::AccessibilityIncomplete {
                aspect: "zoom_reachable",
            });
        }
        for surface in &input.accessibility.entry_surfaces {
            require_ref("entry_surface.route_ref", &surface.route_ref)?;
            if !surface.keyboard_reachable {
                return Err(BuildError::EntrySurfaceNotKeyboardReachable {
                    surface: surface.surface,
                });
            }
        }
        if !input
            .accessibility
            .entry_surfaces
            .iter()
            .any(|s| s.surface == EntrySurfaceClass::StartCenter)
        {
            return Err(BuildError::MissingStartCenterSurface);
        }

        // --- first-useful-work landing ---------------------------------------
        require_ref("landing.target_ref", &input.landing.target_ref)?;
        if !is_reviewable_sentence(&input.landing.detail) {
            return Err(BuildError::InvalidSentence {
                field: "landing.detail",
            });
        }
        if !input.landing.keyboard_reachable {
            return Err(BuildError::LandingNotKeyboardReachable);
        }
        if input.landing.destructive {
            return Err(BuildError::LandingDestructive);
        }
        if input.landing.requires_account {
            return Err(BuildError::LandingRequiresAccount);
        }

        // --- derived rollups --------------------------------------------------
        let summary_counts = FirstRunSummaryCounts {
            entry_verb_count: input.entry.entry_verbs.len() as u32,
            setup_step_count: input.setup_steps.len() as u32,
            offered_step_count: offered,
            deferred_step_count: deferred,
            completed_step_count: completed,
            repair_cue_count: input.repair_cues.len() as u32,
            entry_surface_count: input.accessibility.entry_surfaces.len() as u32,
        };

        let honesty_marker_present = input.health != FirstRunHealthClass::Healthy
            || deferred > 0
            || !input.repair_cues.is_empty();

        // Every condition that would set a display-copy invariant true has been
        // rejected above. Modelling them explicitly keeps the "no lie" contract
        // inspectable on the record.
        let display_copy = FirstRunDisplayCopy::default();

        Ok(Self {
            record_kind: FIRST_RUN_ONBOARDING_RECORD_KIND.to_string(),
            schema_version: FIRST_RUN_ONBOARDING_SCHEMA_VERSION,
            notice: FIRST_RUN_ONBOARDING_NOTICE.to_string(),
            record_id: input.record_id,
            as_of: input.as_of,
            scenario: input.scenario,
            scenario_label: input.scenario.label().to_string(),
            title: input.title,
            summary: input.summary,
            entry: input.entry,
            setup_steps: input.setup_steps,
            health: input.health,
            repair_cues: input.repair_cues,
            accessibility: input.accessibility,
            landing: input.landing,
            honesty_marker_present,
            summary_counts,
            display_copy,
            diagnostics_export_ref: input.diagnostics_export_ref,
            support_export_ref: input.support_export_ref,
            evidence_refs: input.evidence_refs,
            narrative_refs: input.narrative_refs,
        })
    }

    /// Deterministic plaintext truth block for diagnostics / support export.
    ///
    /// Stable, redaction-safe (refs and stable ids only), and quotable verbatim
    /// by the support bundle and the diagnostics packet.
    pub fn support_export_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!("first_run_onboarding: {}", self.record_id),
            format!(
                "scenario: {} ({})",
                self.scenario.as_str(),
                self.scenario_label
            ),
            format!("as_of: {}", self.as_of),
            format!("title: {}", self.title),
            format!("summary: {}", self.summary),
            format!("health: {}", self.health.as_str()),
            format!(
                "entry: account_required={} managed_required={} local_work_available={}",
                self.entry.account_required_for_local_work,
                self.entry.managed_services_required_for_local_work,
                self.entry.local_work_available,
            ),
            format!(
                "entry_verbs: {}",
                self.entry
                    .entry_verbs
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
            "setup_steps:".to_string(),
        ];
        for step in &self.setup_steps {
            lines.push(format!(
                "  - {} posture={} blocks_first_useful_work={} resume={}",
                step.step.as_str(),
                step.posture.as_str(),
                step.blocks_first_useful_work,
                step.resume_route_ref,
            ));
        }
        if self.repair_cues.is_empty() {
            lines.push("repair_cues: none".to_string());
        } else {
            lines.push("repair_cues:".to_string());
            for cue in &self.repair_cues {
                lines.push(format!(
                    "  - {} finding={} repair_transaction={} checkpoint={} redaction={} \
                     preserves_user_work={} dead_end={} silent_reset={} route={}",
                    cue.resource.as_str(),
                    cue.finding_code,
                    cue.repair_transaction_ref,
                    cue.checkpoint_ref,
                    cue.redaction_class,
                    cue.preserves_user_work,
                    cue.dead_end,
                    cue.silent_reset,
                    cue.repair_route_ref,
                ));
            }
        }
        lines.push(format!(
            "landing: class={} target={} keyboard={} destructive={} requires_account={}",
            self.landing.landing.as_str(),
            self.landing.target_ref,
            self.landing.keyboard_reachable,
            self.landing.destructive,
            self.landing.requires_account,
        ));
        lines.push(format!(
            "accessibility: keyboard_complete={} focus_order={} high_contrast={} zoom={}",
            self.accessibility.keyboard_complete,
            self.accessibility.focus_order_defined,
            self.accessibility.high_contrast_reachable,
            self.accessibility.zoom_reachable,
        ));
        for surface in &self.accessibility.entry_surfaces {
            lines.push(format!(
                "  surface {} route={} keyboard={}",
                surface.surface.as_str(),
                surface.route_ref,
                surface.keyboard_reachable,
            ));
        }
        lines.push(format!(
            "honesty_marker_present: {}",
            self.honesty_marker_present
        ));
        lines.push(format!(
            "diagnostics_export_ref: {}",
            self.diagnostics_export_ref
        ));
        lines.push(format!("support_export_ref: {}", self.support_export_ref));
        lines
    }
}

fn require_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_canonical_object_ref(value) {
        Ok(())
    } else {
        Err(BuildError::NonCanonicalRef {
            field,
            value: value.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A minimal honest input the negative tests mutate to trip one invariant.
    fn honest_input() -> FirstRunOnboardingInput {
        FirstRunOnboardingInput {
            record_id: "first_run_onboarding:test.base".to_owned(),
            as_of: "2026-05-25T12:00:00Z".to_owned(),
            scenario: FirstRunScenarioClass::CleanFirstRun,
            title: "Start working locally, no account needed.".to_owned(),
            summary: "Open a folder right away; sign-in and setup can wait.".to_owned(),
            entry: EntryMode {
                account_required_for_local_work: false,
                managed_services_required_for_local_work: false,
                local_work_available: true,
                entry_verbs: vec![
                    EntryVerbClass::OpenLocalFolder,
                    EntryVerbClass::NewScratchFile,
                ],
            },
            setup_steps: vec![SetupStep {
                step: SetupStepClass::SignIn,
                posture: SetupStepPosture::OfferedDeferrable,
                blocks_first_useful_work: false,
                resume_route_ref: "aureline://command/account.sign_in".to_owned(),
                widens_trust_on_defer: false,
                installs_packages_on_defer: false,
                applies_workflow_bundle_on_defer: false,
                suppresses_required_checkpoint_on_defer: false,
                detail: "Sign in later from the account menu.".to_owned(),
            }],
            health: FirstRunHealthClass::Healthy,
            repair_cues: vec![],
            accessibility: AccessibilityDisclosure {
                keyboard_complete: true,
                focus_order_defined: true,
                high_contrast_reachable: true,
                zoom_reachable: true,
                entry_surfaces: vec![EntrySurface {
                    surface: EntrySurfaceClass::StartCenter,
                    route_ref: "aureline://command/start_center.open".to_owned(),
                    keyboard_reachable: true,
                }],
            },
            landing: FirstUsefulWorkLanding {
                landing: FirstUsefulWorkLandingClass::EmptyEditor,
                target_ref: "aureline://editor/scratch".to_owned(),
                keyboard_reachable: true,
                destructive: false,
                requires_account: false,
                detail: "Landed on an empty editor ready for local work.".to_owned(),
            },
            diagnostics_export_ref: "aureline://diagnostics/test".to_owned(),
            support_export_ref: "aureline://support_export/test".to_owned(),
            evidence_refs: vec![],
            narrative_refs: vec![],
        }
    }

    fn safe_cue(resource: FirstRunResourceClass) -> RepairCue {
        RepairCue {
            resource,
            finding_code: "doctor.finding.settings_store_unreadable".to_owned(),
            repair_transaction_ref:
                "repair_transaction:disposable_state_rebuild.settings_store_repair".to_owned(),
            redaction_class: REQUIRED_REDACTION_CLASS.to_owned(),
            checkpoint_ref: "checkpoint:first_run.settings_store".to_owned(),
            repair_route_ref: "aureline://command/settings.repair".to_owned(),
            preserves_user_work: true,
            dead_end: false,
            silent_reset: false,
            detail: "Rebuild the settings store from defaults; your files are untouched."
                .to_owned(),
        }
    }

    #[test]
    fn honest_input_builds_and_has_no_honesty_marker() {
        let record = FirstRunOnboardingRecord::build(honest_input()).expect("honest input builds");
        assert_eq!(record.record_kind, FIRST_RUN_ONBOARDING_RECORD_KIND);
        assert!(!record.honesty_marker_present);
        assert_eq!(record.summary_counts.entry_verb_count, 2);
    }

    #[test]
    fn account_required_for_local_work_is_rejected() {
        let mut input = honest_input();
        input.entry.account_required_for_local_work = true;
        let err = FirstRunOnboardingRecord::build(input).unwrap_err();
        assert_eq!(err, BuildError::LocalWorkRequiresAccount);
    }

    #[test]
    fn managed_services_required_is_rejected() {
        let mut input = honest_input();
        input.entry.managed_services_required_for_local_work = true;
        let err = FirstRunOnboardingRecord::build(input).unwrap_err();
        assert_eq!(err, BuildError::LocalWorkRequiresManagedServices);
    }

    #[test]
    fn no_entry_verb_is_rejected() {
        let mut input = honest_input();
        input.entry.entry_verbs.clear();
        let err = FirstRunOnboardingRecord::build(input).unwrap_err();
        assert_eq!(err, BuildError::NoAccountFreeEntryVerb);
    }

    #[test]
    fn setup_step_blocking_first_useful_work_is_rejected() {
        let mut input = honest_input();
        input.setup_steps[0].blocks_first_useful_work = true;
        let err = FirstRunOnboardingRecord::build(input).unwrap_err();
        assert_eq!(
            err,
            BuildError::SetupStepBlocksFirstUsefulWork {
                step: SetupStepClass::SignIn
            }
        );
    }

    #[test]
    fn deferral_widening_trust_is_rejected() {
        let mut input = honest_input();
        input.setup_steps[0].posture = SetupStepPosture::Deferred;
        input.setup_steps[0].widens_trust_on_defer = true;
        let err = FirstRunOnboardingRecord::build(input).unwrap_err();
        assert_eq!(
            err,
            BuildError::SetupDeferralWidensTrust {
                step: SetupStepClass::SignIn
            }
        );
    }

    #[test]
    fn completed_step_may_carry_setup_side_effects() {
        // A completed step legitimately installed packages; the bounded-deferral
        // guard only applies to outstanding steps.
        let mut input = honest_input();
        input.setup_steps[0].step = SetupStepClass::InstallRecommendedExtensions;
        input.setup_steps[0].posture = SetupStepPosture::CompletedDuringFirstRun;
        input.setup_steps[0].installs_packages_on_defer = true;
        let record = FirstRunOnboardingRecord::build(input).expect("completed step builds");
        assert_eq!(record.summary_counts.completed_step_count, 1);
    }

    #[test]
    fn repair_destroying_user_work_is_rejected() {
        let mut input = honest_input();
        input.health = FirstRunHealthClass::Degraded;
        let mut cue = safe_cue(FirstRunResourceClass::SettingsStore);
        cue.preserves_user_work = false;
        input.repair_cues.push(cue);
        let err = FirstRunOnboardingRecord::build(input).unwrap_err();
        assert_eq!(
            err,
            BuildError::RepairDestroysUserWork {
                resource: FirstRunResourceClass::SettingsStore
            }
        );
    }

    #[test]
    fn repair_dead_end_is_rejected() {
        let mut input = honest_input();
        input.health = FirstRunHealthClass::Degraded;
        let mut cue = safe_cue(FirstRunResourceClass::SettingsStore);
        cue.dead_end = true;
        input.repair_cues.push(cue);
        let err = FirstRunOnboardingRecord::build(input).unwrap_err();
        assert_eq!(
            err,
            BuildError::RepairCueDeadEnd {
                resource: FirstRunResourceClass::SettingsStore
            }
        );
    }

    #[test]
    fn repair_silent_reset_is_rejected() {
        let mut input = honest_input();
        input.health = FirstRunHealthClass::Degraded;
        let mut cue = safe_cue(FirstRunResourceClass::SettingsStore);
        cue.silent_reset = true;
        input.repair_cues.push(cue);
        let err = FirstRunOnboardingRecord::build(input).unwrap_err();
        assert_eq!(
            err,
            BuildError::RepairCueSilentReset {
                resource: FirstRunResourceClass::SettingsStore
            }
        );
    }

    #[test]
    fn repair_redaction_must_be_safe_default() {
        let mut input = honest_input();
        input.health = FirstRunHealthClass::Degraded;
        let mut cue = safe_cue(FirstRunResourceClass::SettingsStore);
        cue.redaction_class = "raw".to_owned();
        input.repair_cues.push(cue);
        let err = FirstRunOnboardingRecord::build(input).unwrap_err();
        assert!(matches!(err, BuildError::RepairRedactionNotSafe { .. }));
    }

    #[test]
    fn repair_finding_code_must_be_canonical() {
        let mut input = honest_input();
        input.health = FirstRunHealthClass::Degraded;
        let mut cue = safe_cue(FirstRunResourceClass::SettingsStore);
        cue.finding_code = "settings_broken".to_owned();
        input.repair_cues.push(cue);
        let err = FirstRunOnboardingRecord::build(input).unwrap_err();
        assert!(matches!(err, BuildError::InvalidFindingCode { .. }));
    }

    #[test]
    fn repair_transaction_ref_must_be_canonical() {
        let mut input = honest_input();
        input.health = FirstRunHealthClass::Degraded;
        let mut cue = safe_cue(FirstRunResourceClass::SettingsStore);
        cue.repair_transaction_ref = "fix-it".to_owned();
        input.repair_cues.push(cue);
        let err = FirstRunOnboardingRecord::build(input).unwrap_err();
        assert!(matches!(
            err,
            BuildError::InvalidRepairTransactionRef { .. }
        ));
    }

    #[test]
    fn needs_repair_without_cue_is_rejected() {
        let mut input = honest_input();
        input.health = FirstRunHealthClass::NeedsRepair;
        let err = FirstRunOnboardingRecord::build(input).unwrap_err();
        assert_eq!(
            err,
            BuildError::MissingRepairCue {
                health: FirstRunHealthClass::NeedsRepair
            }
        );
    }

    #[test]
    fn healthy_with_cue_is_rejected() {
        let mut input = honest_input();
        input
            .repair_cues
            .push(safe_cue(FirstRunResourceClass::SettingsStore));
        let err = FirstRunOnboardingRecord::build(input).unwrap_err();
        assert_eq!(err, BuildError::HealthyRecordWithRepairCue);
    }

    #[test]
    fn missing_start_center_surface_is_rejected() {
        let mut input = honest_input();
        input.accessibility.entry_surfaces[0].surface = EntrySurfaceClass::CommandPalette;
        let err = FirstRunOnboardingRecord::build(input).unwrap_err();
        assert_eq!(err, BuildError::MissingStartCenterSurface);
    }

    #[test]
    fn incomplete_keyboard_is_rejected() {
        let mut input = honest_input();
        input.accessibility.keyboard_complete = false;
        let err = FirstRunOnboardingRecord::build(input).unwrap_err();
        assert_eq!(
            err,
            BuildError::AccessibilityIncomplete {
                aspect: "keyboard_complete"
            }
        );
    }

    #[test]
    fn landing_requiring_account_is_rejected() {
        let mut input = honest_input();
        input.landing.requires_account = true;
        let err = FirstRunOnboardingRecord::build(input).unwrap_err();
        assert_eq!(err, BuildError::LandingRequiresAccount);
    }

    #[test]
    fn destructive_landing_is_rejected() {
        let mut input = honest_input();
        input.landing.destructive = true;
        let err = FirstRunOnboardingRecord::build(input).unwrap_err();
        assert_eq!(err, BuildError::LandingDestructive);
    }

    #[test]
    fn non_canonical_ref_is_rejected() {
        let mut input = honest_input();
        input.landing.target_ref = "https://example.com/x".to_owned();
        let err = FirstRunOnboardingRecord::build(input).unwrap_err();
        assert!(matches!(err, BuildError::NonCanonicalRef { .. }));
    }

    #[test]
    fn finding_and_transaction_validators_accept_canonical_forms() {
        assert!(is_doctor_finding_code(
            "doctor.finding.cache_schema_version_drift"
        ));
        assert!(!is_doctor_finding_code("doctor.finding."));
        assert!(!is_doctor_finding_code("doctor.warning.x"));
        assert!(is_repair_transaction_ref(
            "repair_transaction:disposable_state_rebuild.cache_index_repair"
        ));
        assert!(!is_repair_transaction_ref("repair_transaction:nofamily"));
        assert!(is_opaque_id("checkpoint:first_run.settings_store"));
        assert!(!is_opaque_id("https://x/y"));
        assert!(!is_opaque_id("has space"));
    }
}
