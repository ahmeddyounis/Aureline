# Open/paid boundary, licensing, provenance, and contribution-policy audit

This document is the reviewer-facing companion for the open/paid boundary audit:

- [`/artifacts/release/open_paid_boundary_audit.json`](../../artifacts/release/open_paid_boundary_audit.json)
- schema: [`/schemas/release/open_paid_boundary_audit.schema.json`](../../schemas/release/open_paid_boundary_audit.schema.json)
- proof packet:
  [`/artifacts/release/m4/open_paid_boundary_audit_proof_packet.md`](../../artifacts/release/m4/open_paid_boundary_audit_proof_packet.md)

The audit is the **canonical truth** for the governance facts the stable launch rests
on: where the open-source core ends and the paid/managed tier begins, the licensing
posture, the build provenance, and the contribution policy. It locks one
qualification-and-publication model for those facts instead of side spreadsheets,
stale badges, or optimistic launch language. Downstream docs, Help/About surfaces,
shiproom panels, release packets, and support exports MUST ingest the audit by
`entry_id` and render `effective_label` rather than restating the status in prose.

## Why this audit exists

The stable claim manifest decides the single lifecycle label each public subject
publishes. The stable proof index and version windows ground the requirements and
interface surfaces that are meant to ship at the stable cutline. This audit adds the
governance-fact control beside those gates: an open/paid boundary, licensing,
provenance, or contribution-policy fact must not widen a public claim merely because a
neighbouring row is green. If a row's attestation packet is stale, a required audit
control is unsatisfied, its owner sign-off is missing, or its waiver lapses, its
effective label narrows below the cutline before publication.

## Audit domains

Each `row` belongs to one of four closed domains:

- `open_paid_boundary` — where the open-source core ends and the paid/managed tier
  begins, and the offboarding guarantees that prevent lock-in;
- `licensing` — SPDX coverage, the third-party license inventory, and redistribution
  compatibility;
- `provenance` — build-provenance attestation, SBOM, and signing;
- `contribution_policy` — DCO/CLA enforcement, contribution terms, maintainer
  governance, and the security-disclosure policy.

## Audit rows

Each `row` is one `(governance subject, public claim)` binding. It names:

- the audit domain and the subject it audits (`subject_ref`, `subject_summary`);
- whether it belongs to the release-blocking audit set;
- the stable claim manifest entry it backs (`claim_ref`) and the canonical lifecycle
  label that entry publishes (`claim_label`);
- the `attestation_packet`, with its freshness SLO and recorded `slo_state`;
- the required `audit_controls` the row must satisfy, each with a `satisfied` flag and
  the `control_ref` it checks;
- any waiver, the owner sign-off, active gap reasons, and the `effective_label`
  product surfaces render.

The lifecycle vocabulary is shared with the stable claim matrix:
`lts`, `stable`, `beta`, `preview`, and `withdrawn`.

## The launch cutline

The cutline fixes the boundary between a row that renders as Stable or LTS and one
narrowed below it:

```text
lts > stable   |   beta > preview > withdrawn   (below the cutline)
```

A row renders at or above the cutline only when it carries a captured attestation
packet within its freshness SLO, every required audit control is satisfied, the row
owner has signed, no waiver it relies on has expired, and its backing public claim is
itself at or above the cutline. Otherwise the audit narrows the row to `beta`,
`preview`, or `withdrawn`.

## Packet-freshness SLO {#packet-freshness-slo}

Each `attestation_packet` carries:

- `target_max_age_days` — the maximum age before the packet is stale;
- `warn_within_days` — the remaining-days threshold for `due_for_refresh`;
- `slo_register_ref` — this section, the source of the packet freshness rule.

The audit uses a 90-day target with a 30-day warning window for governance
attestation packets. The CI gate recomputes each packet's state from `captured_at`
against the audit `as_of` date and fails when the declared state is fresher than the
clock allows or when an attested row rides a breached packet.

## Audit states

- `attested` — the row has current proof, satisfied controls, and an owner sign-off,
  and renders the public claim label.
- `attested_on_waiver` — the row renders the claim label only because an active,
  unexpired waiver covers a recorded residual gap.
- `narrowed_unbacked` — a required audit control is unsatisfied, the evidence is
  incomplete, or the owner sign-off is missing.
- `narrowed_claim_narrowed` — the backing public claim is itself below the cutline, so
  the row inherits that ceiling.
- `narrowed_stale` — the attestation packet breached its freshness SLO (or is missing).
- `narrowed_waiver_expired` — a waiver the row relied on has expired.

## Gap reasons and stop rules

The closed gap-reason vocabulary is:

- `claim_label_narrowed`
- `audit_evidence_incomplete`
- `attestation_packet_freshness_breached`
- `attestation_packet_missing`
- `waiver_expired`
- `owner_signoff_missing`
- `audit_control_unsatisfied`

Every reason has a stop `rule` watching for it. The `claim_label_narrowed` rule is
non-blocking because the stable claim manifest already narrowed the upstream claim.
The remaining reasons block promotion when they fire under a Stable or LTS public
claim: they indicate a governance row that could be read as Stable but does not have
the proof, controls, or sign-off to carry that label.

## Coverage

`release_blocking_audit_refs` is the closed set of audit subjects the release line
must cover. The gate fails when:

- a declared release-blocking subject has no row;
- a release-blocking row is not declared;
- an `entry_id` or a `subject_ref` appears on more than one row;
- any of the four domains has no row.

This keeps a governance subject from quietly dropping out of release control.

## Publication verdict

The `publication` block records the shiproom verdict for this audit. It is `hold` when
any blocking rule fires and `proceed` otherwise. The gate recomputes the decision,
`blocking_rule_ids`, `blocking_entry_ids`, and summary counts and fails on any drift.

At this revision the audit holds publication. The licensing redistribution row has an
unresolved copyleft-bundling control, the contribution DCO/CLA attestation breached
its freshness SLO, and the maintainer-governance row relied on an expired waiver. All
three sit under claims still published Stable, so the audit narrows them below the
cutline and blocks promotion until their controls, packets, or waivers are fixed or the
upstream public claims are narrowed.

## CI gate

Run:

```sh
python3 ci/check_open_paid_boundary_audit.py --repo-root .
```

The gate fails when closed vocabularies or the cutline drift; when an attested row
carries active gap reasons, stale proof, an unsatisfied control, or a missing owner
sign-off; when a narrowed row does not drop below the cutline; when a row renders wider
than its public claim; when the claim label disagrees with the stable claim manifest;
when freshness or waiver-expiry arithmetic is overstated; when coverage drops; when
publication or summary fields drift; or when referenced artifacts are missing. It also
runs negative drills and fixture cases under
[`/fixtures/release/open_paid_boundary_audit/`](../../fixtures/release/open_paid_boundary_audit/)
and writes
[`/artifacts/release/captures/open_paid_boundary_audit_validation_capture.json`](../../artifacts/release/captures/open_paid_boundary_audit_validation_capture.json).

Shiproom and release tooling can fail promotion directly from this artifact:

```sh
python3 ci/check_open_paid_boundary_audit.py --repo-root . --require-proceed
```

This exits with code 2 when the recomputed publication verdict is `hold`, distinct from
an invalid artifact failure.

The typed Rust consumer
(`aureline_release::open_paid_boundary_audit::current_open_paid_boundary_audit`)
reads the same audit and exposes `support_export_projection()` for Help/About and
support export consumers, so `cargo test -p aureline-release` enforces the structural
invariants without a separate build step.

## Update rules

1. Capture or refresh the attestation packet first, then point the row at the packet,
   proof-index row, evidence refs, audit controls, and owner sign-off.
2. Set `audit_state`, `active_gap_reasons`, `slo_state`, and `effective_label` to the
   honest posture. A row with a stale packet, an unsatisfied control, an expired
   waiver, or a narrowed backing claim must display below the cutline.
3. Recompute the `publication` and `summary` blocks, run
   `python3 ci/check_open_paid_boundary_audit.py --repo-root . --check`, and commit the
   regenerated validation capture with the audit.
4. If the evidence supports only a narrower label, narrow the row and packet rather than
   preserving optimistic Stable wording.
