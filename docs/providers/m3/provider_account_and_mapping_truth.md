# Provider Account/Session and Board/Project-Mapping Truth

Provider-backed work items, reviews, incidents, and publish-later drafts now
have a typed mapping-review beta page in `aureline-provider`. The account-scope
beta made provider *authority* honest — who Aureline acts as, under which
grant, and with what effective write scope. This page makes provider
*targeting* honest. Before Aureline proposes or executes a hosted mutation on a
claimed beta provider lane, every row answers three questions:

1. **Who is acting?** Each row embeds one governed account/session binding that
   names the acting-identity class (connected human account, installation/app
   grant, or delegated credential), the bound account-scope identity row,
   whether the account was policy-selected, the first-class provider session
   state, and the effective write scope.
2. **What target will be touched?** Each row carries a typed mapping resolution
   state and, when one resolves, the board, project, space, repository, or
   incident queue the comment, issue update, review action, or incident handoff
   will land on — including its container path. Policy-locked and
   unsupported-remap cases are first-class states, never silently disabled
   controls.
3. **Is the next action local, queued, or live?** Each row carries a publish
   posture distinguishing a local draft, a queued publish-later item, a live
   provider mutation, and a read-only inspection, plus the concrete next-safe
   action that moves the row forward.

## First-class session and mapping states

Limited-scope, stale-credential, read-only, offline-capture, and
publish-later-only sessions are preserved as distinct lanes rather than
collapsed into a generic "unavailable" message. A live provider mutation is
admitted only when a single target is resolved (or policy-locked), the session
is live, and effective write scope is held; otherwise the row degrades to a
local draft, a queued publish-later item, or read-only inspection with a named
reason and next-safe action.

When a previously resolved mapping becomes invalid — a target is archived or
deleted, a session goes offline, a credential goes stale, or the provider
refuses a remap — a mapping-invalidation event records the drop. The drop
preserves local drafts, queued transitions, and evidence attachments, keeps
retry/export available, and never silently keeps a live mutation posture.

## Schemas

- `schemas/providers/provider_target_mapping.schema.json` — the mapping page.
- `schemas/providers/provider_account_scope.schema.json` — the embedded
  account/session binding object.

The full account-scope beta page (connected account, installation grant,
delegated credential, effective-scope resolution, and scope-drift events)
remains authoritative in `schemas/providers/effective_scope.schema.json`; the
mapping rows here bind to those account-scope identity rows by ref. The Rust
boundary is `crates/aureline-provider/src/project_mapping/`.

## Fixtures

`fixtures/providers/m3/account_scope_and_mapping/` holds the seeded page, the
redaction-safe support export, and an enum-only corpus matrix.

## Verification

```sh
cargo run -p aureline-provider --bin aureline_provider_target_mapping_beta -- page
cargo run -p aureline-provider --bin aureline_provider_target_mapping_beta -- validate
cargo test -p aureline-provider --test target_mapping_beta
```

## Support export

```sh
cargo run -p aureline-provider --bin aureline_provider_target_mapping_beta -- support-export
```

The support export is metadata-only: account/session identity lineage,
board/project/space/repository target lineage, publish-posture lineage, and
mapping-invalidation trigger and forced-posture lineage are preserved verbatim,
while raw access tokens, raw delegated-token bodies, raw provider payloads, and
raw provider URLs are excluded because the beta projection never carries them.
