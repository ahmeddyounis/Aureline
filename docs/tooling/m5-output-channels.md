# Output channels: stream-first virtualization, trust classes, pin/export, and stale/live truth

This packet freezes the canonical truth for the **individual output channel**: a
**raw log stream**, a **trusted structured report**, an **HTML report bundle**, a
**generated artifact**, or a **trace/profile output** rendered into the shell,
terminal, Problems panel, review surface, timeline, support bundle, or AI-evidence
consumer. Each channel binds its output to the original
**run/step/provider/artifact lineage**, the **stream-first virtualization profile**
that keeps a large log searchable and exportable without full materialization, the
**content trust class** and **pin/export controls** that keep safe-preview distinct
from active/open-in-external content, and the **live/cached/stale freshness** with
**fetched-at** and **provider-unreachable** cues — so a large log never forces full
materialization into shell memory, a user can always tell raw / safe-preview /
trusted-structured / untrusted-active content apart before copying, exporting, or
opening, and a provider-backed channel can never masquerade as live after a freshness
threshold or a lost connection.

It is the per-channel companion to the
[`m5-execution-evidence`](./m5-execution-evidence.md) **lane matrix**, the
[`m5-problem-records`](./m5-problem-records.md) **Problems row**, and the
[`m5-execution-evidence-projections`](./m5-execution-evidence-projections.md)
**projected overlay**. Where the lane matrix freezes one row per
Problems/output/execution-evidence *surface family*, the Problems packet freezes one
row per *finding*, and the projection packet freezes one row per *projected overlay*,
this packet freezes one row per *output channel*. All four speak one vocabulary —
origin class, channel class, confidence tier, freshness state, reopen target, and
proof currency are reused, not re-invented — so shell, terminal, Problems, debug,
pipeline, notebook, CLI/headless, AI evidence, and support export ingest one model
instead of a private channel truth model. Reuse the canonical run/step/provider refs,
generated-artifact ids, and evidence packets already landed earlier; this packet binds
them onto one inspectable, reopenable channel.

If this doc, the
[`m5-output-channels.schema.json`](../../schemas/tooling/m5-output-channels.schema.json)
boundary, the frozen set under
[`/artifacts/tooling/m5-output-channels/`](../../artifacts/tooling/m5-output-channels/),
and the perturbation corpus under
[`/fixtures/tooling/m5-output-channels/`](../../fixtures/tooling/m5-output-channels/)
disagree, the machine-readable schema plus the checked-in support export
(`artifacts/tooling/m5-output-channels/support_export.json`) win, and this doc must
update in the same change.

## Companion artifacts

- [`/schemas/tooling/m5-output-channels.schema.json`](../../schemas/tooling/m5-output-channels.schema.json)
  — boundary schema for the `m5_output_channel_set_packet` and every frozen taxonomy.
- [`/artifacts/tooling/m5-output-channels/support_export.json`](../../artifacts/tooling/m5-output-channels/support_export.json)
  — the canonical channel set (the source of truth for every channel).
- [`/artifacts/tooling/m5-output-channels/report.md`](../../artifacts/tooling/m5-output-channels/report.md)
  — the generated certification report (do not edit by hand; regenerate with the
  Rust dump example).
- [`/fixtures/tooling/m5-output-channels/`](../../fixtures/tooling/m5-output-channels/)
  — the perturbation corpus that pins each narrowing/floor rule.
- `tools/release/output_channel_virtualization.py` — re-derives the effective claim
  and ordered narrowing reasons per channel and validates the set and corpus.
- `crates/aureline-runtime/src/m5_output_channel_virtualization_trust_and_freshness/`
  — the in-process Rust truth source. It deserializes the checked-in support export
  into one typed packet, re-derives the same effective claim, floor/overlay/labs
  ladder, and ordered narrowing reasons as the Python engine, and exposes
  `current_m5_output_channel_set()` so desktop, CLI/headless, AI evidence, support
  export, review, notebook, and pipeline consumers ingest the governed channel without
  re-parsing raw logs or forking a parallel truth model.

## What a channel preserves

An output channel is rendered across many surfaces. To stay honest it must, on every
surface it renders, be able to answer **which run, which step, which provider, which
canonical channel** produced it; whether a **large log** stays **stream-first,
searchable, and exportable** without full materialization; what the **content trust
class** is and whether **safe-preview** is kept distinct from
**active/open-in-external** content; whether the channel is **live, cached, or stale**
and (for provider-backed channels) when it was **fetched** and whether the provider is
**reachable**; and **how to reopen** the originating run, channel, artifact, or packet.
The engine re-derives — rather than trusts — an effective claim from these invariants:

- **Channel + origin lineage** (`lineage.canonical_channel_ref`,
  `integrity.preserves_run_step_lineage`, `preserves_provider_identity`,
  `lineage_visible_on_demand`): the canonical channel id and origin
  run/step/provider identity survive and can be revealed on demand on every surface.
- **Stream-first virtualization** (`virtualization.large_log`, `stream_first`,
  `searchable`, `stable_chunk_ids`, `follow_mode_supported`, `bounded_memory`,
  `exportable_without_full_materialization`): a large log (`large_log`, or more than
  `LARGE_CHANNEL_CHUNK_THRESHOLD` chunks) must stream, search, bound memory, and
  export without full materialization; missing any of those floors, and missing
  stable chunk ids or follow mode narrows.
- **Trust classes + pin/export** (`trust_class`, `access.trust_class_labeled`,
  `safe_preview_available`, `pin_supported`, `export_supported`, `export_is_safe`,
  `open_in_external_requires_confirmation`, `trust_boundary_preserved`): the trust
  class is labelled; untrusted active content never opens externally without
  confirmation; an export never leaks active content; the safe-preview boundary is
  never blurred.
- **Freshness + provider cues** (`declared_freshness_state`,
  `integrity.freshness_state_labeled`, `superseded_state_marked`,
  `freshness.provider_backed`, `fetched_at_present`, `provider_reachable`,
  `provider_unreachable_marked`, `live_state_honest`): stale and superseded states
  stay visibly classified; a provider-backed channel discloses fetched-at and
  unreachable cues and never masquerades as live.
- **Reopen** (`declared_reopen_target`): every channel can reopen its origin; a
  channel that loses its reopen path keeps a `raw_output_backlink` or
  `none_keyboard_fallback`.
- **Surface honesty** (`renderings[*].rendered_claim`): a rendering surface may never
  render a claim wider than the channel's effective claim.

## The effective-claim ladder

| Effective claim | Meaning |
| --- | --- |
| `channel_certified` | Full first-party lineage preserved, virtualized, trust-honest, fresh, reopenable. |
| `channel_narrowed` | A first-party channel held below certified by a stale/labelled gap, but lineage stays reopenable. |
| `channel_read_only_overlay` | Remote/pipeline/imported channel: attributable and reopenable but never claims live local authority. |
| `channel_unreconstructable` | Lineage/identity/virtualization/trust broken: surfaces a raw-output backlink or keyboard fallback instead of a clean-but-false channel. |
| `channel_labs_not_claimed` | Labs/unadvertised: makes no public claim and is never widened. |

**Floor** reasons (`channel_identity_flattened`, `run_step_lineage_flattened`,
`provider_identity_flattened`, `lineage_not_visible`, `reopen_target_lost`,
`raw_output_backlink_missing`, `stream_not_virtualized`, `unbounded_memory`,
`export_forces_full_materialization`, `trust_boundary_blurred`,
`active_content_auto_opens`, `export_unsafe`, `surface_overclaims`,
`imported_channel_claims_live`, `stale_channel_claims_live`,
`channel_content_missing`) break the "stay reopenable / never force full
materialization / never blur the trust boundary / never masquerade as live" contract
outright and drop the channel to `channel_unreconstructable`. The remaining reasons
hold a first-party channel at `channel_narrowed` (still reopenable). An overlay is
already the minimal honest claim, so any non-floor gap drops it below the read-only
overlay too. Labs channels never accrue narrowing.

## Regeneration

```bash
# Rust: regenerate the support export and report (identical bytes each run).
cargo run -p aureline-runtime --example dump_m5_output_channels > \
  artifacts/tooling/m5-output-channels/support_export.json
cargo run -p aureline-runtime --example dump_m5_output_channels summary > \
  artifacts/tooling/m5-output-channels/report.md

# Python: regenerate the perturbation corpus and validate end-to-end.
python3 tools/release/output_channel_virtualization.py emit-corpus
python3 tools/release/output_channel_virtualization.py self-test
```
