use super::*;

const PACKET_ID: &str = "artifact-identity-diff-mode-controls:stable:0001";

fn identity_bars() -> Vec<ArtifactIdentityBar> {
    vec![
        ArtifactIdentityBar {
            component: M5ArtifactComponent::ArtifactIdentityBar,
            bar_id: "bar:notebook".to_owned(),
            artifact_ref: "artifact:notebooks/analysis.ipynb".to_owned(),
            artifact_class_label: "Jupyter notebook".to_owned(),
            origin_class: ArtifactOriginClass::AuthoredInRepo,
            canonical_source_disclosure: "Canonical source: notebooks/analysis.ipynb in this repo"
                .to_owned(),
            parser_schema_state: M5ArtifactFidelityState::StructuredFaithful,
            claims_writable_target: true,
            generated_from_relation: String::new(),
            source_of_truth_pointer: String::new(),
            raw_fallback_note: String::new(),
            rollback_posture: M5ArtifactComponentRollbackPosture::WriteBackAttributable,
            fields_shown: vec![
                "artifact_class".to_owned(),
                "canonical_source".to_owned(),
                "origin_class".to_owned(),
                "parser_schema_state".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_IDENTITY_BAR_CONTRACT_REF.to_owned()
            ],
        },
        ArtifactIdentityBar {
            component: M5ArtifactComponent::ArtifactIdentityBar,
            bar_id: "bar:generated-client".to_owned(),
            artifact_ref: "artifact:gen/api_client.rs".to_owned(),
            artifact_class_label: "generated API client".to_owned(),
            origin_class: ArtifactOriginClass::GeneratedFromSource,
            canonical_source_disclosure:
                "Canonical source: openapi/spec.yaml; this file is generated output".to_owned(),
            parser_schema_state: M5ArtifactFidelityState::StructuredFaithful,
            claims_writable_target: false,
            generated_from_relation: "Generated from openapi/spec.yaml by the client generator"
                .to_owned(),
            source_of_truth_pointer: "Edit openapi/spec.yaml, then regenerate".to_owned(),
            raw_fallback_note: String::new(),
            rollback_posture: M5ArtifactComponentRollbackPosture::RegenerateOnlyNoManualEdit,
            fields_shown: vec![
                "artifact_class".to_owned(),
                "canonical_source".to_owned(),
                "generated_from".to_owned(),
                "origin_class".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_IDENTITY_BAR_CONTRACT_REF.to_owned()
            ],
        },
        ArtifactIdentityBar {
            component: M5ArtifactComponent::ArtifactIdentityBar,
            bar_id: "bar:imported-sbom".to_owned(),
            artifact_ref: "artifact:sbom/spdx.json".to_owned(),
            artifact_class_label: "imported SBOM (SPDX)".to_owned(),
            origin_class: ArtifactOriginClass::ImportedExternal,
            canonical_source_disclosure:
                "Canonical source: build pipeline SBOM export (imported snapshot)".to_owned(),
            parser_schema_state: M5ArtifactFidelityState::StructuredPartial,
            claims_writable_target: false,
            generated_from_relation: String::new(),
            source_of_truth_pointer:
                "Authoritative SBOM lives in the build pipeline artifact store".to_owned(),
            raw_fallback_note: "Structured view is partial; raw JSON fallback remains available"
                .to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::CompareOnlyNoWriteBack,
            fields_shown: vec![
                "artifact_class".to_owned(),
                "canonical_source".to_owned(),
                "source_of_truth".to_owned(),
                "parser_schema_state".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_IDENTITY_BAR_CONTRACT_REF.to_owned()
            ],
        },
        ArtifactIdentityBar {
            component: M5ArtifactComponent::ArtifactIdentityBar,
            bar_id: "bar:design-snapshot".to_owned(),
            artifact_ref: "artifact:design/home.snapshot".to_owned(),
            artifact_class_label: "design snapshot (media)".to_owned(),
            origin_class: ArtifactOriginClass::PolicyOwned,
            canonical_source_disclosure:
                "Canonical source: design system snapshot governed by design policy".to_owned(),
            parser_schema_state: M5ArtifactFidelityState::RenderUntrusted,
            claims_writable_target: false,
            generated_from_relation: String::new(),
            source_of_truth_pointer: "Authoritative snapshot lives in the design system library"
                .to_owned(),
            raw_fallback_note:
                "Render is not fully trusted; raw/export-safe fallback stays reachable".to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::ReadOnlyNoMutation,
            fields_shown: vec![
                "artifact_class".to_owned(),
                "canonical_source".to_owned(),
                "source_of_truth".to_owned(),
                "parser_schema_state".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_IDENTITY_BAR_CONTRACT_REF.to_owned()
            ],
        },
    ]
}

fn diff_switchers() -> Vec<DiffModeSwitcher> {
    vec![
        DiffModeSwitcher {
            component: M5ArtifactComponent::DiffModeSwitcher,
            switcher_id: "switch:notebook".to_owned(),
            artifact_ref: "artifact:notebooks/analysis.ipynb".to_owned(),
            artifact_class_label: "Jupyter notebook".to_owned(),
            options: vec![
                DiffModeOption {
                    lens: DiffReviewLens::StructuredSemantic,
                    availability: DiffLensAvailability::Available,
                    unavailability_reason: String::new(),
                },
                DiffModeOption {
                    lens: DiffReviewLens::SideBySide,
                    availability: DiffLensAvailability::Available,
                    unavailability_reason: String::new(),
                },
                DiffModeOption {
                    lens: DiffReviewLens::ThreeWayMerge,
                    availability: DiffLensAvailability::Available,
                    unavailability_reason: String::new(),
                },
                DiffModeOption {
                    lens: DiffReviewLens::RawTextFallback,
                    availability: DiffLensAvailability::Available,
                    unavailability_reason: String::new(),
                },
            ],
            active_lens: DiffReviewLens::StructuredSemantic,
            compare_write_back_safety: "Write-back to the notebook stays individually attributable"
                .to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::WriteBackAttributable,
            fields_shown: vec![
                "available_lenses".to_owned(),
                "active_lens".to_owned(),
                "compare_write_back_safety".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_DIFF_MODE_CONTRACT_REF.to_owned()
            ],
        },
        DiffModeSwitcher {
            component: M5ArtifactComponent::DiffModeSwitcher,
            switcher_id: "switch:generated-client".to_owned(),
            artifact_ref: "artifact:gen/api_client.rs".to_owned(),
            artifact_class_label: "generated API client".to_owned(),
            options: vec![
                DiffModeOption {
                    lens: DiffReviewLens::StructuredSemantic,
                    availability: DiffLensAvailability::Available,
                    unavailability_reason: String::new(),
                },
                DiffModeOption {
                    lens: DiffReviewLens::SideBySide,
                    availability: DiffLensAvailability::Available,
                    unavailability_reason: String::new(),
                },
                DiffModeOption {
                    lens: DiffReviewLens::RawTextFallback,
                    availability: DiffLensAvailability::Available,
                    unavailability_reason: String::new(),
                },
            ],
            active_lens: DiffReviewLens::StructuredSemantic,
            compare_write_back_safety:
                "Compare-only: regenerate from openapi/spec.yaml rather than writing back"
                    .to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::RegenerateOnlyNoManualEdit,
            fields_shown: vec![
                "available_lenses".to_owned(),
                "active_lens".to_owned(),
                "compare_write_back_safety".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_DIFF_MODE_CONTRACT_REF.to_owned()
            ],
        },
        DiffModeSwitcher {
            component: M5ArtifactComponent::DiffModeSwitcher,
            switcher_id: "switch:imported-sbom".to_owned(),
            artifact_ref: "artifact:sbom/spdx.json".to_owned(),
            artifact_class_label: "imported SBOM (SPDX)".to_owned(),
            options: vec![
                DiffModeOption {
                    lens: DiffReviewLens::StructuredSemantic,
                    availability: DiffLensAvailability::Available,
                    unavailability_reason: String::new(),
                },
                DiffModeOption {
                    lens: DiffReviewLens::ThreeWayMerge,
                    availability: DiffLensAvailability::UnavailablePolicyBlocked,
                    unavailability_reason:
                        "Merge is disabled for imported SBOMs; they are compare-only".to_owned(),
                },
                DiffModeOption {
                    lens: DiffReviewLens::RawTextFallback,
                    availability: DiffLensAvailability::Available,
                    unavailability_reason: String::new(),
                },
            ],
            active_lens: DiffReviewLens::StructuredSemantic,
            compare_write_back_safety: "Compare-only: imported SBOM is never written back"
                .to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::CompareOnlyNoWriteBack,
            fields_shown: vec![
                "available_lenses".to_owned(),
                "active_lens".to_owned(),
                "compare_write_back_safety".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_DIFF_MODE_CONTRACT_REF.to_owned()
            ],
        },
        DiffModeSwitcher {
            component: M5ArtifactComponent::DiffModeSwitcher,
            switcher_id: "switch:design-snapshot".to_owned(),
            artifact_ref: "artifact:design/home.snapshot".to_owned(),
            artifact_class_label: "design snapshot (media)".to_owned(),
            options: vec![
                DiffModeOption {
                    lens: DiffReviewLens::MediaVisual,
                    availability: DiffLensAvailability::Available,
                    unavailability_reason: String::new(),
                },
                DiffModeOption {
                    lens: DiffReviewLens::StructuredSemantic,
                    availability: DiffLensAvailability::UnavailableSchemaUnrecognized,
                    unavailability_reason:
                        "No schema recognizes this snapshot; structured lens is off".to_owned(),
                },
                DiffModeOption {
                    lens: DiffReviewLens::RawTextFallback,
                    availability: DiffLensAvailability::Available,
                    unavailability_reason: String::new(),
                },
            ],
            active_lens: DiffReviewLens::MediaVisual,
            compare_write_back_safety: "Compare-only: design snapshots are read-only here"
                .to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::ReadOnlyNoMutation,
            fields_shown: vec![
                "available_lenses".to_owned(),
                "active_lens".to_owned(),
                "compare_write_back_safety".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_DIFF_MODE_CONTRACT_REF.to_owned()
            ],
        },
    ]
}

fn trust_review() -> ArtifactReviewControlsTrustReview {
    ArtifactReviewControlsTrustReview {
        artifact_class_always_explicit: true,
        canonical_source_never_buried: true,
        generated_from_relation_never_hidden: true,
        compare_only_never_silently_writable: true,
        parser_schema_state_explicit: true,
        review_lenses_enumerated: true,
        lens_unavailability_reason_explicit: true,
        raw_fallback_always_available: true,
        writable_target_matches_origin: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> ArtifactReviewControlsConsumerProjection {
    ArtifactReviewControlsConsumerProjection {
        identity_bar_shows_class_and_canonical_source: true,
        diff_switcher_shows_available_and_unavailable_lenses: true,
        compare_only_truth_shown_inline: true,
        generated_from_relation_shown: true,
        raw_fallback_reachable: true,
        cli_headless_shows_truth: true,
        support_export_shows_truth: true,
        help_about_shows_truth: true,
    }
}

fn proof_freshness() -> ArtifactReviewControlsProofFreshness {
    ArtifactReviewControlsProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-07-08T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<ArtifactReviewControlsDowngradeTrigger> {
    vec![
        ArtifactReviewControlsDowngradeTrigger::ProofStale,
        ArtifactReviewControlsDowngradeTrigger::SchemaUnrecognized,
        ArtifactReviewControlsDowngradeTrigger::RenderUntrusted,
        ArtifactReviewControlsDowngradeTrigger::CompareOnlyEnforced,
        ArtifactReviewControlsDowngradeTrigger::UpstreamDependencyNarrowed,
    ]
}

fn consumer_surfaces() -> Vec<ArtifactReviewControlsConsumerSurface> {
    vec![
        ArtifactReviewControlsConsumerSurface::DiffCompareView,
        ArtifactReviewControlsConsumerSurface::MergeConflictWorkspace,
        ArtifactReviewControlsConsumerSurface::NotebookReview,
        ArtifactReviewControlsConsumerSurface::ArtifactBrowser,
        ArtifactReviewControlsConsumerSurface::CliHeadless,
        ArtifactReviewControlsConsumerSurface::SupportExport,
    ]
}

fn source_contract_refs() -> Vec<String> {
    vec![
        ARTIFACT_REVIEW_CONTROLS_SCHEMA_REF.to_owned(),
        ARTIFACT_REVIEW_CONTROLS_DOC_REF.to_owned(),
        M5_ARTIFACT_COMPONENT_MATRIX_SCHEMA_REF.to_owned(),
        M5_ARTIFACT_COMPONENT_MATRIX_IDENTITY_BAR_CONTRACT_REF.to_owned(),
        M5_ARTIFACT_COMPONENT_MATRIX_DIFF_MODE_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> ArtifactReviewControlsPacket {
    ArtifactReviewControlsPacket::new(ArtifactReviewControlsPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Artifact identity bars and diff-mode switchers".to_owned(),
        identity_bars: identity_bars(),
        diff_switchers: diff_switchers(),
        downgrade_triggers: downgrade_triggers(),
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-08T00:00:00Z".to_owned(),
    })
}

#[test]
fn artifact_review_controls_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn resolver_derives_writable_target_from_origin_and_parser() {
    // Authored + faithful is the only writable target.
    let authored = resolve_artifact_identity_disclosure(
        ArtifactOriginClass::AuthoredInRepo,
        M5ArtifactFidelityState::StructuredFaithful,
    );
    assert!(authored.asserts_writable_target);
    assert!(!authored.needs_source_of_truth_pointer);

    // Authored but schema-unrecognized is not a writable target.
    let unparsed = resolve_artifact_identity_disclosure(
        ArtifactOriginClass::AuthoredInRepo,
        M5ArtifactFidelityState::SchemaUnrecognized,
    );
    assert!(!unparsed.asserts_writable_target);
    assert!(unparsed.needs_raw_fallback_note);

    // Generated always names its generated-from relation and points at the source.
    let generated = resolve_artifact_identity_disclosure(
        ArtifactOriginClass::GeneratedFromSource,
        M5ArtifactFidelityState::StructuredFaithful,
    );
    assert!(!generated.asserts_writable_target);
    assert!(generated.needs_generated_from_relation);
    assert!(generated.needs_source_of_truth_pointer);
}

#[test]
fn authored_artifact_falsely_claiming_writable_when_unparsed_fails() {
    let mut packet = packet();
    packet.identity_bars[0].parser_schema_state = M5ArtifactFidelityState::SchemaUnrecognized;
    // claims_writable_target is still true, but the parser no longer supports it.
    assert!(packet
        .validate()
        .contains(&ArtifactReviewControlsViolation::WritableTargetMisrepresented));
}

#[test]
fn compare_only_artifact_claiming_writable_fails() {
    let mut packet = packet();
    packet.identity_bars[2].claims_writable_target = true;
    assert!(packet
        .validate()
        .contains(&ArtifactReviewControlsViolation::WritableTargetMisrepresented));
}

#[test]
fn generated_artifact_without_relation_fails() {
    let mut packet = packet();
    packet.identity_bars[1].generated_from_relation = String::new();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewControlsViolation::GeneratedFromRelationMissing));
}

#[test]
fn imported_artifact_without_source_of_truth_pointer_fails() {
    let mut packet = packet();
    packet.identity_bars[2].source_of_truth_pointer = String::new();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewControlsViolation::SourceOfTruthPointerMissing));
}

#[test]
fn narrowed_parser_without_raw_fallback_note_fails() {
    let mut packet = packet();
    packet.identity_bars[2].raw_fallback_note = String::new();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewControlsViolation::RawFallbackNoteMissing));
}

#[test]
fn missing_canonical_source_disclosure_fails() {
    let mut packet = packet();
    packet.identity_bars[0].canonical_source_disclosure = String::new();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewControlsViolation::CanonicalSourceDisclosureMissing));
}

#[test]
fn wrong_identity_component_class_fails() {
    let mut packet = packet();
    packet.identity_bars[0].component = M5ArtifactComponent::DiffModeSwitcher;
    assert!(packet
        .validate()
        .contains(&ArtifactReviewControlsViolation::IdentityBarWrongComponentClass));
}

#[test]
fn inconsistent_rollback_posture_fails() {
    let mut packet = packet();
    // Compare-only imported SBOM must not carry a write-back-attributable posture.
    packet.identity_bars[2].rollback_posture =
        M5ArtifactComponentRollbackPosture::WriteBackAttributable;
    assert!(packet
        .validate()
        .contains(&ArtifactReviewControlsViolation::RollbackPostureInconsistent));
}

#[test]
fn missing_origin_coverage_fails() {
    let mut packet = packet();
    packet
        .identity_bars
        .retain(|bar| bar.origin_class != ArtifactOriginClass::ImportedExternal);
    packet
        .diff_switchers
        .retain(|switcher| switcher.artifact_ref != "artifact:sbom/spdx.json");
    assert!(packet
        .validate()
        .contains(&ArtifactReviewControlsViolation::ArtifactOriginCoverageMissing));
}

#[test]
fn unavailable_lens_without_reason_fails() {
    let mut packet = packet();
    packet.diff_switchers[2].options[1].unavailability_reason = String::new();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewControlsViolation::LensUnavailabilityReasonMissing));
}

#[test]
fn missing_raw_fallback_lens_fails() {
    let mut packet = packet();
    packet.diff_switchers[0]
        .options
        .retain(|option| !option.lens.is_raw_fallback());
    assert!(packet
        .validate()
        .contains(&ArtifactReviewControlsViolation::RawFallbackLensMissing));
}

#[test]
fn active_lens_unavailable_fails() {
    let mut packet = packet();
    // Make the active structured lens unavailable without changing active_lens.
    packet.diff_switchers[0].options[0].availability =
        DiffLensAvailability::UnavailableParserMissing;
    packet.diff_switchers[0].options[0].unavailability_reason = "parser missing".to_owned();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewControlsViolation::ActiveLensUnavailable));
}

#[test]
fn missing_compare_write_back_safety_fails() {
    let mut packet = packet();
    packet.diff_switchers[0].compare_write_back_safety = String::new();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewControlsViolation::CompareWriteBackSafetyMissing));
}

#[test]
fn unpaired_artifact_fails() {
    let mut packet = packet();
    packet.diff_switchers.pop();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewControlsViolation::ArtifactPairingIncomplete));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewControlsViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.canonical_source_never_buried = false;
    assert!(packet
        .validate()
        .contains(&ArtifactReviewControlsViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_projection.raw_fallback_reachable = false;
    assert!(packet
        .validate()
        .contains(&ArtifactReviewControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&ArtifactReviewControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_controls() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Identity bars"));
    assert!(summary.contains("## Diff-mode switchers"));
    assert!(summary.contains("generated API client"));
    assert!(summary.contains("structured_semantic"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_artifact_review_controls_export()
        .expect("checked artifact review controls export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-artifact-identity-diff-mode-controls/schema_unrecognized_raw_fallback.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-artifact-identity-diff-mode-controls/generated_regenerate_only.json"
        )),
    ] {
        let packet: ArtifactReviewControlsPacket =
            serde_json::from_str(raw).expect("fixture parses as artifact review controls packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

/// Regenerates the checked release artifacts and narrowed fixtures.
///
/// Guarded behind `GEN_ARTIFACT_IDENTITY_DIFF_MODE_ARTIFACTS` so ordinary test
/// runs never touch the working tree.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_ARTIFACT_IDENTITY_DIFF_MODE_ARTIFACTS").is_err() {
        return;
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_root = std::path::Path::new(manifest_dir).join("..").join("..");

    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let proof_dir = repo_root
        .join("artifacts")
        .join("release")
        .join("m5-artifact-identity-diff-mode-controls-proof");
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
        .join("m5-artifact-identity-diff-mode-controls");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");

    // Fixture 1: an authored artifact whose schema is unrecognized, so it drops to
    // a compare-only raw fallback instead of pretending to be writable.
    let mut unrecognized = packet.clone();
    unrecognized.packet_id =
        "artifact-identity-diff-mode-controls:fixture:schema-unrecognized".to_owned();
    if let Some(bar) = unrecognized
        .identity_bars
        .iter_mut()
        .find(|bar| bar.artifact_ref == "artifact:notebooks/analysis.ipynb")
    {
        bar.parser_schema_state = M5ArtifactFidelityState::SchemaUnrecognized;
        bar.claims_writable_target = false;
        bar.raw_fallback_note =
            "No schema recognizes this artifact; showing the raw/export-safe fallback".to_owned();
        bar.rollback_posture = M5ArtifactComponentRollbackPosture::CompareOnlyNoWriteBack;
    }
    if let Some(switcher) = unrecognized
        .diff_switchers
        .iter_mut()
        .find(|switcher| switcher.artifact_ref == "artifact:notebooks/analysis.ipynb")
    {
        switcher.options = vec![
            DiffModeOption {
                lens: DiffReviewLens::StructuredSemantic,
                availability: DiffLensAvailability::UnavailableSchemaUnrecognized,
                unavailability_reason: "No schema recognizes this artifact".to_owned(),
            },
            DiffModeOption {
                lens: DiffReviewLens::RawTextFallback,
                availability: DiffLensAvailability::Available,
                unavailability_reason: String::new(),
            },
        ];
        switcher.active_lens = DiffReviewLens::RawTextFallback;
        switcher.compare_write_back_safety =
            "Compare-only: schema unrecognized, raw fallback is read-only".to_owned();
        switcher.rollback_posture = M5ArtifactComponentRollbackPosture::CompareOnlyNoWriteBack;
    }
    assert!(
        unrecognized.validate().is_empty(),
        "{:?}",
        unrecognized.validate()
    );
    std::fs::write(
        fixture_dir.join("schema_unrecognized_raw_fallback.json"),
        format!("{}\n", unrecognized.export_safe_json()),
    )
    .expect("write schema-unrecognized fixture");

    // Fixture 2: keep only the generated artifact to spotlight regenerate-only truth.
    let mut generated = packet.clone();
    generated.packet_id =
        "artifact-identity-diff-mode-controls:fixture:generated-regenerate-only".to_owned();
    generated.identity_bars = packet
        .identity_bars
        .iter()
        .filter(|bar| {
            matches!(
                bar.origin_class,
                ArtifactOriginClass::AuthoredInRepo
                    | ArtifactOriginClass::GeneratedFromSource
                    | ArtifactOriginClass::ImportedExternal
            )
        })
        .cloned()
        .collect();
    let kept: BTreeSet<&str> = generated
        .identity_bars
        .iter()
        .map(|bar| bar.artifact_ref.as_str())
        .collect();
    generated.diff_switchers = packet
        .diff_switchers
        .iter()
        .filter(|switcher| kept.contains(switcher.artifact_ref.as_str()))
        .cloned()
        .collect();
    assert!(
        generated.validate().is_empty(),
        "{:?}",
        generated.validate()
    );
    std::fs::write(
        fixture_dir.join("generated_regenerate_only.json"),
        format!("{}\n", generated.export_safe_json()),
    )
    .expect("write generated fixture");
}
