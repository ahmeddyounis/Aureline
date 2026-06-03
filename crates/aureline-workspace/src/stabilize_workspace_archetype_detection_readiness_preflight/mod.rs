//! Stable workspace archetype detection, readiness preflight, admission
//! checkpoints, and first-useful-work routing truth.
//!
//! This module converts workspace detection into a stable launch-wedge surface by
//! making archetype inference, readiness tasks, and first-useful-work routing
//! explicit, source-labeled, and skippable. It mints one governed
//! [`WorkspaceArchetypeReadinessPreflightRecord`] per post-entry projection that
//! the shell, CLI, diagnostics, support exports, Help/About, and docs all read
//! verbatim instead of cloning status text.
//!
//! ## Honesty invariants
//!
//! The builder refuses to mint a record that would silently widen trust, install
//! packages, auto-run setup, or hide required review. Each violation is a
//! [`BuildError`], not a warning, so a dishonest projection fails the row instead
//! of shipping:
//!
//! - **No auto-install, auto-trust, or hidden setup.** `auto_install_allowed`,
//!   `auto_trust_allowed`, `hidden_setup_executed`, and `trust_widened` must all
//!   be `false`.
//! - **Source-labeled signals.** Every archetype signal declares its source class
//!   (manifest, bundle marker, workspace file, admin policy, extension
//!   contribution, or previous user choice) so surfaces can explain why a
//!   recommendation appeared.
//! - **Readiness tasks carry source refs.** Every readiness task names the signal
//!   refs that produced it, so blocking work, recommended work, and optional work
//!   remain traceable.
//! - **Certified and probable states carry evidence freshness.** A certified or
//!   probable archetype label without current evidence freshness is rejected;
//!   stale evidence forces downgrade.
//! - **Mixed workspaces keep boundary choices.** A mixed or ambiguous workspace
//!   must expose `Open whole repo`, `Open probable project`, `Open current folder
//!   only`, and `Create workset` as same-weight options.
//! - **Same-weight bypasses.** Whenever setup is recommended, `Set up later`,
//!   `Open minimal`, and `Dismiss recommendation` remain same-weight options.
//! - **Remembered routing narrows only.** A remembered route may skip optional
//!   prompts, but it may not suppress required trust, policy, import, or
//!   prerequisite review.
//! - **Restricted and missing-prerequisite routes keep minimal paths.** A
//!   policy-blocked or prerequisite-missing workspace must still offer `Open
//!   minimal` and ordinary editing where safe.
//!
//! The canonical artifacts for this lane (suggested-output stem
//! `stabilize-workspace-archetype-detection-readiness-preflight`) are:
//!
//! - [`model`] — the governed record, its closed vocabularies, the builder, and
//!   the honesty invariants. The boundary schema is
//!   `schemas/ux/stabilize-workspace-archetype-detection-readiness-preflight.schema.json`.
//! - [`corpus`] — the deterministic claimed-stable matrix, projected through the
//!   live admission checkpoint builder and pinned on disk under
//!   `fixtures/ux/m4/stabilize-workspace-archetype-detection-readiness-preflight/`.
//!
//! The contract narrative is
//! `docs/ux/m4/stabilize-workspace-archetype-detection-readiness-preflight.md`;
//! the release-evidence packet is
//! `artifacts/ux/m4/stabilize-workspace-archetype-detection-readiness-preflight.md`.

pub mod corpus;
pub mod model;

pub use corpus::{
    workspace_archetype_readiness_preflight_corpus, WorkspaceArchetypeReadinessPreflightScenario,
    CORPUS_AS_OF,
};
pub use model::{
    BuildError, PreflightInput, WorkspaceArchetypeReadinessPreflightRecord,
    WORKSPACE_ARCHETYPE_READINESS_PREFLIGHT_NOTICE,
    WORKSPACE_ARCHETYPE_READINESS_PREFLIGHT_RECORD_KIND,
    WORKSPACE_ARCHETYPE_READINESS_PREFLIGHT_SCHEMA_VERSION,
    WORKSPACE_ARCHETYPE_READINESS_PREFLIGHT_SHARED_CONTRACT_REF,
};
