# M5 stash entries and reflog-recovery banners: restore-scope and checkpoint truth

This lane (M05-958) narrows the two **recovery** components frozen in the
[M5 Git-history and risky-mutation component matrix](./freeze_the_m5_git_history_sequence_component_matrix.md)
— `stash_entry` and `reflog_recovery_banner` — into an implemented, export-safe
row contract so every claimed M5 recovery surface can render restore scope,
distinct restore verbs, and a concrete recovery destination without copying
per-screen chrome.

- Rust module: `crates/aureline-git/src/implement_stash_entries_and_reflog_recovery_banners/`
- Boundary schema: [`schemas/ui/m5-stash-reflog-recovery-component.schema.json`](../../../schemas/ui/m5-stash-reflog-recovery-component.schema.json)
- Checked support export: [`artifacts/release/m5-stash-reflog-recovery-components-proof/support_export.json`](../../../artifacts/release/m5-stash-reflog-recovery-components-proof/support_export.json)
- Protected fixtures: [`fixtures/ui/m5-stash-reflog-recovery-components/`](../../../fixtures/ui/m5-stash-reflog-recovery-components/)

## Goal

Keep stash and reflog recovery flows explicit enough that users never mistake one
restore verb for another, and keep recovery reachable after a risky mutation.
Every stash entry carries its message, created-from ref, untracked/staged scope,
and the four exact restore verbs; every reflog-recovery banner links a risky
mutation to a concrete checkpoint destination, discloses its expiry state, and
stays reachable across surfaces until it is superseded or dismissed.

## Stash entries: the `stash@{n}`-shorthand-is-never-the-only-label axis

A `StashEntryRow` must carry a human message **and** a created-from ref, so the
`stash@{n}` shorthand is never the only meaning-bearing label
(`StashShorthandOnlyLabel` — also fires when the message merely echoes the
shorthand). The untracked/staged scope is spelled out both as a
`StashContentScope` enum and as a human `scope_disclosure`
(`StashScopeDisclosureMissing`), so a restore never surprises the user with
untracked files it silently swept in or index state it kept.

The four restore verbs stay distinct and complete:

| Verb | Keeps stash? | Restores? |
| --- | --- | --- |
| `apply` | yes | yes |
| `pop` | no (removes) | yes |
| `drop` | no (removes) | no (discards) |
| `create_branch_from_stash` | yes | yes (on a new branch) |

Every entry must expose all four (`RestoreVerbCoverageMissing`) and must not
alias/duplicate them into one ambiguous restore (`RestoreVerbsCollapsed`), since
`stash_entry` is a risky-mutation surface in the frozen matrix.

## Reflog-recovery banners: the reachable-until-superseded-or-dismissed axis

`resolve_recovery_banner_disclosure(reachability, expiry_state)` derives what a
banner must disclose. A `reachable` banner must show a **concrete** recovery
destination (`RecoveryDestinationMissing`) and stay reachable from Git history,
review, and help/support surfaces (`RecoveryNotReachableAcrossSurfaces`), so
recovery remains reachable from all three claimed surfaces after a risky mutation.
Every banner discloses its expiry state (`ExpiryStateUndisclosed`), and a banner
whose recovery point has `expired` or been `pruned` can never keep claiming to be
reachable (`ExpiredRecoveryStillReachable`) — it must narrow to `superseded` or
`dismissed`. At least one reachable banner must be present
(`RecoveryReachabilityCoverageMissing`).

## Reuse

- `M5GitHistoryComponent` gates each row's `component` (stash rows must be
  `stash_entry`, banner rows must be `reflog_recovery_banner`).
- `GitHistoryDowngradeState` (the shared matrix downgrade vocabulary) is reused
  for both per-row `downgrade_vocab` and packet-level `downgrade_triggers`.
- `ComponentConsumerSurface` (the shared matrix consumer surfaces) is reused for
  `consumer_surfaces`.

## Acceptance criteria mapping

- **`stash@{n}`-style shorthand never becomes the only meaning-bearing label** —
  `StashShorthandOnlyLabel` + `stash_shorthand_never_only_label` +
  `has_meaning_beyond_shorthand`.
- **Recovery remains reachable from Git history, review, and help/support after
  risky mutations until superseded or dismissed** —
  `RecoveryNotReachableAcrossSurfaces` + `RecoveryDestinationMissing` +
  `ExpiredRecoveryStillReachable` + `RecoveryReachabilityCoverageMissing`.

## Regenerating artifacts

The checked export, Markdown summary, and narrowed fixtures are produced by the
`generate_artifacts` test, gated behind an env var so it is inert in CI:

```
GEN_STASH_REFLOG_RECOVERY_ARTIFACTS=1 cargo test -p aureline-git --lib \
  implement_stash_entries_and_reflog_recovery_banners::tests::generate_artifacts
```

`checked_export_matches_seed` asserts the checked JSON equals the in-Rust seed
packet, so the artifact can never drift from the contract.
