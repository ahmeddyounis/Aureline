//! Unit tests for the canonical constrained-file and degraded-provider
//! assist-narrowing model.

use super::*;

#[test]
fn model_builds_and_every_invariant_holds() {
    let model = constrained_assist_model();
    assert_eq!(model.record_kind, M5_CONSTRAINED_ASSIST_RECORD_KIND);
    assert_eq!(model.schema_ref, M5_CONSTRAINED_ASSIST_SCHEMA_REF);
    assert_eq!(model.model_id, M5_CONSTRAINED_ASSIST_MODEL_ID);
    assert!(
        model.all_invariants_hold(),
        "every frozen invariant must hold: {:?}",
        model
            .invariants
            .iter()
            .filter(|invariant| !invariant.holds)
            .map(|invariant| &invariant.invariant_id)
            .collect::<Vec<_>>()
    );
    assert!(model.is_support_export_safe());
    assert!(model.raw_payload_excluded);
}

#[test]
fn model_serialization_round_trips() {
    let model = constrained_assist_model();
    let json = serde_json::to_string(&model).expect("model serializes");
    let restored: ConstrainedAssistModel = serde_json::from_str(&json).expect("model round-trips");
    assert_eq!(model, restored);
}

#[test]
fn every_state_has_a_profile_with_one_cell_per_channel() {
    let model = constrained_assist_model();
    assert_eq!(
        model.state_profiles.len(),
        ConstrainedFileStateClass::ALL.len()
    );
    for state in ConstrainedFileStateClass::ALL {
        let profile = model.profile(state).expect("profile present");
        assert_eq!(
            profile.cells.len(),
            AssistChannelClass::ALL.len(),
            "state {} must have one cell per channel",
            state.as_str()
        );
        for channel in AssistChannelClass::ALL {
            assert!(
                profile.cell(channel).is_some(),
                "state {} missing channel {}",
                state.as_str(),
                channel.as_str()
            );
        }
    }
}

#[test]
fn narrowed_cells_always_disclose_why() {
    let model = constrained_assist_model();
    for cell in model.all_cells() {
        assert!(
            cell.reason_inspectable(),
            "narrowed channel {} must disclose its reason",
            cell.channel.as_str()
        );
        if cell.is_narrowed() {
            assert!(
                !cell.disabled_state_diagnostic.trim().is_empty(),
                "narrowed channel {} must have a diagnostic",
                cell.channel.as_str()
            );
            assert!(cell.narrow_reason.is_some());
        }
    }
}

#[test]
fn blocked_apply_always_offers_a_route() {
    let model = constrained_assist_model();
    for cell in model.all_cells() {
        assert!(
            cell.apply_block_offers_route(),
            "channel {} blocks apply without a next-safe-action route",
            cell.channel.as_str()
        );
        if cell.apply_blocked {
            let action = cell.next_safe_action.expect("blocked apply has a route");
            let command = cell
                .next_safe_action_command_ref
                .as_ref()
                .expect("route has a command");
            assert_eq!(command, action.command_id());
            assert!(!command.trim().is_empty());
        }
    }
}

#[test]
fn no_apply_capable_channel_is_silently_hidden() {
    let model = constrained_assist_model();
    for cell in model.all_cells() {
        assert!(
            cell.no_silent_hidden_side_effect(),
            "apply-capable channel {} must mark apply blocked and disclose when blocked/unavailable",
            cell.channel.as_str()
        );
    }
}

#[test]
fn large_file_suppresses_semantic_and_apply_channels() {
    let model = constrained_assist_model();
    let profile = model
        .profile(ConstrainedFileStateClass::LargeFile)
        .expect("large file");
    for cell in &profile.cells {
        if cell.channel.is_semantic() || cell.channel.is_apply_capable() {
            assert_eq!(
                cell.degrade_class,
                AssistDegradeClass::SuppressedLargeFile,
                "channel {} must be suppressed on a large file",
                cell.channel.as_str()
            );
            assert!(!cell.disabled_state_diagnostic.trim().is_empty());
            assert_eq!(
                cell.next_safe_action,
                Some(NextSafeActionClass::OpenInFullEditor)
            );
        }
        // Even suppressed cells stay reachable.
        assert!(cell.keyboard_reachable);
    }
}

#[test]
fn partial_index_narrows_semantic_to_pending_without_blocking_apply() {
    let model = constrained_assist_model();
    let profile = model
        .profile(ConstrainedFileStateClass::PartialIndex)
        .expect("partial index");
    assert!(!profile.blocks_direct_apply);
    for cell in &profile.cells {
        if cell.channel.is_semantic() {
            assert_eq!(cell.degrade_class, AssistDegradeClass::PendingPartialIndex);
            assert_eq!(
                cell.narrow_reason,
                Some(NarrowReasonClass::IndexStillBuilding)
            );
            assert_eq!(
                cell.next_safe_action,
                Some(NextSafeActionClass::WaitForIndex)
            );
            assert!(!cell.apply_blocked);
        }
    }
}

#[test]
fn generated_and_managed_block_apply_with_regenerate_routes() {
    let model = constrained_assist_model();
    let generated = model
        .profile(ConstrainedFileStateClass::GeneratedArtifact)
        .expect("generated");
    for cell in generated
        .cells
        .iter()
        .filter(|c| c.channel.is_apply_capable())
    {
        assert!(cell.apply_blocked);
        assert_eq!(
            cell.next_safe_action,
            Some(NextSafeActionClass::OpenGeneratorSource)
        );
        assert_eq!(cell.degrade_class, AssistDegradeClass::ReadOnlyNoApply);
    }

    let managed = model
        .profile(ConstrainedFileStateClass::ManagedRegion)
        .expect("managed");
    for cell in managed
        .cells
        .iter()
        .filter(|c| c.channel.is_apply_capable())
    {
        assert!(cell.apply_blocked);
        assert_eq!(
            cell.next_safe_action,
            Some(NextSafeActionClass::RegenerateFromSource)
        );
    }
}

#[test]
fn restricted_blocks_apply_and_routes_to_approval() {
    let model = constrained_assist_model();
    let profile = model
        .profile(ConstrainedFileStateClass::RestrictedMode)
        .expect("restricted");
    for cell in profile
        .cells
        .iter()
        .filter(|c| c.channel.is_apply_capable())
    {
        assert!(cell.apply_blocked);
        assert_eq!(
            cell.narrow_reason,
            Some(NarrowReasonClass::WriteRequiresApproval)
        );
        assert_eq!(
            cell.next_safe_action,
            Some(NextSafeActionClass::RequestApprovalReview)
        );
    }
}

#[test]
fn captured_evidence_is_inspect_only_but_still_reads() {
    let model = constrained_assist_model();
    let profile = model
        .profile(ConstrainedFileStateClass::CapturedEvidence)
        .expect("captured");
    for cell in profile
        .cells
        .iter()
        .filter(|c| c.channel.is_apply_capable())
    {
        assert!(!cell.applicable);
        assert_eq!(cell.degrade_class, AssistDegradeClass::BlockedUnavailable);
        assert_eq!(
            cell.next_safe_action,
            Some(NextSafeActionClass::ViewOnlyNoAction)
        );
    }
    for channel in [AssistChannelClass::Hover, AssistChannelClass::Peek] {
        let cell = profile.cell(channel).expect("read channel");
        assert_eq!(cell.degrade_class, AssistDegradeClass::FullFidelity);
    }
}

#[test]
fn read_only_and_projection_allow_reading_block_writing() {
    let model = constrained_assist_model();
    for (state, action) in [
        (
            ConstrainedFileStateClass::ReadOnlyBoundary,
            NextSafeActionClass::DuplicateEditableCopy,
        ),
        (
            ConstrainedFileStateClass::ProjectionView,
            NextSafeActionClass::EditUnderlyingSource,
        ),
    ] {
        let profile = model.profile(state).expect("profile");
        for channel in [AssistChannelClass::Hover, AssistChannelClass::Peek] {
            assert_eq!(
                profile.cell(channel).unwrap().degrade_class,
                AssistDegradeClass::FullFidelity
            );
        }
        for cell in profile
            .cells
            .iter()
            .filter(|c| c.channel.is_apply_capable())
        {
            assert!(cell.apply_blocked);
            assert_eq!(cell.next_safe_action, Some(action));
        }
    }
}

#[test]
fn decoration_truth_is_preserved_except_in_large_file() {
    let model = constrained_assist_model();
    for profile in &model.state_profiles {
        let cell = profile.cell(AssistChannelClass::Decoration).unwrap();
        if profile.state_class == ConstrainedFileStateClass::LargeFile {
            assert_eq!(
                cell.degrade_class,
                AssistDegradeClass::SourceLabeledFallback
            );
            assert!(!cell.disabled_state_diagnostic.trim().is_empty());
        } else {
            assert_eq!(
                cell.degrade_class,
                AssistDegradeClass::FullFidelity,
                "decoration must stay full fidelity on {}",
                profile.state_class.as_str()
            );
        }
    }
}

#[test]
fn degraded_provider_cases_are_source_labeled_and_routed() {
    let model = constrained_assist_model();
    assert!(!model.degraded_provider_cases.is_empty());
    for case in &model.degraded_provider_cases {
        assert!(case.is_honest());
        assert_ne!(case.degrade_class, AssistDegradeClass::FullFidelity);
        assert!(case.source_labeled_not_silent);
        assert!(!case.disabled_state_diagnostic.trim().is_empty());
        assert_eq!(
            case.next_safe_action_command_ref,
            case.next_safe_action.command_id()
        );
    }
}

#[test]
fn consumer_proofs_reuse_shared_vocabulary() {
    let model = constrained_assist_model();
    for proof in &model.consumer_proofs {
        assert!(proof.reuses_shared_vocabulary);
        let profile = model.profile(proof.exhibited_state).expect("profile");
        let cell = profile.cell(proof.representative_channel).expect("cell");
        assert_eq!(cell.degrade_class, proof.resolved_degrade);
        assert_eq!(cell.next_safe_action, proof.next_safe_action);
    }
    for surface in [
        EditorSurfaceClass::NotebookCell,
        EditorSurfaceClass::GeneratedFile,
        EditorSurfaceClass::RequestEditor,
        EditorSurfaceClass::DocsCodeBlock,
        EditorSurfaceClass::ProtectedFile,
    ] {
        assert!(
            model
                .consumer_proofs
                .iter()
                .any(|proof| proof.base_editor_surface == Some(surface)),
            "missing consumer proof for {}",
            surface.as_str()
        );
    }
}

#[test]
fn class_catalogs_have_unique_tokens() {
    let model = constrained_assist_model();
    for catalog in [
        &model.state_classes,
        &model.channel_classes,
        &model.degrade_classes,
        &model.reason_classes,
        &model.next_safe_action_classes,
    ] {
        let mut tokens: Vec<&str> = catalog
            .iter()
            .map(|descriptor| descriptor.class_token.as_str())
            .collect();
        let total = tokens.len();
        tokens.sort_unstable();
        tokens.dedup();
        assert_eq!(tokens.len(), total, "catalog tokens must be unique");
    }
    assert_eq!(
        model.state_classes.len(),
        ConstrainedFileStateClass::ALL.len()
    );
    assert_eq!(model.reason_classes.len(), NarrowReasonClass::ALL.len());
    assert_eq!(
        model.next_safe_action_classes.len(),
        NextSafeActionClass::ALL.len()
    );
}

#[test]
fn lines_projection_renders_every_section() {
    let model = constrained_assist_model();
    let lines = constrained_assist_model_lines(&model);
    assert!(lines
        .iter()
        .any(|line| line.contains("Constrained-assist model")));
    assert!(lines.iter().any(|line| line.contains("State profiles:")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Degraded-provider cases:")));
    assert!(lines.iter().any(|line| line.contains("Consumer proofs:")));
    assert!(lines.iter().any(|line| line.contains("Invariants:")));
    for profile in &model.state_profiles {
        assert!(
            lines
                .iter()
                .any(|line| line.contains(profile.state_class.as_str())),
            "lines must mention state {}",
            profile.state_class.as_str()
        );
    }
}
