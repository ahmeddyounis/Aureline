# Runbook governance contract

Aureline represents runbooks as **governed executable guidance**. A runbook is not
free-form prose with implied authority; it is a set of typed objects that declare
where authority comes from, what is being executed, what approval that requires,
what evidence it should produce, and how any pivot off Aureline's governed plane
stays attributable. This document is the contract for that object model. The
machine-readable inventory lives at
[`artifacts/runbooks/m5-runbook-governance.md`](../../artifacts/runbooks/m5-runbook-governance.md)
and the canonical packet at
[`artifacts/runbooks/m5-runbook-governance.json`](../../artifacts/runbooks/m5-runbook-governance.json).

## Why this exists

Incident workspaces, operator dashboards, docs/help, companions, and support
bundles all reference runbooks. Before this contract, that reference was implicit:
runbook authority, step taxonomy, and deviation lineage lived in prose and
screenshots. A claimed incident/operator surface cannot stay trustworthy while its
runbooks remain rich text with invisible step classes and ad-hoc console pivots.
This lane freezes one inventory those surfaces consume instead.

## The object model

The crate `aureline-runbooks` (`m5_runbook_governance`) owns six governed object
classes:

1. **Source descriptor** — where a runbook's authority comes from (its *source
   class*), its owner, the default approval scope its steps inherit, and whether a
   companion may request execution within scope. Imported vendor-console
   references and companion drafts carry no standing execution authority.
2. **Step descriptor** — what *class* of step runs (`inspect`, `diagnose`,
   `mitigate`, `rollback`, `console_handoff`, `approval`, `annotate`), the scope or
   approval it requires, the control-plane boundary it sits on, whether it mutates
   state, the evidence it should produce, and whether a companion may run it.
3. **Execution record** — what actually ran, step by step, with outcomes.
4. **Deviation note** — a departure from declared guidance, with a lineage class,
   the step it departs from, an approver, and an attributability flag. The
   execution record rolls these up into an ordered **deviation lineage**.
5. **Control-plane handoff packet** — a console/browser pivot. It records the
   boundary crossed, an attribution ref, whether control returns to the governed
   plane, and a flag asserting it does not mint a hidden privileged mutate channel.
6. **Archival/export object** — the retained, export-safe execution history. It
   carries metadata and refs only; raw content is never exported.

Source and step descriptors have standalone schemas; the execution schema embeds
deviation notes, handoff packets, and the archival/export object.

## Invariants the validator enforces

- A **mutating step** (`mitigate`/`rollback`) must require approval. A mutating
  step with `no_approval_read_only`, or a companion-permitted step outside
  read-only/self-approve scope, is rejected as a **hidden mutate channel**.
- A step whose boundary leaves the governed plane must carry an **attributable
  handoff packet**; a handoff may never mint a hidden mutate channel.
- A recorded deviation must be **attributable** and name an approver.
- A companion may not drive a mutating step it is not permitted for.
- Execution-record rollups (deviation lineage, attribution, the no-hidden-mutate
  flag) are recomputed and compared, so they can never drift from the steps.
- Exports carry no credential bodies or raw provider/console payloads.

## Release gating

Each claimed runbook-backed surface binds the governed objects it depends on. The
matrix derives a status (`mapped`/`provisional`/`unmapped`), a gate decision
(`governed`/`narrowed`/`blocked`), and an effective claim:

- **Blocked** — the surface binds an object the matrix does not govern, or whose
  proof is missing. Stable promotion fails; the gap is named, never hidden.
- **Narrowed** — the surface binds an object whose proof is stale. The surface
  auto-narrows below Stable.
- **Governed** — every bound object is mapped with current proof.

A blocking gap can be accepted only under a disclosed, time-bounded waiver scoped
to a single object. The waiver narrows the surface instead of blocking it, but the
surface's *true* status stays red — the matrix never hides a real gap behind a
waiver. The release center and public-truth automation read the packet-level
release gate; Help/About, shiproom, and support exports read the same inventory.

## Consuming the contract

Surfaces should reference the contract inventory rather than local prose or
screenshots:

- **Incident workspaces** and **operator dashboards** render execution records,
  deviation lineage, and handoff packets from this model.
- **Docs/Help** describe source and step classes from the published vocabularies.
- **Companions** follow or request within the declared scope encoded here.
- **Support bundles** ship the archival/export object.
- The **release center** gates promotion on the matrix.

See [`artifacts/runbooks/m5-runbook-governance.md`](../../artifacts/runbooks/m5-runbook-governance.md)
for the schema, fixture, and re-mint references.
