//! Canonical seed builders for the M5 runbook companion register.
//!
//! These builders are the single producer of the checked-in companion register, the
//! published inventory, the Markdown proof, and the per-surface fixtures. The headless
//! emitter and the inline tests both call them so the in-code register, the artifacts,
//! and the fixtures never drift. The register narrows the *same* checked-in executable
//! steps the step library publishes, so a companion's authority over a step is derived
//! mechanically from the one governed step object rather than a companion-only copy.

use super::*;

use crate::m5_runbook_steps::seeded_executable_steps;

/// Stable register id for the canonical companion register.
pub const M5_RUNBOOK_COMPANION_REGISTER_ID: &str = "m5-runbook-companion-register:stable:0001";

/// Evaluation / mint timestamp for the canonical register.
const SEED_EVALUATED_AT: &str = "2026-07-06T00:00:00Z";

const REDACTION_CLASS: &str = "metadata_safe_default";

/// The companion surfaces narrowed from the canonical executable steps. Deriving them
/// from [`seeded_executable_steps`] is what keeps the companion register and the step
/// library the same governed objects — a companion cannot be granted authority the
/// step does not declare.
pub fn seeded_companion_surfaces() -> Vec<CompanionRunbookSurface> {
    seeded_executable_steps()
        .iter()
        .map(CompanionRunbookSurface::derive)
        .collect()
}

/// The canonical runbook companion register: every governed executable step narrowed
/// to the companion client scope, with follow/acknowledge/comment available within
/// scope, companion-allowed approvals reusing the desktop refs, and every blocked
/// privileged mutate degrading to an explicit desktop handoff.
pub fn seeded_m5_runbook_companion_register() -> M5RunbookCompanionRegister {
    M5RunbookCompanionRegister::new(M5RunbookCompanionRegisterInput {
        register_id: M5_RUNBOOK_COMPANION_REGISTER_ID.to_owned(),
        report_label: "M5 runbook companion-scoped surface register".to_owned(),
        evaluated_at: SEED_EVALUATED_AT.to_owned(),
        steps: seeded_executable_steps(),
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_EVALUATED_AT.to_owned(),
    })
}
