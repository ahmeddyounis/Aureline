# Optional-surface qualification — proof packet

Reviewer-facing proof packet for the gated claim-narrowing automation that governs optional
surfaces — opt-in capabilities, optional integrations, secondary platforms, and
shipped-but-experimental previews — whose default is *narrowed* and which never inherit a
Stable claim from an adjacent qualified surface.

Canonical machine source (do not clone status text from this packet — ingest the JSON):

- Register: [`/artifacts/release/optional_surface_qualification.json`](../optional_surface_qualification.json)
- Schema: [`/schemas/release/optional_surface_qualification.schema.json`](../../../schemas/release/optional_surface_qualification.schema.json)
- Companion doc: [`/docs/release/optional_surface_qualification.md`](../../../docs/release/optional_surface_qualification.md)
- Validator: `ci/check_optional_surface_qualification.py`
- Validation capture:
  [`/artifacts/release/captures/optional_surface_qualification_validation_capture.json`](../captures/optional_surface_qualification_validation_capture.json)
- Typed consumer: `aureline_release::optional_surface_qualification`

This register is registered under the stable proof index through the
`proof_index_ref` each surface's qualification packet carries
(`artifacts/release/stable_proof_index.json#proof:*`), so a launch reviewer reaches the
optional-surface narrowing from the same proof index that grounds the launch-blocking
requirements, rather than from ad hoc notes.

## What this register proves

1. **Each optional surface binds a public claim to an *optional* qualification packet.**
   Every surface binds one surface (`surface_kind`, `surface_ref`) to the public claim whose
   lifecycle label it backs (`claim_ref`, `claim_label`) and to its qualification packet as an
   `Option` (`qualification_packet`). A `null` packet is the canonical "no stable qualification
   packet" state. The register reuses the stable claim level vocabulary rather than minting
   per-surface labels, so docs, Help/About, the release center, and support exports render one
   label per surface.

2. **A surface that lacks a stable qualification packet is automatically narrowed.** This is
   the inverse of the failure mode the release guardrails warn about: an optional surface does
   not inherit Stable from a neighbouring claim. A surface with `qualification_packet: null`
   must be `narrowed_no_packet`, must name `qualification_packet_absent`, may never render at
   or above the cutline, and the only remediation is to author and capture a packet
   (`author_qualification_packet`). The typed model and the CI gate both enforce this.

3. **The register ingests the stable claim manifest as a hard ceiling.** The CI gate reads the
   stable claim manifest named by `claim_manifest_ref` and fails when a surface's `claim_label`
   is not the label that manifest publishes for the entry named by `claim_ref`, when a surface
   names an entry the manifest does not carry, or when a surface is rendered wider than the
   public claim's canonical label. A surface's displayed label can never outrun the public
   claim it backs.

4. **The packet-freshness, waiver-expiry, and incompleteness stop rules narrow surfaces before
   promotion.** Each present packet carries a freshness SLO and a recorded `slo_state`. The CI
   gate recomputes the freshness state and the waiver-expiry state against the register `as_of`
   date, failing when a declared state overstates the clock, when a qualified surface rides a
   stale packet or an expired waiver, or when a breached packet keeps rendering Stable. A
   capability that is unimplemented or evidence that is incomplete narrows the surface too.

5. **The four surface kinds and the release-relevant surface set stay covered.** The gate fails
   if any of `opt_in_capability`, `optional_integration`, `secondary_platform`, or
   `experimental_preview` has no surface, if a declared release-relevant surface has no covering
   row, if a release-relevant row is not declared, or if a `surface_ref` repeats.

6. **The publication verdict is recomputed, not asserted.** The gate recomputes the
   `hold`/`proceed` decision and the blocking rule/surface sets from the firing stop rules and
   fails on any drift. With `--require-proceed` it exits non-zero on `hold`, so shiproom and
   release tooling fail promotion directly from this artifact.

## Current snapshot (as of 2026-05-21)

The checked-in register holds promotion. Of nine optional surfaces across five public claims,
two render qualified and back Stable claims cleanly (the BYOK provider-routing opt-in
capability, and the notebook kernel bridge — the latter on an active waiver). Seven surfaces
are narrowed below the cutline:

- the **local model sidecar** narrowed to preview because it carries **no stable qualification
  packet at all**;
- the **remote dev-container backend** narrowed to beta because its qualification packet
  breached its freshness SLO;
- the **3D graph preview** narrowed to beta because it names a GPU-acceleration capability the
  build does not yet implement;
- the **AI inline-refactor** capability narrowed to beta because the waiver it relied on
  expired; and
- the **bulk export archive**, **RTL layout preview**, and **air-gapped regulated telemetry**
  surfaces inherit ceilings from public claims already published below the cutline (beta,
  preview, beta).

Four of those — the absent packet, the breached packet, the unimplemented capability, and the
expired waiver — back claims still published Stable, so they fire four blocking stop rules and
hold the `release.shiproom.optional_surface_qualification` gate. The register narrows the
optimistic Stable optional surfaces automatically instead of letting them ride; promotion
clears once the missing packet is authored, the breached packet is refreshed, the capability
lands, and the waiver is renewed (or those public claims are formally narrowed).

## How to re-verify

```
python3 ci/check_optional_surface_qualification.py --repo-root . --check
cargo test -p aureline-release
```

The first command revalidates the register, recomputes the freshness/waiver automations and
the absent-packet narrowing against `as_of`, runs the negative drills and fixture cases, and
writes the validation capture. The second runs the typed contract tests that bind the model to
the checked-in register, the frozen capture, and the negative fixtures. Add `--require-proceed`
to the gate to turn the recorded `hold` into a non-zero exit for shiproom use.
