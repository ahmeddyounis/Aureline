//! Canonical seed for the M5 community-handoff target review sheet set, plus the
//! two narrowed scenario sheets used as protected fixtures.
//!
//! The seed builder is the single mint-from-truth path: the checked-in support
//! export, governance summary, matrix CSV, and fixtures are projections of these
//! functions, and the module tests prove the on-disk artifacts deserialize back
//! to exactly these values.

use super::{
    AuthExpectationClass, BuildContextExport, BuildContextExportClass, CommitmentClass,
    CommitmentHonesty, CommunityHandoffRouteClass, CommunityHandoffTargetSheet, DataExitBoundary,
    DestinationTrustClass, IssueTemplateSupport, LocalSafeFallback,
    M5CommunityHandoffTargetSheetSet, ObjectAnchor, VisibilityBoundaryClass,
    COMMUNITY_HANDOFF_TARGET_SHEET_RECORD_KIND, COMMUNITY_HANDOFF_TARGET_SHEET_SCHEMA_VERSION,
    M5_COMMUNITY_HANDOFF_PACKET_CONTRACT_REF, M5_COMMUNITY_HANDOFF_PUBLIC_MATRIX_REF,
    M5_COMMUNITY_HANDOFF_TARGET_CONTRACT_DOC_REF, M5_COMMUNITY_HANDOFF_TARGET_REVIEW_BASE_REF,
    M5_COMMUNITY_HANDOFF_TARGET_SCHEMA_REF, M5_COMMUNITY_HANDOFF_TARGET_SHEET_SET_RECORD_KIND,
    M5_COMMUNITY_HANDOFF_TARGET_SHEET_SET_SCHEMA_VERSION,
};

/// Stable id of the canonical sheet set.
pub const M5_COMMUNITY_HANDOFF_TARGET_SHEET_SET_ID: &str =
    "m5_community_handoff_target_sheet_set:default";

fn build_context_export(
    class: BuildContextExportClass,
    ref_token: &str,
    summary: &str,
) -> BuildContextExport {
    BuildContextExport {
        export_class: class,
        export_block_ref: ref_token.to_owned(),
        export_block_schema_version: 1,
        redacted_for_audience: class,
        raw_screenshots_excluded: true,
        raw_secrets_excluded: true,
        export_summary: summary.to_owned(),
    }
}

fn local_safe_fallback(ref_token: &str, summary: &str) -> LocalSafeFallback {
    LocalSafeFallback {
        fallback_ref: ref_token.to_owned(),
        trust_class: DestinationTrustClass::LocalOnly,
        visibility_boundary: VisibilityBoundaryClass::LocalNeverLeaves,
        auth_expectation: AuthExpectationClass::LocalNoNetwork,
        data_exit_boundary: DataExitBoundary::NoPayloadLeavesProduct,
        fallback_summary: summary.to_owned(),
    }
}

/// The public-issue route: official, world-readable issue tracker.
fn sheet_public_issue() -> CommunityHandoffTargetSheet {
    CommunityHandoffTargetSheet {
        community_handoff_target_sheet_schema_version: COMMUNITY_HANDOFF_TARGET_SHEET_SCHEMA_VERSION,
        record_kind: COMMUNITY_HANDOFF_TARGET_SHEET_RECORD_KIND.to_owned(),
        target_id: "community_handoff_target:public_issue".to_owned(),
        route_class: CommunityHandoffRouteClass::PublicIssue,
        trust_class: DestinationTrustClass::OfficialPublic,
        visibility_boundary: VisibilityBoundaryClass::WorldReadablePublic,
        auth_expectation: AuthExpectationClass::CommunityAccountTypical,
        data_exit_boundary: DataExitBoundary::MetadataSafeObjectRefs,
        data_exit_note: "Filed on the official public issue tracker; only redaction-safe metadata and object refs leave the product, world-readable to anyone.".to_owned(),
        destination_identity_ref: "destination.official.public_issue_tracker".to_owned(),
        destination_label: "Official public issue tracker".to_owned(),
        recipient_label: "Project maintainers and anyone reading the public tracker".to_owned(),
        commitment_honesty: CommitmentHonesty {
            guaranteed_product_commitment: false,
            commitment_class: CommitmentClass::NoCommitmentPublicForum,
            honesty_note: "Filing a public issue does not guarantee a response or a fix.".to_owned(),
        },
        object_anchor: Some(ObjectAnchor {
            anchor_ref: "anchor.report.origin_surface".to_owned(),
            object_ref: "object.report.subject".to_owned(),
            anchor_label: "Reported object".to_owned(),
        }),
        issue_template: Some(IssueTemplateSupport {
            template_ref: "template.public_issue".to_owned(),
            template_label: "Public issue template".to_owned(),
            export_class: BuildContextExportClass::PublicIssueTemplateBlock,
            carries_structured_fields: true,
            template_summary: "Structured public issue template with redaction-safe build context.".to_owned(),
        }),
        build_context_exports: vec![build_context_export(
            BuildContextExportClass::PublicIssueTemplateBlock,
            "export.public_issue_template_block",
            "Redaction-safe public issue intake block.",
        )],
        safe_fallback_refs: vec!["fallback.copy_refs_to_clipboard".to_owned()],
        local_safe_fallback: local_safe_fallback(
            "fallback.local.save_issue_draft",
            "Save the issue draft locally if the browser handoff is blocked.",
        ),
        requires_prior_review_before_open: true,
        auto_open_from_critical_alert_allowed: false,
        unsupported_profile_disclosure_required: false,
        headline_label: "File a public issue".to_owned(),
        target_summary: "Open the official public issue tracker after reviewing the redacted packet.".to_owned(),
        contract_doc_ref: M5_COMMUNITY_HANDOFF_TARGET_CONTRACT_DOC_REF.to_owned(),
        notes: None,
    }
}

/// The security-disclosure route: private, confidential channel.
fn sheet_security_disclosure() -> CommunityHandoffTargetSheet {
    CommunityHandoffTargetSheet {
        community_handoff_target_sheet_schema_version: COMMUNITY_HANDOFF_TARGET_SHEET_SCHEMA_VERSION,
        record_kind: COMMUNITY_HANDOFF_TARGET_SHEET_RECORD_KIND.to_owned(),
        target_id: "community_handoff_target:security_disclosure".to_owned(),
        route_class: CommunityHandoffRouteClass::SecurityDisclosure,
        trust_class: DestinationTrustClass::PrivateSecurity,
        visibility_boundary: VisibilityBoundaryClass::PrivateSecurityChannel,
        auth_expectation: AuthExpectationClass::SecurityChannelCredential,
        data_exit_boundary: DataExitBoundary::SecurityPayloadsOnly,
        data_exit_note: "Sent only over the private security channel; security payloads stay confidential and never become public.".to_owned(),
        destination_identity_ref: "destination.official.security_channel".to_owned(),
        destination_label: "Private security disclosure channel".to_owned(),
        recipient_label: "Aureline security response team only".to_owned(),
        commitment_honesty: CommitmentHonesty {
            guaranteed_product_commitment: false,
            commitment_class: CommitmentClass::SecurityHandledPrivately,
            honesty_note: "Security reports are handled privately under the disclosure process.".to_owned(),
        },
        object_anchor: Some(ObjectAnchor {
            anchor_ref: "anchor.security.origin_surface".to_owned(),
            object_ref: "object.security.subject".to_owned(),
            anchor_label: "Affected object".to_owned(),
        }),
        issue_template: Some(IssueTemplateSupport {
            template_ref: "template.private_security_intake".to_owned(),
            template_label: "Private security intake".to_owned(),
            export_class: BuildContextExportClass::PrivateSecurityIntakeBlock,
            carries_structured_fields: true,
            template_summary: "Structured private security intake with redaction-safe build context.".to_owned(),
        }),
        build_context_exports: vec![build_context_export(
            BuildContextExportClass::PrivateSecurityIntakeBlock,
            "export.private_security_intake_block",
            "Redaction-safe private security intake block.",
        )],
        safe_fallback_refs: vec!["fallback.encrypted_offline_capture".to_owned()],
        local_safe_fallback: local_safe_fallback(
            "fallback.local.save_security_draft",
            "Save the disclosure draft locally if the encrypted channel is unreachable.",
        ),
        requires_prior_review_before_open: true,
        auto_open_from_critical_alert_allowed: false,
        unsupported_profile_disclosure_required: true,
        headline_label: "Disclose a security issue privately".to_owned(),
        target_summary: "Open the private security disclosure channel after reviewing the redacted packet.".to_owned(),
        contract_doc_ref: M5_COMMUNITY_HANDOFF_TARGET_CONTRACT_DOC_REF.to_owned(),
        notes: None,
    }
}

/// The docs-feedback route: official, world-readable docs feedback.
fn sheet_docs_feedback() -> CommunityHandoffTargetSheet {
    CommunityHandoffTargetSheet {
        community_handoff_target_sheet_schema_version: COMMUNITY_HANDOFF_TARGET_SHEET_SCHEMA_VERSION,
        record_kind: COMMUNITY_HANDOFF_TARGET_SHEET_RECORD_KIND.to_owned(),
        target_id: "community_handoff_target:docs_feedback".to_owned(),
        route_class: CommunityHandoffRouteClass::DocsFeedback,
        trust_class: DestinationTrustClass::OfficialPublic,
        visibility_boundary: VisibilityBoundaryClass::WorldReadablePublic,
        auth_expectation: AuthExpectationClass::CommunityAccountTypical,
        data_exit_boundary: DataExitBoundary::MetadataSafeObjectRefs,
        data_exit_note: "Sent to the official docs feedback tracker; only redaction-safe metadata and the docs anchor leave the product.".to_owned(),
        destination_identity_ref: "destination.official.docs_feedback".to_owned(),
        destination_label: "Official docs feedback".to_owned(),
        recipient_label: "Documentation maintainers and public readers".to_owned(),
        commitment_honesty: CommitmentHonesty {
            guaranteed_product_commitment: false,
            commitment_class: CommitmentClass::NoCommitmentPublicForum,
            honesty_note: "Docs feedback is reviewed on a best-effort basis; no guaranteed turnaround.".to_owned(),
        },
        object_anchor: Some(ObjectAnchor {
            anchor_ref: "anchor.docs.page_section".to_owned(),
            object_ref: "object.docs.page".to_owned(),
            anchor_label: "Documentation page".to_owned(),
        }),
        issue_template: Some(IssueTemplateSupport {
            template_ref: "template.docs_feedback".to_owned(),
            template_label: "Docs feedback template".to_owned(),
            export_class: BuildContextExportClass::PublicIssueTemplateBlock,
            carries_structured_fields: true,
            template_summary: "Docs feedback template anchored to the exact page and section.".to_owned(),
        }),
        build_context_exports: vec![build_context_export(
            BuildContextExportClass::PublicIssueTemplateBlock,
            "export.docs_feedback_block",
            "Redaction-safe docs feedback intake block.",
        )],
        safe_fallback_refs: vec!["fallback.copy_docs_anchor".to_owned()],
        local_safe_fallback: local_safe_fallback(
            "fallback.local.save_docs_feedback_draft",
            "Save the docs feedback draft locally if the browser handoff is blocked.",
        ),
        requires_prior_review_before_open: true,
        auto_open_from_critical_alert_allowed: false,
        unsupported_profile_disclosure_required: false,
        headline_label: "Send docs feedback".to_owned(),
        target_summary: "Open the official docs feedback tracker after reviewing the redacted packet.".to_owned(),
        contract_doc_ref: M5_COMMUNITY_HANDOFF_TARGET_CONTRACT_DOC_REF.to_owned(),
        notes: None,
    }
}

/// The RFC/discussion route: community-run design discussion.
fn sheet_rfc_discussion() -> CommunityHandoffTargetSheet {
    CommunityHandoffTargetSheet {
        community_handoff_target_sheet_schema_version: COMMUNITY_HANDOFF_TARGET_SHEET_SCHEMA_VERSION,
        record_kind: COMMUNITY_HANDOFF_TARGET_SHEET_RECORD_KIND.to_owned(),
        target_id: "community_handoff_target:rfc_discussion".to_owned(),
        route_class: CommunityHandoffRouteClass::RfcDiscussion,
        trust_class: DestinationTrustClass::Community,
        visibility_boundary: VisibilityBoundaryClass::CommunityVisible,
        auth_expectation: AuthExpectationClass::CommunityAccountTypical,
        data_exit_boundary: DataExitBoundary::ProposalRefsOnly,
        data_exit_note: "Posted to the community RFC / discussion board; only the proposal refs you choose leave the product, visible to the community.".to_owned(),
        destination_identity_ref: "destination.community.rfc_board".to_owned(),
        destination_label: "Community RFC / discussion board".to_owned(),
        recipient_label: "The community on the discussion board".to_owned(),
        commitment_honesty: CommitmentHonesty {
            guaranteed_product_commitment: false,
            commitment_class: CommitmentClass::BestEffortCommunity,
            honesty_note: "An RFC is a community discussion, not an accepted commitment.".to_owned(),
        },
        object_anchor: Some(ObjectAnchor {
            anchor_ref: "anchor.rfc.proposal".to_owned(),
            object_ref: "object.rfc.proposal".to_owned(),
            anchor_label: "Proposal".to_owned(),
        }),
        issue_template: Some(IssueTemplateSupport {
            template_ref: "template.rfc_discussion".to_owned(),
            template_label: "RFC discussion template".to_owned(),
            export_class: BuildContextExportClass::CommunityDiscussionBlock,
            carries_structured_fields: true,
            template_summary: "RFC template carrying the proposal refs and discussion prompts.".to_owned(),
        }),
        build_context_exports: vec![build_context_export(
            BuildContextExportClass::CommunityDiscussionBlock,
            "export.rfc_discussion_block",
            "Redaction-safe community discussion block.",
        )],
        safe_fallback_refs: vec!["fallback.copy_proposal_refs".to_owned()],
        local_safe_fallback: local_safe_fallback(
            "fallback.local.save_rfc_draft",
            "Save the RFC draft locally if the browser handoff is blocked.",
        ),
        requires_prior_review_before_open: true,
        auto_open_from_critical_alert_allowed: false,
        unsupported_profile_disclosure_required: false,
        headline_label: "Open an RFC discussion".to_owned(),
        target_summary: "Open the community discussion board after reviewing what the proposal shares.".to_owned(),
        contract_doc_ref: M5_COMMUNITY_HANDOFF_TARGET_CONTRACT_DOC_REF.to_owned(),
        notes: None,
    }
}

/// The community-support route: best-effort community help.
fn sheet_community_support() -> CommunityHandoffTargetSheet {
    CommunityHandoffTargetSheet {
        community_handoff_target_sheet_schema_version: COMMUNITY_HANDOFF_TARGET_SHEET_SCHEMA_VERSION,
        record_kind: COMMUNITY_HANDOFF_TARGET_SHEET_RECORD_KIND.to_owned(),
        target_id: "community_handoff_target:community_support".to_owned(),
        route_class: CommunityHandoffRouteClass::CommunitySupport,
        trust_class: DestinationTrustClass::Community,
        visibility_boundary: VisibilityBoundaryClass::CommunityVisible,
        auth_expectation: AuthExpectationClass::CommunityAccountTypical,
        data_exit_boundary: DataExitBoundary::MetadataSafeObjectRefs,
        data_exit_note: "Posted to the community support forum; only redaction-safe metadata and object refs leave the product, visible to the community.".to_owned(),
        destination_identity_ref: "destination.community.support_forum".to_owned(),
        destination_label: "Community support forum".to_owned(),
        recipient_label: "Community members on the support forum".to_owned(),
        commitment_honesty: CommitmentHonesty {
            guaranteed_product_commitment: false,
            commitment_class: CommitmentClass::BestEffortCommunity,
            honesty_note: "Community support is best-effort help from volunteers, not official support.".to_owned(),
        },
        object_anchor: Some(ObjectAnchor {
            anchor_ref: "anchor.support.origin_surface".to_owned(),
            object_ref: "object.support.subject".to_owned(),
            anchor_label: "Question subject".to_owned(),
        }),
        issue_template: None,
        build_context_exports: vec![build_context_export(
            BuildContextExportClass::CommunityDiscussionBlock,
            "export.community_support_block",
            "Redaction-safe community discussion block.",
        )],
        safe_fallback_refs: vec!["fallback.copy_support_refs".to_owned()],
        local_safe_fallback: local_safe_fallback(
            "fallback.local.save_support_draft",
            "Save the support question locally if the browser handoff is blocked.",
        ),
        requires_prior_review_before_open: true,
        auto_open_from_critical_alert_allowed: false,
        unsupported_profile_disclosure_required: false,
        headline_label: "Ask the community".to_owned(),
        target_summary: "Open the community support forum after reviewing the redacted packet.".to_owned(),
        contract_doc_ref: M5_COMMUNITY_HANDOFF_TARGET_CONTRACT_DOC_REF.to_owned(),
        notes: None,
    }
}

/// The official-support route: authenticated, committed support intake.
fn sheet_official_support() -> CommunityHandoffTargetSheet {
    CommunityHandoffTargetSheet {
        community_handoff_target_sheet_schema_version: COMMUNITY_HANDOFF_TARGET_SHEET_SCHEMA_VERSION,
        record_kind: COMMUNITY_HANDOFF_TARGET_SHEET_RECORD_KIND.to_owned(),
        target_id: "community_handoff_target:official_support".to_owned(),
        route_class: CommunityHandoffRouteClass::OfficialSupport,
        trust_class: DestinationTrustClass::OfficialAuthenticated,
        visibility_boundary: VisibilityBoundaryClass::OfficialAccountVisible,
        auth_expectation: AuthExpectationClass::OfficialAccountRequired,
        data_exit_boundary: DataExitBoundary::RedactedSupportPacket,
        data_exit_note: "Sent to official support behind your authenticated account; a redacted support packet leaves the product, visible only to support.".to_owned(),
        destination_identity_ref: "destination.official.support_intake".to_owned(),
        destination_label: "Official support intake".to_owned(),
        recipient_label: "Official Aureline support, behind your account".to_owned(),
        commitment_honesty: CommitmentHonesty {
            guaranteed_product_commitment: true,
            commitment_class: CommitmentClass::OfficialSupportedCommitment,
            honesty_note: "Official support is a committed channel for accounts with a support plan.".to_owned(),
        },
        object_anchor: Some(ObjectAnchor {
            anchor_ref: "anchor.support.origin_surface".to_owned(),
            object_ref: "object.support.case_subject".to_owned(),
            anchor_label: "Case subject".to_owned(),
        }),
        issue_template: Some(IssueTemplateSupport {
            template_ref: "template.private_support_intake".to_owned(),
            template_label: "Private support intake".to_owned(),
            export_class: BuildContextExportClass::PrivateSupportIntakeBlock,
            carries_structured_fields: true,
            template_summary: "Structured private support intake with redaction-safe build context.".to_owned(),
        }),
        build_context_exports: vec![build_context_export(
            BuildContextExportClass::PrivateSupportIntakeBlock,
            "export.private_support_intake_block",
            "Redaction-safe private support intake block.",
        )],
        safe_fallback_refs: vec!["fallback.copy_support_packet_refs".to_owned()],
        local_safe_fallback: local_safe_fallback(
            "fallback.local.save_support_case_draft",
            "Save the support case draft locally if the authenticated handoff is blocked.",
        ),
        requires_prior_review_before_open: true,
        auto_open_from_critical_alert_allowed: false,
        unsupported_profile_disclosure_required: false,
        headline_label: "Contact official support".to_owned(),
        target_summary: "Open the authenticated official support intake after reviewing the redacted packet.".to_owned(),
        contract_doc_ref: M5_COMMUNITY_HANDOFF_TARGET_CONTRACT_DOC_REF.to_owned(),
        notes: None,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        M5_COMMUNITY_HANDOFF_TARGET_SCHEMA_REF.to_owned(),
        M5_COMMUNITY_HANDOFF_TARGET_CONTRACT_DOC_REF.to_owned(),
        M5_COMMUNITY_HANDOFF_PACKET_CONTRACT_REF.to_owned(),
        M5_COMMUNITY_HANDOFF_TARGET_REVIEW_BASE_REF.to_owned(),
        M5_COMMUNITY_HANDOFF_PUBLIC_MATRIX_REF.to_owned(),
    ]
}

/// Build the canonical M5 community-handoff target review sheet set.
pub fn seeded_m5_community_handoff_target_sheet_set() -> M5CommunityHandoffTargetSheetSet {
    M5CommunityHandoffTargetSheetSet {
        schema_version: M5_COMMUNITY_HANDOFF_TARGET_SHEET_SET_SCHEMA_VERSION,
        record_kind: M5_COMMUNITY_HANDOFF_TARGET_SHEET_SET_RECORD_KIND.to_owned(),
        sheet_set_id: M5_COMMUNITY_HANDOFF_TARGET_SHEET_SET_ID.to_owned(),
        sheet_set_label: "M5 community-handoff target review".to_owned(),
        sheets: vec![
            sheet_public_issue(),
            sheet_security_disclosure(),
            sheet_docs_feedback(),
            sheet_rfc_discussion(),
            sheet_community_support(),
            sheet_official_support(),
        ],
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_object_refs".to_owned(),
        minted_at: "mint.m5_community_handoff_target_sheet_set".to_owned(),
        contract_doc_ref: M5_COMMUNITY_HANDOFF_TARGET_CONTRACT_DOC_REF.to_owned(),
    }
}

/// A standalone security-disclosure sheet for an unsupported profile, proving
/// the private/security route degrades to a labeled local-safe fallback.
pub fn seeded_security_disclosure_sheet_unsupported_profile() -> CommunityHandoffTargetSheet {
    let mut sheet = sheet_security_disclosure();
    sheet.target_id = "community_handoff_target:security_disclosure.unsupported_profile".to_owned();
    sheet.headline_label = "Security disclosure unavailable on this profile".to_owned();
    sheet.target_summary =
        "This profile does not expose the security channel; the disclosure stays local until a supported profile is used.".to_owned();
    sheet.notes = Some(
        "Unsupported profile: the route is labeled and the disclosure is preserved locally rather than dropped.".to_owned(),
    );
    sheet
}

/// A standalone community-support sheet emphasizing it is not a guaranteed
/// product commitment.
pub fn seeded_community_support_sheet_no_commitment() -> CommunityHandoffTargetSheet {
    let mut sheet = sheet_community_support();
    sheet.target_id = "community_handoff_target:community_support.no_commitment".to_owned();
    sheet.commitment_honesty.honesty_note =
        "Community support is volunteer help and carries no guarantee of a response.".to_owned();
    sheet.notes = Some(
        "Community link is labeled best-effort; it does not promise official support.".to_owned(),
    );
    sheet
}
