# Provider work-item link, comment-sync, and publish-review fixtures

This directory covers provider-backed work-item link state, comment-sync
state, and publish-review sheet drills for the beta provider lanes.

The executable fixture source is the seeded page emitted by:

```sh
cargo run -p aureline-provider --bin aureline_provider_work_item_sync_beta -- page
```

The compact corpus matrix in `corpus_matrix.json` pins the drill classes
that the seeded page validates:

- **Link cases.** Branch and review link kinds across provider-confirmed,
  local-draft-pending, queued-for-publish-later, unlink-requested, conflict,
  and imported-handoff postures.
- **Comment cases.** Provider-owned, local-draft, queued, offline-captured,
  publish-failed, and conflicted comment sync states with explicit
  publish posture and conflict class.
- **Publish-review cases.** Create/edit/delete comment, link/unlink,
  status-transition-plus-comment, and retry-after-conflict sheets with
  source class, actor scope, disposition, and publish mode.

The corpus matrix is enum-only so support/export packets never carry raw
comment text, project names, or provider URLs.
