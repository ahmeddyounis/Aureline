use super::*;

fn local_only_route() -> RouteObject {
    RouteObject {
        record_kind: ROUTE_OBJECT_RECORD_KIND.to_owned(),
        route_object_schema_version: ROUTE_OBJECT_SCHEMA_VERSION,
        route_id: "route:test:local:01".to_owned(),
        captured_at: "2026-05-18T12:00:00Z".to_owned(),
        updated_at: "2026-05-18T12:00:00Z".to_owned(),
        route_kind: RouteKind::LocalPortForward,
        lifecycle_state: LifecycleState::Active,
        source: SourceBlock {
            service_ref: "svc:test:api".to_owned(),
            process_ref: Some("proc:test:api:01".to_owned()),
            service_label: "Local API".to_owned(),
            protocol_class: ProtocolClass::Http,
        },
        host_identity: HostIdentityBlock {
            host_class: HostClass::LocalHost,
            host_identity_ref: "host:test:dev".to_owned(),
            workspace_identity_ref: "wks:test".to_owned(),
            environment_identity_ref: "env:test:dev".to_owned(),
            target_identity_witness_ref: "witness:test:01".to_owned(),
        },
        endpoint_handles: EndpointHandlesBlock {
            target_port_handle_ref: "port:test:01".to_owned(),
            local_bind_handle_ref: Some("bind:test:01".to_owned()),
            public_route_handle_ref: None,
            target_path_handle_ref: None,
        },
        exposure_label: ExposureLabelClass::LocalOnlyMutation,
        controlled_exposure_label: ControlledExposureLabel::LocalOnly,
        audience: AudienceBlock {
            audience_class: AudienceClass::SelfOnly,
            audience_ref: None,
            audience_summary: "Self only on loopback".to_owned(),
        },
        auth: AuthBlock {
            auth_source_class: AuthSourceClass::NoAuthLoopbackOnly,
            tls_posture_class: TlsPostureClass::NoTlsLoopbackOnly,
            authority_ticket_ref: None,
            session_handle_ref: None,
        },
        viewer_state_class: ViewerStateClass::LiveService,
        data_sensitivity_class: DataSensitivityClass::WorkspacePrivate,
        expiry: ExpiryBlock {
            expires_at: None,
            ttl_seconds: None,
            renewable: false,
        },
        last_access: LastAccessBlock {
            last_access_class: LastAccessClass::NotObserved,
            last_access_at: None,
        },
        copy_share: CopyShareBlock {
            copy_disclosure_class: CopyDisclosureClass::LocalOnlyCopy,
            share_link_handle_ref: None,
            open_action_allowed: true,
            copy_action_allowed: true,
            share_action_allowed: false,
        },
        revocation: RevocationBlock {
            teardown_state: TeardownState::Active,
            revoke_posture_class: RevokePostureClass::UserSelfRevoke,
            revocation_ref: None,
            affected_link_refs: Vec::new(),
            session_impact_summary: "No external sessions; local only.".to_owned(),
            stale_shared_link_state: StaleSharedLinkState::NoSharedLink,
            reopen_class: ReopenClass::ReopenSameIdentity,
            summary: "Local route; user can self-revoke without affecting external links."
                .to_owned(),
        },
        downgrade: DowngradeBlock {
            downgrade_state: DowngradeState::None,
            local_continuation_allowed: true,
            summary: "No downgrade.".to_owned(),
        },
        summary: "Local-only port forward to local API service.".to_owned(),
        review_ref: None,
    }
}

#[test]
fn local_only_route_validates_clean() {
    let route = local_only_route();
    assert!(
        route.validate().is_empty(),
        "local-only route should be clean: {:?}",
        route.validate()
    );
}

#[test]
fn local_only_route_rejects_share_link() {
    let mut route = local_only_route();
    route.copy_share.share_link_handle_ref = Some("share:test:01".to_owned());
    route.copy_share.share_action_allowed = true;
    let findings = route.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "route_object.local_only_share_link"));
    assert!(findings
        .iter()
        .any(|f| f.check_id == "route_object.local_only_share_allowed"));
}

#[test]
fn shareable_route_rejects_loopback_auth() {
    let mut route = local_only_route();
    route.controlled_exposure_label = ControlledExposureLabel::AuthenticatedOrgRoute;
    route.exposure_label = ExposureLabelClass::WorkspaceVisibleMutation;
    route.copy_share.copy_disclosure_class = CopyDisclosureClass::AuthenticatedRouteCopy;
    let findings = route.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "route_object.shareable_loopback_auth"));
}

#[test]
fn public_route_requires_expiry_and_step_up_copy() {
    let mut route = local_only_route();
    route.controlled_exposure_label = ControlledExposureLabel::PublicRoute;
    route.exposure_label = ExposureLabelClass::TunnelExposedPublic;
    route.audience.audience_class = AudienceClass::PublicEphemeralHolders;
    route.auth.auth_source_class = AuthSourceClass::SignedPreviewLink;
    let findings = route.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "route_object.public_route_expiry_required"));
    assert!(findings
        .iter()
        .any(|f| f.check_id == "route_object.public_route_copy_disclosure"));

    route.expiry.expires_at = Some("2026-05-18T13:00:00Z".to_owned());
    route.expiry.ttl_seconds = Some(3600);
    route.copy_share.copy_disclosure_class = CopyDisclosureClass::PublicLinkStepUpRequired;
    route.copy_share.share_action_allowed = true;
    route.copy_share.share_link_handle_ref = Some("share:test:public".to_owned());
    assert!(route.validate().is_empty());
}

#[test]
fn revoked_route_must_declare_revoked_teardown() {
    let mut route = local_only_route();
    route.lifecycle_state = LifecycleState::Revoked;
    let findings = route.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "route_object.revoked_teardown_state"));
    assert!(findings
        .iter()
        .any(|f| f.check_id == "route_object.non_active_teardown_state"));

    route.revocation.teardown_state = TeardownState::Revoked;
    route.revocation.revoke_posture_class = RevokePostureClass::UserSelfRevoke;
    let findings = route.validate();
    assert!(findings.is_empty(), "revoked route clean: {findings:?}");
}

#[test]
fn stale_target_route_requires_inspect_only_downgrade() {
    let mut route = local_only_route();
    route.lifecycle_state = LifecycleState::StaleTarget;
    let findings = route.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "route_object.stale_downgrade_state"));
    assert!(findings
        .iter()
        .any(|f| f.check_id == "route_object.stale_viewer_state"));

    route.viewer_state_class = ViewerStateClass::StaleMirroredState;
    route.downgrade.downgrade_state = DowngradeState::StaleTargetInspectOnly;
    route.revocation.teardown_state = TeardownState::SuspendedNoTraffic;
    let findings = route.validate();
    assert!(findings.is_empty(), "stale route clean: {findings:?}");
}

fn approve_widen_review() -> ExposureReview {
    ExposureReview {
        record_kind: EXPOSURE_REVIEW_RECORD_KIND.to_owned(),
        exposure_review_schema_version: EXPOSURE_REVIEW_SCHEMA_VERSION,
        review_id: "review:test:01".to_owned(),
        route_id: "route:test:local:01".to_owned(),
        captured_at: "2026-05-18T12:05:00Z".to_owned(),
        reviewer_ref: Some("user:test:reviewer".to_owned()),
        review_outcome: ReviewOutcomeClass::ApprovedAsProposed,
        proposed_transition: ProposedTransition {
            from_controlled_exposure_label: ControlledExposureLabel::LocalOnly,
            to_controlled_exposure_label: ControlledExposureLabel::AuthenticatedOrgRoute,
            transition_admitted: true,
            narrowing_applied: false,
            narrowing_summary: None,
        },
        audience: ReviewAudienceBlock {
            audience_class: AudienceClass::OrganizationMembers,
            audience_ref: Some("org:test".to_owned()),
            audience_summary: "Organization members with SSO.".to_owned(),
        },
        data_sensitivity: DataSensitivityBlock {
            data_sensitivity_class: DataSensitivityClass::WorkspacePrivate,
            data_sensitivity_summary: "Workspace-private dev data.".to_owned(),
        },
        cookie_session_behavior: CookieSessionBlock {
            cookie_session_class: CookieSessionClass::SessionCookieScopedToRoute,
            summary: "Per-route session cookie; not shared with org browser.".to_owned(),
        },
        idle_timeout: IdleTimeoutBlock {
            idle_timeout_seconds: Some(900),
            summary: "Idle session times out after 15 minutes.".to_owned(),
        },
        reachability: ReachabilityBlock {
            reachability_local_class: ReachabilityLocalClass::LoopbackOnly,
            reachability_public_class: ReachabilityPublicClass::NotPublic,
            summary: "Loopback only; reachable behind org auth only.".to_owned(),
        },
        lingering_local_preview: LingeringLocalPreviewBlock {
            lingering_local_preview_class: LingeringLocalPreviewClass::LocalPreviewRemainsAvailable,
            local_preview_handle_ref: Some("preview:test:01".to_owned()),
            summary: "Local preview remains available alongside the org route.".to_owned(),
        },
        viewer_state_truth: ViewerStateTruthBlock {
            viewer_state_class: ViewerStateClass::LiveService,
            summary: "Viewers see live service state.".to_owned(),
        },
        cross_origin_disclosure: CrossOriginBlock {
            cross_origin_class: CrossOriginClass::EmbeddedWebviewOnly,
            system_browser_required: false,
            summary: "Embedded webview only; no system-browser handoff.".to_owned(),
        },
        summary: "Widen from local_only to authenticated_org_route within workspace.".to_owned(),
    }
}

#[test]
fn approved_org_widen_review_validates_clean() {
    let review = approve_widen_review();
    assert!(
        review.validate().is_empty(),
        "review should be clean: {:?}",
        review.validate()
    );
}

#[test]
fn denied_review_cannot_admit_transition() {
    let mut review = approve_widen_review();
    review.review_outcome = ReviewOutcomeClass::Denied;
    let findings = review.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "exposure_review.denied_but_admitted"));
}

#[test]
fn public_widen_requires_idle_timeout_and_public_class() {
    let mut review = approve_widen_review();
    review.proposed_transition.to_controlled_exposure_label = ControlledExposureLabel::PublicRoute;
    review.idle_timeout.idle_timeout_seconds = None;
    let findings = review.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "exposure_review.public_idle_timeout"));
    assert!(findings
        .iter()
        .any(|f| f.check_id == "exposure_review.public_reachability"));
}

#[test]
fn narrowing_requires_approved_with_narrowing_outcome() {
    let mut review = approve_widen_review();
    review.proposed_transition.narrowing_applied = true;
    review.proposed_transition.narrowing_summary = Some("Narrowed to org members.".to_owned());
    let findings = review.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "exposure_review.narrowing_outcome_mismatch"));

    review.review_outcome = ReviewOutcomeClass::ApprovedWithNarrowing;
    assert!(review.validate().is_empty());
}

#[test]
fn revocation_summary_from_route_matches_row_truth() {
    let mut route = local_only_route();
    route.lifecycle_state = LifecycleState::Revoked;
    route.revocation.teardown_state = TeardownState::Revoked;
    route.revocation.summary = "User revoked the route at 12:30Z.".to_owned();
    route
        .revocation
        .affected_link_refs
        .push("link:test:01".to_owned());
    let summary = RevocationSummary::from_route(&route);
    assert_eq!(summary.route_id, route.route_id);
    assert_eq!(
        summary.controlled_exposure_label,
        route.controlled_exposure_label
    );
    assert_eq!(summary.audience_class, route.audience.audience_class);
    assert_eq!(summary.teardown_state, TeardownState::Revoked);
    assert_eq!(
        summary.affected_link_refs,
        route.revocation.affected_link_refs
    );
}

#[test]
fn record_kind_constants_match_schema_tokens() {
    assert_eq!(ROUTE_OBJECT_RECORD_KIND, "route_object_record");
    assert_eq!(EXPOSURE_REVIEW_RECORD_KIND, "exposure_review_record");
    assert_eq!(ROUTE_OBJECT_SCHEMA_VERSION, 1);
    assert_eq!(EXPOSURE_REVIEW_SCHEMA_VERSION, 1);
}

#[test]
fn controlled_exposure_labels_render_stable_tokens_and_labels() {
    assert_eq!(ControlledExposureLabel::LocalOnly.as_str(), "local_only");
    assert_eq!(ControlledExposureLabel::LocalOnly.label(), "Local only");
    assert_eq!(
        ControlledExposureLabel::SameDeviceLan.as_str(),
        "same_device_lan"
    );
    assert_eq!(
        ControlledExposureLabel::SameDeviceLan.label(),
        "Same device / LAN"
    );
    assert_eq!(
        ControlledExposureLabel::AuthenticatedOrgRoute.as_str(),
        "authenticated_org_route"
    );
    assert_eq!(
        ControlledExposureLabel::AuthenticatedOrgRoute.label(),
        "Authenticated org route"
    );
    assert_eq!(
        ControlledExposureLabel::SignedPreviewLink.as_str(),
        "signed_preview_link"
    );
    assert_eq!(
        ControlledExposureLabel::SignedPreviewLink.label(),
        "Signed preview link"
    );
    assert_eq!(
        ControlledExposureLabel::PublicRoute.as_str(),
        "public_route"
    );
    assert_eq!(ControlledExposureLabel::PublicRoute.label(), "Public route");

    assert!(!ControlledExposureLabel::LocalOnly.is_shareable());
    assert!(!ControlledExposureLabel::SameDeviceLan.is_shareable());
    assert!(ControlledExposureLabel::AuthenticatedOrgRoute.is_shareable());
    assert!(ControlledExposureLabel::PublicRoute.is_publicly_reachable());
    assert!(!ControlledExposureLabel::AuthenticatedOrgRoute.is_publicly_reachable());
}
