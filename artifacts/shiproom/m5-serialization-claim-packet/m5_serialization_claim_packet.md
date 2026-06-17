# Shiproom claim packet — M5 serialization qualification

This packet is the shiproom- and release-center-facing view of the serialization-qualification
family. It does not maintain its own summary: the claim scope below is read from the canonical
qualification packet and narrows automatically when a row goes stale, missing, or red.

## Canonical inputs

- Qualification packet: `artifacts/workspace/m5/m5-serialization-qualification.json`
- Reviewer artifact: `artifacts/workspace/m5/m5-serialization-qualification.md`
- Schema: `schemas/workspace/m5-serialization-qualification.schema.json`
- Companion doc: `docs/workspace/m5/m5-serialization-qualification.md`
- Fixtures: `fixtures/workspace/m5/m5-serialization-qualification/`
- Qualifies matrix: `artifacts/workspace/m5/m5-serialization-and-restore-matrix.json`
- Typed model + gate: `aureline-workspace` crate, `m5_serialization_qualification`

- Claim publishable: **yes**
- Published rows: `3`
- Narrowed rows: `4`
- Withheld rows: `2`

## Claim scope

| Family | Profile | Mode | Matrix | Evidence | Published fidelity | Claim | Recovery |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `remembered_state` | `desktop.stable` | `desktop` | exact_restore | current | **exact_restore** | Published | none |
| `restore_fidelity` | `desktop.stable` | `desktop` | exact_restore | current | **exact_restore** | Published | none |
| `restore_fidelity` | `managed.fleet` | `managed_fleet` | compatible_restore | current | **compatible_restore** | Narrowed | adopt_matrix_narrowing |
| `portable_state_review` | `desktop.stable` | `desktop` | exact_restore | current | **exact_restore** | Published | none |
| `portable_state_review` | `companion.browser` | `companion_browser` | exact_restore | aging | **compatible_restore** | Narrowed | refresh_evidence |
| `migration_remap` | `desktop.beta` | `desktop` | compatible_restore | current | **compatible_restore** | Narrowed | rerun_drills |
| `migration_remap` | `managed.fleet` | `managed_fleet` | layout_only | expired | **manual_review** | Withheld | withhold_claim |
| `missing_surface_continuity` | `desktop.stable` | `desktop` | layout_only | current | **layout_only** | Narrowed | rerun_drills |
| `missing_surface_continuity` | `companion.browser` | `companion_browser` | manual_review | missing | **manual_review** | Withheld | withhold_claim |

## Sign-off gate

Promotion of the serialization claim holds unless all of the following are true on the current
qualification packet (`M5SerializationQualification::validate()` returns no violations):

1. Every claimed family carries at least one qualification row, and each
   `(family, profile, deployment_mode)` row carries its own proof — no row borrows a nearby
   profile's restore proof.
2. Every row's `published_fidelity`, `claim_publication`, `downgrade_reasons`, and `downgrade_path`
   equal the recomputed fail-closed gate — a matrix narrowing, stale or missing evidence, or a
   narrowed or failed drill narrows or withholds the row automatically, and no row publishes above
   its matrix claim.
3. No withheld row publishes a qualified class, and every narrowed or withheld row names its
   recovery path, its caveats, and the stale or missing fields driving the narrowing.
4. No missing-surface row implies silent layout deletion: missing dependencies narrow to
   `layout_only` or `manual_review` with slot-preserving placeholders, never below.
5. No portable-state-review row claims full portability where the package depends on machine-local
   state or an unsupported feature pack.
6. The five consumer bindings (docs/help, support export, companion/browser handoff, release center,
   shiproom) are all present and reuse this packet's published fidelity, recovery paths, and
   narrowing.

A narrowed restore claim is never silent: a matrix-narrowed surface, an aging provenance record, a
forward-migrated schema jump, a missing extension, and an unmigratable-schema-plus-expired-evidence
row each surface as their own downgrade reason and recovery path rather than shipping as an implied
exact restore.

## Reviewer checklist

- [ ] `cargo test -p aureline-workspace m5_serialization_qualification` passes.
- [ ] The artifact validates against the schema (no schema/example drift).
- [ ] Three rows publish a full `exact_restore` claim, proving the qualifier is not a blanket
      downgrade.
- [ ] Each narrowed or withheld row names its downgrade reason, recovery path, and stale/missing
      fields.
- [ ] No live authority, secret, or machine-local state is serialized or implied portable.

## Regenerating this packet

This packet is checked in alongside the qualification it renders. When the qualification contract
changes, update the packet, schema, reviewer artifact, and fixtures together, then re-run the gate
before re-reviewing:

```sh
cargo test -p aureline-workspace m5_serialization_qualification
python3 - <<'PY'
import json
from jsonschema import Draft202012Validator
schema = json.load(open("schemas/workspace/m5-serialization-qualification.schema.json"))
data = json.load(open("artifacts/workspace/m5/m5-serialization-qualification.json"))
errors = list(Draft202012Validator(schema).iter_errors(data))
print("schema OK" if not errors else errors)
PY
```
