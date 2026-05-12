# Release center / provenance seed: running build, exact-build identity, and support-export linkage

This is the reviewer-facing landing page for the release-center / provenance
skeleton in the Aureline shell crate. The seed mints a single inspectable
surface that joins the running build's exact-build identity, channel,
provenance row scaffold, and the support-bundle preview before any
publication, advisory, or rollback subsystem comes online.

If this document disagrees with the boundary contracts and schemas under
`/docs/release/` and `/schemas/release/`, those contracts win. The seed
never invents private field shapes.

## Truth source and contract anchors

- Release-center object-model contract:
  [`/docs/release/release_center_object_model_contract.md`](./release_center_object_model_contract.md)
- Release-center / provenance linkage contract:
  [`/docs/release/release_center_provenance_linkage.md`](./release_center_provenance_linkage.md)
- Provenance crosswalk schema:
  [`/schemas/release/release_provenance_crosswalk.schema.json`](../../schemas/release/release_provenance_crosswalk.schema.json)
- Support-bundle manifest schema (support linkage target):
  [`/schemas/support/support_bundle_manifest.schema.json`](../../schemas/support/support_bundle_manifest.schema.json)
- Companion Help/About seed (shared install-mode and provenance row vocabulary):
  [`/docs/help/help_about_truth_source.md`](../help/help_about_truth_source.md)
- Companion support-bundle seed (the linkage surface):
  [`/docs/support/support_bundle_seed.md`](../support/support_bundle_seed.md)
- Exact-build identity model:
  [`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md)

## What this seed owns

- `aureline-shell` crate, `release_center` module:
  - `ReleaseCenterSurface` — the canonical Rust record the seed projects
    to answer "what is the running build, where did it come from, and
    where does support pull the receipts?". One projection feeds
    plaintext copy, the running release-candidate row, the provenance
    row scaffold, the support-linkage row, and the closed action set.
  - `ReleaseCenterInputs` — the input bundle the projection consumes:
    the build-info `BuildIdentityRecord`, the release-channel-class
    token, the exact-build identity ref, and an optional reference to a
    `SupportSeedSurface`.
  - `ReleaseCandidateRoleClass`, `ReleaseCenterActionClass`,
    `ProvenanceLinkState`, and `OriginPostureClass` — closed enums that
    pin the surface's row and action vocabulary so the chrome cannot
    fabricate values the seed has not promised.

## What this seed does NOT own

- Publication, promotion, rollback, revocation, or yank flows.
  Those rows are reserved on the action set with stable tokens so the
  later milestone can light them without renaming.
- The full release-candidate index, staged-candidate browsing, or the
  release-evidence packet body. The seed only mints the running-build
  row.
- Provenance verification (signature, attestation, checksum, SBOM,
  advisory). The provenance scaffold renders typed
  `seed_placeholder_awaiting_wiring` rows so the chrome and the export
  writer agree on stable tokens before the verifier lands.
- Any new redaction profile or build-identity vocabulary. The seed
  reuses `aureline_build_info`, `aureline_support`, and the Help/About
  install-mode + provenance vocabulary verbatim.

## Protected walk

1. Project the release / provenance view through the seed harness.
   `ReleaseCenterSurface::project(...)` reads the running build's
   `BuildIdentityRecord`, the
   `aureline_build_info::release_channel_class()` token, the
   `aureline_build_info::exact_build_identity_ref()` value, and the
   `SupportSeedSurface` from `aureline-shell::support_seed`.
2. Inspect the running-build row. The row carries a non-empty
   `exact_build_identity_ref`, the canonical channel token, a
   deterministic provenance line, and the linkage row's state. The
   plaintext block produced by `render_plaintext()` is stable across
   runs for the same input snapshot so a support packet can quote it
   verbatim.
3. Confirm one lineage surface exists. The support-linkage row reports
   `linked` and exposes
   `cmd:release_center.open_local_support_preview` when the support
   manifest's `build_identity.exact_build_refs` includes the running
   build's identity. The release-center row's
   `linked_support_exact_build_refs` mirrors that set so a reviewer can
   see the join key without round-tripping through the support pane.

The protected walk is exercised by
`crates/aureline-shell/src/release_center/tests.rs` →
`protected_walk_renders_running_build_with_linked_support_preview` and
the fixture replay
`fixture_protected_walk_replays_into_the_release_center_surface`
against
[`/fixtures/release/release_center_cases/protected_walk_running_build_linked.json`](../../fixtures/release/release_center_cases/protected_walk_running_build_linked.json).

## Failure drill

1. Wire a `SupportSeedSurface` whose preview manifest carries an
   exact-build ref that does not match the running build. (In the test
   suite, the failure-drill helper mints a manifest with a different
   commit-short suffix.)
2. Project the release-center surface. The linkage row reports
   `missing_chain`, the running release-candidate row's
   `support_link_state` flips to `missing_chain`, and the seed lights
   `honesty_marker_present`.
3. The `open_local_support_preview` action is held back as
   `blocked_by_missing_linkage` so the chrome cannot route to a preview
   that does not match the running build. The
   `copy_provenance_line_for_support` and `view_exact_build_identity`
   actions stay available so a reviewer can still hand the running build's
   identity to support during the drill.
4. The plaintext block surfaces "Missing chain" and "Honesty marker:
   present" so a copy-for-support hand-off cannot accidentally claim
   completeness.

The failure drill is exercised by
`failure_drill_missing_chain_lights_honesty_marker_and_blocks_open_preview`
and the fixture replay
`fixture_failure_drill_replays_missing_chain` against
[`/fixtures/release/release_center_cases/failure_drill_missing_provenance_chain.json`](../../fixtures/release/release_center_cases/failure_drill_missing_provenance_chain.json).

## Reuse statements

- **Exact-build identity.** The seed quotes
  `aureline_build_info::build_identity` and
  `aureline_build_info::exact_build_identity_ref` verbatim. Help/About,
  the support-bundle manifest, and the release-center surface all read
  the same identity. The seed never re-derives versions.
- **Install-mode and provenance vocabulary.** The seed reuses
  `crate::help_about::InstallModeClass`, `TreeStateClass`,
  `ProvenanceRowClass`, and `ProvenanceRowState` so Help/About and the
  release center cannot drift their tokens.
- **Release-channel class.** The seed maps the build-info channel token
  through `aureline_support::bundle::ReleaseChannelClass` so the
  release-center surface and the support-bundle manifest land on the
  same closed enum.
- **Support-export linkage.** The seed reads the `SupportSeedSurface`
  from `aureline-shell::support_seed`. The
  manifest's `build_identity.exact_build_refs` is the join key; the
  release-center surface never invents its own support-export model.

## Out of scope

- Hosted release portals, advisory publication, or external incident
  workspaces.
- Update / rollback orchestration. The reserved actions render row
  scaffolds for those lanes without claiming depth this seed does not
  own.
- Notification-center breadth beyond the named M1 surfaces.

## Evidence

The seed's primary evidence is captured by the focused unit and fixture
tests above. The owning proof packet is
[`/artifacts/milestones/m1/proof_packets/release_center_seed.md`](../../artifacts/milestones/m1/proof_packets/release_center_seed.md);
see that packet for refresh cadence and the evidence sink.
