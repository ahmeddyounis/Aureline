# M5 Release-Publication Component Surface Certification (M05-867)

Closing capstone for the B101 release-center / publication-component lane. Where
the freeze matrix
(`freeze_the_m5_release_candidate_card_version_bump_row_publish_target_row_artifact_provenance_bundle_card_and_promotion_timeline_component_matrix`)
defines the six reusable components, the M05-861..864 primitive lanes narrow each
one, the M05-865 consumer lane adopts them, and the M05-866 accessibility lane
proves keyboard / screen-reader / CLI-export parity and per-family auto-narrowing,
this lane **certifies** that the shared component truth holds on every claimed M5
release-publication surface — and automatically narrows any surface that cannot
sustain it.

- Rust module: `crates/aureline-release/src/certify_release_candidate_card_version_bump_row_publish_target_row_artifact_provenance_bundle_card_and_promotion_timeline_component_truth_on_every_claimed_m5_release_publication_surface/`
- Boundary schema: `schemas/ui/m5-release-publication-component-certification.schema.json`
- Release proof: `artifacts/release/m5-release-publication-component-certification-proof/`
  (`support_export.json`, `matrix.csv`, `report.md`)
- Fixtures: `fixtures/ui/m5-release-publication-component-certification/`

## What is certified

The packet is keyed on the claimed **surface** a user proposes, publishes,
promotes, mirrors, evaluates, or exports a release through — not on the component
family it renders. The eight certified surfaces are:

`release_center`, `update_center`, `about_help`, `docs`, `enterprise_evaluation`,
`mirror_offline`, `cli_headless`, `support_export`.

Each row certifies its surface across six truth axes — exactly the parity
dimensions the spec requires verifying:

| Axis | Meaning |
| --- | --- |
| `visual` | candidate scope / blocker freshness, target visibility / mutability / auth source, provenance, and rollout ring are shown on the primary surface |
| `keyboard` | the same candidate / target / provenance / timeline truth and its actions are reachable without a pointer |
| `screen_reader` | the same truth is announced non-visually, never relying on color or a badge glyph alone |
| `cli_export` | **always-on**: the certified surface state is reconstructable as text / JSON / Markdown for support and automation |
| `degraded_state` | stale evidence, a partial signature / attestation, a masked target-auth posture, or an unverified mirror honestly downgrades a `Certified` / `Supported` claim to degraded / provisional / unverified / policy-blocked |
| `rollback_revocation` | a rollback or revocation's blast radius and revocation scope are stated before any promotion or emergency action, never reading like a generic status change |

Each surface also cites the frozen component families it renders
(`consumed_families`). Across the whole packet every one of the six families must
be certified on at least one surface (`all_families_covered`), which is how this
capstone proves the full component matrix runs across the claimed consumers.

## Publication-support claim ladder

The claim a surface asserts (and the weakest ceiling it is certified down to) is
the reused M05-866 `M5PublicationSupportClaim` ladder, strongest first:

`certified` > `supported` > `degraded` > `provisional` > `unverified` >
`policy_blocked`.

Certification may only **narrow** a claim, never strengthen it.

## Verdict derivation (green / yellow / red)

The `derived_status` on every row is always recomputed from the axis outcomes and
claim narrowing — never asserted. The invariant is **a degraded axis must produce
a visible claim narrowing**.

- **Green** — every axis certified and the claimed publication-support claim is
  delivered (`claimed_claim == certified_claim`, no `claim_auto_narrow`).
- **Yellow** — an axis is not current and the surface discloses the reduction by
  narrowing its claim to the weakest supported ceiling. The `claim_auto_narrow`
  block must bind to a non-always-on axis that is `disclosed_narrowed`, carry a
  precise (non-generic) `visible_label`, and its `from_claim`/`to_claim` must
  match the row's `claimed_claim`/`certified_claim`. The narrowed axis outcome
  names a frozen `M5ReleaseCenterDowngradeTrigger`.
- **Red** — any of: an axis is `undisclosed_drift`; the always-on `cli_export`
  axis is not certified (or copy/export is incomplete); the certified claim is
  stronger than the claimed one; a degraded axis is retained behind a full claim
  with no narrowing; or the narrowing block is inconsistent (spurious, wrongly
  bound, generic-labelled, or bound to the always-on axis). Red surfaces block
  the release; gaps are expressed as narrowed (yellow) claims or blocked (red)
  rows, never as hidden exceptions.

Every row cites exactly one canonical release-proof bundle —
`artifacts/release/m5-release-center-component-proof/support_export.json`, the
frozen release-center component release proof — rather than cloning per-surface
evidence, and records the M05-866 accessibility support export as supporting
evidence. The packet is metadata-only: raw artifacts, signing keys, publish
credentials, and mirror cursors never cross this boundary.

## Seed certification

The checked-in packet certifies all eight surfaces: **4 green / 4 yellow / 0 red**.

| Surface | Claimed → Certified | Status | Binding axis |
| --- | --- | --- | --- |
| release_center | certified → certified | green | — |
| about_help | supported → supported | green | — |
| docs | supported → supported | green | — |
| support_export | certified → certified | green | — |
| update_center | certified → provisional | yellow | degraded_state |
| enterprise_evaluation | certified → unverified | yellow | degraded_state |
| mirror_offline | supported → provisional | yellow | degraded_state |
| cli_headless | certified → degraded | yellow | rollback_revocation |

## Regenerating the proof

The on-disk `support_export.json` is the `include_str!` canonical for the
round-trip test. Regenerate the artifacts and fixtures after any change to the
seeded builder:

```
GEN_PUBLICATION_CERT_ARTIFACTS=1 cargo test -p aureline-release \
  certify_release_candidate_card_version_bump_row_publish_target_row_artifact_provenance_bundle_card_and_promotion_timeline_component_truth_on_every_claimed_m5_release_publication_surface::tests::generate_artifacts
```

Then rebuild so the baked-in `include_str!` picks up the new content, and run:

```
cargo test -p aureline-release --lib \
  certify_release_candidate_card_version_bump_row_publish_target_row_artifact_provenance_bundle_card_and_promotion_timeline_component_truth_on_every_claimed_m5_release_publication_surface
```
