# Structured round-trip repair, raw-preservation, and compare-before-apply contract

This contract freezes the rules Aureline MUST follow when it proposes a
**structured repair** for notebooks or other user-owned structured
artifacts under fidelity threat: decode failures, schema drift, partial
availability, downgrade compatibility, or other conditions where a
writer cannot prove that a rewrite is safe and reversible.

It exists so Aureline never rewrites user-owned structured artifacts on
trust alone. When a repair is lossy (or cannot be proven lossless),
Aureline MUST preserve an exact raw copy, MUST require an explicit
compare-before-apply step, and MUST publish a machine-readable record
that enumerates what would be dropped, normalized, regenerated, or
reinterpreted.

This contract is normative. A surface that offers “Repair”, “Fix”,
“Normalize”, “Downgrade for compatibility”, “Strip outputs”, or any other
action that rewrites a structured artifact in response to a fidelity
threat MUST comply.

## Companion artifacts

- [`/schemas/notebooks/roundtrip_repair_packet.schema.json`](../../schemas/notebooks/roundtrip_repair_packet.schema.json)
  defines `structured_round_trip_repair_packet_record`.
- [`/fixtures/notebooks/roundtrip_repair_cases/`](../../fixtures/notebooks/roundtrip_repair_cases/)
  contains worked YAML cases for the required repair flows and refusal
  paths.

## Composition, not redefinition

This contract composes with (and MUST NOT contradict) the following
authoritative sources:

- [`/docs/notebooks/notebook_trust_and_roundtrip_preview_contract.md`](./notebook_trust_and_roundtrip_preview_contract.md)
  — round-trip risk taxonomy, apply-gating vocabulary, and preview-first
  disclosure obligations.
- [`/docs/editor/decode_recovery_and_save_consequence_contract.md`](../editor/decode_recovery_and_save_consequence_contract.md)
  — raw-byte preservation postures and “block until reviewed” semantics
  for undecodable/mixed-encoding content.
- [`/docs/state/feature_scoped_migration_failure_contract.md`](../state/feature_scoped_migration_failure_contract.md)
  — unknown-field preservation postures and partial-open routing rules.
- [`/docs/ux/preview_apply_revert_contract.md`](../ux/preview_apply_revert_contract.md)
  — preview/apply/rollback honesty and checkpoint expectation.
- [`/docs/support/repair_transaction_contract.md`](../support/repair_transaction_contract.md)
  — shared repair-transaction grammar and reversal-class vocabulary for
  Project Doctor and support surfaces.
- [`/docs/architecture/generated_artifact_safe_edit_policy.md`](../architecture/generated_artifact_safe_edit_policy.md)
  — generated/mirrored artifact edit posture and regenerate-first rules.
- [`/docs/adr/0022-notebook-document-model-kernel-transport-trust-and-diff-merge.md`](../adr/0022-notebook-document-model-kernel-transport-trust-and-diff-merge.md)
  — notebook identity, trust axes, widget trust, and raw-JSON fallback
  postures.

When these sources disagree, they win and this contract plus its
companion artifacts MUST update in the same change.

## Terminology (frozen meanings)

- **Suspect artifact**: the current artifact bytes or structure that a
  surface cannot safely interpret or cannot safely re-emit without risk.
  “Suspect” includes: decode recovery, schema-version mismatch, unknown
  fields under a non-preserving writer, missing widget/runtime adapters,
  or partial-open degraded availability.
- **Preserved raw copy**: an exact byte-for-byte capture of the suspect
  artifact, stored in an open-buffer attachment, recovery journal, or a
  checkpoint artifact. Raw bytes are never embedded inside the repair
  packet; they are referenced by opaque id and fingerprint only.
- **Candidate repaired version**: a proposed repaired representation
  (structured or serialized) that a surface could apply as the new
  durable artifact. A candidate is *not* truth until the user explicitly
  reviews and admits it through compare-before-apply.
- **Compare-before-apply view**: the mandatory review surface that
  compares preserved raw bytes (or a faithful projection of them) to the
  candidate repaired version and enumerates every dropped, normalized,
  regenerated, or reinterpreted field before any durable write.
- **Repair packet**: the machine-readable record that binds the suspect
  artifact, preserved raw capture, candidate repaired representation,
  compare view, and apply gate into one export-safe truth object.

## Non-negotiable invariants

### 1) No lossy repair rewrite without preserved raw + compare-first

If a repair candidate:

- drops any field, comment, attachment, output, widget state, unknown
  namespace, or formatting that may carry meaning; or
- normalizes or regenerates content in a way that cannot be proven
  byte-for-byte identical to the authored source; or
- depends on a missing adapter/runtime (partial-open) such that fidelity
  cannot be proven,

then the surface MUST:

1. preserve an exact raw copy (or refuse apply if policy or storage
   prevents preservation);
2. require compare-before-apply review; and
3. emit a `structured_round_trip_repair_packet_record` that enumerates
   the impact (dropped/normalized/regenerated fields) in both preview
   and final packet form.

One-click “Repair” is only conforming when the repair is provably
lossless and the packet’s apply gate is `allow_apply`.

### 2) Refuse rather than guess

A surface MUST set `apply_gate_class = refuse_rewrite` when any of the
following are true:

- raw preservation is unavailable (`unavailable_source_missing` or
  policy-redacted) for a candidate that is not provably lossless;
- the surface cannot compute a stable compare view (diff unavailable,
  basis unknown, candidate cannot be serialized deterministically);
- the surface cannot enumerate dropped/normalized/regenerated fields
  (unknown impact is treated as unsafe impact);
- the repair would mutate authority boundaries (policy, trust approvals,
  signing state) outside an explicit support/repair transaction.

### 3) Enumerate loss, even when it “doesn’t matter”

Any dropped, normalized, regenerated, or compatibility-downgraded fields
MUST remain enumerated:

- in the compare-before-apply view;
- in the emitted repair packet (`change_summary`); and
- in support/export surfaces that include the packet as evidence.

A surface MUST NOT hide impact behind “normalized” or “best effort” copy.

### 4) Checkpoints and rollback claims stay honest

When the repair’s reversal class is not `exact`, the repair MUST bind a
rollback checkpoint (or explicitly state the absence of one) before
apply. “Undo” language is forbidden unless the reversal class is exact.

## Required repair flows (minimum contract)

The repair packet is designed to work when the workspace is healthy and
also when it is degraded (partial-open, downgrade, missing runtime).
Each flow below names the minimum obligations and the refusal triggers.

### A) Unknown fields and schema drift

Unknown-field posture MUST follow the frozen unknown-field rules:

- user-owned and workspace-owned human-edited durable artifacts SHOULD be
  `preserve_verbatim` whenever the format allows;
- generator-owned structured artifacts (lockfiles, generated structured
  views, evidence bundles) MUST default to `refuse_read` or
  regenerate-first, not best-effort mutation.

Repair candidates that **move** unknown fields (for example, preserving
them under a namespaced key) are treated as representation-changing and
MUST require preserved raw + compare-first unless proven byte-identical.

If the surface cannot preserve unknown fields or cannot enumerate the
impact, it MUST refuse rewrite and offer compare-only/text-mode options.

### B) Decoder fallback and mixed/invalid bytes

When the suspect artifact involves decode recovery (replacement
characters, mixed encoding, undecodable byte sequences), the surface:

- MUST preserve raw bytes before offering any re-encode or replacement;
- MUST bind the repair packet to a decode-recovery review record by
  opaque ref; and
- MUST refuse rewrite when policy prevents raw-byte preservation but the
  candidate is not provably lossless.

### C) Widget downgrade (unsupported or trust-gated)

Widget downgrade is a view concern first:

- if widget trust or runtime availability denies live binding, the
  surface MUST degrade rendering (static fallback / tombstone) without
  rewriting the notebook by default;
- any candidate that strips widget state, rewrites widget metadata, or
  collapses MIME payloads is lossy and therefore MUST preserve raw +
  compare-first and MUST enumerate the affected widget/output paths.

### D) Manifest normalization

Normalization candidates (key ordering, canonical whitespace, stable
rendering normalization) MUST still be forensically honest:

- normalization that is provably semantics-preserving MAY be
  `warn_allow_apply` but MUST still publish what was normalized;
- normalization that drops comments, duplicate keys, or unsupported
  extension blocks is lossy and MUST be compare-first with preserved
  raw.

### E) Notebook output stripping

Output stripping is a repair candidate, not an automatic side effect.
When offered:

- the surface MUST preserve the raw notebook bytes before stripping;
- the compare view MUST enumerate which output blocks are removed and
  which notebook fields change (at minimum, cell output arrays and any
  execution counters touched);
- the repair packet MUST set a rerun/rebuild intent that states outputs
  are no longer authoritative until re-execution.

### F) Partial-open recovery

Repair packets MUST be valid when the workspace is partial-open:

- runtime-dependent fields (kernel, widget runtime, adapters) are
  optional and MUST accept `null` or explicit “unavailable” posture;
- the packet MUST carry enough context to justify the apply gate without
  assuming a full-open state (for example, it must not require a kernel
  session to be present to refuse a lossy repair).

### G) Downgrade compatibility

Downgrade compatibility candidates (making an artifact writable or
readable in a less-capable build or surface) are repairs with elevated
honesty requirements:

- the packet MUST label the candidate as a compatibility downgrade and
  enumerate exactly what is dropped or flattened;
- raw preservation + compare-first is mandatory unless the downgrade is
  provably byte-identical (rare);
- when the artifact family is `refuse_read` for unknown fields, the
  default posture is read-only/compare-only, not “downgrade by rewrite”.

## Support/export requirement

When a surface emits a repair packet for a repair candidate that is not
provably lossless, support/export flows MUST be able to include:

- the repair packet record;
- the preserved raw artifact reference (not the bytes inline);
- the compare-before-apply view reference; and
- the checkpoint reference (when applicable),

so support can reconstruct what was proposed and what was admitted
without relying on screenshots or free-text descriptions.

