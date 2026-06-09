use super::*;
use crate::OutputTrustClass;

fn sample_sanitized_inline_lane() -> NotebookOutputViewerLane {
    NotebookOutputViewerLane {
        record_kind: NOTEBOOK_OUTPUT_VIEWER_LANE_RECORD_KIND.to_owned(),
        notebook_output_viewer_schema_version: NOTEBOOK_OUTPUT_VIEWER_SCHEMA_VERSION,
        lane_id: "nb.viewer_lane.sanitized.inline.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.01".to_owned(),
        output_block_ref: "nb.output.01".to_owned(),
        trust_class: OutputTrustClass::Sanitized,
        viewer_lane_class: OutputViewerLaneClass::Inline,
        size_bucket: OutputSizeBucket::Small,
        virtualization_state_class: OutputVirtualizationStateClass::NotNeeded,
        compatible_viewer_available: true,
        raw_fallback_available: true,
        summary: "Small sanitized HTML output rendered inline.".to_owned(),
    }
}

fn sample_sandboxed_virtualized_lane() -> NotebookOutputViewerLane {
    NotebookOutputViewerLane {
        record_kind: NOTEBOOK_OUTPUT_VIEWER_LANE_RECORD_KIND.to_owned(),
        notebook_output_viewer_schema_version: NOTEBOOK_OUTPUT_VIEWER_SCHEMA_VERSION,
        lane_id: "nb.viewer_lane.sandboxed.virtualized.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.02".to_owned(),
        output_block_ref: "nb.output.02".to_owned(),
        trust_class: OutputTrustClass::Sandboxed,
        viewer_lane_class: OutputViewerLaneClass::Virtualized,
        size_bucket: OutputSizeBucket::Large,
        virtualization_state_class: OutputVirtualizationStateClass::Virtualized,
        compatible_viewer_available: true,
        raw_fallback_available: true,
        summary: "Large sandboxed table output rendered with virtualization.".to_owned(),
    }
}

fn sample_trusted_active_open_detail_lane() -> NotebookOutputViewerLane {
    NotebookOutputViewerLane {
        record_kind: NOTEBOOK_OUTPUT_VIEWER_LANE_RECORD_KIND.to_owned(),
        notebook_output_viewer_schema_version: NOTEBOOK_OUTPUT_VIEWER_SCHEMA_VERSION,
        lane_id: "nb.viewer_lane.trusted_active.open_detail.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.03".to_owned(),
        output_block_ref: "nb.output.03".to_owned(),
        trust_class: OutputTrustClass::TrustedActive,
        viewer_lane_class: OutputViewerLaneClass::OpenDetail,
        size_bucket: OutputSizeBucket::VeryLarge,
        virtualization_state_class: OutputVirtualizationStateClass::LazyPending,
        compatible_viewer_available: true,
        raw_fallback_available: false,
        summary: "Very large trusted-active widget opened in detail pane.".to_owned(),
    }
}

fn sample_blocked_active_content_lane() -> NotebookOutputViewerLane {
    NotebookOutputViewerLane {
        record_kind: NOTEBOOK_OUTPUT_VIEWER_LANE_RECORD_KIND.to_owned(),
        notebook_output_viewer_schema_version: NOTEBOOK_OUTPUT_VIEWER_SCHEMA_VERSION,
        lane_id: "nb.viewer_lane.blocked.active.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.04".to_owned(),
        output_block_ref: "nb.output.04".to_owned(),
        trust_class: OutputTrustClass::TrustedActive,
        viewer_lane_class: OutputViewerLaneClass::BlockedActiveContent,
        size_bucket: OutputSizeBucket::Medium,
        virtualization_state_class: OutputVirtualizationStateClass::NotNeeded,
        compatible_viewer_available: false,
        raw_fallback_available: true,
        summary: "Trusted-active output blocked because no compatible viewer is available.".to_owned(),
    }
}

fn sample_stale_fallback_lane() -> NotebookOutputViewerLane {
    NotebookOutputViewerLane {
        record_kind: NOTEBOOK_OUTPUT_VIEWER_LANE_RECORD_KIND.to_owned(),
        notebook_output_viewer_schema_version: NOTEBOOK_OUTPUT_VIEWER_SCHEMA_VERSION,
        lane_id: "nb.viewer_lane.stale.fallback.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.05".to_owned(),
        output_block_ref: "nb.output.05".to_owned(),
        trust_class: OutputTrustClass::Stale,
        viewer_lane_class: OutputViewerLaneClass::Inline,
        size_bucket: OutputSizeBucket::Small,
        virtualization_state_class: OutputVirtualizationStateClass::NotNeeded,
        compatible_viewer_available: true,
        raw_fallback_available: true,
        summary: "Stale small output rendered inline with fallback viewer.".to_owned(),
    }
}

fn sample_small_virtualization() -> LargeOutputVirtualizationRecord {
    LargeOutputVirtualizationRecord {
        record_kind: LARGE_OUTPUT_VIRTUALIZATION_RECORD_KIND.to_owned(),
        notebook_output_viewer_schema_version: NOTEBOOK_OUTPUT_VIEWER_SCHEMA_VERSION,
        virtualization_id: "nb.virt.small.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.01".to_owned(),
        output_block_ref: "nb.output.01".to_owned(),
        size_bucket: OutputSizeBucket::Small,
        byte_size_estimate: 512,
        row_count_estimate: 0,
        virtualization_state_class: OutputVirtualizationStateClass::NotNeeded,
        truncation_note: None,
        expand_action_available: false,
        export_action_available: true,
        summary: "Small text output needs no virtualization.".to_owned(),
    }
}

fn sample_large_virtualization() -> LargeOutputVirtualizationRecord {
    LargeOutputVirtualizationRecord {
        record_kind: LARGE_OUTPUT_VIRTUALIZATION_RECORD_KIND.to_owned(),
        notebook_output_viewer_schema_version: NOTEBOOK_OUTPUT_VIEWER_SCHEMA_VERSION,
        virtualization_id: "nb.virt.large.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.02".to_owned(),
        output_block_ref: "nb.output.02".to_owned(),
        size_bucket: OutputSizeBucket::Large,
        byte_size_estimate: 2_500_000,
        row_count_estimate: 50_000,
        virtualization_state_class: OutputVirtualizationStateClass::Virtualized,
        truncation_note: None,
        expand_action_available: true,
        export_action_available: true,
        summary: "Large table virtualized with 50k rows and 2.5 MB payload.".to_owned(),
    }
}

fn sample_truncated_virtualization() -> LargeOutputVirtualizationRecord {
    LargeOutputVirtualizationRecord {
        record_kind: LARGE_OUTPUT_VIRTUALIZATION_RECORD_KIND.to_owned(),
        notebook_output_viewer_schema_version: NOTEBOOK_OUTPUT_VIEWER_SCHEMA_VERSION,
        virtualization_id: "nb.virt.truncated.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.03".to_owned(),
        output_block_ref: "nb.output.03".to_owned(),
        size_bucket: OutputSizeBucket::VeryLarge,
        byte_size_estimate: 150_000_000,
        row_count_estimate: 5_000_000,
        virtualization_state_class: OutputVirtualizationStateClass::Truncated,
        truncation_note: Some("Output truncated after first 10,000 rows. Expand to load more.".to_owned()),
        expand_action_available: true,
        export_action_available: true,
        summary: "Very large output truncated at 10k rows; full payload is 150 MB.".to_owned(),
    }
}

fn sample_lazy_pending_virtualization() -> LargeOutputVirtualizationRecord {
    LargeOutputVirtualizationRecord {
        record_kind: LARGE_OUTPUT_VIRTUALIZATION_RECORD_KIND.to_owned(),
        notebook_output_viewer_schema_version: NOTEBOOK_OUTPUT_VIEWER_SCHEMA_VERSION,
        virtualization_id: "nb.virt.lazy.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        cell_id_ref: "nb.cell.04".to_owned(),
        output_block_ref: "nb.output.04".to_owned(),
        size_bucket: OutputSizeBucket::Large,
        byte_size_estimate: 8_000_000,
        row_count_estimate: 0,
        virtualization_state_class: OutputVirtualizationStateClass::LazyPending,
        truncation_note: None,
        expand_action_available: true,
        export_action_available: false,
        summary: "Large image output deferred until user clicks to load.".to_owned(),
    }
}

#[test]
fn sanitized_inline_validates_clean() {
    let lane = sample_sanitized_inline_lane();
    assert!(
        lane.validate().is_empty(),
        "sanitized inline lane should be clean: {:?}",
        lane.validate()
    );
}

#[test]
fn sandboxed_virtualized_validates_clean() {
    let lane = sample_sandboxed_virtualized_lane();
    assert!(
        lane.validate().is_empty(),
        "sandboxed virtualized lane should be clean: {:?}",
        lane.validate()
    );
}

#[test]
fn trusted_active_open_detail_validates_clean() {
    let lane = sample_trusted_active_open_detail_lane();
    assert!(
        lane.validate().is_empty(),
        "trusted active open_detail lane should be clean: {:?}",
        lane.validate()
    );
}

#[test]
fn blocked_active_content_validates_clean() {
    let lane = sample_blocked_active_content_lane();
    assert!(
        lane.validate().is_empty(),
        "blocked active content lane should be clean: {:?}",
        lane.validate()
    );
}

#[test]
fn stale_fallback_validates_clean() {
    let lane = sample_stale_fallback_lane();
    assert!(
        lane.validate().is_empty(),
        "stale fallback lane should be clean: {:?}",
        lane.validate()
    );
}

#[test]
fn trusted_active_blocked_despite_viewer_is_rejected() {
    let mut lane = sample_blocked_active_content_lane();
    lane.compatible_viewer_available = true;
    let findings = lane.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "output_viewer_lane.trusted_active_blocked_despite_viewer"));
}

#[test]
fn sanitized_blocked_is_rejected() {
    let mut lane = sample_sanitized_inline_lane();
    lane.viewer_lane_class = OutputViewerLaneClass::BlockedActiveContent;
    let findings = lane.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "output_viewer_lane.sanitized_sandboxed_not_blocked"));
}

#[test]
fn virtualized_but_small_is_rejected() {
    let mut lane = sample_sanitized_inline_lane();
    lane.viewer_lane_class = OutputViewerLaneClass::Virtualized;
    let findings = lane.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "output_viewer_lane.virtualized_but_small"));
}

#[test]
fn inline_but_large_is_rejected() {
    let mut lane = sample_sandboxed_virtualized_lane();
    lane.viewer_lane_class = OutputViewerLaneClass::Inline;
    let findings = lane.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "output_viewer_lane.inline_but_large"));
}

#[test]
fn not_needed_but_large_is_rejected() {
    let mut lane = sample_sandboxed_virtualized_lane();
    lane.virtualization_state_class = OutputVirtualizationStateClass::NotNeeded;
    let findings = lane.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "output_viewer_lane.not_needed_but_large"));
}

#[test]
fn viewer_required_but_missing_is_rejected() {
    let mut lane = sample_sanitized_inline_lane();
    lane.compatible_viewer_available = false;
    let findings = lane.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "output_viewer_lane.viewer_required_but_missing"));
}

#[test]
fn small_virtualization_validates_clean() {
    let virt = sample_small_virtualization();
    assert!(
        virt.validate().is_empty(),
        "small virtualization should be clean: {:?}",
        virt.validate()
    );
}

#[test]
fn large_virtualization_validates_clean() {
    let virt = sample_large_virtualization();
    assert!(
        virt.validate().is_empty(),
        "large virtualization should be clean: {:?}",
        virt.validate()
    );
}

#[test]
fn truncated_virtualization_validates_clean() {
    let virt = sample_truncated_virtualization();
    assert!(
        virt.validate().is_empty(),
        "truncated virtualization should be clean: {:?}",
        virt.validate()
    );
}

#[test]
fn lazy_pending_virtualization_validates_clean() {
    let virt = sample_lazy_pending_virtualization();
    assert!(
        virt.validate().is_empty(),
        "lazy pending virtualization should be clean: {:?}",
        virt.validate()
    );
}

#[test]
fn truncated_without_note_is_rejected() {
    let mut virt = sample_truncated_virtualization();
    virt.truncation_note = None;
    let findings = virt.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "large_output_virtualization.truncated_note_required"));
}

#[test]
fn small_but_virtualized_is_rejected() {
    let mut virt = sample_small_virtualization();
    virt.virtualization_state_class = OutputVirtualizationStateClass::Virtualized;
    let findings = virt.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "large_output_virtualization.small_but_virtualized"));
}

#[test]
fn large_but_not_virtualized_is_rejected() {
    let mut virt = sample_large_virtualization();
    virt.virtualization_state_class = OutputVirtualizationStateClass::NotNeeded;
    let findings = virt.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "large_output_virtualization.large_but_not_virtualized"));
}

#[test]
fn no_size_estimate_is_rejected() {
    let mut virt = sample_large_virtualization();
    virt.byte_size_estimate = 0;
    virt.row_count_estimate = 0;
    let findings = virt.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "large_output_virtualization.no_size_estimate"));
}

#[test]
fn packet_validates_clean() {
    let packet = NotebookOutputViewerPacket {
        schema_version: NOTEBOOK_OUTPUT_VIEWER_SCHEMA_VERSION,
        record_kind: NOTEBOOK_OUTPUT_VIEWER_PACKET_RECORD_KIND.to_owned(),
        packet_id: "nb.output_viewer.packet.m5.01".to_owned(),
        as_of: "2026-06-09T00:00:00Z".to_owned(),
        output_viewer_lane_classes: OutputViewerLaneClass::ALL.to_vec(),
        output_size_buckets: OutputSizeBucket::ALL.to_vec(),
        output_virtualization_state_classes: OutputVirtualizationStateClass::ALL.to_vec(),
        example_viewer_lanes: vec![
            sample_sanitized_inline_lane(),
            sample_sandboxed_virtualized_lane(),
            sample_trusted_active_open_detail_lane(),
            sample_blocked_active_content_lane(),
            sample_stale_fallback_lane(),
        ],
        example_large_output_virtualizations: vec![
            sample_small_virtualization(),
            sample_large_virtualization(),
            sample_truncated_virtualization(),
            sample_lazy_pending_virtualization(),
        ],
        summary:
            "Notebook output trust classes, sanitized or sandboxed viewer lanes, and large-output virtualization packet v1."
                .to_owned(),
    };
    assert!(
        packet.validate().is_empty(),
        "packet should be clean: {:?}",
        packet.validate()
    );
}

#[test]
fn embedded_packet_parses() {
    let packet = current_notebook_output_viewer_packet().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, NOTEBOOK_OUTPUT_VIEWER_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, NOTEBOOK_OUTPUT_VIEWER_PACKET_RECORD_KIND);
}

#[test]
fn closed_vocabularies_expose_stable_tokens() {
    assert_eq!(OutputViewerLaneClass::Inline.as_str(), "inline");
    assert_eq!(OutputViewerLaneClass::BlockedActiveContent.as_str(), "blocked_active_content");
    assert!(OutputViewerLaneClass::Inline.requires_compatible_viewer());
    assert!(!OutputViewerLaneClass::BlockedActiveContent.requires_compatible_viewer());
    assert!(OutputViewerLaneClass::BlockedActiveContent.is_placeholder());

    assert_eq!(OutputSizeBucket::Small.as_str(), "small");
    assert!(OutputSizeBucket::Small.admits_inline());
    assert!(!OutputSizeBucket::Small.requires_virtualization());
    assert!(OutputSizeBucket::Large.requires_virtualization());

    assert_eq!(OutputVirtualizationStateClass::Truncated.as_str(), "truncated");
    assert!(OutputVirtualizationStateClass::Truncated.is_partial());
    assert!(!OutputVirtualizationStateClass::NotNeeded.is_partial());
}
