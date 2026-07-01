//! Canonical seed for the M5 handoff-continuity scenario set, plus the two
//! narrowed scenario drafts used as protected fixtures.
//!
//! The seed builder is the single mint-from-truth path: the checked-in support
//! export, governance summary, matrix CSV, and fixtures are projections of these
//! functions, and the module tests prove the on-disk artifacts deserialize back to
//! exactly these values.

use super::{
    AttachmentClass, ContinuityActionClass, DataExitBoundary, DestinationTrustClass,
    DraftAttachment, DraftContinuityState, DraftTextSnapshot, HandoffDraftState,
    HandoffFailureClass, M5HandoffContinuityScenarioSet, ObjectAnchor, RedactableFieldClass,
    RedactionActionClass, RedactionChoiceRow, RedactionPostureClass, RetentionScopeClass,
    VisibilityBoundaryClass, HANDOFF_DRAFT_STATE_RECORD_KIND, HANDOFF_DRAFT_STATE_SCHEMA_VERSION,
    M5_HANDOFF_CONTINUITY_CONTRACT_DOC_REF, M5_HANDOFF_CONTINUITY_HANDOFF_TARGET_REF,
    M5_HANDOFF_CONTINUITY_PUBLIC_MATRIX_REF, M5_HANDOFF_CONTINUITY_REPRO_PACKET_REF,
    M5_HANDOFF_CONTINUITY_SCENARIO_SET_RECORD_KIND,
    M5_HANDOFF_CONTINUITY_SCENARIO_SET_SCHEMA_VERSION, M5_HANDOFF_DRAFT_STATE_SCHEMA_REF,
};

/// Stable id of the canonical scenario set.
pub const M5_HANDOFF_CONTINUITY_SCENARIO_SET_ID: &str =
    "m5_handoff_continuity_scenario_set:default";

fn redaction_row(
    field_class: RedactableFieldClass,
    default_action: RedactionActionClass,
    chosen_action: RedactionActionClass,
    mandatory_redaction: bool,
    choice_summary: &str,
) -> RedactionChoiceRow {
    RedactionChoiceRow {
        field_class,
        default_action,
        chosen_action,
        mandatory_redaction,
        choice_summary: choice_summary.to_owned(),
    }
}

/// The full preserved redaction choices: one row per captured sensitive field, each
/// defaulting to a redaction-safe action, so the user's redaction work survives the
/// failed handoff unchanged.
fn full_redaction_state() -> Vec<RedactionChoiceRow> {
    use RedactableFieldClass as F;
    use RedactionActionClass as A;
    vec![
        redaction_row(
            F::LocalPath,
            A::RedactedPlaceholder,
            A::RedactedPlaceholder,
            false,
            "Local paths stay shown as a project-root placeholder; the user's choice is preserved.",
        ),
        redaction_row(
            F::Username,
            A::RedactedPlaceholder,
            A::RedactedPlaceholder,
            false,
            "The operating-system username stays replaced with a placeholder.",
        ),
        redaction_row(
            F::Hostname,
            A::GeneralizedClass,
            A::GeneralizedClass,
            false,
            "The hostname stays generalized to a class label, not the real machine name.",
        ),
        redaction_row(
            F::Token,
            A::RemovedEntirely,
            A::RemovedEntirely,
            true,
            "Token and credential values stay removed entirely and are never persisted in the draft.",
        ),
        redaction_row(
            F::ExtensionInventory,
            A::IncludedAsObjectRef,
            A::IncludedAsObjectRef,
            false,
            "The extension inventory stays carried as an opaque inventory ref, not a raw list.",
        ),
        redaction_row(
            F::DeploymentProfile,
            A::GeneralizedClass,
            A::GeneralizedClass,
            false,
            "The deployment profile stays carried as a class label only.",
        ),
        redaction_row(
            F::LinkedDiagnostic,
            A::IncludedAsObjectRef,
            A::IncludedAsObjectRef,
            false,
            "Linked diagnostics stay carried as opaque object refs after redaction.",
        ),
    ]
}

fn attachment(class: AttachmentClass, ref_token: &str, summary: &str) -> DraftAttachment {
    DraftAttachment {
        attachment_class: class,
        attachment_ref: ref_token.to_owned(),
        redaction_applied: true,
        selected_by_user: true,
        attachment_summary: summary.to_owned(),
    }
}

fn draft_text(text_ref: &str, character_count: u32, summary: &str) -> DraftTextSnapshot {
    DraftTextSnapshot {
        text_ref: text_ref.to_owned(),
        character_count,
        redaction_applied: true,
        text_summary: summary.to_owned(),
    }
}

/// The full set of continuity actions every live draft offers, so a failure never
/// dead-ends.
fn all_actions() -> Vec<ContinuityActionClass> {
    ContinuityActionClass::ALL.to_vec()
}

#[allow(clippy::too_many_arguments)]
fn live_draft(
    draft_id: &str,
    failure: HandoffFailureClass,
    state: DraftContinuityState,
    trust: DestinationTrustClass,
    visibility: VisibilityBoundaryClass,
    intended_data_exit: DataExitBoundary,
    posture: RedactionPostureClass,
    retention: RetentionScopeClass,
    anchor_ref: &str,
    object_ref: &str,
    anchor_label: &str,
    text: DraftTextSnapshot,
    attachments: Vec<DraftAttachment>,
    redaction_state: Vec<RedactionChoiceRow>,
    headline_label: &str,
    draft_summary: &str,
) -> HandoffDraftState {
    HandoffDraftState {
        handoff_draft_state_schema_version: HANDOFF_DRAFT_STATE_SCHEMA_VERSION,
        record_kind: HANDOFF_DRAFT_STATE_RECORD_KIND.to_owned(),
        draft_id: draft_id.to_owned(),
        failure_class: failure,
        continuity_state: state,
        intended_trust_class: trust,
        visibility_boundary: visibility,
        current_data_exit_boundary: DataExitBoundary::NoPayloadLeavesProduct,
        intended_data_exit_boundary: intended_data_exit,
        redaction_posture: posture,
        object_anchor: ObjectAnchor {
            anchor_ref: anchor_ref.to_owned(),
            object_ref: object_ref.to_owned(),
            anchor_label: anchor_label.to_owned(),
        },
        drafted_text: Some(text),
        attachments,
        redaction_state,
        available_actions: all_actions(),
        retention_scope: retention,
        draft_reusable_offline: true,
        persisted_state_visible_to_user: true,
        offline_capture_first_class: true,
        preserves_target_class_on_retry: true,
        preserves_visibility_boundary_on_export: true,
        target_switch_requires_explicit_user_action: true,
        auto_redirect_to_reachable_target_allowed: false,
        headline_label: headline_label.to_owned(),
        draft_summary: draft_summary.to_owned(),
        contract_doc_ref: M5_HANDOFF_CONTINUITY_CONTRACT_DOC_REF.to_owned(),
        notes: None,
    }
}

/// A browser-blocked public-issue draft captured offline, with the official-public
/// route preserved.
fn draft_browser_blocked_public_issue() -> HandoffDraftState {
    live_draft(
        "handoff_draft:browser_blocked_public_issue",
        HandoffFailureClass::BrowserBlocked,
        DraftContinuityState::CapturedOffline,
        DestinationTrustClass::OfficialPublic,
        VisibilityBoundaryClass::WorldReadablePublic,
        DataExitBoundary::MetadataSafeObjectRefs,
        RedactionPostureClass::FullyRedactedPublicSafe,
        RetentionScopeClass::UntilUserClears,
        "anchor.issue.public_tracker",
        "object.issue.subject",
        "Public issue subject",
        draft_text(
            "draft.text.public_issue",
            420,
            "The drafted public-issue text is preserved locally after the browser could not be launched.",
        ),
        vec![attachment(
            AttachmentClass::LogExcerpt,
            "attach.public_issue.log_excerpt",
            "A redacted log excerpt the user attached to the public-issue draft.",
        )],
        full_redaction_state(),
        "Public issue draft saved offline",
        "A public-issue draft preserved offline after the browser was blocked; the official-public target class is kept.",
    )
}

/// A no-network community-support draft captured offline, with the community route
/// preserved.
fn draft_offline_community_support() -> HandoffDraftState {
    live_draft(
        "handoff_draft:offline_community_support",
        HandoffFailureClass::NoNetworkOffline,
        DraftContinuityState::CapturedOffline,
        DestinationTrustClass::Community,
        VisibilityBoundaryClass::CommunityVisible,
        DataExitBoundary::MetadataSafeObjectRefs,
        RedactionPostureClass::FullyRedactedPublicSafe,
        RetentionScopeClass::UntilUserClears,
        "anchor.community.forum_thread",
        "object.community.subject",
        "Community thread subject",
        draft_text(
            "draft.text.community_support",
            360,
            "The drafted community-support post is preserved locally while the machine is offline.",
        ),
        vec![attachment(
            AttachmentClass::ReproStepsNote,
            "attach.community.repro_steps",
            "A free-text reproduction-steps note the user attached to the community draft.",
        )],
        full_redaction_state(),
        "Community post saved offline",
        "A community-support draft preserved while offline; the community target class and visibility are kept.",
    )
}

/// A policy-denied security-disclosure draft staged for later, with the
/// private/security route preserved and never redirected to a public target.
fn draft_policy_denied_security() -> HandoffDraftState {
    use RedactableFieldClass as F;
    use RedactionActionClass as A;
    // The user tightened the hostname row from the generalized default to removed
    // entirely; that choice is preserved through the failure.
    let mut redaction_state = full_redaction_state();
    if let Some(row) = redaction_state
        .iter_mut()
        .find(|r| r.field_class == F::Hostname)
    {
        row.chosen_action = A::RemovedEntirely;
        row.choice_summary =
            "The user tightened the hostname row to removed entirely; the choice is preserved."
                .to_owned();
    }
    live_draft(
        "handoff_draft:policy_denied_security",
        HandoffFailureClass::PolicyDenied,
        DraftContinuityState::StagedForLater,
        DestinationTrustClass::PrivateSecurity,
        VisibilityBoundaryClass::PrivateSecurityChannel,
        DataExitBoundary::SecurityPayloadsOnly,
        RedactionPostureClass::SecurityChannelOnly,
        RetentionScopeClass::DeclaredRetentionWindow,
        "anchor.security.disclosure_surface",
        "object.security.affected_subject",
        "Security disclosure subject",
        draft_text(
            "draft.text.security_disclosure",
            540,
            "The drafted security disclosure is staged locally after the route was policy-denied.",
        ),
        vec![attachment(
            AttachmentClass::DiagnosticBundle,
            "attach.security.diagnostic_bundle",
            "A redaction-safe diagnostic bundle the user attached to the security draft.",
        )],
        redaction_state,
        "Security disclosure staged for later",
        "A security disclosure staged after a policy denial; the private/security target class is preserved and never redirected to a public route.",
    )
}

/// A handoff-launch-failed official-support draft awaiting retry, with the
/// official-authenticated route preserved.
fn draft_launch_failed_official_support() -> HandoffDraftState {
    live_draft(
        "handoff_draft:launch_failed_official_support",
        HandoffFailureClass::HandoffLaunchFailed,
        DraftContinuityState::AwaitingRetry,
        DestinationTrustClass::OfficialAuthenticated,
        VisibilityBoundaryClass::OfficialAccountVisible,
        DataExitBoundary::RedactedSupportPacket,
        RedactionPostureClass::RedactedSupportScoped,
        RetentionScopeClass::UntilUserClears,
        "anchor.support.intake_surface",
        "object.support.subject",
        "Official support intake subject",
        draft_text(
            "draft.text.official_support",
            480,
            "The drafted official-support intake is preserved for retry after the handoff failed to launch.",
        ),
        vec![
            attachment(
                AttachmentClass::ConfigSnapshot,
                "attach.support.config_snapshot",
                "A sanitized config snapshot the user attached to the support draft.",
            ),
            attachment(
                AttachmentClass::RedactedScreenshot,
                "attach.support.redacted_screenshot",
                "A redacted screenshot the user attached to the support draft.",
            ),
        ],
        full_redaction_state(),
        "Official support draft awaiting retry",
        "An official-support draft preserved for retry after the handoff launch failed; the official-authenticated target class is kept.",
    )
}

/// An unsupported-profile draft exported to a local packet, with the local-only
/// route preserved as a labeled local path.
fn draft_unsupported_profile_local() -> HandoffDraftState {
    live_draft(
        "handoff_draft:unsupported_profile_local",
        HandoffFailureClass::UnsupportedProfile,
        DraftContinuityState::ExportedLocally,
        DestinationTrustClass::LocalOnly,
        VisibilityBoundaryClass::LocalNeverLeaves,
        DataExitBoundary::NoPayloadLeavesProduct,
        RedactionPostureClass::MetadataRefsOnly,
        RetentionScopeClass::ProfileScopedWindow,
        "anchor.local.export_surface",
        "object.local.subject",
        "Local export subject",
        draft_text(
            "draft.text.local_export",
            300,
            "The drafted report is exported to a labeled local packet because the profile does not support the route.",
        ),
        vec![attachment(
            AttachmentClass::OtherArtifact,
            "attach.local.other_artifact",
            "A redaction-safe artifact the user attached before exporting the draft locally.",
        )],
        full_redaction_state(),
        "Draft exported to local packet",
        "A draft exported to a labeled local packet after an unsupported-profile handoff; nothing leaves the product.",
    )
}

/// A cleared draft: the user explicitly discarded their work, so nothing is
/// retained.
fn draft_cleared(draft_id: &str) -> HandoffDraftState {
    HandoffDraftState {
        handoff_draft_state_schema_version: HANDOFF_DRAFT_STATE_SCHEMA_VERSION,
        record_kind: HANDOFF_DRAFT_STATE_RECORD_KIND.to_owned(),
        draft_id: draft_id.to_owned(),
        failure_class: HandoffFailureClass::BrowserBlocked,
        continuity_state: DraftContinuityState::Cleared,
        intended_trust_class: DestinationTrustClass::OfficialPublic,
        visibility_boundary: VisibilityBoundaryClass::WorldReadablePublic,
        current_data_exit_boundary: DataExitBoundary::NoPayloadLeavesProduct,
        intended_data_exit_boundary: DataExitBoundary::MetadataSafeObjectRefs,
        redaction_posture: RedactionPostureClass::FullyRedactedPublicSafe,
        object_anchor: ObjectAnchor {
            anchor_ref: "anchor.cleared.origin_surface".to_owned(),
            object_ref: "object.cleared.subject".to_owned(),
            anchor_label: "Cleared draft subject".to_owned(),
        },
        drafted_text: None,
        attachments: Vec::new(),
        redaction_state: Vec::new(),
        available_actions: Vec::new(),
        retention_scope: RetentionScopeClass::SessionOnly,
        draft_reusable_offline: false,
        persisted_state_visible_to_user: true,
        offline_capture_first_class: true,
        preserves_target_class_on_retry: true,
        preserves_visibility_boundary_on_export: true,
        target_switch_requires_explicit_user_action: true,
        auto_redirect_to_reachable_target_allowed: false,
        headline_label: "Draft cleared".to_owned(),
        draft_summary: "The user explicitly cleared the draft; no text, attachments, or redaction choices are retained.".to_owned(),
        contract_doc_ref: M5_HANDOFF_CONTINUITY_CONTRACT_DOC_REF.to_owned(),
        notes: Some(
            "Cleared by explicit user action; the draft is no longer reusable and nothing persists.".to_owned(),
        ),
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        M5_HANDOFF_DRAFT_STATE_SCHEMA_REF.to_owned(),
        M5_HANDOFF_CONTINUITY_CONTRACT_DOC_REF.to_owned(),
        M5_HANDOFF_CONTINUITY_REPRO_PACKET_REF.to_owned(),
        M5_HANDOFF_CONTINUITY_HANDOFF_TARGET_REF.to_owned(),
        M5_HANDOFF_CONTINUITY_PUBLIC_MATRIX_REF.to_owned(),
    ]
}

/// Build the canonical M5 handoff-continuity scenario set.
pub fn seeded_m5_handoff_continuity_scenario_set() -> M5HandoffContinuityScenarioSet {
    M5HandoffContinuityScenarioSet {
        schema_version: M5_HANDOFF_CONTINUITY_SCENARIO_SET_SCHEMA_VERSION,
        record_kind: M5_HANDOFF_CONTINUITY_SCENARIO_SET_RECORD_KIND.to_owned(),
        scenario_set_id: M5_HANDOFF_CONTINUITY_SCENARIO_SET_ID.to_owned(),
        scenario_set_label: "M5 handoff-continuity review".to_owned(),
        drafts: vec![
            draft_browser_blocked_public_issue(),
            draft_offline_community_support(),
            draft_policy_denied_security(),
            draft_launch_failed_official_support(),
            draft_unsupported_profile_local(),
            draft_cleared("handoff_draft:cleared_public_issue"),
        ],
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "fully_redacted_public_safe".to_owned(),
        minted_at: "mint.m5_handoff_continuity_scenario_set".to_owned(),
        contract_doc_ref: M5_HANDOFF_CONTINUITY_CONTRACT_DOC_REF.to_owned(),
    }
}

/// A standalone offline security draft: a private/security route captured offline
/// with no network, proving the security target class is preserved and never
/// redirected to a reachable public target.
pub fn seeded_offline_security_draft_state() -> HandoffDraftState {
    let mut d = draft_policy_denied_security();
    d.draft_id = "handoff_draft:security.offline_capture".to_owned();
    d.failure_class = HandoffFailureClass::NoNetworkOffline;
    d.continuity_state = DraftContinuityState::CapturedOffline;
    d.headline_label = "Security disclosure captured offline".to_owned();
    d.draft_summary =
        "A security disclosure captured offline; the private/security target class is preserved and never redirected to a public route.".to_owned();
    d.notes = Some(
        "Offline capture keeps the private/security route; auto-redirect to a reachable public target is never allowed.".to_owned(),
    );
    d
}

/// A standalone cleared draft, proving the clear-draft action discards the user's
/// work with visible state and nothing retained.
pub fn seeded_cleared_draft_state() -> HandoffDraftState {
    draft_cleared("handoff_draft:cleared.user_cleared")
}
