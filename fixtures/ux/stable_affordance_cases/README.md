# Stable affordance mapping fixtures

This directory contains worked `stable_affordance_row_record` examples that
exercise the stable affordance mapping contract:

- `Copy command ID`
- `Copy CLI equivalent`
- `Open schema/docs`
- `Run migration check`
- `Open support packet`
- `Copy stable issue/advisory ref`

Fixtures are designed to prove two properties:

1) Stable ids survive across desktop UI, CLI/headless output, docs/help, migration reports, and support exports.
2) When a mapping is unavailable, the slot degrades explicitly (disabled with a reason + contract link) rather than silently disappearing.

Schema: `schemas/ux/stable_affordance_row.schema.json`  
Contract: `docs/ux/stable_affordance_mapping_contract.md`

