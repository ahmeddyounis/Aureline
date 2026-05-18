# Beta Test Identity Ledger

This document is the reviewer-facing landing page for the promoted test
identity ledger. The machine-readable boundary lives at
[`/schemas/testing/test_item.schema.json`](../../../schemas/testing/test_item.schema.json).
The runtime implementation lives at
[`/crates/aureline-runtime/src/testing_identity/`](../../../crates/aureline-runtime/src/testing_identity/).

The ledger sits above pytest discovery, alpha test-attempt packets, and beta
test-runner projections. It does not replace those records. It joins them into
one exportable model where every claimed surface points at the same canonical
test item, session, selector, and latest attempt.

## Runtime Contract

For each claimed runnable test item, the bundle emits:

- `canonical_test_item_record`: durable item identity, adapter ref, selector
  ref, source anchor digest, display-label digest, and remap refs.
- `test_selector_binding_record`: an import-safe `id:` selector for CLI,
  command palette, review, support, and CI-overlay flows. Display text is
  explicitly forbidden as a selector key.
- `canonical_test_session_record`: selection origin, target/environment
  fingerprint, imported-CI truth class, ordered attempt refs, and surface refs.
- `canonical_test_attempt_record`: append-only attempt identity with lineage,
  predecessor/origin refs, execution-context and target/environment identity,
  evidence class, freshness class, timestamps, artifacts, and raw-event refs.
- `test_surface_identity_binding_record`: editor inline, test tree, CLI,
  command palette, review, support, and imported-CI overlay rows that all cite
  the same item/session/selector/latest-attempt tuple.
- `imported_ci_truth_overlay_record`: imported provider evidence kept separate
  from local live truth until a local confirmation attempt exists.

## Surface Rule

A surface may show a short label, but it must not use that label as identity.
The shell and support exports must carry these ids together:

| Surface | Required identity refs |
| --- | --- |
| Editor inline marker | canonical item, selector, session, latest attempt |
| Test tree row | canonical item, selector, session, latest attempt |
| CLI/headless selector | selector binding plus canonical item |
| Command palette | selector binding plus canonical item |
| Review packet | canonical item, selector, session, attempt lineage |
| Imported CI overlay | canonical item, session, imported truth class, freshness |
| Support export | all item, session, attempt, selector, surface, and overlay refs |

`TestIdentityBetaBundle::surface_bindings_resolve_to_same_ids` is the runtime
check used by tests to prove these rows agree without display-name matching.

## Attempt Ledger Rule

Attempts are append-only. Retry, rerun-last, rerun-failed, debug-from-test, CI
import, and local confirmation attempts add records; they do not replace prior
attempts. `CanonicalTestAttempt::follow_up` and
`TestIdentityBetaBundle::append_attempt` enforce the next attempt index and
reject duplicate attempt ids.

Support and release packets can reconstruct what ran from:

- `test_session_id`
- `canonical_test_item_id`
- `selector_ref`
- `execution_context_ref`
- `target_id`
- `environment_fingerprint`
- ordered `attempt_refs`
- artifact and raw-event refs

Raw logs remain additive evidence rather than the default reconstruction path.

## Imported CI Truth

Imported CI evidence has its own truth class:

| Class | Product meaning |
| --- | --- |
| `not_imported` | Local test evidence only. |
| `imported_current_read_only` | Provider evidence is current for the provider but read-only locally. |
| `imported_stale_read_only` | Provider evidence is stale and read-only locally. |
| `local_confirmation_required` | Imported evidence needs a local run before local claims. |
| `confirmed_by_local_rerun` | Fresh local evidence confirmed the imported relationship. |
| `unknown_requires_review` | Fail closed. |

Only `confirmed_by_local_rerun` may set
`current_local_truth_claim_allowed = true` on an imported overlay. Imported
rows remain visibly labelled `imported` or `stale` until that local
confirmation exists.

## Verification

The focused runtime test exercises:

- same item/session/selector/latest-attempt ids across editor, tree, CLI,
  review, support, and imported-CI overlay bindings;
- append-only attempt history;
- imported CI evidence staying imported until local confirmation;
- no claimed pytest row falling back to display-text-only identity.

```sh
cargo test -p aureline-runtime --test testing_identity_beta
```

The shell projection test verifies UI-facing tree and inline rows preserve the
runtime selector and session refs:

```sh
cargo test -p aureline-shell test_runner_beta
```
