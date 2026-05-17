# Preview Drift And Expiry Guard

This contract defines the beta guard used by destructive, remote, publish, and
provider-linked apply paths to prove the reviewed basis still matches the commit
target.

## Canonical Artifacts

- Runtime model: `crates/aureline-runtime/src/preview_drift/`
- Boundary schema: `schemas/approvals/preview_commit_guard.schema.json`
- Replay fixtures: `fixtures/trust/m3/preview_drift/`
- Runtime replay test: `crates/aureline-runtime/tests/preview_drift_beta.rs`

## Guard Basis

Every `preview_commit_guard_record` binds these dimensions before apply:

- target bindings: opaque target refs plus export-safe target hashes
- selected scope: selected files, filters, workset, or provider object set
- host boundary: local, remote, container, managed, or provider boundary
- route binding: transport, tunnel, provider route, or local VFS route
- policy snapshot: policy ref, epoch, and snapshot hash
- approval ticket: ticket ref, ticket hash, state, and expiry
- representation class: source diff, rendered preview, metadata, raw text,
  provider object, or safe-preview snapshot
- lifecycle state: preview, approval, ready, applying, superseded, unavailable,
  or disconnected

The `basis_hash` is a stable digest over those export-safe fields. It is not a
hash of raw source, provider payloads, URLs, command lines, secrets, or token
bodies.

## Commit Rule

Apply must call the guard evaluator immediately before committing. If any
material dimension differs, the evaluator returns
`admission_decision=block_require_review`, `blocks_apply=true`,
`requires_re_review=true`, and `may_auto_refresh=false`.

Re-review is required for:

- target set or target identity drift
- selected scope or filter drift
- host boundary or route drift
- policy epoch or snapshot drift
- missing, expired, revoked, or drifted approval ticket
- remembered decision without a fresh short-lived ticket
- lifecycle drift or superseded previews
- representation-class drift
- preview freshness expiry

## Surface And Export Contract

The evaluator emits the same reason tokens in four places:

- product surface projection (`preview_commit_surface_projection_record`)
- CLI/headless projection (`PreviewCommitCliOutput`)
- support export (`preview_commit_guard_support_export_record`)
- audit event (`preview_commit_guard_audit_event_record`)

Support packets can therefore explain why apply was blocked without private
tooling. They carry refs, class tokens, hashes, and reason tokens only.

## Replay Coverage

The fixture corpus covers:

- destructive local apply blocked by target movement
- remote mutation blocked by host and route drift
- publish action blocked by approval expiry
- provider-linked action blocked by representation drift

Each case proves stale review data cannot reach disk, remote targets,
registries, or provider surfaces without renewed review.
