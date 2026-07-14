//! Canonical seed builders for the M5 portable-mode state-containment and diagnostics registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures. The
//! headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean layout and diagnostics entries prove the colocated
//! / named-sibling state layout, the full durable-root inventory, the absent-or-blocked hidden machine-global
//! mutation, the distinguishable portable-versus-installed state origin, the discoverable diagnostics, and the
//! documented retained-versus-replaced update continuity across the About, update, diagnostics, admin, docs,
//! and support surfaces without any hand-copied per-profile assumption, hidden spill, ambiguous origin,
//! undisclosed field, or presentation-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_PORTABLE_MODE_STATE_CONTAINMENT_AND_DIAGNOSTICS_PACKET_ID: &str =
    "m5-portable-mode-state-containment-and-diagnostics:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn layout(input: M5PortableStateLayoutEntryResolutionInput) -> M5ResolvedPortableStateLayoutEntry {
    resolve_portable_state_layout_entry(input).expect("seed portable-state-layout entry resolves")
}

fn diagnostics(
    input: M5PortableDiagnosticsEntryResolutionInput,
) -> M5ResolvedPortableDiagnosticsEntry {
    resolve_portable_diagnostics_entry(input).expect("seed portable-diagnostics entry resolves")
}

fn all_forms() -> Vec<M5PortablePresentationForm> {
    M5PortablePresentationForm::ALL.to_vec()
}

fn all_durable_classes() -> Vec<M5PortableDurableStateClass> {
    M5PortableDurableStateClass::ALL.to_vec()
}

fn all_diagnostics_fields() -> Vec<M5PortableDiagnosticsField> {
    M5PortableDiagnosticsField::ALL.to_vec()
}

// -- Clean portable-state-layout entries --------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_layout_base(
    entry_id: &str,
    profile_id: &str,
    token_name: &str,
    semantic_role: M5InstallTopologyRole,
    containment: M5PortableStateContainment,
    surface_context: M5PortableSurfaceContext,
    state_origin: M5PortableStateOrigin,
    executable_root: &str,
    colocated_state_root: &str,
    log_and_crash_root: &str,
) -> M5PortableStateLayoutEntryResolutionInput {
    M5PortableStateLayoutEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        profile_id: profile_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        containment,
        surface_context,
        presentation_form_coverage: all_forms(),
        executable_root: executable_root.to_owned(),
        colocated_state_root: colocated_state_root.to_owned(),
        log_and_crash_root: log_and_crash_root.to_owned(),
        durable_classes_covered: all_durable_classes(),
        state_origin,
        bound_to_registry: true,
        hidden_machine_global_write_used: false,
        hidden_machine_global_write_blocked: true,
        proof_fresh: true,
    }
}

fn layout_colocated_about_clean() -> M5ResolvedPortableStateLayoutEntry {
    layout(clean_layout_base(
        "layout:colocated:about",
        "profile.portable_colocated",
        "portable.layout.colocated",
        M5InstallTopologyRole::WritableStateRoots,
        M5PortableStateContainment::ColocatedUnderExecutable,
        M5PortableSurfaceContext::AboutSurface,
        M5PortableStateOrigin::PortableColocated,
        r".\AurelinePortable\app",
        r".\AurelinePortable\state",
        r".\AurelinePortable\logs",
    ))
}

fn layout_sibling_update_clean() -> M5ResolvedPortableStateLayoutEntry {
    layout(clean_layout_base(
        "layout:sibling:update",
        "profile.portable_named_sibling",
        "portable.layout.named_sibling",
        M5InstallTopologyRole::WritableStateRoots,
        M5PortableStateContainment::NamedSiblingDirectory,
        M5PortableSurfaceContext::UpdateFlow,
        M5PortableStateOrigin::PortableNamedSibling,
        r".\AurelinePortable\app",
        r".\Aureline-Portable-State",
        r".\Aureline-Portable-State\logs",
    ))
}

fn layout_colocated_diagnostics_clean() -> M5ResolvedPortableStateLayoutEntry {
    layout(clean_layout_base(
        "layout:colocated:diagnostics",
        "profile.portable_colocated",
        "portable.layout.colocated",
        M5InstallTopologyRole::PolicyRoots,
        M5PortableStateContainment::ColocatedUnderExecutable,
        M5PortableSurfaceContext::DiagnosticsSurface,
        M5PortableStateOrigin::PortableColocated,
        r".\AurelinePortable\app",
        r".\AurelinePortable\state",
        r".\AurelinePortable\logs",
    ))
}

fn layout_colocated_admin_clean() -> M5ResolvedPortableStateLayoutEntry {
    layout(clean_layout_base(
        "layout:colocated:admin",
        "profile.portable_colocated",
        "portable.layout.colocated",
        M5InstallTopologyRole::RollbackTarget,
        M5PortableStateContainment::ColocatedUnderExecutable,
        M5PortableSurfaceContext::AdminSurface,
        M5PortableStateOrigin::PortableColocated,
        r".\AurelinePortable\app",
        r".\AurelinePortable\state",
        r".\AurelinePortable\logs",
    ))
}

fn layout_sibling_support_clean() -> M5ResolvedPortableStateLayoutEntry {
    layout(clean_layout_base(
        "layout:sibling:support",
        "profile.portable_named_sibling",
        "portable.layout.named_sibling",
        M5InstallTopologyRole::WritableStateRoots,
        M5PortableStateContainment::NamedSiblingDirectory,
        M5PortableSurfaceContext::SupportOrExportForm,
        M5PortableStateOrigin::PortableNamedSibling,
        r".\AurelinePortable\app",
        r".\Aureline-Portable-State",
        r".\Aureline-Portable-State\logs",
    ))
}

// -- Degraded portable-state-layout entries -----------------------------------------------------

/// Degraded layout entry: the durable-root inventory is incomplete — shell hooks are not inventoried inside a
/// documented portable root.
fn layout_inventory_incomplete() -> M5ResolvedPortableStateLayoutEntry {
    let mut base = clean_layout_base(
        "layout:colocated:inventory-incomplete",
        "profile.portable_colocated",
        "portable.layout.colocated",
        M5InstallTopologyRole::WritableStateRoots,
        M5PortableStateContainment::ColocatedUnderExecutable,
        M5PortableSurfaceContext::AboutSurface,
        M5PortableStateOrigin::PortableColocated,
        r".\AurelinePortable\app",
        r".\AurelinePortable\state",
        r".\AurelinePortable\logs",
    );
    base.durable_classes_covered = vec![
        M5PortableDurableStateClass::DurableSettings,
        M5PortableDurableStateClass::StoredSecrets,
        M5PortableDurableStateClass::BackgroundServices,
        // ShellHooks is dropped: the inventory can no longer prove every durable root is contained.
    ];
    layout(base)
}

/// Degraded layout entry: portable mode wrote durable state into a hidden machine-global path (spill used and
/// not blocked) — the layout reads as contained when it is not.
fn layout_hidden_spill() -> M5ResolvedPortableStateLayoutEntry {
    let mut base = clean_layout_base(
        "layout:portable:hidden-spill",
        "profile.portable_named_sibling",
        "portable.layout.named_sibling",
        M5InstallTopologyRole::WritableStateRoots,
        M5PortableStateContainment::NamedSiblingDirectory,
        M5PortableSurfaceContext::UpdateFlow,
        M5PortableStateOrigin::PortableNamedSibling,
        r".\AurelinePortable\app",
        r".\Aureline-Portable-State",
        r".\Aureline-Portable-State\logs",
    );
    base.hidden_machine_global_write_used = true;
    base.hidden_machine_global_write_blocked = false;
    layout(base)
}

/// Degraded layout entry: the state origin is ambiguous, so support / export cannot tell portable state from
/// ordinary installed state.
fn layout_origin_ambiguous() -> M5ResolvedPortableStateLayoutEntry {
    let mut base = clean_layout_base(
        "layout:colocated:origin-ambiguous",
        "profile.portable_colocated",
        "portable.layout.colocated",
        M5InstallTopologyRole::WritableStateRoots,
        M5PortableStateContainment::ColocatedUnderExecutable,
        M5PortableSurfaceContext::DiagnosticsSurface,
        M5PortableStateOrigin::OriginAmbiguous,
        r".\AurelinePortable\app",
        r".\AurelinePortable\state",
        r".\AurelinePortable\logs",
    );
    base.state_origin = M5PortableStateOrigin::OriginAmbiguous;
    layout(base)
}

/// Degraded layout entry: the behavior is a hand-copied per-profile assumption instead of tracing to the
/// registry.
fn layout_unbound() -> M5ResolvedPortableStateLayoutEntry {
    let mut base = clean_layout_base(
        "layout:colocated:unbound",
        "profile.portable_colocated",
        "portable.layout.colocated",
        M5InstallTopologyRole::WritableStateRoots,
        M5PortableStateContainment::ColocatedUnderExecutable,
        M5PortableSurfaceContext::AdminSurface,
        M5PortableStateOrigin::PortableColocated,
        r".\AurelinePortable\app",
        r".\AurelinePortable\state",
        r".\AurelinePortable\logs",
    );
    base.bound_to_registry = false;
    layout(base)
}

/// Degraded layout entry: the canonical / accessible / audit presentation-form coverage is incomplete.
fn layout_form_incomplete() -> M5ResolvedPortableStateLayoutEntry {
    let mut base = clean_layout_base(
        "layout:colocated:form-incomplete",
        "profile.portable_colocated",
        "portable.layout.colocated",
        M5InstallTopologyRole::WritableStateRoots,
        M5PortableStateContainment::ColocatedUnderExecutable,
        M5PortableSurfaceContext::AboutSurface,
        M5PortableStateOrigin::PortableColocated,
        r".\AurelinePortable\app",
        r".\AurelinePortable\state",
        r".\AurelinePortable\logs",
    );
    base.presentation_form_coverage = vec![M5PortablePresentationForm::CanonicalObject];
    layout(base)
}

/// Degraded layout entry: the canonical registry token name is unstated.
fn layout_token_unstated() -> M5ResolvedPortableStateLayoutEntry {
    let mut base = clean_layout_base(
        "layout:support:token-unstated",
        "profile.portable_named_sibling",
        "  ",
        M5InstallTopologyRole::WritableStateRoots,
        M5PortableStateContainment::NamedSiblingDirectory,
        M5PortableSurfaceContext::SupportOrExportForm,
        M5PortableStateOrigin::PortableNamedSibling,
        r".\AurelinePortable\app",
        r".\Aureline-Portable-State",
        r".\Aureline-Portable-State\logs",
    );
    base.token_name = "  ".to_owned();
    layout(base)
}

// -- Clean portable-diagnostics entries ---------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_diagnostics_base(
    entry_id: &str,
    profile_id: &str,
    token_name: &str,
    semantic_role: M5InstallTopologyRole,
    diagnostics_surface: M5PortableDiagnosticsSurface,
    surface_context: M5PortableSurfaceContext,
    update_posture: M5PortableUpdatePosture,
    executable_root: &str,
    state_roots: &str,
) -> M5PortableDiagnosticsEntryResolutionInput {
    M5PortableDiagnosticsEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        profile_id: profile_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        diagnostics_surface,
        surface_context,
        presentation_form_coverage: all_forms(),
        executable_root: executable_root.to_owned(),
        state_roots: state_roots.to_owned(),
        disclosed_fields: all_diagnostics_fields(),
        update_posture,
        update_continuity_documented: true,
        unsupported_shell_paths_disclosed: true,
        proof_fresh: true,
    }
}

fn diagnostics_card_about_clean() -> M5ResolvedPortableDiagnosticsEntry {
    diagnostics(clean_diagnostics_base(
        "diagnostics:card:about",
        "profile.portable_colocated",
        "portable.diagnostics.card",
        M5InstallTopologyRole::WritableStateRoots,
        M5PortableDiagnosticsSurface::PortableDiagnosticsCard,
        M5PortableSurfaceContext::AboutSurface,
        M5PortableUpdatePosture::ManualReplace,
        r".\AurelinePortable\app",
        r".\AurelinePortable\state",
    ))
}

fn diagnostics_card_update_clean() -> M5ResolvedPortableDiagnosticsEntry {
    diagnostics(clean_diagnostics_base(
        "diagnostics:card:update",
        "profile.portable_named_sibling",
        "portable.diagnostics.card",
        M5InstallTopologyRole::WritableStateRoots,
        M5PortableDiagnosticsSurface::PortableDiagnosticsCard,
        M5PortableSurfaceContext::UpdateFlow,
        M5PortableUpdatePosture::TightlyControlledInPlace,
        r".\AurelinePortable\app",
        r".\Aureline-Portable-State",
    ))
}

fn diagnostics_card_diagnostics_clean() -> M5ResolvedPortableDiagnosticsEntry {
    diagnostics(clean_diagnostics_base(
        "diagnostics:card:diagnostics",
        "profile.portable_colocated",
        "portable.diagnostics.card",
        M5InstallTopologyRole::PolicyRoots,
        M5PortableDiagnosticsSurface::PortableDiagnosticsCard,
        M5PortableSurfaceContext::DiagnosticsSurface,
        M5PortableUpdatePosture::UpdatesUnsupported,
        r".\AurelinePortable\app",
        r".\AurelinePortable\state",
    ))
}

fn diagnostics_card_admin_clean() -> M5ResolvedPortableDiagnosticsEntry {
    diagnostics(clean_diagnostics_base(
        "diagnostics:card:admin",
        "profile.portable_colocated",
        "portable.diagnostics.card",
        M5InstallTopologyRole::RollbackTarget,
        M5PortableDiagnosticsSurface::PortableDiagnosticsCard,
        M5PortableSurfaceContext::AdminSurface,
        M5PortableUpdatePosture::ManualReplace,
        r".\AurelinePortable\app",
        r".\AurelinePortable\state",
    ))
}

fn diagnostics_docs_clean() -> M5ResolvedPortableDiagnosticsEntry {
    diagnostics(clean_diagnostics_base(
        "diagnostics:docs:help",
        "profile.portable_named_sibling",
        "portable.diagnostics.docs",
        M5InstallTopologyRole::WritableStateRoots,
        M5PortableDiagnosticsSurface::DocsHelpDiagnostics,
        M5PortableSurfaceContext::DiagnosticsSurface,
        M5PortableUpdatePosture::ManualReplace,
        r".\AurelinePortable\app",
        r".\Aureline-Portable-State",
    ))
}

fn diagnostics_support_clean() -> M5ResolvedPortableDiagnosticsEntry {
    diagnostics(clean_diagnostics_base(
        "diagnostics:support:export",
        "profile.portable_named_sibling",
        "portable.diagnostics.support",
        M5InstallTopologyRole::WritableStateRoots,
        M5PortableDiagnosticsSurface::SupportExportDiagnostics,
        M5PortableSurfaceContext::SupportOrExportForm,
        M5PortableUpdatePosture::TightlyControlledInPlace,
        r".\AurelinePortable\app",
        r".\Aureline-Portable-State",
    ))
}

// -- Degraded portable-diagnostics entries ------------------------------------------------------

/// Degraded diagnostics entry: the disclosure is incomplete — the explicitly unsupported shell-integration
/// paths are not disclosed.
fn diagnostics_disclosure_incomplete() -> M5ResolvedPortableDiagnosticsEntry {
    let mut base = clean_diagnostics_base(
        "diagnostics:card:disclosure-incomplete",
        "profile.portable_colocated",
        "portable.diagnostics.card",
        M5InstallTopologyRole::WritableStateRoots,
        M5PortableDiagnosticsSurface::PortableDiagnosticsCard,
        M5PortableSurfaceContext::AboutSurface,
        M5PortableUpdatePosture::ManualReplace,
        r".\AurelinePortable\app",
        r".\AurelinePortable\state",
    );
    base.disclosed_fields = vec![
        M5PortableDiagnosticsField::ExecutableRoot,
        M5PortableDiagnosticsField::StateRoots,
        M5PortableDiagnosticsField::LogAndCrashLocations,
        M5PortableDiagnosticsField::UpdatePosture,
        // UnsupportedShellIntegrationPaths is dropped: an unsupported path stays implicit.
    ];
    base.unsupported_shell_paths_disclosed = false;
    diagnostics(base)
}

/// Degraded diagnostics entry: the retained-versus-replaced update continuity note is absent.
fn diagnostics_continuity_undocumented() -> M5ResolvedPortableDiagnosticsEntry {
    let mut base = clean_diagnostics_base(
        "diagnostics:card:continuity-undocumented",
        "profile.portable_named_sibling",
        "portable.diagnostics.card",
        M5InstallTopologyRole::WritableStateRoots,
        M5PortableDiagnosticsSurface::PortableDiagnosticsCard,
        M5PortableSurfaceContext::UpdateFlow,
        M5PortableUpdatePosture::TightlyControlledInPlace,
        r".\AurelinePortable\app",
        r".\Aureline-Portable-State",
    );
    base.update_continuity_documented = false;
    diagnostics(base)
}

/// Degraded diagnostics entry: the diagnostics surface is unclassified.
fn diagnostics_surface_unclassified() -> M5ResolvedPortableDiagnosticsEntry {
    diagnostics(clean_diagnostics_base(
        "diagnostics:admin:surface-unclassified",
        "profile.portable_colocated",
        "portable.diagnostics.unknown",
        M5InstallTopologyRole::WritableStateRoots,
        M5PortableDiagnosticsSurface::SurfaceUnclassified,
        M5PortableSurfaceContext::AdminSurface,
        M5PortableUpdatePosture::ManualReplace,
        r".\AurelinePortable\app",
        r".\AurelinePortable\state",
    ))
}

/// Degraded diagnostics entry: the canonical / accessible / audit presentation-form coverage is incomplete.
fn diagnostics_form_incomplete() -> M5ResolvedPortableDiagnosticsEntry {
    let mut base = clean_diagnostics_base(
        "diagnostics:docs:form-incomplete",
        "profile.portable_named_sibling",
        "portable.diagnostics.docs",
        M5InstallTopologyRole::WritableStateRoots,
        M5PortableDiagnosticsSurface::DocsHelpDiagnostics,
        M5PortableSurfaceContext::DiagnosticsSurface,
        M5PortableUpdatePosture::ManualReplace,
        r".\AurelinePortable\app",
        r".\Aureline-Portable-State",
    );
    base.presentation_form_coverage = vec![M5PortablePresentationForm::CanonicalObject];
    diagnostics(base)
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5PortableModeConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5InstallTopologyDowngradeTrigger>,
    portable_state_layout_entries: Vec<M5ResolvedPortableStateLayoutEntry>,
    portable_diagnostics_entries: Vec<M5ResolvedPortableDiagnosticsEntry>,
) -> M5PortableModeStateContainmentAndDiagnosticsRow {
    M5PortableModeStateContainmentAndDiagnosticsRow {
        consumer_surface,
        qualification: M5InstallTopologyQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5InstallTopologyDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5InstallTopologyRequiredLabel::Identity,
            M5InstallTopologyRequiredLabel::SemanticRole,
            M5InstallTopologyRequiredLabel::RegistryReference,
            M5InstallTopologyRequiredLabel::InstallMode,
            M5InstallTopologyRequiredLabel::StateRoot,
        ],
        accessibility_routes: M5InstallTopologyAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5PortableAnatomyPart::ALL.to_vec(),
        export_fields: M5PortableExportField::ALL.to_vec(),
        downgrade_triggers,
        portable_state_layout_entries,
        portable_diagnostics_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_PORTABLE_MODE_STATE_CONTAINMENT_AND_DIAGNOSTICS_SCHEMA_REF,
            M5_STATE_ROOT_BOUNDARIES_SCHEMA_REF,
            M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_SCHEMA_REF,
        ]),
        portable_mode_writes_hidden_machine_global_durable_state: false,
        portable_state_indistinguishable_from_installed_state: false,
        portable_update_drops_retained_state_without_notice: false,
        unsupported_shell_integration_path_left_undisclosed: false,
    }
}

fn registry_rows() -> Vec<M5PortableModeStateContainmentAndDiagnosticsRow> {
    use M5InstallTopologyConsumerSurface as C;
    use M5InstallTopologyDowngradeTrigger as D;

    vec![
        base_row(
            C::ShellUi,
            "Shell/About surface owner",
            "About resolves the colocated portable layout to one stable object — executable root, colocated state roots, and the durable-root inventory of settings, secrets, services, and shell hooks — from the shared registry and inspects the portable-diagnostics card; an incomplete durable-root inventory and a diagnostics card that hides an unsupported shell-integration path degrade honestly instead of reading as a clean pass",
            "evidence:m5-portable-shell-ui:001",
            vec![
                D::StateRootUnstated,
                D::PortableModeWroteHiddenMachineGlobalDurableState,
                D::ProofStale,
            ],
            vec![layout_colocated_about_clean(), layout_inventory_incomplete()],
            vec![diagnostics_card_about_clean(), diagnostics_disclosure_incomplete()],
        ),
        base_row(
            C::UpdaterService,
            "Updater/update-flow owner",
            "The update flow resolves the named-sibling portable layout and the portable-diagnostics update posture; a durable-state spill into a hidden machine-global path and an undocumented retained-versus-replaced continuity note are caught before a portable update can silently drop state",
            "evidence:m5-portable-updater:001",
            vec![
                D::PortableModeWroteHiddenMachineGlobalDurableState,
                D::RollbackTargetedPrimaryExecutableWhileSidecarsDrifted,
                D::ProofStale,
            ],
            vec![layout_sibling_update_clean(), layout_hidden_spill()],
            vec![diagnostics_card_update_clean(), diagnostics_continuity_undocumented()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics reports the colocated portable layout and its discoverable diagnostics without manual reconstruction; a layout whose state origin is ambiguous — so support cannot tell portable state from installed state — is caught instead of reading as a clean pass",
            "evidence:m5-portable-diagnostics:001",
            vec![
                D::StateRootBoundaryDriftedByTopology,
                D::StateRootUnstated,
                D::ProofStale,
            ],
            vec![
                layout_colocated_diagnostics_clean(),
                layout_origin_ambiguous(),
            ],
            vec![diagnostics_card_diagnostics_clean()],
        ),
        base_row(
            C::Admin,
            "Admin surface owner",
            "Admin resolves the colocated portable layout while preserving one registry-bound source; a hand-copied per-profile assumption and a diagnostics record on an unclassified surface degrade honestly",
            "evidence:m5-portable-admin:001",
            vec![
                D::StateRootBoundaryDriftedByTopology,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![layout_colocated_admin_clean(), layout_unbound()],
            vec![diagnostics_card_admin_clean(), diagnostics_surface_unclassified()],
        ),
        base_row(
            C::DocsHelp,
            "Docs/help surface owner",
            "Docs and help render the same resolved portable layout and discoverable diagnostics truth the resolvers produced across the canonical, accessible, and audit presentation forms rather than a hand-copied path table",
            "evidence:m5-portable-docs-help:001",
            vec![
                D::RegistryReferenceUnstated,
                D::StateRootUnstated,
                D::ProofStale,
            ],
            vec![layout_colocated_about_clean(), layout_form_incomplete()],
            vec![diagnostics_docs_clean(), diagnostics_form_incomplete()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved portable layout and diagnostics truth, so a hand-copied constant, an unstated registry token, an ambiguous state origin, or a hidden machine-global spill is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-portable-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::PortableModeWroteHiddenMachineGlobalDurableState,
                D::ProofStale,
            ],
            vec![layout_sibling_support_clean(), layout_token_unstated()],
            vec![diagnostics_support_clean()],
        ),
    ]
}

fn governance_review() -> M5PortableModeStateContainmentAndDiagnosticsGovernanceReview {
    M5PortableModeStateContainmentAndDiagnosticsGovernanceReview {
        portable_registry_names_token_role_and_containment: true,
        profile_resolves_to_colocated_or_named_sibling_layout: true,
        all_durable_roots_identified_and_inventoried: true,
        hidden_machine_global_mutation_absent_or_blocked: true,
        portable_state_distinguishable_from_installed_state: true,
        portable_diagnostics_discoverable_across_surfaces: true,
        every_entry_covers_all_presentation_forms: true,
        update_continuity_documented_for_retained_versus_replaced_state: true,
        behavior_bound_to_registry_not_hand_copied: true,
        about_update_diagnostics_admin_read_single_source: true,
        portable_spill_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5PortableModeStateContainmentAndDiagnosticsConsumerProjection {
    M5PortableModeStateContainmentAndDiagnosticsConsumerProjection {
        about_and_update_consume_shared_registries: true,
        diagnostics_and_admin_consume_shared_registries: true,
        installers_and_portable_launcher_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5PortableModeStateContainmentAndDiagnosticsProofFreshness {
    M5PortableModeStateContainmentAndDiagnosticsProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5PortableModeStateContainmentAndDiagnosticsReleasePosture {
    M5PortableModeStateContainmentAndDiagnosticsReleasePosture {
        proof_packet_ref: M5_PORTABLE_MODE_STATE_CONTAINMENT_AND_DIAGNOSTICS_ARTIFACT_REF
            .to_owned(),
        portable_diagnostics_audit_ref:
            M5_PORTABLE_MODE_STATE_CONTAINMENT_AND_DIAGNOSTICS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_PORTABLE_MODE_STATE_CONTAINMENT_AND_DIAGNOSTICS_SCHEMA_REF,
        M5_PORTABLE_MODE_STATE_CONTAINMENT_AND_DIAGNOSTICS_DOC_REF,
        M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF,
        M5_INSTALL_TOPOLOGY_MATRIX_DOC_REF,
        M5_STATE_ROOT_BOUNDARIES_SCHEMA_REF,
        M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 portable-mode state-containment and diagnostics registries packet.
pub fn seeded_m5_portable_mode_state_containment_and_diagnostics(
) -> M5PortableModeStateContainmentAndDiagnosticsPacket {
    M5PortableModeStateContainmentAndDiagnosticsPacket::new(
        M5PortableModeStateContainmentAndDiagnosticsPacketInput {
            packet_id: M5_PORTABLE_MODE_STATE_CONTAINMENT_AND_DIAGNOSTICS_PACKET_ID.to_owned(),
            registries_label:
                "M5 portable-mode state-containment and diagnostics registries enforcing colocated or explicitly named sibling-state layouts, a complete durable-root inventory of settings / secrets / services / shell hooks, absent-or-blocked hidden machine-global mutation, distinguishable portable-versus-installed state origin, discoverable portable diagnostics, and documented retained-versus-replaced update continuity across About, update, diagnostics, admin, docs, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5PortableModeStateContainmentAndDiagnosticsVocabularySet::canonical(),
            governance_review: governance_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            release_posture: release_posture(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Narrowed variant: the diagnostics row is held at Beta pending side-by-side channel portable-diagnostics
/// parity on every platform; every row stays visible and every example stays honest.
pub fn seeded_m5_portable_mode_state_containment_and_diagnostics_side_by_side_channel_beta_narrowed(
) -> M5PortableModeStateContainmentAndDiagnosticsPacket {
    let mut packet = seeded_m5_portable_mode_state_containment_and_diagnostics();
    packet.packet_id =
        "m5-portable-mode-state-containment-and-diagnostics:side-by-side-channel-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5InstallTopologyConsumerSurface::Diagnostics)
        .expect("diagnostics row present");
    row.qualification = M5InstallTopologyQualificationClass::Beta;
    packet
}

/// Narrowed variant: the updater row is narrowed to Preview pending offline / air-gap portable-update parity
/// on every platform; every row stays visible and every example stays honest.
pub fn seeded_m5_portable_mode_state_containment_and_diagnostics_offline_airgap_bundle_preview_narrowed(
) -> M5PortableModeStateContainmentAndDiagnosticsPacket {
    let mut packet = seeded_m5_portable_mode_state_containment_and_diagnostics();
    packet.packet_id =
        "m5-portable-mode-state-containment-and-diagnostics:offline-airgap-bundle-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5InstallTopologyConsumerSurface::UpdaterService)
        .expect("updater-service row present");
    row.qualification = M5InstallTopologyQualificationClass::Preview;
    packet
}
