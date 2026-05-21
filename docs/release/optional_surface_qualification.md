# Optional-surface qualification and claim narrowing

This document is the reviewer-facing companion for the optional-surface
qualification register:

- [`/artifacts/release/optional_surface_qualification.json`](../../artifacts/release/optional_surface_qualification.json)
- schema: [`/schemas/release/optional_surface_qualification.schema.json`](../../schemas/release/optional_surface_qualification.schema.json)
- proof packet:
  [`/artifacts/release/m4/optional_surface_qualification_proof_packet.md`](../../artifacts/release/m4/optional_surface_qualification_proof_packet.md)

The register is the **canonical truth** for optional surfaces that can otherwise
look more mature than their evidence. It governs opt-in capabilities, optional
integrations, secondary platforms, and shipped-but-experimental previews whose
default posture is narrowed until they carry their own stable qualification
packet. Downstream docs, Help/About surfaces, shiproom panels, release packets,
and support exports MUST ingest the register by `surface_id` and render
`displayed_label` rather than restating the status in prose.

## Why this register exists

The stable claim manifest decides the single lifecycle label each public subject
publishes. The stable qualification matrix and proof index ground surfaces that
are meant to ship at the stable cutline. Optional surfaces need the inverse
control: a surface must not inherit Stable merely because a neighboring claim is
green. If a surface lacks a stable qualification packet, its displayed label
narrows below the cutline before publication.

## Surface rows

Each `surface` row is one `(optional surface, public claim)` binding. It names:

- the optional surface (`surface_kind`, `surface_ref`, and `surface_summary`);
- whether it belongs to the release-relevant surface set;
- the stable claim manifest entry it backs (`claim_ref`) and the canonical
  lifecycle label that entry publishes (`claim_label`);
- the optional `qualification_packet`, with no packet represented by omitting the
  block rather than by a placeholder packet;
- any waiver, the owner sign-off, active narrow reasons, and the
  `displayed_label` product surfaces render.

The lifecycle vocabulary is shared with the stable claim matrix:
`lts`, `stable`, `beta`, `preview`, and `withdrawn`.

## The launch cutline

The cutline fixes the boundary between a surface that renders as Stable or LTS
and one narrowed below it:

```text
lts > stable   |   beta > preview > withdrawn   (below the cutline)
```

An optional surface renders at or above the cutline only when it carries a
captured stable qualification packet within its freshness SLO, complete surface
evidence, owner sign-off, no expired waiver, and a backing public claim that is
itself at or above the cutline. Otherwise the register narrows the surface to
`beta`, `preview`, or `withdrawn`.

## Packet-freshness SLO {#packet-freshness-slo}

Each present `qualification_packet` carries:

- `target_max_age_days` — the maximum age before the packet is stale;
- `warn_within_days` — the remaining-days threshold for `due_for_refresh`;
- `slo_register_ref` — this section, the source of the packet freshness rule.

The register uses a 180-day target with a 30-day warning window for optional
surface packets. The CI gate recomputes each packet's state from `captured_at`
against the register `as_of` date and fails when the declared state is fresher
than the clock allows or when a qualified surface rides a breached packet.

## Surface states

- `qualified_stable` — the surface has current proof, complete evidence, and
  owner sign-off, and renders the public claim label.
- `qualified_on_waiver` — the surface renders the claim label only because an
  active, unexpired waiver covers a recorded gap.
- `narrowed_no_packet` — the surface has no stable qualification packet and must
  render below the cutline.
- `narrowed_incomplete` — capability, evidence, or sign-off is incomplete.
- `narrowed_stale` — the qualification packet breached its freshness SLO.
- `narrowed_claim_narrowed` — the backing public claim is itself below the
  cutline, so the surface inherits that ceiling.
- `narrowed_waiver_expired` — a waiver the surface relied on has expired.

## Narrow reasons and stop rules

The closed narrow-reason vocabulary is:

- `claim_label_narrowed`
- `qualification_packet_absent`
- `surface_capability_absent`
- `surface_evidence_incomplete`
- `qualification_packet_breached`
- `waiver_expired`
- `owner_signoff_missing`

Every reason has a `stop_rule` watching for it. The `claim_label_narrowed` rule
is non-blocking because the stable claim manifest already narrowed the upstream
claim. The remaining reasons block promotion when they fire under a Stable or
LTS public claim: they indicate an optional surface that could be interpreted as
Stable but does not have the proof to carry that label.

## Coverage

`release_relevant_surface_refs` is the closed set of optional surfaces the
release line must cover. The gate fails when:

- a declared release-relevant surface has no release-relevant row;
- a release-relevant row is not declared;
- a `surface_ref` appears on more than one row;
- any of the four surface kinds has no row.

This keeps an optional surface from quietly dropping out of release control.

## Publication verdict

The `publication` block records the shiproom verdict for this register. It is
`hold` when any blocking rule fires and `proceed` otherwise. The gate recomputes
the decision, `blocking_rule_ids`, `blocking_surface_ids`, and summary counts and
fails on any drift.

At this revision the register holds publication. The local model sidecar has no
stable qualification packet, the remote dev-container backend has a breached
packet, the three-dimensional graph preview names an unimplemented capability,
and the AI inline-refactor capability relied on an expired waiver. All four sit
under claims still published Stable, so the register narrows them below the
cutline and blocks promotion until their packets, evidence, or waivers are fixed
or the upstream public claims are narrowed.

## CI gate

Run:

```sh
python3 ci/check_optional_surface_qualification.py --repo-root .
```

The gate fails when closed vocabularies or the cutline drift; when a surface with
no packet renders qualified; when a narrowed surface does not drop below the
cutline; when a qualified surface carries active narrow reasons, stale proof, or
missing owner sign-off; when a surface renders wider than its public claim; when
the claim label disagrees with the stable claim manifest; when freshness or
waiver-expiry arithmetic is overstated; when coverage drops; when publication or
summary fields drift; or when referenced artifacts are missing. It also runs
negative drills and fixture cases under
[`/fixtures/release/optional_surface_qualification/`](../../fixtures/release/optional_surface_qualification/)
and writes
[`/artifacts/release/captures/optional_surface_qualification_validation_capture.json`](../../artifacts/release/captures/optional_surface_qualification_validation_capture.json).

Shiproom and release tooling can fail promotion directly from this artifact:

```sh
python3 ci/check_optional_surface_qualification.py --repo-root . --require-proceed
```

This exits with code 2 when the recomputed publication verdict is `hold`,
distinct from an invalid artifact failure.

The typed Rust consumer
(`aureline_release::optional_surface_qualification::current_optional_surface_qualification`)
reads the same register and exposes `support_export_projection()` for Help/About
and support export consumers, so `cargo test -p aureline-release` enforces the
structural invariants without a separate build step.

## Update rules

1. Add or refresh the qualification packet first, then point the surface row at
   the packet, proof-index row, evidence refs, and owner sign-off.
2. Set `surface_state`, `active_narrow_reasons`, `slo_state`, and
   `displayed_label` to the honest posture. A surface with no packet, stale
   packet, expired waiver, incomplete evidence, or narrowed backing claim must
   display below the cutline.
3. Recompute the `publication` and `summary` blocks, run
   `python3 ci/check_optional_surface_qualification.py --repo-root . --check`,
   and commit the regenerated validation capture with the register.
4. If the evidence supports only a narrower label, narrow the surface and packet
   rather than preserving optimistic Stable wording.
