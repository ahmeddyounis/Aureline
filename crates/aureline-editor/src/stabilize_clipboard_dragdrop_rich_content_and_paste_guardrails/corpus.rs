//! Deterministic claimed-stable matrix for transfer-safety packets.
//!
//! The corpus covers rich/raw copy, sensitive copy preview, remote clipboard
//! policy, multiline paste guardrails, drag/drop verb cues, cross-window split,
//! notebook large attach progress, docs rich-content trust, support export
//! redaction, and named undo-group lineage.

use super::model::{
    BoundaryClass, BoundaryContext, DropPreview, DropVerb, LargeTransferFeedback, PasteGuardrail,
    RecoveryClass, RepresentationTruth, RichContentTrust, RichTrustClass, SensitiveReview,
    SurfaceProjection, TransferActionClass, TransferRepresentationClass, TransferSafetyInput,
    TransferSafetyPacket, TransferSurfaceClass, UndoGroupTruth,
};

/// Snapshot timestamp pinned for every record in the corpus.
pub const TRANSFER_SAFETY_CORPUS_AS_OF: &str = "2026-06-06T00:00:00Z";

const SUPPORT_EXPORT_REF: &str = "aureline://support-export/transfer-safety";

/// One scenario in the claimed-stable transfer-safety matrix.
#[derive(Debug, Clone)]
pub struct TransferSafetyScenario {
    /// Stable scenario id (also the packet id).
    pub scenario_id: &'static str,
    /// On-disk fixture filename.
    pub fixture_filename: &'static str,
    /// Surface expected for the scenario.
    pub expected_surface: TransferSurfaceClass,
    /// Action expected for the scenario.
    pub expected_action: TransferActionClass,
    /// True when the fixture must carry a sensitive review.
    pub expects_sensitive_review: bool,
    /// True when the fixture must carry a drop preview.
    pub expects_drop_preview: bool,
    /// True when the fixture must carry a paste guardrail.
    pub expects_paste_guardrail: bool,
    /// True when the fixture must carry large-transfer feedback.
    pub expects_large_transfer: bool,
    /// True when the fixture must carry a named undo group.
    pub expects_named_undo_group: bool,
    packet: TransferSafetyPacket,
}

impl TransferSafetyScenario {
    /// Returns the governed packet for this scenario.
    pub fn packet(&self) -> TransferSafetyPacket {
        self.packet.clone()
    }
}

/// Returns the full claimed-stable transfer-safety corpus.
pub fn transfer_safety_corpus() -> Vec<TransferSafetyScenario> {
    vec![
        editor_rich_copy_preserves_raw(),
        support_sensitive_copy_preview(),
        terminal_remote_clipboard_policy(),
        terminal_multiline_paste_guard(),
        shell_drag_drop_import_preview(),
        shell_cross_window_split_preview(),
        notebook_large_output_attach(),
        docs_sanitized_rich_copy(),
        editor_multi_file_replace_undo(),
    ]
}

fn base_representation(default_representation: TransferRepresentationClass) -> RepresentationTruth {
    RepresentationTruth {
        default_plain_text_preserved: true,
        raw_copy_available: true,
        rendered_copy_available: false,
        escaped_copy_available: true,
        default_representation,
        representation_label: match default_representation {
            TransferRepresentationClass::Raw => "Plain text (raw source)",
            TransferRepresentationClass::Rendered => "Rendered rich text with plain-text fallback",
            TransferRepresentationClass::Escaped => "Escaped plain text",
            TransferRepresentationClass::Sanitized => "Sanitized snapshot with source fallback",
            TransferRepresentationClass::Sandboxed => "Sandboxed rich view with source fallback",
            TransferRepresentationClass::Generated => "Generated text with source attribution",
            TransferRepresentationClass::BlockedMetadataOnly => "Metadata only with body withheld",
        }
        .to_string(),
    }
}

fn raw_trust() -> RichContentTrust {
    RichContentTrust {
        trust_class: RichTrustClass::RawText,
        active_content_blocked_or_isolated: true,
        raw_source_available: true,
        copy_plain_text_available: true,
    }
}

fn sanitized_trust() -> RichContentTrust {
    RichContentTrust {
        trust_class: RichTrustClass::SanitizedRich,
        active_content_blocked_or_isolated: true,
        raw_source_available: true,
        copy_plain_text_available: true,
    }
}

fn projections(surfaces: &[TransferSurfaceClass], summary: &str) -> Vec<SurfaceProjection> {
    surfaces
        .iter()
        .copied()
        .map(|surface| SurfaceProjection {
            surface,
            reads_shared_record: true,
            summary_line: format!(
                "{} consumes transfer-safety packet: {summary}",
                surface.as_str()
            ),
        })
        .collect()
}

fn local_boundary(label: &str) -> BoundaryContext {
    BoundaryContext {
        boundary_class: BoundaryClass::Local,
        visible_boundary_label: label.to_string(),
        shown_before_commit: true,
    }
}

fn ssh_boundary(label: &str) -> BoundaryContext {
    BoundaryContext {
        boundary_class: BoundaryClass::SshRemote,
        visible_boundary_label: label.to_string(),
        shown_before_commit: true,
    }
}

fn named_undo(label: &str, source: &str, journal: &str, recovery: RecoveryClass) -> UndoGroupTruth {
    UndoGroupTruth {
        named: true,
        group_label: label.to_string(),
        source_attribution: source.to_string(),
        mutation_journal_entry_ref: journal.to_string(),
        recovery_class: recovery,
        history_surfaces: vec![
            "editor_undo_stack".to_string(),
            "local_history".to_string(),
            "reopen_history".to_string(),
            "activity_center".to_string(),
        ],
    }
}

fn editor_rich_copy_preserves_raw() -> TransferSafetyScenario {
    let mut representation = base_representation(TransferRepresentationClass::Raw);
    representation.rendered_copy_available = true;
    let packet = TransferSafetyPacket::build(TransferSafetyInput {
        packet_id: "transfer-safety:editor-rich-copy-preserves-raw".to_string(),
        title: "Editor copy keeps raw text as the default".to_string(),
        summary: "Copying a syntax-highlighted selection defaults to raw text while rendered and escaped variants remain explicit additive actions.".to_string(),
        surface: TransferSurfaceClass::Editor,
        action: TransferActionClass::Copy,
        representation,
        sensitive_review: None,
        boundary_context: Some(local_boundary("Local workspace clipboard")),
        paste_guardrail: None,
        drop_preview: None,
        undo_group: None,
        large_transfer: None,
        rich_content: raw_trust(),
        surface_projections: projections(
            &[TransferSurfaceClass::Editor, TransferSurfaceClass::Support],
            "raw copy default plus rendered and escaped alternatives",
        ),
        support_export_refs: vec![SUPPORT_EXPORT_REF.to_string()],
    })
    .expect("editor rich copy packet must build");
    TransferSafetyScenario {
        scenario_id: "transfer-safety:editor-rich-copy-preserves-raw",
        fixture_filename: "editor_rich_copy_preserves_raw.json",
        expected_surface: TransferSurfaceClass::Editor,
        expected_action: TransferActionClass::Copy,
        expects_sensitive_review: false,
        expects_drop_preview: false,
        expects_paste_guardrail: false,
        expects_large_transfer: false,
        expects_named_undo_group: false,
        packet,
    }
}

fn support_sensitive_copy_preview() -> TransferSafetyScenario {
    let mut representation = base_representation(TransferRepresentationClass::Escaped);
    representation.rendered_copy_available = false;
    let packet = TransferSafetyPacket::build(TransferSafetyInput {
        packet_id: "transfer-safety:support-sensitive-copy-preview".to_string(),
        title: "Support link copy shows sensitive preview".to_string(),
        summary: "Copying a support bundle link with private path context shows token/path labels and a policy gate before the clipboard changes.".to_string(),
        surface: TransferSurfaceClass::Support,
        action: TransferActionClass::Copy,
        representation,
        sensitive_review: Some(SensitiveReview {
            content_classes: vec![
                "support_bundle_link".to_string(),
                "private_path".to_string(),
                "token_like_value".to_string(),
            ],
            visible_label: "Private support link; copy once after review".to_string(),
            preview_before_commit: true,
            boundary_crossing: true,
            policy_gate_present: true,
        }),
        boundary_context: Some(BoundaryContext {
            boundary_class: BoundaryClass::SupportExport,
            visible_boundary_label: "Local clipboard -> support handoff".to_string(),
            shown_before_commit: true,
        }),
        paste_guardrail: None,
        drop_preview: None,
        undo_group: None,
        large_transfer: None,
        rich_content: sanitized_trust(),
        surface_projections: projections(
            &[TransferSurfaceClass::Support, TransferSurfaceClass::Docs],
            "sensitive copy chip preserves private-path and support-link labels",
        ),
        support_export_refs: vec![SUPPORT_EXPORT_REF.to_string()],
    })
    .expect("support sensitive copy packet must build");
    TransferSafetyScenario {
        scenario_id: "transfer-safety:support-sensitive-copy-preview",
        fixture_filename: "support_sensitive_copy_preview.json",
        expected_surface: TransferSurfaceClass::Support,
        expected_action: TransferActionClass::Copy,
        expects_sensitive_review: true,
        expects_drop_preview: false,
        expects_paste_guardrail: false,
        expects_large_transfer: false,
        expects_named_undo_group: false,
        packet,
    }
}

fn terminal_remote_clipboard_policy() -> TransferSafetyScenario {
    let packet = TransferSafetyPacket::build(TransferSafetyInput {
        packet_id: "transfer-safety:terminal-remote-clipboard-policy".to_string(),
        title: "Remote clipboard write is policy-aware".to_string(),
        summary: "An OSC 52 clipboard write from an SSH session shows the remote boundary and can be denied by policy before the local clipboard changes.".to_string(),
        surface: TransferSurfaceClass::Terminal,
        action: TransferActionClass::Copy,
        representation: base_representation(TransferRepresentationClass::Raw),
        sensitive_review: Some(SensitiveReview {
            content_classes: vec![
                "osc52_clipboard_write".to_string(),
                "remote_clipboard_bridge".to_string(),
            ],
            visible_label: "Remote host requests local clipboard write".to_string(),
            preview_before_commit: true,
            boundary_crossing: true,
            policy_gate_present: true,
        }),
        boundary_context: Some(ssh_boundary("SSH prod-shell -> local desktop clipboard")),
        paste_guardrail: None,
        drop_preview: None,
        undo_group: None,
        large_transfer: None,
        rich_content: raw_trust(),
        surface_projections: projections(
            &[TransferSurfaceClass::Terminal, TransferSurfaceClass::Shell, TransferSurfaceClass::Support],
            "remote clipboard write keeps boundary and policy outcome visible",
        ),
        support_export_refs: vec![SUPPORT_EXPORT_REF.to_string()],
    })
    .expect("terminal remote clipboard packet must build");
    TransferSafetyScenario {
        scenario_id: "transfer-safety:terminal-remote-clipboard-policy",
        fixture_filename: "terminal_remote_clipboard_policy.json",
        expected_surface: TransferSurfaceClass::Terminal,
        expected_action: TransferActionClass::Copy,
        expects_sensitive_review: true,
        expects_drop_preview: false,
        expects_paste_guardrail: false,
        expects_large_transfer: false,
        expects_named_undo_group: false,
        packet,
    }
}

fn terminal_multiline_paste_guard() -> TransferSafetyScenario {
    let packet = TransferSafetyPacket::build(TransferSafetyInput {
        packet_id: "transfer-safety:terminal-multiline-paste-guard".to_string(),
        title: "Multiline paste into remote terminal is reviewed".to_string(),
        summary: "Pasting 14 lines into a production-labeled SSH terminal shows boundary context, uses bracketed paste, and disables automatic submit.".to_string(),
        surface: TransferSurfaceClass::Terminal,
        action: TransferActionClass::Paste,
        representation: base_representation(TransferRepresentationClass::Raw),
        sensitive_review: Some(SensitiveReview {
            content_classes: vec!["multiline_shell_input".to_string(), "production_host".to_string()],
            visible_label: "Review 14-line paste before sending to prod-shell".to_string(),
            preview_before_commit: true,
            boundary_crossing: true,
            policy_gate_present: true,
        }),
        boundary_context: Some(ssh_boundary("SSH prod-shell as deploy")),
        paste_guardrail: Some(PasteGuardrail {
            line_count: 14,
            bracketed_paste_available: true,
            automatic_submit_disabled: true,
            confirmation_required: true,
            review_summary: "Paste 14 lines into prod-shell; automatic submit disabled.".to_string(),
        }),
        drop_preview: None,
        undo_group: Some(named_undo(
            "Paste into terminal input buffer",
            "system_clipboard",
            "mutation:terminal:paste:prod-shell:14-lines",
            RecoveryClass::EvidenceOnlyNoRerun,
        )),
        large_transfer: None,
        rich_content: raw_trust(),
        surface_projections: projections(
            &[TransferSurfaceClass::Terminal, TransferSurfaceClass::Shell, TransferSurfaceClass::Support],
            "multiline paste banner preserves bracketed-paste and no-auto-submit truth",
        ),
        support_export_refs: vec![SUPPORT_EXPORT_REF.to_string()],
    })
    .expect("terminal multiline paste packet must build");
    TransferSafetyScenario {
        scenario_id: "transfer-safety:terminal-multiline-paste-guard",
        fixture_filename: "terminal_multiline_paste_guard.json",
        expected_surface: TransferSurfaceClass::Terminal,
        expected_action: TransferActionClass::Paste,
        expects_sensitive_review: true,
        expects_drop_preview: false,
        expects_paste_guardrail: true,
        expects_large_transfer: false,
        expects_named_undo_group: true,
        packet,
    }
}

fn shell_drag_drop_import_preview() -> TransferSafetyScenario {
    let packet = TransferSafetyPacket::build(TransferSafetyInput {
        packet_id: "transfer-safety:shell-drag-drop-import-preview".to_string(),
        title: "Workspace drop previews import verb and collisions".to_string(),
        summary: "Dropping four files on the workspace tree shows Import into workspace, insertion indicator, modifier meanings, and a command-backed confirm path.".to_string(),
        surface: TransferSurfaceClass::Shell,
        action: TransferActionClass::DragDrop,
        representation: base_representation(TransferRepresentationClass::Raw),
        sensitive_review: Some(SensitiveReview {
            content_classes: vec!["private_path".to_string(), "overwrite_collision".to_string()],
            visible_label: "Import 4 files; 1 name collision".to_string(),
            preview_before_commit: true,
            boundary_crossing: false,
            policy_gate_present: true,
        }),
        boundary_context: Some(local_boundary("Local workspace drop target")),
        paste_guardrail: None,
        drop_preview: Some(DropPreview {
            verb: DropVerb::Import,
            insertion_indicator_visible: true,
            modifier_cues_visible: true,
            keyboard_route_available: true,
            command_fallback_id: "command:workspace.importFiles".to_string(),
        }),
        undo_group: Some(named_undo(
            "Import dropped files",
            "drag_drop:finder",
            "mutation:workspace:import:dropped-files",
            RecoveryClass::RestoreFromCheckpoint,
        )),
        large_transfer: Some(LargeTransferFeedback {
            progress_visible: true,
            cancellation_available: true,
            post_action_summary_present: true,
            result_summary: "4 files imported; 1 collision renamed after review.".to_string(),
        }),
        rich_content: raw_trust(),
        surface_projections: projections(
            &[TransferSurfaceClass::Shell, TransferSurfaceClass::Editor, TransferSurfaceClass::Support],
            "drop preview advertises import verb, modifiers, progress, and undo group",
        ),
        support_export_refs: vec![SUPPORT_EXPORT_REF.to_string()],
    })
    .expect("shell drag/drop import packet must build");
    TransferSafetyScenario {
        scenario_id: "transfer-safety:shell-drag-drop-import-preview",
        fixture_filename: "shell_drag_drop_import_preview.json",
        expected_surface: TransferSurfaceClass::Shell,
        expected_action: TransferActionClass::DragDrop,
        expects_sensitive_review: true,
        expects_drop_preview: true,
        expects_paste_guardrail: false,
        expects_large_transfer: true,
        expects_named_undo_group: true,
        packet,
    }
}

fn shell_cross_window_split_preview() -> TransferSafetyScenario {
    let packet = TransferSafetyPacket::build(TransferSafetyInput {
        packet_id: "transfer-safety:shell-cross-window-split-preview".to_string(),
        title: "Cross-window tab detach exposes split semantics".to_string(),
        summary: "Detaching an editor tab to another window shows Split to target window, preserves source/target identity, and offers a keyboard route.".to_string(),
        surface: TransferSurfaceClass::Shell,
        action: TransferActionClass::Split,
        representation: base_representation(TransferRepresentationClass::Raw),
        sensitive_review: None,
        boundary_context: Some(local_boundary("Local multi-window shell")),
        paste_guardrail: None,
        drop_preview: Some(DropPreview {
            verb: DropVerb::Split,
            insertion_indicator_visible: true,
            modifier_cues_visible: true,
            keyboard_route_available: true,
            command_fallback_id: "command:window.splitActiveEditorToTarget".to_string(),
        }),
        undo_group: Some(named_undo(
            "Split editor to target window",
            "shell_drag_drop",
            "mutation:shell:split:editor-tab",
            RecoveryClass::ExactUndo,
        )),
        large_transfer: None,
        rich_content: raw_trust(),
        surface_projections: projections(
            &[TransferSurfaceClass::Shell, TransferSurfaceClass::Editor],
            "cross-window split preview keeps keyboard parity and move/copy/split distinction",
        ),
        support_export_refs: vec![SUPPORT_EXPORT_REF.to_string()],
    })
    .expect("shell split packet must build");
    TransferSafetyScenario {
        scenario_id: "transfer-safety:shell-cross-window-split-preview",
        fixture_filename: "shell_cross_window_split_preview.json",
        expected_surface: TransferSurfaceClass::Shell,
        expected_action: TransferActionClass::Split,
        expects_sensitive_review: false,
        expects_drop_preview: true,
        expects_paste_guardrail: false,
        expects_large_transfer: false,
        expects_named_undo_group: true,
        packet,
    }
}

fn notebook_large_output_attach() -> TransferSafetyScenario {
    let packet = TransferSafetyPacket::build(TransferSafetyInput {
        packet_id: "transfer-safety:notebook-large-output-attach".to_string(),
        title: "Notebook output attach shows progress and summary".to_string(),
        summary: "Attaching large notebook outputs uses sanitized snapshots, shows cancellable progress, and records a named output-attach undo group.".to_string(),
        surface: TransferSurfaceClass::Notebook,
        action: TransferActionClass::Attach,
        representation: base_representation(TransferRepresentationClass::Sanitized),
        sensitive_review: Some(SensitiveReview {
            content_classes: vec!["notebook_output".to_string(), "large_transfer".to_string()],
            visible_label: "Attach sanitized notebook outputs".to_string(),
            preview_before_commit: true,
            boundary_crossing: false,
            policy_gate_present: true,
        }),
        boundary_context: Some(local_boundary("Notebook output attach target")),
        paste_guardrail: None,
        drop_preview: Some(DropPreview {
            verb: DropVerb::Attach,
            insertion_indicator_visible: true,
            modifier_cues_visible: true,
            keyboard_route_available: true,
            command_fallback_id: "command:notebook.attachOutputs".to_string(),
        }),
        undo_group: Some(named_undo(
            "Attach notebook outputs",
            "notebook_output_viewer",
            "mutation:notebook:attach:outputs",
            RecoveryClass::RestoreFromCheckpoint,
        )),
        large_transfer: Some(LargeTransferFeedback {
            progress_visible: true,
            cancellation_available: true,
            post_action_summary_present: true,
            result_summary: "3 sanitized outputs attached; raw bodies excluded by policy.".to_string(),
        }),
        rich_content: sanitized_trust(),
        surface_projections: projections(
            &[TransferSurfaceClass::Notebook, TransferSurfaceClass::Support, TransferSurfaceClass::Docs],
            "large notebook attach is cancellable, sanitized, and source-attributed",
        ),
        support_export_refs: vec![SUPPORT_EXPORT_REF.to_string()],
    })
    .expect("notebook output attach packet must build");
    TransferSafetyScenario {
        scenario_id: "transfer-safety:notebook-large-output-attach",
        fixture_filename: "notebook_large_output_attach.json",
        expected_surface: TransferSurfaceClass::Notebook,
        expected_action: TransferActionClass::Attach,
        expects_sensitive_review: true,
        expects_drop_preview: true,
        expects_paste_guardrail: false,
        expects_large_transfer: true,
        expects_named_undo_group: true,
        packet,
    }
}

fn docs_sanitized_rich_copy() -> TransferSafetyScenario {
    let mut representation = base_representation(TransferRepresentationClass::Rendered);
    representation.rendered_copy_available = true;
    let packet = TransferSafetyPacket::build(TransferSafetyInput {
        packet_id: "transfer-safety:docs-sanitized-rich-copy".to_string(),
        title: "Docs rich preview labels rendered copy".to_string(),
        summary: "Copying from a sanitized docs preview labels rendered output and keeps raw Markdown and escaped source copy reachable.".to_string(),
        surface: TransferSurfaceClass::Docs,
        action: TransferActionClass::Copy,
        representation,
        sensitive_review: Some(SensitiveReview {
            content_classes: vec!["rich_content".to_string(), "raw_rendered_divergence".to_string()],
            visible_label: "Rendered docs copy differs from Markdown source".to_string(),
            preview_before_commit: true,
            boundary_crossing: false,
            policy_gate_present: true,
        }),
        boundary_context: Some(local_boundary("Local docs preview")),
        paste_guardrail: None,
        drop_preview: None,
        undo_group: None,
        large_transfer: None,
        rich_content: sanitized_trust(),
        surface_projections: projections(
            &[TransferSurfaceClass::Docs, TransferSurfaceClass::Support],
            "rendered docs copy stays labeled and raw Markdown remains reachable",
        ),
        support_export_refs: vec![SUPPORT_EXPORT_REF.to_string()],
    })
    .expect("docs rich copy packet must build");
    TransferSafetyScenario {
        scenario_id: "transfer-safety:docs-sanitized-rich-copy",
        fixture_filename: "docs_sanitized_rich_copy.json",
        expected_surface: TransferSurfaceClass::Docs,
        expected_action: TransferActionClass::Copy,
        expects_sensitive_review: true,
        expects_drop_preview: false,
        expects_paste_guardrail: false,
        expects_large_transfer: false,
        expects_named_undo_group: false,
        packet,
    }
}

fn editor_multi_file_replace_undo() -> TransferSafetyScenario {
    let packet = TransferSafetyPacket::build(TransferSafetyInput {
        packet_id: "transfer-safety:editor-multi-file-replace-undo".to_string(),
        title: "Multi-file replace records named undo lineage".to_string(),
        summary: "Applying a multi-file replacement from paste/import attribution creates one named undo group with source, checkpoint, and reopen-history rows.".to_string(),
        surface: TransferSurfaceClass::Editor,
        action: TransferActionClass::MultiFileReplace,
        representation: base_representation(TransferRepresentationClass::Raw),
        sensitive_review: Some(SensitiveReview {
            content_classes: vec!["multi_file_replace".to_string(), "paste_import".to_string()],
            visible_label: "Review replace across 6 files".to_string(),
            preview_before_commit: true,
            boundary_crossing: false,
            policy_gate_present: true,
        }),
        boundary_context: Some(local_boundary("Local workspace replace scope")),
        paste_guardrail: None,
        drop_preview: None,
        undo_group: Some(named_undo(
            "Replace text across 6 files",
            "paste_import:review_sheet",
            "mutation:editor:replace:6-files",
            RecoveryClass::RestoreFromCheckpoint,
        )),
        large_transfer: Some(LargeTransferFeedback {
            progress_visible: true,
            cancellation_available: true,
            post_action_summary_present: true,
            result_summary: "18 replacements applied across 6 files; checkpoint ready.".to_string(),
        }),
        rich_content: raw_trust(),
        surface_projections: projections(
            &[TransferSurfaceClass::Editor, TransferSurfaceClass::Shell, TransferSurfaceClass::Support],
            "multi-file mutation shares one named undo group and reopen-history lineage",
        ),
        support_export_refs: vec![SUPPORT_EXPORT_REF.to_string()],
    })
    .expect("editor multi-file replace packet must build");
    TransferSafetyScenario {
        scenario_id: "transfer-safety:editor-multi-file-replace-undo",
        fixture_filename: "editor_multi_file_replace_undo.json",
        expected_surface: TransferSurfaceClass::Editor,
        expected_action: TransferActionClass::MultiFileReplace,
        expects_sensitive_review: true,
        expects_drop_preview: false,
        expects_paste_guardrail: false,
        expects_large_transfer: true,
        expects_named_undo_group: true,
        packet,
    }
}
