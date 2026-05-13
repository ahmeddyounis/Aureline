# Support-bundle seed: manifest, redaction defaults, local preview, and exact-build capture

This is the reviewer-facing landing page for the first trustworthy
support-export path in the live Aureline shell. The seed mints a
structured support-bundle manifest with redaction defaults, exact-build
identity, and a local preview before any share or upload step.

If this document disagrees with the boundary schemas, the schemas
control. The seed never invents private field shapes.

## Truth source and contract anchors

- Manifest schema:
  [`/schemas/support/support_bundle_manifest.schema.json`](../../schemas/support/support_bundle_manifest.schema.json)
- Preview-item schema:
  [`/schemas/support/support_bundle_preview_item.schema.json`](../../schemas/support/support_bundle_preview_item.schema.json)
- Local-first redaction profile fixture:
  [`/fixtures/support/redaction_profiles/local_first_default.yaml`](../../fixtures/support/redaction_profiles/local_first_default.yaml)
- Companion preview contract:
  [`/docs/support/support_bundle_preview_contract.md`](./support_bundle_preview_contract.md)
- Companion bundle contract:
  [`/docs/support/support_bundle_contract.md`](./support_bundle_contract.md)
- Redaction guide for this seed:
  [`/docs/support/support_bundle_redaction_guide.md`](./support_bundle_redaction_guide.md)
- Vocabulary seed for this seed:
  [`/docs/support/support_export_vocabulary_seed.md`](./support_export_vocabulary_seed.md)
- Git/review event export:
  [`/schemas/support/git_review_event_alpha.schema.json`](../../schemas/support/git_review_event_alpha.schema.json)

## What this seed owns

- `aureline-support` crate, `bundle` module:
  - `SupportBundleManifest` and `SupportBundlePreviewItem` Rust records
    that mirror the boundary schemas.
  - `LocalFirstDefaults` profile that resolves a queued row's data
    class and high-risk subtype into a redaction state, decision class,
    and (when the row is held back) an excluded-class entry.
  - `ExactBuildCapture` that quotes
    `aureline_build_info::build_identity()` and
    `aureline_build_info::release_channel_class()` verbatim so the
    manifest carries the same exact-build identity as the running
    binary.
  - `SupportBundlePreviewBuilder` that mints a
    `SupportBundlePreview`. The preview is the read-only projection the
    chrome renders before letting the user export.
- `aureline-shell` crate, `support_seed` module: the live consumer that
  binds the preview into the shell. Exposes the default local preview
  every dogfood build can mint without extra inputs (build-identity
  row plus policy/trust row), the failure-drill preview that proves a
  queued secret-bearing row is rewritten to `prohibited`, and a closed
  action set splitting live `open_local_preview` /
  `copy_manifest_json` from explicitly reserved share/upload and
  hosted-intake rows.
  - `git_review_event_preview(...)`, which adds the structured
    Git/review activity export to the manifest by bundle-member ref so
    support readers can reconstruct branch, target, action, and exact
    reopen identity without scraping activity-center text.

## What this seed does NOT own

- Live byte-level redaction implementation, upload transport, hosted
  intake, or ticket routing. Those land in a later milestone.
- The full diagnostic-artifact-matrix item set. The seed surfaces the
  minimum row classes needed to prove the protected walk and the
  failure drill (exact-build identity, policy/trust metadata, raw
  secret-bearing prohibited).
- A new redaction profile vocabulary. The defaults mirror
  `support.redaction.local_first_default` from the existing fixture.

## Protected walk

1. Open support export from the shell. The chrome reads
   `SupportSeedSurface::default_local_preview(...)` to mint the
   preview.
2. Preview bundle contents and redactions locally. Every row carries a
   visible redaction state (`Metadata only — no redaction needed`),
   every excluded class is named, and every high-risk row carries a
   reviewer-visible high-risk label.
3. Verify exact-build identity is captured before export.
   `SupportSeedSurface::has_exact_build_identity()` returns `true`
   whenever `build_identity.exact_build_refs` is non-empty, which the
   seed pins on every preview.
4. The preview can be persisted to disk by the chrome (or the export
   writer) via `SupportBundlePreviewBuilder::write_preview_snapshot`.
   Reopening the same path returns the same preview manifest verbatim
   without contacting any support service.

The protected walk is exercised by the unit tests
`crates/aureline-support/src/bundle/preview.rs` →
`protected_walk_metadata_only_preview_carries_exact_build_identity`,
`crates/aureline-shell/src/support_seed/mod.rs` →
`protected_walk_default_preview_carries_exact_build_identity`, and the
integration test
`crates/aureline-support/tests/support_bundle_seed_protected_walk.rs`.

## Failure drill

1. Queue a synthetic row that declares `data_class = high_risk` and
   `high_risk_content_class = secret_bearing`.
2. Build the preview. The local-first defaults rewrite the row's
   `redaction_state` to `prohibited`, mint an `ExcludedClass` entry
   that names the row's `support.item.raw_secrets` id with the
   `prohibited_secret_or_token` reason, add the same id to
   `redaction_report.prohibited_items_confirmed_absent`, and append a
   `RedactionReport.high_risk_items` entry that records the handling
   summary.
3. The preview surface lights `honesty_marker_present = true` so the
   chrome's banner cannot fabricate "all clear" while a row was
   prohibited.
4. The seed never exports raw secret bytes. The
   `secret_scan_summary.raw_secret_values_exported` field is pinned
   to `false` by the schema and by the seed.

The failure drill is exercised by
`failure_drill_secret_bearing_row_is_rewritten_to_prohibited_and_omitted`
in `aureline-support` and
`failure_drill_preview_holds_secret_back_and_lights_honesty_marker` in
`aureline-shell`.

## Reuse statements

- **Exact-build identity.** The seed quotes
  `aureline_build_info::build_identity` verbatim. Help/About, the
  release-center skeleton, and the support manifest all read the same
  identity. The seed never re-derives versions.
- **Notification routing.** The seed does not invent its own
  notification channel. Lifecycle notifications about the support
  preview reuse the canonical envelope/router contract under
  `aureline-shell::notifications`.
- **Activity-center persistence.** When the chrome wants to record a
  durable row for a support-export attempt, it routes through the
  existing `aureline-shell::activity_center` lane. The support seed
  itself owns only the manifest + preview projection.
- **Git/review activity.** Git publish, mutation, and review-workspace
  rows use `aureline-shell::activity_center::git_review` as the
  structured source. The support preview references the matching
  `git_review_event_support_export` member rather than rendering or
  parsing activity-center labels.
- **Redaction profile.** The defaults mirror
  `support.redaction.local_first_default`; no new profile is invented.

## Out of scope

- Hosted support portals, advisory publication, or external support-
  intake workflows.
- Notification-center breadth beyond the named M1 surfaces.
- Live byte-level redaction.

## Evidence

The seed's primary evidence is captured by the focused tests above and
by the integration test under
`crates/aureline-support/tests/support_bundle_seed_protected_walk.rs`.
The owning proof packet is
[`/artifacts/milestones/m1/proof_packets/support_bundle.md`](../../artifacts/milestones/m1/proof_packets/support_bundle.md);
see that packet for refresh cadence and the evidence sink.
