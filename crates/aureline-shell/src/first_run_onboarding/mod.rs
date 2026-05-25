//! Stable first-run onboarding truth: no-account local entry, setup-later
//! posture, and repair-safe recovery cues.
//!
//! This module makes the first launch replacement-grade on the claimed stable
//! matrix. It mints one governed [`FirstRunOnboardingRecord`] per first-run
//! scenario that binds:
//!
//! - **No-account local entry** — useful local work is reachable with no account
//!   and no managed service, and at least one entry verb is offered without an
//!   account.
//! - **Setup-later posture** — sign-in, workspace trust, recommended extensions,
//!   appearance / keymap, an AI provider, a remote / managed connection, and an
//!   editor import are all offered but deferrable; deferring never blocks
//!   first-useful-work and never silently widens trust, installs packages,
//!   applies a workflow bundle, or skips a required checkpoint, and every step
//!   keeps a resume route.
//! - **Repair-safe recovery cues** — damaged first-run state (an unreadable
//!   settings store, a partial migration, a missing locale pack, a newer
//!   profile) degrades safely: each cue preserves the user's files, never
//!   dead-ends, never silently resets, routes through the `metadata_safe_default`
//!   redaction class, and carries an export-safe chain of custody (a
//!   `doctor.finding.*` code, a `repair_transaction:*` id, and a checkpoint ref).
//! - **Durable, accessible truth** — first-run truth is durable (never
//!   toast-only) and never theme-only, the entry surfaces (Start Center, command
//!   palette, menu) are keyboard-reachable, and the flow stays reachable in
//!   normal, high-contrast, and zoomed layouts.
//! - **No-account landing** — first-useful-work routes to a keyboard-reachable,
//!   non-destructive landing that does not require an account.
//!
//! The desktop shell, command palette, menus, diagnostics, support exports,
//! Help/About, and docs read this record verbatim instead of cloning status
//! text. The canonical artifacts for this lane (suggested-output stem
//! `finalize-first-run-onboarding-with-no-account-local`) are:
//!
//! - [`model`] — the governed record, its closed vocabularies, the builder, and
//!   the honesty invariants. The boundary schema is
//!   `schemas/ux/finalize-first-run-onboarding-with-no-account-local.schema.json`.
//! - [`corpus`] — the deterministic first-run drill corpus pinned on disk under
//!   `fixtures/ux/m4/finalize-first-run-onboarding-with-no-account-local/`.
//!
//! The contract narrative is
//! `docs/ux/m4/finalize-first-run-onboarding-with-no-account-local.md`; the
//! release-evidence packet is
//! `artifacts/ux/m4/finalize-first-run-onboarding-with-no-account-local.md`. The
//! upstream first-run no-account contract is
//! `docs/ux/no_account_local_entry_contract.md`.

pub mod corpus;
pub mod model;

pub use corpus::{
    first_run_onboarding_corpus, FirstRunOnboardingScenario, CORPUS_AS_OF, CORPUS_RECORD_ID_PREFIX,
};
pub use model::{
    is_canonical_object_ref, is_doctor_finding_code, is_opaque_id, is_repair_transaction_ref,
    AccessibilityDisclosure, BuildError, EntryMode, EntrySurface, EntrySurfaceClass,
    EntryVerbClass, FirstRunDisplayCopy, FirstRunHealthClass, FirstRunOnboardingInput,
    FirstRunOnboardingRecord, FirstRunResourceClass, FirstRunScenarioClass, FirstRunSummaryCounts,
    FirstUsefulWorkLanding, FirstUsefulWorkLandingClass, RepairCue, SetupStep, SetupStepClass,
    SetupStepPosture, DOCTOR_FINDING_PREFIX, FIRST_RUN_ONBOARDING_NOTICE,
    FIRST_RUN_ONBOARDING_RECORD_KIND, FIRST_RUN_ONBOARDING_SCHEMA_VERSION,
    REPAIR_TRANSACTION_PREFIX, REQUIRED_REDACTION_CLASS,
};
