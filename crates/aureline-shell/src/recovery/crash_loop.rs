//! Crash-loop containment projection.
//!
//! When the supervisor's strike budget burns or the start sequence cannot
//! prove safety, the shell projects a crash-loop containment record. The
//! record exposes the four first-class offers the spec requires:
//!
//! - Open safe mode
//! - Disable suspect extension or runtime
//! - Open without restore
//! - Export evidence
//!
//! Plus the cache/index repair candidate as a gated escalation. The record
//! reuses the safe-mode profile and the recovery-ladder rung vocabulary so
//! diagnostics, fixtures, and reviewer artifacts share one truth.

use std::path::Path;

use serde::{Deserialize, Serialize};

use super::ladder::{RecoveryLadderRung, RecoveryLadderRungProjection};
use super::safe_mode::{
    materialize_safe_mode_profile, SafeModeEntryReason, SafeModeProfileRecord,
};

/// Schema version for [`CrashLoopContainmentRecord`].
pub const CRASH_LOOP_RECORD_SCHEMA_VERSION: u32 = 1;

/// Stable offer keys exposed by the crash-loop containment record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrashLoopOfferKey {
    /// Open the shell in the safe-mode profile.
    OpenSafeMode,
    /// Disable the suspect extension or runtime for this launch.
    DisableSuspectExtensionOrRuntime,
    /// Reopen without applying the session-restore proposal.
    OpenWithoutRestore,
    /// Export the current evidence packet.
    ExportEvidence,
    /// Offer a cache or index repair candidate.
    RepairCacheOrIndex,
}

impl CrashLoopOfferKey {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenSafeMode => "open_safe_mode",
            Self::DisableSuspectExtensionOrRuntime => "disable_suspect_extension_or_runtime",
            Self::OpenWithoutRestore => "open_without_restore",
            Self::ExportEvidence => "export_evidence",
            Self::RepairCacheOrIndex => "repair_cache_or_index",
        }
    }

    pub const fn associated_rung(self) -> RecoveryLadderRung {
        match self {
            Self::OpenSafeMode => RecoveryLadderRung::SafeMode,
            Self::DisableSuspectExtensionOrRuntime => {
                RecoveryLadderRung::SuspectExtensionQuarantine
            }
            Self::OpenWithoutRestore => RecoveryLadderRung::OpenWithoutRestore,
            Self::ExportEvidence => RecoveryLadderRung::ExportEvidence,
            Self::RepairCacheOrIndex => RecoveryLadderRung::CacheOrIndexRepair,
        }
    }
}

/// Why crash-loop containment was triggered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrashLoopReasonClass {
    /// The supervisor strike budget burned within the configured window.
    StrikeBudgetExceeded,
    /// The start sequence could not be proven safe (broken cache, suspicious
    /// content, or unknown trust state).
    StartSequenceUnsafe,
    /// User explicitly requested containment from the restore prompt.
    UserRequested,
}

impl CrashLoopReasonClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StrikeBudgetExceeded => "strike_budget_exceeded",
            Self::StartSequenceUnsafe => "start_sequence_unsafe",
            Self::UserRequested => "user_requested",
        }
    }
}

/// One offer row exposed by the crash-loop containment record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashLoopContainmentOffer {
    pub offer_key: String,
    pub command_id: String,
    pub first_class: bool,
    pub requires_confirmation: bool,
    pub explanation: String,
}

impl CrashLoopContainmentOffer {
    fn first_class_for(rung: RecoveryLadderRung, requires_confirmation: bool) -> Self {
        let row = RecoveryLadderRungProjection::offered(rung, requires_confirmation);
        Self {
            offer_key: associated_offer_key_for(rung).as_str().to_string(),
            command_id: row.command_id,
            first_class: true,
            requires_confirmation,
            explanation: row.explanation,
        }
    }

    fn gated_for(rung: RecoveryLadderRung, gate_reason: &str) -> Self {
        let row = RecoveryLadderRungProjection::gated(rung, gate_reason);
        Self {
            offer_key: associated_offer_key_for(rung).as_str().to_string(),
            command_id: row.command_id,
            first_class: false,
            requires_confirmation: true,
            explanation: row.explanation,
        }
    }
}

fn associated_offer_key_for(rung: RecoveryLadderRung) -> CrashLoopOfferKey {
    match rung {
        RecoveryLadderRung::SafeMode => CrashLoopOfferKey::OpenSafeMode,
        RecoveryLadderRung::SuspectExtensionQuarantine => {
            CrashLoopOfferKey::DisableSuspectExtensionOrRuntime
        }
        RecoveryLadderRung::OpenWithoutRestore => CrashLoopOfferKey::OpenWithoutRestore,
        RecoveryLadderRung::CacheOrIndexRepair => CrashLoopOfferKey::RepairCacheOrIndex,
        RecoveryLadderRung::ExportEvidence => CrashLoopOfferKey::ExportEvidence,
        // Restricted fallback is not exposed as a containment offer at this
        // milestone; safe-mode entry already places the shell into reduced
        // capability and the offer set focuses on the four first-class
        // exits the spec requires.
        RecoveryLadderRung::RestrictedFallback => CrashLoopOfferKey::OpenSafeMode,
    }
}

/// Canonical crash-loop containment projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashLoopContainmentRecord {
    pub record_kind: String,
    pub crash_loop_record_schema_version: u32,
    pub reason_class: String,
    pub safe_mode_profile: SafeModeProfileRecord,
    pub offers: Vec<CrashLoopContainmentOffer>,
    pub auto_rerun_forbidden: bool,
    pub never_deletes_state: bool,
    pub summary_line: String,
}

impl CrashLoopContainmentRecord {
    /// True when the record exposes the named offer key as a first-class
    /// option (not gated, not unavailable).
    pub fn first_class(&self, key: CrashLoopOfferKey) -> bool {
        self.offers
            .iter()
            .any(|o| o.offer_key == key.as_str() && o.first_class)
    }

    /// True when the record exposes the named offer key in any state.
    pub fn exposes(&self, key: CrashLoopOfferKey) -> bool {
        self.offers.iter().any(|o| o.offer_key == key.as_str())
    }
}

/// Materialize a crash-loop containment record from the configured reason.
///
/// The resulting record always exposes Open safe mode, Disable suspect
/// extension/runtime, Open without restore, and Export evidence as
/// first-class offers. The cache/index repair candidate is offered as a
/// gated escalation because it requires user review of preserved indexes
/// before running.
pub fn materialize_crash_loop_containment(reason: CrashLoopReasonClass) -> CrashLoopContainmentRecord {
    let safe_mode_profile = materialize_safe_mode_profile(SafeModeEntryReason::CrashLoopDetected);

    let offers = vec![
        CrashLoopContainmentOffer::first_class_for(RecoveryLadderRung::SafeMode, false),
        CrashLoopContainmentOffer::first_class_for(
            RecoveryLadderRung::SuspectExtensionQuarantine,
            true,
        ),
        CrashLoopContainmentOffer::first_class_for(RecoveryLadderRung::OpenWithoutRestore, false),
        CrashLoopContainmentOffer::first_class_for(RecoveryLadderRung::ExportEvidence, false),
        CrashLoopContainmentOffer::gated_for(
            RecoveryLadderRung::CacheOrIndexRepair,
            "preserve_indexed_buffers_first",
        ),
    ];

    let summary_line = format!(
        "crash_loop_containment: reason={reason}; offers={offers}; auto_rerun_forbidden=true; never_deletes_state=true",
        reason = reason.as_str(),
        offers = offers
            .iter()
            .map(|o| o.offer_key.as_str())
            .collect::<Vec<_>>()
            .join(","),
    );

    CrashLoopContainmentRecord {
        record_kind: "crash_loop_containment_record".to_string(),
        crash_loop_record_schema_version: CRASH_LOOP_RECORD_SCHEMA_VERSION,
        reason_class: reason.as_str().to_string(),
        safe_mode_profile,
        offers,
        auto_rerun_forbidden: true,
        never_deletes_state: true,
        summary_line,
    }
}

/// Write a crash-loop containment record under
/// `<recovery_root>/crash_loop_containment_latest.json`.
pub fn write_crash_loop_containment_log(
    recovery_root: &Path,
    record: &CrashLoopContainmentRecord,
) -> Result<(), String> {
    std::fs::create_dir_all(recovery_root)
        .map_err(|err| format!("create recovery root failed: {err}"))?;
    let path = recovery_root.join("crash_loop_containment_latest.json");
    let json = serde_json::to_string_pretty(record)
        .map_err(|err| format!("serialize crash-loop containment failed: {err}"))?;
    std::fs::write(&path, json)
        .map_err(|err| format!("write {} failed: {err}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn containment_exposes_required_first_class_offers() {
        let record = materialize_crash_loop_containment(CrashLoopReasonClass::StrikeBudgetExceeded);

        for key in [
            CrashLoopOfferKey::OpenSafeMode,
            CrashLoopOfferKey::DisableSuspectExtensionOrRuntime,
            CrashLoopOfferKey::OpenWithoutRestore,
            CrashLoopOfferKey::ExportEvidence,
        ] {
            assert!(
                record.first_class(key),
                "{} must be a first-class crash-loop offer",
                key.as_str()
            );
        }

        // Cache/index repair is exposed but gated.
        assert!(record.exposes(CrashLoopOfferKey::RepairCacheOrIndex));
        assert!(!record.first_class(CrashLoopOfferKey::RepairCacheOrIndex));

        assert!(record.auto_rerun_forbidden);
        assert!(record.never_deletes_state);
        assert!(record.summary_line.contains("auto_rerun_forbidden=true"));
        assert!(record.summary_line.contains("never_deletes_state=true"));
        assert_eq!(record.safe_mode_profile.entry_reason_class, "crash_loop_detected");
    }

    #[test]
    fn user_requested_reason_is_recorded_verbatim() {
        let record = materialize_crash_loop_containment(CrashLoopReasonClass::UserRequested);
        assert_eq!(record.reason_class, "user_requested");
    }

    #[test]
    fn write_log_round_trips_via_serde() {
        let dir = tempfile::tempdir().unwrap();
        let record =
            materialize_crash_loop_containment(CrashLoopReasonClass::StartSequenceUnsafe);
        write_crash_loop_containment_log(dir.path(), &record).expect("write");
        let read = std::fs::read_to_string(
            dir.path().join("crash_loop_containment_latest.json"),
        )
        .unwrap();
        let back: CrashLoopContainmentRecord = serde_json::from_str(&read).unwrap();
        assert_eq!(back, record);
    }
}
