//! Cross-surface first-run onboarding drill corpus.
//!
//! ## Why a corpus, not a single seeded record
//!
//! The model in [`crate::first_run_onboarding::model`] proves the no-account
//! local-entry, setup-later, repair-safe, and accessibility invariants in
//! isolation. The stable-claim grade is about the shell, command palette, menus,
//! diagnostics, support exports, Help/About, and docs *agreeing* on what one
//! first run means across the cases the acceptance criteria name. This corpus
//! mints one [`FirstRunOnboardingScenario`] per named drill and pins each
//! rendered [`FirstRunOnboardingRecord`] bit-for-bit on disk under
//! `fixtures/ux/m4/finalize-first-run-onboarding-with-no-account-local/`, so a
//! regression in the no-account rule, the setup-later rule, the repair-safe rule,
//! the accessibility rule, or the landing rule fails the fixture-replay test
//! instead of shipping silently.
//!
//! The drills deliberately exercise the clean first run, every setup posture
//! (offered, deferred, completed), all three health classes (healthy, degraded,
//! needs-repair), every first-run resource that can be unhealthy, every entry
//! verb, every entry surface, and every landing class.

use super::model::{
    AccessibilityDisclosure, EntryMode, EntrySurface, EntrySurfaceClass, EntryVerbClass,
    FirstRunHealthClass, FirstRunOnboardingInput, FirstRunOnboardingRecord, FirstRunResourceClass,
    FirstRunScenarioClass, FirstUsefulWorkLanding, FirstUsefulWorkLandingClass, RepairCue,
    SetupStep, SetupStepClass, SetupStepPosture, REQUIRED_REDACTION_CLASS,
};

/// Stable `as_of` instant the whole corpus is evaluated against. Pinned so the
/// on-disk fixtures stay deterministic.
pub const CORPUS_AS_OF: &str = "2026-05-25T12:00:00Z";

/// Stable record-id prefix shared by every scenario.
pub const CORPUS_RECORD_ID_PREFIX: &str = "first_run_onboarding:m4.stable.corpus.";

/// One drill. Surfaces under review MUST reproduce the same record projection
/// bit-for-bit; the test in
/// `crates/aureline-shell/tests/first_run_onboarding_fixtures.rs` pins each
/// scenario against the on-disk fixture.
#[derive(Clone)]
pub struct FirstRunOnboardingScenario {
    /// Stable identifier, quoted in the matrix, the report, and the doc.
    pub scenario_id: &'static str,
    /// Stable human-readable label.
    pub scenario_label: &'static str,
    /// One-sentence narrative the report and matrix quote.
    pub narrative: &'static str,
    /// On-disk fixture filename (relative to the corpus fixture dir).
    pub fixture_filename: &'static str,
    /// Expected scenario class.
    pub expected_scenario: FirstRunScenarioClass,
    /// Expected health class.
    pub expected_health: FirstRunHealthClass,
    /// Expected landing class.
    pub expected_landing: FirstUsefulWorkLandingClass,
    /// Expected honesty-marker value.
    pub expected_honesty_marker_present: bool,
    /// Expected deferred-step count.
    pub expected_deferred_step_count: u32,
    /// Expected repair-cue count.
    pub expected_repair_cue_count: u32,
    input: FirstRunOnboardingInput,
}

impl FirstRunOnboardingScenario {
    /// Build the rendered record for this scenario. The corpus inputs are
    /// deterministic and honest, so a build failure is a bug.
    pub fn record(&self) -> FirstRunOnboardingRecord {
        FirstRunOnboardingRecord::build(self.input.clone())
            .expect("first-run onboarding corpus scenario must build")
    }
}

// --------------------------------------------------------------------------- //
// Compact constructors
// --------------------------------------------------------------------------- //

#[allow(clippy::too_many_arguments)]
fn step(
    step: SetupStepClass,
    posture: SetupStepPosture,
    resume_route_ref: &str,
    detail: &str,
) -> SetupStep {
    SetupStep {
        step,
        posture,
        blocks_first_useful_work: false,
        resume_route_ref: resume_route_ref.to_owned(),
        widens_trust_on_defer: false,
        installs_packages_on_defer: false,
        applies_workflow_bundle_on_defer: false,
        suppresses_required_checkpoint_on_defer: false,
        detail: detail.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn cue(
    resource: FirstRunResourceClass,
    finding_code: &str,
    repair_transaction_ref: &str,
    checkpoint_ref: &str,
    repair_route_ref: &str,
    detail: &str,
) -> RepairCue {
    RepairCue {
        resource,
        finding_code: finding_code.to_owned(),
        repair_transaction_ref: repair_transaction_ref.to_owned(),
        redaction_class: REQUIRED_REDACTION_CLASS.to_owned(),
        checkpoint_ref: checkpoint_ref.to_owned(),
        repair_route_ref: repair_route_ref.to_owned(),
        preserves_user_work: true,
        dead_end: false,
        silent_reset: false,
        detail: detail.to_owned(),
    }
}

fn surface(surface: EntrySurfaceClass, route_ref: &str) -> EntrySurface {
    EntrySurface {
        surface,
        route_ref: route_ref.to_owned(),
        keyboard_reachable: true,
    }
}

/// The standard, fully-reachable accessibility disclosure shared by the corpus.
/// Every scenario exposes the same three keyboard-reachable entry surfaces.
fn standard_accessibility() -> AccessibilityDisclosure {
    AccessibilityDisclosure {
        keyboard_complete: true,
        focus_order_defined: true,
        high_contrast_reachable: true,
        zoom_reachable: true,
        entry_surfaces: vec![
            surface(
                EntrySurfaceClass::StartCenter,
                "aureline://command/start_center.open",
            ),
            surface(
                EntrySurfaceClass::CommandPalette,
                "aureline://command/palette.first_run_entry",
            ),
            surface(
                EntrySurfaceClass::MenuCommand,
                "aureline://command/menu.open_first_run_entry",
            ),
        ],
    }
}

/// The account-free entry mode every scenario shares.
fn local_entry(entry_verbs: Vec<EntryVerbClass>) -> EntryMode {
    EntryMode {
        account_required_for_local_work: false,
        managed_services_required_for_local_work: false,
        local_work_available: true,
        entry_verbs,
    }
}

fn landing(
    landing: FirstUsefulWorkLandingClass,
    target_ref: &str,
    detail: &str,
) -> FirstUsefulWorkLanding {
    FirstUsefulWorkLanding {
        landing,
        target_ref: target_ref.to_owned(),
        keyboard_reachable: true,
        destructive: false,
        requires_account: false,
        detail: detail.to_owned(),
    }
}

fn record_id(scenario_id: &str) -> String {
    format!("{CORPUS_RECORD_ID_PREFIX}{scenario_id}")
}

fn narrative_refs() -> Vec<String> {
    vec![
        "aureline://doc/finalize_first_run_onboarding_with_no_account_local".to_owned(),
        "aureline://contract/no_account_local_entry".to_owned(),
    ]
}

// --------------------------------------------------------------------------- //
// The corpus
// --------------------------------------------------------------------------- //

/// The full ordered drill corpus.
pub fn first_run_onboarding_corpus() -> Vec<FirstRunOnboardingScenario> {
    vec![
        clean_first_run(),
        setup_deferred_local_only(),
        setup_completed_with_import(),
        degraded_settings_store(),
        needs_repair_partial_migration(),
        missing_locale_pack(),
        newer_profile_incompatible(),
    ]
}

fn clean_first_run() -> FirstRunOnboardingScenario {
    let input = FirstRunOnboardingInput {
        record_id: record_id("clean_first_run"),
        as_of: CORPUS_AS_OF.to_owned(),
        scenario: FirstRunScenarioClass::CleanFirstRun,
        title: "Start working locally right now — no account required.".to_owned(),
        summary: "A fresh device opens to account-free local entry; sign-in and the rest of setup \
                  are offered but you can do them later."
            .to_owned(),
        entry: local_entry(vec![
            EntryVerbClass::OpenLocalFolder,
            EntryVerbClass::OpenLocalFile,
            EntryVerbClass::NewScratchFile,
        ]),
        setup_steps: vec![
            step(
                SetupStepClass::SignIn,
                SetupStepPosture::OfferedDeferrable,
                "aureline://command/account.sign_in",
                "Sign in any time from the account menu; local work never needs it.",
            ),
            step(
                SetupStepClass::ChooseAppearanceAndKeymap,
                SetupStepPosture::OfferedDeferrable,
                "aureline://command/settings.appearance_and_keymap",
                "Pick a theme and keymap now or keep the defaults and change them later.",
            ),
            step(
                SetupStepClass::ConfigureAiProvider,
                SetupStepPosture::OfferedDeferrable,
                "aureline://command/ai.configure_provider",
                "Connect an AI provider later; nothing is configured by default.",
            ),
        ],
        health: FirstRunHealthClass::Healthy,
        repair_cues: vec![],
        accessibility: standard_accessibility(),
        landing: landing(
            FirstUsefulWorkLandingClass::EmptyEditor,
            "aureline://editor/scratch",
            "Landed on an empty editor ready for local work without an account.",
        ),
        diagnostics_export_ref: "aureline://diagnostics/clean_first_run".to_owned(),
        support_export_ref: "aureline://support_export/clean_first_run".to_owned(),
        evidence_refs: vec!["aureline://trace/clean_first_run".to_owned()],
        narrative_refs: narrative_refs(),
    };
    FirstRunOnboardingScenario {
        scenario_id: "clean_first_run",
        scenario_label: "Clean first run",
        narrative:
            "A healthy fresh device opens to account-free local entry with every setup step \
                    offered but deferrable.",
        fixture_filename: "clean_first_run.json",
        expected_scenario: FirstRunScenarioClass::CleanFirstRun,
        expected_health: FirstRunHealthClass::Healthy,
        expected_landing: FirstUsefulWorkLandingClass::EmptyEditor,
        expected_honesty_marker_present: false,
        expected_deferred_step_count: 0,
        expected_repair_cue_count: 0,
        input,
    }
}

fn setup_deferred_local_only() -> FirstRunOnboardingScenario {
    let input = FirstRunOnboardingInput {
        record_id: record_id("setup_deferred_local_only"),
        as_of: CORPUS_AS_OF.to_owned(),
        scenario: FirstRunScenarioClass::SetupDeferredLocalOnly,
        title: "You skipped setup and went straight to your folder.".to_owned(),
        summary: "Sign-in, workspace trust, recommended extensions, and a remote connection are \
                  all deferred; each keeps a resume route and none widened trust."
            .to_owned(),
        entry: local_entry(vec![
            EntryVerbClass::OpenLocalFolder,
            EntryVerbClass::OpenRecent,
        ]),
        setup_steps: vec![
            step(
                SetupStepClass::SignIn,
                SetupStepPosture::Deferred,
                "aureline://command/account.sign_in",
                "Deferred sign-in; resume from the account menu when you want it.",
            ),
            step(
                SetupStepClass::GrantWorkspaceTrust,
                SetupStepPosture::Deferred,
                "aureline://command/workspace.review_trust",
                "Deferred trust review; the folder opened in a restricted, read-safe mode.",
            ),
            step(
                SetupStepClass::InstallRecommendedExtensions,
                SetupStepPosture::Deferred,
                "aureline://command/extensions.review_recommended",
                "Deferred recommended extensions; nothing was installed for you.",
            ),
            step(
                SetupStepClass::ConnectRemoteOrManaged,
                SetupStepPosture::Deferred,
                "aureline://command/remote.connect",
                "Deferred remote / managed connection; local work needs none of it.",
            ),
        ],
        health: FirstRunHealthClass::Healthy,
        repair_cues: vec![],
        accessibility: standard_accessibility(),
        landing: landing(
            FirstUsefulWorkLandingClass::LocalWorkspace,
            "aureline://workspace/local_folder",
            "Landed in the local folder you opened, with deferred setup parked.",
        ),
        diagnostics_export_ref: "aureline://diagnostics/setup_deferred_local_only".to_owned(),
        support_export_ref: "aureline://support_export/setup_deferred_local_only".to_owned(),
        evidence_refs: vec!["aureline://trace/setup_deferred_local_only".to_owned()],
        narrative_refs: narrative_refs(),
    };
    FirstRunOnboardingScenario {
        scenario_id: "setup_deferred_local_only",
        scenario_label: "Setup deferred, local only",
        narrative: "Deferring sign-in, trust, extensions, and a remote connection never blocks \
                    local work and never widens trust; each keeps a resume route.",
        fixture_filename: "setup_deferred_local_only.json",
        expected_scenario: FirstRunScenarioClass::SetupDeferredLocalOnly,
        expected_health: FirstRunHealthClass::Healthy,
        expected_landing: FirstUsefulWorkLandingClass::LocalWorkspace,
        expected_honesty_marker_present: true,
        expected_deferred_step_count: 4,
        expected_repair_cue_count: 0,
        input,
    }
}

fn setup_completed_with_import() -> FirstRunOnboardingScenario {
    let input = FirstRunOnboardingInput {
        record_id: record_id("setup_completed_with_import"),
        as_of: CORPUS_AS_OF.to_owned(),
        scenario: FirstRunScenarioClass::SetupCompletedWithImport,
        title: "Imported your settings and picked a keymap during setup.".to_owned(),
        summary: "You completed an import from another editor and chose appearance and keymap; \
                  sign-in stays deferred and local work is ready."
            .to_owned(),
        entry: local_entry(vec![
            EntryVerbClass::OpenLocalFolder,
            EntryVerbClass::CloneRepository,
        ]),
        setup_steps: vec![
            step(
                SetupStepClass::ImportFromOtherEditor,
                SetupStepPosture::CompletedDuringFirstRun,
                "aureline://command/import.rerun",
                "Imported settings and keybindings from another editor, reviewed before applying.",
            ),
            step(
                SetupStepClass::ChooseAppearanceAndKeymap,
                SetupStepPosture::CompletedDuringFirstRun,
                "aureline://command/settings.appearance_and_keymap",
                "Chose a theme and keymap during setup; change them any time.",
            ),
            step(
                SetupStepClass::SignIn,
                SetupStepPosture::Deferred,
                "aureline://command/account.sign_in",
                "Left sign-in for later; the import needed no account.",
            ),
        ],
        health: FirstRunHealthClass::Healthy,
        repair_cues: vec![],
        accessibility: standard_accessibility(),
        landing: landing(
            FirstUsefulWorkLandingClass::LocalWorkspace,
            "aureline://workspace/imported_local",
            "Landed in your local workspace with imported settings applied.",
        ),
        diagnostics_export_ref: "aureline://diagnostics/setup_completed_with_import".to_owned(),
        support_export_ref: "aureline://support_export/setup_completed_with_import".to_owned(),
        evidence_refs: vec!["aureline://trace/setup_completed_with_import".to_owned()],
        narrative_refs: narrative_refs(),
    };
    FirstRunOnboardingScenario {
        scenario_id: "setup_completed_with_import",
        scenario_label: "Setup completed with import",
        narrative: "Completing an import and keymap choice during first run is account-free, and \
                    sign-in still defers cleanly.",
        fixture_filename: "setup_completed_with_import.json",
        expected_scenario: FirstRunScenarioClass::SetupCompletedWithImport,
        expected_health: FirstRunHealthClass::Healthy,
        expected_landing: FirstUsefulWorkLandingClass::LocalWorkspace,
        expected_honesty_marker_present: true,
        expected_deferred_step_count: 1,
        expected_repair_cue_count: 0,
        input,
    }
}

fn degraded_settings_store() -> FirstRunOnboardingScenario {
    let input = FirstRunOnboardingInput {
        record_id: record_id("degraded_settings_store"),
        as_of: CORPUS_AS_OF.to_owned(),
        scenario: FirstRunScenarioClass::DegradedSettingsStore,
        title: "Your settings were unreadable, so defaults are in use.".to_owned(),
        summary: "The settings, keymap, and appearance stores could not be read; first run fell \
                  back to safe defaults and offers a repair that keeps your files."
            .to_owned(),
        entry: local_entry(vec![
            EntryVerbClass::OpenLocalFolder,
            EntryVerbClass::OpenSampleProject,
        ]),
        setup_steps: vec![step(
            SetupStepClass::ChooseAppearanceAndKeymap,
            SetupStepPosture::OfferedDeferrable,
            "aureline://command/settings.appearance_and_keymap",
            "Re-pick a theme and keymap, or repair the stored ones below.",
        )],
        health: FirstRunHealthClass::Degraded,
        repair_cues: vec![
            cue(
                FirstRunResourceClass::SettingsStore,
                "doctor.finding.settings_store_unreadable",
                "repair_transaction:disposable_state_rebuild.settings_store_repair",
                "checkpoint:first_run.settings_store",
                "aureline://command/settings.repair_store",
                "Rebuild the settings store from defaults; your files stay untouched.",
            ),
            cue(
                FirstRunResourceClass::KeymapProfile,
                "doctor.finding.keymap_profile_unreadable",
                "repair_transaction:disposable_state_rebuild.keymap_profile_repair",
                "checkpoint:first_run.keymap_profile",
                "aureline://command/settings.repair_keymap",
                "Restore the keymap profile to the default preset and re-import later.",
            ),
            cue(
                FirstRunResourceClass::AppearanceProfile,
                "doctor.finding.appearance_profile_unreadable",
                "repair_transaction:disposable_state_rebuild.appearance_profile_repair",
                "checkpoint:first_run.appearance_profile",
                "aureline://command/settings.repair_appearance",
                "Reset the appearance profile to the default theme; re-choose any time.",
            ),
        ],
        accessibility: standard_accessibility(),
        landing: landing(
            FirstUsefulWorkLandingClass::Readme,
            "aureline://readme/first_run_repair",
            "Landed on a README explaining the safe-default fallback and how to repair.",
        ),
        diagnostics_export_ref: "aureline://diagnostics/degraded_settings_store".to_owned(),
        support_export_ref: "aureline://support_export/degraded_settings_store".to_owned(),
        evidence_refs: vec!["aureline://trace/degraded_settings_store".to_owned()],
        narrative_refs: narrative_refs(),
    };
    FirstRunOnboardingScenario {
        scenario_id: "degraded_settings_store",
        scenario_label: "Degraded settings store",
        narrative:
            "An unreadable settings / keymap / appearance store falls back to safe defaults \
                    and offers repairs that preserve the user's files.",
        fixture_filename: "degraded_settings_store.json",
        expected_scenario: FirstRunScenarioClass::DegradedSettingsStore,
        expected_health: FirstRunHealthClass::Degraded,
        expected_landing: FirstUsefulWorkLandingClass::Readme,
        expected_honesty_marker_present: true,
        expected_deferred_step_count: 0,
        expected_repair_cue_count: 3,
        input,
    }
}

fn needs_repair_partial_migration() -> FirstRunOnboardingScenario {
    let input = FirstRunOnboardingInput {
        record_id: record_id("needs_repair_partial_migration"),
        as_of: CORPUS_AS_OF.to_owned(),
        scenario: FirstRunScenarioClass::NeedsRepairPartialMigration,
        title: "A migration finished only partially and needs review.".to_owned(),
        summary: "Migration of your prior profile stopped midway; local work is reachable and the \
                  partial migration and onboarding state are held for an explicit repair."
            .to_owned(),
        entry: local_entry(vec![
            EntryVerbClass::OpenLocalFolder,
            EntryVerbClass::OpenRecent,
        ]),
        setup_steps: vec![step(
            SetupStepClass::ImportFromOtherEditor,
            SetupStepPosture::OfferedDeferrable,
            "aureline://command/import.review",
            "Review or rerun the import once the partial migration is repaired.",
        )],
        health: FirstRunHealthClass::NeedsRepair,
        repair_cues: vec![
            cue(
                FirstRunResourceClass::MigrationState,
                "doctor.finding.migration_partially_applied",
                "repair_transaction:execution_context_reresolve.migration_resume",
                "checkpoint:first_run.migration_state",
                "aureline://command/migration.resume",
                "Resume the partial migration from its checkpoint; nothing is discarded.",
            ),
            cue(
                FirstRunResourceClass::OnboardingState,
                "doctor.finding.onboarding_state_inconsistent",
                "repair_transaction:disposable_state_rebuild.onboarding_state_repair",
                "checkpoint:first_run.onboarding_state",
                "aureline://command/onboarding.repair_state",
                "Rebuild onboarding state from the resumed migration; your files are untouched.",
            ),
        ],
        accessibility: standard_accessibility(),
        landing: landing(
            FirstUsefulWorkLandingClass::LocalWorkspace,
            "aureline://workspace/local_folder",
            "Landed in your local folder while the partial migration waits for repair.",
        ),
        diagnostics_export_ref: "aureline://diagnostics/needs_repair_partial_migration".to_owned(),
        support_export_ref: "aureline://support_export/needs_repair_partial_migration".to_owned(),
        evidence_refs: vec!["aureline://trace/needs_repair_partial_migration".to_owned()],
        narrative_refs: narrative_refs(),
    };
    FirstRunOnboardingScenario {
        scenario_id: "needs_repair_partial_migration",
        scenario_label: "Needs repair: partial migration",
        narrative: "A half-finished migration keeps local work reachable and holds the migration \
                    and onboarding state for an explicit, non-destructive repair.",
        fixture_filename: "needs_repair_partial_migration.json",
        expected_scenario: FirstRunScenarioClass::NeedsRepairPartialMigration,
        expected_health: FirstRunHealthClass::NeedsRepair,
        expected_landing: FirstUsefulWorkLandingClass::LocalWorkspace,
        expected_honesty_marker_present: true,
        expected_deferred_step_count: 0,
        expected_repair_cue_count: 2,
        input,
    }
}

fn missing_locale_pack() -> FirstRunOnboardingScenario {
    let input = FirstRunOnboardingInput {
        record_id: record_id("missing_locale_pack"),
        as_of: CORPUS_AS_OF.to_owned(),
        scenario: FirstRunScenarioClass::MissingLocalePack,
        title: "Your locale pack was missing, so the base locale is in use.".to_owned(),
        summary: "The requested locale pack could not be found; first run fell back to the base \
                  locale and offers a one-click refresh that changes nothing else."
            .to_owned(),
        entry: local_entry(vec![
            EntryVerbClass::OpenLocalFile,
            EntryVerbClass::NewScratchFile,
        ]),
        setup_steps: vec![step(
            SetupStepClass::SignIn,
            SetupStepPosture::OfferedDeferrable,
            "aureline://command/account.sign_in",
            "Sign in later; the locale fallback needs no account.",
        )],
        health: FirstRunHealthClass::Degraded,
        repair_cues: vec![cue(
            FirstRunResourceClass::LocalePack,
            "doctor.finding.locale_pack_missing",
            "repair_transaction:disposable_state_rebuild.locale_pack_refresh",
            "checkpoint:first_run.locale_pack",
            "aureline://command/locale.refresh_pack",
            "Refresh the missing locale pack; the base locale stays usable meanwhile.",
        )],
        accessibility: standard_accessibility(),
        landing: landing(
            FirstUsefulWorkLandingClass::Readme,
            "aureline://readme/locale_fallback",
            "Landed on a README in the base locale describing the fallback and refresh.",
        ),
        diagnostics_export_ref: "aureline://diagnostics/missing_locale_pack".to_owned(),
        support_export_ref: "aureline://support_export/missing_locale_pack".to_owned(),
        evidence_refs: vec!["aureline://trace/missing_locale_pack".to_owned()],
        narrative_refs: narrative_refs(),
    };
    FirstRunOnboardingScenario {
        scenario_id: "missing_locale_pack",
        scenario_label: "Missing locale pack",
        narrative: "A missing locale pack falls back to the base locale and offers a refresh that \
                    changes nothing else.",
        fixture_filename: "missing_locale_pack.json",
        expected_scenario: FirstRunScenarioClass::MissingLocalePack,
        expected_health: FirstRunHealthClass::Degraded,
        expected_landing: FirstUsefulWorkLandingClass::Readme,
        expected_honesty_marker_present: true,
        expected_deferred_step_count: 0,
        expected_repair_cue_count: 1,
        input,
    }
}

fn newer_profile_incompatible() -> FirstRunOnboardingScenario {
    let input = FirstRunOnboardingInput {
        record_id: record_id("newer_profile_incompatible"),
        as_of: CORPUS_AS_OF.to_owned(),
        scenario: FirstRunScenarioClass::NewerProfileIncompatible,
        title: "A profile from a newer build is held, not overwritten.".to_owned(),
        summary: "Your profile was written by a newer build this version cannot read; first run \
                  keeps it intact, opens a safe sample, and offers a guided repair."
            .to_owned(),
        entry: local_entry(vec![
            EntryVerbClass::OpenSampleProject,
            EntryVerbClass::OpenLocalFolder,
        ]),
        setup_steps: vec![step(
            SetupStepClass::ChooseAppearanceAndKeymap,
            SetupStepPosture::OfferedDeferrable,
            "aureline://command/settings.appearance_and_keymap",
            "Pick a temporary theme and keymap while the newer profile is held.",
        )],
        health: FirstRunHealthClass::NeedsRepair,
        repair_cues: vec![cue(
            FirstRunResourceClass::WorkspaceProfile,
            "doctor.finding.profile_version_ahead",
            "repair_transaction:guided_export_escalation.profile_version_ahead",
            "checkpoint:first_run.workspace_profile",
            "aureline://command/profile.guided_repair",
            "Keep the newer profile intact and follow a guided downgrade or upgrade path.",
        )],
        accessibility: standard_accessibility(),
        landing: landing(
            FirstUsefulWorkLandingClass::SampleProject,
            "aureline://sample_project/welcome",
            "Landed on a safe sample project while the newer profile is held for repair.",
        ),
        diagnostics_export_ref: "aureline://diagnostics/newer_profile_incompatible".to_owned(),
        support_export_ref: "aureline://support_export/newer_profile_incompatible".to_owned(),
        evidence_refs: vec!["aureline://trace/newer_profile_incompatible".to_owned()],
        narrative_refs: narrative_refs(),
    };
    FirstRunOnboardingScenario {
        scenario_id: "newer_profile_incompatible",
        scenario_label: "Newer, incompatible profile",
        narrative: "A profile from a newer build is preserved, not overwritten; first run opens a \
                    safe sample and offers a guided repair.",
        fixture_filename: "newer_profile_incompatible.json",
        expected_scenario: FirstRunScenarioClass::NewerProfileIncompatible,
        expected_health: FirstRunHealthClass::NeedsRepair,
        expected_landing: FirstUsefulWorkLandingClass::SampleProject,
        expected_honesty_marker_present: true,
        expected_deferred_step_count: 0,
        expected_repair_cue_count: 1,
        input,
    }
}
