use super::*;

fn clean_posture_input() -> M5CredentialPostureEntryResolutionInput {
    M5CredentialPostureEntryResolutionInput {
        entry_id: "posture:test".to_owned(),
        acquisition_path_id: "entry.acme.clone-remote-public".to_owned(),
        token_name: "credential.posture.anonymous_public".to_owned(),
        semantic_role: M5RepositoryBootstrapRole::CredentialPosture,
        auth_source_kind: M5CredentialAuthSourceKind::AnonymousPublic,
        surface_context: M5TrustSurfaceContext::ShellSurface,
        resolution_form_coverage: M5TrustResolutionForm::ALL.to_vec(),
        auth_source_ref: "auth-source.acme/anonymous".to_owned(),
        proxy_or_mirror_route: "route.acme/public-upstream".to_owned(),
        host_key_or_tls_pin_state: "host-key.tofu-recorded.v3".to_owned(),
        delegated_token_policy: "delegated-token.not-required".to_owned(),
        handle_only_secret_reference: "secret-handle.none".to_owned(),
        mirror_or_signer_provenance: "signer-provenance.acme.v3".to_owned(),
        bound_to_registry: true,
        host_trust_disclosed: true,
        references_secret_material: false,
        secret_kept_handle_only: true,
        proof_fresh: true,
    }
}

fn clean_route_input() -> M5FetchRouteEntryResolutionInput {
    M5FetchRouteEntryResolutionInput {
        entry_id: "route:test".to_owned(),
        source_ref: "entry.acme.clone-remote-public".to_owned(),
        token_name: "fetch.route.public_upstream".to_owned(),
        semantic_role: M5RepositoryBootstrapRole::EvidencePacket,
        route_class: M5FetchRouteClass::PublicUpstreamFetch,
        surface_context: M5TrustSurfaceContext::ShellSurface,
        resolution_form_coverage: M5TrustResolutionForm::ALL.to_vec(),
        route_endpoint_class: "route-class.public-upstream".to_owned(),
        signer_continuity_ref: "signer-continuity.acme.v3".to_owned(),
        digest_continuity_ref: "digest-continuity.acme.v3".to_owned(),
        mirror_provenance_ref: "mirror-provenance.acme.v3".to_owned(),
        recovery_language: "recovery.resume-or-discard".to_owned(),
        trust_proof_ref: "trust-proof.acme.v3".to_owned(),
        keeps_signer_continuity_visible: true,
        route_is_truthful: true,
        crosses_offline_or_mirror: false,
        signer_continuity_preserved: false,
        asserts_recovery: false,
        recovery_explained: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_BOOTSTRAP_CREDENTIAL_POSTURE_FETCH_ROUTE_REGISTRIES_PACKET_ID
    );
}

#[test]
fn posture_clean_names_meaning_and_is_bound() {
    let resolved = resolve_credential_posture_entry(clean_posture_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.posture_resolves_across_entry_flows);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.credential_posture_object_complete);
    assert!(resolved.bound_to_registry);
    assert!(resolved.auth_source_kind_is_classified);
    assert!(resolved.host_trust_disclosed);
    assert_eq!(resolved.semantic_role, "credential_posture");
    assert_eq!(resolved.auth_source_kind, "anonymous_public");
    assert_eq!(resolved.canonical_auth_mode, "anonymous_public_auth");
    assert_eq!(resolved.surface_context, "shell_surface");
    assert_eq!(resolved.next_action, M5TrustNextAction::ExpandTrustMeaning);
}

#[test]
fn posture_token_unstated_degrades() {
    let mut input = clean_posture_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_credential_posture_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CredentialPostureEntryDegradeReason::PostureTokenUnstated)
    );
}

#[test]
fn posture_unbound_and_unclassified_degrade() {
    let mut input = clean_posture_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_credential_posture_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CredentialPostureEntryDegradeReason::PostureNotBoundToRegistry)
    );

    let mut input = clean_posture_input();
    input.auth_source_kind = M5CredentialAuthSourceKind::KindUnclassified;
    assert_eq!(
        resolve_credential_posture_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CredentialPostureEntryDegradeReason::CredentialAuthSourceUnclassified)
    );
}

#[test]
fn posture_object_incomplete_and_embed_and_form_degrade() {
    // An unstated proxy / mirror route leaves the resolved object incomplete.
    let mut input = clean_posture_input();
    input.proxy_or_mirror_route = "  ".to_owned();
    let resolved = resolve_credential_posture_entry(input).unwrap();
    assert!(!resolved.credential_posture_object_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5CredentialPostureEntryDegradeReason::CredentialPostureObjectIncomplete)
    );

    // A secret-referencing posture that embeds raw secret material degrades.
    let mut input = clean_posture_input();
    input.auth_source_kind = M5CredentialAuthSourceKind::DelegatedToken;
    input.references_secret_material = true;
    input.secret_kept_handle_only = false;
    assert_eq!(
        resolve_credential_posture_entry(input)
            .unwrap()
            .degrade_reason,
        Some(
            M5CredentialPostureEntryDegradeReason::CredentialPostureEmbedsRawSecretOrHidesHostTrust
        )
    );

    let mut input = clean_posture_input();
    input.resolution_form_coverage = vec![M5TrustResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_credential_posture_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CredentialPostureEntryDegradeReason::ResolutionFormCoverageIncomplete)
    );
}

#[test]
fn posture_host_trust_and_surface_and_proof_degrade() {
    let mut input = clean_posture_input();
    input.host_trust_disclosed = false;
    // A posture hiding its host-key / TLS-pin state first fails handle-only disclosure.
    assert_eq!(
        resolve_credential_posture_entry(input)
            .unwrap()
            .degrade_reason,
        Some(
            M5CredentialPostureEntryDegradeReason::CredentialPostureEmbedsRawSecretOrHidesHostTrust
        )
    );

    let mut input = clean_posture_input();
    input.surface_context = M5TrustSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_credential_posture_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CredentialPostureEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_posture_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_credential_posture_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CredentialPostureEntryDegradeReason::ProofStale)
    );
}

#[test]
fn posture_empty_id_and_forbidden_material_error() {
    let mut input = clean_posture_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_credential_posture_entry(input).unwrap_err(),
        M5TrustResolutionError::EmptyCredentialPostureEntryId
    );

    let mut input = clean_posture_input();
    input.mirror_or_signer_provenance = "see https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_credential_posture_entry(input).unwrap_err(),
        M5TrustResolutionError::ForbiddenMaterial
    );
}

#[test]
fn credential_posture_stays_handle_only_rejects_embed() {
    assert!(credential_posture_stays_handle_only(
        M5CredentialAuthSourceKind::AnonymousPublic,
        true,
        false,
        true
    ));
    assert!(!credential_posture_stays_handle_only(
        M5CredentialAuthSourceKind::AnonymousPublic,
        false,
        false,
        true
    ));
    assert!(credential_posture_stays_handle_only(
        M5CredentialAuthSourceKind::DelegatedToken,
        true,
        true,
        true
    ));
    assert!(!credential_posture_stays_handle_only(
        M5CredentialAuthSourceKind::DelegatedToken,
        true,
        true,
        false
    ));
    assert!(!credential_posture_stays_handle_only(
        M5CredentialAuthSourceKind::KindUnclassified,
        true,
        false,
        true
    ));
}

#[test]
fn credential_posture_object_is_complete_requires_all_fields() {
    assert!(credential_posture_object_is_complete(
        M5CredentialAuthSourceKind::AnonymousPublic,
        "auth-source.acme/anonymous",
        "route.acme/public-upstream",
        "host-key.tofu-recorded.v3",
        "delegated-token.not-required",
        "secret-handle.none",
        "signer-provenance.acme.v3",
    ));
    assert!(!credential_posture_object_is_complete(
        M5CredentialAuthSourceKind::AnonymousPublic,
        "auth-source.acme/anonymous",
        "  ",
        "host-key.tofu-recorded.v3",
        "delegated-token.not-required",
        "secret-handle.none",
        "signer-provenance.acme.v3",
    ));
    assert!(!credential_posture_object_is_complete(
        M5CredentialAuthSourceKind::KindUnclassified,
        "auth-source.acme/anonymous",
        "route.acme/public-upstream",
        "host-key.tofu-recorded.v3",
        "delegated-token.not-required",
        "secret-handle.none",
        "signer-provenance.acme.v3",
    ));
}

#[test]
fn route_clean_stays_continuous() {
    let resolved = resolve_fetch_route_entry(clean_route_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.route_safe_on_every_source);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.provides_complete_fetch_route);
    assert!(resolved.fetch_route_stays_signer_continuous);
    assert_eq!(resolved.route_class, "public_upstream_fetch");
    assert_eq!(resolved.surface_context, "shell_surface");
}

#[test]
fn route_continuity_break_and_unclassified_degrade() {
    // A mirrored fetch that drops signer continuity breaks the route.
    let mut input = clean_route_input();
    input.crosses_offline_or_mirror = true;
    input.signer_continuity_preserved = false;
    let resolved = resolve_fetch_route_entry(input).unwrap();
    assert!(!resolved.provides_complete_fetch_route);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5FetchRouteEntryDegradeReason::FetchRouteBreaksSignerContinuityOrHidesTrustProof)
    );

    // A route that hides its trust proof (signer continuity not visible) also breaks.
    let mut input = clean_route_input();
    input.keeps_signer_continuity_visible = false;
    assert_eq!(
        resolve_fetch_route_entry(input).unwrap().degrade_reason,
        Some(M5FetchRouteEntryDegradeReason::FetchRouteBreaksSignerContinuityOrHidesTrustProof)
    );

    // An unexplained recovery also breaks the route.
    let mut input = clean_route_input();
    input.asserts_recovery = true;
    input.recovery_explained = false;
    assert_eq!(
        resolve_fetch_route_entry(input).unwrap().degrade_reason,
        Some(M5FetchRouteEntryDegradeReason::FetchRouteBreaksSignerContinuityOrHidesTrustProof)
    );

    let mut input = clean_route_input();
    input.route_class = M5FetchRouteClass::RouteUnclassified;
    assert_eq!(
        resolve_fetch_route_entry(input).unwrap().degrade_reason,
        Some(M5FetchRouteEntryDegradeReason::FetchRouteClassUnclassified)
    );
}

#[test]
fn route_form_and_surface_and_id_and_material() {
    let mut input = clean_route_input();
    input.resolution_form_coverage = vec![M5TrustResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_fetch_route_entry(input).unwrap().degrade_reason,
        Some(M5FetchRouteEntryDegradeReason::RouteFormCoverageIncomplete)
    );

    let mut input = clean_route_input();
    input.surface_context = M5TrustSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_fetch_route_entry(input).unwrap().degrade_reason,
        Some(M5FetchRouteEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_route_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_fetch_route_entry(input).unwrap_err(),
        M5TrustResolutionError::EmptyFetchRouteEntryId
    );

    let mut input = clean_route_input();
    input.trust_proof_ref = "see internal://notes".to_owned();
    assert_eq!(
        resolve_fetch_route_entry(input).unwrap_err(),
        M5TrustResolutionError::ForbiddenMaterial
    );
}

#[test]
fn route_preserved_continuity_and_explained_recovery_stay_clean() {
    // A mirrored fetch that preserves signer continuity stays clean.
    let mut input = clean_route_input();
    input.crosses_offline_or_mirror = true;
    input.signer_continuity_preserved = true;
    assert!(resolve_fetch_route_entry(input).unwrap().is_clean());

    // An explained recovery stays clean.
    let mut input = clean_route_input();
    input.asserts_recovery = true;
    input.recovery_explained = true;
    assert!(resolve_fetch_route_entry(input).unwrap().is_clean());
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(
        seeded_m5_bootstrap_credential_posture_and_fetch_route_registries()
            .vocabulary_set
            .matches_canonical()
    );
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
    packet.vocabulary_set.credential_auth_sources.pop();
    assert!(packet
        .validate()
        .contains(&M5CredentialPostureFetchRouteRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CredentialPostureFetchRouteRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5CredentialPostureFetchRouteRegistriesViolation::DomainSchemaRefMissing));

    let mut packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_BOOTSTRAP_EVIDENCE_DOMAIN_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5CredentialPostureFetchRouteRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5TrustAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5CredentialPostureFetchRouteRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5TrustExportField::CredentialAuthSources);
    assert!(packet
        .validate()
        .contains(&M5CredentialPostureFetchRouteRegistriesViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
    packet.registry_rows[0].fetch_route_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5CredentialPostureFetchRouteRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
    // Force a clean posture entry to also read as object-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.credential_posture_entries[0].degrade_reason = None;
    row.credential_posture_entries[0].credential_posture_object_complete = false;
    assert!(packet
        .validate()
        .contains(&M5CredentialPostureFetchRouteRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.embeds_raw_secret_token_or_host_trust_state_in_portable_manifest = true,
            1 => row.loses_signer_or_mirror_provenance_across_offline_or_mirrored_fetches = true,
            2 => row.hides_bootstrap_credential_posture_behind_generic_connected_state_copy = true,
            _ => row.collapses_distinct_fetch_routes_into_one_runtime_path = true,
        }
        assert!(packet
            .validate()
            .contains(&M5CredentialPostureFetchRouteRegistriesViolation::RowInvariantViolated));
    }
}

#[test]
fn credential_posture_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
    for row in &mut packet.registry_rows {
        row.credential_posture_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5CredentialPostureEntryDegradeReason::CredentialPostureObjectIncomplete)
        });
    }
    assert!(packet.validate().contains(
        &M5CredentialPostureFetchRouteRegistriesViolation::CredentialPostureResolutionNotProven
    ));
}

#[test]
fn credential_posture_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
    // Drop every clean admin-surface posture so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.credential_posture_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "admin_surface"));
    }
    assert!(packet.validate().contains(
        &M5CredentialPostureFetchRouteRegistriesViolation::CredentialPostureResolutionNotProven
    ));
}

#[test]
fn handle_only_preservation_not_proven_when_embed_example_removed() {
    let mut packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
    for row in &mut packet.registry_rows {
        row.credential_posture_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5CredentialPostureEntryDegradeReason::CredentialPostureEmbedsRawSecretOrHidesHostTrust,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5CredentialPostureFetchRouteRegistriesViolation::HandleOnlySecretPreservationNotProven
    ));
}

#[test]
fn handle_only_preservation_not_proven_when_unbound_example_removed() {
    let mut packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
    for row in &mut packet.registry_rows {
        row.credential_posture_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5CredentialPostureEntryDegradeReason::PostureNotBoundToRegistry)
        });
    }
    assert!(packet.validate().contains(
        &M5CredentialPostureFetchRouteRegistriesViolation::HandleOnlySecretPreservationNotProven
    ));
}

#[test]
fn fetch_route_continuity_not_proven_when_break_example_removed() {
    let mut packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
    for row in &mut packet.registry_rows {
        row.fetch_route_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5FetchRouteEntryDegradeReason::FetchRouteBreaksSignerContinuityOrHidesTrustProof,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5CredentialPostureFetchRouteRegistriesViolation::FetchRouteContinuityNotProven
    ));
}

#[test]
fn fetch_route_continuity_not_proven_when_class_dropped() {
    let mut packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
    // Drop every clean air-gap bundle route so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.fetch_route_entries
            .retain(|ex| !(ex.is_clean() && ex.route_class == "air_gap_bundle_import"));
    }
    assert!(packet.validate().contains(
        &M5CredentialPostureFetchRouteRegistriesViolation::FetchRouteContinuityNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
    packet
        .governance_review
        .credential_posture_stays_handle_only_no_raw_secret = false;
    assert!(packet
        .validate()
        .contains(&M5CredentialPostureFetchRouteRegistriesViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet
        .validate()
        .contains(&M5CredentialPostureFetchRouteRegistriesViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5CredentialPostureFetchRouteRegistriesViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5CredentialPostureFetchRouteRegistriesViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://clone.example/repo leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5CredentialPostureFetchRouteRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json =
        seeded_m5_bootstrap_credential_posture_and_fetch_route_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn credential_posture_table_lists_only_clean_postures() {
    let packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
    let table = packet.render_credential_posture_table();
    // The clean anonymous-public and delegated-token postures are rendered from the registry.
    assert!(table.contains("anonymous_public_auth"));
    assert!(table.contains("delegated_token_auth"));
    // A degraded, incomplete entry never leaks into the generated table.
    assert!(!table.contains("incomplete"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk =
        current_stable_m5_bootstrap_credential_posture_and_fetch_route_registries_export()
            .expect("checked M5 credential-posture / fetch-route registries export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_BOOTSTRAP_CREDENTIAL_POSTURE_FETCH_ROUTE_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_bootstrap_credential_posture_and_fetch_route_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_bootstrap_credential_posture_and_fetch_route_registries_air_gap_bundle_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5RepositoryBootstrapConsumerSurface::TrustService)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5RepositoryBootstrapQualificationClass::Beta
    );

    let preview =
        seeded_m5_bootstrap_credential_posture_and_fetch_route_registries_managed_snapshot_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.registry_rows.len(), 6);
    let row = preview
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5RepositoryBootstrapConsumerSurface::Diagnostics)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5RepositoryBootstrapQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5CredentialPostureFetchRouteRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/workspaces/m5-bootstrap-credential-posture-and-fetch-route-registries/air_gap_bundle_beta_narrowed.json"
    )))
    .expect("air-gap fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_bootstrap_credential_posture_and_fetch_route_registries_air_gap_bundle_beta_narrowed()
    );

    let preview: M5CredentialPostureFetchRouteRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/workspaces/m5-bootstrap-credential-posture-and-fetch-route-registries/managed_snapshot_preview_narrowed.json"
    )))
    .expect("managed-snapshot fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_bootstrap_credential_posture_and_fetch_route_registries_managed_snapshot_preview_narrowed()
    );
}

#[test]
fn implemented_families_is_clone_remote_import_bundle_and_resume_snapshot() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5RepositoryBootstrapFamily::CloneRemote,
            M5RepositoryBootstrapFamily::ImportBundle,
            M5RepositoryBootstrapFamily::ResumeSnapshot,
        ]
    );
}
