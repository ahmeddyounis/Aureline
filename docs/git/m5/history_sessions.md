# History-surgery sessions and first consumers

Risky Git flows — conflict resolution, sequence edits, stash/shelf entries,
publish/ref-update proposals, and recovery checkpoints — cannot remain thin
wrappers over shell commands or transient modal state. This contract turns each
one into a durable, serde-serializable product object with explicit identity and
lifecycle, so its state survives reopen, export, support, and provider
degradation. If a user can continue or abort an operation, Aureline can explain
and export it.

- Schema: [`schemas/git/history-session.schema.json`](../../../schemas/git/history-session.schema.json)
- Canonical map: [`artifacts/git/m5/history_sessions/history_session_first_consumers.json`](../../../artifacts/git/m5/history_sessions/history_session_first_consumers.json)
- Summary: [`artifacts/git/m5/history_sessions.md`](../../../artifacts/git/m5/history_sessions.md)
- Fixtures: [`fixtures/git/m5/history-sessions/`](../../../fixtures/git/m5/history-sessions)
- Code: `crates/aureline-git/src/history_sessions/`

The session kinds and their canonical record kinds are the ones frozen by the M5
repository-topology and history-surgery matrix; this lane is their durable
implementation and first consumers.

## Durable session object

Each `git_history_session_descriptor` records one risky object with explicit
identity and lifecycle rather than transient UI state:

| Field | Purpose |
|-------|---------|
| `session_kind` + `canonical_record_kind` | Which history-surgery object this is, bound to the frozen canonical record kind. |
| `repo_ref` + `worktree_ref` | Exact repository and worktree identity, preserved on every projection. |
| `lifecycle_state` | Lifecycle token from the closed vocabulary for the kind. |
| `target_refs` | Target revisions: base/ours/theirs for a conflict, the replay target for a sequence edit, the before/after ref positions for a publish. |
| `path_scope_tokens` | Redaction-safe affected/included path tokens; never raw paths. |
| `unresolved_count` | Count of unresolved conflict rows or blockers. |
| `checkpoint_lineage_refs` | Recovery checkpoint lineage protecting the mutation. |
| `raw_source_text_ref` + `structured_cards_ref` | Refs to the exact raw todo/patch text and the structured cards derived from it, where the workflow requires exact order or source-text inspection. |
| `available_actions` | Distinct action verbs bound to the object (continue/abort/skip, apply/pop/drop/create-branch, publish/withdraw, restore/prune). |

Publish proposals additionally carry `divergence_class`, `approval_state`,
`check_invalidation_state`, `publish_mode`, and the affected approval/check refs;
recovery checkpoints carry `trigger_kind` and `restore_option_classes`. The
descriptors carry only redaction-safe refs.

## One projection drives every consumer

`HistorySession::project` is the single, deterministic function that maps a
descriptor onto a consumer surface — desktop, review, search, AI context,
CLI/headless, support export, and provider overlay. Because each
`git_history_session_consumer_binding` is *derived* from its descriptor, the
checked-in map's validation re-derives every binding to prove the same object
drives every surface, and three guardrails hold by construction:

- **Distinct stash verbs.** Apply, pop, drop, and create-branch from a stash are
  disclosed on every surface, and marked actionable only on mutation surfaces —
  they never collapse into one verb.
- **No silent publish.** A publish/ref-update proposal allows a network mutation
  only when its divergence is known, its affected approvals and checks are not
  invalidated, its recovery lineage is present, and it is explicitly ready to
  publish — and even then only on a mutation surface.
- **Recovery before mutation.** Every mutating session keeps a reachable recovery
  path (an explicit checkpoint or an acknowledged reflog-only fallback); the
  recovery checkpoint is itself the recovery surface.

## Support export

The redaction-safe `git_history_session_support_export` reconstructs every
session from `session_kind`, `repo_ref`, `worktree_ref`, `target_refs`,
`path_scope_tokens`, `unresolved_count`, `checkpoint_lineage_refs`, and
`lifecycle_state`. Raw paths, raw patch/todo bodies, and raw provider payloads
never cross the boundary.
