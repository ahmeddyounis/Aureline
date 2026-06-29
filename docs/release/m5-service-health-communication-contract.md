# M5 service-health communication contract

This contract freezes the typed service-health and **stale-or-mirrored release-data** packet the
service-health panel, Help/About, docs/help, support export, the admin console, and the release
center inspect to tell **which boundary is in trouble, how trustworthy the data on screen is, what
admin notes apply, and what still works locally** — without a single degraded service making the
whole product look broken. It is the continuity-communication layer alongside the
[update-center summary objects](m5-update-center-summary-contract.md), the
[change-impact cards](m5-change-impact-card-contract.md), and the
[support-window cards](m5-support-window-card-contract.md): those answer *what is changing* and *what
support a channel carries*; this one answers *which service boundary is healthy, how stale or
mirrored the data is, and what stays local-safe through an outage*.

Release and outage communication becomes unsafe when cached, mirrored, or policy-limited data looks
live, or when one degraded service makes the whole product look broken. This packet keeps the four
boundaries a user must distinguish separate, downgrades non-live data visibly with source-age truth,
and keeps local-only continuity explicit.

- Packet schema: [`schemas/release/m5-service-health-communication.schema.json`](../../schemas/release/m5-service-health-communication.schema.json)
- Published inventory: [`artifacts/release/m5-service-health-communication.json`](../../artifacts/release/m5-service-health-communication.json)
- Release-grade parity proof: `artifacts/release/m5-stale-release-data-proof/service-health-communication.json` (+ `.md`)
- Machine-readable per-card export: [`artifacts/release/m5-service-health-communication.csv`](../../artifacts/release/m5-service-health-communication.csv)
- Per-state fixtures: `fixtures/release/service-health-and-admin-notes/`
- Producer crate / module: `crates/aureline-release` → `m5_service_health_communication`
- Headless emitter: `aureline_release_m5_service_health_communication`

## What the cards cover

The packet carries two card families, both gate-bound to the shared
[descriptor/badge](../../crates/aureline-release/src/m5_descriptor_badge) vocabulary so every surface
reads one set of states.

### Service-tier health cards — one per boundary

| Tier | Identity | Affects local editing | Optional |
|------|----------|-----------------------|----------|
| `local_machine` | Your local editor and workspace; edits and recovery run here. | yes | no |
| `remote_target` | A remote development target you connected to. | no | no |
| `enterprise_control_plane` | Your organisation's managed policy and licensing plane. | no | no |
| `vendor_hosted_service` | Optional vendor-hosted services such as the update mirror. | no | yes |

Keeping the four tiers distinct is what lets a user tell a **local machine** issue apart from a
**remote target** issue, an **enterprise control-plane** issue, and an optional **vendor-hosted
service** issue, rather than collapsing every outage into one "service down" banner.

Every tier card carries:

- a **health state** (`operational`, `degraded`, `maintenance`, `unknown`, `outage`) — scoped to
  whether the tier can serve live release/update data, not a generic alert vocabulary;
- a **release-data state** of the data shown for it (`live_verified`, `mirrored`, `offline_cached`,
  `stale`, `policy_limited`, `local_only_safe`, `unavailable`) — every weaker state is a first-class
  token, so cached, mirrored, stale, or policy-limited data can never render as live;
- the **source-age truth** behind that data (`observed_at`, `as_of`, and a human-facing `age_label`),
  so a downgraded copy is exportable with its age;
- a **local-safe continuation statement** — whether local editing is safe, a message id naming what
  still works locally, and the recovery path (`continue_locally`, `use_mirror_copy`,
  `retry_when_reachable`, `reconnect_target`, `contact_admin`, `wait_for_maintenance`,
  `not_applicable`).

### Admin-note cards — one per propagated kind

| Note kind | Owner role |
|-----------|------------|
| `channel_change` | `release_channel_owner` |
| `mirror_change` | `mirror_owner` |
| `deployment_change` | `deployment_owner` |

An admin note about a channel, mirror, or deployment change is propagated using the **same
release-data vocabulary** and an export-safe `evidence_refs` path, with the affected tier and channel,
an effective-from date, source-age truth, and an `acknowledged` flag — so an admin note reads
identically on the UI, docs/help, and support exports.

## How a card's verdict is derived

A tier card's gate is the **worse of its two postures**, so a card can never make downgraded data look
live: `worst(health_state, release_data_state)`. An admin-note card's gate is the posture of the
release-data state it sets.

Each state maps to a gate: `operational` / `live_verified` → `governed`; `degraded` / `maintenance` /
`unknown` / `mirrored` / `offline_cached` / `stale` / `policy_limited` / `local_only_safe` →
`narrowed`; `outage` / `unavailable` → `blocked`. The gate maps one-to-one to a **readiness**:
`governed` → `live_trusted`, `narrowed` → `showing_downgraded`, `blocked` → `no_live_data`.

This is the lane's **guardrail** against over-stating freshness:
`ServiceHealthCommunication::validate` rejects any card whose stored gate is *less severe* than the
weakest posture warrants (`overstated_data_freshness`).

## Local-only continuity stays explicit

The **local machine is the only boundary** whose trouble can mark local editing unsafe; a remote,
control-plane, or vendor card is always local-safe (`local_editing_safe == true`), even in outage.
`validate` raises `misreported_local_continuity` if a non-local boundary is marked local-unsafe, or if
the local-machine card's flag disagrees with its health. The packet-level
`continuity.local_editing_safe` aggregates this, so **a managed or vendor outage never implies local
editing or recovery is unsafe** and no surface collapses to "everything broken".

## Cards under trouble carry continuation and recovery guidance

A card under any trouble (`narrowed` or `blocked`) must carry a continuation statement and a real
recovery path instead of a bare red banner, and `carries_recovery_guidance` records that it does.
`validate` raises `missing_continuation_guidance` for any pressured card that lacks it, and the schema
enforces the same with a `then: carries_recovery_guidance == true` guard for any `narrowed` /
`blocked` card.

## How consumers read the cards

Each consumer binds the tiers and admin notes it reads and **derives** its readiness, profiles, and
gaps from the cards — there is no hand-maintained per-consumer status. All six consumers
(`service_health_panel`, `help_about`, `docs_help`, `support_export`, `admin_console`,
`release_center`) read every tier and every admin note, and every consumer also carries the packet's
`local_continuation_safe` truth, so **admin notes and service-health messages stay consistent across
the UI, docs/help, and support exports** and no surface reads as fully broken when only a remote /
vendor boundary is out.

## Stale-data and boundary honesty

The packet-level `boundaries` summary names all four tiers at a glance — health, data state, and
local-safe flag — so the boundaries stay distinguishable in one row each. The `coverage` block
discloses `live_data_cards` versus `downgraded_data_cards` and `no_live_data_cards`, with
`has_data_downgrade` set when anything is not live. The packet `data_state` labels whether the cards
are `live_verified`, `mirrored_labelled`, `offline_cached`, `stale_banner_shown`, or
`local_only_no_live_data`. The `continuity` block lists the live, degraded, and outage tiers, the
affected boundaries, and the unacknowledged admin notes.

## Export safety

The packet carries metadata, refs, source-age labels, and message ids only — no credential bodies or
raw provider payloads, and no hidden operational data outside the existing redaction model — so the
service-health truth is exportable and reviewable outside the app. The JSON, the Markdown report, and
the per-card CSV all render byte-identically across the desktop, CLI / headless, and offline-export
channels.

## Drills

Three drills perturb the canonical (all-healthy) packet and let the derivation recompute every
consumer:

- `fixtures/release/service-health-and-admin-notes/health_vendor_outage.json` — the optional
  `vendor_hosted_service` boundary is in an outage with `unavailable` data (`blocked`), carrying a
  recovery path, so consumers read `no_live_data` **while local editing stays explicitly safe**;
- `fixtures/release/service-health-and-admin-notes/health_mirror_note.json` — the vendor boundary's
  data is downgraded to a labelled `mirrored` copy and an unacknowledged `mirror_change` admin note is
  propagated (`narrowed`), so consumers show downgraded data with source-age truth;
- `fixtures/release/service-health-and-admin-notes/health_local_only.json` — every remote boundary is
  offline (`blocked`) while the local machine stays `operational` and `live_verified`, rendered under a
  `local_only_no_live_data` data state, proving local work continues through a full remote outage.

## Regenerating

```sh
cargo run -q -p aureline-release --bin aureline_release_m5_service_health_communication -- registry  > artifacts/release/m5-service-health-communication.json
cargo run -q -p aureline-release --bin aureline_release_m5_service_health_communication -- proof     > artifacts/release/m5-stale-release-data-proof/service-health-communication.json
cargo run -q -p aureline-release --bin aureline_release_m5_service_health_communication -- markdown  > artifacts/release/m5-stale-release-data-proof/service-health-communication.md
cargo run -q -p aureline-release --bin aureline_release_m5_service_health_communication -- csv       > artifacts/release/m5-service-health-communication.csv
cargo run -q -p aureline-release --bin aureline_release_m5_service_health_communication -- variant canonical     > fixtures/release/service-health-and-admin-notes/health_all_operational.json
cargo run -q -p aureline-release --bin aureline_release_m5_service_health_communication -- variant vendor-outage > fixtures/release/service-health-and-admin-notes/health_vendor_outage.json
cargo run -q -p aureline-release --bin aureline_release_m5_service_health_communication -- variant mirror-note   > fixtures/release/service-health-and-admin-notes/health_mirror_note.json
cargo run -q -p aureline-release --bin aureline_release_m5_service_health_communication -- variant local-only    > fixtures/release/service-health-and-admin-notes/health_local_only.json
```
