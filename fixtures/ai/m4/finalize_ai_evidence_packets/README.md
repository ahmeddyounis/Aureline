# Finalize AI Evidence Packets

This fixture set exercises the stable AI evidence packet finalization record
owned by `aureline_ai::finalize_ai_evidence_packets`. It binds the
mutation-evidence, run-history, rerun-review, and replay lineages into one
export-safe packet.

`finalization_packet.json` covers:

- a stable evidence id with an originating-turn origin and matching turn/run
  refs, plus provider/model identity and approval path aligned with on-screen
  review;
- the six stable evidence blocks — intent and requested scope, context inputs,
  tool and policy decisions, produced diff/write scope, validation and outcome,
  and rollback/export;
- retrieval-lane provenance for a hybrid local-and-managed run, with epoch,
  embedder identity, participating lanes, and raw vectors/chunks excluded;
- distinct omitted, blocked, summarized, and not-requested absence rows;
- all four packet classes (inline evidence stub, operator packet, support
  packet, compliance/audit packet) preserving the same evidence id, redaction
  manifest, rollback/export refs, provider/model identity, approval path,
  validation receipts, and write-scope classes across UI, CLI, and support;
- a redaction manifest treating redaction as evidence, including a row that
  degraded reproducibility and carries a note;
- a retained-artifact inventory distinguishing the policy-retained evidence copy
  from the conversation thread, prompt/result cache, reusable repo fact, and
  explicit saved memory, disclosing what survives thread deletion;
- replay/rerun lineage with a reconstructible-full posture and run-history,
  approval-timeline, rerun-review, and replay-packet refs;
- the AI review-assist branches — candidate test proposal, assumption review,
  sandbox validation, AI review finding, publish preview, and outbound review
  action — preserving outbound authority and redaction posture.

Verify the checked packet with:

```sh
cargo test -p aureline-ai finalize_ai_evidence_packets --no-fail-fast
```

Regenerate the checked artifact, summary, and fixture after intentional changes
with:

```sh
cargo test -p aureline-ai finalize_ai_evidence_packets::tests::emit_artifact -- --ignored
```
