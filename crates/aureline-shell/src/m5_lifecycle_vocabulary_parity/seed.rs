//! Canonical seed builders for the M5 lifecycle-vocabulary parity proof.
//!
//! These builders are the single producer of the checked-in packet, dashboard, support-export, and
//! CSV artifacts plus the narrowed fixtures. The headless emitter and the inline tests both call
//! them so the in-code parity proof, the artifacts, and the fixtures never drift. Every term
//! grounding each row carries — the object families that admit the controlled term, the declared
//! consumer surfaces, and the applicable downgrade triggers — is pulled straight from the frozen
//! lifecycle matrix's seeded packet, so the certification cannot audit a term the matrix does not
//! freeze, and the groundings are derived from the matrix rather than restated by hand.

use super::*;
use crate::freeze_the_m5_lifecycle_state_and_journey_checkpoint_matrix::{
    seeded_m5_lifecycle_matrix, M5_LIFECYCLE_MATRIX_PACKET_ID,
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

/// The vocabulary-parity posture seeded for one controlled term.
struct TermSpec {
    /// When set, the evaluated-surface set used instead of the matrix-derived set (hand-built
    /// checks use this to prove a partial certification blocks).
    evaluated_surfaces_override: Option<Vec<M5LifecycleConsumerSurface>>,
    cross_surface_term: CrossSurfaceTermState,
    semantic_distinction: SemanticDistinctionState,
    export_code_parity: ExportCodeParityState,
    published_copy_narrowing: PublishedCopyNarrowingState,
    headless_parity_preserved: bool,
    waiver: Option<VocabularyParityWaiver>,
    narrowing_reason: Option<&'static str>,
}

impl TermSpec {
    /// A full-parity posture: all four dimensions hold and headless parity is preserved.
    fn stable() -> Self {
        Self {
            evaluated_surfaces_override: None,
            cross_surface_term: CrossSurfaceTermState::TermStableAcrossAllSurfaces,
            semantic_distinction: SemanticDistinctionState::DistinctMeaningPreserved,
            export_code_parity: ExportCodeParityState::CodeExportsIdenticallyAllPaths,
            published_copy_narrowing: PublishedCopyNarrowingState::CopyAutoNarrowsOnStateChange,
            headless_parity_preserved: true,
            waiver: None,
            narrowing_reason: None,
        }
    }
}

/// Short reviewer-facing label for a controlled term.
fn state_label(state: M5LifecycleState) -> &'static str {
    match state {
        M5LifecycleState::Ready => "Ready",
        M5LifecycleState::Warming => "Warming",
        M5LifecycleState::Partial => "Partial",
        M5LifecycleState::Stale => "Stale",
        M5LifecycleState::Rebuilding => "Rebuilding",
        M5LifecycleState::Restricted => "Restricted",
        M5LifecycleState::PolicyBlocked => "Policy blocked",
        M5LifecycleState::Reconnecting => "Reconnecting",
        M5LifecycleState::Degraded => "Degraded",
        M5LifecycleState::ReadOnlyDegraded => "Read-only degraded",
        M5LifecycleState::Unavailable => "Unavailable",
        M5LifecycleState::RollbackAvailable => "Rollback available",
        M5LifecycleState::Deprecated => "Deprecated",
        M5LifecycleState::Experimental => "Experimental",
        M5LifecycleState::RetestPending => "Retest pending",
    }
}

/// Object families whose explicit state machine admits `state`, in canonical family order. Pulled
/// from the frozen matrix so a row cannot claim a term no governed object admits.
fn admitting_object_families(state: M5LifecycleState) -> Vec<M5LifecycleObjectFamily> {
    let matrix = seeded_m5_lifecycle_matrix();
    M5LifecycleObjectFamily::ALL
        .into_iter()
        .filter(|family| {
            matrix
                .object_state_rows
                .iter()
                .find(|row| row.object_family == *family)
                .is_some_and(|row| row.admitted_states.contains(&state))
        })
        .collect()
}

/// The union of downgrade triggers declared by every object family that admits `state`, in
/// canonical trigger order. Pulled from the frozen matrix.
fn applicable_downgrade_triggers(state: M5LifecycleState) -> Vec<M5LifecycleDowngradeTrigger> {
    let matrix = seeded_m5_lifecycle_matrix();
    let families = admitting_object_families(state);
    M5LifecycleDowngradeTrigger::ALL
        .into_iter()
        .filter(|trigger| {
            matrix.object_state_rows.iter().any(|row| {
                families.contains(&row.object_family) && row.downgrade_triggers.contains(trigger)
            })
        })
        .collect()
}

/// Builds one parity row from a controlled term and a parity posture. The admitting object
/// families, the required consumer surfaces, and the applicable downgrade triggers are all pulled
/// from the frozen matrix.
fn row_from_state(state: M5LifecycleState, spec: TermSpec) -> VocabularyParityRow {
    let required_consumer_surfaces = required_consumer_surfaces();
    let evaluated_consumer_surfaces = spec
        .evaluated_surfaces_override
        .unwrap_or_else(|| required_consumer_surfaces.clone());
    let mut row = VocabularyParityRow {
        state,
        state_label: state_label(state).to_owned(),
        admitting_object_families: admitting_object_families(state),
        required_consumer_surfaces,
        evaluated_consumer_surfaces,
        cross_surface_term: spec.cross_surface_term,
        semantic_distinction: spec.semantic_distinction,
        export_code_parity: spec.export_code_parity,
        published_copy_narrowing: spec.published_copy_narrowing,
        headless_parity_preserved: spec.headless_parity_preserved,
        applicable_downgrade_triggers: applicable_downgrade_triggers(state),
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: VocabularyParityStatus::Green,
        term_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.term_causes = row.recompute_causes();
    row
}

/// Builds the experimental surface-paraphrase waiver carried by the seed.
fn experimental_surface_paraphrase_waiver() -> VocabularyParityWaiver {
    VocabularyParityWaiver {
        waiver_id: "waiver:experimental-surface-paraphrase:0001".to_owned(),
        state: M5LifecycleState::Experimental,
        reason: "Release notes present the controlled `experimental` term as a disclosed \
                 reader-facing \"early access\" label while still binding it to the same \
                 experimental status token in every export, so the paraphrase is disclosed and \
                 waivered rather than drifting into a private synonym, and the controlled token is \
                 restored across surfaces when the term qualifies."
            .to_owned(),
        owner_role: "Release notes owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// Returns the seeded vocabulary-parity posture for one controlled term.
fn term_spec(state: M5LifecycleState) -> TermSpec {
    match state {
        M5LifecycleState::Experimental => TermSpec {
            // Release notes present `experimental` as a disclosed, waivered "early access" label
            // bound to the same controlled token.
            cross_surface_term: CrossSurfaceTermState::DisclosedSurfaceParaphrase,
            waiver: Some(experimental_surface_paraphrase_waiver()),
            narrowing_reason: Some(
                "Release notes present the controlled `experimental` term as a disclosed, waivered \
                 reader-facing \"early access\" label while still binding it to the same \
                 experimental status token everywhere else, so the term is narrowed and disclosed \
                 rather than paraphrased into a private synonym.",
            ),
            ..TermSpec::stable()
        },
        M5LifecycleState::ReadOnlyDegraded => TermSpec {
            // A compact status surface groups `read_only_degraded` under a disclosed "Degraded"
            // family header while still naming it individually.
            semantic_distinction: SemanticDistinctionState::DisclosedGroupedPresentation,
            narrowing_reason: Some(
                "A compact status surface groups the controlled `read_only_degraded` term under a \
                 disclosed \"Degraded\" family header while still naming it individually and keeping \
                 its distinct read-only meaning, so the term is narrowed and disclosed rather than \
                 collapsing into a generic degraded state.",
            ),
            ..TermSpec::stable()
        },
        M5LifecycleState::PolicyBlocked => TermSpec {
            // Telemetry exports a disclosed coarse policy code until the specific block class is
            // finalized, while still naming the same controlled state.
            export_code_parity: ExportCodeParityState::DisclosedPartialExport,
            narrowing_reason: Some(
                "Telemetry exports a disclosed coarse policy code for the controlled \
                 `policy_blocked` term until the specific block class is finalized, while still \
                 naming the same controlled state everywhere, so the export is narrowed and \
                 disclosed rather than losing the code.",
            ),
            ..TermSpec::stable()
        },
        M5LifecycleState::Deprecated => TermSpec {
            // Deprecated docs/help copy narrows through a disclosed manual publish step rather than
            // automatically.
            published_copy_narrowing: PublishedCopyNarrowingState::DisclosedManualNarrowing,
            narrowing_reason: Some(
                "Published docs/help copy for the controlled `deprecated` term narrows through a \
                 disclosed manual publish step rather than automatically, so the copy is narrowed \
                 and disclosed rather than left overclaiming after the term is superseded.",
            ),
            ..TermSpec::stable()
        },
        // Every other controlled term holds full parity across all four dimensions.
        _ => TermSpec::stable(),
    }
}

/// Builds the parity rows for the canonical seed, one per controlled term.
fn seeded_rows() -> Vec<VocabularyParityRow> {
    M5LifecycleState::ALL
        .iter()
        .map(|&state| row_from_state(state, term_spec(state)))
        .collect()
}

/// Builds a variant where one term's spec is mutated after the canonical spec is resolved, used by
/// the blocked fixtures.
fn seeded_rows_with<F>(target: M5LifecycleState, mutate: F) -> Vec<VocabularyParityRow>
where
    F: Fn(&mut TermSpec),
{
    M5LifecycleState::ALL
        .iter()
        .map(|&state| {
            let mut spec = term_spec(state);
            if state == target {
                mutate(&mut spec);
            }
            row_from_state(state, spec)
        })
        .collect()
}

fn packet_from_rows(rows: Vec<VocabularyParityRow>) -> VocabularyParityPacket {
    build_m5_lifecycle_vocabulary_parity_packet(VocabularyParityInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_LIFECYCLE_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 lifecycle-vocabulary parity packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and CSV
/// artifacts. Eleven controlled terms keep full parity (green). `experimental` auto-narrows to
/// yellow with a waivered release-notes surface paraphrase, `read_only_degraded` auto-narrows to
/// yellow disclosing a grouped presentation, `policy_blocked` auto-narrows to yellow disclosing a
/// partial telemetry export, and `deprecated` auto-narrows to yellow disclosing a manual copy
/// narrowing — and no row is blocked, so the packet is clean and every row is publishable.
pub fn seeded_m5_lifecycle_vocabulary_parity_packet() -> VocabularyParityPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the `reconnecting` term means different things on different surfaces,
/// proving a cross-surface meaning drift blocks promotion (red) rather than staying a disclosed
/// yellow.
pub fn seeded_m5_lifecycle_vocabulary_parity_packet_reconnecting_term_drift_blocked(
) -> VocabularyParityPacket {
    let rows = seeded_rows_with(M5LifecycleState::Reconnecting, |spec| {
        spec.cross_surface_term = CrossSurfaceTermState::TermMeaningDriftedAcrossSurfaces;
        spec.narrowing_reason = Some(
            "The controlled `reconnecting` term meant \"actively reconnecting\" in the product UI \
             but was reused for \"connection lost\" in the CLI and telemetry, so the same term \
             described a different state depending on the surface, and the term blocks before \
             keeping a vocabulary claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the `retest_pending` term collapses into generic failure wording,
/// proving a semantic collapse blocks promotion (red) rather than staying a disclosed yellow.
pub fn seeded_m5_lifecycle_vocabulary_parity_packet_retest_pending_generic_collapse_blocked(
) -> VocabularyParityPacket {
    let rows = seeded_rows_with(M5LifecycleState::RetestPending, |spec| {
        spec.semantic_distinction = SemanticDistinctionState::CollapsedIntoGenericFailure;
        spec.narrowing_reason = Some(
            "The controlled `retest_pending` term was rendered as a generic \"failed\" error on the \
             product UI and diagnostics, so an awaiting-re-test state could no longer be told apart \
             from an ordinary failure, and the term blocks before keeping a vocabulary claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the `policy_blocked` term's status code stops exporting, proving an
/// unexportable status code blocks promotion (red) rather than staying a disclosed yellow.
pub fn seeded_m5_lifecycle_vocabulary_parity_packet_policy_blocked_status_code_unexportable_blocked(
) -> VocabularyParityPacket {
    let rows = seeded_rows_with(M5LifecycleState::PolicyBlocked, |spec| {
        spec.export_code_parity = ExportCodeParityState::StatusCodeUnexportable;
        spec.narrowing_reason = Some(
            "The controlled `policy_blocked` term's stable status code stopped exporting on the \
             support and telemetry paths, so diagnostics could no longer read the same code the UI \
             shows, and the term blocks before keeping a vocabulary claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the `deprecated` term's published copy stays stale and overclaims,
/// proving stale overclaiming copy blocks promotion (red) rather than staying a disclosed yellow.
pub fn seeded_m5_lifecycle_vocabulary_parity_packet_deprecated_stale_copy_overclaims_blocked(
) -> VocabularyParityPacket {
    let rows = seeded_rows_with(M5LifecycleState::Deprecated, |spec| {
        spec.published_copy_narrowing = PublishedCopyNarrowingState::StaleCopyOverclaims;
        spec.narrowing_reason = Some(
            "After the capability was deprecated, the published release/docs/help copy still \
             claimed the controlled `deprecated` term as fully supported, so published wording \
             overclaimed the current state, and the term blocks before keeping a vocabulary claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the `experimental` term loses the shared state-truth vocabulary in a
/// headless execution, proving a headless/companion-adjacent parity loss blocks promotion (red)
/// rather than staying yellow.
pub fn seeded_m5_lifecycle_vocabulary_parity_packet_experimental_headless_parity_lost_blocked(
) -> VocabularyParityPacket {
    let rows = seeded_rows_with(M5LifecycleState::Experimental, |spec| {
        spec.headless_parity_preserved = false;
        spec.narrowing_reason = Some(
            "In headless / CLI execution the controlled `experimental` term was reported through a \
             private capability-state vocabulary that diverged from the controlled lifecycle term \
             shown in-product, so the same state described a different state language depending on \
             how it ran, and the term blocks before keeping a vocabulary claim.",
        );
    });
    packet_from_rows(rows)
}
