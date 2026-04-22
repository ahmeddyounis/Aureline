# Evidence-id conventions and artifact-linking rules

This document freezes how packet families name proof artifacts and how
they link to one another. It is the naming companion to
`schemas/governance/evidence_packet_header.schema.json`.

## Canonical syntax

Every stable evidence id uses lowercase ASCII tokens separated by `.`
and starts with the `evidence.` prefix.

```text
evidence.<scope>.<subject>[.<artifact>...]
```

Machine-check rule:

```text
^evidence\.[a-z0-9]+(?:[_-][a-z0-9]+)*(?:\.[a-z0-9]+(?:[_-][a-z0-9]+)*)+$
```

Rules:

- Keep ids stable when freshness, review state, or publication posture
  changes. Refresh metadata; do not rename the evidence id.
- Do not encode dates, file extensions, reviewer handles, or temporary
  branch names into the id.
- Reuse the same id everywhere the same artifact is cited: design
  indexes, benchmark packets, verification packets, support drills,
  signoff packets, and release evidence.
- When an artifact truly changes identity rather than freshness, mint a
  new id by changing the subject or artifact segment, not by appending
  prose.

## Segment intent

- **`scope`** identifies the packet family, seed program, issue slug, or
  durable review scope that owns the artifact.
- **`subject`** identifies the feature, workflow, claim family, drill, or
  design slice being evidenced.
- **`artifact`** identifies what the artifact actually is: packet,
  capture, run, trace, corpus, note, manifest, checklist, and so on.

Existing repository ids such as `evidence.m0.renderer.tradeoffs`,
`evidence.seed.benchmark.self_capture`, and
`evidence.signed_binary_chain_bypass.reproduction_steps` remain
conforming and canonical. New ids SHOULD prefer semantic scope tokens
over milestone-only scope tokens when no compatibility constraint
exists.

## Recommended families

| Family | Recommended shape | Example |
|---|---|---|
| Design capture | `evidence.design.<surface>.<capture>` | `evidence.design.renderer.primary_tradeoff_matrix` |
| Benchmark run or packet | `evidence.benchmark.<workflow>.<run_or_packet>` | `evidence.benchmark.editor_latency.reference_capture` |
| Verification corpus or packet | `evidence.verification.<topic>.<corpus_or_packet>` | `evidence.verification.renderer_viability.packet` |
| Support drill | `evidence.support.<drill>.<artifact>` | `evidence.support.recovery_ladder.capture` |
| Public-proof packet | `evidence.public_proof.<claim_family>.<packet>` | `evidence.public_proof.desktop_replacement.packet` |
| Known-limit note | `evidence.known_limit.<area>.<note>` | `evidence.known_limit.remote_attach.current_limit_note` |
| Migration packet | `evidence.migration.<path_or_archetype>.<packet>` | `evidence.migration.vscode_settings.import_packet` |

These shapes are recommendations, not a breaking migration for older
seed ids already checked into the repository.

## Packet-linking rules

Every packet family that embeds
`schemas/governance/evidence_packet_header.schema.json` follows these
link rules:

1. `evidence_id` identifies the packet record itself.
2. `supporting_evidence_ids` lists any design captures, benchmark runs,
   verification corpora, support drills, or other evidence objects the
   packet summary or claim rows rely on.
3. `exact_build_identity_refs`, `fixture_refs`, `archetype_refs`,
   `waiver_refs`, `known_limit_refs`, and `migration_packet_refs` carry
   stable refs or ids only. Do not substitute prose like "see benchmark
   packet above".
4. `claim_row_refs` and `requirement_ids` are the packet's logical join
   keys; `supporting_evidence_ids` are its proof join keys.
5. If two packets cite the same supporting artifact, they SHOULD cite
   the same `evidence_id` and only vary the packet-local rationale,
   freshness state, or visibility.

## Join examples

### Design evidence -> benchmark packet

- Design evidence index row:
  `evidence.m0.renderer.tradeoffs`
- Benchmark packet header:
  `supporting_evidence_ids: [evidence.m0.renderer.tradeoffs, evidence.seed.benchmark.self_capture]`

### Design evidence -> verification packet

- Design evidence index row:
  `evidence.m0.renderer.spike.manifest`
- Verification packet header:
  `supporting_evidence_ids: [evidence.m0.renderer.spike.manifest, evidence.seed.benchmark.self_capture]`

### Verification packet -> signoff packet

- Verification packet:
  `evidence.verification.renderer_viability.packet`
- Signoff packet or scorecard note:
  cite the packet path and preserve the packet header's `evidence_id`
  for cross-packet joins

## Artifact-linking discipline

- Design captures SHOULD link to the fixture, trace, or source-anchor
  refs that made the design review reproducible.
- Benchmark packets SHOULD link exact-build identity, corpus or task
  fixture refs, comparability notes, and any waiver ids in the same
  header.
- Verification packets SHOULD link supporting evidence ids first, then
  exact-build, fixture, archetype, waiver, known-limit, and migration
  refs as applicable.
- Support drills SHOULD preserve the same evidence ids when they are
  quoted in signoff, release, or known-limit packets.
- Public-proof packets SHOULD point back to the same evidence ids used
  in the internal verification packet that justified publication.

## Non-conforming patterns

- Renaming an evidence id because freshness changed.
- Minting one packet-local id and a second dashboard-local id for the
  same artifact.
- Using file paths or URLs as the join key when a stable evidence id
  exists.
- Replacing a stable exact-build, waiver, fixture, or archetype ref
  with narrative prose.
