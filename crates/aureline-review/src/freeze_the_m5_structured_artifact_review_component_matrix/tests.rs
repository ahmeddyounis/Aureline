use super::*;

const PACKET_ID: &str = "m5-artifact-component-matrix:stable:0001";

fn component_rows() -> Vec<M5ArtifactComponentMatrixRow> {
    vec![
        M5ArtifactComponentMatrixRow {
            component: M5ArtifactComponent::ArtifactIdentityBar,
            maturity: M5ArtifactComponentMaturityClass::Stable,
            scope_summary: "Artifact identity bar naming artifact class, canonical source, and parser/schema state at the top of every structured or media-like compare surface".to_owned(),
            canonical_source_disclosure: "Names the artifact class and the canonical source of truth (working-tree path, generated-from origin, or imported snapshot) and never presents an adjunct capture as the source".to_owned(),
            fidelity_narrowing_vocab: vec![
                M5ArtifactFidelityState::StructuredFaithful,
                M5ArtifactFidelityState::SchemaUnrecognized,
                M5ArtifactFidelityState::RawFallback,
                M5ArtifactFidelityState::RedactedOrWithheld,
            ],
            compare_write_back_safety: "Identity bar is read-only and never offers write-back; it only labels whether the underlying artifact is compare-only or writable".to_owned(),
            render_trust_disclosure: "States whether the identity was parsed from a trusted structured header or inferred, so an inferred class is never shown as confirmed".to_owned(),
            generated_from_relation: "When the artifact is generated it names the generating source of truth rather than treating the generated file as authoritative".to_owned(),
            evidence_requirement: M5ArtifactComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:artifact-identity-bar-class-and-source:m5".to_owned(),
                "evidence:artifact-identity-bar-schema-state:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5ArtifactComponentDowngradeTrigger::ProofStale,
                M5ArtifactComponentDowngradeTrigger::SchemaUnrecognized,
                M5ArtifactComponentDowngradeTrigger::TrustNarrowing,
                M5ArtifactComponentDowngradeTrigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: M5ArtifactComponentRollbackPosture::ReadOnlyNoMutation,
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_IDENTITY_BAR_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5ArtifactComponentConsumerSurface::ReviewWorkspace,
                M5ArtifactComponentConsumerSurface::DiffCompareView,
                M5ArtifactComponentConsumerSurface::CliHeadless,
                M5ArtifactComponentConsumerSurface::SupportExport,
            ],
        },
        M5ArtifactComponentMatrixRow {
            component: M5ArtifactComponent::DiffModeSwitcher,
            maturity: M5ArtifactComponentMaturityClass::Stable,
            scope_summary: "Diff-mode switcher exposing the available structured diff modes (structured, cell-aware, rendered, raw) and the active mode without silently collapsing to raw".to_owned(),
            canonical_source_disclosure: "Names the canonical artifact being diffed and which side is base versus compare so the mode never obscures what is compared".to_owned(),
            fidelity_narrowing_vocab: vec![
                M5ArtifactFidelityState::StructuredFaithful,
                M5ArtifactFidelityState::StructuredPartial,
                M5ArtifactFidelityState::SchemaUnrecognized,
                M5ArtifactFidelityState::RawFallback,
            ],
            compare_write_back_safety: "Switching modes is a view-only operation that never writes back; it only changes how the compare is rendered".to_owned(),
            render_trust_disclosure: "Labels each mode's render trust so a rendered mode is marked untrusted rather than presented as an authoritative structured diff".to_owned(),
            generated_from_relation: "When one side is a generated artifact the switcher marks it generated so a diff is never read as a hand-authored change".to_owned(),
            evidence_requirement: M5ArtifactComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:diff-mode-switcher-available-modes:m5".to_owned(),
                "evidence:diff-mode-switcher-raw-fallback-explained:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5ArtifactComponentDowngradeTrigger::ProofStale,
                M5ArtifactComponentDowngradeTrigger::ParserUnavailable,
                M5ArtifactComponentDowngradeTrigger::SchemaUnrecognized,
                M5ArtifactComponentDowngradeTrigger::PolicyBlocked,
            ],
            rollback_posture: M5ArtifactComponentRollbackPosture::ReadOnlyNoMutation,
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_DIFF_MODE_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5ArtifactComponentConsumerSurface::DiffCompareView,
                M5ArtifactComponentConsumerSurface::NotebookReview,
                M5ArtifactComponentConsumerSurface::CliHeadless,
                M5ArtifactComponentConsumerSurface::SupportExport,
            ],
        },
        M5ArtifactComponentMatrixRow {
            component: M5ArtifactComponent::StructureRow,
            maturity: M5ArtifactComponentMaturityClass::Stable,
            scope_summary: "Structure row showing one structured path (cell, node, key, or symbol) and its change class so nested structure is never flattened into opaque line noise".to_owned(),
            canonical_source_disclosure: "Names the structured path against its canonical artifact so a row maps back to a real location rather than a synthetic index".to_owned(),
            fidelity_narrowing_vocab: vec![
                M5ArtifactFidelityState::StructuredFaithful,
                M5ArtifactFidelityState::StructuredPartial,
                M5ArtifactFidelityState::SchemaUnrecognized,
                M5ArtifactFidelityState::RawFallback,
            ],
            compare_write_back_safety: "Structure rows are read-only anchors; selecting one navigates without mutating the artifact".to_owned(),
            render_trust_disclosure: "Marks whether the structure was parsed faithfully or partially so a partial parse is never presented as complete structure".to_owned(),
            generated_from_relation: "When a structure row belongs to a generated artifact it inherits the generated-from marker from the identity bar rather than hiding it".to_owned(),
            evidence_requirement: M5ArtifactComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:structure-row-path-and-change-class:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5ArtifactComponentDowngradeTrigger::ProofStale,
                M5ArtifactComponentDowngradeTrigger::SchemaUnrecognized,
                M5ArtifactComponentDowngradeTrigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: M5ArtifactComponentRollbackPosture::ReadOnlyNoMutation,
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_STRUCTURE_ROW_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5ArtifactComponentConsumerSurface::DiffCompareView,
                M5ArtifactComponentConsumerSurface::NotebookReview,
                M5ArtifactComponentConsumerSurface::ArtifactBrowser,
                M5ArtifactComponentConsumerSurface::SupportExport,
            ],
        },
        M5ArtifactComponentMatrixRow {
            component: M5ArtifactComponent::MergeDecisionRow,
            maturity: M5ArtifactComponentMaturityClass::Stable,
            scope_summary: "Merge-decision row picking base, ours, or theirs for one structured conflict with the resulting write-back safety always explicit".to_owned(),
            canonical_source_disclosure: "Names the conflicting structured path and the base/ours/theirs lineage so a decision is never applied to an ambiguous target".to_owned(),
            fidelity_narrowing_vocab: vec![
                M5ArtifactFidelityState::StructuredFaithful,
                M5ArtifactFidelityState::StructuredPartial,
                M5ArtifactFidelityState::SchemaUnrecognized,
                M5ArtifactFidelityState::RawFallback,
            ],
            compare_write_back_safety: "The row states whether resolving writes back to a writable artifact or stays compare-only, and a compare-only artifact is never silently promoted to writable".to_owned(),
            render_trust_disclosure: "Marks whether the merged preview is a trusted structured merge or a raw fallback so an untrusted merge is never applied blindly".to_owned(),
            generated_from_relation: "When a side is generated the row surfaces the generated-from relation so a merge decision never overwrites a regenerable artifact by hand".to_owned(),
            evidence_requirement: M5ArtifactComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:merge-decision-row-side-and-lineage:m5".to_owned(),
                "evidence:merge-decision-row-write-back-safety:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5ArtifactComponentDowngradeTrigger::ProofStale,
                M5ArtifactComponentDowngradeTrigger::CompareOnlyEnforced,
                M5ArtifactComponentDowngradeTrigger::SchemaUnrecognized,
                M5ArtifactComponentDowngradeTrigger::PolicyBlocked,
            ],
            rollback_posture: M5ArtifactComponentRollbackPosture::WriteBackAttributable,
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_MERGE_DECISION_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5ArtifactComponentConsumerSurface::MergeConflictWorkspace,
                M5ArtifactComponentConsumerSurface::NotebookReview,
                M5ArtifactComponentConsumerSurface::CliHeadless,
                M5ArtifactComponentConsumerSurface::SupportExport,
            ],
        },
        M5ArtifactComponentMatrixRow {
            component: M5ArtifactComponent::GeneratedArtifactNotice,
            maturity: M5ArtifactComponentMaturityClass::Stable,
            scope_summary: "Generated-artifact notice naming the generating source of truth and regeneration path so a generated file is never edited as if it were authoritative".to_owned(),
            canonical_source_disclosure: "Names the generated-from source of truth and marks the artifact non-authoritative, never presenting the generated output as the canonical source".to_owned(),
            fidelity_narrowing_vocab: vec![
                M5ArtifactFidelityState::StructuredFaithful,
                M5ArtifactFidelityState::StructuredPartial,
                M5ArtifactFidelityState::RawFallback,
                M5ArtifactFidelityState::RedactedOrWithheld,
            ],
            compare_write_back_safety: "Marks the artifact regenerate-only so manual write-back is blocked in favor of regenerating from source".to_owned(),
            render_trust_disclosure: "States whether the generation lineage was verified fresh or is stale so a stale generated artifact is labeled rather than trusted".to_owned(),
            generated_from_relation: "This is the primary carrier of the generated-from relation and never hides it behind generic file chrome".to_owned(),
            evidence_requirement: M5ArtifactComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:generated-artifact-notice-source-relation:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5ArtifactComponentDowngradeTrigger::ProofStale,
                M5ArtifactComponentDowngradeTrigger::GeneratedArtifactStale,
                M5ArtifactComponentDowngradeTrigger::PolicyBlocked,
                M5ArtifactComponentDowngradeTrigger::ScopeExpansionUnqualified,
            ],
            rollback_posture: M5ArtifactComponentRollbackPosture::RegenerateOnlyNoManualEdit,
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_GENERATED_NOTICE_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5ArtifactComponentConsumerSurface::ReviewWorkspace,
                M5ArtifactComponentConsumerSurface::DiffCompareView,
                M5ArtifactComponentConsumerSurface::ArtifactBrowser,
                M5ArtifactComponentConsumerSurface::SupportExport,
            ],
        },
        M5ArtifactComponentMatrixRow {
            component: M5ArtifactComponent::RenderedCompareViewer,
            maturity: M5ArtifactComponentMaturityClass::Beta,
            scope_summary: "Rendered compare viewer showing two structured or media-like artifacts side by side with the render-trust class always explicit".to_owned(),
            canonical_source_disclosure: "Names both compared artifacts and their canonical sources so a rendered comparison is anchored to real inputs".to_owned(),
            fidelity_narrowing_vocab: vec![
                M5ArtifactFidelityState::StructuredFaithful,
                M5ArtifactFidelityState::RenderUntrusted,
                M5ArtifactFidelityState::RawFallback,
                M5ArtifactFidelityState::RedactedOrWithheld,
            ],
            compare_write_back_safety: "The viewer is strictly compare-only and never writes back to either rendered artifact".to_owned(),
            render_trust_disclosure: "This is the primary carrier of render-trust class; an untrusted render is labeled untrusted and offers an export-safe raw fallback".to_owned(),
            generated_from_relation: "When a rendered side is generated the viewer surfaces the generated-from relation so a render is never mistaken for the source of truth".to_owned(),
            evidence_requirement: M5ArtifactComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:rendered-compare-viewer-render-trust:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5ArtifactComponentDowngradeTrigger::ProofStale,
                M5ArtifactComponentDowngradeTrigger::RenderUntrusted,
                M5ArtifactComponentDowngradeTrigger::CompareOnlyEnforced,
                M5ArtifactComponentDowngradeTrigger::ScopeExpansionUnqualified,
            ],
            rollback_posture: M5ArtifactComponentRollbackPosture::CompareOnlyNoWriteBack,
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_RENDERED_VIEWER_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5ArtifactComponentConsumerSurface::DiffCompareView,
                M5ArtifactComponentConsumerSurface::BrowserCompanion,
                M5ArtifactComponentConsumerSurface::SupportExport,
                M5ArtifactComponentConsumerSurface::HelpAbout,
            ],
        },
        M5ArtifactComponentMatrixRow {
            component: M5ArtifactComponent::MediaMetadataRail,
            maturity: M5ArtifactComponentMaturityClass::Preview,
            scope_summary: "Media-metadata rail exposing dimensions, encoding, and provenance metadata for media-like artifacts so metadata visibility is never dropped from a preview".to_owned(),
            canonical_source_disclosure: "Names the media artifact and its canonical source, distinguishing captured metadata from inferred metadata".to_owned(),
            fidelity_narrowing_vocab: vec![
                M5ArtifactFidelityState::StructuredFaithful,
                M5ArtifactFidelityState::StructuredPartial,
                M5ArtifactFidelityState::RawFallback,
                M5ArtifactFidelityState::RedactedOrWithheld,
            ],
            compare_write_back_safety: "The rail is read-only and never writes back to the media artifact or its metadata".to_owned(),
            render_trust_disclosure: "Marks whether metadata was extracted from a trusted decoder or is unavailable so missing metadata is shown as missing rather than blank".to_owned(),
            generated_from_relation: "When media is generated (a rendered snapshot or export) the rail names the generated-from relation rather than treating it as an original capture".to_owned(),
            evidence_requirement: M5ArtifactComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:media-metadata-rail-visibility:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5ArtifactComponentDowngradeTrigger::ProofStale,
                M5ArtifactComponentDowngradeTrigger::MediaMetadataUnavailable,
                M5ArtifactComponentDowngradeTrigger::RedactionApplied,
                M5ArtifactComponentDowngradeTrigger::ScopeExpansionUnqualified,
            ],
            rollback_posture: M5ArtifactComponentRollbackPosture::ReadOnlyNoMutation,
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_MEDIA_RAIL_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5ArtifactComponentConsumerSurface::DiffCompareView,
                M5ArtifactComponentConsumerSurface::ArtifactBrowser,
                M5ArtifactComponentConsumerSurface::SupportExport,
                M5ArtifactComponentConsumerSurface::HelpAbout,
            ],
        },
        M5ArtifactComponentMatrixRow {
            component: M5ArtifactComponent::RedactionOrTrustBadgeSet,
            maturity: M5ArtifactComponentMaturityClass::Stable,
            scope_summary: "Redaction or trust badge set naming redaction, export, and safe-preview posture so a redacted or untrusted artifact is never presented as fully visible or fully trusted".to_owned(),
            canonical_source_disclosure: "Names the artifact and its redaction/trust class so the badge maps to a real posture rather than a decorative label".to_owned(),
            fidelity_narrowing_vocab: vec![
                M5ArtifactFidelityState::StructuredFaithful,
                M5ArtifactFidelityState::RenderUntrusted,
                M5ArtifactFidelityState::RawFallback,
                M5ArtifactFidelityState::RedactedOrWithheld,
            ],
            compare_write_back_safety: "The badge set is read-only; it annotates safety and never itself mutates the artifact".to_owned(),
            render_trust_disclosure: "This is the primary carrier of trust/safe-preview class and marks an untrusted preview so it is never opened as trusted".to_owned(),
            generated_from_relation: "When a badge covers a generated artifact it preserves the generated-from marker so trust posture and lineage stay together".to_owned(),
            evidence_requirement: M5ArtifactComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:redaction-trust-badge-set-posture:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5ArtifactComponentDowngradeTrigger::ProofStale,
                M5ArtifactComponentDowngradeTrigger::RedactionApplied,
                M5ArtifactComponentDowngradeTrigger::RenderUntrusted,
                M5ArtifactComponentDowngradeTrigger::PolicyBlocked,
            ],
            rollback_posture: M5ArtifactComponentRollbackPosture::ReadOnlyNoMutation,
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_REDACTION_BADGE_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5ArtifactComponentConsumerSurface::ReviewWorkspace,
                M5ArtifactComponentConsumerSurface::ArtifactBrowser,
                M5ArtifactComponentConsumerSurface::SupportExport,
                M5ArtifactComponentConsumerSurface::HelpAbout,
            ],
        },
        M5ArtifactComponentMatrixRow {
            component: M5ArtifactComponent::CompareSummaryCard,
            maturity: M5ArtifactComponentMaturityClass::Stable,
            scope_summary: "Compare-summary card rolling up an artifact comparison (added/removed/changed structure, render trust, safety) without flattening distinct classes into a single verdict".to_owned(),
            canonical_source_disclosure: "Names the compared artifacts and their canonical sources so the summary is attributable to real inputs".to_owned(),
            fidelity_narrowing_vocab: vec![
                M5ArtifactFidelityState::StructuredFaithful,
                M5ArtifactFidelityState::StructuredPartial,
                M5ArtifactFidelityState::SchemaUnrecognized,
                M5ArtifactFidelityState::RawFallback,
            ],
            compare_write_back_safety: "The card summarizes safety but is read-only and never itself writes back or promotes a compare-only artifact".to_owned(),
            render_trust_disclosure: "Rolls up per-side render trust so an untrusted render is visible in the summary rather than averaged away".to_owned(),
            generated_from_relation: "Surfaces when either side is generated so a summary never hides that a change is regenerable rather than hand-authored".to_owned(),
            evidence_requirement: M5ArtifactComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:compare-summary-card-no-flatten:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5ArtifactComponentDowngradeTrigger::ProofStale,
                M5ArtifactComponentDowngradeTrigger::SchemaUnrecognized,
                M5ArtifactComponentDowngradeTrigger::TrustNarrowing,
                M5ArtifactComponentDowngradeTrigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: M5ArtifactComponentRollbackPosture::ReadOnlyNoMutation,
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_COMPARE_SUMMARY_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5ArtifactComponentConsumerSurface::ReviewWorkspace,
                M5ArtifactComponentConsumerSurface::DiffCompareView,
                M5ArtifactComponentConsumerSurface::CliHeadless,
                M5ArtifactComponentConsumerSurface::SupportExport,
            ],
        },
    ]
}

fn trust_review() -> M5ArtifactComponentMatrixTrustReview {
    M5ArtifactComponentMatrixTrustReview {
        canonical_source_always_explicit: true,
        compare_only_never_silently_writable: true,
        structured_mode_never_flattened_without_explanation: true,
        generated_from_relation_never_hidden: true,
        render_trust_explicit: true,
        metadata_visibility_explicit: true,
        raw_export_safe_fallback_explicit: true,
        redaction_posture_explicit: true,
        parser_schema_state_explicit: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> M5ArtifactComponentMatrixConsumerProjection {
    M5ArtifactComponentMatrixConsumerProjection {
        artifact_identity_bar_shows_class_and_canonical_source: true,
        diff_mode_switcher_shows_available_modes_and_active: true,
        structure_row_shows_path_and_change_class: true,
        merge_decision_row_shows_side_and_write_back_safety: true,
        generated_artifact_notice_shows_generated_from_relation: true,
        rendered_compare_viewer_shows_render_trust: true,
        media_metadata_rail_shows_metadata_visibility: true,
        redaction_or_trust_badge_set_shows_redaction_posture: true,
        compare_summary_card_shows_summary_without_flattening: true,
        cli_headless_shows_component_truth: true,
        support_export_shows_component_truth: true,
    }
}

fn proof_freshness() -> M5ArtifactComponentMatrixProofFreshness {
    M5ArtifactComponentMatrixProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-07-08T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        M5_ARTIFACT_COMPONENT_MATRIX_SCHEMA_REF.to_owned(),
        M5_ARTIFACT_COMPONENT_MATRIX_DOC_REF.to_owned(),
        M5_ARTIFACT_COMPONENT_MATRIX_IDENTITY_BAR_CONTRACT_REF.to_owned(),
        M5_ARTIFACT_COMPONENT_MATRIX_DIFF_MODE_CONTRACT_REF.to_owned(),
        M5_ARTIFACT_COMPONENT_MATRIX_STRUCTURE_ROW_CONTRACT_REF.to_owned(),
        M5_ARTIFACT_COMPONENT_MATRIX_MERGE_DECISION_CONTRACT_REF.to_owned(),
        M5_ARTIFACT_COMPONENT_MATRIX_GENERATED_NOTICE_CONTRACT_REF.to_owned(),
        M5_ARTIFACT_COMPONENT_MATRIX_RENDERED_VIEWER_CONTRACT_REF.to_owned(),
        M5_ARTIFACT_COMPONENT_MATRIX_MEDIA_RAIL_CONTRACT_REF.to_owned(),
        M5_ARTIFACT_COMPONENT_MATRIX_REDACTION_BADGE_CONTRACT_REF.to_owned(),
        M5_ARTIFACT_COMPONENT_MATRIX_COMPARE_SUMMARY_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> M5ArtifactComponentMatrixPacket {
    M5ArtifactComponentMatrixPacket::new(M5ArtifactComponentMatrixPacketInput {
        packet_id: PACKET_ID.to_owned(),
        matrix_label: "M5 Structured-Artifact Review Component Matrix".to_owned(),
        component_rows: component_rows(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-08T00:00:00Z".to_owned(),
    })
}

#[test]
fn m5_artifact_component_matrix_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn matrix_has_nine_components() {
    assert_eq!(packet().component_rows.len(), 9);
    assert_eq!(M5ArtifactComponent::ALL.len(), 9);
}

#[test]
fn missing_component_fails_validation() {
    let mut packet = packet();
    packet
        .component_rows
        .retain(|row| row.component != M5ArtifactComponent::MergeDecisionRow);
    assert!(packet
        .validate()
        .contains(&M5ArtifactComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn component_source_contract_mismatch_fails() {
    let mut packet = packet();
    packet.component_rows[0].source_contract_refs =
        vec![M5_ARTIFACT_COMPONENT_MATRIX_COMPARE_SUMMARY_CONTRACT_REF.to_owned()];
    assert!(packet
        .validate()
        .contains(&M5ArtifactComponentMatrixViolation::ComponentSourceContractMismatch));
}

#[test]
fn each_component_lists_its_canonical_contract() {
    for row in packet().component_rows {
        assert!(
            row.source_contract_refs
                .contains(&row.component.canonical_source_contract_ref().to_owned()),
            "component {} missing canonical contract",
            row.component.as_str()
        );
    }
}

#[test]
fn stable_component_missing_evidence_fails() {
    let mut packet = packet();
    packet.component_rows[0]
        .required_evidence_packet_refs
        .clear();
    assert!(packet
        .validate()
        .contains(&M5ArtifactComponentMatrixViolation::StableComponentMissingEvidence));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.component_rows[1].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5ArtifactComponentMatrixViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = packet();
    packet.component_rows[2].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5ArtifactComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_canonical_source_disclosure_fails() {
    let mut packet = packet();
    packet.component_rows[0].canonical_source_disclosure = "   ".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ArtifactComponentMatrixViolation::CanonicalSourceDisclosureMissing));
}

#[test]
fn missing_fidelity_narrowing_vocab_fails() {
    let mut packet = packet();
    packet.component_rows[4].fidelity_narrowing_vocab.clear();
    assert!(packet
        .validate()
        .contains(&M5ArtifactComponentMatrixViolation::FidelityNarrowingVocabMissing));
}

#[test]
fn missing_compare_write_back_safety_fails() {
    let mut packet = packet();
    packet.component_rows[3].compare_write_back_safety = String::new();
    assert!(packet
        .validate()
        .contains(&M5ArtifactComponentMatrixViolation::CompareWriteBackSafetyMissing));
}

#[test]
fn missing_render_trust_disclosure_fails() {
    let mut packet = packet();
    packet.component_rows[5].render_trust_disclosure = String::new();
    assert!(packet
        .validate()
        .contains(&M5ArtifactComponentMatrixViolation::RenderTrustDisclosureMissing));
}

#[test]
fn missing_generated_from_relation_fails() {
    let mut packet = packet();
    packet.component_rows[4].generated_from_relation = String::new();
    assert!(packet
        .validate()
        .contains(&M5ArtifactComponentMatrixViolation::GeneratedFromRelationMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ArtifactComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.compare_only_never_silently_writable = false;
    assert!(packet
        .validate()
        .contains(&M5ArtifactComponentMatrixViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .rendered_compare_viewer_shows_render_trust = false;
    assert!(packet
        .validate()
        .contains(&M5ArtifactComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ArtifactComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "wrong".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ArtifactComponentMatrixViolation::WrongRecordKind));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    packet.component_rows[0].scope_summary = "leaked password value".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ArtifactComponentMatrixViolation::RawBoundaryMaterialInExport));
}

#[test]
fn only_two_components_are_narrowed() {
    let narrowed = packet()
        .component_rows
        .iter()
        .filter(|row| !row.maturity.is_stable())
        .count();
    assert_eq!(narrowed, 2);
}

#[test]
fn markdown_summary_lists_every_component() {
    let summary = packet().render_markdown_summary();
    for component in M5ArtifactComponent::ALL {
        assert!(
            summary.contains(component.as_str()),
            "summary missing component {}",
            component.as_str()
        );
    }
}

#[test]
fn export_safe_json_roundtrips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: M5ArtifactComponentMatrixPacket =
        serde_json::from_str(&json).expect("export json roundtrips");
    assert_eq!(parsed, packet);
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_artifact_component_matrix_export()
        .expect("checked M5 artifact-component matrix export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let checked = current_stable_m5_artifact_component_matrix_export()
        .expect("checked M5 artifact-component matrix export validates");
    assert_eq!(checked, packet());
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-structured-artifact-review-components/rendered_compare_viewer_render_untrusted.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-structured-artifact-review-components/media_metadata_rail_metadata_unavailable.json"
        )),
    ] {
        let packet: M5ArtifactComponentMatrixPacket =
            serde_json::from_str(raw).expect("fixture parses as matrix packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

/// Regenerates the checked release artifacts and narrowed fixtures.
///
/// Guarded behind `GEN_STRUCTURED_ARTIFACT_REVIEW_ARTIFACTS` so ordinary test
/// runs never touch the working tree. Run with the env var set to refresh the
/// checked-in support export, summary, and fixtures from the seed builder.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_STRUCTURED_ARTIFACT_REVIEW_ARTIFACTS").is_err() {
        return;
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_root = std::path::Path::new(manifest_dir).join("..").join("..");

    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let proof_dir = repo_root
        .join("artifacts")
        .join("release")
        .join("m5-structured-artifact-review-proof");
    std::fs::create_dir_all(&proof_dir).expect("create proof dir");
    std::fs::write(
        proof_dir.join("support_export.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(
        proof_dir.join("summary.md"),
        packet.render_markdown_summary(),
    )
    .expect("write summary");

    let fixture_dir = repo_root
        .join("fixtures")
        .join("ui")
        .join("m5-structured-artifact-review-components");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");

    let mut rendered = packet.clone();
    rendered.packet_id = "m5-artifact-component-matrix:fixture:rendered-untrusted".to_owned();
    if let Some(row) = rendered
        .component_rows
        .iter_mut()
        .find(|row| row.component == M5ArtifactComponent::RenderedCompareViewer)
    {
        row.fidelity_narrowing_vocab = vec![
            M5ArtifactFidelityState::RenderUntrusted,
            M5ArtifactFidelityState::RawFallback,
        ];
    }
    assert!(rendered.validate().is_empty(), "{:?}", rendered.validate());
    std::fs::write(
        fixture_dir.join("rendered_compare_viewer_render_untrusted.json"),
        format!("{}\n", rendered.export_safe_json()),
    )
    .expect("write rendered fixture");

    let mut media = packet.clone();
    media.packet_id = "m5-artifact-component-matrix:fixture:media-metadata-unavailable".to_owned();
    if let Some(row) = media
        .component_rows
        .iter_mut()
        .find(|row| row.component == M5ArtifactComponent::MediaMetadataRail)
    {
        row.fidelity_narrowing_vocab = vec![
            M5ArtifactFidelityState::RawFallback,
            M5ArtifactFidelityState::RedactedOrWithheld,
        ];
    }
    assert!(media.validate().is_empty(), "{:?}", media.validate());
    std::fs::write(
        fixture_dir.join("media_metadata_rail_metadata_unavailable.json"),
        format!("{}\n", media.export_safe_json()),
    )
    .expect("write media fixture");
}
