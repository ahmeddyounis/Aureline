//! Offline-capture and browser-blocked handoff continuity for the M5 help,
//! support, and community-handoff surfaces.
//!
//! This module is the in-product producer of the durable [`HandoffDraftState`]
//! that survives when a public/community/support handoff is delayed or fails.
//! When a browser is blocked, the machine is offline, or a route is policy-denied,
//! the user's work is preserved instead of discarded: the drafted text, the chosen
//! attachments, the redaction choices, and — crucially — the intended destination
//! *trust class* all persist so the user never has to recreate them from scratch.
//!
//! - **Offline capture stays first-class.** Every live draft sets
//!   `offline_capture_first_class` and `draft_reusable_offline`, and its
//!   [`DataExitBoundary`] for what has actually left the product is always
//!   [`DataExitBoundary::NoPayloadLeavesProduct`] — a blocked or offline handoff
//!   degrades to a labeled, reusable local draft, never a dead-end error.
//! - **Target-class truth is preserved.** Each draft pins the intended
//!   [`DestinationTrustClass`] (official public, official authenticated, community,
//!   private/security, local only), its [`VisibilityBoundaryClass`], and the
//!   `intended_data_exit_boundary` the handoff will obey once it succeeds.
//!   `preserves_target_class_on_retry` and `preserves_visibility_boundary_on_export`
//!   are always true, and `auto_redirect_to_reachable_target_allowed` is always
//!   false, so a failed security/private route is never silently rerouted to a
//!   more-reachable public/community target.
//! - **Every failure offers a way forward.** A live draft always exposes the full
//!   [`ContinuityActionClass`] set — retry, export packet, open target later,
//!   switch target class (always an explicit user action), and clear draft — so a
//!   failure never strands the user.
//! - **Nothing persists invisibly.** Every draft is visible to the user
//!   (`persisted_state_visible_to_user`) under a declared [`RetentionScopeClass`]
//!   with a clear-draft action, so a persisted draft never outlives its declared
//!   retention or profile scope without visible state and a clear action.
//!
//! The redaction vocabulary ([`RedactableFieldClass`], [`RedactionActionClass`],
//! [`RedactionPostureClass`]) is reused from the M5 reproduction-packet contract
//! ([`crate::m5_reproduction_packets`]) and the destination/trust vocabulary
//! ([`DestinationTrustClass`], [`VisibilityBoundaryClass`]) from the M5
//! community-handoff target contract ([`crate::m5_community_handoff_targets`]) so a
//! preserved draft carries exactly the same versioned, redaction-safe boundary the
//! preview and route surfaces already publish — the user never has to infer scope
//! from a raw payload.
//!
//! Raw URLs, raw email addresses, raw local paths, raw usernames, raw hostnames,
//! tokens, and raw secret material never cross this boundary; the records carry
//! opaque refs, controlled-vocabulary tokens, and bounded reviewable sentences
//! only. The drafted text body lives in local storage and is named here by opaque
//! ref and character count, never inlined raw.
//!
//! The boundary schema is
//! [`schemas/help/m5-handoff-draft-state.schema.json`](../../../../schemas/help/m5-handoff-draft-state.schema.json).
//! The contract doc is
//! [`docs/help/m5_handoff_continuity_contract.md`](../../../../docs/help/m5_handoff_continuity_contract.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_cleared_draft_state, seeded_m5_handoff_continuity_scenario_set,
    seeded_offline_security_draft_state, M5_HANDOFF_CONTINUITY_SCENARIO_SET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use crate::m5_community_handoff_targets::{DestinationTrustClass, VisibilityBoundaryClass};
pub use crate::m5_reproduction_packets::{
    RedactableFieldClass, RedactionActionClass, RedactionPostureClass,
};
pub use crate::public_truth::DataExitBoundary;

/// Stable record-kind tag carried by [`HandoffDraftState`].
pub const HANDOFF_DRAFT_STATE_RECORD_KIND: &str = "handoff_draft_state_record";

/// Stable record-kind tag carried by [`M5HandoffContinuityScenarioSet`].
pub const M5_HANDOFF_CONTINUITY_SCENARIO_SET_RECORD_KIND: &str =
    "m5_handoff_continuity_scenario_set";

/// Schema version for a single handoff draft state.
pub const HANDOFF_DRAFT_STATE_SCHEMA_VERSION: u32 = 1;

/// Schema version for the bundled scenario set.
pub const M5_HANDOFF_CONTINUITY_SCENARIO_SET_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema this producer projects.
pub const M5_HANDOFF_DRAFT_STATE_SCHEMA_REF: &str =
    "schemas/help/m5-handoff-draft-state.schema.json";

/// Repo-relative path of the contract doc all records point at.
pub const M5_HANDOFF_CONTINUITY_CONTRACT_DOC_REF: &str =
    "docs/help/m5_handoff_continuity_contract.md";

/// Repo-relative path of the reproduction-packet contract whose redaction
/// vocabulary this lane reuses.
pub const M5_HANDOFF_CONTINUITY_REPRO_PACKET_REF: &str =
    "schemas/help/m5-reproduction-packet.schema.json";

/// Repo-relative path of the community-handoff target contract whose
/// destination/trust vocabulary this lane reuses.
pub const M5_HANDOFF_CONTINUITY_HANDOFF_TARGET_REF: &str =
    "schemas/help/m5-handoff-target.schema.json";

/// Repo-relative path of the frozen M5 public-handoff matrix that governs whether
/// this lane may eventually open a route.
pub const M5_HANDOFF_CONTINUITY_PUBLIC_MATRIX_REF: &str =
    "schemas/help/m5-public-handoff-matrix.schema.json";

/// Repo-relative path of the checked support-export artifact.
pub const M5_HANDOFF_CONTINUITY_ARTIFACT_REF: &str =
    "artifacts/help/m5-handoff-continuity-proof/draft_state_set.json";

/// Why the first handoff launch attempt failed, leaving a draft to be preserved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffFailureClass {
    /// The system browser could not be launched (blocked or unavailable).
    BrowserBlocked,
    /// There is no network connectivity to reach the target.
    NoNetworkOffline,
    /// A policy or managed profile denied opening the route.
    PolicyDenied,
    /// The handoff launched but failed before delivering.
    HandoffLaunchFailed,
    /// The current deployment profile does not support this route.
    UnsupportedProfile,
}

impl HandoffFailureClass {
    /// Every failure class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::BrowserBlocked,
        Self::NoNetworkOffline,
        Self::PolicyDenied,
        Self::HandoffLaunchFailed,
        Self::UnsupportedProfile,
    ];

    /// Stable token recorded on the draft.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BrowserBlocked => "browser_blocked",
            Self::NoNetworkOffline => "no_network_offline",
            Self::PolicyDenied => "policy_denied",
            Self::HandoffLaunchFailed => "handoff_launch_failed",
            Self::UnsupportedProfile => "unsupported_profile",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::BrowserBlocked => "Browser blocked",
            Self::NoNetworkOffline => "No network / offline",
            Self::PolicyDenied => "Policy denied",
            Self::HandoffLaunchFailed => "Handoff launch failed",
            Self::UnsupportedProfile => "Unsupported profile",
        }
    }
}

/// The explicit continuity state of a preserved draft.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DraftContinuityState {
    /// Captured and saved locally after a blocked/offline attempt; reusable.
    CapturedOffline,
    /// Queued to retry the original handoff; the draft is preserved.
    AwaitingRetry,
    /// Staged so the user can open the target later, on their own schedule.
    StagedForLater,
    /// Exported to a local packet that never leaves the product.
    ExportedLocally,
    /// The user explicitly cleared the draft; the work is discarded.
    Cleared,
}

impl DraftContinuityState {
    /// Every continuity state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::CapturedOffline,
        Self::AwaitingRetry,
        Self::StagedForLater,
        Self::ExportedLocally,
        Self::Cleared,
    ];

    /// Stable token recorded on the draft.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CapturedOffline => "captured_offline",
            Self::AwaitingRetry => "awaiting_retry",
            Self::StagedForLater => "staged_for_later",
            Self::ExportedLocally => "exported_locally",
            Self::Cleared => "cleared",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CapturedOffline => "Captured offline",
            Self::AwaitingRetry => "Awaiting retry",
            Self::StagedForLater => "Staged for later",
            Self::ExportedLocally => "Exported locally",
            Self::Cleared => "Cleared",
        }
    }

    /// True when the draft's work has been discarded.
    pub const fn is_cleared(self) -> bool {
        matches!(self, Self::Cleared)
    }
}

/// The continuity actions a preserved draft supports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityActionClass {
    /// Retry the original handoff to the same target.
    Retry,
    /// Export the draft as a local, redaction-safe packet.
    ExportPacket,
    /// Keep the draft and open the target later.
    OpenTargetLater,
    /// Switch to a different target trust class (always an explicit user choice).
    SwitchTargetClass,
    /// Discard the draft and its preserved work.
    ClearDraft,
}

impl ContinuityActionClass {
    /// Every continuity action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Retry,
        Self::ExportPacket,
        Self::OpenTargetLater,
        Self::SwitchTargetClass,
        Self::ClearDraft,
    ];

    /// Stable token recorded on the draft.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Retry => "retry",
            Self::ExportPacket => "export_packet",
            Self::OpenTargetLater => "open_target_later",
            Self::SwitchTargetClass => "switch_target_class",
            Self::ClearDraft => "clear_draft",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Retry => "Retry",
            Self::ExportPacket => "Export packet",
            Self::OpenTargetLater => "Open target later",
            Self::SwitchTargetClass => "Switch target class",
            Self::ClearDraft => "Clear draft",
        }
    }
}

/// How long a preserved draft is retained. Every scope is visible to the user and
/// clearable, so nothing persists invisibly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionScopeClass {
    /// Kept only for the current session.
    SessionOnly,
    /// Kept until the user clears it.
    UntilUserClears,
    /// Kept within the current profile's scope.
    ProfileScopedWindow,
    /// Kept for a declared, bounded retention window.
    DeclaredRetentionWindow,
}

impl RetentionScopeClass {
    /// Stable token recorded on the draft.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionOnly => "session_only",
            Self::UntilUserClears => "until_user_clears",
            Self::ProfileScopedWindow => "profile_scoped_window",
            Self::DeclaredRetentionWindow => "declared_retention_window",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SessionOnly => "Session only",
            Self::UntilUserClears => "Until user clears",
            Self::ProfileScopedWindow => "Profile-scoped window",
            Self::DeclaredRetentionWindow => "Declared retention window",
        }
    }
}

/// The kind of attachment a user chose to include with a draft.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttachmentClass {
    /// A redacted excerpt of a log.
    LogExcerpt,
    /// A screenshot with redaction applied.
    RedactedScreenshot,
    /// A sanitized configuration snapshot.
    ConfigSnapshot,
    /// A redaction-safe diagnostic bundle.
    DiagnosticBundle,
    /// A free-text reproduction-steps note.
    ReproStepsNote,
    /// Any other redaction-safe artifact.
    OtherArtifact,
}

impl AttachmentClass {
    /// Stable token recorded on the draft.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LogExcerpt => "log_excerpt",
            Self::RedactedScreenshot => "redacted_screenshot",
            Self::ConfigSnapshot => "config_snapshot",
            Self::DiagnosticBundle => "diagnostic_bundle",
            Self::ReproStepsNote => "repro_steps_note",
            Self::OtherArtifact => "other_artifact",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LogExcerpt => "Log excerpt",
            Self::RedactedScreenshot => "Redacted screenshot",
            Self::ConfigSnapshot => "Config snapshot",
            Self::DiagnosticBundle => "Diagnostic bundle",
            Self::ReproStepsNote => "Repro steps note",
            Self::OtherArtifact => "Other artifact",
        }
    }
}

/// The exact anchor / object identity a draft is about, so the preserved report
/// names a precise locus rather than a fuzzy description.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectAnchor {
    /// Opaque ref of the originating anchor (surface, position, selection).
    pub anchor_ref: String,
    /// Opaque ref of the object the report is about.
    pub object_ref: String,
    /// Reviewer-facing anchor label.
    pub anchor_label: String,
}

/// A preserved snapshot of the drafted text. The body lives in local storage; this
/// record names it by opaque ref and character count, never inlining raw text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DraftTextSnapshot {
    /// Opaque ref of the locally persisted draft body.
    pub text_ref: String,
    /// The character count of the drafted text, so the user sees their work is
    /// intact without exporting it.
    pub character_count: u32,
    /// Whether redaction has been applied to the preserved text; always true.
    pub redaction_applied: bool,
    /// A bounded reviewable sentence describing the preserved text.
    pub text_summary: String,
}

impl DraftTextSnapshot {
    fn validate(&self, draft_id: &str) -> Result<(), HandoffContinuityError> {
        if !ref_is_opaque(&self.text_ref) {
            return Err(HandoffContinuityError::RawRefLeak {
                record_id: draft_id.to_owned(),
                field: "drafted_text.text_ref",
            });
        }
        if self.character_count == 0 {
            return Err(HandoffContinuityError::EmptyDraftText {
                draft_id: draft_id.to_owned(),
            });
        }
        if !self.redaction_applied {
            return Err(HandoffContinuityError::TextNotRedacted {
                draft_id: draft_id.to_owned(),
            });
        }
        if non_empty(&self.text_summary).is_none() {
            return Err(HandoffContinuityError::EmptyRequiredField {
                record_id: draft_id.to_owned(),
                field: "drafted_text.text_summary",
            });
        }
        Ok(())
    }
}

/// One redaction-safe attachment the user chose to keep with the draft.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DraftAttachment {
    /// The kind of attachment.
    pub attachment_class: AttachmentClass,
    /// Opaque ref of the attachment in local storage.
    pub attachment_ref: String,
    /// Whether redaction has been applied; always true.
    pub redaction_applied: bool,
    /// Whether the user explicitly selected this attachment; always true.
    pub selected_by_user: bool,
    /// A bounded reviewable sentence describing the attachment.
    pub attachment_summary: String,
}

impl DraftAttachment {
    fn validate(&self, draft_id: &str) -> Result<(), HandoffContinuityError> {
        if !ref_is_opaque(&self.attachment_ref) {
            return Err(HandoffContinuityError::RawRefLeak {
                record_id: draft_id.to_owned(),
                field: "attachments.attachment_ref",
            });
        }
        if !self.redaction_applied {
            return Err(HandoffContinuityError::AttachmentNotRedacted {
                draft_id: draft_id.to_owned(),
            });
        }
        if !self.selected_by_user {
            return Err(HandoffContinuityError::AttachmentNotUserSelected {
                draft_id: draft_id.to_owned(),
            });
        }
        if non_empty(&self.attachment_summary).is_none() {
            return Err(HandoffContinuityError::EmptyRequiredField {
                record_id: draft_id.to_owned(),
                field: "attachments.attachment_summary",
            });
        }
        Ok(())
    }
}

/// One row of the preserved redaction choices: a captured sensitive field, the
/// action Aureline proposed, and the action the user picked. Reuses the
/// reproduction-packet redaction vocabulary so the choices survive a failed handoff
/// unchanged.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedactionChoiceRow {
    /// The kind of sensitive field this row redacts.
    pub field_class: RedactableFieldClass,
    /// The redaction action Aureline proposed for this field.
    pub default_action: RedactionActionClass,
    /// The redaction action the user chose; may tighten the default, never loosen
    /// it.
    pub chosen_action: RedactionActionClass,
    /// Whether this field cannot be loosened below its default (always true for
    /// tokens and secrets).
    pub mandatory_redaction: bool,
    /// A bounded reviewable sentence describing the preserved choice.
    pub choice_summary: String,
}

impl RedactionChoiceRow {
    fn validate(&self, draft_id: &str) -> Result<(), HandoffContinuityError> {
        if !self.field_class.allows_action(self.default_action) {
            return Err(HandoffContinuityError::FieldActionNotAllowed {
                draft_id: draft_id.to_owned(),
                field: self.field_class,
                action: self.default_action,
            });
        }
        if !self.field_class.allows_action(self.chosen_action) {
            return Err(HandoffContinuityError::FieldActionNotAllowed {
                draft_id: draft_id.to_owned(),
                field: self.field_class,
                action: self.chosen_action,
            });
        }
        // The user may only tighten a row: the chosen action must not expose more
        // than the proposed default.
        if self.chosen_action.exposure_level() > self.default_action.exposure_level() {
            return Err(HandoffContinuityError::ChosenLoosensRedaction {
                draft_id: draft_id.to_owned(),
                field: self.field_class,
            });
        }
        // A field that must always be removed is removed by both default and chosen
        // action and is flagged mandatory.
        if self.field_class.is_always_removed()
            && (self.default_action != RedactionActionClass::RemovedEntirely
                || self.chosen_action != RedactionActionClass::RemovedEntirely
                || !self.mandatory_redaction)
        {
            return Err(HandoffContinuityError::MandatoryFieldNotRemoved {
                draft_id: draft_id.to_owned(),
                field: self.field_class,
            });
        }
        if non_empty(&self.choice_summary).is_none() {
            return Err(HandoffContinuityError::EmptyRequiredField {
                record_id: draft_id.to_owned(),
                field: "redaction_state.choice_summary",
            });
        }
        Ok(())
    }
}

/// One persisted handoff draft state that survives a blocked, offline, or failed
/// handoff and lets the user resume without recreating their work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffDraftState {
    /// Schema version for this draft-state shape.
    pub handoff_draft_state_schema_version: u32,
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable draft id; prefixed `handoff_draft:`.
    pub draft_id: String,
    /// Why the first launch attempt failed.
    pub failure_class: HandoffFailureClass,
    /// The explicit continuity state of the draft.
    pub continuity_state: DraftContinuityState,
    /// The intended destination trust class, preserved through the failure.
    pub intended_trust_class: DestinationTrustClass,
    /// The visibility boundary the intended route applies once it succeeds.
    pub visibility_boundary: VisibilityBoundaryClass,
    /// What has actually left the product so far; always
    /// [`DataExitBoundary::NoPayloadLeavesProduct`] while the draft is preserved.
    pub current_data_exit_boundary: DataExitBoundary,
    /// The data-exit boundary the handoff will obey once it eventually succeeds.
    pub intended_data_exit_boundary: DataExitBoundary,
    /// The redaction posture prepared for the intended destination.
    pub redaction_posture: RedactionPostureClass,
    /// The exact object anchor of the locus of concern.
    pub object_anchor: ObjectAnchor,
    /// The preserved drafted text; `None` only when the draft was cleared.
    pub drafted_text: Option<DraftTextSnapshot>,
    /// The preserved chosen attachments.
    pub attachments: Vec<DraftAttachment>,
    /// The preserved redaction choices, one row per captured sensitive field.
    pub redaction_state: Vec<RedactionChoiceRow>,
    /// The continuity actions this draft offers.
    pub available_actions: Vec<ContinuityActionClass>,
    /// The declared retention scope of the persisted draft.
    pub retention_scope: RetentionScopeClass,
    /// Whether the draft stays reusable offline; true for every live draft, false
    /// once cleared.
    pub draft_reusable_offline: bool,
    /// Whether the persisted state is visible to the user; always true.
    pub persisted_state_visible_to_user: bool,
    /// Whether offline capture is treated as a first-class state; always true.
    pub offline_capture_first_class: bool,
    /// Whether retry preserves the intended target class; always true.
    pub preserves_target_class_on_retry: bool,
    /// Whether export preserves the visibility-boundary truth; always true.
    pub preserves_visibility_boundary_on_export: bool,
    /// Whether switching target class requires an explicit user action; true
    /// whenever the switch action is offered.
    pub target_switch_requires_explicit_user_action: bool,
    /// Whether a failed route may be auto-redirected to a reachable target; always
    /// false so a security/private route is never silently made public.
    pub auto_redirect_to_reachable_target_allowed: bool,
    /// Reviewer-facing headline label.
    pub headline_label: String,
    /// A bounded reviewable sentence summarizing the draft state.
    pub draft_summary: String,
    /// Frozen contract-doc ref.
    pub contract_doc_ref: String,
    /// Optional reviewer note.
    pub notes: Option<String>,
}

impl HandoffDraftState {
    /// Validate the draft state against the handoff-continuity contract.
    pub fn validate(&self) -> Result<(), HandoffContinuityError> {
        if self.handoff_draft_state_schema_version != HANDOFF_DRAFT_STATE_SCHEMA_VERSION {
            return Err(HandoffContinuityError::WrongDraftSchemaVersion {
                draft_id: self.draft_id.clone(),
                actual: self.handoff_draft_state_schema_version,
            });
        }
        if self.record_kind != HANDOFF_DRAFT_STATE_RECORD_KIND {
            return Err(HandoffContinuityError::WrongDraftRecordKind {
                draft_id: self.draft_id.clone(),
                actual: self.record_kind.clone(),
            });
        }
        if !self.draft_id.starts_with("handoff_draft:") {
            return Err(HandoffContinuityError::MalformedDraftId {
                draft_id: self.draft_id.clone(),
            });
        }
        if self.contract_doc_ref != M5_HANDOFF_CONTINUITY_CONTRACT_DOC_REF {
            return Err(HandoffContinuityError::WrongContractDocRef {
                record_id: self.draft_id.clone(),
                actual: self.contract_doc_ref.clone(),
            });
        }

        for (field, value) in [
            ("headline_label", &self.headline_label),
            ("draft_summary", &self.draft_summary),
            (
                "object_anchor.anchor_label",
                &self.object_anchor.anchor_label,
            ),
        ] {
            if non_empty(value).is_none() {
                return Err(HandoffContinuityError::EmptyRequiredField {
                    record_id: self.draft_id.clone(),
                    field,
                });
            }
        }
        if !ref_is_opaque(&self.object_anchor.anchor_ref)
            || !ref_is_opaque(&self.object_anchor.object_ref)
        {
            return Err(HandoffContinuityError::RawRefLeak {
                record_id: self.draft_id.clone(),
                field: "object_anchor",
            });
        }

        // Continuity guarantees: offline capture is first-class, retry preserves
        // the target class, export preserves the visibility boundary, the user can
        // always see the persisted state, and a failed route is never silently
        // redirected to a reachable target.
        if !self.offline_capture_first_class {
            return Err(HandoffContinuityError::OfflineCaptureNotFirstClass {
                draft_id: self.draft_id.clone(),
            });
        }
        if !self.preserves_target_class_on_retry || !self.preserves_visibility_boundary_on_export {
            return Err(HandoffContinuityError::TargetClassNotPreserved {
                draft_id: self.draft_id.clone(),
            });
        }
        if !self.persisted_state_visible_to_user {
            return Err(HandoffContinuityError::PersistedStateNotVisible {
                draft_id: self.draft_id.clone(),
            });
        }
        if self.auto_redirect_to_reachable_target_allowed {
            return Err(HandoffContinuityError::SilentRouteRedirectAllowed {
                draft_id: self.draft_id.clone(),
            });
        }

        // Nothing has left the product while a draft is being preserved.
        if self.current_data_exit_boundary != DataExitBoundary::NoPayloadLeavesProduct {
            return Err(HandoffContinuityError::PreservedDraftLeftProduct {
                draft_id: self.draft_id.clone(),
                data_exit: self.current_data_exit_boundary,
            });
        }

        // Target-class truth: trust class pins the visibility boundary, the
        // intended data exit, and the redaction posture.
        if !self
            .visibility_boundary
            .allowed_for_trust(self.intended_trust_class)
        {
            return Err(HandoffContinuityError::TrustVisibilityMismatch {
                draft_id: self.draft_id.clone(),
                trust: self.intended_trust_class,
                visibility: self.visibility_boundary,
            });
        }
        if !trust_allows_data_exit(self.intended_trust_class, self.intended_data_exit_boundary) {
            return Err(HandoffContinuityError::TrustDataExitMismatch {
                draft_id: self.draft_id.clone(),
                trust: self.intended_trust_class,
                data_exit: self.intended_data_exit_boundary,
            });
        }
        if !self
            .redaction_posture
            .allows_data_exit(self.intended_data_exit_boundary)
        {
            return Err(HandoffContinuityError::PostureDataExitMismatch {
                draft_id: self.draft_id.clone(),
                posture: self.redaction_posture,
                data_exit: self.intended_data_exit_boundary,
            });
        }

        // Switching target class is never automatic: when the action is offered it
        // must require an explicit user action.
        let offers_switch = self
            .available_actions
            .contains(&ContinuityActionClass::SwitchTargetClass);
        if offers_switch && !self.target_switch_requires_explicit_user_action {
            return Err(HandoffContinuityError::TargetSwitchNotExplicit {
                draft_id: self.draft_id.clone(),
            });
        }

        // Actions must be unique.
        let mut seen_actions: BTreeSet<ContinuityActionClass> = BTreeSet::new();
        for action in &self.available_actions {
            if !seen_actions.insert(*action) {
                return Err(HandoffContinuityError::DuplicateAction {
                    draft_id: self.draft_id.clone(),
                    action: *action,
                });
            }
        }

        if self.continuity_state.is_cleared() {
            // A cleared draft has discarded the user's work: no actions, no text,
            // no attachments, no redaction rows, and it is no longer reusable.
            if !self.available_actions.is_empty() {
                return Err(HandoffContinuityError::ClearedDraftHasActions {
                    draft_id: self.draft_id.clone(),
                });
            }
            if self.drafted_text.is_some()
                || !self.attachments.is_empty()
                || !self.redaction_state.is_empty()
            {
                return Err(HandoffContinuityError::ClearedDraftRetainsWork {
                    draft_id: self.draft_id.clone(),
                });
            }
            if self.draft_reusable_offline {
                return Err(HandoffContinuityError::ClearedDraftStillReusable {
                    draft_id: self.draft_id.clone(),
                });
            }
        } else {
            // A live draft preserves the user's work and offers every continuity
            // action so a failure never dead-ends.
            for action in ContinuityActionClass::ALL {
                if !seen_actions.contains(&action) {
                    return Err(HandoffContinuityError::MissingContinuityAction {
                        draft_id: self.draft_id.clone(),
                        action,
                    });
                }
            }
            if !self.draft_reusable_offline {
                return Err(HandoffContinuityError::LiveDraftNotReusable {
                    draft_id: self.draft_id.clone(),
                });
            }
            match &self.drafted_text {
                Some(text) => text.validate(&self.draft_id)?,
                None => {
                    return Err(HandoffContinuityError::LiveDraftMissingText {
                        draft_id: self.draft_id.clone(),
                    })
                }
            }
            for attachment in &self.attachments {
                attachment.validate(&self.draft_id)?;
            }
            // Redaction choices are preserved and valid; no field repeats.
            if self.redaction_state.is_empty() {
                return Err(HandoffContinuityError::EmptyRedactionState {
                    draft_id: self.draft_id.clone(),
                });
            }
            let mut seen_fields: BTreeSet<RedactableFieldClass> = BTreeSet::new();
            for row in &self.redaction_state {
                row.validate(&self.draft_id)?;
                if !seen_fields.insert(row.field_class) {
                    return Err(HandoffContinuityError::DuplicateRedactionField {
                        draft_id: self.draft_id.clone(),
                        field: row.field_class,
                    });
                }
            }
        }

        Ok(())
    }

    /// Render a deterministic, redaction-safe plaintext continuity summary — a
    /// reviewer-facing preview of the preserved draft. Stable for the same input
    /// snapshot.
    pub fn render_continuity_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("[{}] {}\n", self.draft_id, self.headline_label));
        out.push_str(&format!(
            "    failure={} state={} reusable_offline={}\n",
            self.failure_class.as_str(),
            self.continuity_state.as_str(),
            self.draft_reusable_offline,
        ));
        out.push_str(&format!(
            "    intended trust={} visibility={} intended_data_exit={} current_data_exit={}\n",
            self.intended_trust_class.as_str(),
            self.visibility_boundary.as_str(),
            self.intended_data_exit_boundary.as_str(),
            self.current_data_exit_boundary.as_str(),
        ));
        out.push_str(&format!(
            "    anchor: {} (object={})\n",
            self.object_anchor.anchor_ref, self.object_anchor.object_ref,
        ));
        if let Some(text) = &self.drafted_text {
            out.push_str(&format!(
                "    drafted text: {} ({} chars)\n",
                text.text_ref, text.character_count,
            ));
        }
        for attachment in &self.attachments {
            out.push_str(&format!(
                "    attachment: {} ({})\n",
                attachment.attachment_class.as_str(),
                attachment.attachment_ref,
            ));
        }
        out.push_str("    redaction choices:\n");
        for row in &self.redaction_state {
            out.push_str(&format!(
                "      - {} -> {}\n",
                row.field_class.as_str(),
                row.chosen_action.as_str(),
            ));
        }
        out.push_str("    actions:");
        for action in &self.available_actions {
            out.push_str(&format!(" {}", action.as_str()));
        }
        out.push('\n');
        out
    }
}

/// A bundled set of handoff draft states, one per failure scenario, checked in as
/// the canonical M5 source for offline-capture and browser-blocked handoff
/// continuity truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HandoffContinuityScenarioSet {
    /// Schema version for the scenario-set shape.
    pub schema_version: u32,
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable id for the scenario set.
    pub scenario_set_id: String,
    /// Reviewer-facing label for the scenario set.
    pub scenario_set_label: String,
    /// One draft per failure scenario.
    pub drafts: Vec<HandoffDraftState>,
    /// Source contracts this set binds to by id.
    pub source_contract_refs: Vec<String>,
    /// Redaction-class token covering the export boundary.
    pub redaction_class_token: String,
    /// Opaque mint timestamp ref.
    pub minted_at: String,
    /// Frozen contract-doc ref.
    pub contract_doc_ref: String,
}

impl M5HandoffContinuityScenarioSet {
    /// Validate the scenario set: every draft validates, every failure class,
    /// trust class, continuity state, and continuity action is represented, every
    /// redactable field is covered by some live draft, official and community
    /// routes stay distinguishable, no two drafts share an id, and the source
    /// contracts are present.
    pub fn validate(&self) -> Result<(), HandoffContinuityError> {
        if self.schema_version != M5_HANDOFF_CONTINUITY_SCENARIO_SET_SCHEMA_VERSION {
            return Err(HandoffContinuityError::WrongSetSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_HANDOFF_CONTINUITY_SCENARIO_SET_RECORD_KIND {
            return Err(HandoffContinuityError::WrongSetRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        if non_empty(&self.scenario_set_id).is_none()
            || non_empty(&self.scenario_set_label).is_none()
            || non_empty(&self.redaction_class_token).is_none()
            || non_empty(&self.minted_at).is_none()
        {
            return Err(HandoffContinuityError::SetIdentityIncomplete);
        }
        if self.contract_doc_ref != M5_HANDOFF_CONTINUITY_CONTRACT_DOC_REF {
            return Err(HandoffContinuityError::WrongContractDocRef {
                record_id: self.scenario_set_id.clone(),
                actual: self.contract_doc_ref.clone(),
            });
        }

        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for draft in &self.drafts {
            draft.validate()?;
            if !seen.insert(draft.draft_id.as_str()) {
                return Err(HandoffContinuityError::DuplicateDraftId {
                    draft_id: draft.draft_id.clone(),
                });
            }
        }

        // Every failure class is named by some draft.
        for failure in HandoffFailureClass::ALL {
            if !self.drafts.iter().any(|d| d.failure_class == failure) {
                return Err(HandoffContinuityError::FailureClassMissing { failure });
            }
        }

        // Every continuity state is exercised by some draft.
        for state in DraftContinuityState::ALL {
            if !self.drafts.iter().any(|d| d.continuity_state == state) {
                return Err(HandoffContinuityError::ContinuityStateMissing { state });
            }
        }

        // Every intended trust class is preserved by some draft, so each of
        // official, community, security/private, and local-only stays first-class.
        for trust in DestinationTrustClass::ALL {
            if !self.drafts.iter().any(|d| d.intended_trust_class == trust) {
                return Err(HandoffContinuityError::TrustClassMissing { trust });
            }
        }

        // Every continuity action is offered by some draft.
        for action in ContinuityActionClass::ALL {
            if !self
                .drafts
                .iter()
                .any(|d| d.available_actions.contains(&action))
            {
                return Err(HandoffContinuityError::ContinuityActionMissing { action });
            }
        }

        // Official and community routes stay distinguishable.
        if !self
            .drafts
            .iter()
            .any(|d| d.intended_trust_class.is_official())
        {
            return Err(HandoffContinuityError::OfficialRouteMissing);
        }
        if !self
            .drafts
            .iter()
            .any(|d| d.intended_trust_class.is_community())
        {
            return Err(HandoffContinuityError::CommunityRouteMissing);
        }

        // Every redactable field is covered by some live draft's preserved
        // redaction choices, so a sensitive field never slips through unmodeled.
        for field in RedactableFieldClass::ALL {
            let covered = self
                .drafts
                .iter()
                .any(|d| d.redaction_state.iter().any(|r| r.field_class == field));
            if !covered {
                return Err(HandoffContinuityError::FieldClassUncovered { field });
            }
        }

        // Source contracts bound by id.
        let refs: BTreeSet<&str> = self
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        for required in [
            M5_HANDOFF_DRAFT_STATE_SCHEMA_REF,
            M5_HANDOFF_CONTINUITY_CONTRACT_DOC_REF,
            M5_HANDOFF_CONTINUITY_REPRO_PACKET_REF,
            M5_HANDOFF_CONTINUITY_HANDOFF_TARGET_REF,
            M5_HANDOFF_CONTINUITY_PUBLIC_MATRIX_REF,
        ] {
            if !refs.contains(required) {
                return Err(HandoffContinuityError::MissingSourceContracts);
            }
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("handoff continuity scenario set serializes"),
        ) {
            return Err(HandoffContinuityError::RawMaterialInExport);
        }

        Ok(())
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only set fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("handoff continuity scenario set serializes")
    }

    /// Deterministic, machine-readable CSV: one row per draft, naming its failure
    /// class, intended trust class, visibility boundary, continuity state,
    /// intended data-exit boundary, action count, and the target-class-preserved
    /// gate.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "draft,failure_class,intended_trust_class,visibility_boundary,continuity_state,intended_data_exit_boundary,actions,preserves_target_class\n",
        );
        for draft in &self.drafts {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                draft.draft_id,
                draft.failure_class.as_str(),
                draft.intended_trust_class.as_str(),
                draft.visibility_boundary.as_str(),
                draft.continuity_state.as_str(),
                draft.intended_data_exit_boundary.as_str(),
                draft.available_actions.len(),
                draft.preserves_target_class_on_retry,
            ));
        }
        out
    }

    /// Deterministic Markdown governance summary for support, docs, or review
    /// handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 handoff-continuity review\n\n");
        out.push_str(&format!("Scenario set: `{}`\n\n", self.scenario_set_id));
        out.push_str(
            "| Draft | Failure | Intended trust | Visibility | State | Intended data exit | Reusable offline? |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for draft in &self.drafts {
            out.push_str(&format!(
                "| `{}` | {} | {} | {} | {} | `{}` | {} |\n",
                draft.draft_id,
                draft.failure_class.label(),
                draft.intended_trust_class.label(),
                draft.visibility_boundary.label(),
                draft.continuity_state.label(),
                draft.intended_data_exit_boundary.as_str(),
                draft.draft_reusable_offline,
            ));
        }
        out.push('\n');
        out.push_str("Every preserved draft keeps the drafted text, attachments, redaction choices, and intended target class, ");
        out.push_str("nothing leaves the product while a draft is held, and a failed security/private route is never silently redirected to a public target.\n");
        out
    }
}

/// Whether an intended trust class permits the given data-exit boundary. Mirrors
/// the community-handoff target contract so the intended boundary stays consistent
/// with where the handoff is headed.
fn trust_allows_data_exit(trust: DestinationTrustClass, data_exit: DataExitBoundary) -> bool {
    use DataExitBoundary as D;
    use DestinationTrustClass as T;
    match trust {
        T::OfficialPublic => matches!(
            data_exit,
            D::NoPayloadLeavesProduct
                | D::MetadataSafeObjectRefs
                | D::ProposalRefsOnly
                | D::ExternalPublicBrowse
        ),
        T::Community => matches!(
            data_exit,
            D::NoPayloadLeavesProduct | D::MetadataSafeObjectRefs | D::ProposalRefsOnly
        ),
        T::OfficialAuthenticated => matches!(
            data_exit,
            D::RedactedSupportPacket | D::MetadataSafeObjectRefs | D::NoPayloadLeavesProduct
        ),
        T::PrivateSecurity => matches!(data_exit, D::SecurityPayloadsOnly),
        T::LocalOnly => matches!(data_exit, D::NoPayloadLeavesProduct),
    }
}

/// True when a ref is an opaque token rather than a raw URL, email, or blank.
fn ref_is_opaque(value: &str) -> bool {
    let trimmed = value.trim();
    !trimmed.is_empty()
        && trimmed == value
        && !trimmed.contains("://")
        && !trimmed.contains('@')
        && !trimmed.contains(char::is_whitespace)
}

fn non_empty(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Closed validation-error vocabulary for the handoff-continuity contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandoffContinuityError {
    WrongDraftSchemaVersion {
        draft_id: String,
        actual: u32,
    },
    WrongDraftRecordKind {
        draft_id: String,
        actual: String,
    },
    MalformedDraftId {
        draft_id: String,
    },
    OfflineCaptureNotFirstClass {
        draft_id: String,
    },
    TargetClassNotPreserved {
        draft_id: String,
    },
    PersistedStateNotVisible {
        draft_id: String,
    },
    SilentRouteRedirectAllowed {
        draft_id: String,
    },
    PreservedDraftLeftProduct {
        draft_id: String,
        data_exit: DataExitBoundary,
    },
    TrustVisibilityMismatch {
        draft_id: String,
        trust: DestinationTrustClass,
        visibility: VisibilityBoundaryClass,
    },
    TrustDataExitMismatch {
        draft_id: String,
        trust: DestinationTrustClass,
        data_exit: DataExitBoundary,
    },
    PostureDataExitMismatch {
        draft_id: String,
        posture: RedactionPostureClass,
        data_exit: DataExitBoundary,
    },
    TargetSwitchNotExplicit {
        draft_id: String,
    },
    DuplicateAction {
        draft_id: String,
        action: ContinuityActionClass,
    },
    MissingContinuityAction {
        draft_id: String,
        action: ContinuityActionClass,
    },
    LiveDraftNotReusable {
        draft_id: String,
    },
    LiveDraftMissingText {
        draft_id: String,
    },
    EmptyDraftText {
        draft_id: String,
    },
    TextNotRedacted {
        draft_id: String,
    },
    AttachmentNotRedacted {
        draft_id: String,
    },
    AttachmentNotUserSelected {
        draft_id: String,
    },
    EmptyRedactionState {
        draft_id: String,
    },
    DuplicateRedactionField {
        draft_id: String,
        field: RedactableFieldClass,
    },
    FieldActionNotAllowed {
        draft_id: String,
        field: RedactableFieldClass,
        action: RedactionActionClass,
    },
    ChosenLoosensRedaction {
        draft_id: String,
        field: RedactableFieldClass,
    },
    MandatoryFieldNotRemoved {
        draft_id: String,
        field: RedactableFieldClass,
    },
    ClearedDraftHasActions {
        draft_id: String,
    },
    ClearedDraftRetainsWork {
        draft_id: String,
    },
    ClearedDraftStillReusable {
        draft_id: String,
    },
    WrongSetSchemaVersion {
        actual: u32,
    },
    WrongSetRecordKind {
        actual: String,
    },
    SetIdentityIncomplete,
    DuplicateDraftId {
        draft_id: String,
    },
    FailureClassMissing {
        failure: HandoffFailureClass,
    },
    ContinuityStateMissing {
        state: DraftContinuityState,
    },
    TrustClassMissing {
        trust: DestinationTrustClass,
    },
    ContinuityActionMissing {
        action: ContinuityActionClass,
    },
    OfficialRouteMissing,
    CommunityRouteMissing,
    FieldClassUncovered {
        field: RedactableFieldClass,
    },
    MissingSourceContracts,
    RawMaterialInExport,
    WrongContractDocRef {
        record_id: String,
        actual: String,
    },
    EmptyRequiredField {
        record_id: String,
        field: &'static str,
    },
    RawRefLeak {
        record_id: String,
        field: &'static str,
    },
}

impl fmt::Display for HandoffContinuityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongDraftSchemaVersion { draft_id, actual } => write!(
                f,
                "draft {draft_id} has unsupported handoff_draft_state_schema_version {actual}"
            ),
            Self::WrongDraftRecordKind { draft_id, actual } => {
                write!(f, "draft {draft_id} has unsupported record kind {actual}")
            }
            Self::MalformedDraftId { draft_id } => {
                write!(f, "draft id {draft_id} must start with handoff_draft:")
            }
            Self::OfflineCaptureNotFirstClass { draft_id } => write!(
                f,
                "draft {draft_id} must treat offline capture as first-class"
            ),
            Self::TargetClassNotPreserved { draft_id } => write!(
                f,
                "draft {draft_id} must preserve the target class on retry and the visibility boundary on export"
            ),
            Self::PersistedStateNotVisible { draft_id } => write!(
                f,
                "draft {draft_id} persists without a visible state for the user"
            ),
            Self::SilentRouteRedirectAllowed { draft_id } => write!(
                f,
                "draft {draft_id} must not allow a failed route to auto-redirect to a reachable target"
            ),
            Self::PreservedDraftLeftProduct { draft_id, data_exit } => write!(
                f,
                "draft {draft_id} is preserved but reports data exit {}; nothing should have left the product",
                data_exit.as_str()
            ),
            Self::TrustVisibilityMismatch {
                draft_id,
                trust,
                visibility,
            } => write!(
                f,
                "draft {draft_id} trust class {} cannot use visibility boundary {}",
                trust.as_str(),
                visibility.as_str()
            ),
            Self::TrustDataExitMismatch {
                draft_id,
                trust,
                data_exit,
            } => write!(
                f,
                "draft {draft_id} trust class {} cannot use intended data exit {}",
                trust.as_str(),
                data_exit.as_str()
            ),
            Self::PostureDataExitMismatch {
                draft_id,
                posture,
                data_exit,
            } => write!(
                f,
                "draft {draft_id} posture {} cannot use intended data exit {}",
                posture.as_str(),
                data_exit.as_str()
            ),
            Self::TargetSwitchNotExplicit { draft_id } => write!(
                f,
                "draft {draft_id} offers switch-target-class but does not require an explicit user action"
            ),
            Self::DuplicateAction { draft_id, action } => write!(
                f,
                "draft {draft_id} repeats continuity action {}",
                action.as_str()
            ),
            Self::MissingContinuityAction { draft_id, action } => write!(
                f,
                "draft {draft_id} is live but does not offer continuity action {}",
                action.as_str()
            ),
            Self::LiveDraftNotReusable { draft_id } => {
                write!(f, "draft {draft_id} is live but not reusable offline")
            }
            Self::LiveDraftMissingText { draft_id } => {
                write!(f, "draft {draft_id} is live but has no preserved drafted text")
            }
            Self::EmptyDraftText { draft_id } => {
                write!(f, "draft {draft_id} preserves an empty drafted text")
            }
            Self::TextNotRedacted { draft_id } => {
                write!(f, "draft {draft_id} preserved text is not redacted")
            }
            Self::AttachmentNotRedacted { draft_id } => {
                write!(f, "draft {draft_id} carries an unredacted attachment")
            }
            Self::AttachmentNotUserSelected { draft_id } => {
                write!(f, "draft {draft_id} carries an attachment not selected by the user")
            }
            Self::EmptyRedactionState { draft_id } => write!(
                f,
                "draft {draft_id} is live but preserves no redaction choices"
            ),
            Self::DuplicateRedactionField { draft_id, field } => write!(
                f,
                "draft {draft_id} repeats redaction field {}",
                field.as_str()
            ),
            Self::FieldActionNotAllowed {
                draft_id,
                field,
                action,
            } => write!(
                f,
                "draft {draft_id} field {} cannot take redaction action {}",
                field.as_str(),
                action.as_str()
            ),
            Self::ChosenLoosensRedaction { draft_id, field } => write!(
                f,
                "draft {draft_id} field {} chosen action loosens the proposed redaction",
                field.as_str()
            ),
            Self::MandatoryFieldNotRemoved { draft_id, field } => write!(
                f,
                "draft {draft_id} field {} must always be removed entirely",
                field.as_str()
            ),
            Self::ClearedDraftHasActions { draft_id } => write!(
                f,
                "draft {draft_id} is cleared but still offers continuity actions"
            ),
            Self::ClearedDraftRetainsWork { draft_id } => write!(
                f,
                "draft {draft_id} is cleared but still retains text, attachments, or redaction choices"
            ),
            Self::ClearedDraftStillReusable { draft_id } => {
                write!(f, "draft {draft_id} is cleared but still marked reusable offline")
            }
            Self::WrongSetSchemaVersion { actual } => {
                write!(f, "scenario set has unsupported schema_version {actual}")
            }
            Self::WrongSetRecordKind { actual } => {
                write!(f, "scenario set has unsupported record kind {actual}")
            }
            Self::SetIdentityIncomplete => {
                write!(f, "scenario set is missing required identity fields")
            }
            Self::DuplicateDraftId { draft_id } => {
                write!(f, "scenario set has duplicate draft id {draft_id}")
            }
            Self::FailureClassMissing { failure } => {
                write!(f, "scenario set is missing failure class {}", failure.as_str())
            }
            Self::ContinuityStateMissing { state } => {
                write!(f, "scenario set is missing continuity state {}", state.as_str())
            }
            Self::TrustClassMissing { trust } => {
                write!(f, "scenario set never preserves trust class {}", trust.as_str())
            }
            Self::ContinuityActionMissing { action } => {
                write!(f, "scenario set never offers continuity action {}", action.as_str())
            }
            Self::OfficialRouteMissing => {
                write!(f, "scenario set must preserve at least one official route")
            }
            Self::CommunityRouteMissing => {
                write!(f, "scenario set must preserve at least one community route")
            }
            Self::FieldClassUncovered { field } => write!(
                f,
                "scenario set never preserves redaction field {}",
                field.as_str()
            ),
            Self::MissingSourceContracts => {
                write!(f, "scenario set is missing a required source contract ref")
            }
            Self::RawMaterialInExport => {
                write!(f, "scenario set export carries forbidden raw material")
            }
            Self::WrongContractDocRef { record_id, actual } => {
                write!(f, "record {record_id} cites wrong contract doc {actual}")
            }
            Self::EmptyRequiredField { record_id, field } => {
                write!(f, "record {record_id} is missing required field {field}")
            }
            Self::RawRefLeak { record_id, field } => write!(
                f,
                "record {record_id} field {field} contains a raw URL, email, or whitespace; opaque refs only"
            ),
        }
    }
}

impl Error for HandoffContinuityError {}

/// Reads and validates the checked-in stable handoff-continuity scenario set.
pub fn current_stable_m5_handoff_continuity_scenario_set(
) -> Result<M5HandoffContinuityScenarioSet, Box<dyn Error>> {
    let set: M5HandoffContinuityScenarioSet = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/help/m5-handoff-continuity-proof/draft_state_set.json"
    )))?;
    set.validate()?;
    Ok(set)
}
