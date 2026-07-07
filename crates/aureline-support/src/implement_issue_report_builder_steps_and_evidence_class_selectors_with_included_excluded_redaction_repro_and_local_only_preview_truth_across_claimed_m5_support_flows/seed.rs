//! Canonical seed builders for the M5 issue-report-builder-step primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical builder-step primitive packet.
pub const M5_ISSUE_REPORT_BUILDER_STEP_PACKET_ID: &str =
    "m5-support-issue-report-builder-step-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked issue-report-builder-step resolution case from a full step state.
#[allow(clippy::too_many_arguments)]
fn builder_case(
    step_kind: M5ReportBuilderStepKind,
    summary: &str,
    repro_steps: &[&str],
    selected_evidence: &[M5SupportEvidenceClass],
    excluded_evidence: &[M5SupportEvidenceClass],
    redaction_state: M5SupportRedactionState,
    share_requested: bool,
    step_identity: &str,
) -> M5IssueReportBuilderStepResolutionCase {
    M5IssueReportBuilderStepResolutionCase::resolved(M5IssueReportBuilderStepResolutionInput {
        step_kind,
        summary: summary.to_owned(),
        repro_steps: repro_steps.iter().map(|s| (*s).to_owned()).collect(),
        selected_evidence: selected_evidence.to_vec(),
        excluded_evidence: excluded_evidence.to_vec(),
        redaction_state,
        share_requested,
        step_identity: step_identity.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full builder-step anatomy, builder
/// step kind, evidence class, data-risk class, redaction state, posture, action, export-
/// field, and accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5IssueReportBuilderConsumerSurface,
    qualification: M5SupportQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    builder_examples: Vec<M5IssueReportBuilderStepResolutionCase>,
) -> M5IssueReportBuilderConsumerRow {
    M5IssueReportBuilderConsumerRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5SupportSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5SupportDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5IssueReportBuilderStepAnatomyPart::ALL.to_vec(),
        builder_step_kinds: M5ReportBuilderStepKind::ALL.to_vec(),
        evidence_classes: M5SupportEvidenceClass::ALL.to_vec(),
        data_classes: DataClass::ALL.to_vec(),
        redaction_states: M5SupportRedactionState::ALL.to_vec(),
        step_postures: M5IssueReportBuilderStepPosture::ALL.to_vec(),
        step_actions: M5IssueReportBuilderStepAction::ALL.to_vec(),
        export_fields: M5IssueReportBuilderStepExportField::ALL.to_vec(),
        accessibility_routes: M5SupportAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5SupportConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5SupportDowngradeTrigger::EvidenceClassMasked,
            M5SupportDowngradeTrigger::RedactionStateUndisclosed,
            M5SupportDowngradeTrigger::AlternateStateLabelInvented,
            M5SupportDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_ISSUE_REPORT_BUILDER_STEP_SCHEMA_REF,
            M5_ISSUE_REPORT_BUILDER_STEP_DATA_RISK_CLASS_REF,
            M5_ISSUE_REPORT_BUILDER_STEP_EXPORT_REDACTION_PROFILE_REF,
            M5_ISSUE_REPORT_BUILDER_STEP_SUPPORT_BUNDLE_MANIFEST_REF,
        ]),
        builder_examples,
        masks_evidence_class: false,
        hides_redaction_state: false,
        drops_local_only_preview: false,
        collapses_report_into_blob: false,
    }
}

fn rows() -> Vec<M5IssueReportBuilderConsumerRow> {
    use M5ReportBuilderStepKind as Step;
    use M5SupportEvidenceClass as Evidence;
    use M5SupportRedactionState as Redaction;

    vec![
        // 1. Support-center builder — a describe-symptom step that selects only
        //    metadata-only and environment-adjacent evidence and is ready to share, and an
        //    attach-evidence step that selects code-adjacent findings under a full-metadata
        //    posture, so a redaction review is required before anything crosses.
        base_row(
            M5IssueReportBuilderConsumerSurface::SupportCenterBuilder,
            M5SupportQualificationClass::Stable,
            "Support center builder owner",
            "The support-center report builder renders the shared issue-report builder step so a describe-symptom step carrying a human-readable summary, ordered reproduction steps, and only activity-timeline and environment-snapshot evidence is ready to share with a user note held excluded, and an attach-evidence step selecting code-adjacent Doctor findings and crash forensics under a full-metadata posture forces a redaction review before anything leaves the local boundary",
            "evidence:m5-builder-step-support-center:001",
            vec![
                builder_case(
                    Step::DescribeSymptom,
                    "Editing stalls for several seconds after the project loads",
                    &[
                        "Open the workspace from a cold start",
                        "Edit any file and wait for the stall",
                        "Note the timeline entry for the freeze",
                    ],
                    &[Evidence::ActivityTimeline, Evidence::EnvironmentSnapshot],
                    &[Evidence::UserNote],
                    Redaction::FullMetadata,
                    true,
                    "builder:support-center:describe-symptom",
                ),
                builder_case(
                    Step::AttachEvidence,
                    "Attach the startup findings and the crash forensics for the freeze",
                    &[
                        "Reproduce the freeze once more to refresh the finding",
                        "Attach the Doctor finding and the crash forensics",
                    ],
                    &[Evidence::DoctorFinding, Evidence::CrashForensics],
                    &[Evidence::UserNote],
                    Redaction::FullMetadata,
                    true,
                    "builder:support-center:attach-evidence",
                ),
            ],
        ),
        // 2. Recovery-center builder — a review-redaction step selecting code-adjacent
        //    evidence under a credentials-scrubbed posture that is ready to share (proving
        //    sensitive evidence can cross once redacted), and a choose-scenario step with no
        //    evidence selected yet.
        base_row(
            M5IssueReportBuilderConsumerSurface::RecoveryCenterBuilder,
            M5SupportQualificationClass::Stable,
            "Recovery center builder owner",
            "The recovery-center report builder renders the shared issue-report builder step so a review-redaction step selecting a repair transaction and crash forensics under a credentials-scrubbed posture is ready to share — proving code-adjacent evidence can cross the local boundary once redacted — and a choose-scenario step with nothing selected yet names its no-evidence-selected posture rather than pretending a report is ready",
            "evidence:m5-builder-step-recovery-center:001",
            vec![
                builder_case(
                    Step::ReviewRedaction,
                    "Review the repair transaction and forensics with credentials scrubbed",
                    &[
                        "Run the guided repair and capture the transaction",
                        "Confirm the scrubbed redaction posture before sharing",
                    ],
                    &[Evidence::RepairTransaction, Evidence::CrashForensics],
                    &[Evidence::UserNote],
                    Redaction::CredentialsScrubbed,
                    true,
                    "builder:recovery-center:review-redaction",
                ),
                builder_case(
                    Step::ChooseScenario,
                    "Pick the recovery scenario before choosing any evidence",
                    &["Open the recovery center and choose the scenario"],
                    &[],
                    &[Evidence::UserNote, Evidence::CrashForensics],
                    Redaction::FullMetadata,
                    false,
                    "builder:recovery-center:choose-scenario",
                ),
            ],
        ),
        // 3. Doctor handoff builder — a confirm-scope step being previewed locally only (the
        //    evidence stays on the device), and a submit-or-export step selecting a
        //    high-risk user note under a full-metadata posture that requires redaction
        //    review.
        base_row(
            M5IssueReportBuilderConsumerSurface::DoctorHandoffBuilder,
            M5SupportQualificationClass::Stable,
            "Doctor handoff builder owner",
            "The Project Doctor handoff report builder renders the shared issue-report builder step so a confirm-scope step previewed locally only keeps its Doctor finding and activity-timeline evidence on the device until a share is requested, and a submit-or-export step selecting a high-risk user note under a full-metadata posture requires a redaction review before it can leave the local boundary",
            "evidence:m5-builder-step-doctor-handoff:001",
            vec![
                builder_case(
                    Step::ConfirmScope,
                    "Confirm the handoff scope while previewing the draft locally",
                    &[
                        "Confirm the finding scope in the handoff builder",
                        "Preview the assembled draft without sharing",
                    ],
                    &[Evidence::DoctorFinding, Evidence::ActivityTimeline],
                    &[Evidence::EnvironmentSnapshot],
                    Redaction::PathsRedacted,
                    false,
                    "builder:doctor-handoff:confirm-scope",
                ),
                builder_case(
                    Step::SubmitOrExport,
                    "Submit the handoff with the reporter note attached",
                    &[
                        "Write the reporter note describing the impact",
                        "Request the handoff submission",
                    ],
                    &[Evidence::UserNote],
                    &[],
                    Redaction::FullMetadata,
                    true,
                    "builder:doctor-handoff:submit-or-export",
                ),
            ],
        ),
        // 4. Headless / CLI builder — a submit-or-export step selecting metadata-only
        //    evidence under a bodies-omitted posture that is ready to share, and an
        //    attach-evidence step whose export is blocked by policy (nothing crosses, only
        //    the local-only preview remains).
        base_row(
            M5IssueReportBuilderConsumerSurface::HeadlessCliBuilder,
            M5SupportQualificationClass::Stable,
            "Headless CLI builder owner",
            "The headless / CLI report builder renders the shared issue-report builder step so a submit-or-export step selecting only an activity timeline under a bodies-omitted posture is ready to share without a desktop UI, and an attach-evidence step whose export is blocked by policy still names its share-blocked posture and keeps the same-weight local-only preview instead of faking a share",
            "evidence:m5-builder-step-headless-cli:001",
            vec![
                builder_case(
                    Step::SubmitOrExport,
                    "Export the timeline-only report from the command line",
                    &[
                        "Run the builder in headless mode",
                        "Select the activity timeline and export",
                    ],
                    &[Evidence::ActivityTimeline],
                    &[Evidence::UserNote],
                    Redaction::BodiesOmitted,
                    true,
                    "builder:headless-cli:submit-or-export",
                ),
                builder_case(
                    Step::AttachEvidence,
                    "Attach the environment and repair evidence while export is blocked",
                    &[
                        "Attempt to attach the environment snapshot and repair transaction",
                        "Observe the policy block on export",
                    ],
                    &[Evidence::EnvironmentSnapshot, Evidence::RepairTransaction],
                    &[Evidence::UserNote],
                    Redaction::ExportBlocked,
                    true,
                    "builder:headless-cli:attach-evidence",
                ),
            ],
        ),
        // 5. Support-packet export — a submit-or-export step selecting a code-adjacent
        //    Doctor finding under a policy-restricted posture that is still ready to share,
        //    and a review-redaction step selecting a high-risk user note under a
        //    full-metadata posture that requires review — the same steps a support reviewer
        //    reads elsewhere.
        base_row(
            M5IssueReportBuilderConsumerSurface::SupportPacketExport,
            M5SupportQualificationClass::Stable,
            "Support packet export owner",
            "The support-packet export surface renders the shared issue-report builder step so a submit-or-export step selecting a code-adjacent Doctor finding under a policy-restricted posture is ready to share with the user note and activity timeline held excluded, and a review-redaction step selecting a high-risk user note under a full-metadata posture requires a redaction review — reconstructing the same summary, repro, evidence, and redaction truth a support reviewer reads",
            "evidence:m5-builder-step-support-export:001",
            vec![
                builder_case(
                    Step::SubmitOrExport,
                    "Export the Doctor finding under the policy-restricted profile",
                    &[
                        "Select the Doctor finding for the packet",
                        "Confirm the policy-restricted redaction and export",
                    ],
                    &[Evidence::DoctorFinding],
                    &[Evidence::UserNote, Evidence::ActivityTimeline],
                    Redaction::PolicyRestricted,
                    true,
                    "builder:support-export:submit-or-export",
                ),
                builder_case(
                    Step::ReviewRedaction,
                    "Review the reporter note before it can leave the boundary",
                    &[
                        "Open the reporter note in the redaction review",
                        "Decide whether the note may be shared",
                    ],
                    &[Evidence::UserNote],
                    &[Evidence::CrashForensics],
                    Redaction::FullMetadata,
                    true,
                    "builder:support-export:review-redaction",
                ),
            ],
        ),
    ]
}

fn governance_review() -> M5IssueReportBuilderStepGovernanceReview {
    M5IssueReportBuilderStepGovernanceReview {
        builder_step_shows_summary_and_repro_steps: true,
        builder_step_shows_selected_and_excluded_evidence: true,
        builder_step_shows_redaction_posture: true,
        included_and_excluded_use_shared_data_class_vocabulary: true,
        user_can_tell_which_classes_cross_local_boundary: true,
        repro_and_evidence_survive_reopen_without_collapse: true,
        same_weight_local_only_preview_never_dropped: true,
        redaction_review_required_before_sensitive_share: true,
        builder_steps_stable_across_deployment_lines: true,
        builder_steps_stable_across_consumer_surfaces: true,
        every_row_declares_accessibility_route: true,
        support_export_reconstructs_builder_truth: true,
        later_rows_cannot_invent_parallel_evidence_vocabulary: true,
        no_surface_masks_evidence_or_redaction: true,
    }
}

fn consumer_projection() -> M5IssueReportBuilderStepConsumerProjection {
    M5IssueReportBuilderStepConsumerProjection {
        doctor_and_support_surfaces_consume_evidence_vocabulary: true,
        step_posture_reads_single_source: true,
        boundary_actions_read_single_source: true,
        support_export_reads_single_source: true,
        headless_and_desktop_read_single_source: true,
    }
}

fn proof_freshness() -> M5IssueReportBuilderStepProofFreshness {
    M5IssueReportBuilderStepProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5IssueReportBuilderStepReleasePosture {
    M5IssueReportBuilderStepReleasePosture {
        release_packet_ref: M5_ISSUE_REPORT_BUILDER_STEP_ARTIFACT_REF.to_owned(),
        support_case_audit_ref: M5_ISSUE_REPORT_BUILDER_STEP_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_ISSUE_REPORT_BUILDER_STEP_SCHEMA_REF,
        M5_ISSUE_REPORT_BUILDER_STEP_DOC_REF,
        M5_ISSUE_REPORT_BUILDER_STEP_COMPONENT_MATRIX_REF,
        M5_ISSUE_REPORT_BUILDER_STEP_DATA_RISK_CLASS_REF,
        M5_ISSUE_REPORT_BUILDER_STEP_EXPORT_REDACTION_PROFILE_REF,
        M5_ISSUE_REPORT_BUILDER_STEP_SUPPORT_BUNDLE_MANIFEST_REF,
        M5_ISSUE_REPORT_BUILDER_STEP_DOCTOR_FINDING_REF,
    ])
}

/// Builds the canonical M5 issue-report-builder-step packet.
pub fn seeded_m5_issue_report_builder_step_packet() -> M5IssueReportBuilderStepPacket {
    M5IssueReportBuilderStepPacket::new(M5IssueReportBuilderStepPacketInput {
        packet_id: M5_ISSUE_REPORT_BUILDER_STEP_PACKET_ID.to_owned(),
        matrix_label:
            "M5 issue-report-builder-step primitive: human-readable summary, ordered reproduction steps, selected and excluded evidence classes with their metadata/environment-adjacent/code-adjacent/high-risk data class, redaction posture, derived step posture, per-class local-boundary disposition, and bounded reveal-boundary/preview-local-only/edit-selection/review-redaction/share/export actions with a same-weight local-only preview"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5IssueReportBuilderStepVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the recovery-center builder consumer is narrowed to Preview pending
/// redaction-review parity proof across every deployment; every consumer stays visible.
pub fn seeded_m5_issue_report_builder_step_recovery_center_builder_preview_narrowed(
) -> M5IssueReportBuilderStepPacket {
    let mut packet = seeded_m5_issue_report_builder_step_packet();
    packet.packet_id =
        "m5-support-issue-report-builder-step-primitive:recovery-center-builder-preview:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| {
            row.consumer_surface == M5IssueReportBuilderConsumerSurface::RecoveryCenterBuilder
        })
        .expect("recovery-center-builder row present");
    row.qualification = M5SupportQualificationClass::Preview;
    packet
}

/// Narrowed variant: the headless / CLI builder consumer is held at Beta because a slice of
/// headless steps do not yet render the keyboard route cue on every profile; every consumer
/// stays visible.
pub fn seeded_m5_issue_report_builder_step_headless_cli_builder_beta_narrowed(
) -> M5IssueReportBuilderStepPacket {
    let mut packet = seeded_m5_issue_report_builder_step_packet();
    packet.packet_id =
        "m5-support-issue-report-builder-step-primitive:headless-cli-builder-beta:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5IssueReportBuilderConsumerSurface::HeadlessCliBuilder)
        .expect("headless-cli-builder row present");
    row.qualification = M5SupportQualificationClass::Beta;
    packet
}
