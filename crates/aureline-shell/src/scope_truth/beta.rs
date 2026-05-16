//! Shell-facing beta scope-truth projection.
//!
//! The chrome consumes [`WorksetScopeBetaTruth`] verbatim — admission rows,
//! lineage entries, and excluded-root entries are quoted directly so a search,
//! graph, refactor, AI, or export surface never re-mints its own scope
//! vocabulary. This module owns one thin projection helper that turns a beta
//! truth into a small set of human-readable lines for support packets, review
//! sheets, and log surfaces.

use aureline_workspace::{
    BroadActionAdmission, BroadActionDecision, ScopeLineageEntry, WorksetScopeBetaSupportExport,
    WorksetScopeBetaTruth,
};

/// Renders a [`WorksetScopeBetaTruth`] as a small set of human-readable lines
/// matched to the surfaces that already render [`super::card::ScopeTruthChipCard`].
/// Returns the lines in render order; the caller decides whether to indent or
/// wrap.
pub fn render_beta_truth_lines(truth: &WorksetScopeBetaTruth) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!(
        "[{surface}] {scope_class} · {workset_name}",
        surface = truth.consumer_surface.as_str(),
        scope_class = truth.scope_class.as_str(),
        workset_name = truth.workset_name,
    ));
    lines.push(format!(
        "scope_identity: {id}   mode: {mode}",
        id = truth.stable_scope_id,
        mode = truth.scope_mode.as_str(),
    ));
    if !truth.included_roots.is_empty() {
        let roots: Vec<String> = truth
            .included_roots
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
        lines.push(format!("included_roots: {}", roots.join(", ")));
    }
    if !truth.excluded_roots.is_empty() {
        let excluded: Vec<String> = truth
            .excluded_roots
            .iter()
            .map(|entry| {
                let label = entry
                    .presentation_label
                    .as_deref()
                    .unwrap_or(entry.root_ref.as_str());
                format!("{label} [{}]", entry.reason.as_str())
            })
            .collect();
        lines.push(format!("excluded_roots: {}", excluded.join(", ")));
    }
    if !truth.include_patterns.is_empty() {
        lines.push(format!(
            "include_patterns: {}",
            truth.include_patterns.join(", ")
        ));
    }
    if !truth.exclude_patterns.is_empty() {
        lines.push(format!(
            "exclude_patterns: {}",
            truth.exclude_patterns.join(", ")
        ));
    }
    for admission in &truth.admissions {
        lines.push(render_admission_line(admission));
    }
    if !truth.lineage.is_empty() {
        lines.push(format!(
            "lineage: {}",
            truth
                .lineage
                .iter()
                .map(render_lineage_entry)
                .collect::<Vec<_>>()
                .join(" -> ")
        ));
    }
    if truth.outside_current_scope_marker_visible {
        lines.push("marker: outside_current_scope".to_string());
    }
    if let Some(note) = truth.explain_note.as_deref() {
        lines.push(format!("note: {note}"));
    }
    lines
}

/// Renders a [`WorksetScopeBetaSupportExport`] as a header line followed by
/// the per-truth render. Triage surfaces consume this verbatim instead of
/// re-deriving scope from the bundled JSON.
pub fn render_support_export_lines(packet: &WorksetScopeBetaSupportExport) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!(
        "[support_export] {scope_class} · {workset_name}",
        scope_class = packet.artifact_scope_class.as_str(),
        workset_name = packet.artifact_workset_name,
    ));
    lines.push(format!(
        "artifact_workset_ref: {}   stable_scope_id: {}",
        packet.artifact_workset_ref, packet.artifact_stable_scope_id,
    ));
    lines.push(format!(
        "lineage: {}",
        packet
            .lineage
            .iter()
            .map(render_lineage_entry)
            .collect::<Vec<_>>()
            .join(" -> ")
    ));
    for truth in &packet.truths {
        lines.push(String::new());
        lines.extend(render_beta_truth_lines(truth));
    }
    lines
}

fn render_admission_line(admission: &BroadActionAdmission) -> String {
    let reason = admission
        .reason
        .map(|r| r.as_str())
        .unwrap_or("(no_reason)");
    let mut line = format!(
        "admission: {action} -> {decision}",
        action = admission.action_class.as_str(),
        decision = admission.decision.as_str(),
    );
    if admission.decision != BroadActionDecision::Allowed {
        line.push_str(&format!("   reason: {reason}"));
    }
    if let Some(note) = admission.explain_note.as_deref() {
        line.push_str(&format!("   note: {note}"));
    }
    line
}

fn render_lineage_entry(entry: &ScopeLineageEntry) -> String {
    let label = entry
        .presentation_label
        .as_deref()
        .unwrap_or(entry.workset_ref.as_str());
    let cause = entry
        .narrowing_cause
        .map(|c| match c {
            aureline_workspace::NarrowingCause::AdminPolicy => "admin_policy",
            aureline_workspace::NarrowingCause::TrustPolicy => "trust_policy",
            aureline_workspace::NarrowingCause::LicenseOrExportControl => {
                "license_or_export_control"
            }
            aureline_workspace::NarrowingCause::RemoteUnavailable => "remote_unavailable",
            aureline_workspace::NarrowingCause::IndexNotBuilt => "index_not_built",
            aureline_workspace::NarrowingCause::UserMuted => "user_muted",
        })
        .unwrap_or("(no_narrowing)");
    format!(
        "{label} [{class}, {readiness}, {portability}, {cause}]",
        class = entry.scope_class.as_str(),
        readiness = entry.readiness_state.as_str(),
        portability = portability_token(entry.portability_class),
    )
}

const fn portability_token(class: aureline_workspace::WorksetPortabilityClass) -> &'static str {
    match class {
        aureline_workspace::WorksetPortabilityClass::FullyPortable => "fully_portable",
        aureline_workspace::WorksetPortabilityClass::PortableWithRebinding => {
            "portable_with_rebinding"
        }
        aureline_workspace::WorksetPortabilityClass::MachineLocalOnly => "machine_local_only",
        aureline_workspace::WorksetPortabilityClass::ManagedProviderLocked => {
            "managed_provider_locked"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_workspace::{
        BetaConsumerSurface, BroadActionClass, IncludedRootRef, MemberRef, MemberRefKind,
        MembershipPolicy, PartialTruthLabel, PortabilityMetadata, ReadinessMetadata,
        ReadinessState, ScopeClass, ScopeMode, ScopeObservationInputs, SourceClass,
        WorksetArtifactRecord, WorksetArtifactRecordKind, WorksetPortabilityClass,
        WorksetScopeBetaSupportExport, WorkspaceRootKind,
    };

    fn sparse_artifact() -> WorksetArtifactRecord {
        WorksetArtifactRecord {
            record_kind: WorksetArtifactRecordKind::WorksetArtifactRecord,
            workset_artifact_schema_version: 1,
            workset_id: "wks:beta:shell:sparse".to_string(),
            scope_id: Some("scope:beta:shell:sparse".to_string()),
            workset_name: "Frontend sparse slice".to_string(),
            presentation_subtitle: None,
            scope_class: ScopeClass::SparseSlice,
            scope_mode: ScopeMode::Sparse,
            workspace_ref: Some("wksp:test".to_string()),
            root_refs: vec!["fs-r-0".to_string()],
            included_roots: vec![IncludedRootRef {
                root_ref: "fs-r-0".to_string(),
                root_kind: WorkspaceRootKind::LocalRepoRoot,
                partial_truth: PartialTruthLabel::ManifestKnown,
                presentation_label: Some("acme-frontend".to_string()),
            }],
            patterns: vec![],
            membership_policy: MembershipPolicy::ExplicitRootList,
            member_refs: vec![MemberRef {
                ref_kind: MemberRefKind::Root,
                ref_id: "fs-r-0".to_string(),
                partial_truth: PartialTruthLabel::ManifestKnown,
                presentation_label: Some("acme-frontend".to_string()),
            }],
            policy_limitation: None,
            portability: PortabilityMetadata {
                source_class: SourceClass::LocalOnly,
                portability_class: WorksetPortabilityClass::PortableWithRebinding,
                includes_machine_local_refs: true,
                includes_managed_provider_refs: false,
                requires_rebinding_on_import: true,
                profile_sync_group_ref: None,
            },
            readiness: ReadinessMetadata {
                readiness_state: ReadinessState::Partial,
                hidden_result_count_known: true,
                hidden_result_count: Some(99),
                partial_index_note: None,
            },
            parent_workset_ref: None,
            manifest_source_ref: None,
            created_at: "mono:0".to_string(),
            updated_at: "mono:1".to_string(),
            notes: None,
        }
    }

    #[test]
    fn beta_truth_lines_quote_admissions_and_lineage() {
        let artifact = sparse_artifact();
        let workspace_roots = vec!["fs-r-0".to_string(), "fs-r-backend".to_string()];
        let truth = artifact.project_beta_truth(
            BetaConsumerSurface::Refactor,
            ScopeObservationInputs {
                workspace_root_refs: &workspace_roots,
                workspace_root_labels: &[],
                parent_artifact: None,
            },
            "mono:beta:shell:1",
        );
        let lines = render_beta_truth_lines(&truth);
        assert!(lines[0].starts_with("[refactor] sparse_slice"));
        let admission_lines: Vec<&String> = lines
            .iter()
            .filter(|l| l.starts_with("admission:"))
            .collect();
        assert!(admission_lines
            .iter()
            .any(|line| line.contains("refactor_apply -> narrowed_to_scope")));
        let excluded_line = lines
            .iter()
            .find(|l| l.starts_with("excluded_roots:"))
            .expect("excluded line must render");
        assert!(excluded_line.contains("not_in_workset_root_list"));
        assert!(lines.iter().any(|l| l.starts_with("lineage:")));
        // Ensure we cover every admission class on the projection lines.
        for action in BroadActionClass::all() {
            assert!(
                admission_lines
                    .iter()
                    .any(|line| line.contains(action.as_str())),
                "missing admission line for {action:?}"
            );
        }
    }

    #[test]
    fn support_export_render_includes_packet_header_and_each_truth() {
        let artifact = sparse_artifact();
        let workspace_roots = vec!["fs-r-0".to_string()];
        let inputs = || ScopeObservationInputs {
            workspace_root_refs: &workspace_roots,
            workspace_root_labels: &[],
            parent_artifact: None,
        };
        let truths = vec![
            artifact.project_beta_truth(BetaConsumerSurface::Search, inputs(), "mono:1"),
            artifact.project_beta_truth(BetaConsumerSurface::Refactor, inputs(), "mono:2"),
        ];
        let packet = WorksetScopeBetaSupportExport::from_truths(truths, "mono:3")
            .expect("packet must validate");
        let lines = render_support_export_lines(&packet);
        assert!(lines[0].starts_with("[support_export] sparse_slice"));
        assert!(lines.iter().any(|l| l.starts_with("[search] sparse_slice")));
        assert!(lines
            .iter()
            .any(|l| l.starts_with("[refactor] sparse_slice")));
    }
}
