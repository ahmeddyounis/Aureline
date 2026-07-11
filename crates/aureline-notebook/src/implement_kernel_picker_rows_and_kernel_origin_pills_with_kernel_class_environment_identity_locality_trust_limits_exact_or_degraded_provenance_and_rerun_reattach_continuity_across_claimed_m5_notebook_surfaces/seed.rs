//! Canonical seed builders for the kernel-picker-row / kernel-origin-pill controls.
//!
//! These builders are the single producer of the checked-in support export and the scenario
//! fixtures. The headless emitter and the inline tests both call them so the in-code components,
//! the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical kernel-picker-row / kernel-origin-pill packet.
pub const KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_PACKET_ID: &str =
    "m5-kernel-picker-row-kernel-origin-pill-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn picker_source_refs() -> Vec<String> {
    strings(&[
        M5_KERNEL_PICKER_ROW_SCHEMA_REF,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
    ])
}

fn pill_source_refs() -> Vec<String> {
    strings(&[
        M5_KERNEL_ORIGIN_PILL_SCHEMA_REF,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
    ])
}

fn picker_downgrade_triggers() -> Vec<M5NotebookKernelOutputDowngradeTrigger> {
    vec![
        M5NotebookKernelOutputDowngradeTrigger::KernelClassCollapsed,
        M5NotebookKernelOutputDowngradeTrigger::AlternateStateLabelInvented,
        M5NotebookKernelOutputDowngradeTrigger::ProofStale,
    ]
}

fn pill_downgrade_triggers() -> Vec<M5NotebookKernelOutputDowngradeTrigger> {
    vec![
        M5NotebookKernelOutputDowngradeTrigger::KernelOriginUnstated,
        M5NotebookKernelOutputDowngradeTrigger::KernelClassCollapsed,
        M5NotebookKernelOutputDowngradeTrigger::ProvenanceSevered,
        M5NotebookKernelOutputDowngradeTrigger::AlternateStateLabelInvented,
        M5NotebookKernelOutputDowngradeTrigger::ProofStale,
    ]
}

/// Builds a kernel picker row, deriving the choice state, the selectable / current claims, and the
/// required notes from the honest inputs so the seed is always self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn picker_row(
    row_id: &str,
    row_label: &str,
    candidate_kind: M5KernelCandidateKind,
    selection_state: M5KernelSelectionState,
    kernel_class_label: &str,
    environment_identity_label: &str,
    locality_label: &str,
    compatibility_note: &str,
    trust_policy_limit_note: &str,
    last_seen_label: &str,
    context_note: &str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    picker_actions: Vec<KernelPickerAction>,
    dispositions: Vec<M5NotebookKernelOutputDisposition>,
) -> KernelPickerRow {
    let disclosure = resolve_kernel_picker_row(candidate_kind, selection_state);
    KernelPickerRow {
        component: M5NotebookKernelOutputComponentFamily::KernelPickerRow,
        row_id: row_id.to_owned(),
        row_label: row_label.to_owned(),
        candidate_kind,
        selection_state,
        choice_state: disclosure.choice_state,
        claims_selectable_now: disclosure.is_selectable_now,
        claims_current: disclosure.is_current,
        incompatible_note: if disclosure.needs_incompatible_note {
            "Incompatible with this notebook's kernelspec; choosing it would fail to attach"
                .to_owned()
        } else {
            String::new()
        },
        unavailable_note: if disclosure.needs_unavailable_note {
            "Currently unavailable / offline; it cannot be attached until it comes back".to_owned()
        } else {
            String::new()
        },
        install_note: if disclosure.needs_install_note {
            "Needs an install / setup step before it can be selected".to_owned()
        } else {
            String::new()
        },
        kernel_class_label: kernel_class_label.to_owned(),
        environment_identity_label: environment_identity_label.to_owned(),
        locality_label: locality_label.to_owned(),
        compatibility_note: compatibility_note.to_owned(),
        trust_policy_limit_note: trust_policy_limit_note.to_owned(),
        last_seen_label: last_seen_label.to_owned(),
        context_note: context_note.to_owned(),
        deep_link_kind,
        deep_link_ref: deep_link_ref.to_owned(),
        picker_actions,
        dispositions,
        downgrade_triggers: picker_downgrade_triggers(),
        required_labels: M5NotebookKernelOutputRequiredLabel::ALL.to_vec(),
        surface_families: M5NotebookKernelOutputSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5NotebookKernelOutputDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5NotebookKernelOutputAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5NotebookKernelOutputConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "kernel_class_label",
            "candidate_kind",
            "selection_state",
            "choice_state",
            "environment_identity_label",
            "locality_label",
            "compatibility_note",
            "trust_policy_limit_note",
            "last_seen_label",
            "deep_link_kind",
        ]),
        source_contract_refs: picker_source_refs(),
        collapses_kernel_origins_into_one_badge: false,
        implies_exact_continuity_on_material_drift: false,
        hides_trust_or_compatibility_behind_hover_only: false,
        overwrites_provenance_without_review: false,
    }
}

/// Builds a kernel origin pill, deriving the provenance class, the exact-provenance and
/// exact-continuity claims, and the required notes from the honest inputs so the seed is always
/// self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn origin_pill(
    pill_id: &str,
    pill_label: &str,
    origin_class: M5KernelOriginClass,
    trust_state: M5KernelOriginTrustState,
    fingerprint_state: KernelFingerprintState,
    origin_label: &str,
    provenance_label: &str,
    trust_limit_note: &str,
    continuity_note: &str,
    context_note: &str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    pill_actions: Vec<KernelPillAction>,
    dispositions: Vec<M5NotebookKernelOutputDisposition>,
) -> KernelOriginPill {
    let disclosure = resolve_kernel_origin_pill(origin_class, trust_state, fingerprint_state);
    KernelOriginPill {
        component: M5NotebookKernelOutputComponentFamily::KernelOriginPill,
        pill_id: pill_id.to_owned(),
        pill_label: pill_label.to_owned(),
        origin_class,
        trust_state,
        fingerprint_state,
        provenance_class: disclosure.provenance_class,
        claims_exact_provenance: disclosure.is_exact_provenance,
        claims_exact_continuity: disclosure.may_claim_exact_continuity,
        degraded_note: if disclosure.needs_degraded_note {
            "Degraded provenance; this origin is only partially attributed — verify before trusting"
                .to_owned()
        } else {
            String::new()
        },
        restricted_note: if disclosure.needs_restricted_note {
            "Restricted origin; policy limits what this kernel may do and export".to_owned()
        } else {
            String::new()
        },
        unknown_origin_note: if disclosure.needs_unknown_origin_note {
            "Unknown origin; Aureline could not attribute where this kernel runs".to_owned()
        } else {
            String::new()
        },
        drift_note: if disclosure.needs_drift_note {
            "Environment fingerprint differs from the last run; do not assume exact continuity"
                .to_owned()
        } else {
            String::new()
        },
        origin_label: origin_label.to_owned(),
        provenance_label: provenance_label.to_owned(),
        trust_limit_note: trust_limit_note.to_owned(),
        continuity_note: continuity_note.to_owned(),
        context_note: context_note.to_owned(),
        deep_link_kind,
        deep_link_ref: deep_link_ref.to_owned(),
        pill_actions,
        dispositions,
        downgrade_triggers: pill_downgrade_triggers(),
        required_labels: M5NotebookKernelOutputRequiredLabel::ALL.to_vec(),
        surface_families: M5NotebookKernelOutputSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5NotebookKernelOutputDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5NotebookKernelOutputAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5NotebookKernelOutputConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "origin_label",
            "origin_class",
            "trust_state",
            "provenance_class",
            "fingerprint_state",
            "provenance_label",
            "trust_limit_note",
            "continuity_note",
            "deep_link_kind",
        ]),
        source_contract_refs: pill_source_refs(),
        collapses_kernel_origins_into_one_badge: false,
        implies_exact_continuity_on_material_drift: false,
        hides_trust_or_compatibility_behind_hover_only: false,
        overwrites_provenance_without_review: false,
    }
}

fn picker_rows() -> Vec<KernelPickerRow> {
    use DeepLinkKind as Link;
    use KernelPickerAction as Action;
    use M5KernelCandidateKind as Kind;
    use M5KernelSelectionState as Sel;
    use M5NotebookKernelOutputDisposition as Disp;

    vec![
        // 1. Local interpreter, selected → currently selected (the current kernel).
        picker_row(
            "row-local-cpython",
            "CPython 3.11 (local interpreter)",
            Kind::LocalInterpreter,
            Sel::Selected,
            "Local interpreter kernel",
            "CPython 3.11.6 · /usr/bin/python3 · sha env-fp-local-311",
            "Runs on this workstation",
            "Compatible with this notebook's kernelspec",
            "Trusted first-party interpreter; no policy limits",
            "Last seen: now (attached)",
            "Choice truth: which candidate is the current kernel and where it runs",
            Link::KernelManager,
            "kernel:local/cpython-311",
            vec![
                Action::ChooseKernel,
                Action::InspectCandidate,
                Action::ViewCompatibility,
                Action::KeepCurrentKernel,
                Action::OpenDeepLink,
            ],
            vec![Disp::Ready, Disp::Active],
        ),
        // 2. Virtual env, recommended → recommended choice.
        picker_row(
            "row-venv-analysis",
            "analysis-venv (virtual env)",
            Kind::VirtualEnv,
            Sel::Recommended,
            "Virtual environment kernel",
            "venv analysis · python 3.11 · sha env-fp-venv-analysis",
            "Runs on this workstation",
            "Matches the notebook's recorded environment fingerprint",
            "Trusted first-party env; no policy limits",
            "Last seen: 2 minutes ago",
            "Choice truth: a recommended env that matches this notebook",
            Link::KernelManager,
            "kernel:venv/analysis",
            vec![
                Action::ChooseKernel,
                Action::InspectCandidate,
                Action::ViewCompatibility,
                Action::OpenDeepLink,
            ],
            vec![Disp::Active, Disp::ChooseAnotherKernel],
        ),
        // 3. Conda env, available → available choice.
        picker_row(
            "row-conda-ml",
            "ml-conda (conda env)",
            Kind::CondaEnv,
            Sel::Available,
            "Conda environment kernel",
            "conda ml · python 3.10 · sha env-fp-conda-ml",
            "Runs on this workstation",
            "Compatible but a minor version behind the notebook's last run",
            "Trusted first-party env; no policy limits",
            "Last seen: 10 minutes ago",
            "Choice truth: an available conda env you can attach now",
            Link::DocsAnchor,
            "docs:notebooks/choosing-a-kernel",
            vec![
                Action::ChooseKernel,
                Action::InspectCandidate,
                Action::ViewCompatibility,
                Action::OpenDeepLink,
            ],
            vec![Disp::Active, Disp::ChooseAnotherKernel],
        ),
        // 4. Container kernel, incompatible → incompatible choice (needs incompatible note).
        picker_row(
            "row-container-legacy",
            "legacy-py38 (container)",
            Kind::ContainerKernel,
            Sel::Incompatible,
            "Container kernel",
            "container legacy-py38 · python 3.8 · sha env-fp-container-py38",
            "Runs inside the project container",
            "Incompatible: notebook requires 3.11, container ships 3.8",
            "Trusted first-party container image; no policy limits",
            "Last seen: 1 hour ago",
            "Choice truth: an incompatible container kernel that cannot attach cleanly",
            Link::DocsAnchor,
            "docs:notebooks/kernel-compatibility",
            vec![
                Action::ChooseKernel,
                Action::InspectCandidate,
                Action::ViewCompatibility,
                Action::OpenDeepLink,
            ],
            vec![Disp::ChooseAnotherKernel],
        ),
        // 5. Remote kernel, needs install → needs setup first (needs install note).
        picker_row(
            "row-remote-gpu",
            "gpu-pool (remote)",
            Kind::RemoteKernel,
            Sel::NeedsInstall,
            "Remote kernel",
            "remote gpu-pool · python 3.11 (unprovisioned) · sha env-fp-remote-gpu",
            "Runs on the connected remote host",
            "Compatible once provisioned; the kernel is not installed yet",
            "Third-party remote; policy limits export of remote outputs",
            "Last seen: available for provisioning",
            "Choice truth: a remote kernel that needs an install step before use",
            Link::KernelManager,
            "kernel:remote/gpu-pool",
            vec![
                Action::ChooseKernel,
                Action::InspectCandidate,
                Action::ViewCompatibility,
                Action::InstallKernel,
                Action::OpenDeepLink,
            ],
            vec![Disp::Remote, Disp::ChooseAnotherKernel],
        ),
        // 6. Managed kernel, unavailable → unavailable choice (needs unavailable note).
        picker_row(
            "row-managed-offline",
            "team-managed (managed)",
            Kind::ManagedKernel,
            Sel::Unavailable,
            "Managed kernel",
            "managed team-managed · python 3.11 · sha env-fp-managed-team",
            "Runs in the managed workspace",
            "Compatible but the managed workspace is offline right now",
            "Managed origin; policy limits govern its lifecycle and export",
            "Last seen: 30 minutes ago",
            "Choice truth: a managed kernel that is compatible but currently offline",
            Link::SupportBundle,
            "support:bundle/managed-kernel-state",
            vec![
                Action::ChooseKernel,
                Action::InspectCandidate,
                Action::ViewCompatibility,
            ],
            vec![Disp::Managed, Disp::ChooseAnotherKernel],
        ),
    ]
}

fn origin_pills() -> Vec<KernelOriginPill> {
    use DeepLinkKind as Link;
    use KernelFingerprintState as Fp;
    use KernelPillAction as Action;
    use M5KernelOriginClass as Origin;
    use M5KernelOriginTrustState as Trust;
    use M5NotebookKernelOutputDisposition as Disp;

    vec![
        // 1. Local host, trusted, matched fingerprint → exact provenance, may claim continuity.
        origin_pill(
            "pill-local-trusted",
            "Kernel origin: local host",
            Origin::LocalHost,
            Trust::TrustedOrigin,
            Fp::FingerprintMatched,
            "Local host: this workstation",
            "Exact provenance (trusted, first-party)",
            "Trusted origin; no policy limits on this kernel",
            "Fingerprint matches the last run; reattaching keeps exact continuity",
            "Origin truth: where the current kernel runs and how trusted it is",
            Link::KernelManager,
            "kernel:local/cpython-311",
            vec![
                Action::InspectOrigin,
                Action::ViewProvenance,
                Action::CopyOriginIdentity,
                Action::ReattachKernel,
                Action::OpenDeepLink,
            ],
            vec![Disp::Ready, Disp::Active],
        ),
        // 2. SSH remote, first-party, matched → exact provenance, may claim continuity.
        origin_pill(
            "pill-ssh-firstparty",
            "Kernel origin: SSH remote",
            Origin::SshRemote,
            Trust::FirstParty,
            Fp::FingerprintMatched,
            "SSH remote: research-workstation over SSH",
            "Exact provenance (first-party over SSH)",
            "First-party origin; standard export policy applies",
            "Fingerprint matches the last run; reattaching keeps exact continuity",
            "Origin truth: a first-party SSH kernel with matched environment",
            Link::KernelManager,
            "kernel:ssh/research-workstation",
            vec![
                Action::InspectOrigin,
                Action::ViewProvenance,
                Action::CopyOriginIdentity,
                Action::ReattachKernel,
                Action::ReviewContinuity,
                Action::OpenDeepLink,
            ],
            vec![Disp::Remote, Disp::Active],
        ),
        // 3. Container, third-party, drifted → degraded provenance, cannot claim continuity
        //    (needs degraded + drift notes).
        origin_pill(
            "pill-container-thirdparty",
            "Kernel origin: container",
            Origin::Container,
            Trust::ThirdParty,
            Fp::FingerprintDrifted,
            "Container: third-party image community/py311",
            "Degraded provenance (third-party image)",
            "Third-party origin; policy limits export of container outputs",
            "Fingerprint drifted since the last run; rerun does not keep exact continuity",
            "Origin truth: a third-party container kernel whose environment drifted",
            Link::DocsAnchor,
            "docs:notebooks/kernel-origin-and-provenance",
            vec![
                Action::InspectOrigin,
                Action::ViewProvenance,
                Action::CopyOriginIdentity,
                Action::ReviewContinuity,
                Action::OpenDeepLink,
            ],
            vec![Disp::StaleOutput, Disp::ChooseAnotherKernel],
        ),
        // 4. Devcontainer, unverified, drifted → degraded provenance, cannot claim continuity
        //    (needs degraded + drift notes).
        origin_pill(
            "pill-devcontainer-unverified",
            "Kernel origin: devcontainer",
            Origin::Devcontainer,
            Trust::UnverifiedOrigin,
            Fp::FingerprintDrifted,
            "Devcontainer: .devcontainer image (unverified)",
            "Degraded provenance (unverified devcontainer)",
            "Unverified origin; verify before trusting outputs or exporting",
            "Fingerprint drifted since the last run; reattach would not restore exact continuity",
            "Origin truth: an unverified devcontainer kernel whose environment drifted",
            Link::DocsAnchor,
            "docs:notebooks/devcontainer-kernels",
            vec![
                Action::InspectOrigin,
                Action::ViewProvenance,
                Action::CopyOriginIdentity,
                Action::ReviewContinuity,
                Action::OpenDeepLink,
            ],
            vec![Disp::StaleOutput, Disp::ChooseAnotherKernel],
        ),
        // 5. Managed workspace, restricted, unknown fingerprint → restricted provenance
        //    (needs restricted + drift notes).
        origin_pill(
            "pill-managed-restricted",
            "Kernel origin: managed workspace",
            Origin::ManagedWorkspace,
            Trust::RestrictedOrigin,
            Fp::FingerprintUnknown,
            "Managed workspace: team-managed",
            "Restricted provenance (managed, policy-limited)",
            "Restricted origin; policy limits what this kernel may run and export",
            "Fingerprint could not be compared; do not assume exact continuity",
            "Origin truth: a managed, policy-restricted kernel with an unknown fingerprint",
            Link::SupportBundle,
            "support:bundle/managed-kernel-origin",
            vec![
                Action::InspectOrigin,
                Action::ViewProvenance,
                Action::CopyOriginIdentity,
                Action::ReviewContinuity,
                Action::OpenDeepLink,
            ],
            vec![Disp::Managed, Disp::ChooseAnotherKernel],
        ),
        // 6. Browser bridge, unknown origin, not evaluated → unknown provenance
        //    (needs unknown-origin + drift notes).
        origin_pill(
            "pill-browser-unknown",
            "Kernel origin: browser bridge",
            Origin::BrowserBridge,
            Trust::UnknownOrigin,
            Fp::FingerprintNotEvaluated,
            "Browser bridge: in-browser runtime",
            "Unknown provenance (browser bridge)",
            "Unknown origin; treat outputs as untrusted until attributed",
            "No fingerprint evaluated yet; exact continuity cannot be assumed",
            "Origin truth: a browser-bridge kernel whose origin is not yet attributed",
            Link::NoDeepLink,
            "",
            vec![
                Action::InspectOrigin,
                Action::ViewProvenance,
                Action::CopyOriginIdentity,
            ],
            vec![Disp::ChooseAnotherKernel],
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5NotebookKernelOutputDowngradeTrigger> {
    vec![
        M5NotebookKernelOutputDowngradeTrigger::KernelOriginUnstated,
        M5NotebookKernelOutputDowngradeTrigger::KernelClassCollapsed,
        M5NotebookKernelOutputDowngradeTrigger::ProvenanceSevered,
        M5NotebookKernelOutputDowngradeTrigger::AlternateStateLabelInvented,
        M5NotebookKernelOutputDowngradeTrigger::ProofStale,
    ]
}

fn kernel_review() -> KernelPickerOriginReview {
    KernelPickerOriginReview {
        picker_shows_kernel_class: true,
        picker_shows_environment_identity: true,
        picker_shows_compatibility_and_trust_limits: true,
        picker_offers_choose_inspect_compatibility: true,
        pill_shows_kernel_origin_class: true,
        pill_shows_provenance_confidence: true,
        pill_offers_inspect_provenance_copy: true,
        provenance_and_choice_derived_never_asserted: true,
        choose_another_kernel_without_losing_provenance: true,
        kernel_origin_visible_in_tabs_headers_debug_support: true,
        exact_continuity_never_implied_on_material_drift: true,
        no_kernel_origins_collapsed_into_one_badge: true,
        trust_and_compatibility_never_hover_only: true,
        every_next_step_names_stable_deep_link: true,
        picker_and_pill_consistent_across_surfaces: true,
        no_component_widens_export_scope_or_exposes_raw_by_default: true,
        components_stable_across_deployment_lines: true,
        copy_and_export_safe: true,
        no_surface_invents_alternate_state_label: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> KernelPickerOriginConsumerProjection {
    KernelPickerOriginConsumerProjection {
        kernel_manager_surface_reads_single_source: true,
        notebook_tab_shows_kernel_origin: true,
        debug_bridge_shows_kernel_origin: true,
        support_export_shows_kernel_origin: true,
        picker_choice_visible_before_run: true,
        help_docs_shows_component_truth: true,
    }
}

fn proof_freshness() -> KernelPickerOriginProofFreshness {
    KernelPickerOriginProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_SCHEMA_REF,
        KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_DOC_REF,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_DOC_REF,
        M5_KERNEL_PICKER_ROW_SCHEMA_REF,
        M5_KERNEL_ORIGIN_PILL_SCHEMA_REF,
    ])
}

/// Builds the canonical kernel-picker-row / kernel-origin-pill controls packet.
pub fn seeded_kernel_picker_row_kernel_origin_pill_controls(
) -> KernelPickerRowKernelOriginPillControlsPacket {
    KernelPickerRowKernelOriginPillControlsPacket::new(
        KernelPickerRowKernelOriginPillControlsPacketInput {
            packet_id: KERNEL_PICKER_ROW_KERNEL_ORIGIN_PILL_PACKET_ID.to_owned(),
            surface_label:
                "M5 kernel picker rows and kernel origin pills: kernel class, environment identity, locality, trust limits, exact or degraded provenance, and rerun/reattach continuity across claimed notebook surfaces"
                    .to_owned(),
            picker_rows: picker_rows(),
            origin_pills: origin_pills(),
            downgrade_triggers: downgrade_triggers(),
            consumer_surfaces: M5NotebookKernelOutputConsumerSurface::ALL.to_vec(),
            kernel_review: kernel_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Scenario fixture: spotlights an incompatible kernel picker row that must never read as a clean,
/// selectable choice. Every candidate kind, selection state, and choice state stays covered so the
/// fixture validates on its own.
pub fn seeded_kernel_picker_row_kernel_origin_pill_controls_kernel_picker_row_incompatible(
) -> KernelPickerRowKernelOriginPillControlsPacket {
    let mut packet = seeded_kernel_picker_row_kernel_origin_pill_controls();
    packet.packet_id =
        "m5-kernel-picker-row-kernel-origin-pill-controls:fixture:kernel-picker-row-incompatible"
            .to_owned();
    packet.surface_label =
        "M5 kernel picker rows: an incompatible candidate never reads as a clean, selectable choice"
            .to_owned();
    packet
}

/// Scenario fixture: spotlights a degraded / drifted kernel origin pill that must never imply exact
/// continuity. Every origin class, trust state, provenance class, and fingerprint state stays
/// covered so the fixture validates on its own.
pub fn seeded_kernel_picker_row_kernel_origin_pill_controls_kernel_origin_pill_degraded(
) -> KernelPickerRowKernelOriginPillControlsPacket {
    let mut packet = seeded_kernel_picker_row_kernel_origin_pill_controls();
    packet.packet_id =
        "m5-kernel-picker-row-kernel-origin-pill-controls:fixture:kernel-origin-pill-degraded"
            .to_owned();
    packet.surface_label =
        "M5 kernel origin pills: a degraded, drifted origin never implies exact continuity"
            .to_owned();
    packet
}
