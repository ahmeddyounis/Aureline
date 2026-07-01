//! Canonical seed builders for the M5 resume-breadcrumb proof.
//!
//! These builders are the single producer of the checked-in packet, dashboard, support-export, and
//! CSV artifacts plus the narrowed fixtures. The headless emitter and the inline tests both call
//! them so the in-code certification proof, the artifacts, and the fixtures never drift. Every
//! attribute each family row certifies over — the driving matrix journey, the explicit state machine
//! (admitted states), the named recovery affordance the not-resumed disclosure anchors on, the
//! checkpoint lineage the breadcrumb replays, the declared consumer surfaces, the applicable
//! downgrade triggers, and the controlled last-failure reason classes — is pulled straight from the
//! frozen lifecycle matrix's seeded packet, so the certification cannot audit a family the matrix
//! does not anchor, and the bindings are derived from the matrix rather than restated by hand. Only
//! the provenance classes distinguished, the lineage facets preserved, the per-family posture, and
//! the scope summary are authored here.

use super::*;
use crate::freeze_the_m5_lifecycle_state_and_journey_checkpoint_matrix::{
    seeded_m5_lifecycle_matrix, M5JourneyCheckpointRow, M5ObjectStateRow,
    M5_LIFECYCLE_MATRIX_PACKET_ID,
};

/// Deterministic generated-at value carried by the seeded packet.
const GENERATED_AT: &str = "2026-06-30T00:00:00Z";

/// Frozen, representative exact-build identity ref used by the seed.
///
/// A live runtime stamps the exact build identity here; the seed uses a fixed value so the
/// checked-in fixtures stay reproducible.
pub const SEED_BUILD_IDENTITY_REF: &str =
    "build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2";

/// Frozen, representative release-channel class used by the seed.
pub const SEED_RELEASE_CHANNEL_CLASS: &str = "stable";

/// The breadcrumb posture seeded for one object family.
struct FamilySpec {
    /// Short breadcrumb scope summary.
    scope_summary: &'static str,
    /// The provenance classes this row distinguishes (defaults to all four).
    distinguished_provenance_classes: Vec<M5ResumeProvenanceClass>,
    /// The lineage facets this row preserves (defaults to all four).
    preserved_lineage_facets: Vec<M5BreadcrumbLineageFacet>,
    /// When set, the evaluated-surface set used instead of the object's declared set (blocked
    /// fixtures use this to prove a partial certification blocks).
    evaluated_surfaces_override: Option<Vec<M5LifecycleConsumerSurface>>,
    provenance_labeling: ProvenanceLabelingState,
    lineage_breadcrumb: LineageBreadcrumbState,
    not_resumed_disclosure: NotResumedDisclosureState,
    capture_parity: CaptureParityState,
    headless_parity_preserved: bool,
    waiver: Option<ResumeBreadcrumbWaiver>,
    narrowing_reason: Option<&'static str>,
}

/// Short reviewer-facing label for an object family.
fn object_label(family: M5LifecycleObjectFamily) -> &'static str {
    match family {
        M5LifecycleObjectFamily::Workspace => "Workspace / window session",
        M5LifecycleObjectFamily::Extension => "Installed extension",
        M5LifecycleObjectFamily::RemoteSession => "Remote / tunnel session",
        M5LifecycleObjectFamily::CollaborationSession => "Collaboration session",
        M5LifecycleObjectFamily::AiAction => "AI assistant action",
        M5LifecycleObjectFamily::UpdateRollback => "Update / rollback",
        M5LifecycleObjectFamily::NotebookRuntime => "Notebook runtime",
        M5LifecycleObjectFamily::RequestApiRun => "Request / API run",
        M5LifecycleObjectFamily::PreviewSession => "Preview / live-server session",
        M5LifecycleObjectFamily::PipelineRun => "Pipeline / task run",
        M5LifecycleObjectFamily::DataSession => "Data / database session",
        M5LifecycleObjectFamily::ProfilerCapture => "Profiler / trace capture",
        M5LifecycleObjectFamily::CompanionSession => "Companion / paired-device session",
    }
}

/// Returns the frozen matrix object-state row for a family.
fn matrix_object_row(object_family: M5LifecycleObjectFamily) -> M5ObjectStateRow {
    seeded_m5_lifecycle_matrix()
        .object_state_rows
        .into_iter()
        .find(|row| row.object_family == object_family)
        .expect("frozen lifecycle matrix declares every governed object family")
}

/// Returns the frozen matrix journey-checkpoint row that drives a family.
fn matrix_journey_row(object_family: M5LifecycleObjectFamily) -> M5JourneyCheckpointRow {
    seeded_m5_lifecycle_matrix()
        .journey_checkpoint_rows
        .into_iter()
        .find(|row| row.object_family == object_family)
        .expect("frozen lifecycle matrix declares a journey for every governed object family")
}

/// Builds one certification row from an object family and a breadcrumb posture. Every binding — the
/// driving matrix journey, the object's qualification, owner, state machine (admitted states),
/// recovery affordance, last-failure reason classes, checkpoint lineage, declared consumer surfaces,
/// and downgrade triggers — is pulled from the frozen matrix rows for the family.
fn row_from_family(family: M5LifecycleObjectFamily, spec: FamilySpec) -> ResumeBreadcrumbRow {
    let object = matrix_object_row(family);
    let journey = matrix_journey_row(family);
    let required_consumer_surfaces = object.consumer_surfaces.clone();
    let evaluated_consumer_surfaces = spec
        .evaluated_surfaces_override
        .unwrap_or_else(|| object.consumer_surfaces.clone());
    let mut row = ResumeBreadcrumbRow {
        object_family: family,
        object_label: object_label(family).to_owned(),
        matrix_journey: journey.journey,
        qualification: object.qualification,
        owner_role: object.owner_role.clone(),
        scope_summary: spec.scope_summary.to_owned(),
        admitted_states: object.admitted_states.clone(),
        recovery_affordance: object.recovery_affordance,
        last_failure_reason_classes: object.last_failure_reason_classes.clone(),
        checkpoint_lineage: journey.checkpoints.clone(),
        distinguished_provenance_classes: spec.distinguished_provenance_classes,
        preserved_lineage_facets: spec.preserved_lineage_facets,
        required_consumer_surfaces,
        evaluated_consumer_surfaces,
        provenance_labeling: spec.provenance_labeling,
        lineage_breadcrumb: spec.lineage_breadcrumb,
        not_resumed_disclosure: spec.not_resumed_disclosure,
        capture_parity: spec.capture_parity,
        headless_parity_preserved: spec.headless_parity_preserved,
        applicable_downgrade_triggers: object.downgrade_triggers.clone(),
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: ResumeBreadcrumbStatus::Green,
        breadcrumb_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.breadcrumb_causes = row.recompute_causes();
    row
}

/// Builds the collaboration grouped-not-resumed waiver carried by the seed.
fn collaboration_grouped_not_resumed_waiver() -> ResumeBreadcrumbWaiver {
    ResumeBreadcrumbWaiver {
        waiver_id: "waiver:collaboration-grouped-not-resumed:0001".to_owned(),
        object_family: M5LifecycleObjectFamily::CollaborationSession,
        reason:
            "When a collaboration session reconnects after a dropped shared connection, the journey \
             discloses a grouped summary of the actions it intentionally did not rerun or \
             reauthorize — the pending control-transfer requests and outbound presence broadcasts \
             are named as one withheld category rather than each request individually — while still \
             disclosing that actions were withheld and offering the reconnect affordance to \
             reauthorize them. The grouped summary is disclosed, never silent, and the itemized \
             not-resumed set is restored the moment the collaboration lane rejoins."
                .to_owned(),
        owner_role: "Collaboration owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// A full-breadcrumb posture: all four breadcrumb dimensions hold, all four provenance classes and
/// lineage facets are present, and headless parity is preserved.
fn full(scope_summary: &'static str) -> FamilySpec {
    FamilySpec {
        scope_summary,
        distinguished_provenance_classes: M5ResumeProvenanceClass::ALL.to_vec(),
        preserved_lineage_facets: M5BreadcrumbLineageFacet::ALL.to_vec(),
        evaluated_surfaces_override: None,
        provenance_labeling: ProvenanceLabelingState::ProvenanceClassLabeledOnEverySurface,
        lineage_breadcrumb: LineageBreadcrumbState::SourceActorBoundaryCheckpointPreserved,
        not_resumed_disclosure: NotResumedDisclosureState::NotResumedActionsExplicit,
        capture_parity: CaptureParityState::BreadcrumbsCapturedInExportAndScreenshot,
        headless_parity_preserved: true,
        waiver: None,
        narrowing_reason: None,
    }
}

/// Returns the seeded breadcrumb posture for one object family.
fn family_spec(family: M5LifecycleObjectFamily) -> FamilySpec {
    use M5LifecycleObjectFamily as F;
    match family {
        F::Workspace => full(
            "Workspace restore breadcrumbs distinguish a freshly computed layout (live truth) from a \
             snapshot rehydration (restored context), a stale open-editor preview (cached evidence), \
             and a session that needs an explicit reopen (restart-required placeholder), naming the \
             source, restoring subsystem, host, and checkpoint each resumed from",
        ),
        F::Extension => full(
            "Extension activation breadcrumbs name whether a capability is live, restored from its \
             last enabled state, cached, or a reinstall-required placeholder, and disclose any \
             entitlement it did not silently reauthorize",
        ),
        F::RemoteSession => full(
            "Remote reconnect breadcrumbs name the source, the reconnecting subsystem, the host or \
             boundary crossed, and the checkpoint the tunnel resumed from, and make explicit which \
             forwarded ports and trust grants were intentionally not reauthorized",
        ),
        F::AiAction => full(
            "AI action breadcrumbs distinguish a live run from a restored draft, cached context, or \
             a reauthorize-required placeholder, and make explicit which tool calls and file writes \
             it intentionally did not replay after a restore",
        ),
        F::UpdateRollback => full(
            "Update / rollback breadcrumbs name whether the shown state is the live update, the \
             restored prior version, cached release notes, or a restart-required placeholder, and \
             make explicit any migration it intentionally did not rerun",
        ),
        F::NotebookRuntime => full(
            "Notebook runtime breadcrumbs distinguish a live kernel from a restored session, cached \
             outputs, or a restart-required placeholder, and make explicit which cells were \
             intentionally not re-executed on reconnect",
        ),
        F::RequestApiRun => full(
            "Request / API run breadcrumbs name whether a response is live, restored from history, \
             cached, or a re-send-required placeholder, and make explicit which side-effecting \
             requests were intentionally not replayed",
        ),
        F::PipelineRun => full(
            "Pipeline run breadcrumbs name the source, the executing subsystem, the host, and the \
             checkpoint each stage resumed from, and make explicit which downstream stages were \
             intentionally not rerun after a partial replay",
        ),
        F::DataSession => full(
            "Data session breadcrumbs distinguish a live connection from a restored session, cached \
             result sets, or a reconnect-required placeholder, and make explicit which uncommitted \
             transactions were intentionally not reapplied",
        ),
        // Companion discloses a coarse provenance grouping on the small paired-device surface (yellow).
        F::CompanionSession => FamilySpec {
            provenance_labeling: ProvenanceLabelingState::DisclosedCoarseProvenanceGrouping,
            narrowing_reason: Some(
                "On the small companion / paired-device surface the session presents a disclosed \
                 coarse provenance grouping — restored context and cached evidence are grouped under \
                 one disclosed recovered-context header while live truth and the restart-required \
                 placeholder stay distinct — so the companion breadcrumb is narrowed and disclosed \
                 rather than leaving the provenance ambiguous.",
            ),
            ..full(
                "Companion session breadcrumbs distinguish live truth, restored context, cached \
                 evidence, and a restart-required placeholder, grouping the two recovered classes \
                 under one disclosed header on the small paired-device surface",
            )
        },
        // Preview discloses a partial lineage breadcrumb on the compact preview strip (yellow).
        F::PreviewSession => FamilySpec {
            lineage_breadcrumb: LineageBreadcrumbState::DisclosedPartialLineageBreadcrumb,
            narrowing_reason: Some(
                "On the compact preview status strip the preview session shows a disclosed partial \
                 lineage breadcrumb — the host/boundary facet is dropped while the source class, \
                 rebuilding subsystem, and checkpoint lineage are still named — so the preview \
                 breadcrumb is narrowed and disclosed rather than collapsing into generic recovered \
                 wording.",
            ),
            ..full(
                "Preview build breadcrumbs name the source, the rebuilding subsystem, the host, and \
                 the checkpoint each preview resumed from, dropping only the host/boundary facet on \
                 the compact strip",
            )
        },
        // Profiler captures a disclosed reduced subset of breadcrumb detail in its compact export (yellow).
        F::ProfilerCapture => FamilySpec {
            capture_parity: CaptureParityState::DisclosedPartialCapture,
            narrowing_reason: Some(
                "The profiler capture exports a disclosed reduced subset of its breadcrumb detail — \
                 intermediate lineage steps are collapsed in the compact trace export while the \
                 provenance header and terminal breadcrumb are still captured — so the captured \
                 breadcrumb truth is narrowed and disclosed rather than absent from the export.",
            ),
            ..full(
                "Profiler capture breadcrumbs distinguish a live trace from a restored capture, \
                 cached samples, or a recapture-required placeholder, capturing a reduced breadcrumb \
                 subset in the compact export",
            )
        },
        // Collaboration discloses a grouped not-resumed summary under a waiver (yellow).
        F::CollaborationSession => FamilySpec {
            not_resumed_disclosure: NotResumedDisclosureState::DisclosedGroupedNotResumedSummary,
            waiver: Some(collaboration_grouped_not_resumed_waiver()),
            narrowing_reason: Some(
                "When a collaboration session reconnects, the journey discloses a grouped, waivered \
                 summary of the actions it intentionally did not rerun or reauthorize — the pending \
                 control-transfer requests and outbound presence broadcasts are named as one \
                 withheld category rather than each individually — while still disclosing that \
                 actions were withheld and offering the reconnect affordance, so the collaboration \
                 breadcrumb is narrowed and disclosed rather than leaving the not-resumed set \
                 silently absent.",
            ),
            ..full(
                "Collaboration join breadcrumbs name the source, the reconnecting subsystem, the \
                 host, and the checkpoint each rejoin resumed from, disclosing a grouped summary of \
                 the actions intentionally not reauthorized on reconnect",
            )
        },
    }
}

/// Builds the certification rows for the canonical seed, one per object family.
fn seeded_rows() -> Vec<ResumeBreadcrumbRow> {
    M5LifecycleObjectFamily::ALL
        .iter()
        .map(|&family| row_from_family(family, family_spec(family)))
        .collect()
}

/// Builds a variant where one family's spec is mutated after the canonical spec is resolved, used by
/// the blocked fixtures.
fn seeded_rows_with<F>(target: M5LifecycleObjectFamily, mutate: F) -> Vec<ResumeBreadcrumbRow>
where
    F: Fn(&mut FamilySpec),
{
    M5LifecycleObjectFamily::ALL
        .iter()
        .map(|&family| {
            let mut spec = family_spec(family);
            if family == target {
                mutate(&mut spec);
            }
            row_from_family(family, spec)
        })
        .collect()
}

fn packet_from_rows(rows: Vec<ResumeBreadcrumbRow>) -> ResumeBreadcrumbPacket {
    build_m5_resume_breadcrumbs_packet(ResumeBreadcrumbInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_LIFECYCLE_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 resume-breadcrumb packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and CSV
/// artifacts. Nine families keep full breadcrumbs (green). The companion session auto-narrows to
/// yellow disclosing a coarse provenance grouping, the preview session auto-narrows to yellow
/// disclosing a partial lineage breadcrumb, the profiler capture auto-narrows to yellow disclosing a
/// partial capture, and the collaboration session auto-narrows to yellow with a waivered grouped
/// not-resumed summary — and no row is blocked, so the packet is clean and every row is publishable.
pub fn seeded_m5_resume_breadcrumbs_packet() -> ResumeBreadcrumbPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the notebook runtime leaves its provenance class ambiguous or missing,
/// proving that failing to distinguish restored/cached/live/placeholder blocks promotion (red)
/// rather than staying a disclosed yellow.
pub fn seeded_m5_resume_breadcrumbs_packet_notebook_provenance_ambiguous_blocked(
) -> ResumeBreadcrumbPacket {
    let rows = seeded_rows_with(M5LifecycleObjectFamily::NotebookRuntime, |spec| {
        spec.provenance_labeling = ProvenanceLabelingState::ProvenanceClassAmbiguousOrMissing;
        spec.narrowing_reason = Some(
            "After a notebook kernel reconnect, the runtime showed its restored outputs and cached \
             results under the same live header, leaving the provenance class ambiguous — a user \
             could not tell whether an output was live truth, restored context, or cached evidence — \
             so the runtime blocks before keeping a breadcrumb claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the remote session shows only generic "recovered" wording with no lineage,
/// proving that a generic recovered label blocks promotion (red) rather than staying green.
pub fn seeded_m5_resume_breadcrumbs_packet_remote_generic_recovered_blocked(
) -> ResumeBreadcrumbPacket {
    let rows = seeded_rows_with(M5LifecycleObjectFamily::RemoteSession, |spec| {
        spec.lineage_breadcrumb = LineageBreadcrumbState::GenericRecoveredWordingOnly;
        spec.narrowing_reason = Some(
            "After a dropped tunnel, the remote session labeled its restored state with a bare \
             \"recovered\" banner naming no source class, no reconnecting subsystem, no host or \
             boundary, and no checkpoint lineage, so neither the user nor support could attribute \
             the recovered value, and the session blocks before keeping a breadcrumb claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the data session silently drops the actions it did not rerun or
/// reauthorize, proving that a silently-absent not-resumed set blocks promotion (red) rather than
/// staying a disclosed yellow.
pub fn seeded_m5_resume_breadcrumbs_packet_data_not_resumed_silent_blocked(
) -> ResumeBreadcrumbPacket {
    let rows = seeded_rows_with(M5LifecycleObjectFamily::DataSession, |spec| {
        spec.not_resumed_disclosure = NotResumedDisclosureState::NotResumedActionsSilentlyAbsent;
        spec.narrowing_reason = Some(
            "When the data session reconnected, it silently discarded the uncommitted transactions \
             it did not reapply and gave no disclosure that writes had been withheld, so the user \
             could not tell what Aureline intentionally did not do, and the session blocks before \
             keeping a breadcrumb claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the AI action's breadcrumbs do not survive capture, proving that
/// breadcrumbs absent from export/screenshot/support capture block promotion (red) rather than
/// staying green.
pub fn seeded_m5_resume_breadcrumbs_packet_ai_capture_absent_blocked() -> ResumeBreadcrumbPacket {
    let rows = seeded_rows_with(M5LifecycleObjectFamily::AiAction, |spec| {
        spec.capture_parity = CaptureParityState::BreadcrumbsAbsentFromCapture;
        spec.narrowing_reason = Some(
            "The AI action rendered its provenance header and lineage breadcrumb only in a transient \
             overlay that a screenshot, support packet, and export all dropped, so support could not \
             reproduce whether an applied change was live, restored, or cached, and the action \
             blocks before keeping a breadcrumb claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the extension loses the shared state-truth vocabulary in a headless
/// execution, proving that a headless/companion-adjacent parity loss blocks promotion (red) rather
/// than staying green.
pub fn seeded_m5_resume_breadcrumbs_packet_extension_headless_parity_lost_blocked(
) -> ResumeBreadcrumbPacket {
    let rows = seeded_rows_with(M5LifecycleObjectFamily::Extension, |spec| {
        spec.headless_parity_preserved = false;
        spec.narrowing_reason = Some(
            "In headless / CLI execution the extension reported a private provenance and lineage \
             vocabulary that diverged from the controlled breadcrumbs shown in-product, so the same \
             capability described its restored and cached state with a different language depending \
             on how it ran, and the extension blocks before keeping a breadcrumb claim.",
        );
    });
    packet_from_rows(rows)
}
