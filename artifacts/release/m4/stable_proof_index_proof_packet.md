# Stable proof index — proof packet

Reviewer-facing proof packet for the gated stable proof index linking
launch-blocking requirements, proof packets, waivers, and public claims.

Canonical machine source (do not clone status text from this packet — ingest the
JSON):

- Index: [`/artifacts/release/stable_proof_index.json`](../stable_proof_index.json)
- Schema: [`/schemas/release/stable_proof_index.schema.json`](../../../schemas/release/stable_proof_index.schema.json)
- Companion doc: [`/docs/release/stable_proof_index.md`](../../../docs/release/stable_proof_index.md)
- Validator: `ci/check_stable_proof_index.py`
- Validation capture:
  [`/artifacts/release/captures/stable_proof_index_validation_capture.json`](../captures/stable_proof_index_validation_capture.json)
- Typed consumer: `aureline_release::stable_proof_index`

## What this packet proves

1. **Each launch-blocking requirement is bound to a packet, a waiver, and a public
   claim.** Every row binds one requirement (`requirement_ref`,
   `requirement_class`) to the proof packet that proves it (`proof_packet`), the
   waiver that holds it provisionally (`waiver`), and the public claim whose
   lifecycle label it backs (`claim_ref`, `claim_label`). The index reuses the
   stable claim level vocabulary rather than minting per-requirement labels, so
   docs, Help/About, the release center, and support exports render one label per
   row.

2. **The index ingests the stable claim manifest as a hard ceiling.** The CI gate
   reads the stable claim manifest named by `claim_manifest_ref` and fails when a
   row's `claim_label` is not the label that manifest publishes for the entry named
   by `claim_ref`, when a row names an entry the manifest does not carry, or when a
   proof is backed wider than the public claim's canonical label. A requirement's
   proven label can never outrun the public claim it backs.

3. **The packet-freshness SLO automation narrows stale proofs before publication.**
   Each row's proof packet carries a freshness SLO (`target_max_age_days`,
   `warn_within_days`) and a recorded `slo_state`. The CI gate recomputes the state
   from `captured_at` against the index `as_of` date and fails when a declared state
   overstates freshness or when a proven row rides a `breached`/`missing` packet.
   Provider completion quality is the worked example: its packet aged past the SLO,
   so the automation narrows it to beta and holds publication even though the
   provider public claim is still published Stable.

4. **The launch-blocking requirement set stays fully covered.** The gate fails
   closed when a declared launch-blocking requirement has no covering row, when a
   launch-blocking row is undeclared, or when a requirement ref repeats — so a
   launch-blocking requirement cannot quietly drop out of the index.

## Proof-index registration

Each row's proof packet registers under one row of the public-proof artifact index
([`/artifacts/milestones/m3/public_proof_index.md`](../../milestones/m3/public_proof_index.md))
via its `proof_packet.proof_index_ref`, so this lane's proof is anchored to the
public-proof artifact index rather than to ad hoc notes.

## Requirement → claim matrix at this revision

Per launch-blocking requirement, the lifecycle label the proof backs (public claim
ceiling in parentheses):

| Requirement (class) | Public claim (ceiling) | Proven label | Posture |
|---|---|---|---|
| Provider routing and fallback (functional_capability) | provider-aware language intelligence (stable) | stable | proven |
| Provider completion quality (functional_capability) | provider-aware language intelligence (stable) | **beta** | unproven — packet breached |
| Rollback live-hardware drill (data_safety) | repair and rollback safety (stable) | stable | proven on waiver |
| Rollback state integrity (data_safety) | repair and rollback safety (stable) | **beta** | unproven — waiver expired |
| Export/offboarding fidelity (data_portability) | export and offboarding support (beta) | beta | inherits narrowed claim |
| Localization locale coverage (internationalization) | localization readiness (preview) | preview | unbacked under narrowed claim |
| Regulated sovereign evidence (regulated_assurance) | regulated-environment assurance (beta) | **withdrawn** | unproven — packet missing |

A non-launch-blocking advisory requirement (regulated control-plane attestation) is
also indexed; it is unsigned with incomplete evidence under an already-narrowed
claim, and does not hold publication.

## Current posture

At this revision two of eight requirements are proven at the Stable label, six are
narrowed below the cutline, and seven of the eight are launch-blocking (two proven,
five unproven). Two launch-blocking requirements whose public claims are still
published Stable are themselves unproven — provider completion quality (packet
breached) and rollback state integrity (waiver expired) — so two blocking proof
rules fire and stable proof-index publication **holds**. The remaining narrowed rows
inherit a public claim that is already below the cutline; they record their gaps but
do not hold publication beyond the upstream narrowing. That is the honest posture:
the repository is pre-implementation and the aggregate public claims are optimistic,
so the proof index narrows the requirement-level truth beneath them.

## Accessibility of this lane

The index and its companion doc are text/JSON artifacts: the doc renders as headed
sections and plain Markdown tables (no color-only encoding), and the machine source
carries the same truth so Help/About, the release center, support exports, docs, and
shiproom dashboards ingest one record per requirement rather than restating status
text.

## How to refresh

1. Land the upstream stable claim manifest entry, the requirement-level
   qualification evidence, refreshed proof packets, and waivers first; point each
   row's `claim_ref`, `claim_label`, `proof_packet`, and `waiver` at the canonical
   records.
2. Set each row's `index_state`, `slo_state`, `active_gap_reasons`, and
   `proven_label` to the honest posture.
3. Recompute the `publication` block and `summary`, then run
   `python3 ci/check_stable_proof_index.py --repo-root .` and commit the
   regenerated capture in the same change set.
4. If delivery proves a narrower requirement than planned, narrow the proven label
   and update the index — do not paper over the gap with prose.
