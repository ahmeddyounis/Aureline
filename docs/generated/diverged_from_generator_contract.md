# Diverged-from-generator, override-review, and rebuild-intent contract

This contract freezes how Aureline represents, reviews, and recovers from
**direct edits to non-canonical artifacts** — generated outputs, mirrored
artifacts, imported artifacts with incomplete lineage, and structured-derived
artifacts (lockfiles, snapshots, notebook outputs, previews).

The goal is to make divergence **legible and recoverable**. A direct write to a
non-authoritative artifact MUST NOT silently blur the boundary between canonical
source and derived bytes.

This document is normative. If it disagrees with the PRD, TAD, ADRs, or the
generated-artifact safe-edit policy, those sources win and this contract plus
its companion schema/fixtures update in the same change.

## Companion artifacts

- [`/docs/architecture/generated_artifact_safe_edit_policy.md`](../architecture/generated_artifact_safe_edit_policy.md)
  — shared posture vocabulary (`artifact_origin_class`, `provenance_state`,
  `default_edit_posture`, `rebuild_intent`, `override_policy`,
  `active_override_provenance`) and per-class policy matrix.
- [`/docs/generated/lineage_hint_packet.md`](./lineage_hint_packet.md)
  — row-level projection rules for explorer/search/AI/support surfaces.
- [`/docs/workspace/mutation_lineage_model.md`](../workspace/mutation_lineage_model.md)
  — generated-artifact lineage model (`drift_state = manually_diverged`) and the
  writable-boundary contract that forces divergence markers.
- [`/schemas/generated/artifact_edit_posture.schema.json`](../../schemas/generated/artifact_edit_posture.schema.json)
  — boundary schema for the cross-surface posture record.
- [`/schemas/workspace/generated_artifact_lineage.schema.json`](../../schemas/workspace/generated_artifact_lineage.schema.json)
  — boundary schema for the full lineage/provenance record.
- [`/schemas/workspace/mutation_journal.schema.json`](../../schemas/workspace/mutation_journal.schema.json)
  — boundary schema for mutation-journal entries; divergence events are always
  attributable and previewable when policy requires.
- [`/schemas/generated/divergence_record.schema.json`](../../schemas/generated/divergence_record.schema.json)
  — boundary schema for a divergence/override decision record produced at the
  write boundary.
- [`/fixtures/generated/divergence_cases/`](../../fixtures/generated/divergence_cases/)
  — worked cases for override, refusal, drift, and recovery.

## Non-negotiable invariants

1. **Non-canonical bytes never masquerade as canonical source.**
   Any surface that shows or exports a derived artifact MUST preserve
   `do_not_imply_canonical_source = true` and MUST keep
   `(artifact_origin_class, provenance_state)` explicit.
2. **A direct write never makes an artifact “become canonical” implicitly.**
   A generated, mirrored, preview, or imported artifact stays non-canonical
   until an explicit adoption/promotion workflow (out of scope for this
   contract) updates its origin class.
3. **Divergence is contract-backed, not UI-copy-backed.**
   “Diverged” is a typed state carried through posture, lineage, row hints, and
   export/support packets. No surface invents local labels.
4. **Override is never silent.**
   When a write crosses a declared writable boundary, the pipeline MUST record
   (a) divergence state, (b) review gating, (c) override provenance, and (d) the
   rebuild intent that explains how (or whether) the artifact can be made
   authoritative again.
5. **Recovery guidance is typed.**
   Surfaces point at `rebuild_intent.action_ref` (or explicitly state the
   absence of one) instead of relying on prose-only “try regenerating”.

## Terminology (frozen meanings)

- **Canonical source**: the authored object that is the approved place to make
  semantic changes (`artifact_origin_class = canonical_source`).
- **Derived artifact**: an artifact produced from canonical inputs by a
  generator, resolver, runtime, build toolchain, mirror pipeline, or importer.
- **Direct edit / override**: a write to a derived artifact that is not proven
  to be round-trip-safe for that class, or that lands outside declared safe
  ranges for a structured artifact.
- **Diverged-from-generator**: the explicit state where a derived artifact’s
  bytes are no longer the generator/mirror/runtime output and the difference is
  the result of a direct write admitted through an override policy (or an
  external rewrite detected at reopen that is treated equivalently at surfaces).

## 1. Diverged-from-generator state (frozen mapping)

“Diverged” is represented across two layers, both required:

### 1.1 Posture layer (cross-surface projection)

The artifact’s `artifact_edit_posture_record` MUST reflect divergence as:

- `artifact_origin_class != canonical_source`
- `provenance_state = diverged_from_generator`
- `do_not_imply_canonical_source = true`
- `active_override_provenance` present (required by the posture schema)

Rules:

1. A posture record MUST NOT use `provenance_state = in_sync` once a write is
   admitted outside the writable boundary, even if the canonical inputs have
   not changed.
2. A posture record with `provenance_state = diverged_from_generator` MUST be
   sufficient for list rows, AI citations, and exports to remain honest even
   when the full lineage record is unavailable.

### 1.2 Lineage layer (full generated-artifact lineage record)

When a generated-artifact lineage record is available, divergence MUST also be
reflected as:

- `drift_state = manually_diverged`

and the lineage record MUST preserve:

- the writable-boundary kind that was crossed (or the fact that a “full override
  with divergence marker” policy applied);
- the regeneration hint(s) (when available), or an explicit
  `manual_instructions` hint when automated regeneration cannot be proven.

### 1.3 Precedence (single-axis posture state)

`provenance_state` is a single axis. When multiple drift facts exist, the
following precedence applies:

1. `unknown_lineage` (cannot prove the lineage or canonical source) dominates.
2. `diverged_from_generator` dominates `stale_inputs` and `generator_changed`
   because the artifact is already past the writable boundary.
3. Otherwise, `generator_changed` dominates `stale_inputs`.

Surfaces MAY show additional detail (e.g. “generator changed since last run”)
in a lineage panel, but they MUST keep the chip tokens above.

## 2. Override admission rules (what is allowed, and when)

Override is policy-driven and artifact-class-driven. A write is admissible only
when the posture record declares it.

### 2.1 Allowed override classes

Override policy is the only admissible “permission to edit” for derived
artifacts:

- `override_policy.policy_class = declared_safe_ranges_only`
  - Meaning: Only the declared safe ranges may be edited.
  - Writes outside those ranges are treated as override attempts that MUST be
    refused unless the class also supports full override.
- `override_policy.policy_class = declared_full_override_with_divergence`
  - Meaning: A full override is admissible, but it MUST force divergence state
    and MUST record override provenance + review gating.
- `override_policy.policy_class = mirror_promotion_only`
  - Meaning: No local edits on the mirror-controlled path; recovery is mirror
    refresh/promotion only.
- `override_policy.policy_class = not_available`
  - Meaning: No direct edit path exists for this artifact class in this state.

### 2.2 Review gating (override-review sheet)

If a posture record’s override policy says review is required, then the write
pipeline MUST enforce it:

- A write that requires review MUST NOT proceed without a `review_ref` that can
  be resolved by support/export surfaces.
- The review MUST be bound to an explicit compare surface:
  - structured rewrites use compare-before-apply;
  - ordinary file edits use compare-before-write at the save boundary;
  - generator/mirror override flows additionally require a “compare to expected
    generated/mirrored basis” view when that basis can be computed.

### 2.3 Compare requirements (frozen)

- **Compare-before-write** is mandatory for any durable write that replaces an
  on-disk object identity token (ADR-0006); a derived artifact never opts out.
- **Compare-before-apply** is mandatory for structured rewrites that are not
  provably lossless or whose round-trip guarantees are uncertain.

If the required compare surface cannot be produced (basis unknown, diff
unavailable, policy blocks raw preservation where needed), the pipeline MUST
refuse the write rather than guessing.

## 3. Required lineage field updates after an admitted override

When a write is admitted and crosses the writable boundary (full override or an
out-of-range structured edit), the producer MUST update the following fields.
The updates are contract-required so downstream surfaces never improvise repair
paths.

### 3.1 Posture record updates

The artifact’s posture record MUST:

1. set `provenance_state = diverged_from_generator`;
2. ensure `do_not_imply_canonical_source = true`;
3. set or update `rebuild_intent` to the correct recovery intent (see §4);
4. include `active_override_provenance` with:
   - `declared_by`, `declared_at`;
   - `reason_class`;
   - `review_ref`;
   - `rebuild_acknowledgement_class`;
   - optional `command_or_doc_ref`.

### 3.2 Generated-artifact lineage updates (when available)

The lineage record MUST:

1. set `drift_state = manually_diverged`;
2. update `output_digest` to the new on-disk bytes’ digest;
3. preserve the regeneration hints and keep them visible even when divergence
   exists (regenerate may still be the recovery path);
4. update `provenance.mutation_group_id` (or equivalent linkage) so mutation
   journal, lineage, and support bundles can round-trip.

### 3.3 Divergence record emission

Every admitted override that forces divergence MUST emit one
`divergence_record` (schema in
[`/schemas/generated/divergence_record.schema.json`](../../schemas/generated/divergence_record.schema.json))
so search, review, AI, export, and support surfaces can quote a single
machine-readable answer to:

- what was overridden;
- why it was admissible or refused;
- what compare/review evidence exists;
- what recovery/rebuild intent applies now.

## 4. Rebuild intent and recovery rules (frozen guidance)

Rebuild intent is the shared “what makes this authoritative again?” answer.
Surfaces MUST prefer the typed `rebuild_intent` field over free-form guidance.

Minimum rules:

1. If the canonical source is known and a generator/mirror action exists,
   `rebuild_intent.intent_class` MUST be one of:
   - `regenerate_required_before_authoritative_claims` (codegen/lockfile);
   - `mirror_refresh_required` (mirror artifacts);
   - `reexecute_required_for_fresh_output` (notebook outputs / preview/runtime).
2. If canonical source is unknown or lineage is incomplete,
   `rebuild_intent.intent_class` MUST be `manual_recovery_required`.
3. A divergence that is intentionally preserved is still non-canonical; the
   posture MUST remain explicit and the rebuild intent MUST not claim
   authoritative parity.

## 5. Seed cases

The worked divergence cases under
[`/fixtures/generated/divergence_cases/`](../../fixtures/generated/divergence_cases/)
cover, at minimum:

1. safe override admitted (generated code sibling; review recorded);
2. blocked direct edit (mirror-controlled artifact);
3. unknown canonical source (refuse or force manual recovery posture);
4. generator version drift (compare-to-basis required before override);
5. imported artifact with partial lineage (non-canonical remains explicit);
6. recovery back to canonical or mirror-controlled state (regen/refresh intent).

## 6. Change rules

- Adding a new override/refusal outcome class, compare requirement class, or
  recovery intent is additive-minor and must update:
  - this contract;
  - the divergence record schema; and
  - at least one worked case under `fixtures/generated/divergence_cases/`.
- Repurposing an existing token is breaking and requires a new decision row
  plus companion updates across posture, lineage, and surface packets.

