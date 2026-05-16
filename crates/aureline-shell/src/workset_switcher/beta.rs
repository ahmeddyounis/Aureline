//! Shell-facing beta workset-switcher projection.
//!
//! The chrome consumes the durable
//! [`aureline_workspace::WorksetSwitcherBetaRecord`],
//! [`aureline_workspace::WorksetActivationPreview`], and
//! [`aureline_workspace::WorksetReopenParityPacket`] verbatim. This module
//! owns one thin set of plaintext renderers that the support packet writer,
//! the headless CLI, and review surfaces all reuse so a switcher row never
//! re-mints its own portability vocabulary.

use aureline_workspace::{
    ReopenParityDowngrade, SwitcherRowAction, WorksetActivationPreview, WorksetReopenParityPacket,
    WorksetSwitcherBetaRecord, WorksetSwitcherBetaRow, WorksetSwitcherBetaSupportExport,
};

/// Renders a [`WorksetSwitcherBetaRecord`] as deterministic plaintext lines.
pub fn render_switcher_beta_lines(record: &WorksetSwitcherBetaRecord) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!(
        "[switcher] {workspace} · active={active}",
        workspace = record.workspace_ref,
        active = record.active_workset_ref,
    ));
    for row in &record.rows {
        lines.extend(render_switcher_row_lines(row));
    }
    lines
}

/// Renders one [`WorksetSwitcherBetaRow`] as deterministic plaintext lines.
pub fn render_switcher_row_lines(row: &WorksetSwitcherBetaRow) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!(
        "row{active} {name} ({scope_class}/{scope_mode}, {portability}, {readiness})",
        active = if row.is_active { " [ACTIVE]" } else { "" },
        name = row.workset_name,
        scope_class = row.scope_class.as_str(),
        scope_mode = row.scope_mode.as_str(),
        portability = row.portability_label.as_str(),
        readiness = row.readiness_state.as_str(),
    ));
    lines.push(format!(
        "  identity: workset_ref={workset_ref} stable_scope_id={scope}",
        workset_ref = row.workset_ref,
        scope = row.stable_scope_id,
    ));
    let roots: Vec<String> = row
        .root_taxonomy
        .iter()
        .map(|root| {
            let label = root
                .presentation_label
                .as_deref()
                .unwrap_or(root.root_ref.as_str());
            format!(
                "{label} ({kind}, {state})",
                kind = root.root_kind.as_str(),
                state = root.partial_truth.as_str(),
            )
        })
        .collect();
    lines.push(format!("  roots: {}", roots.join(", ")));
    if !row.include_patterns.is_empty() {
        lines.push(format!("  include_patterns: {}", row.include_patterns.join(", ")));
    }
    if !row.exclude_patterns.is_empty() {
        lines.push(format!("  exclude_patterns: {}", row.exclude_patterns.join(", ")));
    }
    if let Some(overlay) = row.policy_overlay.as_ref() {
        lines.push(format!(
            "  policy_overlay: cause={cause} visible={visible} hidden={hidden} hidden_list_visible={list} underlying={under}",
            cause = match overlay.narrowing_cause {
                aureline_workspace::NarrowingCause::AdminPolicy => "admin_policy",
                aureline_workspace::NarrowingCause::TrustPolicy => "trust_policy",
                aureline_workspace::NarrowingCause::LicenseOrExportControl => "license_or_export_control",
                aureline_workspace::NarrowingCause::RemoteUnavailable => "remote_unavailable",
                aureline_workspace::NarrowingCause::IndexNotBuilt => "index_not_built",
                aureline_workspace::NarrowingCause::UserMuted => "user_muted",
            },
            visible = overlay.visible_member_count,
            hidden = overlay.hidden_member_count,
            list = overlay.hidden_member_list_visible,
            under = overlay.underlying_workset_ref,
        ));
    }
    let actions: Vec<&'static str> = row
        .offered_actions
        .iter()
        .map(|a: &SwitcherRowAction| a.as_str())
        .collect();
    lines.push(format!("  offered_actions: {}", actions.join(", ")));
    if let Some(note) = row.partial_index_note.as_deref() {
        lines.push(format!("  partial_index_note: {note}"));
    }
    lines
}

/// Renders a [`WorksetActivationPreview`] as deterministic plaintext lines.
pub fn render_activation_preview_lines(preview: &WorksetActivationPreview) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!(
        "[preview] {drift}: base={base} candidate={candidate}",
        drift = preview.scope_drift.as_str(),
        base = preview.base_workset_ref,
        candidate = preview.candidate_workset_ref,
    ));
    lines.push(format!(
        "  identity: same={same} base_scope={base_scope} candidate_scope={cand_scope}",
        same = preview.same_identity,
        base_scope = preview.base_stable_scope_id,
        cand_scope = preview.candidate_stable_scope_id,
    ));
    lines.push(format!(
        "  posture: base={base_label}/{base_readiness} candidate={cand_label}/{cand_readiness}",
        base_label = preview.base_portability_label.as_str(),
        base_readiness = preview.base_readiness.as_str(),
        cand_label = preview.candidate_portability_label.as_str(),
        cand_readiness = preview.candidate_readiness.as_str(),
    ));
    if !preview.root_additions.is_empty() {
        let entries: Vec<String> = preview
            .root_additions
            .iter()
            .map(|root| {
                format!(
                    "+{} ({})",
                    root.presentation_label
                        .as_deref()
                        .unwrap_or(root.root_ref.as_str()),
                    root.root_kind.as_str(),
                )
            })
            .collect();
        lines.push(format!("  additions: {}", entries.join(", ")));
    }
    if !preview.root_removals.is_empty() {
        let entries: Vec<String> = preview
            .root_removals
            .iter()
            .map(|root| {
                format!(
                    "-{} ({})",
                    root.presentation_label
                        .as_deref()
                        .unwrap_or(root.root_ref.as_str()),
                    root.root_kind.as_str(),
                )
            })
            .collect();
        lines.push(format!("  removals: {}", entries.join(", ")));
    }
    if preview.changes_portability {
        lines.push("  portability_changed: yes".to_string());
    }
    if preview.changes_readiness {
        lines.push("  readiness_changed: yes".to_string());
    }
    if let Some(note) = preview.explain_note.as_deref() {
        lines.push(format!("  note: {note}"));
    }
    lines
}

/// Renders a [`WorksetReopenParityPacket`] as deterministic plaintext lines.
pub fn render_reopen_parity_lines(packet: &WorksetReopenParityPacket) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!(
        "[reopen_parity] {name} ({label}) identity_preserved={preserved}",
        name = packet.workset_name,
        label = packet.portability_label.as_str(),
        preserved = packet.identity_preserved_across_consumers,
    ));
    lines.push(format!(
        "  scope_identity: workset_ref={workset_ref} stable_scope_id={scope}",
        workset_ref = packet.workset_ref,
        scope = packet.stable_scope_id,
    ));
    for binding in &packet.bindings {
        let reopen = match binding.reopen_state {
            aureline_workspace::ScopeReopenState::Exact => "exact".to_string(),
            aureline_workspace::ScopeReopenState::Degraded => {
                let reason = binding
                    .degraded_reason
                    .map(|r| r.as_str())
                    .unwrap_or("(no_reason)");
                format!("degraded[{reason}]")
            }
        };
        lines.push(format!(
            "  binding[{consumer}] -> {reopen}",
            consumer = binding.consumer_class.as_str()
        ));
    }
    for entry in &packet.degraded {
        lines.push(render_downgrade_line(entry));
    }
    lines
}

fn render_downgrade_line(entry: &ReopenParityDowngrade) -> String {
    format!(
        "  downgrade: {consumer} -> {reason} ({note})",
        consumer = entry.consumer_class.as_str(),
        reason = entry.reason.as_str(),
        note = entry.note,
    )
}

/// Renders a [`WorksetSwitcherBetaSupportExport`] bundle as deterministic
/// plaintext.
pub fn render_support_export_bundle_lines(
    bundle: &WorksetSwitcherBetaSupportExport,
) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!(
        "[support_export_bundle] workspace={ws} switcher={sw}",
        ws = bundle.switcher.workspace_ref,
        sw = bundle.switcher.switcher_id,
    ));
    lines.push(String::new());
    lines.extend(render_switcher_beta_lines(&bundle.switcher));
    for preview in &bundle.activation_previews {
        lines.push(String::new());
        lines.extend(render_activation_preview_lines(preview));
    }
    for parity in &bundle.reopen_parity_packets {
        lines.push(String::new());
        lines.extend(render_reopen_parity_lines(parity));
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_workspace::{
        project_switcher_record, IncludedRootRef, MemberRef, MemberRefKind, MembershipPolicy,
        PartialTruthLabel, PortabilityMetadata, ReadinessMetadata, ReadinessState, ScopeClass,
        ScopeDegradedReason, ScopeMode, ScopeReopenPosture, SourceClass, WorkspaceRootKind,
        WorksetArtifactRecord, WorksetArtifactRecordKind, WorksetPortabilityClass,
        WORKSET_SWITCHER_BETA_SCHEMA_VERSION,
    };

    fn fixture_artifact() -> WorksetArtifactRecord {
        WorksetArtifactRecord {
            record_kind: WorksetArtifactRecordKind::WorksetArtifactRecord,
            workset_artifact_schema_version: 1,
            workset_id: "wks:shell:beta:portable".to_string(),
            scope_id: Some("scope:shell:beta:portable".to_string()),
            workset_name: "Shell portable".to_string(),
            presentation_subtitle: None,
            scope_class: ScopeClass::SelectedWorkset,
            scope_mode: ScopeMode::Full,
            workspace_ref: Some("wksp:shell:beta".to_string()),
            root_refs: vec!["fs-r-0".to_string(), "fs-r-1".to_string()],
            included_roots: vec![
                IncludedRootRef {
                    root_ref: "fs-r-0".to_string(),
                    root_kind: WorkspaceRootKind::LocalRepoRoot,
                    partial_truth: PartialTruthLabel::Loaded,
                    presentation_label: Some("repo-a".to_string()),
                },
                IncludedRootRef {
                    root_ref: "fs-r-1".to_string(),
                    root_kind: WorkspaceRootKind::LocalRepoRoot,
                    partial_truth: PartialTruthLabel::Loaded,
                    presentation_label: Some("repo-b".to_string()),
                },
            ],
            patterns: vec![],
            membership_policy: MembershipPolicy::ExplicitRootList,
            member_refs: vec![
                MemberRef {
                    ref_kind: MemberRefKind::Root,
                    ref_id: "fs-r-0".to_string(),
                    partial_truth: PartialTruthLabel::Loaded,
                    presentation_label: None,
                },
                MemberRef {
                    ref_kind: MemberRefKind::Root,
                    ref_id: "fs-r-1".to_string(),
                    partial_truth: PartialTruthLabel::Loaded,
                    presentation_label: None,
                },
            ],
            policy_limitation: None,
            portability: PortabilityMetadata {
                source_class: SourceClass::WorkspaceShared,
                portability_class: WorksetPortabilityClass::PortableWithRebinding,
                includes_machine_local_refs: false,
                includes_managed_provider_refs: false,
                requires_rebinding_on_import: true,
                profile_sync_group_ref: None,
            },
            readiness: ReadinessMetadata {
                readiness_state: ReadinessState::Ready,
                hidden_result_count_known: true,
                hidden_result_count: Some(0),
                partial_index_note: None,
            },
            parent_workset_ref: None,
            manifest_source_ref: None,
            created_at: "mono:0".to_string(),
            updated_at: "mono:0".to_string(),
            notes: None,
        }
    }

    #[test]
    fn switcher_beta_lines_quote_identity_and_roots() {
        let artifact = fixture_artifact();
        let switcher = project_switcher_record(
            "switcher:shell:beta",
            "wksp:shell:beta",
            &artifact.workset_id,
            &[artifact.clone()],
            "mono:test",
        );
        let lines = render_switcher_beta_lines(&switcher);
        assert!(lines[0].starts_with("[switcher]"));
        assert!(lines.iter().any(|l| l.contains("repo-a (local_repo_root, loaded)")));
        assert!(lines.iter().any(|l| l.contains("ACTIVE")));
        assert!(lines.iter().any(|l| l.contains("portable_with_rebinding")));
    }

    #[test]
    fn reopen_parity_lines_quote_each_consumer_state() {
        let artifact = fixture_artifact();
        let packet = artifact.project_reopen_parity_packet(
            "parity:shell",
            ScopeReopenPosture::Exact,
            ScopeReopenPosture::Degraded(ScopeDegradedReason::RebindingRequired),
            ScopeReopenPosture::Exact,
            "mono:test",
        );
        let lines = render_reopen_parity_lines(&packet);
        assert!(lines[0].starts_with("[reopen_parity]"));
        assert!(lines.iter().any(|l| l.contains("binding[local_ui] -> exact")));
        assert!(lines
            .iter()
            .any(|l| l.contains("binding[remote_ui] -> degraded[rebinding_required]")));
        assert!(lines.iter().any(|l| l.contains("binding[headless] -> exact")));
        assert!(lines.iter().any(|l| l.contains("downgrade: remote_ui -> rebinding_required")));
    }

    #[test]
    fn activation_preview_lines_quote_additions_and_drift() {
        let artifact = fixture_artifact();
        let mut candidate = artifact.clone();
        candidate.workset_id = "wks:shell:beta:candidate".to_string();
        candidate.scope_id = Some("scope:shell:beta:candidate".to_string());
        candidate.workset_name = "Shell candidate".to_string();
        candidate.root_refs.push("fs-r-2".to_string());
        candidate.included_roots.push(IncludedRootRef {
            root_ref: "fs-r-2".to_string(),
            root_kind: WorkspaceRootKind::LocalRepoRoot,
            partial_truth: PartialTruthLabel::Loaded,
            presentation_label: Some("repo-c".to_string()),
        });
        candidate.member_refs.push(MemberRef {
            ref_kind: MemberRefKind::Root,
            ref_id: "fs-r-2".to_string(),
            partial_truth: PartialTruthLabel::Loaded,
            presentation_label: None,
        });
        let preview = artifact.project_activation_preview(
            &candidate,
            "preview:shell",
            "diff:shell",
            "mono:test",
        );
        let lines = render_activation_preview_lines(&preview);
        assert!(lines[0].starts_with("[preview] widens"));
        assert!(lines.iter().any(|l| l.contains("+repo-c")));
    }

    #[test]
    fn support_export_bundle_lines_compose_subrenders() {
        let artifact = fixture_artifact();
        let switcher = project_switcher_record(
            "switcher:shell:bundle",
            "wksp:shell:beta",
            &artifact.workset_id,
            &[artifact.clone()],
            "mono:test",
        );
        let parity = artifact.project_reopen_parity_packet(
            "parity:bundle",
            ScopeReopenPosture::Exact,
            ScopeReopenPosture::Exact,
            ScopeReopenPosture::Exact,
            "mono:test",
        );
        let bundle = WorksetSwitcherBetaSupportExport {
            record_kind: WorksetSwitcherBetaSupportExport::RECORD_KIND.to_string(),
            schema_version: WORKSET_SWITCHER_BETA_SCHEMA_VERSION,
            switcher,
            activation_previews: vec![],
            reopen_parity_packets: vec![parity],
            emitted_at: "mono:test".to_string(),
        };
        let lines = render_support_export_bundle_lines(&bundle);
        assert!(lines[0].starts_with("[support_export_bundle]"));
        assert!(lines.iter().any(|l| l.starts_with("[switcher]")));
        assert!(lines.iter().any(|l| l.starts_with("[reopen_parity]")));
    }
}
