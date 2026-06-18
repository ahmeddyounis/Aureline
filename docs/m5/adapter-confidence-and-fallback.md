# Adapter-confidence labels and fallback honesty

This contract turns confidence preservation into a **user-visible contract**
rather than a parser-internal heuristic. The canonical event envelope
([task-event-envelope.md](task-event-envelope.md)) already carries a source
kind, an adapter-priority rank, a confidence level, and a downgrade flag on
every record, and the frozen policy layer
([task-event-and-adapter-policy.md](task-event-and-adapter-policy.md)) fixes the
native-first priority ladder, the per-source confidence ceilings, and the closed
downgrade vocabulary. This lane makes those facts visible and enforceable: it
binds one **confidence label** to every claimed surface and arbitrates the case
the docs care most about — two sources describing the same lifecycle slot or
artifact — so a weaker or imported emission may *enrich* context but can never
*silently overwrite* native, BSP, or Bazel BEP/BES truth.

The stable truth source is `AdapterConfidenceAudit` in `aureline-runtime`
(`crates/aureline-runtime/src/m5_adapter_confidence_labels/`). The headless
inspector and regenerator is
`cargo run -p aureline-runtime --example dump_m5_adapter_confidence_labels`.

## The confidence label is three cues, never one badge

A `ConfidenceLabel` keeps three cues as **separate fields** on purpose:

- a **source-class chip** (`native`, `bsp`, `bazel-bep`, `structured-output`, or
  `heuristic-parser`),
- a **confidence chip** (`high`, `medium-high`, `medium`, or `low`), and
- a **heuristic-fallback banner** with its `fallback_reason`, shown only for a
  heuristic-parser emission.

Source class and confidence are never compressed into one badge or a generic
"partial" label. A heuristic-parser emission always carries the banner; a
native/BSP/BEP/structured emission carries neither.

## Per-surface bindings

Every claimed surface declares a `SurfaceLabelBinding` proving it reads the
canonical label rather than rendered text and that it keeps the cues visible.
The eight surfaces are the five product surfaces — `task_center`, `test_tree`,
`coverage_flaky`, `pipeline_overlay`, `notebook_run_history` — plus the three
export surfaces — `support_export`, `cli_headless`, `ai_evidence`. Each binding
must keep the source class and confidence as two distinct chips, show the
heuristic-fallback banner and its reason, keep the overwrite decision visible,
and keep the full claim lineage inspectable.

## No-lower-confidence-overwrite arbitration

When more than one source describes the same `ClaimSubject` — a lifecycle slot
or a published artifact — the audit resolves them without dropping any claim:

- The **strongest** claim (lowest priority rank, then highest confidence) is
  `accepted_authoritative`.
- A weaker claim that **attempted to overwrite** the authoritative slot is
  `blocked_lower_confidence`, with the reason `weaker_source_class` or
  `lower_confidence_tier`. This is the core guardrail: a weaker source can never
  silently replace stronger truth.
- A weaker claim that **never asserted authority** is `enriched_context_only`,
  with the reason `never_claimed_authority`; it stays as inspectable context.

Every claim a subject saw is retained in `claims`, so the raw-to-authoritative
chain is never flattened to resolve a conflict.

## Source-quality changes (reusable vocabulary)

Each subject records one `source_quality_change`, the reusable token desktop,
CLI/headless, AI, and support flows all read instead of inferring a quality
shift from text:

- `held_authoritative` — authority stayed at the same source class with no
  contested overwrite.
- `upgraded_to_authoritative` — a stronger source took over from a weaker prior
  authority (for example, native superseding a prior heuristic).
- `downgraded_to_fallback` — the prior authority dropped and only a weaker or
  heuristic source remains; that source is authoritative-but-flagged.
- `overwrite_blocked` — a weaker source attempted to overwrite and was refused;
  authority held.
- `enriched_without_overwrite` — a weaker source added context without changing
  authority.

## Stability rules

- The audit must bind every one of the eight claimed surfaces, and each binding
  must keep source class and confidence distinct, show the heuristic-fallback
  banner and its reason, and keep the overwrite decision and claim lineage
  inspectable.
- Every claim's priority rank must bind to its source kind, every claim's
  confidence must stay at or below its source's ceiling, and a heuristic label
  must carry the banner and its reason while a non-heuristic label must carry
  neither.
- Each subject must name the authoritative claim the canonical arbitration
  picks. A weaker, overwrite-attempting claim that is **not** blocked is the core
  invariant breach (`lower_confidence_overwrite_accepted`); any other decision or
  source-quality drift blocks stable.
- A decision that references a claim the subject no longer retains is
  `lineage_dropped`: provenance must stay inspectable.
- An audit with any blocker finding is `blocks_stable`; otherwise it is
  `stable`.

## Companion artifacts

- `schemas/tooling/adapter-confidence-audit.schema.json` — boundary schema for
  the audit, its support export, its CLI/headless view, and its AI evidence view.
- `schemas/tooling/task-event-envelope.schema.json` — boundary schema for the
  per-event task-event envelope whose source-kind, confidence, and
  heuristic-fallback vocabulary this lane reuses.
- `artifacts/m5/tooling/adapter-confidence-audit/` — the checked-in audit,
  support export, CLI/headless view, AI evidence view, and compact rendering.
- `fixtures/tooling/m5/confidence-preservation/` — the baseline and the blocking
  mutation cases the typed consumer and the gate replay.
- `tools/ci/m5/adapter_confidence_audit_check.py` — the fail-closed gate.

The typed Rust consumer mints the same audit, so
`cargo test -p aureline-runtime --test m5_adapter_confidence_labels` enforces the
same structural invariants and that the fixtures and artifacts are bit-for-bit
derivable from the seed.
