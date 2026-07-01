//! Canonical seed for the M5 reproduction-packet set, plus the two narrowed
//! scenario packets used as protected fixtures.
//!
//! The seed builder is the single mint-from-truth path: the checked-in support
//! export, governance summary, matrix CSV, and fixtures are projections of these
//! functions, and the module tests prove the on-disk artifacts deserialize back
//! to exactly these values.

use super::{
    DataExitBoundary, IncludedContextClass, IncludedContextItem, M5ReproductionPacketSet,
    ObjectAnchor, OriginatingSurfaceClass, PacketFlowClass, RedactableFieldClass,
    RedactionActionClass, RedactionPostureClass, RedactionPreviewRow, ReproductionPacket,
    M5_REPRODUCTION_PACKET_CONTRACT_DOC_REF, M5_REPRODUCTION_PACKET_HANDOFF_TARGET_REF,
    M5_REPRODUCTION_PACKET_PREVIEW_BASE_REF, M5_REPRODUCTION_PACKET_PUBLIC_MATRIX_REF,
    M5_REPRODUCTION_PACKET_SCHEMA_REF, M5_REPRODUCTION_PACKET_SET_RECORD_KIND,
    M5_REPRODUCTION_PACKET_SET_SCHEMA_VERSION, REPRODUCTION_PACKET_RECORD_KIND,
    REPRODUCTION_PACKET_SCHEMA_VERSION,
};

/// Stable id of the canonical packet set.
pub const M5_REPRODUCTION_PACKET_SET_ID: &str = "m5_reproduction_packet_set:default";

fn redaction_row(
    field_class: RedactableFieldClass,
    default_action: RedactionActionClass,
    chosen_action: RedactionActionClass,
    mandatory_redaction: bool,
    redacted_preview_label: &str,
    field_summary: &str,
) -> RedactionPreviewRow {
    RedactionPreviewRow {
        field_class,
        default_action,
        chosen_action,
        mandatory_redaction,
        redacted_preview_label: redacted_preview_label.to_owned(),
        field_summary: field_summary.to_owned(),
    }
}

/// The full redaction preview every packet shows: one row per sensitive field
/// class the spec names, each defaulting to a redaction-safe action.
fn full_redaction_preview() -> Vec<RedactionPreviewRow> {
    use RedactableFieldClass as F;
    use RedactionActionClass as A;
    vec![
        redaction_row(
            F::LocalPath,
            A::RedactedPlaceholder,
            A::RedactedPlaceholder,
            false,
            "<project-root>/…",
            "Local paths are shown as a project-root placeholder; the absolute path never leaves the machine.",
        ),
        redaction_row(
            F::Username,
            A::RedactedPlaceholder,
            A::RedactedPlaceholder,
            false,
            "<user>",
            "The operating-system username is replaced with a placeholder.",
        ),
        redaction_row(
            F::Hostname,
            A::GeneralizedClass,
            A::GeneralizedClass,
            false,
            "host-a",
            "The hostname is generalized to a class label, not the real machine name.",
        ),
        redaction_row(
            F::Token,
            A::RemovedEntirely,
            A::RemovedEntirely,
            true,
            "removed",
            "Token and credential values are removed entirely and never exported.",
        ),
        redaction_row(
            F::ExtensionInventory,
            A::IncludedAsObjectRef,
            A::IncludedAsObjectRef,
            false,
            "extension-inventory ref",
            "The extension inventory is carried as an opaque inventory ref, not a raw list of names or paths.",
        ),
        redaction_row(
            F::DeploymentProfile,
            A::GeneralizedClass,
            A::GeneralizedClass,
            false,
            "managed-profile class",
            "The deployment profile is carried as a class label only.",
        ),
        redaction_row(
            F::LinkedDiagnostic,
            A::IncludedAsObjectRef,
            A::IncludedAsObjectRef,
            false,
            "diagnostic ref",
            "Linked diagnostics and artifacts are carried as opaque object refs after redaction.",
        ),
    ]
}

fn included_context(
    class: IncludedContextClass,
    ref_token: &str,
    summary: &str,
) -> IncludedContextItem {
    IncludedContextItem {
        context_class: class,
        context_ref: ref_token.to_owned(),
        redaction_applied: true,
        item_summary: summary.to_owned(),
    }
}

fn standard_included_context() -> Vec<IncludedContextItem> {
    vec![
        included_context(
            IncludedContextClass::BuildIdentity,
            "context.build_identity",
            "Redaction-safe build identity: version, channel, and commit ref.",
        ),
        included_context(
            IncludedContextClass::EnvironmentCapsule,
            "context.environment_capsule",
            "Redaction-safe environment capsule: class facts only, no raw paths or hostnames.",
        ),
        included_context(
            IncludedContextClass::AnchorObjectRef,
            "context.anchor_object_ref",
            "Opaque ref of the anchored object so the recipient lands on the same locus.",
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn packet(
    packet_id: &str,
    surface: OriginatingSurfaceClass,
    flow: PacketFlowClass,
    posture: RedactionPostureClass,
    data_exit: DataExitBoundary,
    preview_confirmed_before_share: bool,
    offline_reusable: bool,
    anchor_ref: &str,
    object_ref: &str,
    anchor_label: &str,
    headline_label: &str,
    packet_summary: &str,
    included: Vec<IncludedContextItem>,
) -> ReproductionPacket {
    ReproductionPacket {
        reproduction_packet_schema_version: REPRODUCTION_PACKET_SCHEMA_VERSION,
        record_kind: REPRODUCTION_PACKET_RECORD_KIND.to_owned(),
        packet_id: packet_id.to_owned(),
        originating_surface: surface,
        object_anchor: ObjectAnchor {
            anchor_ref: anchor_ref.to_owned(),
            object_ref: object_ref.to_owned(),
            anchor_label: anchor_label.to_owned(),
        },
        flow,
        redaction_posture: posture,
        data_exit_boundary: data_exit,
        redaction_preview: full_redaction_preview(),
        included_context: included,
        preview_confirmed_before_share,
        offline_reusable,
        auto_submit_on_create_allowed: false,
        raw_secrets_excluded: true,
        raw_screenshots_excluded: true,
        hidden_approvals_excluded: true,
        unmanaged_capture_excluded: true,
        headline_label: headline_label.to_owned(),
        packet_summary: packet_summary.to_owned(),
        contract_doc_ref: M5_REPRODUCTION_PACKET_CONTRACT_DOC_REF.to_owned(),
        notes: None,
    }
}

/// The docs-pane packet: copy a redaction-safe summary about a docs problem.
fn packet_docs_pane() -> ReproductionPacket {
    packet(
        "reproduction_packet:docs_pane",
        OriginatingSurfaceClass::DocsPane,
        PacketFlowClass::CopySummary,
        RedactionPostureClass::MetadataRefsOnly,
        DataExitBoundary::MetadataSafeObjectRefs,
        true,
        false,
        "anchor.docs.page_section",
        "object.docs.page",
        "Documentation page section",
        "Report a docs problem",
        "Copy a redaction-safe summary anchored to the exact docs page and section.",
        standard_included_context(),
    )
}

/// The trust-warning packet: stage a security-scoped report for later submission.
fn packet_trust_warning() -> ReproductionPacket {
    let mut included = standard_included_context();
    included.push(included_context(
        IncludedContextClass::RedactedLogTail,
        "context.redacted_log_tail",
        "Redacted tail of the log around the trust warning; secrets and tokens stripped.",
    ));
    packet(
        "reproduction_packet:trust_warning",
        OriginatingSurfaceClass::TrustWarning,
        PacketFlowClass::SubmitLater,
        RedactionPostureClass::SecurityChannelOnly,
        DataExitBoundary::SecurityPayloadsOnly,
        true,
        true,
        "anchor.trust.warning_surface",
        "object.trust.affected_subject",
        "Trust warning subject",
        "Report a trust warning",
        "Stage a security-scoped report about a trust warning; it is submitted later, never on create.",
        included,
    )
}

/// The update-screen packet: save an install/update report locally and offline.
fn packet_update_screen() -> ReproductionPacket {
    let mut included = standard_included_context();
    included.push(included_context(
        IncludedContextClass::SanitizedConfigSnapshot,
        "context.sanitized_config_snapshot",
        "Sanitized config snapshot from the update screen with deployment profile generalized.",
    ));
    packet(
        "reproduction_packet:update_screen",
        OriginatingSurfaceClass::UpdateScreen,
        PacketFlowClass::SaveLocal,
        RedactionPostureClass::MetadataRefsOnly,
        DataExitBoundary::NoPayloadLeavesProduct,
        true,
        true,
        "anchor.update.screen",
        "object.update.release_subject",
        "Update screen subject",
        "Save an update report locally",
        "Save a redaction-safe update/install report to a local artifact that never leaves the product.",
        included,
    )
}

/// The workflow-bundle packet: stage a support-scoped report for later submission.
fn packet_workflow_bundle() -> ReproductionPacket {
    let mut included = standard_included_context();
    included.push(included_context(
        IncludedContextClass::ReproStepsText,
        "context.repro_steps_text",
        "Free-text reproduction steps describing how to replay the workflow bundle.",
    ));
    packet(
        "reproduction_packet:workflow_bundle",
        OriginatingSurfaceClass::WorkflowBundle,
        PacketFlowClass::SubmitLater,
        RedactionPostureClass::RedactedSupportScoped,
        DataExitBoundary::RedactedSupportPacket,
        true,
        true,
        "anchor.workflow.bundle",
        "object.workflow.bundle_subject",
        "Workflow bundle subject",
        "Report a workflow bundle issue",
        "Stage a redacted support-scoped report about a workflow bundle; it is submitted later, never on create.",
        included,
    )
}

/// The other-surface packet: copy a public-safe summary from any other surface.
fn packet_other_surface() -> ReproductionPacket {
    let mut included = standard_included_context();
    included.push(included_context(
        IncludedContextClass::PerformanceTrace,
        "context.performance_trace",
        "Redaction-safe performance-trace ref for the reported surface.",
    ));
    packet(
        "reproduction_packet:other_surface",
        OriginatingSurfaceClass::OtherSurface,
        PacketFlowClass::CopySummary,
        RedactionPostureClass::FullyRedactedPublicSafe,
        DataExitBoundary::MetadataSafeObjectRefs,
        true,
        false,
        "anchor.other.origin_surface",
        "object.other.subject",
        "Reported object",
        "Report from another surface",
        "Copy a fully redacted, public-safe summary anchored to the originating surface.",
        included,
    )
}

fn source_contract_refs() -> Vec<String> {
    vec![
        M5_REPRODUCTION_PACKET_SCHEMA_REF.to_owned(),
        M5_REPRODUCTION_PACKET_CONTRACT_DOC_REF.to_owned(),
        M5_REPRODUCTION_PACKET_PREVIEW_BASE_REF.to_owned(),
        M5_REPRODUCTION_PACKET_HANDOFF_TARGET_REF.to_owned(),
        M5_REPRODUCTION_PACKET_PUBLIC_MATRIX_REF.to_owned(),
    ]
}

/// Build the canonical M5 reproduction-packet set.
pub fn seeded_m5_reproduction_packet_set() -> M5ReproductionPacketSet {
    M5ReproductionPacketSet {
        schema_version: M5_REPRODUCTION_PACKET_SET_SCHEMA_VERSION,
        record_kind: M5_REPRODUCTION_PACKET_SET_RECORD_KIND.to_owned(),
        packet_set_id: M5_REPRODUCTION_PACKET_SET_ID.to_owned(),
        packet_set_label: "M5 reproduction-packet review".to_owned(),
        packets: vec![
            packet_docs_pane(),
            packet_trust_warning(),
            packet_update_screen(),
            packet_workflow_bundle(),
            packet_other_surface(),
        ],
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "fully_redacted_public_safe".to_owned(),
        minted_at: "mint.m5_reproduction_packet_set".to_owned(),
        contract_doc_ref: M5_REPRODUCTION_PACKET_CONTRACT_DOC_REF.to_owned(),
    }
}

/// A standalone save-local packet whose preview is not yet confirmed: a saved
/// offline draft that stays reusable on the machine until the user reviews and
/// shares it.
pub fn seeded_save_local_offline_draft_packet() -> ReproductionPacket {
    let mut p = packet_update_screen();
    p.packet_id = "reproduction_packet:update_screen.offline_draft".to_owned();
    p.preview_confirmed_before_share = false;
    p.headline_label = "Saved update report (offline draft)".to_owned();
    p.packet_summary =
        "A saved offline draft that never leaves the product and stays reusable until the user reviews and shares it.".to_owned();
    p.notes = Some(
        "Saved locally before preview confirmation; nothing leaves the machine and the draft is reusable offline.".to_owned(),
    );
    p
}

/// A standalone packet emphasizing that tokens, hidden approvals, and unmanaged
/// capture are removed and never exported, even when present in local logs.
pub fn seeded_tokens_and_approvals_removed_packet() -> ReproductionPacket {
    let mut p = packet_trust_warning();
    p.packet_id = "reproduction_packet:trust_warning.secrets_removed".to_owned();
    p.headline_label = "Trust report with secrets removed".to_owned();
    p.packet_summary =
        "A trust report whose token row is removed entirely; hidden approvals and unmanaged capture are excluded too.".to_owned();
    p.notes = Some(
        "Tokens are removed entirely and hidden approvals and unmanaged capture are never collected, even from local logs.".to_owned(),
    );
    p
}
