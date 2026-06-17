# M5 Git Certification Register (contract)

This document is the human-readable contract for the M5 Git certification
register. The machine-readable boundary is the schema and the checked support
export:

- Schema: `schemas/git/certify-m5-git-topology-history-recovery-and-provider-parity-rows.schema.json`
- Support export: `artifacts/git/m5/certify_m5_git_topology_history_recovery_and_provider_parity_rows/support_export.json`
- Typed model: `crates/aureline-git/src/certify_m5_git_topology_history_recovery_and_provider_parity_rows/`

## Purpose

The earlier M5 Git rows — repository topology, worktree/root scoping,
history-surgery preview/recovery, stash/reflog/checkpoint recovery, and
provider-degraded local continuity — only matter if every *claimed* M5 Git or
source-acquisition surface can prove, under current evidence, that it is honest
about topology, scopes operations to the right root, previews and can recover any
history rewrite, and keeps local Git truth authoritative when a provider overlay
is degraded or absent.

This register makes that proof a governed product object. It is the single source
of truth for whether a claimed Git/source-acquisition row may keep its published
claim or must narrow. It references the frozen topology/history matrix, the
topology, topology-action, history-surgery, and stash-recovery contracts by id
instead of redefining them, so product, docs/help, CLI, support, evaluation
packs, claim-publication manifests, and release/public-truth surfaces all read
one register.

## Certification dimensions

Every claimed row carries exactly one entry per dimension:

- **`topology_honesty`** — current topology is reported truthfully; omitted stays
  distinct from missing and from complete.
- **`worktree_root_scoping`** — operations target the correct worktree or root;
  the wrong-root guard blocks ambient bulk mutation.
- **`history_surgery_preview_recovery`** — a history rewrite is previewed and a
  recovery checkpoint, or a disclosed reflog-only fallback, stays reachable. This
  dimension is *not applicable* on rows that perform no history rewrite.
- **`local_provider_parity`** — local Git truth stays authoritative when the
  provider overlay is stale, degraded, or absent.

Each dimension entry records evidence `freshness` (`current` / `stale` /
`missing`), a `proof_state` (`proven` / `narrowed` / `failed` / `not_run`), the
`evidence_refs` that back it, and a `summary`.

## Claimed rows

| Row | History rewrite? |
| --- | --- |
| Source acquisition and topology initialization | no |
| Repository topology honesty | no |
| Worktree and root scoping | no |
| Topology-aware search, AI context, and review parity | no |
| History-surgery preview and recovery | yes |
| Stash, reflog, and checkpoint recovery | yes |
| Conflict-resolution continuity | yes |
| Publish and provider parity | yes |

## Fail-closed verdict derivation

A row's `verdict` is derived from its dimensions and validated against the
declared value, so the artifact cannot lie:

| Dimension evidence | Contribution |
| --- | --- |
| `proven` + `current` | `certified` |
| `narrowed` + `current` | `limited` |
| `not_run`, or any `stale` | `retest_pending` |
| `failed`, or any `missing` | `unsupported` |

The worst applicable dimension wins. A row that is not `certified` MUST name a
`narrowing_reason`; a `certified` row MUST NOT. A row with no applicable
dimension is `unsupported` (it certifies nothing).

## Downgrade automation

The `downgrade_automation` block binds the narrowing targets to the derivation
above (`stale_or_unrun_narrows_to: retest_pending`, `partial_narrows_to:
limited`, `failure_or_missing_narrows_to: unsupported`) and asserts that
narrowing propagates into docs/help, support packets, evaluation packs, and
claim-publication manifests, and that release/public-truth surfaces stop
overclaiming when a row slips. Claim truth is therefore never a manual flag.

## Parity audit

The `parity_audit` block proves product, docs/help, CLI, support export,
evaluation packs, claim-publication manifests, and release/public-truth all
reflect the same row verdicts, that no surface advertises wider than its current
machine-readable row, and that local truth is authoritative over any provider
overlay.

## Freshness posture

`freshness_posture` records the review SLO, the last review timestamp, whether
stale evidence auto-narrows, and whether the evidence validity window is open.
A closed window or a missing review fails validation.

## Degraded corpus

`fixtures/git/m5/certification-corpus/` holds full, schema-valid packets that
exercise each narrowing path: a stale topology dimension (`retest_pending`), a
failed provider-parity dimension (`unsupported`), and an honestly partial
history-recovery dimension (`limited`).

## Boundary

Certification truth is never reduced to a badge: the rows decide whether a claim
may be published. Raw paths, raw object bytes, raw branch names, raw
patch/reflog/stash bodies, raw provider payloads, and credentials never cross
this boundary.
