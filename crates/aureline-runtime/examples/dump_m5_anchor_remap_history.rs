//! Conformance dump for the M5 anchor-remap history set packet.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_runtime::diagnostics::DiagnosticAnchorRemapStateClass;
use aureline_runtime::record_anchor_remap_history_with_revision_pairs_and_drift_states_across_m5_lanes::*;

const PACKET_ID: &str = "m5-anchor-remap-history:stable:0001";
const WORKSPACE_ID: &str = "workspace:m5:anchor-remap";
const MINTED_AT: &str = "2026-06-19T00:00:00Z";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

#[allow(clippy::too_many_arguments)]
fn entry(
    label: &str,
    sequence: u32,
    lane: AnchorDriftLaneClass,
    old_anchor: Option<&str>,
    new_anchor: Option<&str>,
    basis: AnchorRemapEvidenceBasisClass,
    from_rev: &str,
    to_rev: &str,
    actor: AnchorRemapActorClass,
    summary: &str,
) -> AnchorRemapHistoryEntry {
    AnchorRemapHistoryEntry::new(AnchorRemapHistoryEntryInput {
        entry_id: format!("entry:{label}:{sequence:04}"),
        sequence,
        drift_lane_class: lane,
        old_anchor_ref: old_anchor.map(str::to_owned),
        new_anchor_ref: new_anchor.map(str::to_owned),
        evidence_basis_class: basis,
        evidence_basis_ref: format!("evidence:{label}:{sequence:04}"),
        revision_pair: RevisionPair::new(from_rev, to_rev),
        actor_class: actor,
        actor_tool_ref: format!("actor-tool:{label}"),
        produced_at: MINTED_AT.to_owned(),
        export_safe_summary: summary.to_owned(),
    })
}

/// File edit: a finding is created exact, then a later edit moves it so it only
/// contextually survives.
fn file_edit_history() -> AnchorRemapHistory {
    let label = "file-edit";
    AnchorRemapHistory::new(AnchorRemapHistoryInput {
        history_id: "history:m5:file-edit:0001".to_owned(),
        anchor_family_id: "anchor-family:m5:file-edit:0001".to_owned(),
        diagnostic_id: "diagnostic:m5:file-edit:0001".to_owned(),
        entries: vec![
            entry(
                label,
                0,
                AnchorDriftLaneClass::FileEdit,
                None,
                Some("anchor:file-edit:rev0"),
                AnchorRemapEvidenceBasisClass::ExactRangePreserved,
                "rev:file-edit:0",
                "rev:file-edit:0",
                AnchorRemapActorClass::EditorEditTracker,
                "Finding anchored to an exact range at its first revision.",
            ),
            entry(
                label,
                1,
                AnchorDriftLaneClass::FileEdit,
                Some("anchor:file-edit:rev0"),
                Some("anchor:file-edit:rev1"),
                AnchorRemapEvidenceBasisClass::SurroundingContextMatch,
                "rev:file-edit:0",
                "rev:file-edit:1",
                AnchorRemapActorClass::EditorEditTracker,
                "A live edit moved the range; it was re-anchored from surrounding context.",
            ),
        ],
        export_safe_summary:
            "A file edit moved the anchored range; the finding now only contextually survives."
                .to_owned(),
    })
}

/// Notebook cell identity change: a cell is re-keyed and no newer mapping is
/// found, so the finding is retained against a stale epoch.
fn notebook_cell_history() -> AnchorRemapHistory {
    let label = "notebook-cell";
    AnchorRemapHistory::new(AnchorRemapHistoryInput {
        history_id: "history:m5:notebook-cell:0001".to_owned(),
        anchor_family_id: "anchor-family:m5:notebook-cell:0001".to_owned(),
        diagnostic_id: "diagnostic:m5:notebook-cell:0001".to_owned(),
        entries: vec![
            entry(
                label,
                0,
                AnchorDriftLaneClass::NotebookCellIdentityChange,
                None,
                Some("anchor:notebook-cell:cell-a"),
                AnchorRemapEvidenceBasisClass::ExactRangePreserved,
                "rev:notebook:0",
                "rev:notebook:0",
                AnchorRemapActorClass::NotebookCellTracker,
                "Finding anchored to an exact range within a notebook cell.",
            ),
            entry(
                label,
                1,
                AnchorDriftLaneClass::NotebookCellIdentityChange,
                Some("anchor:notebook-cell:cell-a"),
                Some("anchor:notebook-cell:cell-a-stale"),
                AnchorRemapEvidenceBasisClass::StaleEpochRetained,
                "rev:notebook:0",
                "rev:notebook:1",
                AnchorRemapActorClass::NotebookCellTracker,
                "The cell was re-keyed; with no newer mapping the finding is retained as stale.",
            ),
        ],
        export_safe_summary:
            "A notebook cell identity change left no fresh mapping; the finding is retained against a stale epoch."
                .to_owned(),
    })
}

/// Generated artifact churn: a generated region is regenerated and the anchor can
/// no longer be located, so the finding becomes unmapped.
fn generated_artifact_history() -> AnchorRemapHistory {
    let label = "generated-artifact";
    AnchorRemapHistory::new(AnchorRemapHistoryInput {
        history_id: "history:m5:generated-artifact:0001".to_owned(),
        anchor_family_id: "anchor-family:m5:generated-artifact:0001".to_owned(),
        diagnostic_id: "diagnostic:m5:generated-artifact:0001".to_owned(),
        entries: vec![
            entry(
                label,
                0,
                AnchorDriftLaneClass::GeneratedArtifactChurn,
                None,
                Some("anchor:generated:region-1"),
                AnchorRemapEvidenceBasisClass::ExactRangePreserved,
                "rev:generated:0",
                "rev:generated:0",
                AnchorRemapActorClass::GeneratedArtifactReprojector,
                "Finding anchored to an exact range within a generated region.",
            ),
            entry(
                label,
                1,
                AnchorDriftLaneClass::GeneratedArtifactChurn,
                Some("anchor:generated:region-1"),
                None,
                AnchorRemapEvidenceBasisClass::NoMappingFound,
                "rev:generated:0",
                "rev:generated:1",
                AnchorRemapActorClass::GeneratedArtifactReprojector,
                "The artifact was regenerated; the region churned and the anchor could not be located.",
            ),
        ],
        export_safe_summary:
            "Generated-artifact churn dropped the anchored region; the finding is now unmapped, not silently discarded."
                .to_owned(),
    })
}

/// Imported snapshot comparison: an imported scan carries a static location that is
/// later mapped onto a local revision from surrounding context.
fn imported_snapshot_history() -> AnchorRemapHistory {
    let label = "imported-snapshot";
    AnchorRemapHistory::new(AnchorRemapHistoryInput {
        history_id: "history:m5:imported-snapshot:0001".to_owned(),
        anchor_family_id: "anchor-family:m5:imported-snapshot:0001".to_owned(),
        diagnostic_id: "diagnostic:m5:imported-snapshot:0001".to_owned(),
        entries: vec![
            entry(
                label,
                0,
                AnchorDriftLaneClass::ImportedSnapshotComparison,
                None,
                Some("anchor:imported-snapshot:static"),
                AnchorRemapEvidenceBasisClass::ImportedStaticLocation,
                "rev:imported-snapshot:snapshot",
                "rev:imported-snapshot:snapshot",
                AnchorRemapActorClass::ImportedScanComparator,
                "Imported scan carried a snapshot-only static location, not yet revalidated locally.",
            ),
            entry(
                label,
                1,
                AnchorDriftLaneClass::ImportedSnapshotComparison,
                Some("anchor:imported-snapshot:static"),
                Some("anchor:imported-snapshot:mapped"),
                AnchorRemapEvidenceBasisClass::SurroundingContextMatch,
                "rev:imported-snapshot:snapshot",
                "rev:imported-snapshot:local-1",
                AnchorRemapActorClass::ImportedScanComparator,
                "The static location was mapped onto a later local revision from surrounding context.",
            ),
        ],
        export_safe_summary:
            "An imported snapshot's static location was compared against a later local revision and mapped contextually."
                .to_owned(),
    })
}

/// Imported replay comparison: a replayed support bundle carries a static location
/// that has not been locally revalidated.
fn imported_replay_history() -> AnchorRemapHistory {
    let label = "imported-replay";
    AnchorRemapHistory::new(AnchorRemapHistoryInput {
        history_id: "history:m5:imported-replay:0001".to_owned(),
        anchor_family_id: "anchor-family:m5:imported-replay:0001".to_owned(),
        diagnostic_id: "diagnostic:m5:imported-replay:0001".to_owned(),
        entries: vec![entry(
            label,
            0,
            AnchorDriftLaneClass::ImportedReplayComparison,
            None,
            Some("anchor:imported-replay:static"),
            AnchorRemapEvidenceBasisClass::ImportedStaticLocation,
            "rev:imported-replay:bundle",
            "rev:imported-replay:bundle",
            AnchorRemapActorClass::ReplayComparator,
            "Replayed support bundle carried a snapshot-only static location, not locally revalidated.",
        )],
        export_safe_summary:
            "A replayed support bundle carries an imported-static location preserved as snapshot-only evidence."
                .to_owned(),
    })
}

fn histories() -> Vec<AnchorRemapHistory> {
    vec![
        file_edit_history(),
        notebook_cell_history(),
        generated_artifact_history(),
        imported_snapshot_history(),
        imported_replay_history(),
    ]
}

fn guardrails() -> AnchorRemapGuardrails {
    AnchorRemapGuardrails {
        drift_never_silently_dropped: true,
        same_remap_vocabulary_across_lanes: true,
        history_is_append_only: true,
        history_is_exportable: true,
        imported_static_supported_for_snapshot_only: true,
        no_silent_anchor_repair: true,
        revision_pair_recorded_per_remap: true,
    }
}

fn consumer_projection() -> AnchorRemapConsumerProjection {
    AnchorRemapConsumerProjection {
        editor_shows_remap_state: true,
        problems_shows_remap_state: true,
        review_shows_remap_history: true,
        cli_shows_remap_state: true,
        support_export_preserves_history: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    refs(&[
        M5_ANCHOR_REMAP_HISTORY_SET_SCHEMA_REF,
        M5_ANCHOR_REMAP_HISTORY_SET_DOC_REF,
        M5_ANCHOR_REMAP_HISTORY_SET_ARTIFACT_REF,
        CANONICAL_DIAGNOSTIC_RECORD_SET_SCHEMA_REF,
        "schemas/quality/diagnostic-source-and-collection.schema.json",
        "schemas/quality/m5-diagnostic-truth-lane.schema.json",
    ])
}

fn packet() -> AnchorRemapHistorySetPacket {
    AnchorRemapHistorySetPacket::new(AnchorRemapHistorySetPacketInput {
        packet_id: PACKET_ID.to_owned(),
        set_label: "M5 Anchor-Remap History Set".to_owned(),
        workspace_id: WORKSPACE_ID.to_owned(),
        histories: histories(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = packet();

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );
    assert!(
        packet.covers_all_drift_lanes(),
        "packet must exercise every drift lane"
    );
    for state in [
        DiagnosticAnchorRemapStateClass::Exact,
        DiagnosticAnchorRemapStateClass::Contextual,
        DiagnosticAnchorRemapStateClass::Stale,
        DiagnosticAnchorRemapStateClass::Unmapped,
        DiagnosticAnchorRemapStateClass::ImportedStatic,
    ] {
        assert!(
            packet.represented_states().contains(&state),
            "packet must exercise the {state:?} remap state"
        );
    }

    if which == "summary" {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
