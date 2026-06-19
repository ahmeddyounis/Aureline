# Anchor Remap and Diagnostic Drift

When a finding is reported, it is anchored to a place in your work — a range in a
file, a region of a notebook cell, a span of a generated artifact, or a location
named by an imported scan. That place can **move**: you edit the file, a notebook
cell is split or re-keyed, a generator regenerates the artifact, or an imported
snapshot is compared against a later revision of your code. When the anchor moves,
Aureline does **not** silently drop the finding, quietly "fix" its location, or
relabel it as something new. Instead it records the move as explicit,
append-only **anchor-remap** evidence.

This page describes the M5 anchor-remap history set, which gives every moved
finding a causal trail across the file-edit, notebook-cell, generated-artifact,
and imported-snapshot/replay lanes — using one shared vocabulary rather than a
feature-specific drift state per surface.

- Record kind: `m5_anchor_remap_history_set`
- Packet type: `AnchorRemapHistorySetPacket`
  (`crates/aureline-runtime/src/record_anchor_remap_history_with_revision_pairs_and_drift_states_across_m5_lanes/`)
- Boundary schema: [`schemas/quality/anchor-remap-record.schema.json`](../../schemas/quality/anchor-remap-record.schema.json)
- Canonical record schema (sat above, not replaced): [`schemas/quality/diagnostic-record.schema.json`](../../schemas/quality/diagnostic-record.schema.json)
- Checked support export: [`artifacts/m5/diagnostics/anchor-remap-proof/support_export.json`](../../artifacts/m5/diagnostics/anchor-remap-proof/support_export.json)
- Summary artifact: [`artifacts/m5/diagnostics/anchor-remap-proof/support_export.md`](../../artifacts/m5/diagnostics/anchor-remap-proof/support_export.md)
- Fixtures: [`fixtures/quality/m5/anchor-remap/`](../../fixtures/quality/m5/anchor-remap/)
- Loader: `aureline_runtime::record_anchor_remap_history_with_revision_pairs_and_drift_states_across_m5_lanes::current_m5_anchor_remap_history_set_export`
- Conformance dump: `cargo run -p aureline-runtime --example dump_m5_anchor_remap_history`

## The five remap states

A finding's current anchor always carries one of five remap states. These are the
same five states the canonical
[diagnostic record](./diagnostic-records.md) already uses, reused unchanged so a
notebook change and an imported scan do not invent their own drift vocabulary:

| State | Meaning |
| --- | --- |
| `exact` | The anchored range survived unchanged; the finding still maps cleanly. |
| `contextual` | The range was re-located by matching surrounding context after a change. |
| `stale` | No newer mapping was found; the finding is retained against an older revision. |
| `unmapped` | The anchor could not be located; the finding has no current range (but is **not** discarded). |
| `imported_static` | An imported snapshot's static location that has not been locally revalidated. |

Any state other than `exact` is disclosed in the editor, Problems, review, and
CLI surfaces so you can tell at a glance whether a finding still maps cleanly or
only contextually survives.

## The append-only history

Every anchor family carries an **append-only history**: an ordered list of remap
entries that never rewrites the past. Each entry records:

- the **old anchor ref** and the **new anchor ref** (the new ref is absent when the
  finding became `unmapped`);
- the **resulting remap state**, derived from a typed **evidence basis** — so a
  finding can never claim it maps `exact` again without `exact_range_preserved`
  evidence. This is the *no silent repair* guarantee;
- a **revision pair** naming the from/to revisions the move spanned;
- the **drift lane** that produced the move (file edit, notebook cell identity
  change, generated-artifact churn, imported snapshot comparison, or imported
  replay comparison); and
- the **actor/tool** that produced the remap.

Entries are sequence-ordered with contiguous numbers, each entry's from-revision
continues the prior entry's to-revision, and each entry's old anchor continues the
prior entry's new anchor. That continuity is what makes the trail auditable: a
support or review reader can replay exactly how a finding moved from where it was
first reported to where it is now.

## What you see on each surface

- **Editor** — the decoration shows the current remap state; a non-`exact` state is
  cued so a squiggle never silently sits on a stale or contextually-recovered
  location.
- **Problems** — the row shows the current remap state and lets you open the full
  history.
- **Review** — annotations expose the append-only history, so a moved finding
  carries its causal trail into review.
- **CLI / headless** — explain output lists the current remap state.
- **Support export** — the export preserves each history's ordered entry trail (not
  a lossy display-only row), giving support a causal trail for moved findings.

## Imported and replayed findings

Findings imported from a scanner snapshot or replayed from a support bundle start
in the `imported_static` state: their location is the snapshot's static location,
which has not yet been revalidated against your live workspace. An imported finding
can later be mapped onto a local revision — moving to `contextual` or `exact` — but
it never *silently* reads as live local truth: the `imported_static` flag and state
stay explicit, and only an imported lane can produce an `imported_static` mapping.

## Guarantees the validator enforces

`AnchorRemapHistorySetPacket::validate` refuses a packet that:

- is **not append-only** (broken sequence or revision continuity);
- **silently repairs** an anchor (a remap state that disagrees with its evidence
  basis);
- breaks the **anchor chain** across entries;
- lets a history's **current state** disagree with its latest entry;
- renders an **imported-static** mapping inconsistently (flag, state, and lane must
  agree); or
- hides the remap history from a required editor, Problems, review, CLI, or support
  surface.

Raw source text, raw payloads, credentials, and raw artifact bodies never cross
this boundary; the packet carries only typed class tokens, booleans, opaque ids,
and redaction-aware reviewable labels.
