# M5 public-handoff & capture-boundary certification

Generated from the seeded packet in
[`crate::m5_public_handoff_certification`](../../crates/aureline-shell/src/m5_public_handoff_certification/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_public_handoff_certification -- markdown > \
  artifacts/help/m5-public-handoff-certification.md
```

- Packet id: `m5-public-handoff-certification:stable:0001`
- Source schema ref: `schemas/help/m5-public-handoff-certification.schema.json`
- Certifies matrix packet: `m5-public-handoff-matrix:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Rows certified: 8
- Green (fully proven): 6
- Yellow (auto-narrowed): 2
- Red (blocked): 0
- All rows publishable: `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Boundary-truth rows

| Certified surface | Status | Qualification | Disclosure freshness | Boundary honesty | Redaction readiness | Waiver |
| ----------------- | ------ | ------------- | -------------------- | ---------------- | ------------------- | ------ |
| Post-install notice / provenance card | `green` | `stable` | `proven_current` | `honestly_disclosed` | `proven` | — |
| Provenance / source-authenticity disclosure | `green` | `stable` | `proven_current` | `honestly_disclosed` | `proven` | — |
| Official-vs-community handoff route | `green` | `stable` | `proven_current` | `honestly_disclosed` | `proven` | — |
| Redaction-safe reproduction packet | `green` | `stable` | `proven_current` | `honestly_disclosed` | `proven` | — |
| Offline-capture continuity | `green` | `stable` | `proven_current` | `honestly_disclosed` | `proven` | — |
| Device / mic permission boundary | `yellow` | `beta` | `proven_current` | `honestly_disclosed` | `proven` | — |
| Embedded webview / auth boundary | `yellow` | `beta` | `proven_current` | `disclosed_gap` | `proven` | `waiver:embedded-boundary-label-sync:0001` |
| Release / service-health notice | `green` | `stable` | `proven_current` | `honestly_disclosed` | `proven` | — |

## Auto-narrowed rows

- `device_permission_boundary` (`yellow`) — The device/mic permission boundary is qualified at Beta in the frozen public-handoff matrix; capture ships with a disclosed Beta posture and is narrowed below a Stable public claim.
- `embedded_auth_boundary` (`yellow`) — The embedded webview / auth boundary is qualified at Beta; the embedded surface and the system-browser handoff disclose a known chrome-wording gap that is waivered pending the next cross-surface boundary-copy sync.

## Exact stale-proof causes

- `device_permission_boundary` — `upstream_dependency_narrowed` (disclosed: `true`) — Frozen public-handoff matrix qualifies this object at `beta`, below a Stable public claim.
- `embedded_auth_boundary` — `upstream_dependency_narrowed` (disclosed: `true`) — Frozen public-handoff matrix qualifies this object at `beta`, below a Stable public claim.
- `embedded_auth_boundary` — `native_chrome_impersonation` (disclosed: `true`) — Disclosed boundary-labeling gap across consumer surfaces, held under a waiver.

## Active waivers

- `waiver:embedded-boundary-label-sync:0001` (`embedded_auth_boundary`, owner: Browser/auth boundary owner, expires `2026-09-30T00:00:00Z`) — The embedded webview and the system-browser handoff both label the external origin and route trust class, but the exact chrome wording is being unified in the next cross-surface boundary-copy sync. The difference is disclosed, never hidden, and no surface impersonates native chrome.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_public_handoff_certification -- validate
cargo test -p aureline-shell --test m5_public_handoff_certification_fixtures
```
