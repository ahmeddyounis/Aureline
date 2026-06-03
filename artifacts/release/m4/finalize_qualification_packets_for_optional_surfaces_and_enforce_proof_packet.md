# Finalize qualification packets for optional surfaces and enforce narrower-than-stable labeling where required — proof packet

Reviewer-facing proof packet for the M4 stable-line finalization of optional-surface qualification.

## Canonical sources

- Register: [`/artifacts/release/finalize_qualification_packets_for_optional_surfaces_and_enforce.json`](../finalize_qualification_packets_for_optional_surfaces_and_enforce.json)
- Schema: [`/schemas/release/finalize-qualification-packets-for-optional-surfaces-and-enforce.schema.json`](../../schemas/release/finalize-qualification-packets-for-optional-surfaces-and-enforce.schema.json)
- Companion doc: [`/docs/m4/finalize-qualification-packets-for-optional-surfaces-and-enforce-narrower-than-stable-labeling-where-required.md`](../../docs/m4/finalize-qualification-packets-for-optional-surfaces-and-enforce-narrower-than-stable-labeling-where-required.md)
- Typed consumer: `aureline_release::finalize_qualification_packets_for_optional_surfaces_and_enforce`

## What this packet proves

1. **Per-deployment-target narrowing.** Every optional surface records its access mode (`stable`, `preview`, `inspect-only`, `handoff-only`, `hidden`, `client/profile-limited`) on each deployment target (`desktop_local`, `remote_helper`, `managed`, `self_hosted`, `air_gapped`). A missing packet forces automatic downgrade on every target.

2. **Explicit family coverage.** The matrix contains rows for notebook/data-rich, voice/dictation, browser/mobile companion, preview/designer/publish, AI-adjacent, browser-runtime inspectors, package/dependency mutation, infrastructure/cluster live-state, pipeline/run-control overlays, collaboration session admission, observer/follow modes, shared terminal/debug control, consent/retention envelopes, and session export/delete surfaces.

3. **No inheritance of adjacent green rows.** An optional surface with no packet, a breached packet, incomplete evidence, an expired waiver, or a narrowed backing claim is structurally required to drop below the cutline rather than inherit an adjacent qualified surface.

4. **Shiproom can fail promotion from this artifact.** The register publication verdict is `hold` because multiple surfaces back Stable claims while narrowed. Recomputing the decision from the firing stop rules yields the same `hold` verdict.

## Current snapshot (as of 2026-06-02)

- 23 optional surfaces across 17 public claims
- 2 qualified stable (1 on active waiver)
- 21 narrowed below the cutline
- 108 of 115 deployment rows narrowed
- Publication verdict: `hold`

## How to re-verify

```
cargo test -p aureline-release finalize_qualification_packets_for_optional_surfaces_and_enforce
```
