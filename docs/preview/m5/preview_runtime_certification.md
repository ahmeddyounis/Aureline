# M5 source-first preview / browser-runtime release certification

This document is the contract for the M5 preview/runtime **release certification**
packet. It is the auto-narrowing gate that decides whether each claimed M5 framework
/ preview / browser-runtime row may ship at the depth it claims: a row is
release-certified only when every proof lane release requires for it is currently
backed by fresh evidence, and a row whose proof has gone stale or missing
auto-narrows below its claim and blocks promotion.

Where each earlier module in this lane owns one preview/runtime truth lane —
[source-first preview sessions](preview_session_descriptors.md),
[inspect-to-source mapping](inspect_to_source_tree_mapping.md),
[browser-runtime inspectors](browser_runtime_inspectors.md),
[visual-edit transforms](visual_edit_transforms.md),
[drift/recovery drills](preview_drift_recovery_drills.md), and
[extension-provider conformance](extension_provider_conformance.md) — and the
[source-first preview / browser-runtime inspection matrix](freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix.md)
freezes the *qualification* of each claimed surface, this packet **rolls those lanes
up into one release-bearing certification** per claimed row. Product, docs/help,
diagnostics, provider conformance, and release surfaces ingest this single
certification result instead of re-narrating preview/runtime maturity by hand.

Source remains canonical; the certification packet is derivative — never a second
writable truth model.

## Source of truth

- Packet type: `PreviewRuntimeCertificationPacket`
  (`crates/aureline-preview/src/preview_runtime_certification/`).
- Boundary schema:
  `schemas/preview/preview_runtime_certification.schema.json`.
- Checked support export:
  `artifacts/preview/m5/preview_runtime_certification/support_export.json`.
- Markdown summary:
  `artifacts/preview/m5/preview_runtime_certification.md`.
- Protected fixtures:
  `fixtures/preview/m5/preview_runtime_certification/`.
- Certification dump:
  `cargo run -p aureline-preview --example dump_m5_preview_runtime_certification [support|summary]`.

## Certification lanes

Each row lists the `CertificationLane`s release requires for its surface and carries
one `LaneProof` per lane. Every lane binds to the canonical upstream B33 schema it
rolls up, so a lane proof ingests that lane's truth rather than re-deriving it:

| Lane | Canonical schema |
| --- | --- |
| `source_first_preview` | `schemas/preview/preview_session_descriptor_set.schema.json` |
| `inspect_to_source_fidelity` | `schemas/preview/inspect_to_source_tree_mapping.schema.json` |
| `browser_runtime_inspection` | `schemas/preview/browser_runtime_inspectors.schema.json` |
| `round_trip_honesty` | `schemas/preview/visual_edit_transforms.schema.json` |
| `drift_recovery` | `schemas/preview/preview_drift_recovery_drill_set.schema.json` |
| `provider_conformance` | `schemas/preview/extension_provider_conformance.schema.json` |

A `LaneProof.source_lane_ref` MUST equal `CertificationLane::canonical_schema_ref`
for its lane, and its `status` is one of `current`, `stale`, `missing`, or
`not_applicable`. A `current` or `stale` proof MUST carry a `last_refresh`
timestamp.

## Auto-narrowing release gate

A row's `claimed_certification` is its public release claim
(`certified` > `beta` > `preview` > `held` > `blocked`). The gate computes the
`effective_certification`:

- When **every** release-required lane is `current`, the effective certification
  equals the claim and `promotion_blocked` is `false`.
- When **any** release-required lane is `stale` or `missing` (a *blocking gap*), the
  effective certification MUST rank strictly below the claim, the row MUST record a
  `narrow_trigger` and a precise non-generic `degraded_label`, and
  `promotion_blocked` MUST be `true`.

Specific regressions are recorded as the trigger so the chrome can quote them
verbatim instead of a generic error: `stale_source_map`, `unlabeled_runtime_target`,
`weak_provider_replacement`, `hidden_inspect_only_fallback`, `stale_lane_proof`,
`missing_lane_proof`, `upstream_lane_narrowed`, and `policy_narrowed`.

A write-capable claim (`claims_write_capable`) appears only when the
`round_trip_honesty` lane is currently proven **and** the row is not narrowed, so an
inspect-only or degraded row is never auto-upgraded into a write-capable designer
flow.

## Guardrails

`PreviewRuntimeCertificationPacket::validate` enforces that the packet:

- keeps source canonical and never mints a second writable truth model;
- never lets runtime state or extension-private wording hide source-mapping
  uncertainty behind a certified label;
- never auto-upgrades an inspect-only row into a write-capable designer flow;
- never blurs embedded preview/browser boundaries into product-native authority;
- auto-narrows any claimed row that lacks current proof on a required lane;
- blocks promotion (or visibly narrows) on stale-source-map, unlabeled-runtime-target,
  weak-provider-replacement, and hidden-inspect-only-fallback regressions; and
- exposes one certification result that product, docs/help, diagnostics, provider
  conformance, and release surfaces ingest rather than cloning maturity text.

## Coverage

The checked packet covers all eight claimed `PreviewSurface`s, requires all six
certification lanes across its rows, and demonstrates the gate with both a
stale-source-map narrowed row (`beta` → `held`, blocked) and a missing-proof blocked
row (`beta` → `blocked`).
