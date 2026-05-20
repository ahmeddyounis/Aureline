# Offline / Cached Learnability Audit

## Scope

This audit proves that teaching/classroom sessions stay honest about
offline, cached, mirrored, and not-installed docs packs during guided learning.
The exit-gate concern is that a guided session must never pretend remote
enrichment is available when it is not. Every teaching segment carries an
explicit docs-pack state, and every degraded state carries a user-visible
disclosure.

The audit is derived from the same seeded corpus the runtime and fixtures use
(`fixtures/help/m3/teaching_classroom/corpus.json`), so it cannot drift from the
shipped behavior.

## Docs-Pack States

| State | Locally available | Stale | Requires reconnect | Requires install | Disclosure required |
| --- | --- | --- | --- | --- | --- |
| `installed` | yes | no | no | no | no |
| `cached` | yes | yes | no | no | yes |
| `mirrored` | yes | yes | no | no | yes |
| `offline` | yes (local content) | yes | yes | no | yes |
| `not_installed` | no | — | no | yes | yes |

`installed` is the only state that needs no disclosure. Every other state sets
`docs_pack_disclosure_ref` so the UI stays explicit about what is locally
available, what is stale, and what requires reconnect or install.

## Where Each State Is Exercised

| State | Seeded segment | Docs pack | What the session shows |
| --- | --- | --- | --- |
| `installed` | `segment:installed-01:1` (and the rest of the installed walkthrough) | `docs-pack:aureline-help:guided-tours` | Current content, no disclosure. |
| `cached` | `segment:cached-02:1` | `docs-pack:mirror:aureline-guided-learning` | Cached label visible; content is stale-but-disclosed. |
| `mirrored` | `segment:mirrored-04:1` | `docs-pack:mirror:aureline-guided-learning` | Offline-mirror label visible; mirror snapshot date is called out. |
| `offline` | `segment:offline-03:1` | `docs-pack:aureline-help:graph-map-placeholder` | Local content shows with a reconnect-required disclosure instead of a live remote graph. |
| `not_installed` | `segment:not-installed-05:2` | `docs-pack:mirror:aureline-guided-learning` | Content blocked behind an explicit install disclosure rather than faked enrichment. |

## What This Audit Asserts

For every seeded session and every segment:

- The segment names a `docs_pack_ref` and a `docs_pack_state`.
- If the state is not `installed`, the segment carries a non-empty
  `docs_pack_disclosure_ref` (`docs_pack_states_disclosed()` holds for the
  session). The `corpus_covers_every_role_state_pack_and_restore_trigger` and
  `degraded_docs_pack_states_stay_visible_and_disclosed` replay tests fail if any
  degraded state hides its disclosure.
- An `offline` state reports `requires_reconnect()`, so fresh enrichment is
  honestly gated behind reconnecting to the live source.
- A `not_installed` state reports `requires_install()` and is **not**
  locally available, so its content is blocked behind an explicit install.
- The cited learning-mode objects (tours, exercise steps, docs nodes, graph
  nodes) are the same ids learning mode ships, so cached/mirrored/offline content
  is a disclosed projection of canonical truth, not a separate copy source.

## Constrained-Client Behavior During Degraded States

Limited and low-bandwidth clients join degraded-state sessions as observers or
note-takers. They are never handed a drive or mutation control they cannot use;
heavy live affordances are omitted rather than rendered disabled. Read-only
open-cited-docs and (where the role allows) note-taking remain available, so a
constrained guest stays productive even while a docs pack is offline or cached.

## Coverage Summary

The seeded corpus exercises all five docs-pack states across teaching and
classroom sessions, paired with every replay policy and retention class and all
three restore triggers, so offline/cached learnability truth is proven across the
full session matrix rather than a single happy path.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_teaching_session -- corpus
cargo test -p aureline-shell --test teaching_classroom_beta_fixtures \
  degraded_docs_pack_states_stay_visible_and_disclosed
```
