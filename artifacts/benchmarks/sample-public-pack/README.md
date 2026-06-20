# Sample public-comparison reproducibility pack

This directory holds a **worked, standalone reproducibility pack** that an
independent reviewer can read to rerun or audit a public benchmark comparison.
It is the copyable example for the public-comparison pack lane.

- [`head-to-head-reproducibility-pack.json`](./head-to-head-reproducibility-pack.json)
  — a fully populated `public_head_to_head_comparison` pack. It backs the
  `publication_pack.head_to_head.first_useful_edit` row in the
  [benchmark-governance matrix](../m5-benchmark-governance.json) and carries the
  raw configuration, exact commands, corpus and revision refs, reference-hardware
  and lab-image identity, environment notes, caveats, raw-run-metadata refs, and
  reproduction recipe a reviewer needs.

## How this fits the lane

| Artifact | Role |
|---|---|
| [`schemas/benchmarks/public-comparison-pack.schema.json`](../../../schemas/benchmarks/public-comparison-pack.schema.json) | Boundary schema for a reproducibility pack and the pack register. |
| [`artifacts/benchmarks/public-comparison-pack-register.json`](../public-comparison-pack-register.json) | Canonical register binding every governance publication pack to a reproducibility pack — the truth source consumers cite. |
| This directory | Worked standalone example a reviewer or author copies. |
| [`docs/benchmarks/public-comparison-packs.md`](../../../docs/benchmarks/public-comparison-packs.md) | Normative policy for the lane. |
| [`ci/check_public_comparison_pack.py`](../../../ci/check_public_comparison_pack.py) | Validator that enforces coverage and the fail-closed rules. |

## Rerun recipe

1. Provision the bound reference-lab hardware profile
   (`hardware_definition.ref.macos15.arm64.apple_silicon_14in`) and the bound
   lab-image revision (`lab_image.macos15.arm64.rev1`); verify thermal headroom.
2. Resolve the corpus, protected-metrics, and fitness-catalog revisions from
   their manifests so you exercise the same revisions the pack discloses.
3. Run the exact `command_lines` from the pack with the published `config_knobs`
   for both Aureline and the compared product, back to back.
4. Compare both retained captures against the published `task_parity_note` and
   `caveats`; if any disclosed comparability axis changed, the claim is refreshed
   or withdrawn rather than republished by implication.

Credential bodies, raw run logs, raw provider payloads, and raw machine labels
never appear in a pack — only stable ids, publishable command lines and config
knobs, and reviewable sentences do.
