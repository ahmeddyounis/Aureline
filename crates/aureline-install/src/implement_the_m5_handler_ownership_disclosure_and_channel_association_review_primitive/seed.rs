// Canonical seed for the M5 handler-ownership primitive. Included from `mod.rs` so the seeded
// builder, its worked cases, the fixture generator, and the on-disk support export all stay
// byte-aligned.

/// Convenience constructor for one channel-association input. The change is never a silent
/// takeover, it is reviewed, previewable, and reversible, and the current owner is disclosed;
/// the proposed-owner ref is derived from the channel ref and proposed class.
#[allow(clippy::too_many_arguments)]
fn channel(
    channel_ref: &str,
    channel_class: M5HandlerChannelClass,
    current_owner_ref: &str,
    current_owner_class: M5HandlerOwnerClass,
    proposed_owner_class: M5HandlerOwnerClass,
    change_state: M5HandlerChangeState,
    impact_class: M5HandlerImpactClass,
) -> M5ChannelAssociationInput {
    let proposed_owner_ref = if change_state.is_change() {
        format!("owner:{}:proposed", proposed_owner_class.as_str())
    } else {
        current_owner_ref.to_owned()
    };
    M5ChannelAssociationInput {
        channel_ref: channel_ref.to_owned(),
        channel_class,
        current_owner_ref: current_owner_ref.to_owned(),
        current_owner_class,
        proposed_owner_ref,
        proposed_owner_class,
        change_state,
        impact_class,
        last_writer_wins_capture: false,
        reviewed_before_apply: true,
        previewable: true,
        reversible: true,
        discloses_current_owner: true,
    }
}

/// An About desktop-integration section explaining a side-by-side stable install that holds
/// primary precedence for its file and protocol handlers.
fn about_integration_input() -> M5HandlerOwnershipInput {
    M5HandlerOwnershipInput {
        ownership_id: "ownership:about-integration:0001".to_owned(),
        surface_label: "About desktop-integration section for a side-by-side stable install"
            .to_owned(),
        deployment_mode: M5DeploymentMode::Desktop,
        install_identity_ref: "install:desktop:stable".to_owned(),
        owner_class: M5HandlerOwnerClass::PrimaryStableInstall,
        precedence_state: M5HandlerPrecedenceState::PrimaryAmongInstalls,
        ownership_reason:
            "The stable build registered the .aur file association and the aureline protocol handler first and holds primary precedence; the side-by-side beta is registered at lower precedence"
                .to_owned(),
        rollback_identity_ref: "rollback:handler:about-integration".to_owned(),
        inspectable_without_installer: true,
        discloses_current_owner: true,
        channels: vec![
            channel(
                "channel:file-association:aur",
                M5HandlerChannelClass::FileAssociation,
                "owner:stable-build",
                M5HandlerOwnerClass::PrimaryStableInstall,
                M5HandlerOwnerClass::PrimaryStableInstall,
                M5HandlerChangeState::NoChange,
                M5HandlerImpactClass::OpensInThisBuild,
            ),
            channel(
                "channel:protocol-handler:aureline",
                M5HandlerChannelClass::ProtocolHandler,
                "owner:stable-build",
                M5HandlerOwnerClass::PrimaryStableInstall,
                M5HandlerOwnerClass::PrimaryStableInstall,
                M5HandlerChangeState::NoChange,
                M5HandlerImpactClass::RoutesToThisBuild,
            ),
        ],
        degraded: None,
    }
}

/// A diagnostics handler-ownership pane surfacing a contested file association across two
/// installs, with the change awaiting a review decision.
fn diagnostics_handlers_input() -> M5HandlerOwnershipInput {
    M5HandlerOwnershipInput {
        ownership_id: "ownership:diagnostics-handlers:0002".to_owned(),
        surface_label: "Diagnostics handler-ownership pane for a contested file association"
            .to_owned(),
        deployment_mode: M5DeploymentMode::Desktop,
        install_identity_ref: "install:desktop:beta".to_owned(),
        owner_class: M5HandlerOwnerClass::SideBySideBetaInstall,
        precedence_state: M5HandlerPrecedenceState::SharedContested,
        ownership_reason:
            "Two installs contest the .aur file association; diagnostics names the current owner, the contesting stable build, and the precedence order so the user can decide"
                .to_owned(),
        rollback_identity_ref: "rollback:handler:diagnostics".to_owned(),
        inspectable_without_installer: true,
        discloses_current_owner: true,
        channels: vec![
            channel(
                "channel:file-association:aur",
                M5HandlerChannelClass::FileAssociation,
                "owner:beta-build",
                M5HandlerOwnerClass::SideBySideBetaInstall,
                M5HandlerOwnerClass::PrimaryStableInstall,
                M5HandlerChangeState::ContestedAwaitingReview,
                M5HandlerImpactClass::RequiresUserChoice,
            ),
            channel(
                "channel:deep-link:aureline-scheme",
                M5HandlerChannelClass::DeepLink,
                "owner:beta-build",
                M5HandlerOwnerClass::SideBySideBetaInstall,
                M5HandlerOwnerClass::SideBySideBetaInstall,
                M5HandlerChangeState::NoChange,
                M5HandlerImpactClass::RoutesToThisBuild,
            ),
        ],
        degraded: Some(DegradedState {
            trigger: M5DeploymentDowngradeTrigger::HandlerOwnershipContested,
            degraded_label:
                "The .aur file association is contested between the stable and beta installs; the pane names both installs and the precedence order, keeps keep / reassign / cancel actions, and leaves the choice to the user rather than capturing the handler"
                    .to_owned(),
        }),
    }
}

/// An install / side-by-side review previewing a portable install reassigning the protocol
/// handler from the desktop install.
fn install_review_input() -> M5HandlerOwnershipInput {
    M5HandlerOwnershipInput {
        ownership_id: "ownership:install-review:0003".to_owned(),
        surface_label: "Install review previewing a portable install taking the protocol handler"
            .to_owned(),
        deployment_mode: M5DeploymentMode::Portable,
        install_identity_ref: "install:portable:drive".to_owned(),
        owner_class: M5HandlerOwnerClass::PortableInstall,
        precedence_state: M5HandlerPrecedenceState::PrimaryAmongInstalls,
        ownership_reason:
            "The portable install is reassigning the aureline protocol handler from the desktop install; the change previews the new owner and can be reverted before it applies"
                .to_owned(),
        rollback_identity_ref: "rollback:handler:install-review".to_owned(),
        inspectable_without_installer: true,
        discloses_current_owner: true,
        channels: vec![
            channel(
                "channel:protocol-handler:aureline",
                M5HandlerChannelClass::ProtocolHandler,
                "owner:stable-build",
                M5HandlerOwnerClass::PrimaryStableInstall,
                M5HandlerOwnerClass::PortableInstall,
                M5HandlerChangeState::ReassignToThisInstall,
                M5HandlerImpactClass::RoutesToThisBuild,
            ),
            channel(
                "channel:file-association:aur",
                M5HandlerChannelClass::FileAssociation,
                "owner:portable-build",
                M5HandlerOwnerClass::PortableInstall,
                M5HandlerOwnerClass::PortableInstall,
                M5HandlerChangeState::NoChange,
                M5HandlerImpactClass::OpensInThisBuild,
            ),
        ],
        degraded: None,
    }
}

/// A support / export replay reconstructing which fleet build owned the handlers during a
/// desktop-integration incident, with every recovery path carrying the rollback identity.
fn support_replay_input() -> M5HandlerOwnershipInput {
    M5HandlerOwnershipInput {
        ownership_id: "ownership:support-replay:0004".to_owned(),
        surface_label: "Support replay reconstructing fleet handler ownership after an incident"
            .to_owned(),
        deployment_mode: M5DeploymentMode::Managed,
        install_identity_ref: "install:managed:fleet".to_owned(),
        owner_class: M5HandlerOwnerClass::ManagedFleetInstall,
        precedence_state: M5HandlerPrecedenceState::SoleOwner,
        ownership_reason:
            "Support replay reconstructs that the managed fleet build was the sole owner of the handlers during the desktop-integration incident, with the rollback identity for each recovery path"
                .to_owned(),
        rollback_identity_ref: "rollback:handler:support-replay".to_owned(),
        inspectable_without_installer: true,
        discloses_current_owner: true,
        channels: vec![
            channel(
                "channel:system-open:aur",
                M5HandlerChannelClass::SystemOpen,
                "owner:fleet-build",
                M5HandlerOwnerClass::ManagedFleetInstall,
                M5HandlerOwnerClass::ManagedFleetInstall,
                M5HandlerChangeState::NoChange,
                M5HandlerImpactClass::OpensInThisBuild,
            ),
            channel(
                "channel:deep-link:aureline-scheme",
                M5HandlerChannelClass::DeepLink,
                "owner:fleet-build",
                M5HandlerOwnerClass::ManagedFleetInstall,
                M5HandlerOwnerClass::ManagedFleetInstall,
                M5HandlerChangeState::NoChange,
                M5HandlerImpactClass::RoutesToThisBuild,
            ),
            channel(
                "channel:recent-item:workspace",
                M5HandlerChannelClass::RecentItemReopen,
                "owner:fleet-build",
                M5HandlerOwnerClass::ManagedFleetInstall,
                M5HandlerOwnerClass::ManagedFleetInstall,
                M5HandlerChangeState::NoChange,
                M5HandlerImpactClass::ResolvesInPlace,
            ),
            channel(
                "channel:notification-action:update",
                M5HandlerChannelClass::NotificationAction,
                "owner:fleet-build",
                M5HandlerOwnerClass::ManagedFleetInstall,
                M5HandlerOwnerClass::ManagedFleetInstall,
                M5HandlerChangeState::NoChange,
                M5HandlerImpactClass::RoutesToThisBuild,
            ),
        ],
        degraded: None,
    }
}

/// A notification / activity center framing a sole-owner desktop install whose notification and
/// deep-link recovery routes activate this build.
fn notification_center_input() -> M5HandlerOwnershipInput {
    M5HandlerOwnershipInput {
        ownership_id: "ownership:notification-center:0005".to_owned(),
        surface_label: "Notification center for a sole-owner desktop install".to_owned(),
        deployment_mode: M5DeploymentMode::Desktop,
        install_identity_ref: "install:desktop:stable".to_owned(),
        owner_class: M5HandlerOwnerClass::PrimaryStableInstall,
        precedence_state: M5HandlerPrecedenceState::SoleOwner,
        ownership_reason:
            "The stable desktop build is the sole owner of notification actions and deep links; recovery routes activate this build and carry the rollback identity"
                .to_owned(),
        rollback_identity_ref: "rollback:handler:notification-center".to_owned(),
        inspectable_without_installer: true,
        discloses_current_owner: true,
        channels: vec![
            channel(
                "channel:notification-action:advisory",
                M5HandlerChannelClass::NotificationAction,
                "owner:stable-build",
                M5HandlerOwnerClass::PrimaryStableInstall,
                M5HandlerOwnerClass::PrimaryStableInstall,
                M5HandlerChangeState::NoChange,
                M5HandlerImpactClass::RoutesToThisBuild,
            ),
            channel(
                "channel:deep-link:aureline-scheme",
                M5HandlerChannelClass::DeepLink,
                "owner:stable-build",
                M5HandlerOwnerClass::PrimaryStableInstall,
                M5HandlerOwnerClass::PrimaryStableInstall,
                M5HandlerChangeState::NoChange,
                M5HandlerImpactClass::RoutesToThisBuild,
            ),
        ],
        degraded: None,
    }
}

/// A docs handler reference framing a single-install desktop that solely owns the .aur file
/// association.
fn docs_reference_input() -> M5HandlerOwnershipInput {
    M5HandlerOwnershipInput {
        ownership_id: "ownership:docs-reference:0006".to_owned(),
        surface_label: "Docs handler reference for a single-install desktop".to_owned(),
        deployment_mode: M5DeploymentMode::Desktop,
        install_identity_ref: "install:desktop:stable".to_owned(),
        owner_class: M5HandlerOwnerClass::PrimaryStableInstall,
        precedence_state: M5HandlerPrecedenceState::SoleOwner,
        ownership_reason:
            "The docs reference frames a single-install desktop that solely owns the .aur file association, with no contesting build present"
                .to_owned(),
        rollback_identity_ref: "rollback:handler:docs-reference".to_owned(),
        inspectable_without_installer: true,
        discloses_current_owner: true,
        channels: vec![channel(
            "channel:file-association:aur",
            M5HandlerChannelClass::FileAssociation,
            "owner:stable-build",
            M5HandlerOwnerClass::PrimaryStableInstall,
            M5HandlerOwnerClass::PrimaryStableInstall,
            M5HandlerChangeState::NoChange,
            M5HandlerImpactClass::OpensInThisBuild,
        )],
        degraded: None,
    }
}

fn case(input: M5HandlerOwnershipInput) -> M5HandlerOwnershipCase {
    M5HandlerOwnershipCase::resolved(input)
}

fn seeded_surface_rows() -> Vec<M5HandlerOwnershipSurfaceRow> {
    let base_source_refs = vec![
        M5_HANDLER_OWNERSHIP_SCHEMA_REF.to_owned(),
        M5_HANDLER_OWNERSHIP_COMPONENT_MATRIX_REF.to_owned(),
    ];
    let all_export_fields = M5HandlerOwnershipExportField::ALL.to_vec();

    vec![
        M5HandlerOwnershipSurfaceRow {
            surface_family: M5HandlerSurfaceFamily::AboutIntegration,
            owner_role: "Desktop-integration guild".to_owned(),
            scope_summary: "About desktop-integration section naming the primary owner and precedence for the file and protocol handlers"
                .to_owned(),
            channel_classes: vec![
                M5HandlerChannelClass::FileAssociation,
                M5HandlerChannelClass::ProtocolHandler,
            ],
            precedence_states: vec![M5HandlerPrecedenceState::PrimaryAmongInstalls],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![M5DeploymentDowngradeTrigger::HandlerOwnershipContested],
            consumer_surfaces: vec!["about_page".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_cases: vec![case(about_integration_input())],
            shows_silent_takeover: false,
            hides_current_owner: false,
            forces_manual_installer_inspection: false,
            drops_rollback_identity: false,
        },
        M5HandlerOwnershipSurfaceRow {
            surface_family: M5HandlerSurfaceFamily::DiagnosticsHandlers,
            owner_role: "Diagnostics guild".to_owned(),
            scope_summary: "Diagnostics handler-ownership pane naming both installs contesting a file association and leaving the choice to the user"
                .to_owned(),
            channel_classes: vec![
                M5HandlerChannelClass::FileAssociation,
                M5HandlerChannelClass::DeepLink,
            ],
            precedence_states: vec![M5HandlerPrecedenceState::SharedContested],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5DeploymentDowngradeTrigger::HandlerOwnershipContested,
                M5DeploymentDowngradeTrigger::ProvenanceIncomplete,
            ],
            consumer_surfaces: vec!["diagnostics".to_owned(), "support_export".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_cases: vec![case(diagnostics_handlers_input())],
            shows_silent_takeover: false,
            hides_current_owner: false,
            forces_manual_installer_inspection: false,
            drops_rollback_identity: false,
        },
        M5HandlerOwnershipSurfaceRow {
            surface_family: M5HandlerSurfaceFamily::InstallReview,
            owner_role: "Install / side-by-side guild".to_owned(),
            scope_summary: "Install review previewing a portable install reassigning the protocol handler with a reversible change"
                .to_owned(),
            channel_classes: vec![
                M5HandlerChannelClass::ProtocolHandler,
                M5HandlerChannelClass::FileAssociation,
            ],
            precedence_states: vec![M5HandlerPrecedenceState::PrimaryAmongInstalls],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![M5DeploymentDowngradeTrigger::HandlerOwnershipContested],
            consumer_surfaces: vec!["install_review".to_owned(), "about_page".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_cases: vec![case(install_review_input())],
            shows_silent_takeover: false,
            hides_current_owner: false,
            forces_manual_installer_inspection: false,
            drops_rollback_identity: false,
        },
        M5HandlerOwnershipSurfaceRow {
            surface_family: M5HandlerSurfaceFamily::SupportExportReplay,
            owner_role: "Support / export guild".to_owned(),
            scope_summary: "Support replay reconstructing sole fleet handler ownership with every recovery path carrying the rollback identity"
                .to_owned(),
            channel_classes: vec![
                M5HandlerChannelClass::SystemOpen,
                M5HandlerChannelClass::DeepLink,
                M5HandlerChannelClass::RecentItemReopen,
                M5HandlerChannelClass::NotificationAction,
            ],
            precedence_states: vec![M5HandlerPrecedenceState::SoleOwner],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5DeploymentDowngradeTrigger::HandlerOwnershipContested,
                M5DeploymentDowngradeTrigger::StateRootUnavailable,
            ],
            consumer_surfaces: vec!["support_export".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_cases: vec![case(support_replay_input())],
            shows_silent_takeover: false,
            hides_current_owner: false,
            forces_manual_installer_inspection: false,
            drops_rollback_identity: false,
        },
        M5HandlerOwnershipSurfaceRow {
            surface_family: M5HandlerSurfaceFamily::NotificationCenter,
            owner_role: "Notification / activity guild".to_owned(),
            scope_summary: "Notification center framing a sole-owner desktop install whose notification and deep-link recovery routes activate this build"
                .to_owned(),
            channel_classes: vec![
                M5HandlerChannelClass::NotificationAction,
                M5HandlerChannelClass::DeepLink,
            ],
            precedence_states: vec![M5HandlerPrecedenceState::SoleOwner],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![M5DeploymentDowngradeTrigger::HandlerOwnershipContested],
            consumer_surfaces: vec!["notification_center".to_owned(), "about_page".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_cases: vec![case(notification_center_input())],
            shows_silent_takeover: false,
            hides_current_owner: false,
            forces_manual_installer_inspection: false,
            drops_rollback_identity: false,
        },
        M5HandlerOwnershipSurfaceRow {
            surface_family: M5HandlerSurfaceFamily::DocsHandlerReference,
            owner_role: "Docs / help guild".to_owned(),
            scope_summary: "Docs handler reference framing a single-install desktop that solely owns the file association"
                .to_owned(),
            channel_classes: vec![M5HandlerChannelClass::FileAssociation],
            precedence_states: vec![M5HandlerPrecedenceState::SoleOwner],
            export_fields: all_export_fields,
            downgrade_triggers: vec![M5DeploymentDowngradeTrigger::HandlerOwnershipContested],
            consumer_surfaces: vec!["docs_reference".to_owned(), "about_page".to_owned()],
            source_contract_refs: base_source_refs,
            example_cases: vec![case(docs_reference_input())],
            shows_silent_takeover: false,
            hides_current_owner: false,
            forces_manual_installer_inspection: false,
            drops_rollback_identity: false,
        },
    ]
}

fn seeded_governance_review() -> M5HandlerOwnershipGovernanceReview {
    M5HandlerOwnershipGovernanceReview {
        one_primitive_carries_all_surfaces: true,
        ownership_identity_preserved_across_surfaces: true,
        current_owner_and_precedence_always_disclosed: true,
        changes_previewable_and_reversible_never_silent: true,
        recovery_aligned_with_owner_and_rollback_identity: true,
        support_export_reconstructs_ownership: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn seeded_consumer_projection() -> M5HandlerOwnershipConsumerProjection {
    M5HandlerOwnershipConsumerProjection {
        integration_surfaces_consume_shared_primitive: true,
        resolver_reads_single_model: true,
        disclosure_card_reads_single_ownership_source: true,
        support_export_reads_single_source: true,
    }
}

fn seeded_release_posture() -> M5HandlerOwnershipReleasePosture {
    M5HandlerOwnershipReleasePosture {
        release_packet_ref: M5_HANDLER_OWNERSHIP_ARTIFACT_REF.to_owned(),
        deployment_audit_ref: M5_HANDLER_OWNERSHIP_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

/// Builds the canonical, checked-in M5 handler-ownership primitive packet. This is the one
/// source of truth shared by the tests, the fixture generator, and the on-disk support export so
/// all three stay byte-aligned.
pub fn seeded_m5_handler_ownership_packet() -> M5HandlerOwnershipPrimitivePacket {
    M5HandlerOwnershipPrimitivePacket::new(M5HandlerOwnershipPrimitivePacketInput {
        packet_id: "m5-handler-ownership-primitive:stable:0001".to_owned(),
        matrix_label:
            "M5 Handler-Ownership Primitive: Ownership / Precedence Disclosure Card, Channel-Association Review Rows, and Recovery Alignment"
                .to_owned(),
        surface_rows: seeded_surface_rows(),
        vocabulary_set: M5HandlerOwnershipVocabularySet::canonical(),
        governance_review: seeded_governance_review(),
        consumer_projection: seeded_consumer_projection(),
        release_posture: seeded_release_posture(),
        source_contract_refs: vec![
            M5_HANDLER_OWNERSHIP_SCHEMA_REF.to_owned(),
            M5_HANDLER_OWNERSHIP_DOC_REF.to_owned(),
            M5_HANDLER_OWNERSHIP_COMPONENT_MATRIX_REF.to_owned(),
            M5_HANDLER_OWNERSHIP_ARTIFACT_REF.to_owned(),
        ],
        redaction_class_token: "install_component_boundary_v1".to_owned(),
        minted_at: "2026-07-04T00:00:00Z".to_owned(),
    })
}
