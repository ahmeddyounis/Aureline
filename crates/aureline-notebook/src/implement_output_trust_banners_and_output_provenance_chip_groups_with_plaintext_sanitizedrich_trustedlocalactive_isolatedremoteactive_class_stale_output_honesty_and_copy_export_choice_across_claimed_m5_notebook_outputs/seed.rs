//! Canonical seed builders for the output-trust-banner / output-provenance-chip-group controls.
//!
//! These builders are the single producer of the checked-in support export and the scenario
//! fixtures. The headless emitter and the inline tests both call them so the in-code components,
//! the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical output-trust-banner / output-provenance-chip-group packet.
pub const OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_PACKET_ID: &str =
    "m5-output-trust-banner-output-provenance-chip-group-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn banner_source_refs() -> Vec<String> {
    strings(&[
        M5_OUTPUT_TRUST_BANNER_SCHEMA_REF,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
    ])
}

fn chip_source_refs() -> Vec<String> {
    strings(&[
        M5_OUTPUT_PROVENANCE_CHIP_GROUP_SCHEMA_REF,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
    ])
}

fn banner_downgrade_triggers() -> Vec<M5NotebookKernelOutputDowngradeTrigger> {
    vec![
        M5NotebookKernelOutputDowngradeTrigger::OutputTrustUnstated,
        M5NotebookKernelOutputDowngradeTrigger::StaleOutputShownAsLive,
        M5NotebookKernelOutputDowngradeTrigger::TrustClassHoverOnly,
        M5NotebookKernelOutputDowngradeTrigger::AlternateStateLabelInvented,
        M5NotebookKernelOutputDowngradeTrigger::ProofStale,
    ]
}

fn chip_downgrade_triggers() -> Vec<M5NotebookKernelOutputDowngradeTrigger> {
    vec![
        M5NotebookKernelOutputDowngradeTrigger::ProvenanceSevered,
        M5NotebookKernelOutputDowngradeTrigger::AlternateStateLabelInvented,
        M5NotebookKernelOutputDowngradeTrigger::ProofStale,
    ]
}

/// Builds an output trust banner, deriving the presentation class, the active-content and live
/// claims, and the required notes from the honest inputs so the seed is always self-consistent with
/// the resolver.
#[allow(clippy::too_many_arguments)]
fn trust_banner(
    banner_id: &str,
    banner_label: &str,
    trust_class: M5OutputTrustClass,
    freshness_state: M5OutputFreshnessState,
    representation_mode: OutputRepresentationMode,
    trust_class_label: &str,
    representation_label: &str,
    freshness_label: &str,
    copy_export_choice_note: &str,
    context_note: &str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    banner_actions: Vec<OutputBannerAction>,
    dispositions: Vec<M5NotebookKernelOutputDisposition>,
) -> OutputTrustBanner {
    let disclosure = resolve_output_trust_banner(trust_class, freshness_state);
    OutputTrustBanner {
        component: M5NotebookKernelOutputComponentFamily::OutputTrustBanner,
        banner_id: banner_id.to_owned(),
        banner_label: banner_label.to_owned(),
        trust_class,
        freshness_state,
        representation_mode,
        presentation_class: disclosure.presentation_class,
        claims_active_content: disclosure.is_active_content,
        claims_live: disclosure.may_present_as_live,
        sanitized_note: if disclosure.needs_sanitized_note {
            "Rich output rendered with active content stripped; open raw to see the source"
                .to_owned()
        } else {
            String::new()
        },
        active_content_note: if disclosure.needs_active_content_note {
            "Carries active content; it runs code — review before you trust or share it".to_owned()
        } else {
            String::new()
        },
        isolation_note: if disclosure.needs_isolation_note {
            "Active content runs only inside an isolated sandbox; it is not trusted local content"
                .to_owned()
        } else {
            String::new()
        },
        blocked_note: if disclosure.needs_blocked_note {
            "Output withheld by policy; the raw representation is available behind review"
                .to_owned()
        } else {
            String::new()
        },
        unknown_trust_note: if disclosure.needs_unknown_trust_note {
            "Trust class could not be determined; treat this output as untrusted".to_owned()
        } else {
            String::new()
        },
        stale_note: if disclosure.needs_stale_note {
            "Stale output: it no longer matches the current cell, kernel, or environment — rerun to refresh"
                .to_owned()
        } else {
            String::new()
        },
        cached_note: if disclosure.needs_cached_note {
            "Cached output from an earlier run; it is not live".to_owned()
        } else {
            String::new()
        },
        cleared_note: if disclosure.needs_cleared_note {
            "Output cleared or absent; nothing is rendered here".to_owned()
        } else {
            String::new()
        },
        trust_class_label: trust_class_label.to_owned(),
        representation_label: representation_label.to_owned(),
        freshness_label: freshness_label.to_owned(),
        copy_export_choice_note: copy_export_choice_note.to_owned(),
        context_note: context_note.to_owned(),
        deep_link_kind,
        deep_link_ref: deep_link_ref.to_owned(),
        banner_actions,
        dispositions,
        downgrade_triggers: banner_downgrade_triggers(),
        required_labels: M5NotebookKernelOutputRequiredLabel::ALL.to_vec(),
        surface_families: M5NotebookKernelOutputSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5NotebookKernelOutputDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5NotebookKernelOutputAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5NotebookKernelOutputConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "trust_class_label",
            "trust_class",
            "freshness_state",
            "presentation_class",
            "representation_label",
            "freshness_label",
            "copy_export_choice_note",
            "deep_link_kind",
        ]),
        source_contract_refs: banner_source_refs(),
        presents_stale_output_as_live: false,
        hides_trust_class_behind_hover_only: false,
        flattens_output_into_ambiguous_evidence: false,
        severs_output_provenance: false,
    }
}

/// Builds an output provenance chip group, deriving the origin and lineage classes, the
/// internal-origin and current-lineage claims, and the required notes from the honest inputs so the
/// seed is always self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn provenance_chip_group(
    group_id: &str,
    group_label: &str,
    provenance_kind: M5OutputProvenanceKind,
    provenance_state: M5OutputProvenanceState,
    cell_run_identity_label: &str,
    origin_class_label: &str,
    attached_artifacts_label: &str,
    persistence_retention_note: &str,
    context_note: &str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    chip_actions: Vec<OutputChipAction>,
    dispositions: Vec<M5NotebookKernelOutputDisposition>,
) -> OutputProvenanceChipGroup {
    let disclosure = resolve_output_provenance_chip_group(provenance_kind, provenance_state);
    OutputProvenanceChipGroup {
        component: M5NotebookKernelOutputComponentFamily::OutputProvenanceChipGroup,
        group_id: group_id.to_owned(),
        group_label: group_label.to_owned(),
        provenance_kind,
        provenance_state,
        origin_class: disclosure.origin_class,
        resolution_class: disclosure.resolution_class,
        claims_internal_origin: disclosure.is_internal_origin,
        claims_current_lineage: disclosure.may_claim_current_lineage,
        external_note: if disclosure.needs_external_note {
            "Not produced by this notebook; its origin is imported, restored, external, or unknown"
                .to_owned()
        } else {
            String::new()
        },
        partial_note: if disclosure.needs_partial_note {
            "Lineage only partially resolved; some of the producing run is missing".to_owned()
        } else {
            String::new()
        },
        missing_note: if disclosure.needs_missing_note {
            "Lineage unresolved; the producing run could not be attributed".to_owned()
        } else {
            String::new()
        },
        drift_note: if disclosure.needs_drift_note {
            "Execution count drifted from the run; this is not a current pinned lineage".to_owned()
        } else {
            String::new()
        },
        stale_note: if disclosure.needs_stale_note {
            "Lineage resolution is stale; re-resolve before trusting it as current".to_owned()
        } else {
            String::new()
        },
        cell_run_identity_label: cell_run_identity_label.to_owned(),
        origin_class_label: origin_class_label.to_owned(),
        attached_artifacts_label: attached_artifacts_label.to_owned(),
        persistence_retention_note: persistence_retention_note.to_owned(),
        context_note: context_note.to_owned(),
        deep_link_kind,
        deep_link_ref: deep_link_ref.to_owned(),
        chip_actions,
        dispositions,
        downgrade_triggers: chip_downgrade_triggers(),
        required_labels: M5NotebookKernelOutputRequiredLabel::ALL.to_vec(),
        surface_families: M5NotebookKernelOutputSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5NotebookKernelOutputDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5NotebookKernelOutputAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5NotebookKernelOutputConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "cell_run_identity_label",
            "provenance_kind",
            "provenance_state",
            "origin_class",
            "resolution_class",
            "origin_class_label",
            "attached_artifacts_label",
            "persistence_retention_note",
            "deep_link_kind",
        ]),
        source_contract_refs: chip_source_refs(),
        presents_stale_output_as_live: false,
        hides_trust_class_behind_hover_only: false,
        flattens_output_into_ambiguous_evidence: false,
        severs_output_provenance: false,
    }
}

fn trust_banners() -> Vec<OutputTrustBanner> {
    use DeepLinkKind as Link;
    use M5NotebookKernelOutputDisposition as Disp;
    use M5OutputFreshnessState as Fresh;
    use M5OutputTrustClass as Trust;
    use OutputBannerAction as Action;
    use OutputRepresentationMode as Repr;

    vec![
        // 1. Trusted + live → trusted local active content, live (may present as live).
        trust_banner(
            "banner-trusted-live",
            "Trusted local active output",
            Trust::TrustedOutput,
            Fresh::LiveOutput,
            Repr::RenderedRich,
            "Trust class: trusted local active content",
            "Representation: rendered rich (raw source available)",
            "Freshness: live (fresh from the current run)",
            "Copy and export preserve this trust class and let you choose the raw representation",
            "Output truth: what this output is and whether it is safe to act on now",
            Link::OutputViewer,
            "output:viewer/trusted-active",
            vec![
                Action::OpenRaw,
                Action::ExportOutput,
                Action::CopyOutput,
                Action::OpenDeepLink,
            ],
            vec![Disp::Ready, Disp::Active],
        ),
        // 2. Sanitized + stale → sanitized rich content, stale (needs sanitized + stale notes).
        trust_banner(
            "banner-sanitized-stale",
            "Sanitized rich output (stale)",
            Trust::SanitizedOutput,
            Fresh::StaleOutput,
            Repr::RenderedRich,
            "Trust class: sanitized rich content",
            "Representation: rendered rich (active content stripped; raw available)",
            "Freshness: stale after an edit or kernel change",
            "Copy and export preserve the sanitized trust class and the raw representation choice",
            "Output truth: sanitized rich content that no longer matches the current state",
            Link::NotebookLocation,
            "notebook:cell/analysis-plot",
            vec![
                Action::OpenRaw,
                Action::ExportOutput,
                Action::CopyOutput,
                Action::RerunToRefresh,
                Action::OpenDeepLink,
            ],
            vec![Disp::StaleOutput, Disp::Sanitized],
        ),
        // 3. Sandboxed + cached → isolated remote active content, cached
        //    (needs active-content + isolation + cached notes).
        trust_banner(
            "banner-sandboxed-cached",
            "Isolated remote active output (cached)",
            Trust::SandboxedOutput,
            Fresh::CachedOutput,
            Repr::RenderedRich,
            "Trust class: isolated remote active content",
            "Representation: rendered rich inside a sandbox (raw available)",
            "Freshness: cached from an earlier run (not live)",
            "Copy and export preserve the isolated trust class and the raw representation choice",
            "Output truth: active content isolated in a sandbox, served from cache",
            Link::OutputViewer,
            "output:viewer/sandboxed-active",
            vec![
                Action::OpenRaw,
                Action::ExportOutput,
                Action::CopyOutput,
                Action::OpenDeepLink,
            ],
            vec![Disp::Remote, Disp::Active],
        ),
        // 4. Raw active + superseded → plain text, superseded (needs stale note).
        trust_banner(
            "banner-raw-superseded",
            "Raw output shown as plain text (superseded)",
            Trust::RawActiveOutput,
            Fresh::SupersededOutput,
            Repr::RawSource,
            "Trust class: plain text (raw, never run as active content)",
            "Representation: raw source shown literally",
            "Freshness: superseded by a later run",
            "Copy and export preserve the plain-text representation and never run the raw content",
            "Output truth: an untrusted raw output shown inert and already superseded",
            Link::NotebookLocation,
            "notebook:cell/raw-html-dump",
            vec![
                Action::OpenRaw,
                Action::ExportOutput,
                Action::CopyOutput,
                Action::RerunToRefresh,
                Action::OpenDeepLink,
            ],
            vec![Disp::StaleOutput, Disp::Active],
        ),
        // 5. Blocked + cleared → blocked content, cleared (needs blocked + cleared notes).
        trust_banner(
            "banner-blocked-cleared",
            "Blocked output (cleared)",
            Trust::BlockedOutput,
            Fresh::ClearedOutput,
            Repr::RedactedRepresentation,
            "Trust class: blocked content",
            "Representation: redacted (raw available behind review)",
            "Freshness: cleared (no output rendered)",
            "Copy and export preserve the redacted representation and keep the raw behind review",
            "Output truth: an output withheld by policy and since cleared",
            Link::SupportBundle,
            "support:bundle/blocked-output",
            vec![Action::OpenRaw, Action::ExportOutput, Action::CopyOutput],
            vec![Disp::Sanitized, Disp::StaleOutput],
        ),
        // 6. Unknown trust + no output → unknown content, no output
        //    (needs unknown-trust + cleared notes).
        trust_banner(
            "banner-unknown-nooutput",
            "Unknown-trust output (no output)",
            Trust::UnknownTrust,
            Fresh::NoOutput,
            Repr::RawSource,
            "Trust class: unknown (treat as untrusted)",
            "Representation: raw source only",
            "Freshness: no output present",
            "Copy and export keep the raw representation and never assert a trust class we lack",
            "Output truth: an output whose trust class we could not determine and that is absent",
            Link::DocsAnchor,
            "docs:notebooks/output-trust-classes",
            vec![Action::OpenRaw, Action::ExportOutput, Action::CopyOutput],
            vec![Disp::ChooseAnotherKernel],
        ),
    ]
}

fn provenance_chip_groups() -> Vec<OutputProvenanceChipGroup> {
    use DeepLinkKind as Link;
    use M5NotebookKernelOutputDisposition as Disp;
    use M5OutputProvenanceKind as Kind;
    use M5OutputProvenanceState as State;
    use OutputChipAction as Action;

    vec![
        // 1. Produced by cell + complete → cell-produced, fully resolved (may claim current
        //    lineage).
        provenance_chip_group(
            "chips-cell-complete",
            "Cell-produced output (fully resolved)",
            Kind::ProducedByCell,
            State::ProvenanceComplete,
            "Cell `analysis-01`, run #12",
            "Origin: produced by a cell in this notebook",
            "Attached artifacts: 1 figure (fig-analysis-01)",
            "Persistence: retained in the notebook document; no external retention",
            "Provenance truth: which cell and run produced this output and how completely",
            Link::NotebookLocation,
            "notebook:cell/analysis-01",
            vec![
                Action::InspectProvenance,
                Action::ViewArtifacts,
                Action::CopyLineageIdentity,
                Action::ViewPersistence,
                Action::OpenDeepLink,
            ],
            vec![Disp::Ready, Disp::Active],
        ),
        // 2. Produced by run + pinned → run-produced, lineage pinned (may claim current lineage).
        provenance_chip_group(
            "chips-run-pinned",
            "Run-produced output (execution count pinned)",
            Kind::ProducedByRun,
            State::ExecutionCountPinned,
            "Run #12, cell `train-model`",
            "Origin: produced by a run of this notebook",
            "Attached artifacts: 1 metrics table, 1 model checkpoint ref",
            "Persistence: retained; checkpoint ref pinned to the run",
            "Provenance truth: a run-produced output with an execution count pinned to the run",
            Link::NotebookLocation,
            "notebook:run/12",
            vec![
                Action::InspectProvenance,
                Action::ViewArtifacts,
                Action::CopyLineageIdentity,
                Action::ViewPersistence,
                Action::OpenDeepLink,
            ],
            vec![Disp::Ready, Disp::Active],
        ),
        // 3. Imported + partial → imported origin, partially resolved
        //    (needs external + partial notes).
        provenance_chip_group(
            "chips-imported-partial",
            "Imported output (partial lineage)",
            Kind::ImportedOutput,
            State::ProvenancePartial,
            "Imported from `shared-analysis.ipynb`",
            "Origin: imported from another notebook",
            "Attached artifacts: 1 table (source run unknown)",
            "Persistence: retained; retention governed by the source workspace",
            "Provenance truth: an imported output whose producing run is only partly known",
            Link::DocsAnchor,
            "docs:notebooks/output-provenance",
            vec![
                Action::InspectProvenance,
                Action::ViewArtifacts,
                Action::CopyLineageIdentity,
                Action::OpenDeepLink,
            ],
            vec![Disp::StaleOutput, Disp::ChooseAnotherKernel],
        ),
        // 4. Restored + drifted → restored origin, lineage drifted (needs external + drift notes).
        provenance_chip_group(
            "chips-restored-drifted",
            "Restored output (execution count drifted)",
            Kind::RestoredOutput,
            State::ExecutionCountDrifted,
            "Restored from checkpoint `ckpt-08`, cell `eval`",
            "Origin: restored from a saved checkpoint",
            "Attached artifacts: 1 figure (restored)",
            "Persistence: retained from checkpoint; retention window applies",
            "Provenance truth: a restored output whose execution count no longer matches the run",
            Link::SupportBundle,
            "support:bundle/restored-output-lineage",
            vec![
                Action::InspectProvenance,
                Action::ViewArtifacts,
                Action::CopyLineageIdentity,
                Action::ViewPersistence,
                Action::OpenDeepLink,
            ],
            vec![Disp::StaleOutput, Disp::ChooseAnotherKernel],
        ),
        // 5. External + missing → external origin, unresolved (needs external + missing notes).
        provenance_chip_group(
            "chips-external-missing",
            "External output (lineage missing)",
            Kind::ExternalOutput,
            State::ProvenanceMissing,
            "External source (producing run not recorded)",
            "Origin: produced by an external source",
            "Attached artifacts: 1 image (no lineage)",
            "Persistence: not retained by this notebook; external retention only",
            "Provenance truth: an external output whose producing run could not be attributed",
            Link::DocsAnchor,
            "docs:notebooks/external-output-provenance",
            vec![
                Action::InspectProvenance,
                Action::ViewArtifacts,
                Action::CopyLineageIdentity,
                Action::OpenDeepLink,
            ],
            vec![Disp::StaleOutput, Disp::ChooseAnotherKernel],
        ),
        // 6. Unknown + stale → unknown origin, resolution stale (needs external + stale notes).
        provenance_chip_group(
            "chips-unknown-stale",
            "Unknown-origin output (stale lineage)",
            Kind::UnknownProvenance,
            State::ProvenanceStale,
            "Origin unknown; last resolved long ago",
            "Origin: could not be determined",
            "Attached artifacts: none recorded",
            "Persistence: unknown; treat retention as unverified",
            "Provenance truth: an output whose origin is unknown and whose lineage is stale",
            Link::NoDeepLink,
            "",
            vec![
                Action::InspectProvenance,
                Action::ViewArtifacts,
                Action::CopyLineageIdentity,
            ],
            vec![Disp::ChooseAnotherKernel],
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5NotebookKernelOutputDowngradeTrigger> {
    vec![
        M5NotebookKernelOutputDowngradeTrigger::OutputTrustUnstated,
        M5NotebookKernelOutputDowngradeTrigger::StaleOutputShownAsLive,
        M5NotebookKernelOutputDowngradeTrigger::TrustClassHoverOnly,
        M5NotebookKernelOutputDowngradeTrigger::ProvenanceSevered,
        M5NotebookKernelOutputDowngradeTrigger::AlternateStateLabelInvented,
        M5NotebookKernelOutputDowngradeTrigger::ProofStale,
    ]
}

fn output_review() -> OutputTrustProvenanceReview {
    OutputTrustProvenanceReview {
        banner_shows_trust_class: true,
        banner_shows_raw_vs_rendered: true,
        banner_shows_stale_state: true,
        banner_offers_open_raw_and_export: true,
        chip_shows_cell_run_identity: true,
        chip_shows_origin_class: true,
        chip_shows_attached_artifacts: true,
        chip_shows_persistence_or_retention: true,
        trust_and_provenance_derived_never_asserted: true,
        stale_output_never_presented_as_live: true,
        trust_class_never_hover_only: true,
        copy_export_preserves_trust_and_representation: true,
        output_provenance_never_severed: true,
        output_trust_visible_in_notebook_ai_support: true,
        every_next_step_names_stable_deep_link: true,
        banner_and_chip_consistent_across_surfaces: true,
        no_component_widens_export_scope_or_exposes_raw_by_default: true,
        components_stable_across_deployment_lines: true,
        no_surface_invents_alternate_state_label: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> OutputTrustProvenanceConsumerProjection {
    OutputTrustProvenanceConsumerProjection {
        output_viewer_reads_single_source: true,
        notebook_output_shows_trust_class: true,
        ai_context_shows_output_provenance: true,
        support_export_shows_trust_and_provenance: true,
        copy_export_preserves_representation: true,
        help_docs_shows_component_truth: true,
    }
}

fn proof_freshness() -> OutputTrustProvenanceProofFreshness {
    OutputTrustProvenanceProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_SCHEMA_REF,
        OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_DOC_REF,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_DOC_REF,
        M5_OUTPUT_TRUST_BANNER_SCHEMA_REF,
        M5_OUTPUT_PROVENANCE_CHIP_GROUP_SCHEMA_REF,
    ])
}

/// Builds the canonical output-trust-banner / output-provenance-chip-group controls packet.
pub fn seeded_output_trust_banner_output_provenance_chip_group_controls(
) -> OutputTrustBannerOutputProvenanceChipGroupControlsPacket {
    OutputTrustBannerOutputProvenanceChipGroupControlsPacket::new(
        OutputTrustBannerOutputProvenanceChipGroupControlsPacketInput {
            packet_id: OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_PACKET_ID.to_owned(),
            surface_label:
                "M5 output trust banners and output provenance chip groups: plain-text, sanitized-rich, trusted-local-active, and isolated-remote-active trust classes, stale-output honesty, and copy/export choice across claimed notebook outputs"
                    .to_owned(),
            trust_banners: trust_banners(),
            provenance_chip_groups: provenance_chip_groups(),
            downgrade_triggers: downgrade_triggers(),
            consumer_surfaces: M5NotebookKernelOutputConsumerSurface::ALL.to_vec(),
            output_review: output_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Scenario fixture: spotlights a stale output trust banner that must stay visibly stale and never
/// read as live truth. Every trust class, freshness state, and presentation class stays covered so
/// the fixture validates on its own.
pub fn seeded_output_trust_banner_output_provenance_chip_group_controls_output_trust_banner_stale(
) -> OutputTrustBannerOutputProvenanceChipGroupControlsPacket {
    let mut packet = seeded_output_trust_banner_output_provenance_chip_group_controls();
    packet.packet_id =
        "m5-output-trust-banner-output-provenance-chip-group-controls:fixture:output-trust-banner-stale"
            .to_owned();
    packet.surface_label =
        "M5 output trust banners: a stale output stays visibly stale and never reads as live truth"
            .to_owned();
    packet
}

/// Scenario fixture: spotlights a drifted output provenance chip group that must never claim a
/// current pinned lineage. Every provenance kind, provenance state, origin class, and lineage
/// resolution stays covered so the fixture validates on its own.
pub fn seeded_output_trust_banner_output_provenance_chip_group_controls_output_provenance_chip_group_drifted(
) -> OutputTrustBannerOutputProvenanceChipGroupControlsPacket {
    let mut packet = seeded_output_trust_banner_output_provenance_chip_group_controls();
    packet.packet_id =
        "m5-output-trust-banner-output-provenance-chip-group-controls:fixture:output-provenance-chip-group-drifted"
            .to_owned();
    packet.surface_label =
        "M5 output provenance chip groups: a drifted lineage never claims a current pinned lineage"
            .to_owned();
    packet
}
