# External alpha migration parity proof packet

This packet registers the migration-parity alpha scoreboard, import-gap
taxonomy, retained diagnostics packet, validator, and protected fixture
manifest as the current proof root for switching-path migration truth.

## Canonical Artifacts

- Parity scoreboard: `artifacts/migration/m2_parity_scoreboard.yaml`
- Import-gap taxonomy: `artifacts/migration/import_gap_taxonomy.yaml`
- Retained diagnostics packet: `docs/migration/import_diagnostics_packet.md`
- Protected fixture manifest: `fixtures/migration/parity_alpha_cases/manifest.yaml`
- Validator: `ci/check_migration_parity_alpha.py`
- Latest capture: `artifacts/milestones/m2/captures/migration_parity_validation_capture.json`
- Known limits: `artifacts/feedback/external_alpha_known_limits.md`
- Alpha scope row: `scoreboard_row:alpha_scope.migration_parity`

## Acceptance Coverage

The scoreboard distinguishes:

- `native_parity`
- `bridged_parity`
- `lossy_mapping`
- `unsupported_items`
- `manual_follow_up`

Every row carries retained import diagnostics with migration session, outcome
packet, migration report, support export, export packet, and revisit-surface
refs. Non-native rows cite taxonomy gaps, known-limit ids, issue-template refs,
claimed alpha wedges, and support-export refs.

## First Consumer

Run the CLI/support projection:

```sh
python3 ci/check_migration_parity_alpha.py --repo-root . --render-retained-diagnostics
```

Refresh the checked-in validation capture:

```sh
python3 ci/check_migration_parity_alpha.py --repo-root . --report artifacts/milestones/m2/captures/migration_parity_validation_capture.json
```

## Current Posture

This is a seeded proof scaffold. It makes the migration truth source real and
inspectable, but it does not implement importer runtime behavior or promote any
row to replacement-grade parity. Downstream importer, supportability, and
public-proof lanes should cite this packet and the alpha migration parity row
before widening claims.
