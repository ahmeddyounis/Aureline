# Stable publication pack — known-limits, public benchmark, compatibility, migration

This document is the reviewer-facing companion for the gated stable publication pack:

- [`/artifacts/release/stable_publication_pack.json`](../../artifacts/release/stable_publication_pack.json)
- schema: [`/schemas/release/stable_publication_pack.schema.json`](../../schemas/release/stable_publication_pack.schema.json)
- proof packet:
  [`/artifacts/release/m4/stable_publication_pack_proof_packet.md`](../../artifacts/release/m4/stable_publication_pack_proof_packet.md)

The pack is the **canonical truth** for whether each outward-facing publication the
release line ships about its own limits and behavior — a known-limits publication, a
public benchmark publication, a compatibility publication, or a migration publication —
is actually **backed** for the stable line. The other stable launch-control artifacts
govern what the release line *is*: the
[stable claim manifest](./stable_claim_manifest.md) decides the single canonical label
each *subject* publishes, the [stable proof index](./stable_proof_index.md) decides
whether each launch-blocking *requirement* is proven, the
[stable version-window freeze](./stable_version_windows.md) freezes each interface
surface's version window, and the [maintenance-control packet](./maintenance_control_packet.md)
governs each post-release maintenance lane. This pack answers the publication question:
**for each publication, is it backed by a fresh proof packet, within its published
p50/p95 budget where it makes a performance claim, and owner-signed?** Downstream
dashboards, docs, Help/About surfaces, release packets, and support exports MUST ingest
this pack by `entry_id` and render its `published_label`, `publication_state`, and
`benchmark_budget` rather than minting their own per-publication caveat, benchmark, or
maturity wording.

It is the publication layer over the four upstream source contracts: each row binds its
publication to the known-limits register (`known_limits_register_ref`), the
benchmark-publication pack template (`benchmark_pack_template_ref`), the
compatibility-report template (`compatibility_report_template_ref`), or the migration
contract (`migration_contract_ref`), instead of cloning their status into a side
spreadsheet.

## Publications, proof packets, budgets, claims — one row each

Each `row` is one `(publication, public claim)` binding. It names:

- the **publication** it governs — `publication_kind` (`known_limit`, `benchmark`,
  `compatibility`, or `migration`), `surface_ref`, `surface_summary`, and whether it is
  `release_blocking`;
- the **proof packet** that grounds it — `proof_packet` (id, packet ref, the
  stable-proof-index registration ref, captured-at date, freshness SLO, SLO state, and
  evidence refs);
- for a benchmark publication, the **benchmark budget** it must hold —
  `benchmark_budget` (`metric_ref`, `published_p50_ms`, `published_p95_ms`,
  `measured_p50_ms`, `measured_p95_ms`, `corpus_ref`, `trace_ref`, `tightened`);
- the **waiver** (if any) that holds it provisionally — `waiver`;
- the public **claim** it backs — `claim_ref` (a stable-claim-manifest entry) and
  `claim_label`, the canonical lifecycle label that entry publishes.

## The claim ceiling — no per-publication widening

`claim_label` is a **hard ceiling**: a row may carry the public claim at its label or
narrow below it, but its `published_label` may never be **wider** (stronger) than the
public claim's canonical label. This is what makes the pack *ingest* the claim manifest
rather than restate it — the CI gate reads the stable claim manifest named by
`claim_manifest_ref` and fails when a row's `claim_label` is not the label the claim
manifest publishes for the entry named by `claim_ref`. The pack reuses the stable claim
level vocabulary — `lts`, `stable`, `beta`, `preview`, `withdrawn` — rather than minting
per-publication labels.

## The launch cutline

The cutline fixes the boundary between a publication backing a Stable (or LTS) claim and
one narrowed below it:

```
lts > stable   |   beta > preview > withdrawn   (below the cutline)
              cutline
```

A publication carries a Stable (or LTS) public claim only when **all** of the following
hold: its proof packet is within its freshness SLO, any waiver it relies on is unexpired,
its benchmark measurements (where it makes a performance claim) are within the published
p50/p95 budget with corpus metadata and a benchmark-lab trace, an owner has signed off,
and the public claim it backs is itself at or above the cutline. A publication that loses
any of those is structurally required to drop its `published_label` **below** the cutline
(`beta`, `preview`, `withdrawn`); it never inherits an adjacent backed publication's
label.

## Publication states

| `publication_state` | Meaning | Carries the claim's label? |
|---|---|---|
| `published` | Fresh proof packet, benchmark within budget, owner-signed | yes |
| `published_on_waiver` | Holds the label only via an active, unexpired waiver (e.g. an intentionally tightened budget) | yes |
| `narrowed_unbacked` | Row evidence incomplete, corpus/trace missing, capability absent, or owner sign-off absent | no — narrows |
| `narrowed_claim_narrowed` | The backing public claim is itself below the cutline | no — inherits ceiling |
| `narrowed_stale` | The proof packet breached its freshness SLO or is missing | no — narrows |
| `narrowed_waiver_expired` | The waiver the publication relied on expired | no — narrows |
| `narrowed_budget_regressed` | A benchmark's measured p50/p95 regressed beyond the published budget | no — narrows |

## Protected benchmark budgets

A `benchmark` publication MUST carry a `benchmark_budget`; a non-benchmark publication
MUST NOT. The budget records the **published** p50/p95 ceilings (the public promise), the
**measured** p50/p95 from the benchmark-lab trace, the `corpus_ref` (corpus metadata) and
`trace_ref` (the lab trace), and whether the threshold is intentionally `tightened`. The
budget protects the published numbers:

- a benchmark whose measured p50 or p95 **exceeds** the published budget narrows to
  `narrowed_budget_regressed` and names `budget_regressed`;
- a benchmark missing its `corpus_ref` or `trace_ref` narrows and names
  `corpus_metadata_missing` — an untraceable benchmark cannot carry a Stable claim;
- a benchmark whose budget is intentionally `tightened` below a prior baseline and whose
  measured numbers have not yet caught up may hold its label provisionally under an active
  waiver (`published_on_waiver`); the waiver narrows it automatically the moment it lapses.

## Proof-packet freshness SLO

Each row's `proof_packet` carries a `freshness_slo` — a `target_max_age_days`, a
`warn_within_days` threshold, and an `slo_register_ref` — plus a recorded `slo_state`
(`current`, `due_for_refresh`, `breached`, or `missing`). The CI gate recomputes the
state from the packet's `captured_at` against the pack `as_of` date and fails when a
declared state is **fresher** than the clock allows, or when a `published`/
`published_on_waiver` row rides a packet that is `breached` or `missing`. A publication
whose proof packet ages past its SLO narrows automatically before publication — the
publication cannot rest on a stale proof.

## Waiver expiry

A row in `published_on_waiver` whose `waiver.expires_at` has passed against `as_of` is
rejected; a row in `narrowed_waiver_expired` whose waiver is still active is also
rejected. A waiver narrows the publication the moment it lapses.

## Coverage

The pack must cover all four publication kinds (`known_limit`, `benchmark`,
`compatibility`, `migration`); every declared `release_blocking_surface_refs` entry must
have exactly one covering release-blocking row; every release-blocking row must be
declared; and no `surface_ref` may repeat. A publication kind cannot quietly drop out of
the pack.

## Publication gate

`publication` records the proceed/hold verdict for the `stable_publication_pack`
publication gate. Each `publication_rule` names a closed gap reason it watches, the labels
it applies to (`lts`, `stable`), a default action, and whether it `blocks_publication`.
The verdict is `hold` when any blocking rule fires — that is, when a row whose public
claim is still at or above the cutline carries the rule's trigger reason.
`ci/check_stable_publication_pack.py --require-proceed` exits non-zero on `hold`, so
shiproom and release tooling can block publication directly from this artifact. A
publication whose backing claim is already narrowed below the cutline inherits that
ceiling and does **not** hold publication on its own — that narrowing is owned upstream by
the stable claim manifest.

## Why this is not a spreadsheet

The pack is metadata-only: typed states, measured/published budget numbers, and opaque
refs, never raw artifacts, logs, signatures, or credentials. The typed Rust consumer
(`aureline_release::stable_publication_pack`) and the CI gate read the *same* JSON, so the
model and the gate agree without a cargo build in CI. Any surface that needs to show
publication posture renders `StablePublicationPack::support_export_projection` rather than
re-deriving status — there is exactly one place the truth lives.
