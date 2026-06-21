# Structured-native versus heuristic fallback: the M5 proof corpus and failure drills

This packet freezes the canonical truth for the **parse-evidence drill case**: one
parse-evidence case — a **native structured diagnostic**, a **normalized task event**, an
**imported provider annotation**, or a **heuristic text parse** — exercised through a
**failure drill** (native baseline, malformed output, stale run, superseded retry,
reconnect, lost channel, partial export, imported evidence, or output-channel
virtualization) and rendered onto the claimed **M5 tooling profiles** (Problems panel,
output channel, terminal runner, debug console, notebook output, pipeline overlay, AI-tool
evidence, support export). It proves that the structured-native versus heuristic
distinction, the confidence label, and the raw-output backlink survive malformed output,
heuristic parsing, stale retries, imported provider evidence, and reconnect-heavy
workflows — and that a failure in causal linking or confidence labeling automatically
narrows the affected profile claims.

It is the proof-corpus companion to the
[`m5-execution-evidence`](./m5-execution-evidence.md) **lane matrix**, the
[`m5-problem-records`](./m5-problem-records.md) **Problems row**, the
[`m5-execution-evidence-projections`](./m5-execution-evidence-projections.md) **projected
overlay**, the [`m5-chronology-reuse`](./m5-chronology-reuse.md) **chronology entry**, and
the [`m5-output-channels`](./m5-output-channels.md) **output channel**. All share one
vocabulary — origin class, problem-source kind, output-channel class, confidence tier,
freshness state, reopen target, and proof currency are reused, not re-invented — so
Problems, output-channel, terminal, debug, notebook, pipeline, AI-tool, and support
surfaces ingest one model instead of a private structured-versus-heuristic model. Reuse
the canonical task-event envelopes, diagnostic ids, run/channel refs, and provider objects
already landed earlier; this packet binds them onto one inspectable, reopenable drill case.

If this doc, the
[`m5-fallback-evidence-drills.schema.json`](../../schemas/tooling/m5-fallback-evidence-drills.schema.json)
boundary, the frozen set under
[`/artifacts/tooling/m5-fallback-evidence-drills/`](../../artifacts/tooling/m5-fallback-evidence-drills/),
and the perturbation corpus under
[`/fixtures/tooling/m5-fallback-evidence-drills/`](../../fixtures/tooling/m5-fallback-evidence-drills/)
disagree, the machine-readable schema plus the checked-in support export
(`artifacts/tooling/m5-fallback-evidence-drills/support_export.json`) win, and this doc
must update in the same change.

## Companion artifacts

- [`/schemas/tooling/m5-fallback-evidence-drills.schema.json`](../../schemas/tooling/m5-fallback-evidence-drills.schema.json)
  — boundary schema for the `m5_fallback_evidence_drill_set_packet` and every frozen
  taxonomy.
- [`/artifacts/tooling/m5-fallback-evidence-drills/support_export.json`](../../artifacts/tooling/m5-fallback-evidence-drills/support_export.json)
  — the canonical drill set (the source of truth for every case).
- [`/artifacts/tooling/m5-fallback-evidence-drills/report.md`](../../artifacts/tooling/m5-fallback-evidence-drills/report.md)
  — the generated certification report (do not edit by hand; regenerate with the Rust
  dump example).
- [`/fixtures/tooling/m5-fallback-evidence-drills/`](../../fixtures/tooling/m5-fallback-evidence-drills/)
  — the perturbation corpus that pins each narrowing/floor rule.
- `tools/release/fallback_evidence_drills.py` — re-derives the effective claim and ordered
  narrowing reasons per case and validates the set and corpus.
- `crates/aureline-runtime/src/m5_structured_versus_heuristic_fallback_drills/` — the
  in-process Rust truth source. It deserializes the checked-in support export into one
  typed packet, re-derives the same effective claim, floor/overlay/labs ladder, and
  ordered narrowing reasons as the Python engine, and exposes
  `current_m5_fallback_evidence_drill_set()` so Problems, output-channel, terminal, debug,
  notebook, pipeline, AI-tool, and support consumers ingest the governed corpus without
  re-parsing raw logs or forking a parallel structured-versus-heuristic model.

## The two axes a drill case crosses

A parse-evidence case crosses two orthogonal axes plus the claimed profiles:

- **Problem-source axis** (`problem_source_kind`): `structured_language_diagnostic`,
  `normalized_task_event`, `heuristic_output_parse`, or `imported_provider_annotation`.
  This is the structured-versus-heuristic distinction. A heuristic source — or a heuristic
  confidence tier — marks a **heuristic** case that must read visibly distinct from
  structured evidence and keep a raw-output backlink.
- **Failure-drill axis** (`drill_kind`): the scenario the case is exercised through —
  `native_structured`, `normalized_task_event`, `heuristic_text_parse`,
  `imported_evidence`, `malformed_output`, `stale_run`, `superseded_retry`, `reconnect`,
  `lost_channel`, `partial_export`, or `channel_virtualization`.
- **Profiles** (`profiles[*].profile`): the claimed M5 tooling surfaces the case is
  rendered on. Acceptance is **per-profile**: a heuristic case must read distinctly, and a
  narrowed/floored case must not overclaim, on *every* profile.

## The causal chain a drill case preserves

The engine re-derives — rather than trusts — an effective claim from these invariants:

- **Structured-versus-heuristic distinctness**
  (`integrity.preserves_source_kind`, `heuristic_visibly_distinct_from_structured`,
  `profiles[*].fallback_visibly_distinct`, `raw_output_backlink_present`): the
  problem-source class survives, a heuristic case reads visibly distinct on every profile,
  and a heuristic case keeps a raw-output backlink.
- **Lineage & one canonical id** (`integrity.preserves_run_channel_lineage`,
  `channel_identity_stable`, `profiles[*].lineage_visible`, `bound_run_ref`,
  `bound_channel_ref`, `bound_problem_ref`): run/step/provider/channel lineage and the
  stable channel id survive and stay revealable on demand; a profile that points at a
  *different* run/channel/problem id breaks the single-id contract and floors.
- **Failure drills are honest** (`integrity.reconnect_preserves_evidence`,
  `partial_export_self_contained`, `imported_evidence_read_only`): a reconnect or
  lost-channel drill never drops evidence; a partial export stays reviewable without the
  originating UI; an imported/remote/pipeline origin never claims live local authority.
- **Freshness & confidence** (`declared_freshness_state`, `integrity.freshness_state_labeled`,
  `superseded_state_marked`, `integrity.confidence_label_visible`): stale and superseded
  states stay visibly classified; the confidence tier is visible; missing evidence floors.
- **Output-channel virtualization** (`virtualization.stream_first`, `searchable`,
  `copy_exportable`, `bounded_memory`): a large log stays stream-first, searchable,
  copy/exportable, and bounded; losing any of these narrows the case.
- **Reopen** (`declared_reopen_target`): every case can reopen its origin; a case that
  loses its reopen path keeps a `raw_output_backlink` or `none_keyboard_fallback`.
- **Profile honesty** (`profiles[*].rendered_claim`): a profile may never render a claim
  wider than the case's effective claim.

## The effective-claim ladder

| Effective claim | Meaning |
| --- | --- |
| `fallback_certified` | Structured-native or a clearly-distinct heuristic fallback, fresh, lineage and confidence intact, reopenable. |
| `fallback_narrowed` | A first-party case held below certified by a stale/labelled gap, but lineage stays reopenable. |
| `fallback_read_only_overlay` | Remote/pipeline/imported parse evidence: attributable and reopenable but never claims live local authority. |
| `fallback_unreconstructable` | Distinctness/lineage/reopen broken or a drill dropped evidence: surfaces a raw-output backlink or keyboard fallback instead of a clean-but-false row. |
| `fallback_labs_not_claimed` | Labs/unadvertised: makes no public claim and is never widened. |

**Floor** reasons (`source_kind_flattened`, `heuristic_indistinct_from_structured`,
`run_channel_lineage_flattened`, `channel_identity_flattened`, `canonical_id_divergence`,
`raw_output_backlink_missing`, `reopen_target_lost`, `reconnect_drops_evidence`,
`partial_export_incomplete`, `surface_overclaims`, `imported_overlay_claims_live`,
`evidence_missing`) break the "stay distinct / stay reopenable / never flatten lineage /
never drop evidence / never masquerade as live" contract outright and drop the case to
`fallback_unreconstructable`. The remaining reasons (`confidence_unlabeled`,
`freshness_unlabeled`, `superseded_state_not_marked`, `virtualization_not_stream_first`,
`search_unavailable`, `copy_export_unavailable`, `evidence_stale`,
`verification_proof_stale`, `verification_proof_missing`) hold a first-party case at
`fallback_narrowed` (still reopenable). A read-only overlay is already the minimal honest
claim, so any non-floor gap drops it below the overlay too. Labs cases never accrue
narrowing.

## Regeneration

```bash
# Rust: regenerate the support export and report (identical bytes each run).
cargo run -p aureline-runtime --example dump_m5_fallback_evidence_drills > \
  artifacts/tooling/m5-fallback-evidence-drills/support_export.json
cargo run -p aureline-runtime --example dump_m5_fallback_evidence_drills summary > \
  artifacts/tooling/m5-fallback-evidence-drills/report.md

# Python: regenerate the perturbation corpus and validate end-to-end.
python3 tools/release/fallback_evidence_drills.py emit-corpus
python3 tools/release/fallback_evidence_drills.py self-test
```
