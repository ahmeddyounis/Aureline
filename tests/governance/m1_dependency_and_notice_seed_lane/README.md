# M1 critical-dependency / import register seed validation lane

Unattended Python validator for the canonical critical-dependency /
import register seed published at
[`artifacts/governance/critical_dependency_register.yaml`](../../../artifacts/governance/critical_dependency_register.yaml).

The runner replays every row in the seed against the envelope schema
([`schemas/governance/critical_dependency_register.schema.json`](../../../schemas/governance/critical_dependency_register.schema.json))
and the row schema
([`schemas/governance/critical_dependency_register_entry.schema.json`](../../../schemas/governance/critical_dependency_register_entry.schema.json))
and asserts the structural invariants the reviewer landing page at
[`docs/governance/m1_dependency_and_notice_seed.md`](../../../docs/governance/m1_dependency_and_notice_seed.md)
quotes.

## Usage

```sh
python3 tests/governance/m1_dependency_and_notice_seed_lane/run_m1_dependency_and_notice_seed_lane.py --repo-root .
```

The runner writes a durable JSON capture at
`artifacts/milestones/m1/captures/dependency_and_notice_seed_validation_capture.json`.

The lane also asserts that the draft notice / report outputs declared
in `draft_output_refs` exist on disk. Regenerate them with the draft
pipeline before running the lane on a clean tree:

```sh
python3 tools/governance/build_dependency_notice_seed.py --repo-root .
```

## Failure drills

Reproduce a row's named failure drill loudly with `--force-drill`:

```sh
python3 tests/governance/m1_dependency_and_notice_seed_lane/run_m1_dependency_and_notice_seed_lane.py \
  --repo-root . \
  --force-drill 'cdr.runtime_dependency.renderer_wgpu:critical_dependency_register_drill.runtime_dependency_third_party_notice_dropped'
```

The drill exits 0 only when the runner reproduces the row's declared
`expected_check_id`. Every row in the seed carries a named drill; the
full catalogue is in
[`docs/governance/m1_dependency_and_notice_seed.md`](../../../docs/governance/m1_dependency_and_notice_seed.md).

## Refresh

Re-run the lane after any change to:

- the seed YAML,
- the envelope or row schema,
- the reviewer landing page,
- a companion register (`dependency_register.yaml`,
  `third_party_import_register.yaml`,
  `release_notice_seed.yaml`) whose row identity or critical posture
  changed,
- the draft pipeline tool, or
- the build-identity record the capture embeds.
