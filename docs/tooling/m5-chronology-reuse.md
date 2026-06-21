# Chronology reuse: one run-lifecycle grammar across every surface

This packet freezes the canonical truth for the **individual chronology entry**: one
durable run-lifecycle event — a run **started**, reported **progress**, was
**retried**, was **cancelled**, **failed**, or **completed** — written once and
*reused* across the **activity center**, the **history/timeline**, an exported **issue
packet**, a **support bundle**, and an **AI-evidence packet** rather than each surface
re-summarising what ran. Each entry binds its **actor/action/object/outcome** grammar
to the canonical **task/run/channel/problem** objects, the **provider/adapter** and
**target scope** it ran against, its **retry lineage**, the evidence
**freshness/stale/superseded state**, the **confidence tier**, and the
**reopen-to-origin target** — so a failure shown in three places points to one
canonical run/channel/problem id rather than three rephrasings.

It is the timeline companion to the
[`m5-execution-evidence`](./m5-execution-evidence.md) **lane matrix**, the
[`m5-problem-records`](./m5-problem-records.md) **Problems row**, and the
[`m5-execution-evidence-projections`](./m5-execution-evidence-projections.md)
**projected overlay**. Where the lane matrix freezes one row per surface *family*, the
Problems packet freezes one row per *finding*, and the projection packet freezes one
row per *overlay*, this packet freezes one row per *chronology entry*. All four speak
one vocabulary — origin class, confidence tier, freshness state, reopen target, and
proof currency are reused, not re-invented — so activity, history, issue export,
support export, and AI evidence ingest one model instead of a private run-history
model. Reuse the canonical task-event envelopes, diagnostic ids, run/channel refs, and
activity rows already landed earlier; this packet binds them onto one inspectable,
reopenable chronology row.

If this doc, the
[`m5-chronology-reuse.schema.json`](../../schemas/tooling/m5-chronology-reuse.schema.json)
boundary, the frozen set under
[`/artifacts/tooling/m5-chronology-reuse/`](../../artifacts/tooling/m5-chronology-reuse/),
and the perturbation corpus under
[`/fixtures/tooling/m5-chronology-reuse/`](../../fixtures/tooling/m5-chronology-reuse/)
disagree, the machine-readable schema plus the checked-in support export
(`artifacts/tooling/m5-chronology-reuse/support_export.json`) win, and this doc must
update in the same change.

## Companion artifacts

- [`/schemas/tooling/m5-chronology-reuse.schema.json`](../../schemas/tooling/m5-chronology-reuse.schema.json)
  — boundary schema for the `m5_chronology_reuse_set_packet` and every frozen
  taxonomy.
- [`/artifacts/tooling/m5-chronology-reuse/support_export.json`](../../artifacts/tooling/m5-chronology-reuse/support_export.json)
  — the canonical chronology set (the source of truth for every entry).
- [`/artifacts/tooling/m5-chronology-reuse/report.md`](../../artifacts/tooling/m5-chronology-reuse/report.md)
  — the generated certification report (do not edit by hand; regenerate with the Rust
  dump example).
- [`/fixtures/tooling/m5-chronology-reuse/`](../../fixtures/tooling/m5-chronology-reuse/)
  — the perturbation corpus that pins each narrowing/floor rule.
- `tools/release/chronology_reuse.py` — re-derives the effective claim and ordered
  narrowing reasons per entry and validates the set and corpus.
- `crates/aureline-runtime/src/m5_task_problem_output_chronology_reuse/` — the
  in-process Rust truth source. It deserializes the checked-in support export into one
  typed packet, re-derives the same effective claim, floor/overlay/labs ladder, and
  ordered narrowing reasons as the Python engine, and exposes
  `current_m5_chronology_reuse_set()` so activity-center, history, issue-export,
  support-export, and AI-evidence consumers ingest the governed chronology without
  re-parsing raw logs or forking a parallel run-history model.

## The causal chain a chronology entry preserves

A chronology entry is reused away from where it was first written. To stay honest it
must, on every surface it is reused on, be able to answer **who did what to which
object with what outcome**; **which run, which step, which channel, which problem** it
binds to; what **provider/adapter** and **target scope** it ran against; whether it is
a **retry** and of which run; whether the evidence is **fresh, stale, or superseded**;
and **how to reopen** the originating run, channel, artifact, or packet. The engine
re-derives — rather than trusts — an effective claim from these invariants:

- **Grammar & lineage** (`integrity.preserves_actor_action_object_outcome`,
  `preserves_provider_adapter`, `preserves_target_scope`, `preserves_retry_lineage`,
  `preserves_canonical_ids`, `lineage_visible_on_demand`): the actor/action/object/
  outcome grammar, the provider/adapter and target scope, the retry lineage, and the
  canonical run/channel/problem ids all survive into the reused entry and can be
  revealed on demand on every reuse surface.
- **One canonical id across surfaces** (`bindings[*].bound_run_ref`,
  `bound_channel_ref`, `bound_problem_ref`): each reuse surface points at the entry's
  own canonical ids. A surface that points at a *different* run/channel/problem id
  breaks the single-id contract and floors — this is the guarantee that a failure
  shown in the activity center, a support bundle, and an AI-evidence packet resolves to
  one id, not three.
- **Freshness & confidence** (`declared_freshness_state`,
  `integrity.freshness_state_labeled`, `superseded_state_marked`,
  `declared_confidence_tier`, `integrity.confidence_label_visible`,
  `raw_output_backlink_present`): stale and superseded states stay visibly classified;
  the confidence tier is visible; a heuristic entry keeps a raw-output backlink;
  missing evidence floors.
- **Self-contained export** (`integrity.export_self_contained`): an entry reused in an
  issue packet, a support bundle, or an AI-evidence packet stays reviewable and
  reopenable without the originating live UI state.
- **Reopen** (`declared_reopen_target`): every entry can reopen its origin; an entry
  that loses its reopen path keeps a `raw_output_backlink` or `none_keyboard_fallback`.
- **Surface honesty** (`bindings[*].rendered_claim`): a reuse surface may never render
  a claim wider than the entry's effective claim.

A recorded **action** and a recorded **outcome** can never silently disagree
(`run_failed` ⇒ `failed`, `run_completed` ⇒ `succeeded`, and so on), and a
**`run_retried`** entry must carry an attempt index ≥ 2 and a prior-run ref so a rerun
never reads as a first attempt.

## The effective-claim ladder

| Effective claim | Meaning |
| --- | --- |
| `chronology_reused` | Full first-party chronology preserved, fresh, grammar/ids intact, reopenable — reused faithfully across every surface. |
| `chronology_narrowed` | A first-party entry held below reused by a stale/labelled gap, but lineage stays reopenable. |
| `chronology_read_only_overlay` | Remote/pipeline/imported chronology: attributable and reopenable but never claims live local authority. |
| `chronology_unreconstructable` | Grammar/lineage/reopen broken or canonical ids disagree: surfaces a raw-output backlink or keyboard fallback instead of a clean-but-false row. |
| `chronology_labs_not_claimed` | Labs/unadvertised: makes no public claim and is never widened. |

**Floor** reasons (`grammar_flattened`, `provider_adapter_flattened`,
`target_scope_flattened`, `retry_lineage_flattened`, `canonical_id_flattened`,
`canonical_id_divergence`, `lineage_not_visible`, `raw_output_backlink_missing`,
`reopen_target_lost`, `export_not_self_contained`, `surface_overclaims`,
`imported_chronology_claims_live`, `evidence_missing`) break the "stay reopenable /
never flatten grammar or lineage / never masquerade as live / stay self-contained"
contract outright and drop the entry to `chronology_unreconstructable`. The remaining
reasons hold a first-party entry at `chronology_narrowed` (still reopenable). A
read-only overlay is already the minimal honest claim, so any non-floor gap drops it
below the overlay too. Labs entries never accrue narrowing.

## Regeneration

```bash
# Rust: regenerate the support export and report (identical bytes each run).
cargo run -p aureline-runtime --example dump_m5_chronology_reuse > \
  artifacts/tooling/m5-chronology-reuse/support_export.json
cargo run -p aureline-runtime --example dump_m5_chronology_reuse summary > \
  artifacts/tooling/m5-chronology-reuse/report.md

# Python: regenerate the perturbation corpus and validate end-to-end.
python3 tools/release/chronology_reuse.py emit-corpus
python3 tools/release/chronology_reuse.py self-test
```
