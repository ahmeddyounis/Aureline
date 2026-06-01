# Prompt Composer Stabilization

This fixture set exercises the stable prompt-composer stabilization packet owned
by `aureline_ai::stabilize_prompt_composer`. It promotes the beta conformance
packet in `fixtures/ai/m3/prompt_composer_drills` to the stable line.

`stabilization_packet.json` covers:

- a thread header with scope, provider/model, retention mode, memory access, and
  a Remember/Save preview that names what is retained, where it lives, and who
  can reuse it;
- typed attachment semantics for files, symbols, docs references, diagnostics,
  tests, terminal/tool outputs, and external text, each with origin, trust or
  taint class, freshness, and current inclusion posture;
- pinned context rows including a `pinned_but_stale` row with a drift source that
  blocks silent reuse until refreshed or removed;
- omitted-context review rows that stay inspectable after send for replay,
  support, and audit;
- forked-thread lineage with parent thread/run, inherited context snapshot, and
  divergence point;
- compare-answer rows preserving same-versus-different context truth, including a
  hidden-drift comparison;
- a context-drift banner that requires re-review;
- surface-consistency rows proving keyboard and screen-reader reachability across
  editor-attached, sidebar, and detached composers.

Verify the checked packet with:

```sh
cargo test -p aureline-ai stabilize_prompt_composer --no-fail-fast
```

Regenerate the checked artifact and summary after intentional changes with:

```sh
cargo test -p aureline-ai stabilize_prompt_composer::tests::emit_artifact -- --ignored
```
