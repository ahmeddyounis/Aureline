# M5 Git Certification Register

- Packet: `m5-git-certification:0001`
- Schema: `schemas/git/certify-m5-git-topology-history-recovery-and-provider-parity-rows.schema.json`
- Support export: `artifacts/git/m5/certify_m5_git_topology_history_recovery_and_provider_parity_rows/support_export.json`
- Contract doc: `docs/git/m5/certify_m5_git_topology_history_recovery_and_provider_parity_rows.md`
- Fixtures: `fixtures/git/m5/certification-corpus/`

## What this register certifies

This register is the claim-hardening capstone over the M5 Git depth lane. Every
claimed M5 Git and source-acquisition row stays green only if it can prove, under
current evidence, all four certification dimensions:

- **Topology honesty** — current topology (sparse, partial clone, shallow,
  submodule, nested, worktree, LFS) is reported truthfully; omitted is never
  silently completed to missing or complete.
- **Worktree / root scoping** — status, search, blame, and mutation target the
  correct worktree or root; the wrong-root guard blocks ambient bulk mutation.
- **History-surgery preview / recovery** — any history rewrite is previewed and
  a recovery checkpoint (or a disclosed reflog-only fallback) stays reachable.
- **Local / provider parity** — local Git truth stays authoritative when a
  provider overlay is stale, degraded, or absent.

## Rows (8)

1. **Source acquisition and topology initialization** — clone/open/initialize/hydrate.
2. **Repository topology honesty** — honest sparse/partial/shallow/submodule/nested/LFS truth.
3. **Worktree and root scoping** — scope status/search/blame/mutation to the right root.
4. **Topology-aware search, AI context, and review parity** — one topology vocabulary across surfaces.
5. **History-surgery preview and recovery** — rebase/cherry-pick/reset/revert preview + checkpoint.
6. **Stash, reflog, and checkpoint recovery** — distinct stash verbs + restorable anchors.
7. **Conflict-resolution continuity** — resume resolution across reopen/restart with provenance.
8. **Publish and provider parity** — previewed ref update, rollback, provider-degraded local continuity.

The history-surgery dimension is marked not-applicable on the read/scope rows
(1–4), which perform no history rewrite, and applicable on the mutating rows
(5–8). Every other dimension is applicable on every row.

## Fail-closed downgrade automation

A row's verdict is derived from its dimensions, never declared independently:

| Dimension evidence | Verdict contribution |
| --- | --- |
| Current + proven | `certified` |
| Current + honestly partial | `limited` |
| Stale, or not yet run | `retest_pending` |
| Failed, or evidence missing | `unsupported` |

The worst applicable dimension wins. Validation rejects any packet whose declared
verdict does not match its evidence, so a stale or failed dimension cannot be
hand-waved back to certified. The `downgrade_automation` block binds these
narrowing targets to the same semantics and asserts that narrowing propagates
into docs/help, support packets, evaluation packs, and claim-publication
manifests, and that release/public-truth surfaces stop overclaiming on a slip.

## Parity and consumer surfaces

The `parity_audit` block proves product, docs/help, CLI, support export,
evaluation packs, claim-publication manifests, and release/public-truth all
reflect the same row verdicts, that no surface advertises wider than its current
row, and that local truth is authoritative over any provider overlay.

## Degraded corpus

The protected fixtures demonstrate the automation end to end:

- `stale_topology_retest_pending.json` — a stale topology dimension narrows the
  topology-honesty row to `retest_pending`.
- `failed_provider_parity_unsupported.json` — a failed provider-parity dimension
  narrows the publish row to `unsupported`.
- `partial_history_recovery_limited.json` — an honestly partial history-recovery
  dimension narrows the history-surgery row to `limited`.

## Regeneration

```
cargo run -p aureline-git --example dump_m5_git_certification              > artifacts/git/m5/certify_m5_git_topology_history_recovery_and_provider_parity_rows/support_export.json
cargo run -p aureline-git --example dump_m5_git_certification -- stale-topology   > fixtures/git/m5/certification-corpus/stale_topology_retest_pending.json
cargo run -p aureline-git --example dump_m5_git_certification -- failed-parity    > fixtures/git/m5/certification-corpus/failed_provider_parity_unsupported.json
cargo run -p aureline-git --example dump_m5_git_certification -- partial-history  > fixtures/git/m5/certification-corpus/partial_history_recovery_limited.json
```

## Boundary

Certification truth is never reduced to a badge: the rows decide whether a claim
may be published. Raw paths, raw object bytes, raw branch names, raw
patch/reflog/stash bodies, raw provider payloads, and credentials never cross
this boundary.
