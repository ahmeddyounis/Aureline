// Seed rows for the M05-851 workflow-bundle surface certification packet. Included
// from `mod.rs` via `include!`, so this file shares the module scope and needs no
// imports.

fn all_labels() -> Vec<M5BundleRequiredLabel> {
    M5BundleRequiredLabel::ALL.to_vec()
}

fn all_export_fields() -> Vec<M5BundleCertExportField> {
    M5BundleCertExportField::ALL.to_vec()
}

fn copy_export() -> CertCopyExportParity {
    CertCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        screenshot_only_prohibited: true,
    }
}

/// The union of canonical families for the consumed groups, in matrix order.
fn families_for(groups: &[M5WorkflowBundleComponentGroup]) -> Vec<M5WorkflowBundleComponentFamily> {
    M5WorkflowBundleComponentFamily::ALL
        .into_iter()
        .filter(|family| {
            groups
                .iter()
                .any(|group| group.families().contains(family))
        })
        .collect()
}

fn current(path: M5BundleDistributionPath) -> BundleDistributionCompatibility {
    BundleDistributionCompatibility {
        path,
        parity: M5BundleDistributionParityState::Current,
        note: String::new(),
    }
}

fn degraded(path: M5BundleDistributionPath, note: &str) -> BundleDistributionCompatibility {
    BundleDistributionCompatibility {
        path,
        parity: M5BundleDistributionParityState::DisclosedNarrowed,
        note: note.to_owned(),
    }
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:bundle-surface-cert:{id}")]
}

#[allow(clippy::too_many_arguments)]
fn mk_row(
    row_id: &str,
    surface: M5WorkflowBundleClaimedSurface,
    ctx: &str,
    groups: Vec<M5WorkflowBundleComponentGroup>,
    declared: M5BundleSupportClaim,
    effective: M5BundleSupportClaim,
    launch: LaunchWedgeTruthState,
    detail: DetailReviewTruthState,
    drift: DriftOverrideTruthState,
    rollback: RollbackRemoveTruthState,
    class: ClassDisclosureTruthState,
    export: ClaimExportParityState,
    notes: Vec<BundleDistributionCompatibility>,
    narrow: Option<SurfaceClaimAutoNarrow>,
    source_ref: &str,
) -> BundleSurfaceCertRow {
    let consumer_families = families_for(&groups);
    BundleSurfaceCertRow {
        record_kind: BUNDLE_SURFACE_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: BUNDLE_SURFACE_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        claimed_surface: surface,
        source_matrix_ref: BUNDLE_SURFACE_CERT_COMPONENT_MATRIX_REF.to_owned(),
        certification_bundle_ref: BUNDLE_SURFACE_CERT_BUNDLE_REF.to_owned(),
        bundle_context_ref: ctx.to_owned(),
        consumed_groups: groups,
        declared_claim: declared,
        effective_claim: effective,
        launch_wedge_truth: launch,
        detail_review_truth: detail,
        drift_override_truth: drift,
        rollback_remove_truth: rollback,
        class_disclosure_truth: class,
        export_parity: export,
        compatibility_notes: notes,
        claim_auto_narrow: narrow,
        copy_export: copy_export(),
        export_fields: all_export_fields(),
        required_labels: all_labels(),
        consumer_families,
        source_refs: vec![source_ref.to_owned(), BUNDLE_SURFACE_CERT_DOC_REF.to_owned()],
        observed_at: "2026-07-06T00:00:00Z".to_owned(),
        evidence_refs: ev(row_id),
    }
}

fn narrow(
    to: M5BundleSupportClaim,
    group: M5WorkflowBundleComponentGroup,
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

fn seeded_rows() -> Vec<BundleSurfaceCertRow> {
    use ClassDisclosureTruthState as Class;
    use DetailReviewTruthState as Detail;
    use DriftOverrideTruthState as Drift;
    use LaunchWedgeTruthState as Launch;
    use M5BundleDistributionPath as P;
    use M5BundleSupportClaim as C;
    use M5WorkflowBundleClaimedSurface as S;
    use M5WorkflowBundleComponentGroup as G;
    use RollbackRemoveTruthState as Roll;

    vec![
        // Start-center picker — offers certified bundles; launch wedge, detail review,
        // and class disclosure all certified on native and managed paths (green).
        mk_row(
            "cert:start-center-picker",
            S::StartCenterPicker,
            "bundle:start-center:0001",
            vec![G::LaunchWedge, G::DetailReview, G::ClassDisclosure],
            C::Certified,
            C::Certified,
            Launch::Certified,
            Detail::Certified,
            Drift::NotApplicable,
            Roll::NotApplicable,
            Class::Certified,
            ClaimExportParityState::Certified,
            vec![current(P::Native), current(P::Managed)],
            None,
            "UI/UX Spec §6.15",
        ),
        // Onboarding flow — guided stack entry with a supported launch wedge and detail
        // review on the native path (green).
        mk_row(
            "cert:onboarding-flow",
            S::OnboardingFlow,
            "bundle:onboarding:0002",
            vec![G::LaunchWedge, G::DetailReview],
            C::Supported,
            C::Supported,
            Launch::Certified,
            Detail::Certified,
            Drift::NotApplicable,
            Roll::NotApplicable,
            Class::NotApplicable,
            ClaimExportParityState::Certified,
            vec![current(P::Native)],
            None,
            "UI/UX Spec §6.17",
        ),
        // Migration center — an imported-user handoff bundle narrows the class
        // disclosure to imported, so the surface claim narrows to imported (yellow).
        mk_row(
            "cert:migration-center",
            S::MigrationCenter,
            "bundle:migration:0003",
            vec![G::DetailReview, G::DriftOverride, G::ClassDisclosure],
            C::Supported,
            C::Imported,
            Launch::NotApplicable,
            Detail::Certified,
            Drift::Certified,
            Roll::NotApplicable,
            Class::DisclosedNarrowed,
            ClaimExportParityState::Certified,
            vec![
                current(P::Native),
                degraded(
                    P::Imported,
                    "Imported-user handoff bundle is bridged, not a native first-party read; class narrows to imported",
                ),
            ],
            narrow(
                C::Imported,
                G::ClassDisclosure,
                "Imported-user handoff bundle — class disclosed as imported, not native parity",
            ),
            "UI/UX Spec §23.49",
        ),
        // Docs / help center — supported launch wedge and class disclosure current on
        // the native and mirror paths (green).
        mk_row(
            "cert:docs-help",
            S::DocsHelp,
            "bundle:docs-help:0004",
            vec![G::LaunchWedge, G::ClassDisclosure],
            C::Supported,
            C::Supported,
            Launch::Certified,
            Detail::NotApplicable,
            Drift::NotApplicable,
            Roll::NotApplicable,
            Class::Certified,
            ClaimExportParityState::Certified,
            vec![current(P::Native), current(P::Mirror)],
            None,
            "UX Guide §16.20",
        ),
        // Diagnostics — a local override drifted from the bundle, so the drift/override
        // group narrows the surface claim to limited (yellow).
        mk_row(
            "cert:diagnostics",
            S::Diagnostics,
            "bundle:diagnostics:0005",
            vec![G::DetailReview, G::DriftOverride, G::RollbackRemove],
            C::Supported,
            C::Limited,
            Launch::NotApplicable,
            Detail::Certified,
            Drift::DisclosedNarrowed,
            Roll::Certified,
            Class::NotApplicable,
            ClaimExportParityState::Certified,
            vec![current(P::Native)],
            narrow(
                C::Limited,
                G::DriftOverride,
                "Local override drifted from the bundle — support limited pending resolve",
            ),
            "TDD §12.3",
        ),
        // CLI / headless — the managed entitlement plane is unresolved, so the detail /
        // review group narrows the surface claim to limited (yellow).
        mk_row(
            "cert:cli-headless",
            S::CliHeadless,
            "bundle:cli-headless:0006",
            vec![G::LaunchWedge, G::DetailReview],
            C::Supported,
            C::Limited,
            Launch::Certified,
            Detail::DisclosedNarrowed,
            Drift::NotApplicable,
            Roll::NotApplicable,
            Class::NotApplicable,
            ClaimExportParityState::Certified,
            vec![
                current(P::Native),
                degraded(
                    P::Managed,
                    "Managed entitlement dependency unresolved; install/update review reachable but gated",
                ),
            ],
            narrow(
                C::Limited,
                G::DetailReview,
                "Managed entitlement dependency unresolved — review reachable, install gated",
            ),
            "TDD §12.3",
        ),
        // Support / export replay (evidence) — replays every group's certified truth on
        // the native and offline paths, read-only by nature (green).
        mk_row(
            "cert:support-export-replay",
            S::SupportExportReplay,
            "bundle:support-replay:0007",
            vec![
                G::LaunchWedge,
                G::DetailReview,
                G::DriftOverride,
                G::RollbackRemove,
                G::ClassDisclosure,
            ],
            C::Supported,
            C::Supported,
            Launch::Certified,
            Detail::Certified,
            Drift::Certified,
            Roll::Certified,
            Class::Certified,
            ClaimExportParityState::Certified,
            vec![current(P::Native), current(P::Offline)],
            None,
            "TAD supportability architecture",
        ),
        // Docs / help embeds (evidence) — launch wedge and class disclosure certified,
        // native path (green).
        mk_row(
            "cert:docs-help-embeds",
            S::DocsHelpEmbeds,
            "bundle:docs-embeds:0008",
            vec![G::LaunchWedge, G::ClassDisclosure],
            C::Supported,
            C::Supported,
            Launch::Certified,
            Detail::NotApplicable,
            Drift::NotApplicable,
            Roll::NotApplicable,
            Class::Certified,
            ClaimExportParityState::Certified,
            vec![current(P::Native)],
            None,
            "Milestones v3.1 first-run truth",
        ),
        // Release proof (evidence) — detail review and rollback/remove certified, native
        // path (green).
        mk_row(
            "cert:release-proof",
            S::ReleaseProof,
            "bundle:release-proof:0009",
            vec![G::DetailReview, G::RollbackRemove],
            C::Supported,
            C::Supported,
            Launch::NotApplicable,
            Detail::Certified,
            Drift::NotApplicable,
            Roll::Certified,
            Class::NotApplicable,
            ClaimExportParityState::Certified,
            vec![current(P::Native)],
            None,
            "Milestones v3.1 workflow-bundle claim linkage",
        ),
    ]
}
