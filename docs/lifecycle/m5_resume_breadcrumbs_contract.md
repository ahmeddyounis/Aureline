# M5 resume breadcrumbs contract

This lane is the **partial-truth and resume-breadcrumb capstone** on top of the frozen
[M5 lifecycle-state and journey-checkpoint matrix](m5_lifecycle_matrix_contract.md). The matrix
freezes, for every long-lived M5 object family, an explicit state machine, a recovery affordance, a
controlled last-failure reason, and an ordered inventory of milestone checkpoints. This lane
certifies that when a journey is **degraded, resumed, restored, partially replayed, or blocked**, the
object preserves breadcrumbs that let a user, support, automation, and docs tell exactly what they
are looking at and what Aureline intentionally did not do.

The lane exists so that M5 can honestly ship its growing mix of notebook, data/API, AI, remote,
preview, pipeline, docs, and support surfaces without object state, checkpoint boundaries, and
recovery vocabulary drifting by surface or disappearing in export paths. A user must be able to tell
whether they are looking at live truth, restored context, cached evidence, or a restart-required
placeholder — and what was intentionally not rerun or reauthorized after a restore or reconnect.

## Certified object families

The certification covers exactly the thirteen governed object families the matrix freezes, and
refuses to ship if any is missing: `workspace`, `extension`, `remote_session`,
`collaboration_session`, `ai_action`, `update_rollback`, `notebook_runtime`, `request_api_run`,
`preview_session`, `pipeline_run`, `data_session`, `profiler_capture`, and `companion_session`.

Every attribute a row certifies over — the driving matrix journey, the object's explicit state
machine (the admitted controlled states, always including `ready`), the named recovery affordance the
not-resumed disclosure anchors on, the controlled last-failure reason classes, the checkpoint lineage
the breadcrumb replays, the declared consumer surfaces, and the applicable downgrade triggers — is
pulled straight from the frozen matrix's seeded packet, so this lane mints no parallel lifecycle
vocabulary and cannot certify a family the matrix does not anchor. Only the provenance classes
distinguished, the lineage facets preserved, the per-family posture, and the scope summary are
authored here.

## Provenance classes and lineage facets

Each row proves it distinguishes all four **provenance classes** — the breadcrumb headers a user
reads to tell live from recovered data:

- `live_truth` — the surface reflects the object's current, freshly computed state.
- `restored_context` — the surface was rehydrated from a durable snapshot after a restart.
- `cached_evidence` — the surface shows a prior value held past its freshness floor.
- `restart_required_placeholder` — a labeled placeholder that needs an explicit restart or
  reauthorize before it can carry live truth.

Each row also proves it preserves all four **lineage facets** — so a recovered breadcrumb names its
origin instead of a generic "recovered" label:

- `source_class` — whether the value is live, restored, cached, or a placeholder.
- `actor_subsystem` — which controlled actor or subsystem produced the value.
- `host_boundary` — which host or trust boundary the value crossed.
- `checkpoint_lineage` — which milestone checkpoint the value resumed from.

## Certified breadcrumb dimensions

Each row is certified across the four breadcrumb dimensions the acceptance criteria require
(`provenance_labeling`, `lineage_breadcrumb`, `not_resumed_disclosure`, `capture_parity`):

- **provenance labeling** — `provenance_class_labeled_on_every_surface` (green: every surface shows a
  controlled provenance header naming which of the four classes the value is), a disclosed
  `disclosed_coarse_provenance_grouping` where the classes are grouped on a compact surface while
  each is still distinguished (yellow), or `provenance_class_ambiguous_or_missing` (red: a restored,
  cached, or restart-required value could be mistaken for live truth).
- **lineage breadcrumb** — `source_actor_boundary_checkpoint_preserved` (green: a breadcrumb names
  the source class, actor/subsystem, host/boundary, and checkpoint lineage it resumed from), a
  disclosed `disclosed_partial_lineage_breadcrumb` where one facet is dropped on a compact surface
  while the rest are named (yellow), or `generic_recovered_wording_only` (red: only generic
  "recovered" wording with no lineage).
- **not-resumed disclosure** — `not_resumed_actions_explicit` (green: each action the object
  intentionally did not rerun or reauthorize after a restore or reconnect is named), a disclosed
  `disclosed_grouped_not_resumed_summary` where the withheld set is grouped into a category while
  still disclosed (yellow, **requires an active waiver**), or `not_resumed_actions_silently_absent`
  (red: actions were silently dropped, so the user cannot tell what Aureline intentionally did not
  do).
- **capture parity** — `breadcrumbs_captured_in_export_and_screenshot` (green: the same provenance
  headers, lineage breadcrumbs, and not-resumed disclosures the user sees live are captured in a
  screenshot, a support packet, and an export), a disclosed `disclosed_partial_capture` where a
  reduced subset is captured while the provenance header and terminal breadcrumb are still captured
  (yellow), or `breadcrumbs_absent_from_capture` (red: the breadcrumbs did not survive
  export/screenshot/support capture).

A `headless_parity_preserved` flag records that the same state-truth vocabulary survives a headless
or companion-adjacent execution; losing it is a hard blocker. An incomplete provenance-class set or
lineage-facet set is likewise a hard blocker — it cannot prove that restored, resumed, cached, and
live states stay distinguishable or that a breadcrumb fully attributes its value.

## Auto-narrowing and completeness

Each row's green/yellow/red status is **derived**, never asserted. Any hard blocker — an ambiguous
provenance class, generic recovered wording, silently-absent not-resumed actions, breadcrumbs absent
from capture, a headless/companion-adjacent vocabulary loss, an incomplete provenance-class or
lineage-facet set, or a row that did not certify every consumer surface the matrix declares for the
family — forces `red`; any disclosed narrowing forces `yellow`; otherwise `green`. A disclosed
grouped not-resumed summary must carry an active waiver to stay publishable, and every non-green row
must disclose a reason. The consumer-surface, provenance-class, and lineage-facet completeness checks
are the lints that keep a certification from regressing into a partial view that would let a
restored, cached, or restart-required state be mistaken for live truth on the surfaces it did not
certify.

The seeded certification is **9 green** and **4 yellow** (companion session disclosing a coarse
provenance grouping on its small paired-device surface, preview session disclosing a partial lineage
breadcrumb on its compact strip, profiler capture disclosing a partial capture in its compact export,
and collaboration session with a waivered grouped not-resumed summary on reconnect), with **0 red**.
Five protected blocked fixtures prove the red path for each acceptance-criteria failure mode: the
notebook runtime leaving its provenance ambiguous, the remote session showing only generic recovered
wording, the data session silently dropping its not-resumed actions, the AI action dropping its
breadcrumbs from capture, and the extension losing headless parity.

## Artifacts

- Schema: `schemas/lifecycle/m5-resume-breadcrumbs.schema.json`
- Report: `artifacts/lifecycle/m5-resume-breadcrumbs.md`
- Proof packet: `artifacts/release/m5-resume-breadcrumbs-proof/packet.json`
- Proof dashboard: `artifacts/release/m5-resume-breadcrumbs-proof/dashboard.json`
- Proof support export: `artifacts/release/m5-resume-breadcrumbs-proof/support_export.json`
- Proof CSV: `artifacts/release/m5-resume-breadcrumbs-proof/matrix.csv`
- Fixtures: `fixtures/state/m5-resume-breadcrumbs/packet.json` (and `dashboard.json`,
  `support_export.json`, `compact.txt`)

The Rust validator `validate_m5_resume_breadcrumbs_packet` in
`crates/aureline-shell/src/m5_resume_breadcrumbs/` is the authoritative gate; the schema above
documents the shape. The headless emitter `aureline_shell_m5_resume_breadcrumbs` is the only
mint-from-truth path.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_resume_breadcrumbs -- validate
cargo test -p aureline-shell --test m5_resume_breadcrumbs_fixtures
cargo test -p aureline-shell --lib m5_resume_breadcrumbs
```
