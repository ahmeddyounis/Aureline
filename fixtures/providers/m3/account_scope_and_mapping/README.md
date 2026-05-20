# Provider account/session and board/project-mapping truth fixtures

This directory covers the provider-account/session binding and the
board/project/space/repository mapping-review surface for the beta provider
lanes (issue/work-item, review, incident handoff, and publish-later).

The executable fixture source is the seeded page emitted by:

```sh
cargo run -p aureline-provider --bin aureline_provider_target_mapping_beta -- page
cargo run -p aureline-provider --bin aureline_provider_target_mapping_beta -- support-export
cargo run -p aureline-provider --bin aureline_provider_target_mapping_beta -- validate
```

- `page.json` — the seeded target-mapping beta page. Every mapping-review row
  embeds one governed account/session binding (who Aureline acts as), a typed
  mapping resolution state and the target it would touch, and a publish posture
  distinguishing local draft, queued publish-later, live provider mutation, and
  read-only inspection.
- `support_export.json` — the redaction-safe support export. Identity, target,
  posture, and invalidation lineage are preserved verbatim; raw access tokens,
  raw delegated-token bodies, raw provider payloads, and raw provider URLs are
  excluded.
- `corpus_matrix.json` — the enum-only matrix pinning the drill classes the
  seeded page validates.

## What the corpus pins

- **Account/session truth.** Each row names the acting-identity class
  (connected account, installation grant, delegated credential), the bound
  account-scope identity row, whether the account was policy-selected, the
  first-class provider session state (live, limited-scope, stale-credential,
  read-only, offline-capture, publish-later-only), and the effective write
  scope.
- **Mapping truth.** Resolved-single-target, policy-locked-target,
  ambiguous-needs-selection, unsupported-remap, stale-needs-refresh, and
  invalidated states each occupy a distinct lane. Policy-locked and
  unsupported-remap cases surface concrete reasons and next-safe actions, never
  a silently disabled control.
- **Posture truth.** Local-draft, queued-publish-later, live-provider-mutation,
  and read-only-inspection postures are distinct. A live mutation is only
  admitted when a single target is resolved, the session is live, and effective
  write scope is held.
- **Durable continuity.** The invalidation drills (incident queue archived,
  mirror credential went stale) prove that when a mapping becomes invalid, the
  row drops to a non-live posture while local drafts, queued transitions, and
  evidence attachments stay durable and a next-safe action is named.

## Boundary schemas

- `schemas/providers/provider_target_mapping.schema.json` — the mapping page.
- `schemas/providers/provider_account_scope.schema.json` — the embedded
  account/session binding object.

The full account-scope beta page (connected account, installation grant,
delegated credential, effective-scope resolution, and scope-drift events)
remains authoritative in `schemas/providers/effective_scope.schema.json`; the
mapping rows here bind to those account-scope identity rows by ref.

The corpus matrix is enum-only so support/export packets never carry raw
comment text, project names, or provider URLs.
