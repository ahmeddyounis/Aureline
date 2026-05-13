# Target-Confidence Cards Alpha

This lane turns the canonical execution-context resolver output into
inspectable cards for target discovery, confidence, and host-boundary truth.
The implementation lives in
[`crates/aureline-runtime/src/targets`](../../crates/aureline-runtime/src/targets)
and consumes
[`ExecutionContext`](../../crates/aureline-runtime/src/execution_context/mod.rs)
records directly.

## Contract

Every `target_confidence_card_record` carries:

- execution-context and resolver-provenance refs;
- target class, canonical target id, reachability, and lane class;
- target-discovery confidence token;
- resolver confidence level plus reason tokens;
- host-boundary cue token, label, and cue-stack tokens;
- explanation rows for why the target won;
- inspect and change-target action refs.

The card does not own target truth. It quotes resolver fields for target
identity, precedence, prebuild state, mixed-version posture, trust, and policy.

## Consumers

`TargetConfidenceSupportExport` copies the cards, host-boundary rows, and
redaction-safe execution-context provenance into a support packet.

`TargetConfidenceReviewPacket` copies the same card and boundary rows into a
pre-dispatch review packet. Helper-backed lanes, non-high confidence, or
divergence/inference reasons mark the packet as requiring review before
dispatch.

## Fixtures

Protected fixtures live under
[`fixtures/runtime/target_confidence_alpha`](../../fixtures/runtime/target_confidence_alpha):

- `local_and_helper_cards.json` proves one local task lane and one
  managed-helper lane preserve discovery confidence and host-boundary labels
  through cards, support export, and review packet projections.

## Verify

```sh
cargo test -p aureline-runtime target_confidence
```
