//! Safe-mode profile projection.
//!
//! The safe-mode profile is the visibly reduced startup posture the shell
//! enters when the normal path cannot prove it is safe to run. The profile
//! enumerates which capabilities are disabled or narrowed and which durable
//! state classes are preserved verbatim. It uses the same vocabulary as the
//! recovery-rung examples under
//! `/fixtures/recovery/recovery_rung_examples/startup_crash_loop_safe_mode.yaml`
//! so the live runtime, exported fixtures, and reviewer artifacts share one
//! truth instead of inventing parallel labels.

use std::path::Path;

use serde::{Deserialize, Serialize};

/// Schema version for [`SafeModeProfileRecord`].
pub const SAFE_MODE_PROFILE_SCHEMA_VERSION: u32 = 1;

/// Canonical command id used to enter safe mode from the shell.
pub const SAFE_MODE_ENTER_COMMAND_ID: &str = "cmd:workspace.enter_safe_mode";

/// Canonical command id used to leave safe mode after explicit review.
pub const SAFE_MODE_EXIT_COMMAND_ID: &str = "cmd:workspace.exit_safe_mode_after_review";

/// Why the shell entered safe mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafeModeEntryReason {
    /// Operator entered safe mode from the restore prompt or palette.
    UserRequested,
    /// Crash-loop containment forced safe mode after the strike budget burned.
    CrashLoopDetected,
    /// Suspicious-content findings on a primary surface forced reduced posture.
    SuspiciousContentDetected,
    /// Cache or index appears broken; safe mode protects the run while the
    /// repair candidate is offered.
    BrokenCacheSuspected,
    /// Trust state is unknown or revoked; reduced posture protects the run.
    TrustUncertain,
}

impl SafeModeEntryReason {
    /// Stable string used in records, fixtures, and a11y exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserRequested => "user_requested",
            Self::CrashLoopDetected => "crash_loop_detected",
            Self::SuspiciousContentDetected => "suspicious_content_detected",
            Self::BrokenCacheSuspected => "broken_cache_suspected",
            Self::TrustUncertain => "trust_uncertain",
        }
    }
}

/// Canonical safe-mode profile record.
///
/// Field names follow the recovery-rung-example contract verbatim so the
/// live runtime, the fixture corpus, and reviewer-facing artifacts can be
/// diffed character-for-character.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafeModeProfileRecord {
    pub record_kind: String,
    pub safe_mode_profile_schema_version: u32,
    pub entry_reason_class: String,
    pub trust_state_after_entry: String,
    pub disabled_or_narrowed_capabilities: Vec<String>,
    pub preserved_state_classes: Vec<String>,
    pub entry_visible_and_logged: bool,
    pub entry_confirmation_required: bool,
    pub trust_widening_forbidden_without_review: bool,
    pub auto_restore_forbidden: bool,
    pub safe_mode_enter_command_id: String,
    pub safe_mode_exit_command_id: String,
    pub next_options_after_entry: Vec<String>,
    pub explanation_lines: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl SafeModeProfileRecord {
    /// True when `next_options_after_entry` includes the named offer key.
    pub fn offers(&self, key: &str) -> bool {
        self.next_options_after_entry.iter().any(|v| v == key)
    }
}

/// Materialize the canonical safe-mode profile for a given entry reason.
///
/// The profile narrows the surface to the named preserved state classes and
/// turns off the named capabilities. It is the substrate every recovery-
/// ladder rung is allowed to assume the shell is running under.
pub fn materialize_safe_mode_profile(reason: SafeModeEntryReason) -> SafeModeProfileRecord {
    let disabled = vec![
        "extension_auto_activation".to_string(),
        "extension_host_launch".to_string(),
        "session_restore_auto_reopen".to_string(),
        "remote_helper_attach".to_string(),
        "ai_runtime_access".to_string(),
        "background_rebuild".to_string(),
        "terminal_repo_recipe_launch".to_string(),
        "debug_launch".to_string(),
        "notebook_kernel_connect".to_string(),
        "environment_activator_run".to_string(),
    ];

    let preserved = vec![
        "user_authored_files".to_string(),
        "open_buffer_selection".to_string(),
        "durable_workspace_indexes".to_string(),
        "workspace_trust_store".to_string(),
        "credential_store".to_string(),
        "managed_policy_overrides".to_string(),
        "session_restore_store".to_string(),
        "support_export_store".to_string(),
    ];

    let next_options = vec![
        "widen_to_full_mode_with_review".to_string(),
        "escalate_to_extension_quarantine".to_string(),
        "escalate_to_cache_reset_candidate".to_string(),
        "open_without_restore".to_string(),
        "export_escalation_packet".to_string(),
    ];

    let explanation_lines = match reason {
        SafeModeEntryReason::UserRequested => vec![
            "Safe mode reduces the surface area of this launch.".to_string(),
            "Extensions, auto-restore, and remote helpers stay off until you review them."
                .to_string(),
            "Files, journals, and trust state are preserved unchanged.".to_string(),
        ],
        SafeModeEntryReason::CrashLoopDetected => vec![
            "Safe mode is in effect because the previous launches did not finish cleanly."
                .to_string(),
            "Auto-restore and extension activation are paused so you can inspect the cause."
                .to_string(),
            "Use Export evidence to hand off the crash packet without leaving safe mode."
                .to_string(),
        ],
        SafeModeEntryReason::SuspiciousContentDetected => vec![
            "Safe mode is in effect because suspicious content was detected on a protected surface."
                .to_string(),
            "Raw and escaped inspection paths stay reachable; bytes are not rewritten.".to_string(),
            "Review the finding before widening trust or running active content.".to_string(),
        ],
        SafeModeEntryReason::BrokenCacheSuspected => vec![
            "Safe mode is in effect because the workspace cache or index looks inconsistent."
                .to_string(),
            "A cache repair candidate is offered as the next rung; user-authored files stay intact."
                .to_string(),
        ],
        SafeModeEntryReason::TrustUncertain => vec![
            "Safe mode is in effect because the workspace trust state is unknown or revoked."
                .to_string(),
            "Trust widening requires explicit review.".to_string(),
        ],
    };

    SafeModeProfileRecord {
        record_kind: "safe_mode_profile_record".to_string(),
        safe_mode_profile_schema_version: SAFE_MODE_PROFILE_SCHEMA_VERSION,
        entry_reason_class: reason.as_str().to_string(),
        trust_state_after_entry: "restricted_recovery_fallback".to_string(),
        disabled_or_narrowed_capabilities: disabled,
        preserved_state_classes: preserved,
        entry_visible_and_logged: true,
        entry_confirmation_required: matches!(reason, SafeModeEntryReason::UserRequested),
        trust_widening_forbidden_without_review: true,
        auto_restore_forbidden: true,
        safe_mode_enter_command_id: SAFE_MODE_ENTER_COMMAND_ID.to_string(),
        safe_mode_exit_command_id: SAFE_MODE_EXIT_COMMAND_ID.to_string(),
        next_options_after_entry: next_options,
        explanation_lines,
        notes: Some(
            "Safe mode never deletes state. Recovery rungs are offered as next options."
                .to_string(),
        ),
    }
}

/// Write the safe-mode profile under `<recovery_root>/safe_mode_profile_latest.json`.
pub fn write_safe_mode_profile_log(
    recovery_root: &Path,
    record: &SafeModeProfileRecord,
) -> Result<(), String> {
    std::fs::create_dir_all(recovery_root)
        .map_err(|err| format!("create recovery root failed: {err}"))?;
    let path = recovery_root.join("safe_mode_profile_latest.json");
    let json = serde_json::to_string_pretty(record)
        .map_err(|err| format!("serialize safe-mode profile failed: {err}"))?;
    std::fs::write(&path, json).map_err(|err| format!("write {} failed: {err}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_requested_profile_disables_extensions_and_auto_restore() {
        let record = materialize_safe_mode_profile(SafeModeEntryReason::UserRequested);
        assert!(record.auto_restore_forbidden);
        assert!(record.entry_confirmation_required);
        assert!(record.trust_widening_forbidden_without_review);
        assert!(record
            .disabled_or_narrowed_capabilities
            .iter()
            .any(|c| c == "extension_auto_activation"));
        assert!(record
            .disabled_or_narrowed_capabilities
            .iter()
            .any(|c| c == "session_restore_auto_reopen"));
        assert!(record
            .preserved_state_classes
            .iter()
            .any(|c| c == "user_authored_files"));
        assert!(record.offers("export_escalation_packet"));
        assert_eq!(
            record.safe_mode_enter_command_id,
            SAFE_MODE_ENTER_COMMAND_ID
        );
    }

    #[test]
    fn crash_loop_profile_does_not_require_user_confirmation_to_enter() {
        // Containment forces safe mode without an extra prompt; review is
        // required to leave it.
        let record = materialize_safe_mode_profile(SafeModeEntryReason::CrashLoopDetected);
        assert!(!record.entry_confirmation_required);
        assert!(record.trust_widening_forbidden_without_review);
        assert_eq!(record.entry_reason_class, "crash_loop_detected");
    }

    #[test]
    fn write_log_produces_round_trippable_json() {
        let dir = tempfile::tempdir().unwrap();
        let record = materialize_safe_mode_profile(SafeModeEntryReason::CrashLoopDetected);
        write_safe_mode_profile_log(dir.path(), &record).expect("write");
        let read =
            std::fs::read_to_string(dir.path().join("safe_mode_profile_latest.json")).unwrap();
        let back: SafeModeProfileRecord = serde_json::from_str(&read).unwrap();
        assert_eq!(back, record);
    }
}
