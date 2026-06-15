use super::*;
use crate::m5_author_and_publish_preview::current_m5_author_publish_matrix;

fn board() -> M5AntiAbuseReproBoard {
    current_m5_anti_abuse_repro_board().expect("embedded anti-abuse-and-repro packet must parse")
}

fn matrix() -> M5AuthorPublishMatrix {
    current_m5_author_publish_matrix().expect("embedded publish-preview packet must parse")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let board = board();
    assert_eq!(board.schema_version, M5_ANTI_ABUSE_REPRO_SCHEMA_VERSION);
    assert_eq!(board.record_kind, M5_ANTI_ABUSE_REPRO_RECORD_KIND);
    let violations = board.validate();
    assert!(
        violations.is_empty(),
        "embedded packet must be valid, found: {violations:?}"
    );
}

#[test]
fn summary_counts_match_rows() {
    let board = board();
    assert_eq!(board.summary, board.computed_summary());
}

#[test]
fn every_claimed_family_has_exactly_one_row() {
    let board = board();
    assert_eq!(board.artifact_families, ArtifactFamily::ALL.to_vec());
    assert_eq!(board.rows.len(), ArtifactFamily::ALL.len());
    for family in ArtifactFamily::ALL {
        assert!(
            board.row(family).is_some(),
            "claimed family {} must carry a row",
            family.as_str()
        );
    }
}

#[test]
fn every_row_is_internally_consistent() {
    let board = board();
    for row in &board.rows {
        assert!(
            row.row_consistent(),
            "row {} must be internally consistent",
            row.row_id
        );
        assert!(
            row.has_required_evidence(),
            "row {} must carry its cross-surface refs",
            row.row_id
        );
    }
    assert!(board.all_rows_consistent());
}

#[test]
fn local_dev_and_sideload_and_unsigned_render_local_only() {
    let board = board();
    for row in &board.rows {
        let locally_capped = row.signature_state.is_local_or_untrusted()
            || row.origin.caps_to_local_only()
            || matches!(
                row.bind_decision,
                BindPublishedIdentity::StayLocal | BindPublishedIdentity::BindReviewRequired
            );
        if locally_capped {
            assert_eq!(
                row.rendered_trust_posture,
                TrustPosture::UnsignedLocalOnly,
                "row {} is locally capped and must render local-only",
                row.row_id
            );
        }
    }
}

#[test]
fn signed_package_in_local_dev_workspace_does_not_inherit_trust() {
    let board = board();
    let recipe = board
        .row(ArtifactFamily::SignedRecipePack)
        .expect("signed recipe pack row");
    assert_eq!(recipe.signature_state, SignatureState::SignedVerified);
    assert_eq!(recipe.origin, WorkspaceOrigin::LocalDevWorkspace);
    assert_eq!(
        recipe.rendered_trust_posture,
        TrustPosture::UnsignedLocalOnly,
        "a signed package in a local-dev workspace must not inherit a trusted badge"
    );
}

#[test]
fn vanity_metrics_never_dominate_ranking() {
    let board = board();
    for row in &board.rows {
        assert_ne!(
            row.ranking_explainability,
            RankingExplainability::VanityDominated,
            "row {} must not be vanity-dominated",
            row.row_id
        );
        let vanity = row
            .ranking_reasons
            .iter()
            .filter(|c| c.is_vanity_metric())
            .count();
        if vanity > 0 {
            let substantive = row
                .ranking_reasons
                .iter()
                .filter(|c| c.is_substantive())
                .count();
            assert!(
                substantive >= vanity
                    || row.ranking_explainability == RankingExplainability::AntiAbuseLed,
                "row {} shows vanity metrics but they must not dominate",
                row.row_id
            );
        }
    }
}

#[test]
fn anti_abuse_demotion_leads_ranking() {
    let board = board();
    for row in &board.rows {
        let has_demotion = row
            .ranking_reasons
            .iter()
            .any(|c| c.is_anti_abuse_demotion());
        if has_demotion {
            assert_eq!(
                row.ranking_explainability,
                RankingExplainability::AntiAbuseLed,
                "row {} carries an anti-abuse demotion and must be anti-abuse-led",
                row.row_id
            );
        }
    }
}

#[test]
fn quarantine_is_reflected_in_ranking_chips_not_hidden() {
    let board = board();
    for row in &board.rows {
        let has_quarantine_chip = row
            .ranking_reasons
            .contains(&RankingReasonChip::AntiAbuseQuarantined);
        assert_eq!(
            has_quarantine_chip,
            row.quarantine_history_state.is_currently_withheld(),
            "row {} anti-abuse quarantine must be reflected in the ranking chips",
            row.row_id
        );
    }
    // The currently-withheld family is exposed on the visible board, not buried.
    let mirror = board
        .row(ArtifactFamily::MirroredRegistryVariant)
        .expect("mirrored variant row");
    assert_eq!(
        mirror.quarantine_history_state,
        QuarantineHistoryState::CurrentlyWithheld
    );
    assert_eq!(
        mirror.transparency_disposition,
        TransparencyDisposition::WithheldQuarantined
    );
    assert!(mirror
        .ranking_reasons
        .contains(&RankingReasonChip::AntiAbuseQuarantined));
}

#[test]
fn quarantine_and_publisher_loss_stay_disclosed() {
    let board = board();
    for row in &board.rows {
        let adverse = row.quarantine_history_state == QuarantineHistoryState::PriorActionDisclosed
            || row.publisher_continuity_state.is_disclosable();
        if adverse && !row.is_withheld() {
            assert_eq!(
                row.transparency_disposition,
                TransparencyDisposition::VisibleWithHistoryDisclosure,
                "row {} adverse history must be disclosed on the visible surface",
                row.row_id
            );
        }
    }
    // A verified-publisher loss is surfaced, not hidden in a moderation-only tool.
    assert_eq!(
        board.summary.verified_publisher_lost_rows, 1,
        "the verified-publisher loss must be visible in the board summary"
    );
    assert_eq!(board.summary.publisher_transferred_rows, 1);
    assert_eq!(board.summary.prior_action_rows, 1);
}

#[test]
fn repro_export_carries_required_fields_and_is_self_contained() {
    let board = board();
    for row in &board.rows {
        let repro = &row.repro_export;
        assert!(
            repro.has_core_identity(),
            "row {} repro export must carry package id and digest",
            row.row_id
        );
        assert!(
            repro.self_contained,
            "row {} repro export must be self-contained (no supervisor traces or paid services)",
            row.row_id
        );
        assert_eq!(
            repro.host_abi, row.host_abi,
            "row {} repro export must reproduce against the row's host ABI",
            row.row_id
        );
        assert_eq!(repro.state, repro.computed_state());
        if repro.state == ReproExportState::Complete {
            assert!(!repro.logs_ref.trim().is_empty());
            assert!(!repro.conformance_results_ref.trim().is_empty());
            assert!(!repro.manifest_ref.trim().is_empty());
        }
    }
    // Exactly one row discloses an incomplete export rather than faking completeness.
    assert_eq!(board.summary.incomplete_repro_export_rows, 1);
}

#[test]
fn local_to_published_rebind_requires_explicit_review() {
    let board = board();
    for row in &board.rows {
        if row.bind_decision.requires_review_ref() {
            assert!(
                !row.bind_review_ref.trim().is_empty(),
                "row {} pending/completing a bind must carry a review ref",
                row.row_id
            );
        }
        // A bound published identity may never appear on a still-local origin.
        if row.bind_decision == BindPublishedIdentity::BoundPublishedIdentity {
            assert!(
                !row.origin.is_local_authored(),
                "row {} bound published identity must not sit on a local origin",
                row.row_id
            );
        }
    }
    // Both halves of the flow are exercised: a pending review and a completed bind.
    assert!(board.summary.bind_review_required_rows >= 1);
    assert_eq!(board.summary.bound_published_identity_rows, 1);
}

#[test]
fn freshly_bound_identity_does_not_leap_to_verified_publisher() {
    let board = board();
    let docs = board.row(ArtifactFamily::DocsPack).expect("docs pack row");
    assert_eq!(
        docs.bind_decision,
        BindPublishedIdentity::BoundPublishedIdentity
    );
    assert_eq!(docs.declared_trust_posture, TrustPosture::VerifiedPublisher);
    assert_eq!(
        docs.rendered_trust_posture,
        TrustPosture::RegistryBound,
        "a freshly bound identity caps at registry_bound, not verified_publisher"
    );
}

#[test]
fn every_closed_vocabulary_is_exercised() {
    let board = board();

    let families: BTreeSet<ArtifactFamily> = board.rows.iter().map(|r| r.artifact_family).collect();
    for v in ArtifactFamily::ALL {
        assert!(families.contains(&v), "family {} unexercised", v.as_str());
    }
    let runtimes: BTreeSet<RuntimeClass> = board.rows.iter().map(|r| r.runtime_class).collect();
    for v in RuntimeClass::ALL {
        assert!(runtimes.contains(&v), "runtime {} unexercised", v.as_str());
    }
    let hosts: BTreeSet<HostAbiClass> = board.rows.iter().map(|r| r.host_abi).collect();
    for v in HostAbiClass::ALL {
        assert!(hosts.contains(&v), "host {} unexercised", v.as_str());
    }
    let origins: BTreeSet<WorkspaceOrigin> = board.rows.iter().map(|r| r.origin).collect();
    for v in WorkspaceOrigin::ALL {
        assert!(origins.contains(&v), "origin {} unexercised", v.as_str());
    }
    let signatures: BTreeSet<SignatureState> =
        board.rows.iter().map(|r| r.signature_state).collect();
    for v in SignatureState::ALL {
        assert!(
            signatures.contains(&v),
            "signature {} unexercised",
            v.as_str()
        );
    }
    let rendered: BTreeSet<TrustPosture> = board
        .rows
        .iter()
        .map(|r| r.rendered_trust_posture)
        .collect();
    for v in TrustPosture::ALL {
        assert!(
            rendered.contains(&v),
            "trust posture {} unexercised",
            v.as_str()
        );
    }
    let chips: BTreeSet<RankingReasonChip> = board
        .rows
        .iter()
        .flat_map(|r| r.ranking_reasons.iter().copied())
        .collect();
    for v in RankingReasonChip::ALL {
        assert!(
            chips.contains(&v),
            "ranking chip {} unexercised",
            v.as_str()
        );
    }
    let categories: BTreeSet<RankingReasonCategory> = chips.iter().map(|c| c.category()).collect();
    for v in RankingReasonCategory::ALL {
        assert!(
            categories.contains(&v),
            "category {} unexercised",
            v.as_str()
        );
    }
    let events: BTreeSet<HistoryEventKind> = board
        .rows
        .iter()
        .flat_map(|r| r.history_events.iter().map(|e| e.kind))
        .collect();
    for v in HistoryEventKind::ALL {
        assert!(events.contains(&v), "event kind {} unexercised", v.as_str());
    }
    let quarantines: BTreeSet<QuarantineHistoryState> = board
        .rows
        .iter()
        .map(|r| r.quarantine_history_state)
        .collect();
    for v in QuarantineHistoryState::ALL {
        assert!(
            quarantines.contains(&v),
            "quarantine state {} unexercised",
            v.as_str()
        );
    }
    let continuities: BTreeSet<PublisherContinuityState> = board
        .rows
        .iter()
        .map(|r| r.publisher_continuity_state)
        .collect();
    for v in PublisherContinuityState::ALL {
        assert!(
            continuities.contains(&v),
            "continuity state {} unexercised",
            v.as_str()
        );
    }
    let repro: BTreeSet<ReproExportState> =
        board.rows.iter().map(|r| r.repro_export.state).collect();
    for v in ReproExportState::ALL {
        assert!(repro.contains(&v), "repro state {} unexercised", v.as_str());
    }
    let binds: BTreeSet<BindPublishedIdentity> =
        board.rows.iter().map(|r| r.bind_decision).collect();
    for v in BindPublishedIdentity::ALL {
        assert!(
            binds.contains(&v),
            "bind decision {} unexercised",
            v.as_str()
        );
    }
    let dispositions: BTreeSet<TransparencyDisposition> = board
        .rows
        .iter()
        .map(|r| r.transparency_disposition)
        .collect();
    for v in TransparencyDisposition::ALL {
        assert!(
            dispositions.contains(&v),
            "disposition {} unexercised",
            v.as_str()
        );
    }
    // Ranking-explainability is exercised by trust-led and anti-abuse-led rows; the
    // flagged vanity-dominated state never appears in a valid board.
    let explainability: BTreeSet<RankingExplainability> = board
        .rows
        .iter()
        .map(|r| r.ranking_explainability)
        .collect();
    assert!(explainability.contains(&RankingExplainability::TrustLed));
    assert!(explainability.contains(&RankingExplainability::AntiAbuseLed));
    assert!(!explainability.contains(&RankingExplainability::VanityDominated));
}

#[test]
fn cross_check_matrix_agrees_with_publish_gate() {
    let board = board();
    let matrix = matrix();
    let violations = board.cross_check_matrix(&matrix);
    assert!(
        violations.is_empty(),
        "no row may render a stronger badge than the publish gate, found: {violations:?}"
    );
    for row in &board.rows {
        let gate = matrix.family(row.artifact_family).expect("matrix row");
        assert!(
            row.rendered_trust_posture.rank() <= gate.published_trust_posture.rank(),
            "row {} renders above the publish gate",
            row.row_id
        );
    }
}

#[test]
fn export_projection_reflects_the_board() {
    let board = board();
    let projection = board.export_projection();
    assert_eq!(projection.packet_id, board.packet_id);
    assert_eq!(projection.as_of, board.as_of);
    assert_eq!(projection.rows.len(), board.rows.len());
    assert!(projection.all_rows_consistent);
    assert_eq!(projection.visible_count, board.visible_rows().count());
    assert_eq!(projection.withheld_count, board.withheld_rows().count());
    assert_eq!(projection.local_only_count, board.local_only_rows().count());
    for (export, row) in projection.rows.iter().zip(&board.rows) {
        assert_eq!(export.row_id, row.row_id);
        assert_eq!(
            export.rendered_trust_posture,
            row.rendered_trust_posture.as_str()
        );
        assert_eq!(export.local_only, row.is_local_only());
        assert_eq!(export.withheld, row.is_withheld());
    }
}

#[test]
fn overstated_rendered_trust_is_flagged() {
    let mut board = board();
    let row = board
        .rows
        .iter_mut()
        .find(|r| r.artifact_family == ArtifactFamily::SignedRecipePack)
        .unwrap();
    row.rendered_trust_posture = TrustPosture::EnterpriseApproved;
    let violations = board.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5AntiAbuseReproViolation::OverstatedTrustPosture { .. })));
    assert!(violations.iter().any(|v| matches!(
        v,
        M5AntiAbuseReproViolation::LocalArtifactInheritedTrust { .. }
    )));
}

#[test]
fn vanity_dominated_ranking_is_flagged() {
    let mut board = board();
    let row = board
        .rows
        .iter_mut()
        .find(|r| r.artifact_family == ArtifactFamily::FirstPartyFrameworkPack)
        .unwrap();
    row.ranking_reasons = vec![
        RankingReasonChip::InstallCountPopularity,
        RankingReasonChip::StarRatingPopularity,
        RankingReasonChip::TrendingVelocity,
    ];
    row.ranking_explainability = RankingExplainability::VanityDominated;
    let violations = board.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5AntiAbuseReproViolation::VanityMetricsDominateRanking { .. }
    )));
}

#[test]
fn hidden_quarantine_is_flagged() {
    let mut board = board();
    let row = board
        .rows
        .iter_mut()
        .find(|r| r.artifact_family == ArtifactFamily::MirroredRegistryVariant)
        .unwrap();
    // Drop the anti-abuse quarantine chip while the package is still quarantined.
    row.ranking_reasons = vec![RankingReasonChip::CompatibilityCurrent];
    let violations = board.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5AntiAbuseReproViolation::QuarantineNotReflectedInRanking { .. }
    )));
}

#[test]
fn repro_export_not_self_contained_is_flagged() {
    let mut board = board();
    board.rows[0].repro_export.self_contained = false;
    let violations = board.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5AntiAbuseReproViolation::ReproExportNotSelfContained { .. }
    )));
}

#[test]
fn tampered_repro_export_state_is_flagged() {
    let mut board = board();
    // The complete framework-pack export is mislabeled incomplete.
    board.rows[0].repro_export.state = ReproExportState::Incomplete;
    let violations = board.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5AntiAbuseReproViolation::ReproExportStateMismatch { .. }
    )));
}

#[test]
fn silent_published_identity_bind_is_flagged() {
    let mut board = board();
    let row = board
        .rows
        .iter_mut()
        .find(|r| r.artifact_family == ArtifactFamily::SignedRecipePack)
        .unwrap();
    // A local-dev workspace claims a bound published identity without going to review.
    row.bind_decision = BindPublishedIdentity::BoundPublishedIdentity;
    let violations = board.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5AntiAbuseReproViolation::SilentPublishedIdentityBind { .. }
    )));
}

#[test]
fn missing_bind_review_ref_is_flagged() {
    let mut board = board();
    let row = board
        .rows
        .iter_mut()
        .find(|r| r.artifact_family == ArtifactFamily::DocsPack)
        .unwrap();
    row.bind_review_ref = String::new();
    let violations = board.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5AntiAbuseReproViolation::MissingBindReviewRef { .. })));
}

#[test]
fn tampered_quarantine_history_state_is_flagged() {
    let mut board = board();
    let row = board
        .rows
        .iter_mut()
        .find(|r| r.artifact_family == ArtifactFamily::MirroredRegistryVariant)
        .unwrap();
    row.quarantine_history_state = QuarantineHistoryState::Clean;
    let violations = board.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5AntiAbuseReproViolation::QuarantineHistoryStateMismatch { .. }
    )));
}

#[test]
fn tampered_publisher_continuity_state_is_flagged() {
    let mut board = board();
    let row = board
        .rows
        .iter_mut()
        .find(|r| r.artifact_family == ArtifactFamily::MirroredRegistryVariant)
        .unwrap();
    row.publisher_continuity_state = PublisherContinuityState::Continuous;
    let violations = board.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5AntiAbuseReproViolation::PublisherContinuityStateMismatch { .. }
    )));
}

#[test]
fn non_monotonic_history_sequence_is_flagged() {
    let mut board = board();
    let row = board
        .rows
        .iter_mut()
        .find(|r| r.artifact_family == ArtifactFamily::TemplateArtifact)
        .unwrap();
    row.history_events[1].sequence = row.history_events[0].sequence;
    let violations = board.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5AntiAbuseReproViolation::HistorySequenceNotMonotonic { .. }
    )));
}

#[test]
fn row_exceeding_publish_gate_is_flagged() {
    let mut board = board();
    let row = board
        .rows
        .iter_mut()
        .find(|r| r.artifact_family == ArtifactFamily::LocalModelPack)
        .unwrap();
    // The publish gate grants only unsigned_local_only for the local model pack.
    row.rendered_trust_posture = TrustPosture::RegistryBound;
    let violations = board.cross_check_matrix(&matrix());
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5AntiAbuseReproViolation::RowExceedsPublishGate { .. })));
}
