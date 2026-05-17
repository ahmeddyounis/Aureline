# Distributed compatibility beta manifests

Status: generated-data consumer
As-of: 2026-05-17
Owner: @ahmeddyounis

This page is the partner and release-review entrypoint for beta distributed
compatibility truth. It consumes generated manifest data; it does not restate a
private compatibility spreadsheet.

## Generated sources

- Manifest index: `artifacts/compat/m3/distributed_manifests/manifest_index.json`
- Client/helper manifest: `artifacts/compat/m3/distributed_manifests/client_helper.json`
- Client/extension manifest: `artifacts/compat/m3/distributed_manifests/client_extension.json`
- Schema manifest: `artifacts/compat/m3/distributed_manifests/schema.json`
- Provider manifest: `artifacts/compat/m3/distributed_manifests/provider.json`
- Skew harness report: `artifacts/compat/m3/distributed_manifests/skew_harness_report.json`
- Release packet: `artifacts/release/m3/distributed_compatibility/release_packet.json`
- Support export projection: `artifacts/release/m3/distributed_compatibility/support_export_projection.json`
- Harness fixtures: `fixtures/release/m3/skew_harness/manifest.yaml`
- Validator: `ci/check_m3_distributed_compatibility.py`

The generator reads `artifacts/compat/m3/compatibility_report.json`,
`artifacts/compat/skew_windows.yaml`, and
`artifacts/compat/version_skew_register.yaml`. A row that is not present in
those sources is not claim-bearing in the distributed manifests.

## Manifest families

| Family | Generated manifest | Claim boundary |
|---|---|---|
| `client_helper` | `artifacts/compat/m3/distributed_manifests/client_helper.json` | Launcher/local sidecar and desktop/CLI/remote-agent rows. |
| `client_extension` | `artifacts/compat/m3/distributed_manifests/client_extension.json` | Extension host, SDK, WIT ABI, and permission vocabulary rows. |
| `schema` | `artifacts/compat/m3/distributed_manifests/schema.json` | Profile/state, command descriptor, task-event, and additive schema rows. |
| `provider` | `artifacts/compat/m3/distributed_manifests/provider.json` | Provider API family, managed control plane, and browser-handoff rows. |

Each manifest row carries the compatibility row ref, the compatibility-report
row ref, declared negotiation fields, supported and unsupported skew cases,
upgrade order, rollback order, unsupported-state behavior, and repair hints.
Partner docs should quote those row ids directly rather than paraphrasing
support windows.

## Skew harness

The skew harness exercises supported and unsupported windows for every generated
family. The checked-in cases live under
`fixtures/release/m3/skew_harness/` and the generated report is
`artifacts/compat/m3/distributed_manifests/skew_harness_report.json`.

The required protected outcomes are:

- supported rows admit the bounded action described by the manifest row;
- unsupported rows block mutation and render a typed repair or continuation
  path;
- support export remains metadata-only and cites the same manifest row ids as
  release evidence.

## Release packet consumption

`artifacts/release/m3/distributed_compatibility/release_packet.json` is generated
from the manifest index and family manifests. It carries the family summaries,
supported and unsupported skew-case refs, harness totals, and a release policy
that downgrades stale or degraded proof rather than widening a beta claim.

`artifacts/release/m3/distributed_compatibility/support_export_projection.json`
is the support/export consumer. It quotes each manifest row, effective support
class, current skew case, unsupported-case refs, out-of-window posture, repair
hints, and release packet ref with `raw_private_material_excluded = true`.

## Refresh

Run:

```sh
python3 ci/check_m3_distributed_compatibility.py --repo-root .
```

Use `--check` in CI to fail when the generated manifests, release packet,
support projection, skew harness report, or validation capture drift from the
checked-in compatibility sources and harness fixtures.

