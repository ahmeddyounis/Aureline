# DRI map and escalation

This document is the authoritative narrative for ownership, blocker aging,
and escalation at the current milestone. It exists so that protected paths,
public-truth surfaces, and supportability artifact families cannot become
"shared responsibility" by default.

Companion artifacts:

- [`/CODEOWNERS`](../../CODEOWNERS) — pull-request review routing.
- [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml)
  — machine-readable ownership and backup-owner waivers.
- [`/artifacts/governance/package_inventory.yaml`](../../artifacts/governance/package_inventory.yaml)
  — package topology and protected-path posture.
- [`/docs/governance/blocker_aging_slas.md`](./blocker_aging_slas.md)
  — canonical blocker-aging clock and escalation table for freeze
  blockers, stale evidence, owner gaps, and unresolved waivers.

**No silent blanks.** Every protected lane has either a named backup owner or
a recorded waiver with named expiry, reason, and escalation path. If this
document and the ownership matrix ever disagree, the matrix wins for tooling
and this document must be updated in the same change.

## Terms

- **DRI (Directly Responsible Individual).** Single accountable owner for a
  lane. Named, not a group. May not be "TBD".
- **Backup owner.** Second named person who can approve on the DRI's behalf
  and who inherits the DRI role during outages. Required on every protected
  lane unless a waiver is in effect.
- **Protected lane.** Any crate marked `protected_path: true` in the package
  inventory, plus the non-code lanes explicitly named protected below
  (release evidence, governance packet families, docs/public truth,
  support/export schema).
- **Backup-owner waiver.** Time-boxed, named-expiry document that makes the
  absence of a backup owner explicit. Recorded in the ownership matrix; never
  implicit.
- **Narrowing.** Reducing the committed scope of a lane for the current
  milestone without abandoning its eventual delivery.
- **Re-baseline.** Moving a milestone's committed scope, dates, or acceptance
  thresholds because the prior baseline is no longer achievable.

## Current solo-maintainer posture

The project is in its pre-implementation stage with a single active
maintainer. Under this posture:

- Every DRI below resolves to the sole maintainer. The ownership matrix
  records this plainly.
- Backup-owner absence on protected lanes is covered by a single
  **single-maintainer backup waiver** with a named expiry (see the ownership
  matrix). When a second maintainer is confirmed, the waiver closes and
  named backups populate both this document and the matrix in the same
  change.
- Escalation for a solo maintainer is a self-escalation step (explicit
  written entry in the shiproom packet with date, cause, decision) plus a
  public contributor-community thread. This is weaker than a multi-person
  escalation on purpose; the waiver exists to make that weakness visible.

The solo-maintainer posture is not acceptable past beta entry. Replacing it
with named backups is part of the exit criteria of the next program review.

## Lane ownership

| Lane                              | Primary DRI     | Backup owner | Backup status                 | Protected |
|-----------------------------------|-----------------|--------------|-------------------------------|-----------|
| Renderer                          | @ahmeddyounis    | (none)       | Single-maintainer waiver      | yes       |
| Buffer / editor core              | @ahmeddyounis    | (none)       | Single-maintainer waiver      | yes       |
| Workspace VFS                     | @ahmeddyounis    | (none)       | Single-maintainer waiver      | yes       |
| Text foundation                   | @ahmeddyounis    | (none)       | Single-maintainer waiver      | yes       |
| RPC transport                     | @ahmeddyounis    | (none)       | Single-maintainer waiver      | yes       |
| Telemetry foundation              | @ahmeddyounis    | (none)       | Single-maintainer waiver      | yes       |
| Shell / command system            | @ahmeddyounis    | (none)       | Single-maintainer waiver      | yes       |
| Shell spike (throwaway)           | @ahmeddyounis    | (none)       | Not required (unprotected)    | no        |
| Benchmark lab                     | @ahmeddyounis    | (none)       | Single-maintainer waiver      | yes       |
| Release evidence                  | @ahmeddyounis    | (none)       | Single-maintainer waiver      | yes       |
| Docs / public truth               | @ahmeddyounis    | (none)       | Single-maintainer waiver      | yes       |
| Support / export                  | @ahmeddyounis    | (none)       | Single-maintainer waiver      | yes       |
| Design-system seeds               | @ahmeddyounis    | (none)       | Single-maintainer waiver      | yes       |
| Accessibility / input review      | @ahmeddyounis    | (none)       | Single-maintainer waiver      | yes       |
| Governance forums                 | @ahmeddyounis    | (none)       | Single-maintainer waiver      | yes       |

The `Shell / command system` row is listed separately from the `Shell spike`
row on purpose: the command-system lane persists beyond the spike and is
protected; the spike itself is disposable.

Governance forum families owned under the `Governance forums` row include the
architecture council, performance council, security and trust review,
accessibility review, ecosystem and compatibility review, milestone scope
review (`product_scope_review`), open community sync, release council, and
shiproom / executive scope review. Under the solo-maintainer posture these
forums are single-attendee decision logs; adding a second chair is part of
closing the backup waiver.

## Scope of each lane

- **Renderer** — `crates/aureline-render` and its protected rendering
  primitives. Owns renderer ADRs and renderer fitness functions.
- **Buffer / editor core** — `crates/aureline-buffer` (piece tree,
  selections, undo/redo). Owns buffer ADRs and editor fitness functions.
- **Workspace VFS** — `crates/aureline-vfs` (roots, watchers, canonical path
  identity). Owns workspace persistence ADRs.
- **Text foundation** — `crates/aureline-text` (encoding, segmentation).
  Leaf L0 foundation shared by renderer and buffer.
- **RPC transport** — `crates/aureline-rpc`. Owns cross-process contracts.
- **Telemetry foundation** — `crates/aureline-telemetry`. Leaf L0 foundation
  for hot-path instrumentation, tracing, and metrics.
- **Shell / command system** — command identity, action routing, and the
  shell contract layer. Currently hosted in `crates/aureline-shell-spike`
  while a non-throwaway home is chosen; the lane persists regardless.
- **Shell spike (throwaway)** — the spike crate itself
  (`crates/aureline-shell-spike`). Unprotected and time-boxed.
- **Benchmark lab** — `crates/aureline-bench`, `/fixtures/`, and the
  benchmark-council charter, corpora, and protected fitness references.
- **Release evidence** — `/artifacts/release/` plus
  `docs/release/qualification_cadence.md` and
  `docs/release/shiproom_runbook.md` (provenance, SBOMs,
  compatibility reports, claim manifests, qualification cadence and
  ownership maps, rollback/shiproom packets, shiproom dashboard seeds,
  and reviewer operating order).
- **Docs / public truth** — `/docs/`, `/README.md`, `/AGENTS.md`,
  `/CLAUDE.md`, external-facing copy, known-limits matrix,
  support-window statements, migration guides.
- **Support / export** — `/artifacts/support/` and `/schemas/support/`
  (support bundles with redaction rules, doctor probes, recovery
  ladder, export-safe packet schemas, the support-packet family index,
  field runbooks, and crash-diagnostics corpus).
- **Design-system seeds** — `/artifacts/ux/` design-system snapshots, token
  sources, and component references.
- **Accessibility / input review** — `/artifacts/ux/` accessibility audits,
  input-method review packets, reduced-motion / contrast artifacts.
- **Governance forums** — the standing forum families listed above, plus
  their packet-profile and output-routing rules in
  `docs/governance/forum_charters.md` and
  `artifacts/governance/forum_matrix.yaml`.

## Blocker aging SLAs

The canonical SLA table now lives in
[`/docs/governance/blocker_aging_slas.md`](./blocker_aging_slas.md).
Use that document for the active aging clock, the first containment
step, and the escalation timing on:

- architecture-freeze blockers;
- stale or rerun-triggered evidence;
- owner gaps on protected or release-blocking lanes; and
- expired or repeatedly renewed waivers.

The summary floors are unchanged:

- owner gaps must resolve to a named owner or waiver inside one
  business day, with a 4-hour floor for active support incidents;
- architecture-freeze blockers need an explicit resolution plan inside
  48 business hours;
- stale evidence downgrades the scorecard immediately and needs a rerun
  owner inside one business day; and
- waiver expiry is immediate containment, not a warning.

Under the current solo-maintainer posture, the concrete escalation path
is still the shiproom self-escalation log, then the contributor
community thread, then a public repository notice until named backup
coverage closes the waiver.

## Authority for narrowing, waivers, and re-baselining

Decisions below require the named authority. Any decision must be recorded
in the appropriate governance packet with the decider, date, reason, and
expiry.

| Decision                                                                  | Required authority                                        |
|---------------------------------------------------------------------------|-----------------------------------------------------------|
| Narrowing a non-protected lane for the current milestone.                 | Lane DRI.                                                 |
| Narrowing a protected lane for the current milestone.                     | Lane DRI plus architecture council.                       |
| Opening or renewing a backup-owner waiver on a protected lane.            | Lane DRI plus architecture council.                       |
| Opening or renewing a waiver on a protected fitness function (rows in [`/artifacts/bench/fitness_function_catalog.yaml`](../../artifacts/bench/fitness_function_catalog.yaml)). | Lane DRI plus performance council, with named expiry. |
| Opening or renewing a waiver on a public-truth claim.                     | Docs / public-truth DRI plus the claim's evidence owner.  |
| Re-baselining milestone scope, dates, or acceptance thresholds.           | Architecture council plus release council.                |
| Declaring a supportability family ready for end-user promotion.           | Support / export DRI plus release council.                |
| Closing a recurring waiver into a correction program.                     | Architecture council plus the affected lane DRI.          |
| Approving a late feature or schema change inside the release candidate.   | Shiproom / executive scope review.                        |

Under the solo-maintainer posture, the sole maintainer convenes each
forum as a single-attendee decision log. That arrangement is itself
waived by the single-maintainer waiver and closes when named chairs
populate the ownership matrix.

## Change discipline

- Changes to CODEOWNERS, this document, or the ownership matrix must land
  together when they disagree; tooling treats the matrix as the machine
  form and this document as the human-readable source.
- Adding a new protected lane requires: an entry here, a row in the
  ownership matrix, and, if it has code, an entry in the package
  inventory. All in the same change.
- Removing a waiver requires noting closure (date, reason) in the matrix
  rather than deleting the waiver entry.
