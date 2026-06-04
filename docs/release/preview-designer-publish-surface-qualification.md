# Preview, designer, and publish surface qualification

This document explains the canonical packet:

- packet: [`artifacts/release/m4/preview-designer-publish-surface-qualification.json`](../../artifacts/release/m4/preview-designer-publish-surface-qualification.json)
- proof packet: [`artifacts/release/m4/preview-designer-publish-surface-qualification.md`](../../artifacts/release/m4/preview-designer-publish-surface-qualification.md)
- schema: [`schemas/release/preview-designer-publish-surface-qualification.schema.json`](../../schemas/release/preview-designer-publish-surface-qualification.schema.json)

The packet is canonical for preview runtime, device/viewport preview, visual
designer, share/export, and publish/deploy surfaces. Downstream docs, Help/About
surfaces, release packets, product copy, and support exports must ingest the
packet by `surface_id` rather than restating labels in prose.

## Stable rule

A row can render at Stable only when all of these are true:

- the row has a captured current `qualification_packet`;
- `source_mapping_quality` is `canonical_source_mapping`;
- `source_sync_state` is `in_sync`;
- generated-versus-source truth is visible before trust or export;
- safe preview is available by default or by explicit review;
- side-effectful rows expose dry run or closest safe preview plus
  preview/apply/revert lineage;
- source, diff, and rollback fallback paths remain available;
- browser-runtime inspection depth is explicitly governed elsewhere.

Rows that cannot meet those conditions render below Stable. This prevents a
preview runtime or designer canvas from becoming a hidden second source of truth
and prevents publish helpers from behaving like one-click external mutation.

## Current posture

The source-mapped preview runtime is the only Stable row in this packet. Device
preview, visual designer, share/export, and publish/deploy rows are intentionally
narrowed until each has family-specific proof for canonical mappings,
round-trip-safe editing, downstream artifact truth, and side-effect review.

## Browser-runtime boundary

Preview runtime qualification may share runtime identity with browser-runtime
records, but it does not qualify DOM/CSS inspection, console, network/storage,
source-map drift handling, or live runtime mutation. Those remain governed by
[`docs/runtime/browser_runtime_contract.md`](../runtime/browser_runtime_contract.md).

## Packet-freshness SLO

Green rows use a 180-day max age and a 30-day warning window. Stale or missing
packets narrow below Stable until refreshed and owner-signed.

## Verification

Run:

```sh
cargo test -p aureline-release --test preview_designer_publish_surface_qualification
```

The test validates the checked-in packet, recomputes summary counts, confirms
visible generated/source truth and browser-runtime boundaries, checks support
export projection, and runs negative fixture drills.

