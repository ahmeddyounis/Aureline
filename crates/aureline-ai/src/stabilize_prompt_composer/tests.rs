use crate::context_inspector::{ContextFreshnessClass, ContextOmissionReasonClass};
use crate::prompt_composer::current_beta_prompt_composer_conformance_export;
use crate::SourceClass;

use super::*;

const CONFORMANCE_REF: &str = "prompt-composer-conformance:beta:0001";
const SNAPSHOT_REF: &str = "context-snapshot:prompt-composer:beta:0001";
const SESSION_REF: &str = "composer-session:prompt-composer:beta:0001";
const DRAFT_REF: &str = "turn-draft:prompt-composer:beta:0001";

fn thread_header() -> ThreadHeaderRow {
    ThreadHeaderRow {
        thread_id: "ai-thread:prompt-composer:stable:0001".to_owned(),
        current_scope_label: "Current diff".to_owned(),
        provider_label: "Aureline managed hosted AI".to_owned(),
        model_label: "Hosted context review".to_owned(),
        retention_mode_class: ThreadRetentionModeClass::RepoShared,
        memory_class_token: "repo_review_memory".to_owned(),
        save_memory_action_ref: "action:prompt-composer.save-memory".to_owned(),
        delete_action_ref: "action:prompt-composer.delete-thread".to_owned(),
        export_action_ref: "action:prompt-composer.export-thread".to_owned(),
        remember_preview: RememberPreview {
            retained_summary_label:
                "Saves the retry review rationale and pinned object refs, not raw prompt text."
                    .to_owned(),
            retention_locus_class: RetentionLocusClass::RepoScoped,
            reuse_audience_class: ReuseAudienceClass::RepoCollaborators,
            memory_class_token: "repo_review_memory".to_owned(),
            preview_action_ref: "action:prompt-composer.preview-remember".to_owned(),
        },
    }
}

fn attachment_rows() -> Vec<StableAttachmentSemanticRow> {
    let specs = [
        (
            "att:file:retry",
            "file:payments-retry",
            StableAttachmentSourceClass::WorkspaceFile,
            AttachmentTaintClass::TrustedFirstParty,
            ContextFreshnessClass::AuthoritativeLive,
            InclusionPostureClass::IncludedPinned,
        ),
        (
            "att:symbol:route",
            "symbol:RetryRoute",
            StableAttachmentSourceClass::Symbol,
            AttachmentTaintClass::TrustedFirstParty,
            ContextFreshnessClass::AuthoritativeLive,
            InclusionPostureClass::IncludedRaw,
        ),
        (
            "att:docs:policy",
            "docs:retry-policy",
            StableAttachmentSourceClass::DocsReference,
            AttachmentTaintClass::WorkspaceDerived,
            ContextFreshnessClass::WarmCached,
            InclusionPostureClass::Summarized,
        ),
        (
            "att:diag:retry",
            "diagnostic:retry-warn",
            StableAttachmentSourceClass::Diagnostic,
            AttachmentTaintClass::WorkspaceDerived,
            ContextFreshnessClass::AuthoritativeLive,
            InclusionPostureClass::IncludedRaw,
        ),
        (
            "att:test:retry",
            "test:retry-suite",
            StableAttachmentSourceClass::TestResult,
            AttachmentTaintClass::WorkspaceDerived,
            ContextFreshnessClass::Stale,
            InclusionPostureClass::OmittedInspectable,
        ),
        (
            "att:run:last-test",
            "run:last-test",
            StableAttachmentSourceClass::TerminalToolOutput,
            AttachmentTaintClass::UntrustedExternal,
            ContextFreshnessClass::Stale,
            InclusionPostureClass::Trimmed,
        ),
        (
            "att:ext:notes",
            "external-text:reviewer-notes",
            StableAttachmentSourceClass::ExternalText,
            AttachmentTaintClass::TaintedQuarantined,
            ContextFreshnessClass::Unverified,
            InclusionPostureClass::BlockedQuarantined,
        ),
    ];
    specs
        .into_iter()
        .map(
            |(id, object, source_class, taint_class, freshness_class, inclusion_posture)| {
                StableAttachmentSemanticRow {
                    attachment_id: id.to_owned(),
                    stable_object_ref: object.to_owned(),
                    origin_label: format!("Origin: {object}"),
                    source_class,
                    taint_class,
                    freshness_class,
                    inclusion_posture,
                    keyboard_reachable: true,
                    screen_reader_label: format!(
                        "{} attachment {object}, {} trust, {} posture",
                        source_class.as_str(),
                        taint_class.as_str(),
                        inclusion_posture.as_str()
                    ),
                }
            },
        )
        .collect()
}

fn pinned_rows() -> Vec<PinnedContextRow> {
    vec![
        PinnedContextRow {
            pin_id: "pin:file:retry".to_owned(),
            stable_object_ref: "file:payments-retry".to_owned(),
            display_label: "retry.rs selected lines".to_owned(),
            freshness_state: PinnedFreshnessStateClass::PinnedFresh,
            drift_source: None,
            refresh_action_ref: "action:prompt-composer.refresh-pin:pin:file:retry".to_owned(),
            remove_action_ref: "action:prompt-composer.remove-pin:pin:file:retry".to_owned(),
            blocks_send_until_resolved: false,
            keyboard_reachable: true,
        },
        PinnedContextRow {
            pin_id: "pin:test:retry".to_owned(),
            stable_object_ref: "test:retry-suite".to_owned(),
            display_label: "retry suite results".to_owned(),
            freshness_state: PinnedFreshnessStateClass::PinnedButStale,
            drift_source: Some(DriftSourceClass::TestState),
            refresh_action_ref: "action:prompt-composer.refresh-pin:pin:test:retry".to_owned(),
            remove_action_ref: "action:prompt-composer.remove-pin:pin:test:retry".to_owned(),
            blocks_send_until_resolved: true,
            keyboard_reachable: true,
        },
    ]
}

fn omitted_rows() -> Vec<OmittedContextReviewRow> {
    vec![
        OmittedContextReviewRow {
            source_ref: "history:large-diff".to_owned(),
            source_class: SourceClass::WorkspaceSearchResult,
            omission_reason_class: ContextOmissionReasonClass::Budget,
            inspectable_after_send: true,
            inspect_action_ref: "action:prompt-composer.inspect-omitted:history:large-diff"
                .to_owned(),
            replay_explains_exclusion: true,
            keyboard_reachable: true,
        },
        OmittedContextReviewRow {
            source_ref: "external-text:reviewer-notes".to_owned(),
            source_class: SourceClass::UserSuppliedText,
            omission_reason_class: ContextOmissionReasonClass::Tainted,
            inspectable_after_send: true,
            inspect_action_ref:
                "action:prompt-composer.inspect-omitted:external-text:reviewer-notes".to_owned(),
            replay_explains_exclusion: true,
            keyboard_reachable: true,
        },
    ]
}

fn forked_lineage() -> ForkedThreadLineage {
    ForkedThreadLineage {
        thread_id: "ai-thread:prompt-composer:stable:0001".to_owned(),
        is_forked: true,
        parent_thread_ref: Some("ai-thread:prompt-composer:stable:0000".to_owned()),
        parent_run_ref: Some("ai-run:prompt-composer:stable:0000".to_owned()),
        inherited_context_snapshot_ref: Some(
            "context-snapshot:prompt-composer:stable:0000".to_owned(),
        ),
        divergence_point_ref: Some("divergence:prompt-composer:stable:0001".to_owned()),
    }
}

fn compare_rows() -> Vec<CompareAnswerRow> {
    vec![
        CompareAnswerRow {
            comparison_id: "compare:same-context".to_owned(),
            left_run_ref: "ai-run:prompt-composer:stable:0001a".to_owned(),
            right_run_ref: "ai-run:prompt-composer:stable:0001b".to_owned(),
            left_context_snapshot_ref: SNAPSHOT_REF.to_owned(),
            right_context_snapshot_ref: SNAPSHOT_REF.to_owned(),
            context_parity_class: ContextParityClass::SameContextSnapshot,
            provider_model_delta_label: Some("Model A vs Model B".to_owned()),
            instruction_stack_delta_label: None,
            hidden_drift_warning: false,
        },
        CompareAnswerRow {
            comparison_id: "compare:drifted-context".to_owned(),
            left_run_ref: "ai-run:prompt-composer:stable:0001a".to_owned(),
            right_run_ref: "ai-run:prompt-composer:stable:0001c".to_owned(),
            left_context_snapshot_ref: SNAPSHOT_REF.to_owned(),
            right_context_snapshot_ref: "context-snapshot:prompt-composer:stable:0002".to_owned(),
            context_parity_class: ContextParityClass::DifferentContextDrift,
            provider_model_delta_label: None,
            instruction_stack_delta_label: Some("Repo rules changed between runs".to_owned()),
            hidden_drift_warning: true,
        },
    ]
}

fn drift_banners() -> Vec<ContextDriftBanner> {
    vec![ContextDriftBanner {
        banner_id: "drift:test:retry".to_owned(),
        drift_source: DriftSourceClass::TestState,
        affected_object_ref: "test:retry-suite".to_owned(),
        previously_reviewed_snapshot_ref: SNAPSHOT_REF.to_owned(),
        requires_rereview: true,
        explanation_label: "The retry suite changed since this composer state was reviewed."
            .to_owned(),
        keyboard_reachable: true,
        screen_reader_label: "Context drift: retry suite changed; re-review required.".to_owned(),
    }]
}

fn surface_rows() -> Vec<SurfaceConsistencyRow> {
    ComposerSurfaceClass::required_coverage()
        .into_iter()
        .map(|surface_class| SurfaceConsistencyRow {
            surface_class,
            attachment_pills_keyboard_reachable: true,
            mention_rows_screen_reader_describable: true,
            omitted_context_review_reachable: true,
            forked_thread_comparison_reachable: true,
            context_drift_banner_reachable: true,
        })
        .collect()
}

fn source_contract_refs() -> Vec<String> {
    vec![
        PROMPT_COMPOSER_STABILIZATION_AI_DOC_REF.to_owned(),
        PROMPT_COMPOSER_STABILIZATION_BASE_CONTRACT_REF.to_owned(),
        PROMPT_COMPOSER_STABILIZATION_SCHEMA_REF.to_owned(),
        PROMPT_COMPOSER_STABILIZATION_BETA_ARTIFACT_REF.to_owned(),
    ]
}

fn input() -> PromptComposerStabilizationInput {
    PromptComposerStabilizationInput {
        packet_id: "prompt-composer-stabilization:stable:0001".to_owned(),
        workflow_or_surface_id: "surface:prompt-composer:stable".to_owned(),
        display_label: "Prompt composer stabilization".to_owned(),
        composer_conformance_packet_ref: CONFORMANCE_REF.to_owned(),
        composer_context_snapshot_ref: SNAPSHOT_REF.to_owned(),
        composer_session_ref: SESSION_REF.to_owned(),
        composer_draft_ref: DRAFT_REF.to_owned(),
        thread_header: thread_header(),
        attachment_semantic_rows: attachment_rows(),
        pinned_context_rows: pinned_rows(),
        omitted_context_review_rows: omitted_rows(),
        forked_thread_lineage: forked_lineage(),
        compare_answer_rows: compare_rows(),
        context_drift_banners: drift_banners(),
        surface_consistency_rows: surface_rows(),
        source_contract_refs: source_contract_refs(),
        json_export_ref: PROMPT_COMPOSER_STABILIZATION_ARTIFACT_REF.to_owned(),
        markdown_summary_ref: PROMPT_COMPOSER_STABILIZATION_SUMMARY_REF.to_owned(),
        minted_at: "2026-05-31T20:00:00Z".to_owned(),
    }
}

fn packet() -> PromptComposerStabilizationPacket {
    PromptComposerStabilizationPacket::new(input())
}

#[test]
fn stabilization_packet_validates_against_conformance() {
    let packet = packet();
    let conformance = current_beta_prompt_composer_conformance_export()
        .expect("checked conformance export validates");
    assert!(
        packet.validate(&conformance).is_empty(),
        "{:?}",
        packet.validate(&conformance)
    );
}

#[test]
fn attachment_rows_cover_every_typed_source_class() {
    let mut packet = packet();
    assert!(packet.validate_self().is_empty(), "{:?}", packet.validate_self());
    packet
        .attachment_semantic_rows
        .retain(|row| row.source_class != StableAttachmentSourceClass::ExternalText);

    assert!(packet.validate_self().contains(
        &PromptComposerStabilizationViolation::AttachmentSourceClassCoverageMissing
    ));
}

#[test]
fn pinned_drift_must_read_as_pinned_but_stale() {
    let mut packet = packet();
    let stale = packet
        .pinned_context_rows
        .iter_mut()
        .find(|row| row.freshness_state == PinnedFreshnessStateClass::PinnedButStale)
        .expect("stale pin");
    stale.freshness_state = PinnedFreshnessStateClass::PinnedFresh;

    assert!(packet
        .validate_self()
        .contains(&PromptComposerStabilizationViolation::PinnedStaleNotSurfaced));
}

#[test]
fn omitted_context_must_stay_inspectable_after_send() {
    let mut packet = packet();
    packet.omitted_context_review_rows[0].inspectable_after_send = false;

    assert!(packet
        .validate_self()
        .contains(&PromptComposerStabilizationViolation::OmittedContextNotInspectable));
}

#[test]
fn forked_thread_requires_parent_inherited_and_divergence() {
    let mut packet = packet();
    packet.forked_thread_lineage.divergence_point_ref = None;

    assert!(packet
        .validate_self()
        .contains(&PromptComposerStabilizationViolation::ForkedThreadLineageIncomplete));
}

#[test]
fn drifted_compare_answers_must_flag_hidden_drift() {
    let mut packet = packet();
    let drift = packet
        .compare_answer_rows
        .iter_mut()
        .find(|row| row.context_parity_class == ContextParityClass::DifferentContextDrift)
        .expect("drift comparison");
    drift.hidden_drift_warning = false;

    assert!(packet
        .validate_self()
        .contains(&PromptComposerStabilizationViolation::CompareAnswerTruthMissing));
}

#[test]
fn context_drift_banner_must_require_rereview() {
    let mut packet = packet();
    packet.context_drift_banners[0].requires_rereview = false;

    assert!(packet
        .validate_self()
        .contains(&PromptComposerStabilizationViolation::ContextDriftBannerIncomplete));
}

#[test]
fn shared_remember_preview_must_name_audience() {
    let mut packet = packet();
    packet.thread_header.remember_preview.reuse_audience_class = ReuseAudienceClass::Nobody;

    assert!(packet
        .validate_self()
        .contains(&PromptComposerStabilizationViolation::RememberPreviewIncomplete));
}

#[test]
fn every_composer_surface_must_be_consistent() {
    let mut packet = packet();
    packet
        .surface_consistency_rows
        .retain(|row| row.surface_class != ComposerSurfaceClass::Detached);

    assert!(packet
        .validate_self()
        .contains(&PromptComposerStabilizationViolation::SurfaceConsistencyMissing));
}

#[test]
fn mismatched_conformance_ref_is_rejected() {
    let mut packet = packet();
    packet.composer_conformance_packet_ref = "prompt-composer-conformance:other".to_owned();
    let conformance = current_beta_prompt_composer_conformance_export()
        .expect("checked conformance export validates");

    assert!(packet
        .validate(&conformance)
        .contains(&PromptComposerStabilizationViolation::EmbeddedConformanceInvalid));
}

#[test]
#[ignore = "run manually to regenerate the checked artifact"]
fn emit_artifact() {
    let packet = packet();
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    let dir = format!("{root}/artifacts/ai/m4/prompt_composer_stabilization");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        format!("{dir}/support_export.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .unwrap();
    std::fs::write(
        format!("{dir}/summary.md"),
        packet.render_markdown_summary(),
    )
    .unwrap();
}

#[test]
fn checked_artifact_validates() {
    let packet = current_stable_prompt_composer_stabilization_export()
        .expect("checked prompt-composer stabilization export validates");
    assert!(packet.validate_self().is_empty());
    let conformance = current_beta_prompt_composer_conformance_export()
        .expect("checked conformance export validates");
    assert!(packet.validate(&conformance).is_empty());
}
