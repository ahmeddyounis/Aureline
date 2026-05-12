# Retained import diagnostics packet

This packet defines the retained diagnostics that make migration outcomes
reviewable after first-run onboarding closes. The canonical row sources are:

- `artifacts/migration/m2_parity_scoreboard.yaml`
- `artifacts/migration/import_gap_taxonomy.yaml`
- `artifacts/milestones/m2/alpha_wedge_matrix.yaml`
- `artifacts/milestones/m2/exit_gate_scoreboard.yaml`
- `artifacts/feedback/external_alpha_known_limits.md`

The packet is structural. It does not implement importers or UI. It freezes the
refs, states, and issue-template bindings that importers, migration center,
support export, and design-partner intake must preserve.

## Parity States

| State | Meaning |
|---|---|
| `native_parity` | The target can represent the imported object as an Aureline-native record without a migration caveat beyond normal canonicalization. |
| `bridged_parity` | Continuity depends on an explicit bridge, shim, or native-alternative recommendation; native parity is not claimed. |
| `lossy_mapping` | A semantic mapping exists, but behavior, gesture, token, or execution detail narrows and must stay visible. |
| `unsupported_items` | No safe target exists for the source concept; the row remains visible and may block apply for that object. |
| `manual_follow_up` | A human must accept, edit, or reject the row before the workflow can be treated as migrated. |

## Retained Diagnostics

Every parity row carries a `retained_import_diagnostics` block with:

| Field | Required use |
|---|---|
| `migration_session_ref` | Stable session id that can be reopened from migration center history. |
| `outcome_packet_ref` | Grouped importer-outcome packet with all six outcome counters. |
| `migration_report_ref` | Export-safe report id for docs, support, and design-partner review. |
| `support_export_ref` | Support bundle or support packet ref that carries the row without raw source payloads. |
| `export_packet_ref` | This packet anchor or later versioned packet anchor. |
| `revisit_surface_refs` | Must include `migration_center_history`, `support_export`, and `issue_template`. |

The retained packet may carry refs, counts, states, caveats, known-limit ids,
scorecard ids, and docs/help links. It must not carry raw source profile
bodies, raw absolute paths, extension storage blobs, credentials, secrets, or
workspace file contents.

## Native Parity

Native parity rows still retain diagnostics. A clean settings or common keymap
import can be reopened later to compare the source descriptor, selected
domains, imported rows, and support/export refs.

## Bridged Parity

Bridge-backed rows keep the bridge or native-alternative scorecard attached.
The migration center, issue template, and support export must show that the row
is bridge-backed and cannot be described as native parity.

## Lossy Mapping

Lossy rows keep the original source value, mapped target value, caveat, and
known-gap taxonomy refs. Shortcut rows also keep the original gesture, remapped
gesture, conflict resolution, and muscle-memory risk note.

## Unsupported Items

Unsupported rows remain visible in preview, final report, support export, and
issue-template payloads. Unsupported is not a subtype of skipped. When a row is
blocked by runtime, permission, policy, or missing native target, the blocker
travels with the report.

## Manual Follow-Up

Manual follow-up rows remain open until validation records an accepted,
edited, rejected, or rolled-back decision. Run/debug and task rows must keep
execution-context review refs so a later support engineer can distinguish a
seeded limitation from a regression.

## Issue Template Binding

Any row below `native_parity` must cite import-gap taxonomy rows. Each taxonomy
row binds the gap to:

- claimed alpha wedge refs,
- known-limit ids,
- issue-template refs,
- required issue fields,
- docs/help refs, and
- support-export refs.

Required issue fields are listed in `artifacts/migration/import_gap_taxonomy.yaml`.
They include source tool/version, alpha wedge, migration session, outcome
packet, migration report, taxonomy gap refs, parity scoreboard row, support
export, and known-limit refs.

## First Consumer

The first consumer is the CLI/support projection:

```sh
python3 ci/check_migration_parity_alpha.py --repo-root . --render-retained-diagnostics
```

The validator checks that every parity state is represented, every non-native
row binds to taxonomy gaps and issue templates, retained diagnostics survive
after onboarding, and every taxonomy gap points to claimed alpha wedges and
known-limit ids.

Refresh the checked-in validation capture with:

```sh
python3 ci/check_migration_parity_alpha.py --repo-root . --report artifacts/milestones/m2/captures/migration_parity_validation_capture.json
```
