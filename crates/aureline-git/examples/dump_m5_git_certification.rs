//! Conformance dump for the M5 Git certification register.
//!
//! Prints the canonical export-safe packet as deterministic JSON. The optional
//! first argument selects a degraded fixture variant that demonstrates the
//! fail-closed downgrade automation instead of the canonical packet:
//!
//! * (no argument) — the canonical register; every claimed row certified
//! * `stale-topology` — a stale topology dimension narrows a row to retest-pending
//! * `failed-parity` — a failed provider-parity dimension narrows a row to unsupported
//! * `partial-history` — an honestly partial history-recovery dimension narrows a row to limited
//!
//! These four documents are the source of the checked-in support export and the
//! protected certification-corpus fixtures.

use aureline_git::{
    CertificationConsumerSurface, CertificationDimension, CertificationFreshnessPosture,
    CertificationGovernanceReview, CertificationParityAudit, CertificationVerdict,
    DimensionProofState, DimensionQualification, DowngradeAutomation, EvidenceFreshness,
    M5GitCertificationPacket, M5GitCertificationPacketInput, M5GitCertificationRow, M5GitClaimRow,
    M5_GIT_CERTIFICATION_DOC_REF, M5_GIT_CERTIFICATION_HISTORY_SURGERY_CONTRACT_REF,
    M5_GIT_CERTIFICATION_MATRIX_CONTRACT_REF, M5_GIT_CERTIFICATION_SCHEMA_REF,
    M5_GIT_CERTIFICATION_STASH_RECOVERY_CONTRACT_REF,
    M5_GIT_CERTIFICATION_TOPOLOGY_ACTION_CONTRACT_REF, M5_GIT_CERTIFICATION_TOPOLOGY_CONTRACT_REF,
};

const MINTED_AT: &str = "2026-06-17T00:00:00Z";

/// Evidence backing the topology-honesty dimension.
fn topology_evidence() -> Vec<String> {
    vec![
        "artifacts/git/m5/git_topology/git_topology_review.json".to_owned(),
        "artifacts/git/m5/git_topology/topology_first_consumers.json".to_owned(),
        "fixtures/git/m5/topology-corpus/submodule_uninitialized_narrowed.json".to_owned(),
    ]
}

/// Evidence backing the worktree/root-scoping dimension.
fn scoping_evidence() -> Vec<String> {
    vec![
        "artifacts/git/m5/git_topology/topology_action_review.json".to_owned(),
        "fixtures/git/m5/widen-deepen-initialize-hydrate/wrong_root.json".to_owned(),
        "fixtures/git/m5/widen-deepen-initialize-hydrate/multi_root.json".to_owned(),
    ]
}

/// Evidence backing the history-surgery preview/recovery dimension.
fn history_evidence() -> Vec<String> {
    vec![
        "artifacts/git/m5/history_surgery_review/history_surgery_review_sheets.json".to_owned(),
        "artifacts/git/m5/stash_recovery/stash_recovery.json".to_owned(),
        "fixtures/git/m5/rebase-cherry-pick-reset/force_push_protected_blocked.json".to_owned(),
    ]
}

/// Evidence backing the local/provider-parity dimension.
fn parity_evidence() -> Vec<String> {
    vec![
        "artifacts/git/m5/git_topology/topology_propagation/review_topology_overlay.json"
            .to_owned(),
        "fixtures/git/m5/stash-recovery/stash_apply_provider_outage_local_only.json".to_owned(),
        "fixtures/git/m5/rebase-cherry-pick-reset/reset_provider_outage_local_only.json".to_owned(),
    ]
}

/// Builds a current-and-proven qualification for an applicable dimension.
fn proven(
    dimension: CertificationDimension,
    evidence_refs: Vec<String>,
    summary: &str,
) -> DimensionQualification {
    DimensionQualification {
        dimension,
        applicable: true,
        freshness: EvidenceFreshness::Current,
        proof_state: DimensionProofState::Proven,
        evidence_refs,
        summary: summary.to_owned(),
    }
}

/// Builds a not-applicable qualification (e.g. history rewrite on a read row).
fn not_applicable(dimension: CertificationDimension, summary: &str) -> DimensionQualification {
    DimensionQualification {
        dimension,
        applicable: false,
        freshness: EvidenceFreshness::Missing,
        proof_state: DimensionProofState::NotRun,
        evidence_refs: Vec::new(),
        summary: summary.to_owned(),
    }
}

/// The four dimensions for a row that performs a history rewrite.
fn rewriting_dimensions(history_summary: &str) -> Vec<DimensionQualification> {
    vec![
        proven(
            CertificationDimension::TopologyHonesty,
            topology_evidence(),
            "Topology class, degraded vocabulary, and honesty label are reported before the operation; coverage is never silently completed",
        ),
        proven(
            CertificationDimension::WorktreeRootScoping,
            scoping_evidence(),
            "The operation targets an explicit worktree or root; the wrong-root guard blocks ambient bulk mutation",
        ),
        proven(
            CertificationDimension::HistorySurgeryPreviewRecovery,
            history_evidence(),
            history_summary,
        ),
        proven(
            CertificationDimension::LocalProviderParity,
            parity_evidence(),
            "Local Git truth stays authoritative when the provider overlay is degraded or absent; recovery stays reachable offline",
        ),
    ]
}

/// The four dimensions for a read/scope row that performs no history rewrite.
fn reading_dimensions(
    topology_summary: &str,
    scoping_summary: &str,
    parity_summary: &str,
) -> Vec<DimensionQualification> {
    vec![
        proven(
            CertificationDimension::TopologyHonesty,
            topology_evidence(),
            topology_summary,
        ),
        proven(
            CertificationDimension::WorktreeRootScoping,
            scoping_evidence(),
            scoping_summary,
        ),
        not_applicable(
            CertificationDimension::HistorySurgeryPreviewRecovery,
            "Not applicable: this row reports or scopes repository truth and performs no history rewrite",
        ),
        proven(
            CertificationDimension::LocalProviderParity,
            parity_evidence(),
            parity_summary,
        ),
    ]
}

fn all_surfaces() -> Vec<CertificationConsumerSurface> {
    CertificationConsumerSurface::ALL.to_vec()
}

fn certified_row(
    claim_row: M5GitClaimRow,
    row_label: &str,
    published_claim: &str,
    dimensions: Vec<DimensionQualification>,
) -> M5GitCertificationRow {
    let row = M5GitCertificationRow {
        claim_row,
        row_label: row_label.to_owned(),
        published_claim: published_claim.to_owned(),
        dimensions,
        verdict: CertificationVerdict::Certified,
        narrowing_reason: None,
        consumer_surfaces: all_surfaces(),
    };
    // Keep the declared verdict honest against the evidence at construction time.
    M5GitCertificationRow {
        verdict: row.derive_verdict(),
        ..row
    }
}

fn rows() -> Vec<M5GitCertificationRow> {
    vec![
        certified_row(
            M5GitClaimRow::SourceAcquisitionAndTopologyInitialization,
            "Source acquisition and topology initialization",
            "Clone, open, initialize, and hydrate repositories with an honest topology and previewed widen/deepen",
            reading_dimensions(
                "Initialize/hydrate previews the resulting topology and never claims content it has not fetched",
                "Acquisition targets the selected root; nested and submodule roots are initialized explicitly",
                "A degraded or offline provider does not block local open; local truth leads the overlay",
            ),
        ),
        certified_row(
            M5GitClaimRow::RepositoryTopologyHonesty,
            "Repository topology honesty",
            "Report sparse, partial-clone, shallow, submodule, nested, worktree, and LFS topology truthfully",
            reading_dimensions(
                "Every topology class carries its degraded vocabulary and honesty label; omitted is distinct from missing",
                "Topology truth is reported per root so cross-root reads are not conflated",
                "A stale provider overlay never overwrites local topology truth",
            ),
        ),
        certified_row(
            M5GitClaimRow::WorktreeAndRootScoping,
            "Worktree and root scoping",
            "Scope status, search, blame, and mutation to the correct worktree or root",
            reading_dimensions(
                "Scope widening is previewed before it applies and the active slice is disclosed",
                "Operations target an explicit worktree/root; the wrong-root guard blocks ambient bulk mutation",
                "Scope truth is identical with or without a provider overlay",
            ),
        ),
        certified_row(
            M5GitClaimRow::TopologyAwareSearchAiReviewParity,
            "Topology-aware search, AI context, and review parity",
            "Search, AI context, and review overlays express the same topology and scope vocabulary",
            reading_dimensions(
                "Zero-result and cross-root rows carry the topology honesty label rather than implying completeness",
                "Search and review honor the active root boundary and flag wrong-target reads",
                "AI context and review overlays read local topology truth, not a provider snapshot",
            ),
        ),
        certified_row(
            M5GitClaimRow::HistorySurgeryPreviewAndRecovery,
            "History-surgery preview and recovery",
            "Rebase, cherry-pick, reset, and revert with full preview and a reachable recovery checkpoint",
            rewriting_dimensions(
                "The full rewrite plan is previewed and a recovery checkpoint precedes the run; reflog-only fallback is disclosed when no checkpoint is possible",
            ),
        ),
        certified_row(
            M5GitClaimRow::StashReflogCheckpointRecovery,
            "Stash, reflog, and checkpoint recovery",
            "Apply, pop, drop, and branch-from-stash with distinct verbs and restorable recovery anchors",
            rewriting_dimensions(
                "Each stash verb stays distinct, the diff is previewed, and a stash-shelf or reflog/checkpoint restore stays reachable with disclosed caveats",
            ),
        ),
        certified_row(
            M5GitClaimRow::ConflictResolutionContinuity,
            "Conflict-resolution continuity",
            "Resume conflict resolution across reopen and restart with provenance preserved",
            rewriting_dimensions(
                "Conflict provenance and the resolution session survive reopen; a checkpoint precedes any continue and recovery stays reachable",
            ),
        ),
        certified_row(
            M5GitClaimRow::PublishAndProviderParity,
            "Publish and provider parity",
            "Publish and force-with-lease with a previewed ref update, rollback, and provider-degraded local continuity",
            rewriting_dimensions(
                "The before/after ref position is previewed and a rollback to the prior position stays available; force-with-lease on a protected ref is blocked",
            ),
        ),
    ]
}

fn parity_audit() -> CertificationParityAudit {
    CertificationParityAudit {
        product_reflects_row_verdicts: true,
        docs_help_reflects_row_verdicts: true,
        cli_reflects_row_verdicts: true,
        support_export_reflects_row_verdicts: true,
        evaluation_packs_reflect_row_verdicts: true,
        claim_publication_manifests_reflect_row_verdicts: true,
        release_public_truth_reflects_row_verdicts: true,
        no_surface_claims_wider_than_row: true,
        local_truth_authoritative_over_provider: true,
    }
}

fn downgrade_automation() -> DowngradeAutomation {
    DowngradeAutomation {
        auto_narrow_on_stale: true,
        stale_or_unrun_narrows_to: CertificationVerdict::RetestPending,
        partial_narrows_to: CertificationVerdict::Limited,
        failure_or_missing_narrows_to: CertificationVerdict::Unsupported,
        propagates_to_docs_help: true,
        propagates_to_support_packets: true,
        propagates_to_evaluation_packs: true,
        propagates_to_claim_publication_manifests: true,
        release_surface_stops_overclaiming_on_slip: true,
    }
}

fn governance_review() -> CertificationGovernanceReview {
    CertificationGovernanceReview {
        requires_current_topology_and_recovery_evidence_per_row: true,
        stale_or_underqualified_rows_narrow_automatically: true,
        release_surfaces_stop_overclaiming_on_slip: true,
        claim_truth_is_not_manual: true,
        fails_closed_to_retest_limited_or_unsupported: true,
        provider_degraded_local_continuity_required: true,
        one_certification_register_across_surfaces: true,
        no_claim_broadened_beyond_proof_packet: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        M5_GIT_CERTIFICATION_SCHEMA_REF.to_owned(),
        M5_GIT_CERTIFICATION_DOC_REF.to_owned(),
        M5_GIT_CERTIFICATION_MATRIX_CONTRACT_REF.to_owned(),
        M5_GIT_CERTIFICATION_TOPOLOGY_CONTRACT_REF.to_owned(),
        M5_GIT_CERTIFICATION_TOPOLOGY_ACTION_CONTRACT_REF.to_owned(),
        M5_GIT_CERTIFICATION_HISTORY_SURGERY_CONTRACT_REF.to_owned(),
        M5_GIT_CERTIFICATION_STASH_RECOVERY_CONTRACT_REF.to_owned(),
    ]
}

fn canonical_packet() -> M5GitCertificationPacket {
    M5GitCertificationPacket::new(M5GitCertificationPacketInput {
        packet_id: "m5-git-certification:0001".to_owned(),
        certification_label: "M5 Git Topology, History-Recovery, and Provider-Parity Certification"
            .to_owned(),
        rows: rows(),
        parity_audit: parity_audit(),
        downgrade_automation: downgrade_automation(),
        governance_review: governance_review(),
        freshness_posture: CertificationFreshnessPosture {
            review_slo_hours: 720,
            last_reviewed_at: MINTED_AT.to_owned(),
            auto_narrow_on_stale: true,
            evidence_window_open: true,
        },
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

/// Re-derives a row's verdict and narrowing reason after a dimension changed.
fn narrow_row(packet: &mut M5GitCertificationPacket, claim_row: M5GitClaimRow, reason: &str) {
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.claim_row == claim_row)
        .expect("row present");
    row.verdict = row.derive_verdict();
    row.narrowing_reason = Some(reason.to_owned());
}

fn stale_topology_packet() -> M5GitCertificationPacket {
    let mut packet = canonical_packet();
    packet.packet_id = "m5-git-certification:stale-topology-retest-pending:0001".to_owned();
    {
        let row = packet
            .rows
            .iter_mut()
            .find(|row| row.claim_row == M5GitClaimRow::RepositoryTopologyHonesty)
            .expect("topology row present");
        let dimension = row
            .dimensions
            .iter_mut()
            .find(|dimension| dimension.dimension == CertificationDimension::TopologyHonesty)
            .expect("topology dimension present");
        dimension.freshness = EvidenceFreshness::Stale;
        dimension.summary =
            "Topology proof exists but is past the freshness window and must be re-run before the claim stands"
                .to_owned();
    }
    narrow_row(
        &mut packet,
        M5GitClaimRow::RepositoryTopologyHonesty,
        "Topology-honesty evidence is stale; the row is held at retest-pending until re-run",
    );
    packet
}

fn failed_parity_packet() -> M5GitCertificationPacket {
    let mut packet = canonical_packet();
    packet.packet_id = "m5-git-certification:failed-provider-parity-unsupported:0001".to_owned();
    {
        let row = packet
            .rows
            .iter_mut()
            .find(|row| row.claim_row == M5GitClaimRow::PublishAndProviderParity)
            .expect("publish row present");
        let dimension = row
            .dimensions
            .iter_mut()
            .find(|dimension| dimension.dimension == CertificationDimension::LocalProviderParity)
            .expect("parity dimension present");
        dimension.proof_state = DimensionProofState::Failed;
        dimension.summary =
            "Provider-degraded local continuity could not be reproduced; publish recovery was not reachable offline"
                .to_owned();
    }
    narrow_row(
        &mut packet,
        M5GitClaimRow::PublishAndProviderParity,
        "Local/provider parity failed; the publish claim is unsupported until parity is restored",
    );
    packet
}

fn partial_history_packet() -> M5GitCertificationPacket {
    let mut packet = canonical_packet();
    packet.packet_id = "m5-git-certification:partial-history-recovery-limited:0001".to_owned();
    {
        let row = packet
            .rows
            .iter_mut()
            .find(|row| row.claim_row == M5GitClaimRow::HistorySurgeryPreviewAndRecovery)
            .expect("history row present");
        let dimension = row
            .dimensions
            .iter_mut()
            .find(|dimension| {
                dimension.dimension == CertificationDimension::HistorySurgeryPreviewRecovery
            })
            .expect("history dimension present");
        dimension.proof_state = DimensionProofState::Narrowed;
        dimension.summary =
            "Preview and checkpoint recovery are proven for rebase and reset; revert recovery is only reflog-only and is disclosed as such"
                .to_owned();
    }
    narrow_row(
        &mut packet,
        M5GitClaimRow::HistorySurgeryPreviewAndRecovery,
        "History-recovery proof is partial; the claim is limited to the previewed-and-checkpointed operations",
    );
    packet
}

fn main() {
    let variant = std::env::args().nth(1).unwrap_or_default();
    let packet = match variant.as_str() {
        "stale-topology" => stale_topology_packet(),
        "failed-parity" => failed_parity_packet(),
        "partial-history" => partial_history_packet(),
        _ => canonical_packet(),
    };
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "certification invalid: {violations:?}"
    );
    println!("{}", packet.export_safe_json());
}
