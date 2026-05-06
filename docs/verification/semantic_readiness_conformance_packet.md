# Semantic-readiness conformance and path-identity drift verification seed

This packet makes **semantic-readiness** auditable on difficult filesystem
identity and save-target scenarios so path truth and readiness truth cannot
diverge silently under export, recovery, migration, or support flows.

It is intentionally additive over the frozen readiness projection contract in
[`docs/filesystem/semantic_readiness_projection.md`](../filesystem/semantic_readiness_projection.md)
and the path-truth chip/inspector contract in
[`docs/fs/path_truth_packet.md`](../fs/path_truth_packet.md). Where this packet
disagrees with the frozen vocabulary or schemas, those sources win and this
packet must be updated in the same change.

Companion artifacts:

- [`/artifacts/filesystem/semantic_readiness_parity_matrix.yaml`](../../artifacts/filesystem/semantic_readiness_parity_matrix.yaml)
  — cross-surface parity contract (field-by-field comparability).
- [`/schemas/filesystem/semantic_readiness_failure.schema.json`](../../schemas/filesystem/semantic_readiness_failure.schema.json)
  — stable failure codes for readiness parity and export narrowing.
- [`/fixtures/filesystem/semantic_readiness_edge_cases/`](../../fixtures/filesystem/semantic_readiness_edge_cases/)
  — edge-case corpus spanning alias drift, case-only rename, moved targets,
  watcher staleness, save-target mismatch, and export timing skew.
- [`/fixtures/filesystem/semantic_readiness_cases/`](../../fixtures/filesystem/semantic_readiness_cases/)
  — baseline readiness projection cases (producer- and workspace-scoped).
- [`/schemas/filesystem/semantic_readiness_view.schema.json`](../../schemas/filesystem/semantic_readiness_view.schema.json)
  — readiness-inspector view record contract.
- [`/schemas/filesystem/save_target_token.schema.json`](../../schemas/filesystem/save_target_token.schema.json)
  — canonical filesystem-identity + semantic-readiness export boundary.

Normative sources projected here:

- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  — `REL-FS-013`, `REL-SUPPORT-002`, Section 12.2.1, and Section 27.22.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md`
  — Section 11.17 (canonical filesystem identity, alias sets, save
  coordination), plus the cross-surface “same record” posture for export and
  headless parity.
- `docs/filesystem/filesystem_identity_vocabulary.md`
  — frozen tokens and surface rules for identity layers and readiness states.
- `docs/filesystem/semantic_readiness_projection.md`
  — minimum parity fields and surface projection matrix.

## Shared header

```yaml
schema_version: 1
header_kind: evidence_packet_header
packet_family: verification_packet
packet_id: verification.filesystem.semantic_readiness_conformance_seed
evidence_id: evidence.filesystem.semantic_readiness_conformance_seed
title: Semantic-readiness conformance and path-identity drift seed
ownership:
  owner_dri: "@ahmeddyounis"
  evidence_owner: "@ahmeddyounis"
  backup_owner: null
  backup_waiver: single-maintainer-backup
coverage:
  requirement_ids:
    - REL-FS-013
    - REL-SUPPORT-002
  claim_row_refs: []
  covered_lanes:
    - support_export
    - recovery_ladder
    - migration
    - governance_packets
result_status: seed_only
visibility_class: internal
freshness:
  captured_at: 2026-05-06T00:00:00Z
  stale_after: P30D
  freshness_class: warm_cached
  source_revision: commit:working_tree
  trigger_revision: semantic_readiness_conformance_packet@1
environment:
  channel_context: not_applicable
  deployment_context:
    - not_applicable
  environment_summary: >
    Seed packet over the frozen semantic-readiness projection contract and the
    filesystem identity vocabulary, with an added edge-case corpus and a parity
    matrix that makes export, recovery, migration, and headless surfaces
    comparable field-for-field.
artifact_links:
  supporting_evidence_ids:
    - evidence.filesystem.semantic_readiness_parity_matrix
    - evidence.filesystem.semantic_readiness_edge_case_corpus
    - evidence.filesystem.semantic_readiness_failure_codes
  exact_build_identity_refs: []
  fixture_refs:
    - fixtures/filesystem/semantic_readiness_cases/
    - fixtures/filesystem/semantic_readiness_edge_cases/
  archetype_refs: []
  source_anchor_refs:
    - docs/filesystem/semantic_readiness_projection.md
    - docs/filesystem/filesystem_identity_vocabulary.md
    - docs/fs/path_truth_packet.md
  waiver_refs: []
  known_limit_refs: []
  migration_packet_refs: []
```

## Summary

This seed freezes one reviewer-facing conformance story for semantic-readiness
and filesystem identity drift:

- A readiness record is only comparable across surfaces if its parity-floor
  fields are preserved byte-for-byte (state, reason, safe actions, producer id,
  timestamps, and explainer copy).
- Any record that represents an object on a path-identity edge case must carry
  both the **presentation path** and the **canonical filesystem object**
  (including alias evidence) whenever they differ, so “which file is this?”
  cannot be answered differently on desktop, headless, migration, recovery, or
  support surfaces.
- When export/recovery/migration cannot preserve readiness parity, the surface
  must fail safely (narrow its claim) and emit a stable failure code rather
  than silently dropping fields or widening by omission.

This packet does not claim complete automation. It claims that the corpus, the
parity matrix, and the failure-code schema now exist so parity can be audited
without screenshots or per-surface interpretation.

## Claim coverage

| Packet row | Requirement id(s) | Status | Visibility | Supporting evidence ids | Notes |
|---|---|---|---|---|---|
| `packet_row:semantic_readiness.parity_matrix` | `REL-FS-013` | `seed_only` | `internal` | `evidence.filesystem.semantic_readiness_parity_matrix` | Field-by-field parity contract for desktop, headless/CLI, migration, recovery, and support export. |
| `packet_row:semantic_readiness.edge_case_corpus` | `REL-FS-013` | `seed_only` | `internal` | `evidence.filesystem.semantic_readiness_edge_case_corpus` | Reviewable fixtures covering path-identity drift and save-target mismatch cases. |
| `packet_row:semantic_readiness.failure_codes` | `REL-SUPPORT-002` | `seed_only` | `internal` | `evidence.filesystem.semantic_readiness_failure_codes` | Stable failure codes so parity drift is reportable and routable. |
| `packet_row:semantic_readiness.export_parity_floor` | `REL-SUPPORT-002`, `REL-FS-013` | `seed_only` | `internal` | `evidence.filesystem.semantic_readiness_parity_matrix` | Support and migration exports preserve readiness state + safe actions for the same fixture. |

## Conformance field set (parity floor)

The parity floor extends the readiness projection contract with **identity
evidence** required on path-drift cases.

### Always required (readiness parity)

For the same subject, these fields must match byte-for-byte across desktop UI,
headless/CLI, migration, recovery packets, and support exports:

- `subject` (stable id / identity carrier record)
- `semantic_readiness.state`
- `semantic_readiness.not_ready_reason` (when `state != exact`)
- `semantic_readiness.safe_next_action` (when `state != exact`)
- `semantic_readiness.safe_next_actions[]` (when present; order-preserving)
- `semantic_readiness.producer_id`
- `semantic_readiness.producer_version`
- `semantic_readiness.observed_at`
- `semantic_readiness.not_ready_explainer.title` + `.body` (when present)
- `semantic_readiness.support_export.packet_family` + `.redaction_policy`
- `semantic_readiness.support_export.parity_signature` (when present)

### Conditionally required (path-identity drift evidence)

When a record is about a concrete filesystem object **and**
`presentation_path.uri != canonical_filesystem_object.canonical_uri`, a
conforming surface must preserve enough identity evidence to prove which is
which:

- `subject.identity.presentation_path`
- `subject.identity.canonical_filesystem_object`
- `subject.identity.alias_set`
- `subject.identity.logical_workspace_identity` (join key across surfaces)

If any of the conditional evidence above is missing on an export or recovery
surface, the surface must narrow and emit a failure item with
`failure_code = missing_explainer` or `canonical_target_mismatch` (see
`schemas/filesystem/semantic_readiness_failure.schema.json`).

## Edge-case corpus

The edge-case corpus under
[`fixtures/filesystem/semantic_readiness_edge_cases/`](../../fixtures/filesystem/semantic_readiness_edge_cases/)
adds path-identity and save-target drift scenarios that cannot be derived from
“warm index” fixtures alone:

- alias-set drift (saved/exported alias truth differs from live truth);
- case-only rename (case-insensitive roots must not turn into false staleness);
- symlink/junction divergence (presentation path remains stable while canonical
  target changes);
- moved targets (rename/move that preserves canonical identity must stay exact);
- stale watcher state (watch fidelity downgrade must be visible and exported);
- save-target mismatch (user-entered path no longer maps to the same canonical
  object; save requires review or safe alternative);
- partial index (partial enumeration is visible and exported as non-exact); and
- exported snapshot timing skew (export uses the same parity fields and
  discloses any narrowing rather than silently widening by omission).

Each fixture is deliberately render-stable: a consumer may project it in a UI,
serialize it through machine output, or embed it into a recovery/support packet
without inventing new vocabulary.

