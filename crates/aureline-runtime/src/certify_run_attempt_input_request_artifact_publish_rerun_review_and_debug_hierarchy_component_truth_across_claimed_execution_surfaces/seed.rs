// Seed rows for the M05-827 execution surface certification packet. Included from
// `mod.rs` via `include!`, so this file shares the module scope and needs no imports.

fn all_labels() -> Vec<M5ExecutionRequiredLabel> {
    M5ExecutionRequiredLabel::ALL.to_vec()
}

fn all_export_fields() -> Vec<M5ExecutionCertExportField> {
    M5ExecutionCertExportField::ALL.to_vec()
}

fn copy_export() -> CertCopyExportParity {
    CertCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        screenshot_only_prohibited: true,
    }
}

/// The union of canonical families for the consumed groups, in matrix order.
fn families_for(groups: &[M5ExecutionComponentGroup]) -> Vec<M5ExecutionComponentFamily> {
    M5ExecutionComponentFamily::ALL
        .into_iter()
        .filter(|family| {
            groups
                .iter()
                .any(|group| group.families().contains(family))
        })
        .collect()
}

fn current(path: M5ExecutionPathClass) -> ExecutionPathCompatibility {
    ExecutionPathCompatibility {
        path_class: path,
        parity: M5ExecutionPathParityState::Current,
        note: String::new(),
    }
}

fn degraded(path: M5ExecutionPathClass, note: &str) -> ExecutionPathCompatibility {
    ExecutionPathCompatibility {
        path_class: path,
        parity: M5ExecutionPathParityState::DisclosedNarrowed,
        note: note.to_owned(),
    }
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:execution-surface-cert:{id}")]
}

#[allow(clippy::too_many_arguments)]
fn mk_row(
    row_id: &str,
    surface: M5ExecutionClaimedSurface,
    ctx: &str,
    groups: Vec<M5ExecutionComponentGroup>,
    declared: M5ExecutionInteractiveClaim,
    effective: M5ExecutionInteractiveClaim,
    run: RunAttemptTruthState,
    input: InputRequestTruthState,
    artifact: ArtifactPublishTruthState,
    rerun: RerunReviewTruthState,
    debug: DebugHierarchyTruthState,
    export: ClaimExportParityState,
    notes: Vec<ExecutionPathCompatibility>,
    narrow: Option<SurfaceClaimAutoNarrow>,
    source_ref: &str,
) -> ExecutionSurfaceCertRow {
    let consumer_families = families_for(&groups);
    ExecutionSurfaceCertRow {
        record_kind: EXECUTION_SURFACE_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: EXECUTION_SURFACE_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        claimed_surface: surface,
        source_matrix_ref: EXECUTION_SURFACE_CERT_COMPONENT_MATRIX_REF.to_owned(),
        certification_bundle_ref: EXECUTION_SURFACE_CERT_BUNDLE_REF.to_owned(),
        execution_context_ref: ctx.to_owned(),
        consumed_groups: groups,
        declared_claim: declared,
        effective_claim: effective,
        run_attempt_truth: run,
        input_request_truth: input,
        artifact_publish_truth: artifact,
        rerun_review_truth: rerun,
        debug_hierarchy_truth: debug,
        export_parity: export,
        compatibility_notes: notes,
        claim_auto_narrow: narrow,
        copy_export: copy_export(),
        export_fields: all_export_fields(),
        required_labels: all_labels(),
        consumer_families,
        source_refs: vec![source_ref.to_owned(), EXECUTION_SURFACE_CERT_DOC_REF.to_owned()],
        observed_at: "2026-07-04T00:00:00Z".to_owned(),
        evidence_refs: ev(row_id),
    }
}

fn narrow(
    to: M5ExecutionInteractiveClaim,
    group: M5ExecutionComponentGroup,
    label: &str,
) -> Option<SurfaceClaimAutoNarrow> {
    Some(SurfaceClaimAutoNarrow {
        narrowed_to: to,
        binding_group: group,
        trigger: group.default_trigger(),
        narrowed_label: label.to_owned(),
        preserves_component_identity: true,
    })
}

fn seeded_rows() -> Vec<ExecutionSurfaceCertRow> {
    use M5ExecutionClaimedSurface as S;
    use M5ExecutionComponentGroup as G;
    use M5ExecutionInteractiveClaim as C;
    use M5ExecutionPathClass as P;

    vec![
        // Task execution — consumes every group; all axes certified, full control on
        // the local and remote paths (green).
        mk_row(
            "cert:task-execution",
            S::TaskExecution,
            "run:task:0001",
            vec![
                G::RunAttempt,
                G::InputRequest,
                G::ArtifactPublish,
                G::RerunReview,
                G::DebugHierarchy,
            ],
            C::FullInteractive,
            C::FullInteractive,
            RunAttemptTruthState::Certified,
            InputRequestTruthState::Certified,
            ArtifactPublishTruthState::Certified,
            RerunReviewTruthState::Certified,
            DebugHierarchyTruthState::Certified,
            ClaimExportParityState::Certified,
            vec![current(P::Local), current(P::Remote)],
            None,
            "UI/UX Spec §14.4",
        ),
        // Test execution — run/artifact/rerun/debug certified; no input requests
        // (green).
        mk_row(
            "cert:test-execution",
            S::TestExecution,
            "run:test:0002",
            vec![G::RunAttempt, G::ArtifactPublish, G::RerunReview, G::DebugHierarchy],
            C::FullInteractive,
            C::FullInteractive,
            RunAttemptTruthState::Certified,
            InputRequestTruthState::NotApplicable,
            ArtifactPublishTruthState::Certified,
            RerunReviewTruthState::Certified,
            DebugHierarchyTruthState::Certified,
            ClaimExportParityState::Certified,
            vec![current(P::Local), current(P::Container)],
            None,
            "TDD §9.20",
        ),
        // Notebook execution — run/input/artifact/debug certified; no rerun sheet
        // (green).
        mk_row(
            "cert:notebook-execution",
            S::NotebookExecution,
            "run:notebook:0003",
            vec![G::RunAttempt, G::InputRequest, G::ArtifactPublish, G::DebugHierarchy],
            C::FullInteractive,
            C::FullInteractive,
            RunAttemptTruthState::Certified,
            InputRequestTruthState::Certified,
            ArtifactPublishTruthState::Certified,
            RerunReviewTruthState::NotApplicable,
            DebugHierarchyTruthState::Certified,
            ClaimExportParityState::Certified,
            vec![current(P::Local)],
            None,
            "TDD §8.54",
        ),
        // Publish execution — run/artifact/rerun certified on local and managed paths
        // (green).
        mk_row(
            "cert:publish-execution",
            S::PublishExecution,
            "run:publish:0004",
            vec![G::RunAttempt, G::ArtifactPublish, G::RerunReview],
            C::FullInteractive,
            C::FullInteractive,
            RunAttemptTruthState::Certified,
            InputRequestTruthState::NotApplicable,
            ArtifactPublishTruthState::Certified,
            RerunReviewTruthState::Certified,
            DebugHierarchyTruthState::NotApplicable,
            ClaimExportParityState::Certified,
            vec![current(P::Local), current(P::Managed)],
            None,
            "TDD §8.32",
        ),
        // Request execution — the remote replay context drifted from the recorded
        // attempt, so the rerun review narrows control to review-required (yellow).
        mk_row(
            "cert:request-execution",
            S::RequestExecution,
            "run:request:0005",
            vec![G::RunAttempt, G::InputRequest, G::ArtifactPublish, G::RerunReview],
            C::FullInteractive,
            C::ReviewRequired,
            RunAttemptTruthState::Certified,
            InputRequestTruthState::Certified,
            ArtifactPublishTruthState::Certified,
            RerunReviewTruthState::DisclosedNarrowed,
            DebugHierarchyTruthState::NotApplicable,
            ClaimExportParityState::Certified,
            vec![
                current(P::Local),
                degraded(
                    P::Remote,
                    "Remote request replay context drifted from the recorded attempt; rerun gated behind review",
                ),
            ],
            narrow(
                C::ReviewRequired,
                G::RerunReview,
                "Remote request context drifted — rerun gated behind context review",
            ),
            "TDD §9.21",
        ),
        // Database execution — managed warehouse result retention expired, so the
        // artifact-publish row narrows to read-only (yellow).
        mk_row(
            "cert:database-execution",
            S::DatabaseExecution,
            "run:database:0006",
            vec![G::RunAttempt, G::InputRequest, G::ArtifactPublish],
            C::FullInteractive,
            C::ReadOnly,
            RunAttemptTruthState::Certified,
            InputRequestTruthState::Certified,
            ArtifactPublishTruthState::DisclosedNarrowed,
            RerunReviewTruthState::NotApplicable,
            DebugHierarchyTruthState::NotApplicable,
            ClaimExportParityState::Certified,
            vec![
                current(P::Local),
                degraded(
                    P::Managed,
                    "Managed warehouse result retention expired; lineage copyable, re-open disabled",
                ),
            ],
            narrow(
                C::ReadOnly,
                G::ArtifactPublish,
                "Managed result retention expired — lineage copyable, re-open disabled",
            ),
            "TDD §8.32",
        ),
        // AI-mediated execution — the provider-backed approval consequence is deferred,
        // so the input-request prompt narrows to review-required (yellow).
        mk_row(
            "cert:ai-execution",
            S::AiExecution,
            "run:ai:0007",
            vec![
                G::RunAttempt,
                G::InputRequest,
                G::ArtifactPublish,
                G::RerunReview,
                G::DebugHierarchy,
            ],
            C::FullInteractive,
            C::ReviewRequired,
            RunAttemptTruthState::Certified,
            InputRequestTruthState::DisclosedNarrowed,
            ArtifactPublishTruthState::Certified,
            RerunReviewTruthState::Certified,
            DebugHierarchyTruthState::Certified,
            ClaimExportParityState::Certified,
            vec![
                current(P::Local),
                degraded(
                    P::ProviderBacked,
                    "Provider-backed approval consequence deferred; answer gated behind review",
                ),
            ],
            narrow(
                C::ReviewRequired,
                G::InputRequest,
                "Provider-backed approval consequence deferred — answer gated behind review",
            ),
            "UI/UX Spec §14.4",
        ),
        // Preview execution — the container preview build is stale, so the
        // artifact-publish row narrows to read-only (yellow).
        mk_row(
            "cert:preview-execution",
            S::PreviewExecution,
            "run:preview:0008",
            vec![G::RunAttempt, G::ArtifactPublish],
            C::FullInteractive,
            C::ReadOnly,
            RunAttemptTruthState::Certified,
            InputRequestTruthState::NotApplicable,
            ArtifactPublishTruthState::DisclosedNarrowed,
            RerunReviewTruthState::NotApplicable,
            DebugHierarchyTruthState::NotApplicable,
            ClaimExportParityState::Certified,
            vec![
                current(P::Local),
                degraded(
                    P::Container,
                    "Container preview build stale; rendered artifact read-only pending refresh",
                ),
            ],
            narrow(
                C::ReadOnly,
                G::ArtifactPublish,
                "Container preview build stale — artifact read-only pending refresh",
            ),
            "TDD §8.54",
        ),
        // Debug execution — the remote debug connector dropped, so the debug hierarchy
        // narrows to inspect-only captured evidence (yellow).
        mk_row(
            "cert:debug-execution",
            S::DebugExecution,
            "run:debug:0009",
            vec![G::RunAttempt, G::RerunReview, G::DebugHierarchy],
            C::FullInteractive,
            C::InspectOnly,
            RunAttemptTruthState::Certified,
            InputRequestTruthState::NotApplicable,
            ArtifactPublishTruthState::NotApplicable,
            RerunReviewTruthState::Certified,
            DebugHierarchyTruthState::DisclosedNarrowed,
            ClaimExportParityState::Certified,
            vec![
                current(P::Local),
                degraded(
                    P::Remote,
                    "Remote debug connector dropped; thread/process hierarchy captured, inspect-only",
                ),
            ],
            narrow(
                C::InspectOnly,
                G::DebugHierarchy,
                "Remote debug connector dropped — hierarchy captured, inspect-only",
            ),
            "UI/UX Spec §14.5",
        ),
        // Support / export replay (evidence) — replays every group's certified truth,
        // read-only by nature (green).
        mk_row(
            "cert:support-export-replay",
            S::SupportExportReplay,
            "run:support:0010",
            vec![
                G::RunAttempt,
                G::InputRequest,
                G::ArtifactPublish,
                G::RerunReview,
                G::DebugHierarchy,
            ],
            C::ReadOnly,
            C::ReadOnly,
            RunAttemptTruthState::Certified,
            InputRequestTruthState::Certified,
            ArtifactPublishTruthState::Certified,
            RerunReviewTruthState::Certified,
            DebugHierarchyTruthState::Certified,
            ClaimExportParityState::Certified,
            vec![current(P::Local)],
            None,
            "TAD supportability architecture",
        ),
        // Docs / help embeds (evidence) — run/rerun certified, inspect-only (green).
        mk_row(
            "cert:docs-help-embeds",
            S::DocsHelpEmbeds,
            "run:docs:0011",
            vec![G::RunAttempt, G::RerunReview],
            C::InspectOnly,
            C::InspectOnly,
            RunAttemptTruthState::Certified,
            InputRequestTruthState::NotApplicable,
            ArtifactPublishTruthState::NotApplicable,
            RerunReviewTruthState::Certified,
            DebugHierarchyTruthState::NotApplicable,
            ClaimExportParityState::Certified,
            vec![current(P::Local)],
            None,
            "Milestones v3.1 field-readiness controls",
        ),
        // Release proof (evidence) — run/artifact/debug certified, read-only (green).
        mk_row(
            "cert:release-proof",
            S::ReleaseProof,
            "run:release:0012",
            vec![G::RunAttempt, G::ArtifactPublish, G::DebugHierarchy],
            C::ReadOnly,
            C::ReadOnly,
            RunAttemptTruthState::Certified,
            InputRequestTruthState::NotApplicable,
            ArtifactPublishTruthState::Certified,
            RerunReviewTruthState::NotApplicable,
            DebugHierarchyTruthState::Certified,
            ClaimExportParityState::Certified,
            vec![current(P::Local)],
            None,
            "Milestones v3.1 durable progress truth",
        ),
    ]
}
