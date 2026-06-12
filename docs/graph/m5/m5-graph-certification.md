# M5 graph-depth certification report

This document describes the canonical packet that certifies the **M5 graph and
codebase-understanding lane** — the single qualification report that graduates the
workset-scope, graph-topology, impact-query, ownership-source, architecture-explainer,
graph-freshness, and navigation-recall code-understanding rows **only where their evidence is
current and provable**, and automatically narrows the rest to a smaller label before
publication. It is the certification layer above the
[M5 graph-governance matrix](m5-graph-governance.md): it does not re-derive each lane's
truth, it ingests the governance packet's published claim for the row, runs the per-row
qualification drills the certification suite owns, scores how fresh the certification evidence
is, and publishes the certification label no input can exceed. It is the user-facing companion
to the governed artifact at `artifacts/graph/m5/m5-graph-certification.json` and the typed
model in the `aureline-graph` crate (`m5_graph_certification`).

This packet answers the graduation question for the graph lane as a whole: **is this
code-understanding row qualified with current proof, or automatically downgraded to a narrower
label before publication — and can release evidence, docs/help, onboarding, review, AI
context, and support exports all read that same answer?**

## What this packet covers

The packet carries one certification row for every claimed M5 graph subject, each pinned to
its row in the governance matrix at `artifacts/graph/m5/m5-graph-governance.json`:

1. **`workset_scope`** — workset and sparse-scope honesty.
2. **`graph_topology`** — topology node and edge identity stability.
3. **`impact_query`** — impact-query result-class distinctness.
4. **`ownership_source`** — ownership-source classification.
5. **`architecture_explainer`** — generated-versus-curated explainer citations.
6. **`graph_freshness`** — graph freshness and invalidation.
7. **`navigation_recall`** — graph-backed navigation, docs recall, and onboarding context.

Each row answers, for its subject:

- **Who owns it?** An `owner` accountable for the row's evidence and conformance.
- **What did governance already decide?** A `governance_claim` — the published claim the
  upstream matrix already gated. The certification can only narrow from here; it never
  re-broadens a governance-narrowed row.
- **How fresh is the certification evidence?** An `evidence_freshness` of `current`, `aging`,
  `expired`, or `missing`.
- **Did every drill pass?** A `drill_results` list with one entry per required drill —
  `workset_scope`, `topology_identity`, `impact_query`, `ownership_source`,
  `explainer_citation`, `accessibility`, and `export_join` — each recording an `outcome` of
  `passed`, `narrowed`, `failed`, or `not_run`, an `evidence_ref` whenever the drill ran, and
  a `checked_at` timestamp.
- **What label is published?** A `published_label` of `authoritative`, `scope_qualified`,
  `provisional`, or `withheld`, plus the `declared_label` the row's own evidence asserted and
  the `certification_decision` the gate took.
- **Why, and how does the owner recover?** A `downgrade_reasons` set and a single
  `downgrade_path` of `rerun_drills`, `refresh_evidence`, `adopt_governance_narrowing`,
  `withhold_row`, or `none`.

## The certification gate

The gate is **non-inheriting** and **fail-closed**. A row's published label is the weakest
ceiling implied by three inputs:

- the **governance claim** the upstream matrix already published,
- the **evidence freshness** ceiling (`current` → authoritative, `aging` → scope-qualified,
  `expired` → provisional, `missing` → withheld), and
- the **drill ceiling**, the weakest of every required drill (`passed` → authoritative,
  `narrowed` → scope-qualified, `failed`/`not_run` → withheld; a missing required drill caps
  the row at withheld).

So a governance-narrowed row, stale or missing certification evidence, or an unproven,
narrowed, or failed drill all narrow or withhold the certified label automatically rather than
leaving a row green by inertia. The published label may never exceed the governance claim —
the cornerstone of the non-inheritance guarantee — and the recorded label, decision, downgrade
reasons, and recovery path must all equal the recomputed gate, so a downgrade can never be
asserted or hidden by hand.

A **certified** (authoritative) row must be genuinely whole-provable: its governance claim is
authoritative, its evidence is current, every drill passed, its declared label is
authoritative, and nothing narrows it. This is the guardrail against a blanket "codebase
understanding complete" badge over an unproven row.

## How the same packet reaches every surface

Every required consumer surface — **release evidence**, **docs/help**, **onboarding**,
**review**, **AI context**, and **support export** — binds to this one packet through a
`consumer_bindings` entry that must ingest the packet, preserve its labels and recovery paths,
and narrow with it. Each binding is stamped with the active `scope_snapshot_ref` so support
and evidence packets can reconstruct the scope the certification answered. Because all six
surfaces read the same certification packet, a row narrowed here cannot stay authoritative on a
marketed row, a docs badge, or a support export.

## Worked rows

- **`graph_topology`** and **`ownership_source`** are full-workspace, fresh, and curated with
  every drill passing on current evidence — the gate publishes an **authoritative**
  certification, proving the certifier is not a blanket downgrade.
- **`workset_scope`** drills all pass on current evidence, but governance qualified it to the
  active workset; the certification **adopts that narrowing** to a scope-qualified label
  rather than re-asserting whole-workspace certainty.
- **`impact_query`** is governance-provisional with aging evidence and a narrowed impact
  drill; it is marked **provisional** with a `rerun_drills` recovery.
- **`architecture_explainer`** and **`graph_freshness`** pass their drills but carry expired
  or aging evidence on top of a governance-provisional claim; they are marked **provisional**
  with a `refresh_evidence` recovery.
- **`navigation_recall`** is governance-withheld with missing evidence and failed or not-run
  drills; it is **withheld** entirely with a `withhold_row` recovery, so an unproven recall row
  never inherits a code-understanding claim from an adjacent certified row.

## Truth-source note

This certification report, its export projection, and its support export are the canonical
qualification source for the M5 graph and codebase-understanding lane. Marketed rows, docs
badges, onboarding tours, review explanations, AI context, and support exports should narrow
from these artifacts automatically rather than restating a row's certification by hand. The
packet is metadata-only: every field is a typed state, a count, or an opaque ref, and it
carries no credential bodies, raw provider payloads, or graph node contents.
