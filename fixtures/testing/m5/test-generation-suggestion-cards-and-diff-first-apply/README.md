# Test-generation proposal fixtures

Proof fixtures for the test-generation proposal packet
(`test_generation_proposal_packet`). Each fixture is an export-safe packet that
`TestGenerationProposalPacket::validate` accepts and that exercises a specific truth
the contract guarantees.

- `sandbox_gate_blocks_apply.json` — a packet whose cards span a locally generated,
  sandbox-validated proposal applied through the preview-first diff pipeline with a
  follow-on rerun link; a bug-reproduction proposal whose apply stays
  `blocked_needs_validation` until sandbox validation succeeds; a previewed
  branch-coverage proposal whose sandbox run failed so it is not accept-eligible; an
  imported proposal held read-only; and a proposal that would widen beyond its
  evidenced scope and is rejected rather than silently applied. It proves that a
  generated test cannot be applied without an isolated sandbox pass, that an imported
  proposal never reads as a local apply, and that a widening proposal is routed to
  review.

The boundary schema is
`schemas/testing/test-generation-suggestion-cards-and-diff-first-apply.schema.json`;
the contract doc is
`docs/testing/m5/test-generation-suggestion-cards-and-diff-first-apply.md`.
Regenerate the canonical export and this fixture with:

```bash
cargo run -p aureline-runtime --example dump_test_generation_suggestion_cards
cargo run -p aureline-runtime --example dump_test_generation_suggestion_cards fixture
```
