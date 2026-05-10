//! Recovery-ladder rung stubs.
//!
//! The recovery ladder is the ordered set of options the shell may offer
//! after entering safe mode. Each rung is a stub at this milestone — it
//! names the action, the entry reason it admits, the state it preserves,
//! and what the rung is allowed to widen into. The full rung implementations
//! land in later lanes; this module freezes the substrate so downstream
//! work does not invent parallel rung vocabularies.

use serde::{Deserialize, Serialize};

/// Schema version for [`RecoveryLadderRungProjection`].
pub const RECOVERY_LADDER_SCHEMA_VERSION: u32 = 1;

/// Closed recovery-ladder rung vocabulary.
///
/// Spellings reuse the `rung_class` tokens from
/// `/fixtures/recovery/recovery_rung_examples/` so the live runtime, the
/// fixture set, and the doc seed share one vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryLadderRung {
    /// Enter the safe-mode profile.
    SafeMode,
    /// Quarantine a suspect extension or runtime; the host stays in safe mode.
    SuspectExtensionQuarantine,
    /// Reopen the workspace without applying the session-restore proposal.
    OpenWithoutRestore,
    /// Offer a cache or index repair candidate; user-authored files unchanged.
    CacheOrIndexRepair,
    /// Restricted fallback: continue with reduced capability and visible chrome.
    RestrictedFallback,
    /// Export the evidence packet for the current run.
    ExportEvidence,
}

impl RecoveryLadderRung {
    /// Stable string used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafeMode => "safe_mode",
            Self::SuspectExtensionQuarantine => "suspect_extension_quarantine",
            Self::OpenWithoutRestore => "open_without_restore",
            Self::CacheOrIndexRepair => "cache_or_index_repair",
            Self::RestrictedFallback => "restricted_fallback",
            Self::ExportEvidence => "export_evidence",
        }
    }

    /// Returns the canonical command id for the rung.
    pub const fn command_id(self) -> &'static str {
        match self {
            Self::SafeMode => "cmd:workspace.enter_safe_mode",
            Self::SuspectExtensionQuarantine => "cmd:workspace.quarantine_suspect_extension",
            Self::OpenWithoutRestore => "cmd:workspace.open_without_restore",
            Self::CacheOrIndexRepair => "cmd:workspace.repair_cache_or_index_candidate",
            Self::RestrictedFallback => "cmd:workspace.continue_in_restricted_mode",
            Self::ExportEvidence => "cmd:workspace.export_recovery_evidence",
        }
    }

    /// Stable enum-iteration order used by projections.
    pub const fn ordered_all() -> [Self; 6] {
        [
            Self::SafeMode,
            Self::SuspectExtensionQuarantine,
            Self::OpenWithoutRestore,
            Self::CacheOrIndexRepair,
            Self::RestrictedFallback,
            Self::ExportEvidence,
        ]
    }
}

/// Whether a rung is offered, gated, or unavailable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryLadderRungState {
    /// Rung is offered as a first-class option.
    Offered,
    /// Rung is offered but disabled until a precondition holds.
    GatedRequiresReview,
    /// Rung is not currently reachable from the projected entry reason.
    Unavailable,
}

impl RecoveryLadderRungState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Offered => "offered",
            Self::GatedRequiresReview => "gated_requires_review",
            Self::Unavailable => "unavailable",
        }
    }
}

/// One row in the recovery-ladder projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryLadderRungProjection {
    pub rung_class: String,
    pub command_id: String,
    pub state: String,
    pub requires_confirmation: bool,
    pub never_deletes_state: bool,
    pub explanation: String,
}

impl RecoveryLadderRungProjection {
    /// Construct an offered rung that matches the recovery-rung-example
    /// vocabulary (no state deletion, explicit review for trust widening).
    pub fn offered(rung: RecoveryLadderRung, requires_confirmation: bool) -> Self {
        Self {
            rung_class: rung.as_str().to_string(),
            command_id: rung.command_id().to_string(),
            state: RecoveryLadderRungState::Offered.as_str().to_string(),
            requires_confirmation,
            never_deletes_state: true,
            explanation: explanation_for(rung).to_string(),
        }
    }

    /// Construct a gated row that requires a precondition before it runs.
    pub fn gated(rung: RecoveryLadderRung, gate_reason: &str) -> Self {
        Self {
            rung_class: rung.as_str().to_string(),
            command_id: rung.command_id().to_string(),
            state: RecoveryLadderRungState::GatedRequiresReview
                .as_str()
                .to_string(),
            requires_confirmation: true,
            never_deletes_state: true,
            explanation: format!("{} ({})", explanation_for(rung), gate_reason),
        }
    }
}

const fn explanation_for(rung: RecoveryLadderRung) -> &'static str {
    match rung {
        RecoveryLadderRung::SafeMode => {
            "Open with reduced capabilities; user-authored files and journals preserved."
        }
        RecoveryLadderRung::SuspectExtensionQuarantine => {
            "Disable the suspect extension or runtime for this launch; do not delete its files."
        }
        RecoveryLadderRung::OpenWithoutRestore => {
            "Skip the session-restore proposal; layout is not rehydrated automatically."
        }
        RecoveryLadderRung::CacheOrIndexRepair => {
            "Offer a candidate repair for the workspace cache or index; user-authored files unchanged."
        }
        RecoveryLadderRung::RestrictedFallback => {
            "Continue with the narrowest capability set and visible recovery chrome."
        }
        RecoveryLadderRung::ExportEvidence => {
            "Export the current crash and recovery evidence without leaving safe mode."
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordered_all_covers_each_rung_once() {
        let all = RecoveryLadderRung::ordered_all();
        let mut seen = std::collections::HashSet::new();
        for rung in all {
            assert!(seen.insert(rung.as_str()));
        }
        assert_eq!(seen.len(), 6);
    }

    #[test]
    fn offered_rung_never_deletes_state() {
        for rung in RecoveryLadderRung::ordered_all() {
            let row = RecoveryLadderRungProjection::offered(rung, true);
            assert!(row.never_deletes_state, "{} must not delete state", rung.as_str());
        }
    }

    #[test]
    fn gated_rung_requires_confirmation() {
        let row = RecoveryLadderRungProjection::gated(
            RecoveryLadderRung::CacheOrIndexRepair,
            "preserve indexed buffers first",
        );
        assert!(row.requires_confirmation);
        assert_eq!(row.state, "gated_requires_review");
    }
}
