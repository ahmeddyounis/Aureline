# M5 Git-History Component Accessibility, Headless, and Export Parity

Contract doc for the accessibility / headless / export capstone over the twelve
shared M5 Git-history and risky-mutation components (commit-graph header,
history-graph row, branch-comparison chip, worktree row, stash entry,
reflog-recovery banner, rebase-todo row, sequence-editor header, cherry-pick/revert
review sheet, patch-apply review sheet, conflict-checkpoint card, and force-push
review dialog).

This lane closes B113. Where the consumer-adoption lane
(`add_shared_history_sidebar_review_workspace_command_help_support_and_export_consumers_...`)
proves ref / worktree / recovery / verb parity across desktop surfaces, this lane
proves the harder claim: commit / worktree / stash / sequence / patch / recovery
state is exposed just as honestly in assistive, headless, and exported forms as it is
on desktop, and a claim-bearing component automatically narrows the moment repo
topology, checkpoint availability, or provider-linked recovery truth stops being
trustworthy.

- Boundary schema:
  [`schemas/ui/m5-git-history-component-accessibility-parity.schema.json`](../../../schemas/ui/m5-git-history-component-accessibility-parity.schema.json)
- Rust module:
  `crates/aureline-git/src/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_repo_topology_checkpoint_availability_or_provider_linked_recovery_truth_is_partial_or_stale_across_claimed_m5_git_history_components`
- Release proof:
  [`artifacts/release/m5-git-history-component-accessibility-proof/`](../../../artifacts/release/m5-git-history-component-accessibility-proof/)
- Protected fixtures:
  [`fixtures/ui/m5-git-history-component-accessibility-parity/`](../../../fixtures/ui/m5-git-history-component-accessibility-parity/)

## Honesty axes

### AC1 — parity across keyboard / screen-reader / CLI / export

Every claimed component row exposes five accessibility fields (keyboard label,
screen-reader label, CLI enum token, export enum token, explanation field) and must
render on all three rendering surfaces (`desktop_full`, `cli_headless`,
`support_export`). Three guardrail booleans must be false on every row:
`is_pointer_only`, `is_export_opaque`, and `desktop_stronger_than_cli`. No claimed M5
Git-history component may become pointer-only, export-opaque, or semantically stronger
on the desktop than it is in CLI or support output.

### AC2 — automatic narrowing prevents overstated recovery / mutation safety

Each row carries an `effective_claim` drawn from `GitHistoryClaimTier`
(strongest → weakest): `recoverable_in_product`, `locally_recoverable`,
`partial_history_only`, `reflog_only_recovery`, `local_continue_only`. The
`condition` governing the row pins a ceiling:

| Condition | Permitted ceiling | Required disclosure |
| --- | --- | --- |
| `local_truth_aligned` | `recoverable_in_product` | none |
| `provider_review_state_stale` | `locally_recoverable` | narrowing + local-continue note |
| `repo_topology_partial` | `partial_history_only` | narrowing + topology note + local-continue note |
| `checkpoint_recovery_unavailable` | `reflog_only_recovery` | narrowing + recovery note + local-continue note |
| `offline_local_only` | `local_continue_only` | narrowing + local-continue note |

An effective claim whose strength rank exceeds its condition's ceiling raises
`claim_ceiling_exceeded`. Every weakening condition must disclose an explicit
`narrowing` naming the downgrade trigger, the narrowed-to tier, a preserved-truth
note, and a next action. Partial topology keeps the incomplete-history truth spelled
out; an unavailable checkpoint keeps the reflog-only recovery destination named; every
weakening condition preserves a local-continue path so the reviewer's history work
never vanishes.

## Coverage

The canonical packet carries twelve rows — one per shared component — covering all
five conditions and all five claim tiers. Coverage violations fire when a component,
condition, or claim tier is absent.

## Regeneration

Regenerate the checked-in support export, summary, and fixtures with:

```
GEN_GIT_HISTORY_ACCESSIBILITY_ARTIFACTS=1 cargo test -p aureline-git --lib \
  regenerate_git_history_component_accessibility_artifacts
```

Then review the diff. Raw paths, object bytes, branch names, patch/reflog/stash
bodies, provider payloads, and credentials stay outside the support boundary.
