//! Workset switcher beta — durable typed truth for the switcher row, the
//! activation-preview diff, and the reopen-parity packet.
//!
//! The alpha switcher / scope-banner records (in `aureline-shell`) project a
//! readable list and banner from the durable [`WorksetArtifactRecord`]; this
//! beta layer adds the harder cross-surface contract the M3 spec calls out:
//!
//! 1. Every switcher row names every included root with `root_kind`,
//!    `partial_truth`, and presentation label — a multi-root workset never
//!    collapses to "Three repos" without naming each one.
//! 2. Every row carries one closed [`WorksetPortabilityLabel`]
//!    (`portable`, `portable_with_rebinding`, `local_only`,
//!    `policy_limited`, or `managed_provider_locked`) so support packets and
//!    reopen consumers quote a single user-visible portability truth.
//! 3. Picking a candidate row produces a typed
//!    [`WorksetActivationPreview`] — a diff against the active artifact that
//!    classifies root additions, root removals, and scope drift before the
//!    user applies it.
//! 4. The [`WorksetReopenParityPacket`] bundles local, remote, and headless
//!    consumer bindings for one workset so every reopen path preserves the
//!    same `stable_scope_id` with explicit downgrade reasons.
//!
//! The schema lives at
//! `schemas/workspace/workset_switcher_beta.schema.json`; the reviewer doc
//! lives at `docs/workspace/m3/workset_artifact_beta.md`; the canonical
//! fixtures live under `fixtures/workspace/m3/workset_switcher/`.

pub mod beta;

pub use beta::{
    derive_portability_label, project_switcher_record, project_switcher_row,
    root_taxonomy_badge, ReopenParityDowngrade, ScopeDriftClass, SwitcherRowAction,
    PolicyOverlaySummary, WorksetActivationPreview, WorksetActivationPreviewError, WorksetPortabilityLabel,
    WorksetReopenParityError, WorksetReopenParityPacket, WorksetSwitcherBetaError,
    WorksetSwitcherBetaRecord, WorksetSwitcherBetaRow, WorksetSwitcherBetaSupportExport,
    WORKSET_ACTIVATION_PREVIEW_RECORD_KIND, WORKSET_REOPEN_PARITY_PACKET_RECORD_KIND,
    WORKSET_SWITCHER_BETA_RECORD_KIND, WORKSET_SWITCHER_BETA_ROW_RECORD_KIND,
    WORKSET_SWITCHER_BETA_SCHEMA_VERSION, WORKSET_SWITCHER_BETA_SUPPORT_EXPORT_RECORD_KIND,
};
