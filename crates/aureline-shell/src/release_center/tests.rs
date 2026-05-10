use std::path::Path;

use aureline_build_info::BuildIdentityRecord;
use aureline_support::bundle::{ExactBuildCapture, ReleaseChannelClass};
use serde::Deserialize;

use super::*;
use crate::support_seed::SupportSeedSurface;

const FIXTURE_EXACT_BUILD_REF: &str =
    "build-id:aureline:dev:0.0.0:aarch64-apple-darwin:debug:0123456";

fn fixture_build_identity() -> BuildIdentityRecord {
    BuildIdentityRecord {
        schema_version: 1,
        commit: "0123456789abcdef0123456789abcdef01234567".to_owned(),
        commit_short: "0123456".to_owned(),
        dirty: false,
        toolchain_channel: "stable".to_owned(),
        rustc_version: "rustc 1.78.0".to_owned(),
        cargo_version: "cargo 1.78.0".to_owned(),
        host_triple: "aarch64-apple-darwin".to_owned(),
        target_triple: "aarch64-apple-darwin".to_owned(),
        profile: "debug".to_owned(),
        workspace_version: "0.0.0".to_owned(),
        source_date_epoch: 1_714_492_800,
        build_timestamp_utc: "2024-04-30T12:00:00Z".to_owned(),
    }
}

fn linked_support_seed() -> SupportSeedSurface {
    let capture = ExactBuildCapture::for_fixture(
        FIXTURE_EXACT_BUILD_REF,
        "0.0.0",
        ReleaseChannelClass::DevLocal,
    );
    SupportSeedSurface::default_local_preview(capture, "2026-05-10T07:00:00Z")
        .expect("build default support preview")
}

fn missing_chain_support_seed() -> SupportSeedSurface {
    // Mint a support-seed surface whose manifest carries a different
    // exact-build identity than the running build. The release-center
    // surface's linkage row must surface `missing_chain`.
    let capture = ExactBuildCapture::for_fixture(
        "build-id:aureline:dev:0.0.0:aarch64-apple-darwin:debug:deadbeef",
        "0.0.0",
        ReleaseChannelClass::DevLocal,
    );
    SupportSeedSurface::default_local_preview(capture, "2026-05-10T07:01:00Z")
        .expect("build mismatched support preview")
}

#[test]
fn protected_walk_renders_running_build_with_linked_support_preview() {
    let identity = fixture_build_identity();
    let support = linked_support_seed();

    let surface = ReleaseCenterSurface::project(ReleaseCenterInputs {
        build_identity: &identity,
        release_channel_class_token: "dev_local",
        exact_build_identity_ref: FIXTURE_EXACT_BUILD_REF,
        support_seed: Some(&support),
    });

    assert_eq!(surface.record_kind, RELEASE_CENTER_SURFACE_RECORD_KIND);
    assert_eq!(
        surface.schema_version,
        RELEASE_CENTER_SURFACE_SCHEMA_VERSION
    );
    assert!(surface.has_running_build_exact_build_identity());
    assert!(surface.support_linkage_is_linked());
    assert!(!surface.honesty_marker_present);

    // Build identity quotes the build-info record verbatim.
    assert_eq!(surface.build_identity.workspace_version, "0.0.0");
    assert_eq!(surface.build_identity.commit_short, "0123456");
    assert_eq!(
        surface.build_identity.tree_state_class,
        TreeStateClass::CleanCheckout
    );
    assert_eq!(
        surface.build_identity.install_mode_class,
        InstallModeClass::DevLocalBuiltFromSource
    );
    assert_eq!(
        surface.build_identity.origin_posture_class,
        OriginPostureClass::LocalDevBuild
    );
    assert!(!surface.build_identity.honesty_marker_present);

    // The running release-candidate row carries the exact-build identity
    // and reports a healthy support link.
    let running = &surface.running_release_candidate;
    assert_eq!(running.role_class, ReleaseCandidateRoleClass::RunningBuild);
    assert_eq!(running.exact_build_identity_ref, FIXTURE_EXACT_BUILD_REF);
    assert_eq!(running.support_link_state, ProvenanceLinkState::Linked);
    assert!(running
        .linked_support_exact_build_refs
        .contains(&FIXTURE_EXACT_BUILD_REF.to_owned()));
    assert!(!running.honesty_marker_present);
    assert!(running.provenance_line.contains(FIXTURE_EXACT_BUILD_REF));

    // Provenance scaffold rows are seed placeholders.
    assert_eq!(surface.provenance_scaffold.rows.len(), 5);
    for row in &surface.provenance_scaffold.rows {
        assert_eq!(row.state, ProvenanceRowState::SeedPlaceholderAwaitingWiring);
    }
    assert!(!surface.provenance_scaffold.honesty_marker_present);

    // Support linkage is linked, exposes the open-preview command, and
    // forwards the manifest's exact-build refs.
    assert_eq!(
        surface.support_linkage.link_state,
        ProvenanceLinkState::Linked
    );
    assert_eq!(
        surface.support_linkage.open_preview_command_id.as_deref(),
        Some(COMMAND_ID_OPEN_LOCAL_SUPPORT_PREVIEW)
    );
    assert!(surface.support_linkage.support_seed_record_kind_present);
    assert!(!surface.support_linkage.support_manifest_has_prohibited_row);
    assert!(!surface.support_linkage.honesty_marker_present);

    // Live actions stay live; reserved actions stay reserved.
    let open = surface
        .find_action(ReleaseCenterActionClass::OpenLocalSupportPreview)
        .expect("open-preview action present");
    assert_eq!(open.availability, ReleaseCenterActionAvailability::Live);
    assert_eq!(
        open.command_id.as_deref(),
        Some(COMMAND_ID_OPEN_LOCAL_SUPPORT_PREVIEW)
    );

    for class in [
        ReleaseCenterActionClass::PublishStagedCandidate,
        ReleaseCenterActionClass::PromoteReleaseCandidate,
        ReleaseCenterActionClass::RollbackToPriorBuild,
        ReleaseCenterActionClass::RevokePublishedArtifact,
        ReleaseCenterActionClass::YankPublishedArtifact,
    ] {
        let action = surface.find_action(class).expect("reserved row present");
        assert_eq!(
            action.availability,
            ReleaseCenterActionAvailability::ReservedForLaterMilestone
        );
        assert!(action.command_id.is_none());
        assert!(action.reserved_reason.is_some());
    }

    // Plaintext renders the headings the chrome will quote verbatim.
    let plaintext = surface.render_plaintext();
    assert!(plaintext.contains("Release center surface"));
    assert!(plaintext.contains("[Build identity]"));
    assert!(plaintext.contains("[Release candidate]"));
    assert!(plaintext.contains("[Provenance]"));
    assert!(plaintext.contains("[Support linkage]"));
    assert!(plaintext.contains("[Actions]"));
    assert!(plaintext.contains("Honesty marker: none"));
}

#[test]
fn failure_drill_missing_chain_lights_honesty_marker_and_blocks_open_preview() {
    let identity = fixture_build_identity();
    let support = missing_chain_support_seed();

    let surface = ReleaseCenterSurface::project(ReleaseCenterInputs {
        build_identity: &identity,
        release_channel_class_token: "dev_local",
        exact_build_identity_ref: FIXTURE_EXACT_BUILD_REF,
        support_seed: Some(&support),
    });

    // The linkage row exposes the missing chain and the global honesty
    // marker fires.
    assert_eq!(
        surface.support_linkage.link_state,
        ProvenanceLinkState::MissingChain
    );
    assert!(surface.support_linkage.honesty_marker_present);
    assert!(surface.honesty_marker_present);
    assert_eq!(
        surface.running_release_candidate.support_link_state,
        ProvenanceLinkState::MissingChain
    );
    assert!(surface.running_release_candidate.honesty_marker_present);

    // The "open local support preview" action is held back so the chrome
    // cannot route to a preview that does not match the running build.
    let open = surface
        .find_action(ReleaseCenterActionClass::OpenLocalSupportPreview)
        .expect("open-preview row present");
    assert_eq!(
        open.availability,
        ReleaseCenterActionAvailability::BlockedByMissingLinkage
    );
    assert!(open.command_id.is_none());

    // Copy provenance and view exact-build identity stay live so a
    // reviewer can still copy the surface for a support packet.
    let copy = surface
        .find_action(ReleaseCenterActionClass::CopyProvenanceLineForSupport)
        .expect("copy action present");
    assert_eq!(copy.availability, ReleaseCenterActionAvailability::Live);
    assert_eq!(
        copy.command_id.as_deref(),
        Some(COMMAND_ID_COPY_PROVENANCE_LINE_FOR_SUPPORT)
    );

    let view = surface
        .find_action(ReleaseCenterActionClass::ViewExactBuildIdentity)
        .expect("view-identity action present");
    assert_eq!(view.availability, ReleaseCenterActionAvailability::Live);

    // The plaintext block surfaces the missing chain.
    let plaintext = surface.render_plaintext();
    assert!(plaintext.contains("Missing chain"));
    assert!(plaintext.contains("Honesty marker: present"));
}

#[test]
fn no_support_seed_wired_renders_not_wired_state_without_collapsing_surface() {
    let identity = fixture_build_identity();

    let surface = ReleaseCenterSurface::project(ReleaseCenterInputs {
        build_identity: &identity,
        release_channel_class_token: "dev_local",
        exact_build_identity_ref: FIXTURE_EXACT_BUILD_REF,
        support_seed: None,
    });

    assert_eq!(
        surface.support_linkage.link_state,
        ProvenanceLinkState::NotWired
    );
    assert!(!surface.support_linkage.support_seed_record_kind_present);
    assert!(surface
        .support_linkage
        .support_manifest_exact_build_refs
        .is_empty());
    assert!(surface.honesty_marker_present);

    // The running candidate row keeps its build identity even when the
    // support lane is not wired.
    assert_eq!(
        surface.running_release_candidate.exact_build_identity_ref,
        FIXTURE_EXACT_BUILD_REF
    );
    assert_eq!(
        surface.running_release_candidate.support_link_state,
        ProvenanceLinkState::NotWired
    );

    let open = surface
        .find_action(ReleaseCenterActionClass::OpenLocalSupportPreview)
        .expect("open-preview row present");
    assert_eq!(
        open.availability,
        ReleaseCenterActionAvailability::BlockedByMissingLinkage
    );
}

#[test]
fn unknown_channel_token_lights_honesty_marker_and_renders_unknown_origin() {
    let identity = fixture_build_identity();
    let support = linked_support_seed();

    let surface = ReleaseCenterSurface::project(ReleaseCenterInputs {
        build_identity: &identity,
        release_channel_class_token: "weird-channel",
        exact_build_identity_ref: FIXTURE_EXACT_BUILD_REF,
        support_seed: Some(&support),
    });

    assert_eq!(
        surface.build_identity.install_mode_class,
        InstallModeClass::UnknownInstallMode
    );
    assert_eq!(
        surface.build_identity.origin_posture_class,
        OriginPostureClass::UnknownOriginPosture
    );
    assert!(surface.build_identity.honesty_marker_present);
    assert!(surface.honesty_marker_present);
}

#[test]
fn empty_exact_build_identity_ref_lights_honesty_marker() {
    let identity = fixture_build_identity();
    let support = linked_support_seed();

    let surface = ReleaseCenterSurface::project(ReleaseCenterInputs {
        build_identity: &identity,
        release_channel_class_token: "dev_local",
        exact_build_identity_ref: "",
        support_seed: Some(&support),
    });

    assert!(surface.build_identity.honesty_marker_present);
    assert!(surface
        .running_release_candidate
        .exact_build_identity_ref
        .is_empty());
    // With no exact-build identity, the linkage row reports `missing_chain`
    // because the running build's identity cannot match any manifest ref.
    assert_eq!(
        surface.support_linkage.link_state,
        ProvenanceLinkState::MissingChain
    );
    assert!(surface.honesty_marker_present);
    assert!(surface
        .running_release_candidate
        .provenance_line
        .contains("unknown_exact_build_identity"));
}

#[test]
fn surface_round_trips_through_serde() {
    let identity = fixture_build_identity();
    let support = linked_support_seed();

    let surface = ReleaseCenterSurface::project(ReleaseCenterInputs {
        build_identity: &identity,
        release_channel_class_token: "dev_local",
        exact_build_identity_ref: FIXTURE_EXACT_BUILD_REF,
        support_seed: Some(&support),
    });

    let json = serde_json::to_string(&surface).expect("ser");
    let parsed: ReleaseCenterSurface = serde_json::from_str(&json).expect("de");
    assert_eq!(parsed, surface);
}

#[test]
fn fixture_protected_walk_replays_into_the_release_center_surface() {
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../fixtures/release/release_center_cases/protected_walk_running_build_linked.json",
    );
    let payload = std::fs::read_to_string(&fixture_path).expect("fixture must read");
    let fixture: ReleaseCenterFixture = serde_json::from_str(&payload).expect("fixture must parse");

    let identity = fixture_build_identity();
    let support = if fixture.input.wire_support_seed {
        Some(linked_support_seed())
    } else {
        None
    };

    let surface = ReleaseCenterSurface::project(ReleaseCenterInputs {
        build_identity: &identity,
        release_channel_class_token: &fixture.input.release_channel_class_token,
        exact_build_identity_ref: &fixture.input.exact_build_identity_ref,
        support_seed: support.as_ref(),
    });

    assert_eq!(surface.record_kind, fixture.expect.record_kind);
    assert_eq!(
        surface.honesty_marker_present,
        fixture.expect.honesty_marker_present
    );
    assert_eq!(
        surface.build_identity.install_mode_token,
        fixture.expect.install_mode_token
    );
    assert_eq!(
        surface.build_identity.origin_posture_token,
        fixture.expect.origin_posture_token
    );
    assert_eq!(
        surface.support_linkage.link_state_token,
        fixture.expect.support_link_state_token
    );
    assert_eq!(
        surface.running_release_candidate.support_link_state_token,
        fixture.expect.support_link_state_token
    );
}

#[test]
fn fixture_failure_drill_replays_missing_chain() {
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../fixtures/release/release_center_cases/failure_drill_missing_provenance_chain.json",
    );
    let payload = std::fs::read_to_string(&fixture_path).expect("fixture must read");
    let fixture: ReleaseCenterFixture = serde_json::from_str(&payload).expect("fixture must parse");

    let identity = fixture_build_identity();
    let support = if fixture.input.wire_support_seed {
        Some(missing_chain_support_seed())
    } else {
        None
    };

    let surface = ReleaseCenterSurface::project(ReleaseCenterInputs {
        build_identity: &identity,
        release_channel_class_token: &fixture.input.release_channel_class_token,
        exact_build_identity_ref: &fixture.input.exact_build_identity_ref,
        support_seed: support.as_ref(),
    });

    assert!(surface.honesty_marker_present);
    assert_eq!(
        surface.support_linkage.link_state_token,
        fixture.expect.support_link_state_token
    );
    // Even when the chain is broken, the support-export copy stays live so
    // a reviewer can still hand the running build's identity to support.
    let copy = surface
        .find_action(ReleaseCenterActionClass::CopyProvenanceLineForSupport)
        .expect("copy action present");
    assert_eq!(copy.availability, ReleaseCenterActionAvailability::Live);
}

#[derive(Debug, Deserialize)]
struct ReleaseCenterFixture {
    input: ReleaseCenterFixtureInput,
    expect: ReleaseCenterFixtureExpect,
}

#[derive(Debug, Deserialize)]
struct ReleaseCenterFixtureInput {
    release_channel_class_token: String,
    exact_build_identity_ref: String,
    wire_support_seed: bool,
}

#[derive(Debug, Deserialize)]
struct ReleaseCenterFixtureExpect {
    record_kind: String,
    honesty_marker_present: bool,
    install_mode_token: String,
    origin_posture_token: String,
    support_link_state_token: String,
}
