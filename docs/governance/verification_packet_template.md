# Verification-packet template

<!--
Machine-readable packet families SHOULD embed a header that conforms to
schemas/governance/evidence_packet_header.schema.json.

Stable evidence ids, artifact-linking rules, and family naming
conventions live in artifacts/governance/evidence_id_conventions.md.
Freshness ceilings and rerun-trigger ids live in
artifacts/governance/evidence_freshness_slos.yaml and
artifacts/governance/evidence_rerun_triggers.yaml.

This template is the reviewer-facing narrative companion. It extends the
minimum Appendix-Y verification packet in the technical architecture
document so current packets can join design evidence, benchmark packets,
support drills, and future public-proof artifacts by stable ids instead
of free-text naming.
-->

Rules:

- Every verification packet SHOULD carry one shared header block. Packet
  bodies may vary by lane, but they do not redefine packet identity,
  ownership, freshness, visibility, or evidence-link fields.
- Every claim-bearing row SHOULD cite canonical requirement ids,
  stable claim-row refs, and stable supporting evidence ids in the same
  packet.
- If a packet summary, reviewer signoff, or waiver note relies on a
  design capture, benchmark run, verification corpus, support drill,
  known-limit note, or migration packet, that artifact SHOULD appear in
  the header's `supporting_evidence_ids` or packet body by stable id.
- Exact-build, fixture, archetype, waiver, and source-anchor joins
  SHOULD use stable refs, not prose such as "same build as above".
- `visibility_class` states whether the packet is public, internal, or
  mixed; narrow or restricted export posture may still be layered on top
  by the packet family that embeds this template.

## Shared header

Fill a machine-readable block that conforms to
`schemas/governance/evidence_packet_header.schema.json`.

```yaml
schema_version: 1
header_kind: evidence_packet_header
packet_family: verification_packet
packet_id: verification.<topic>.<scope>
evidence_id: evidence.<scope>.<subject>.<artifact>
title: <short verification title>
ownership:
  owner_dri: "@handle"
  evidence_owner: "@handle"
  backup_owner: null
  backup_waiver: single-maintainer-backup
coverage:
  requirement_ids:
    - ARCH-INV-001
  claim_row_refs:
    - claim_row:<family>.<row>
  covered_lanes:
    - governance_packets
result_status: pass
visibility_class: internal
freshness:
  captured_at: 2026-04-21T23:45:00Z
  stale_after: P14D
  freshness_class: warm_cached
  source_revision: commit:<sha-or-doc-revision>
  trigger_revision: requirement_register_seed@1
environment:
  channel_context: not_applicable
  deployment_context:
    - not_applicable
  environment_summary: >
    Reviewer packet over canonical design, benchmark, and signoff artifacts.
artifact_links:
  supporting_evidence_ids:
    - evidence.m0.renderer.tradeoffs
    - evidence.seed.benchmark.self_capture
  exact_build_identity_refs: []
  fixture_refs:
    - fixtures/text/renderer_decision_examples/
  archetype_refs:
    - compat_row:reference_workspaces.small_rust_self_host
  source_anchor_refs:
    - docs/adr/0002-renderer-text-stack-and-shaping-fallback.md
  waiver_refs: []
  known_limit_refs: []
  migration_packet_refs: []
```

## Packet

- **Packet family:** `verification_packet`
- **Packet scope:** subsystem, feature, release gate, matrix row, or
  architecture slice being verified
- **Summary:** one or two sentences stating what is being verified and
  what remains out of scope
- **Decision posture:** `pass` | `fail` | `mixed` | `waived` |
  `seed_only` | `needs_review` | `blocked`

## Claim coverage

List every claim-bearing or narrowing row the packet speaks for.

| Claim-row ref | Requirement id(s) | Status | Visibility | Supporting evidence ids | Notes |
|---|---|---|---|---|---|
| `<claim_row:...>` | `<REQ-...>` | `pass` | `public` | `<evidence-id>` | `<narrowing or caveat>` |
| ... | ... | ... | ... | ... | ... |

Rules:

- A row that cannot cite its canonical requirement id or supporting
  evidence ids is not promotion-grade proof.
- If a row is `waived`, list the waiver ref here and again in the
  waiver section below.
- If a row is `mixed` or `needs_review`, say exactly which scope is
  narrower than the nominal claim.

## Environment and scope

- **Environment summary:** repeat or expand the shared header's
  `environment_summary` when reviewers need more detail.
- **Release/deployment envelope:** channel, profile, locality, mirror,
  or offline posture if the packet is claim-bearing beyond local review.
- **Fixture refs:** stable fixture-set or corpus refs used in the
  verification.
- **Archetype refs:** stable certified-archetype, compatibility-row, or
  reference-workspace refs when the packet makes workflow or
  compatibility claims.
- **Exact-build linkage:** list `exact_build_identity_ref` values or say
  `not_applicable`.

## Evidence joins

Use this section when the packet aggregates evidence minted elsewhere.

| `evidence_id` | Family / source kind | Why it is linked here | Freshness note | Artifact ref |
|---|---|---|---|---|
| `<evidence-id>` | `<design_capture | benchmark_run | verification_corpus | support_drill | ...>` | `<supports which claim row>` | `<current/stale/narrowed>` | `<path or stable ref>` |
| ... | ... | ... | ... | ... |

Rules:

- Reuse the same `evidence_id` every time the same artifact is cited in
  benchmark, verification, support, or signoff packets.
- Do not mint a packet-local alias for an existing design-evidence or
  benchmark-evidence id.
- If freshness changes but the underlying artifact identity does not,
  update the packet header or row freshness fields; do not rename the
  `evidence_id`.

## Verification method

- **Verification classes used:** benchmark, fixture replay, schema
  validation, design review, support drill, exact-build audit,
  compatibility row, or other classes actually exercised
- **Procedure summary:** concise description of what was checked
- **Automation refs:** validators, scripts, or CI workflows that backed
  the result

## Known gaps and waivers

- **Waiver refs:** stable waiver ids or `none`
- **Known-limit refs:** stable note refs or `none`
- **Migration-packet refs:** stable migration or narrowing packet refs
  or `none`
- **Explicit gaps:** one line per gap that remains outside the verified
  envelope

## Reviewer signoff

For each reviewer or forum:

- **Reviewer / forum:** `@handle` or forum id
- **Decision:** `accept` | `reject` | `needs_follow_up` | `waived`
- **Date:** `YYYY-MM-DD`
- **Reviewed claim rows:** stable row refs
- **Blocking refs:** stable evidence ids, waiver refs, or packet refs

## Refresh trigger

- **Named rerun trigger:** prefer a trigger id from
  `artifacts/governance/evidence_rerun_triggers.yaml`
- **Expected freshness window:** when the packet should be refreshed if
  nothing else changes; it may not exceed the proof-class ceiling in
  `artifacts/governance/evidence_freshness_slos.yaml`
- **Next packet family to update with the same evidence ids:** release,
  benchmark-publication, migration, support, or signoff packet
