# M1 device-registry and settings-sync seed validation lane

Unattended Python validator for the canonical device-registry and
settings-sync seed published at
[`artifacts/settings/m1_device_registry_and_sync_seed.yaml`](../../../artifacts/settings/m1_device_registry_and_sync_seed.yaml).

The runner replays every row in the seed against the envelope schema
([`schemas/settings/device_registry.schema.json`](../../../schemas/settings/device_registry.schema.json))
and the row schema
([`schemas/settings/settings_sync_state.schema.json`](../../../schemas/settings/settings_sync_state.schema.json))
and asserts the structural invariants the docs landing page at
[`docs/settings/m1_sync_and_device_seed.md`](../../../docs/settings/m1_sync_and_device_seed.md)
quotes.

## Usage

```sh
python3 tests/governance/m1_device_registry_and_sync_seed_lane/run_m1_device_registry_and_sync_seed_lane.py --repo-root .
```

The runner writes a durable JSON capture at
`artifacts/milestones/m1/captures/device_registry_and_sync_seed_validation_capture.json`.

## Failure drills

Reproduce a row's named failure drill loudly with `--force-drill`:

```sh
python3 tests/governance/m1_device_registry_and_sync_seed_lane/run_m1_device_registry_and_sync_seed_lane.py \
  --repo-root . \
  --force-drill 'active.local_authoritative.no_conflict:device_sync_state_drill.active_local_authoritative_non_widening_affirmation_dropped'
```

The drill exits 0 only when the runner reproduces the row's declared
`expected_check_id`. Every row in the seed carries a named drill; the
full catalogue is in
[`docs/settings/m1_sync_and_device_seed.md`](../../../docs/settings/m1_sync_and_device_seed.md).

## Refresh

Re-run the lane after any change to:

- the seed YAML,
- the envelope or row schema,
- the reviewer landing page,
- the upstream optional-sync contract the seed projects against, or
- the build-identity record the capture embeds.
