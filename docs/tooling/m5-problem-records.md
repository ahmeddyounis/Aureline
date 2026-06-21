# Problem records: source-task correlation, confidence labels, and rerun/jump parity

This packet freezes the canonical truth for the **individual Problems-panel row**:
a single run-derived finding bound to its **source tool/run refs**, its
**file/span anchor**, its **structured-versus-heuristic parse class**, its
**confidence tier** and **raw-output backlink**, the **editor decoration**,
**timeline entry**, **source task**, and **owning output channel** it is
correlated with, and the **freshness/stale/superseded state** of the run it came
from.

It is the per-record companion to the
[`m5-execution-evidence`](./m5-execution-evidence.md) **lane matrix**. Where the
lane matrix freezes one row per Problems/output/execution-evidence *surface
family*, this packet freezes one row per *finding*. Both speak one vocabulary —
problem-source kind, confidence tier, freshness state, origin class, output-channel
class, and proof currency are reused, not re-invented — so Problems, the editor,
output, timeline, review, CLI/headless, AI evidence, and support export ingest one
model instead of a private bottom-panel truth model. Reuse the canonical
task-event envelopes, diagnostic IDs, activity rows, run objects, and output
channels already landed earlier; this packet binds them onto one inspectable,
reopenable Problems row.

If this doc, the
[`m5-problem-records.schema.json`](../../schemas/tooling/m5-problem-records.schema.json)
boundary, the frozen set under
[`/artifacts/tooling/m5-problem-records/`](../../artifacts/tooling/m5-problem-records/),
and the perturbation corpus under
[`/fixtures/tooling/m5-problem-records/`](../../fixtures/tooling/m5-problem-records/)
disagree, the machine-readable schema plus the checked-in support export
(`artifacts/tooling/m5-problem-records/support_export.json`) win, and this doc must
update in the same change.

## Companion artifacts

- [`/schemas/tooling/m5-problem-records.schema.json`](../../schemas/tooling/m5-problem-records.schema.json)
  — boundary schema for the `m5_problem_record_set_packet` and every frozen
  taxonomy.
- [`/artifacts/tooling/m5-problem-records/support_export.json`](../../artifacts/tooling/m5-problem-records/support_export.json)
  — the canonical Problems-record set (the source of truth for every row).
- [`/artifacts/tooling/m5-problem-records/report.md`](../../artifacts/tooling/m5-problem-records/report.md)
  — the generated certification report (do not edit by hand; regenerate with the
  Rust dump example or the Python validator, which emit identical bytes).
- [`/fixtures/tooling/m5-problem-records/`](../../fixtures/tooling/m5-problem-records/)
  — the perturbation corpus that pins each narrowing/floor rule and the per-action
  availability projection.
- `tools/release/problem_records_causality.py` — re-derives the effective status,
  downgrade reasons, and per-action availability per record and emits/validates the
  report and corpus.
- `crates/aureline-runtime/src/m5_problem_records_source_task_correlation_and_rerun_jump_parity/`
  — the in-process Rust truth source. It deserializes the checked-in support export
  into one typed packet, re-derives the same per-record status, downgrade reasons,
  floor/overlay/labs ladder, and action availability as the Python engine, and
  exposes `current_m5_problem_record_set()` so desktop, CLI/headless, AI evidence,
  support export, review, and docs consumers ingest the governed projection without
  re-parsing raw logs or forking a parallel truth model. The
  `dump_m5_problem_records` example regenerates the support export and report from
  the in-crate builder so the artifacts never drift away from Rust.

## What a problem record carries

Every record binds, with stable refs (never freeform display text):

- **Source identity** — the producing tool/adapter, the owning run/step, the
  provider (for overlays), the build/toolchain and host/target, the task-event
  envelope it was projected from, and a **raw-output backlink** into the chunk that
  produced it.
- **Parse class** — whether the finding came from a **native structured
  diagnostic**, a **normalized task event**, an **imported provider annotation**,
  or **heuristic text parsing**. The four origins always stay inspectable; a user
  can answer where any row came from.
- **Confidence tier** — structured findings are full confidence; a heuristic parse
  carries an explicit `heuristic_high|medium|low` tier *and* a raw-output backlink;
  imported provider mappings carry their mapping quality; `unmapped_requires_review`
  is a first-class state, not an invisible gap.
- **File/span anchor** — an opaque workspace-relative file ref plus one-based
  line/column numbers and an owning symbol ref. No absolute paths or raw source
  text cross the boundary.
- **Correlations** — the editor decoration, the activity-center timeline entry, the
  source task (the rerun/inspect handle), and the owning output channel. These join
  the existing surfaces rather than re-deriving a bottom-panel model.
- **Freshness / stale / superseded state** — `live`, `cached_within_window`,
  `stale_expired`, `superseded_by_newer_run`, `unanchored`, or `missing`, plus a
  `mapping_downgraded` flag. Stale, superseded, and downgraded findings stay
  visibly classified, never dropped and never silently upgraded.

## Re-derived status and action parity

The packet re-derives, rather than trusts, an effective **status** per row and a
per-action **availability** for the three canonical actions. The status ladder, in
ascending order of actionable certainty:

| Status | Meaning |
| --- | --- |
| `raw_evidence_only` | Origin/lineage broken or dishonest; the row surfaces a raw-output backlink instead of a clean-but-false actionable claim. |
| `read_only_imported` | Remote/pipeline/imported finding; inspectable and reopenable but never a live local actionable row. |
| `narrowed_actionable` | A first-party row held below fully actionable by a stale/superseded/downgraded/uncorrelated gap, but still jumpable and inspectable. |
| `actionable` | Honest origin, current evidence, full correlations, every applicable action available. |
| `labs_not_claimed` | Labs/unadvertised; makes no public actionability claim and is never widened. |

A row **floors** to `raw_evidence_only` when it conflates structured/heuristic
origin, drops a heuristic raw-output backlink, loses its source-tool ref, leaves a
superseded retry unmarked, has missing run evidence, or lets an imported overlay
claim live local authority. Otherwise a first-party gap (stale run, superseded
retry, downgraded mapping, missing correlation, stale/missing proof) holds the row
at `narrowed_actionable` — still jumpable and inspectable, with a precise,
non-generic downgrade label naming the trigger.

Each row's three actions are re-derived from its refs and rerun authority:

- **Jump to source** — available when the file/span anchor resolves; unavailable
  when the anchor is missing or the run evidence is gone.
- **Open owning output** — available when the owning channel carries a stable ref;
  `not_applicable` for findings with no channel (e.g. a pure language diagnostic);
  unavailable when a real channel lost its ref.
- **Rerun or inspect the originating task/session** — `available` for a local task
  with a correlated source task; `gated_requires_authority` when rerun is permitted
  but needs explicit authority; `read_only_inspect_only` for remote/imported
  origins (which never rerun locally); `unavailable` when policy denies it or the
  source task is uncorrelated.

A projection may never render a row wider than its effective status, and a Labs row
may only render as itself.

## Guardrails

- A heuristic parse always keeps an explicit confidence tier and a raw-output
  backlink; structured and heuristic origins never collapse into one.
- Stale, superseded, and downgraded findings stay visibly classified until
  dismissed or replaced by current evidence — never silently dropped, never
  silently upgraded to fresh certainty.
- Imported/remote findings stay read-only overlays that name their provider and
  never claim a live local rerun.
- A floored row keeps a reopenable raw-output backlink rather than a clean-but-false
  actionable claim.
- No raw stdout/stderr bytes, command lines, provider log bodies, env bodies,
  absolute paths, URLs, or secrets cross this boundary.
