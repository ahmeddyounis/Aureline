# Maintenance-control packet — hotfix, backport, correction-train, support-window

This document is the reviewer-facing companion for the gated maintenance-control
packet:

- [`/artifacts/release/maintenance_control_packet.json`](../../artifacts/release/maintenance_control_packet.json)
- schema: [`/schemas/release/maintenance_control_packet.schema.json`](../../schemas/release/maintenance_control_packet.schema.json)
- proof packet:
  [`/artifacts/release/m4/maintenance_control_packet_proof_packet.md`](../../artifacts/release/m4/maintenance_control_packet_proof_packet.md)

The packet is the **canonical truth** for whether each post-release maintenance lane —
an emergency hotfix lane, a supported-line backport lane, a planned correction-train
lane, or a support-window commitment — is actually **governed** for the release line.
The other stable launch-control artifacts answer adjacent questions — the
[stable claim manifest](./stable_claim_manifest.md) decides the single canonical label
each *subject* publishes, the [stable proof index](./stable_proof_index.md) decides
whether each launch-blocking *requirement* is proven, and the
[stable version-window freeze](./stable_version_windows.md) freezes each interface
surface's version window. This packet answers the maintenance question: **for each
maintenance lane, is the lane staffed and the support window held — backed by a fresh
control packet, a complete and unexpired support window, and an owner sign-off?**
Downstream dashboards, docs, Help/About surfaces, release packets, and support exports
MUST ingest this packet by `control_id` and render its `controlled_label`,
`support_window`, and `control_state` rather than minting their own per-lane support or
maturity wording.

It is the publication layer over the shared correction-train packet *form* in
[`correction_train`](../../crates/aureline-release/src/correction_train/mod.rs): each
row binds its lane to the canonical correction-train packet via `correction_packet_ref`
and to the support-window contract via the packet's `support_window_contract_ref`,
instead of cloning their status into a side spreadsheet.

## Lanes, support windows, control packets, claims — one row each

Each `row` is one `(lane, public claim)` binding. It names:

- the maintenance **lane** it governs — `lane_kind` (`hotfix`, `backport`,
  `correction_train`, or `support_window`), `lane_ref`, `lane_summary`, and whether it
  is `release_blocking`;
- the **support window** the lane commits to — `support_window` (`opened_at`,
  `review_through_date`, `end_of_support_date`, `support_posture`);
- the **control packet** that proves the lane is staffed — `control_packet` (id, packet
  ref, the stable-proof-index registration ref, captured-at date, freshness SLO, SLO
  state, and evidence refs);
- the shared **correction-train packet** it rides — `correction_packet_ref`;
- the **waiver** (if any) that holds it provisionally — `waiver`;
- the public **claim** it backs — `claim_ref` (a stable-claim-manifest entry) and
  `claim_label`, the canonical lifecycle label that entry publishes.

## The claim ceiling — no per-lane widening

`claim_label` is a **hard ceiling**: a row may govern the public claim at its label or
narrow below it, but its `controlled_label` may never be **wider** (stronger) than the
public claim's canonical label. This is what makes the packet *ingest* the claim
manifest rather than restate it — the CI gate reads the stable claim manifest named by
`claim_manifest_ref` and fails when a row's `claim_label` is not the label the claim
manifest publishes for the entry named by `claim_ref`. The packet reuses the stable
claim level vocabulary — `lts`, `stable`, `beta`, `preview`, `withdrawn` — rather than
minting per-lane labels.

## The launch cutline

The cutline fixes the boundary between a lane whose control backs a Stable (or LTS)
claim and one narrowed below it:

```
lts > stable   |   beta > preview > withdrawn   (below the cutline)
              cutline
```

A lane governs a Stable (or LTS) maintenance promise only when **all** of the
following hold: its control packet is within its freshness SLO, any waiver it relies on
is unexpired, its support window is complete and not past its end-of-support date, an
owner has signed off, and the public claim it backs is itself at or above the cutline.
A lane that loses any of those is structurally required to drop its `controlled_label`
**below** the cutline (`beta`, `preview`, `withdrawn`); it never inherits an adjacent
governed lane's label.

## Control states

| `control_state` | Meaning | Governs the claim's label? |
|---|---|---|
| `governed` | Fresh control packet, complete unexpired support window, owner-signed | yes |
| `governed_on_waiver` | Holds the label only via an active, unexpired waiver | yes |
| `ungoverned_unbacked` | Lane evidence or support window incomplete, or owner sign-off absent | no — narrows |
| `ungoverned_claim_narrowed` | The backing public claim is itself below the cutline | no — inherits ceiling |
| `ungoverned_stale` | The control packet breached its freshness SLO or is missing | no — narrows |
| `ungoverned_waiver_expired` | The waiver the lane relied on expired | no — narrows |
| `ungoverned_support_expired` | The support window passed its end-of-support date without renewal | no — narrows |

## Control-packet freshness SLO

Each row's `control_packet` carries a `freshness_slo` — a `target_max_age_days`, a
`warn_within_days` threshold, and an `slo_register_ref` — plus a recorded `slo_state`
(`current`, `due_for_refresh`, `breached`, or `missing`). The CI gate recomputes the
state from the packet's `captured_at` against the packet `as_of` date and fails when a
declared state is **fresher** than the clock allows, or when a `governed`/
`governed_on_waiver` lane rides a packet that is `breached` or `missing`. A lane whose
control packet ages past its SLO narrows automatically before publication — the freeze
cannot rest on a stale staffing proof.

## Waiver expiry and support-window expiry

Two further date automations live in the gate:

- **Waiver expiry.** A lane in `governed_on_waiver` whose `waiver.expires_at` has
  passed against `as_of` is rejected; a lane in `ungoverned_waiver_expired` whose
  waiver is still active is also rejected. A waiver narrows the lane the moment it
  lapses.
- **Support-window expiry.** A lane whose `support_window.end_of_support_date` has
  passed against `as_of` while the window still claims active support (its
  `support_posture` is not `end_of_life_scheduled`) must name `support_window_expired`
  and narrow; a `governed` lane carrying such a window is rejected. A window formally
  moved to `end_of_life_scheduled` is expected to pass its end-of-support date and is
  not flagged.

## Coverage

The packet must cover all four lane kinds (`hotfix`, `backport`, `correction_train`,
`support_window`); every declared `release_blocking_lane_refs` entry must have exactly
one covering release-blocking row; every release-blocking row must be declared; and no
`lane_ref` may repeat. A maintenance kind cannot quietly drop out of the packet.

## Publication gate

`publication` records the proceed/hold verdict for the
`maintenance_control_packet_publication` gate. Each `control_rule` names a closed gap
reason it watches, the labels it applies to (`lts`, `stable`), a default action, and
whether it `blocks_publication`. The verdict is `hold` when any blocking rule fires —
that is, when a row whose public claim is still at or above the cutline carries the
rule's trigger reason. `ci/check_maintenance_control_packet.py --require-proceed`
exits non-zero on `hold`, so shiproom and release tooling can block maintenance
publication directly from this artifact. A lane whose backing claim is already narrowed
below the cutline inherits that ceiling and does **not** hold publication on its own —
that narrowing is owned upstream by the stable claim manifest.

## Why this is not a spreadsheet

The packet is metadata-only: typed states and opaque refs, never raw artifacts, logs,
signatures, or credentials. The typed Rust consumer
(`aureline_release::maintenance_control_packet`) and the CI gate read the *same* JSON,
so the model and the gate agree without a cargo build in CI. Any surface that needs to
show maintenance posture renders `MaintenanceControlPacket::support_export_projection`
rather than re-deriving status — there is exactly one place the truth lives.
