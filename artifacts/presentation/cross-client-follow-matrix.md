# Cross-client presentation follow matrix

- Packet: `presentation-cross-client-follow-matrix:stable:0001`
- Label: `Cross-client presentation follow / breakaway / snapshot matrix`
- Clients: desktop, browser, companion
- Follow modes: 6 / 6 (`presenting`, `following_live`, `broken_away`, `requesting_follow`, `requesting_take_over`, `cached_snapshot`)
- Liveness classes: 3 / 3 (`live`, `independent`, `cached_snapshot`)
- Source of truth: `aureline-shell::presentation::follow_state` seed corpus
- Fixtures: `fixtures/presentation/browser-and-companion-follow/`
- Schema: `schemas/presentation/follow-state-truth.schema.json`
- Contract: `docs/ux/presentation-follow-and-breakaway.md`

This matrix is a human-readable projection of the seeded follow-state corpus. It
shows, per scenario and per claimed client, the explicit follow mode, the
liveness a viewer reads, whether a durable breakaway banner or a self-labeled
cached snapshot is present, and the recovery actions offered. The vocabulary and
recovery actions are identical across clients by construction; the machine packet
asserts the parity, durability, and non-inference guardrails.

## Coverage

| Invariant                                              | Holds |
| ------------------------------------------------------ | ----- |
| Same follow vocabulary on every client                 | yes   |
| Same recovery actions on every client (canonical)      | yes   |
| Every breakaway banner is durable (not a toast)        | yes   |
| No state inferred from viewport drift                  | yes   |
| No state inferred from connection timing               | yes   |
| No state carried by a transient toast alone            | yes   |
| No cached snapshot claims to be a live shared route    | yes   |
| No follow state widens mutation or control authority   | yes   |

## Scenarios

### `follow-case:all-live-cross-client`

A live session observed from every client.

| Client    | Follow mode      | Liveness | Banner | Snapshot | Recovery actions |
| --------- | ---------------- | -------- | ------ | -------- | ---------------- |
| desktop   | `presenting`     | live     | —      | —        | —                |
| browser   | `following_live` | live     | —      | —        | —                |
| companion | `following_live` | live     | —      | —        | —                |

### `follow-case:mixed-independent`

The browser breaks away behind a durable banner; the companion has requested
follow and is waiting to resync.

| Client    | Follow mode         | Liveness    | Banner   | Snapshot | Recovery actions      |
| --------- | ------------------- | ----------- | -------- | -------- | --------------------- |
| desktop   | `presenting`        | live        | —        | —        | —                     |
| browser   | `broken_away`       | independent | durable  | —        | return to presenter   |
| companion | `requesting_follow` | independent | —        | —        | return to presenter   |

### `follow-case:companion-cached-snapshot`

The provider went offline for the companion, which now shows a self-labeled
cached snapshot while the desktop presents and the browser follows live.

| Client    | Follow mode      | Liveness        | Banner | Snapshot                         | Recovery actions                   |
| --------- | ---------------- | --------------- | ------ | -------------------------------- | ---------------------------------- |
| desktop   | `presenting`     | live            | —      | —                                | —                                  |
| browser   | `following_live` | live            | —      | —                                | —                                  |
| companion | `cached_snapshot`| cached snapshot | —      | labeled snapshot, `provider_offline` | refresh live, return to presenter |

### `follow-case:browser-take-over-request`

A browser co-presenter explicitly requests take-over while still seeing the live
route — a distinct, attributable state, not an inferred control grab.

| Client    | Follow mode            | Liveness | Banner | Snapshot | Recovery actions    |
| --------- | ---------------------- | -------- | ------ | -------- | ------------------- |
| desktop   | `presenting`           | live     | —      | —        | —                   |
| browser   | `requesting_take_over` | live     | —      | —        | return to presenter |
| companion | `following_live`       | live     | —      | —        | —                   |
