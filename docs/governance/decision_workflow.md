# Decision workflow

This document is the rulebook for how architecture decisions are opened,
discussed, closed, superseded, and narrowed. It is written so that a
contributor arriving for the first time can follow the discipline
without inventing metadata or shortcuts.

Companion artifacts:

- [`/docs/governance/decision_backlog.md`](./decision_backlog.md) —
  navigational index of the decision register.
- [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  — machine-readable register.
- [`/schemas/governance/decision_index.schema.json`](../../schemas/governance/decision_index.schema.json)
  — schema the register conforms to.
- [`/docs/adr/`](../adr/) and [`/docs/rfc/`](../rfc/) — decision records
  and proposals.
- [`/docs/governance/dri_map.md`](./dri_map.md) — ownership, blocker
  aging, and narrowing authority.
- [`/docs/governance/templates/`](./templates/) — waiver, verification
  packet, and freeze-exception templates.

## Core rule: no architecture in a quiet PR

Protected-lane-visible behaviour changes **MUST** link to an ADR (or an
RFC that will close as an ADR) before broad implementation. "Broad
implementation" means any landing that goes beyond a throwaway spike or
a documentation-only change.

A change crosses this line when it:

- adds, removes, or renames a type, trait, or contract in a protected
  crate's public surface;
- changes the behaviour of a protected lane in a way that is visible to
  another lane or to users;
- locks in a cross-lane contract (RPC envelope, subscription envelope,
  schema format, identity mode, release channel, build-identity rule);
- narrows a protected lane's committed scope for the current milestone.

A PR that would cross this line without a linked ADR is not ready to
merge. The author either links to an existing decision row (and the ADR
that closed it), or opens a new decision row and lands the ADR in the
same PR.

## Opening a decision

1. **Append a decision row.** Add a row to
   `artifacts/governance/decision_index.yaml` using the next unused
   `D-NNNN` id. Fill every required field, including
   `default_if_unresolved`.
2. **Add a navigational entry.** Update the table in
   `docs/governance/decision_backlog.md`.
3. **Decide the proposal vehicle.**
    - For a small decision with an obvious answer: open an ADR
      directly from [`/docs/adr/0000-template.md`](../adr/0000-template.md).
    - For a decision that needs written exploration: open an RFC
      from [`/docs/rfc/0000-template.md`](../rfc/0000-template.md) and
      set the decision row's status to `deciding`.

## Closing a decision

An ADR closes a decision. When the ADR merges:

- set the decision row's `status` to `decided`;
- set `linked_adr` to the ADR's repo-relative path;
- append a `decision_history` entry with outcome `accept`.

If a forum explicitly rejects a decision option without picking an
alternative, the row stays open (or moves to `deferred`) and the ADR
that recorded the rejection is referenced from the `decision_history`
entry rather than from `linked_adr`.

## Superseding a decision

A decision is never rewritten in place. Superseding a decision means:

1. Mint a new row with a fresh `D-NNNN` id that supersedes the old one.
2. Set the old row's `status` to `superseded` and its `superseded_by`
   array to include the new id.
3. Append a `supersede` entry to the old row's `decision_history`.
4. Do not delete the old row's body. The `decision_history` preserves
   what was decided, when, by whom, and why.

ADRs follow the same rule: a superseded ADR sets its `Status` to
`Superseded by ADR-NNNN` and adds a line to its supersession history;
its body stays.

## Narrowing by default (freeze-deadline behaviour)

Every decision row declares a `default_if_unresolved` posture and an
`applies_on` date. When the freeze date passes with the row still
`open` or `deciding`, tooling applies the default automatically:

- `narrow` — the row's description tells the forum what the narrowed
  scope looks like. The row moves to `narrowed_by_default`. Reopening
  the wider scope requires a **new** decision row; the narrowed row
  survives as audit.
- `defer` — the forum has implicitly accepted a deferral. The row moves
  to `deferred` with a restated freeze date chosen by the next decision
  forum meeting.
- `freeze_lane` — dependent work on the named lane stops. The row
  stays in its current status but every downstream PR must link to a
  freeze-exception packet (see below) or hold until the ADR lands.
- `rebaseline` — the milestone scorecard enters rebaseline review.
  Architecture council plus release council approve the new baseline
  per [`dri_map.md`](./dri_map.md).

`applies_on` normally equals `freeze_by_date`. When a default posture
requires forum approval rather than firing automatically, the row's
`requires_approval_from` lists the forums whose sign-off is needed;
tooling holds the row in `deciding` until one of them meets.

Worked example: decision `D-0012` in the register demonstrates the
`narrow` posture. Its freeze date has already passed; no ADR closed
it; the row is in `narrowed_by_default` with a `narrow` entry in
`decision_history` dated on its `applies_on` field.

## Freeze exceptions

A freeze exception is the only way to land a change after a freeze has
fired. Open one by copying
[`/docs/governance/templates/freeze_exception_template.md`](./templates/freeze_exception_template.md).
Exceptions must be narrow, list a rollback plan, and be approved by the
forum named in the narrowing authority table in
[`dri_map.md`](./dri_map.md).

## Linkage rules (the contract tooling relies on)

These linkage rules are what make the register useful. Breaking any of
them is a tooling failure, not a style preference.

1. Every ADR references exactly one `decision_id`.
2. Every decided row has exactly one `linked_adr`.
3. Every `linked_adr` path exists; every `linked_rfc` path exists.
4. Every decision row names a backup owner or cites an active waiver
   from `ownership_matrix.yaml#waivers`.
5. Every superseded row has a non-empty `superseded_by`.
6. `decision_history` is append-only. Editing past entries is a
   governance error even when the edit is cosmetic.
7. Adding a new metadata field anywhere in the register, an ADR, an
   RFC, a waiver, a verification packet, or a freeze-exception packet
   requires a schema change in
   [`/schemas/governance/decision_index.schema.json`](../../schemas/governance/decision_index.schema.json)
   (or the relevant packet schema). No ad-hoc fields.
