//! Deep-link entry validator for the live shell.
//!
//! Aureline accepts deep links (protocol handler activations, default-browser
//! callbacks, system-share targets) only after a small set of admission rules
//! have run. The validator never executes the link itself; it returns one of:
//!
//! - [`DeepLinkValidationOutcome::Admitted`] — the route is acceptable. The
//!   shell may dispatch through the same entry-flow resolver Start Center
//!   uses; when `reviewed_sheet_required` is `true`, the shell MUST reopen
//!   the reviewed entry-flow sheet first instead of executing inline.
//! - [`DeepLinkValidationOutcome::Denied`] — the route fails closed. The
//!   shell preserves user intent (locate / reconnect / remove / open-anyway)
//!   instead of replaying the link silently.
//!
//! The validator is a pure projection over the
//! `schemas/platform/deep_link_intent.schema.json` boundary vocabulary. It
//! does not see raw URLs, raw callback bodies, or raw provider payloads;
//! callers strip those before constructing a [`DeepLinkIntent`].

use serde::{Deserialize, Serialize};

/// Origin class for an inbound deep-link intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkOriginClass {
    OsShell,
    SystemDefaultBrowser,
    FirstPartyWeb,
    TrustedCompanion,
    ExternalProvider,
    CollaborationService,
    LocalCli,
    InstallerOrUpdateFlow,
    UnknownUntrusted,
}

impl DeepLinkOriginClass {
    /// Stable token used in records, fixtures, and logs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OsShell => "os_shell",
            Self::SystemDefaultBrowser => "system_default_browser",
            Self::FirstPartyWeb => "first_party_web",
            Self::TrustedCompanion => "trusted_companion",
            Self::ExternalProvider => "external_provider",
            Self::CollaborationService => "collaboration_service",
            Self::LocalCli => "local_cli",
            Self::InstallerOrUpdateFlow => "installer_or_update_flow",
            Self::UnknownUntrusted => "unknown_untrusted",
        }
    }
}

/// Target class an inbound deep-link intent claims.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkTargetClass {
    LocalFile,
    LocalFolder,
    WorkspaceRoot,
    RecentWorkEntry,
    ReviewThread,
    WorkItem,
    ManagedWorkspace,
    CommandTarget,
    UnknownTarget,
}

impl DeepLinkTargetClass {
    /// Stable token used in records, fixtures, and logs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalFile => "local_file",
            Self::LocalFolder => "local_folder",
            Self::WorkspaceRoot => "workspace_root",
            Self::RecentWorkEntry => "recent_work_entry",
            Self::ReviewThread => "review_thread",
            Self::WorkItem => "work_item",
            Self::ManagedWorkspace => "managed_workspace",
            Self::CommandTarget => "command_target",
            Self::UnknownTarget => "unknown_target",
        }
    }
}

/// Command class an inbound deep-link intent requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkCommandClass {
    InspectOnly,
    RevealOnly,
    OpenExistingContext,
    CreateOrAddContext,
    JoinPresence,
    ResumeSession,
    AuthReturn,
    RetryOrReconnect,
    AcknowledgeNotification,
    MutatingCommandRequest,
    PrivilegedAuthorityWidening,
}

impl DeepLinkCommandClass {
    /// Stable token used in records, fixtures, and logs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectOnly => "inspect_only",
            Self::RevealOnly => "reveal_only",
            Self::OpenExistingContext => "open_existing_context",
            Self::CreateOrAddContext => "create_or_add_context",
            Self::JoinPresence => "join_presence",
            Self::ResumeSession => "resume_session",
            Self::AuthReturn => "auth_return",
            Self::RetryOrReconnect => "retry_or_reconnect",
            Self::AcknowledgeNotification => "acknowledge_notification",
            Self::MutatingCommandRequest => "mutating_command_request",
            Self::PrivilegedAuthorityWidening => "privileged_authority_widening",
        }
    }

    /// True when the requested command class is boundary-raising and a
    /// reviewed sheet is mandatory before execution.
    pub const fn is_boundary_raising(self) -> bool {
        matches!(
            self,
            Self::MutatingCommandRequest
                | Self::PrivilegedAuthorityWidening
                | Self::ResumeSession
                | Self::CreateOrAddContext
                | Self::JoinPresence
        )
    }
}

/// One inbound deep-link intent admitted to the validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeepLinkIntent {
    pub intent_id: String,
    pub origin_class: DeepLinkOriginClass,
    pub target_class: DeepLinkTargetClass,
    pub command_class: DeepLinkCommandClass,
    /// Opaque, redaction-safe label rendered to the user when reopening the
    /// entry-flow sheet. Raw URLs, paths, or callback payloads MUST NOT
    /// appear here.
    pub route_label: String,
    /// True when the intent has already been consumed (single-use replay
    /// posture).
    #[serde(default)]
    pub replay_consumed: bool,
    /// True when the origin still owns the deep-link handler claim.
    #[serde(default = "default_handler_owned")]
    pub handler_ownership_verified: bool,
}

const fn default_handler_owned() -> bool {
    true
}

/// Schema version exported with [`DeepLinkValidationRecord`].
pub type DeepLinkValidationSchemaVersion = u32;

/// Closed denial vocabulary returned by the validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkDenialClass {
    /// Origin class is `unknown_untrusted` (cannot be reviewed without
    /// out-of-band proof).
    OriginUnverified,
    /// Target class is `unknown_target` (cannot be reviewed without further
    /// disambiguation).
    TargetUnresolved,
    /// Replay was already consumed (single-use posture).
    ReplayConsumed,
    /// The route would raise a boundary the validator cannot satisfy without
    /// a reviewed sheet but the surface refused review (recorded for audit
    /// purposes; the live consumer turns this into a reviewed-sheet redirect
    /// rather than a hard denial).
    BoundaryRaisingWithoutReview,
    /// Handler ownership was lost (another app claims this protocol).
    HandlerOwnershipLost,
}

impl DeepLinkDenialClass {
    /// Stable string used in records, fixtures, and logs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OriginUnverified => "origin_unverified",
            Self::TargetUnresolved => "target_unresolved",
            Self::ReplayConsumed => "replay_consumed",
            Self::BoundaryRaisingWithoutReview => "boundary_raising_without_review",
            Self::HandlerOwnershipLost => "handler_ownership_lost",
        }
    }
}

/// Successful admission of a deep-link intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeepLinkAdmission {
    /// True when the live shell MUST reopen the reviewed entry-flow sheet
    /// before executing the inferred command.
    pub reviewed_sheet_required: bool,
    /// Reason the reviewed sheet is required (or "none" when not required).
    pub reviewed_sheet_reason: String,
    /// Human-readable explanation suitable for log lines and audit packets.
    pub summary: String,
}

/// Denied admission of a deep-link intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeepLinkDenial {
    pub denial_class: DeepLinkDenialClass,
    pub summary: String,
}

/// Outcome of `validate_deep_link_intent`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum DeepLinkValidationOutcome {
    Admitted(DeepLinkAdmission),
    Denied(DeepLinkDenial),
}

impl DeepLinkValidationOutcome {
    /// True when the route was admitted.
    pub const fn is_admitted(&self) -> bool {
        matches!(self, Self::Admitted(_))
    }

    /// True when the route was admitted but a reviewed sheet is required.
    pub const fn requires_reviewed_sheet(&self) -> bool {
        matches!(
            self,
            Self::Admitted(admission) if admission.reviewed_sheet_required
        )
    }
}

/// Materialized validation record exported by the validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeepLinkValidationRecord {
    pub record_kind: String,
    pub deep_link_validation_schema_version: DeepLinkValidationSchemaVersion,
    pub intent: DeepLinkIntent,
    pub outcome: DeepLinkValidationOutcome,
}

/// Validates an inbound deep-link intent.
///
/// The validator never replays the intent; it only decides whether the live
/// shell should reopen a reviewed entry-flow sheet or fail closed with a
/// recovery-aware denial.
pub fn validate_deep_link_intent(intent: &DeepLinkIntent) -> DeepLinkValidationOutcome {
    if intent.origin_class == DeepLinkOriginClass::UnknownUntrusted {
        return DeepLinkValidationOutcome::Denied(DeepLinkDenial {
            denial_class: DeepLinkDenialClass::OriginUnverified,
            summary: "deep-link origin is unknown_untrusted; review cannot proceed without out-of-band proof".to_string(),
        });
    }

    if intent.target_class == DeepLinkTargetClass::UnknownTarget {
        return DeepLinkValidationOutcome::Denied(DeepLinkDenial {
            denial_class: DeepLinkDenialClass::TargetUnresolved,
            summary:
                "deep-link target_class is unknown_target; reopen disambiguation before execution"
                    .to_string(),
        });
    }

    if intent.replay_consumed {
        return DeepLinkValidationOutcome::Denied(DeepLinkDenial {
            denial_class: DeepLinkDenialClass::ReplayConsumed,
            summary: "deep-link single-use replay was already consumed".to_string(),
        });
    }

    if !intent.handler_ownership_verified {
        return DeepLinkValidationOutcome::Denied(DeepLinkDenial {
            denial_class: DeepLinkDenialClass::HandlerOwnershipLost,
            summary: "handler ownership for this deep-link route is no longer held by the shell"
                .to_string(),
        });
    }

    let mut reviewed_sheet_required = false;
    let mut reasons: Vec<&'static str> = Vec::new();

    if intent.command_class.is_boundary_raising() {
        reviewed_sheet_required = true;
        reasons.push("boundary_raising_command_class");
    }
    if matches!(
        intent.target_class,
        DeepLinkTargetClass::ManagedWorkspace
            | DeepLinkTargetClass::ReviewThread
            | DeepLinkTargetClass::WorkItem
            | DeepLinkTargetClass::CommandTarget
    ) {
        reviewed_sheet_required = true;
        reasons.push("boundary_raising_target_class");
    }
    if matches!(
        intent.origin_class,
        DeepLinkOriginClass::ExternalProvider
            | DeepLinkOriginClass::CollaborationService
            | DeepLinkOriginClass::InstallerOrUpdateFlow
    ) {
        reviewed_sheet_required = true;
        reasons.push("non_local_origin");
    }

    let reviewed_sheet_reason = if reviewed_sheet_required {
        reasons.join("|")
    } else {
        "none".to_string()
    };

    let summary = if reviewed_sheet_required {
        format!(
            "deep-link admitted; reviewed sheet required ({})",
            reviewed_sheet_reason
        )
    } else {
        "deep-link admitted; no reviewed sheet required".to_string()
    };

    DeepLinkValidationOutcome::Admitted(DeepLinkAdmission {
        reviewed_sheet_required,
        reviewed_sheet_reason,
        summary,
    })
}

/// Builds a [`DeepLinkValidationRecord`] from an intent.
pub fn materialize_deep_link_validation_record(intent: DeepLinkIntent) -> DeepLinkValidationRecord {
    let outcome = validate_deep_link_intent(&intent);
    DeepLinkValidationRecord {
        record_kind: "deep_link_validation_record".to_string(),
        deep_link_validation_schema_version: 1,
        intent,
        outcome,
    }
}

/// Writes a deep-link validation record to
/// `<recovery_root>/deep_link_validation_latest.json`.
pub fn write_deep_link_validation_log(
    recovery_root: &std::path::Path,
    record: &DeepLinkValidationRecord,
) -> Result<(), String> {
    std::fs::create_dir_all(recovery_root)
        .map_err(|err| format!("create recovery root failed: {err}"))?;
    let path = recovery_root.join("deep_link_validation_latest.json");
    let json = serde_json::to_string_pretty(record)
        .map_err(|err| format!("serialize deep link validation failed: {err}"))?;
    std::fs::write(&path, json).map_err(|err| format!("write {} failed: {err}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn intent(
        origin: DeepLinkOriginClass,
        target: DeepLinkTargetClass,
        command: DeepLinkCommandClass,
    ) -> DeepLinkIntent {
        DeepLinkIntent {
            intent_id: "intent:test:01".to_string(),
            origin_class: origin,
            target_class: target,
            command_class: command,
            route_label: "redacted-route".to_string(),
            replay_consumed: false,
            handler_ownership_verified: true,
        }
    }

    #[test]
    fn unknown_untrusted_origin_is_denied() {
        let outcome = validate_deep_link_intent(&intent(
            DeepLinkOriginClass::UnknownUntrusted,
            DeepLinkTargetClass::WorkspaceRoot,
            DeepLinkCommandClass::OpenExistingContext,
        ));
        match outcome {
            DeepLinkValidationOutcome::Denied(denial) => {
                assert_eq!(denial.denial_class, DeepLinkDenialClass::OriginUnverified);
            }
            other => panic!("expected denied, got {other:?}"),
        }
    }

    #[test]
    fn unknown_target_is_denied() {
        let outcome = validate_deep_link_intent(&intent(
            DeepLinkOriginClass::SystemDefaultBrowser,
            DeepLinkTargetClass::UnknownTarget,
            DeepLinkCommandClass::OpenExistingContext,
        ));
        assert_eq!(
            outcome,
            DeepLinkValidationOutcome::Denied(DeepLinkDenial {
                denial_class: DeepLinkDenialClass::TargetUnresolved,
                summary: "deep-link target_class is unknown_target; reopen disambiguation before execution".to_string(),
            })
        );
    }

    #[test]
    fn replay_consumed_is_denied() {
        let mut intent = intent(
            DeepLinkOriginClass::SystemDefaultBrowser,
            DeepLinkTargetClass::WorkspaceRoot,
            DeepLinkCommandClass::OpenExistingContext,
        );
        intent.replay_consumed = true;
        let outcome = validate_deep_link_intent(&intent);
        match outcome {
            DeepLinkValidationOutcome::Denied(denial) => {
                assert_eq!(denial.denial_class, DeepLinkDenialClass::ReplayConsumed);
            }
            other => panic!("expected denied, got {other:?}"),
        }
    }

    #[test]
    fn handler_ownership_lost_is_denied() {
        let mut intent = intent(
            DeepLinkOriginClass::SystemDefaultBrowser,
            DeepLinkTargetClass::WorkspaceRoot,
            DeepLinkCommandClass::OpenExistingContext,
        );
        intent.handler_ownership_verified = false;
        let outcome = validate_deep_link_intent(&intent);
        match outcome {
            DeepLinkValidationOutcome::Denied(denial) => {
                assert_eq!(
                    denial.denial_class,
                    DeepLinkDenialClass::HandlerOwnershipLost
                );
            }
            other => panic!("expected denied, got {other:?}"),
        }
    }

    #[test]
    fn local_workspace_open_admitted_without_review() {
        let outcome = validate_deep_link_intent(&intent(
            DeepLinkOriginClass::SystemDefaultBrowser,
            DeepLinkTargetClass::WorkspaceRoot,
            DeepLinkCommandClass::OpenExistingContext,
        ));
        match outcome {
            DeepLinkValidationOutcome::Admitted(admission) => {
                assert!(!admission.reviewed_sheet_required);
                assert_eq!(admission.reviewed_sheet_reason, "none");
            }
            other => panic!("expected admitted, got {other:?}"),
        }
    }

    #[test]
    fn managed_workspace_resume_requires_reviewed_sheet() {
        let outcome = validate_deep_link_intent(&intent(
            DeepLinkOriginClass::ExternalProvider,
            DeepLinkTargetClass::ManagedWorkspace,
            DeepLinkCommandClass::ResumeSession,
        ));
        match outcome {
            DeepLinkValidationOutcome::Admitted(admission) => {
                assert!(admission.reviewed_sheet_required);
                assert!(admission
                    .reviewed_sheet_reason
                    .contains("boundary_raising_command_class"));
                assert!(admission
                    .reviewed_sheet_reason
                    .contains("boundary_raising_target_class"));
                assert!(admission.reviewed_sheet_reason.contains("non_local_origin"));
            }
            other => panic!("expected admitted, got {other:?}"),
        }
    }

    #[test]
    fn mutating_command_request_requires_reviewed_sheet_even_locally() {
        let outcome = validate_deep_link_intent(&intent(
            DeepLinkOriginClass::OsShell,
            DeepLinkTargetClass::CommandTarget,
            DeepLinkCommandClass::MutatingCommandRequest,
        ));
        assert!(outcome.requires_reviewed_sheet());
    }

    #[test]
    fn fixture_deeplink_admitted_round_trips() {
        let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "../../fixtures/ux/restore_and_deeplink_cases/deeplink_admitted_workspace_open.json",
        );
        let payload = std::fs::read_to_string(&fixture_path).expect("fixture must read");
        let record: DeepLinkValidationRecord =
            serde_json::from_str(&payload).expect("fixture must parse");
        assert_eq!(record.record_kind, "deep_link_validation_record");
        match record.outcome {
            DeepLinkValidationOutcome::Admitted(admission) => {
                assert!(!admission.reviewed_sheet_required);
            }
            other => panic!("fixture should be admitted, got {other:?}"),
        }
    }

    #[test]
    fn fixture_deeplink_denied_round_trips() {
        let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "../../fixtures/ux/restore_and_deeplink_cases/deeplink_denied_unknown_origin.json",
        );
        let payload = std::fs::read_to_string(&fixture_path).expect("fixture must read");
        let record: DeepLinkValidationRecord =
            serde_json::from_str(&payload).expect("fixture must parse");
        assert_eq!(record.record_kind, "deep_link_validation_record");
        match record.outcome {
            DeepLinkValidationOutcome::Denied(denial) => {
                assert_eq!(denial.denial_class, DeepLinkDenialClass::OriginUnverified);
            }
            other => panic!("fixture should be denied, got {other:?}"),
        }
    }

    #[test]
    fn fixture_deeplink_review_required_round_trips() {
        let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "../../fixtures/ux/restore_and_deeplink_cases/deeplink_review_required_managed_resume.json",
        );
        let payload = std::fs::read_to_string(&fixture_path).expect("fixture must read");
        let record: DeepLinkValidationRecord =
            serde_json::from_str(&payload).expect("fixture must parse");
        assert!(record.outcome.requires_reviewed_sheet());
    }
}
