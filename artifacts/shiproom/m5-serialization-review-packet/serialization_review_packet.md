# Shiproom review packet — M5 workspace-serialization and restore fidelity

This packet is the shiproom-facing view of the serialization-and-restore matrix. It does not
maintain its own summary: every claim below is read from the canonical objects, and the row table
is rendered from the same packet as
`artifacts/workspace/m5/m5-serialization-and-restore-matrix.md`.

## Canonical inputs

- Matrix packet: `artifacts/workspace/m5/m5-serialization-and-restore-matrix.json`
- Reviewer artifact: `artifacts/workspace/m5/m5-serialization-and-restore-matrix.md`
- Schema: `schemas/workspace/m5-serialization-matrix.schema.json`
- Companion doc: `docs/workspace/m5/m5-serialization-and-restore.md`
- Fixtures: `fixtures/workspace/m5/m5-serialization-and-restore/`
- Typed model + gate: `aureline-workspace` crate, `m5_serialization_and_restore_matrix`

## Sign-off gate

Promotion holds unless all of the following are true on the current matrix
(`M5SerializationMatrix::validate()` returns no violations):

1. Every remembered-state artifact class and every restorable surface carries exactly one row;
   no class or surface inherits a restore label from an adjacent one.
2. Every row's `published_fidelity`, `downgrade_reasons`, and `recovery_path` equal the
   recomputed fail-closed gate — a schema drift, missing dependency, changed topology, or stale
   evidence narrows the row automatically, and no row publishes above its declared maximum or
   beyond what its class supports.
3. No row would silently delete layout: every `missing_dependency_behavior` is
   `placeholder_slot_preserved` or `reopen_as_context`; `silent_delete` is rejected outright.
4. Portability is backed: every exportable row excludes secrets, live authority, machine-local
   anchors, and raw provider payloads, and no `machine_local` row is exportable.
5. No surface out-claims what it persists: a surface's restore fidelity and portability never
   exceed the artifact classes it persists, and a portable/shared surface persists at least one
   exportable class.
6. The four continuity cross-links (crash recovery, browser/companion handoff, import/export,
   claim publication) and the three reviewer bindings (shiproom, docs/help, support export) are
   all present and reuse this matrix's vocabulary.

A narrowed restore is never silent: a forward-migrated import, an adapted topology, an aging
provenance record, a missing dependency, and an unmigratable-schema-plus-expired-evidence summary
each surface as their own downgrade reason and recovery path rather than shipping as an implied
exact restore.

## Reviewer checklist

- [ ] `cargo test -p aureline-workspace m5_serialization` passes.
- [ ] The artifact validates against the schema (no schema/example drift).
- [ ] Each artifact-class row's ownership, redaction policy, and restore-fidelity ceiling read
      correctly against the portable-state, restore-provenance, and pane-tree contracts.
- [ ] Layout restore, portable-state export, and crash-recovery evidence are kept distinct — not
      collapsed because they share artifacts.
- [ ] No live authority, secret, or machine-local state is serialized or implied portable.

## Regenerating this packet

This packet is checked in alongside the matrix it reviews. When the matrix contract changes,
update the matrix, schema, reviewer artifact, and fixtures together, then re-run the gate before
re-reviewing:

```sh
cargo test -p aureline-workspace m5_serialization
python3 - <<'PY'
import json
from jsonschema import Draft202012Validator
schema = json.load(open("schemas/workspace/m5-serialization-matrix.schema.json"))
data = json.load(open("artifacts/workspace/m5/m5-serialization-and-restore-matrix.json"))
errors = list(Draft202012Validator(schema).iter_errors(data))
print("schema OK" if not errors else errors)
PY
```
