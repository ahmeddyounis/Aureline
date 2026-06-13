use super::*;

fn packet() -> M5ExecutionSurfaceResolutionPacket {
    frozen_stable_m5_execution_surface_resolution_packet()
}

fn row_for(
    resolution: &M5PlatformResolution,
    surface: M5ExecutingSurface,
) -> &M5ResolvedSurfaceRow {
    resolution
        .resolved_rows
        .iter()
        .find(|row| row.surface == surface)
        .expect("surface present in platform resolution")
}

fn platform(
    packet: &M5ExecutionSurfaceResolutionPacket,
    platform: M5ExecutionPlatform,
) -> &M5PlatformResolution {
    packet
        .platform_resolutions
        .iter()
        .find(|resolution| resolution.platform == platform)
        .expect("platform present in packet")
}

#[test]
fn frozen_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn frozen_packet_covers_every_platform_and_surface() {
    let packet = packet();
    for platform in M5ExecutionPlatform::ALL {
        let resolution = packet
            .platform_resolutions
            .iter()
            .find(|resolution| resolution.platform == platform)
            .expect("platform resolved");
        let surfaces: std::collections::BTreeSet<_> = resolution
            .resolved_rows
            .iter()
            .map(|row| row.surface)
            .collect();
        for surface in M5ExecutingSurface::ALL {
            assert!(
                surfaces.contains(&surface),
                "platform {} missing surface {}",
                platform.as_str(),
                surface.as_str()
            );
        }
    }
}

#[test]
fn every_launch_path_binds_to_a_governing_surface() {
    let packet = packet();
    let present: std::collections::BTreeSet<_> = packet
        .launch_path_bindings
        .iter()
        .map(|binding| binding.launch_path)
        .collect();
    for launch_path in M5ExecutionLaunchPath::ALL {
        assert!(
            present.contains(&launch_path),
            "missing launch path binding for {}",
            launch_path.as_str()
        );
    }
}

#[test]
fn descriptor_exists_for_every_profile() {
    let packet = packet();
    for profile in ordered_profiles() {
        assert!(
            packet
                .profile_descriptors
                .iter()
                .any(|descriptor| descriptor.profile == profile),
            "missing descriptor for {}",
            profile.profile_id()
        );
    }
}

#[test]
fn linux_supports_container_preview_surface() {
    let packet = packet();
    let linux = platform(&packet, M5ExecutionPlatform::LinuxDesktop);
    let preview = row_for(linux, M5ExecutingSurface::PreviewServer);
    assert_eq!(
        preview.resolution_status,
        M5ProfileResolutionStatus::Supported
    );
    assert_eq!(
        preview.effective_profile,
        Some(M5SandboxProfile::ContainerIsolatedLocal)
    );
}

#[test]
fn macos_narrows_container_preview_to_remote() {
    let packet = packet();
    let macos = platform(&packet, M5ExecutionPlatform::MacosDesktop);
    let preview = row_for(macos, M5ExecutingSurface::PreviewServer);
    assert_eq!(
        preview.resolution_status,
        M5ProfileResolutionStatus::NarrowedToStricterProfile
    );
    assert_eq!(
        preview.effective_profile,
        Some(M5SandboxProfile::IsolatedRemoteRuntime),
        "macOS lacks a local container backend; preview must narrow to the stricter remote runtime"
    );
    // Narrowing must never widen isolation.
    assert!(
        preview.effective_profile.unwrap().isolation_rank()
            > preview.default_profile.isolation_rank()
    );
}

#[test]
fn headless_ci_narrows_preview_to_inert() {
    let packet = packet();
    let ci = platform(&packet, M5ExecutionPlatform::HeadlessCi);
    let preview = row_for(ci, M5ExecutingSurface::PreviewServer);
    assert_eq!(
        preview.resolution_status,
        M5ProfileResolutionStatus::NarrowedToStricterProfile
    );
    assert_eq!(
        preview.effective_profile,
        Some(M5SandboxProfile::InertNoExecution),
        "CI has neither container nor remote backends; preview must narrow to inert read-only"
    );
    assert!(
        !preview.stripped_capability_classes.is_empty(),
        "narrowing to inert must strip execution capabilities"
    );
}

#[test]
fn headless_ci_fails_closed_for_remote_mutation() {
    let packet = packet();
    let ci = platform(&packet, M5ExecutionPlatform::HeadlessCi);
    let remote = row_for(ci, M5ExecutingSurface::RemoteMutation);
    assert_eq!(
        remote.resolution_status,
        M5ProfileResolutionStatus::UnsupportedFailClosed
    );
    assert!(remote.effective_profile.is_none());
    assert_eq!(
        remote.effective_qualification,
        M5RuntimeAuthorityQualificationClass::Unavailable
    );
}

#[test]
fn brokered_surfaces_supported_on_every_platform() {
    let packet = packet();
    for platform in M5ExecutionPlatform::ALL {
        let resolution = self::platform(&packet, platform);
        let request = row_for(resolution, M5ExecutingSurface::RequestApiSend);
        assert_eq!(
            request.resolution_status,
            M5ProfileResolutionStatus::Supported,
            "brokered network send must be supported on {}",
            platform.as_str()
        );
    }
}

#[test]
fn no_resolution_widens_isolation() {
    let packet = packet();
    for resolution in &packet.platform_resolutions {
        for row in &resolution.resolved_rows {
            if let Some(effective) = row.effective_profile {
                if row.resolution_status == M5ProfileResolutionStatus::NarrowedToStricterProfile {
                    assert!(
                        effective.isolation_rank() > row.default_profile.isolation_rank(),
                        "{} on {} widened isolation",
                        row.surface.as_str(),
                        resolution.platform.as_str()
                    );
                }
            }
        }
    }
}

#[test]
fn missing_platform_fails_validation() {
    let mut packet = packet();
    packet
        .platform_resolutions
        .retain(|resolution| resolution.platform != M5ExecutionPlatform::HeadlessCi);
    assert!(packet
        .validate()
        .contains(&M5ExecutionSurfaceResolutionViolation::MissingPlatformCoverage));
}

#[test]
fn missing_surface_coverage_fails_validation() {
    let mut packet = packet();
    packet.platform_resolutions[0]
        .resolved_rows
        .retain(|row| row.surface != M5ExecutingSurface::NotebookKernel);
    assert!(packet
        .validate()
        .contains(&M5ExecutionSurfaceResolutionViolation::MissingProfileCoverage));
}

#[test]
fn widened_narrow_fails_validation() {
    let mut packet = packet();
    let row = packet.platform_resolutions[0]
        .resolved_rows
        .iter_mut()
        .find(|row| row.surface == M5ExecutingSurface::PreviewServer)
        .expect("preview row exists");
    // Force a fake narrowing to a less isolated profile.
    row.resolution_status = M5ProfileResolutionStatus::NarrowedToStricterProfile;
    row.effective_profile = Some(M5SandboxProfile::InProcessTrustedLocal);
    assert!(packet
        .validate()
        .contains(&M5ExecutionSurfaceResolutionViolation::ProfileWidenedOnNarrow));
}

#[test]
fn fail_closed_with_effective_profile_fails_validation() {
    let mut packet = packet();
    let ci = packet
        .platform_resolutions
        .iter_mut()
        .find(|resolution| resolution.platform == M5ExecutionPlatform::HeadlessCi)
        .expect("ci resolution exists");
    let remote = ci
        .resolved_rows
        .iter_mut()
        .find(|row| row.surface == M5ExecutingSurface::RemoteMutation)
        .expect("remote mutation row exists");
    remote.effective_profile = Some(M5SandboxProfile::SubprocessIsolatedLocal);
    assert!(packet
        .validate()
        .contains(&M5ExecutionSurfaceResolutionViolation::FailClosedRowHasEffectiveProfile));
}

#[test]
fn descriptor_inconsistency_fails_validation() {
    let mut packet = packet();
    packet.profile_descriptors[0].isolation_rank = 99;
    assert!(packet
        .validate()
        .contains(&M5ExecutionSurfaceResolutionViolation::ProfileDescriptorInconsistent));
}

#[test]
fn matrix_packet_id_mismatch_fails_validation() {
    let mut packet = packet();
    packet.matrix_packet_id = "not-the-matrix".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ExecutionSurfaceResolutionViolation::MatrixPacketIdMismatch));
}

#[test]
fn missing_source_contracts_fails_validation() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ExecutionSurfaceResolutionViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails_validation() {
    let mut packet = packet();
    packet.trust_review.never_widens_on_narrow = false;
    assert!(packet
        .validate()
        .contains(&M5ExecutionSurfaceResolutionViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails_validation() {
    let mut packet = packet();
    packet.consumer_projection.desktop_shows_profile_descriptor = false;
    assert!(packet
        .validate()
        .contains(&M5ExecutionSurfaceResolutionViolation::ConsumerProjectionIncomplete));
}

#[test]
fn markdown_summary_lists_every_platform() {
    let summary = packet().render_markdown_summary();
    for platform in M5ExecutionPlatform::ALL {
        assert!(
            summary.contains(platform.as_str()),
            "summary missing platform {}",
            platform.as_str()
        );
    }
}

#[test]
fn checked_support_export_matches_frozen_packet() {
    let checked = current_stable_m5_execution_surface_resolution_export()
        .expect("checked M5 execution-surface resolution export validates");
    assert_eq!(checked.packet_id, M5_EXECUTION_SURFACE_RESOLUTION_PACKET_ID);
    assert_eq!(
        checked,
        frozen_stable_m5_execution_surface_resolution_packet(),
        "checked-in support export drifted from the frozen in-code packet; regenerate with the resolution dumper"
    );
}

#[test]
fn checked_platform_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/execution-auth/m5/implement-execution-surface-classes-sandbox-profile-descriptors-and-unsupported-or-stricter-profile-truth/macos_preview_narrowed_to_remote_runtime.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/execution-auth/m5/implement-execution-surface-classes-sandbox-profile-descriptors-and-unsupported-or-stricter-profile-truth/headless_ci_fail_closed_and_inert.json"
        )),
    ] {
        let resolution: M5PlatformResolution =
            serde_json::from_str(raw).expect("fixture parses as platform resolution");
        assert!(
            resolution.validate().is_empty(),
            "fixture failed validation: {:?}",
            resolution.validate()
        );
    }
}
