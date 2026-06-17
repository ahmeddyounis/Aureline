use super::*;

fn packet() -> ManifestScopeReview {
    current_manifest_scope_review().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, MANIFEST_SCOPE_REVIEW_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, MANIFEST_SCOPE_REVIEW_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn path_is_stable() {
    assert_eq!(
        MANIFEST_SCOPE_REVIEW_PATH,
        "artifacts/deps/m5/manifest-scope-review.json"
    );
}

#[test]
fn every_row_binds_to_the_frozen_matrix() {
    let packet = packet();
    let matrix = current_m5_package_state_matrix().expect("matrix loads");
    assert_eq!(packet.references_matrix_id, matrix.packet_id);
    assert!(packet.all_bind_matrix());
    for row in &packet.rows {
        for label in row.applicable_labels() {
            assert!(
                matrix.state(label).is_some(),
                "row {} surfaces unbound label {}",
                row.row_id,
                label.as_str()
            );
        }
    }
}

#[test]
fn root_and_member_targets_are_distinguished() {
    let packet = packet();
    let root = packet
        .row("msr:root:cargo:workspace-bump:exact")
        .expect("root row");
    let member = packet
        .row("msr:member:node:add-dep:exact")
        .expect("member row");
    assert!(root.targets_root() && !root.targets_member());
    assert!(member.targets_member() && !member.targets_root());
    assert_ne!(
        root.requested_scope.manifest_id,
        member.requested_scope.manifest_id
    );
    // A member names its parent root; a root names none.
    assert!(member.requested_scope.parent_manifest_id.is_some());
    assert!(root.requested_scope.parent_manifest_id.is_none());
}

#[test]
fn exact_operation_does_not_broaden() {
    let packet = packet();
    let exact = packet
        .row("msr:member:node:add-dep:exact")
        .expect("exact row");
    assert!(!exact.broadens_beyond_target());
    assert!(!exact.is_silent_broadening());
    assert!(exact.can_apply());
    let diff = exact.scope_diff();
    assert!(!diff.broadened);
    assert_eq!(
        diff.affected_manifest_ids,
        vec![exact.requested_scope.manifest_id.clone()]
    );
}

#[test]
fn disclosed_shared_lockfile_broadens_but_stays_appliable() {
    let packet = packet();
    let row = packet
        .row("msr:member:cargo:shared-lockfile")
        .expect("shared lockfile row");
    assert_eq!(row.fidelity, ScopeFidelity::DisclosedSharedLockfile);
    assert!(row.broadens_beyond_target());
    assert!(!row.is_silent_broadening());
    // The member stays the target even though the root lockfile is affected.
    assert!(row.targets_member());
    assert!(row
        .affected_manifest_ids
        .contains(&"manifest:cargo:workspace-root".to_owned()));
    assert!(row.can_apply());
}

#[test]
fn confirmed_workspace_wide_requires_confirmation() {
    let packet = packet();
    let row = packet
        .row("msr:root:node:workspace-converge:confirmed")
        .expect("confirmed row");
    assert_eq!(row.fidelity, ScopeFidelity::ConfirmedWorkspaceWide);
    assert!(row.broadening_confirmed);
    assert!(row.broadens_beyond_target());
    assert!(row.resolved_scope.requires_explicit_confirmation());
    assert!(row.can_apply());
}

#[test]
fn unconfirmed_broadening_is_never_appliable() {
    let packet = packet();
    let row = packet
        .row("msr:member:pip:would-broaden:blocked")
        .expect("blocked row");
    assert_eq!(row.fidelity, ScopeFidelity::UnconfirmedBroadening);
    assert!(row.is_silent_broadening());
    assert!(row.broadens_beyond_target());
    // The guard: a member request widening to the workspace without confirmation
    // can never be applied.
    assert!(!row.can_apply());
    assert!(packet.no_appliable_silent_broadening());
}

#[test]
fn member_request_does_not_silently_become_workspace_wide() {
    let packet = packet();
    let row = packet
        .row("msr:member:pip:would-broaden:blocked")
        .expect("blocked row");
    let diff = row.scope_diff();
    // The requested target is a member; the resolved scope is the workspace root.
    assert_eq!(diff.requested_role, ManifestRole::WorkspaceMember.as_str());
    assert_eq!(diff.resolved_role, ManifestRole::WorkspaceRoot.as_str());
    assert!(diff.role_changed);
    assert!(diff.scope_class_changed);
    assert!(diff.broadened);
    assert!(!diff.broadening_confirmed);
    assert!(diff.silent_broadening);
}

#[test]
fn auth_blocked_source_blocks_trust_and_apply() {
    let packet = packet();
    let row = packet
        .row("msr:member:node:auth-blocked")
        .expect("auth-blocked row");
    assert!(row.source_cue.trust_blocked());
    assert!(!row.can_apply());
    assert!(row.resolved.is_none());
    // The source still discloses itself specifically, never a generic failure.
    assert!(row.source_cue.message_class().is_specific());
    assert!(row.source_view().must_disclose);
}

#[test]
fn revoked_source_blocks_trust() {
    let packet = packet();
    let row = packet
        .row("msr:member:cargo:revoked-mirror")
        .expect("revoked row");
    assert_eq!(row.source_cue.revocation, RevocationState::Revoked);
    assert!(row.source_cue.trust_blocked());
    assert!(!row.can_apply());
}

#[test]
fn offline_source_discloses_but_does_not_block() {
    let packet = packet();
    let row = packet
        .row("msr:member:pip:offline-snapshot")
        .expect("offline row");
    assert_eq!(row.source_cue.freshness, SourceFreshness::OfflineSnapshot);
    assert!(!row.source_cue.trust_blocked());
    assert!(row.source_view().must_disclose);
    assert!(row.can_apply());
}

#[test]
fn mirror_owner_is_present_only_for_private_or_mirror_sources() {
    let packet = packet();
    for row in &packet.rows {
        let has_owner = RegistrySourceCue::class_has_owner(row.source_cue.source_class);
        assert_eq!(
            row.source_cue.mirror_owner.is_some(),
            has_owner,
            "row {} mirror owner presence disagrees with source class",
            row.row_id
        );
    }
    let public = packet
        .row("msr:root:cargo:workspace-bump:exact")
        .expect("public row");
    assert!(public.source_cue.mirror_owner.is_none());
    let mirror = packet
        .row("msr:member:cargo:shared-lockfile")
        .expect("mirror row");
    assert!(mirror.source_cue.mirror_owner.is_some());
}

#[test]
fn requested_and_resolved_identity_stay_separate() {
    let packet = packet();
    assert!(packet.requested_resolved_separate());
    let pinned = packet
        .row("msr:member:cargo:revoked-mirror")
        .expect("pinned row");
    // A policy pin is a requested constraint, never a resolved fact.
    assert!(pinned
        .requested_labels()
        .contains(&PackageStateLabel::PolicyPinned));
    assert!(!pinned
        .resolved_labels()
        .contains(&PackageStateLabel::PolicyPinned));
    assert!(pinned.requested_and_resolved_separate());
}

#[test]
fn scope_identity_aligns_package_and_selector() {
    let packet = packet();
    for row in &packet.rows {
        assert!(
            row.scope_identity_aligned(),
            "row {} package manifest scope disagrees with the selector",
            row.row_id
        );
        assert_eq!(
            row.requested.manifest_scope,
            row.requested_scope.scope_class
        );
    }
}

#[test]
fn applied_row_preserves_durable_identity() {
    let packet = packet();
    let applied = packet
        .row("msr:member:cargo:applied-continuity")
        .expect("applied row");
    assert!(applied.is_applied());
    assert!(applied.continuity_preserved());
    let after = applied.post_apply_scope.as_ref().expect("post apply");
    assert!(applied.resolved_scope.same_identity(after));
    assert_eq!(applied.resolved_scope.manifest_id, after.manifest_id);
    assert_eq!(
        applied.resolved_scope.continuity_token,
        after.continuity_token
    );
}

#[test]
fn standalone_and_path_targets_use_the_same_vocabulary() {
    let packet = packet();
    let standalone = packet
        .row("msr:standalone:other:select")
        .expect("standalone row");
    assert_eq!(
        standalone.requested_scope.role,
        ManifestRole::StandaloneManifest
    );
    assert!(!standalone.targets_root() && !standalone.targets_member());
    assert!(standalone.requested_scope.parent_manifest_id.is_none());
    assert!(standalone.can_apply());

    let path = packet.row("msr:path:pip:vendored").expect("path row");
    assert_eq!(path.requested_scope.role, ManifestRole::PathOrVcsTarget);
    // A path/VCS source discloses a local-cache source, not a registry.
    assert_eq!(
        path.source_cue.source_class,
        RegistrySourceAuthority::LocalCache
    );
    assert!(path.source_cue.mirror_owner.is_none());
    assert!(path.can_apply());
}

#[test]
fn no_source_cue_collapses_into_a_generic_message() {
    let packet = packet();
    assert!(packet.no_generic_collapse());
    for row in &packet.rows {
        assert!(
            row.source_cue.message_class().is_specific(),
            "row {} renders a generic source message",
            row.row_id
        );
    }
}

#[test]
fn the_same_row_object_feeds_every_surface() {
    let packet = packet();
    let row = packet.row("msr:member:cargo:shared-lockfile").expect("row");
    let view = row.view();
    let export = row.export_row();
    assert_eq!(view.row_id, export.row_id);
    assert_eq!(view.scope_diff.fidelity, export.fidelity);
    assert_eq!(view.source.source_class, export.source_class);
    assert_eq!(view.can_apply, export.can_apply);
}

#[test]
fn surface_projections_pin_write_authority_and_gate_apply() {
    let packet = packet();
    let row = packet
        .row("msr:member:node:add-dep:exact")
        .expect("appliable row");
    let desktop = row.surface_projection(PackageSurface::DesktopPackageWorkspace);
    assert!(desktop.can_apply_here);
    assert!(!desktop.redacted);
    let ai = row.surface_projection(PackageSurface::AiContext);
    // An inspect-only surface can never apply, even an appliable row.
    assert!(!ai.can_apply_here);
    let support = row.surface_projection(PackageSurface::SupportExport);
    assert!(!support.can_apply_here);
    assert!(support.redacted);
    assert_eq!(desktop.view, ai.view);

    // A blocked row is never appliable, even from a mutating surface.
    let blocked = packet
        .row("msr:member:pip:would-broaden:blocked")
        .expect("blocked row");
    assert!(
        !blocked
            .surface_projection(PackageSurface::DesktopPackageWorkspace)
            .can_apply_here
    );
}

#[test]
fn export_projection_is_redaction_safe() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.rows.len(), packet.rows.len());
    assert!(projection.all_consistent);
    assert!(projection.no_appliable_silent_broadening);
    assert!(projection.requested_resolved_separate);
    assert!(projection.no_generic_collapse);
    assert!(projection.all_bind_matrix);
    for row in &projection.rows {
        assert!(!row.redacted_source_label.contains("://"));
        if let Some(owner) = &row.mirror_owner {
            assert!(!owner.contains("://"));
        }
    }
}

#[test]
fn no_field_leaks_a_raw_url() {
    let packet = packet();
    for row in &packet.rows {
        assert!(!row.requested_scope.redacted_manifest_path.contains("://"));
        assert!(!row.resolved_scope.redacted_manifest_path.contains("://"));
        assert!(!row.requested.requested_ref.contains("://"));
        assert!(!row.source_cue.redacted_source_label.contains("://"));
        if let Some(resolved) = &row.resolved {
            assert!(!resolved.resolved_ref.contains("://"));
        }
    }
}

#[test]
fn every_manifest_role_is_represented() {
    let packet = packet();
    let present: BTreeSet<ManifestRole> =
        packet.rows.iter().map(|r| r.requested_scope.role).collect();
    for role in ManifestRole::ALL {
        assert!(
            present.contains(&role),
            "no row exercises manifest role {}",
            role.as_str()
        );
    }
}

#[test]
fn role_does_not_permit_a_disallowed_scope() {
    // A member may not anchor a whole-workspace operation.
    assert!(!ManifestRole::WorkspaceMember.permits_scope(ManifestScopeClass::WholeWorkspace));
    assert!(ManifestRole::WorkspaceMember.permits_scope(ManifestScopeClass::WorkspaceMember));
    // Only a root may.
    assert!(ManifestRole::WorkspaceRoot.permits_scope(ManifestScopeClass::WholeWorkspace));
    // A standalone manifest is always a selected manifest.
    assert!(ManifestRole::StandaloneManifest.permits_scope(ManifestScopeClass::SelectedManifest));
    assert!(!ManifestRole::StandaloneManifest.permits_scope(ManifestScopeClass::WorkspaceMember));
}

#[test]
fn validate_flags_silent_broadening_marked_appliable() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|r| r.row_id == "msr:member:node:add-dep:exact")
        .expect("member row");
    // Force a contradictory row: an unconfirmed-broadening fidelity that is
    // nonetheless marked confirmed and broadened, so it would slip past the apply
    // gate. The contract must catch the silent broadening before it applies.
    row.fidelity = ScopeFidelity::UnconfirmedBroadening;
    row.broadening_confirmed = true;
    row.affected_manifest_ids
        .push("manifest:node:workspace-root".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ManifestScopeReviewViolation::SilentBroadeningAppliable { .. }
    )));
}

#[test]
fn validate_flags_fidelity_mismatch() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|r| r.row_id == "msr:member:node:add-dep:exact")
        .expect("member row");
    // An exact row that claims it broadens disagrees with its affected set.
    row.fidelity = ScopeFidelity::UnconfirmedBroadening;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, ManifestScopeReviewViolation::FidelityMismatch { .. })));
}

#[test]
fn validate_flags_scope_identity_mismatch() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|r| r.row_id == "msr:member:node:add-dep:exact")
        .expect("member row");
    row.requested.manifest_scope = ManifestScopeClass::SelectedManifest;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ManifestScopeReviewViolation::ScopeIdentityMismatch { .. }
    )));
}

#[test]
fn validate_flags_selector_inconsistent() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|r| r.row_id == "msr:member:node:add-dep:exact")
        .expect("member row");
    // A member without a parent is inconsistent.
    row.requested_scope.parent_manifest_id = None;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, ManifestScopeReviewViolation::SelectorInconsistent { .. })));
}

#[test]
fn validate_flags_source_cue_inconsistent() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|r| r.row_id == "msr:root:cargo:workspace-bump:exact")
        .expect("public row");
    // A public registry must not name a mirror owner.
    row.source_cue.mirror_owner = Some("unexpected-owner".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ManifestScopeReviewViolation::SourceCueInconsistent { .. }
    )));
}

#[test]
fn validate_flags_continuity_broken() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|r| r.row_id == "msr:member:cargo:applied-continuity")
        .expect("applied row");
    row.post_apply_scope
        .as_mut()
        .expect("post apply")
        .continuity_token = "ct:changed".to_owned();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, ManifestScopeReviewViolation::ContinuityBroken { .. })));
}

#[test]
fn validate_flags_raw_url_leak() {
    let mut packet = packet();
    packet.rows[0].source_cue.redacted_source_label =
        "https://secret.example.com/registry".to_owned();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, ManifestScopeReviewViolation::RawUrlLeak { .. })));
}

#[test]
fn validate_flags_matrix_binding_mismatch() {
    let mut packet = packet();
    packet.references_matrix_id = "some-other-matrix:v9".to_owned();
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ManifestScopeReviewViolation::MatrixBindingMismatch { .. }
    )));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_rows = packet.summary.total_rows.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&ManifestScopeReviewViolation::SummaryMismatch));
}

#[test]
fn validate_flags_duplicate_row_id() {
    let mut packet = packet();
    let clone = packet.rows[0].clone();
    packet.rows.push(clone);
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, ManifestScopeReviewViolation::DuplicateRowId { .. })));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(ManifestRole::WorkspaceRoot.as_str(), "workspace_root");
    assert_eq!(ManifestRole::WorkspaceMember.as_str(), "workspace_member");
    assert_eq!(
        ScopeFidelity::DisclosedSharedLockfile.as_str(),
        "disclosed_shared_lockfile"
    );
    assert_eq!(
        ScopeFidelity::UnconfirmedBroadening.as_str(),
        "unconfirmed_broadening"
    );
    assert_eq!(
        SourceFreshness::OfflineSnapshot.as_str(),
        "offline_snapshot"
    );
    assert_eq!(RevocationState::Revoked.as_str(), "revoked");
}

#[test]
fn every_vocabulary_round_trips_through_serde() {
    fn round_trip<T>(all: &[T])
    where
        T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug,
    {
        for value in all {
            let json = serde_json::to_string(value).expect("serialize");
            let back: T = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(&back, value);
        }
    }
    round_trip(&ManifestRole::ALL);
    round_trip(&ScopeFidelity::ALL);
    round_trip(&SourceFreshness::ALL);
    round_trip(&RevocationState::ALL);
}

/// Manifest-scope fixtures, embedded so they validate without a runtime walk.
const FIXTURE_MEMBER_EXACT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/dependencies/m5/manifest-scope/member_exact_private_registry.json"
));
const FIXTURE_SHARED_LOCKFILE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/dependencies/m5/manifest-scope/member_disclosed_shared_lockfile.json"
));
const FIXTURE_BLOCKED_BROADENING: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/dependencies/m5/manifest-scope/member_unconfirmed_broadening_blocked.json"
));
const FIXTURE_REVOKED_MIRROR: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/dependencies/m5/manifest-scope/revoked_mirror_trust_blocked.json"
));

#[test]
fn fixtures_parse_and_validate() {
    for (name, json) in [
        ("member_exact_private_registry", FIXTURE_MEMBER_EXACT),
        ("member_disclosed_shared_lockfile", FIXTURE_SHARED_LOCKFILE),
        (
            "member_unconfirmed_broadening_blocked",
            FIXTURE_BLOCKED_BROADENING,
        ),
        ("revoked_mirror_trust_blocked", FIXTURE_REVOKED_MIRROR),
    ] {
        let packet: ManifestScopeReview =
            serde_json::from_str(json).unwrap_or_else(|e| panic!("{name} parses: {e}"));
        assert_eq!(packet.validate(), Vec::new(), "{name} validates");
        assert!(packet.all_bind_matrix(), "{name} binds the matrix");
    }
}

#[test]
fn fixtures_cover_the_scope_and_trust_guards() {
    let blocked: ManifestScopeReview =
        serde_json::from_str(FIXTURE_BLOCKED_BROADENING).expect("blocked fixture");
    assert!(blocked.rows[0].is_silent_broadening());
    assert!(!blocked.rows[0].can_apply());

    let revoked: ManifestScopeReview =
        serde_json::from_str(FIXTURE_REVOKED_MIRROR).expect("revoked fixture");
    assert!(revoked.rows[0].source_cue.trust_blocked());
    assert!(!revoked.rows[0].can_apply());

    let shared: ManifestScopeReview =
        serde_json::from_str(FIXTURE_SHARED_LOCKFILE).expect("shared fixture");
    assert!(shared.rows[0].broadens_beyond_target());
    assert!(shared.rows[0].can_apply());
}
