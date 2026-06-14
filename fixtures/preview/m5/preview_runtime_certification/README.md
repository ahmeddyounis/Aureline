# M5 Source-First Preview / Browser-Runtime Release Certification Fixtures

## certification_rollup_gate_and_narrowing.json

A rollup, auto-narrowing, and release-gate drill fixture for the preview/runtime
certification packet. The eight rows cover every claimed preview/runtime surface in
the frozen `PreviewSurface` vocabulary and, between them, require all six
certification lanes — source-first preview, inspect-to-source fidelity,
browser-runtime inspection, round-trip honesty, drift/recovery, and provider
conformance.

Each row lists the lanes release requires for its surface and carries one lane proof
per lane bound to the canonical upstream B33 lane schema it rolls up (so the
certification ingests each lane's truth instead of re-narrating it).

The packet demonstrates the certification rules:

- A **release-certified source-first framework preview** whose four required lanes
  are all currently proven; it claims `certified` and stays `certified`.
- **Beta-certified** mapping, browser-runtime inspection, and device/simulator rows
  whose required lanes are all current.
- A **preview-certified** embedded webview row.
- A **release-certified, write-capable** visual-edit-transform row — write capability
  appears only because its round-trip-honesty lane is currently proven and the row is
  not narrowed.
- A **narrowed and blocked** full-stack preview loop whose source map went stale: the
  source-first-preview lane proof is `stale`, so the claim auto-narrows from `beta` to
  `held`, records a `stale_source_map` trigger and a precise degraded label, and
  `promotion_blocked` is `true`.
- A **blocked** support/export projection whose provider-conformance lane proof is
  `missing` entirely: the claim auto-narrows from `beta` to `blocked` with a
  `missing_lane_proof` trigger.

Every narrowed or blocked row carries an explicit narrowing trigger, a precise
non-generic degraded label, and `promotion_blocked = true`; every fully-proven row
carries none. No narrowed row claims write capability.

The fixture validates against
`schemas/preview/preview_runtime_certification.schema.json` and is byte-aligned with
the in-crate builder via
`cargo run -p aureline-preview --example dump_m5_preview_runtime_certification`.
