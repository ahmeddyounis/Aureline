# Generated-artifact local history and timeline

Ordinary local history implies full-source byte continuity: every entry looks
like a complete snapshot the user can restore exactly. Generated artifacts
break that assumption. A scaffolded file may carry a full snapshot; a notebook
output or preview derivative may store only metadata and a reference to its
canonical source; a regenerated candidate may be a fresh re-run rather than
the original bytes; an oversized or policy-withheld artifact may be omitted
entirely. If the timeline presents all of these as ordinary full-source
history, restore and compare quietly lie about what they can reproduce.

This lane freezes one typed model for a generated artifact's history. It is
implemented in
[`crates/aureline-generated/src/generated_timeline/mod.rs`](../../crates/aureline-generated/src/generated_timeline/mod.rs)
and serialized to
[`artifacts/generated/generated-timeline-packet.json`](../../artifacts/generated/generated-timeline-packet.json).

It is the checked-in truth source for:

- the boundary schema at
  [`schemas/generated/generated-timeline-entry.schema.json`](../../schemas/generated/generated-timeline-entry.schema.json)
- the proof packet and reviewer summary under
  [`artifacts/generated/`](../../artifacts/generated/)
- fixture replay in
  [`crates/aureline-generated/tests/generated_timeline.rs`](../../crates/aureline-generated/tests/generated_timeline.rs)
- the fixture corpus under
  [`fixtures/generated/timeline/`](../../fixtures/generated/timeline/)

## What each timeline entry records

Every entry records, explicitly:

- **Capture mode** — `full_snapshot`, `metadata_plus_reference`,
  `regenerated_candidate`, or `omitted_bytes`. This is the structural form of
  what local history actually stored.
- **Redaction class** — `none`, `secrets_redacted`, `size_capped`, or
  `policy_withheld`. Redaction is orthogonal to the capture mode: a full
  snapshot can still be redacted, in which case the stored bytes are no longer
  a faithful copy.
- **Lineage links** — the generator identity, the canonical-source reference,
  the divergence (drift) state, and the reversible-checkpoint lineage
  reference, so an entry always traces back to what produced it and what it
  derives from.

## The engine and what it decides

One engine (`classify_generated_history`) folds the capture mode, the
redaction class, and the divergence state into a single outcome:

- a **restore fidelity** — `exact_snapshot`, `compatible_regeneration`, or
  `evidence_only`;
- whether **exact generated-byte continuity** may be claimed at all;
- a **byte-provenance** explanation of what was captured directly,
  reconstructed from the canonical source plus metadata, or intentionally
  omitted;
- a **compare basis** — `byte_snapshot`, `regenerated_candidate`, or
  `evidence_manifest`;
- a **restore availability** — `available`, `review_required`, or
  `disabled_export_only`;
- and stable **block-reason tokens** naming every input that narrowed the
  fidelity.

The fidelity only narrows. It starts at the capture mode's base
(`full_snapshot` → exact, reference/candidate → compatible, omitted →
evidence), is floored by redaction (any redaction caps below exact; a policy
withholding leaves only evidence), and — when restore must regenerate — is
floored again by a missing canonical source (no source to rebuild from leaves
only evidence). A full snapshot holds the original bytes locally, so a
drifting or missing canonical source never weakens its exact restore.

## The frozen guardrail

**Exact generated-byte continuity is claimed only when the timeline captured a
full, unredacted snapshot.** A metadata-plus-reference, regenerated-candidate,
omitted, or redacted capture never lets restore or compare claim exact byte
continuity, and a byte-snapshot compare is offered only when that claim holds.
The fixture corpus exercises each case:

| Fixture | Capture | Redaction | Source | Restore fidelity | Exact continuity |
| --- | --- | --- | --- | --- | --- |
| `full_snapshot_exact` | `full_snapshot` | `none` | `linked` | `exact_snapshot` | yes |
| `metadata_plus_reference_compatible` | `metadata_plus_reference` | `none` | `linked` | `compatible_regeneration` | no |
| `regenerated_candidate_drifting` | `regenerated_candidate` | `none` | `linked` (drifting) | `compatible_regeneration` | no |
| `omitted_bytes_evidence_only` | `omitted_bytes` | `size_capped` | `linked` | `evidence_only` | no |
| `redacted_full_snapshot_not_exact` | `full_snapshot` | `secrets_redacted` | `linked` | `compatible_regeneration` | no |
| `reference_source_missing_evidence_only` | `metadata_plus_reference` | `none` | `missing` | `evidence_only` | no |
| `policy_withheld_evidence_only` | `metadata_plus_reference` | `policy_withheld` | `linked` | `evidence_only` | no |

## Surface bindings

Every binding ingests the same packet id
(`generated.generated_timeline.v1`) and consumes the entry outcome rather than
re-deriving history semantics:

- `history_timeline` — `crates/aureline-history/src/local_history/mod.rs`
- `compare_view` — `crates/aureline-review/src/change_inspector/mod.rs`
- `restore_preview` — `crates/aureline-recovery/src/lib.rs`
- `support_export` — `crates/aureline-support/src/generated_lineage/mod.rs`

## Export safety

The support/export projection on every entry is metadata-safe and
lineage-preserving: it keeps the entry id, the generator identity, the
canonical-source and checkpoint references, the capture mode, and the restore
fidelity, and excludes raw captured bodies, secret material, and live
authority. Checkpoint packets therefore stay export-safe while still tracing
back to generator identity, canonical source, and divergence state.
