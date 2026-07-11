//! Tests for the M05-1089 notebook-component accessibility parity capstone: the honest
//! auto-narrowing logic, the per-family parity contract, the large-output virtualization /
//! collapsed-output honesty, the stale/partial/degraded/severed-never-live guarantee, no-loss
//! notebook-truth integrity, and the checked-in support export / CSV / report.

use super::*;

fn row(id: &str) -> NotebookComponentAccessibilityRow {
    seeded_m5_notebook_kernel_output_component_a11y_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_notebook_kernel_output_component_a11y_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified() {
    let packet = seeded_m5_notebook_kernel_output_component_a11y_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5NotebookKernelOutputComponentFamily::ALL.len()
    );
    // Eight rows cover the eight families one-to-one (one certified fully green — the live document
    // header — the other seven narrowed-yellow: six auto-narrowed claims plus the collapsed-output
    // provenance chip group).
    assert_eq!(packet.rows.len(), 8);
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_notebook_kernel_output_component_a11y_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5NotebookComponentClaimDimension::ALL.len()
    );
}

#[test]
fn every_condition_state_is_exercised() {
    let packet = seeded_m5_notebook_kernel_output_component_a11y_packet();
    let states = packet.exercised_condition_states();
    for state in M5NotebookComponentConditionState::ALL {
        assert!(
            states.contains(&state),
            "condition state {} is not exercised",
            state.as_str()
        );
    }
}

#[test]
fn every_result_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_notebook_kernel_output_component_a11y_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5NotebookComponentClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_notebook_kernel_output_component_a11y_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5NotebookKernelOutputConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_one_green_seven_yellow_zero_red() {
    let packet = seeded_m5_notebook_kernel_output_component_a11y_packet();
    assert_eq!(packet.summary.green_count, 1);
    assert_eq!(packet.summary.yellow_count, 7);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary.row_count, 8);
    assert_eq!(
        packet.summary.family_count,
        M5NotebookKernelOutputComponentFamily::ALL.len()
    );
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn summary_reports_all_truth_and_live_and_virtualization_honesty() {
    let packet = seeded_m5_notebook_kernel_output_component_a11y_packet();
    assert!(packet.summary.all_truth_preserved);
    assert!(packet.summary.all_live_truth_honesty_holds);
    assert!(packet.summary.all_virtualized_outputs_attributable);
}

// --- AC3: partial / debugger-unsupported / degraded / stale / severed / no-kernel narrow ---

#[test]
fn live_document_header_is_live_trusted_and_green() {
    let header = row("a11y:notebook-document-header-live");
    assert_eq!(
        header.full_notebook_claim,
        M5NotebookComponentClaim::LiveTrustedResult
    );
    assert_eq!(
        header.effective_claim(),
        M5NotebookComponentClaim::LiveTrustedResult
    );
    assert!(header.claim_narrow.is_none());
    assert_eq!(
        header.status(),
        NotebookComponentAccessibilityStatus::Parity
    );
    assert!(header.effective_claim().asserts_live_trusted_result());
}

#[test]
fn reviewable_provenance_chip_group_collapses_to_disclosed_yellow() {
    let chips = row("a11y:output-provenance-chip-group-complete");
    // A complete provenance lineage stays a reviewable result with no claim narrow, but its dense
    // chip group is collapsed to a summary — an honest disclosed reduction (yellow), never red.
    assert_eq!(
        chips.effective_claim(),
        M5NotebookComponentClaim::ReviewableResult
    );
    assert!(chips.claim_narrow.is_none());
    assert!(chips.is_reduced());
    assert_eq!(
        chips.status(),
        NotebookComponentAccessibilityStatus::NarrowedDisclosed
    );
    assert!(chips.effective_claim().asserts_trustworthy_result());
    assert!(!chips.effective_claim().asserts_live_trusted_result());
}

#[test]
fn no_kernel_narrows_to_no_kernel_projection() {
    let strip = row("a11y:kernel-state-strip-no-kernel");
    assert_eq!(
        strip.effective_claim(),
        M5NotebookComponentClaim::NoKernelProjection
    );
    let narrow = strip.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5NotebookKernelOutputDowngradeTrigger::ReconnectShownAsFresh
    );
    assert!(strip.claim_is_honest());
    // A kernel-free strip is an honest offline state, not a live overstatement.
    assert!(strip.live_truth_honesty_holds());
}

#[test]
fn partial_parity_narrows_to_partial_kernel_parity_projection() {
    let picker = row("a11y:kernel-picker-row-partial-parity");
    assert_eq!(
        picker.effective_claim(),
        M5NotebookComponentClaim::PartialKernelParityProjection
    );
    assert!(!picker.effective_claim().asserts_live_trusted_result());
    let narrow = picker.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5NotebookKernelOutputDowngradeTrigger::KernelClassCollapsed
    );
    assert_eq!(
        narrow.binding_dimension,
        M5NotebookComponentClaimDimension::KernelParity
    );
    assert!(picker.claim_is_honest());
    // A partial kernel parity cannot be shown as live trusted.
    assert!(picker.live_truth_honesty_holds());
}

#[test]
fn degraded_origin_narrows_to_degraded_origin_projection() {
    let pill = row("a11y:kernel-origin-pill-degraded");
    assert_eq!(
        pill.effective_claim(),
        M5NotebookComponentClaim::DegradedOriginProjection
    );
    let narrow = pill.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5NotebookKernelOutputDowngradeTrigger::KernelOriginUnstated
    );
    assert!(pill.claim_is_honest());
    assert!(pill.live_truth_honesty_holds());
}

#[test]
fn stale_output_narrows_to_stale_output_projection() {
    let banner = row("a11y:output-trust-banner-stale");
    assert_eq!(
        banner.effective_claim(),
        M5NotebookComponentClaim::StaleOutputProjection
    );
    assert!(!banner.effective_claim().asserts_live_trusted_result());
    let narrow = banner.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5NotebookKernelOutputDowngradeTrigger::StaleOutputShownAsLive
    );
    assert!(banner.claim_is_honest());
    // The KEY guardrail: a stale output can never be shown as live.
    assert!(banner.live_truth_honesty_holds());
}

#[test]
fn debugger_unsupported_narrows_to_debugger_unsupported_projection() {
    let card = row("a11y:restart-consequence-card-debugger-unsupported");
    assert_eq!(
        card.effective_claim(),
        M5NotebookComponentClaim::DebuggerUnsupportedProjection
    );
    let narrow = card.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.binding_dimension,
        M5NotebookComponentClaimDimension::RestartConsequenceClarity
    );
    assert!(card.claim_is_honest());
    // An unsupported debugger is an honest capability state, not a live overstatement.
    assert!(card.live_truth_honesty_holds());
}

#[test]
fn severed_environment_narrows_to_unprovenanced_environment_projection() {
    let card = row("a11y:kernel-recovery-card-unprovenanced-environment");
    assert_eq!(
        card.effective_claim(),
        M5NotebookComponentClaim::UnprovenancedEnvironmentProjection
    );
    let narrow = card.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5NotebookKernelOutputDowngradeTrigger::ProvenanceSevered
    );
    assert!(card.claim_is_honest());
    assert!(card.live_truth_honesty_holds());
    // The recovery card never implies a hidden rerun.
    assert!(card
        .copy_export
        .export_fields
        .iter()
        .any(|f| f == "no_rerun_note"));
}

#[test]
fn over_asserting_control_is_rejected() {
    // Force the effective claim above the permitted ceiling: a stale output claiming
    // LiveTrustedResult.
    let mut banner = row("a11y:output-trust-banner-stale");
    banner.claim_narrow = None;
    assert!(!banner.claim_is_honest());
    assert_eq!(
        banner.status(),
        NotebookComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn stale_state_shown_as_live_is_rejected() {
    // A stale-output row whose narrow claims LiveTrustedResult violates live-truth honesty.
    let mut banner = row("a11y:output-trust-banner-stale");
    if let Some(narrow) = banner.claim_narrow.as_mut() {
        narrow.narrowed_to = M5NotebookComponentClaim::LiveTrustedResult;
    }
    assert!(!banner.live_truth_honesty_holds());
    let violations = {
        let mut packet = seeded_m5_notebook_kernel_output_component_a11y_packet();
        let idx = packet
            .rows
            .iter()
            .position(|r| r.row_id == "a11y:output-trust-banner-stale")
            .expect("row present");
        packet.rows[idx] = banner;
        packet.validate()
    };
    assert!(violations.iter().any(|v| matches!(
        v,
        NotebookComponentAccessibilityViolation::StaleStateShownAsLive { .. }
    )));
}

#[test]
fn live_truth_honesty_unproven_when_no_unprovable_row() {
    let mut packet = seeded_m5_notebook_kernel_output_component_a11y_packet();
    // Drop the four partial-parity / degraded-origin / stale-output / severed-provenance rows.
    packet.rows.retain(|r| {
        r.row_id != "a11y:kernel-picker-row-partial-parity"
            && r.row_id != "a11y:kernel-origin-pill-degraded"
            && r.row_id != "a11y:output-trust-banner-stale"
            && r.row_id != "a11y:kernel-recovery-card-unprovenanced-environment"
    });
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        NotebookComponentAccessibilityViolation::LiveTruthHonestyUnproven
    )));
}

#[test]
fn spurious_narrow_on_live_row_is_rejected() {
    let mut header = row("a11y:notebook-document-header-live");
    header.claim_narrow = Some(NotebookComponentClaimAutoNarrow {
        narrowed_to: M5NotebookComponentClaim::NoKernelProjection,
        binding_dimension: M5NotebookComponentClaimDimension::KernelLiveness,
        trigger: M5NotebookKernelOutputDowngradeTrigger::ReconnectShownAsFresh,
        narrowed_label: "spurious narrowing that should not exist here".to_owned(),
        preserves_canonical_identity: true,
        preserves_truth_continuity: true,
    });
    assert!(!header.claim_is_honest());
}

#[test]
fn narrow_must_bind_the_ceiling_imposing_dimension() {
    let mut picker = row("a11y:kernel-picker-row-partial-parity");
    if let Some(narrow) = picker.claim_narrow.as_mut() {
        narrow.binding_dimension = M5NotebookComponentClaimDimension::OutputProvenance;
    }
    assert!(!picker.claim_is_honest());
}

#[test]
fn narrow_trigger_must_match_binding_condition() {
    let mut picker = row("a11y:kernel-picker-row-partial-parity");
    if let Some(narrow) = picker.claim_narrow.as_mut() {
        narrow.trigger = M5NotebookKernelOutputDowngradeTrigger::ProvenanceSevered;
    }
    assert!(!picker.claim_is_honest());
}

#[test]
fn narrowed_label_must_not_be_generic() {
    let mut picker = row("a11y:kernel-picker-row-partial-parity");
    if let Some(narrow) = picker.claim_narrow.as_mut() {
        narrow.narrowed_label = "partial".to_owned();
    }
    assert!(!picker.claim_is_honest());
}

#[test]
fn permitted_ceilings_map_condition_states_one_to_one() {
    use M5NotebookComponentClaim as S;
    use M5NotebookComponentConditionState as C;
    assert_eq!(C::LiveTrusted.permitted_ceiling(), S::LiveTrustedResult);
    assert_eq!(
        C::KernelParityPartial.permitted_ceiling(),
        S::PartialKernelParityProjection
    );
    assert_eq!(
        C::DebuggerUnsupported.permitted_ceiling(),
        S::DebuggerUnsupportedProjection
    );
    assert_eq!(
        C::KernelOriginDegraded.permitted_ceiling(),
        S::DegradedOriginProjection
    );
    assert_eq!(
        C::OutputTrustStale.permitted_ceiling(),
        S::StaleOutputProjection
    );
    assert_eq!(
        C::EnvironmentProvenanceSevered.permitted_ceiling(),
        S::UnprovenancedEnvironmentProjection
    );
    assert_eq!(
        C::KernelUnavailable.permitted_ceiling(),
        S::NoKernelProjection
    );
}

#[test]
fn condition_triggers_map_to_frozen_matrix_vocabulary() {
    use M5NotebookComponentConditionState as C;
    use M5NotebookKernelOutputDowngradeTrigger as T;
    assert_eq!(
        C::KernelParityPartial.default_trigger(),
        T::KernelClassCollapsed
    );
    assert_eq!(C::DebuggerUnsupported.default_trigger(), T::ProofStale);
    assert_eq!(
        C::KernelOriginDegraded.default_trigger(),
        T::KernelOriginUnstated
    );
    assert_eq!(
        C::OutputTrustStale.default_trigger(),
        T::StaleOutputShownAsLive
    );
    assert_eq!(
        C::EnvironmentProvenanceSevered.default_trigger(),
        T::ProvenanceSevered
    );
    assert_eq!(
        C::KernelUnavailable.default_trigger(),
        T::ReconnectShownAsFresh
    );
}

#[test]
fn cannot_be_shown_live_states_are_flagged() {
    use M5NotebookComponentConditionState as C;
    assert!(C::KernelParityPartial.cannot_be_shown_live_trusted());
    assert!(C::KernelOriginDegraded.cannot_be_shown_live_trusted());
    assert!(C::OutputTrustStale.cannot_be_shown_live_trusted());
    assert!(C::EnvironmentProvenanceSevered.cannot_be_shown_live_trusted());
    assert!(!C::DebuggerUnsupported.cannot_be_shown_live_trusted());
    assert!(!C::KernelUnavailable.cannot_be_shown_live_trusted());
    assert!(!C::LiveTrusted.cannot_be_shown_live_trusted());
}

// --- AC1: large-output virtualization / collapsed-output honesty ---

#[test]
fn stale_output_banner_virtualizes_but_stays_attributable() {
    let banner = row("a11y:output-trust-banner-stale");
    assert!(banner.output_virtualization.is_truncating());
    assert_eq!(
        banner.output_virtualization.state,
        M5NotebookOutputVirtualizationState::VirtualizedAttributed
    );
    assert!(banner.virtualized_output_stays_attributable());
    // A virtualized output is a disclosed reduction (yellow) but not stranded.
    assert!(banner.is_reduced());
    assert_eq!(
        banner.status(),
        NotebookComponentAccessibilityStatus::NarrowedDisclosed
    );
}

#[test]
fn provenance_chip_group_collapses_but_stays_attributable() {
    let chips = row("a11y:output-provenance-chip-group-complete");
    assert_eq!(
        chips.output_virtualization.state,
        M5NotebookOutputVirtualizationState::CollapsedSummarized
    );
    assert!(chips.virtualized_output_stays_attributable());
}

#[test]
fn anonymous_blob_output_strands_a_row() {
    let mut banner = row("a11y:output-trust-banner-stale");
    banner.output_virtualization.state = M5NotebookOutputVirtualizationState::AnonymousBlob;
    assert!(!banner.virtualized_output_stays_attributable());
    assert_eq!(
        banner.status(),
        NotebookComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn virtualized_output_losing_trust_class_strands_a_row() {
    let mut banner = row("a11y:output-trust-banner-stale");
    banner.output_virtualization.preserves_trust_class = false;
    assert!(!banner.virtualized_output_stays_attributable());
    assert_eq!(
        banner.status(),
        NotebookComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn virtualization_honesty_unproven_when_no_virtualized_row() {
    let mut packet = seeded_m5_notebook_kernel_output_component_a11y_packet();
    for r in packet.rows.iter_mut() {
        r.output_virtualization = NotebookOutputVirtualizationDisclosure::full();
    }
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        NotebookComponentAccessibilityViolation::VirtualizationHonestyUnproven
    )));
}

// --- AC2: accessibility / CLI / export reach the same canonical truth + no-loss ---

#[test]
fn every_row_reaches_canonical_truth_via_at() {
    for r in seeded_m5_notebook_kernel_output_component_a11y_packet().rows {
        assert!(
            r.reaches_canonical_truth_via_at(),
            "row {} strands assistive tech",
            r.row_id
        );
    }
}

#[test]
fn hierarchy_heavy_chip_group_binds_a_non_visual_fallback() {
    let chips = row("a11y:output-provenance-chip-group-complete");
    assert!(chips.is_hierarchy_heavy());
    assert!(chips.has_non_visual_fallback());
    assert!(chips
        .fallback_modalities
        .contains(&M5NotebookComponentFallbackModality::Structured));
}

#[test]
fn view_only_keyboard_trap_strands_and_reds_a_row() {
    let mut header = row("a11y:notebook-document-header-live");
    header.keyboard_reach = NotebookComponentNonVisualReachState::ViewOnlyTrap;
    assert!(!header.reaches_canonical_truth_via_at());
    assert_eq!(
        header.status(),
        NotebookComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn high_zoom_trap_strands_a_row() {
    let mut header = row("a11y:notebook-document-header-live");
    header.high_zoom_reach = NotebookComponentNonVisualReachState::ViewOnlyTrap;
    assert!(!header.reaches_canonical_truth_via_at());
    assert_eq!(
        header.status(),
        NotebookComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn reduced_motion_trap_strands_a_row() {
    let mut header = row("a11y:notebook-document-header-live");
    header.reduced_motion_reach = NotebookComponentNonVisualReachState::ViewOnlyTrap;
    assert!(!header.reaches_canonical_truth_via_at());
    assert_eq!(
        header.status(),
        NotebookComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn cli_trap_strands_a_row() {
    let mut pill = row("a11y:kernel-origin-pill-degraded");
    pill.cli_reach = NotebookComponentNonVisualReachState::ViewOnlyTrap;
    assert!(!pill.reaches_canonical_truth_via_at());
    assert_eq!(
        pill.status(),
        NotebookComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn empty_notebook_context_ref_strands_a_row() {
    let mut header = row("a11y:notebook-document-header-live");
    header.notebook_context_ref = "  ".to_owned();
    assert!(!header.reaches_canonical_truth_via_at());
}

#[test]
fn export_needing_raw_payload_is_rejected() {
    let mut header = row("a11y:notebook-document-header-live");
    header.export_summary = NotebookComponentExportSummaryState::RequiresRawPayload;
    assert!(!header.export_preserves_meaning());
    assert_eq!(
        header.status(),
        NotebookComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn copy_export_requires_text_json_markdown() {
    let mut header = row("a11y:notebook-document-header-live");
    header.copy_export.formats.retain(|f| f != "markdown");
    assert!(!header.export_preserves_meaning());
}

#[test]
fn dropped_truth_strands_a_row() {
    let mut banner = row("a11y:output-trust-banner-stale");
    banner.truth_preserved = false;
    assert!(!banner.preserves_truth_continuity());
    assert_eq!(
        banner.status(),
        NotebookComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn narrow_dropping_truth_continuity_strands_a_row() {
    let mut banner = row("a11y:output-trust-banner-stale");
    if let Some(narrow) = banner.claim_narrow.as_mut() {
        narrow.preserves_truth_continuity = false;
    }
    assert!(!banner.preserves_truth_continuity());
    assert!(!banner.claim_is_honest());
}

// --- AC / narrowing disclosed across every narrower surface ---

#[test]
fn narrowed_surface_without_disclosure_is_rejected() {
    let mut picker = row("a11y:kernel-picker-row-partial-parity");
    picker.narrowing_disclosures.clear();
    assert!(!picker.narrowing_disclosed());
}

#[test]
fn silently_dropped_surface_is_rejected() {
    let mut picker = row("a11y:kernel-picker-row-partial-parity");
    picker.narrowing_disclosures[0].state =
        NotebookComponentNarrowingDisclosureState::SilentlyDropped;
    assert!(!picker.narrowing_disclosed());
}

#[test]
fn narrowed_surface_must_preserve_labels() {
    let mut picker = row("a11y:kernel-picker-row-partial-parity");
    picker.narrowing_disclosures[0].preserved_labels.clear();
    assert!(!picker.narrowing_disclosed());
}

#[test]
fn dropping_a_mandatory_label_strands_a_row() {
    let mut header = row("a11y:notebook-document-header-live");
    header
        .required_labels
        .retain(|l| *l != M5NotebookKernelOutputRequiredLabel::Identity);
    assert!(!header.preserves_mandatory_labels());
    assert_eq!(
        header.status(),
        NotebookComponentAccessibilityStatus::Stranded
    );
}

// --- packet-level validation ---

#[test]
fn missing_family_coverage_is_flagged() {
    let mut packet = seeded_m5_notebook_kernel_output_component_a11y_packet();
    packet.rows.retain(|r| {
        r.component_family != M5NotebookKernelOutputComponentFamily::KernelRecoveryCard
    });
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        NotebookComponentAccessibilityViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn single_consumer_surface_fails_parity() {
    let mut packet = seeded_m5_notebook_kernel_output_component_a11y_packet();
    packet.rows[0].consumer_surfaces = vec![M5NotebookKernelOutputConsumerSurface::SupportExport];
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        NotebookComponentAccessibilityViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut packet = seeded_m5_notebook_kernel_output_component_a11y_packet();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        NotebookComponentAccessibilityViolation::DuplicateId { .. }
    )));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut packet = seeded_m5_notebook_kernel_output_component_a11y_packet();
    packet.summary.green_count += 1;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, NotebookComponentAccessibilityViolation::SummaryMismatch)));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut packet = seeded_m5_notebook_kernel_output_component_a11y_packet();
    packet.rows[0]
        .source_refs
        .push("api_key=hunter2".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        NotebookComponentAccessibilityViolation::RawNotebookMaterialInExport
    )));
}

// --- rendering ---

#[test]
fn chip_tokens_render_stable_fields() {
    let chip = row("a11y:output-trust-banner-stale").chip_tokens();
    assert!(chip.contains("family=output_trust_banner"));
    assert!(chip.contains("effective_claim=stale_output_projection"));
    assert!(chip.contains("virtualization=virtualized_attributed"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_notebook_kernel_output_component_a11y_packet();
    assert_eq!(packet.record_kind, NOTEBOOK_KERNEL_OUTPUT_A11Y_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        NOTEBOOK_KERNEL_OUTPUT_A11Y_SCHEMA_VERSION
    );
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_notebook_kernel_output_component_a11y_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn csv_has_one_header_and_one_line_per_row() {
    let packet = seeded_m5_notebook_kernel_output_component_a11y_packet();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), packet.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,component_family"));
}

// --- checked-in artifacts ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_m5_notebook_kernel_output_component_a11y_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(
        packet,
        seeded_m5_notebook_kernel_output_component_a11y_packet()
    );
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_notebook_kernel_output_component_a11y_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-notebook-kernel-output-component-accessibility-parity/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected =
        seeded_m5_notebook_kernel_output_component_a11y_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-notebook-kernel-output-component-accessibility-parity.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var so it never runs
/// in the normal suite. Run with
/// `GEN_NOTEBOOK_KERNEL_OUTPUT_A11Y_ARTIFACTS=1 cargo test -p aureline-notebook generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_NOTEBOOK_KERNEL_OUTPUT_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_notebook_kernel_output_component_a11y_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest)
        .join("../../artifacts/release/m5-notebook-kernel-output-component-accessibility-parity");
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(
        Path::new(manifest).join(
            "../../artifacts/release/m5-notebook-kernel-output-component-accessibility-parity.md",
        ),
        &report,
    )
    .expect("write report");

    let fixtures = Path::new(manifest)
        .join("../../fixtures/ui/m5-notebook-kernel-output-component-accessibility-parity");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
}
