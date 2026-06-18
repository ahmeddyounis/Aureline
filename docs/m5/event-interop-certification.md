# Event-interop tooling-profile certification

The canonical build/test event interoperability packet
([build-test-event-interoperability](../runtime/m4/build-test-event-interoperability.md))
freezes *one* event envelope, the native-first adapter ladder, the
confidence/raw-retention rules, and the replay/export contracts. The
[interop conformance suite](build-test-interop-corpora.md) re-runs that contract
across the adapter *families* and *archetypes*. This lane closes the loop at the
*consumer* level: it turns those frozen contracts into a claim-bearing **tooling
profile matrix** so each M5 run/test/debug/pipeline/notebook surface can only
*claim* event-interoperability coherence when its own path is current and
machine-readable.

The stable truth source is `EventInteropCertificationPacket` in `aureline-runtime`
(`crates/aureline-runtime/src/m5_event_interop_certification/`). The headless
inspector and regenerator is
`cargo run -p aureline-runtime --example dump_m5_event_interop_certification`.

## Claimed tooling profiles

Each claimed M5 profile that depends on the task-event contract carries one
certification:

| Profile | Certifies |
| --- | --- |
| `task_center_run` | task center run history, reopen, and rerun flows |
| `test_session` | test explorer sessions, inline results, and watch trees |
| `debug_session` | debug sessions and chronology views |
| `pipeline_overlay` | pipeline overlays over imported / remote CI runs |
| `notebook_run` | notebook-backed runs and their cell execution history |
| `coverage_intelligence` | coverage, flaky, and snapshot intelligence overlays |

A missing profile blocks stable (`missing_profile`), so the matrix cannot
silently shrink to the profiles that still happen to pass.

## Eight graded certification dimensions

Every profile is graded on the dimensions the contract requires, and a profile is
`certified` only when **all** pass:

- `event_envelope_reuse` — the profile reads the canonical event envelope (its
  `consumer_truth_source` is `canonical_event_envelope`) and cites at least one
  upstream evidence packet, rather than a `private_session_history`,
  `rendered_log_scraping`, `unlabeled_heuristic_parsing`, or a `missing_raw_lineage`
  path.
- `adapter_hierarchy` — a native-first capability handshake is evidenced.
- `fallback_reason` — a degraded/unsupported path names an explicit fallback
  reason, and a negotiated path names none.
- `confidence_preservation` — the observed confidence does not overclaim its
  source (a heuristic source, or an explicitly unsupported capability, cannot
  claim more than `low`).
- `raw_payload_retention` — the raw payload is retained behind a reference and
  digest with private material excluded.
- `replay_stability` — the profile replays deterministically from canonical
  envelopes.
- `degraded_state_disclosure` — a degraded/unsupported capability is visibly
  disclosed.
- `export_parity` — support / release / AI exports preserve source, confidence,
  and refs.

A profile that fails any dimension emits a precise blocker finding
(`event_envelope_not_reused`, `missing_evidence_ref`, `adapter_hierarchy_missing`,
`fallback_reason_missing`, `fallback_reason_unexpected`, `confidence_overclaim`,
`raw_payload_not_retained`, `replay_unstable`, `degraded_state_not_disclosed`,
`export_parity_broken`) and blocks stable. This is how the lane blocks or narrows
any profile that still relies on unlabeled heuristic parsing, private session
histories, or missing raw lineage.

## Each profile cites machine-readable evidence

Every profile draws its proof from the checked-in upstream artifacts (the
event-envelope first-consumer bus, the native-first adapter negotiation baseline,
the adapter-confidence audit, the raw-plus-normalized replay bundle, the
cross-surface event-reuse proof, and the interop conformance suite). A profile
that cites no evidence blocks stable (`missing_evidence_ref`), so a claim can
never rest on an unverifiable assertion.

## Freshness narrows aged proof

Every profile carries a recorded proof age and a freshness window. A profile whose
proof has aged past its window emits a **warning** (`profile_evidence_stale`),
its claim state becomes `narrowed_below_stable`, and the packet **narrows below
stable** rather than blocking — but it cannot stay green. This is the
stale-evidence narrowing the release lane relies on so an interop claim cannot
coast on aged proof.

## Certification index

The derived `certification_index` is the one canonical execution-truth index
release, support, AI, and docs/help surfaces ingest. It names which profiles are
`claimable` (current and certified), which have `narrowed` below stable on aged
proof, and which are `blocked`, and records whether every profile is current and
certified. Release packets ingest this index to show **current** interop proof
instead of re-deriving profile maturity by hand.

## Stability rules

- All six tooling profiles must be present exactly once.
- Every profile must reuse the canonical event envelope and cite at least one
  upstream evidence packet.
- Every profile must certify across all eight dimensions.
- A stale profile narrows below stable (warning); a non-certified profile blocks
  stable (blocker).
- The stored per-profile dimension outcomes, freshness/claim states, the profile
  digest, and the certification index must all match the derivation; any drift
  blocks stable.
- A packet with any blocker finding is `blocks_stable`; a packet with only
  warnings is `narrowed_below_stable`; otherwise it is `stable`.

## Companion artifacts

- `schemas/tooling/m5-event-interop-certification.schema.json` — boundary schema
  for the packet, its support export, its evidence joins, and the CLI/headless
  view.
- `schemas/tooling/task-event-envelope.schema.json` — boundary schema for the
  per-event task-event envelope vocabulary this lane reuses.
- `artifacts/m5/tooling/event-interop-certification/` — the checked-in packet,
  support export, AI evidence join, incident packet join, CLI/headless view, and
  compact rendering.
- `fixtures/tooling/m5/event-interop-certification/` — the baseline and the
  blocking / narrowing mutation cases the typed consumer and the gate replay.
- `tools/ci/m5/event_interop_certification_check.py` — the fail-closed gate.

The typed Rust consumer mints the same packet, so
`cargo test -p aureline-runtime --test m5_event_interop_certification` enforces
the same structural invariants and that the fixtures and artifacts are
bit-for-bit derivable from the seed.
