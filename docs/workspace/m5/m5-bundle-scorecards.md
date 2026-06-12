# M5 bundle scorecards

This document is the contract for the `m5_bundle_scorecards_packet`. The canonical packet is checked
in at `artifacts/workspace/m5/m5-bundle-scorecards.json`, validated by
`schemas/workspace/m5-bundle-scorecards.schema.json`, and backed by the typed model in the
`aureline-workspace` crate (`m5_bundle_scorecards`).

## What the packet governs

Every claimed M5 launch bundle — the workflow bundles defined by the workflow-bundle manifests (see
`docs/workspace/m5/m5-workflow-bundle-manifests.md`) — gets one machine-readable **compatibility
scorecard** that attaches honest compatibility, imported-versus-native confidence, and
certified-archetype linkage to the bundle it scores. Each [`BundleScorecard`] answers:

- **What it scores** — a `bundle_id` matching a workflow-bundle manifest and the
  `source_manifest_version` of that manifest.
- **Where it runs** — a non-empty, de-duplicated `supported_platforms` list drawn from `linux`,
  `macos`, `windows`, and `web`, and a `platform_bounded` flag that must equal whether any platform
  is omitted.
- **What it depends on** — a `bundle_dependencies` list, each carrying a `lifecycle_stage` of
  `stable`, `preview`, `labs`, `policy_gated`, `mirror_only`, or `bounded_platform`. A non-stable
  stage is a disclosed dependency marker.
- **How native it is** — an `imported_vs_native_confidence` of `native`, `bridged`, `approximated`,
  or `unverified`.
- **How current its proof is** — an `evidence_freshness` of `fresh`, `aging`, `stale`, or `missing`.
- **What it links to** — a `reference_workspace_ref` to a certified reference workspace, a
  `manifest_ref`, a `compatibility_scorecard_ref`, and an `archetype_cert_ref`.

## Claimed class versus effective class

A scorecard records both the `claimed_class` a bundle source asserts and the `effective_class` the
evidence actually backs. The six classes — `certified`, `probable`, `community`, `imported`,
`preview`, `local_draft` — form an assurance ladder ([`BundleScorecardClass::rank`]). The effective
class is recomputed ([`BundleScorecard::computed_effective_class`]) as the **minimum** of three
ranks: the claimed-class rank, the confidence cap, and the freshness cap. It can therefore never
out-rank any of its inputs:

- `bridged` confidence caps the class at `probable`; `approximated` caps it at `imported`;
  `unverified` caps it at `preview`.
- `stale` evidence caps the class at `probable`; `missing` evidence caps it at `imported`.

So a bundle that *claims* `certified` but is only `bridged`, or whose evidence has gone `stale`,
narrows automatically to `probable`; an imported, `approximated` bundle stays `imported`. Imported or
approximate behavior can no longer inherit native or certified language by inertia. The recorded
`effective_class` must equal the recomputed one, and `presents_as_certified` must equal whether the
effective class is `certified` — only a bundle whose evidence genuinely backs certification presents
as certified.

## Public copy narrows when proof is weak, stale, or bounded

Narrowing is never silent. A scorecard requires a caveat
([`BundleScorecard::caveats_required`]) whenever its public copy must narrow: it was downgraded
below its claimed class, its confidence is not native, its evidence is stale or missing, or its
support is platform-bounded. Only a `certified`, `native`, `fresh`, full-platform bundle escapes a
caveat. A certified but platform-bounded bundle keeps its certified class for the platforms it
supports while carrying a caveat that names the bound, so the copy narrows to the proof.

## Joined to existing proofs, not another silo

Every scorecard joins the proof formats this lane already publishes instead of minting another
unlinked one ([`BundleScorecard::linkage_complete`]): a `manifest_ref` into the workflow-bundle
manifest it scores, a `compatibility_scorecard_ref` into the compatibility-scorecard packet, an
`archetype_cert_ref` into the archetype-certification packet, and a `reference_workspace_ref` into a
certified reference workspace.

## One scorecard, many consumers

The same packet drives discovery, review, diagnostics, and claim publication. Each scorecard carries
a `start_center_ref`, a `migration_center_ref`, a `help_about_ref`, a `release_center_ref`, a
`docs_help_ref`, plus `support_export_ref` and `diagnostics_ref`. Start center, migration center,
help/About, release center, and docs/help bundle surfaces ingest this one object model instead of
cloning status text, so public copy narrows automatically when the underlying scorecard narrows. The
packet is metadata-only — every field is a typed state, a count, or an opaque ref, and it carries no
credential bodies, raw provider payloads, raw local paths, or bundle binary contents.

## How it is validated

The typed model parses the embedded packet and runs `validate()`, which checks the closed
vocabularies, full wedge coverage, the recomputed effective class, the platform-bounded
recomputation and well-formed platform set, the certified-presentation rule, complete linkage and
consumer refs, per-dependency consistency, required caveats, and the recomputed summary. The unit
tests in `crates/aureline-workspace/src/m5_bundle_scorecards/tests.rs` assert the embedded packet
validates clean and that every wedge, claimed class, effective class, confidence level, freshness
level, platform, and dependency lifecycle stage is exercised.

[`BundleScorecard`]: ../../../crates/aureline-workspace/src/m5_bundle_scorecards/mod.rs
[`BundleScorecardClass::rank`]: ../../../crates/aureline-workspace/src/m5_bundle_scorecards/mod.rs
[`BundleScorecard::computed_effective_class`]: ../../../crates/aureline-workspace/src/m5_bundle_scorecards/mod.rs
[`BundleScorecard::caveats_required`]: ../../../crates/aureline-workspace/src/m5_bundle_scorecards/mod.rs
[`BundleScorecard::linkage_complete`]: ../../../crates/aureline-workspace/src/m5_bundle_scorecards/mod.rs
