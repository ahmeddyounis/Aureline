# AI test-generation assumption and sandbox truth

This fixture set exercises the stable AI test-generation truth packet owned by
`aureline_ai::ai_test_generation`.

`truth_packet.json` covers:

- a draft-class generated test proposal with a concrete bug-report trigger,
  target refs, low-confidence and potentially-flaky labels retained after a
  sandbox pass;
- an assumption sheet exposing clock control, fixture creation, and the
  unsupported real-network path before apply;
- separated diff classes for logic assertions, helper/fixture additions, and a
  snapshot/golden update with baseline identity;
- local sandbox validation with target/environment lineage, network denied,
  temporary file-system allowed, secrets denied, a passed outcome, rerun, and
  open-log action refs;
- estimated coverage impact that does not count toward release or benchmark
  truth; and
- consumer projections for suggestion card, test explorer overlay, coverage
  view, CLI/headless, support export, and release packet.

Verify the checked packet with:

```sh
cargo test -p aureline-ai ai_test_generation --no-fail-fast
```

Regenerate the checked artifact, summary, and fixture after intentional changes
with:

```sh
cargo test -p aureline-ai ai_test_generation::tests::emit_artifact -- --ignored
```
