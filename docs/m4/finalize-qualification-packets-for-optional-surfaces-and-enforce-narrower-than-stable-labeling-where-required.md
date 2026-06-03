# Finalize qualification packets for optional surfaces and enforce narrower-than-stable labeling where required

Reviewer-facing M4 stable-line proof packet for the finalized optional-surface qualification register. This register governs every optional surface exposed in the promoted M4 build and records per-deployment-target access modes so a missing packet forces automatic downgrade everywhere the surface appears.

Canonical machine source (do not clone status text from this packet — ingest the JSON):

- Register: [`/artifacts/release/finalize_qualification_packets_for_optional_surfaces_and_enforce.json`](../../artifacts/release/finalize_qualification_packets_for_optional_surfaces_and_enforce.json)
- Schema: [`/schemas/release/finalize-qualification-packets-for-optional-surfaces-and-enforce.schema.json`](../../schemas/release/finalize-qualification-packets-for-optional-surfaces-and-enforce.schema.json)
- Companion doc: [`/docs/release/optional_surface_qualification.md`](../../docs/release/optional_surface_qualification.md)
- Typed consumer: `aureline_release::finalize_qualification_packets_for_optional_surfaces_and_enforce`

This register is registered under the stable proof index through the `proof_index_ref` each surface's qualification packet carries, so a launch reviewer reaches the optional-surface narrowing from the same proof index that grounds the launch-blocking requirements.

## What this register proves

1. **Each optional surface binds a public claim to an optional qualification packet with per-deployment-target granularity.** Every surface binds one surface to the public claim whose lifecycle label it backs and to its qualification packet as an `Option`. A `null` packet is the canonical "no stable qualification packet" state. Each surface additionally records whether it is `Stable`, `Preview`, `inspect-only`, `handoff-only`, `hidden`, or `client/profile-limited` on `desktop_local`, `remote_helper`, `managed`, `self_hosted`, and `air_gapped` rows.

2. **A surface that lacks a stable qualification packet is automatically narrowed on every deployment target.** This is the inverse of the failure mode the release guardrails warn about: an optional surface does not inherit Stable from a neighbouring claim. A surface with `qualification_packet: null` must be `narrowed_no_packet`, must name `qualification_packet_absent`, may never render at or above the cutline on any deployment target, and never inherits an adjacent qualified surface.

3. **The register ingests the stable claim manifest as a hard ceiling.** The CI gate reads the stable claim manifest named by `claim_manifest_ref` and fails when a surface's `claim_label` is not the label that manifest publishes for the entry named by `claim_ref`. A surface's displayed label and every deployment access mode can never outrun the public claim it backs.

4. **The packet-freshness, waiver-expiry, and incompleteness stop rules narrow surfaces before promotion.** Each present packet carries a freshness SLO and a recorded `slo_state`. The CI gate recomputes the freshness state and the waiver-expiry state against the register `as_of` date. A capability that is unimplemented or evidence that is incomplete narrows the surface and all its deployment rows.

5. **The four surface kinds and the release-relevant surface set stay covered.** The gate fails if any of `opt_in_capability`, `optional_integration`, `secondary_platform`, or `experimental_preview` has no surface, if a declared release-relevant surface has no covering row, if a release-relevant row is not declared, or if a `surface_ref` repeats.

6. **The optional-surface qualification matrix contains explicit family rows for notebook/data-rich, voice/dictation, browser/mobile companion, preview/designer/publish, and AI-adjacent surfaces.** No family inherits a green label from a different packet class. v13 additions (browser-runtime inspectors, integrated package/dependency mutation, infrastructure/cluster live-state, pipeline/run-control overlays) and v22 additions (collaboration session admission, observer/follow modes, shared terminal/debug control, consent/retention envelopes, session export/delete) are enumerated with per-deployment-target labels.

7. **The publication verdict is recomputed, not asserted.** The gate recomputes the `hold`/`proceed` decision and the blocking rule/surface sets from the firing stop rules and fails on any drift. With `--require-proceed` it exits non-zero on `hold`, so shiproom and release tooling fail promotion directly from this artifact.

## Current snapshot (as of 2026-06-02)

The checked-in register holds promotion. Of 23 optional surfaces across 17 public claims, two render qualified stable (the BYOK provider-routing opt-in capability, and the notebook kernel bridge — the latter on an active waiver). Twenty-one surfaces are narrowed below the cutline:

- the **local model sidecar**, **notebook data explorer**, **voice dictation input**, **browser mobile companion**, **designer publish preview**, **AI memory context**, **browser runtime inspector**, **package dependency mutation**, **infrastructure cluster live state**, **pipeline run control overlay**, **collaboration session admission**, **collaboration observer follow**, **shared terminal debug control**, **consent retention envelope**, and **session export delete** surfaces are narrowed to `preview` because they carry **no stable qualification packet at all**;
- the **remote dev-container backend** is narrowed to `preview` because its qualification packet **breached its freshness SLO**;
- the **3D graph preview** is narrowed to `preview` because it names a GPU-acceleration capability the build does not yet implement;
- the **AI inline-refactor** capability is narrowed to `preview` because the **waiver it relied on expired**; and
- the **bulk export archive**, **RTL layout preview**, and **regulated air-gapped telemetry** surfaces inherit ceilings from public claims already published below the cutline.

Of 115 deployment rows across all release-relevant surfaces, 108 are narrowed and 7 render stable. Every optional surface that backs a Stable public claim but lacks a packet, carries a breached packet, or relies on an expired waiver fires a blocking stop rule. The register narrows these optimistic Stable optional surfaces automatically instead of letting them ride; promotion clears once the missing packets are authored, the breached packet is refreshed, the expired waiver is renewed, or those public claims are formally narrowed.

## How to re-verify

```
cargo test -p aureline-release finalize_qualification_packets_for_optional_surfaces_and_enforce
```

This runs the typed contract tests that bind the model to the checked-in register and the negative fixtures.
