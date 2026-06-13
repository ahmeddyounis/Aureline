use super::*;

fn page() -> TransportTrustPage {
    seeded_transport_trust_page()
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
    assert!(validate_transport_trust_page(&page).is_ok());
    assert!(page.qualifies_stable());
    assert!(page.no_withdrawn_rows());
    assert_eq!(
        page.summary.overall_qualification_token,
        TrustQualificationClass::Stable.as_str()
    );
}

#[test]
fn seeded_page_passes_all_stability_conditions() {
    let page = page();
    assert!(page.covers_all_required_surfaces());
    assert!(page.no_record_ships_direct_ca_override());
    assert!(page.no_record_allows_silent_trust_downgrade());
    assert!(page.all_records_expose_host_proof_state());
    assert!(page.all_records_expose_trust_inputs());
    assert!(page.rotation_cues_consistent());
    assert!(page.denied_records_carry_reasons());
    assert!(page.egress_classes_have_policy_epoch_refs());
}

#[test]
fn seeded_page_covers_all_required_surfaces() {
    let page = page();
    let covered = page.trust_snapshot.covered_surface_tokens();
    assert_eq!(covered.len(), REQUIRED_SURFACES.len());
    for required in &REQUIRED_SURFACES {
        assert!(
            covered.contains(required.as_str()),
            "required surface '{}' must have a record",
            required.as_str()
        );
    }
    assert_eq!(page.rows.len(), REQUIRED_SURFACES.len());
    assert_eq!(page.summary.stable_row_count, REQUIRED_SURFACES.len());
}

#[test]
fn every_record_excludes_raw_material_and_uses_handle_only_descriptors() {
    let snapshot = seeded_transport_trust_snapshot();
    for record in &snapshot.records {
        assert!(
            record.raw_material_excluded(),
            "record '{}' must exclude raw trust and private-key material",
            record.record_id
        );
        assert!(record.raw_trust_material_excluded);
        assert!(record.private_key_material_excluded);
        assert!(
            record.ca_bundle.bundle_handle.starts_with("trust_bundle:"),
            "ca bundle for '{}' must be an opaque handle, got '{}'",
            record.record_id,
            record.ca_bundle.bundle_handle
        );
        assert!(!record.ca_bundle.is_direct_ca_override);
        assert!(
            record.host_proof.proof_handle.starts_with("host_proof:"),
            "host proof for '{}' must be an opaque handle",
            record.record_id
        );
        // A client-cert binding handle, when present, is opaque.
        if record.client_cert.posture.expects_binding() {
            assert!(
                record
                    .client_cert
                    .binding_handle
                    .starts_with("client_cert:"),
                "client cert binding for '{}' must be an opaque handle",
                record.record_id
            );
        }
    }
}

#[test]
fn every_record_is_fully_classified_and_does_not_bypass() {
    let snapshot = seeded_transport_trust_snapshot();
    for record in &snapshot.records {
        assert!(
            record.is_fully_classified(),
            "record '{}' must be fully classified",
            record.record_id
        );
        assert!(
            record.exposes_host_proof_state(),
            "record '{}' must expose a host-proof state",
            record.record_id
        );
        assert!(
            record.rotation_cue_consistent(),
            "record '{}' must carry a consistent rotation cue",
            record.record_id
        );
        assert!(
            record.no_bypass(),
            "record '{}' must not bypass trust governance",
            record.record_id
        );
    }
}

#[test]
fn seeded_snapshot_exercises_every_trust_store_source_and_a_denial() {
    let snapshot = seeded_transport_trust_snapshot();
    let sources: BTreeSet<&str> = snapshot
        .records
        .iter()
        .map(|r| r.ca_bundle.trust_store_source_token.as_str())
        .collect();
    for source in [
        TrustStoreSourceClass::SystemTrustStore,
        TrustStoreSourceClass::PinnedCaSet,
        TrustStoreSourceClass::ManagedOrgBundle,
        TrustStoreSourceClass::MirrorRoot,
        TrustStoreSourceClass::NoTlsLoopback,
    ] {
        assert!(
            sources.contains(source.as_str()),
            "seeded snapshot must use trust-store source '{}'",
            source.as_str()
        );
    }
    // A typed deny_trust outcome (changed host proof) must be present.
    assert!(snapshot.records.iter().any(|r| {
        r.outcome == TrustOutcomeClass::DenyTrust
            && r.denial_reason == Some(TrustDenialClass::HostProofChanged)
    }));
    // A rotation cue (rotate_soon) and an in-progress rotation must be present.
    assert!(snapshot
        .records
        .iter()
        .any(|r| r.trust_root.rotation_cue == RotationCueClass::RotateSoon));
    assert!(snapshot
        .records
        .iter()
        .any(|r| r.trust_root.freshness == TrustRootFreshnessClass::RotationInProgress));
}

#[test]
fn seeded_snapshot_exercises_client_cert_postures() {
    let snapshot = seeded_transport_trust_snapshot();
    let postures: BTreeSet<&str> = snapshot
        .records
        .iter()
        .map(|r| r.client_cert.posture_token.as_str())
        .collect();
    assert!(postures.contains(ClientCertPostureClass::RequiredPresented.as_str()));
    assert!(postures.contains(ClientCertPostureClass::ManagedProvisioned.as_str()));
    assert!(postures.contains(ClientCertPostureClass::NotRequired.as_str()));
}

#[test]
fn drill_missing_surface_narrows_to_preview() {
    let mut snapshot = seeded_transport_trust_snapshot();
    snapshot
        .records
        .retain(|r| r.surface != SurfaceClass::AiGateway);
    let page = TransportTrustPage::new(
        "remote:networked_surface_transport_trust:drill:missing-surface",
        "Drill — required surface absent (preview)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert!(!page.covers_all_required_surfaces());
    assert_eq!(
        page.summary.overall_qualification_token,
        TrustQualificationClass::Preview.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == TrustNarrowReasonClass::RequiredSurfaceMissing));
}

#[test]
fn drill_raw_trust_material_withdraws_packet() {
    let mut snapshot = seeded_transport_trust_snapshot();
    for r in snapshot.records.iter_mut() {
        if r.surface == SurfaceClass::RequestApiClient {
            r.raw_trust_material_excluded = false;
        }
    }
    let page = TransportTrustPage::new(
        "remote:networked_surface_transport_trust:drill:raw-material",
        "Drill — raw trust material exposed (withdrawn)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        TrustQualificationClass::Withdrawn.as_str()
    );
    assert!(!page.no_withdrawn_rows());
    assert!(page
        .rows
        .iter()
        .all(|r| r.qualification_token == TrustQualificationClass::Withdrawn.as_str()));
}

#[test]
fn drill_private_key_material_withdraws_packet() {
    let mut snapshot = seeded_transport_trust_snapshot();
    for r in snapshot.records.iter_mut() {
        if r.surface == SurfaceClass::DatabaseCloudConnector {
            r.private_key_material_excluded = false;
        }
    }
    let page = TransportTrustPage::new(
        "remote:networked_surface_transport_trust:drill:private-key",
        "Drill — raw private-key material exposed (withdrawn)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        TrustQualificationClass::Withdrawn.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == TrustNarrowReasonClass::PrivateKeyMaterialExposed));
}

#[test]
fn drill_direct_ca_override_withdraws_packet() {
    let mut snapshot = seeded_transport_trust_snapshot();
    for r in snapshot.records.iter_mut() {
        if r.surface == SurfaceClass::DocsBrowserFetcher {
            r.ca_bundle.is_direct_ca_override = true;
            r.no_direct_ca_override = false;
        }
    }
    let page = TransportTrustPage::new(
        "remote:networked_surface_transport_trust:drill:ca-override",
        "Drill — direct CA override shipped (withdrawn)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        TrustQualificationClass::Withdrawn.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == TrustNarrowReasonClass::DirectCaOverrideShipped));
}

#[test]
fn drill_silent_trust_downgrade_withdraws_packet() {
    let mut snapshot = seeded_transport_trust_snapshot();
    for r in snapshot.records.iter_mut() {
        if r.surface == SurfaceClass::RegistryRead {
            r.no_silent_trust_downgrade = false;
        }
    }
    let page = TransportTrustPage::new(
        "remote:networked_surface_transport_trust:drill:silent-downgrade",
        "Drill — silent trust downgrade (withdrawn)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        TrustQualificationClass::Withdrawn.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == TrustNarrowReasonClass::SilentTrustDowngrade));
}

#[test]
fn drill_denied_without_reason_narrows_to_beta() {
    let mut snapshot = seeded_transport_trust_snapshot();
    for r in snapshot.records.iter_mut() {
        if r.surface == SurfaceClass::SyncOffboarding {
            r.outcome = TrustOutcomeClass::DenyTrust;
            r.outcome_token = TrustOutcomeClass::DenyTrust.as_str().to_owned();
            r.denial_reason = None;
            r.denial_reason_token = String::new();
        }
    }
    let page = TransportTrustPage::new(
        "remote:networked_surface_transport_trust:drill:denied-no-reason",
        "Drill — denied without reason (beta)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        TrustQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == TrustNarrowReasonClass::DenyReasonMissing));
}

#[test]
fn drill_missing_rotation_cue_narrows_row_to_beta() {
    let mut snapshot = seeded_transport_trust_snapshot();
    // docs_browser_fetcher needs an active cue (rotation_due); clear it.
    for r in snapshot.records.iter_mut() {
        if r.surface == SurfaceClass::DocsBrowserFetcher {
            r.trust_root.rotation_cue = RotationCueClass::None;
            r.trust_root.rotation_cue_token = RotationCueClass::None.as_str().to_owned();
            // Keep the outcome consistent with a non-deny host proof.
            r.outcome = TrustOutcomeClass::Trusted;
            r.outcome_token = TrustOutcomeClass::Trusted.as_str().to_owned();
        }
    }
    let page = TransportTrustPage::new(
        "remote:networked_surface_transport_trust:drill:missing-cue",
        "Drill — rotation cue missing (beta)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        TrustQualificationClass::Beta.as_str()
    );
    let row = page
        .rows
        .iter()
        .find(|r| r.surface_token == SurfaceClass::DocsBrowserFetcher.as_str())
        .expect("docs row present");
    assert_eq!(
        row.narrow_reason_token,
        TrustNarrowReasonClass::RotationCueMissing.as_str()
    );
}

#[test]
fn drill_missing_policy_epoch_on_required_egress_narrows_to_beta() {
    let mut snapshot = seeded_transport_trust_snapshot();
    for r in snapshot.records.iter_mut() {
        if r.surface == SurfaceClass::AiGateway {
            r.policy_epoch_ref = None;
        }
    }
    let page = TransportTrustPage::new(
        "remote:networked_surface_transport_trust:drill:missing-epoch",
        "Drill — missing policy epoch (beta)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == TrustNarrowReasonClass::PolicyEpochRefMissing));
    assert_eq!(
        page.summary.overall_qualification_token,
        TrustQualificationClass::Beta.as_str()
    );
}

#[test]
fn drill_inconsistent_trusted_outcome_over_deny_host_proof_narrows_to_beta() {
    let mut snapshot = seeded_transport_trust_snapshot();
    // Mark the request client trusted while its host proof is changed — inconsistent.
    for r in snapshot.records.iter_mut() {
        if r.surface == SurfaceClass::RequestApiClient {
            r.host_proof.state = HostProofStateClass::ChangedMismatch;
            r.host_proof.state_token = HostProofStateClass::ChangedMismatch.as_str().to_owned();
        }
    }
    let page = TransportTrustPage::new(
        "remote:networked_surface_transport_trust:drill:inconsistent",
        "Drill — trusted over deny host proof (beta)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        TrustQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == TrustNarrowReasonClass::TrustInputClassificationIncomplete));
}

#[test]
fn summary_rolls_up_sources_states_and_outcomes() {
    let page = page();
    assert_eq!(
        page.summary.no_direct_ca_override_count,
        REQUIRED_SURFACES.len()
    );
    assert_eq!(
        page.summary.host_proof_state_exposed_count,
        REQUIRED_SURFACES.len()
    );
    let denied = page
        .summary
        .outcome_counts
        .get(TrustOutcomeClass::DenyTrust.as_str())
        .copied()
        .unwrap_or(0);
    assert_eq!(denied, 1, "exactly one deny_trust record is seeded");
    let managed = page
        .summary
        .trust_store_source_counts
        .get(TrustStoreSourceClass::ManagedOrgBundle.as_str())
        .copied()
        .unwrap_or(0);
    assert!(
        managed >= 1,
        "managed org bundle must be used at least once"
    );
    let pinned_match = page
        .summary
        .host_proof_state_counts
        .get(HostProofStateClass::PinnedMatch.as_str())
        .copied()
        .unwrap_or(0);
    assert!(pinned_match >= 1);
}

#[test]
fn support_export_rolls_up_defects_and_excludes_raw_material() {
    let page = page();
    let export = TrustSupportExport::from_page(
        "remote:networked_surface_transport_trust:support-export:fixture-001",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert!(export.raw_trust_material_excluded);
    assert!(export.private_key_material_excluded);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.defect_counts_by_narrow_reason.is_empty());
    assert_eq!(
        export.record_kind,
        TRANSPORT_TRUST_SUPPORT_EXPORT_RECORD_KIND
    );
}

#[test]
fn cli_view_quotes_stable_trust_and_outcome_tokens() {
    let page = page();
    let view = page.render_cli_view();
    assert!(view.contains(TrustStoreSourceClass::ManagedOrgBundle.as_str()));
    assert!(view.contains(TrustOutcomeClass::DenyTrust.as_str()));
    assert!(view.contains(TrustDenialClass::HostProofChanged.as_str()));
    assert!(view.contains(RotationCueClass::RotateSoon.as_str()));
    assert!(view.contains("ai_gateway"));
}

#[test]
fn page_carries_stable_metadata_and_evidence_index_ref() {
    let page = page();
    assert_eq!(page.record_kind, TRANSPORT_TRUST_PAGE_RECORD_KIND);
    assert_eq!(page.schema_version, TRANSPORT_TRUST_SCHEMA_VERSION);
    assert_eq!(
        page.shared_contract_ref,
        TRANSPORT_TRUST_SHARED_CONTRACT_REF
    );
    assert_eq!(page.evidence_index_ref, TRANSPORT_TRUST_EVIDENCE_INDEX_REF);
}

#[test]
fn page_round_trips_through_json() {
    let page = page();
    let json = serde_json::to_string(&page).expect("serialize");
    let restored: TransportTrustPage = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(page, restored);
}
