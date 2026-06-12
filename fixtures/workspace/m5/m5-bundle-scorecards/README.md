# Fixtures: M5 bundle scorecards

This directory contains fixture metadata for the `m5_bundle_scorecards_packet`.

The canonical full corpus is checked in at:

`artifacts/workspace/m5/m5-bundle-scorecards.json`

and is validated by `schemas/workspace/m5-bundle-scorecards.schema.json` and the typed model in the
`aureline-workspace` crate (`m5_bundle_scorecards`).

## Coverage

- **Every wedge, one scorecard.** Each of the eight M5 launch wedges — `notebook_workspace`,
  `data_and_api_workspace`, `profiler_workspace`, `framework_pack_workspace`, `docs_workspace`,
  `companion_workspace`, `sync_handoff_workspace`, and `local_folder_workspace` — has one bundle
  scorecard (`covers_every_wedge`).
- **Every claimed and effective class.** `certified`, `probable`, `community`, `imported`,
  `preview`, and `local_draft` are all exercised as both claimed and effective classes.
- **Every confidence and freshness level.** `native`, `bridged`, `approximated`, and `unverified`
  confidence and `fresh`, `aging`, `stale`, and `missing` freshness are all present.
- **Every platform and dependency lifecycle stage.** `linux`, `macos`, `windows`, and `web` all
  appear in supported-platform sets; dependency `stable`, `preview`, `labs`, `policy_gated`,
  `mirror_only`, and `bounded_platform` lifecycle stages are all exercised.

## What the corpus proves

- **No inheritance of certified language by inertia.** The `profiler` scorecard claims `certified`
  but its `bridged` confidence and `stale` evidence narrow it to an effective `probable`; the
  `companion` scorecard is `imported`/`approximated` with `missing` evidence and stays `imported`.
  Approximate or unverified confidence and stale or missing evidence never present as certified.
- **Public copy narrows when proof is stale or bounded.** The `data-and-api` scorecard stays
  `certified` but is platform-bounded to Linux and macOS, so it carries a caveat that narrows its
  certified claim; the `profiler` scorecard carries a caveat naming its downgrade. Only the
  `notebook` scorecard — certified, native, fresh, full-platform — needs no caveat.
- **Joined to existing proofs.** Every scorecard carries `manifest_ref`, `compatibility_scorecard_ref`,
  `archetype_cert_ref`, and `reference_workspace_ref`, joining the workflow-bundle manifest, the
  compatibility-scorecard packet, the archetype-certification packet, and a certified reference
  workspace rather than minting another unlinked proof format.
- **One object model, many consumers.** Every scorecard carries `start_center_ref`,
  `migration_center_ref`, `help_about_ref`, `release_center_ref`, `docs_help_ref`,
  `support_export_ref`, and `diagnostics_ref`.

The `corpus_manifest.json` in this directory enumerates each scorecard's `corpus_id`, the
`bundle_id` it scores, its claimed and effective classes, and the invariants it proves.
