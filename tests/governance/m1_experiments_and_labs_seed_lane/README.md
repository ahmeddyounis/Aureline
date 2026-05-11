# M1 experiments / flags / Labs registry seed validation lane

Unattended Python validator for the canonical experiments / flags /
Labs registry seed published at
[`artifacts/governance/experiments_registry.yaml`](../../../artifacts/governance/experiments_registry.yaml).

The runner replays every row in the seed against the envelope schema
([`schemas/governance/experiment_registry.schema.json`](../../../schemas/governance/experiment_registry.schema.json))
and the row schema
([`schemas/governance/experiment_registry_entry.schema.json`](../../../schemas/governance/experiment_registry_entry.schema.json))
and asserts the structural invariants the reviewer landing page at
[`docs/governance/m1_experiments_and_labs_seed.md`](../../../docs/governance/m1_experiments_and_labs_seed.md)
quotes.

## Usage

```sh
python3 tests/governance/m1_experiments_and_labs_seed_lane/run_m1_experiments_and_labs_seed_lane.py --repo-root .
```

The runner writes a durable JSON capture at
`artifacts/milestones/m1/captures/experiments_and_labs_seed_validation_capture.json`.

## Failure drills

Reproduce a row's named failure drill loudly with `--force-drill`:

```sh
python3 tests/governance/m1_experiments_and_labs_seed_lane/run_m1_experiments_and_labs_seed_lane.py \
  --repo-root . \
  --force-drill 'ereg.exp.vfs.save_prototype:owner_dri_dropped'
```

The drill exits 0 only when the runner reproduces the row's declared
`expected_check_id`. Every row in the seed carries a named drill; the
full catalogue is in
[`docs/governance/m1_experiments_and_labs_seed.md`](../../../docs/governance/m1_experiments_and_labs_seed.md).

## Refresh

Re-run the lane after any change to:

- the seed YAML,
- the envelope or row schema,
- the reviewer landing page,
- the upstream `experiments_register.yaml` or `labs_register.yaml`
  whose row identity, audience, or lifecycle state changed, or
- the build-identity record the capture embeds.
