//! Canonical seed builders for the M5 runbook execution history.
//!
//! These builders are the single producer of the checked-in execution-history
//! inventory, the Markdown proof, and the release-grade export. The headless emitter
//! and the inline tests both call them so the in-code history, the artifacts, and the
//! operator-scenario fixtures never drift. The history reuses the governance lane's
//! operator-scenario execution records directly, so the same records that demonstrate
//! the object model are the rows operator history, support exports, and incident
//! packets read — runbooks are never a privileged exception path.

use super::*;

use crate::m5_runbook_governance::seeded_operator_scenario_records;

/// Stable history id for the canonical execution history.
pub const M5_RUNBOOK_EXECUTION_HISTORY_ID: &str = "m5-runbook-execution-history:stable:0001";

/// Evaluation / mint timestamp for the canonical history.
const SEED_EVALUATED_AT: &str = "2026-07-06T00:00:00Z";

const REDACTION_CLASS: &str = "metadata_safe_default";

/// The checked-in governed execution records: the four operator scenarios, each with
/// its rows bound to the shared preview-hash and approval reuse.
pub fn seeded_runbook_execution_records() -> Vec<RunbookExecutionRecord> {
    seeded_operator_scenario_records()
}

/// The canonical runbook execution history: every operator scenario's rows projected
/// into the shared preview/approval/audit reuse vocabulary, exposed identically on
/// operator history, support exports, and incident packets.
pub fn seeded_m5_runbook_execution_history() -> M5RunbookExecutionHistory {
    M5RunbookExecutionHistory::new(M5RunbookExecutionHistoryInput {
        history_id: M5_RUNBOOK_EXECUTION_HISTORY_ID.to_owned(),
        report_label: "M5 runbook execution history".to_owned(),
        evaluated_at: SEED_EVALUATED_AT.to_owned(),
        executions: seeded_runbook_execution_records(),
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_EVALUATED_AT.to_owned(),
    })
}
