use super::*;

const PACKET_ID: &str = "m5-docs-authoring-matrix:stable:0001";

fn lane_rows() -> Vec<M5AuthoringMatrixLaneRow> {
    vec![
        M5AuthoringMatrixLaneRow {
            surface: M5AuthoringSurface::MarkdownAuthoringWorkspace,
            qualification: M5AuthoringQualificationClass::Stable,
            scope_summary: "Governed Markdown authoring workspace for README, changelog, help, tutorial, and module docs with source/split/rendered modes; rendered views stay safe and labeled and never become a privileged execution path".to_owned(),
            supported_workspace_modes: vec![
                M5AuthoringWorkspaceMode::Source,
                M5AuthoringWorkspaceMode::Split,
                M5AuthoringWorkspaceMode::Rendered,
            ],
            preview_safety_class: M5AuthoringPreviewSafetyClass::SanitizedSafe,
            validation_states: vec![
                M5AuthoringValidationState::Validated,
                M5AuthoringValidationState::SuspectedStale,
                M5AuthoringValidationState::UnchangedUnverified,
                M5AuthoringValidationState::NotValidated,
            ],
            suggestion_triggers: vec![],
            evidence_handoff_scope: M5AuthoringEvidenceHandoffScope::LocalOnly,
            evidence_requirement: M5AuthoringEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:docs-authoring-workspace-conformance:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5AuthoringDowngradeTrigger::ProofStale,
                M5AuthoringDowngradeTrigger::SourceVersionMismatch,
                M5AuthoringDowngradeTrigger::TrustNarrowing,
                M5AuthoringDowngradeTrigger::UnsafePreviewBlocked,
            ],
            rollback_posture: M5AuthoringRollbackPosture::SourceCanonicalNoMutation,
            source_contract_refs: vec![M5_AUTHORING_MATRIX_MAINTENANCE_CONTRACT_REF.to_owned()],
            consumer_surfaces: vec![
                M5AuthoringConsumerSurface::AuthoringWorkspace,
                M5AuthoringConsumerSurface::PreviewPane,
                M5AuthoringConsumerSurface::CliHeadless,
                M5AuthoringConsumerSurface::SupportExport,
            ],
        },
        M5AuthoringMatrixLaneRow {
            surface: M5AuthoringSurface::CommonMarkPreview,
            qualification: M5AuthoringQualificationClass::Stable,
            scope_summary: "CommonMark preview baseline that renders Markdown to a sanitized, labeled view; embedded raw HTML, scripts, iframes, and event handlers are stripped or blocked and diagram engines stay opt-in and non-privileged".to_owned(),
            supported_workspace_modes: vec![
                M5AuthoringWorkspaceMode::Split,
                M5AuthoringWorkspaceMode::Rendered,
            ],
            preview_safety_class: M5AuthoringPreviewSafetyClass::SanitizedSafe,
            validation_states: vec![
                M5AuthoringValidationState::Validated,
                M5AuthoringValidationState::Unsupported,
                M5AuthoringValidationState::NotValidated,
            ],
            suggestion_triggers: vec![],
            evidence_handoff_scope: M5AuthoringEvidenceHandoffScope::LocalOnly,
            evidence_requirement: M5AuthoringEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:commonmark-preview-sanitization-conformance:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5AuthoringDowngradeTrigger::ProofStale,
                M5AuthoringDowngradeTrigger::PolicyBlocked,
                M5AuthoringDowngradeTrigger::UnsafePreviewBlocked,
            ],
            rollback_posture: M5AuthoringRollbackPosture::SourceCanonicalNoMutation,
            source_contract_refs: vec![M5_AUTHORING_MATRIX_MAINTENANCE_CONTRACT_REF.to_owned()],
            consumer_surfaces: vec![
                M5AuthoringConsumerSurface::PreviewPane,
                M5AuthoringConsumerSurface::AuthoringWorkspace,
                M5AuthoringConsumerSurface::CliHeadless,
                M5AuthoringConsumerSurface::SupportExport,
            ],
        },
        M5AuthoringMatrixLaneRow {
            surface: M5AuthoringSurface::DocsMaintenanceSuggestions,
            qualification: M5AuthoringQualificationClass::Stable,
            scope_summary: "Docs-maintenance and stale-example suggestions that stay diff-first: every suggestion is a reviewable draft or review diff tied to a trigger, never silently applied to source".to_owned(),
            supported_workspace_modes: vec![
                M5AuthoringWorkspaceMode::Source,
                M5AuthoringWorkspaceMode::Split,
            ],
            preview_safety_class: M5AuthoringPreviewSafetyClass::SanitizedSafe,
            validation_states: vec![
                M5AuthoringValidationState::Validated,
                M5AuthoringValidationState::SuspectedStale,
                M5AuthoringValidationState::UnchangedUnverified,
                M5AuthoringValidationState::StaleRerunRequired,
            ],
            suggestion_triggers: vec![
                M5AuthoringSuggestionTrigger::CodeDiff,
                M5AuthoringSuggestionTrigger::StaleExample,
                M5AuthoringSuggestionTrigger::ReleaseNoteDrift,
                M5AuthoringSuggestionTrigger::FailingSnippet,
                M5AuthoringSuggestionTrigger::ContractChange,
                M5AuthoringSuggestionTrigger::HumanNote,
            ],
            evidence_handoff_scope: M5AuthoringEvidenceHandoffScope::ReviewHandoffScoped,
            evidence_requirement: M5AuthoringEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:docs-maintenance-suggestion-diff-conformance:m5".to_owned(),
                "evidence:stale-example-governance:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5AuthoringDowngradeTrigger::ProofStale,
                M5AuthoringDowngradeTrigger::SourceVersionMismatch,
                M5AuthoringDowngradeTrigger::FreshnessExpired,
                M5AuthoringDowngradeTrigger::TrustNarrowing,
                M5AuthoringDowngradeTrigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: M5AuthoringRollbackPosture::DiffFirstReviewRequired,
            source_contract_refs: vec![
                M5_AUTHORING_MATRIX_SUGGESTION_CONTRACT_REF.to_owned(),
                M5_AUTHORING_MATRIX_MAINTENANCE_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5AuthoringConsumerSurface::DocsMaintenancePanel,
                M5AuthoringConsumerSurface::AuthoringWorkspace,
                M5AuthoringConsumerSurface::CliHeadless,
                M5AuthoringConsumerSurface::SupportExport,
                M5AuthoringConsumerSurface::Diagnostics,
            ],
        },
        M5AuthoringMatrixLaneRow {
            surface: M5AuthoringSurface::DocsValidation,
            qualification: M5AuthoringQualificationClass::Stable,
            scope_summary: "Docs validation states for documented examples and links: validated, suspected-stale, unverified, unsupported, skipped, stale-rerun-required, or not-validated truth stays visible and never silently upgrades to verified".to_owned(),
            supported_workspace_modes: vec![M5AuthoringWorkspaceMode::Source],
            preview_safety_class: M5AuthoringPreviewSafetyClass::NotApplicable,
            validation_states: vec![
                M5AuthoringValidationState::Validated,
                M5AuthoringValidationState::SuspectedStale,
                M5AuthoringValidationState::UnchangedUnverified,
                M5AuthoringValidationState::Unsupported,
                M5AuthoringValidationState::Skipped,
                M5AuthoringValidationState::StaleRerunRequired,
                M5AuthoringValidationState::NotValidated,
            ],
            suggestion_triggers: vec![],
            evidence_handoff_scope: M5AuthoringEvidenceHandoffScope::LocalOnly,
            evidence_requirement: M5AuthoringEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:docs-example-validation-conformance:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5AuthoringDowngradeTrigger::ProofStale,
                M5AuthoringDowngradeTrigger::FreshnessExpired,
                M5AuthoringDowngradeTrigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: M5AuthoringRollbackPosture::SourceCanonicalNoMutation,
            source_contract_refs: vec![M5_AUTHORING_MATRIX_MAINTENANCE_CONTRACT_REF.to_owned()],
            consumer_surfaces: vec![
                M5AuthoringConsumerSurface::DocsMaintenancePanel,
                M5AuthoringConsumerSurface::CliHeadless,
                M5AuthoringConsumerSurface::SupportExport,
                M5AuthoringConsumerSurface::Diagnostics,
            ],
        },
        M5AuthoringMatrixLaneRow {
            surface: M5AuthoringSurface::DocsEvidenceHandoff,
            qualification: M5AuthoringQualificationClass::Beta,
            scope_summary: "Docs evidence handoff that ties a prose change back to the code, schema, or release truth it depends on; handoff is scoped and source-linked and never hides owner, origin, or boundary changes or silently widens authority".to_owned(),
            supported_workspace_modes: vec![M5AuthoringWorkspaceMode::Source],
            preview_safety_class: M5AuthoringPreviewSafetyClass::NotApplicable,
            validation_states: vec![
                M5AuthoringValidationState::Validated,
                M5AuthoringValidationState::UnchangedUnverified,
                M5AuthoringValidationState::NotValidated,
            ],
            suggestion_triggers: vec![],
            evidence_handoff_scope: M5AuthoringEvidenceHandoffScope::PublishHandoffScoped,
            evidence_requirement: M5AuthoringEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:docs-evidence-handoff-source-link:m5".to_owned(),
                "evidence:docs-browser-truth-packet:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5AuthoringDowngradeTrigger::ProofStale,
                M5AuthoringDowngradeTrigger::PolicyBlocked,
                M5AuthoringDowngradeTrigger::TrustNarrowing,
                M5AuthoringDowngradeTrigger::ScopeExpansionUnqualified,
                M5AuthoringDowngradeTrigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: M5AuthoringRollbackPosture::ReturnPathPreserved,
            source_contract_refs: vec![
                M5_AUTHORING_MATRIX_BROWSER_HANDOFF_CONTRACT_REF.to_owned(),
                M5_AUTHORING_MATRIX_DOCS_PACK_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5AuthoringConsumerSurface::DocsBrowser,
                M5AuthoringConsumerSurface::BrowserCompanion,
                M5AuthoringConsumerSurface::ReleaseCenter,
                M5AuthoringConsumerSurface::SupportExport,
                M5AuthoringConsumerSurface::HelpAbout,
            ],
        },
    ]
}

fn trust_review() -> M5AuthoringMatrixTrustReview {
    M5AuthoringMatrixTrustReview {
        source_canonical_rendered_safe_and_labeled: true,
        rendered_preview_safe_by_default: true,
        preview_not_privileged_execution_path: true,
        suggestions_diff_first_never_auto_applied: true,
        source_version_freshness_truth_visible: true,
        validation_state_never_silently_upgraded: true,
        evidence_handoff_source_linked: true,
        handoff_never_hides_owner_origin_or_boundary: true,
        handoff_never_silently_widens_authority: true,
        no_full_browser_collab_editor_or_remote_cms: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> M5AuthoringMatrixConsumerProjection {
    M5AuthoringMatrixConsumerProjection {
        authoring_workspace_shows_mode_and_source_truth: true,
        preview_shows_safety_class: true,
        maintenance_panel_shows_suggestion_triggers_and_diff: true,
        validation_shows_state: true,
        evidence_handoff_shows_scope_and_source_link: true,
        cli_headless_shows_qualification: true,
        support_export_shows_qualification: true,
        release_center_shows_qualification: true,
        help_about_shows_qualification: true,
        preview_labs_label_for_unqualified_surfaces: true,
    }
}

fn proof_freshness() -> M5AuthoringMatrixProofFreshness {
    M5AuthoringMatrixProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-18T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5AuthoringMatrixReleasePosture {
    M5AuthoringMatrixReleasePosture {
        release_packet_ref: "evidence:docs-authoring-release-packet:m5".to_owned(),
        mirror_offline_packet_ref: "evidence:docs-authoring-mirror-offline-packet:m5".to_owned(),
        support_export_parity_required: true,
        mirror_offline_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        M5_AUTHORING_MATRIX_SCHEMA_REF.to_owned(),
        M5_AUTHORING_MATRIX_DOC_REF.to_owned(),
        M5_AUTHORING_MATRIX_MAINTENANCE_CONTRACT_REF.to_owned(),
        M5_AUTHORING_MATRIX_SUGGESTION_CONTRACT_REF.to_owned(),
        M5_AUTHORING_MATRIX_BROWSER_HANDOFF_CONTRACT_REF.to_owned(),
        M5_AUTHORING_MATRIX_DOCS_PACK_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> M5AuthoringMatrixPacket {
    M5AuthoringMatrixPacket::new(M5AuthoringMatrixPacketInput {
        packet_id: PACKET_ID.to_owned(),
        matrix_label: "M5 Docs Authoring, Preview, Maintenance, and Evidence-Handoff Matrix"
            .to_owned(),
        lane_rows: lane_rows(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-18T00:00:00Z".to_owned(),
    })
}

#[test]
fn m5_authoring_matrix_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_surface_fails_validation() {
    let mut packet = packet();
    packet
        .lane_rows
        .retain(|row| row.surface != M5AuthoringSurface::DocsValidation);
    assert!(packet
        .validate()
        .contains(&M5AuthoringMatrixViolation::RequiredSurfaceMissing));
}

#[test]
fn stable_surface_missing_evidence_fails() {
    let mut packet = packet();
    packet.lane_rows[0].required_evidence_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AuthoringMatrixViolation::StableSurfaceMissingEvidence));
}

#[test]
fn rendering_surface_without_concrete_preview_safety_fails() {
    let mut packet = packet();
    // CommonMark preview renders, so a not_applicable safety class is unsafe.
    packet.lane_rows[1].preview_safety_class = M5AuthoringPreviewSafetyClass::NotApplicable;
    assert!(packet
        .validate()
        .contains(&M5AuthoringMatrixViolation::UnsafePreviewDefault));
}

#[test]
fn maintenance_surface_without_suggestion_triggers_fails() {
    let mut packet = packet();
    packet.lane_rows[2].suggestion_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5AuthoringMatrixViolation::SuggestionTriggersMissing));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.lane_rows[1].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5AuthoringMatrixViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = packet();
    packet.lane_rows[3].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5AuthoringMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AuthoringMatrixViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .trust_review
        .suggestions_diff_first_never_auto_applied = false;
    assert!(packet
        .validate()
        .contains(&M5AuthoringMatrixViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .preview_labs_label_for_unqualified_surfaces = false;
    assert!(packet
        .validate()
        .contains(&M5AuthoringMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5AuthoringMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = packet();
    packet.release_posture.mirror_offline_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5AuthoringMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_surface() {
    let summary = packet().render_markdown_summary();
    for surface in M5AuthoringSurface::ALL {
        assert!(
            summary.contains(surface.as_str()),
            "summary missing surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_markdown_authoring_matrix_export()
        .expect("checked M5 docs-authoring matrix export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/docs/m5/freeze_the_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix/unsafe_preview_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/docs/m5/freeze_the_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix/evidence_handoff_held.json"
        )),
    ] {
        let packet: M5AuthoringMatrixPacket =
            serde_json::from_str(raw).expect("fixture parses as matrix packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
