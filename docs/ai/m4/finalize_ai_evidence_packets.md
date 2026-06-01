# Finalize the AI evidence packet

This stable lane turns AI evidence packets into first-class, replay-safe product
artifacts. The runtime owner is `aureline_ai::finalize_ai_evidence_packets`.

An AI result is not stable unless its evidence packet survives export,
redaction, replay, and support handoff without depending on live UI state or
vague prose. The finalization packet binds the existing evidence lineages into
one export-safe artifact with a stable evidence id and the blocks a reviewer,
operator, support engineer, or auditor needs to reconstruct the run.

## Contract

The finalization packet does **not** re-derive mutation, run-history, or replay
truth. The `aureline_ai::evidence::AiMutationEvidencePacket` mutation wedge, the
`aureline_ai::run_history` run-history / rerun-review lane, and the replay-packet
boundary remain canonical for their own slices. The finalization packet
references those lineages by id and adds:

- **Stable evidence id and origin lineage** — one canonical evidence id with a
  declared origin (originating turn, agent run, branch-agent job, replay action,
  or rerun action) and the matching origin refs.
- **Six stable evidence blocks** — intent and requested scope, context inputs,
  tool and policy decisions, produced diff/write scope, validation and outcome,
  and rollback/export.
- **Right-sized packet classes** — inline evidence stub, operator packet,
  support packet, and compliance/audit packet, each proving it preserves the
  same evidence id, redaction manifest, rollback/export refs, provider/model
  identity, approval path, validation receipts, and write-scope classes across
  the UI, CLI/headless, and support/export paths.
- **Redaction as evidence** — an export-safe redaction manifest that shows what
  was removed, why, and whether reproducibility changed.
- **Distinct absence states** — omitted, blocked, summarized, and not-requested
  context candidates stay in distinct buckets instead of collapsing into one
  generic absence.
- **Retained-artifact inventory** — retained evidence copies stay distinct from
  conversation threads, prompt/result caches, reusable repo facts, and explicit
  saved memory; the inventory discloses which classes survive thread deletion
  and which copies are retained only because evidence policy requires them.
- **Replay/rerun lineage** — run-history, approval-timeline, rerun-review, and
  replay-packet refs with an explicit replay posture. When replay is incomplete
  because connector output, provider data, or external tool evidence was not
  retained, the packet says so plainly and requires fresh approval for any new
  tool calls. Replay and rerun packets cite the original packet and produce a
  new one rather than overwriting history.
- **Retrieval-lane provenance** — when hybrid retrieval or semantic recall
  influenced context, the packet records participating lanes, locality,
  retrieval epoch, embedder/model identity, source and chunk/anchor counts, and
  which candidate classes were omitted/blocked/summarized/not-requested, while
  always omitting raw vectors and raw code-bearing chunks.
- **AI review-assist branches** — first-class branches for candidate test
  proposals, assumption reviews, sandbox validations, AI review findings,
  publish previews, and outbound review actions, each preserving whether the
  finding stayed local, was copied/exported, or was published, together with the
  exact outbound target class and redaction posture.

## Required behavior

`AiEvidencePacketFinalization::validate` rejects a packet when:

- the origin refs do not match the declared origin class, or a branch-agent /
  replay / rerun origin is missing its job or action ref;
- any of the six stable evidence blocks is incomplete, or a tool/policy decision
  that required approval has no approval ref, or the validation block carries no
  validation summary refs;
- semantic recall ran without an epoch, embedder identity, or raw-vector and
  raw-chunk exclusion, or a run that used no recall still implies a managed
  posture or claims participating lanes;
- the omitted, blocked, summarized, and not-requested absence states are not all
  kept distinct;
- a packet class is not covered, or a class projection drops the evidence id,
  redaction manifest, rollback/export refs, provider/model identity, approval
  path, validation receipts, write-scope classes, or any of UI/CLI/support
  parity;
- a redaction-manifest row that changed reproducibility carries no note;
- a conversation thread or prompt/result cache claims to survive thread
  deletion, the evidence-policy retention class is not disclosed, or a non-policy
  class claims retention purely for evidence policy;
- an incomplete replay omits its reason or does not require fresh approval for
  new tool calls, or a replay/rerun packet fails to cite the original packet or
  cites its own id;
- an AI review-assist branch class is not covered, or a published branch claims
  it stayed local (or a local branch fails to report the stayed-local target).

## Boundary

The packet is export-safe. It carries refs, state tokens, coarse classes,
counts, and review labels only. Raw prompt bodies, source file bodies, provider
payloads, raw retrieval vectors or chunks, endpoint URLs, credentials, raw token
counts, exact prices, and billing-account ids stay outside the support boundary.

## Truth source

The checked artifact at
`artifacts/ai/m4/finalize_ai_evidence_packets/support_export.json` is canonical
for this lane. Dashboards, docs, Help/About surfaces, and support exports should
ingest it instead of cloning status text. The boundary schema is
`schemas/ai/ai_evidence_packet_finalization.schema.json`; the protected fixture
is `fixtures/ai/m4/finalize_ai_evidence_packets/`.

Verify the checked packet with:

```sh
cargo test -p aureline-ai finalize_ai_evidence_packets
```
