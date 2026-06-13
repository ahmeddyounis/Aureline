use super::*;

fn page() -> NetworkedSurfaceTransportMatrixPage {
    seeded_networked_surface_matrix_page()
}

#[test]
fn seeded_page_seeds_zero_defects_and_qualifies_stable() {
    let page = page();
    assert_eq!(
        page.defects.len(),
        0,
        "seeded page must be clean: {:?}",
        page.defects
    );
    assert!(validate_networked_surface_matrix_page(&page).is_ok());
    assert!(page.qualifies_stable());
    assert!(page.no_withdrawn_rows());
    assert_eq!(
        page.summary.overall_qualification_token,
        MatrixQualificationClass::Stable.as_str()
    );
}

#[test]
fn seeded_page_passes_all_stability_conditions() {
    let page = page();
    assert!(
        page.covers_all_required_surfaces(),
        "all required surfaces must be covered"
    );
    assert!(
        page.all_surfaces_preserve_local_core_continuity(),
        "all surfaces must preserve local-core continuity"
    );
    assert!(
        page.no_surface_allows_silent_public_fallback(),
        "no surface may permit a silent public fall-through"
    );
    assert!(
        page.replay_queues_are_idempotent_only(),
        "offline/replay queues must be idempotent-only"
    );
    assert!(
        page.egress_classes_have_policy_epoch_refs(),
        "egress classes that require a policy epoch must carry one"
    );
    assert!(
        page.all_surfaces_declare_trust_and_denial(),
        "all surfaces must declare trust material and a denial vocabulary"
    );
    assert!(
        page.all_surface_proofs_usable(),
        "all surface proofs must be usable"
    );
}

#[test]
fn seeded_page_covers_all_required_surfaces() {
    let page = page();
    let covered = page.matrix_snapshot.covered_surface_tokens();
    assert_eq!(covered.len(), REQUIRED_SURFACES.len());
    for required in &REQUIRED_SURFACES {
        assert!(
            covered.contains(required.as_str()),
            "required surface '{}' must be covered",
            required.as_str()
        );
    }
    assert_eq!(page.rows.len(), REQUIRED_SURFACES.len());
    assert_eq!(page.summary.stable_row_count, REQUIRED_SURFACES.len());
}

#[test]
fn every_surface_record_excludes_raw_material_and_uses_handle_only_auth() {
    let snapshot = seeded_networked_surface_matrix_snapshot();
    for record in &snapshot.records {
        assert!(
            record.raw_private_material_excluded,
            "surface '{}' must exclude raw private material",
            record.surface_token
        );
        // Every non-anonymous auth posture must be a handle (token contains
        // "handle"); no raw credential shapes are permitted.
        if record.auth_posture != AuthPostureClass::Anonymous {
            assert!(
                record.auth_posture_token.ends_with("_handle"),
                "surface '{}' auth posture '{}' must be handle-only",
                record.surface_token,
                record.auth_posture_token
            );
        }
    }
}

#[test]
fn confined_surfaces_never_fall_through_to_public() {
    let snapshot = seeded_networked_surface_matrix_snapshot();
    for record in &snapshot.records {
        if record.egress_class.is_confined() {
            assert!(
                record.no_silent_public_fallback,
                "confined surface '{}' must forbid silent public fall-through",
                record.surface_token
            );
            assert_ne!(
                record.mirror_offline_behavior,
                MirrorOfflineBehaviorClass::CachedOffline,
                "confined surface '{}' must not silently serve from a public-fed cache",
                record.surface_token
            );
        }
    }
}

#[test]
fn drill_missing_surface_narrows_to_preview() {
    let mut snapshot = seeded_networked_surface_matrix_snapshot();
    snapshot
        .records
        .retain(|r| r.surface != SurfaceClass::AiGateway);
    let page = NetworkedSurfaceTransportMatrixPage::new(
        "remote:networked_surface_transport_matrix:drill:missing-surface",
        "Drill — required surface absent (preview)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert!(!page.covers_all_required_surfaces());
    assert_eq!(
        page.summary.overall_qualification_token,
        MatrixQualificationClass::Preview.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == MatrixNarrowReasonClass::RequiredSurfaceMissing));
}

#[test]
fn drill_raw_material_withdraws_packet() {
    let mut snapshot = seeded_networked_surface_matrix_snapshot();
    for rec in snapshot.records.iter_mut() {
        if rec.surface == SurfaceClass::RequestApiClient {
            rec.raw_private_material_excluded = false;
        }
    }
    let page = NetworkedSurfaceTransportMatrixPage::new(
        "remote:networked_surface_transport_matrix:drill:raw-material",
        "Drill — raw private material exposed (withdrawn)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        MatrixQualificationClass::Withdrawn.as_str()
    );
    assert!(!page.no_withdrawn_rows());
    // Withdrawal taints every row.
    assert!(page
        .rows
        .iter()
        .all(|r| r.qualification_token == MatrixQualificationClass::Withdrawn.as_str()));
}

#[test]
fn drill_silent_public_fallback_withdraws_packet() {
    let mut snapshot = seeded_networked_surface_matrix_snapshot();
    for rec in snapshot.records.iter_mut() {
        if rec.surface == SurfaceClass::RegistryRead {
            rec.no_silent_public_fallback = false;
        }
    }
    let page = NetworkedSurfaceTransportMatrixPage::new(
        "remote:networked_surface_transport_matrix:drill:silent-fallback",
        "Drill — silent public fall-through permitted (withdrawn)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        MatrixQualificationClass::Withdrawn.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == MatrixNarrowReasonClass::SilentPublicFallbackAllowed));
}

#[test]
fn drill_non_idempotent_replay_withdraws_packet() {
    let mut snapshot = seeded_networked_surface_matrix_snapshot();
    for rec in snapshot.records.iter_mut() {
        if rec.surface == SurfaceClass::SyncOffboarding {
            rec.offline_deferral_allowed = true;
            rec.replay_idempotent_only = false;
        }
    }
    let page = NetworkedSurfaceTransportMatrixPage::new(
        "remote:networked_surface_transport_matrix:drill:non-idempotent-replay",
        "Drill — non-idempotent replay queued (withdrawn)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        MatrixQualificationClass::Withdrawn.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == MatrixNarrowReasonClass::NonIdempotentReplayQueued));
}

#[test]
fn drill_stale_proof_narrows_row_to_beta() {
    let mut snapshot = seeded_networked_surface_matrix_snapshot();
    for rec in snapshot.records.iter_mut() {
        if rec.surface == SurfaceClass::DocsBrowserFetcher {
            rec.proof_freshness = ProofFreshnessClass::ExpiredBeyondWindow;
            rec.proof_freshness_token =
                ProofFreshnessClass::ExpiredBeyondWindow.as_str().to_owned();
        }
    }
    let page = NetworkedSurfaceTransportMatrixPage::new(
        "remote:networked_surface_transport_matrix:drill:stale-proof",
        "Drill — stale proof beyond window (beta)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        MatrixQualificationClass::Beta.as_str()
    );
    let docs_row = page
        .rows
        .iter()
        .find(|r| r.surface_token == SurfaceClass::DocsBrowserFetcher.as_str())
        .expect("docs row present");
    assert_eq!(
        docs_row.narrow_reason_token,
        MatrixNarrowReasonClass::ProofStaleBeyondWindow.as_str()
    );
    assert_eq!(
        docs_row.qualification_token,
        MatrixQualificationClass::Beta.as_str()
    );
    // Other rows remain stable.
    let ai_row = page
        .rows
        .iter()
        .find(|r| r.surface_token == SurfaceClass::AiGateway.as_str())
        .expect("ai row present");
    assert_eq!(
        ai_row.qualification_token,
        MatrixQualificationClass::Stable.as_str()
    );
}

#[test]
fn missing_policy_epoch_on_required_egress_narrows_to_beta() {
    let mut snapshot = seeded_networked_surface_matrix_snapshot();
    for rec in snapshot.records.iter_mut() {
        if rec.surface == SurfaceClass::AiGateway {
            rec.policy_epoch_ref = None;
        }
    }
    let page = NetworkedSurfaceTransportMatrixPage::new(
        "remote:networked_surface_transport_matrix:drill:missing-epoch",
        "Drill — missing policy epoch (beta)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == MatrixNarrowReasonClass::PolicyEpochRefMissing));
    assert_eq!(
        page.summary.overall_qualification_token,
        MatrixQualificationClass::Beta.as_str()
    );
}

#[test]
fn support_export_rolls_up_defects_and_excludes_raw_material() {
    let page = page();
    let export = NetworkedSurfaceMatrixSupportExport::from_page(
        "remote:networked_surface_transport_matrix:support-export:fixture-001",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert!(export.raw_private_material_excluded);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.defect_counts_by_narrow_reason.is_empty());
    assert_eq!(
        export.record_kind,
        NETWORKED_SURFACE_MATRIX_SUPPORT_EXPORT_RECORD_KIND
    );
}

#[test]
fn page_carries_stable_metadata_and_evidence_index_ref() {
    let page = page();
    assert_eq!(page.record_kind, NETWORKED_SURFACE_MATRIX_PAGE_RECORD_KIND);
    assert_eq!(page.schema_version, NETWORKED_SURFACE_MATRIX_SCHEMA_VERSION);
    assert_eq!(
        page.shared_contract_ref,
        NETWORKED_SURFACE_MATRIX_SHARED_CONTRACT_REF
    );
    assert_eq!(
        page.evidence_index_ref,
        NETWORKED_SURFACE_MATRIX_EVIDENCE_INDEX_REF
    );
}

#[test]
fn page_round_trips_through_json() {
    let page = page();
    let json = serde_json::to_string(&page).expect("serialize");
    let restored: NetworkedSurfaceTransportMatrixPage =
        serde_json::from_str(&json).expect("deserialize");
    assert_eq!(page, restored);
}
