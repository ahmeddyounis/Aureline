use super::*;

const PACKET_ID: &str = "artifact-review-component-consumer:stable:0001";

fn trust_review() -> ArtifactReviewComponentConsumerTrustReview {
    ArtifactReviewComponentConsumerTrustReview {
        component_reuse_proven_by_fixtures: true,
        same_object_same_language_across_surfaces: true,
        compare_only_never_silently_writable: true,
        structured_mode_never_flattened_without_explanation: true,
        generated_from_relation_never_hidden: true,
        canonical_source_labels_identical_across_surfaces: true,
        risk_status_language_identical_across_surfaces: true,
        raw_export_safe_fallback_kept_explicit: true,
        redaction_posture_kept_explicit: true,
        help_support_export_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> ArtifactReviewComponentConsumerProjection {
    ArtifactReviewComponentConsumerProjection {
        diff_toolbar_reuses_shared_components: true,
        merge_sheet_reuses_shared_components: true,
        review_workspace_reuses_shared_components: true,
        help_surface_reuses_shared_components: true,
        support_packet_reuses_shared_components: true,
        exported_view_reuses_shared_components: true,
        every_component_adopted_by_two_or_more_consumers: true,
        parity_facets_identical_for_same_object: true,
        narrowing_disclosed_not_hidden: true,
        export_preserves_canonical_source_and_compare_only_posture: true,
    }
}

fn proof_freshness() -> ArtifactReviewComponentConsumerProofFreshness {
    ArtifactReviewComponentConsumerProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-07-08T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<ArtifactReviewComponentConsumerDowngradeTrigger> {
    vec![
        ArtifactReviewComponentConsumerDowngradeTrigger::ProofStale,
        ArtifactReviewComponentConsumerDowngradeTrigger::SchemaUnrecognized,
        ArtifactReviewComponentConsumerDowngradeTrigger::RenderUntrusted,
        ArtifactReviewComponentConsumerDowngradeTrigger::RedactionApplied,
        ArtifactReviewComponentConsumerDowngradeTrigger::ParityDriftDetected,
        ArtifactReviewComponentConsumerDowngradeTrigger::UpstreamComponentNarrowed,
    ]
}

fn consumer_surfaces() -> Vec<ArtifactReviewComponentConsumer> {
    ArtifactReviewComponentConsumer::ALL.to_vec()
}

fn source_contract_refs() -> Vec<String> {
    vec![
        ARTIFACT_REVIEW_COMPONENT_CONSUMER_SCHEMA_REF.to_owned(),
        ARTIFACT_REVIEW_COMPONENT_CONSUMER_DOC_REF.to_owned(),
        ARTIFACT_REVIEW_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        ARTIFACT_REVIEW_COMPONENT_CONSUMER_IDENTITY_DIFF_CONTROLS_CONTRACT_REF.to_owned(),
        ARTIFACT_REVIEW_COMPONENT_CONSUMER_STRUCTURE_COMPARE_CONTROLS_CONTRACT_REF.to_owned(),
        ARTIFACT_REVIEW_COMPONENT_CONSUMER_MERGE_GENERATED_CONTROLS_CONTRACT_REF.to_owned(),
        ARTIFACT_REVIEW_COMPONENT_CONSUMER_MEDIA_TRUST_CONTROLS_CONTRACT_REF.to_owned(),
    ]
}

fn binding_refs(component: M5ArtifactComponent) -> Vec<String> {
    vec![
        ARTIFACT_REVIEW_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        component_canonical_schema_ref(component).to_owned(),
    ]
}

/// Builds one binding, deriving render mode, parity state, narrow banner, and
/// disclosure notes from the object's render fidelity so the fixture stays
/// self-consistent by construction.
#[allow(clippy::too_many_arguments)]
fn binding(
    binding_id: &str,
    object_id: &str,
    object_label: &str,
    component: M5ArtifactComponent,
    consumer: ArtifactReviewComponentConsumer,
    fidelity: M5ArtifactFidelityState,
    facets: &ArtifactReviewComponentParityFacetValues,
) -> ArtifactReviewComponentConsumerBinding {
    let disclosure = resolve_artifact_component_render_disclosure(fidelity);

    let narrow_banner = disclosure.narrow_reason.map(|reason| {
        let (next_action, next_action_label) = match reason {
            ArtifactReviewComponentNarrowReason::StructuredFidelityDegraded => (
                ArtifactReviewComponentNarrowNextAction::OpenRawExportSafeFallback,
                "Open the raw / export-safe view to inspect unmapped structure".to_owned(),
            ),
            ArtifactReviewComponentNarrowReason::StructuredModeUnavailableRawFallback => (
                ArtifactReviewComponentNarrowNextAction::ReviewParserSchemaState,
                "No parser recognizes this artifact; review the raw / export-safe fallback"
                    .to_owned(),
            ),
            ArtifactReviewComponentNarrowReason::ContentRedactedOrWithheld => (
                ArtifactReviewComponentNarrowNextAction::ReviewRedactionPosture,
                "Content is withheld here; review the redaction / export posture".to_owned(),
            ),
        };
        ArtifactReviewComponentNarrowBanner {
            reason,
            preserved_facets_note:
                "Canonical source, mode/action, risk language, and provenance are preserved; only fidelity narrowed"
                    .to_owned(),
            next_action,
            next_action_label,
        }
    });

    let raw_fallback_note = if disclosure.needs_raw_fallback_note {
        "A raw / export-safe fallback is shown here because structured fidelity narrowed".to_owned()
    } else {
        String::new()
    };
    let redaction_note = if disclosure.needs_redaction_note {
        "Some content is redacted or withheld under the export/redaction posture".to_owned()
    } else {
        String::new()
    };

    ArtifactReviewComponentConsumerBinding {
        binding_id: binding_id.to_owned(),
        artifact_object_id: object_id.to_owned(),
        artifact_object_label: object_label.to_owned(),
        component,
        consumer,
        render_fidelity: fidelity,
        render_mode: disclosure.expected_mode,
        parity_facets: facets.clone(),
        parity_state: parity_state_for_mode(disclosure.expected_mode),
        narrow_banner,
        raw_fallback_note,
        redaction_note,
        promotes_compare_only_to_writable_state: false,
        flattens_structured_mode_without_explanation: false,
        hides_generated_from_relation_behind_generic_chrome: false,
        drops_raw_or_export_safe_fallback: false,
        rewords_artifact_labels_per_surface: false,
        source_contract_refs: binding_refs(component),
    }
}

fn facets(
    canonical_source: &str,
    mode_action: &str,
    risk: &str,
    provenance: &str,
) -> ArtifactReviewComponentParityFacetValues {
    ArtifactReviewComponentParityFacetValues {
        canonical_source_label: canonical_source.to_owned(),
        mode_action: mode_action.to_owned(),
        risk_status_language: risk.to_owned(),
        provenance_relation: provenance.to_owned(),
    }
}

/// The canonical binding set: nine components, each adopted by >= 2 consumers,
/// covering all six consumer surfaces and every render-fidelity state. Objects
/// sharing an id share parity facets.
fn consumer_bindings() -> Vec<ArtifactReviewComponentConsumerBinding> {
    // Object 1: artifact identity bar, faithful, on review workspace + diff toolbar.
    let ib = facets(
        "config/app.toml · authored artifact",
        "Structured diff",
        "2 keys changed · compare-only",
        "authored · no generated-from relation",
    );
    // Object 2: diff-mode switcher, partial structure, on diff toolbar + exported view.
    let dm = facets(
        "notebook.ipynb · imported artifact",
        "Cell-aware diff (2 modes)",
        "1 cell changed · structured partial",
        "imported · source-of-truth is the checked-in file",
    );
    // Object 3: structure row, schema unrecognized, on diff toolbar + review workspace.
    let sr = facets(
        "vendor.bin · unrecognized schema",
        "Raw / export-safe view",
        "Bytes differ · no parser available",
        "authored · no generated-from relation",
    );
    // Object 4: merge-decision row, faithful, on merge sheet + review workspace.
    let md = facets(
        "manifest.json · authored artifact",
        "Take incoming (base/ours/theirs)",
        "1 conflict · compare-only until accepted",
        "authored · preserve unknown fields",
    );
    // Object 5: generated-artifact notice, render untrusted, on merge sheet + support packet.
    let ga = facets(
        "Cargo.lock · generated artifact",
        "Regenerate from source",
        "Lockfile diverged · regenerate first",
        "generated-from Cargo.toml · source-of-truth is the manifest",
    );
    // Object 6: rendered compare viewer, render untrusted, on diff toolbar + help surface.
    let rc = facets(
        "design-snapshot.png · imported artifact",
        "Rendered compare (untrusted)",
        "Pixels differ · render not fully trusted",
        "imported · source-of-truth is the exported snapshot",
    );
    // Object 7: media-metadata rail, raw fallback, on diff toolbar + exported view.
    let mr = facets(
        "capture.webp · imported artifact",
        "Metadata rail + raw fallback",
        "Dimensions changed · raw / export-safe view",
        "imported · source-of-truth is the captured file",
    );
    // Object 8: redaction/trust badge set, redacted, on support packet + exported view.
    let rb = facets(
        "crash-adjunct.json · policy artifact",
        "Trust / redaction badges",
        "Content withheld · redacted for export",
        "policy · source-of-truth stays in-product",
    );
    // Object 9: compare-summary card, faithful, on review workspace + support packet.
    let cs = facets(
        "sbom.spdx.json · authored artifact",
        "Compare summary (no flatten)",
        "3 added · 1 removed · compare-only",
        "authored · no generated-from relation",
    );

    vec![
        binding(
            "bind:ib-1:workspace",
            "obj:ib-1",
            "config/app.toml",
            M5ArtifactComponent::ArtifactIdentityBar,
            ArtifactReviewComponentConsumer::ReviewWorkspace,
            M5ArtifactFidelityState::StructuredFaithful,
            &ib,
        ),
        binding(
            "bind:ib-1:diff",
            "obj:ib-1",
            "config/app.toml",
            M5ArtifactComponent::ArtifactIdentityBar,
            ArtifactReviewComponentConsumer::DiffToolbar,
            M5ArtifactFidelityState::StructuredFaithful,
            &ib,
        ),
        binding(
            "bind:dm-2:diff",
            "obj:dm-2",
            "notebook.ipynb",
            M5ArtifactComponent::DiffModeSwitcher,
            ArtifactReviewComponentConsumer::DiffToolbar,
            M5ArtifactFidelityState::StructuredPartial,
            &dm,
        ),
        binding(
            "bind:dm-2:export",
            "obj:dm-2",
            "notebook.ipynb",
            M5ArtifactComponent::DiffModeSwitcher,
            ArtifactReviewComponentConsumer::ExportedView,
            M5ArtifactFidelityState::StructuredPartial,
            &dm,
        ),
        binding(
            "bind:sr-3:diff",
            "obj:sr-3",
            "vendor.bin",
            M5ArtifactComponent::StructureRow,
            ArtifactReviewComponentConsumer::DiffToolbar,
            M5ArtifactFidelityState::SchemaUnrecognized,
            &sr,
        ),
        binding(
            "bind:sr-3:workspace",
            "obj:sr-3",
            "vendor.bin",
            M5ArtifactComponent::StructureRow,
            ArtifactReviewComponentConsumer::ReviewWorkspace,
            M5ArtifactFidelityState::SchemaUnrecognized,
            &sr,
        ),
        binding(
            "bind:md-4:merge",
            "obj:md-4",
            "manifest.json",
            M5ArtifactComponent::MergeDecisionRow,
            ArtifactReviewComponentConsumer::MergeSheet,
            M5ArtifactFidelityState::StructuredFaithful,
            &md,
        ),
        binding(
            "bind:md-4:workspace",
            "obj:md-4",
            "manifest.json",
            M5ArtifactComponent::MergeDecisionRow,
            ArtifactReviewComponentConsumer::ReviewWorkspace,
            M5ArtifactFidelityState::StructuredFaithful,
            &md,
        ),
        binding(
            "bind:ga-5:merge",
            "obj:ga-5",
            "Cargo.lock",
            M5ArtifactComponent::GeneratedArtifactNotice,
            ArtifactReviewComponentConsumer::MergeSheet,
            M5ArtifactFidelityState::RenderUntrusted,
            &ga,
        ),
        binding(
            "bind:ga-5:support",
            "obj:ga-5",
            "Cargo.lock",
            M5ArtifactComponent::GeneratedArtifactNotice,
            ArtifactReviewComponentConsumer::SupportPacket,
            M5ArtifactFidelityState::RenderUntrusted,
            &ga,
        ),
        binding(
            "bind:rc-6:diff",
            "obj:rc-6",
            "design-snapshot.png",
            M5ArtifactComponent::RenderedCompareViewer,
            ArtifactReviewComponentConsumer::DiffToolbar,
            M5ArtifactFidelityState::RenderUntrusted,
            &rc,
        ),
        binding(
            "bind:rc-6:help",
            "obj:rc-6",
            "design-snapshot.png",
            M5ArtifactComponent::RenderedCompareViewer,
            ArtifactReviewComponentConsumer::HelpSurface,
            M5ArtifactFidelityState::RenderUntrusted,
            &rc,
        ),
        binding(
            "bind:mr-7:diff",
            "obj:mr-7",
            "capture.webp",
            M5ArtifactComponent::MediaMetadataRail,
            ArtifactReviewComponentConsumer::DiffToolbar,
            M5ArtifactFidelityState::RawFallback,
            &mr,
        ),
        binding(
            "bind:mr-7:export",
            "obj:mr-7",
            "capture.webp",
            M5ArtifactComponent::MediaMetadataRail,
            ArtifactReviewComponentConsumer::ExportedView,
            M5ArtifactFidelityState::RawFallback,
            &mr,
        ),
        binding(
            "bind:rb-8:support",
            "obj:rb-8",
            "crash-adjunct.json",
            M5ArtifactComponent::RedactionOrTrustBadgeSet,
            ArtifactReviewComponentConsumer::SupportPacket,
            M5ArtifactFidelityState::RedactedOrWithheld,
            &rb,
        ),
        binding(
            "bind:rb-8:export",
            "obj:rb-8",
            "crash-adjunct.json",
            M5ArtifactComponent::RedactionOrTrustBadgeSet,
            ArtifactReviewComponentConsumer::ExportedView,
            M5ArtifactFidelityState::RedactedOrWithheld,
            &rb,
        ),
        binding(
            "bind:cs-9:workspace",
            "obj:cs-9",
            "sbom.spdx.json",
            M5ArtifactComponent::CompareSummaryCard,
            ArtifactReviewComponentConsumer::ReviewWorkspace,
            M5ArtifactFidelityState::StructuredFaithful,
            &cs,
        ),
        binding(
            "bind:cs-9:support",
            "obj:cs-9",
            "sbom.spdx.json",
            M5ArtifactComponent::CompareSummaryCard,
            ArtifactReviewComponentConsumer::SupportPacket,
            M5ArtifactFidelityState::StructuredFaithful,
            &cs,
        ),
    ]
}

fn packet_with(
    bindings: Vec<ArtifactReviewComponentConsumerBinding>,
) -> ArtifactReviewComponentConsumerPacket {
    ArtifactReviewComponentConsumerPacket::new(ArtifactReviewComponentConsumerPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Shared artifact-review-component consumers".to_owned(),
        consumer_bindings: bindings,
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

fn packet() -> ArtifactReviewComponentConsumerPacket {
    packet_with(consumer_bindings())
}

#[test]
fn consumer_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn render_disclosure_maps_fidelity_to_mode() {
    let faithful =
        resolve_artifact_component_render_disclosure(M5ArtifactFidelityState::StructuredFaithful);
    assert_eq!(
        faithful.expected_mode,
        ArtifactReviewComponentRenderMode::FullParity
    );
    assert!(!faithful.needs_narrow_banner);
    assert!(!faithful.needs_raw_fallback_note);
    assert!(!faithful.needs_redaction_note);

    let partial =
        resolve_artifact_component_render_disclosure(M5ArtifactFidelityState::StructuredPartial);
    assert_eq!(
        partial.expected_mode,
        ArtifactReviewComponentRenderMode::StructuredFidelityNarrowed
    );
    assert!(partial.needs_narrow_banner);
    assert!(partial.needs_raw_fallback_note);
    assert!(!partial.needs_redaction_note);

    let untrusted =
        resolve_artifact_component_render_disclosure(M5ArtifactFidelityState::RenderUntrusted);
    assert_eq!(
        untrusted.expected_mode,
        ArtifactReviewComponentRenderMode::StructuredFidelityNarrowed
    );
    assert!(untrusted.needs_raw_fallback_note);

    let unrecognized =
        resolve_artifact_component_render_disclosure(M5ArtifactFidelityState::SchemaUnrecognized);
    assert_eq!(
        unrecognized.expected_mode,
        ArtifactReviewComponentRenderMode::RawFallbackDisclosed
    );
    assert!(unrecognized.needs_raw_fallback_note);
    assert!(!unrecognized.needs_redaction_note);

    let raw = resolve_artifact_component_render_disclosure(M5ArtifactFidelityState::RawFallback);
    assert_eq!(
        raw.expected_mode,
        ArtifactReviewComponentRenderMode::RawFallbackDisclosed
    );
    assert!(raw.needs_raw_fallback_note);

    let redacted =
        resolve_artifact_component_render_disclosure(M5ArtifactFidelityState::RedactedOrWithheld);
    assert_eq!(
        redacted.expected_mode,
        ArtifactReviewComponentRenderMode::RedactionNarrowed
    );
    assert!(redacted.needs_redaction_note);
    assert!(!redacted.needs_raw_fallback_note);
}

#[test]
fn parity_drift_across_surfaces_fails() {
    let mut packet = packet();
    // Reword the canonical-source label on one surface for a shared object.
    packet.consumer_bindings[1]
        .parity_facets
        .canonical_source_label = "Reworded label for the diff toolbar".to_owned();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewComponentConsumerViolation::ParityDriftAcrossSurfaces));
}

#[test]
fn mode_action_drift_across_surfaces_fails() {
    let mut packet = packet();
    packet.consumer_bindings[7].parity_facets.mode_action = "Different action".to_owned();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewComponentConsumerViolation::ParityDriftAcrossSurfaces));
}

#[test]
fn component_reuse_by_single_consumer_fails() {
    let mut bindings = consumer_bindings();
    // Drop the second compare-summary-card binding so it is adopted by one consumer.
    bindings.retain(|b| b.binding_id != "bind:cs-9:support");
    let packet = packet_with(bindings);
    assert!(packet
        .validate()
        .contains(&ArtifactReviewComponentConsumerViolation::ArtifactComponentReuseUnproven));
}

#[test]
fn missing_component_coverage_fails() {
    let mut bindings = consumer_bindings();
    bindings.retain(|b| b.component != M5ArtifactComponent::CompareSummaryCard);
    let packet = packet_with(bindings);
    assert!(packet
        .validate()
        .contains(&ArtifactReviewComponentConsumerViolation::ComponentCoverageMissing));
}

#[test]
fn missing_consumer_coverage_fails() {
    let mut bindings = consumer_bindings();
    // Remove the only Help-surface binding.
    bindings.retain(|b| b.consumer != ArtifactReviewComponentConsumer::HelpSurface);
    let packet = packet_with(bindings);
    assert!(packet
        .validate()
        .contains(&ArtifactReviewComponentConsumerViolation::ConsumerCoverageMissing));
}

#[test]
fn help_support_export_without_canonical_refs_fails() {
    let mut packet = packet();
    // A support-packet binding drops its canonical component ref.
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.consumer == ArtifactReviewComponentConsumer::SupportPacket)
        .expect("support-packet binding present");
    packet.consumer_bindings[index].source_contract_refs =
        vec![ARTIFACT_REVIEW_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF.to_owned()];
    assert!(packet
        .validate()
        .contains(&ArtifactReviewComponentConsumerViolation::HelpSupportExportReferenceMissing));
}

#[test]
fn render_mode_mismatch_fails() {
    let mut packet = packet();
    // Claim full parity on a schema-unrecognized structure row.
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.render_fidelity == M5ArtifactFidelityState::SchemaUnrecognized)
        .expect("schema-unrecognized binding present");
    packet.consumer_bindings[index].render_mode = ArtifactReviewComponentRenderMode::FullParity;
    assert!(packet
        .validate()
        .contains(&ArtifactReviewComponentConsumerViolation::RenderModeMismatch));
}

#[test]
fn parity_state_mismatch_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].parity_state =
        ArtifactReviewComponentParityState::FacetsDisclosedNarrowed;
    assert!(packet
        .validate()
        .contains(&ArtifactReviewComponentConsumerViolation::ParityStateMismatch));
}

#[test]
fn narrowed_binding_without_banner_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.is_narrowed())
        .expect("narrowed binding present");
    packet.consumer_bindings[index].narrow_banner = None;
    assert!(packet
        .validate()
        .contains(&ArtifactReviewComponentConsumerViolation::NarrowBannerMissing));
}

#[test]
fn full_parity_binding_with_banner_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].narrow_banner = Some(ArtifactReviewComponentNarrowBanner {
        reason: ArtifactReviewComponentNarrowReason::StructuredFidelityDegraded,
        preserved_facets_note: "note".to_owned(),
        next_action: ArtifactReviewComponentNarrowNextAction::OpenRawExportSafeFallback,
        next_action_label: "Open raw".to_owned(),
    });
    assert!(packet
        .validate()
        .contains(&ArtifactReviewComponentConsumerViolation::NarrowBannerMissing));
}

#[test]
fn narrow_reason_mismatch_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.render_fidelity == M5ArtifactFidelityState::SchemaUnrecognized)
        .expect("schema-unrecognized binding present");
    if let Some(banner) = packet.consumer_bindings[index].narrow_banner.as_mut() {
        banner.reason = ArtifactReviewComponentNarrowReason::ContentRedactedOrWithheld;
    }
    assert!(packet
        .validate()
        .contains(&ArtifactReviewComponentConsumerViolation::NarrowReasonMismatch));
}

#[test]
fn narrow_banner_missing_preserved_facets_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.is_narrowed())
        .expect("narrowed binding present");
    if let Some(banner) = packet.consumer_bindings[index].narrow_banner.as_mut() {
        banner.preserved_facets_note = String::new();
    }
    assert!(packet
        .validate()
        .contains(&ArtifactReviewComponentConsumerViolation::NarrowBannerPreservedFacetsMissing));
}

#[test]
fn raw_fallback_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.render_fidelity == M5ArtifactFidelityState::RawFallback)
        .expect("raw-fallback binding present");
    packet.consumer_bindings[index].raw_fallback_note = String::new();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewComponentConsumerViolation::RawFallbackNoteMissing));
}

#[test]
fn redaction_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.render_fidelity == M5ArtifactFidelityState::RedactedOrWithheld)
        .expect("redacted binding present");
    packet.consumer_bindings[index].redaction_note = String::new();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewComponentConsumerViolation::RedactionNoteMissing));
}

#[test]
fn compare_only_promoted_to_writable_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].promotes_compare_only_to_writable_state = true;
    assert!(packet
        .validate()
        .contains(&ArtifactReviewComponentConsumerViolation::CompareOnlyPromotedToWritable));
}

#[test]
fn structured_mode_flattened_without_explanation_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].flattens_structured_mode_without_explanation = true;
    assert!(packet.validate().contains(
        &ArtifactReviewComponentConsumerViolation::StructuredModeFlattenedWithoutExplanation
    ));
}

#[test]
fn generated_from_relation_hidden_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].hides_generated_from_relation_behind_generic_chrome = true;
    assert!(packet.validate().contains(
        &ArtifactReviewComponentConsumerViolation::GeneratedFromRelationHiddenBehindGenericChrome
    ));
}

#[test]
fn raw_or_export_safe_fallback_dropped_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].drops_raw_or_export_safe_fallback = true;
    assert!(packet
        .validate()
        .contains(&ArtifactReviewComponentConsumerViolation::RawOrExportSafeFallbackDropped));
}

#[test]
fn artifact_labels_reworded_per_surface_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].rewords_artifact_labels_per_surface = true;
    assert!(packet
        .validate()
        .contains(&ArtifactReviewComponentConsumerViolation::ArtifactLabelsRewordedPerSurface));
}

#[test]
fn parity_facet_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0]
        .parity_facets
        .provenance_relation = String::new();
    // Rewording one surface also trips drift; assert the incomplete facet is reported.
    assert!(packet
        .validate()
        .contains(&ArtifactReviewComponentConsumerViolation::ParityFacetIncomplete));
}

#[test]
fn incomplete_binding_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].artifact_object_label = String::new();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewComponentConsumerViolation::BindingIncomplete));
}

#[test]
fn missing_bindings_fails() {
    let mut packet = packet();
    packet.consumer_bindings.clear();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewComponentConsumerViolation::ConsumerBindingsMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&ArtifactReviewComponentConsumerViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.compare_only_never_silently_writable = false;
    assert!(packet
        .validate()
        .contains(&ArtifactReviewComponentConsumerViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .every_component_adopted_by_two_or_more_consumers = false;
    assert!(packet
        .validate()
        .contains(&ArtifactReviewComponentConsumerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&ArtifactReviewComponentConsumerViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_bindings() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Consumer bindings"));
    assert!(summary.contains("artifact_identity_bar"));
    assert!(summary.contains("redaction_or_trust_badge_set"));
    assert!(summary.contains("raw_fallback_disclosed"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_artifact_review_component_consumer_export()
        .expect("checked artifact-review consumer export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-structured-artifact-review-component-consumers/structured_fidelity_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-structured-artifact-review-component-consumers/raw_fallback_and_redaction.json"
        )),
    ] {
        let packet: ArtifactReviewComponentConsumerPacket = serde_json::from_str(raw)
            .expect("fixture parses as artifact-review consumer packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

// --- Fixture builders ---------------------------------------------------------

/// Re-derives the canonical bindings after overriding some objects' fidelity,
/// keeping the parity facets identical per object so the packet still validates.
fn bindings_with_fidelity_overrides(
    overrides: &[(&str, M5ArtifactFidelityState)],
) -> Vec<ArtifactReviewComponentConsumerBinding> {
    consumer_bindings()
        .into_iter()
        .map(|existing| {
            if let Some((_, fidelity)) = overrides
                .iter()
                .find(|(object_id, _)| *object_id == existing.artifact_object_id)
            {
                binding(
                    &existing.binding_id,
                    &existing.artifact_object_id,
                    &existing.artifact_object_label,
                    existing.component,
                    existing.consumer,
                    *fidelity,
                    &existing.parity_facets,
                )
            } else {
                existing
            }
        })
        .collect()
}

fn fixture_structured_fidelity_narrowed() -> ArtifactReviewComponentConsumerPacket {
    let bindings = bindings_with_fidelity_overrides(&[
        ("obj:ib-1", M5ArtifactFidelityState::StructuredPartial),
        ("obj:md-4", M5ArtifactFidelityState::RenderUntrusted),
    ]);
    ArtifactReviewComponentConsumerPacket::new(ArtifactReviewComponentConsumerPacketInput {
        packet_id: "artifact-review-component-consumer:fixture:structured-fidelity-narrowed"
            .to_owned(),
        surface_label: "Shared artifact-review-component consumers: structured fidelity narrowed"
            .to_owned(),
        consumer_bindings: bindings,
        downgrade_triggers: vec![
            ArtifactReviewComponentConsumerDowngradeTrigger::RenderUntrusted,
            ArtifactReviewComponentConsumerDowngradeTrigger::UpstreamComponentNarrowed,
        ],
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-08T00:00:00Z".to_owned(),
    })
}

fn fixture_raw_fallback_and_redaction() -> ArtifactReviewComponentConsumerPacket {
    let bindings = bindings_with_fidelity_overrides(&[
        ("obj:cs-9", M5ArtifactFidelityState::SchemaUnrecognized),
        ("obj:md-4", M5ArtifactFidelityState::RedactedOrWithheld),
    ]);
    ArtifactReviewComponentConsumerPacket::new(ArtifactReviewComponentConsumerPacketInput {
        packet_id: "artifact-review-component-consumer:fixture:raw-fallback-and-redaction"
            .to_owned(),
        surface_label: "Shared artifact-review-component consumers: raw fallback and redaction"
            .to_owned(),
        consumer_bindings: bindings,
        downgrade_triggers: vec![
            ArtifactReviewComponentConsumerDowngradeTrigger::SchemaUnrecognized,
            ArtifactReviewComponentConsumerDowngradeTrigger::RedactionApplied,
        ],
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-08T00:00:00Z".to_owned(),
    })
}

/// Regenerates the checked-in support export, summary, and fixtures.
///
/// Gated behind `GEN_ARTIFACT_REVIEW_CONSUMER_ARTIFACTS` so it never writes during
/// a normal test run. Run with the env var set to refresh the artifacts after a
/// contract change, then review the diff.
#[test]
fn regenerate_artifact_review_consumer_artifacts() {
    if std::env::var("GEN_ARTIFACT_REVIEW_CONSUMER_ARTIFACTS").is_err() {
        return;
    }

    let manifest = env!("CARGO_MANIFEST_DIR");
    let root = format!("{manifest}/../..");

    let canonical = packet();
    assert!(
        canonical.validate().is_empty(),
        "{:?}",
        canonical.validate()
    );

    let artifact_dir =
        format!("{root}/artifacts/release/m5-structured-artifact-review-consumers-proof");
    std::fs::create_dir_all(&artifact_dir).expect("create artifact dir");
    std::fs::write(
        format!("{artifact_dir}/support_export.json"),
        format!("{}\n", canonical.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(
        format!("{artifact_dir}/summary.md"),
        canonical.render_markdown_summary(),
    )
    .expect("write summary");

    let fixture_dir =
        format!("{root}/fixtures/ui/m5-structured-artifact-review-component-consumers");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");
    for (name, fixture) in [
        (
            "structured_fidelity_narrowed.json",
            fixture_structured_fidelity_narrowed(),
        ),
        (
            "raw_fallback_and_redaction.json",
            fixture_raw_fallback_and_redaction(),
        ),
    ] {
        assert!(
            fixture.validate().is_empty(),
            "{name}: {:?}",
            fixture.validate()
        );
        std::fs::write(
            format!("{fixture_dir}/{name}"),
            format!("{}\n", fixture.export_safe_json()),
        )
        .expect("write fixture");
    }
}
