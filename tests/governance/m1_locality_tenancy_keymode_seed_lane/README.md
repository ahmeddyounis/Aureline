# M1 locality / tenancy / key-mode vocabulary seed validation lane

Unattended Python validator for the canonical locality / tenancy /
key-mode vocabulary seed published at
[`artifacts/governance/locality_examples.yaml`](../../../artifacts/governance/locality_examples.yaml).

The runner replays every row in the seed against the envelope schema
([`schemas/governance/m1_locality_tenancy_keymode_seed.schema.json`](../../../schemas/governance/m1_locality_tenancy_keymode_seed.schema.json))
and the row schema
([`schemas/governance/locality_tenancy_keymode.schema.json`](../../../schemas/governance/locality_tenancy_keymode.schema.json))
and asserts the structural invariants the docs landing page at
[`docs/governance/m1_locality_tenancy_keymode_vocabulary.md`](../../../docs/governance/m1_locality_tenancy_keymode_vocabulary.md)
quotes.

## Usage

```sh
python3 tests/governance/m1_locality_tenancy_keymode_seed_lane/run_m1_locality_tenancy_keymode_seed_lane.py --repo-root .
```

The runner writes a durable JSON capture at
`artifacts/milestones/m1/captures/locality_tenancy_keymode_vocabulary_validation_capture.json`.

## Failure drills

Reproduce a row's named failure drill loudly with `--force-drill`:

```sh
python3 tests/governance/m1_locality_tenancy_keymode_seed_lane/run_m1_locality_tenancy_keymode_seed_lane.py \
  --repo-root . \
  --force-drill 'managed_control_plane.hosted_control_plane_uncertainty:locality_tenancy_keymode_drill.managed_control_plane_certifies_unknown_tenancy'
```

The drill exits 0 only when the runner reproduces the row's declared
`expected_check_id`. Every row in the seed carries a named drill;
the full catalogue is in
[`docs/governance/m1_locality_tenancy_keymode_vocabulary.md`](../../../docs/governance/m1_locality_tenancy_keymode_vocabulary.md).

## Refresh

Re-run the lane after any change to:

- the seed YAML,
- the envelope or row schema,
- the reviewer landing page,
- the upstream internal boundary manifest the seed projects against, or
- the build-identity record the capture embeds.
