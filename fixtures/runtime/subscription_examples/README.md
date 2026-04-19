# Subscription decision-example fixtures

These fixtures are short, reviewable scenarios that anchor the
subscription-envelope fields, the freshness and stale-reason
vocabulary, and the protected-hot-path hook names defined in
[ADR 0005](../../../docs/adr/0005-subscription-envelope-and-invalidation-semantics.md)
to concrete inputs and observable outcomes. They are not a test
suite; they are the vocabulary the shell, VFS, editor, search,
graph, review, AI, CLI, and support-export lanes use when they
instrument a hook or a code path.

**Scope rules**

- Every fixture names the envelope fields or hooks it exercises,
  the surface it stresses, and the observable outcome
  instrumentation should capture.
- Fixtures never assert latency numbers; the benchmark lab owns
  budgets. Fixtures only describe *what* to measure, not *how
  fast*.
- Fixtures describe the logical envelope contents as JSON that
  validates against
  [`/schemas/runtime/subscription_envelope.schema.json`](../../../schemas/runtime/subscription_envelope.schema.json);
  they do not encode wire bytes or the enclosing ADR-0004 event
  envelope.
- A new fixture MUST exercise at least one protected-hot-path hook
  or one frozen envelope field and MUST cite the ADR section that
  motivates it.

**Index**

| Fixture                            | Primary hooks / fields                                                                                 | Surface stressed                                                           |
|------------------------------------|--------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------|
| [`ready.json`](./ready.json)       | `subscription_snapshot_emit`, `freshness: authoritative`, `completeness: full`, `frame_class: snapshot`| VFS file-tree snapshot; happy-path authoritative full snapshot             |
| [`partial.json`](./partial.json)   | `subscription_snapshot_emit`, `freshness: warming`, `completeness: partial`, `frame_class: snapshot`   | Derived search results warming; partial results surfaced before full index |
| [`stale.json`](./stale.json)       | `subscription_resync_required_emit`, `freshness: stale`, `stale_reason: upstream_input_stale`          | Derived diagnostics whose upstream input digest drifted; causal continuity lost |
| [`replayed.json`](./replayed.json) | `subscription_replay_begin`, `freshness: replayed`, `stale_reason: replayed_from_bundle`               | Support-bundle replay of a captured subscription; live epoch never advances |
| [`failed.json`](./failed.json)     | `subscription_terminate`, `completeness: unavailable`, `terminal_reason: unavailable`                  | Provider overlay terminates unavailable; consumer must resubscribe on repair |
| [`imported.json`](./imported.json) | `subscription_imported_attach`, `freshness: imported`, `stale_reason: imported_from_external`          | Imported LSIF-style precomputed index attached alongside live graph         |
