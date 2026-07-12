//! Canonical seed builders for the M5 diff-view / review-thread controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce. Clean diff views
//! and review threads are built so the shared diff-context / hunk-identity and thread-state / anchor /
//! provider-locality grammar is proven across editor, diff, review, notebook, support, and product
//! surfaces without any hidden moved / elided context, rendered-versus-source blur, silent hunk / anchor
//! drift, blurred outdated-resolved state, or draft-as-published thread.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_DIFF_REVIEW_CONTROLS_PACKET_ID: &str =
    "m5-diff-view-review-thread-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-12T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn diff(input: M5DiffViewResolutionInput) -> M5ResolvedDiffView {
    resolve_diff_view(input).expect("seed diff-view input resolves")
}

fn thread(input: M5ReviewThreadResolutionInput) -> M5ResolvedReviewThread {
    resolve_review_thread(input).expect("seed review-thread input resolves")
}

// -- Clean diff-view examples (shared change/context/hunk grammar) ------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_diff_base(
    diff_id: &str,
    hunk_label: &str,
    change_kind: M5DiffChangeKind,
    visibility: M5DiffContextVisibility,
    rendering: M5DiffSourceRendering,
    hunk_identity: M5DiffHunkIdentity,
) -> M5DiffViewResolutionInput {
    M5DiffViewResolutionInput {
        diff_id: diff_id.to_owned(),
        hunk_label: hunk_label.to_owned(),
        change_kind,
        change_kind_stated: true,
        context_visibility: visibility,
        moved_disclosed: true,
        hidden_context_disclosed: true,
        source_rendering: rendering,
        rendering_disclosed: true,
        hunk_identity,
        hunk_reidentification_disclosed: true,
        export_summary_structured: true,
        detail_command_available: true,
        proof_fresh: true,
    }
}

/// Clean added hunk with full context, exact source, and a stable hunk id.
fn diff_added_full_clean() -> M5ResolvedDiffView {
    diff(clean_diff_base(
        "diff:editor:added-14",
        "fn parse_header(&self) -> Result<Header, Error>",
        M5DiffChangeKind::Added,
        M5DiffContextVisibility::FullContext,
        M5DiffSourceRendering::SourceExact,
        M5DiffHunkIdentity::StableHunkId,
    ))
}

/// Clean moved hunk whose provenance is disclosed as moved.
fn diff_moved_disclosed_clean() -> M5ResolvedDiffView {
    diff(clean_diff_base(
        "diff:editor:moved-7",
        "moved: helper `normalize_path` relocated to `paths.rs`",
        M5DiffChangeKind::Moved,
        M5DiffContextVisibility::MovedContext,
        M5DiffSourceRendering::SourceExact,
        M5DiffHunkIdentity::StableHunkId,
    ))
}

/// Clean modified hunk whose collapsed context is disclosed.
fn diff_collapsed_disclosed_clean() -> M5ResolvedDiffView {
    diff(clean_diff_base(
        "diff:diff:collapsed-3",
        "modified: 4 lines changed (12 unchanged lines collapsed)",
        M5DiffChangeKind::Modified,
        M5DiffContextVisibility::CollapsedContext,
        M5DiffSourceRendering::RenderedFaithful,
        M5DiffHunkIdentity::StableHunkId,
    ))
}

/// Clean modified hunk whose elided context is disclosed with an explicit gap marker.
fn diff_elided_disclosed_clean() -> M5ResolvedDiffView {
    diff(clean_diff_base(
        "diff:diff:elided-9",
        "modified: signature updated (surrounding body elided)",
        M5DiffChangeKind::Modified,
        M5DiffContextVisibility::ElidedContext,
        M5DiffSourceRendering::SourceExact,
        M5DiffHunkIdentity::StableHunkId,
    ))
}

/// Clean re-anchored hunk after the underlying text moved.
fn diff_reanchored_clean() -> M5ResolvedDiffView {
    diff(clean_diff_base(
        "diff:diff:reanchored-5",
        "modified: re-anchored after upstream insertion",
        M5DiffChangeKind::Modified,
        M5DiffContextVisibility::ReAnchoredContext,
        M5DiffSourceRendering::SourceExact,
        M5DiffHunkIdentity::StableHunkId,
    ))
}

/// Clean hunk with a rebased hunk id disclosed as re-identified.
fn diff_rebased_hunk_clean() -> M5ResolvedDiffView {
    diff(clean_diff_base(
        "diff:notebook:rebased-2",
        "added: cell edit (hunk id rebased onto latest run)",
        M5DiffChangeKind::Added,
        M5DiffContextVisibility::FullContext,
        M5DiffSourceRendering::SourceExact,
        M5DiffHunkIdentity::RebasedHunkId,
    ))
}

/// Clean hunk rendered approximately, disclosed as not the exact source bytes.
fn diff_rendered_approximate_disclosed_clean() -> M5ResolvedDiffView {
    diff(clean_diff_base(
        "diff:support:approx-8",
        "modified: rendered preview (approximate; open source for exact bytes)",
        M5DiffChangeKind::Modified,
        M5DiffContextVisibility::FullContext,
        M5DiffSourceRendering::RenderedApproximate,
        M5DiffHunkIdentity::StableHunkId,
    ))
}

// -- Degraded diff-view examples ---------------------------------------------------------------

/// Degraded diff: the hunk identity / label is unstated.
fn diff_identity_unstated() -> M5ResolvedDiffView {
    let mut input = clean_diff_base(
        "diff:support:no-label",
        "   ",
        M5DiffChangeKind::Modified,
        M5DiffContextVisibility::FullContext,
        M5DiffSourceRendering::SourceExact,
        M5DiffHunkIdentity::StableHunkId,
    );
    input.hunk_label = "   ".to_owned();
    diff(input)
}

/// Degraded diff: the change kind is collapsed into a generic change.
fn diff_change_kind_collapsed() -> M5ResolvedDiffView {
    let mut input = clean_diff_base(
        "diff:editor:collapsed-kind",
        "change shown only by a generic marker",
        M5DiffChangeKind::Modified,
        M5DiffContextVisibility::FullContext,
        M5DiffSourceRendering::SourceExact,
        M5DiffHunkIdentity::StableHunkId,
    );
    input.change_kind_stated = false;
    diff(input)
}

/// Degraded diff: the context visibility cannot be resolved.
fn diff_context_unresolved() -> M5ResolvedDiffView {
    diff(clean_diff_base(
        "diff:review:context-unknown",
        "hunk with no resolvable context visibility",
        M5DiffChangeKind::Modified,
        M5DiffContextVisibility::VisibilityUnresolved,
        M5DiffSourceRendering::SourceExact,
        M5DiffHunkIdentity::StableHunkId,
    ))
}

/// Degraded diff: a moved region is hidden rather than disclosed as moved.
fn diff_moved_hidden() -> M5ResolvedDiffView {
    let mut input = clean_diff_base(
        "diff:diff:moved-hidden",
        "region actually moved but shown as an added/removed pair",
        M5DiffChangeKind::Moved,
        M5DiffContextVisibility::MovedContext,
        M5DiffSourceRendering::SourceExact,
        M5DiffHunkIdentity::StableHunkId,
    );
    input.moved_disclosed = false;
    diff(input)
}

/// Degraded diff: collapsed / elided context is not disclosed.
fn diff_hidden_context_not_disclosed() -> M5ResolvedDiffView {
    let mut input = clean_diff_base(
        "diff:diff:hidden-context",
        "hunk pretending a full view while context is collapsed",
        M5DiffChangeKind::Modified,
        M5DiffContextVisibility::CollapsedContext,
        M5DiffSourceRendering::SourceExact,
        M5DiffHunkIdentity::StableHunkId,
    );
    input.hidden_context_disclosed = false;
    diff(input)
}

/// Degraded diff: the source-versus-rendered relationship cannot be resolved.
fn diff_source_rendering_unresolved() -> M5ResolvedDiffView {
    diff(clean_diff_base(
        "diff:product:rendering-unknown",
        "hunk with no resolvable source-versus-rendered relationship",
        M5DiffChangeKind::Modified,
        M5DiffContextVisibility::FullContext,
        M5DiffSourceRendering::RenderingUnresolved,
        M5DiffHunkIdentity::StableHunkId,
    ))
}

/// Degraded diff: a rendered / transformed diff is blurred with the exact source.
fn diff_source_vs_rendered_blurred() -> M5ResolvedDiffView {
    let mut input = clean_diff_base(
        "diff:support:rendered-as-source",
        "transformed preview presented as the exact source bytes",
        M5DiffChangeKind::Modified,
        M5DiffContextVisibility::FullContext,
        M5DiffSourceRendering::RenderedTransformed,
        M5DiffHunkIdentity::StableHunkId,
    );
    input.rendering_disclosed = false;
    diff(input)
}

/// Degraded diff: the hunk identity cannot be resolved.
fn diff_hunk_identity_unresolved() -> M5ResolvedDiffView {
    diff(clean_diff_base(
        "diff:product:hunk-unknown",
        "hunk with no resolvable identity",
        M5DiffChangeKind::Modified,
        M5DiffContextVisibility::FullContext,
        M5DiffSourceRendering::SourceExact,
        M5DiffHunkIdentity::HunkIdUnresolved,
    ))
}

/// Degraded diff: the hunk identity drifted without being disclosed as re-identified.
fn diff_hunk_drifted() -> M5ResolvedDiffView {
    let mut input = clean_diff_base(
        "diff:notebook:hunk-drift",
        "hunk whose identity silently drifted between runs",
        M5DiffChangeKind::Modified,
        M5DiffContextVisibility::FullContext,
        M5DiffSourceRendering::SourceExact,
        M5DiffHunkIdentity::UnstableHunkId,
    );
    input.hunk_reidentification_disclosed = false;
    diff(input)
}

/// Degraded diff: the structural summary is opaque rather than inspectable.
fn diff_structural_summary_opaque() -> M5ResolvedDiffView {
    let mut input = clean_diff_base(
        "diff:support:opaque-summary",
        "diff whose summary is an opaque blob with no structure",
        M5DiffChangeKind::Modified,
        M5DiffContextVisibility::FullContext,
        M5DiffSourceRendering::SourceExact,
        M5DiffHunkIdentity::StableHunkId,
    );
    input.export_summary_structured = false;
    diff(input)
}

/// Degraded diff: no command-backed detail path is reachable.
fn diff_detail_missing() -> M5ResolvedDiffView {
    let mut input = clean_diff_base(
        "diff:product:detail-missing",
        "hunk with no command-backed detail path",
        M5DiffChangeKind::Modified,
        M5DiffContextVisibility::FullContext,
        M5DiffSourceRendering::SourceExact,
        M5DiffHunkIdentity::StableHunkId,
    );
    input.detail_command_available = false;
    diff(input)
}

// -- Clean review-thread examples --------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_thread_base(
    thread_id: &str,
    comment: &str,
    state: M5ReviewThreadState,
    anchor: M5AnchorDurability,
    locality: M5ReviewProviderLocality,
) -> M5ReviewThreadResolutionInput {
    M5ReviewThreadResolutionInput {
        thread_id: thread_id.to_owned(),
        comment_label: comment.to_owned(),
        thread_state: state,
        thread_state_stated: true,
        outdated_resolved_distinguished: true,
        anchor_durability: anchor,
        anchor_drift_disclosed: true,
        provider_locality: locality,
        provider_distinction_explicit: true,
        pending_send_disclosed: true,
        detail_command_available: true,
        proof_fresh: true,
    }
}

/// Clean draft comment stored provider-locally.
fn thread_draft_local_clean() -> M5ResolvedReviewThread {
    thread(clean_thread_base(
        "thread:editor:draft-21",
        "draft: consider extracting this into a helper",
        M5ReviewThreadState::Draft,
        M5AnchorDurability::AnchoredExact,
        M5ReviewProviderLocality::ProviderLocal,
    ))
}

/// Clean published comment hosted by the review provider.
fn thread_published_hosted_clean() -> M5ResolvedReviewThread {
    thread(clean_thread_base(
        "thread:review:published-8",
        "published: please add a test for the empty-input case",
        M5ReviewThreadState::Published,
        M5AnchorDurability::AnchoredExact,
        M5ReviewProviderLocality::ProviderHosted,
    ))
}

/// Clean resolved thread, cleanly re-anchored, hosted.
fn thread_resolved_clean() -> M5ResolvedReviewThread {
    thread(clean_thread_base(
        "thread:review:resolved-12",
        "resolved: fixed in the latest revision",
        M5ReviewThreadState::Resolved,
        M5AnchorDurability::ReAnchored,
        M5ReviewProviderLocality::ProviderHosted,
    ))
}

/// Clean outdated thread whose drifted anchor is disclosed, stored provider-locally.
fn thread_outdated_clean() -> M5ResolvedReviewThread {
    thread(clean_thread_base(
        "thread:review:outdated-4",
        "outdated: the code below this comment has since moved",
        M5ReviewThreadState::Outdated,
        M5AnchorDurability::DriftedApproximate,
        M5ReviewProviderLocality::ProviderLocal,
    ))
}

/// Clean re-anchored thread on a locally mirrored copy.
fn thread_reanchored_clean() -> M5ResolvedReviewThread {
    thread(clean_thread_base(
        "thread:review:reanchored-6",
        "re-anchored: comment reattached after an upstream edit",
        M5ReviewThreadState::ReAnchored,
        M5AnchorDurability::ReAnchored,
        M5ReviewProviderLocality::MirroredLocal,
    ))
}

/// Clean locked thread hosted by the review provider.
fn thread_locked_clean() -> M5ResolvedReviewThread {
    thread(clean_thread_base(
        "thread:notebook:locked-3",
        "locked: this thread is locked pending a decision",
        M5ReviewThreadState::Locked,
        M5AnchorDurability::AnchoredExact,
        M5ReviewProviderLocality::ProviderHosted,
    ))
}

/// Clean pending-send comment mid-handoff between desktop and browser.
fn thread_pending_send_clean() -> M5ResolvedReviewThread {
    thread(clean_thread_base(
        "thread:support:pending-15",
        "pending send: queued but not yet published to the provider",
        M5ReviewThreadState::PendingSend,
        M5AnchorDurability::AnchoredExact,
        M5ReviewProviderLocality::HandoffPending,
    ))
}

// -- Degraded review-thread examples -----------------------------------------------------------

/// Degraded thread: the thread identity / comment label is unstated.
fn thread_identity_unstated() -> M5ResolvedReviewThread {
    let mut input = clean_thread_base(
        "thread:support:no-comment",
        "   ",
        M5ReviewThreadState::Published,
        M5AnchorDurability::AnchoredExact,
        M5ReviewProviderLocality::ProviderHosted,
    );
    input.comment_label = "   ".to_owned();
    thread(input)
}

/// Degraded thread: the thread state cannot be resolved.
fn thread_state_unresolved() -> M5ResolvedReviewThread {
    thread(clean_thread_base(
        "thread:product:state-unknown",
        "comment with no resolvable thread state",
        M5ReviewThreadState::StateUnknown,
        M5AnchorDurability::AnchoredExact,
        M5ReviewProviderLocality::ProviderHosted,
    ))
}

/// Degraded thread: the state is encoded by color / provider-specific jargon rather than named.
fn thread_state_color_only() -> M5ResolvedReviewThread {
    let mut input = clean_thread_base(
        "thread:editor:state-color-only",
        "state shown only by a colored dot / provider jargon",
        M5ReviewThreadState::Outdated,
        M5AnchorDurability::AnchoredExact,
        M5ReviewProviderLocality::ProviderHosted,
    );
    input.thread_state_stated = false;
    thread(input)
}

/// Degraded thread: outdated and resolved state are blurred together.
fn thread_outdated_resolved_blurred() -> M5ResolvedReviewThread {
    let mut input = clean_thread_base(
        "thread:review:outdated-resolved-blur",
        "outdated thread reading the same as a resolved one",
        M5ReviewThreadState::Outdated,
        M5AnchorDurability::AnchoredExact,
        M5ReviewProviderLocality::ProviderHosted,
    );
    input.outdated_resolved_distinguished = false;
    thread(input)
}

/// Degraded thread: the comment-anchor durability cannot be resolved.
fn thread_anchor_unresolved() -> M5ResolvedReviewThread {
    thread(clean_thread_base(
        "thread:product:anchor-unknown",
        "comment with no resolvable anchor durability",
        M5ReviewThreadState::Published,
        M5AnchorDurability::AnchorUnresolved,
        M5ReviewProviderLocality::ProviderHosted,
    ))
}

/// Degraded thread: the comment anchor drifted without being disclosed.
fn thread_anchor_drift_hidden() -> M5ResolvedReviewThread {
    let mut input = clean_thread_base(
        "thread:review:anchor-drift",
        "comment whose anchor silently drifted off its line",
        M5ReviewThreadState::Published,
        M5AnchorDurability::OutdatedAnchor,
        M5ReviewProviderLocality::ProviderHosted,
    );
    input.anchor_drift_disclosed = false;
    thread(input)
}

/// Degraded thread: the provider locality cannot be resolved.
fn thread_provider_locality_unresolved() -> M5ResolvedReviewThread {
    thread(clean_thread_base(
        "thread:product:locality-unknown",
        "comment with no resolvable provider locality",
        M5ReviewThreadState::Published,
        M5AnchorDurability::AnchoredExact,
        M5ReviewProviderLocality::LocalityUnresolved,
    ))
}

/// Degraded thread: the provider-local-versus-provider-hosted distinction is implicit.
fn thread_provider_distinction_implicit() -> M5ResolvedReviewThread {
    let mut input = clean_thread_base(
        "thread:support:locality-implicit",
        "comment leaving provider-local vs hosted implicit on export",
        M5ReviewThreadState::Published,
        M5AnchorDurability::AnchoredExact,
        M5ReviewProviderLocality::ProviderHosted,
    );
    input.provider_distinction_explicit = false;
    thread(input)
}

/// Degraded thread: a draft / pending-send thread reads as published.
fn thread_pending_send_hidden() -> M5ResolvedReviewThread {
    let mut input = clean_thread_base(
        "thread:notebook:pending-hidden",
        "unsent draft presented as if already published",
        M5ReviewThreadState::Draft,
        M5AnchorDurability::AnchoredExact,
        M5ReviewProviderLocality::ProviderLocal,
    );
    input.pending_send_disclosed = false;
    thread(input)
}

// -- Row builders ------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5DiffReviewConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5EditorInlineDowngradeTrigger>,
    diff_examples: Vec<M5ResolvedDiffView>,
    thread_examples: Vec<M5ResolvedReviewThread>,
) -> M5DiffReviewControlsRow {
    M5DiffReviewControlsRow {
        consumer_surface,
        qualification: M5EditorInlineQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5EditorInlineDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5EditorInlineRequiredLabel::Identity,
            M5EditorInlineRequiredLabel::State,
            M5EditorInlineRequiredLabel::KeyboardRoute,
            M5EditorInlineRequiredLabel::AnchorAndFreshness,
            M5EditorInlineRequiredLabel::ConfidenceAndSource,
        ],
        accessibility_routes: M5EditorInlineAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5DiffReviewAnatomyPart::ALL.to_vec(),
        export_fields: M5DiffReviewExportField::ALL.to_vec(),
        downgrade_triggers,
        diff_examples,
        thread_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_DIFF_REVIEW_CONTROLS_SCHEMA_REF,
            M5_DIFF_VIEW_SCHEMA_REF,
            M5_REVIEW_THREAD_SCHEMA_REF,
        ]),
        diff_moved_or_hidden_context_pretends_immutable_view: false,
        diff_hunk_identity_or_source_rendering_silently_drifts: false,
        review_outdated_and_resolved_state_blurred: false,
        review_anchor_or_provider_locality_silently_drifts: false,
    }
}

fn controls_rows() -> Vec<M5DiffReviewControlsRow> {
    use M5EditorInlineConsumerSurface as C;
    use M5EditorInlineDowngradeTrigger as D;

    vec![
        base_row(
            C::EditorUi,
            "Editor surface owner",
            "The editor names diff change kinds and moved-versus-hidden context with no collapsed generic change, and shows draft-versus-published review threads with one controlled vocabulary; both degrade honestly when a change kind is collapsed or a thread state is encoded by color alone",
            "evidence:m5-diff-review-editor-ui:001",
            vec![
                D::DiffChangeKindCollapsed,
                D::TabMarkerDiagnosticColorOnly,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                diff_added_full_clean(),
                diff_moved_disclosed_clean(),
                diff_change_kind_collapsed(),
            ],
            vec![
                thread_draft_local_clean(),
                thread_published_hosted_clean(),
                thread_state_color_only(),
            ],
        ),
        base_row(
            C::DiffUi,
            "Diff surface owner",
            "The diff surface stays honest when context is moved, elided, collapsed, or re-anchored rather than pretending one immutable view, and degrades honestly when a moved region is hidden or collapsed context is not disclosed",
            "evidence:m5-diff-review-diff-ui:001",
            vec![
                D::DiffChangeKindCollapsed,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                diff_collapsed_disclosed_clean(),
                diff_elided_disclosed_clean(),
                diff_reanchored_clean(),
                diff_moved_hidden(),
                diff_hidden_context_not_disclosed(),
            ],
            vec![thread_resolved_clean(), thread_reanchored_clean()],
        ),
        base_row(
            C::ReviewUi,
            "Review surface owner",
            "The review surface exposes the same thread-state grammar and anchor durability across desktop, browser handoff, and exported packets, keeps outdated and resolved distinct without color, and degrades honestly when the two are blurred or an anchor silently drifts",
            "evidence:m5-diff-review-review-ui:001",
            vec![
                D::OutdatedAndResolvedBlurred,
                D::CommentAnchorDriftedSilently,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![diff_added_full_clean(), diff_context_unresolved()],
            vec![
                thread_outdated_clean(),
                thread_resolved_clean(),
                thread_outdated_resolved_blurred(),
                thread_anchor_drift_hidden(),
            ],
        ),
        base_row(
            C::NotebookUi,
            "Notebook review owner",
            "The notebook reuses the same diff and review-thread grammar in code cells, discloses a rebased hunk id rather than reading as stable, and degrades honestly when a hunk identity silently drifts or a draft reads as published",
            "evidence:m5-diff-review-notebook-ui:001",
            vec![
                D::AnchorStateUnstated,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![diff_rebased_hunk_clean(), diff_hunk_drifted()],
            vec![thread_locked_clean(), thread_pending_send_hidden()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved diff and thread truth, so a rendered-versus-source blur, an opaque summary, an implicit provider locality, or an unstated identity is visible in evidence rather than hidden behind compact chrome",
            "evidence:m5-diff-review-support-export:001",
            vec![
                D::EvidenceTimelineOpaqueLog,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                diff_rendered_approximate_disclosed_clean(),
                diff_source_vs_rendered_blurred(),
                diff_structural_summary_opaque(),
                diff_identity_unstated(),
            ],
            vec![
                thread_pending_send_clean(),
                thread_identity_unstated(),
                thread_provider_distinction_implicit(),
            ],
        ),
        base_row(
            C::ProductUi,
            "In-product review owner",
            "In-product surfaces reuse the same diff and thread grammar a user sees in the editor, always offering the command-backed detail path and degrading honestly when the trace path is missing, the rendering or hunk identity is unresolved, or the provider locality is unresolved",
            "evidence:m5-diff-review-product-ui:001",
            vec![
                D::GenericChromeWordingUsed,
                D::AnchorStateUnstated,
                D::ProofStale,
            ],
            vec![
                diff_added_full_clean(),
                diff_detail_missing(),
                diff_source_rendering_unresolved(),
                diff_hunk_identity_unresolved(),
            ],
            vec![
                thread_draft_local_clean(),
                thread_state_unresolved(),
                thread_anchor_unresolved(),
                thread_provider_locality_unresolved(),
            ],
        ),
    ]
}

fn governance_review() -> M5DiffReviewGovernanceReview {
    M5DiffReviewGovernanceReview {
        diff_names_change_context_and_rendering: true,
        diff_keeps_stable_hunk_identity: true,
        moved_and_hidden_context_always_disclosed: true,
        diff_keeps_inspectable_structural_summary: true,
        thread_names_state_with_one_vocabulary: true,
        outdated_and_resolved_never_blurred: true,
        comment_anchors_never_silently_drift: true,
        provider_locality_stays_explicit: true,
        draft_or_pending_never_reads_as_published: true,
        thread_grammar_holds_across_surfaces: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5DiffReviewConsumerProjection {
    M5DiffReviewConsumerProjection {
        editor_surfaces_consume_diff_and_thread_vocabulary: true,
        diff_surfaces_consume_context_and_hunk_vocabulary: true,
        review_surfaces_consume_thread_state_and_anchor_vocabulary: true,
        browser_handoff_and_export_preserve_provider_locality: true,
        facts_trace_to_single_component_contract: true,
        support_export_reads_single_editor_source: true,
    }
}

fn proof_freshness() -> M5DiffReviewProofFreshness {
    M5DiffReviewProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5DiffReviewReleasePosture {
    M5DiffReviewReleasePosture {
        proof_packet_ref: M5_DIFF_REVIEW_CONTROLS_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_DIFF_REVIEW_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_DIFF_REVIEW_CONTROLS_SCHEMA_REF,
        M5_DIFF_REVIEW_CONTROLS_DOC_REF,
        M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF,
        M5_EDITOR_INLINE_COMPONENT_DOC_REF,
        M5_DIFF_VIEW_SCHEMA_REF,
        M5_REVIEW_THREAD_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 diff-view / review-thread controls packet.
pub fn seeded_m5_diff_review_controls() -> M5DiffReviewControlsPacket {
    M5DiffReviewControlsPacket::new(M5DiffReviewControlsPacketInput {
        packet_id: M5_DIFF_REVIEW_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 diff-view and review-thread controls with change-kind, moved-versus-hidden context, source-versus-rendered truth, stable hunk identity, one thread-state vocabulary, comment-anchor durability, and provider-local-versus-provider-hosted parity aligned across editor, diff, review, notebook, support, and product surfaces"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5DiffReviewVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the diff-UI row is held at Beta pending moved / hidden-context parity on every
/// deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_diff_review_controls_diff_ui_beta_narrowed() -> M5DiffReviewControlsPacket {
    let mut packet = seeded_m5_diff_review_controls();
    packet.packet_id = "m5-diff-view-review-thread-controls:diff-ui-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5EditorInlineConsumerSurface::DiffUi)
        .expect("diff-ui row present");
    row.qualification = M5EditorInlineQualificationClass::Beta;
    packet
}

/// Narrowed variant: the review-UI row is narrowed to Preview pending thread-state / anchor parity on
/// every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_diff_review_controls_review_ui_preview_narrowed() -> M5DiffReviewControlsPacket {
    let mut packet = seeded_m5_diff_review_controls();
    packet.packet_id = "m5-diff-view-review-thread-controls:review-ui-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5EditorInlineConsumerSurface::ReviewUi)
        .expect("review-ui row present");
    row.qualification = M5EditorInlineQualificationClass::Preview;
    packet
}
